//! Reusable fullscreen passes. Intermediate targets remain linear HDR.
use crate::{Error, Result, renderer::*};

pub struct Effect {
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    format: wgpu::TextureFormat,
    slot: std::cell::RefCell<crate::draw_gpu::Slot>,
    /// Sixteen vec4 parameters, available as `params` in WGSL.
    pub parameters: [[f32; 4]; 16],
}
impl Effect {
    /// Define `fn effect(uv:vec2<f32>)->vec4<f32>`. Bindings expose
    /// `input_texture`, `input_sampler`, `history_texture`, and `params`.
    pub async fn new(renderer: &Renderer, format: wgpu::TextureFormat, wgsl: &str) -> Result<Self> {
        let device = &renderer.device;
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("effect inputs"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
        let source = format!("{}\n{wgsl}", include_str!("shaders/effect.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fullscreen effect"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("fullscreen effect"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
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
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self {
            layout,
            slot: Default::default(),
            pipeline,
            format,
            sampler: device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            parameters: [[0.0; 4]; 16],
        })
    }
    pub fn apply(
        &self,
        renderer: &Renderer,
        input: &RenderTarget,
        history: Option<&RenderTarget>,
        output: &RenderTarget,
    ) -> Result<()> {
        let history = history.unwrap_or(input);
        if input.texture == output.texture || history.texture == output.texture {
            return Err(Error::Invalid("effect input/output alias"));
        }
        if output.options().format != self.format
            || output.options().samples > 1
            || [input, history, output]
                .iter()
                .any(|t| t.options().depth != 1 || t.dimension != wgpu::TextureDimension::D2)
        {
            return Err(Error::Invalid(
                "effect target format, samples or dimensions",
            ));
        }
        let device = &renderer.device;
        let mut slot = self.slot.borrow_mut();
        let uniform = slot.uniform(
            device,
            &renderer.queue,
            bytemuck::cast_slice(&self.parameters),
        );
        let bind = slot.bindings(
            device,
            &self.layout,
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&input.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&history.view),
                },
            ],
        );
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fullscreen effect"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind, &[]);
            pass.draw(0..3, 0..1);
        }
        renderer.queue.submit([encoder.finish()]);
        Ok(())
    }
}

/// Separable Gaussian blur pass. Direction and texel size are supplied at render time.
pub const GAUSSIAN_BLUR: &str = r#"
fn effect(uv:vec2<f32>)->vec4<f32> {
    let step=params[0].xy/vec2<f32>(textureDimensions(input_texture));
    var color=textureSample(input_texture,input_sampler,uv)*0.227027027;
    color+=(textureSample(input_texture,input_sampler,uv+step*1.384615385)+textureSample(input_texture,input_sampler,uv-step*1.384615385))*0.316216216;
    color+=(textureSample(input_texture,input_sampler,uv+step*3.230769231)+textureSample(input_texture,input_sampler,uv-step*3.230769231))*0.070270270;
    return color;
}"#;
/// Bright-pass extraction for a bloom chain. params[0].x is the linear threshold.
pub const BRIGHT_PASS: &str = r#"
fn effect(uv:vec2<f32>)->vec4<f32> {
    let color=textureSample(input_texture,input_sampler,uv);
    let luma=dot(color.rgb,vec3(0.2126,0.7152,0.0722));
    return vec4(color.rgb*smoothstep(params[0].x,params[0].x+0.01,luma),color.a);
}"#;
/// Add the blurred input to the original (history). params[0].x is bloom strength.
pub const BLOOM_COMPOSITE: &str = r#"
fn effect(uv:vec2<f32>)->vec4<f32> {
    let original=textureSample(history_texture,input_sampler,uv);
    return vec4(original.rgb+textureSample(input_texture,input_sampler,uv).rgb*params[0].x,original.a);
}"#;
