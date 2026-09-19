//! Reusable fullscreen passes. Intermediate targets remain linear HDR.
use crate::{Error, Result, renderer::*};

pub struct Effect {
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    format: wgpu::TextureFormat,
    slots: std::cell::RefCell<Vec<(wgpu::Texture, wgpu::Texture, crate::draw_gpu::Slot)>>,
    texture_layout: wgpu::BindGroupLayout,
    textures: wgpu::BindGroup,
    texture_count: usize,
    /// Sixteen vec4 parameters, available as `params` in WGSL.
    pub parameters: [[f32; 4]; 16],
}
impl Effect {
    /// Define `fn effect(uv:vec2<f32>)->vec4<f32>`. Bindings expose
    /// `input_texture`, `input_sampler`, `history_texture`, and `params`.
    pub async fn new(renderer: &Renderer, format: wgpu::TextureFormat, wgsl: &str) -> Result<Self> {
        Self::with_textures(renderer, format, wgsl, &[]).await
    }
    /// Extra filterable 2D texture/sampler pairs use consecutive bindings in group 1.
    pub async fn with_textures(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        Self::build(renderer, format, wgsl, textures, None).await
    }
    /// Fullscreen pass with an explicit blend state (for GPU accumulation).
    pub async fn with_blend(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        blend: wgpu::BlendState,
    ) -> Result<Self> {
        Self::build(renderer, format, wgsl, &[], Some(blend)).await
    }
    async fn build(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        blend: Option<wgpu::BlendState>,
    ) -> Result<Self> {
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
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("effect extra textures"),
            entries: &textures
                .iter()
                .enumerate()
                .flat_map(|(i, _)| {
                    [
                        wgpu::BindGroupLayoutEntry {
                            binding: (i * 2) as u32,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: (i * 2 + 1) as u32,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ]
                })
                .collect::<Vec<_>>(),
        });
        let texture_bindings = extra_bindings(device, &texture_layout, textures);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout, &texture_layout],
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
                    blend,
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
            slots: Default::default(),
            texture_layout,
            textures: texture_bindings,
            texture_count: textures.len(),
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
    /// Rebind views after resizing external render targets without recompiling.
    pub fn set_textures(
        &mut self,
        renderer: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<()> {
        if textures.len() != self.texture_count {
            return Err(Error::Invalid("effect texture count"));
        }
        self.textures = extra_bindings(&renderer.device, &self.texture_layout, textures);
        Ok(())
    }
    pub fn apply(
        &self,
        renderer: &Renderer,
        input: &RenderTarget,
        history: Option<&RenderTarget>,
        output: &RenderTarget,
    ) -> Result<()> {
        self.apply_with_load(renderer, input, history, output, false)
    }
    /// Retain destination pixels when accumulating additional samples.
    pub fn apply_with_load(
        &self,
        renderer: &Renderer,
        input: &RenderTarget,
        history: Option<&RenderTarget>,
        output: &RenderTarget,
        load: bool,
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
        // Keep both sides of a history ping-pong resident. Bound the cache when
        // resizing or when an application replaces its source textures.
        let mut slots = self.slots.borrow_mut();
        let index = if let Some(i) = slots
            .iter()
            .position(|(a, b, _)| a == &input.texture && b == &history.texture)
        {
            i
        } else {
            if slots.len() == 2 {
                slots.remove(0);
            }
            slots.push((
                input.texture.clone(),
                history.texture.clone(),
                Default::default(),
            ));
            slots.len() - 1
        };
        let slot = &mut slots[index].2;
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
                        load: if load {
                            wgpu::LoadOp::Load
                        } else {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind, &[]);
            pass.set_bind_group(1, &self.textures, &[]);
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

fn extra_bindings(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("effect extra textures"),
        layout,
        entries: &textures
            .iter()
            .enumerate()
            .flat_map(|(i, (view, sampler))| {
                [
                    wgpu::BindGroupEntry {
                        binding: (i * 2) as u32,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: (i * 2 + 1) as u32,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ]
            })
            .collect::<Vec<_>>(),
    })
}

pub mod ssaa;
