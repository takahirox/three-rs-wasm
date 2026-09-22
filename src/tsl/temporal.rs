//! GPU temporal reprojection with r186 variance clipping and Halton jitter.
//! The caller renders linear RGBA16F beauty, unjittered NDC motion, and depth
//! into a single-sampled MRT. Geometry, including previous deformation, stays
//! on the GPU. This pass owns only its resolve/history images.
use crate::{Error, Result, camera::ViewOffset, math::Matrix4, renderer::*};

pub struct TemporalAA {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    uniform: wgpu::Buffer,
    bindings: Option<wgpu::BindGroup>,
    input: Option<[wgpu::Texture; 4]>,
    resolve: RenderTarget,
    history: RenderTarget,
    previous_world: Matrix4,
    previous_projection_inverse: Matrix4,
    initialized: bool,
    frame: u32,
    upscaling: bool,
    output_size: Option<[u32; 2]>,
    history_depth: wgpu::Texture,
    seed: Option<crate::postprocessing::Effect>,
}
impl TemporalAA {
    pub async fn new(r: &Renderer) -> Result<Self> {
        Self::create(r, false).await
    }
    /// TAAU resolves to a separately configured output resolution.
    pub async fn upscaling(r: &Renderer) -> Result<Self> {
        Self::create(r, true).await
    }
    /// Set the drawing-buffer resolution for a TAAU instance.
    pub fn set_output_size(&mut self, width: u32, height: u32) -> Result<()> {
        if !self.upscaling || width == 0 || height == 0 {
            return Err(Error::Invalid("TAAU output size"));
        }
        self.output_size = Some([width, height]);
        Ok(())
    }
    async fn create(r: &Renderer, upscaling: bool) -> Result<Self> {
        let d = &r.device;
        d.push_error_scope(wgpu::ErrorFilter::Validation);
        let mut entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];
        for binding in 1..=5 {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: if binding == 2 || binding == 5 {
                        wgpu::TextureSampleType::Depth
                    } else {
                        wgpu::TextureSampleType::Float { filterable: true }
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 6,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        let layout = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("temporal resolve inputs"),
            entries: &entries,
        });
        let shader = d.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TRAA"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "{}\n{}",
                    include_str!("temporal_common.wgsl"),
                    if upscaling {
                        include_str!("taau.wgsl")
                    } else {
                        include_str!("traa.wgsl")
                    }
                )
                .into(),
            ),
        });
        let pipeline = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TRAA resolve"),
            layout: Some(&d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            })),
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
                    format: wgpu::TextureFormat::Rgba16Float,
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
        if let Some(e) = d.pop_error_scope().await {
            return Err(Error::Gpu(e.to_string()));
        }
        let uniform = d.create_buffer(&wgpu::BufferDescriptor {
            label: Some("temporal camera matrices"),
            size: 176,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let make = |depth_buffer| {
            RenderTarget::with_options(
                d,
                1,
                1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer,
                    ..Default::default()
                },
            )
        };
        Ok(Self {
            pipeline,
            layout,
            uniform,
            sampler: d.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            bindings: None,
            input: None,
            resolve: make(false)?,
            history: make(false)?,
            history_depth: make_depth(d, 1, 1),
            upscaling,
            output_size: None,
            seed: if upscaling {
                Some(
                    super::effect(
                        r,
                        wgpu::TextureFormat::Rgba16Float,
                        &super::Texture::Input.sample(super::uv()),
                    )
                    .await?,
                )
            } else {
                None
            },
            previous_world: Matrix4::IDENTITY,
            previous_projection_inverse: Matrix4::IDENTITY,
            initialized: false,
            frame: 0,
        })
    }
    /// Apply before rendering beauty; retain the unjittered projection for motion.
    pub fn view_offset(&self, width: u32, height: u32) -> ViewOffset {
        fn halton(mut index: u32, base: u32) -> f64 {
            let mut fraction = 1.;
            let mut result = 0.;
            while index > 0 {
                fraction /= base as f64;
                result += fraction * (index % base) as f64;
                index /= base;
            }
            result
        }
        ViewOffset {
            full_width: width as f64,
            full_height: height as f64,
            width: width as f64,
            height: height as f64,
            offset_x: halton(self.frame % 32 + 1, 2) - 0.5,
            offset_y: halton(self.frame % 32 + 1, 3) - 0.5,
        }
    }
    pub fn output(&self) -> &RenderTarget {
        &self.resolve
    }
    /// Restart history after a camera cut. Resizing/replacing the MRT also resets it.
    pub fn reset(&mut self) {
        self.initialized = false;
    }
    /// Reproject a standard WebGPU depth buffer (no reversed/logarithmic depth).
    /// `projection` is the jittered matrix used to render the input.
    pub fn apply(
        &mut self,
        r: &Renderer,
        input: &RenderTarget,
        world: Matrix4,
        projection: Matrix4,
        near_far: [f64; 2],
        orthographic: bool,
    ) -> Result<()> {
        let depth = input
            .depth_texture()
            .ok_or(Error::Invalid("TRAA depth input"))?;
        if input.options().samples > 1
            || input.options().count != 2
            || input.options().depth != 1
            || input.texture.dimension() != wgpu::TextureDimension::D2
            || input.texture.format() != wgpu::TextureFormat::Rgba16Float
            || input.textures[1].format() != wgpu::TextureFormat::Rgba16Float
            || depth.format() != wgpu::TextureFormat::Depth32Float
            || !depth.usage().contains(wgpu::TextureUsages::COPY_SRC)
        {
            return Err(Error::Invalid(
                "TRAA requires single-sampled RGBA16F beauty/motion and copyable Depth32Float",
            ));
        }
        if !world.is_finite()
            || !projection.is_finite()
            || world.determinant() == 0.
            || projection.determinant() == 0.
            || !near_far.iter().all(|x| x.is_finite())
            || near_far[0] < 0.
            || near_far[1] <= near_far[0]
            || (!orthographic && near_far[0] == 0.)
        {
            return Err(Error::Invalid("TRAA camera"));
        }
        let d = &r.device;
        let output_size = if self.upscaling {
            self.output_size
                .ok_or(Error::Invalid("TAAU output size not set"))?
        } else {
            [input.width, input.height]
        };
        let resized = output_size != [self.history.width, self.history.height];
        let inputs = [
            input.texture.clone(),
            input.textures[1].clone(),
            depth.clone(),
            self.history_depth.clone(),
        ];
        if self.input.as_ref() != Some(&inputs) || resized {
            self.resolve.set_size(d, output_size[0], output_size[1])?;
            self.history.set_size(d, output_size[0], output_size[1])?;
            let depth_view = self.history_depth.create_view(&Default::default());
            let views = [
                &input.view,
                input.depth_view.as_ref().unwrap(),
                &input.views[1],
                &self.history.view,
                &depth_view,
            ];
            let mut entries = vec![wgpu::BindGroupEntry {
                binding: 0,
                resource: self.uniform.as_entire_binding(),
            }];
            for (i, view) in views.iter().enumerate() {
                entries.push(wgpu::BindGroupEntry {
                    binding: i as u32 + 1,
                    resource: wgpu::BindingResource::TextureView(view),
                });
            }
            entries.push(wgpu::BindGroupEntry {
                binding: 6,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            });
            self.bindings = Some(d.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("temporal history"),
                layout: &self.layout,
                entries: &entries,
            }));
            self.input = Some(inputs);
            if resized || !self.upscaling {
                self.initialized = false;
            }
        }
        let mut values = Vec::with_capacity(44);
        values.extend(
            (world.inverse() * self.previous_world)
                .to_cols_array()
                .map(|v| v as f32),
        );
        values.extend(
            self.previous_projection_inverse
                .to_cols_array()
                .map(|v| v as f32),
        );
        values.extend([
            near_far[0] as f32,
            near_far[1] as f32,
            if orthographic { 1. } else { 0. },
            0.,
        ]);
        values.extend([0.0005, 0.001, 128., 1.]);
        let jitter = self.view_offset(input.width, input.height);
        values.extend([jitter.offset_x as f32, jitter.offset_y as f32, 0., 0.]);
        r.queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&values));
        if !self.initialized
            && let Some(seed) = &self.seed
        {
            seed.apply(r, input, None, &self.history)?;
        }
        let mut encoder = d.create_command_encoder(&Default::default());
        let copy =
            |encoder: &mut wgpu::CommandEncoder, a: &wgpu::Texture, b: &wgpu::Texture, aspect| {
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: a,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: b,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect,
                    },
                    wgpu::Extent3d {
                        width: a.width(),
                        height: a.height(),
                        depth_or_array_layers: 1,
                    },
                );
            };
        if !self.initialized && !self.upscaling {
            copy(
                &mut encoder,
                &input.texture,
                &self.history.texture,
                wgpu::TextureAspect::All,
            );
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("TRAA resolve"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.resolve.view,
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
            pass.set_bind_group(0, self.bindings.as_ref().unwrap(), &[]);
            pass.draw(0..3, 0..1);
        }
        copy(
            &mut encoder,
            &self.resolve.texture,
            &self.history.texture,
            wgpu::TextureAspect::All,
        );
        // Reprojection reads the previous frame at its original input size.
        // Replace its storage only after resolve, then bind the new view next frame.
        let next_depth = (self.history_depth.width() != input.width
            || self.history_depth.height() != input.height)
            .then(|| make_depth(d, input.width, input.height));
        copy(
            &mut encoder,
            depth,
            next_depth.as_ref().unwrap_or(&self.history_depth),
            wgpu::TextureAspect::DepthOnly,
        );
        if let Some(depth) = next_depth {
            self.history_depth = depth;
        }
        r.queue.submit(Some(encoder.finish()));
        self.previous_world = world;
        self.previous_projection_inverse = projection.inverse();
        self.initialized = true;
        self.frame = (self.frame + 1) % 32;
        Ok(())
    }
}

fn make_depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("temporal previous depth"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
