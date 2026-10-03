//! webgpu_volume_fire: a GPU fluid fire rising from a glowing teapot. Each
//! simulation step ( 1 / 120 s × the simulation speed, as many as the page's
//! accumulator allows ) runs the page's compute kernels over the 100 × 100 ×
//! 200 half-float voxel grids: velocity advection with buoyancy, curl-noise
//! turbulence and the teapot's wind, the divergence, two Jacobi pressure
//! iterations, the projection, the dye advection and the emission from the
//! teapot's vertices, ping-ponging the dye grids. The curl noise is computed
//! once at load. Each frame then draws the spot light's colored shadow map from
//! the volume's shadow caster, the scene ( the lava teapot and the floor lit by
//! the fire's point light, and the caster's colorless pass ), the half
//! resolution ray-marched volume, its Gaussian denoise, BloomNode ( a high
//! pass, five H / V levels and the composite ) and the output with ACES Filmic
//! tone mapping. Every stage runs the WGSL three.js r186 generates for the page
//! ( in `volume_fire/`, with the bloom levels shared with `volume_caustics/` ).
//! DragControls move the teapot with a CPU raycast against its triangles, as
//! the original's does.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::trackball_sprites::noise;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, raycast::Raycaster, render_target::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW_SIZE: u32 = 1024;
const GRID: (u32, u32, u32) = (100, 100, 200);
const CELLS: u32 = GRID.0 * GRID.1 * GRID.2;
const VOLUME: [f64; 3] = [12., 12., 24.];
/// BloomNode's mip levels.
const LEVELS: usize = 5;
/// The volume boxes' center height.
const VOLUME_Y: f64 = 6.4;
const FLOOR_Y: f64 = 0.8;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("volume_fire/", $name, ".wgsl"))
    };
}
macro_rules! shared {
    ($name:literal) => {
        include_str!(concat!("volume_caustics/", $name, ".wgsl"))
    };
}
fn sized(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: pack(source, name, &[])?.len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
fn texture2d(r: &Renderer, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("volume fire target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
fn half_size(size: (u32, u32)) -> (u32, u32) {
    (
        ((size.0 as f64 / 2.).round() as u32).max(1),
        ((size.1 as f64 / 2.).round() as u32).max(1),
    )
}
/// A triangle pipeline with the material's face, depth, blending and color mask.
#[allow(clippy::too_many_arguments)]
fn pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    depth: Option<(wgpu::CompareFunction, bool)>,
    cw: bool,
    blend: Option<wgpu::BlendState>,
    write_mask: wgpu::ColorWrites,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let (vs, fs) = (module(vs), module(fs));
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vs,
                entry_point: Some("main"),
                buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs,
                entry_point: Some("main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend,
                    write_mask,
                })],
                compilation_options: Default::default(),
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
            depth_stencil: depth.map(|(depth_compare, depth_write_enabled)| {
                wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled,
                    depth_compare,
                    stencil: Default::default(),
                    bias: Default::default(),
                }
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn compute(r: &Renderer, label: &str, source: &str) -> wgpu::ComputePipeline {
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
const P0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const P1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
const P1X4: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x4];
const UV0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
fn layout(stride: u64, attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout<'_> {
    wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    }
}
/// Vertex buffers of a mesh: position, normal and index.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
}
type Draw = (wgpu::RenderPipeline, [wgpu::BindGroup; 2]);
/// The page's GUI values, in the fixture's control order.
#[derive(Clone)]
struct Params {
    simulate: bool,
    sim_speed: f64,
    resolution: f64,
    steps: f64,
    denoise: bool,
    denoise_strength: f64,
    bloom: bool,
    bloom_strength: f64,
    bloom_radius: f64,
    bloom_threshold: f64,
    glow_spread: f64,
    fire_hue: f64,
    saturation: f64,
    /// Start, mid and end colors.
    colors: [u32; 3],
    emit_temperature: f64,
    emit_density: f64,
    motion_boost: f64,
    wind_strength: f64,
    teapot_emissive: f64,
    asymmetry: f64,
    powder: f64,
    multi_scattering: f64,
    shadow_absorption: f64,
    shadow_ambient: f64,
    buoyancy: f64,
    vel_damping: f64,
    fire_lifespan: f64,
    smoke_lifespan: f64,
    turbulence: f64,
    turbulence_decay: f64,
    turb_frequency: f64,
    key_intensity: f64,
    light_smoke: f64,
    light_near: f64,
    light_far: f64,
    light_reflection: f64,
    projection_radius: f64,
    projection_frequency: f64,
    projection_noise_fade: f64,
    projection_center_fade: f64,
}
impl Default for Params {
    fn default() -> Self {
        Self {
            simulate: true,
            sim_speed: 1.2,
            resolution: 0.5,
            steps: 16.,
            denoise: true,
            denoise_strength: 0.5,
            bloom: true,
            bloom_strength: 0.1,
            bloom_radius: 1.,
            bloom_threshold: 0.5,
            glow_spread: 5.,
            fire_hue: 0.,
            saturation: 1.1,
            colors: [0xffe68c, 0xff7305, 0xff0000],
            emit_temperature: 5.5,
            emit_density: 7.,
            motion_boost: 0.25,
            wind_strength: 6.5,
            teapot_emissive: 0.2,
            asymmetry: 0.,
            powder: 0.59,
            multi_scattering: 1.,
            shadow_absorption: 2.,
            shadow_ambient: 0.5,
            buoyancy: 3.,
            vel_damping: 0.25,
            fire_lifespan: 1.3,
            smoke_lifespan: 3.5,
            turbulence: 3.2,
            turbulence_decay: 0.1,
            turb_frequency: 10.,
            key_intensity: 1000.,
            light_smoke: 2.,
            light_near: 10.,
            light_far: 15.,
            light_reflection: 10.,
            projection_radius: 20.,
            projection_frequency: 0.2,
            projection_noise_fade: 17.,
            projection_center_fade: 3.25,
        }
    }
}
/// updateTemporalUniforms( time ).
#[derive(Clone, Copy)]
struct Temporal {
    time: f64,
    flame_height: f64,
    sway: Vector3,
    flicker: f64,
    color_noise: f64,
    rotation: f64,
}
impl Temporal {
    fn at(time: f64) -> Self {
        Self {
            time: time % 1000.,
            flame_height: 3.5 + noise(0., time * 2.5, 0.) * 0.8,
            sway: Vector3::new(
                noise(time * 3.5, 0., 0.) * 0.4,
                0.,
                noise(0., 0., time * 3.5) * 0.4,
            ),
            flicker: noise(0., time * 0.8, 0.) * 0.12 + noise(0., time * 15., 0.) * 0.06 + 0.82,
            color_noise: noise(time * 5., time * 5., 0.) * 0.08,
            rotation: time * 0.25,
        }
    }
}
/// Per dye parity: the kernels' bind groups.
struct Kernels {
    advect_velocity: (wgpu::ComputePipeline, [wgpu::BindGroup; 2]),
    divergence: (wgpu::ComputePipeline, wgpu::BindGroup),
    jacobi: [(wgpu::ComputePipeline, wgpu::BindGroup); 2],
    project: (wgpu::ComputePipeline, wgpu::BindGroup),
    advect_dye: (wgpu::ComputePipeline, [wgpu::BindGroup; 2]),
    emit: (wgpu::ComputePipeline, [wgpu::BindGroup; 2]),
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    resolution: f64,
    scene: wgpu::TextureView,
    depth: wgpu::TextureView,
    volume_size: (u32, u32),
    volume_target: wgpu::TextureView,
    blur_targets: [wgpu::TextureView; 2],
    /// The horizontal and vertical denoise draws.
    blurs: [(wgpu::RenderPipeline, wgpu::BindGroup); 2],
    bright: wgpu::TextureView,
    /// Per level: the horizontal and vertical targets.
    levels: Vec<(wgpu::TextureView, wgpu::TextureView, (u32, u32))>,
    /// Denoised and raw volume inputs.
    high: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Per level: the horizontal and vertical draws with their uniforms.
    bloom_blurs: Vec<[(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Buffer); 2]>,
    composite: (wgpu::RenderPipeline, wgpu::BindGroup),
    screen: RenderTarget,
    /// Denoised or raw volume, with or without bloom.
    output: (
        wgpu::RenderPipeline,
        wgpu::BindGroup,
        [[wgpu::BindGroup; 2]; 2],
    ),
}
pub(super) struct Demo {
    controls: Controls,
    params: Params,
    /// The gallery clock, the last animate()'s clock ( ms ) and requested frames.
    time: f64,
    last: f64,
    frame_id: u32,
    /// A requested or animated frame runs animate(); re-renders for input do not.
    advance: bool,
    simulation_time: f64,
    accumulator: f64,
    temporal: Temporal,
    /// Steps queued by prepare(): their temporal uniforms.
    steps: Vec<Temporal>,
    /// The dye grid read this step ( 0: A ).
    parity: usize,
    teapot_position: Vector3,
    previous_position: Vector3,
    teapot_speed: f64,
    teapot_velocity: Vector3,
    point_distance: f64,
    /// The teapot's object uniforms are written when its matrix changes, as
    /// three refreshes a static material's object group.
    teapot_written: Option<Matrix4>,
    /// Pointer events, the DragControls selection ( plane and offset ) and the pointer.
    queue: Vec<(u32, f64, f64)>,
    pointer: Vector2,
    drag: Option<(Plane, Vector3)>,
    selected: bool,
    teapot_triangles: Vec<[Vector3; 3]>,
    teapot_min_y: f64,
    teapot: Mesh,
    teapot_storage: wgpu::Buffer,
    floor: Mesh,
    volume_box: Mesh,
    kernels: Kernels,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    shadow: (wgpu::RenderPipeline, [[wgpu::BindGroup; 2]; 2]),
    teapot_draw: Draw,
    floor_draw: Draw,
    caster: Draw,
    volume: (wgpu::RenderPipeline, [[wgpu::BindGroup; 2]; 2]),
    advect_velocity_object: wgpu::Buffer,
    advect_dye_object: wgpu::Buffer,
    emit_object: wgpu::Buffer,
    shadow_render: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    teapot_render: wgpu::Buffer,
    teapot_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    caster_render: wgpu::Buffer,
    caster_object: wgpu::Buffer,
    volume_render: wgpu::Buffer,
    volume_object: wgpu::Buffer,
    blur_objects: [wgpu::Buffer; 2],
    high_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    tints: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    black: wgpu::TextureView,
    quad_uv: wgpu::Buffer,
    linear: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The spot light's shadow camera: 2 × angle, near 1, far 60.
fn spot() -> (Vector3, Vector3, Matrix4, Matrix4) {
    let position = Vector3::new(
        -3. * (VOLUME[0] / 8.),
        6. * (VOLUME[1] / 8.) + VOLUME_Y,
        3. * (VOLUME[2] / 8.),
    );
    let target = Vector3::new(1., 0., 0.);
    let (near, far) = (1., 20. * (VOLUME[2] / 8.));
    let f = 1. / (PI / 5.).tan();
    let projection = Matrix4::from_cols_array(&[
        f,
        0.,
        0.,
        0.,
        0.,
        f,
        0.,
        0.,
        0.,
        0.,
        -far / (far - near),
        -1.,
        0.,
        0.,
        -far * near / (far - near),
        0.,
    ]);
    (
        position,
        target,
        projection,
        Matrix4::look_at_rh(position, target, Vector3::Y),
    )
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(14., 5.5, 4.4);
        let mut controls = Controls::new(None, (2., 40.), PI, true);
        controls.set_target(Vector3::new(0., 3.6, 0.));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let attribute = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("volume fire attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let mesh = |g: &BufferGeometry| -> Result<Mesh> {
            let index = g.index.clone().ok_or(Error::Invalid("volume fire index"))?;
            Ok(Mesh {
                positions: init(
                    "volume fire positions",
                    bytemuck::cast_slice(&attribute(g, "position")?),
                    vertex,
                ),
                normals: init(
                    "volume fire normals",
                    bytemuck::cast_slice(&attribute(g, "normal")?),
                    vertex,
                ),
                index: init(
                    "volume fire index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
            })
        };
        // TeapotGeometry( 0.8, 28 ): its positions also feed the emission
        // kernel as a padded vec3 storage buffer ( and the teapot's draw ).
        let teapot_geometry =
            super::models_modifiers::teapot(0.8, 28, true, true, true, true, true)?;
        let positions = attribute(&teapot_geometry, "position")?;
        let points: Vec<Vector3> = positions
            .chunks(3)
            .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        let teapot_min_y = points.iter().map(|p| p.y).fold(f64::MAX, f64::min);
        let padded: Vec<f32> = positions
            .chunks(3)
            .flat_map(|p| [p[0], p[1], p[2], 0.])
            .collect();
        let teapot_storage = init(
            "volume fire teapot vertices",
            bytemuck::cast_slice(&padded),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
        );
        let teapot_index = teapot_geometry
            .index
            .clone()
            .ok_or(Error::Invalid("volume fire teapot index"))?;
        let teapot_triangles = teapot_index
            .chunks(3)
            .map(|t| {
                [
                    points[t[0] as usize],
                    points[t[1] as usize],
                    points[t[2] as usize],
                ]
            })
            .collect();
        let teapot = mesh(&teapot_geometry)?;
        let floor = mesh(&PlaneGeometry::build(80., 80., 1, 1)?)?;
        let volume_box = mesh(&BoxGeometry::build(VOLUME[0], VOLUME[1], VOLUME[2])?)?;
        let sampler = |address| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                address_mode_w: address,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            })
        };
        let linear = sampler(wgpu::AddressMode::ClampToEdge);
        let repeat = sampler(wgpu::AddressMode::Repeat);
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        // Storage3DTexture( 100, 100, 200 ): rgba16float, storage-writable and filterable.
        let grid = || {
            view(&r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("volume fire grid"),
                size: wgpu::Extent3d {
                    width: GRID.0,
                    height: GRID.1,
                    depth_or_array_layers: GRID.2,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format: HALF,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }))
        };
        let velocity = [grid(), grid()];
        let dye = [grid(), grid()];
        let divergence = grid();
        let pressure = [grid(), grid()];
        let curl = grid();
        let sampler_res = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let cells = |source: &str, name: &str| -> Result<Vec<u8>> {
            pack(source, "objectStruct", &[(name, &[CELLS as f64])])
        };
        let uniform_init = |label, data: Vec<u8>| init(label, &data, wgpu::BufferUsages::UNIFORM);
        // The curl noise, once at load.
        {
            let p = compute(r, "volume fire curl noise", wgsl!("curl_cs"));
            let object = uniform_init(
                "curl noise",
                pack(
                    wgsl!("curl_cs"),
                    "objectStruct",
                    &[("nodeUniform1", &[10.]), ("nodeUniform2", &[CELLS as f64])],
                )?,
            );
            let group = bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, tex(&curl)), (1, object.as_entire_binding())],
            );
            let mut encoder = r.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&p);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups(CELLS.div_ceil(64), 1, 1);
            }
            r.queue.submit([encoder.finish()]);
        }
        let advect_velocity_object = sized(
            r,
            "advect velocity",
            wgsl!("advect_velocity_cs"),
            "objectStruct",
        )?;
        let advect_dye_object = sized(r, "advect dye", wgsl!("advect_dye_cs"), "objectStruct")?;
        let emit_object = sized(r, "emit teapot", wgsl!("emit_cs"), "objectStruct")?;
        let kernels = {
            let advect_velocity = compute(r, "advect velocity", wgsl!("advect_velocity_cs"));
            let advect_velocity_groups = [0, 1].map(|k| {
                bind(
                    r,
                    advect_velocity.get_bind_group_layout(0),
                    &[
                        (0, sampler_res(&linear)),
                        (1, tex(&velocity[0])),
                        (2, advect_velocity_object.as_entire_binding()),
                        (3, sampler_res(&linear)),
                        (4, tex(&dye[k])),
                        (5, sampler_res(&repeat)),
                        (6, tex(&curl)),
                        (7, tex(&velocity[1])),
                    ],
                )
            });
            let divergence_pipeline = compute(r, "divergence", wgsl!("divergence_cs"));
            let divergence_object =
                uniform_init("divergence", cells(wgsl!("divergence_cs"), "nodeUniform2")?);
            let divergence_group = bind(
                r,
                divergence_pipeline.get_bind_group_layout(0),
                &[
                    (0, tex(&divergence)),
                    (1, sampler_res(&linear)),
                    (2, tex(&velocity[1])),
                    (3, divergence_object.as_entire_binding()),
                ],
            );
            // jacobiAB reads pressure A and writes B; jacobiBA the reverse.
            let jacobi = [
                (wgsl!("jacobi_ab_cs"), 0usize, "jacobiAB"),
                (wgsl!("jacobi_ba_cs"), 1, "jacobiBA"),
            ]
            .map(|(source, read, label)| -> Result<_> {
                let p = compute(r, label, source);
                let object = uniform_init(label, cells(source, "nodeUniform3")?);
                let group = bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, tex(&pressure[1 - read])),
                        (1, sampler_res(&linear)),
                        (2, tex(&pressure[read])),
                        (3, sampler_res(&linear)),
                        (4, tex(&divergence)),
                        (5, object.as_entire_binding()),
                    ],
                );
                Ok((p, group))
            });
            let [ab, ba] = jacobi;
            let project = compute(r, "project", wgsl!("project_cs"));
            let project_object =
                uniform_init("project", cells(wgsl!("project_cs"), "nodeUniform3")?);
            let project_group = bind(
                r,
                project.get_bind_group_layout(0),
                &[
                    (0, tex(&velocity[0])),
                    (1, sampler_res(&linear)),
                    (2, tex(&velocity[1])),
                    (3, sampler_res(&linear)),
                    (4, tex(&pressure[0])),
                    (5, project_object.as_entire_binding()),
                ],
            );
            let advect_dye = compute(r, "advect dye", wgsl!("advect_dye_cs"));
            let advect_dye_groups = [0, 1].map(|k| {
                bind(
                    r,
                    advect_dye.get_bind_group_layout(0),
                    &[
                        (0, sampler_res(&linear)),
                        (1, tex(&dye[k])),
                        (2, sampler_res(&linear)),
                        (3, tex(&velocity[0])),
                        (4, advect_dye_object.as_entire_binding()),
                        (5, tex(&dye[1 - k])),
                    ],
                )
            });
            let emit = compute(r, "emit teapot", wgsl!("emit_cs"));
            let emit_groups = [0, 1].map(|k| {
                bind(
                    r,
                    emit.get_bind_group_layout(0),
                    &[
                        (0, emit_object.as_entire_binding()),
                        (1, teapot_storage.as_entire_binding()),
                        (2, tex(&dye[1 - k])),
                        (3, sampler_res(&linear)),
                        (4, tex(&dye[k])),
                    ],
                )
            });
            Kernels {
                advect_velocity: (advect_velocity, advect_velocity_groups),
                divergence: (divergence_pipeline, divergence_group),
                jacobi: [ab?, ba?],
                project: (project, project_group),
                advect_dye: (advect_dye, advect_dye_groups),
                emit: (emit, emit_groups),
            }
        };
        let shadow_color = view(&texture2d(
            r,
            (SHADOW_SIZE, SHADOW_SIZE),
            wgpu::TextureFormat::Rgba8Unorm,
        ));
        let shadow_depth = view(&texture2d(r, (SHADOW_SIZE, SHADOW_SIZE), DEPTH));
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let always = Some((wgpu::CompareFunction::Always, false));
        let all = wgpu::ColorWrites::ALL;
        // The volume's shadow caster in the spot shadow ( front faces ).
        let shadow_render = sized(r, "shadow render", wgsl!("shadow_fs"), "renderStruct")?;
        let shadow_object = sized(r, "shadow object", wgsl!("shadow_fs"), "objectStruct")?;
        let shadow_pipeline = pipeline(
            r,
            "volume fire shadow",
            (wgsl!("shadow_vs"), wgsl!("shadow_fs")),
            &[layout(12, &P0)],
            wgpu::TextureFormat::Rgba8Unorm,
            less,
            false,
            None,
            all,
        );
        // Per dye parity: the groups with the dye read by the next step.
        let volume_groups = |p: &wgpu::RenderPipeline,
                             render: &wgpu::Buffer,
                             object: &wgpu::Buffer,
                             shadowed: bool|
         -> [[wgpu::BindGroup; 2]; 2] {
            [0, 1].map(|k| {
                let mut entries = vec![(0, object.as_entire_binding())];
                let base = if shadowed {
                    entries.extend([
                        (1, sampler_res(&linear)),
                        (2, tex(&shadow_color)),
                        (3, sampler_res(&compare)),
                        (4, tex(&shadow_depth)),
                    ]);
                    5
                } else {
                    1
                };
                entries.extend([
                    (base, sampler_res(&linear)),
                    (base + 1, tex(&velocity[0])),
                    (base + 2, sampler_res(&linear)),
                    (base + 3, tex(&dye[k])),
                ]);
                [
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), &entries),
                ]
            })
        };
        let shadow_groups = volume_groups(&shadow_pipeline, &shadow_render, &shadow_object, false);
        let lit_groups = |p: &wgpu::RenderPipeline,
                          render: &wgpu::Buffer,
                          object: &wgpu::Buffer,
                          dfg: bool|
         -> [wgpu::BindGroup; 2] {
            let mut entries = vec![(0, object.as_entire_binding())];
            let base = if dfg {
                entries.extend([(1, sampler_res(&linear)), (2, tex(&r.dfg))]);
                3
            } else {
                1
            };
            entries.extend([
                (base, sampler_res(&linear)),
                (base + 1, tex(&shadow_color)),
                (base + 2, sampler_res(&compare)),
                (base + 3, tex(&shadow_depth)),
            ]);
            [
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), &entries),
            ]
        };
        let teapot_render = sized(r, "teapot render", wgsl!("teapot_fs"), "renderStruct")?;
        let teapot_object = sized(r, "teapot object", wgsl!("teapot_fs"), "objectStruct")?;
        let teapot_pipeline = pipeline(
            r,
            "volume fire teapot",
            (wgsl!("teapot_vs"), wgsl!("teapot_fs")),
            &[layout(12, &P0), layout(16, &P1X4)],
            HALF,
            less,
            false,
            None,
            all,
        );
        let teapot_groups = lit_groups(&teapot_pipeline, &teapot_render, &teapot_object, true);
        let floor_render = sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?;
        let floor_object = sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?;
        let floor_pipeline = pipeline(
            r,
            "volume fire floor",
            (wgsl!("floor_vs"), wgsl!("floor_fs")),
            &[layout(12, &P0), layout(12, &P1)],
            HALF,
            less,
            false,
            None,
            all,
        );
        let floor_groups = lit_groups(&floor_pipeline, &floor_render, &floor_object, true);
        // The shadow caster's main-pass draw: back faces, no color writes.
        let caster_render = sized(r, "caster render", wgsl!("caster_fs"), "renderStruct")?;
        let caster_object = sized(r, "caster object", wgsl!("caster_fs"), "objectStruct")?;
        let caster_pipeline = pipeline(
            r,
            "volume fire caster",
            (wgsl!("caster_vs"), wgsl!("caster_fs")),
            &[layout(12, &P0), layout(12, &P1)],
            HALF,
            always,
            true,
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Zero,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            wgpu::ColorWrites::empty(),
        );
        let caster_groups = lit_groups(&caster_pipeline, &caster_render, &caster_object, false);
        // The ray-marched volume: back faces, additive.
        let volume_render = sized(r, "volume render", wgsl!("volume_fs"), "renderStruct")?;
        let volume_object = sized(r, "volume object", wgsl!("volume_fs"), "objectStruct")?;
        let volume_pipeline = pipeline(
            r,
            "volume fire volume",
            (wgsl!("volume_vs"), wgsl!("volume_fs")),
            &[layout(12, &P0), layout(12, &P1)],
            HALF,
            None,
            true,
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            all,
        );
        let volume = volume_groups(&volume_pipeline, &volume_render, &volume_object, true);
        let tints: Vec<f32> = (0..LEVELS).flat_map(|_| [1f32, 1., 1., 0.]).collect();
        let black = r.device.create_texture_with_data(
            &r.queue,
            &wgpu::TextureDescriptor {
                label: Some("volume fire no bloom"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: HALF,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            Default::default(),
            &[0; 8],
        );
        let demo = Self {
            controls,
            params: Params::default(),
            time: 0.,
            last: 0.,
            frame_id: 0,
            advance: false,
            simulation_time: 0.,
            accumulator: 0.,
            temporal: Temporal {
                time: 0.,
                flame_height: 3.5,
                sway: Vector3::ZERO,
                flicker: 1.,
                color_noise: 0.,
                rotation: 0.,
            },
            steps: vec![],
            parity: 0,
            teapot_position: Vector3::new(0., FLOOR_Y - teapot_min_y, 0.),
            previous_position: Vector3::new(0., FLOOR_Y - teapot_min_y, 0.),
            teapot_speed: 0.,
            teapot_velocity: Vector3::ZERO,
            point_distance: 0.01,
            teapot_written: None,
            queue: vec![],
            pointer: Vector2::ZERO,
            drag: None,
            selected: false,
            teapot_triangles,
            teapot_min_y,
            teapot,
            teapot_storage,
            floor,
            volume_box,
            kernels,
            shadow_color,
            shadow_depth,
            shadow: (shadow_pipeline, shadow_groups),
            teapot_draw: (teapot_pipeline, teapot_groups),
            floor_draw: (floor_pipeline, floor_groups),
            caster: (caster_pipeline, caster_groups),
            volume: (volume_pipeline, volume),
            advect_velocity_object,
            advect_dye_object,
            emit_object,
            shadow_render,
            shadow_object,
            teapot_render,
            teapot_object,
            floor_render,
            caster_render,
            caster_object,
            volume_render,
            volume_object,
            blur_objects: [
                sized(r, "denoise", wgsl!("blur_h_fs"), "objectStruct")?,
                sized(r, "denoise", wgsl!("blur_v_fs"), "objectStruct")?,
            ],
            high_object: sized(r, "bloom high pass", wgsl!("high_fs"), "objectStruct")?,
            composite_object: sized(r, "bloom composite", wgsl!("composite_fs"), "objectStruct")?,
            tints: init(
                "bloom tints",
                bytemuck::cast_slice(&tints),
                wgpu::BufferUsages::UNIFORM,
            ),
            output_render: sized(r, "volume fire output", wgsl!("output_fs"), "renderStruct")?,
            output_object: sized(r, "volume fire output", wgsl!("output_fs"), "objectStruct")?,
            black: view(&black),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            linear,
            targets: None,
        };
        // The floor's material is static: three writes its object uniforms
        // ( the fire light's inputs ) at its first render only.
        let floor_model = Matrix4::from_translation(Vector3::new(0., FLOOR_Y, 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        let mut floor_values = vec![
            (
                "nodeUniform0",
                Color::from_hex(0x111115).0.to_array().to_vec(),
            ),
            ("nodeUniform1", vec![1.]),
            ("nodeUniform2", vec![0.]),
            ("nodeUniform3", vec![0.8]),
            ("nodeUniform5", m3(floor_model.inverse().transpose())),
            ("nodeUniform6", vec![0.; 3]),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform9", m4(floor_model)),
        ];
        floor_values.extend(demo.light_object(
            [
                "nodeUniform11",
                "nodeUniform12",
                "nodeUniform13",
                "nodeUniform14",
                "nodeUniform15",
                "nodeUniform16",
                "nodeUniform17",
                "nodeUniform18",
                "nodeUniform19",
                "nodeUniform20",
                "nodeUniform21",
                "nodeUniform22",
                "nodeUniform23",
                "nodeUniform24",
                "nodeUniform25",
                "nodeUniform26",
                "nodeUniform27",
                "nodeUniform28",
                "nodeUniform29",
                "nodeUniform30",
                "nodeUniform31",
                "nodeUniform32",
                "nodeUniform33",
            ],
            &demo.temporal,
        ));
        let values: Vec<(&str, &[f64])> = floor_values.iter().map(|(n, v)| (*n, &v[..])).collect();
        r.queue.write_buffer(
            &floor_object,
            0,
            &pack(wgsl!("floor_fs"), "objectStruct", &values)?,
        );
        Ok(demo)
    }
    /// The fire light's object uniforms, by the order of a material's struct:
    /// end color, temperature rate, hue, mid color, start color, saturation,
    /// color noise, teapot position, sway, flame height, projection radius and
    /// frequency, time, center and noise fades, density rate, fire intensity,
    /// smoke and reflection intensities, flicker, near and far scales and the
    /// far distance.
    fn light_object(
        &self,
        names: [&'static str; 23],
        t: &Temporal,
    ) -> Vec<(&'static str, Vec<f64>)> {
        let p = &self.params;
        let color = |hex: u32| Color::from_hex(hex).0.to_array().to_vec();
        vec![
            (names[0], color(p.colors[2])),
            (names[1], vec![p.emit_temperature]),
            (names[2], vec![p.fire_hue.to_radians()]),
            (names[3], color(p.colors[1])),
            (names[4], color(p.colors[0])),
            (names[5], vec![p.saturation]),
            (names[6], vec![t.color_noise]),
            (names[7], self.teapot_position.to_array().to_vec()),
            (names[8], t.sway.to_array().to_vec()),
            (names[9], vec![t.flame_height]),
            (names[10], vec![p.projection_radius]),
            (names[11], vec![p.projection_frequency]),
            (names[12], vec![t.time]),
            (names[13], vec![p.projection_center_fade]),
            (names[14], vec![p.projection_noise_fade]),
            (names[15], vec![p.emit_density]),
            (names[16], vec![40.]),
            (names[17], vec![p.light_smoke]),
            (names[18], vec![p.light_reflection]),
            (names[19], vec![t.flicker]),
            (names[20], vec![p.light_near]),
            (names[21], vec![p.light_far]),
            (names[22], vec![10.]),
        ]
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.frame_id += 1;
            self.advance = true;
        }
        Ok(())
    }
    fn teapot_model(&self, rotation: f64) -> Matrix4 {
        Matrix4::from_rotation_translation(
            Quaternion::from_rotation_y(rotation),
            self.teapot_position,
        )
    }
    /// animate(): the teapot's motion, the simulation steps the accumulator
    /// allows ( each with updateTemporalUniforms ) and the point light's range.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.drain(s, c)?;
        if !std::mem::take(&mut self.advance) {
            return self.controls.update(s, c);
        }
        let now = self.time * 1000.;
        let delta = ((now - self.last) * 0.001).min(1. / 30.);
        self.last = now;
        let distance = self.teapot_position.distance(self.previous_position);
        self.teapot_speed = if delta > 0. { distance / delta } else { 0. };
        self.teapot_velocity = if delta > 0. {
            (self.teapot_position - self.previous_position) * (1. / delta)
        } else {
            Vector3::ZERO
        };
        self.previous_position = self.teapot_position;
        let p = &self.params;
        if p.simulate && p.sim_speed > 0. {
            self.accumulator += delta * p.sim_speed;
            let step = 1. / 120. * p.sim_speed;
            self.accumulator = self.accumulator.min(step * 8.);
            while self.accumulator >= step {
                self.simulation_time += step;
                self.temporal = Temporal::at(self.simulation_time);
                self.steps.push(self.temporal);
                self.accumulator -= step;
            }
        } else {
            self.temporal = Temporal::at(self.simulation_time);
        }
        let size = ((p.emit_temperature / 8.34) * (p.emit_density / 11.02) * (40. / 5.63)).sqrt();
        let t = (self.simulation_time / 3.).clamp(0., 1.);
        let fade = t * t * (3. - 2. * t);
        self.point_distance = (40. * size.max(0.2) * fade).max(0.01);
        self.controls.update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let scene = view(&texture2d(r, size, HALF));
        let depth = view(&texture2d(r, size, DEPTH));
        // PassNode.setResolutionScale: the volume pass's size.
        let scale = self.params.resolution;
        let volume_size = (
            ((size.0 as f64 * scale).floor() as u32).max(1),
            ((size.1 as f64 * scale).floor() as u32).max(1),
        );
        let volume_target = view(&texture2d(r, volume_size, HALF));
        let blur_targets = [
            view(&texture2d(r, volume_size, HALF)),
            view(&texture2d(r, volume_size, HALF)),
        ];
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let uv = [layout(8, &UV0)];
        let all = wgpu::ColorWrites::ALL;
        let quad = |label, vs, fs, format| {
            pipeline(r, label, (vs, fs), &uv, format, None, false, None, all)
        };
        let blurs = [
            (wgsl!("blur_h_fs"), &volume_target, 0),
            (wgsl!("blur_v_fs"), &blur_targets[0], 1),
        ]
        .map(|(fs, input, i)| {
            let p = quad("volume fire denoise", wgsl!("blur_vs"), fs, HALF);
            let group = bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(input)),
                    (2, self.blur_objects[i].as_entire_binding()),
                ],
            );
            (p, group)
        });
        let bright_size = half_size(size);
        let bright = view(&texture2d(r, bright_size, HALF));
        // The high pass reads the scene and the ( denoised ) volume.
        let high = quad(
            "bloom high pass",
            shared!("composite_vs"),
            wgsl!("high_fs"),
            HALF,
        );
        let high_groups = [&blur_targets[1], &volume_target].map(|volume| {
            bind(
                r,
                high.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&scene)),
                    (2, sampler(&self.linear)),
                    (3, tex(volume)),
                    (4, self.high_object.as_entire_binding()),
                ],
            )
        });
        // BloomNode.setSize: each level halves the last, rounded.
        let mut levels = vec![];
        let mut level_size = bright_size;
        for _ in 0..LEVELS {
            levels.push((
                view(&texture2d(r, level_size, HALF)),
                view(&texture2d(r, level_size, HALF)),
                level_size,
            ));
            level_size = half_size(level_size);
        }
        let blur_shaders = [
            shared!("blur0_fs"),
            shared!("blur1_fs"),
            shared!("blur2_fs"),
            shared!("blur3_fs"),
            shared!("blur4_fs"),
        ];
        let mut bloom_blurs = vec![];
        for (i, fs) in blur_shaders.iter().enumerate() {
            let p = quad("bloom blur", shared!("blur_vs"), fs, HALF);
            let input = if i == 0 { &bright } else { &levels[i - 1].1 };
            let pass = |input: &wgpu::TextureView| -> Result<_> {
                let object = sized(r, "bloom blur", fs, "objectStruct")?;
                let group = bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(input)),
                        (2, object.as_entire_binding()),
                    ],
                );
                Ok((p.clone(), group, object))
            };
            bloom_blurs.push([pass(input)?, pass(&levels[i].0)?]);
        }
        let composite = quad(
            "bloom composite",
            shared!("composite_vs"),
            wgsl!("composite_fs"),
            HALF,
        );
        let mut entries = vec![
            (0, self.composite_object.as_entire_binding()),
            (1, self.tints.as_entire_binding()),
        ];
        for (i, (_, vertical, _)) in levels.iter().enumerate() {
            entries.push((2 + i as u32 * 2, sampler(&self.linear)));
            entries.push((3 + i as u32 * 2, tex(vertical)));
        }
        let composite_group = bind(r, composite.get_bind_group_layout(0), &entries);
        let output = quad(
            "volume fire output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            out.options.format,
        );
        let output_render = bind(
            r,
            output.get_bind_group_layout(0),
            &[(0, self.output_render.as_entire_binding())],
        );
        let output_groups = [&blur_targets[1], &volume_target].map(|volume| {
            [&levels[0].0, &self.black].map(|bloom| {
                bind(
                    r,
                    output.get_bind_group_layout(1),
                    &[
                        (0, sampler(&self.linear)),
                        (1, tex(&scene)),
                        (2, sampler(&self.linear)),
                        (3, tex(volume)),
                        (4, self.output_object.as_entire_binding()),
                        (5, sampler(&self.linear)),
                        (6, tex(bloom)),
                    ],
                )
            })
        });
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            resolution: scale,
            scene,
            depth,
            volume_size,
            volume_target,
            blur_targets,
            blurs,
            bright,
            levels,
            high: (high, high_groups),
            bloom_blurs,
            composite: (composite, composite_group),
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
            output: (output, output_render, output_groups),
        });
        Ok(())
    }
    /// One simulation step: its uniforms, then the seven kernels.
    fn step(&mut self, r: &Renderer, t: &Temporal) -> Result<()> {
        let p = &self.params;
        let speed = p.sim_speed;
        let dt = 1. / 120. * speed;
        let turbulence = if speed > 0. {
            p.turbulence / speed.sqrt()
        } else {
            0.
        };
        let dissipation = if p.smoke_lifespan >= 100. {
            0.
        } else {
            1. / p.smoke_lifespan
        };
        let volume = VOLUME.to_vec();
        let write =
            |buffer: &wgpu::Buffer, source: &str, values: &[(&str, Vec<f64>)]| -> Result<()> {
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (*n, &v[..])).collect();
                r.queue
                    .write_buffer(buffer, 0, &pack(source, "objectStruct", &values)?);
                Ok(())
            };
        write(
            &self.advect_velocity_object,
            wgsl!("advect_velocity_cs"),
            &[
                ("nodeUniform1", volume.clone()),
                ("nodeUniform2", vec![dt]),
                ("nodeUniform4", vec![p.buoyancy]),
                ("nodeUniform5", vec![0.15]),
                ("nodeUniform7", vec![p.turb_frequency]),
                ("nodeUniform8", vec![turbulence]),
                ("nodeUniform9", vec![p.turbulence_decay]),
                ("nodeUniform10", vec![t.time]),
                ("nodeUniform11", vec![p.vel_damping]),
                ("nodeUniform12", self.teapot_position.to_array().to_vec()),
                ("nodeUniform13", self.teapot_velocity.to_array().to_vec()),
                ("nodeUniform14", vec![p.wind_strength]),
                ("nodeUniform15", vec![self.teapot_speed]),
                ("nodeUniform17", vec![CELLS as f64]),
            ],
        )?;
        write(
            &self.advect_dye_object,
            wgsl!("advect_dye_cs"),
            &[
                ("nodeUniform2", volume.clone()),
                ("nodeUniform3", vec![dt]),
                ("nodeUniform4", vec![dissipation]),
                ("nodeUniform5", vec![1. / p.fire_lifespan]),
                ("nodeUniform7", vec![CELLS as f64]),
            ],
        )?;
        write(
            &self.emit_object,
            wgsl!("emit_cs"),
            &[
                ("nodeUniform0", m4(self.teapot_model(t.rotation))),
                ("nodeUniform2", volume),
                ("nodeUniform3", vec![p.emit_density]),
                ("nodeUniform4", vec![t.time]),
                ("nodeUniform5", vec![p.emit_temperature]),
                ("nodeUniform6", vec![self.teapot_speed]),
                ("nodeUniform7", vec![p.motion_boost]),
                (
                    "nodeUniform10",
                    vec![self.teapot_triangles_vertex_count() as f64],
                ),
            ],
        )?;
        let k = &self.kernels;
        let cur = self.parity;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let groups = cells_groups(CELLS);
        let passes: [(&wgpu::ComputePipeline, &wgpu::BindGroup, u32); 7] = [
            (&k.advect_velocity.0, &k.advect_velocity.1[cur], groups),
            (&k.divergence.0, &k.divergence.1, groups),
            (&k.jacobi[0].0, &k.jacobi[0].1, groups),
            (&k.jacobi[1].0, &k.jacobi[1].1, groups),
            (&k.project.0, &k.project.1, groups),
            (&k.advect_dye.0, &k.advect_dye.1[cur], groups),
            (
                &k.emit.0,
                &k.emit.1[cur],
                (self.teapot_triangles_vertex_count() as u32).div_ceil(64),
            ),
        ];
        for (pipeline, group, count) in passes {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.dispatch_workgroups(count, 1, 1);
        }
        r.queue.submit([encoder.finish()]);
        // Ping-pong the dye grids.
        self.parity = 1 - cur;
        Ok(())
    }
    fn teapot_triangles_vertex_count(&self) -> usize {
        (self.teapot_storage.size() / 16) as usize
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.resolution != self.params.resolution
        }) {
            self.resize(r, out)?;
        }
        for t in std::mem::take(&mut self.steps) {
            self.step(r, &t)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        let (spot_position, spot_target, spot_projection, spot_view) = spot();
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = m4(bias * spot_projection * spot_view);
        let p = self.params.clone();
        let temporal = self.temporal;
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(&str, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        // The render group: the camera, the point light ( distance, decay 2 and
        // view position ), the spot light ( cone, penumbra, distance 0, decay 2,
        // color, view and world positions, target ) and its shadow.
        let light_render = |names: [&'static str; 17]| {
            vec![
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraPosition", camera_position.to_array().to_vec()),
                (names[0], vec![self.point_distance]),
                (names[1], vec![2.]),
                (names[2], vec![(PI / 5.).cos()]),
                (names[3], vec![1.]),
                (names[4], vec![0.]),
                (names[5], vec![2.]),
                (names[6], vec![p.key_intensity; 3]),
                (
                    names[7],
                    view.transform_point3(self.teapot_position)
                        .to_array()
                        .to_vec(),
                ),
                (
                    names[8],
                    view.transform_point3(spot_position).to_array().to_vec(),
                ),
                (names[9], spot_position.to_array().to_vec()),
                (names[10], spot_target.to_array().to_vec()),
                (names[11], shadow_matrix.clone()),
                (names[12], vec![0.]),
                (names[13], vec![-0.001]),
                (names[14], vec![1.]),
                (names[15], vec![SHADOW_SIZE as f64; 2]),
                (names[16], vec![0.98]),
            ]
        };
        let n = |i: u32| -> &'static str { NAMES[i as usize] };
        let names = |ids: [u32; 17]| ids.map(n);
        write(
            &self.teapot_render,
            wgsl!("teapot_fs"),
            "renderStruct",
            &light_render(names([
                33, 34, 46, 47, 50, 51, 36, 19, 35, 48, 49, 38, 40, 41, 43, 44, 45,
            ])),
        )?;
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &light_render(names([
                34, 35, 47, 48, 51, 52, 37, 10, 36, 49, 50, 39, 41, 42, 44, 45, 46,
            ])),
        )?;
        let frame = ("nodeUniform5", vec![self.frame_id as f64]);
        let mut caster_render = light_render(names([
            29, 32, 34, 35, 39, 40, 33, 30, 36, 37, 38, 42, 45, 46, 48, 49, 50,
        ]));
        caster_render.push(frame.clone());
        write(
            &self.caster_render,
            wgsl!("caster_fs"),
            "renderStruct",
            &caster_render,
        )?;
        let mut volume_render = light_render(names([
            29, 32, 44, 45, 49, 50, 33, 30, 46, 47, 48, 35, 38, 39, 41, 42, 43,
        ]));
        volume_render.push(frame);
        write(
            &self.volume_render,
            wgsl!("volume_fs"),
            "renderStruct",
            &volume_render,
        )?;
        write(
            &self.shadow_render,
            wgsl!("shadow_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(spot_projection)),
                ("cameraViewMatrix", m4(spot_view)),
                ("cameraPosition", spot_position.to_array().to_vec()),
            ],
        )?;
        let volume_model = Matrix4::from_translation(Vector3::new(0., VOLUME_Y, 0.));
        let volume = VOLUME.to_vec();
        write(
            &self.shadow_object,
            wgsl!("shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![16.]),
                ("nodeUniform1", m4(volume_model)),
                ("nodeUniform3", volume.clone()),
                ("nodeUniform6", vec![p.shadow_absorption]),
                ("nodeUniform7", vec![1.]),
            ],
        )?;
        let radius = (VOLUME.iter().map(|v| (v / 2.) * (v / 2.)).sum::<f64>()).sqrt();
        let light_names = |ids: [u32; 23]| ids.map(n);
        let volume_object = |steps: f64, mat3: &'static str| {
            let mut v = vec![
                ("nodeUniform0", vec![1.]),
                ("nodeUniform2", m4(volume_model)),
                ("nodeUniform3", vec![radius]),
                ("nodeUniform4", vec![steps]),
                (mat3, m3(Matrix4::IDENTITY)),
            ];
            v.extend(self.light_object(
                light_names([
                    6, 7, 12, 9, 10, 11, 8, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27, 28,
                ]),
                &temporal,
            ));
            v
        };
        write(
            &self.caster_object,
            wgsl!("caster_fs"),
            "objectStruct",
            &volume_object(16., "nodeUniform44"),
        )?;
        let mut volume_values = volume_object(p.steps, "nodeUniform37");
        volume_values.extend([
            ("nodeUniform51", volume),
            ("nodeUniform54", spot_position.to_array().to_vec()),
            ("nodeUniform55", vec![p.shadow_absorption]),
            ("nodeUniform56", vec![p.multi_scattering]),
            ("nodeUniform57", vec![p.powder]),
            ("nodeUniform58", vec![p.shadow_ambient]),
            ("nodeUniform59", vec![p.asymmetry]),
            ("nodeUniform60", vec![p.glow_spread]),
        ]);
        write(
            &self.volume_object,
            wgsl!("volume_fs"),
            "objectStruct",
            &volume_values,
        )?;
        let teapot_model = self.teapot_model(temporal.rotation);
        if self.teapot_written != Some(teapot_model) {
            self.teapot_written = Some(teapot_model);
            let mut v = vec![
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(teapot_model.inverse().transpose())),
                ("nodeUniform16", vec![p.teapot_emissive]),
                ("nodeUniform18", m4(teapot_model)),
            ];
            v.extend(self.light_object(
                light_names([
                    6, 12, 11, 8, 9, 10, 20, 21, 22, 23, 24, 25, 7, 26, 27, 13, 14, 28, 29, 15, 30,
                    31, 32,
                ]),
                &temporal,
            ));
            write(&self.teapot_object, wgsl!("teapot_fs"), "objectStruct", &v)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("volume fire targets"))?;
        for (object, source) in self
            .blur_objects
            .iter()
            .zip([wgsl!("blur_h_fs"), wgsl!("blur_v_fs")])
        {
            write(
                object,
                source,
                "objectStruct",
                &[
                    ("nodeUniform1", vec![p.denoise_strength]),
                    (
                        "nodeUniform2",
                        vec![1. / t.volume_size.0 as f64, 1. / t.volume_size.1 as f64],
                    ),
                ],
            )?;
        }
        write(
            &self.high_object,
            wgsl!("high_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![p.saturation]),
                ("nodeUniform3", vec![p.bloom_threshold]),
                ("nodeUniform4", vec![0.01]),
            ],
        )?;
        let identity = m3(Matrix4::IDENTITY);
        for (blur, (_, _, level)) in t.bloom_blurs.iter().zip(&t.levels) {
            for ((_, _, object), direction) in blur.iter().zip([[1., 0.], [0., 1.]]) {
                write(
                    object,
                    shared!("blur0_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform1", identity.clone()),
                        ("nodeUniform2", identity.clone()),
                        ("nodeUniform3", direction.to_vec()),
                        (
                            "nodeUniform4",
                            vec![1. / level.0 as f64, 1. / level.1 as f64],
                        ),
                        ("nodeUniform5", identity.clone()),
                    ],
                )?;
            }
        }
        write(
            &self.composite_object,
            wgsl!("composite_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![p.bloom_radius]),
                ("nodeUniform3", identity.clone()),
                ("nodeUniform5", identity.clone()),
                ("nodeUniform7", identity.clone()),
                ("nodeUniform9", identity.clone()),
                ("nodeUniform11", identity),
                ("nodeUniform12", vec![p.bloom_strength]),
            ],
        )?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform4", vec![2.])],
        )?;
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[("nodeUniform2", vec![p.saturation])],
        )?;
        let dye = self.parity;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        fn attachment(
            view: &wgpu::TextureView,
            clear: wgpu::Color,
        ) -> Option<wgpu::RenderPassColorAttachment<'_>> {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: wgpu::StoreOp::Store,
                },
            })
        }
        let depth_attachment = |view| {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            })
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, buffers: [&wgpu::Buffer; 2]| {
            pass.set_vertex_buffer(0, buffers[0].slice(..));
            pass.set_vertex_buffer(1, buffers[1].slice(..));
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let set = |pass: &mut wgpu::RenderPass,
                   pipeline: &wgpu::RenderPipeline,
                   groups: &[wgpu::BindGroup; 2]| {
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &groups[0], &[]);
            pass.set_bind_group(1, &groups[1], &[]);
        };
        // The spot's shadow map: the volume's colored shadow caster.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume fire shadow"),
                color_attachments: &[attachment(&self.shadow_color, wgpu::Color::TRANSPARENT)],
                depth_stencil_attachment: depth_attachment(&self.shadow_depth),
                ..Default::default()
            });
            set(&mut pass, &self.shadow.0, &self.shadow.1[dye]);
            let b = &self.volume_box;
            pass.set_vertex_buffer(0, b.positions.slice(..));
            pass.set_index_buffer(b.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..b.count, 0, 0..1);
        }
        // The scene pass: the teapot, the floor and the caster's colorless draw.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume fire scene"),
                color_attachments: &[attachment(&t.scene, wgpu::Color::BLACK)],
                depth_stencil_attachment: depth_attachment(&t.depth),
                ..Default::default()
            });
            set(&mut pass, &self.teapot_draw.0, &self.teapot_draw.1);
            draw(
                &mut pass,
                &self.teapot,
                [&self.teapot.normals, &self.teapot_storage],
            );
            set(&mut pass, &self.floor_draw.0, &self.floor_draw.1);
            draw(
                &mut pass,
                &self.floor,
                [&self.floor.normals, &self.floor.positions],
            );
            set(&mut pass, &self.caster.0, &self.caster.1);
            let b = &self.volume_box;
            draw(&mut pass, b, [&b.positions, &b.normals]);
        }
        // The volumetric layer at its resolution scale.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume fire volume"),
                color_attachments: &[attachment(&t.volume_target, wgpu::Color::BLACK)],
                ..Default::default()
            });
            set(&mut pass, &self.volume.0, &self.volume.1[dye]);
            let b = &self.volume_box;
            draw(&mut pass, b, [&b.positions, &b.normals]);
        }
        let quad_pass = |encoder: &mut wgpu::CommandEncoder,
                         target: &wgpu::TextureView,
                         pipeline: &wgpu::RenderPipeline,
                         groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume fire quad"),
                color_attachments: &[attachment(target, wgpu::Color::BLACK)],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, g) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *g, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if p.denoise {
            for ((pipeline, group), target) in t.blurs.iter().zip(&t.blur_targets) {
                quad_pass(&mut encoder, target, pipeline, &[group]);
            }
        }
        let raw = usize::from(!p.denoise);
        if p.bloom {
            quad_pass(&mut encoder, &t.bright, &t.high.0, &[&t.high.1[raw]]);
            for (blur, (horizontal, vertical, _)) in t.bloom_blurs.iter().zip(&t.levels) {
                for ((pipeline, group, _), target) in blur.iter().zip([horizontal, vertical]) {
                    quad_pass(&mut encoder, target, pipeline, &[group]);
                }
            }
            quad_pass(
                &mut encoder,
                &t.levels[0].0,
                &t.composite.0,
                &[&t.composite.1],
            );
        }
        let output = &t.output;
        quad_pass(
            &mut encoder,
            &t.screen.view,
            &output.0,
            &[&output.1, &output.2[raw][usize::from(!p.bloom)]],
        );
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// Queued pointer events, in CSS pixels.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push((kind, x, y));
    }
    /// DragControls: a press on the teapot selects it on the plane facing the
    /// camera through its position; moves follow the plane, clamped to the
    /// volume; the release ends the drag.
    fn drain(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let events = std::mem::take(&mut self.queue);
        if events.is_empty() {
            return Ok(());
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (camera, world) = (camera.clone(), world);
        let ray = |pointer: Vector2| -> Result<Ray> {
            let mut r = Raycaster::default();
            r.set_from_camera(pointer, &camera, world)?;
            Ok(r.ray)
        };
        for (kind, x, y) in events {
            let (w, h, _) = viewport_css();
            self.pointer = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
            match kind {
                10..=12 => {
                    let ray = ray(self.pointer)?;
                    if self.hit(ray) {
                        self.selected = true;
                        // Left and middle translate; right rotates ( rotateSpeed 0 ).
                        if kind != 12 {
                            let normal = world.transform_vector3(Vector3::NEG_Z).normalize();
                            let plane = Plane::from_normal_point(normal, self.teapot_position);
                            if let Some(point) = ray.intersect_plane(plane) {
                                self.drag = Some((plane, point - self.teapot_position));
                            }
                        }
                    }
                }
                0 => {
                    if let Some((plane, offset)) = self.drag
                        && let Some(point) = ray(self.pointer)?.intersect_plane(plane)
                    {
                        let p = point - offset;
                        let limit_x = VOLUME[0] / 2. - 1.5;
                        let limit_z = VOLUME[2] / 2. - 1.5;
                        self.teapot_position = Vector3::new(
                            p.x.clamp(-limit_x, limit_x),
                            p.y.min(VOLUME[1] - 1.5).max(FLOOR_Y - self.teapot_min_y),
                            p.z.clamp(-limit_z, limit_z),
                        );
                    }
                }
                20..=22 => {
                    self.selected = false;
                    self.drag = None;
                }
                _ => {}
            }
        }
        Ok(())
    }
    /// The teapot's front-facing triangles under the ray ( Mesh.raycast ).
    fn hit(&self, ray: Ray) -> bool {
        let model = self.teapot_model(self.temporal.rotation);
        let local = ray.transformed(model.inverse());
        self.teapot_triangles.iter().any(|[a, b, c]| {
            let (e1, e2) = (*b - *a, *c - *a);
            let normal = e1.cross(e2);
            let ddn = local.direction.dot(normal);
            if ddn >= 0. {
                return false;
            }
            let diff = local.origin - *a;
            let u = -local.direction.dot(diff.cross(e2)) / ddn;
            let v = -local.direction.dot(e1.cross(diff)) / ddn;
            let t = diff.dot(normal) / -ddn;
            u >= 0. && v >= 0. && u + v <= 1. && t >= 0.
        })
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        self.drain(s, c)?;
        // dragstart disables the orbit until dragend.
        if self.selected && wheel == 0. {
            return Ok(());
        }
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// The GUI in its order, through Proj Center Fade ( the tone mapping
    /// controls are not exposed ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        let p = &mut self.params;
        match index {
            0 => p.simulate = v != 0.,
            1 => p.sim_speed = v,
            2 => p.resolution = v,
            3 => p.steps = v.round(),
            4 => p.denoise = v != 0.,
            5 => p.denoise_strength = v,
            6 => p.bloom = v != 0.,
            7 => p.bloom_strength = v,
            8 => p.bloom_radius = v,
            9 => p.bloom_threshold = v,
            10 => p.glow_spread = v,
            11 => p.fire_hue = v,
            12 => p.saturation = v,
            13..=15 => p.colors[index - 13] = value as u32,
            16 => p.emit_temperature = v,
            17 => p.emit_density = v,
            18 => p.motion_boost = v,
            19 => p.wind_strength = v,
            20 => p.teapot_emissive = v,
            21 => p.asymmetry = v,
            22 => p.powder = v,
            23 => p.multi_scattering = v,
            24 => p.shadow_absorption = v,
            25 => p.shadow_ambient = v,
            26 => p.buoyancy = v,
            27 => p.vel_damping = v,
            28 => p.fire_lifespan = v,
            29 => p.smoke_lifespan = v,
            30 => p.turbulence = v,
            31 => p.turbulence_decay = v,
            32 => p.turb_frequency = v,
            33 => p.key_intensity = v,
            34 => p.light_smoke = v,
            35 => p.light_near = v,
            36 => p.light_far = v,
            37 => p.light_reflection = v,
            38 => p.projection_radius = v,
            39 => p.projection_frequency = v,
            40 => p.projection_noise_fade = v,
            41 => p.projection_center_fade = v,
            _ => return Err(Error::Invalid("volume fire parameter")),
        }
        Ok(())
    }
    /// A requested frame: nodeFrame.update() advances the frame id.
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.frame_id += 1;
        self.advance = true;
    }
}
fn cells_groups(cells: u32) -> u32 {
    cells.div_ceil(64)
}
/// nodeUniform names by number.
const NAMES: [&str; 61] = [
    "nodeUniform0",
    "nodeUniform1",
    "nodeUniform2",
    "nodeUniform3",
    "nodeUniform4",
    "nodeUniform5",
    "nodeUniform6",
    "nodeUniform7",
    "nodeUniform8",
    "nodeUniform9",
    "nodeUniform10",
    "nodeUniform11",
    "nodeUniform12",
    "nodeUniform13",
    "nodeUniform14",
    "nodeUniform15",
    "nodeUniform16",
    "nodeUniform17",
    "nodeUniform18",
    "nodeUniform19",
    "nodeUniform20",
    "nodeUniform21",
    "nodeUniform22",
    "nodeUniform23",
    "nodeUniform24",
    "nodeUniform25",
    "nodeUniform26",
    "nodeUniform27",
    "nodeUniform28",
    "nodeUniform29",
    "nodeUniform30",
    "nodeUniform31",
    "nodeUniform32",
    "nodeUniform33",
    "nodeUniform34",
    "nodeUniform35",
    "nodeUniform36",
    "nodeUniform37",
    "nodeUniform38",
    "nodeUniform39",
    "nodeUniform40",
    "nodeUniform41",
    "nodeUniform42",
    "nodeUniform43",
    "nodeUniform44",
    "nodeUniform45",
    "nodeUniform46",
    "nodeUniform47",
    "nodeUniform48",
    "nodeUniform49",
    "nodeUniform50",
    "nodeUniform51",
    "nodeUniform52",
    "nodeUniform53",
    "nodeUniform54",
    "nodeUniform55",
    "nodeUniform56",
    "nodeUniform57",
    "nodeUniform58",
    "nodeUniform59",
    "nodeUniform60",
];
