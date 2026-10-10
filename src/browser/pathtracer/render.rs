//! WebGLPathTracer.renderSample on the GPU: the blurred environment
//! ( PMREMGenerator.fromEquirectangular's top level, BlurredEnvMapGenerator's
//! copy at blur 0 and its read-back half floats ), the data textures, the
//! tiled PhysicalPathTracingMaterial passes accumulated with NormalBlending,
//! and ClampedInterpolationMaterial's quad.
use super::gpu::{self, Resource};
use super::sampling::{Random, StratifiedSamples};
use super::textures;
use crate::browser::lights_projector::pack;
use crate::{Error, Result, renderer::Renderer};
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

const PATH_TRACING: &str = include_str!("path_tracing.wgsl");
/// FEATURE_BACKGROUND_MAP 0: the material without a backgroundMap.
const PATH_TRACING_NO_BACKGROUND: &str = include_str!("path_tracing_no_background.wgsl");
const TO_CUBE_UV: &str = include_str!("equirect_to_cube_uv.wgsl");
const PMREM_COPY: &str = include_str!("pmrem_copy.wgsl");
const OUTPUT: &str = include_str!("clamped_interpolation.wgsl");
const OUTPUT_LINEAR: &str = include_str!("clamped_interpolation_linear.wgsl");
const ACCUMULATION: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
pub(crate) const MIN_SAMPLES: f64 = 3.;
const RENDER_DELAY: f64 = 100.;
const FADE_DURATION: f64 = 500.;

/// NormalBlending into the float accumulation, which WebGPU cannot blend:
/// the same products in a pass over the tile.
const BLEND: &str = "
@group(0) @binding(0) var source: texture_2d<f32>;
@group(0) @binding(1) var destination: texture_2d<f32>;
@fragment fn main(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    let c = vec2<i32>(p.xy);
    let s = textureLoad(source, c, 0);
    let d = textureLoad(destination, c, 0);
    return vec4<f32>(s.rgb * s.a + d.rgb * (1.0 - s.a), s.a + d.a * (1.0 - s.a));
}";

/// BlendMaterial: the alpha mode's weighted blend of the accumulation and
/// the latest sample ( texel centres, so texture2D at vUv is textureLoad ).
const ALPHA_BLEND: &str = "
@group(0) @binding(0) var target1: texture_2d<f32>;
@group(0) @binding(1) var target2: texture_2d<f32>;
@group(0) @binding(2) var<uniform> opacity: vec4<f32>;
@fragment fn main(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    let c = vec2<i32>(p.xy);
    let color1 = textureLoad(target1, c, 0);
    let color2 = textureLoad(target2, c, 0);
    let invOpacity = 1.0 - opacity.x;
    let totalAlpha = color1.a * invOpacity + color2.a * opacity.x;
    if (color1.a != 0.0 || color2.a != 0.0) {
        return vec4<f32>(color1.rgb * (invOpacity * color1.a / totalAlpha) + color2.rgb * (opacity.x * color2.a / totalAlpha), totalAlpha);
    }
    return vec4<f32>(0.0);
}";

/// The half-float environment the path tracer samples and its tables.
pub(crate) struct Environment {
    map: wgpu::Texture,
    marginal: wgpu::Texture,
    conditional: wgpu::Texture,
    pub total: f64,
}

/// A pending read-back of the blurred environment ( RGBA32F rows ).
pub(crate) struct Readback {
    buffer: wgpu::Buffer,
    width: u32,
    height: u32,
    ready: Arc<Mutex<Option<std::result::Result<(), wgpu::BufferAsyncError>>>>,
}

/// _createPlanes( lodMax )'s first mesh: the six face quads of the cube-UV
/// top level with their output directions.
fn planes(lod_max: u32) -> Vec<f32> {
    let size = 2f64.powi(lod_max as i32);
    let texel = 1.0 / (size - 2.);
    let (min, max) = (-texel, 1. + texel);
    let uv1 = [min, min, max, min, max, max, min, min, max, max, min, max];
    let mut out = vec![];
    for face in 0..6 {
        let x = (face % 3) as f64 * 2. / 3. - 1.;
        let y = if face > 2 { 0. } else { -1. };
        let coordinates = [
            [x, y],
            [x + 2. / 3., y],
            [x + 2. / 3., y + 1.],
            [x, y],
            [x + 2. / 3., y + 1.],
            [x, y + 1.],
        ];
        for vertex in 0..6 {
            let u = uv1[vertex * 2] * 2. - 1.;
            let v = uv1[vertex * 2 + 1] * 2. - 1.;
            let d = match face {
                0 => [1., v, u],
                1 => [-u, 1., -v],
                2 => [-u, v, 1.],
                3 => [-1., v, -u],
                4 => [-u, -1., v],
                _ => [u, v, -1.],
            };
            out.extend([
                coordinates[vertex][0] as f32,
                coordinates[vertex][1] as f32,
                0.,
            ]);
            out.extend(d.map(|c| c as f32));
        }
    }
    out
}

/// The built-in uniforms three adds to a ShaderMaterial, then the given ones.
fn uniforms(source: &str, values: &[(&str, Vec<f64>)]) -> Result<Vec<u8>> {
    let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
    pack(source, "Uniforms", &values)
}

/// The UltraHDR texture ( flipY, its generated mips ), the cube-UV top level
/// and the equirect copy, read back.
pub(crate) fn blur_environment(
    r: &Renderer,
    (width, height, data): &(usize, usize, Vec<u16>),
) -> Result<Readback> {
    let (w, h) = (*width as u32, *height as u32);
    let levels = w.max(h).ilog2() + 1;
    let source = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("UltraHDR equirect"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    // flipY: the last image row is texture row 0.
    let row = *width * 4;
    let mut flipped = Vec::with_capacity(data.len());
    for y in (0..*height).rev() {
        flipped.extend_from_slice(&data[y * row..(y + 1) * row]);
    }
    r.queue.write_texture(
        source.as_image_copy(),
        bytemuck::cast_slice(&flipped),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(w * 8),
            rows_per_image: Some(h),
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    let mut encoder = r.device.create_command_encoder(&Default::default());
    generate_mipmaps(r, &mut encoder, &source, levels);
    // PMREMGenerator: cube size width / 4, its top level faces into the
    // 3 × size by 2 × size viewport of the 3 · size × 4 · size atlas.
    let lod_max = (w as f64 / 4.).log2().floor() as u32;
    let cube = 1u32 << lod_max;
    let atlas = gpu::target(
        r,
        "cube UV",
        (3 * cube.max(16 * 7), 4 * cube),
        wgpu::TextureFormat::Rgba16Float,
    );
    {
        let view = source.create_view(&Default::default());
        let resources = |name: &str| {
            (name == "envMap").then_some(Resource {
                view: &view,
                filter: true,
                repeat: (false, false),
                mipmaps: true,
            })
        };
        let buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("cube UV uniforms"),
                contents: &uniforms(TO_CUBE_UV, &[])?,
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let (layout, group) = gpu::bind_group(r, TO_CUBE_UV, Some(&buffer), &resources)?;
        let vs = gpu::module(
            r,
            "cube UV planes",
            "struct O { @location(0) d: vec3<f32>, @builtin(position) p: vec4<f32> }
@vertex fn main(@location(0) position: vec3<f32>, @location(1) direction: vec3<f32>) -> O {
    return O(direction, vec4<f32>(position.x, -position.y, position.z, 1.0));
}",
        );
        let fs = gpu::module(r, "EquirectangularToCubeUV", TO_CUBE_UV);
        let attributes = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
        let buffers = [wgpu::VertexBufferLayout {
            array_stride: 24,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }];
        let pipeline = gpu::pipeline(
            r,
            "EquirectangularToCubeUV",
            &vs,
            &fs,
            &layout,
            &buffers,
            wgpu::TextureFormat::Rgba16Float,
            None,
        );
        let vertices = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("cube UV planes"),
                contents: bytemuck::cast_slice(&planes(lod_max)),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let target = atlas.create_view(&Default::default());
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("cube UV"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_viewport(0., 0., 3. * cube as f32, 2. * cube as f32, 0., 1.);
        pass.set_scissor_rect(0, 0, 3 * cube, 2 * cube);
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..36, 0..1);
    }
    // BlurredEnvMapGenerator: textureCubeUV( envMap, equirectUvToDirection( vUv ), 0 )
    // into a float target of the source's size.
    let copy = gpu::target(
        r,
        "blurred environment",
        (w, h),
        wgpu::TextureFormat::Rgba32Float,
    );
    {
        let view = atlas.create_view(&Default::default());
        let resources = |name: &str| {
            (name == "envMap").then_some(Resource {
                view: &view,
                filter: true,
                repeat: (false, false),
                mipmaps: false,
            })
        };
        let buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("PMREM copy uniforms"),
                contents: &uniforms(PMREM_COPY, &[("blur", vec![0.])])?,
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let (layout, group) = gpu::bind_group(r, PMREM_COPY, Some(&buffer), &resources)?;
        let vs = gpu::module(r, "PMREM copy quad", &gpu::fullscreen_vs(PMREM_COPY, true));
        let fs = gpu::module(r, "PMREMCopyMaterial", PMREM_COPY);
        let pipeline = gpu::pipeline(
            r,
            "PMREMCopyMaterial",
            &vs,
            &fs,
            &layout,
            &[],
            wgpu::TextureFormat::Rgba32Float,
            None,
        );
        let target = copy.create_view(&Default::default());
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("PMREM copy"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
    let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("blurred environment read-back"),
        size: (w * 16 * h) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        copy.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 16),
                rows_per_image: Some(h),
            },
        },
        copy.size(),
    );
    r.queue.submit([encoder.finish()]);
    let ready = Arc::new(Mutex::new(None));
    let signal = ready.clone();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            *signal.lock().unwrap() = Some(result);
        });
    Ok(Readback {
        buffer,
        width: w,
        height: h,
        ready,
    })
}

/// WebGL's generateMipmap: each level the box average of the previous one.
fn generate_mipmaps(
    r: &Renderer,
    encoder: &mut wgpu::CommandEncoder,
    texture: &wgpu::Texture,
    levels: u32,
) {
    let module = gpu::module(
        r,
        "mipmap",
        "@group(0) @binding(0) var t: texture_2d<f32>;
struct O { @builtin(position) p: vec4<f32> }
@vertex fn vs(@builtin(vertex_index) i: u32) -> O {
    var p = array<vec2<f32>, 3>(vec2<f32>(-1.0, 3.0), vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0));
    return O(vec4<f32>(p[i], 0.0, 1.0));
}
@fragment fn fs(o: O) -> @location(0) vec4<f32> {
    let c = vec2<i32>(o.p.xy) * 2;
    let s = vec2<i32>(textureDimensions(t, 0)) - 1;
    let a = textureLoad(t, min(c, s), 0);
    let b = textureLoad(t, min(c + vec2<i32>(1, 0), s), 0);
    let d = textureLoad(t, min(c + vec2<i32>(0, 1), s), 0);
    let e = textureLoad(t, min(c + vec2<i32>(1, 1), s), 0);
    return (a + b + d + e) * 0.25;
}",
    );
    let layout = r
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mipmap"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
    let pipeline_layout = r
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mipmap"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
    let pipeline = r
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mipmap"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(texture.format().into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });
    for level in 1..levels {
        let view = |l: u32| {
            texture.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: l,
                mip_level_count: Some(1),
                ..Default::default()
            })
        };
        let (from, to) = (view(level - 1), view(level));
        let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mipmap"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&from),
            }],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mipmap"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &to,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
}

impl Readback {
    /// The read-back floats as DataUtils.toHalfFloat stores them, once mapped.
    pub fn take(&self) -> Option<Result<Vec<u16>>> {
        let result = self.ready.lock().unwrap().take()?;
        if result.is_err() {
            return Some(Err(Error::Invalid("blurred environment read-back")));
        }
        let mapped = self.buffer.slice(..).get_mapped_range();
        let floats: &[f32] = bytemuck::cast_slice(&mapped);
        let halfs = floats
            .iter()
            .map(|&v| textures::to_half(v as f64))
            .collect();
        drop(mapped);
        self.buffer.unmap();
        Some(Ok(halfs))
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// EquirectHdrInfoUniform.updateFrom over the blurred halfs.
pub(crate) fn environment(r: &Renderer, (w, h): (u32, u32), halfs: &[u16]) -> Environment {
    let (marginal, conditional, total) = textures::equirect_info(w as usize, h as usize, halfs);
    Environment {
        map: gpu::texture(
            r,
            "environment",
            (w, h, 1),
            wgpu::TextureFormat::Rgba16Float,
            bytemuck::cast_slice(halfs),
        ),
        marginal: gpu::texture(
            r,
            "marginal weights",
            (h, 1, 1),
            wgpu::TextureFormat::R16Float,
            bytemuck::cast_slice(&marginal),
        ),
        conditional: gpu::texture(
            r,
            "conditional weights",
            (w, h, 1),
            wgpu::TextureFormat::R16Float,
            bytemuck::cast_slice(&conditional),
        ),
        total,
    }
}

/// three's Clock on the example clock ( milliseconds ).
#[derive(Default)]
struct Clock {
    old: f64,
    elapsed: f64,
}
impl Clock {
    fn start(&mut self, now: f64) {
        self.old = now;
        self.elapsed = 0.;
    }
    fn delta(&mut self, now: f64) -> f64 {
        let diff = (now - self.old) / 1000.;
        self.old = now;
        self.elapsed += diff;
        diff
    }
}

/// The camera and scene uniforms PhysicalPathTracingMaterial reads.
pub(crate) struct Camera {
    pub world: [f64; 16],
    pub world_inverse: [f64; 16],
    pub projection_inverse: [f64; 16],
    pub position: [f64; 3],
}

/// The path tracer's render targets at one size: PathTracingRenderer's
/// primary target ( `accumulation[0]` ), the blend emulation's sample and
/// second target, the alpha mode's two blend targets, and the bind groups
/// that read them.
struct Targets {
    size: (u32, u32),
    scratch: wgpu::Texture,
    accumulation: [wgpu::Texture; 2],
    blend_targets: [wgpu::Texture; 2],
    blend_group: wgpu::BindGroup,
    /// BlendMaterial ( target1, target2 = primary ) into blend target 1, then 0.
    alpha_groups: [wgpu::BindGroup; 2],
    /// [ tone mapped, linear ][ primary, blend target 1 ].
    output_groups: [[wgpu::BindGroup; 2]; 2],
}

/// One compiled PhysicalPathTracingMaterial variant.
struct Variant {
    source: &'static str,
    uniforms: wgpu::Buffer,
    group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

/// The path tracer's GPU state and WebGLPathTracer's per-frame state.
pub(crate) struct Tracer {
    _textures: Vec<wgpu::Texture>,
    materials_texture: wgpu::Texture,
    materials_dim: usize,
    /// [ backgroundMap, none ].
    variants: [Variant; 2],
    blend_pipeline: wgpu::RenderPipeline,
    blend_layout: wgpu::BindGroupLayout,
    alpha_pipeline: wgpu::RenderPipeline,
    alpha_layout: wgpu::BindGroupLayout,
    alpha_uniforms: wgpu::Buffer,
    output: [(wgpu::RenderPipeline, wgpu::RenderPipeline); 2],
    output_layouts: [wgpu::BindGroupLayout; 2],
    output_uniforms: [wgpu::Buffer; 2],
    stratified_texture: wgpu::Texture,
    targets: Targets,
    /// The drawing buffer the render scale applies to.
    pub canvas: (u32, u32),
    random: Random,
    stratified: StratifiedSamples,
    environment_total: f64,
    // WebGLPathTracer / PathTracingRenderer state.
    pub samples: f64,
    seed: i32,
    tile: usize,
    /// Samples begun by the current renderTask ( its blend targets' swaps ).
    task_samples: usize,
    /// tiles.set( n, n ), and the count of the sample in progress.
    pub tiles: usize,
    sample_tiles: usize,
    opacity: f64,
    pub quad_opacity: f64,
    queue_reset: bool,
    /// The variant the material was last compiled with; a change recompiles
    /// on the next update, which that frame skips.
    compiled: Option<usize>,
    clock: Clock,
    pub enable: bool,
    pub pause: bool,
    pub tone_mapping: bool,
    /// scene.background === null: no backgroundMap, backgroundAlpha 0, and
    /// the alpha mode's manual blending.
    pub transparent_background: bool,
    alpha: bool,
    pub render_scale: f64,
}

fn blend_state_free_pipeline(
    r: &Renderer,
    label: &str,
    source: &str,
    layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let vs = gpu::module(r, label, &gpu::fullscreen_vs("", true));
    let fs = gpu::module(r, label, source);
    gpu::pipeline(r, label, &vs, &fs, layout, &[], ACCUMULATION, None)
}

fn texture_layout(
    r: &Renderer,
    label: &str,
    textures: u32,
    uniform: bool,
) -> wgpu::BindGroupLayout {
    let mut entries: Vec<wgpu::BindGroupLayoutEntry> = (0..textures)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        })
        .collect();
    if uniform {
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: textures,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
    }
    r.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &entries,
        })
}

impl Tracer {
    pub fn new(
        r: &Renderer,
        prepared: &super::Prepared,
        environment: &Environment,
        canvas: (u32, u32),
        format: wgpu::TextureFormat,
    ) -> Result<Self> {
        let p = prepared;
        let m = &p.merged;
        let mut random = Random::new();
        // The fixture's rebuilt samplers: init( 1, 1 ), then the blue noise.
        let stratified = StratifiedSamples::new(&mut random);
        let blue = super::sampling::blue_noise(64, &mut random);
        let mut dereferenced = Vec::with_capacity(m.index.len());
        for &t in &p.bvh.indirect {
            dereferenced.extend_from_slice(&m.index[t as usize * 3..t as usize * 3 + 3]);
        }
        let ((bounds_dim, bounds), (contents_dim, contents)) = super::bvh::textures(&p.bvh);
        let (position_dim, _, position) = textures::float_texture(&m.positions, 3);
        let (index_dim, index) = textures::index_texture(&dereferenced);
        let (attributes_dim, attributes) =
            textures::attribute_array(&m.normals, &m.tangents, &m.uvs, &m.colors);
        let material_dim = textures::dimension(m.material_index.len());
        let mut material_index = m.material_index.clone();
        material_index.resize(material_dim * material_dim, 0);
        let materials: Vec<super::scene::Material> =
            p.meshes.iter().map(|m| m.material.clone()).collect();
        let (materials_dim, materials) = textures::materials(&materials);
        let gradient = textures::gradient(
            512,
            crate::math::Color::from_hex(0xeeeeee).0.to_array(),
            crate::math::Color::from_hex(0xeaeaea).0.to_array(),
        );
        let d = |n: usize| n as u32;
        use wgpu::TextureFormat as F;
        let made = vec![
            gpu::texture(
                r,
                "stratified",
                (25, 20, 1),
                F::Rgba32Float,
                &vec![0u8; 25 * 20 * 16],
            ),
            gpu::texture(
                r,
                "blue noise",
                (64, 64, 1),
                F::R32Float,
                bytemuck::cast_slice(&blue),
            ),
            gpu::texture(r, "sobol", (1, 1, 1), F::Rgba32Float, &[0u8; 16]),
            gpu::texture(r, "ies profiles", (1, 1, 1), F::Rgba16Float, &[0u8; 8]),
            gpu::texture(r, "lights", (1, 1, 1), F::Rgba32Float, &[0u8; 16]),
            gpu::texture(
                r,
                "background",
                (512, 512, 1),
                F::Rgba32Float,
                bytemuck::cast_slice(&gradient),
            ),
            gpu::texture(
                r,
                "attributes",
                (d(attributes_dim), d(attributes_dim), 4),
                F::Rgba32Float,
                bytemuck::cast_slice(&attributes),
            ),
            gpu::texture(
                r,
                "material index",
                (d(material_dim), d(material_dim), 1),
                F::R8Uint,
                &material_index,
            ),
            gpu::texture(
                r,
                "materials",
                (d(materials_dim), d(materials_dim), 1),
                F::Rgba32Float,
                bytemuck::cast_slice(&materials),
            ),
            gpu::texture(r, "textures", (1024, 1024, 1), F::Rgba8Unorm, &p.floor_map),
            gpu::texture(
                r,
                "bvh index",
                (d(index_dim), d(index_dim), 1),
                F::Rgba32Uint,
                bytemuck::cast_slice(&index),
            ),
            gpu::texture(
                r,
                "bvh position",
                (d(position_dim), d(position_dim), 1),
                F::Rgba32Float,
                bytemuck::cast_slice(&position),
            ),
            gpu::texture(
                r,
                "bvh bounds",
                (bounds_dim, bounds_dim, 1),
                F::Rgba32Float,
                bytemuck::cast_slice(&bounds),
            ),
            gpu::texture(
                r,
                "bvh contents",
                (contents_dim, contents_dim, 1),
                F::Rg32Uint,
                bytemuck::cast_slice(&contents),
            ),
        ];
        let array = |t: &wgpu::Texture| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            })
        };
        let plain = |t: &wgpu::Texture| t.create_view(&Default::default());
        let views: Vec<wgpu::TextureView> = made
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if [3, 6, 9].contains(&i) {
                    array(t)
                } else {
                    plain(t)
                }
            })
            .collect();
        let env_views = [
            plain(&environment.map),
            plain(&environment.marginal),
            plain(&environment.conditional),
        ];
        let nearest = |view| Resource {
            view,
            filter: false,
            repeat: (false, false),
            mipmaps: false,
        };
        let linear = |view, repeat| Resource {
            view,
            filter: true,
            repeat,
            mipmaps: false,
        };
        let resources = |name: &str| -> Option<Resource> {
            Some(match name {
                "stratifiedTexture" => nearest(&views[0]),
                "stratifiedOffsetTexture" => nearest(&views[1]),
                "sobolTexture" => nearest(&views[2]),
                "iesProfiles" => linear(&views[3], (false, false)),
                "lights_tex" => nearest(&views[4]),
                "backgroundMap" => linear(&views[5], (true, false)),
                "attributesArray" => nearest(&views[6]),
                "materialIndexAttribute" => nearest(&views[7]),
                "materials" => nearest(&views[8]),
                "textures" => linear(&views[9], (true, true)),
                "bvh_index" => nearest(&views[10]),
                "bvh_position" => nearest(&views[11]),
                "bvh_bvhBounds" => nearest(&views[12]),
                "bvh_bvhContents" => nearest(&views[13]),
                "envMapInfo_map" => linear(&env_views[0], (true, false)),
                "envMapInfo_marginalWeights" => linear(&env_views[1], (false, false)),
                "envMapInfo_conditionalWeights" => linear(&env_views[2], (false, false)),
                // The shared samplers ( the view is not bound ).
                "pt_nearest" => nearest(&views[0]),
                "pt_linear" => linear(&views[3], (false, false)),
                "pt_linear_repeat_u" => linear(&views[5], (true, false)),
                "pt_linear_repeat" => linear(&views[9], (true, true)),
                _ => return None,
            })
        };
        let variant = |source: &'static str| -> Result<Variant> {
            let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("path tracing uniforms"),
                size: uniforms(source, &[])?.len() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let (layout, group) = gpu::bind_group(r, source, Some(&buffer), &resources)?;
            let vs = gpu::module(r, "path tracing quad", &gpu::fullscreen_vs(source, true));
            let fs = gpu::module(r, "PhysicalPathTracingMaterial", source);
            let pipeline = gpu::pipeline(
                r,
                "PhysicalPathTracingMaterial",
                &vs,
                &fs,
                &layout,
                &[],
                ACCUMULATION,
                None,
            );
            Ok(Variant {
                source,
                uniforms: buffer,
                group,
                pipeline,
            })
        };
        let variants = [variant(PATH_TRACING)?, variant(PATH_TRACING_NO_BACKGROUND)?];
        let blend_layout = texture_layout(r, "accumulation blend", 2, false);
        let blend_pipeline =
            blend_state_free_pipeline(r, "accumulation blend", BLEND, &blend_layout);
        let alpha_layout = texture_layout(r, "BlendMaterial", 2, true);
        let alpha_pipeline =
            blend_state_free_pipeline(r, "BlendMaterial", ALPHA_BLEND, &alpha_layout);
        let alpha_uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("BlendMaterial opacity"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // ClampedInterpolationMaterial, tone mapped or not, without and with
        // ( premultiplied ) NormalBlending.
        let mut output = vec![];
        let mut output_layouts = vec![];
        let mut output_uniforms = vec![];
        let probe = gpu::target(r, "layout probe", (1, 1), ACCUMULATION);
        let probe_view = probe.create_view(&Default::default());
        for source in [OUTPUT, OUTPUT_LINEAR] {
            let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ClampedInterpolationMaterial uniforms"),
                size: uniforms(source, &[])?.len() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let map = |name: &str| {
                (name == "map").then_some(Resource {
                    view: &probe_view,
                    filter: true,
                    repeat: (false, false),
                    mipmaps: false,
                })
            };
            let (layout, _) = gpu::bind_group(r, source, Some(&buffer), &map)?;
            let vs = gpu::module(r, "output quad", &gpu::fullscreen_vs(source, false));
            let fs = gpu::module(r, "ClampedInterpolationMaterial", source);
            output.push((
                gpu::pipeline(
                    r,
                    "ClampedInterpolationMaterial",
                    &vs,
                    &fs,
                    &layout,
                    &[],
                    format,
                    None,
                ),
                gpu::pipeline(
                    r,
                    "ClampedInterpolationMaterial blended",
                    &vs,
                    &fs,
                    &layout,
                    &[],
                    format,
                    Some(gpu::normal_blending(true)),
                ),
            ));
            output_layouts.push(layout);
            output_uniforms.push(buffer);
        }
        let output_layouts: [wgpu::BindGroupLayout; 2] =
            [output_layouts.remove(0), output_layouts.remove(0)];
        let output_uniforms: [wgpu::Buffer; 2] =
            [output_uniforms.remove(0), output_uniforms.remove(0)];
        let size = scaled(canvas, 1.);
        let targets = Self::targets(
            r,
            size,
            &blend_layout,
            &alpha_layout,
            &alpha_uniforms,
            &output_layouts,
            &output_uniforms,
        )?;
        let mut textures = made;
        textures.push(environment.map.clone());
        let tracer = Self {
            stratified_texture: textures[0].clone(),
            materials_texture: textures[8].clone(),
            materials_dim,
            _textures: textures,
            variants,
            blend_pipeline,
            blend_layout,
            alpha_pipeline,
            alpha_layout,
            alpha_uniforms,
            output: [output.remove(0), output.remove(0)],
            output_layouts,
            output_uniforms,
            targets,
            canvas,
            random,
            stratified,
            environment_total: environment.total,
            samples: 0.,
            seed: 0,
            tile: 0,
            task_samples: 0,
            tiles: 3,
            sample_tiles: 3,
            opacity: 1.,
            quad_opacity: 0.,
            queue_reset: true,
            compiled: None,
            clock: Clock::default(),
            enable: true,
            pause: false,
            tone_mapping: true,
            transparent_background: false,
            alpha: false,
            render_scale: 1.,
        };
        tracer.clear(r);
        Ok(tracer)
    }

    fn targets(
        r: &Renderer,
        size: (u32, u32),
        blend_layout: &wgpu::BindGroupLayout,
        alpha_layout: &wgpu::BindGroupLayout,
        alpha_uniforms: &wgpu::Buffer,
        output_layouts: &[wgpu::BindGroupLayout; 2],
        output_uniforms: &[wgpu::Buffer; 2],
    ) -> Result<Targets> {
        let make = |label| gpu::target(r, label, size, ACCUMULATION);
        let scratch = make("path tracing sample");
        let accumulation = [make("path tracing target"), make("path tracing blend")];
        let blend_targets = [make("blend target 0"), make("blend target 1")];
        let view = |t: &wgpu::Texture| t.create_view(&Default::default());
        let group = |layout: &wgpu::BindGroupLayout,
                     label: &str,
                     a: &wgpu::Texture,
                     b: &wgpu::Texture,
                     uniform: Option<&wgpu::Buffer>| {
            let (va, vb) = (view(a), view(b));
            let mut entries = vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&va),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&vb),
                },
            ];
            if let Some(u) = uniform {
                entries.push(wgpu::BindGroupEntry {
                    binding: 2,
                    resource: u.as_entire_binding(),
                });
            }
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout,
                entries: &entries,
            })
        };
        let blend_group = group(
            blend_layout,
            "accumulation blend",
            &scratch,
            &accumulation[0],
            None,
        );
        let alpha_groups = [
            group(
                alpha_layout,
                "BlendMaterial",
                &blend_targets[0],
                &accumulation[0],
                Some(alpha_uniforms),
            ),
            group(
                alpha_layout,
                "BlendMaterial",
                &blend_targets[1],
                &accumulation[0],
                Some(alpha_uniforms),
            ),
        ];
        let output_group = |k: usize, t: &wgpu::Texture| -> Result<wgpu::BindGroup> {
            let v = view(t);
            let source = [OUTPUT, OUTPUT_LINEAR][k];
            let map = |name: &str| {
                (name == "map").then_some(Resource {
                    view: &v,
                    filter: true,
                    repeat: (false, false),
                    mipmaps: false,
                })
            };
            let (_, g) = gpu::bind_group_with_layout(
                r,
                source,
                Some(&output_uniforms[k]),
                &map,
                &output_layouts[k],
            )?;
            Ok(g)
        };
        let output_groups = [
            [
                output_group(0, &accumulation[0])?,
                output_group(0, &blend_targets[1])?,
            ],
            [
                output_group(1, &accumulation[0])?,
                output_group(1, &blend_targets[1])?,
            ],
        ];
        Ok(Targets {
            size,
            scratch,
            accumulation,
            blend_targets,
            blend_group,
            alpha_groups,
            output_groups,
        })
    }

    /// PathTracingRenderer.reset: the targets cleared to transparent black,
    /// the sample in progress dropped.
    fn clear(&self, r: &Renderer) {
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let t = &self.targets;
        for t in [
            &t.accumulation[0],
            &t.accumulation[1],
            &t.scratch,
            &t.blend_targets[0],
            &t.blend_targets[1],
        ] {
            let view = t.create_view(&Default::default());
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("path tracing reset"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        r.queue.submit([encoder.finish()]);
    }

    fn renderer_reset(&mut self, r: &Renderer) {
        self.clear(r);
        self.samples = 0.;
        self.tile = 0;
        self.task_samples = 0;
    }

    /// WebGLPathTracer.reset: queued for the next renderSample.
    pub fn reset(&mut self) {
        self.queue_reset = true;
        self.samples = 0.;
    }

    /// updateMaterials: MaterialsTexture.updateFrom over the changed list.
    pub fn update_materials(&mut self, r: &Renderer, materials: &[super::scene::Material]) {
        let (dim, data) = textures::materials(materials);
        debug_assert_eq!(dim, self.materials_dim);
        r.queue.write_texture(
            self.materials_texture.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(dim as u32 * 16),
                rows_per_image: Some(dim as u32),
            },
            wgpu::Extent3d {
                width: dim as u32,
                height: dim as u32,
                depth_or_array_layers: 1,
            },
        );
        self.reset();
    }

    /// _updateScale: the target follows the drawing buffer and render scale.
    fn update_scale(&mut self, r: &Renderer) -> Result<()> {
        let size = scaled(self.canvas, self.render_scale);
        if size != self.targets.size {
            self.targets = Self::targets(
                r,
                size,
                &self.blend_layout,
                &self.alpha_layout,
                &self.alpha_uniforms,
                &self.output_layouts,
                &self.output_uniforms,
            )?;
            self.renderer_reset(r);
        }
        Ok(())
    }

    /// PathTracingRenderer.update: one tile of the current sample.
    fn update(&mut self, r: &Renderer, camera: &Camera) -> Result<()> {
        let variant = usize::from(self.transparent_background);
        if self.compiled != Some(variant) {
            // onBeforeRender changes the material's defines: the recompile
            // skips this frame.
            self.compiled = Some(variant);
            return Ok(());
        }
        let (w, h) = self.targets.size;
        if self.tile == 0 {
            self.sample_tiles = self.tiles;
            self.task_samples += 1;
            self.opacity = 1. / (self.samples + 1.);
            self.stratified.init(20, 10 + 10 + 5, &mut self.random);
            self.stratified.next(&mut self.random);
            self.seed += 1;
            r.queue.write_texture(
                self.stratified_texture.as_image_copy(),
                bytemuck::cast_slice(&self.stratified.samples),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(25 * 16),
                    rows_per_image: Some(20),
                },
                wgpu::Extent3d {
                    width: 25,
                    height: 20,
                    depth_or_array_layers: 1,
                },
            );
            r.queue.write_buffer(
                &self.alpha_uniforms,
                0,
                bytemuck::cast_slice(&[self.opacity as f32, 0., 0., 0.]),
            );
        }
        let ident = vec![
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ];
        let v = &self.variants[variant];
        let data = uniforms(
            v.source,
            &[
                ("viewMatrix", camera.world_inverse.to_vec()),
                ("cameraPosition", camera.position.to_vec()),
                ("envMapInfo_totalSum", vec![self.environment_total]),
                ("environmentRotation", ident.clone()),
                ("environmentIntensity", vec![1.]),
                ("lights_count", vec![0.]),
                ("backgroundBlur", vec![0.]),
                (
                    "backgroundAlpha",
                    vec![if self.transparent_background { 0. } else { 1. }],
                ),
                ("backgroundRotation", ident),
                ("backgroundIntensity", vec![1.]),
                ("cameraWorldMatrix", camera.world.to_vec()),
                ("invProjectionMatrix", camera.projection_inverse.to_vec()),
                ("physicalCamera_focusDistance", vec![10.]),
                ("physicalCamera_anamorphicRatio", vec![1.]),
                ("physicalCamera_bokehSize", vec![0.]),
                ("physicalCamera_apertureBlades", vec![0.]),
                ("physicalCamera_apertureRotation", vec![0.]),
                ("bounces", vec![10.]),
                ("transmissiveBounces", vec![10.]),
                ("filterGlossyFactor", vec![1.]),
                ("seed", vec![self.seed as f64]),
                ("resolution", vec![w as f64, h as f64]),
                // The alpha mode renders the sample at full opacity.
                ("opacity", vec![if self.alpha { 1. } else { self.opacity }]),
            ],
        )?;
        r.queue.write_buffer(&v.uniforms, 0, &data);
        // The tile's scissor, rows counted from texture row 0 as in WebGL.
        let tiles = self.sample_tiles as u32;
        let tile_w = w.div_ceil(tiles);
        let tile_h = h.div_ceil(tiles);
        let (tx, ty) = (self.tile as u32 % tiles, self.tile as u32 / tiles);
        let reverse_ty = tiles - ty - 1;
        let (x, y) = (tx * tile_w, reverse_ty * tile_h);
        let (sw, sh) = (
            tile_w.min(w.saturating_sub(x)),
            tile_h.min(h.saturating_sub(y)),
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let pass = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    pipeline: &wgpu::RenderPipeline,
                    group: &wgpu::BindGroup,
                    scissor: bool| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("path tracing tile"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            if scissor {
                pass.set_scissor_rect(x, y, sw, sh);
            }
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.draw(0..3, 0..1);
        };
        let t = &self.targets;
        let view = |t: &wgpu::Texture| t.create_view(&Default::default());
        if sw > 0 && sh > 0 {
            if self.alpha {
                // NoBlending into the primary target.
                pass(
                    &mut encoder,
                    &view(&t.accumulation[0]),
                    &v.pipeline,
                    &v.group,
                    true,
                );
            } else {
                pass(&mut encoder, &view(&t.scratch), &v.pipeline, &v.group, true);
                pass(
                    &mut encoder,
                    &view(&t.accumulation[1]),
                    &self.blend_pipeline,
                    &t.blend_group,
                    true,
                );
                let origin = wgpu::Origin3d { x, y, z: 0 };
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &t.accumulation[1],
                        mip_level: 0,
                        origin,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &t.accumulation[0],
                        mip_level: 0,
                        origin,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: sw,
                        height: sh,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }
        if self.alpha {
            // The renderTask's local blend targets swap after every sample;
            // `target` stays blend target 1.
            let odd = self.task_samples % 2 == 1;
            let (write, group) = if odd {
                (&t.blend_targets[1], &t.alpha_groups[0])
            } else {
                (&t.blend_targets[0], &t.alpha_groups[1])
            };
            pass(
                &mut encoder,
                &view(write),
                &self.alpha_pipeline,
                group,
                false,
            );
        }
        r.queue.submit([encoder.finish()]);
        self.samples += 1. / (self.sample_tiles * self.sample_tiles) as f64;
        if self.tile == self.sample_tiles * self.sample_tiles - 1 {
            self.samples = self.samples.round();
        }
        self.tile = (self.tile + 1) % (self.sample_tiles * self.sample_tiles);
        Ok(())
    }

    /// renderSample: the scale, the reset, the clock, one tile after the
    /// render delay, the alpha mode and the fade. Returns whether the raster
    /// fallback draws.
    pub fn render_sample(&mut self, r: &Renderer, camera: &Camera, now: f64) -> Result<bool> {
        self.update_scale(r)?;
        if self.queue_reset {
            self.renderer_reset(r);
            self.queue_reset = false;
            self.quad_opacity = 0.;
            self.clock.start(now);
        }
        let delta = self.clock.delta(now) * 1e3;
        // getElapsedTime steps the clock again.
        self.clock.delta(now);
        let elapsed = self.clock.elapsed * 1e3;
        if !self.pause && self.enable && RENDER_DELAY <= elapsed {
            self.update(r, camera)?;
        }
        // pathTracer.alpha = backgroundAlpha !== 1: a change resets the renderer.
        if self.alpha != self.transparent_background {
            self.alpha = self.transparent_background;
            self.renderer_reset(r);
        }
        if elapsed >= RENDER_DELAY && self.samples >= MIN_SAMPLES {
            self.quad_opacity = (self.quad_opacity + delta / FADE_DURATION).min(1.);
        }
        Ok(!self.enable || self.samples < MIN_SAMPLES || self.quad_opacity < 1.)
    }

    /// The quad onto the canvas target ( blended while it fades in ).
    pub fn draw_quad(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: bool,
    ) -> Result<()> {
        if !(self.enable && self.quad_opacity > 0.) {
            return Ok(());
        }
        let k = if self.tone_mapping { 0 } else { 1 };
        let source = [OUTPUT, OUTPUT_LINEAR][k];
        let data = uniforms(
            source,
            &[
                ("toneMappingExposure", vec![1.]),
                ("opacity", vec![self.quad_opacity]),
            ],
        )?;
        r.queue.write_buffer(&self.output_uniforms[k], 0, &data);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ClampedInterpolationMaterial"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        let (plain, blended) = &self.output[k];
        pass.set_pipeline(if self.quad_opacity < 1. {
            blended
        } else {
            plain
        });
        pass.set_bind_group(
            0,
            &self.targets.output_groups[k][usize::from(self.alpha)],
            &[],
        );
        pass.draw(0..3, 0..1);
        Ok(())
    }
}

/// floor( renderScale × drawing buffer ), at least one texel.
fn scaled((w, h): (u32, u32), scale: f64) -> (u32, u32) {
    (
        ((scale * w as f64).floor() as u32).max(1),
        ((scale * h as f64).floor() as u32).max(1),
    )
}
