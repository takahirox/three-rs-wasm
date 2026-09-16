use crate::{environment_gpu::GpuEnvironment, math::Matrix4, render_target::RenderTarget};
use wgpu::util::DeviceExt;
pub(crate) type PipelineCache = std::collections::HashMap<
    (wgpu::TextureFormat, u32, Option<wgpu::TextureFormat>),
    wgpu::RenderPipeline,
>;
pub(crate) fn prepare(
    cache: &mut PipelineCache,
    device: &wgpu::Device,
    target: &RenderTarget,
    environment: &GpuEnvironment,
    projection: Matrix4,
    camera: Matrix4,
    options: [f64; 2],
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let key = (
        target.options.format,
        target.options.samples.max(1),
        target.depth_format(),
    );
    let pipeline = cache
        .entry(key)
        .or_insert_with(|| {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("HDR background"),
                source: wgpu::ShaderSource::Wgsl(
                    format!(
                        "{}\n{}",
                        include_str!("shaders/cube_uv.wgsl"),
                        include_str!("shaders/background.wgsl")
                    )
                    .into(),
                ),
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("HDR background"),
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
                        format: target.options.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: target.depth_format().map(|format| wgpu::DepthStencilState {
                    format,
                    depth_write_enabled: false,
                    depth_compare: wgpu::CompareFunction::Always,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: target.options.samples.max(1),
                    ..Default::default()
                },
                multiview: None,
                cache: None,
            })
        })
        .clone();
    let mut parameters = Vec::from(projection.inverse().as_mat4().to_cols_array());
    parameters.extend(camera.as_mat4().to_cols_array());
    parameters.extend([
        options[0] as f32,
        options[1] as f32,
        environment.max_mip,
        0.0,
    ]);
    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("background camera"),
        contents: bytemuck::cast_slice(&parameters),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("background"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&environment.source),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&environment.view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&environment.sampler),
            },
        ],
    });
    (pipeline, bind_group)
}
