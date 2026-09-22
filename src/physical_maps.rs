//! Extension maps share one array binding to stay within WebGPU's 16-texture limit.
//! Occupied layers retain original dimensions and GPU-generated mip chains.
use crate::{Error, Result, material::*};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
pub const COUNT: usize = 12;
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct MapUniforms {
    pub matrices: [[f32; 4]; COUNT * 3],
    pub sizes: [[f32; 4]; COUNT],
    pub wraps: [[f32; 4]; COUNT],
    // minification filter, mip filter, maximum mip level, packed layer index.
    pub sampling: [[f32; 4]; COUNT],
}
impl MapUniforms {
    pub fn new(maps: &[Option<&Arc<Texture>>; COUNT]) -> Self {
        let mut result = Self {
            matrices: [[0.0; 4]; COUNT * 3],
            sizes: [[0.0; 4]; COUNT],
            wraps: [[0.0; 4]; COUNT],
            sampling: [[0.0; 4]; COUNT],
        };
        let mut layer = 0;
        for (i, map) in maps.iter().enumerate() {
            if let Some(t) = map {
                for c in 0..3 {
                    let column = t.uv_matrix().col(c).as_vec3();
                    result.matrices[i * 3 + c] = [
                        column.x,
                        column.y,
                        column.z,
                        if c == 2 {
                            t.tex_coord as f32
                        } else if c == 0 {
                            f32::from(t.flip_y)
                        } else {
                            0.0
                        },
                    ];
                }
                result.sizes[i] = [
                    t.width as f32,
                    t.height as f32,
                    f32::from(t.filter == Filter::Linear),
                    f32::from(t.srgb),
                ];
                let wrap = |w| match w {
                    Wrapping::Clamp => 0.0,
                    Wrapping::Repeat => 1.0,
                    Wrapping::Mirror => 2.0,
                };
                result.wraps[i] = [wrap(t.wrap_s), wrap(t.wrap_t), 0.0, 0.0];
                result.sampling[i] = [
                    f32::from(t.min_filter.unwrap_or(t.filter) == Filter::Linear),
                    f32::from(t.mipmap_filter == Some(Filter::Linear)),
                    if t.mipmap_filter.is_some() {
                        t.width.max(t.height).max(1).ilog2() as f32
                    } else {
                        0.0
                    },
                    layer as f32,
                ];
                layer += 1;
            }
        }
        result
    }
}
type Entry = (Vec<Weak<Texture>>, wgpu::TextureView);
#[derive(Default)]
pub(crate) struct Cache {
    entries: HashMap<[usize; COUNT], Entry>,
}
impl Cache {
    pub fn prune(&mut self) {
        self.entries
            .retain(|_, (owners, _)| owners.iter().all(|o| o.strong_count() > 0));
    }
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        maps: [Option<&Arc<Texture>>; COUNT],
    ) -> Result<wgpu::TextureView> {
        self.entries
            .retain(|_, (owners, _)| owners.iter().all(|o| o.strong_count() > 0));
        let key = maps.map(|m| m.map_or(0, |m| Arc::as_ptr(m) as usize));
        if let Some((_, view)) = self.entries.get(&key) {
            return Ok(view.clone());
        }
        if maps.iter().flatten().any(|t| t.basis.is_some()) {
            return Err(Error::Invalid(
                "compressed physical-extension texture arrays are unsupported",
            ));
        }
        let width = maps.iter().flatten().map(|t| t.width).max().unwrap_or(1);
        let height = maps.iter().flatten().map(|t| t.height).max().unwrap_or(1);
        let layers = maps.iter().flatten().count().max(1) as u32;
        let levels = maps
            .iter()
            .flatten()
            .map(|t| {
                if t.mipmap_filter.is_some() {
                    t.width.max(t.height).max(1).ilog2() + 1
                } else {
                    1
                }
            })
            .max()
            .unwrap_or(1);
        if width == 0
            || height == 0
            || width > device.limits().max_texture_dimension_2d
            || height > device.limits().max_texture_dimension_2d
            || (0..levels)
                .map(|level| {
                    u64::from((width >> level).max(1))
                        * u64::from((height >> level).max(1))
                        * u64::from(layers)
                        * 4
                })
                .sum::<u64>()
                > 256 * 1024 * 1024
        {
            return Err(Error::Invalid(
                "physical map array dimensions (256 MiB limit)",
            ));
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("physical extension maps"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: layers,
            },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // Reuse the normal GPU mip generator, then release these temporary source
        // textures. The resident array allocates only occupied layers, not all 12 slots.
        let mut sources = crate::texture_gpu::TextureCache::default();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("physical map mip copies"),
        });
        let mut temporary = Vec::new();
        for (layer, image) in maps.iter().flatten().enumerate() {
            let source = sources.get(device, queue, image)?;
            temporary.push(source.texture.clone());
            for mip in 0..source.texture.mip_level_count() {
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &source.texture,
                        mip_level: mip,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: mip,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: layer as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: (image.width >> mip).max(1),
                        height: (image.height >> mip).max(1),
                        depth_or_array_layers: 1,
                    },
                );
            }
        }
        queue.submit([encoder.finish()]);
        for texture in temporary {
            texture.destroy();
        }
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        self.entries.insert(
            key,
            (
                maps.iter().flatten().map(|t| Arc::downgrade(t)).collect(),
                view.clone(),
            ),
        );
        Ok(view)
    }
}
