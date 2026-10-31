//! webgpu_lightprobes and webgpu_lightprobes_complex: shadow-casting point
//! lights and LightProbeGrid volumes baked on the GPU as three.js r187 bakes
//! them: probes at the cells' centers, per batch of 8 probes one render of
//! the 48 90° sub-cameras ( ArrayCamera ) into a 32² face atlas, the
//! per-face solid-angle SH projection into the batch rows ( 9 × 6 × 4
//! columns per probe ), and the seven repack passes summing the columns
//! into the padded RGBA16F 3D atlas. The pages bake a direct pass and
//! `bounces` more: each volume's bake adds its bounce grid ( a snapshot of
//! its previous pass, unlit in the direct pass ) to the scene, and after the
//! direct pass the other volumes light its captures too. The scene renders
//! with 4× MSAA through the lit materials sampling the atlases, then the
//! ACES output pass. GI, resolution, bounces and light intensity ( the
//! three rebake before the next frame ) and the probe helpers ( hidden
//! during the bake ) follow the GUI. Every stage runs the WGSL three.js r187
//! generates for the pages: the Cornell box's lit materials in
//! `lightprobes/`, the rest in `lightprobes_complex/` ( the scene of
//! webgpu_lightprobes_complex is `lightprobes_complex/rooms.rs`; the output
//! pass is cubemap_dynamic's, which is byte-identical ).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::{target, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;
#[path = "lightprobes_complex/rooms.rs"]
mod rooms;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The point shadows' map size, near and far.
const SHADOW: u32 = 256;
const SHADOW_NEAR: f64 = 0.5;
const SHADOW_FAR: f64 = 500.;
/// The bake's cubemapSize, near and far.
const CUBE: u32 = 32;
const CUBE_NEAR: f64 = 0.05;
const CUBE_FAR: f64 = 20.;
/// LightProbeGrid's probes per capture render, and the SH columns per probe
/// ( 9 coefficients × 6 faces × 4 row blocks ).
const BATCH: u32 = 8;
const COLUMNS: u32 = 9 * 6 * 4;
/// Color( 0x111111 ) in linear space: the background clear.
const BACKGROUND: f64 = 0.005605391621829108;
/// The shaders both pages share.
macro_rules! shared {
    ($name:literal) => {
        include_str!(concat!("lightprobes_complex/", $name, ".wgsl"))
    };
}
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("lightprobes/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
const HELPER_FS: &str = shared!("helper_fs");
const PROBE_SH_FS: &str = shared!("probe_sh_fs");
/// The repack passes by the SH texture they write.
const REPACK_FS: [&str; 7] = [
    shared!("probe_repack_0_fs"),
    shared!("probe_repack_1_fs"),
    shared!("probe_repack_2_fs"),
    shared!("probe_repack_3_fs"),
    shared!("probe_repack_4_fs"),
    shared!("probe_repack_5_fs"),
    shared!("probe_repack_6_fs"),
];
/// PointShadowNode's WebGPU cube faces ( CubeCamera's ): directions and ups.
const SHADOW_FACES: [([f64; 3], [f64; 3]); 6] = [
    ([-1., 0., 0.], [0., 1., 0.]),
    ([1., 0., 0.], [0., 1., 0.]),
    ([0., 1., 0.], [0., 0., -1.]),
    ([0., -1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., 1., 0.]),
    ([0., 0., -1.], [0., 1., 0.]),
];
/// LightProbeGrid's capture cameras per face: forward and up.
const CAPTURE_FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., 1., 0.]),
    ([-1., 0., 0.], [0., 1., 0.]),
    ([0., 1., 0.], [0., 0., -1.]),
    ([0., -1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., 1., 0.]),
    ([0., 0., -1.], [0., 1., 0.]),
];
/// The lit materials' variants: front side receiving shadows, front side
/// without them ( receiveShadow false ) and a back-side shell.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Shadowed,
    Plain,
    Back,
}
/// The material builds: the main camera without grids ( GI off ) and with
/// every volume, and the capture cameras with one grid ( the direct pass:
/// the unlit bounce grid; with a single volume, the bounce passes too ) and
/// two ( the bounce passes: the other volume and the bounce grid ).
#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Main0,
    Main,
    Bake1,
    Bake2,
}
impl Stage {
    fn bake(self) -> bool {
        matches!(self, Stage::Bake1 | Stage::Bake2)
    }
}
/// Uniform values by name.
type Uniforms = Vec<(String, Vec<f64>)>;
fn u(n: usize) -> String {
    format!("nodeUniform{n}")
}
/// A build's uniform numbering: the material's normal matrix, emissive,
/// emissive intensity and model ( after color, opacity, metalness and
/// roughness ), the lights' first uniforms, and the first grid's.
struct Names {
    material: [usize; 4],
    lights: [usize; 2],
    grids: usize,
}
impl Names {
    /// The capture builds number the material's uniforms one higher.
    fn new(stage: Stage, lights: [usize; 2], grids: usize) -> Self {
        let material = if stage.bake() {
            [6, 7, 8, 10]
        } else {
            [5, 6, 7, 9]
        };
        Self {
            material,
            lights,
            grids,
        }
    }
}
/// A shadow-casting PointLight ( intensity from the GUI ): position, color
/// and its shadow's bias, normalBias and radius.
struct Light {
    position: Vector3,
    color: u32,
    bias: f64,
    normal_bias: f64,
    radius: f64,
}
/// A LightProbeGrid's center and size.
#[derive(Clone, Copy)]
struct Volume {
    center: Vector3,
    size: Vector3,
}
impl Volume {
    /// getProbePosition: each probe at the center of its cell.
    fn probe(&self, res: u32, i: [u32; 3]) -> Vector3 {
        let (c, s) = (self.center.to_array(), self.size.to_array());
        let axis = |k: usize| c[k] - s[k] / 2. + (i[k] as f64 + 0.5) * s[k] / res as f64;
        Vector3::new(axis(0), axis(1), axis(2))
    }
    /// The bounding box's max and min.
    fn bounds(&self) -> (Vector3, Vector3) {
        (self.center + self.size / 2., self.center - self.size / 2.)
    }
}
/// MeshStandardMaterial color, roughness and metalness.
#[derive(Clone, Copy)]
struct Material {
    color: u32,
    roughness: f64,
    metalness: f64,
}
const fn standard(color: u32) -> Material {
    Material {
        color,
        roughness: 1.,
        metalness: 0.,
    }
}
/// One mesh's geometry, model, material, variant, whether it casts shadows
/// and whether its shadowSide is FrontSide.
type MeshSetup = (BufferGeometry, Matrix4, Material, Kind, bool, bool);
/// One page's scene: the camera's distance, the default light intensity,
/// the lights, volumes and meshes, the variants and builds it uses, and
/// per build and variant the WGSL and its uniform numbering.
struct Setup {
    camera_z: f64,
    intensity: f64,
    lights: Vec<Light>,
    volumes: Vec<Volume>,
    meshes: Vec<MeshSetup>,
    kinds: &'static [Kind],
    stages: &'static [Stage],
    shaders: fn(Stage, Kind) -> (&'static str, &'static str),
    names: fn(Stage, Kind) -> Names,
}
/// The Cornell box's builds: GI off, GI and the capture.
fn cornell_shaders(stage: Stage, _kind: Kind) -> (&'static str, &'static str) {
    match stage {
        Stage::Main0 => (wgsl!("standard_vs"), wgsl!("standard_fs")),
        Stage::Main => (wgsl!("gi_vs"), wgsl!("gi_fs")),
        Stage::Bake1 | Stage::Bake2 => (wgsl!("bake_vs"), wgsl!("bake_fs")),
    }
}
fn cornell_names(stage: Stage, _kind: Kind) -> Names {
    if stage.bake() {
        Names::new(stage, [12, 0], 28)
    } else {
        Names::new(stage, [10, 0], 27)
    }
}
/// The Cornell box of webgpu_lightprobes: one light ( intensity 20, shadow
/// normalBias -0.02 and radius 10 ) and one 6 × 5 × 6 volume at y 2.5.
fn cornell() -> Result<Setup> {
    let wall = standard(0xcccccc);
    let object = standard(0xeeeeee);
    let t = |x, y, z| Matrix4::from_translation(Vector3::new(x, y, z));
    let (rx, ry) = (Matrix4::from_rotation_x, Matrix4::from_rotation_y);
    let floor = || PlaneGeometry::build(6., 6., 1, 1);
    let side = || PlaneGeometry::build(6., 5., 1, 1);
    let lit = Kind::Shadowed;
    Ok(Setup {
        camera_z: 8.,
        intensity: 20.,
        lights: vec![Light {
            position: Vector3::new(0., 4.5, 0.),
            color: 0xffffff,
            bias: 0.,
            normal_bias: -0.02,
            radius: 10.,
        }],
        volumes: vec![Volume {
            center: Vector3::new(0., 2.5, 0.),
            size: Vector3::new(6., 5., 6.),
        }],
        meshes: vec![
            (floor()?, rx(-PI / 2.), wall, lit, false, false),
            (
                floor()?,
                t(0., 5., 0.) * rx(PI / 2.),
                wall,
                lit,
                false,
                false,
            ),
            (side()?, t(0., 2.5, -3.), wall, lit, false, false),
            (side()?, t(0., 2.5, 3.) * ry(PI), wall, lit, false, false),
            (
                side()?,
                t(-3., 2.5, 0.) * ry(PI / 2.),
                standard(0xff0000),
                lit,
                false,
                false,
            ),
            (
                side()?,
                t(3., 2.5, 0.) * ry(-PI / 2.),
                standard(0x00ff00),
                lit,
                false,
                false,
            ),
            (
                BoxGeometry::build(1.2, 2.5, 1.2)?,
                t(-0.8, 1.25, -0.8) * ry(PI / 8.),
                object,
                lit,
                true,
                false,
            ),
            (
                BoxGeometry::build(1.2, 1.2, 1.2)?,
                t(1., 0.6, 0.5) * ry(-PI / 6.),
                object,
                lit,
                true,
                false,
            ),
            (
                SphereGeometry::build(0.5, 32, 32)?,
                t(1., 1.9, 0.5),
                object,
                lit,
                true,
                false,
            ),
        ],
        kinds: &[Kind::Shadowed],
        stages: &[Stage::Main0, Stage::Main, Stage::Bake1],
        shaders: cornell_shaders,
        names: cornell_names,
    })
}
/// One mesh: normals and positions, index, local bounding sphere, model,
/// material, variant, shadow casting and shadowSide, its object struct per
/// build and its shadow pass object.
struct Mesh {
    normals: wgpu::Buffer,
    positions: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
    material: Material,
    kind: Kind,
    caster: bool,
    shadow_front: bool,
    objects: Vec<wgpu::Buffer>,
    shadow: wgpu::Buffer,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_vertex_buffer(0, self.normals.slice(..));
        pass.set_vertex_buffer(1, self.positions.slice(..));
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    /// The MeshStandardMaterial's object uniforms in a build: color,
    /// opacity, metalness, roughness, normal matrix, emissive and its
    /// intensity, and the model.
    fn uniforms(&self, n: &Names) -> Vec<(String, Vec<f64>)> {
        let [normal, emissive, intensity, model] = n.material;
        vec![
            (
                u(0),
                Color::from_hex(self.material.color).0.to_array().to_vec(),
            ),
            (u(1), vec![1.]),
            (u(2), vec![self.material.metalness]),
            (u(3), vec![self.material.roughness]),
            (u(normal), m3(self.model.inverse().transpose())),
            (u(emissive), vec![0.; 3]),
            (u(intensity), vec![1.]),
            (u(model), m4(self.model)),
        ]
    }
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
}
/// A LightProbeGrid's atlas, its bounce grid's snapshot, and its helper's
/// instance matrices, UVWs and object struct.
struct Grid {
    atlas: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    bounce: wgpu::Texture,
    bounce_view: wgpu::TextureView,
    matrices: wgpu::Buffer,
    uvw: wgpu::Buffer,
    helper: wgpu::Buffer,
}
/// One point light's shadow: per face the color and depth layers and the
/// camera buffer, and the depth cube.
struct Shadow {
    faces: Vec<(wgpu::TextureView, wgpu::TextureView, wgpu::Buffer)>,
    cube: wgpu::TextureView,
}
/// The capture cameras' uniforms: the 48 view and projection matrices and a
/// cameraIndex buffer per sub-camera.
struct Capture {
    views: wgpu::Buffer,
    projections: wgpu::Buffer,
    indices: Vec<wgpu::Buffer>,
    faces_view: wgpu::TextureView,
    depth: wgpu::TextureView,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// Per mesh: the GI draw and the GI-off draw.
    gi: Vec<Draw>,
    standard: Vec<Draw>,
    /// Per grid, its helper.
    helpers: Vec<Draw>,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// GI enabled, resolution, bounces, light intensity and show probes.
    params: [f64; 5],
    /// The page's setup ( its meshes moved into `meshes` ).
    setup: Setup,
    meshes: Vec<Mesh>,
    /// The helper's sphere: positions, normals, index and count.
    helper_mesh: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    shadows: Vec<Shadow>,
    /// The shadow pipelines ( BackSide, and FrontSide for the shadowSide
    /// FrontSide meshes ) and the meshes' shadow groups.
    shadow_pipelines: [wgpu::RenderPipeline; 2],
    shadow_cameras: Vec<Vec<[wgpu::BindGroup; 2]>>,
    shadow_objects: Vec<[wgpu::BindGroup; 2]>,
    /// Per build and variant, the render struct.
    renders: Vec<Vec<wgpu::Buffer>>,
    /// Per build and variant, the capture builds' pipeline and its
    /// cameraIndex groups ( empty for the main camera's builds ).
    bake_pipelines: Vec<Vec<(wgpu::RenderPipeline, Vec<wgpu::BindGroup>)>>,
    capture: Capture,
    sh_pipeline: wgpu::RenderPipeline,
    sh_object: wgpu::Buffer,
    repack_pipelines: Vec<wgpu::RenderPipeline>,
    /// The baked resolution, bounces and light intensity, and the grids.
    baked: Option<(u32, u32, f64)>,
    grids: Vec<Grid>,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    /// The helpers' camera struct.
    helper_camera: Option<wgpu::Buffer>,
    targets: Option<Targets>,
}
fn attributes(locations: &[u32]) -> Vec<[wgpu::VertexAttribute; 1]> {
    locations
        .iter()
        .map(|&shader_location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location,
            }]
        })
        .collect()
}
fn layouts(attributes: &[[wgpu::VertexAttribute; 1]]) -> Vec<wgpu::VertexBufferLayout<'_>> {
    attributes
        .iter()
        .map(|a| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: a,
        })
        .collect()
}
/// A lit material's pipeline: normal at location 0, position at 1; the back
/// side winds clockwise, as three.js flips frontFace for BackSide.
fn lit_pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    samples: u32,
    back: bool,
) -> wgpu::RenderPipeline {
    let a = attributes(&[0, 1]);
    raw_pipeline(
        r,
        label,
        vs,
        fs,
        &layouts(&a),
        HALF,
        samples,
        Some((wgpu::CompareFunction::LessEqual, true)),
        back,
    )
}
/// A lit material's object group: the object struct, the DFG LUT, the
/// lights' shadow cubes ( shadowed variants ) and the grids' atlases.
fn lit_entries<'a>(
    object: &'a wgpu::Buffer,
    (linear, compare): (&'a wgpu::Sampler, &'a wgpu::Sampler),
    dfg: &'a wgpu::TextureView,
    shadows: &'a [wgpu::TextureView],
    atlases: &[&'a wgpu::TextureView],
) -> Vec<(u32, wgpu::BindingResource<'a>)> {
    let mut entries = vec![
        (0, object.as_entire_binding()),
        (1, wgpu::BindingResource::Sampler(linear)),
        (2, wgpu::BindingResource::TextureView(dfg)),
    ];
    for view in shadows {
        let n = entries.len() as u32;
        entries.extend([
            (n, wgpu::BindingResource::Sampler(compare)),
            (n + 1, wgpu::BindingResource::TextureView(view)),
        ]);
    }
    for view in atlases {
        let n = entries.len() as u32;
        entries.extend([
            (n, wgpu::BindingResource::Sampler(linear)),
            (n + 1, wgpu::BindingResource::TextureView(view)),
        ]);
    }
    entries
}
fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lightprobes target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
fn layer(t: &wgpu::Texture, i: u32) -> wgpu::TextureView {
    t.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2),
        base_array_layer: i,
        array_layer_count: Some(1),
        ..Default::default()
    })
}
fn write(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    source: &str,
    name: &str,
    values: &[(String, Vec<f64>)],
) -> Result<()> {
    let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
    r.queue
        .write_buffer(buffer, 0, &pack(source, name, &values)?);
    Ok(())
}
/// The face camera's view: Object3D.lookAt from `p` along `direction`.
fn look(p: Vector3, (direction, up): ([f64; 3], [f64; 3])) -> Matrix4 {
    Matrix4::look_at_rh(
        p,
        p + Vector3::from_array(direction),
        Vector3::from_array(up),
    )
}
/// LightProbeGridHelper's pipeline: positions, normals, UVWs and the
/// instance matrices as attributes.
fn helper_pipeline(r: &Renderer, samples: u32) -> wgpu::RenderPipeline {
    let f3 = |location| wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x3,
        offset: 0,
        shader_location: location,
    };
    let (a0, a1, a2) = ([f3(0)], [f3(1)], [f3(2)]);
    let matrix = [3, 4, 5, 6].map(|k| wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x4,
        offset: 16 * (k - 3) as u64,
        shader_location: k,
    });
    let vertex = |stride, step_mode, attributes| wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode,
        attributes,
    };
    let (v, i) = (wgpu::VertexStepMode::Vertex, wgpu::VertexStepMode::Instance);
    raw_pipeline(
        r,
        "lightprobes helper",
        shared!("helper_vs"),
        HELPER_FS,
        &[
            vertex(12, v, &a0[..]),
            vertex(12, v, &a1[..]),
            vertex(12, i, &a2[..]),
            vertex(64, i, &matrix[..]),
        ],
        HALF,
        samples,
        Some((wgpu::CompareFunction::LessEqual, true)),
        false,
    )
}
impl Demo {
    async fn create(s: &mut Scene, c: Object3D, r: &Renderer, mut setup: Setup) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 2.5, setup.camera_z);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2.5, 0.));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("lightprobes attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let mut meshes = vec![];
        let shaders = setup.shaders;
        for (g, model, material, kind, caster, shadow_front) in std::mem::take(&mut setup.meshes) {
            let positions = read(&g, "position")?;
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let (lo, hi) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) * 0.5;
            let index = g.index.clone().ok_or(Error::Invalid("lightprobes index"))?;
            let mut shadow = vec![1f32, 0., 0., 0.];
            shadow.extend(model.to_cols_array().map(|v| v as f32));
            let objects = setup
                .stages
                .iter()
                .map(|&stage| {
                    uniform(
                        r,
                        "lightprobes object",
                        shaders(stage, kind).1,
                        "objectStruct",
                    )
                })
                .collect::<Result<_>>()?;
            meshes.push(Mesh {
                normals: init(
                    "lightprobes normals",
                    bytemuck::cast_slice(&read(&g, "normal")?),
                    vertex,
                ),
                positions: init(
                    "lightprobes positions",
                    bytemuck::cast_slice(&positions),
                    vertex,
                ),
                index: init(
                    "lightprobes index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
                model,
                material,
                kind,
                caster,
                shadow_front,
                objects,
                shadow: init(
                    "lightprobes shadow object",
                    bytemuck::cast_slice(&shadow),
                    wgpu::BufferUsages::UNIFORM,
                ),
            });
        }
        let helper = SphereGeometry::build(0.12, 16, 16)?;
        let helper_index = helper
            .index
            .clone()
            .ok_or(Error::Invalid("lightprobes index"))?;
        let helper_mesh = (
            init(
                "helper positions",
                bytemuck::cast_slice(&read(&helper, "position")?),
                vertex,
            ),
            init(
                "helper normals",
                bytemuck::cast_slice(&read(&helper, "normal")?),
                vertex,
            ),
            init(
                "helper index",
                bytemuck::cast_slice(&helper_index),
                wgpu::BufferUsages::INDEX,
            ),
            helper_index.len() as u32,
        );
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        // The point shadows: FrontSide casters draw their back faces
        // ( clockwise ), the shadowSide FrontSide ones their front faces.
        let position = attributes(&[0]);
        let shadow_pipelines = [true, false].map(|cw| {
            raw_pipeline(
                r,
                "lightprobes shadow",
                shared!("shadow_vs"),
                shared!("shadow_fs"),
                &layouts(&position),
                wgpu::TextureFormat::Rgba8Unorm,
                1,
                Some((wgpu::CompareFunction::LessEqual, true)),
                cw,
            )
        });
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., SHADOW_NEAR, SHADOW_FAR);
        let mut shadows = vec![];
        let mut shadow_cameras = vec![];
        for light in &setup.lights {
            let color = texture(
                r,
                (SHADOW, SHADOW, 6),
                wgpu::TextureDimension::D2,
                wgpu::TextureFormat::Rgba8Unorm,
            );
            let depth = texture(r, (SHADOW, SHADOW, 6), wgpu::TextureDimension::D2, DEPTH);
            let mut faces = vec![];
            let mut groups = vec![];
            for (i, face) in SHADOW_FACES.iter().enumerate() {
                let mut data: Vec<f32> = face_projection.to_cols_array().map(|v| v as f32).to_vec();
                data.extend(
                    look(light.position, *face)
                        .to_cols_array()
                        .map(|v| v as f32),
                );
                let buffer = init(
                    "lightprobes shadow camera",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::UNIFORM,
                );
                groups.push(shadow_pipelines.clone().map(|p| {
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, buffer.as_entire_binding())],
                    )
                }));
                faces.push((layer(&color, i as u32), layer(&depth, i as u32), buffer));
            }
            shadows.push(Shadow {
                faces,
                cube: depth.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::Cube),
                    ..Default::default()
                }),
            });
            shadow_cameras.push(groups);
        }
        let shadow_objects = meshes
            .iter()
            .map(|m| {
                shadow_pipelines.clone().map(|p| {
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[(0, m.shadow.as_entire_binding())],
                    )
                })
            })
            .collect();
        let renders = setup
            .stages
            .iter()
            .map(|&stage| {
                setup
                    .kinds
                    .iter()
                    .map(|&kind| {
                        uniform(
                            r,
                            "lightprobes render",
                            shaders(stage, kind).1,
                            "renderStruct",
                        )
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        // The capture: ArrayCamera's 48 sub-cameras into the face atlas.
        let matrices = |label| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: (BATCH * 6 * 64) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let faces = texture(
            r,
            (6 * CUBE, BATCH * CUBE, 1),
            wgpu::TextureDimension::D2,
            HALF,
        );
        let capture = Capture {
            views: matrices("lightprobes capture views"),
            projections: matrices("lightprobes capture projections"),
            indices: (0..BATCH * 6)
                .map(|i| {
                    init(
                        "lightprobes camera index",
                        bytemuck::cast_slice(&[i, 0, 0, 0]),
                        wgpu::BufferUsages::UNIFORM,
                    )
                })
                .collect(),
            faces_view: faces.create_view(&Default::default()),
            depth: texture(
                r,
                (6 * CUBE, BATCH * CUBE, 1),
                wgpu::TextureDimension::D2,
                DEPTH,
            )
            .create_view(&Default::default()),
        };
        let bake_pipelines = setup
            .stages
            .iter()
            .map(|&stage| {
                let kinds = if stage.bake() { setup.kinds } else { &[] };
                kinds
                    .iter()
                    .map(|&kind| {
                        let pipeline = lit_pipeline(
                            r,
                            "lightprobes bake",
                            shaders(stage, kind),
                            1,
                            kind == Kind::Back,
                        );
                        let groups = capture
                            .indices
                            .iter()
                            .map(|b| {
                                bind(
                                    r,
                                    pipeline.get_bind_group_layout(1),
                                    &[(0, b.as_entire_binding())],
                                )
                            })
                            .collect();
                        (pipeline, groups)
                    })
                    .collect()
            })
            .collect();
        let sh_pipeline = raw_pipeline(
            r,
            "lightprobes SH",
            shared!("probe_vs"),
            PROBE_SH_FS,
            &[],
            FLOAT,
            1,
            None,
            false,
        );
        let repack_pipelines = REPACK_FS
            .iter()
            .map(|fs| {
                raw_pipeline(
                    r,
                    "lightprobes repack",
                    shared!("probe_vs"),
                    fs,
                    &[],
                    HALF,
                    1,
                    None,
                    false,
                )
            })
            .collect();
        Ok(Self {
            controls,
            params: [1., 6., 3., setup.intensity, 0.],
            setup,
            meshes,
            helper_mesh,
            shadows,
            shadow_pipelines,
            shadow_cameras,
            shadow_objects,
            renders,
            bake_pipelines,
            capture,
            sh_pipeline,
            sh_object: uniform(r, "lightprobes SH object", PROBE_SH_FS, "objectStruct")?,
            repack_pipelines,
            baked: None,
            grids: vec![],
            output_render: uniform(r, "lightprobes output render", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "lightprobes output object", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "lightprobes quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            linear,
            compare,
            helper_camera: None,
            targets: None,
        })
    }
    /// A build's index in the setup's builds.
    fn stage_index(&self, stage: Stage) -> Result<usize> {
        self.setup
            .stages
            .iter()
            .position(|&x| x == stage)
            .ok_or(Error::Invalid("lightprobes build"))
    }
    /// A variant's index in the setup's variants.
    fn kind_index(&self, kind: Kind) -> usize {
        self.setup
            .kinds
            .iter()
            .position(|&x| x == kind)
            .unwrap_or(0)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.frame_update(s, c)
    }
    /// The lights' render uniforms in a build: per light its position
    /// ( in view space for the main camera, in world space for the capture
    /// cameras, which transform it per sub-camera ), color, the shadow's
    /// matrix, normalBias, far, near, bias, radius and map size, the
    /// filter's far, near and bias, the shadow intensity, the cutoff
    /// distance and the decay.
    fn light_uniforms(&self, stage: Stage, kind: Kind, view: Matrix4) -> Vec<(String, Vec<f64>)> {
        let intensity = self.baked.map_or(self.params[3], |b| b.2);
        let n = (self.setup.names)(stage, kind);
        let mut values = vec![];
        for (i, (l, &b)) in self.setup.lights.iter().zip(&n.lights).enumerate() {
            let color = Color::from_hex(l.color)
                .0
                .to_array()
                .map(|v| v * intensity)
                .to_vec();
            let position = if stage.bake() {
                l.position
            } else {
                view.transform_point3(l.position)
            };
            values.extend([(u(b), position.to_array().to_vec()), (u(b + 1), color)]);
            if kind == Kind::Plain {
                values.extend([(u(b + 2), vec![0.]), (u(b + 3), vec![2.])]);
                continue;
            }
            // The main camera's builds number one more uniform after the
            // first light's shadow matrix.
            let k = b + usize::from(!stage.bake() && i == 0);
            values.extend([
                (u(b + 2), m4(Matrix4::from_translation(-l.position))),
                (u(k + 3), vec![l.normal_bias]),
                (u(k + 4), vec![SHADOW_FAR]),
                (u(k + 5), vec![SHADOW_NEAR]),
                (u(k + 6), vec![l.bias]),
                (u(k + 7), vec![l.radius]),
                (u(k + 8), vec![SHADOW as f64; 2]),
                (u(k + 9), vec![SHADOW_FAR]),
                (u(k + 10), vec![SHADOW_NEAR]),
                (u(k + 11), vec![l.bias]),
                (u(k + 13), vec![1.]),
                (u(k + 14), vec![0.]),
                (u(k + 15), vec![2.]),
            ]);
        }
        values
    }
    /// The grids' uniforms in a build, in the lights' order: bounds max and
    /// min and resolution ( object ), and priority and intensity ( render ).
    fn grid_uniforms(
        &self,
        stage: Stage,
        kind: Kind,
        grids: &[(Volume, f64)],
        res: u32,
    ) -> (Uniforms, Uniforms) {
        let base = (self.setup.names)(stage, kind).grids;
        let (mut object, mut render) = (vec![], vec![]);
        for (j, (volume, intensity)) in grids.iter().enumerate() {
            let g = base + 6 * j;
            let (max, min) = volume.bounds();
            object.extend([
                (u(g), max.to_array().to_vec()),
                (u(g + 1), min.to_array().to_vec()),
                (u(g + 2), vec![res as f64; 3]),
            ]);
            render.extend([(u(g + 3), vec![0.]), (u(g + 5), vec![*intensity])]);
        }
        (object, render)
    }
    /// The point lights' cube shadows, each face culling the casters.
    fn encode_shadows(&self, encoder: &mut wgpu::CommandEncoder) {
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., SHADOW_NEAR, SHADOW_FAR);
        for (l, (light, shadow)) in self.setup.lights.iter().zip(&self.shadows).enumerate() {
            for (i, (color, depth, _)) in shadow.faces.iter().enumerate() {
                let frustum = Frustum::from_projection(
                    face_projection * look(light.position, SHADOW_FACES[i]),
                );
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Discard,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                for (k, m) in self.meshes.iter().enumerate() {
                    if !m.caster || !m.visible(&frustum) {
                        continue;
                    }
                    let side = usize::from(m.shadow_front);
                    pass.set_pipeline(&self.shadow_pipelines[side]);
                    pass.set_bind_group(0, &self.shadow_cameras[l][i][side], &[]);
                    pass.set_bind_group(1, &self.shadow_objects[k][side], &[]);
                    pass.set_vertex_buffer(0, m.positions.slice(..));
                    pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..1);
                }
            }
        }
    }
    /// The page's bake( resolution ): new volumes, then for each pass ( the
    /// direct one and `bounces` more ) each volume's bake in turn, then the
    /// helpers' update.
    fn bake(&mut self, r: &Renderer, res: u32, bounces: u32) -> Result<()> {
        self.baked = Some((res, bounces, self.params[3]));
        let total = res * res * res;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        self.grids = self
            .setup
            .volumes
            .iter()
            .map(|volume| {
                let atlas_size = (res, res, 7 * (res + 2));
                let atlas = texture(r, atlas_size, wgpu::TextureDimension::D3, HALF);
                let bounce = texture(r, atlas_size, wgpu::TextureDimension::D3, HALF);
                // LightProbeGridHelper.update(): instance matrices and
                // texel-center UVWs.
                let mut matrices: Vec<f32> = vec![];
                let mut uvw: Vec<f32> = vec![];
                for iz in 0..res {
                    for iy in 0..res {
                        for ix in 0..res {
                            uvw.extend(
                                [ix, iy, iz].map(|i| ((i as f64 + 0.5) / res as f64) as f32),
                            );
                            let p = volume.probe(res, [ix, iy, iz]);
                            matrices.extend(
                                Matrix4::from_translation(p)
                                    .to_cols_array()
                                    .map(|v| v as f32),
                            );
                        }
                    }
                }
                Ok(Grid {
                    atlas_view: atlas.create_view(&Default::default()),
                    atlas,
                    bounce_view: bounce.create_view(&Default::default()),
                    bounce,
                    matrices: init(
                        "helper matrices",
                        bytemuck::cast_slice(&matrices),
                        wgpu::BufferUsages::VERTEX,
                    ),
                    uvw: init(
                        "helper uvw",
                        bytemuck::cast_slice(&uvw),
                        wgpu::BufferUsages::VERTEX,
                    ),
                    helper: uniform(r, "lightprobes helper object", HELPER_FS, "objectStruct")?,
                })
            })
            .collect::<Result<_>>()?;
        let batch = texture(r, (COLUMNS, total, 1), wgpu::TextureDimension::D2, FLOAT)
            .create_view(&Default::default());
        let shadow_cubes: Vec<wgpu::TextureView> =
            self.shadows.iter().map(|s| s.cube.clone()).collect();
        // The shadow maps render once for the bake ( and every frame after ).
        let mut encoder = r.device.create_command_encoder(&Default::default());
        self.encode_shadows(&mut encoder);
        r.queue.submit([encoder.finish()]);
        for pass in 0..=bounces {
            for g in 0..self.setup.volumes.len() {
                let mut encoder = r.device.create_command_encoder(&Default::default());
                // _updateBounceGrid: the snapshot of the previous pass.
                if pass > 0 {
                    encoder.copy_texture_to_texture(
                        self.grids[g].atlas.as_image_copy(),
                        self.grids[g].bounce.as_image_copy(),
                        wgpu::Extent3d {
                            width: res,
                            height: res,
                            depth_or_array_layers: 7 * (res + 2),
                        },
                    );
                }
                r.queue.submit([encoder.finish()]);
                // The scene's grids by creation: the direct pass sees the
                // unlit bounce grid alone; later passes the other volumes,
                // then the bounce grid ( created after them ).
                let volumes = &self.setup.volumes;
                let (mut grids, mut atlases): (Vec<(Volume, f64)>, Vec<&wgpu::TextureView>) =
                    (vec![], vec![]);
                if pass > 0 {
                    for (o, other) in self.grids.iter().enumerate().filter(|&(o, _)| o != g) {
                        grids.push((volumes[o], 1.));
                        atlases.push(&other.atlas_view);
                    }
                }
                grids.push((volumes[g], if pass == 0 { 0. } else { 1. }));
                atlases.push(&self.grids[g].bounce_view);
                let stage = if grids.len() == 1 {
                    Stage::Bake1
                } else {
                    Stage::Bake2
                };
                let s = self.stage_index(stage)?;
                let pipelines = &self.bake_pipelines[s];
                for (k, &kind) in self.setup.kinds.iter().enumerate() {
                    let mut values = self.light_uniforms(stage, kind, Matrix4::IDENTITY);
                    values.extend(self.grid_uniforms(stage, kind, &grids, res).1);
                    write(
                        r,
                        &self.renders[s][k],
                        (self.setup.shaders)(stage, kind).1,
                        "renderStruct",
                        &values,
                    )?;
                }
                let mut draws = vec![];
                for m in &self.meshes {
                    let n = (self.setup.names)(stage, m.kind);
                    let mut values = m.uniforms(&n);
                    values.extend(self.grid_uniforms(stage, m.kind, &grids, res).0);
                    let fs = (self.setup.shaders)(stage, m.kind).1;
                    write(r, &m.objects[s], fs, "objectStruct", &values)?;
                    let k = self.kind_index(m.kind);
                    let pipeline = &pipelines[k].0;
                    let shadowed: &[wgpu::TextureView] = if m.kind == Kind::Plain {
                        &[]
                    } else {
                        &shadow_cubes
                    };
                    let render = bind(
                        r,
                        pipeline.get_bind_group_layout(0),
                        &[
                            (0, self.capture.views.as_entire_binding()),
                            (1, self.renders[s][k].as_entire_binding()),
                            (2, self.capture.views.as_entire_binding()),
                            (3, self.capture.projections.as_entire_binding()),
                        ],
                    );
                    let object = bind(
                        r,
                        pipeline.get_bind_group_layout(2),
                        &lit_entries(
                            &m.objects[s],
                            (&self.linear, &self.compare),
                            &r.dfg,
                            shadowed,
                            &atlases,
                        ),
                    );
                    draws.push((k, render, object));
                }
                self.capture_probes(r, g, res, &batch, pipelines, &draws)?;
                self.repack(r, g, res, &batch)?;
            }
        }
        self.targets = None;
        Ok(())
    }
    /// _captureProbes: per batch of 8 probes the 48 sub-cameras' render into
    /// the face atlas ( unused slots draw nothing ), then the SH projection
    /// into the batch rows, one submit per batch.
    fn capture_probes(
        &self,
        r: &Renderer,
        g: usize,
        res: u32,
        batch: &wgpu::TextureView,
        pipelines: &[(wgpu::RenderPipeline, Vec<wgpu::BindGroup>)],
        draws: &[(usize, wgpu::BindGroup, wgpu::BindGroup)],
    ) -> Result<()> {
        let total = res * res * res;
        let volume = self.setup.volumes[g];
        let projection = Matrix4::perspective_rh(PI / 2., 1., CUBE_NEAR, CUBE_FAR);
        let sh_group = bind(
            r,
            self.sh_pipeline.get_bind_group_layout(0),
            &[
                (0, self.sh_object.as_entire_binding()),
                (
                    1,
                    wgpu::BindingResource::TextureView(&self.capture.faces_view),
                ),
            ],
        );
        for start in (0..total).step_by(BATCH as usize) {
            let count = BATCH.min(total - start);
            let mut views: Vec<Matrix4> = vec![Matrix4::IDENTITY; (BATCH * 6) as usize];
            for slot in 0..count {
                let probe = start + slot;
                let (ix, iy, iz) = (probe % res, probe / (res * res), (probe / res) % res);
                let p = volume.probe(res, [ix, iy, iz]);
                for (face, f) in CAPTURE_FACES.iter().enumerate() {
                    views[(slot * 6) as usize + face] = look(p, *f);
                }
            }
            let flat = |m: &[Matrix4]| -> Vec<f32> {
                m.iter()
                    .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                    .collect()
            };
            r.queue
                .write_buffer(&self.capture.views, 0, bytemuck::cast_slice(&flat(&views)));
            r.queue.write_buffer(
                &self.capture.projections,
                0,
                bytemuck::cast_slice(&flat(&vec![projection; (BATCH * 6) as usize])),
            );
            write(
                r,
                &self.sh_object,
                PROBE_SH_FS,
                "objectStruct",
                &[
                    (u(0), vec![1.]),
                    (u(1), vec![start as f64]),
                    (u(3), m3(Matrix4::IDENTITY)),
                ],
            )?;
            let mut encoder = r.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes capture"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.capture.faces_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: BACKGROUND,
                                g: BACKGROUND,
                                b: BACKGROUND,
                                a: 1.,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.capture.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                for camera in 0..(count * 6) as usize {
                    let (slot, face) = (camera / 6, camera % 6);
                    pass.set_viewport(
                        (face as u32 * CUBE) as f32,
                        (slot as u32 * CUBE) as f32,
                        CUBE as f32,
                        CUBE as f32,
                        0.,
                        1.,
                    );
                    let frustum = Frustum::from_projection(projection * views[camera]);
                    for (m, (k, render, object)) in self.meshes.iter().zip(draws) {
                        if !m.visible(&frustum) {
                            continue;
                        }
                        let (pipeline, indices) = &pipelines[*k];
                        pass.set_pipeline(pipeline);
                        pass.set_bind_group(0, render, &[]);
                        pass.set_bind_group(1, &indices[camera], &[]);
                        pass.set_bind_group(2, object, &[]);
                        m.draw(&mut pass);
                    }
                }
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes SH"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: batch,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_viewport(0., start as f32, COLUMNS as f32, count as f32, 0., 1.);
                pass.set_pipeline(&self.sh_pipeline);
                pass.set_bind_group(0, &sh_group, &[]);
                pass.draw(0..3, 0..1);
            }
            r.queue.submit([encoder.finish()]);
        }
        Ok(())
    }
    /// _repackProbes: the seven SH textures per Z slice ( one rectangle of
    /// the whole slice ), into the slice and, at the ends, the padding.
    fn repack(&self, r: &Renderer, g: usize, res: u32, batch: &wgpu::TextureView) -> Result<()> {
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut objects = vec![];
        for iz in 0..res {
            let object = uniform(r, "lightprobes repack object", REPACK_FS[0], "objectStruct")?;
            write(
                r,
                &object,
                REPACK_FS[0],
                "objectStruct",
                &[
                    (u(0), vec![1.]),
                    (u(1), vec![iz as f64]),
                    (u(2), vec![res as f64; 3]),
                    (u(4), m3(Matrix4::IDENTITY)),
                ],
            )?;
            objects.push(object);
        }
        for (iz, object) in objects.iter().enumerate() {
            let iz = iz as u32;
            for (t, pipeline) in self.repack_pipelines.iter().enumerate() {
                let group = bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[
                        (0, object.as_entire_binding()),
                        (1, wgpu::BindingResource::TextureView(batch)),
                    ],
                );
                let base = t as u32 * (res + 2);
                let mut slices = vec![base + 1 + iz];
                if iz == 0 {
                    slices.push(base);
                }
                if iz == res - 1 {
                    slices.push(base + 1 + res);
                }
                for slice in slices {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("lightprobes repack"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &self.grids[g].atlas_view,
                            depth_slice: Some(slice),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &group, &[]);
                    pass.draw(0..3, 0..1);
                }
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let shadow_cubes: Vec<wgpu::TextureView> =
            self.shadows.iter().map(|s| s.cube.clone()).collect();
        let atlases: Vec<&wgpu::TextureView> = self.grids.iter().map(|g| &g.atlas_view).collect();
        let res = self.baked.map_or(6, |b| b.0);
        let grids: Vec<(Volume, f64)> = self.setup.volumes.iter().map(|v| (*v, 1.)).collect();
        let samplers = (&self.linear, &self.compare);
        let mut draws = [vec![], vec![]];
        for (d, stage) in [Stage::Main0, Stage::Main].into_iter().enumerate() {
            let i = self.stage_index(stage)?;
            let n = if stage == Stage::Main { grids.len() } else { 0 };
            let pipelines: Vec<wgpu::RenderPipeline> = self
                .setup
                .kinds
                .iter()
                .map(|&kind| {
                    lit_pipeline(
                        r,
                        "lightprobes",
                        (self.setup.shaders)(stage, kind),
                        samples,
                        kind == Kind::Back,
                    )
                })
                .collect();
            for m in &self.meshes {
                let k = self.kind_index(m.kind);
                let mut values = m.uniforms(&(self.setup.names)(stage, m.kind));
                values.extend(self.grid_uniforms(stage, m.kind, &grids[..n], res).0);
                write(
                    r,
                    &m.objects[i],
                    (self.setup.shaders)(stage, m.kind).1,
                    "objectStruct",
                    &values,
                )?;
                let shadowed: &[wgpu::TextureView] = if m.kind == Kind::Plain {
                    &[]
                } else {
                    &shadow_cubes
                };
                let pipeline = &pipelines[k];
                draws[d].push((
                    pipeline.clone(),
                    vec![
                        bind(
                            r,
                            pipeline.get_bind_group_layout(0),
                            &[(0, self.renders[i][k].as_entire_binding())],
                        ),
                        bind(
                            r,
                            pipeline.get_bind_group_layout(1),
                            &lit_entries(&m.objects[i], samplers, &r.dfg, shadowed, &atlases[..n]),
                        ),
                    ],
                ));
            }
        }
        let [standard, gi] = draws;
        let helper_pipeline = helper_pipeline(r, samples);
        // The helper's render struct: the camera's projection and view.
        let helper_camera = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lightprobes helper camera"),
            size: 128,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let helper_render = bind(
            r,
            helper_pipeline.get_bind_group_layout(0),
            &[(0, helper_camera.as_entire_binding())],
        );
        let helpers = self
            .grids
            .iter()
            .map(|g| {
                (
                    helper_pipeline.clone(),
                    vec![
                        helper_render.clone(),
                        bind(
                            r,
                            helper_pipeline.get_bind_group_layout(1),
                            &[
                                (0, wgpu::BindingResource::Sampler(&self.linear)),
                                (1, wgpu::BindingResource::TextureView(&g.atlas_view)),
                                (2, g.helper.as_entire_binding()),
                            ],
                        ),
                    ],
                )
            })
            .collect();
        let color = target(r, size, HALF, 1);
        let quad = attributes(&[0]);
        let output_pipeline = raw_pipeline(
            r,
            "lightprobes output",
            OUTPUT_VS,
            OUTPUT_FS,
            &layouts(&quad),
            out.options.format,
            1,
            None,
            false,
        );
        let output = (
            output_pipeline.clone(),
            vec![
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(0),
                    &[(0, self.output_render.as_entire_binding())],
                ),
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(&color)),
                        (2, self.output_object.as_entire_binding()),
                    ],
                ),
            ],
        );
        self.helper_camera = Some(helper_camera);
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            msaa: (samples > 1).then(|| {
                (
                    target(r, size, HALF, samples),
                    target(r, size, DEPTH, samples),
                )
            }),
            depth: target(r, size, DEPTH, 1),
            color,
            gi,
            standard,
            helpers,
            output,
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
        let [enabled, resolution, bounces, intensity, show] = self.params;
        let res = resolution.round().clamp(2., 12.) as u32;
        let bounces = bounces.round().clamp(0., 10.) as u32;
        // A changed resolution, bounce count or light intensity rebakes
        // before the frame ( pendingBake ).
        if self.baked != Some((res, bounces, intensity)) {
            self.bake(r, res, bounces)?;
        }
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != if out.options.samples > 1 { 4 } else { 1 }
        }) {
            self.resize(r, out)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("lightprobes targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let grids: Vec<(Volume, f64)> = self.setup.volumes.iter().map(|v| (*v, 1.)).collect();
        for stage in [Stage::Main0, Stage::Main] {
            let i = self.stage_index(stage)?;
            let n = if stage == Stage::Main { grids.len() } else { 0 };
            for (k, &kind) in self.setup.kinds.iter().enumerate() {
                let mut values = vec![
                    ("cameraProjectionMatrix".to_string(), m4(projection)),
                    ("cameraViewMatrix".to_string(), m4(view)),
                ];
                values.extend(self.light_uniforms(stage, kind, view));
                values.extend(self.grid_uniforms(stage, kind, &grids[..n], res).1);
                write(
                    r,
                    &self.renders[i][k],
                    (self.setup.shaders)(stage, kind).1,
                    "renderStruct",
                    &values,
                )?;
            }
        }
        let camera_data: Vec<f32> = [projection, view]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|v| v as f32))
            .collect();
        if let Some(b) = &self.helper_camera {
            r.queue
                .write_buffer(b, 0, bytemuck::cast_slice(&camera_data));
        }
        for g in &self.grids {
            write(
                r,
                &g.helper,
                HELPER_FS,
                "objectStruct",
                &[
                    (u(5), vec![res as f64; 3]),
                    (u(7), m3(Matrix4::IDENTITY)),
                    (u(10), m4(Matrix4::IDENTITY)),
                ],
            )?;
        }
        write(
            r,
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix".into(),
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix".into(), m4(Matrix4::IDENTITY)),
                ("nodeUniform1".into(), vec![t.width as f64, t.height as f64]),
                ("nodeUniform2".into(), vec![1.]),
            ],
        )?;
        write(
            r,
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5".into(), m4(Matrix4::IDENTITY))],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        self.encode_shadows(&mut encoder);
        let frustum = Frustum::from_projection(projection * view);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lightprobes scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.msaa.as_ref().map_or(&t.color, |m| &m.0),
                    depth_slice: None,
                    resolve_target: t.msaa.as_ref().map(|_| &t.color),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: BACKGROUND,
                            g: BACKGROUND,
                            b: BACKGROUND,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            let draws = if enabled > 0.5 { &t.gi } else { &t.standard };
            for (m, draw) in self.meshes.iter().zip(draws) {
                if m.visible(&frustum) {
                    set(&mut pass, draw);
                    m.draw(&mut pass);
                }
            }
            if show > 0.5 {
                let (positions, normals, index, count) = &self.helper_mesh;
                for (g, draw) in self.grids.iter().zip(&t.helpers) {
                    set(&mut pass, draw);
                    pass.set_vertex_buffer(0, positions.slice(..));
                    pass.set_vertex_buffer(1, normals.slice(..));
                    pass.set_vertex_buffer(2, g.uvw.slice(..));
                    pass.set_vertex_buffer(3, g.matrices.slice(..));
                    pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..*count, 0, 0..res.pow(3));
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lightprobes output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            set(&mut pass, &t.output);
            pass.set_vertex_buffer(0, self.output_quad.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
    }
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
    /// GI, resolution, bounces, light intensity ( these three rebake ) and
    /// show probes, in the GUI's order.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("lightprobes parameter"))? = value as f64;
        Ok(())
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        if let Some(target) = self.output() {
            return Some(Some(target));
        }
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        Some(Ok(()))
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_lightprobes.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lightprobes(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 4,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                100.,
                Box::new(Demo::create(scene, camera, renderer, cornell()?).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_lightprobes_complex.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lightprobes_complex(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 4,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                100.,
                Box::new(Demo::create(scene, camera, renderer, rooms::setup()?).await?),
            ))
        },
    )
    .await
}
