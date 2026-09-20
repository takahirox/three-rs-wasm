use crate::{environment_gpu::GpuEnvironment, math::Matrix4, render_target::RenderTarget};

pub(crate) type PipelineCache = std::collections::HashMap<
    (
        Vec<wgpu::TextureFormat>,
        u32,
        Option<wgpu::TextureFormat>,
        Vec<crate::scene::BackgroundOutput>,
    ),
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
    options: [f64; 6],
    outputs: &[crate::scene::BackgroundOutput],
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let key = (
        target.color_formats(),
        target.options.samples.max(1),
        target.depth_format(),
        outputs.to_vec(),
    );
    let (pipeline, layout, slot) = cache.entry(key).or_insert_with(|| {
        let mut source = format!("{}\n{}\n{}", include_str!("shaders/cube_uv.wgsl"),
            include_str!("shaders/output.wgsl"), include_str!("shaders/background.wgsl"));
        if !outputs.is_empty() {
            source = source.replace("@fragment fn fs(in:Out)->@location(0) vec4<f32>",
                "fn color_main(in:Out)->vec4<f32>");
            let fields = (0..target.options.count)
                .map(|i| format!("@location({i}) attachment_{i}:vec4<f32>,"))
                .collect::<String>();
            source.push_str(&format!("\nstruct BackgroundMrt{{{fields}}}\n@fragment fn fs(in:Out)->BackgroundMrt{{let color=color_main(in);let view=u.inverse_projection*vec4(in.ndc,1.0,1.0);let normal=normalize(-view.xyz)*0.5+0.5;var result:BackgroundMrt;"));
            for i in 0..target.options.count {
                let value = match outputs.get(i as usize) {
                    Some(crate::scene::BackgroundOutput::Color) => "color",
                    Some(crate::scene::BackgroundOutput::NormalView) => "vec4(normal,1.0)",
                    _ => "vec4(0.0)",
                };
                source.push_str(&format!("result.attachment_{i}={value};"));
            }
            source.push_str("return result;}");
        }
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("HDR background"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
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
                targets: &target
                    .color_formats()
                    .into_iter()
                    .enumerate()
                    .map(|(i, format)| {
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: None,
                            write_mask: if i == 0 || !outputs.is_empty() {
                                wgpu::ColorWrites::ALL
                            } else {
                                wgpu::ColorWrites::empty()
                            },
                        })
                    })
                    .collect::<Vec<_>>(),
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
        options[4] as f32,
        options[5] as f32,
        f32::from(target.options.encode_srgb),
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
