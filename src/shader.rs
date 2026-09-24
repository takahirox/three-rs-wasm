//! WGSL mesh programs. Keep shader compilation explicit and fallible.
use crate::{
    Error, Result,
    compute::{BufferAccess, GpuBuffer},
    renderer::Renderer,
};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(1);
pub(crate) const DEFAULT_ENVIRONMENT: &str = "fn environment_sample(direction:vec3<f32>,roughness:f32)->vec3<f32>{return default_environment_sample(direction,roughness);}";
pub(crate) const DEFAULT_OUTPUT: &str =
    "fn transform_output(value:vec4<f32>)->vec4<f32>{return value;}";
pub(crate) const DEFAULT_HOOKS: &str = "fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;} fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{return base;}";
pub(crate) const DEFAULT_PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
pub(crate) const DEFAULT_SURFACE: &str =
    "fn transform_surface(surface:VertexOut,value:LitSurface)->LitSurface{return value;}";
/// Interpolation of the primary UV varying at triangle edges.
#[derive(Clone, Copy, Debug)]
pub enum UvInterpolation {
    Center,
    Centroid,
    Sample,
    FlatFirst,
    FlatEither,
}
impl UvInterpolation {
    fn wgsl(self) -> &'static str {
        match self {
            Self::Center => "perspective,center",
            Self::Centroid => "perspective,centroid",
            Self::Sample => "perspective,sample",
            Self::FlatFirst => "flat,first",
            Self::FlatEither => "flat,either",
        }
    }
}
#[derive(Clone, Debug)]
pub struct ShaderProgram {
    pub(crate) id: u64,
    pub(crate) outputs: u32,
    pub(crate) custom_environment: bool,
    pub(crate) viewport: u8,
    pub(crate) module: wgpu::ShaderModule,
    pub(crate) layout: wgpu::BindGroupLayout,
    pub(crate) bindings: wgpu::BindGroup,
    binding_counts: (usize, usize),
}
impl ShaderProgram {
    /// Replace resources after render-target resize without recompiling the
    /// shader or changing its pipeline identity. Types must match the original
    /// layout; the GPU validates resource types when creating the bind group.
    pub fn rebind(
        &mut self,
        renderer: &Renderer,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<()> {
        if self.binding_counts != (buffers.len(), textures.len()) {
            return Err(crate::Error::Invalid("shader binding count"));
        }
        let entries: Vec<_> = buffers
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
            .collect();
        self.bindings = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("rebound shader resources"),
                layout: &self.layout,
                entries: &entries,
            });
        Ok(())
    }
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
        Self::build(
            renderer,
            wgsl,
            buffers,
            textures,
            DEFAULT_OUTPUT,
            DEFAULT_PROJECTION,
            DEFAULT_SURFACE,
            &[],
            &[],
            None,
            None,
        )
        .await
    }
    /// Bind sampled textures with explicit dimensions, including 2D arrays.
    pub async fn with_texture_dimensions(
        renderer: &Renderer,
        source: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        dimensions: &[wgpu::TextureViewDimension],
        sample_types: &[wgpu::TextureSampleType],
    ) -> Result<Self> {
        if dimensions.len() != textures.len() || sample_types.len() != textures.len() {
            return Err(Error::Invalid("shader texture dimensions"));
        }
        Self::build(
            renderer,
            source,
            buffers,
            textures,
            DEFAULT_OUTPUT,
            DEFAULT_PROJECTION,
            DEFAULT_SURFACE,
            dimensions,
            sample_types,
            None,
            None,
        )
        .await
    }
    /// Change UV interpolation without altering positions or the sample count.
    pub async fn with_uv_interpolation(
        renderer: &Renderer,
        source: &str,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        interpolation: UvInterpolation,
    ) -> Result<Self> {
        Self::build(
            renderer,
            source,
            &[],
            textures,
            DEFAULT_OUTPUT,
            DEFAULT_PROJECTION,
            DEFAULT_SURFACE,
            &[],
            &[],
            None,
            Some(interpolation),
        )
        .await
    }
    /// Transform a lit material's linear output before tone mapping, without a
    /// fullscreen pass. Bind via MaterialProperties::vertex_program and use its
    /// vertex_uniforms for the shared u.custom slots.
    pub async fn with_output(renderer: &Renderer, output: &str) -> Result<Self> {
        Self::build(
            renderer,
            DEFAULT_HOOKS,
            &[],
            &[],
            output,
            DEFAULT_PROJECTION,
            DEFAULT_SURFACE,
            &[],
            &[],
            None,
            None,
        )
        .await
    }
    /// A custom vertex projection with explicitly typed (e.g. cube) texture views.
    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn with_projection_and_dimensions(
        renderer: &Renderer,
        wgsl: &str,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        dimensions: &[wgpu::TextureViewDimension],
        projection: &str,
    ) -> Result<Self> {
        let sample_types =
            vec![wgpu::TextureSampleType::Float { filterable: true }; dimensions.len()];
        if dimensions.len() != textures.len() {
            return Err(Error::Invalid("shader texture dimensions"));
        }
        Self::build(
            renderer,
            wgsl,
            &[],
            textures,
            DEFAULT_OUTPUT,
            projection,
            DEFAULT_SURFACE,
            dimensions,
            &sample_types,
            None,
            None,
        )
        .await
    }
    pub(crate) async fn with_projection(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        projection: &str,
    ) -> Result<Self> {
        Self::build(
            renderer,
            wgsl,
            buffers,
            textures,
            DEFAULT_OUTPUT,
            projection,
            DEFAULT_SURFACE,
            &[],
            &[],
            None,
            None,
        )
        .await
    }
    pub(crate) async fn with_surface(
        renderer: &Renderer,
        wgsl: &str,
        surface: &str,
        output: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        Self::build(
            renderer,
            wgsl,
            buffers,
            textures,
            output,
            DEFAULT_PROJECTION,
            surface,
            &[],
            &[],
            None,
            None,
        )
        .await
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn with_surface_texture_types(
        renderer: &Renderer,
        wgsl: &str,
        surface: &str,
        output: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        dimensions: &[wgpu::TextureViewDimension],
        sample_types: &[wgpu::TextureSampleType],
    ) -> Result<Self> {
        Self::build(
            renderer,
            wgsl,
            buffers,
            textures,
            output,
            DEFAULT_PROJECTION,
            surface,
            dimensions,
            sample_types,
            None,
            None,
        )
        .await
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn with_mrt(
        renderer: &Renderer,
        wgsl: &str,
        surface: &str,
        output: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        count: u32,
        mrt: &str,
        projection: &str,
    ) -> Result<Self> {
        if count == 0 || count > renderer.device.limits().max_color_attachments {
            return Err(Error::Invalid("MRT attachment count"));
        }
        Self::build(
            renderer,
            wgsl,
            buffers,
            textures,
            output,
            projection,
            surface,
            &[],
            &[],
            Some((count, mrt)),
            None,
        )
        .await
    }
    #[allow(clippy::too_many_arguments)]
    async fn build(
        renderer: &Renderer,
        wgsl: &str,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        output: &str,
        projection: &str,
        surface: &str,
        dimensions: &[wgpu::TextureViewDimension],
        sample_types: &[wgpu::TextureSampleType],
        mrt: Option<(u32, &str)>,
        interpolation: Option<UvInterpolation>,
    ) -> Result<Self> {
        let device = &renderer.device;
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let mut source = format!(
            "{}\n{}\n{wgsl}\n{output}\n{projection}\n{surface}",
            include_str!("shaders/cube_uv.wgsl"),
            concat!(
                include_str!("shaders/deformation.wgsl"),
                "\n",
                include_str!("shaders/output.wgsl"),
                "\n",
                include_str!("shader.wgsl")
            )
        );
        if projection.contains("fn project_motion(") {
            source.push_str("\nvar<private> tsl_motion_original_position:vec3<f32>;\n");
            source = source.replace(
                "tsl_vertex_index=vertex;",
                "tsl_vertex_index=vertex;tsl_motion_original_position=input_position;",
            );
            source = source.replace("struct VertexOut {", "struct VertexOut { @location(12) motion_current:vec4<f32>, @location(13) motion_previous:vec4<f32>,");
        }
        if wgsl.contains("fn transform_light_color(") {
            source=source.replacen("fn transform_light_color(surface:VertexOut,tsl_light_index:u32,tsl_light_color:vec3<f32>)->vec3<f32>{return tsl_light_color;}","",1);
        }
        if wgsl.contains("fn transform_vertex_normal(") {
            source = source.replacen(
                "fn transform_vertex_normal(normal:vec3<f32>)->vec3<f32>{return normal;}",
                "",
                1,
            );
        }
        if !wgsl.contains("fn environment_sample(") {
            source.push_str(DEFAULT_ENVIRONMENT);
        }
        if let Some(interpolation) = interpolation {
            source = source.replace(
                "@location(2) uv: vec2<f32>",
                &format!(
                    "@location(2) @interpolate({}) uv: vec2<f32>",
                    interpolation.wgsl()
                ),
            );
        }
        if let Some((_, code)) = mrt {
            source=source.replace("@fragment fn fs_main(in:VertexOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32>","fn color_main(in:VertexOut,front:bool)->vec4<f32>");
            source.push_str(code);
        }
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
                                sample_type: sample_types
                                    .get(i)
                                    .copied()
                                    .unwrap_or(wgpu::TextureSampleType::Float { filterable: true }),
                                view_dimension: dimensions
                                    .get(i)
                                    .copied()
                                    .unwrap_or(wgpu::TextureViewDimension::D2),
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: binding + 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Sampler(
                                if sample_types.get(i) == Some(&wgpu::TextureSampleType::Depth) {
                                    wgpu::SamplerBindingType::Comparison
                                } else if sample_types.get(i)
                                    == Some(&wgpu::TextureSampleType::Float { filterable: false })
                                {
                                    // Float32 targets are sampled with nearest (non-filtering) samplers.
                                    wgpu::SamplerBindingType::NonFiltering
                                } else {
                                    wgpu::SamplerBindingType::Filtering
                                },
                            ),
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
        let viewport = u8::from(wgsl.contains("viewport_read_color"))
            | (u8::from(wgsl.contains("viewport_read_depth")) << 1);
        renderer.validate_shader_program(
            &module,
            &layout,
            mrt.map_or(1, |(count, _)| count),
            viewport != 0,
        );
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            custom_environment: wgsl.contains("fn environment_sample("),
            viewport,
            outputs: mrt.map_or(1, |(count, _)| count),
            module,
            layout,
            bindings,
            binding_counts: (buffers.len(), textures.len()),
        })
    }
}
