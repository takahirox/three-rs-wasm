//! WGSL mesh programs. Keep shader compilation explicit and fallible.
use crate::{
    Error, Result,
    compute::{BufferAccess, GpuBuffer},
    renderer::Renderer,
};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(1);
pub(crate) const DEFAULT_HOOKS: &str = "fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;} fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{return base;}";
#[derive(Debug)]
pub struct ShaderProgram {
    pub(crate) id: u64,
    pub(crate) module: wgpu::ShaderModule,
    pub(crate) layout: wgpu::BindGroupLayout,
    pub(crate) bindings: wgpu::BindGroup,
}
impl ShaderProgram {
    /// Provide `deform(position,normal,uv)->vec3<f32>` and
    /// `shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>`.
    /// `u.custom` holds 16 application vec4s. Optional group 1 buffers are
    /// consecutive uniform or read-only storage bindings, and may be written
    /// by a compute kernel between renders without a CPU readback.
    pub async fn new(renderer: &Renderer, wgsl: &str, buffers: &[&GpuBuffer]) -> Result<Self> {
        Self::with_textures(renderer, wgsl, buffers, &[]).await
    }
    /// Group 1 contains buffers followed by consecutive texture/sampler pairs.
    /// Views stay resident in the bind group, including render-to-texture outputs.
    pub async fn with_textures(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        let device = &renderer.device;
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let source = format!(
            "{}\n{}\n{wgsl}",
            include_str!("shaders/cube_uv.wgsl"),
            concat!(
                include_str!("shaders/deformation.wgsl"),
                "\n",
                include_str!("shaders/output.wgsl"),
                "\n",
                include_str!("shader.wgsl")
            )
        );
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("custom mesh shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("custom mesh buffers"),
            entries: &buffers
                .iter()
                .enumerate()
                .map(|(i, b)| wgpu::BindGroupLayoutEntry {
                    binding: i as u32,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: if matches!(b.access, BufferAccess::Uniform) {
                            wgpu::BufferBindingType::Uniform
                        } else {
                            wgpu::BufferBindingType::Storage { read_only: true }
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                })
                .chain(textures.iter().enumerate().flat_map(|(i, _)| {
                    let binding = (buffers.len() + i * 2) as u32;
                    [
                        wgpu::BindGroupLayoutEntry {
                            binding,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: binding + 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ]
                }))
                .collect::<Vec<_>>(),
        });
        let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &buffers
                .iter()
                .enumerate()
                .map(|(i, b)| wgpu::BindGroupEntry {
                    binding: i as u32,
                    resource: b.buffer.as_entire_binding(),
                })
                .chain(
                    textures
                        .iter()
                        .enumerate()
                        .flat_map(|(i, (view, sampler))| {
                            let binding = (buffers.len() + i * 2) as u32;
                            [
                                wgpu::BindGroupEntry {
                                    binding,
                                    resource: wgpu::BindingResource::TextureView(view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: binding + 1,
                                    resource: wgpu::BindingResource::Sampler(sampler),
                                },
                            ]
                        }),
                )
                .collect::<Vec<_>>(),
        });
        // Validate the group layout contract at creation, before render() caches
        // format-specific variants. This pipeline is deliberately not submitted.
        renderer.validate_shader_program(&module, &layout);
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            module,
            layout,
            bindings,
        })
    }
}
