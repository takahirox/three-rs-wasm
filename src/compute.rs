//! Validated reusable WGSL compute kernels with persistent GPU buffers.
use crate::{Error, Result, renderer::Renderer};

#[derive(Clone, Copy, Debug)]
pub enum BufferAccess {
    Uniform,
    Read,
    ReadWrite,
}
pub struct GpuBuffer {
    pub buffer: wgpu::Buffer,
    pub access: BufferAccess,
}
impl GpuBuffer {
    pub fn new(renderer: &Renderer, bytes: &[u8], access: BufferAccess) -> Result<Self> {
        let out = Self::zeroed(renderer, bytes.len() as u64, access)?;
        out.write(renderer, 0, bytes)?;
        Ok(out)
    }
    /// Allocate resident GPU storage without allocating/uploading a CPU zero array.
    pub fn zeroed(renderer: &Renderer, size: u64, access: BufferAccess) -> Result<Self> {
        let uniform = matches!(access, BufferAccess::Uniform);
        let limits = renderer.device.limits();
        let limit = if uniform {
            limits.max_uniform_buffer_binding_size
        } else {
            limits.max_storage_buffer_binding_size
        } as u64;
        if size == 0 || !size.is_multiple_of(4) || size > limit {
            return Err(Error::Invalid("GPU buffer size/alignment"));
        }
        Ok(Self {
            access,
            buffer: renderer.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("application GPU buffer"),
                size,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC
                    | if uniform {
                        wgpu::BufferUsages::UNIFORM
                    } else {
                        wgpu::BufferUsages::STORAGE
                            | wgpu::BufferUsages::VERTEX
                            | wgpu::BufferUsages::INDEX
                            | wgpu::BufferUsages::INDIRECT
                    },
            }),
        })
    }
    pub fn write(&self, renderer: &Renderer, offset: u64, bytes: &[u8]) -> Result<()> {
        if !offset.is_multiple_of(4)
            || !bytes.len().is_multiple_of(4)
            || offset
                .checked_add(bytes.len() as u64)
                .is_none_or(|end| end > self.buffer.size())
        {
            return Err(Error::Invalid("GPU buffer update range"));
        }
        renderer.queue.write_buffer(&self.buffer, offset, bytes);
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read(&self, renderer: &Renderer) -> Result<Vec<u8>> {
        let staging = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("GPU readback"),
            size: self.buffer.size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.buffer, 0, &staging, 0, self.buffer.size());
        renderer.queue.submit([encoder.finish()]);
        let (send, recv) = std::sync::mpsc::channel();
        staging.slice(..).map_async(wgpu::MapMode::Read, move |r| {
            let _ = send.send(r);
        });
        renderer
            .device
            .poll(wgpu::PollType::Wait)
            .map_err(|e| Error::Gpu(e.to_string()))?;
        recv.recv()
            .map_err(|e| Error::Gpu(e.to_string()))?
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let bytes = staging.slice(..).get_mapped_range().to_vec();
        staging.unmap();
        Ok(bytes)
    }
}
pub struct ComputeKernel {
    pipeline: wgpu::ComputePipeline,
    bindings: wgpu::BindGroup,
}
impl ComputeKernel {
    /// WGSL uses group 0, consecutive bindings matching `buffers`, and `main`.
    /// Shader and binding errors are returned instead of invoking wgpu's panic handler.
    pub async fn new(renderer: &Renderer, wgsl: &str, buffers: &[&GpuBuffer]) -> Result<Self> {
        Self::with_storage_textures(renderer, wgsl, buffers, &[]).await
    }
    /// Storage textures follow the buffers in group 0; each is write-only 2D.
    pub async fn with_storage_textures(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, wgpu::TextureFormat)],
    ) -> Result<Self> {
        let textures: Vec<_> = textures
            .iter()
            .map(|(v, f)| (*v, *f, wgpu::StorageTextureAccess::WriteOnly))
            .collect();
        Self::with_texture_access(renderer, wgsl, buffers, &textures).await
    }
    /// Explicit storage access permits resident ping-pong textures without copies.
    pub async fn with_texture_access(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(
            &wgpu::TextureView,
            wgpu::TextureFormat,
            wgpu::StorageTextureAccess,
        )],
    ) -> Result<Self> {
        let layouts: Vec<_> = textures
            .iter()
            .map(|(v, f, a)| (*v, *f, *a, wgpu::TextureViewDimension::D2))
            .collect();
        Self::with_texture_dimensions(renderer, wgsl, buffers, &layouts).await
    }
    /// Explicit storage view dimensions, including 3D compute textures.
    pub async fn with_texture_dimensions(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(
            &wgpu::TextureView,
            wgpu::TextureFormat,
            wgpu::StorageTextureAccess,
            wgpu::TextureViewDimension,
        )],
    ) -> Result<Self> {
        let device = &renderer.device;
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let entries = buffers
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: match b.access {
                        BufferAccess::Uniform => wgpu::BufferBindingType::Uniform,
                        BufferAccess::Read => wgpu::BufferBindingType::Storage { read_only: true },
                        BufferAccess::ReadWrite => {
                            wgpu::BufferBindingType::Storage { read_only: false }
                        }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .chain(
                textures
                    .iter()
                    .enumerate()
                    .map(
                        |(i, (_, format, access, dimension))| wgpu::BindGroupLayoutEntry {
                            binding: (buffers.len() + i) as u32,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: *access,
                                format: *format,
                                view_dimension: *dimension,
                            },
                            count: None,
                        },
                    ),
            )
            .collect::<Vec<_>>();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute buffers"),
            entries: &entries,
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
                .chain(textures.iter().enumerate().map(|(i, (view, _, _, _))| {
                    wgpu::BindGroupEntry {
                        binding: (buffers.len() + i) as u32,
                        resource: wgpu::BindingResource::TextureView(view),
                    }
                }))
                .collect::<Vec<_>>(),
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("application compute"),
            source: wgpu::ShaderSource::Wgsl(wgsl.into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("application compute"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self { pipeline, bindings })
    }
    pub fn dispatch(&self, renderer: &Renderer, workgroups: [u32; 3]) -> Result<()> {
        if workgroups.iter().any(|&n| {
            n > renderer
                .device
                .limits()
                .max_compute_workgroups_per_dimension
        }) {
            return Err(Error::Invalid("compute workgroup count"));
        }
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bindings, &[]);
            pass.dispatch_workgroups(workgroups[0], workgroups[1], workgroups[2]);
        }
        renderer.queue.submit([encoder.finish()]);
        Ok(())
    }
}
