use crate::{Error, Result, material::*};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
#[derive(Clone)]
pub(crate) struct GpuTexture {
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
#[derive(Default)]
pub(crate) struct TextureCache {
    entries: HashMap<usize, (Weak<Texture>, GpuTexture)>,
    pub uploads: u64,
    mip_pipelines: HashMap<wgpu::TextureFormat, wgpu::RenderPipeline>,
}
impl TextureCache {
    pub fn prune(&mut self) {
        self.entries
            .retain(|_, (owner, _)| owner.strong_count() > 0);
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &Arc<Texture>,
    ) -> Result<GpuTexture> {
        let key = Arc::as_ptr(image) as usize;
        if let Some((owner, gpu)) = self.entries.get(&key)
            && owner.strong_count() > 0
        {
            return Ok(gpu.clone());
        }
        if image.width == 0
            || image.height == 0
            || image.width > device.limits().max_texture_dimension_2d
            || image.height > device.limits().max_texture_dimension_2d
            || image.rgba.len() != image.width as usize * image.height as usize * 4
        {
            return Err(Error::Invalid("texture dimensions/data"));
        }
        let levels = if image.mipmap_filter.is_some() {
            image.width.max(image.height).ilog2() + 1
        } else {
            1
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cached material texture"),
            size: wgpu::Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if image.srgb {
                wgpu::TextureFormat::Rgba8UnormSrgb
            } else {
                wgpu::TextureFormat::Rgba8Unorm
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        queue.write_texture(
            texture.as_image_copy(),
            &image.rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(image.width * 4),
                rows_per_image: Some(image.height),
            },
            texture.size(),
        );
        if levels > 1 {
            let pipeline = self
                .mip_pipelines
                .entry(texture.format())
                .or_insert_with(|| {
                    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("linear mip filtering"),
                        source: wgpu::ShaderSource::Wgsl(
                            include_str!("shaders/mipmap.wgsl").into(),
                        ),
                    });
                    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("mip filter"),
                        layout: None,
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: Some("vs"),
                            compilation_options: Default::default(),
                            buffers: &[],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &shader,
                            entry_point: Some("fs"),
                            compilation_options: Default::default(),
                            targets: &[Some(wgpu::ColorTargetState {
                                format: texture.format(),
                                blend: None,
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: Default::default(),
                        depth_stencil: None,
                        multisample: Default::default(),
                        multiview: None,
                        cache: None,
                    })
                });
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            let views = (0..levels)
                .map(|level| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        base_mip_level: level,
                        mip_level_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect::<Vec<_>>();
            let mut encoder = device.create_command_encoder(&Default::default());
            for level in 1..levels as usize {
                let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("mip source"),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&views[level - 1]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("mip generation"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &views[level],
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &bindings, &[]);
                pass.draw(0..3, 0..1);
            }
            queue.submit([encoder.finish()]);
        }
        let wrap = |v| match v {
            Wrapping::Clamp => wgpu::AddressMode::ClampToEdge,
            Wrapping::Repeat => wgpu::AddressMode::Repeat,
            Wrapping::Mirror => wgpu::AddressMode::MirrorRepeat,
        };
        let filter = |v| match v {
            Filter::Nearest => wgpu::FilterMode::Nearest,
            Filter::Linear => wgpu::FilterMode::Linear,
        };
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wrap(image.wrap_s),
            address_mode_v: wrap(image.wrap_t),
            mag_filter: filter(image.filter),
            min_filter: filter(image.min_filter.unwrap_or(image.filter)),
            mipmap_filter: filter(image.mipmap_filter.unwrap_or(Filter::Nearest)),
            ..Default::default()
        });
        let gpu = GpuTexture {
            view: texture.create_view(&Default::default()),
            sampler,
        };
        self.entries
            .insert(key, (Arc::downgrade(image), gpu.clone()));
        self.uploads += 1;
        Ok(gpu)
    }
}
