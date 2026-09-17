use crate::{environment_gpu::GpuEnvironment, math::Matrix4, render_target::RenderTarget};

pub(crate) type PipelineCache = std::collections::HashMap<
    (wgpu::TextureFormat, u32, Option<wgpu::TextureFormat>),
    (
        wgpu::RenderPipeline,
        wgpu::BindGroupLayout,
        crate::draw_gpu::Slot,
    ),
>;
#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare(
    cache: &mut PipelineCache,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    target: &RenderTarget,
    environment: &GpuEnvironment,
    projection: Matrix4,
    camera: Matrix4,
    options: [f64; 4],
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let key = (
        target.options.format,
        target.options.samples.max(1),
        target.depth_format(),
    );
    let (pipeline, layout, slot) = cache.entry(key).or_insert_with(|| {
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
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        });
        let layout = pipeline.get_bind_group_layout(0);
        (pipeline, layout, Default::default())
    });
    let mut parameters = Vec::from(projection.inverse().as_mat4().to_cols_array());
    parameters.extend(camera.as_mat4().to_cols_array());
    parameters.extend([
        options[0] as f32,
        options[1] as f32,
        environment.max_mip,
        options[2] as f32,
        options[3] as f32,
        0.0,
        0.0,
        0.0,
    ]);
    let uniform = slot.uniform(device, queue, bytemuck::cast_slice(&parameters));
    let bind_group = slot.bindings(
        device,
        layout,
        &[
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
    );
    (pipeline.clone(), bind_group)
}
