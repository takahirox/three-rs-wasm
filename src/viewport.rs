//! Resident framebuffer snapshots for material nodes. No CPU readback/re-render.
use crate::{Error, Result, render_target::RenderTarget};
pub(crate) struct Snapshot {
    source: wgpu::Texture,
    source_depth: Option<wgpu::Texture>,
    color: wgpu::Texture,
    pub color_view: wgpu::TextureView,
    pub depth_view: wgpu::TextureView,
    depth_pipeline: Option<wgpu::RenderPipeline>,
    depth_bindings: Option<wgpu::BindGroup>,
}
impl Snapshot {
    pub fn matches(&self, target: &RenderTarget) -> bool {
        self.source == target.texture && self.source_depth.as_ref() == target.depth_texture()
    }
    pub fn new(device: &wgpu::Device, target: &RenderTarget) -> Result<Self> {
        if target.dimension != wgpu::TextureDimension::D2
            || target.texture.depth_or_array_layers() != 1
            || (target.options.samples > 1
                && (!target.options.resolve_color_buffer
                    || !target.options.store_multisampled_color_buffer
                    || !target.options.store_multisampled_depth_buffer))
        {
            return Err(Error::Invalid(
                "viewport snapshots require retained 2D color/depth",
            ));
        }
        let texture = |format| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("viewport snapshot"),
                size: target.texture.size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
        };
        let color = texture(target.texture.format());
        let depth = texture(wgpu::TextureFormat::R32Float);
        let depth_view = depth.create_view(&Default::default());
        let (depth_pipeline, depth_bindings) = if let Some(source) = target.depth_texture() {
            let multisampled = source.sample_count() > 1;
            let ty = if multisampled {
                "texture_depth_multisampled_2d"
            } else {
                "texture_depth_2d"
            };
            let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("viewport depth resolve"),source:wgpu::ShaderSource::Wgsl(format!(r#"
@group(0) @binding(0) var depth:{ty};
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{{
 let p=array<vec2<f32>,3>(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);
}}
@fragment fn fs(@builtin(position) p:vec4<f32>)->@location(0) f32{{return textureLoad(depth,vec2<i32>(p.xy),0);}}
"#).into())});
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled,
                    },
                    count: None,
                }],
            });
            let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&source.create_view(
                        &wgpu::TextureViewDescriptor {
                            aspect: wgpu::TextureAspect::DepthOnly,
                            ..Default::default()
                        },
                    )),
                }],
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("viewport depth resolve"),
                layout: Some(
                    &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[&layout],
                        push_constant_ranges: &[],
                    }),
                ),
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
                        format: wgpu::TextureFormat::R32Float,
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
            (Some(pipeline), Some(bindings))
        } else {
            (None, None)
        };
        Ok(Self {
            source: target.texture.clone(),
            source_depth: target.depth_texture().cloned(),
            color_view: color.create_view(&Default::default()),
            color,
            depth_view,
            depth_pipeline,
            depth_bindings,
        })
    }
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder, flags: u8) {
        if flags & 1 != 0 {
            encoder.copy_texture_to_texture(
                self.source.as_image_copy(),
                self.color.as_image_copy(),
                self.source.size(),
            );
        }
        if flags & 2 != 0 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viewport depth snapshot"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.depth_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            if let (Some(pipeline), Some(bindings)) = (&self.depth_pipeline, &self.depth_bindings) {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, bindings, &[]);
                pass.draw(0..3, 0..1);
            }
        }
    }
}
