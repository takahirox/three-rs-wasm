//! Resident GPU mip generation for renderable 2D textures, including HDR storage.
use crate::{Error, Result};
pub struct MipGenerator {
    views: Vec<wgpu::TextureView>,
    bindings: Vec<wgpu::BindGroup>,
    pipeline: wgpu::RenderPipeline,
}
impl MipGenerator {
    /// The texture must have TEXTURE_BINDING and RENDER_ATTACHMENT usage.
    /// Resources remain resident; update performs GPU filtering only.
    pub fn new(device: &wgpu::Device, texture: &wgpu::Texture) -> Result<Self> {
        if texture.dimension() != wgpu::TextureDimension::D2
            || texture.sample_count() != 1
            || !texture.usage().contains(
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            )
            || texture.format().sample_type(None, Some(device.features()))
                != Some(wgpu::TextureSampleType::Float { filterable: true })
            || !texture
                .format()
                .guaranteed_format_features(device.features())
                .allowed_usages
                .contains(wgpu::TextureUsages::RENDER_ATTACHMENT)
        {
            return Err(Error::Invalid("mip generator texture"));
        }
        let levels = texture.mip_level_count();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("GPU mip filter"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/mipmap.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("GPU mip filter"),
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
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut views = Vec::new();
        let mut bindings = Vec::new();
        for layer in 0..texture.depth_or_array_layers() {
            let layer_views: Vec<_> = (0..levels)
                .map(|level| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_array_layer: layer,
                        array_layer_count: Some(1),
                        base_mip_level: level,
                        mip_level_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            for pair in layer_views.windows(2) {
                bindings.push(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("GPU mip source"),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&pair[0]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                }));
                views.push(pair[1].clone());
            }
        }
        Ok(Self {
            views,
            bindings,
            pipeline,
        })
    }
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder) {
        for (binding, view) in self.bindings.iter().zip(self.views.iter()) {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("transmission mip"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, binding, &[]);
            pass.draw(0..3, 0..1);
        }
    }
    pub fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&Default::default());
        self.encode(&mut encoder);
        queue.submit([encoder.finish()]);
    }
}
