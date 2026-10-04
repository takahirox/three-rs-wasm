//! webgpu_generator_city: CityGenerator's 2 × 2 blocks of seeded
//! SkyscraperGenerator towers on rounded sidewalk slabs, with the street
//! furniture ( streetlights, mast-arm signals, litter baskets, benches,
//! hydrants, swaying street trees, parked and travelling cars, walking and
//! standing pedestrians ) instanced along every curb, on the wet-asphalt
//! road with its lane and crosswalk paint. The SkyMesh sky drives the sun's
//! colour, its 4096² shadow fitted around the city and the PMREM
//! environment ( scene.environmentIntensity 0.05 ). A 10 × 7 × 8
//! LightProbeGrid bakes the box proxy of the towers, the ground and the sky
//! ten probes per frame, then a second pass for the bounce from a snapshot
//! of the first; every lit material samples the grid. The scene renders
//! with 4× MSAA, then quarter-resolution BloomNode ( 0.05 ) and ACES
//! ( exposure 0.13 ), under FirstPersonControls. The generators run on the
//! CPU when the seed changes, as the page's do; the towers are one draw
//! each and the furniture one instanced draw per generator. Every stage
//! runs the WGSL three.js r186 generates for the page ( `generator_city/`;
//! the sky, PMREM, probe SH and repack, bloom and output modules are the
//! custom_fog, retro, lightprobes, ssr, volume_caustics and water ones,
//! byte-identical ).
mod city;
mod furniture;
pub(super) mod geo;
mod prims;
pub(super) mod uniforms;
use super::custom_fog::{
    Buffers, FACE_PROJECTION, FACE_VIEWS, layouts, pipeline, two_groups, upload,
};
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::pack;
use super::pmrem_cube_uv::{GGX_8, Pmrem, bind, raw_pipeline};
use super::retro::uniform;
use super::trackball_sprites::FirstPerson;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use geo::*;
use std::collections::HashMap;
use std::f64::consts::PI;
use uniforms::*;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
/// The sun's shadow map size and normal bias.
const MAP: u32 = 4096;
const NORMAL_BIAS: f64 = 0.05;
/// The probe bake's cubemapSize, near and far.
const CUBE: u32 = 16;
const CUBE_NEAR: f64 = 0.1;
const CUBE_FAR: f64 = 20000.;
/// The probe grid: resolution, size and centre ( GRID_SIZE.y / 2 + 1 ).
const GRID_RES: [u32; 3] = [10, 7, 8];
const GRID_SIZE: [f64; 3] = [240., 120., 170.];
const GRID_CENTER: [f64; 3] = [0., 61., 0.];
const PROBES: u32 = GRID_RES[0] * GRID_RES[1] * GRID_RES[2];
/// BloomNode's mip levels.
const LEVELS: usize = 5;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("generator_city/", $name, ".wgsl"))
    };
}
const BOX_VS: &str = include_str!("custom_fog/box_vs.wgsl");
const BOX_FS: &str = include_str!("custom_fog/box_fs.wgsl");
const SKY_VS: &str = include_str!("custom_fog/sky_vs.wgsl");
const SKY_FS: &str = include_str!("custom_fog/sky_fs.wgsl");
const SH_VS: &str = include_str!("lightprobes/sh_vs.wgsl");
const SH_FS: &str = include_str!("lightprobes/sh_fs.wgsl");
const REPACK_VS: &str = include_str!("lightprobes/repack_vs.wgsl");
const SEED_VS: &str = include_str!("ssr_denoise/seed_vs.wgsl");
const REPACK_FS: [&str; 7] = [
    include_str!("lightprobes/repack_0_fs.wgsl"),
    include_str!("lightprobes/repack_1_fs.wgsl"),
    include_str!("lightprobes/repack_2_fs.wgsl"),
    include_str!("lightprobes/repack_3_fs.wgsl"),
    include_str!("lightprobes/repack_4_fs.wgsl"),
    include_str!("lightprobes/repack_5_fs.wgsl"),
    include_str!("lightprobes/repack_6_fs.wgsl"),
];
const HIGH_VS: &str = include_str!("ssr/blur_vs.wgsl");
const HIGH_FS: &str = include_str!("water/high_fs.wgsl");
const BLUR_VS: &str = include_str!("volume_caustics/blur_vs.wgsl");
const BLUR_FS: [&str; LEVELS] = [
    include_str!("volume_caustics/blur0_fs.wgsl"),
    include_str!("volume_caustics/blur1_fs.wgsl"),
    include_str!("volume_caustics/blur2_fs.wgsl"),
    include_str!("volume_caustics/blur3_fs.wgsl"),
    include_str!("volume_caustics/blur4_fs.wgsl"),
];
const QUAD_VS: &str = include_str!("volume_caustics/composite_vs.wgsl");
const COMPOSITE_FS: &str = include_str!("water/composite_fs.wgsl");
const OUTPUT_FS: &str = include_str!("water/output_fs.wgsl");

/// What a group 1 binding holds.
#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Object,
    Instances,
    Pillars,
    Linear,
    Compare,
    Dfg,
    Depth,
    Atlas,
    Env,
}
/// A vertex attribute: location, name, format and whether it steps per
/// instance.
struct Attr {
    location: u32,
    name: String,
    format: wgpu::VertexFormat,
    instance: bool,
}
/// One material stage pair: its sources, attributes, group 1 bindings and
/// struct layouts.
struct Shader {
    vs: String,
    fs: &'static str,
    attrs: Vec<Attr>,
    slots: Vec<(u32, Slot)>,
    object: Layout<ObjectSlot>,
    render: Layout<RenderSlot>,
}
/// The vertex stage's attributes from its main().
fn attributes(vs: &str) -> Vec<Attr> {
    let Some(start) = vs.find("fn main(") else {
        return vec![];
    };
    let signature = &vs[start..start + vs[start..].find("->").unwrap_or(0)];
    signature
        .split("@location(")
        .skip(1)
        .filter_map(|part| {
            let (location, rest) = part.split_once(')')?;
            let (name, ty) = rest.split_once(':')?;
            let ty = ty.trim();
            let format = if ty.starts_with("vec2") {
                wgpu::VertexFormat::Float32x2
            } else if ty.starts_with("vec3") {
                wgpu::VertexFormat::Float32x3
            } else {
                wgpu::VertexFormat::Float32
            };
            let name = name.trim().to_string();
            Some(Attr {
                location: location.trim().parse().ok()?,
                instance: name == "paintColor" || name == "personSeed",
                name,
                format,
            })
        })
        .collect()
}
/// The group 1 bindings of both stages.
fn slots(vs: &str, fs: &str) -> Vec<(u32, Slot)> {
    let mut out: Vec<(u32, Slot)> = vec![];
    let mut textures = 0;
    for source in [vs, fs] {
        let mut rest = source;
        while let Some(i) = rest.find("@binding( ") {
            rest = &rest[i + 10..];
            let Some((n, after)) = rest.split_once(' ') else {
                break;
            };
            let Ok(binding) = n.parse::<u32>() else {
                continue;
            };
            if !after.starts_with(") @group( 1 )") {
                continue;
            }
            let decl = &after[..after.find(';').unwrap_or(after.len())];
            let slot = if decl.contains("objectStruct") {
                Slot::Object
            } else if decl.contains("NodeBuffer_") {
                let name = decl
                    .split(':')
                    .nth(1)
                    .map(str::trim)
                    .unwrap_or_default()
                    .to_string();
                let def = source
                    .find(&format!("struct {name} {{"))
                    .map(|j| &source[j..])
                    .unwrap_or("");
                if def[..def.find('}').unwrap_or(0)].contains("mat4x4") {
                    Slot::Instances
                } else {
                    Slot::Pillars
                }
            } else if decl.contains("sampler_comparison") {
                Slot::Compare
            } else if decl.contains(": sampler") {
                Slot::Linear
            } else if decl.contains("texture_depth_2d") {
                Slot::Depth
            } else if decl.contains("texture_3d") {
                Slot::Atlas
            } else if decl.contains("texture_2d") {
                if out.iter().any(|(b, _)| *b == binding) {
                    continue;
                }
                textures += 1;
                if textures == 1 { Slot::Dfg } else { Slot::Env }
            } else {
                continue;
            };
            if !out.iter().any(|(b, _)| *b == binding) {
                out.push((binding, slot));
            }
        }
    }
    out
}
/// A material's stages with its instance array sized to `capacity`.
fn shader(vs: &'static str, fs: &'static str, capacity: Option<usize>) -> Result<Shader> {
    let mut vs = vs.to_string();
    if let Some(capacity) = capacity {
        // createInstances: the instance matrices' uniform array holds the
        // capacity ( a power of two ).
        if let Some(i) = vs.find("array< mat4x4<f32>, ") {
            let start = i + "array< mat4x4<f32>, ".len();
            let end = start + vs[start..].find(' ').unwrap_or(0);
            vs.replace_range(start..end, &capacity.to_string());
        }
    }
    let bad = |e: String| Error::Asset(format!("city uniforms: {e}"));
    Ok(Shader {
        attrs: attributes(&vs),
        slots: slots(&vs, fs),
        object: object_layout(&vs, fs).map_err(bad)?,
        render: render_layout(&vs, fs).map_err(bad)?,
        vs,
        fs,
    })
}
/// MathUtils.ceilPowerOfTwo( max( 1, n ) ).
fn capacity(n: usize) -> usize {
    n.max(1).next_power_of_two()
}
/// A resident geometry: attribute buffers by name, index and bounds.
struct Mesh {
    buffers: Vec<(String, wgpu::Buffer)>,
    index: Option<wgpu::Buffer>,
    count: u32,
    /// The geometry's bounding sphere ( centre, radius ).
    sphere: (V3, f64),
}
fn vertex_buffer(r: &Renderer, label: &str, data: &[f32]) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        })
}
impl Mesh {
    fn new(r: &Renderer, g: &G) -> Self {
        let buffers = g
            .attributes
            .iter()
            .map(|(name, _, v)| (name.to_string(), vertex_buffer(r, "city attribute", v)))
            .collect();
        let index = g.index.as_ref().map(|i| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("city index"),
                    contents: bytemuck::cast_slice(i),
                    usage: wgpu::BufferUsages::INDEX,
                })
        });
        Self {
            buffers,
            index,
            count: g.index.as_ref().map_or(g.count(), Vec::len) as u32,
            sphere: g.bounding_sphere(),
        }
    }
    fn tower(r: &Renderer, b: &city::Tower) -> Self {
        let g = &b.building;
        let attribute =
            |name: &str, data: &[f32]| (name.to_string(), vertex_buffer(r, "tower", data));
        Self {
            buffers: vec![
                attribute("partId", &g.part_id),
                attribute("roomCenter", &g.room_center),
                attribute("roomSize", &g.room_size),
                attribute("position", &g.position),
                attribute("normal", &g.normal),
                attribute("uv", &g.uv),
            ],
            index: None,
            count: g.part_id.len() as u32,
            sphere: (g.center, g.radius),
        }
    }
}
/// An instanced generator's per-instance data.
struct Instances {
    matrices: wgpu::Buffer,
    count: u32,
    /// Instance-step attributes ( paintColor, personSeed ).
    extra: Vec<(String, wgpu::Buffer)>,
    /// InstancedMesh.computeBoundingSphere(): the instances' union.
    sphere: (V3, f64),
}
impl Instances {
    fn new(
        r: &Renderer,
        placements: &[M4],
        capacity: usize,
        extra: Vec<(&str, Vec<f32>)>,
        geometry: (V3, f64),
    ) -> Self {
        let mut data = vec![0f32; capacity.max(placements.len()) * 16];
        for (i, m) in placements.iter().enumerate() {
            for k in 0..16 {
                data[i * 16 + k] = m[k] as f32;
            }
        }
        let matrices = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("city instances"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        // Sphere.union over each instance's sphere ( applyMatrix4: the
        // centre transformed, the radius by the largest scale ).
        let mut sphere: Option<(V3, f64)> = None;
        for m in placements {
            let m: M4 = std::array::from_fn(|k| m[k] as f32 as f64);
            let c = geometry.0;
            let center = [
                m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12],
                m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13],
                m[2] * c[0] + m[6] * c[1] + m[10] * c[2] + m[14],
            ];
            let scale = (0..3)
                .map(|col| {
                    m[col * 4] * m[col * 4]
                        + m[col * 4 + 1] * m[col * 4 + 1]
                        + m[col * 4 + 2] * m[col * 4 + 2]
                })
                .fold(0f64, f64::max)
                .sqrt();
            let s = (center, geometry.1 * scale);
            sphere = Some(match sphere {
                None => s,
                Some(a) => union(a, s),
            });
        }
        Self {
            matrices,
            count: placements.len() as u32,
            extra: extra
                .into_iter()
                .map(|(n, v)| {
                    (
                        n.to_string(),
                        vertex_buffer(r, "city instance attribute", &v),
                    )
                })
                .collect(),
            sphere: sphere.unwrap_or(([0.; 3], -1.)),
        }
    }
}
/// Sphere.union( sphere ).
fn union(a: (V3, f64), b: (V3, f64)) -> (V3, f64) {
    if a.1 < 0. {
        return b;
    }
    if b.1 < 0. {
        return a;
    }
    let (mut center, mut radius) = a;
    if center == b.0 {
        return (center, radius.max(b.1));
    }
    // expandByPoint for the far points of b along the centre line.
    let d = sub(b.0, center);
    let len = length(d);
    let dir = if len == 0. {
        [0., 0., 0.]
    } else {
        normalize(d)
    };
    let far = [
        b.0[0] + dir[0] * b.1,
        b.0[1] + dir[1] * b.1,
        b.0[2] + dir[2] * b.1,
    ];
    let near = [
        b.0[0] - dir[0] * b.1,
        b.0[1] - dir[1] * b.1,
        b.0[2] - dir[2] * b.1,
    ];
    for p in [far, near] {
        let delta = sub(p, center);
        let length_sq = dot(delta, delta);
        if length_sq > radius * radius {
            let l = length_sq.sqrt();
            let shift = (l - radius) * 0.5;
            for k in 0..3 {
                center[k] += delta[k] * (shift / l);
            }
            radius += shift;
        }
    }
    (center, radius)
}
/// The page's materials.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum K {
    Tower,
    Ground,
    Slab,
    Curb,
    Light,
    Signal,
    Can,
    Bench,
    Hydrant,
    Tree,
    Walk,
    Stand,
    Suv,
    Sedan,
    Taxi,
    GroundBake,
    ProxyBake,
    ProxyGi,
}
/// A material: its scene and shadow stages and its own values.
struct Kind {
    main: Shader,
    shadow: Option<Shader>,
    roughness: f64,
    metalness: f64,
    custom: Vec<Vec<f64>>,
}
/// The car material's uniforms ( radius, axle, seams, handles, belt,
/// lamps ) and pillars.
fn car_uniforms(spec: &furniture::Spec) -> (Vec<Vec<f64>>, [f64; 2]) {
    let front = spec.front_base();
    let door_end = if spec.rails {
        spec.pillars[1] - 0.08
    } else {
        spec.rear_base()[2] + 0.1
    };
    (
        vec![
            vec![spec.wheel_radius],
            vec![spec.wheel_z],
            vec![front[2] - 0.04, spec.pillars[0], door_end],
            vec![spec.pillars[0] + 0.17, door_end + 0.17],
            vec![front[1]],
            spec.lamps.to_vec(),
        ],
        [spec.pillars[0], spec.pillars.get(1).copied().unwrap_or(9.)],
    )
}
/// A scene draw: material, mesh, transform and instances.
struct Item {
    kind: K,
    mesh: usize,
    model: M4,
    instances: Option<usize>,
    cast: bool,
}
/// The probe grid's bounds ( max, min ), resolution and intensity.
fn grid_uniforms() -> Grid {
    let max = [0, 1, 2].map(|k| GRID_CENTER[k] + GRID_SIZE[k] / 2.);
    let min = [0, 1, 2].map(|k| GRID_CENTER[k] - GRID_SIZE[k] / 2.);
    (max, min, GRID_RES.map(f64::from), 1.)
}
/// LightProbeGrid.getProbePosition( ix, iy, iz ).
fn probe_position(i: [u32; 3]) -> V3 {
    [0, 1, 2].map(|k| {
        let (c, s, n) = (GRID_CENTER[k], GRID_SIZE[k], GRID_RES[k]);
        if n > 1 {
            c - s / 2. + i[k] as f64 * s / (n - 1) as f64
        } else {
            c
        }
    })
}
/// CubeCamera's WebGPU faces: the projection and the six face rotations.
fn cube_cameras() -> (M4, [M4; 6]) {
    let m = CUBE_FAR / (CUBE_NEAR - CUBE_FAR);
    let projection = [
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
    ];
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
    ];
    (projection, views)
}
fn mat(m: Matrix4) -> M4 {
    m.to_cols_array()
}
fn transform(m: &M4, p: V3) -> [f64; 4] {
    [
        m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
        m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
        m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
        m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15],
    ]
}
/// A frustum's planes from a projection × view ( WebGPU depth ).
fn frustum(m: &M4) -> [[f64; 4]; 6] {
    let row = |r: usize| [m[r], m[4 + r], m[8 + r], m[12 + r]];
    let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
    let plane = |a: [f64; 4], s: f64, b: [f64; 4]| {
        let p = [0, 1, 2, 3].map(|k| a[k] + s * b[k]);
        let l = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        p.map(|v| v / l)
    };
    [
        plane(r3, -1., r0),
        plane(r3, 1., r0),
        plane(r3, 1., r1),
        plane(r3, -1., r1),
        plane(r3, -1., r2),
        plane(r2, 0., r2),
    ]
}
fn visible(planes: &[[f64; 4]; 6], (c, r): (V3, f64)) -> bool {
    planes
        .iter()
        .all(|p| p[0] * c[0] + p[1] * c[1] + p[2] * c[2] + p[3] >= -r)
}
/// A camera's matrices.
#[derive(Clone, Copy)]
struct View {
    projection: M4,
    view: M4,
    world: M4,
}
/// Camera.updateMatrixWorld's matrixWorldInverse: the world matrix
/// inverted when its scale is exactly one, else recomposed from its
/// decomposed rotation without the scale first.
fn camera_inverse(world: &M4) -> M4 {
    let column = |c: usize| length([world[c * 4], world[c * 4 + 1], world[c * 4 + 2]]);
    let (sx, sy, sz) = (column(0), column(1), column(2));
    if sx == 1. && sy == 1. && sz == 1. {
        return inverse(world);
    }
    // Matrix4.decompose ( the city's camera has a positive determinant ).
    let mut m = *world;
    for (c, s) in [sx, sy, sz].iter().enumerate() {
        for r in 0..3 {
            m[c * 4 + r] *= 1. / s;
        }
    }
    let q = rotation_quaternion(&m);
    inverse(&compose([world[12], world[13], world[14]], q, [1.; 3]))
}
/// Quaternion.setFromRotationMatrix( m ).
fn rotation_quaternion(m: &M4) -> Q {
    let (m11, m12, m13) = (m[0], m[4], m[8]);
    let (m21, m22, m23) = (m[1], m[5], m[9]);
    let (m31, m32, m33) = (m[2], m[6], m[10]);
    let trace = m11 + m22 + m33;
    if trace > 0. {
        let s = 0.5 / (trace + 1.).sqrt();
        [(m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s]
    } else if m11 > m22 && m11 > m33 {
        let s = 2. * (1. + m11 - m22 - m33).sqrt();
        [0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s]
    } else if m22 > m33 {
        let s = 2. * (1. + m22 - m11 - m33).sqrt();
        [(m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s]
    } else {
        let s = 2. * (1. + m33 - m11 - m22).sqrt();
        [(m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s]
    }
}
/// Matrix4.lookAt( eye, target, up )'s basis, positioned at the eye.
fn look_at(eye: V3, target: V3) -> M4 {
    let mut z = sub(eye, target);
    if dot(z, z) == 0. {
        z[2] = 1.;
    }
    let z = normalize(z);
    let mut x = cross([0., 1., 0.], z);
    if dot(x, x) == 0. {
        let mut zz = z;
        zz[2] += 0.0001;
        x = cross([0., 1., 0.], normalize(zz));
    }
    let x = normalize(x);
    let y = cross(z, x);
    set_position(basis(x, y, z), eye)
}
/// The sun for the time of day ( updateSun ): direction, the key light's
/// colour × intensity, and the shadow camera fitted around the city.
struct Sun {
    direction: V3,
    color: [f64; 3],
    position: V3,
    shadow: View,
    matrix: M4,
}
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}
fn hex(c: u32) -> [f64; 3] {
    [16, 8, 0].map(|s| srgb_to_linear(((c >> s) & 255) as f64 / 255.))
}
fn sun(time_of_day: f64, layout: &city::Layout) -> Sun {
    let u = (time_of_day - 12.) / 6.;
    let elevation = (1. - u * u).max(0.) * 72.;
    let azimuth = 90. - u * 55.;
    // MathUtils.degToRad: degrees × DEG2RAD.
    let deg2rad = PI / 180.;
    let (phi, theta) = ((90. - elevation) * deg2rad, azimuth * deg2rad);
    let direction = [phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos()];
    let sin_elevation = (elevation * deg2rad).sin();
    let transmittance = sin_elevation.max(0.).sqrt();
    let (a, b) = (hex(0xffb072), hex(0xfff4e8));
    let intensity = 100. * transmittance;
    let color = [0, 1, 2].map(|k| (a[k] + (b[k] - a[k]) * transmittance) * intensity);
    let position = direction.map(|v| v * 600.);
    // The city box in light space: the ortho bounds and the depth range.
    let floor_w = layout.city_w + 2. * layout.street;
    let floor_d = layout.city_d + 2. * layout.street;
    let (min, max) = (
        [-floor_w / 2., 0., -floor_d / 2.],
        [floor_w / 2., 160., floor_d / 2.],
    );
    // The light-space quaternion: Matrix4.lookAt( sun, origin, up ) as a
    // rotation, inverted; the shadow camera's own lookAt composes the same
    // rotation at the light's position.
    let q = rotation_quaternion(&look_at(position, [0.; 3]));
    let world = compose(position, q, [1.; 3]);
    let view = camera_inverse(&world);
    let light_space = [-q[0], -q[1], -q[2], q[3]];
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..8 {
        let corner = [
            if i & 1 != 0 { max[0] } else { min[0] },
            if i & 2 != 0 { max[1] } else { min[1] },
            if i & 4 != 0 { max[2] } else { min[2] },
        ];
        let p = apply_quaternion(sub(corner, position), light_space);
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let pad = 2.;
    let (left, right, bottom, top) = (lo[0] - pad, hi[0] + pad, lo[1] - pad, hi[1] + pad);
    let (near, far) = (-hi[2] - pad, -lo[2] + pad);
    // OrthographicCamera.updateProjectionMatrix ( zoom 1 ): the bounds about
    // their centre.
    let (dx, dy) = ((right - left) / 2., (top - bottom) / 2.);
    let (cx, cy) = ((right + left) / 2., (top + bottom) / 2.);
    let (left, right, top, bottom) = (cx - dx, cx + dx, cy + dy, cy - dy);
    // makeOrthographic in WebGPU depth.
    let projection = [
        2. / (right - left),
        0.,
        0.,
        0.,
        0.,
        2. / (top - bottom),
        0.,
        0.,
        0.,
        0.,
        -1. / (far - near),
        0.,
        -(right + left) / (right - left),
        -(top + bottom) / (top - bottom),
        -near / (far - near),
        1.,
    ];
    let bias = [
        0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
    ];
    let matrix = multiply(&bias, &multiply(&projection, &view));
    Sun {
        direction,
        color,
        position,
        shadow: View {
            projection,
            view,
            world,
        },
        matrix,
    }
}
/// The SkyMesh uniforms ( turbidity 8, rayleigh 3, mieCoefficient 0.008,
/// mieDirectionalG 0.88 ) with or without the sun disc.
fn sky_values(direction: V3, disc: bool) -> Vec<(&'static str, Vec<f64>)> {
    vec![
        ("nodeUniform0", scaling([10000.; 3]).to_vec()),
        ("nodeUniform2", vec![0.88]),
        ("nodeUniform3", vec![if disc { 1. } else { 0. }]),
        ("nodeUniform4", vec![0.4]),
        ("nodeUniform5", vec![0.5]),
        ("nodeUniform6", vec![0.0002]),
        ("nodeUniform8", vec![0.00002]),
        ("nodeUniform9", vec![0.4]),
        ("nodeUniform10", vec![1.]),
        ("nodeUniform11", direction.to_vec()),
        ("nodeUniform12", vec![3.]),
        ("nodeUniform13", vec![8.]),
        ("nodeUniform14", vec![0.008]),
    ]
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
fn write_static(
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
fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    samples: u32,
    copy: bool,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("generator city"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: samples,
        dimension,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | if samples == 1 {
                wgpu::TextureUsages::TEXTURE_BINDING
            } else {
                wgpu::TextureUsages::empty()
            }
            | if copy {
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST
            } else {
                wgpu::TextureUsages::empty()
            },
        view_formats: &[],
    })
}
fn view2d(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
/// The draws of one camera's pass: per item its draw and object buffer.
struct PassDraws {
    draws: Vec<(usize, Draw)>,
}
/// The bloom chain's targets and draws for one size.
struct Bloom {
    bright: wgpu::TextureView,
    levels: Vec<(wgpu::TextureView, wgpu::TextureView, (u32, u32))>,
    high: (wgpu::RenderPipeline, wgpu::BindGroup),
    blurs: Vec<[(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Buffer); 2]>,
    composite: (wgpu::RenderPipeline, wgpu::BindGroup),
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    main: PassDraws,
    sky: Draw,
    bloom: Bloom,
    output: Draw,
    screen: RenderTarget,
}
/// The scene's resident data for one seed.
struct Built {
    meshes: Vec<Mesh>,
    instances: Vec<Instances>,
    items: Vec<Item>,
    /// The tower proxy ( an instanced unit box ).
    proxy: (usize, usize),
    /// Per item and stage ( main, shadow ), the object buffer.
    objects: HashMap<(usize, u8), wgpu::Buffer>,
    /// The car materials' pillars.
    pillars: HashMap<K, wgpu::Buffer>,
    shadow: PassDraws,
    /// Per face, the bake's draws for pass 0 and pass 1 ( sky, ground, proxy ).
    bake: [Vec<[Draw; 3]>; 2],
}
pub(super) struct Demo {
    walker: FirstPerson,
    pending: bool,
    time: f64,
    last: f64,
    /// seed, timeOfDay, exposure, gi and showProbes.
    params: [f64; 5],
    rebuild: bool,
    /// probes.visible changed: the scene materials' grid intensity.
    gi_pending: bool,
    /// The node time updateSun() bakes the sky at.
    pmrem_pending: Option<f64>,
    bake_index: u32,
    bake_pass: u32,
    layout: city::Layout,
    kinds: HashMap<K, Kind>,
    /// The instance capacities the materials' arrays are sized for.
    capacities: Vec<usize>,
    pipelines: HashMap<(K, u8, u32), wgpu::RenderPipeline>,
    built: Option<Built>,
    /// Per material and camera, the render buffer.
    renders: HashMap<(K, u8), wgpu::Buffer>,
    sky_box: Buffers,
    sky_render: wgpu::Buffer,
    sky_object: wgpu::Buffer,
    sky_bake_object: wgpu::Buffer,
    sky_face_renders: Vec<wgpu::Buffer>,
    pmrem: Pmrem,
    pmrem_depth: wgpu::TextureView,
    box_draw: Draw,
    pmrem_draws: Vec<Draw>,
    pmrem_renders: Vec<wgpu::Buffer>,
    pmrem_object: wgpu::Buffer,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    atlas: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    bounce: wgpu::Texture,
    bounce_view: wgpu::TextureView,
    cube_faces: Vec<wgpu::TextureView>,
    cube_depth: wgpu::TextureView,
    batch: wgpu::TextureView,
    sh: Draw,
    repack: Vec<(wgpu::RenderPipeline, Vec<wgpu::BindGroup>)>,
    high_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    tints: wgpu::Buffer,
    output_render: wgpu::Buffer,
    quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The cameras a material's render buffer serves.
const MAIN: u8 = 0;
const SHADOW: u8 = 1;
const FACE: u8 = 2;
fn kind(main: Shader, shadow: Option<Shader>) -> Kind {
    Kind {
        main,
        shadow,
        roughness: 1.,
        metalness: 0.,
        custom: vec![],
    }
}
/// The page's materials, with the instance arrays sized for this city.
fn kinds(f: &city::Furniture, cars: [usize; 3], people: [usize; 2]) -> Result<HashMap<K, Kind>> {
    let n = |len: usize| Some(capacity(len));
    let car_cap = n(f.cars.len());
    let person_cap = n(f.people.len());
    let mut k = HashMap::new();
    let _ = (cars, people);
    k.insert(
        K::Tower,
        kind(
            shader(wgsl!("building_vs"), wgsl!("building_fs"), None)?,
            Some(shader(
                wgsl!("building_shadow_vs"),
                wgsl!("building_shadow_fs"),
                None,
            )?),
        ),
    );
    k.insert(
        K::Ground,
        kind(shader(wgsl!("ground_vs"), wgsl!("ground_fs"), None)?, None),
    );
    let blocks = n(4);
    k.insert(
        K::Slab,
        kind(shader(wgsl!("slab_vs"), wgsl!("slab_fs"), blocks)?, None),
    );
    k.insert(
        K::Curb,
        kind(shader(wgsl!("curb_vs"), wgsl!("curb_fs"), blocks)?, None),
    );
    let pair = |vs, fs, svs, sfs, cap| -> Result<Kind> {
        Ok(kind(shader(vs, fs, cap)?, Some(shader(svs, sfs, cap)?)))
    };
    k.insert(
        K::Light,
        pair(
            wgsl!("streetlight_vs"),
            wgsl!("streetlight_fs"),
            wgsl!("streetlight_shadow_vs"),
            wgsl!("streetlight_shadow_fs"),
            n(f.lights.len()),
        )?,
    );
    k.insert(
        K::Signal,
        pair(
            wgsl!("signal_vs"),
            wgsl!("signal_fs"),
            wgsl!("signal_shadow_vs"),
            wgsl!("signal_shadow_fs"),
            n(f.signals.len()),
        )?,
    );
    k.insert(
        K::Can,
        pair(
            wgsl!("trashcan_vs"),
            wgsl!("trashcan_fs"),
            wgsl!("trashcan_shadow_vs"),
            wgsl!("trashcan_shadow_fs"),
            n(f.cans.len()),
        )?,
    );
    k.insert(
        K::Bench,
        pair(
            wgsl!("bench_vs"),
            wgsl!("bench_fs"),
            wgsl!("bench_shadow_vs"),
            wgsl!("bench_shadow_fs"),
            n(f.benches.len()),
        )?,
    );
    k.insert(
        K::Hydrant,
        pair(
            wgsl!("hydrant_vs"),
            wgsl!("hydrant_fs"),
            wgsl!("hydrant_shadow_vs"),
            wgsl!("hydrant_shadow_fs"),
            n(f.hydrants.len()),
        )?,
    );
    k.insert(
        K::Tree,
        pair(
            wgsl!("tree_vs"),
            wgsl!("tree_fs"),
            wgsl!("tree_shadow_vs"),
            wgsl!("tree_shadow_fs"),
            n(f.trees.len()),
        )?,
    );
    for (key, vs, svs) in [
        (K::Walk, wgsl!("walk_vs"), wgsl!("walk_shadow_vs")),
        (K::Stand, wgsl!("stand_vs"), wgsl!("stand_shadow_vs")),
    ] {
        let mut m = pair(
            vs,
            wgsl!("people_fs"),
            svs,
            wgsl!("people_shadow_fs"),
            person_cap,
        )?;
        // uniform( 1.75 / height ).
        m.custom = vec![vec![1.]];
        k.insert(key, m);
    }
    for (key, spec, vs, fs, svs, sfs) in [
        (
            K::Suv,
            &furniture::SUV,
            wgsl!("suv_vs"),
            wgsl!("suv_fs"),
            wgsl!("suv_shadow_vs"),
            wgsl!("suv_shadow_fs"),
        ),
        (
            K::Sedan,
            &furniture::SEDAN,
            wgsl!("sedan_vs"),
            wgsl!("sedan_fs"),
            wgsl!("sedan_shadow_vs"),
            wgsl!("sedan_shadow_fs"),
        ),
        (
            K::Taxi,
            &furniture::TAXI,
            wgsl!("taxi_vs"),
            wgsl!("taxi_fs"),
            wgsl!("taxi_shadow_vs"),
            wgsl!("taxi_shadow_fs"),
        ),
    ] {
        let mut m = pair(vs, fs, svs, sfs, car_cap)?;
        m.custom = car_uniforms(spec).0;
        k.insert(key, m);
    }
    k.insert(
        K::GroundBake,
        kind(
            shader(wgsl!("ground_bake_vs"), wgsl!("ground_bake_fs"), None)?,
            None,
        ),
    );
    // The proxy: roughness 0.4, metalness 0.4, one box per tower.
    let towers = Some(24);
    let mut proxy = kind(
        shader(wgsl!("proxy_bake_vs"), wgsl!("proxy_bake_fs"), towers)?,
        Some(shader(
            wgsl!("proxy_shadow_vs"),
            wgsl!("proxy_shadow_fs"),
            towers,
        )?),
    );
    proxy.roughness = 0.4;
    proxy.metalness = 0.4;
    k.insert(K::ProxyBake, proxy);
    let mut gi = kind(
        shader(wgsl!("proxy_gi_vs"), wgsl!("proxy_gi_fs"), towers)?,
        None,
    );
    gi.roughness = 0.4;
    gi.metalness = 0.4;
    k.insert(K::ProxyGi, gi);
    Ok(k)
}
/// A material pipeline: separate buffers per attribute, back faces culled.
fn material_pipeline(
    r: &Renderer,
    s: &Shader,
    format: wgpu::TextureFormat,
    samples: u32,
    cw: bool,
) -> wgpu::RenderPipeline {
    let lists: Vec<[wgpu::VertexAttribute; 1]> = s
        .attrs
        .iter()
        .map(|a| {
            [wgpu::VertexAttribute {
                format: a.format,
                offset: 0,
                shader_location: a.location,
            }]
        })
        .collect();
    let buffers: Vec<wgpu::VertexBufferLayout> = s
        .attrs
        .iter()
        .zip(&lists)
        .map(|(a, list)| wgpu::VertexBufferLayout {
            array_stride: a.format.size(),
            step_mode: if a.instance {
                wgpu::VertexStepMode::Instance
            } else {
                wgpu::VertexStepMode::Vertex
            },
            attributes: list,
        })
        .collect();
    culled_pipeline(
        r,
        "generator city",
        (&s.vs, s.fs),
        &buffers,
        &[format],
        Some((wgpu::CompareFunction::LessEqual, true)),
        (cw, false),
        (samples, wgpu::PrimitiveTopology::TriangleList),
        Some(wgpu::Face::Back),
    )
}
/// The scene textures a material binds.
struct Resources<'a> {
    linear: &'a wgpu::Sampler,
    compare: &'a wgpu::Sampler,
    dfg: &'a wgpu::TextureView,
    depth: &'a wgpu::TextureView,
    atlas: &'a wgpu::TextureView,
    env: &'a wgpu::TextureView,
}
#[allow(clippy::too_many_arguments)]
fn material_draw(
    r: &Renderer,
    pipeline: &wgpu::RenderPipeline,
    s: &Shader,
    render: &wgpu::Buffer,
    object: &wgpu::Buffer,
    instances: Option<&Instances>,
    pillars: Option<&wgpu::Buffer>,
    res: &Resources,
) -> Result<Draw> {
    let mut entries = vec![];
    for &(binding, slot) in &s.slots {
        let resource = match slot {
            Slot::Object => object.as_entire_binding(),
            Slot::Instances => instances
                .ok_or(Error::Invalid("city instances"))?
                .matrices
                .as_entire_binding(),
            Slot::Pillars => pillars
                .ok_or(Error::Invalid("city pillars"))?
                .as_entire_binding(),
            Slot::Linear => wgpu::BindingResource::Sampler(res.linear),
            Slot::Compare => wgpu::BindingResource::Sampler(res.compare),
            Slot::Dfg => wgpu::BindingResource::TextureView(res.dfg),
            Slot::Depth => wgpu::BindingResource::TextureView(res.depth),
            Slot::Atlas => wgpu::BindingResource::TextureView(res.atlas),
            Slot::Env => wgpu::BindingResource::TextureView(res.env),
        };
        entries.push((binding, resource));
    }
    Ok((
        pipeline.clone(),
        vec![
            bind(
                r,
                pipeline.get_bind_group_layout(0),
                &[(0, render.as_entire_binding())],
            ),
            bind(r, pipeline.get_bind_group_layout(1), &entries),
        ],
    ))
}
/// Binds a mesh's attributes in the shader's order and draws it.
fn draw_mesh(pass: &mut wgpu::RenderPass, s: &Shader, mesh: &Mesh, instances: Option<&Instances>) {
    for (slot, a) in s.attrs.iter().enumerate() {
        let buffer = if a.instance {
            instances
                .and_then(|i| i.extra.iter().find(|(n, _)| *n == a.name))
                .map(|(_, b)| b)
        } else {
            mesh.buffers
                .iter()
                .find(|(n, _)| *n == a.name)
                .map(|(_, b)| b)
        };
        if let Some(b) = buffer {
            pass.set_vertex_buffer(slot as u32, b.slice(..));
        }
    }
    let count = instances.map_or(1, |i| i.count);
    match &mesh.index {
        Some(index) => {
            pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.count, 0, 0..count);
        }
        None => pass.draw(0..mesh.count, 0..count),
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 55.,
            near: 1.,
            far: 20000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-35., 55., -100.);
        // controls.lookAt( 35, 55, 0 ): the latitude and longitude of the look.
        s.look_at(c, Vector3::new(35., 55., 0.))?;
        let mut walker = FirstPerson::default();
        walker.speed = 15.;
        let look = s.get(c)?.quaternion * -Vector3::Z;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        let sampler = |compare| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare,
                ..Default::default()
            })
        };
        let linear = sampler(None);
        let compare = sampler(Some(wgpu::CompareFunction::LessEqual));
        let d2 = wgpu::TextureDimension::D2;
        let shadow_color = view2d(&texture(r, (MAP, MAP, 1), d2, BYTE, 1, false));
        let shadow_depth = view2d(&texture(r, (MAP, MAP, 1), d2, DEPTH, 1, false));
        // The PMREM ( fromScene of the sky without its disc ), as the
        // building page bakes it.
        let pmrem = Pmrem::new(r, &linear, 8, GGX_8)?;
        let pmrem_depth = view2d(&texture(r, (768, 1024, 1), d2, DEPTH, 1, false));
        let unit = crate::geometry::BoxGeometry::build(1., 1., 1.)?;
        let positions: Vec<f32> = {
            let a = unit
                .attributes
                .get("position")
                .ok_or(Error::Invalid("sky box position"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect::<Result<_>>()?
        };
        let index = unit.index.clone().ok_or(Error::Invalid("sky box index"))?;
        let sky_box = upload(r, &[bytemuck::cast_slice(&positions)], &index);
        let position_only = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let box_layouts = layouts(&position_only, &[(12, false)]);
        let box_render = uniform(r, "PMREM box", BOX_VS, "renderStruct")?;
        let box_object = uniform(r, "PMREM box", BOX_FS, "objectStruct")?;
        let box_draw = two_groups(
            r,
            pipeline(
                r,
                "PMREM box",
                (BOX_VS, BOX_FS),
                &box_layouts,
                HALF,
                1,
                (wgpu::CompareFunction::Always, false),
                true,
            ),
            &box_render,
            &[(0, box_object.as_entire_binding())],
        );
        write_static(
            r,
            &box_render,
            BOX_VS,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", FACE_PROJECTION.to_vec()),
                ("cameraViewMatrix", IDENTITY.to_vec()),
            ],
        )?;
        write_static(
            r,
            &box_object,
            BOX_FS,
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform4", IDENTITY.to_vec()),
            ],
        )?;
        let pmrem_object = uniform(r, "sky bake", SKY_FS, "objectStruct")?;
        let bake_pipeline = pipeline(
            r,
            "sky bake",
            (SKY_VS, SKY_FS),
            &box_layouts,
            HALF,
            1,
            (wgpu::CompareFunction::LessEqual, false),
            true,
        );
        let mut pmrem_renders = vec![];
        let mut pmrem_draws = vec![];
        for _ in 0..6 {
            let render = uniform(r, "sky bake", SKY_FS, "renderStruct")?;
            pmrem_draws.push(two_groups(
                r,
                bake_pipeline.clone(),
                &render,
                &[(0, pmrem_object.as_entire_binding())],
            ));
            pmrem_renders.push(render);
        }
        // The probe grid's live atlas and the bounce pass's snapshot: seven
        // sub-volumes, each padded by a slice at both ends.
        let [nx, ny, nz] = GRID_RES;
        let atlas_size = (nx, ny, 7 * (nz + 2));
        let d3 = wgpu::TextureDimension::D3;
        let atlas = texture(r, atlas_size, d3, HALF, 1, true);
        let bounce = texture(r, atlas_size, d3, HALF, 1, true);
        let atlas_view = view2d(&atlas);
        let bounce_view = view2d(&bounce);
        let cube = texture(r, (CUBE, CUBE, 6), d2, HALF, 1, false);
        let cube_faces = (0..6)
            .map(|i| {
                cube.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_array_layer: i,
                    array_layer_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let cube_view = cube.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let cube_depth = view2d(&texture(r, (CUBE, CUBE, 1), d2, DEPTH, 1, false));
        let batch = view2d(&texture(r, (9, PROBES, 1), d2, FLOAT, 1, false));
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let sh_pipeline =
            raw_pipeline(r, "city probe SH", SH_VS, SH_FS, &[], FLOAT, 1, None, false);
        let sh_render = init(
            "city SH render",
            &pack(
                SH_FS,
                "renderStruct",
                &[
                    ("cameraWorldMatrix", &IDENTITY),
                    (
                        "cameraProjectionMatrixInverse",
                        &[
                            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                        ],
                    ),
                ],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let sh_object = init(
            "city SH object",
            &pack(
                SH_FS,
                "objectStruct",
                &[("nodeUniform0", &[1.]), ("nodeUniform4", &IDENTITY)],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let sh = (
            sh_pipeline.clone(),
            vec![
                bind(
                    r,
                    sh_pipeline.get_bind_group_layout(0),
                    &[(0, sh_render.as_entire_binding())],
                ),
                bind(
                    r,
                    sh_pipeline.get_bind_group_layout(1),
                    &[
                        (0, sh_object.as_entire_binding()),
                        (1, wgpu::BindingResource::Sampler(&linear)),
                        (2, wgpu::BindingResource::TextureView(&cube_view)),
                    ],
                ),
            ],
        );
        // The seven repack materials ( the last one's quad stage is the
        // seed module ), per Z slice with its object struct.
        let repack_objects: Vec<wgpu::Buffer> = (0..nz)
            .map(|_| uniform(r, "city repack object", REPACK_FS[0], "objectStruct"))
            .collect::<Result<_>>()?;
        for (iz, object) in repack_objects.iter().enumerate() {
            write_static(
                r,
                object,
                REPACK_FS[0],
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.]),
                    ("nodeUniform2", vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]),
                    ("nodeUniform3", GRID_RES.map(f64::from).to_vec()),
                    ("nodeUniform4", vec![iz as f64]),
                    ("nodeUniform5", vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]),
                ],
            )?;
        }
        let repack = REPACK_FS
            .iter()
            .enumerate()
            .map(|(t, fs)| {
                let p = raw_pipeline(
                    r,
                    "city repack",
                    if t == 6 { SEED_VS } else { REPACK_VS },
                    fs,
                    &[],
                    HALF,
                    1,
                    None,
                    false,
                );
                let groups = repack_objects
                    .iter()
                    .map(|object| {
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[
                                (0, object.as_entire_binding()),
                                (1, wgpu::BindingResource::TextureView(&batch)),
                            ],
                        )
                    })
                    .collect();
                (p, groups)
            })
            .collect();
        let tints: Vec<f32> = (0..LEVELS).flat_map(|_| [1f32, 1., 1., 0.]).collect();
        // QuadMesh's uvs for its three vertices.
        let quad: [f32; 6] = [0., -1., 0., 1., 2., 1.];
        let mut demo = Self {
            walker,
            pending: true,
            time: 0.,
            last: 0.,
            params: [94., 6.4, 0.13, 1., 0.],
            rebuild: true,
            gi_pending: false,
            pmrem_pending: Some(0.),
            bake_index: 0,
            bake_pass: 0,
            layout: city::layout(),
            kinds: HashMap::new(),
            capacities: vec![],
            pipelines: HashMap::new(),
            built: None,
            renders: HashMap::new(),
            sky_box,
            sky_render: uniform(r, "sky", SKY_FS, "renderStruct")?,
            sky_object: uniform(r, "sky", SKY_FS, "objectStruct")?,
            sky_bake_object: uniform(r, "sky probe bake", SKY_FS, "objectStruct")?,
            sky_face_renders: (0..6)
                .map(|_| uniform(r, "sky probe face", SKY_FS, "renderStruct"))
                .collect::<Result<_>>()?,
            pmrem,
            pmrem_depth,
            box_draw,
            pmrem_draws,
            pmrem_renders,
            pmrem_object,
            shadow_color,
            shadow_depth,
            atlas,
            atlas_view,
            bounce,
            bounce_view,
            cube_faces,
            cube_depth,
            batch,
            sh,
            repack,
            high_object: uniform(r, "bloom high pass", HIGH_FS, "objectStruct")?,
            composite_object: uniform(r, "bloom composite", COMPOSITE_FS, "objectStruct")?,
            tints: init(
                "bloom tints",
                bytemuck::cast_slice(&tints),
                wgpu::BufferUsages::UNIFORM,
            ),
            output_render: uniform(r, "city output", OUTPUT_FS, "renderStruct")?,
            quad: init(
                "city quad",
                bytemuck::cast_slice(&quad),
                wgpu::BufferUsages::VERTEX,
            ),
            linear,
            compare,
            targets: None,
        };
        demo.build(r)?;
        Ok(demo)
    }
}
/// PlaneGeometry( width, height ).
fn plane(width: f64, height: f64) -> G {
    let (hw, hh) = (width / 2., height / 2.);
    let (mut p, mut n, mut uv) = (vec![], vec![], vec![]);
    for iy in 0..2 {
        let y = iy as f64 * height - hh;
        for ix in 0..2 {
            let x = ix as f64 * width - hw;
            p.extend([x as f32, -y as f32, 0.]);
            n.extend([0f32, 0., 1.]);
            uv.extend([ix as f32, (1. - iy as f64) as f32]);
        }
    }
    G {
        attributes: vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        index: Some(vec![0, 2, 1, 2, 3, 1]),
    }
}
/// Which instance buffer an instanced generator's capacity follows.
fn capacities(c: &city::City) -> Vec<usize> {
    let f = &c.furniture;
    [
        f.lights.len(),
        f.signals.len(),
        f.cans.len(),
        f.benches.len(),
        f.hydrants.len(),
        f.trees.len(),
        f.people.len(),
        f.cars.len(),
    ]
    .map(capacity)
    .to_vec()
}
impl Demo {
    fn kind(&self, k: K) -> Result<&Kind> {
        self.kinds.get(&k).ok_or(Error::Invalid("city material"))
    }
    /// The shader of a material's stage ( 0 scene, 1 shadow ).
    fn stage(&self, k: K, stage: u8) -> Result<&Shader> {
        let kind = self.kind(k)?;
        if stage == SHADOW {
            kind.shadow
                .as_ref()
                .ok_or(Error::Invalid("city shadow stage"))
        } else {
            Ok(&kind.main)
        }
    }
    fn render_buffer(&mut self, r: &Renderer, k: K, camera: u8) -> Result<wgpu::Buffer> {
        if let Some(b) = self.renders.get(&(k, camera)) {
            return Ok(b.clone());
        }
        let s = self.stage(k, if camera == SHADOW { SHADOW } else { MAIN })?;
        let source = if s.fs.contains("struct renderStruct {") {
            s.fs
        } else {
            s.vs.as_str()
        };
        let b = uniform(r, "city render", source, "renderStruct")?;
        self.renders.insert((k, camera), b.clone());
        Ok(b)
    }
    /// An item's object struct for a material stage, written once.
    fn object_buffer(
        &self,
        r: &Renderer,
        item: &Item,
        k: K,
        stage: u8,
        gi: f64,
    ) -> Result<wgpu::Buffer> {
        let kind = self.kind(k)?;
        let s = self.stage(k, stage)?;
        let source = if s.fs.contains("struct objectStruct {") {
            s.fs
        } else {
            s.vs.as_str()
        };
        let buffer = uniform(r, "city object", source, "objectStruct")?;
        let mut grid = grid_uniforms();
        grid.3 = gi;
        let values = object_values(
            &s.object,
            &Object {
                model: item.model,
                seed: self.params[0] as u32,
                roughness: kind.roughness,
                metalness: kind.metalness,
                grid: Some(grid),
                custom: &kind.custom,
            },
        );
        write(r, &buffer, source, "objectStruct", &values)?;
        Ok(buffer)
    }
    /// generateCity(): the generators for the seed and their resident
    /// buffers, the shadow and probe-capture draws.
    fn build(&mut self, r: &Renderer) -> Result<()> {
        let c = city::build(self.params[0]);
        let caps = capacities(&c);
        if self.kinds.is_empty() || self.capacities != caps {
            self.kinds = kinds(&c.furniture, [0; 3], [0; 2])?;
            self.capacities = caps;
            self.renders.clear();
            self.pipelines.clear();
        }
        let (mut meshes, mut instances, mut items) = (vec![], vec![], vec![]);
        let l = self.layout;
        let ground_mesh =
            plane(l.city_w + 2. * l.street, l.city_d + 2. * l.street).rotate_x(-PI / 2.);
        meshes.push(Mesh::new(r, &ground_mesh));
        items.push(Item {
            kind: K::Ground,
            mesh: 0,
            model: IDENTITY,
            instances: None,
            cast: false,
        });
        for t in &c.towers {
            meshes.push(Mesh::tower(r, t));
            items.push(Item {
                kind: K::Tower,
                mesh: meshes.len() - 1,
                model: translation(t.position),
                instances: None,
                cast: true,
            });
        }
        // createInstances( geometry, material, count ): the capacity is the
        // generator's placement count, shared by the buckets.
        let mut instanced = |kind: K,
                             g: &G,
                             (placements, cap): (&[M4], usize),
                             extra: Vec<(&str, Vec<f32>)>,
                             cast: bool,
                             meshes: &mut Vec<Mesh>,
                             instances: &mut Vec<Instances>| {
            let mesh = Mesh::new(r, g);
            instances.push(Instances::new(r, placements, cap, extra, mesh.sphere));
            meshes.push(mesh);
            items.push(Item {
                kind,
                mesh: meshes.len() - 1,
                model: IDENTITY,
                instances: Some(instances.len() - 1),
                cast: cast && !placements.is_empty(),
            });
        };
        let (slab, curb) = city::sidewalk();
        instanced(
            K::Slab,
            &slab,
            (&c.slabs, capacity(c.slabs.len())),
            vec![],
            false,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Curb,
            &curb,
            (&c.slabs, capacity(c.slabs.len())),
            vec![],
            false,
            &mut meshes,
            &mut instances,
        );
        let f = &c.furniture;
        instanced(
            K::Light,
            &furniture::streetlight(),
            (&f.lights, capacity(f.lights.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Signal,
            &furniture::trafficlight(),
            (&f.signals, capacity(f.signals.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Can,
            &furniture::trashcan(),
            (&f.cans, capacity(f.cans.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Bench,
            &furniture::bench(),
            (&f.benches, capacity(f.benches.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Hydrant,
            &furniture::hydrant(),
            (&f.hydrants, capacity(f.hydrants.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        instanced(
            K::Tree,
            &furniture::street_tree(),
            (&f.trees, capacity(f.trees.len())),
            vec![],
            true,
            &mut meshes,
            &mut instances,
        );
        // PersonGenerator.build: the scaled placements bucketed by pose, with
        // each placement's index as its seed.
        let people_cap = capacity(f.people.len());
        let (mut walk, mut stand) = ((vec![], vec![]), (vec![], vec![]));
        for (i, p) in f.people.iter().enumerate() {
            let (posed, walking) = city::pose_person(i, p);
            let bucket = if walking { &mut walk } else { &mut stand };
            bucket.0.push(posed);
            bucket.1.push(i as f32);
        }
        for (kind, pose, (matrices, seeds)) in [(K::Walk, true, walk), (K::Stand, false, stand)] {
            let mut seed = vec![0f32; people_cap];
            seed[..seeds.len()].copy_from_slice(&seeds);
            instanced(
                kind,
                &furniture::person(pose),
                (&matrices, people_cap),
                vec![("personSeed", seed)],
                true,
                &mut meshes,
                &mut instances,
            );
        }
        // CarGenerator.build: the bodies bucketed by type in the order they
        // first appear, each bucket's paint in its own attribute.
        let car_cap = capacity(f.cars.len());
        let mut buckets: Vec<(usize, Vec<M4>, Vec<f32>)> = vec![];
        for (i, car) in f.cars.iter().enumerate() {
            let t = city::car_type(i, car.color);
            let at = match buckets.iter().position(|b| b.0 == t) {
                Some(at) => at,
                None => {
                    buckets.push((t, vec![], vec![]));
                    buckets.len() - 1
                }
            };
            buckets[at].1.push(car.matrix);
            buckets[at].2.extend(hex(car.color).map(|v| v as f32));
        }
        for (t, matrices, mut paint) in buckets {
            paint.resize(car_cap * 3, 0.);
            let (kind, spec) = match t {
                0 => (K::Sedan, &furniture::SEDAN),
                1 => (K::Suv, &furniture::SUV),
                _ => (K::Taxi, &furniture::TAXI),
            };
            instanced(
                kind,
                &furniture::car(spec),
                (&matrices, car_cap),
                vec![("paintColor", paint)],
                true,
                &mut meshes,
                &mut instances,
            );
        }
        // buildProxy(): a unit box per tower, scaled to it.
        let boxes: Vec<M4> = c
            .towers
            .iter()
            .map(|t| {
                let (p, s) = t.proxy;
                set_position(scaling(s), p)
            })
            .collect();
        let unit = prims::cuboid(1., 1., 1.);
        let proxy_mesh = Mesh::new(r, &unit);
        instances.push(Instances::new(
            r,
            &boxes,
            boxes.len(),
            vec![],
            proxy_mesh.sphere,
        ));
        meshes.push(proxy_mesh);
        let proxy = (meshes.len() - 1, instances.len() - 1);
        let mut pillars = HashMap::new();
        for (k, spec) in [
            (K::Suv, &furniture::SUV),
            (K::Sedan, &furniture::SEDAN),
            (K::Taxi, &furniture::TAXI),
        ] {
            let [a, b] = car_uniforms(spec).1;
            let data = [a as f32, 0., 0., 0., b as f32, 0., 0., 0.];
            pillars.insert(
                k,
                r.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("car pillars"),
                        contents: bytemuck::cast_slice(&data),
                        usage: wgpu::BufferUsages::UNIFORM,
                    }),
            );
        }
        let gi = if self.params[3] > 0.5 { 1. } else { 0. };
        let mut objects = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            objects.insert((i, MAIN), self.object_buffer(r, item, item.kind, MAIN, gi)?);
            if item.cast {
                objects.insert(
                    (i, SHADOW),
                    self.object_buffer(r, item, item.kind, SHADOW, 1.)?,
                );
            }
        }
        let proxy_item = Item {
            kind: K::ProxyBake,
            mesh: proxy.0,
            model: IDENTITY,
            instances: Some(proxy.1),
            cast: true,
        };
        let mut built = Built {
            meshes,
            instances,
            items,
            proxy,
            objects,
            pillars,
            shadow: PassDraws { draws: vec![] },
            bake: [vec![], vec![]],
        };
        // The proxy's and the bake's object structs: the bounce grid keeps
        // its intensity of 1.
        let ground = Item {
            kind: K::Ground,
            mesh: 0,
            model: IDENTITY,
            instances: None,
            cast: false,
        };
        let extra = [
            (usize::MAX, K::ProxyBake, MAIN, &proxy_item),
            (usize::MAX - 1, K::ProxyBake, SHADOW, &proxy_item),
            (usize::MAX - 2, K::ProxyGi, MAIN, &proxy_item),
            (usize::MAX - 3, K::GroundBake, MAIN, &ground),
            (usize::MAX - 4, K::Ground, MAIN, &ground),
        ];
        for (key, k, stage, item) in extra {
            let b = self.object_buffer(r, item, k, stage, 1.)?;
            built.objects.insert((key, stage), b);
        }
        self.built = Some(built);
        self.build_draws(r)?;
        self.targets = None;
        // resetProbeBake().
        self.bake_index = 0;
        self.bake_pass = 0;
        Ok(())
    }
    /// The scene materials' object structs with the grid's intensity for
    /// probes.visible ( the bake's bounce grid keeps 1 ).
    fn write_gi(&self, r: &Renderer) -> Result<()> {
        let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
        let gi = if self.params[3] > 0.5 { 1. } else { 0. };
        for (i, item) in b.items.iter().enumerate() {
            let kind = self.kind(item.kind)?;
            let s = &kind.main;
            let source = if s.fs.contains("struct objectStruct {") {
                s.fs
            } else {
                s.vs.as_str()
            };
            let mut grid = grid_uniforms();
            grid.3 = gi;
            let values = object_values(
                &s.object,
                &Object {
                    model: item.model,
                    seed: self.params[0] as u32,
                    roughness: kind.roughness,
                    metalness: kind.metalness,
                    grid: Some(grid),
                    custom: &kind.custom,
                },
            );
            let buffer = b
                .objects
                .get(&(i, MAIN))
                .ok_or(Error::Invalid("city object"))?;
            write(r, buffer, source, "objectStruct", &values)?;
        }
        Ok(())
    }
    fn pipeline(
        &mut self,
        r: &Renderer,
        k: K,
        stage: u8,
        samples: u32,
    ) -> Result<wgpu::RenderPipeline> {
        if let Some(p) = self.pipelines.get(&(k, stage, samples)) {
            return Ok(p.clone());
        }
        let s = self.stage(k, stage)?;
        let p = match stage {
            SHADOW => material_pipeline(r, s, BYTE, 1, true),
            _ => material_pipeline(r, s, HALF, samples, false),
        };
        self.pipelines.insert((k, stage, samples), p.clone());
        Ok(p)
    }
    /// One item's draw for a material stage and camera.
    #[allow(clippy::too_many_arguments)]
    fn item_draw(
        &mut self,
        r: &Renderer,
        item: usize,
        object_key: (usize, u8),
        k: K,
        stage: u8,
        camera: u8,
        samples: u32,
        bounce: bool,
    ) -> Result<Draw> {
        let pipeline = self.pipeline(r, k, stage, samples)?;
        let render = self.render_buffer(r, k, camera)?;
        let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
        let instances = b
            .items
            .get(item)
            .and_then(|i| i.instances)
            .or(if item == usize::MAX {
                Some(b.proxy.1)
            } else {
                None
            });
        let object = b
            .objects
            .get(&object_key)
            .ok_or(Error::Invalid("city object"))?;
        let res = Resources {
            linear: &self.linear,
            compare: &self.compare,
            dfg: &r.dfg,
            depth: &self.shadow_depth,
            atlas: if bounce {
                &self.bounce_view
            } else {
                &self.atlas_view
            },
            env: &self.pmrem.view,
        };
        material_draw(
            r,
            &pipeline,
            self.stage(k, stage)?,
            &render,
            object,
            instances.map(|i| &b.instances[i]),
            b.pillars.get(&k),
            &res,
        )
    }
    /// The shadow pass's and the probe captures' draws.
    fn build_draws(&mut self, r: &Renderer) -> Result<()> {
        let items: Vec<(usize, K, bool)> = self
            .built
            .as_ref()
            .map(|b| {
                b.items
                    .iter()
                    .enumerate()
                    .map(|(i, it)| (i, it.kind, it.cast))
                    .collect()
            })
            .unwrap_or_default();
        let mut shadow = vec![];
        for (i, k, cast) in items {
            if cast {
                shadow.push((
                    i,
                    self.item_draw(r, i, (i, SHADOW), k, SHADOW, SHADOW, 1, false)?,
                ));
            }
        }
        // The bake's proxy shadow is the last draw ( usize::MAX ).
        shadow.push((
            usize::MAX,
            self.item_draw(
                r,
                usize::MAX,
                (usize::MAX - 1, SHADOW),
                K::ProxyBake,
                SHADOW,
                SHADOW,
                1,
                false,
            )?,
        ));
        let sky_pipeline = pipeline(
            r,
            "sky probe face",
            (SKY_VS, SKY_FS),
            &layouts(
                &[wgpu::vertex_attr_array![0 => Float32x3].to_vec()],
                &[(12, false)],
            ),
            HALF,
            1,
            (wgpu::CompareFunction::LessEqual, false),
            true,
        );
        let mut bake: [Vec<[Draw; 3]>; 2] = [vec![], vec![]];
        let (sky_renders, sky_object) =
            (self.sky_face_renders.clone(), self.sky_bake_object.clone());
        for f in 0..6u8 {
            let sky = || {
                two_groups(
                    r,
                    sky_pipeline.clone(),
                    &sky_renders[f as usize],
                    &[(0, sky_object.as_entire_binding())],
                )
            };
            let pass0 = [
                sky(),
                self.item_draw(
                    r,
                    0,
                    (usize::MAX - 3, MAIN),
                    K::GroundBake,
                    MAIN,
                    FACE + f,
                    1,
                    false,
                )?,
                self.item_draw(
                    r,
                    usize::MAX,
                    (usize::MAX, MAIN),
                    K::ProxyBake,
                    MAIN,
                    FACE + f,
                    1,
                    false,
                )?,
            ];
            let pass1 = [
                sky(),
                self.item_draw(
                    r,
                    0,
                    (usize::MAX - 4, MAIN),
                    K::Ground,
                    MAIN,
                    FACE + f,
                    1,
                    true,
                )?,
                self.item_draw(
                    r,
                    usize::MAX,
                    (usize::MAX - 2, MAIN),
                    K::ProxyGi,
                    MAIN,
                    FACE + f,
                    1,
                    true,
                )?,
            ];
            bake[0].push(pass0);
            bake[1].push(pass1);
        }
        let b = self.built.as_mut().ok_or(Error::Invalid("city scene"))?;
        b.shadow = PassDraws { draws: shadow };
        b.bake = bake;
        Ok(())
    }
}
/// One frame's camera.
fn render_for(view: &View, sun: &Sun, time: f64) -> Render {
    Render {
        projection: view.projection,
        view: view.view,
        world: view.world,
        light_color: sun.color,
        light_position: sun.position,
        shadow_matrix: sun.matrix,
        normal_bias: NORMAL_BIAS,
        map_size: MAP as f64,
        time,
    }
}
fn color_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    label: &str,
    view: &'a wgpu::TextureView,
    resolve: Option<&'a wgpu::TextureView>,
    depth: Option<&'a wgpu::TextureView>,
    load: bool,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: resolve,
            ops: wgpu::Operations {
                load: if load {
                    wgpu::LoadOp::Load
                } else {
                    wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                },
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: depth.map(|view| wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    })
}
impl Demo {
    fn sun(&self) -> Sun {
        sun(self.params[1], &self.layout)
    }
    /// The bloom chain and the output for a size.
    fn bloom(&self, r: &Renderer, size: (u32, u32), scene: &wgpu::TextureView) -> Result<Bloom> {
        let uv = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];
        let quad = |label, vs, fs| {
            culled_pipeline(
                r,
                label,
                (vs, fs),
                &uv,
                &[HALF],
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                Some(wgpu::Face::Back),
            )
        };
        let target = |s: (u32, u32)| {
            view2d(&texture(
                r,
                (s.0, s.1, 1),
                wgpu::TextureDimension::D2,
                HALF,
                1,
                false,
            ))
        };
        // setResolutionScale( 0.25 ): Math.round( size × 0.25 ), then halved
        // and rounded per level.
        let quarter = (
            ((size.0 as f64 * 0.25 + 0.5).floor() as u32).max(1),
            ((size.1 as f64 * 0.25 + 0.5).floor() as u32).max(1),
        );
        let round = |(w, h): (u32, u32)| {
            (
                ((w as f64 / 2. + 0.5).floor() as u32).max(1),
                ((h as f64 / 2. + 0.5).floor() as u32).max(1),
            )
        };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let bright = target(quarter);
        let high = quad("bloom high pass", HIGH_VS, HIGH_FS);
        let high_group = bind(
            r,
            high.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(scene)),
                (2, self.high_object.as_entire_binding()),
            ],
        );
        let mut levels = vec![];
        let mut level_size = quarter;
        for _ in 0..LEVELS {
            levels.push((target(level_size), target(level_size), level_size));
            level_size = round(level_size);
        }
        let mut blurs = vec![];
        for (i, fs) in BLUR_FS.iter().enumerate() {
            let p = quad("bloom blur", BLUR_VS, fs);
            let input = if i == 0 { &bright } else { &levels[i - 1].1 };
            let pass = |input: &wgpu::TextureView| -> Result<_> {
                let object = uniform(r, "bloom blur", fs, "objectStruct")?;
                let group = bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(input)),
                        (2, object.as_entire_binding()),
                    ],
                );
                Ok((p.clone(), group, object))
            };
            blurs.push([pass(input)?, pass(&levels[i].0)?]);
        }
        let composite = quad("bloom composite", QUAD_VS, COMPOSITE_FS);
        let mut entries = vec![
            (0, self.composite_object.as_entire_binding()),
            (1, self.tints.as_entire_binding()),
        ];
        for (i, (_, vertical, _)) in levels.iter().enumerate() {
            entries.push((2 + i as u32 * 2, sampler(&self.linear)));
            entries.push((3 + i as u32 * 2, tex(vertical)));
        }
        let composite_group = bind(r, composite.get_bind_group_layout(0), &entries);
        Ok(Bloom {
            bright,
            levels,
            high: (high, high_group),
            blurs,
            composite: (composite, composite_group),
        })
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let d2 = wgpu::TextureDimension::D2;
        let color = view2d(&texture(r, (size.0, size.1, 1), d2, HALF, samples, false));
        let resolve =
            (samples > 1).then(|| view2d(&texture(r, (size.0, size.1, 1), d2, HALF, 1, false)));
        let depth = view2d(&texture(r, (size.0, size.1, 1), d2, DEPTH, samples, false));
        let items: Vec<(usize, K)> = self
            .built
            .as_ref()
            .map(|b| {
                b.items
                    .iter()
                    .enumerate()
                    .map(|(i, it)| (i, it.kind))
                    .collect()
            })
            .unwrap_or_default();
        let mut draws = vec![];
        for (i, k) in items {
            draws.push((
                i,
                self.item_draw(r, i, (i, MAIN), k, MAIN, MAIN, samples, false)?,
            ));
        }
        let sky = two_groups(
            r,
            pipeline(
                r,
                "sky",
                (SKY_VS, SKY_FS),
                &layouts(
                    &[wgpu::vertex_attr_array![0 => Float32x3].to_vec()],
                    &[(12, false)],
                ),
                HALF,
                samples,
                (wgpu::CompareFunction::LessEqual, false),
                true,
            ),
            &self.sky_render,
            &[(0, self.sky_object.as_entire_binding())],
        );
        let scene = resolve.as_ref().unwrap_or(&color).clone();
        let bloom = self.bloom(r, size, &scene)?;
        let uv = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];
        let output_pipeline = culled_pipeline(
            r,
            "city output",
            (QUAD_VS, OUTPUT_FS),
            &uv,
            &[out.options.format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
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
                        (1, wgpu::BindingResource::TextureView(&scene)),
                        (2, wgpu::BindingResource::Sampler(&self.linear)),
                        (3, wgpu::BindingResource::TextureView(&bloom.levels[0].0)),
                    ],
                ),
            ],
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve,
            depth,
            main: PassDraws { draws },
            sky,
            bloom,
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
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// updateSun()'s bake: the sky without its disc into the PMREM.
    fn bake_pmrem(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        time: f64,
    ) -> Result<()> {
        let sun = self.sun();
        write_static(
            r,
            &self.pmrem_object,
            SKY_FS,
            "objectStruct",
            &sky_values(sun.direction, false),
        )?;
        for (render, view) in self.pmrem_renders.iter().zip(FACE_VIEWS) {
            write_static(
                r,
                render,
                SKY_FS,
                "renderStruct",
                &[
                    ("nodeUniform7", vec![time]),
                    ("cameraProjectionMatrix", FACE_PROJECTION.to_vec()),
                    ("cameraViewMatrix", view.to_vec()),
                    ("cameraPosition", vec![0.; 3]),
                ],
            )?;
        }
        let pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("PMREM fromScene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.pmrem.view,
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
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.pmrem_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: if clear {
                                wgpu::LoadOp::Clear(1.)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        drop(pass(encoder, true));
        {
            let mut p = pass(encoder, false);
            p.set_viewport(0., 0., 768., 1024., 0., 1.);
            set(&mut p, &self.box_draw);
            self.sky_box.draw(&mut p, 1);
        }
        for (i, draw) in self.pmrem_draws.iter().enumerate() {
            let mut p = pass(encoder, false);
            p.set_viewport(
                (i % 3) as f32 * 256.,
                (i / 3) as f32 * 256.,
                256.,
                256.,
                0.,
                1.,
            );
            set(&mut p, draw);
            self.sky_box.draw(&mut p, 1);
        }
        self.pmrem.encode_levels(encoder);
        Ok(())
    }
    /// The shadow map: the given draws ( front to back by the shadow
    /// camera ).
    fn encode_shadow(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        sun: &Sun,
        proxy: bool,
    ) -> Result<()> {
        let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
        let mut pass = color_pass(
            encoder,
            "city shadow",
            &self.shadow_color,
            None,
            Some(&self.shadow_depth),
            false,
        );
        let screen = multiply(&sun.shadow.projection, &sun.shadow.view);
        let planes = frustum(&screen);
        let mut order = vec![];
        for (i, draw) in &b.shadow.draws {
            let is_proxy = *i == usize::MAX;
            if is_proxy != proxy {
                continue;
            }
            let (z, sphere) = self.item_sphere(b, *i, &screen);
            if visible(&planes, sphere) {
                order.push((z, *i, draw));
            }
        }
        order.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        for (_, i, draw) in order {
            set(&mut pass, draw);
            let (k, mesh, instances) = self.item_parts(b, i);
            draw_mesh(
                &mut pass,
                self.stage(k, SHADOW)?,
                &b.meshes[mesh],
                instances.map(|n| &b.instances[n]),
            );
        }
        Ok(())
    }
    /// An item's material, mesh and instances ( usize::MAX: the proxy ).
    fn item_parts(&self, b: &Built, i: usize) -> (K, usize, Option<usize>) {
        if i == usize::MAX {
            (K::ProxyBake, b.proxy.0, Some(b.proxy.1))
        } else {
            let it = &b.items[i];
            (it.kind, it.mesh, it.instances)
        }
    }
    /// The sort depth ( the geometry's centre in clip space ) and the world
    /// bounding sphere of an item.
    fn item_sphere(&self, b: &Built, i: usize, screen: &M4) -> (f64, (V3, f64)) {
        let (_, mesh, instances) = self.item_parts(b, i);
        let model = if i == usize::MAX {
            IDENTITY
        } else {
            b.items[i].model
        };
        let center = b.meshes[mesh].sphere.0;
        let z = transform(&multiply(screen, &model), center)[2];
        let sphere = match instances {
            Some(n) => b.instances[n].sphere,
            None => {
                let c = transform(&model, center);
                ([c[0], c[1], c[2]], b.meshes[mesh].sphere.1)
            }
        };
        (z, sphere)
    }
}
impl Demo {
    /// updateProbes(): one row of ten probes per frame, the direct pass
    /// over the whole grid first, then the bounce pass from its snapshot.
    fn bake_probes(&mut self, r: &Renderer, sun: &Sun) -> Result<()> {
        if self.bake_index >= PROBES {
            return Ok(());
        }
        let (start, pass) = (self.bake_index, self.bake_pass as usize);
        let end = start + GRID_RES[0];
        let [nx, ny, nz] = GRID_RES;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // _updateBounceGrid: the indirect pass snapshots the direct bake.
        if pass == 1 && start == 0 {
            let size = self.atlas.size();
            encoder.copy_texture_to_texture(
                self.atlas.as_image_copy(),
                self.bounce.as_image_copy(),
                size,
            );
        }
        // The shadow map renders once for the bake, with the proxy standing
        // in for the city.
        let shadow = render_for(&sun.shadow, sun, self.time);
        self.write_render(r, K::ProxyBake, SHADOW, &shadow)?;
        self.encode_shadow(&mut encoder, sun, true)?;
        r.queue.submit([encoder.finish()]);
        let (projection, rotations) = cube_cameras();
        let (ground, proxy) = if pass == 0 {
            (K::GroundBake, K::ProxyBake)
        } else {
            (K::Ground, K::ProxyGi)
        };
        for probe in start..end {
            let (ix, iy, iz) = (probe % nx, probe / (nx * nz), (probe / nx) % nz);
            let p = probe_position([ix, iy, iz]);
            let mut encoder = r.device.create_command_encoder(&Default::default());
            for (f, rotation) in rotations.iter().enumerate() {
                let view = multiply(rotation, &translation(p.map(|v| -v)));
                let world = inverse(&view);
                let face = View {
                    projection,
                    view,
                    world,
                };
                let values = render_for(&face, sun, self.time);
                self.write_render(r, ground, FACE + f as u8, &values)?;
                self.write_render(r, proxy, FACE + f as u8, &values)?;
                write_static(
                    r,
                    &self.sky_face_renders[f],
                    SKY_FS,
                    "renderStruct",
                    &[
                        ("nodeUniform7", vec![self.time]),
                        ("cameraProjectionMatrix", projection.to_vec()),
                        ("cameraViewMatrix", view.to_vec()),
                        ("cameraPosition", p.to_vec()),
                    ],
                )?;
                let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
                let planes = frustum(&multiply(&projection, &view));
                let mut pass_ = color_pass(
                    &mut encoder,
                    "city probe face",
                    &self.cube_faces[f],
                    None,
                    Some(&self.cube_depth),
                    false,
                );
                let [sky, g, x] = &b.bake[pass][f];
                set(&mut pass_, sky);
                self.sky_box.draw(&mut pass_, 1);
                if visible(&planes, self.item_sphere(b, 0, &IDENTITY).1) {
                    set(&mut pass_, g);
                    draw_mesh(&mut pass_, self.stage(ground, MAIN)?, &b.meshes[0], None);
                }
                if visible(&planes, b.instances[b.proxy.1].sphere) {
                    set(&mut pass_, x);
                    draw_mesh(
                        &mut pass_,
                        self.stage(proxy, MAIN)?,
                        &b.meshes[b.proxy.0],
                        Some(&b.instances[b.proxy.1]),
                    );
                }
            }
            {
                // Batch rows in texture order ( X, Y, Z ).
                let row = ix + iy * nx + iz * nx * ny;
                let mut sh =
                    color_pass(&mut encoder, "city probe SH", &self.batch, None, None, true);
                sh.set_viewport(0., row as f32, 9., 1., 0., 1.);
                set(&mut sh, &self.sh);
                sh.draw(0..3, 0..1);
            }
            r.queue.submit([encoder.finish()]);
        }
        // _repackProbes: per Z slice the range's rows as rectangles, into
        // the slice and, at the ends, the padding slices.
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let per_layer = nx * nz;
        let (start_y, end_y) = (start / per_layer, end / per_layer);
        let padded = nz + 2;
        for iz in 0..nz {
            let clamp = |v: i64| v.clamp(0, nx as i64) as u32;
            let slice_start = start_y * nx + clamp((start % per_layer) as i64 - (iz * nx) as i64);
            let slice_end = end_y * nx + clamp((end % per_layer) as i64 - (iz * nx) as i64);
            let mut probe = slice_start;
            while probe < slice_end {
                let (ix, iy) = (probe % nx, probe / nx);
                let width = (nx - ix).min(slice_end - probe);
                let height = if width == nx {
                    (ny - iy).min((slice_end - probe) / nx)
                } else {
                    1
                };
                for (t, (pipeline, groups)) in self.repack.iter().enumerate() {
                    let base = t as u32 * padded;
                    let mut slices = vec![base + 1 + iz];
                    if iz == 0 {
                        slices.push(base);
                    }
                    if iz == nz - 1 {
                        slices.push(base + 1 + nz);
                    }
                    for slice in slices {
                        let mut pass_ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("city repack"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &self.atlas_view,
                                depth_slice: Some(slice),
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            ..Default::default()
                        });
                        pass_.set_viewport(
                            ix as f32,
                            iy as f32,
                            width as f32,
                            height as f32,
                            0.,
                            1.,
                        );
                        pass_.set_pipeline(pipeline);
                        pass_.set_bind_group(0, &groups[iz as usize], &[]);
                        pass_.draw(0..3, 0..1);
                    }
                }
                probe += width * height;
            }
        }
        r.queue.submit([encoder.finish()]);
        self.bake_index = end;
        if self.bake_index == PROBES && self.bake_pass == 0 {
            self.bake_index = 0;
            self.bake_pass = 1;
        }
        Ok(())
    }
    fn write_render(&mut self, r: &Renderer, k: K, camera: u8, values: &Render) -> Result<()> {
        let buffer = self.render_buffer(r, k, camera)?;
        let s = self.stage(k, if camera == SHADOW { SHADOW } else { MAIN })?;
        let source = if s.fs.contains("struct renderStruct {") {
            s.fs
        } else {
            s.vs.as_str()
        };
        write(
            r,
            &buffer,
            source,
            "renderStruct",
            &render_values(&s.render, values),
        )
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = color_pass(encoder, "city output", &t.screen.view, None, None, false);
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.quad.slice(..));
        pass.draw(0..3, 0..1);
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let mut resized = false;
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
            resized = true;
        }
        let advance = std::mem::take(&mut self.pending);
        if !advance && !resized {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            let t = self
                .targets
                .as_ref()
                .ok_or(Error::Invalid("city targets"))?;
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::take(&mut self.rebuild) {
            self.build(r)?;
            self.resize(r, out)?;
        }
        if std::mem::take(&mut self.gi_pending) {
            self.write_gi(r)?;
        }
        let sun = self.sun();
        if let Some(time) = self.pmrem_pending.take() {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            write_static(
                r,
                &self.sky_bake_object,
                SKY_FS,
                "objectStruct",
                &sky_values(sun.direction, false),
            )?;
            write_static(
                r,
                &self.sky_object,
                SKY_FS,
                "objectStruct",
                &sky_values(sun.direction, true),
            )?;
            self.bake_pmrem(r, &mut encoder, time)?;
            r.queue.submit([encoder.finish()]);
        }
        // animate(): the controls by the timer's delta, the probe rows, then
        // the render pipeline.
        if advance {
            // Timer.getDelta(): a clock set back gives a negative delta.
            let delta = self.time - self.last;
            self.last = self.time;
            self.walker.update(s, c, delta)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let main = View {
            projection: mat(camera.projection_matrix()?),
            view: mat(world.inverse()),
            world: mat(world),
        };
        if advance {
            self.bake_probes(r, &sun)?;
        }
        let shadow = render_for(&sun.shadow, &sun, self.time);
        let frame = render_for(&main, &sun, self.time);
        let kinds: Vec<K> = self
            .built
            .as_ref()
            .map(|b| {
                let mut k: Vec<K> = b.items.iter().map(|i| i.kind).collect();
                k.dedup();
                k
            })
            .unwrap_or_default();
        let mut shadowed = vec![];
        for &k in &kinds {
            if !shadowed.contains(&k) {
                shadowed.push(k);
                self.write_render(r, k, MAIN, &frame)?;
                if self.kind(k)?.shadow.is_some() {
                    self.write_render(r, k, SHADOW, &shadow)?;
                }
            }
        }
        write_static(
            r,
            &self.sky_render,
            SKY_FS,
            "renderStruct",
            &[
                ("nodeUniform7", vec![self.time]),
                ("cameraProjectionMatrix", main.projection.to_vec()),
                ("cameraViewMatrix", main.view.to_vec()),
                ("cameraPosition", main.world[12..15].to_vec()),
            ],
        )?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("city targets"))?;
        let identity3 = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
        write_static(
            r,
            &self.high_object,
            HIGH_FS,
            "objectStruct",
            &[("nodeUniform1", vec![0.]), ("nodeUniform2", vec![0.01])],
        )?;
        for (blur, (_, _, level)) in t.bloom.blurs.iter().zip(&t.bloom.levels) {
            for ((_, _, object), direction) in blur.iter().zip([[1., 0.], [0., 1.]]) {
                write_static(
                    r,
                    object,
                    BLUR_FS[0],
                    "objectStruct",
                    &[
                        ("nodeUniform1", identity3.clone()),
                        ("nodeUniform2", identity3.clone()),
                        ("nodeUniform3", direction.to_vec()),
                        (
                            "nodeUniform4",
                            vec![1. / level.0 as f64, 1. / level.1 as f64],
                        ),
                        ("nodeUniform5", identity3.clone()),
                    ],
                )?;
            }
        }
        // bloom( scenePassColor, 0.05, 0, 0 ).
        write_static(
            r,
            &self.composite_object,
            COMPOSITE_FS,
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.]),
                ("nodeUniform3", identity3.clone()),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform7", identity3.clone()),
                ("nodeUniform9", identity3.clone()),
                ("nodeUniform11", identity3),
                ("nodeUniform12", vec![0.05]),
            ],
        )?;
        write_static(
            r,
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[("nodeUniform2", vec![self.params[2]])],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        self.encode_shadow(&mut encoder, &sun, false)?;
        {
            let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
            let mut pass = color_pass(
                &mut encoder,
                "city scene",
                &t.color,
                t.resolve.as_ref(),
                Some(&t.depth),
                false,
            );
            let screen = multiply(&main.projection, &main.view);
            let planes = frustum(&screen);
            // The opaque list: the sky first ( it was created first and its
            // centre is the origin's ), the rest front to back.
            let sky_z = transform(&screen, [0.; 3])[2];
            let mut order: Vec<(f64, usize, Option<&Draw>)> = vec![(sky_z, 0, None)];
            for (i, draw) in &t.main.draws {
                let (z, sphere) = self.item_sphere(b, *i, &screen);
                if visible(&planes, sphere) {
                    order.push((z, i + 1, Some(draw)));
                }
            }
            order.sort_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then(a.1.cmp(&b.1))
            });
            for (_, id, draw) in order {
                match draw {
                    None => {
                        set(&mut pass, &t.sky);
                        self.sky_box.draw(&mut pass, 1);
                    }
                    Some(draw) => {
                        set(&mut pass, draw);
                        let (k, mesh, instances) = self.item_parts(b, id - 1);
                        draw_mesh(
                            &mut pass,
                            self.stage(k, MAIN)?,
                            &b.meshes[mesh],
                            instances.map(|n| &b.instances[n]),
                        );
                    }
                }
            }
        }
        let fullscreen = |pass: &mut wgpu::RenderPass| {
            pass.set_vertex_buffer(0, self.quad.slice(..));
            pass.draw(0..3, 0..1);
        };
        let bl = &t.bloom;
        {
            let mut pass = color_pass(
                &mut encoder,
                "bloom high pass",
                &bl.bright,
                None,
                None,
                false,
            );
            pass.set_pipeline(&bl.high.0);
            pass.set_bind_group(0, &bl.high.1, &[]);
            fullscreen(&mut pass);
        }
        for (blur, (horizontal, vertical, _)) in bl.blurs.iter().zip(&bl.levels) {
            for ((pipeline, group, _), target) in blur.iter().zip([horizontal, vertical]) {
                let mut pass = color_pass(&mut encoder, "bloom blur", target, None, None, false);
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                fullscreen(&mut pass);
            }
        }
        {
            let mut pass = color_pass(
                &mut encoder,
                "bloom composite",
                &bl.levels[0].0,
                None,
                None,
                false,
            );
            pass.set_pipeline(&bl.composite.0);
            pass.set_bind_group(0, &bl.composite.1, &[]);
            fullscreen(&mut pass);
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// FirstPersonControls' pointer: down ( 10 + button ), move, up ( 20 + button ).
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
    /// seed ( generateCity ), time of day ( updateSun ), exposure, global
    /// illumination and the probe helper; the first two restart the bake.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let old = *self
            .params
            .get(index)
            .ok_or(Error::Invalid("city parameter"))?;
        let value = value as f64;
        self.params[index] = value;
        match index {
            0 if old != value => self.rebuild = true,
            1 if old != value => {
                // updateSun() runs in the GUI handler: the sky renders at the
                // last frame's time.
                self.pmrem_pending = Some(self.time);
                self.bake_index = 0;
                self.bake_pass = 0;
            }
            3 if old != value => self.gi_pending = true,
            _ => {}
        }
        self.pending = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
