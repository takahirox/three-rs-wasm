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
    if count == 0 {
        return Err(Error::Invalid("TSL compute count"));
    }
    let mut compiler = Compiler::new(Stage::Compute, 0);
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
        "@group(0) @binding(0) var<uniform> params:array<vec4<f32>,16>;\n@group(0) @binding(1) var destination:texture_storage_2d<rgba8unorm,write>;\n{functions}\n@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) invocation:vec3<u32>) {{let tsl_index=invocation.x;if tsl_index>={count}u {{return;}}\n{}\ntextureStore(destination,vec2<i32>({coordinate}),{color});\n}}",
        compiler.body
    ))
}
