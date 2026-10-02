//! webgpu_compute_rasterizer_ibl: 15,625 rotating DamagedHelmets ( a 125 ×
//! 125 plane or a 25³ volume ) drawn by the compute-shader software
//! rasterizer with a hardware fallback, lit by royal_esplanade's PMREM under
//! FirstPersonControls. The page's Meshopt LODs and 64-triangle meshlets are
//! the official libraries' output for the helmet ( baked once by
//! `tools/tsl/prepare-rasterizer-ibl.mjs` and packed at load as the page packs
//! them ). Each frame runs the page's compute passes on the resident storage
//! buffers: Compute Clear, Compute Frustum ( frustum, depth-pyramid occlusion
//! against the previous frame and LOD selection, per instance and per chunk ),
//! the indirect dispatch, Compute Rasterize into the depth-packed atomic
//! visibility buffers and the hardware queue's indirect draw. The scene pass
//! draws the blurred background, resolves the visibility buffers through the
//! standard material ( or the meshlet and channel views ) with their depth,
//! and draws the queued large triangles; the HZB kernels then build next
//! frame's depth pyramid, and the blit and output passes tone map to the
//! canvas. Every stage runs the WGSL three.js r186 generates for the page ( in
//! `compute_rasterizer_ibl/` ), without its unused subgroup declarations; the
//! HZB kernels past level 1 are level 1's with their level indices, as the
//! page's loop generates them.
use super::deferred::ultra_hdr_environment;
use super::gltf_viewer::{fetch, load_asset};
use super::lights_projector::{m3, m4};
use super::pmrem_cube_uv::Pmrem;
use super::retro::{mipmapped, target};
use super::shadowmap_opacity::Mipmaps;
use super::trackball_sprites::FirstPerson;
use super::wgsl_bind::{Uniforms, groups};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const SCENE_DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const INSTANCES: usize = 15_625;
const MAX_RASTER_SIZE: f64 = 32.;
const MAX_WORK_ITEMS: u64 = 2_820_000;
const MAX_HW_TRIANGLES: u64 = 100_000;
const MAX_HZB_LEVELS: usize = 16;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("compute_rasterizer_ibl/", $name, ".wgsl"))
    };
}
const CLEAR: &str = wgsl!("clear");
const FRUSTUM: &str = wgsl!("frustum");
const DISPATCH: &str = wgsl!("dispatch");
const RASTERIZE: &str = wgsl!("rasterize");
const HW_ARGS: &str = wgsl!("hw_args");
const HZB0: &str = wgsl!("hzb0");
const HZB: &str = wgsl!("hzb");
const BACKGROUND: (&str, &str) = (
    include_str!("compute_cloth/background_vs.wgsl"),
    include_str!("compute_cloth/background_fs.wgsl"),
);
/// The resolve and hardware materials: shaded ( Default ), meshlet debug and
/// the channel views.
const RESOLVE: [(&str, &str); 3] = [
    (wgsl!("resolve_vs"), wgsl!("resolve_fs")),
    (wgsl!("debug_resolve_vs"), wgsl!("debug_resolve_fs")),
    (wgsl!("vis_resolve_vs"), wgsl!("vis_resolve_fs")),
];
const HW: [(&str, &str); 3] = [
    (wgsl!("hw_vs"), wgsl!("hw_fs")),
    (wgsl!("debug_hw_vs"), wgsl!("debug_hw_fs")),
    (wgsl!("vis_hw_vs"), wgsl!("vis_hw_fs")),
];
const BLIT: (&str, &str) = (
    include_str!("volume_caustics/high_vs.wgsl"),
    wgsl!("blit_fs"),
);
/// The output pass with ACES ( Default ) and without tone mapping.
const OUTPUT: [(&str, &str); 2] = [
    (
        include_str!("cubemap_dynamic/output_vs.wgsl"),
        include_str!("cubemap_dynamic/output_fs.wgsl"),
    ),
    (
        include_str!("compute_rasterizer/8.wgsl"),
        include_str!("compute_rasterizer/9.wgsl"),
    ),
];
/// The HZB kernel for pyramid level `k` ≥ 1: level 1's with its level
/// indices.
fn hzb_source(k: usize) -> String {
    HZB.replace("NodeBuffer_1002.value[ 1u ]", "@LEVEL@")
        .replace("NodeBuffer_1002.value[ 0u ]", "@SOURCE@")
        .replace("HZB Level 1", &format!("HZB Level {k}"))
        .replace("@LEVEL@", &format!("NodeBuffer_1002.value[ {k}u ]"))
        .replace("@SOURCE@", &format!("NodeBuffer_1002.value[ {}u ]", k - 1))
}
fn compute_pipeline(r: &Renderer, label: &str, source: &str) -> wgpu::ComputePipeline {
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    r.device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(label),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        })
}
/// A render pipeline with back-face culling.
fn render_pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    depth: Option<(wgpu::TextureFormat, wgpu::CompareFunction, bool)>,
    cw: bool,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: depth.map(|(format, compare, write)| wgpu::DepthStencilState {
                format,
                depth_write_enabled: write,
                depth_compare: compare,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn storage(r: &Renderer, label: &str, contents: &[u8], indirect: bool) -> wgpu::Buffer {
    let usage = wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | if indirect {
            wgpu::BufferUsages::INDIRECT
        } else {
            wgpu::BufferUsages::empty()
        };
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage,
        })
}
fn zeroed(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn uniform_array(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn vertex(r: &Renderer, label: &str, contents: &[u8]) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: wgpu::BufferUsages::VERTEX,
        })
}
/// The page's packed LOD buffers: vec4 positions and normals and the uvs
/// ( one copy per LOD ), the padded index, a meshlet id per triangle, the
/// chunk bounding spheres and the LOD offsets ( triangleStart, numTriangles,
/// chunkStart, 0 ).
type Lods = (
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<u32>,
    Vec<u32>,
    Vec<f32>,
    Vec<f32>,
);
fn pack_lods(bytes: &[u8]) -> Result<Lods> {
    let mut at = 0usize;
    let mut take = |n: usize| -> Result<&[u8]> {
        let s = bytes
            .get(at..at + n)
            .ok_or(Error::Invalid("helmet LOD data"))?;
        at += n;
        Ok(s)
    };
    let word = |s: &[u8]| u32::from_le_bytes([s[0], s[1], s[2], s[3]]);
    let vertex_count = word(take(4)?) as usize;
    let lod_count = word(take(4)?) as usize;
    let floats = |s: &[u8]| -> Vec<f32> {
        s.as_chunks::<4>()
            .0
            .iter()
            .map(|&c| f32::from_le_bytes(c))
            .collect()
    };
    let positions = floats(take(vertex_count * 12)?);
    let normals = floats(take(vertex_count * 12)?);
    let uvs = floats(take(vertex_count * 8)?);
    let (mut vertex_array, mut normal_array, mut uv_array) = (vec![], vec![], vec![]);
    let (mut index_array, mut meshlet_ids, mut bounds, mut offsets) =
        (vec![], vec![], vec![], vec![]);
    let mut meshlet = 1u32;
    for lod in 0..lod_count {
        for v in 0..vertex_count {
            vertex_array.extend([
                positions[v * 3],
                positions[v * 3 + 1],
                positions[v * 3 + 2],
                1.,
            ]);
            normal_array.extend([normals[v * 3], normals[v * 3 + 1], normals[v * 3 + 2], 0.]);
            uv_array.extend([uvs[v * 2], uvs[v * 2 + 1]]);
        }
        let chunks = word(take(4)?) as usize;
        let base = (lod * vertex_count) as u32;
        offsets.extend([
            (index_array.len() / 3) as f32,
            (chunks * 64) as f32,
            (bounds.len() / 4) as f32,
            0.,
        ]);
        for _ in 0..chunks {
            for &c in take(64 * 3 * 2)?.as_chunks::<2>().0 {
                index_array.push(base + u32::from(u16::from_le_bytes(c)));
            }
            meshlet_ids.extend([meshlet; 64]);
            meshlet += 1;
            bounds.extend(floats(take(16)?));
        }
    }
    Ok((
        vertex_array,
        normal_array,
        uv_array,
        index_array,
        meshlet_ids,
        bounds,
        offsets,
    ))
}
/// The resident storage buffers, by the page's names.
struct Buffers {
    lod_offsets: wgpu::Buffer,
    chunk_bounds: wgpu::Buffer,
    vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: wgpu::Buffer,
    indices: wgpu::Buffer,
    meshlet_ids: wgpu::Buffer,
    instance_data: wgpu::Buffer,
    instance_world: wgpu::Buffer,
    instance_prev_world: wgpu::Buffer,
    instance_mvp: wgpu::Buffer,
    work_queue_count: wgpu::Buffer,
    dispatch: wgpu::Buffer,
    work_queue: wgpu::Buffer,
    hw_queue: wgpu::Buffer,
    hw_draw: wgpu::Buffer,
    frustum_planes: wgpu::Buffer,
    hzb_table: wgpu::Buffer,
    /// The hardware mesh's ( unread ) positions and the fullscreen triangle.
    hw_positions: wgpu::Buffer,
    fullscreen: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    /// The background sphere: normal, position, index and count.
    sphere: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
}
/// The size-dependent resources ( createScreenBuffers ).
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// sceneRT ( HalfFloat ) and its FloatType depth.
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// The renderer's output buffer the blit draws into.
    present: wgpu::TextureView,
    _screen: [wgpu::Buffer; 2],
    _hzb: wgpu::Buffer,
    levels: Vec<(u32, u32, u32)>,
    clear: Vec<wgpu::BindGroup>,
    frustum: Vec<wgpu::BindGroup>,
    rasterize: Vec<wgpu::BindGroup>,
    hzb: Vec<Vec<wgpu::BindGroup>>,
    background: Vec<wgpu::BindGroup>,
    resolve: [Vec<wgpu::BindGroup>; 3],
    hw: [Vec<wgpu::BindGroup>; 3],
    blit: Vec<wgpu::BindGroup>,
    output: [Vec<wgpu::BindGroup>; 2],
    screen: RenderTarget,
}
struct Stages {
    clear: Uniforms,
    frustum: Uniforms,
    dispatch: Uniforms,
    rasterize: Uniforms,
    hw_args: Uniforms,
    hzb: Vec<Uniforms>,
    background: Uniforms,
    resolve: [Uniforms; 3],
    hw: [Uniforms; 3],
    blit: Uniforms,
    output: [Uniforms; 2],
}
struct Pipelines {
    compute: [wgpu::ComputePipeline; 5],
    hzb: Vec<wgpu::ComputePipeline>,
    background: wgpu::RenderPipeline,
    resolve: [wgpu::RenderPipeline; 3],
    hw: [wgpu::RenderPipeline; 3],
    blit: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    walker: FirstPerson,
    /// Output ( 0 Default, 1 Meshlet Debug, 2 Geometry Normal … 8 Emissive ),
    /// Rasterizer ( 0 SW Only, 1 HW Only, 2 Both ), Grid ( 0 XZ, 1 XYZ ),
    /// Occlusion Bias, LOD Threshold and Animation Speed.
    params: [f64; 6],
    time: f64,
    delta: f64,
    buffers: Buffers,
    stages: Stages,
    pipelines: Pipelines,
    /// Albedo, metal-roughness, emissive, AO and normal.
    maps: [wgpu::TextureView; 5],
    pmrem: Pmrem,
    repeat: wgpu::Sampler,
    clamp: wgpu::Sampler,
    /// The previous frame's projScreenMatrix and camera position, once seeded.
    previous: Option<(Matrix4, Vector3)>,
    /// Whether a frame has rendered: the page's first frame computes its
    /// matrices before the renderer switches the camera to WebGPU's clip
    /// space.
    rendered: bool,
    /// A Grid change waiting for the next frame: updateGrid()'s layout.
    grid: Option<bool>,
    /// The dispatch and HW args passes' bind groups.
    dispatch_groups: Vec<wgpu::BindGroup>,
    hw_args_groups: Vec<wgpu::BindGroup>,
    /// The output pipelines, for the canvas format.
    output: Option<(wgpu::TextureFormat, [wgpu::RenderPipeline; 2])>,
    targets: Option<Targets>,
}
/// updateGrid(): the instance layout and the camera placement.
fn grid(xyz: bool) -> (Vec<f32>, Vector3, Vector3) {
    let mut data = Vec::with_capacity(INSTANCES * 4);
    if xyz {
        for x in 0..25 {
            for y in 0..25 {
                for z in 0..25 {
                    data.extend([
                        ((x - 12) * 4) as f32,
                        ((y - 12) * 4) as f32,
                        ((z - 12) * 4) as f32,
                        1.,
                    ]);
                }
            }
        }
        (data, Vector3::new(2., 2., 40.), Vector3::ZERO)
    } else {
        for x in 0..125 {
            for z in 0..125 {
                data.extend([((x - 62) * 4) as f32, -1., ((z - 62) * 4) as f32, 1.]);
            }
        }
        (data, Vector3::new(0., 8., 30.), Vector3::new(0., -1., 0.))
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.25,
            far: 1_000_000.,
            aspect,
            ..Default::default()
        }));
        let mut walker = FirstPerson::default();
        walker.speed = 10.;
        let (instance_data, position, look) = grid(false);
        Self::place(s, c, &mut walker, position, look)?;
        let lods = pack_lods(&fetch("/web/gallery/assets/rasterizer-ibl/helmet-lods.bin").await?)?;
        let (vertices, normals, uvs, indices, meshlet_ids, bounds, offsets) = lods;
        let (_, _, images) =
            load_asset("/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf").await?;
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("helmet image"));
        let mut mipmaps = Mipmaps::new(r);
        let (srgb, linear) = (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba8Unorm,
        );
        let maps = [
            mipmapped(r, &mut mipmaps, image(0)?, srgb),
            mipmapped(r, &mut mipmaps, image(1)?, linear),
            mipmapped(r, &mut mipmaps, image(2)?, srgb),
            mipmapped(r, &mut mipmaps, image(3)?, linear),
            mipmapped(r, &mut mipmaps, image(4)?, linear),
        ];
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let linear_mips = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // scene.background and scene.environment: the UltraHDR equirect's PMREM.
        let (_, _, pmrem, _) = ultra_hdr_environment(
            r,
            &linear_mips,
            &clamp,
            "/web/environments/royal_esplanade_2k.hdr.jpg",
        )
        .await?;
        let bytes = |v: &[f32]| bytemuck::cast_slice::<f32, u8>(v).to_vec();
        let words = |v: &[u32]| bytemuck::cast_slice::<u32, u8>(v).to_vec();
        let mat_bytes = INSTANCES as u64 * 64;
        let sphere = SphereGeometry::build(1., 32, 32)?;
        let read = |name: &str| -> Result<Vec<f32>> {
            match sphere.attributes.get(name) {
                Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
                _ => Err(Error::Invalid("background attribute")),
            }
        };
        let sphere_index = sphere
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        let lod_offsets = uniform_array(r, "lod offsets", 96);
        r.queue.write_buffer(&lod_offsets, 0, &bytes(&offsets));
        let buffers = Buffers {
            lod_offsets,
            chunk_bounds: storage(r, "chunk bounds", &bytes(&bounds), false),
            vertices: storage(r, "helmet vertices", &bytes(&vertices), false),
            normals: storage(r, "helmet normals", &bytes(&normals), false),
            uvs: storage(r, "helmet uvs", &bytes(&uvs), false),
            indices: storage(r, "helmet indices", &words(&indices), false),
            meshlet_ids: storage(r, "meshlet ids", &words(&meshlet_ids), false),
            instance_data: storage(r, "instance data", &bytes(&instance_data), false),
            instance_world: zeroed(r, "instance world", mat_bytes),
            instance_prev_world: zeroed(r, "instance previous world", mat_bytes),
            instance_mvp: zeroed(r, "instance mvp", mat_bytes),
            work_queue_count: zeroed(r, "work queue count", 4),
            dispatch: storage(r, "dispatch", &[0; 12], true),
            work_queue: zeroed(r, "work queue", MAX_WORK_ITEMS * 16),
            hw_queue: zeroed(r, "hw queue", (1 + MAX_HW_TRIANGLES * 2) * 4),
            hw_draw: storage(r, "hw draw", &[0; 16], true),
            frustum_planes: uniform_array(r, "frustum planes", 96),
            hzb_table: uniform_array(r, "hzb level table", MAX_HZB_LEVELS as u64 * 16),
            hw_positions: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hw positions"),
                size: MAX_HW_TRIANGLES * 36,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            }),
            fullscreen: vertex(
                r,
                "resolve triangle",
                bytemuck::cast_slice(&[-1f32, -1., 0., 3., -1., 0., -1., 3., 0.]),
            ),
            quad_uv: vertex(
                r,
                "blit uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
            ),
            sphere: (
                vertex(r, "background normal", &bytes(&read("normal")?)),
                vertex(r, "background position", &bytes(&read("position")?)),
                r.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("background index"),
                        contents: bytemuck::cast_slice(&sphere_index),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                sphere_index.len() as u32,
            ),
        };
        let hzb_sources: Vec<&'static str> = (1..MAX_HZB_LEVELS)
            .map(|k| &*Box::leak(hzb_source(k).into_boxed_str()))
            .collect();
        let u = |label: &str, sources: &[&'static str]| Uniforms::new(r, label, sources);
        let mut hzb = vec![u("HZB Level 0", &[HZB0])?];
        for source in &hzb_sources {
            hzb.push(u("HZB Level", &[source])?);
        }
        let pair = |label: &str, (vs, fs): (&'static str, &'static str)| u(label, &[vs, fs]);
        let stages = Stages {
            clear: u("Compute Clear", &[CLEAR])?,
            frustum: u("Compute Frustum", &[FRUSTUM])?,
            dispatch: u("Compute Dispatch", &[DISPATCH])?,
            rasterize: u("Compute Rasterize", &[RASTERIZE])?,
            hw_args: u("Compute HW Args", &[HW_ARGS])?,
            hzb,
            background: pair("background", BACKGROUND)?,
            resolve: [
                pair("resolve", RESOLVE[0])?,
                pair("resolve debug", RESOLVE[1])?,
                pair("resolve vis", RESOLVE[2])?,
            ],
            hw: [
                pair("hw mesh", HW[0])?,
                pair("hw debug", HW[1])?,
                pair("hw vis", HW[2])?,
            ],
            blit: pair("blit", BLIT)?,
            output: [pair("output", OUTPUT[0])?, pair("output", OUTPUT[1])?],
        };
        // The constant uniforms.
        let st = &stages;
        st.dispatch
            .write(r, "objectStruct", &[("nodeUniform2", &[1.])])?;
        st.hw_args
            .write(r, "objectStruct", &[("nodeUniform2", &[1.])])?;
        st.rasterize
            .write(r, "objectStruct", &[("nodeUniform6", &[MAX_RASTER_SIZE])])?;
        st.blit.write(
            r,
            "objectStruct",
            &[
                ("nodeUniform1", &m3(Matrix4::IDENTITY)),
                ("nodeUniform2", &[1.]),
            ],
        )?;
        let identity3 = m3(Matrix4::IDENTITY);
        st.background.write(
            r,
            "objectStruct",
            &[
                ("nodeUniform1", &[9.]),
                ("nodeUniform2", &m4(Matrix4::IDENTITY)),
                ("nodeUniform5", &identity3),
                ("nodeUniform6", &[1. / 1536.]),
                ("nodeUniform7", &[1. / 2048.]),
                ("nodeUniform10", &[1.]),
                ("nodeUniform12", &m4(Matrix4::IDENTITY)),
            ],
        )?;
        st.output[0].write(
            r,
            "objectStruct",
            &[("nodeUniform5", &m4(Matrix4::IDENTITY))],
        )?;
        st.output[1].write(
            r,
            "objectStruct",
            &[("nodeUniform4", &m4(Matrix4::IDENTITY))],
        )?;
        let hzb_pipelines = std::iter::once(compute_pipeline(r, "HZB Level 0", HZB0))
            .chain(
                hzb_sources
                    .iter()
                    .map(|source| compute_pipeline(r, "HZB Level", source)),
            )
            .collect();
        let float3 = |location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: location,
            }]
        };
        let (normal, position) = (float3(0), float3(1));
        let layout = |attributes| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
        let position0 = float3(0);
        let less = Some((SCENE_DEPTH, wgpu::CompareFunction::LessEqual, true));
        let scene = |label, shaders| {
            render_pipeline(r, label, shaders, &[layout(&position0)], HALF, less, false)
        };
        let uv_attribute = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }];
        let pipelines = Pipelines {
            compute: [
                compute_pipeline(r, "Compute Clear", CLEAR),
                compute_pipeline(r, "Compute Frustum", FRUSTUM),
                compute_pipeline(r, "Compute Dispatch", DISPATCH),
                compute_pipeline(r, "Compute Rasterize", RASTERIZE),
                compute_pipeline(r, "Compute HW Args", HW_ARGS),
            ],
            hzb: hzb_pipelines,
            background: render_pipeline(
                r,
                "background",
                BACKGROUND,
                &[layout(&normal), layout(&position)],
                HALF,
                Some((SCENE_DEPTH, wgpu::CompareFunction::Always, false)),
                true,
            ),
            resolve: [
                scene("resolve", RESOLVE[0]),
                scene("resolve debug", RESOLVE[1]),
                scene("resolve vis", RESOLVE[2]),
            ],
            hw: [
                scene("hw mesh", HW[0]),
                scene("hw debug", HW[1]),
                scene("hw vis", HW[2]),
            ],
            blit: render_pipeline(
                r,
                "blit",
                BLIT,
                &[wgpu::VertexBufferLayout {
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &uv_attribute,
                }],
                HALF,
                None,
                false,
            ),
        };
        let mut demo = Self {
            walker,
            params: [0., 2., 0., 0.0008, 3., 1.],
            time: 0.,
            delta: 0.,
            buffers,
            stages,
            pipelines,
            maps,
            pmrem,
            repeat,
            clamp,
            previous: None,
            rendered: false,
            grid: None,
            dispatch_groups: vec![],
            hw_args_groups: vec![],
            output: None,
            targets: None,
        };
        demo.dispatch_groups = demo.make_dispatch_groups(r)?;
        demo.hw_args_groups = demo.make_hw_args_groups(r)?;
        Ok(demo)
    }
    /// camera.position.set( … ) and controls.lookAt( … ).
    fn place(
        s: &mut Scene,
        c: Object3D,
        walker: &mut FirstPerson,
        position: Vector3,
        target: Vector3,
    ) -> Result<()> {
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        let look = (target - position).normalize();
        s.look_at(c, target)?;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.delta += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        if let Some(xyz) = self.grid.take() {
            let (data, position, look) = grid(xyz);
            r.queue
                .write_buffer(&self.buffers.instance_data, 0, bytemuck::cast_slice(&data));
            Self::place(s, c, &mut self.walker, position, look)?;
        }
        // controls.update( timer.getDelta() ).
        let delta = std::mem::take(&mut self.delta);
        self.walker.update(s, c, delta)
    }
    /// The bind groups of a stage, resolving its declarations by name.
    fn bind<'a>(
        r: &Renderer,
        layout: impl Fn(u32) -> wgpu::BindGroupLayout,
        sources: &[&str],
        uniforms: &'a Uniforms,
        resources: &[(&str, wgpu::BindingResource<'a>)],
    ) -> Result<Vec<wgpu::BindGroup>> {
        groups(r, layout, sources, |name| match name {
            "object" => uniforms.object.as_ref().map(|b| b.as_entire_binding()),
            "render" => uniforms.render.as_ref().map(|b| b.as_entire_binding()),
            _ => resources
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, res)| res.clone()),
        })
    }
    /// createScreenBuffers(): the visibility buffers, sceneRT and the depth
    /// pyramid at the drawing-buffer size, with the bind groups that use them.
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let pixels = u64::from(size.0) * u64::from(size.1);
        let screen = [
            zeroed(r, "screen tri", pixels * 4),
            zeroed(r, "screen inst", pixels * 4),
        ];
        // The pyramid: level 0 at half resolution, each level the max of the
        // 2 × 2 below, filled with the far plane ( occluding nothing ).
        let (mut w, mut h, mut total) = (size.0.div_ceil(2), size.1.div_ceil(2), 0u32);
        let mut levels = vec![];
        while levels.len() < MAX_HZB_LEVELS {
            levels.push((total, w, h));
            total += w * h;
            if w == 1 && h == 1 {
                break;
            }
            w = w.div_ceil(2).max(1);
            h = h.div_ceil(2).max(1);
        }
        let hzb = storage(
            r,
            "hzb",
            bytemuck::cast_slice(&vec![1f32; total as usize]),
            false,
        );
        let mut table = [0f32; MAX_HZB_LEVELS * 4];
        for (i, (offset, w, h)) in levels.iter().enumerate() {
            table[i * 4..i * 4 + 3].copy_from_slice(&[*offset as f32, *w as f32, *h as f32]);
        }
        r.queue
            .write_buffer(&self.buffers.hzb_table, 0, bytemuck::cast_slice(&table));
        let color = target(r, size, HALF, 1);
        let depth = target(r, size, SCENE_DEPTH, 1);
        let present = target(r, size, HALF, 1);
        let b = &self.buffers;
        let [tri, inst] = &screen;
        let entire = wgpu::Buffer::as_entire_binding;
        let common: Vec<(&str, wgpu::BindingResource)> = vec![
            ("NodeBuffer_991", entire(&b.lod_offsets)),
            ("NodeBuffer_992", entire(&b.chunk_bounds)),
            ("NodeBuffer_993", entire(&b.vertices)),
            ("NodeBuffer_994", entire(&b.normals)),
            ("NodeBuffer_995", entire(&b.uvs)),
            ("NodeBuffer_996", entire(&b.indices)),
            ("NodeBuffer_997", entire(&b.meshlet_ids)),
            ("NodeBuffer_1001", entire(&b.instance_data)),
            ("NodeBuffer_1002", entire(&b.hzb_table)),
            ("NodeBuffer_1004", entire(tri)),
            ("NodeBuffer_1005", entire(tri)),
            ("NodeBuffer_1006", entire(inst)),
            ("NodeBuffer_1007", entire(inst)),
            ("NodeBuffer_1008", entire(&hzb)),
            ("NodeBuffer_1009", entire(&hzb)),
            ("NodeBuffer_1010", entire(&b.instance_world)),
            ("NodeBuffer_1011", entire(&b.instance_mvp)),
            ("NodeBuffer_1012", entire(&b.instance_world)),
            ("NodeBuffer_1013", entire(&b.instance_prev_world)),
            ("NodeBuffer_1014", entire(&b.work_queue_count)),
            ("NodeBuffer_1015", entire(&b.work_queue_count)),
            ("NodeBuffer_1016", entire(&b.dispatch)),
            ("NodeBuffer_1017", entire(&b.work_queue)),
            ("NodeBuffer_1018", entire(&b.hw_queue)),
            ("NodeBuffer_1019", entire(&b.hw_queue)),
            ("NodeBuffer_1020", entire(&b.hw_draw)),
            ("NodeBuffer_1023", entire(&b.frustum_planes)),
        ];
        let (p, st) = (&self.pipelines, &self.stages);
        let compute = |i: usize, source: &str, uniforms: &Uniforms| {
            Self::bind(
                r,
                |g| p.compute[i].get_bind_group_layout(g),
                &[source],
                uniforms,
                &common,
            )
        };
        let clear = compute(0, CLEAR, &st.clear)?;
        let frustum = compute(1, FRUSTUM, &st.frustum)?;
        let rasterize = compute(3, RASTERIZE, &st.rasterize)?;
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let mut hzb_groups = vec![];
        for (k, pipeline) in p.hzb.iter().enumerate() {
            let mut resources = common.clone();
            resources.push(("nodeUniform1", tex(&depth)));
            hzb_groups.push(Self::bind(
                r,
                |g| pipeline.get_bind_group_layout(g),
                &st.hzb[k].sources,
                &st.hzb[k],
                &resources,
            )?);
        }
        let background = Self::bind(
            r,
            |g| p.background.get_bind_group_layout(g),
            &[BACKGROUND.0, BACKGROUND.1],
            &st.background,
            &[
                ("nodeUniform8_sampler", sampler(&self.clamp)),
                ("nodeUniform8", tex(&self.pmrem.view)),
            ],
        )?;
        let [albedo, metal_rough, emissive, ao, normal] = &self.maps;
        // The texture names of each material, from its generated shader.
        let materials: [[(&str, &wgpu::TextureView); 7]; 2] = [
            [
                ("nodeUniform2", albedo),
                ("nodeUniform10", ao),
                ("nodeUniform11", metal_rough),
                ("nodeUniform13", emissive),
                ("nodeUniform14", &r.dfg),
                ("nodeUniform16", normal),
                ("nodeUniform22", &self.pmrem.view),
            ],
            [
                ("nodeUniform6", albedo),
                ("nodeUniform8", ao),
                ("nodeUniform9", metal_rough),
                ("nodeUniform10", emissive),
                ("nodeUniform11", &r.dfg),
                ("nodeUniform13", normal),
                ("nodeUniform21", &self.pmrem.view),
            ],
        ];
        let vis: [[(&str, &wgpu::TextureView); 4]; 2] = [
            [
                ("nodeUniform10", normal),
                ("nodeUniform11", metal_rough),
                ("nodeUniform12", ao),
                ("nodeUniform13", emissive),
            ],
            [
                ("nodeUniform7", normal),
                ("nodeUniform8", metal_rough),
                ("nodeUniform9", ao),
                ("nodeUniform10", emissive),
            ],
        ];
        let render_groups =
            |pipeline: &wgpu::RenderPipeline,
             (vs, fs): (&str, &str),
             uniforms: &Uniforms,
             resources: &[(String, wgpu::BindingResource)]| {
                let borrowed: Vec<(&str, wgpu::BindingResource)> = resources
                    .iter()
                    .map(|(n, res)| (n.as_str(), res.clone()))
                    .collect();
                Self::bind(
                    r,
                    |g| pipeline.get_bind_group_layout(g),
                    &[vs, fs],
                    uniforms,
                    &borrowed,
                )
            };
        let clamped = [&r.dfg, &self.pmrem.view];
        let shaded: [Vec<(String, wgpu::BindingResource)>; 2] = [
            named(
                &common,
                with_textures(&materials[0], &clamped, &self.clamp, &self.repeat),
            ),
            named(
                &common,
                with_textures(&materials[1], &clamped, &self.clamp, &self.repeat),
            ),
        ];
        let vis_resources: [Vec<(String, wgpu::BindingResource)>; 2] = [
            named(
                &common,
                with_textures(&vis[0], &[], &self.clamp, &self.repeat),
            ),
            named(
                &common,
                with_textures(&vis[1], &[], &self.clamp, &self.repeat),
            ),
        ];
        let plain = named(&common, vec![]);
        let resolve = [
            render_groups(&p.resolve[0], RESOLVE[0], &st.resolve[0], &shaded[0])?,
            render_groups(&p.resolve[1], RESOLVE[1], &st.resolve[1], &plain)?,
            render_groups(&p.resolve[2], RESOLVE[2], &st.resolve[2], &vis_resources[0])?,
        ];
        let hw = [
            render_groups(&p.hw[0], HW[0], &st.hw[0], &shaded[1])?,
            render_groups(&p.hw[1], HW[1], &st.hw[1], &plain)?,
            render_groups(&p.hw[2], HW[2], &st.hw[2], &vis_resources[1])?,
        ];
        let blit = render_groups(
            &p.blit,
            BLIT,
            &st.blit,
            &[
                ("nodeUniform0".into(), tex(&color)),
                ("nodeUniform0_sampler".into(), sampler(&self.clamp)),
            ],
        )?;
        if self
            .output
            .as_ref()
            .is_none_or(|(f, _)| *f != out.options.format)
        {
            let output = |shaders| {
                render_pipeline(
                    r,
                    "output",
                    shaders,
                    &[POSITION],
                    out.options.format,
                    None,
                    false,
                )
            };
            self.output = Some((out.options.format, [output(OUTPUT[0]), output(OUTPUT[1])]));
        }
        let (_, outputs) = self.output.as_ref().ok_or(Error::Invalid("output"))?;
        let output_groups = [0, 1].map(|i| {
            render_groups(
                &outputs[i],
                OUTPUT[i],
                &st.output[i],
                &[
                    ("nodeUniform0".into(), tex(&present)),
                    ("nodeUniform0_sampler".into(), sampler(&self.clamp)),
                ],
            )
        });
        let [o0, o1] = output_groups;
        let (w, h) = (f64::from(size.0), f64::from(size.1));
        st.clear
            .write(r, "objectStruct", &[("nodeUniform4", &[pixels as f64])])?;
        st.rasterize
            .write(r, "renderStruct", &[("nodeUniform5", &[w, h])])?;
        st.frustum.write(
            r,
            "renderStruct",
            &[
                ("nodeUniform16", &[self.time]),
                ("nodeUniform9", &[w, h]),
                ("nodeUniform13", &[w, h]),
            ],
        )?;
        let identity3 = m3(Matrix4::IDENTITY);
        for (k, uniforms) in st.hzb.iter().enumerate() {
            let (_, lw, lh) = levels[k.min(levels.len() - 1)];
            let count = f64::from(lw * lh);
            if k == 0 {
                uniforms.write(
                    r,
                    "objectStruct",
                    &[
                        ("nodeUniform2", &identity3),
                        ("nodeUniform4", &identity3),
                        ("nodeUniform5", &identity3),
                        ("nodeUniform6", &identity3),
                        ("nodeUniform8", &[count]),
                    ],
                )?;
                uniforms.write(r, "renderStruct", &[("nodeUniform3", &[w, h])])?;
            } else {
                uniforms.write(r, "objectStruct", &[("nodeUniform2", &[count])])?;
            }
        }
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            color,
            depth,
            present,
            _screen: screen,
            _hzb: hzb,
            levels,
            clear,
            frustum,
            rasterize,
            hzb: hzb_groups,
            background,
            resolve,
            hw,
            blit,
            output: [o0?, o1?],
            screen: RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                RenderTargetOptions {
                    samples: 0,
                    depth_buffer: false,
                    ..out.options.clone()
                },
            )?,
        });
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("rasterizer targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (mut projection, view) = (camera.projection_matrix()?, world.inverse());
        let gpu_projection = projection;
        if !self.rendered {
            // The first frame's camera still projects to WebGL's clip space.
            if let Camera::Perspective(p) = camera {
                let (n, f) = (p.near, p.far);
                projection.z_axis.z = -(f + n) / (f - n);
                projection.w_axis.z = -2. * f * n / (f - n);
            }
        }
        let screen = projection * view;
        let position = world.w_axis.truncate();
        let (prev_screen, prev_position) = self.previous.unwrap_or((screen, position));
        self.previous = Some((screen, position));
        // frustum.setFromProjectionMatrix( projScreenMatrix ) in the WebGL
        // coordinate system, as the page calls it.
        let me = screen.to_cols_array();
        let row = |i: usize| [me[i], me[i + 4], me[i + 8], me[i + 12]];
        let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
        let combine = |a: [f64; 4], b: [f64; 4], sign: f64| {
            std::array::from_fn::<f64, 4, _>(|k| a[k] + sign * b[k])
        };
        let planes = [
            combine(r3, r0, -1.),
            combine(r3, r0, 1.),
            combine(r3, r1, 1.),
            combine(r3, r1, -1.),
            combine(r3, r2, -1.),
            combine(r3, r2, 1.),
        ];
        let mut plane_data = [0f32; 24];
        for (i, p) in planes.iter().enumerate() {
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            for k in 0..4 {
                plane_data[i * 4 + k] = (p[k] / length) as f32;
            }
        }
        let b = &self.buffers;
        r.queue
            .write_buffer(&b.frustum_planes, 0, bytemuck::cast_slice(&plane_data));
        let st = &self.stages;
        let (w, h) = (f64::from(t.width), f64::from(t.height));
        let [output, raster, _, bias, threshold, speed] = self.params;
        let p3 = |v: Vector3| [v.x, v.y, v.z];
        st.frustum.write(
            r,
            "renderStruct",
            &[
                ("nodeUniform16", &[self.time]),
                ("nodeUniform9", &[w, h]),
                ("nodeUniform13", &[w, h]),
            ],
        )?;
        st.frustum.write(
            r,
            "objectStruct",
            &[
                ("nodeUniform4", &p3(prev_position)),
                ("nodeUniform5", &m4(prev_screen)),
                ("nodeUniform8", &[projection.y_axis.y]),
                ("nodeUniform10", &[t.levels.len() as f64]),
                ("nodeUniform11", &[bias]),
                ("nodeUniform12", &p3(position)),
                ("nodeUniform14", &[threshold]),
                ("nodeUniform17", &[speed]),
                ("nodeUniform22", &m4(screen)),
                ("nodeUniform23", &[INSTANCES as f64]),
            ],
        )?;
        let camera_world = m4(world);
        let (proj, view_values) = (m4(gpu_projection), m4(view));
        // The material: 0 shaded, 1 meshlet debug, 2 channel view.
        let material = match output as usize {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        let mode = match output as usize {
            0 | 1 => 0.,
            m => (m - 1) as f64,
        };
        let identity = m4(Matrix4::IDENTITY);
        let env = |maxmip: &str, rot: &str, tx: &str, ty: &str, intensity: &str, opacity: &str| {
            [
                (maxmip.to_string(), vec![9.]),
                (rot.to_string(), identity.clone()),
                (tx.to_string(), vec![1. / 1536.]),
                (ty.to_string(), vec![1. / 2048.]),
                (intensity.to_string(), vec![1.]),
                (opacity.to_string(), vec![1.]),
            ]
        };
        let write = |u: &Uniforms, name: &str, values: &[(String, Vec<f64>)]| {
            let v: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
            u.write(r, name, &v)
        };
        let s_ = |n: &str, v: Vec<f64>| (n.to_string(), v);
        match material {
            0 => {
                let mut object = vec![s_("nodeUniform5", m4(screen))];
                object.extend(env(
                    "nodeUniform17",
                    "nodeUniform18",
                    "nodeUniform20",
                    "nodeUniform21",
                    "nodeUniform23",
                    "nodeUniform9",
                ));
                write(&st.resolve[0], "objectStruct", &object)?;
                write(
                    &st.resolve[0],
                    "renderStruct",
                    &[
                        s_("cameraViewMatrix", view_values.clone()),
                        s_("nodeUniform1", vec![w, h]),
                        s_("cameraWorldMatrix", camera_world.clone()),
                    ],
                )?;
                let mut object = vec![s_("nodeUniform15", identity.clone())];
                object.extend(env(
                    "nodeUniform16",
                    "nodeUniform17",
                    "nodeUniform19",
                    "nodeUniform20",
                    "nodeUniform22",
                    "nodeUniform7",
                ));
                write(&st.hw[0], "objectStruct", &object)?;
                write(
                    &st.hw[0],
                    "renderStruct",
                    &[
                        s_("cameraProjectionMatrix", proj.clone()),
                        s_("cameraViewMatrix", view_values.clone()),
                        s_("cameraWorldMatrix", camera_world.clone()),
                    ],
                )?;
            }
            1 => {
                write(
                    &st.resolve[1],
                    "renderStruct",
                    &[s_("nodeUniform1", vec![w, h])],
                )?;
                write(
                    &st.hw[1],
                    "objectStruct",
                    &[s_("nodeUniform9", identity.clone())],
                )?;
                write(
                    &st.hw[1],
                    "renderStruct",
                    &[
                        s_("cameraProjectionMatrix", proj.clone()),
                        s_("cameraViewMatrix", view_values.clone()),
                    ],
                )?;
            }
            _ => {
                write(
                    &st.resolve[2],
                    "renderStruct",
                    &[s_("nodeUniform1", vec![w, h])],
                )?;
                write(
                    &st.resolve[2],
                    "objectStruct",
                    &[
                        s_("nodeUniform2", vec![mode]),
                        s_("nodeUniform7", m4(screen)),
                    ],
                )?;
                write(
                    &st.hw[2],
                    "objectStruct",
                    &[
                        s_("nodeUniform6", vec![mode]),
                        s_("nodeUniform13", identity.clone()),
                    ],
                )?;
                write(
                    &st.hw[2],
                    "renderStruct",
                    &[
                        s_("cameraProjectionMatrix", proj.clone()),
                        s_("cameraViewMatrix", view_values.clone()),
                    ],
                )?;
            }
        }
        write(
            &st.background,
            "renderStruct",
            &[
                s_("nodeUniform0", vec![0.5]),
                s_("nodeUniform9", vec![1.]),
                s_("nodeUniform3", identity.clone()),
                s_("cameraProjectionMatrix", proj.clone()),
                s_("cameraViewMatrix", view_values.clone()),
            ],
        )?;
        // The output pass: ACES in the Default output, none otherwise.
        let tone = usize::from(output != 0.);
        let flat = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
        ];
        if tone == 0 {
            write(
                &st.output[0],
                "renderStruct",
                &[
                    s_("cameraProjectionMatrix", flat.to_vec()),
                    s_("cameraViewMatrix", identity.clone()),
                    s_("nodeUniform1", vec![w, h]),
                    s_("nodeUniform2", vec![1.]),
                ],
            )?;
        } else {
            write(
                &st.output[1],
                "renderStruct",
                &[
                    s_("cameraProjectionMatrix", flat.to_vec()),
                    s_("cameraViewMatrix", identity.clone()),
                    s_("nodeUniform1", vec![w, h]),
                ],
            )?;
        }
        let p = &self.pipelines;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            let groups = |pass: &mut wgpu::ComputePass, g: &[wgpu::BindGroup]| {
                for (i, g) in g.iter().enumerate() {
                    pass.set_bind_group(i as u32, g, &[]);
                }
            };
            let pixels = t.width * t.height;
            pass.set_pipeline(&p.compute[0]);
            groups(&mut pass, &t.clear);
            pass.dispatch_workgroups(pixels.div_ceil(256), 1, 1);
            pass.set_pipeline(&p.compute[1]);
            groups(&mut pass, &t.frustum);
            pass.dispatch_workgroups((INSTANCES as u32).div_ceil(64), 1, 1);
            pass.set_pipeline(&p.compute[2]);
            groups(&mut pass, &self.dispatch_groups);
            pass.dispatch_workgroups(1, 1, 1);
            pass.set_pipeline(&p.compute[3]);
            groups(&mut pass, &t.rasterize);
            pass.dispatch_workgroups_indirect(&b.dispatch, 0);
            pass.set_pipeline(&p.compute[4]);
            groups(&mut pass, &self.hw_args_groups);
            pass.dispatch_workgroups(1, 1, 1);
        }
        {
            // renderer.render( scene, camera ) into sceneRT.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&p.background);
            for (i, g) in t.background.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            pass.set_vertex_buffer(0, b.sphere.0.slice(..));
            pass.set_vertex_buffer(1, b.sphere.1.slice(..));
            pass.set_index_buffer(b.sphere.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..b.sphere.3, 0, 0..1);
            if raster != 1. {
                pass.set_pipeline(&p.resolve[material]);
                for (i, g) in t.resolve[material].iter().enumerate() {
                    pass.set_bind_group(i as u32, g, &[]);
                }
                pass.set_vertex_buffer(0, b.fullscreen.slice(..));
                pass.draw(0..3, 0..1);
            }
            if raster != 0. {
                pass.set_pipeline(&p.hw[material]);
                for (i, g) in t.hw[material].iter().enumerate() {
                    pass.set_bind_group(i as u32, g, &[]);
                }
                pass.set_vertex_buffer(0, b.hw_positions.slice(..));
                pass.draw_indirect(&b.hw_draw, 0);
            }
        }
        {
            // The depth pyramid for next frame's occlusion culling.
            let mut pass = encoder.begin_compute_pass(&Default::default());
            for (k, (_, lw, lh)) in t.levels.iter().enumerate() {
                pass.set_pipeline(&p.hzb[k]);
                for (i, g) in t.hzb[k].iter().enumerate() {
                    pass.set_bind_group(i as u32, g, &[]);
                }
                pass.dispatch_workgroups((lw * lh).div_ceil(64), 1, 1);
            }
        }
        {
            // blitQuad.render( renderer ) into the renderer's output buffer.
            let mut pass = fullscreen_pass(&mut encoder, &t.present);
            pass.set_pipeline(&p.blit);
            for (i, g) in t.blit.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            pass.set_vertex_buffer(0, b.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        {
            let (_, outputs) = self.output.as_ref().ok_or(Error::Invalid("output"))?;
            let mut pass = fullscreen_pass(&mut encoder, &t.screen.view);
            pass.set_pipeline(&outputs[tone]);
            for (i, g) in t.output[tone].iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            pass.set_vertex_buffer(0, b.fullscreen.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        self.rendered = true;
        Ok(true)
    }
    fn make_dispatch_groups(&self, r: &Renderer) -> Result<Vec<wgpu::BindGroup>> {
        let b = &self.buffers;
        Self::bind(
            r,
            |g| self.pipelines.compute[2].get_bind_group_layout(g),
            &[DISPATCH],
            &self.stages.dispatch,
            &[
                ("NodeBuffer_1016", b.dispatch.as_entire_binding()),
                ("NodeBuffer_1015", b.work_queue_count.as_entire_binding()),
            ],
        )
    }
    fn make_hw_args_groups(&self, r: &Renderer) -> Result<Vec<wgpu::BindGroup>> {
        let b = &self.buffers;
        Self::bind(
            r,
            |g| self.pipelines.compute[4].get_bind_group_layout(g),
            &[HW_ARGS],
            &self.stages.hw_args,
            &[
                ("NodeBuffer_1018", b.hw_queue.as_entire_binding()),
                ("NodeBuffer_1020", b.hw_draw.as_entire_binding()),
            ],
        )
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.walker.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.walker.key(code, down);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    /// Output, Rasterizer, Grid, Occlusion Bias, LOD Threshold and Animation
    /// Speed. Changing the grid rewrites the instance layout and places the
    /// camera, as updateGrid() does.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let param = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("rasterizer parameter"))?;
        if index == 2 && *param != value as f64 {
            self.grid = Some(value > 0.5);
        }
        *param = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn fullscreen_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("present"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        ..Default::default()
    })
}
const POSITION_ATTRIBUTE: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 0,
}];
const POSITION: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: 12,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &POSITION_ATTRIBUTE,
};
/// Each texture by name with its sampler: the maps repeat with mipmaps; the
/// DFG table and the PMREM clamp.
fn with_textures<'a>(
    textures: &[(&str, &'a wgpu::TextureView)],
    clamped: &[&wgpu::TextureView],
    clamp: &'a wgpu::Sampler,
    repeat: &'a wgpu::Sampler,
) -> Vec<(String, wgpu::BindingResource<'a>)> {
    let mut out = vec![];
    for (name, view) in textures {
        let clamps = clamped.iter().any(|c| std::ptr::eq(*c, *view));
        out.push((name.to_string(), wgpu::BindingResource::TextureView(view)));
        out.push((
            format!("{name}_sampler"),
            wgpu::BindingResource::Sampler(if clamps { clamp } else { repeat }),
        ));
    }
    out
}
fn named<'a>(
    common: &[(&str, wgpu::BindingResource<'a>)],
    extra: Vec<(String, wgpu::BindingResource<'a>)>,
) -> Vec<(String, wgpu::BindingResource<'a>)> {
    let mut v: Vec<(String, wgpu::BindingResource)> = common
        .iter()
        .map(|(n, res)| (n.to_string(), res.clone()))
        .collect();
    v.extend(extra);
    v
}
