//! Reusable fullscreen passes. Intermediate targets remain linear HDR.
use crate::{Error, Result, renderer::*};

pub struct Effect {
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    /// The same pass for an 8-bit RGBA target in another format: a canvas's
    /// preferred channel order ( BGRA ) or sRGB view, or a canvas where the
    /// effect was made for half-float.
    /// Keyed by format and whether the pass applies the output transform.
    variants: std::cell::RefCell<Vec<(wgpu::TextureFormat, bool, wgpu::RenderPipeline)>>,
    output_layout: std::cell::OnceCell<wgpu::PipelineLayout>,
    shader: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    blend: Option<wgpu::BlendState>,
    sampler: wgpu::Sampler,
    format: wgpu::TextureFormat,
    slots: std::cell::RefCell<Vec<(wgpu::Texture, wgpu::Texture, crate::draw_gpu::Slot)>>,
    texture_layout: wgpu::BindGroupLayout,
    textures: wgpu::BindGroup,
    texture_count: usize,
    has_depth: bool,
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
        Self::build(renderer, format, wgsl, textures, None, None, None).await
    }
    /// Fullscreen pass with an explicit blend state (for GPU accumulation).
    pub async fn with_blend(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        blend: wgpu::BlendState,
    ) -> Result<Self> {
        Self::build(renderer, format, wgsl, &[], Some(blend), None, None).await
    }
    /// Fullscreen pass with an explicit blend state and extra texture/sampler pairs.
    pub async fn with_blend_and_textures(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        blend: wgpu::BlendState,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        Self::build(renderer, format, wgsl, textures, Some(blend), None, None).await
    }
    /// Fullscreen effect with a single-sampled depth view at group 1 binding 0.
    /// Use textureLoad for exact depth reads; no depth copy or CPU readback.
    pub async fn with_depth(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        depth: &wgpu::TextureView,
    ) -> Result<Self> {
        Self::build(
            renderer,
            format,
            wgsl,
            &[],
            None,
            Some((depth, false)),
            None,
        )
        .await
    }
    /// Bind filterable color/normal inputs followed by a raw depth attachment.
    /// Depth occupies group 1 binding `2 * textures.len()`.
    pub async fn with_depth_and_textures(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        depth: &wgpu::TextureView,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        Self::build(
            renderer,
            format,
            wgsl,
            textures,
            None,
            Some((depth, false)),
            None,
        )
        .await
    }
    pub fn set_depth_and_textures(
        &mut self,
        renderer: &Renderer,
        depth: &wgpu::TextureView,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<()> {
        if !self.has_depth || textures.len() != self.texture_count {
            return Err(Error::Invalid("effect depth/texture bindings"));
        }
        self.textures = extra_bindings(
            &renderer.device,
            &self.texture_layout,
            textures,
            Some(depth),
        );
        Ok(())
    }
    /// Sample an MSAA depth attachment directly on the GPU (sample index is chosen by WGSL).
    pub async fn with_multisampled_depth(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        depth: &wgpu::TextureView,
    ) -> Result<Self> {
        Self::build(renderer, format, wgsl, &[], None, Some((depth, true)), None).await
    }
    pub fn set_depth(&mut self, renderer: &Renderer, depth: &wgpu::TextureView) -> Result<()> {
        if !self.has_depth || self.texture_count != 0 {
            return Err(Error::Invalid("effect has no depth binding"));
        }
        self.textures = extra_bindings(&renderer.device, &self.texture_layout, &[], Some(depth));
        Ok(())
    }
    /// Explicit texture view dimensions for native volume/cube effect inputs.
    pub async fn with_texture_dimensions(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        textures: &[(
            &wgpu::TextureView,
            &wgpu::Sampler,
            wgpu::TextureViewDimension,
        )],
    ) -> Result<Self> {
        let pairs = textures
            .iter()
            .map(|(v, s, _)| (*v, *s))
            .collect::<Vec<_>>();
        let dimensions = textures.iter().map(|(_, _, d)| *d).collect::<Vec<_>>();
        Self::build(
            renderer,
            format,
            wgsl,
            &pairs,
            None,
            None,
            Some(&dimensions),
        )
        .await
    }
    #[allow(clippy::too_many_arguments)]
    async fn build(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        wgsl: &str,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        blend: Option<wgpu::BlendState>,
        depth: Option<(&wgpu::TextureView, bool)>,
        dimensions: Option<&[wgpu::TextureViewDimension]>,
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
        let source = format!(
            "{}\n{wgsl}\n{}",
            include_str!("shaders/effect.wgsl"),
            fused_output()
        );
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
                                view_dimension: dimensions
                                    .map_or(wgpu::TextureViewDimension::D2, |d| d[i]),
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
                .chain(
                    depth
                        .iter()
                        .map(|(_, multisampled)| wgpu::BindGroupLayoutEntry {
                            binding: (textures.len() * 2) as u32,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Depth,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: *multisampled,
                            },
                            count: None,
                        }),
                )
                .collect::<Vec<_>>(),
        });
        let texture_bindings = extra_bindings(
            device,
            &texture_layout,
            textures,
            depth.map(|(view, _)| view),
        );
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout, &texture_layout],
            push_constant_ranges: &[],
        });
        let pipeline = effect_pipeline(device, &pipeline_layout, &shader, format, blend, "fs_main");
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self {
            layout,
            slots: Default::default(),
            texture_layout,
            textures: texture_bindings,
            texture_count: textures.len(),
            has_depth: depth.is_some(),
            pipeline,
            variants: Default::default(),
            output_layout: Default::default(),
            shader,
            pipeline_layout,
            blend,
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
        if self.has_depth || textures.len() != self.texture_count {
            return Err(Error::Invalid("effect texture count"));
        }
        self.textures = extra_bindings(&renderer.device, &self.texture_layout, textures, None);
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
        let format = output.options().format;
        // A pass made for another 8-bit order or sRGB view renders into this
        // one; a display pass made for half-float renders into a canvas's.
        let variant =
            format != self.format && rgba8(format) && (rgba8(self.format) || output.canvas);
        // A half-float chain's last pass into an encoded target applies the
        // output transform itself, as RenderPipeline's renderOutput does.
        let fused = variant && !rgba8(self.format) && output.options().encode_srgb;
        if format != self.format && !variant
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
            let mut variants = self.variants.borrow_mut();
            pass.set_pipeline(if variant {
                let index = match variants
                    .iter()
                    .position(|(f, o, _)| *f == format && *o == fused)
                {
                    Some(index) => index,
                    None => {
                        let pipeline = if fused {
                            let layout = self.output_layout.get_or_init(|| {
                                renderer.device.create_pipeline_layout(
                                    &wgpu::PipelineLayoutDescriptor {
                                        label: Some("fused output effect"),
                                        bind_group_layouts: &[
                                            &self.layout,
                                            &self.texture_layout,
                                            &renderer.output_binding().layout,
                                        ],
                                        push_constant_ranges: &[],
                                    },
                                )
                            });
                            effect_pipeline(
                                &renderer.device,
                                layout,
                                &self.shader,
                                format,
                                self.blend,
                                "fs_output",
                            )
                        } else {
                            effect_pipeline(
                                &renderer.device,
                                &self.pipeline_layout,
                                &self.shader,
                                format,
                                self.blend,
                                "fs_main",
                            )
                        };
                        variants.push((format, fused, pipeline));
                        variants.len() - 1
                    }
                };
                &variants[index].2
            } else {
                &self.pipeline
            });
            if fused {
                pass.set_bind_group(2, &renderer.output_binding().group, &[]);
            }
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
    depth: Option<&wgpu::TextureView>,
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
            .chain(depth.into_iter().map(|view| wgpu::BindGroupEntry {
                binding: (textures.len() * 2) as u32,
                resource: wgpu::BindingResource::TextureView(view),
            }))
            .collect::<Vec<_>>(),
    })
}

pub mod ssaa;

pub mod afterimage;

fn rgba8(format: wgpu::TextureFormat) -> bool {
    matches!(
        format,
        wgpu::TextureFormat::Rgba8Unorm
            | wgpu::TextureFormat::Bgra8Unorm
            | wgpu::TextureFormat::Rgba8UnormSrgb
            | wgpu::TextureFormat::Bgra8UnormSrgb
    )
}
fn effect_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    fragment: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("fullscreen effect"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fragment),
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
    })
}
/// The output transform for a fused last pass: output.wgsl under its own
/// names, since effects may include output.wgsl themselves.
fn fused_output() -> String {
    let mut wgsl = include_str!("shaders/output.wgsl").to_string();
    for name in [
        "aces_output",
        "srgb_output",
        "srgb_input",
        "agx_output",
        "tone_output",
    ] {
        wgsl = wgsl.replace(name, &format!("fused_{name}"));
    }
    wgsl + "@group(2) @binding(0) var<uniform> effect_output:vec4<f32>;\n\
        @fragment fn fs_output(in:Out)->@location(0) vec4<f32> {\
        let value=effect(in.uv);\
        return vec4(fused_srgb_output(fused_tone_output(value.rgb,effect_output.x,effect_output.y)),value.a);}\n"
}
