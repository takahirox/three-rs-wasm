use crate::{Error, Result, environment::EnvironmentMap};
use wgpu::util::DeviceExt;
pub(crate) type GpuEnvironment = crate::environment::PrefilteredEnvironment;
pub(crate) fn build(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &EnvironmentMap,
) -> Result<GpuEnvironment> {
    if let Some(gpu) = &image.gpu {
        return Ok(gpu.clone());
    }
    if image.width < 64
        || image.height == 0
        || image.rgba.len() != image.width as usize * image.height as usize * 4
        || image.height > device.limits().max_texture_dimension_2d
        || image.width > device.limits().max_texture_dimension_2d
    {
        return Err(Error::Invalid("HDR dimensions"));
    }

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("HDR equirectangular"),
        size: wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        texture.as_image_copy(),
        bytemuck::cast_slice(&image.rgba),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(image.width * 8),
            rows_per_image: Some(image.height),
        },
        texture.size(),
    );
    let source = texture.create_view(&Default::default());
    filter(device, queue, source, (image.width / 4).ilog2(), false, 0.0)
}
fn filter(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    source: wgpu::TextureView,
    max_mip: u32,
    captured: bool,
    sigma: f32,
) -> Result<GpuEnvironment> {
    let cube_size = 1 << max_mip;
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    let atlas = || {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("PMREM atlas"),
            size: wgpu::Extent3d {
                width: 3 * cube_size.max(112),
                height: 4 * cube_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            // COPY_SRC lets exporters read the atlas back, as readRenderTargetPixels does.
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    };
    let atlas = atlas();
    let temporary = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("PMREM scratch"),
        size: atlas.size(),
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[],
    });
    let view = atlas.create_view(&Default::default());
    let temporary_view = temporary.create_view(&Default::default());
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("r186 GGX PMREM"),
        source: wgpu::ShaderSource::Wgsl(
            format!(
                "{}\n{}",
                include_str!("shaders/cube_uv.wgsl"),
                include_str!("shaders/prefilter.wgsl")
            )
            .into(),
        ),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("PMREM"),
        layout: None,
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    let layout = pipeline.get_bind_group_layout(0);
    let levels = max_mip - 4 + 1 + 6;
    let mut encoder = device.create_command_encoder(&Default::default());
    let passes: Vec<(u32, u32)> = if captured {
        let mut p = vec![(0, 3)];
        if sigma > 0.0 {
            p.extend([(0, 4), (0, 5)]);
        }
        p.extend((1..levels).flat_map(|l| [(l, 1), (l, 2)]));
        p
    } else {
        (0..levels)
            .flat_map(|l| {
                if l == 0 {
                    vec![(0, 0)]
                } else {
                    vec![(l, 1), (l, 2)]
                }
            })
            .collect()
    };
    for (level, mode) in passes {
        let size = 1 << max_mip.saturating_sub(level).max(4);
        let target = level as f32 / (levels - 1) as f32;
        let previous = level.saturating_sub(1) as f32 / (levels - 1) as f32;
        let roughness = (target * target - previous * previous).sqrt() * target * 1.25;
        // WGSL vec3 pad starts at offset 32; reserve 48 bytes.
        let data = [
            size,
            level,
            max_mip,
            mode,
            if mode >= 4 {
                (sigma / std::f32::consts::SQRT_2).to_bits()
            } else {
                roughness.to_bits()
            },
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM parameters"),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("PMREM"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(match mode {
                        0 | 3 => &source,
                        4 => &view,
                        5 => &temporary_view,
                        1 => &view,
                        _ => &temporary_view,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(if mode == 1 || mode == 4 {
                        &temporary_view
                    } else {
                        &view
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: uniform.as_entire_binding(),
                },
            ],
        });
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindings, &[]);
            pass.dispatch_workgroups((size * 3).div_ceil(8), (size * 2).div_ceil(8), 1);
        }
    }
    queue.submit([encoder.finish()]);
    Ok(GpuEnvironment {
        texture: atlas,
        view,
        source,
        sampler,
        max_mip: max_mip as f32,
        source_is_cube_uv: captured,
    })
}

impl EnvironmentMap {
    /// Capture six GPU views and prefilter them for image-based lighting. CPU
    /// readback is never used. Call again only when the source scene changes.
    pub fn from_scene(
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        size: u32,
        sigma: f32,
    ) -> Result<Self> {
        use crate::{camera::*, math::*, renderer::*, scene::*};
        if size < 16
            || !size.is_power_of_two()
            || size > renderer.device.limits().max_texture_dimension_2d / 4
            || !sigma.is_finite()
            || sigma < 0.0
        {
            return Err(Error::Invalid("environment capture size/sigma"));
        }
        let mut target = RenderTarget::with_options(
            &renderer.device,
            3 * size.max(112),
            4 * size,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.0,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
            ..Default::default()
        })));
        let result = (|| {
            for (i, (direction, up)) in [
                (Vector3::X, Vector3::Y),
                (-Vector3::Y, Vector3::Z),
                (Vector3::Z, Vector3::Y),
                (-Vector3::X, Vector3::Y),
                (Vector3::Y, -Vector3::Z),
                (-Vector3::Z, Vector3::Y),
            ]
            .into_iter()
            .enumerate()
            {
                scene.get_mut(camera)?.up = up;
                scene.look_at(camera, direction)?;
                target.viewport = [(i as u32 % 3) * size, (i as u32 / 3) * size, size, size];
                target.scissor = Some(target.viewport);
                target.set_load_color(i > 0);
                renderer.render(scene, camera, &target)?;
            }
            filter(
                &renderer.device,
                &renderer.queue,
                target.view.clone(),
                size.ilog2(),
                true,
                sigma.min(std::f32::consts::PI),
            )
        })();
        scene.dispose(camera)?;
        Ok(Self {
            width: 3 * size.max(112),
            height: 4 * size,
            rgba: vec![],
            gpu: Some(result?),
        })
    }
}

impl EnvironmentMap {
    /// Prepare six linear HDR faces (+X,-X,+Y,-Y,+Z,-Z) entirely on the GPU.
    /// `rgba` stores all faces consecutively; no mip or filtering work runs on CPU.
    pub fn from_cube_hdr(
        renderer: &crate::renderer::Renderer,
        size: u32,
        rgba: &[half::f16],
    ) -> Result<Self> {
        let cube = crate::texture_gpu::GpuTexture::from_cube_hdr(renderer, size, rgba)?;
        Self::from_cube_texture(renderer, &cube)
    }
    /// Prefilter a resident HDR cube, sharing the original upload with a sky.
    pub fn from_cube_texture(
        renderer: &crate::renderer::Renderer,
        cube: &crate::texture_gpu::GpuTexture,
    ) -> Result<Self> {
        let size = cube.texture.width();
        if size < 16
            || !size.is_power_of_two()
            || size > renderer.device.limits().max_texture_dimension_2d / 4
            || cube.texture.height() != size
            || cube.texture.depth_or_array_layers() != 6
        {
            return Err(Error::Invalid("PMREM cube dimensions"));
        }
        Self::from_cube_view(renderer, size, &cube.view)
    }
    fn from_cube_view(
        renderer: &crate::renderer::Renderer,
        size: u32,
        cube: &wgpu::TextureView,
    ) -> Result<Self> {
        let device = &renderer.device;
        let queue = &renderer.queue;
        let atlas = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("cube PMREM source"),
                size: wgpu::Extent3d {
                    width: 3 * size.max(112),
                    height: 4 * size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba16Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cube to PMREM"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "{}\n{}",
                    include_str!("shaders/cube_uv.wgsl"),
                    include_str!("shaders/cube_to_pmrem.wgsl")
                )
                .into(),
            ),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("cube to PMREM"),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[size, 0, 0, 0, 0, 0, 0, 0]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(cube),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&atlas),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: params.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindings, &[]);
            pass.dispatch_workgroups((3 * size).div_ceil(8), (2 * size).div_ceil(8), 1);
        }
        queue.submit([encoder.finish()]);
        Ok(Self {
            width: 3 * size.max(112),
            height: 4 * size,
            rgba: vec![],
            gpu: Some(filter(device, queue, atlas, size.ilog2(), true, 0.0)?),
        })
    }
}

impl EnvironmentMap {
    /// Capture a CubeCamera at a world-space origin, then generate its PMREM.
    /// The six render passes, cube-UV padding and convolution stay on the GPU.
    pub fn from_cube_scene(
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        position: crate::math::Vector3,
        near: f64,
        far: f64,
        size: u32,
    ) -> Result<Self> {
        use crate::{camera::*, math::*, renderer::*, scene::*};
        if !position.is_finite()
            || !near.is_finite()
            || !far.is_finite()
            || near <= 0.0
            || far <= near
            || size < 16
            || !size.is_power_of_two()
            || size > renderer.device.limits().max_texture_dimension_2d / 4
        {
            return Err(Error::Invalid("cube capture camera/size"));
        }
        let mut target = RenderTarget::with_options(
            &renderer.device,
            size,
            size,
            RenderTargetOptions {
                depth: 6,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.0,
            aspect: 1.0,
            near,
            far,
            ..Default::default()
        })));
        scene.get_mut(camera)?.position = position;
        // CubeCamera uses negative FOV; reversing its up vectors is equivalent.
        let result = (|| {
            for (i, (direction, up)) in [
                (Vector3::NEG_X, Vector3::Y),
                (Vector3::X, Vector3::Y),
                (Vector3::Y, Vector3::NEG_Z),
                (Vector3::NEG_Y, Vector3::Z),
                (Vector3::Z, Vector3::Y),
                (Vector3::NEG_Z, Vector3::Y),
            ]
            .into_iter()
            .enumerate()
            {
                scene.get_mut(camera)?.up = up;
                scene.look_at(camera, position + direction)?;
                target.set_layer(i as u32)?;
                renderer.render(scene, camera, &target)?;
            }
            Self::from_cube_view(
                renderer,
                size,
                &target.texture.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::Cube),
                    ..Default::default()
                }),
            )
        })();
        scene.dispose(camera)?;
        result
    }
}
