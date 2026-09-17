//! Extension maps share one array binding to stay within WebGPU's 16-texture limit.
//! Layers retain original pixels and dimensions; sampling is bilinear, without mipmaps.
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
}
impl MapUniforms {
    pub fn new(maps: &[Option<&Arc<Texture>>; COUNT]) -> Self {
        let mut result = Self {
            matrices: [[0.0; 4]; COUNT * 3],
            sizes: [[0.0; 4]; COUNT],
            wraps: [[0.0; 4]; COUNT],
        };
        for (i, map) in maps.iter().enumerate() {
            if let Some(t) = map {
                for c in 0..3 {
                    let column = t.uv_matrix().col(c).as_vec3();
                    result.matrices[i * 3 + c] = [
                        column.x,
                        column.y,
                        column.z,
                        if c == 2 { t.tex_coord as f32 } else { 0.0 },
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
        let width = maps.iter().flatten().map(|t| t.width).max().unwrap_or(1);
        let height = maps.iter().flatten().map(|t| t.height).max().unwrap_or(1);
        let layers = if maps.iter().any(Option::is_some) {
            COUNT as u32
        } else {
            1
        };
        if width == 0
            || height == 0
            || width > device.limits().max_texture_dimension_2d
            || height > device.limits().max_texture_dimension_2d
            || u64::from(width) * u64::from(height) * u64::from(layers) * 4 > 256 * 1024 * 1024
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
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for (layer, map) in maps.iter().enumerate() {
            if let Some(t) = map {
                if t.rgba.len() != t.width as usize * t.height as usize * 4 {
                    return Err(Error::Invalid("physical texture data"));
                }
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: layer as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &t.rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(t.width * 4),
                        rows_per_image: Some(t.height),
                    },
                    wgpu::Extent3d {
                        width: t.width,
                        height: t.height,
                        depth_or_array_layers: 1,
                    },
                );
            }
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
