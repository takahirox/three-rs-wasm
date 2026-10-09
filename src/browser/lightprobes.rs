//! webgpu_lightprobes and webgpu_lightprobes_complex: rooms lit by
//! shadow-casting point lights, with diffuse GI from LightProbeGrid volumes
//! baked on the GPU. The Cornell box has one light and one 6×6×6 grid; the
//! complex scene two rooms joined by a doorway, a warm and a cool light and
//! two grids that fade past their bounds (falloff 1). Each grid bakes per
//! probe a 32² HalfFloat CubeCamera capture of the scene (no GI, as the grids
//! are out of the scene while baking), the 512-direction Fibonacci SH
//! projection into a row of the 9×N float batch target, and then the seven
//! repack passes into its padded RGBA16F 3D atlas. The scene renders with
//! 4× MSAA through the lit materials sampling the atlases, then the ACES
//! output pass. Every stage runs the WGSL three.js r186 generates for the
//! pages (in `lightprobes/` and `lightprobes_complex/`; the point-shadow pass
//! is the godrays one and the output pass the cubemap_dynamic one, which are
//! byte-identical).
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
/// Color( 0x111111 ) in linear space: the background clear.
const BACKGROUND: f64 = 0.005605391621829108;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("lightprobes/", $name, ".wgsl"))
    };
}
macro_rules! complex {
    ($name:literal) => {
        include_str!(concat!("lightprobes_complex/", $name, ".wgsl"))
    };
}
const SHADOW_VS: &str = include_str!("godrays/shadow_vs.wgsl");
const SHADOW_FS: &str = include_str!("godrays/shadow_fs.wgsl");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
const REPACK_FS: [&str; 7] = [
    include_str!("../shaders/light_probe_grid/repack_0_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_1_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_2_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_3_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_4_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_5_fs.wgsl"),
    include_str!("../shaders/light_probe_grid/repack_6_fs.wgsl"),
];
/// PointShadowNode's WebGPU cube faces: directions and ups.
const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., -1., 0.]),
    ([-1., 0., 0.], [0., -1., 0.]),
    ([0., -1., 0.], [0., 0., -1.]),
    ([0., 1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., -1., 0.]),
    ([0., 0., -1.], [0., -1., 0.]),
];
/// The lit materials' variants: front side receiving shadows, front side
/// without them (receiveShadow false) and back side (the complex room shell).
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Shadowed,
    Plain,
    Back,
}
const KINDS: [Kind; 3] = [Kind::Shadowed, Kind::Plain, Kind::Back];
impl Kind {
    fn index(self) -> usize {
        self as usize
    }
}
/// Per light, the uniform names of the shadowed variants: color, distance,
/// decay, view position, shadow matrix, normalBias, shadow intensity, near,
/// far, bias, radius and map size.
const SHADOWED_LIGHTS: [[&str; 12]; 2] = [
    [
        "nodeUniform11",
        "nodeUniform22",
        "nodeUniform23",
        "nodeUniform10",
        "nodeUniform12",
        "nodeUniform14",
        "nodeUniform21",
        "nodeUniform16",
        "nodeUniform15",
        "nodeUniform17",
        "nodeUniform19",
        "nodeUniform20",
    ],
    [
        "nodeUniform25",
        "nodeUniform35",
        "nodeUniform36",
        "nodeUniform24",
        "nodeUniform26",
        "nodeUniform27",
        "nodeUniform34",
        "nodeUniform29",
        "nodeUniform28",
        "nodeUniform30",
        "nodeUniform32",
        "nodeUniform33",
    ],
];
/// The same for the variant without shadows: color, distance, decay and view position.
const PLAIN_LIGHTS: [[&str; 4]; 2] = [
    [
        "nodeUniform11",
        "nodeUniform12",
        "nodeUniform13",
        "nodeUniform10",
    ],
    [
        "nodeUniform15",
        "nodeUniform16",
        "nodeUniform17",
        "nodeUniform14",
    ],
];
/// Per grid, the GI object uniforms: bounds max and min, resolution,
/// intensity and falloff (the Cornell box's grid has none).
const BOX_GRID: [[&str; 5]; 1] = [[
    "nodeUniform25",
    "nodeUniform26",
    "nodeUniform27",
    "nodeUniform28",
    "",
]];
const COMPLEX_GRIDS: [[&str; 5]; 2] = [
    [
        "nodeUniform38",
        "nodeUniform39",
        "nodeUniform40",
        "nodeUniform41",
        "nodeUniform42",
    ],
    [
        "nodeUniform44",
        "nodeUniform45",
        "nodeUniform46",
        "nodeUniform47",
        "nodeUniform48",
    ],
];
const COMPLEX_PLAIN_GRIDS: [[&str; 5]; 2] = [
    [
        "nodeUniform20",
        "nodeUniform21",
        "nodeUniform22",
        "nodeUniform23",
        "nodeUniform24",
    ],
    [
        "nodeUniform26",
        "nodeUniform27",
        "nodeUniform28",
        "nodeUniform29",
        "nodeUniform30",
    ],
];
/// A shadow-casting PointLight: position, color, intensity and its shadow's
/// bias, normalBias and radius.
struct PointShadowLight {
    position: Vector3,
    color: u32,
    intensity: f64,
    bias: f64,
    normal_bias: f64,
    radius: f64,
}
/// A LightProbeGrid: center, size and falloff.
struct Volume {
    center: Vector3,
    size: Vector3,
    falloff: f64,
}
impl Volume {
    /// LightProbeGrid.getProbePosition.
    fn probe(&self, res: u32, i: [u32; 3]) -> Vector3 {
        let (c, s) = (self.center.to_array(), self.size.to_array());
        let axis = |k: usize| {
            if res > 1 {
                c[k] - s[k] / 2. + i[k] as f64 * s[k] / (res - 1) as f64
            } else {
                c[k]
            }
        };
        Vector3::new(axis(0), axis(1), axis(2))
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
/// One example's scene: camera distance, lights, grids and meshes, and the
/// WGSL per material variant ( bake / GI-off vertex and fragment, GI vertex
/// and fragment ).
struct Setup {
    complex: bool,
    camera_z: f64,
    lights: Vec<PointShadowLight>,
    volumes: Vec<Volume>,
    meshes: Vec<(BufferGeometry, Matrix4, Material, Kind, bool)>,
}
impl Setup {
    fn shaders(&self, kind: Kind) -> [&'static str; 4] {
        match (self.complex, kind) {
            (false, _) => [
                wgsl!("standard_vs"),
                wgsl!("standard_fs"),
                wgsl!("gi_vs"),
                wgsl!("gi_fs"),
            ],
            (true, Kind::Shadowed) => [
                complex!("shadowed_vs"),
                complex!("shadowed_fs"),
                complex!("gi_vs"),
                complex!("gi_fs"),
            ],
            (true, Kind::Plain) => [
                complex!("plain_vs"),
                complex!("plain_fs"),
                complex!("gi_plain_vs"),
                complex!("gi_plain_fs"),
            ],
            (true, Kind::Back) => [
                complex!("back_vs"),
                complex!("back_fs"),
                complex!("gi_vs"),
                complex!("gi_back_fs"),
            ],
        }
    }
    fn grid_names(&self, kind: Kind) -> &'static [[&'static str; 5]] {
        match (self.complex, kind) {
            (false, _) => &BOX_GRID,
            (true, Kind::Plain) => &COMPLEX_PLAIN_GRIDS,
            _ => &COMPLEX_GRIDS,
        }
    }
    /// The lights' render uniforms for a variant and camera view.
    fn light_uniforms(&self, kind: Kind, view: Matrix4) -> Vec<(&'static str, Vec<f64>)> {
        let mut values = vec![];
        for (i, l) in self.lights.iter().enumerate() {
            let color = Color::from_hex(l.color)
                .0
                .to_array()
                .map(|v| v * l.intensity)
                .to_vec();
            let position = view.transform_point3(l.position).to_array().to_vec();
            if kind == Kind::Plain {
                let n = PLAIN_LIGHTS[i];
                values.extend([
                    (n[0], color),
                    (n[1], vec![0.]),
                    (n[2], vec![2.]),
                    (n[3], position),
                ]);
            } else {
                let n = SHADOWED_LIGHTS[i];
                values.extend([
                    (n[0], color),
                    (n[1], vec![0.]),
                    (n[2], vec![2.]),
                    (n[3], position),
                    (n[4], m4(Matrix4::from_translation(-l.position))),
                    (n[5], vec![l.normal_bias]),
                    (n[6], vec![1.]),
                    (n[7], vec![SHADOW_NEAR]),
                    (n[8], vec![SHADOW_FAR]),
                    (n[9], vec![l.bias]),
                    (n[10], vec![l.radius]),
                    (n[11], vec![SHADOW as f64; 2]),
                ]);
            }
        }
        values
    }
}
/// The Cornell box of webgpu_lightprobes.
fn cornell() -> Result<Setup> {
    let wall = standard(0xcccccc);
    let object = standard(0xeeeeee);
    let t = |x, y, z| Matrix4::from_translation(Vector3::new(x, y, z));
    let (rx, ry) = (Matrix4::from_rotation_x, Matrix4::from_rotation_y);
    let floor = || PlaneGeometry::build(6., 6., 1, 1);
    let side = || PlaneGeometry::build(6., 5., 1, 1);
    let lit = Kind::Shadowed;
    Ok(Setup {
        complex: false,
        camera_z: 8.,
        lights: vec![PointShadowLight {
            position: Vector3::new(0., 4.5, 0.),
            color: 0xffffff,
            intensity: 40.,
            bias: 0.,
            normal_bias: -0.02,
            radius: 10.,
        }],
        volumes: vec![Volume {
            center: Vector3::new(0., 2.45, 0.),
            size: Vector3::new(5.6, 4.7, 5.6),
            falloff: 0.,
        }],
        meshes: vec![
            (floor()?, rx(-PI / 2.), wall, lit, false),
            (floor()?, t(0., 5., 0.) * rx(PI / 2.), wall, lit, false),
            (side()?, t(0., 2.5, -3.), wall, lit, false),
            (side()?, t(0., 2.5, 3.) * ry(PI), wall, lit, false),
            (
                side()?,
                t(-3., 2.5, 0.) * ry(PI / 2.),
                standard(0xff0000),
                lit,
                false,
            ),
            (
                side()?,
                t(3., 2.5, 0.) * ry(-PI / 2.),
                standard(0x00ff00),
                lit,
                false,
            ),
            (
                BoxGeometry::build(1.2, 2.5, 1.2)?,
                t(-0.8, 1.25, -0.8) * ry(PI / 8.),
                object,
                lit,
                true,
            ),
            (
                BoxGeometry::build(1.2, 1.2, 1.2)?,
                t(1., 0.6, 0.5) * ry(-PI / 6.),
                object,
                lit,
                true,
            ),
            (
                SphereGeometry::build(0.5, 32, 32)?,
                t(1., 1.9, 0.5),
                object,
                lit,
                true,
            ),
        ],
    })
}
/// The two rooms of webgpu_lightprobes_complex.
fn rooms() -> Result<Setup> {
    let white = standard(0xeeeeee);
    let divider = standard(0xcccccc);
    let wood = standard(0x886644);
    let t = |x, y, z| Matrix4::from_translation(Vector3::new(x, y, z));
    let ry = Matrix4::from_rotation_y;
    let (lit, plain) = (Kind::Shadowed, Kind::Plain);
    let gap = 1.25;
    let column = || CylinderGeometry::build(0.3, 0.3, 4., 16, 1, false, 0., 2. * PI);
    let mut meshes = vec![
        (
            BoxGeometry::build(16., 5., 8.)?,
            t(0., 2.5, 0.),
            standard(0xcccccc),
            Kind::Back,
            false,
        ),
        (
            PlaneGeometry::build(8., 5., 1, 1)?,
            t(-7.99, 2.5, 0.) * ry(PI / 2.),
            standard(0xdd2200),
            plain,
            false,
        ),
        (
            PlaneGeometry::build(8., 5., 1, 1)?,
            t(7.99, 2.5, 0.) * ry(-PI / 2.),
            standard(0x0044ff),
            plain,
            false,
        ),
        (
            BoxGeometry::build(0.3, 5., 4. - gap)?,
            t(0., 2.5, -(gap + (4. - gap) / 2.)),
            divider,
            lit,
            true,
        ),
        (
            BoxGeometry::build(0.3, 5., 4. - gap)?,
            t(0., 2.5, gap + (4. - gap) / 2.),
            divider,
            lit,
            true,
        ),
        (
            BoxGeometry::build(0.3, 1.2, gap * 2.)?,
            t(0., 4.4, 0.),
            divider,
            lit,
            true,
        ),
    ];
    for x in [-6., -2.] {
        for z in [-2.5, 2.5] {
            meshes.push((column()?, t(x, 2., z), white, lit, true));
        }
    }
    meshes.push((
        BoxGeometry::build(2.4, 0.12, 1.4)?,
        t(-4., 1., 0.),
        wood,
        lit,
        true,
    ));
    for x in [-5.05, -2.95] {
        for z in [-0.55, 0.55] {
            meshes.push((
                BoxGeometry::build(0.1, 1., 0.1)?,
                t(x, 0.5, z),
                wood,
                plain,
                true,
            ));
        }
    }
    meshes.extend([
        (
            SphereGeometry::build(0.35, 32, 32)?,
            t(-4., 1.41, 0.),
            Material {
                color: 0xffd700,
                roughness: 0.4,
                metalness: 0.3,
            },
            lit,
            true,
        ),
        (
            BoxGeometry::build(1.5, 0.5, 1.)?,
            t(-7., 0.25, 0.),
            white,
            lit,
            true,
        ),
        (
            BoxGeometry::build(1., 1., 1.)?,
            t(-7., 0.5, 0.),
            white,
            lit,
            true,
        ),
        (
            BoxGeometry::build(0.5, 1.5, 1.)?,
            t(-7., 0.75, 0.),
            white,
            lit,
            true,
        ),
    ]);
    for x in [2., 6.] {
        for z in [-2.5, 2.5] {
            meshes.push((column()?, t(x, 2., z), white, lit, true));
        }
    }
    meshes.extend([
        (
            BoxGeometry::build(0.8, 1.2, 0.8)?,
            t(4., 0.6, 0.),
            white,
            lit,
            true,
        ),
        (
            TorusKnotGeometry::build(0.3, 0.1, 64, 16, 2, 3)?,
            t(4., 1.65, 0.),
            Material {
                color: 0xff44aa,
                roughness: 0.5,
                metalness: 0.2,
            },
            lit,
            true,
        ),
        (
            CylinderGeometry::build(0., 0.4, 2.5, 5, 1, false, 0., 2. * PI)?,
            t(6.5, 1.25, -1.5),
            white,
            lit,
            true,
        ),
    ]);
    let light = |x, color| PointShadowLight {
        position: Vector3::new(x, 4.5, 0.),
        color,
        intensity: 30.,
        bias: -0.002,
        normal_bias: 0.,
        radius: 12.,
    };
    let volume = |x| Volume {
        center: Vector3::new(x, 2.45, 0.),
        size: Vector3::new(7.8, 4.7, 7.6),
        falloff: 1.,
    };
    Ok(Setup {
        complex: true,
        camera_z: 16.,
        lights: vec![light(-4., 0xffaa44), light(4., 0x88bbff)],
        volumes: vec![volume(-3.9), volume(3.9)],
        meshes,
    })
}
/// CubeCamera( near, far )'s WebGPU projection and its six face rotations, in layer order.
fn cube_cameras() -> (Matrix4, [Matrix4; 6]) {
    let m = CUBE_FAR / (CUBE_NEAR - CUBE_FAR);
    let projection = Matrix4::from_cols_array(&[
        -1.,
        0.,
        0.,
        0.,
        0.,
        -1.,
        0.,
        0.,
        0.,
        0.,
        m,
        -1.,
        0.,
        0.,
        m * CUBE_NEAR,
        0.,
    ]);
    let views = [
        [
            0., 0., 1., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.,
        ],
        [
            0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
        ],
        [
            -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    ]
    .map(|v| Matrix4::from_cols_array(&v));
    (projection, views)
}
/// One mesh: normals and positions, index, local bounding sphere, model,
/// material, variant and whether it casts shadows.
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
    /// The object structs of the variant's bake (and GI-off) material, of
    /// its GI material and of the shadow pass.
    standard: wgpu::Buffer,
    gi: wgpu::Buffer,
    shadow: wgpu::Buffer,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_vertex_buffer(0, self.normals.slice(..));
        pass.set_vertex_buffer(1, self.positions.slice(..));
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    /// The MeshStandardMaterial's object uniforms: color, opacity, metalness,
    /// roughness, normal matrix, emissive and the model.
    fn uniforms(&self) -> Vec<(&'static str, Vec<f64>)> {
        vec![
            (
                "nodeUniform0",
                Color::from_hex(self.material.color).0.to_array().to_vec(),
            ),
            ("nodeUniform1", vec![1.]),
            ("nodeUniform2", vec![self.material.metalness]),
            ("nodeUniform3", vec![self.material.roughness]),
            ("nodeUniform5", m3(self.model.inverse().transpose())),
            ("nodeUniform6", vec![0.; 3]),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform9", m4(self.model)),
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
/// A baked grid: the atlas and its helper's instance matrices, UVWs and
/// object struct.
struct Grid {
    atlas: wgpu::TextureView,
    matrices: wgpu::Buffer,
    uvw: wgpu::Buffer,
    helper: wgpu::Buffer,
}
/// One point light's shadow: per face the color and depth layers and the
/// camera group, and the depth cube.
struct Shadow {
    faces: Vec<(wgpu::TextureView, wgpu::TextureView, Draw)>,
    cube: wgpu::TextureView,
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
    setup: Setup,
    controls: Controls,
    /// GI enabled, resolution and show probes.
    params: [f64; 3],
    meshes: Vec<Mesh>,
    /// The helper's sphere: positions, normals, index and count.
    helper_mesh: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    shadows: Vec<Shadow>,
    shadow_objects: Vec<wgpu::BindGroup>,
    /// The bake: per variant the six faces' render structs, per face the
    /// mesh draws, and the cube target's faces, cube view and depth.
    face_renders: Vec<Vec<wgpu::Buffer>>,
    face_draws: Vec<Vec<Draw>>,
    cube_faces: Vec<wgpu::TextureView>,
    cube: wgpu::TextureView,
    cube_depth: wgpu::TextureView,
    sh_pipeline: wgpu::RenderPipeline,
    sh_buffers: (wgpu::Buffer, wgpu::Buffer),
    repack_pipelines: Vec<wgpu::RenderPipeline>,
    /// The baked resolution and grids.
    res: u32,
    grids: Vec<Grid>,
    /// Per variant, the main camera's render structs for the GI-off and GI materials.
    standard_renders: Vec<wgpu::Buffer>,
    gi_renders: Vec<wgpu::Buffer>,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
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
fn layouts<'a>(
    attributes: &'a [[wgpu::VertexAttribute; 1]],
    instanced: &[bool],
) -> Vec<wgpu::VertexBufferLayout<'a>> {
    attributes
        .iter()
        .zip(instanced)
        .map(|(a, &instanced)| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: if instanced {
                wgpu::VertexStepMode::Instance
            } else {
                wgpu::VertexStepMode::Vertex
            },
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
        &layouts(&a, &[false, false]),
        HALF,
        samples,
        Some((wgpu::CompareFunction::LessEqual, true)),
        back,
    )
}
/// A lit material's object group: the object struct, the DFG LUT, the
/// lights' shadow cubes (shadowed variants) and the grids' atlases (GI).
fn lit_entries<'a>(
    object: &'a wgpu::Buffer,
    (linear, compare): (&'a wgpu::Sampler, &'a wgpu::Sampler),
    dfg: &'a wgpu::TextureView,
    shadows: &'a [wgpu::TextureView],
    atlases: &'a [wgpu::TextureView],
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
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
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
fn cube_view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::Cube),
        ..Default::default()
    })
}
fn write(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    source: &str,
    name: &str,
    values: &[(&str, Vec<f64>)],
) -> Result<()> {
    let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
    r.queue
        .write_buffer(buffer, 0, &pack(source, name, &values)?);
    Ok(())
}
/// The helper's instancing WGSL for `count` probes: a uniform mat4 array up
/// to 1000 instances, as InstanceNode builds it.
fn helper_vs(count: u32) -> String {
    let source = wgsl!("helper_vs").replace(
        "array< mat4x4<f32>, 216 >",
        &format!("array< mat4x4<f32>, {count} >"),
    );
    if count > 1000 {
        // Beyond a uniform buffer's reach: the same array as storage.
        source.replace(
            "var<uniform> NodeBuffer_6579",
            "var<storage, read> NodeBuffer_6579",
        )
    } else {
        source
    }
}
/// LightProbeGridHelper's pipeline for `count` probes.
fn helper_pipeline(r: &Renderer, count: u32, samples: u32) -> wgpu::RenderPipeline {
    let a = attributes(&[0, 1, 2]);
    raw_pipeline(
        r,
        "lightprobes helper",
        &helper_vs(count),
        wgsl!("helper_fs"),
        &layouts(&a, &[false, false, true]),
        HALF,
        samples,
        Some((wgpu::CompareFunction::LessEqual, true)),
        false,
    )
}
/// The helper's object struct for a grid resolution.
fn helper_uniforms(res: u32) -> Vec<(&'static str, Vec<f64>)> {
    vec![
        ("nodeUniform2", vec![res as f64; 3]),
        ("nodeUniform4", m3(Matrix4::IDENTITY)),
        ("nodeUniform7", m4(Matrix4::IDENTITY)),
    ]
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let setup = if id == 393 { rooms()? } else { cornell()? };
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
        for (g, model, material, kind, caster) in &setup.meshes {
            let positions = read(g, "position")?;
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
            let [_, fs, _, gi_fs] = setup.shaders(*kind);
            let mut shadow = vec![1f32, 0., 0., 0.];
            shadow.extend(model.to_cols_array().map(|v| v as f32));
            let mesh = Mesh {
                normals: init(
                    "lightprobes normals",
                    bytemuck::cast_slice(&read(g, "normal")?),
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
                model: *model,
                material: *material,
                kind: *kind,
                caster: *caster,
                standard: uniform(r, "lightprobes object", fs, "objectStruct")?,
                gi: uniform(r, "lightprobes GI object", gi_fs, "objectStruct")?,
                shadow: init(
                    "lightprobes shadow object",
                    bytemuck::cast_slice(&shadow),
                    wgpu::BufferUsages::UNIFORM,
                ),
            };
            write(r, &mesh.standard, fs, "objectStruct", &mesh.uniforms())?;
            meshes.push(mesh);
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
        // The point shadows: front-side casters draw back faces ( clockwise ).
        let position = attributes(&[0]);
        let shadow_pipeline = raw_pipeline(
            r,
            "lightprobes shadow",
            SHADOW_VS,
            SHADOW_FS,
            &layouts(&position, &[false]),
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            Some((wgpu::CompareFunction::LessEqual, true)),
            true,
        );
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., SHADOW_NEAR, SHADOW_FAR);
        let shadows = setup
            .lights
            .iter()
            .map(|light| {
                let color = texture(
                    r,
                    (SHADOW, SHADOW, 6),
                    wgpu::TextureDimension::D2,
                    wgpu::TextureFormat::Rgba8Unorm,
                );
                let depth = texture(r, (SHADOW, SHADOW, 6), wgpu::TextureDimension::D2, DEPTH);
                let faces = (0..6)
                    .map(|i| {
                        let (direction, up) = FACES[i];
                        let p = light.position;
                        let view = Matrix4::look_at_rh(
                            p,
                            p + Vector3::from_array(direction),
                            Vector3::from_array(up),
                        );
                        let mut data: Vec<f32> =
                            face_projection.to_cols_array().map(|v| v as f32).to_vec();
                        data.extend(view.to_cols_array().map(|v| v as f32));
                        let buffer = init(
                            "lightprobes shadow camera",
                            bytemuck::cast_slice(&data),
                            wgpu::BufferUsages::UNIFORM,
                        );
                        let group = bind(
                            r,
                            shadow_pipeline.get_bind_group_layout(0),
                            &[(0, buffer.as_entire_binding())],
                        );
                        (
                            layer(&color, i as u32),
                            layer(&depth, i as u32),
                            (shadow_pipeline.clone(), vec![group]),
                        )
                    })
                    .collect();
                Shadow {
                    faces,
                    cube: cube_view(&depth),
                }
            })
            .collect::<Vec<_>>();
        let shadow_objects = meshes
            .iter()
            .map(|m| {
                bind(
                    r,
                    shadow_pipeline.get_bind_group_layout(1),
                    &[(0, m.shadow.as_entire_binding())],
                )
            })
            .collect();
        let shadow_cubes: Vec<wgpu::TextureView> = shadows.iter().map(|s| s.cube.clone()).collect();
        // The bake: the cube capture, the SH projection and the repack.
        let cube_texture = texture(r, (CUBE, CUBE, 6), wgpu::TextureDimension::D2, HALF);
        let cube_faces = (0..6).map(|i| layer(&cube_texture, i)).collect();
        let cube = cube_view(&cube_texture);
        let cube_depth = texture(r, (CUBE, CUBE, 1), wgpu::TextureDimension::D2, DEPTH)
            .create_view(&Default::default());
        let face_renders: Vec<Vec<wgpu::Buffer>> = KINDS
            .iter()
            .map(|&kind| {
                (0..6)
                    .map(|_| {
                        uniform(
                            r,
                            "lightprobes face render",
                            setup.shaders(kind)[1],
                            "renderStruct",
                        )
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?;
        let bake_pipelines: Vec<Option<wgpu::RenderPipeline>> = KINDS
            .iter()
            .map(|&kind| {
                meshes.iter().any(|m| m.kind == kind).then(|| {
                    let [vs, fs, ..] = setup.shaders(kind);
                    lit_pipeline(r, "lightprobes bake", (vs, fs), 1, kind == Kind::Back)
                })
            })
            .collect();
        let mut face_draws = vec![vec![]; 6];
        for m in &meshes {
            let pipeline = bake_pipelines[m.kind.index()]
                .as_ref()
                .ok_or(Error::Invalid("lightprobes bake pipeline"))?;
            let shadowed: &[wgpu::TextureView] = if m.kind == Kind::Plain {
                &[]
            } else {
                &shadow_cubes
            };
            let object = bind(
                r,
                pipeline.get_bind_group_layout(1),
                &lit_entries(&m.standard, (&linear, &compare), &r.dfg, shadowed, &[]),
            );
            for (face, draws) in face_draws.iter_mut().enumerate() {
                let render = bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[(0, face_renders[m.kind.index()][face].as_entire_binding())],
                );
                draws.push((pipeline.clone(), vec![render, object.clone()]));
            }
        }
        let sh_pipeline = raw_pipeline(
            r,
            "lightprobes SH",
            include_str!("../shaders/light_probe_grid/sh_vs.wgsl"),
            include_str!("../shaders/light_probe_grid/sh_fs.wgsl"),
            &[],
            FLOAT,
            1,
            None,
            false,
        );
        let sh_buffers = (
            init(
                "lightprobes SH render",
                &pack(
                    include_str!("../shaders/light_probe_grid/sh_fs.wgsl"),
                    "renderStruct",
                    &[
                        ("cameraWorldMatrix", &m4(Matrix4::IDENTITY)),
                        (
                            "cameraProjectionMatrixInverse",
                            &[
                                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                            ],
                        ),
                    ],
                )?,
                wgpu::BufferUsages::UNIFORM,
            ),
            init(
                "lightprobes SH object",
                &pack(
                    include_str!("../shaders/light_probe_grid/sh_fs.wgsl"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", &[1.]),
                        ("nodeUniform4", &m4(Matrix4::IDENTITY)),
                    ],
                )?,
                wgpu::BufferUsages::UNIFORM,
            ),
        );
        let repack_pipelines = REPACK_FS
            .iter()
            .map(|fs| {
                raw_pipeline(
                    r,
                    "lightprobes repack",
                    include_str!("../shaders/light_probe_grid/repack_vs.wgsl"),
                    fs,
                    &[],
                    HALF,
                    1,
                    None,
                    false,
                )
            })
            .collect();
        let renders = |gi: bool| -> Result<Vec<wgpu::Buffer>> {
            KINDS
                .iter()
                .map(|&kind| {
                    uniform(
                        r,
                        "lightprobes render",
                        setup.shaders(kind)[if gi { 3 } else { 1 }],
                        "renderStruct",
                    )
                })
                .collect()
        };
        Ok(Self {
            controls,
            params: [1., 6., 0.],
            meshes,
            helper_mesh,
            shadows,
            shadow_objects,
            face_renders,
            face_draws,
            cube_faces,
            cube,
            cube_depth,
            sh_pipeline,
            sh_buffers,
            repack_pipelines,
            res: 0,
            grids: vec![],
            standard_renders: renders(false)?,
            gi_renders: renders(true)?,
            output_render: uniform(r, "lightprobes output render", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "lightprobes output object", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "lightprobes quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            linear,
            compare,
            targets: None,
            setup,
        })
    }
    /// The material variants the scene uses.
    fn kinds(&self) -> Vec<Kind> {
        KINDS
            .into_iter()
            .filter(|&k| self.meshes.iter().any(|m| m.kind == k))
            .collect()
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.frame_update(s, c)
    }
    /// The point lights' cube shadows ( re-rendered every frame, as
    /// shadow.autoUpdate does ), each face culling the casters.
    fn encode_shadows(&self, encoder: &mut wgpu::CommandEncoder) {
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., SHADOW_NEAR, SHADOW_FAR);
        for (light, shadow) in self.setup.lights.iter().zip(&self.shadows) {
            for (i, (color, depth, draw)) in shadow.faces.iter().enumerate() {
                let (direction, up) = FACES[i];
                let p = light.position;
                let view = Matrix4::look_at_rh(
                    p,
                    p + Vector3::from_array(direction),
                    Vector3::from_array(up),
                );
                let frustum = Frustum::from_projection(face_projection * view);
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
                set(&mut pass, draw);
                for (k, m) in self.meshes.iter().enumerate() {
                    if !m.caster || !m.visible(&frustum) {
                        continue;
                    }
                    pass.set_bind_group(1, &self.shadow_objects[k], &[]);
                    pass.set_vertex_buffer(0, m.positions.slice(..));
                    pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..1);
                }
            }
        }
    }
    /// The page's bake( resolution ): every grid's LightProbeGrid.bake(
    /// renderer, scene, { cubemapSize: 32, near: 0.05, far: 20 } ) in turn,
    /// with the grids out of the scene, then the helpers' update.
    fn bake(&mut self, r: &Renderer, res: u32) -> Result<()> {
        // A rebake with the probes shown captures the helpers too: the page
        // disposes the old grids first, so each helper (still on its old
        // grid's instances) samples its atlas recreated empty, drawing
        // black spheres.
        let mut stale = vec![];
        if self.params[2] > 0.5 {
            let count = self.res.pow(3);
            for old in &self.grids {
                write(
                    r,
                    &old.helper,
                    wgsl!("helper_fs"),
                    "objectStruct",
                    &helper_uniforms(self.res),
                )?;
                let empty = texture(
                    r,
                    (self.res, self.res, 7 * (self.res + 2)),
                    wgpu::TextureDimension::D3,
                    HALF,
                )
                .create_view(&Default::default());
                let pipeline = helper_pipeline(r, count, 1);
                let object = bind(
                    r,
                    pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(&empty)),
                        (2, old.helper.as_entire_binding()),
                        (3, old.matrices.as_entire_binding()),
                    ],
                );
                // The helper's render struct is the camera's: any variant's face buffer.
                let draws: Vec<Draw> = self.face_renders[Kind::Shadowed.index()]
                    .iter()
                    .map(|buffer| {
                        let render = bind(
                            r,
                            pipeline.get_bind_group_layout(0),
                            &[(0, buffer.as_entire_binding())],
                        );
                        (pipeline.clone(), vec![render, object.clone()])
                    })
                    .collect();
                stale.push((draws, &old.uvw, count));
            }
        }
        let mut grids = vec![];
        for volume in &self.setup.volumes {
            grids.push(self.bake_grid(r, volume, res, &stale)?);
        }
        drop(stale);
        self.grids = grids;
        self.res = res;
        // The GI materials' grid uniforms.
        for m in &self.meshes {
            let mut values = m.uniforms();
            for (volume, names) in self.setup.volumes.iter().zip(self.setup.grid_names(m.kind)) {
                let half = volume.size / 2.;
                values.extend([
                    (names[0], (volume.center + half).to_array().to_vec()),
                    (names[1], (volume.center - half).to_array().to_vec()),
                    (names[2], vec![res as f64; 3]),
                    (names[3], vec![1.]),
                    (names[4], vec![volume.falloff]),
                ]);
            }
            write(
                r,
                &m.gi,
                self.setup.shaders(m.kind)[3],
                "objectStruct",
                &values,
            )?;
        }
        self.targets = None;
        Ok(())
    }
    /// One grid's bake at `res`³ probes: per probe the six cube faces and
    /// the SH row ( one submit each, as the face cameras' uniforms change ),
    /// then the seven repack passes per Z slice with the boundary padding.
    fn bake_grid(
        &self,
        r: &Renderer,
        volume: &Volume,
        res: u32,
        stale: &[(Vec<Draw>, &wgpu::Buffer, u32)],
    ) -> Result<Grid> {
        let total = res * res * res;
        let batch = texture(r, (9, total, 1), wgpu::TextureDimension::D2, FLOAT)
            .create_view(&Default::default());
        let atlas = texture(
            r,
            (res, res, 7 * (res + 2)),
            wgpu::TextureDimension::D3,
            HALF,
        );
        let atlas_view = atlas.create_view(&Default::default());
        let sh_groups = [
            bind(
                r,
                self.sh_pipeline.get_bind_group_layout(0),
                &[(0, self.sh_buffers.0.as_entire_binding())],
            ),
            bind(
                r,
                self.sh_pipeline.get_bind_group_layout(1),
                &[
                    (0, self.sh_buffers.1.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.linear)),
                    (2, wgpu::BindingResource::TextureView(&self.cube)),
                ],
            ),
        ];
        let (projection, rotations) = cube_cameras();
        for probe in 0..total {
            let (ix, iy, iz) = (probe % res, probe / (res * res), (probe / res) % res);
            let p = volume.probe(res, [ix, iy, iz]);
            let mut encoder = r.device.create_command_encoder(&Default::default());
            for (face, rotation) in rotations.iter().enumerate() {
                let view = *rotation * Matrix4::from_translation(-p);
                for kind in self.kinds() {
                    let mut values = vec![
                        ("cameraProjectionMatrix", m4(projection)),
                        ("cameraViewMatrix", m4(view)),
                    ];
                    values.extend(self.setup.light_uniforms(kind, view));
                    write(
                        r,
                        &self.face_renders[kind.index()][face],
                        self.setup.shaders(kind)[1],
                        "renderStruct",
                        &values,
                    )?;
                }
                let frustum = Frustum::from_projection(projection * view);
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes cube face"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.cube_faces[face],
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
                        view: &self.cube_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                for (draws, uvw, count) in stale {
                    let (positions, normals, index, indices) = &self.helper_mesh;
                    set(&mut pass, &draws[face]);
                    pass.set_vertex_buffer(0, positions.slice(..));
                    pass.set_vertex_buffer(1, normals.slice(..));
                    pass.set_vertex_buffer(2, uvw.slice(..));
                    pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..*indices, 0, 0..*count);
                }
                for (k, m) in self.meshes.iter().enumerate() {
                    if m.visible(&frustum) {
                        set(&mut pass, &self.face_draws[face][k]);
                        m.draw(&mut pass);
                    }
                }
            }
            {
                // Batch rows in texture order ( X, Y, Z ).
                let row = ix + iy * res + iz * res * res;
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes SH"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &batch,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_viewport(0., row as f32, 9., 1., 0., 1.);
                pass.set_pipeline(&self.sh_pipeline);
                pass.set_bind_group(0, &sh_groups[0], &[]);
                pass.set_bind_group(1, &sh_groups[1], &[]);
                pass.draw(0..3, 0..1);
            }
            r.queue.submit([encoder.finish()]);
        }
        // The repack: one rectangle per Z slice, into the slice and, at the
        // ends, the padding slices.
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for iz in 0..res {
            let object = uniform(r, "lightprobes repack object", REPACK_FS[0], "objectStruct")?;
            write(
                r,
                &object,
                REPACK_FS[0],
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.]),
                    ("nodeUniform2", m3(Matrix4::IDENTITY)),
                    ("nodeUniform3", vec![res as f64; 3]),
                    ("nodeUniform4", vec![iz as f64]),
                    ("nodeUniform5", m3(Matrix4::IDENTITY)),
                ],
            )?;
            for (t, pipeline) in self.repack_pipelines.iter().enumerate() {
                let group = bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[
                        (0, object.as_entire_binding()),
                        (1, wgpu::BindingResource::TextureView(&batch)),
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
                            view: &atlas_view,
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
        // LightProbeGridHelper.update(): instance matrices and texel-center UVWs.
        let mut matrices: Vec<f32> = vec![];
        let mut uvw: Vec<f32> = vec![];
        for iz in 0..res {
            for iy in 0..res {
                for ix in 0..res {
                    uvw.extend([ix, iy, iz].map(|i| ((i as f64 + 0.5) / res as f64) as f32));
                    let p = volume.probe(res, [ix, iy, iz]);
                    matrices.extend(
                        Matrix4::from_translation(p)
                            .to_cols_array()
                            .map(|v| v as f32),
                    );
                }
            }
        }
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        Ok(Grid {
            atlas: atlas_view,
            matrices: init(
                "helper matrices",
                bytemuck::cast_slice(&matrices),
                if total > 1000 {
                    wgpu::BufferUsages::STORAGE
                } else {
                    wgpu::BufferUsages::UNIFORM
                },
            ),
            uvw: init(
                "helper uvw",
                bytemuck::cast_slice(&uvw),
                wgpu::BufferUsages::VERTEX,
            ),
            helper: uniform(
                r,
                "lightprobes helper object",
                wgsl!("helper_fs"),
                "objectStruct",
            )?,
        })
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let shadow_cubes: Vec<wgpu::TextureView> =
            self.shadows.iter().map(|s| s.cube.clone()).collect();
        let atlases: Vec<wgpu::TextureView> = self.grids.iter().map(|g| g.atlas.clone()).collect();
        let mut pipelines: Vec<Option<(wgpu::RenderPipeline, wgpu::RenderPipeline)>> = vec![];
        for kind in KINDS {
            pipelines.push(self.meshes.iter().any(|m| m.kind == kind).then(|| {
                let [vs, fs, gi_vs, gi_fs] = self.setup.shaders(kind);
                (
                    lit_pipeline(
                        r,
                        "lightprobes GI",
                        (gi_vs, gi_fs),
                        samples,
                        kind == Kind::Back,
                    ),
                    lit_pipeline(r, "lightprobes", (vs, fs), samples, kind == Kind::Back),
                )
            }));
        }
        let mut gi = vec![];
        let mut standard = vec![];
        for m in &self.meshes {
            let (gi_pipeline, standard_pipeline) = pipelines[m.kind.index()]
                .as_ref()
                .ok_or(Error::Invalid("lightprobes pipeline"))?;
            let shadowed: &[wgpu::TextureView] = if m.kind == Kind::Plain {
                &[]
            } else {
                &shadow_cubes
            };
            let samplers = (&self.linear, &self.compare);
            gi.push((
                gi_pipeline.clone(),
                vec![
                    bind(
                        r,
                        gi_pipeline.get_bind_group_layout(0),
                        &[(0, self.gi_renders[m.kind.index()].as_entire_binding())],
                    ),
                    bind(
                        r,
                        gi_pipeline.get_bind_group_layout(1),
                        &lit_entries(&m.gi, samplers, &r.dfg, shadowed, &atlases),
                    ),
                ],
            ));
            standard.push((
                standard_pipeline.clone(),
                vec![
                    bind(
                        r,
                        standard_pipeline.get_bind_group_layout(0),
                        &[(0, self.standard_renders[m.kind.index()].as_entire_binding())],
                    ),
                    bind(
                        r,
                        standard_pipeline.get_bind_group_layout(1),
                        &lit_entries(&m.standard, samplers, &r.dfg, shadowed, &[]),
                    ),
                ],
            ));
        }
        let helper_pipeline = helper_pipeline(r, self.res.pow(3), samples);
        // The helper's render struct is the camera's: any variant's.
        let helper_render = bind(
            r,
            helper_pipeline.get_bind_group_layout(0),
            &[(
                0,
                self.gi_renders[Kind::Shadowed.index()].as_entire_binding(),
            )],
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
                                (1, wgpu::BindingResource::TextureView(&g.atlas)),
                                (2, g.helper.as_entire_binding()),
                                (3, g.matrices.as_entire_binding()),
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
            &layouts(&quad, &[false]),
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
        let [enabled, resolution, show] = self.params;
        let res = resolution.round().clamp(2., 12.) as u32;
        if self.res != res {
            // The bake reads the lights' shadows, rendered first.
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.encode_shadows(&mut encoder);
            r.queue.submit([encoder.finish()]);
            self.bake(r, res)?;
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
        for kind in self.kinds() {
            let mut values = vec![
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
            ];
            values.extend(self.setup.light_uniforms(kind, view));
            let [_, fs, _, gi_fs] = self.setup.shaders(kind);
            write(
                r,
                &self.standard_renders[kind.index()],
                fs,
                "renderStruct",
                &values,
            )?;
            write(
                r,
                &self.gi_renders[kind.index()],
                gi_fs,
                "renderStruct",
                &values,
            )?;
        }
        for g in &self.grids {
            write(
                r,
                &g.helper,
                wgsl!("helper_fs"),
                "objectStruct",
                &helper_uniforms(self.res),
            )?;
        }
        write(
            r,
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![1.]),
            ],
        )?;
        write(
            r,
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
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
                    pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..*count, 0, 0..self.res.pow(3));
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
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
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
    /// GI, resolution ( rebakes ) and show probes.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("lightprobes parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
