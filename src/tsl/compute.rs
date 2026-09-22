//! TSL textureStore graph compiled to a bounded one-dimensional GPU dispatch.
use super::*;
use crate::compute::{BufferAccess, ComputeKernel, GpuBuffer};

/// RGBA8 storage texture, matching Three.js StorageTexture's default format.
/// The kernel and bindings remain resident; no CPU pixel evaluation/readback.
pub struct TextureCompute {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    kernel: ComputeKernel,
    uniforms: GpuBuffer,
    groups: u32,
}
impl TextureCompute {
    pub async fn new(
        renderer: &Renderer,
        width: u32,
        height: u32,
        coordinate: Node,
        color: Node,
    ) -> Result<Self> {
        let limits = renderer.device.limits();
        let count = width
            .checked_mul(height)
            .ok_or(Error::Invalid("TSL compute texture size"))?;
        if width == 0
            || height == 0
            || width > limits.max_texture_dimension_2d
            || height > limits.max_texture_dimension_2d
            || count.div_ceil(64) > limits.max_compute_workgroups_per_dimension
        {
            return Err(Error::Invalid("TSL compute texture size"));
        }
        let source = texture_store_wgsl(count, &coordinate, &color)?;
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("TSL storage texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let uniforms = GpuBuffer::new(
            renderer,
            bytemuck::cast_slice(&[[0.0f32; 4]; 16]),
            BufferAccess::Uniform,
        )?;
        let kernel = ComputeKernel::with_storage_textures(
            renderer,
            &source,
            &[&uniforms],
            &[(&view, wgpu::TextureFormat::Rgba8Unorm)],
        )
        .await?;
        let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Ok(Self {
            texture,
            view,
            sampler,
            kernel,
            uniforms,
            groups: count.div_ceil(64),
        })
    }
    pub fn set_uniforms(&self, renderer: &Renderer, values: &[[f32; 4]; 16]) -> Result<()> {
        self.uniforms
            .write(renderer, 0, bytemuck::cast_slice(values))
    }
    pub fn dispatch(&self, renderer: &Renderer) -> Result<()> {
        self.kernel.dispatch(renderer, [self.groups, 1, 1])
    }
}
/// Compile a write-only textureStore. Threads beyond `count` return before
/// evaluating the graph. Out-of-range texels are ignored by WebGPU.
pub fn texture_store_wgsl(count: u32, coordinate: &Node, color: &Node) -> Result<String> {
    texture_kernel_wgsl(
        count,
        coordinate,
        color,
        wgpu::TextureFormat::Rgba8Unorm,
        &[],
    )
}
fn storage_format(format: wgpu::TextureFormat) -> Result<&'static str> {
    match format {
        wgpu::TextureFormat::Rgba8Unorm => Ok("rgba8unorm"),
        wgpu::TextureFormat::Rgba16Float => Ok("rgba16float"),
        wgpu::TextureFormat::Rgba32Float => Ok("rgba32float"),
        _ => Err(Error::Invalid("TSL storage texture format")),
    }
}
pub fn texture_kernel_wgsl(
    count: u32,
    coordinate: &Node,
    color: &Node,
    format: wgpu::TextureFormat,
    inputs: &[wgpu::TextureFormat],
) -> Result<String> {
    let format = storage_format(format)?;
    let mut declarations = String::new();
    for (i, f) in inputs.iter().enumerate() {
        declarations.push_str(&format!(
            "@group(0) @binding({}) var tsl_storage_{i}:texture_storage_2d<{},read>;\n",
            i + 2,
            storage_format(*f)?
        ));
    }
    if count == 0 {
        return Err(Error::Invalid("TSL compute count"));
    }
    let mut compiler = Compiler::new(Stage::Compute, inputs.len());
    let (ty, coordinate) = compiler.emit(coordinate)?;
    if ty != Type::UVec2 {
        return Err(Error::Invalid("TSL textureStore coordinate requires uvec2"));
    }
    let (ty, color) = compiler.emit(color)?;
    if ty != Type::Vec4 {
        return Err(Error::Invalid("TSL textureStore value requires vec4"));
    }
    let mut functions: Vec<_> = compiler.functions.values().collect();
    functions.sort();
    let functions = functions
        .into_iter()
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!(
        "@group(0) @binding(0) var<uniform> params:array<vec4<f32>,16>;\n@group(0) @binding(1) var destination:texture_storage_2d<{format},write>;\n{declarations}\n{functions}\n@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) invocation:vec3<u32>) {{let tsl_index=invocation.x;if tsl_index>={count}u {{return;}}\n{}\ntextureStore(destination,vec2<i32>({coordinate}),{color});\n}}",
        compiler.body
    ))
}

/// A simultaneous per-invocation update of typed storage elements. All right-hand
/// sides are evaluated before any store, so position/velocity updates share the
/// same snapshot. `new` requires independent invocation indices (no atomics);
/// `new_workgroup_snapshot` additionally synchronizes reads within one workgroup.
pub struct BufferStore {
    pub binding: usize,
    pub index: Node,
    pub value: Node,
}
pub struct BufferCompute {
    kernel: ComputeKernel,
    uniforms: GpuBuffer,
    groups: u32,
}
impl BufferCompute {
    pub async fn new(
        renderer: &Renderer,
        count: u32,
        buffers: &[(&GpuBuffer, Type)],
        stores: &[BufferStore],
    ) -> Result<Self> {
        Self::build(renderer, count, buffers, stores, false, &[]).await
    }
    /// Update persistent storage using sampled 2D attachments. Compute samples
    /// must specify an explicit mip level (typically zero for collision maps).
    pub async fn with_textures(
        renderer: &Renderer,
        count: u32,
        buffers: &[(&GpuBuffer, Type)],
        stores: &[BufferStore],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        Self::build(renderer, count, buffers, stores, false, textures).await
    }
    /// Simultaneous cross-invocation updates within a single workgroup (up to 64 elements).
    /// Storage reads finish before any stores, with a uniform storage barrier.
    pub async fn new_workgroup_snapshot(
        renderer: &Renderer,
        count: u32,
        buffers: &[(&GpuBuffer, Type)],
        stores: &[BufferStore],
    ) -> Result<Self> {
        if count > 64 {
            return Err(Error::Invalid("single workgroup snapshot count"));
        }
        Self::build(renderer, count, buffers, stores, true, &[]).await
    }
    async fn build(
        renderer: &Renderer,
        count: u32,
        buffers: &[(&GpuBuffer, Type)],
        stores: &[BufferStore],
        barrier: bool,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        if count == 0
            || count.div_ceil(64)
                > renderer
                    .device
                    .limits()
                    .max_compute_workgroups_per_dimension
            || buffers
                .iter()
                .any(|(b, _)| matches!(b.access, BufferAccess::Uniform))
        {
            return Err(Error::Invalid("TSL buffer compute count or binding"));
        }
        for s in stores {
            if buffers
                .get(s.binding)
                .is_none_or(|(b, _)| !matches!(b.access, BufferAccess::ReadWrite))
            {
                return Err(Error::Invalid("TSL store requires writable buffer"));
            }
        }
        let types: Vec<_> = buffers.iter().map(|(_, t)| *t).collect();
        let writable: Vec<_> = buffers
            .iter()
            .map(|(b, _)| matches!(b.access, BufferAccess::ReadWrite))
            .collect();
        let source =
            buffer_store_source(count, &types, &writable, stores, barrier, textures.len())?;
        let uniforms = GpuBuffer::zeroed(renderer, 256, BufferAccess::Uniform)?;
        let mut refs = vec![&uniforms];
        refs.extend(buffers.iter().map(|(b, _)| *b));
        let kernel =
            ComputeKernel::with_sampled_textures(renderer, &source, &refs, textures).await?;
        Ok(Self {
            kernel,
            uniforms,
            groups: count.div_ceil(64),
        })
    }
    pub fn set_uniforms(&self, renderer: &Renderer, values: &[[f32; 4]; 16]) -> Result<()> {
        self.uniforms
            .write(renderer, 0, bytemuck::cast_slice(values))
    }
    pub fn dispatch(&self, renderer: &Renderer) -> Result<()> {
        self.kernel.dispatch(renderer, [self.groups, 1, 1])
    }
}
pub fn buffer_store_wgsl(
    count: u32,
    types: &[Type],
    writable: &[bool],
    stores: &[BufferStore],
) -> Result<String> {
    buffer_store_source(count, types, writable, stores, false, 0)
}
fn buffer_store_source(
    count: u32,
    types: &[Type],
    writable: &[bool],
    stores: &[BufferStore],
    barrier: bool,
    textures: usize,
) -> Result<String> {
    validate_storage_types(types)?;
    if count == 0 || types.len() != writable.len() || stores.is_empty() {
        return Err(Error::Invalid("TSL buffer compute layout"));
    }
    let mut c = Compiler::new(Stage::Compute, textures);
    c.sampled_compute = true;
    c.buffers = types.to_vec();
    let mut writes = String::new();
    for s in stores {
        if !writable.get(s.binding).copied().unwrap_or(false) {
            return Err(Error::Invalid("TSL store binding"));
        }
        let (it, index) = c.emit(&s.index)?;
        let (vt, value) = c.emit(&s.value)?;
        if it != Type::Uint || vt != types[s.binding] {
            return Err(Error::Invalid("TSL store index/value type"));
        }
        writes.push_str(&format!("tsl_attribute_{}[{index}]={value};\n", s.binding));
    }
    let declarations = types
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "@group(0) @binding({}) var<storage,{}> tsl_attribute_{i}:array<{}>;",
                i + 1,
                if writable[i] { "read_write" } else { "read" },
                t.wgsl()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut functions: Vec<_> = c.functions.values().cloned().collect();
    functions.sort();
    let declarations = format!("{declarations}\n{}", texture_declarations(textures));
    let workgroup = if barrier { count } else { 64 };
    let guard = if barrier {
        String::new()
    } else {
        format!("if tsl_index>={count}u {{return;}}")
    };
    let barrier = if barrier { "storageBarrier();" } else { "" };
    Ok(format!(
        "@group(0) @binding(0) var<uniform> params:array<vec4<f32>,16>;\n{declarations}\n{}\n@compute @workgroup_size({workgroup}) fn main(@builtin(global_invocation_id) invocation:vec3<u32>) {{let tsl_index=invocation.x;{guard}\n{}\n{barrier}\n{writes}}}",
        functions.join("\n"),
        c.body
    ))
}

/// A reusable texture kernel bound to an existing destination and read-only inputs.
/// Bindings are created once; swapping kernels swaps ping/pong roles without copies.
pub struct TextureKernel {
    kernel: ComputeKernel,
    uniforms: GpuBuffer,
    groups: u32,
}
impl TextureKernel {
    pub async fn new(
        renderer: &Renderer,
        count: u32,
        coordinate: Node,
        color: Node,
        destination: (&wgpu::TextureView, wgpu::TextureFormat),
        inputs: &[(&wgpu::TextureView, wgpu::TextureFormat)],
    ) -> Result<Self> {
        if count == 0
            || count.div_ceil(64)
                > renderer
                    .device
                    .limits()
                    .max_compute_workgroups_per_dimension
        {
            return Err(Error::Invalid("TSL compute count"));
        }
        let formats: Vec<_> = inputs.iter().map(|(_, f)| *f).collect();
        let source = texture_kernel_wgsl(count, &coordinate, &color, destination.1, &formats)?;
        let uniforms = GpuBuffer::zeroed(renderer, 256, BufferAccess::Uniform)?;
        let mut textures = vec![(
            destination.0,
            destination.1,
            wgpu::StorageTextureAccess::WriteOnly,
        )];
        textures.extend(
            inputs
                .iter()
                .map(|(v, f)| (*v, *f, wgpu::StorageTextureAccess::ReadOnly)),
        );
        let kernel =
            ComputeKernel::with_texture_access(renderer, &source, &[&uniforms], &textures).await?;
        Ok(Self {
            kernel,
            uniforms,
            groups: count.div_ceil(64),
        })
    }
    pub fn set_uniforms(&self, renderer: &Renderer, values: &[[f32; 4]; 16]) -> Result<()> {
        self.uniforms
            .write(renderer, 0, bytemuck::cast_slice(values))
    }
    pub fn dispatch(&self, renderer: &Renderer) -> Result<()> {
        self.kernel.dispatch(renderer, [self.groups, 1, 1])
    }
}
