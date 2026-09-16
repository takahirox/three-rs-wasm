use crate::{Error, Result, environment::EnvironmentMap};
use wgpu::util::DeviceExt;
pub(crate) struct GpuEnvironment {
    pub view: wgpu::TextureView,
    pub source: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub max_mip: f32,
}
pub(crate) fn build(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &EnvironmentMap,
) -> Result<GpuEnvironment> {
    if image.width < 64
        || image.height == 0
        || image.rgba.len() != image.width as usize * image.height as usize * 4
        || image.height > device.limits().max_texture_dimension_2d
        || image.width > device.limits().max_texture_dimension_2d
    {
        return Err(Error::Invalid("HDR dimensions"));
    }
    let max_mip = (image.width / 4).ilog2();
    let cube_size = 1 << max_mip;
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
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
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
    for level in 0..levels {
        let size = 1 << max_mip.saturating_sub(level).max(4);
        let target = level as f32 / (levels - 1) as f32;
        let previous = level.saturating_sub(1) as f32 / (levels - 1) as f32;
        let roughness = (target * target - previous * previous).sqrt() * target * 1.25;
        for &mode in if level == 0 {
            &[0u32][..]
        } else {
            &[1u32, 2][..]
        } {
            // WGSL vec3 pad starts at offset 32; reserve 48 bytes.
            let data = [
                size,
                level,
                max_mip,
                mode,
                roughness.to_bits(),
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
                            0 => &source,
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
                        resource: wgpu::BindingResource::TextureView(if mode == 1 {
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
    }
    queue.submit([encoder.finish()]);
    Ok(GpuEnvironment {
        view,
        source,
        sampler,
        max_mip: max_mip as f32,
    })
}
