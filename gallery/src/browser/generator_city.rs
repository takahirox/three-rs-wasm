//! webgpu_generator_city: CityGenerator's 2 × 2 blocks of seeded
//! SkyscraperGenerator towers on rounded sidewalk slabs, with the street
//! furniture ( streetlights, mast-arm signals, litter baskets, benches,
//! hydrants, swaying street trees, parked and travelling cars, walking and
//! standing pedestrians ) instanced along every curb, on the wet-asphalt
//! road with its lane and crosswalk paint. The SkyMesh sky drives the sun's
//! colour, its 4096² shadow fitted around the city and the PMREM
//! environment ( r187's cube PMREM from fromScene, pmrem.rs;
//! scene.environmentIntensity 0.05 ). A 10 × 7 × 8 LightProbeGrid bakes the
//! box proxy of the towers, the ground and the sky ten probes per frame,
//! then a second pass for the bounce from a snapshot of the first: batches
//! of eight probes render their 48 faces in one ArrayCamera pass into a face
//! atlas, whose texels project to SH by their solid angles. Every lit
//! material samples the grid. The scene renders with 4× MSAA, then
//! quarter-resolution BloomNode ( 0.05 ) and ACES ( exposure 0.13 ), under
//! FirstPersonControls. The generators run on the CPU when the seed changes,
//! as the page's do; the towers are one draw each and the furniture one
//! instanced draw per generator. Every stage runs the WGSL three.js r187
//! generates for the page ( `generator_city/`; the sky vertex stage, the
//! PMREM, bloom and output modules are the custom_fog, pmrem and water ones,
//! byte-identical ).
mod city;
mod furniture;
pub(super) mod geo;
mod prims;
pub(super) mod uniforms;
use super::custom_fog::{Buffers, layouts, pipeline, two_groups, upload};
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::pack;
use super::pmrem::{Pmrem, face_cameras};
use super::pmrem_cube_uv::{bind, raw_pipeline};
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
/// LightProbeGrid's probes per capture pass ( six sub-cameras each ) and
/// row blocks per face.
const PROBES_PER_BATCH: u32 = 8;
const FACE_BLOCKS: u32 = 4;
const CAMERAS: usize = PROBES_PER_BATCH as usize * 6;
/// The capture cameras' forward and up vectors per face.
const FACE_FORWARD: [V3; 6] = [
    [1., 0., 0.],
    [-1., 0., 0.],
    [0., 1., 0.],
    [0., -1., 0.],
    [0., 0., 1.],
    [0., 0., -1.],
];
const FACE_UP: [V3; 6] = [
    [0., 1., 0.],
    [0., 1., 0.],
    [0., 0., -1.],
    [0., 0., 1.],
    [0., 1., 0.],
    [0., 1., 0.],
];
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
const SKY_VS: &str = include_str!("custom_fog/sky_vs.wgsl");
const SKY_FS: &str = wgsl!("sky_fs");
const PROBE_SKY: (&str, &str) = (wgsl!("probe_sky_vs"), wgsl!("probe_sky_fs"));
const PROBE_GROUND: (&str, &str) = (wgsl!("probe_ground_vs"), wgsl!("probe_ground_fs"));
const PROBE_PROXY: (&str, &str) = (wgsl!("probe_proxy_vs"), wgsl!("probe_proxy_fs"));
const QUAD_VS: &str = wgsl!("quad_vs");
const SH_FS: &str = wgsl!("sh_fs");
/// The repack materials in texture order.
const REPACK_FS: [&str; 7] = [
    wgsl!("repack0_fs"),
    wgsl!("repack1_fs"),
    wgsl!("repack2_fs"),
    wgsl!("repack3_fs"),
    wgsl!("repack4_fs"),
    wgsl!("repack5_fs"),
    wgsl!("repack6_fs"),
];
const UV_VS: &str = wgsl!("uv_quad_vs");
const HIGH_FS: &str = include_str!("water/high_fs.wgsl");
const BLUR_FS: [&str; LEVELS] = [
    include_str!("water/blur0_fs.wgsl"),
    include_str!("water/blur1_fs.wgsl"),
    include_str!("water/blur2_fs.wgsl"),
    include_str!("water/blur3_fs.wgsl"),
    include_str!("water/blur4_fs.wgsl"),
];
const COMPOSITE_FS: &str = include_str!("water/composite_fs.wgsl");
const OUTPUT_FS: &str = include_str!("water/output_fs.wgsl");
/// What a binding of the object's group holds.
#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Object,
    Pillars,
    Linear,
    /// The PMREM's LinearMipmapLinearFilter.
    Trilinear,
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
/// One material stage pair: its sources, attributes, object group bindings
/// and struct layouts.
struct Shader {
    vs: &'static str,
    fs: &'static str,
    attrs: Vec<Attr>,
    slots: Vec<(u32, Slot)>,
    object: Layout<ObjectSlot>,
    render: Layout<RenderSlot>,
}
/// The instance matrix's columns ( InstanceNode's instanceMatrix, an
/// instanced attribute of four vec4 ): the column of an attribute.
fn matrix_column(name: &str) -> Option<u64> {
    name.strip_prefix("nodeAttribute")?.parse().ok()
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
            } else if ty.starts_with("vec4") {
                wgpu::VertexFormat::Float32x4
            } else {
                wgpu::VertexFormat::Float32
            };
            let name = name.trim().to_string();
            Some(Attr {
                location: location.trim().parse().ok()?,
                instance: name == "paintColor"
                    || name == "personSeed"
                    || matrix_column(&name).is_some(),
                name,
                format,
            })
        })
        .collect()
}
/// The bindings of both stages in the object struct's group ( 1, or 2
/// under an ArrayCamera ).
fn slots(vs: &str, fs: &str) -> Vec<(u32, Slot)> {
    let group = if vs.contains("@binding( 0 ) @group( 2 )") {
        2
    } else {
        1
    };
    let tag = format!(") @group( {group} )");
    let mut out: Vec<(u32, Slot)> = vec![];
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
            if !after.starts_with(&tag) {
                continue;
            }
            let decl = &after[..after.find(';').unwrap_or(after.len())];
            let slot = if decl.contains("objectStruct") {
                Slot::Object
            } else if decl.contains("var<uniform>") {
                // A uniformArray ( the car's pillars ).
                Slot::Pillars
            } else if decl.contains("sampler_comparison") {
                Slot::Compare
            } else if decl.contains(": sampler") {
                // A texture's sampler: `<name>_sampler`.
                let name = decl
                    .split("var ")
                    .nth(1)
                    .and_then(|v| v.split(" :").next())
                    .and_then(|v| v.strip_suffix("_sampler"))
                    .unwrap_or_default();
                if source.contains(&format!("var {name} : texture_cube")) {
                    Slot::Trilinear
                } else {
                    Slot::Linear
                }
            } else if decl.contains("texture_depth_2d") {
                Slot::Depth
            } else if decl.contains("texture_3d") {
                Slot::Atlas
            } else if decl.contains("texture_cube") {
                Slot::Env
            } else if decl.contains("texture_2d") {
                Slot::Dfg
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
/// A material's stages.
fn shader(vs: &'static str, fs: &'static str) -> Result<Shader> {
    let bad = |e: String| Error::Asset(format!("city uniforms: {e}"));
    Ok(Shader {
        attrs: attributes(vs),
        slots: slots(vs, fs),
        object: object_layout(vs, fs).map_err(bad)?,
        render: render_layout(vs, fs).map_err(bad)?,
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
    /// instanceMatrix: an instanced attribute, one mat4 per instance.
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
                usage: wgpu::BufferUsages::VERTEX,
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
    /// The tower proxy ( buildProxy ), drawn only by the bake.
    Proxy,
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
/// The probe grid's bounds ( max, min ) and resolution: one cell per
/// probe.
fn grid_uniforms() -> Grid {
    let max = [0, 1, 2].map(|k| GRID_CENTER[k] + GRID_SIZE[k] / 2.);
    let min = [0, 1, 2].map(|k| GRID_CENTER[k] - GRID_SIZE[k] / 2.);
    (max, min, GRID_RES.map(f64::from))
}
/// LightProbeGrid.getProbePosition( ix, iy, iz ): each probe at the centre
/// of its cell.
fn probe_position(i: [u32; 3]) -> V3 {
    [0, 1, 2].map(|k| {
        let (c, s, n) = (GRID_CENTER[k], GRID_SIZE[k], GRID_RES[k]);
        c - s / 2. + (i[k] as f64 + 0.5) * s / n as f64
    })
}
/// The capture sub-cameras' PerspectiveCamera( 90, 1, near, far ) in the
/// WebGPU coordinate system.
fn capture_projection() -> M4 {
    // updateProjectionMatrix: top = near × tan( 45° ), as f64 computes it.
    let top = CUBE_NEAR * (PI / 180. * 0.5 * 90.).tan();
    let x = 2. * CUBE_NEAR / (2. * top);
    let (c, d) = (
        -CUBE_FAR / (CUBE_FAR - CUBE_NEAR),
        -CUBE_FAR * CUBE_NEAR / (CUBE_FAR - CUBE_NEAR),
    );
    [x, 0., 0., 0., 0., x, 0., 0., 0., 0., c, -1., 0., 0., d, 0.]
}
/// A capture sub-camera at `p` looking along a face: camera.up = FACE_UP,
/// lookAt( p + FACE_FORWARD ), its world matrix ( Matrix4.lookAt's basis,
/// through the quaternion as Object3D.lookAt sets it ).
fn capture_world(p: V3, face: usize) -> M4 {
    let z = normalize(FACE_FORWARD[face].map(|v| -v));
    let x = normalize(cross(FACE_UP[face], z));
    let y = cross(z, x);
    let q = rotation_quaternion(&basis(x, y, z));
    compose(p, q, [1.; 3])
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
/// The same uniforms in the capture's sky ( its fields one later, the time
/// in the render group's nodeUniform8 ).
fn probe_sky_values(direction: V3) -> Vec<(&'static str, Vec<f64>)> {
    const NAMES: [&str; 13] = [
        "nodeUniform0",
        "nodeUniform3",
        "nodeUniform4",
        "nodeUniform5",
        "nodeUniform6",
        "nodeUniform7",
        "nodeUniform9",
        "nodeUniform10",
        "nodeUniform11",
        "nodeUniform12",
        "nodeUniform13",
        "nodeUniform14",
        "nodeUniform15",
    ];
    NAMES
        .into_iter()
        .zip(sky_values(direction, false))
        .map(|(name, (_, v))| (name, v))
        .collect()
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
}
/// A material under the bake's ArrayCamera: its pipeline, its render and
/// object structs, the camera arrays' group, one cameraIndex group per
/// sub-camera and its object group.
struct CaptureDraw {
    pipeline: wgpu::RenderPipeline,
    render: wgpu::Buffer,
    object: wgpu::Buffer,
    cameras: wgpu::BindGroup,
    indices: Vec<wgpu::BindGroup>,
    group: wgpu::BindGroup,
}
/// LightProbeGrid's capture: the face atlas ( one row of six faces per
/// batch slot ) with its depth, the ArrayCamera's arrays and the sky, ground
/// and proxy materials.
struct Capture {
    faces: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// cameraPositions, cameraViewMatrices, cameraProjectionMatrices and
    /// cameraWorldMatrices.
    arrays: [wgpu::Buffer; 4],
    sky: CaptureDraw,
    ground: CaptureDraw,
    proxy: CaptureDraw,
    /// The ground's capture material ( the proxy's is K::Proxy's ).
    ground_shader: Shader,
}
pub(super) struct Demo {
    walker: FirstPerson,
    pending: bool,
    time: f64,
    last: f64,
    /// seed, timeOfDay, exposure, gi and showProbes.
    params: [f64; 5],
    rebuild: bool,
    /// The node time updateSun() bakes the sky at.
    pmrem_pending: Option<f64>,
    bake_index: u32,
    bake_pass: u32,
    layout: city::Layout,
    kinds: HashMap<K, Kind>,
    pipelines: HashMap<(K, u8, u32), wgpu::RenderPipeline>,
    built: Option<Built>,
    /// Per material and camera, the render buffer.
    renders: HashMap<(K, u8), wgpu::Buffer>,
    sky_box: Buffers,
    sky_render: wgpu::Buffer,
    sky_object: wgpu::Buffer,
    pmrem: Pmrem,
    pmrem_depth: wgpu::TextureView,
    pmrem_draws: Vec<Draw>,
    pmrem_renders: Vec<wgpu::Buffer>,
    pmrem_object: wgpu::Buffer,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    atlas: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    bounce: wgpu::Texture,
    capture: Capture,
    batch: wgpu::TextureView,
    sh: Draw,
    sh_object: wgpu::Buffer,
    repack: Vec<(wgpu::RenderPipeline, Vec<wgpu::BindGroup>)>,
    high_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    tints: wgpu::Buffer,
    output_render: wgpu::Buffer,
    quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    trilinear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The cameras a material's render buffer serves.
const MAIN: u8 = 0;
const SHADOW: u8 = 1;
fn kind(main: Shader, shadow: Option<Shader>) -> Kind {
    Kind {
        main,
        shadow,
        roughness: 1.,
        metalness: 0.,
        custom: vec![],
    }
}
/// The page's materials ( the instanced ones read instanceMatrix as an
/// instanced attribute ).
fn kinds() -> Result<HashMap<K, Kind>> {
    let mut k = HashMap::new();
    k.insert(
        K::Tower,
        kind(
            shader(wgsl!("building_vs"), wgsl!("building_fs"))?,
            Some(shader(
                wgsl!("building_shadow_vs"),
                wgsl!("building_shadow_fs"),
            )?),
        ),
    );
    k.insert(
        K::Ground,
        kind(shader(wgsl!("ground_vs"), wgsl!("ground_fs"))?, None),
    );
    k.insert(
        K::Slab,
        kind(shader(wgsl!("slab_vs"), wgsl!("slab_fs"))?, None),
    );
    k.insert(
        K::Curb,
        kind(shader(wgsl!("curb_vs"), wgsl!("curb_fs"))?, None),
    );
    let pair =
        |vs, fs, svs, sfs| -> Result<Kind> { Ok(kind(shader(vs, fs)?, Some(shader(svs, sfs)?))) };
    // The streetlight's and the signal's shadow vertex stages are the same
    // module.
    k.insert(
        K::Light,
        pair(
            wgsl!("streetlight_vs"),
            wgsl!("streetlight_fs"),
            wgsl!("pole_shadow_vs"),
            wgsl!("streetlight_shadow_fs"),
        )?,
    );
    k.insert(
        K::Signal,
        pair(
            wgsl!("signal_vs"),
            wgsl!("signal_fs"),
            wgsl!("pole_shadow_vs"),
            wgsl!("signal_shadow_fs"),
        )?,
    );
    k.insert(
        K::Can,
        pair(
            wgsl!("trashcan_vs"),
            wgsl!("trashcan_fs"),
            wgsl!("trashcan_shadow_vs"),
            wgsl!("trashcan_shadow_fs"),
        )?,
    );
    k.insert(
        K::Bench,
        pair(
            wgsl!("bench_vs"),
            wgsl!("bench_fs"),
            wgsl!("bench_shadow_vs"),
            wgsl!("bench_shadow_fs"),
        )?,
    );
    k.insert(
        K::Hydrant,
        pair(
            wgsl!("hydrant_vs"),
            wgsl!("hydrant_fs"),
            wgsl!("hydrant_shadow_vs"),
            wgsl!("hydrant_shadow_fs"),
        )?,
    );
    k.insert(
        K::Tree,
        pair(
            wgsl!("tree_vs"),
            wgsl!("tree_fs"),
            wgsl!("tree_shadow_vs"),
            wgsl!("tree_shadow_fs"),
        )?,
    );
    // The walking and the standing people share their material.
    for key in [K::Walk, K::Stand] {
        let mut m = pair(
            wgsl!("people_vs"),
            wgsl!("people_fs"),
            wgsl!("people_shadow_vs"),
            wgsl!("people_shadow_fs"),
        )?;
        // uniform( 1.75 / height ).
        m.custom = vec![vec![1.]];
        k.insert(key, m);
    }
    // The car types share their shaders; their uniforms differ.
    for (key, spec) in [
        (K::Suv, &furniture::SUV),
        (K::Sedan, &furniture::SEDAN),
        (K::Taxi, &furniture::TAXI),
    ] {
        let mut m = pair(
            wgsl!("car_vs"),
            wgsl!("car_fs"),
            wgsl!("car_shadow_vs"),
            wgsl!("car_shadow_fs"),
        )?;
        m.custom = car_uniforms(spec).0;
        k.insert(key, m);
    }
    // The proxy: roughness 0.4, metalness 0.4, one box per tower. Its scene
    // stage is the ArrayCamera capture's.
    let mut proxy = kind(
        shader(PROBE_PROXY.0, PROBE_PROXY.1)?,
        Some(shader(wgsl!("proxy_shadow_vs"), wgsl!("proxy_shadow_fs"))?),
    );
    proxy.roughness = 0.4;
    proxy.metalness = 0.4;
    k.insert(K::Proxy, proxy);
    Ok(k)
}
/// A material's vertex buffers: one per attribute, the instance matrix's
/// four columns in one ( an instanced mat4 ).
fn vertex_buffers(attrs: &[Attr]) -> Vec<Vec<&Attr>> {
    let mut out: Vec<Vec<&Attr>> = vec![];
    for a in attrs {
        match matrix_column(&a.name) {
            Some(column) if column > 0 => {
                if let Some(matrix) = out.iter_mut().find(|b| matrix_column(&b[0].name).is_some()) {
                    matrix.push(a);
                }
            }
            _ => out.push(vec![a]),
        }
    }
    out
}
/// The layouts of [`vertex_buffers`]: the attributes and ( stride,
/// per instance ) of each.
fn vertex_layouts(attrs: &[Attr]) -> (Vec<Vec<wgpu::VertexAttribute>>, Vec<(u64, bool)>) {
    let buffers = vertex_buffers(attrs);
    let lists = buffers
        .iter()
        .map(|b| {
            b.iter()
                .map(|a| wgpu::VertexAttribute {
                    format: a.format,
                    offset: matrix_column(&a.name).map_or(0, |c| c * 16),
                    shader_location: a.location,
                })
                .collect()
        })
        .collect();
    let strides = buffers
        .iter()
        .map(|b| {
            let stride = if matrix_column(&b[0].name).is_some() {
                64
            } else {
                b[0].format.size()
            };
            (stride, b[0].instance)
        })
        .collect();
    (lists, strides)
}
/// A material pipeline: separate buffers per attribute, back faces culled.
fn material_pipeline(
    r: &Renderer,
    s: &Shader,
    format: wgpu::TextureFormat,
    samples: u32,
    cw: bool,
) -> wgpu::RenderPipeline {
    let (lists, strides) = vertex_layouts(&s.attrs);
    let buffers = layouts(&lists, &strides);
    culled_pipeline(
        r,
        "generator city",
        (s.vs, s.fs),
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
    trilinear: &'a wgpu::Sampler,
    compare: &'a wgpu::Sampler,
    dfg: &'a wgpu::TextureView,
    depth: &'a wgpu::TextureView,
    atlas: &'a wgpu::TextureView,
    env: &'a wgpu::TextureView,
}
/// The object group's entries of a material.
fn object_entries<'a>(
    s: &Shader,
    object: &'a wgpu::Buffer,
    pillars: Option<&'a wgpu::Buffer>,
    res: &Resources<'a>,
) -> Result<Vec<(u32, wgpu::BindingResource<'a>)>> {
    let mut entries = vec![];
    for &(binding, slot) in &s.slots {
        let resource = match slot {
            Slot::Object => object.as_entire_binding(),
            Slot::Pillars => pillars
                .ok_or(Error::Invalid("city pillars"))?
                .as_entire_binding(),
            Slot::Linear => wgpu::BindingResource::Sampler(res.linear),
            Slot::Trilinear => wgpu::BindingResource::Sampler(res.trilinear),
            Slot::Compare => wgpu::BindingResource::Sampler(res.compare),
            Slot::Dfg => wgpu::BindingResource::TextureView(res.dfg),
            Slot::Depth => wgpu::BindingResource::TextureView(res.depth),
            Slot::Atlas => wgpu::BindingResource::TextureView(res.atlas),
            Slot::Env => wgpu::BindingResource::TextureView(res.env),
        };
        entries.push((binding, resource));
    }
    Ok(entries)
}
fn material_draw(
    r: &Renderer,
    pipeline: &wgpu::RenderPipeline,
    s: &Shader,
    render: &wgpu::Buffer,
    object: &wgpu::Buffer,
    pillars: Option<&wgpu::Buffer>,
    res: &Resources,
) -> Result<Draw> {
    let entries = object_entries(s, object, pillars, res)?;
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
    for (slot, b) in vertex_buffers(&s.attrs).iter().enumerate() {
        let a = b[0];
        let buffer = if matrix_column(&a.name).is_some() {
            instances.map(|i| i.matrices.slice(..))
        } else if a.instance {
            instances
                .and_then(|i| i.extra.iter().find(|(n, _)| *n == a.name))
                .map(|(_, b)| b.slice(..))
        } else {
            mesh.buffers
                .iter()
                .find(|(n, _)| *n == a.name)
                .map(|(_, b)| b.slice(..))
        };
        if let Some(b) = buffer {
            pass.set_vertex_buffer(slot as u32, b);
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
        let trilinear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let d2 = wgpu::TextureDimension::D2;
        let shadow_color = view2d(&texture(r, (MAP, MAP, 1), d2, BYTE, 1, false));
        let shadow_depth = view2d(&texture(r, (MAP, MAP, 1), d2, DEPTH, 1, false));
        // PMREMGenerator.fromScene( envScene ) of the sky without its disc:
        // the CubeCamera's faces into the source target, then the levels.
        let pmrem = Pmrem::new(r, 256)?;
        let pmrem_depth = view2d(&texture(
            r,
            (pmrem.size, pmrem.size, 1),
            d2,
            DEPTH,
            1,
            false,
        ));
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
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let kinds = kinds()?;
        // The capture: the face atlas, the ArrayCamera's arrays and one
        // cameraIndex per sub-camera.
        let faces = view2d(&texture(
            r,
            (6 * CUBE, PROBES_PER_BATCH * CUBE, 1),
            d2,
            HALF,
            1,
            false,
        ));
        let capture_depth = view2d(&texture(
            r,
            (6 * CUBE, PROBES_PER_BATCH * CUBE, 1),
            d2,
            DEPTH,
            1,
            false,
        ));
        let array = |label, stride: usize| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: (CAMERAS * stride) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let arrays = [
            array("cameraPositions", 16),
            array("cameraViewMatrices", 64),
            array("cameraProjectionMatrices", 64),
            array("cameraWorldMatrices", 64),
        ];
        let projection: Vec<f32> = capture_projection().map(|v| v as f32).to_vec();
        let projections: Vec<f32> = (0..CAMERAS).flat_map(|_| projection.clone()).collect();
        r.queue
            .write_buffer(&arrays[2], 0, bytemuck::cast_slice(&projections));
        let index_source = PROBE_SKY.0;
        let indices: Vec<wgpu::Buffer> = (0..CAMERAS)
            .map(|i| -> Result<wgpu::Buffer> {
                Ok(init(
                    "cameraIndex",
                    &pack(
                        index_source,
                        "cameraIndexStruct",
                        &[("u_cameraIndex", &[i as f64])],
                    )?,
                    wgpu::BufferUsages::UNIFORM,
                ))
            })
            .collect::<Result<_>>()?;
        let ground_shader = shader(PROBE_GROUND.0, PROBE_GROUND.1)?;
        let res = Resources {
            linear: &linear,
            trilinear: &trilinear,
            compare: &compare,
            dfg: &r.dfg,
            depth: &shadow_depth,
            atlas: &bounce_view,
            env: &pmrem.view,
        };
        let structs = |(vs, fs): (&str, &str)| -> Result<(wgpu::Buffer, wgpu::Buffer)> {
            let object_source = if fs.contains("struct objectStruct {") {
                fs
            } else {
                vs
            };
            Ok((
                uniform(r, "city probe render", fs, "renderStruct")?,
                uniform(r, "city probe object", object_source, "objectStruct")?,
            ))
        };
        let capture_draw = |(vs, fs): (&'static str, &'static str),
                            attrs: &[Attr],
                            (render, object): (wgpu::Buffer, wgpu::Buffer),
                            entries: &[(u32, wgpu::BindingResource)],
                            sky: bool|
         -> CaptureDraw {
            let (lists, strides) = vertex_layouts(attrs);
            let buffers = layouts(&lists, &strides);
            // The sky ( BackSide, no depth write ) and the materials.
            let p = culled_pipeline(
                r,
                "city probe capture",
                (vs, fs),
                &buffers,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, !sky)),
                (sky, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                Some(wgpu::Face::Back),
            );
            let cameras = bind(
                r,
                p.get_bind_group_layout(0),
                &camera_entries(vs, fs, &arrays, &render),
            );
            let index_groups = indices
                .iter()
                .map(|b| bind(r, p.get_bind_group_layout(1), &[(0, b.as_entire_binding())]))
                .collect();
            let group = bind(r, p.get_bind_group_layout(2), entries);
            CaptureDraw {
                pipeline: p,
                render,
                object,
                cameras,
                indices: index_groups,
                group,
            }
        };
        let buffers = structs(PROBE_SKY)?;
        let entries: Vec<(u32, wgpu::BindingResource)> = slots(PROBE_SKY.0, PROBE_SKY.1)
            .iter()
            .map(|&(b, _)| (b, buffers.1.as_entire_binding()))
            .collect();
        let sky = capture_draw(
            PROBE_SKY,
            &attributes(PROBE_SKY.0),
            buffers.clone(),
            &entries,
            true,
        );
        let buffers = structs(PROBE_GROUND)?;
        let entries = object_entries(&ground_shader, &buffers.1, None, &res)?;
        let ground = capture_draw(
            PROBE_GROUND,
            &ground_shader.attrs,
            buffers.clone(),
            &entries,
            false,
        );
        let proxy_shader = &kinds
            .get(&K::Proxy)
            .ok_or(Error::Invalid("city proxy"))?
            .main;
        let buffers = structs(PROBE_PROXY)?;
        let entries = object_entries(proxy_shader, &buffers.1, None, &res)?;
        let proxy = capture_draw(
            PROBE_PROXY,
            &proxy_shader.attrs,
            buffers.clone(),
            &entries,
            false,
        );
        // The ground's capture object struct is the seed's too.
        write(
            r,
            &ground.object,
            PROBE_GROUND.1,
            "objectStruct",
            &object_values(
                &ground_shader.object,
                &Object {
                    model: IDENTITY,
                    seed: 0,
                    roughness: 1.,
                    metalness: 0.,
                    grid: Some(grid_uniforms()),
                    max_lod: pmrem.max_lod as f64,
                    custom: &[],
                },
            ),
        )?;
        let capture = Capture {
            faces,
            depth: capture_depth,
            arrays,
            sky,
            ground,
            proxy,
            ground_shader,
        };
        // One row per probe in bake order: 9 coefficients × 6 faces × the
        // face blocks.
        let batch = view2d(&texture(
            r,
            (9 * 6 * FACE_BLOCKS, PROBES, 1),
            d2,
            FLOAT,
            1,
            false,
        ));
        let identity3 = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
        let sh_pipeline = raw_pipeline(
            r,
            "city probe SH",
            QUAD_VS,
            SH_FS,
            &[],
            FLOAT,
            1,
            None,
            false,
        );
        let sh_object = uniform(r, "city SH object", SH_FS, "objectStruct")?;
        write_static(
            r,
            &sh_object,
            SH_FS,
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform3", identity3.clone()),
            ],
        )?;
        let sh = (
            sh_pipeline.clone(),
            vec![bind(
                r,
                sh_pipeline.get_bind_group_layout(0),
                &[
                    (0, sh_object.as_entire_binding()),
                    (1, wgpu::BindingResource::TextureView(&capture.faces)),
                ],
            )],
        );
        // The seven repack materials, per Z slice with its object struct.
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
                    ("nodeUniform1", vec![iz as f64]),
                    ("nodeUniform2", GRID_RES.map(f64::from).to_vec()),
                    ("nodeUniform4", identity3.clone()),
                ],
            )?;
        }
        let repack = REPACK_FS
            .iter()
            .map(|fs| {
                let p = raw_pipeline(r, "city repack", QUAD_VS, fs, &[], HALF, 1, None, false);
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
            pmrem_pending: Some(0.),
            bake_index: 0,
            bake_pass: 0,
            layout: city::layout(),
            kinds,
            pipelines: HashMap::new(),
            built: None,
            renders: HashMap::new(),
            sky_box,
            sky_render: uniform(r, "sky", SKY_FS, "renderStruct")?,
            sky_object: uniform(r, "sky", SKY_FS, "objectStruct")?,
            pmrem,
            pmrem_depth,
            pmrem_draws,
            pmrem_renders,
            pmrem_object,
            shadow_color,
            shadow_depth,
            atlas,
            atlas_view,
            bounce,
            capture,
            batch,
            sh,
            sh_object,
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
            trilinear,
            compare,
            targets: None,
        };
        demo.build(r)?;
        Ok(demo)
    }
}
/// The camera arrays' group of a material under the ArrayCamera: each
/// group 0 uniform by its name.
fn camera_entries<'a>(
    vs: &str,
    fs: &str,
    arrays: &'a [wgpu::Buffer; 4],
    render: &'a wgpu::Buffer,
) -> Vec<(u32, wgpu::BindingResource<'a>)> {
    let mut out: Vec<(u32, wgpu::BindingResource)> = vec![];
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
            if !after.starts_with(") @group( 0 )") || out.iter().any(|(b, _)| *b == binding) {
                continue;
            }
            let decl = &after[..after.find(';').unwrap_or(after.len())];
            let Some(name) = decl
                .split("var<uniform> ")
                .nth(1)
                .and_then(|v| v.split(" :").next())
            else {
                continue;
            };
            let buffer = match name {
                "cameraPositions" => &arrays[0],
                "cameraViewMatrices" => &arrays[1],
                "cameraProjectionMatrices" => &arrays[2],
                "cameraWorldMatrices" => &arrays[3],
                _ => render,
            };
            out.push((binding, buffer.as_entire_binding()));
        }
    }
    out
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
            s.vs
        };
        let b = uniform(r, "city render", source, "renderStruct")?;
        self.renders.insert((k, camera), b.clone());
        Ok(b)
    }
    /// An item's object struct for a material stage, written once.
    fn object_buffer(&self, r: &Renderer, item: &Item, k: K, stage: u8) -> Result<wgpu::Buffer> {
        let s = self.stage(k, stage)?;
        let source = if s.fs.contains("struct objectStruct {") {
            s.fs
        } else {
            s.vs
        };
        let buffer = uniform(r, "city object", source, "objectStruct")?;
        self.write_object(r, &buffer, item.model, k, stage)?;
        Ok(buffer)
    }
    /// A material stage's object struct for a transform.
    fn write_object(
        &self,
        r: &Renderer,
        buffer: &wgpu::Buffer,
        model: M4,
        k: K,
        stage: u8,
    ) -> Result<()> {
        let kind = self.kind(k)?;
        let s = self.stage(k, stage)?;
        let source = if s.fs.contains("struct objectStruct {") {
            s.fs
        } else {
            s.vs
        };
        let values = object_values(
            &s.object,
            &Object {
                model,
                seed: self.params[0] as u32,
                roughness: kind.roughness,
                metalness: kind.metalness,
                grid: Some(grid_uniforms()),
                max_lod: self.pmrem.max_lod as f64,
                custom: &kind.custom,
            },
        );
        write(r, buffer, source, "objectStruct", &values)
    }
    /// generateCity(): the generators for the seed and their resident
    /// buffers, the shadow and probe-capture draws.
    fn build(&mut self, r: &Renderer) -> Result<()> {
        let c = city::build(self.params[0]);
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
        let mut objects = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            objects.insert((i, MAIN), self.object_buffer(r, item, item.kind, MAIN)?);
            if item.cast {
                objects.insert((i, SHADOW), self.object_buffer(r, item, item.kind, SHADOW)?);
            }
        }
        let proxy_item = Item {
            kind: K::Proxy,
            mesh: proxy.0,
            model: IDENTITY,
            instances: Some(proxy.1),
            cast: true,
        };
        // The proxy's shadow object struct ( usize::MAX ) and its capture's.
        objects.insert(
            (usize::MAX, SHADOW),
            self.object_buffer(r, &proxy_item, K::Proxy, SHADOW)?,
        );
        self.write_object(r, &self.capture.proxy.object, IDENTITY, K::Proxy, MAIN)?;
        self.built = Some(Built {
            meshes,
            instances,
            items,
            proxy,
            objects,
            pillars,
            shadow: PassDraws { draws: vec![] },
        });
        self.build_draws(r)?;
        self.targets = None;
        // resetProbeBake().
        self.bake_index = 0;
        self.bake_pass = 0;
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
    fn item_draw(
        &mut self,
        r: &Renderer,
        object_key: (usize, u8),
        k: K,
        stage: u8,
        samples: u32,
    ) -> Result<Draw> {
        let pipeline = self.pipeline(r, k, stage, samples)?;
        let render = self.render_buffer(r, k, stage)?;
        let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
        let object = b
            .objects
            .get(&object_key)
            .ok_or(Error::Invalid("city object"))?;
        let res = Resources {
            linear: &self.linear,
            trilinear: &self.trilinear,
            compare: &self.compare,
            dfg: &r.dfg,
            depth: &self.shadow_depth,
            atlas: &self.atlas_view,
            env: &self.pmrem.view,
        };
        material_draw(
            r,
            &pipeline,
            self.stage(k, stage)?,
            &render,
            object,
            b.pillars.get(&k),
            &res,
        )
    }
    /// The shadow pass's draws.
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
                shadow.push((i, self.item_draw(r, (i, SHADOW), k, SHADOW, 1)?));
            }
        }
        // The bake's proxy shadow is the last draw ( usize::MAX ).
        shadow.push((
            usize::MAX,
            self.item_draw(r, (usize::MAX, SHADOW), K::Proxy, SHADOW, 1)?,
        ));
        let b = self.built.as_mut().ok_or(Error::Invalid("city scene"))?;
        b.shadow = PassDraws { draws: shadow };
        Ok(())
    }
}
/// One frame's camera, with the probe grid's intensity.
fn render_for(view: &View, sun: &Sun, time: f64, grid: f64) -> Render {
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
        grid,
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
        let high = quad("bloom high pass", UV_VS, HIGH_FS);
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
            let p = quad("bloom blur", UV_VS, fs);
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
        let composite = quad("bloom composite", UV_VS, COMPOSITE_FS);
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
            draws.push((i, self.item_draw(r, (i, MAIN), k, MAIN, samples)?));
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
            (UV_VS, OUTPUT_FS),
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
    /// updateSun()'s bake: the sky without its disc into the PMREM
    /// ( fromScene( envScene, 0, 0.1, 100 ) ).
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
        // CubeCamera( 0.1, 100 ) at the origin; the cleared faces take the
        // renderer's clear color ( black ) as the scene's background.
        let cameras = face_cameras(0.1, 100., Vector3::ZERO);
        for (render, (projection, view, _)) in self.pmrem_renders.iter().zip(&cameras) {
            write_static(
                r,
                render,
                SKY_FS,
                "renderStruct",
                &[
                    ("nodeUniform7", vec![time]),
                    ("cameraProjectionMatrix", mat(*projection).to_vec()),
                    ("cameraViewMatrix", mat(*view).to_vec()),
                    ("cameraPosition", vec![0.; 3]),
                ],
            )?;
        }
        for (face, draw) in self.pmrem.source_faces().iter().zip(&self.pmrem_draws) {
            let mut p = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("PMREM fromScene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: face,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.pmrem_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut p, draw);
            self.sky_box.draw(&mut p, 1);
        }
        self.pmrem.encode_from_scene(encoder);
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
            (K::Proxy, b.proxy.0, Some(b.proxy.1))
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
        let (start, pass) = (self.bake_index, self.bake_pass);
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
        let shadow = render_for(&sun.shadow, sun, self.time, 1.);
        self.write_render(r, K::Proxy, SHADOW, &shadow)?;
        self.encode_shadow(&mut encoder, sun, true)?;
        r.queue.submit([encoder.finish()]);
        // The capture materials' render structs: the bounce grid adds
        // nothing to the direct pass ( intensity 0 ), as it shares its
        // lights and shaders with the bounce pass.
        let values = render_for(&sun.shadow, sun, self.time, pass as f64);
        let c = &self.capture;
        write(
            r,
            &c.ground.render,
            PROBE_GROUND.1,
            "renderStruct",
            &render_values(&c.ground_shader.render, &values),
        )?;
        write(
            r,
            &c.proxy.render,
            PROBE_PROXY.1,
            "renderStruct",
            &render_values(&self.stage(K::Proxy, MAIN)?.render, &values),
        )?;
        write_static(
            r,
            &c.sky.render,
            PROBE_SKY.1,
            "renderStruct",
            &[("nodeUniform8", vec![self.time])],
        )?;
        let b = self.built.as_ref().ok_or(Error::Invalid("city scene"))?;
        let projection = capture_projection();
        let identity3 = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
        // _captureProbes: per batch, its slots' 48 faces in one ArrayCamera
        // render into the face atlas, then their SH projection into the
        // batch rows ( one per probe, in bake order ).
        for batch_start in (start..end).step_by(PROBES_PER_BATCH as usize) {
            let count = (end - batch_start).min(PROBES_PER_BATCH);
            let mut positions = vec![0f32; CAMERAS * 4];
            let mut views = vec![0f32; CAMERAS * 16];
            let mut worlds = vec![0f32; CAMERAS * 16];
            // Unused slots' cameras draw nothing ( their layers disabled ).
            let mut cameras = vec![];
            for slot in 0..count {
                let probe = batch_start + slot;
                let (ix, iy, iz) = (probe % nx, probe / (nx * nz), (probe / nx) % nz);
                let p = probe_position([ix, iy, iz]);
                for face in 0..6 {
                    let i = slot as usize * 6 + face;
                    let world = capture_world(p, face);
                    let view = inverse(&world);
                    for k in 0..3 {
                        positions[i * 4 + k] = p[k] as f32;
                    }
                    for k in 0..16 {
                        views[i * 16 + k] = view[k] as f32;
                        worlds[i * 16 + k] = world[k] as f32;
                    }
                    cameras.push((i, frustum(&multiply(&projection, &view))));
                }
            }
            r.queue
                .write_buffer(&c.arrays[0], 0, bytemuck::cast_slice(&positions));
            r.queue
                .write_buffer(&c.arrays[1], 0, bytemuck::cast_slice(&views));
            r.queue
                .write_buffer(&c.arrays[3], 0, bytemuck::cast_slice(&worlds));
            write_static(
                r,
                &self.sh_object,
                SH_FS,
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.]),
                    ("nodeUniform1", vec![batch_start as f64]),
                    ("nodeUniform3", identity3.clone()),
                ],
            )?;
            let mut encoder = r.device.create_command_encoder(&Default::default());
            {
                let mut p = color_pass(
                    &mut encoder,
                    "city probe faces",
                    &c.faces,
                    None,
                    Some(&c.depth),
                    false,
                );
                let ground = self.item_sphere(b, 0, &IDENTITY).1;
                let proxy = b.instances[b.proxy.1].sphere;
                let proxy_shader = self.stage(K::Proxy, MAIN)?;
                for (draw, sphere) in [
                    (&c.sky, None),
                    (&c.ground, Some(ground)),
                    (&c.proxy, Some(proxy)),
                ] {
                    p.set_pipeline(&draw.pipeline);
                    p.set_bind_group(0, &draw.cameras, &[]);
                    p.set_bind_group(2, &draw.group, &[]);
                    // perCameraFrustumCulled: each sub-camera tests the
                    // object against its own frustum.
                    for (i, planes) in &cameras {
                        if sphere.is_some_and(|s| !visible(planes, s)) {
                            continue;
                        }
                        let (x, y) = ((*i % 6) as u32 * CUBE, (*i / 6) as u32 * CUBE);
                        p.set_viewport(x as f32, y as f32, CUBE as f32, CUBE as f32, 0., 1.);
                        p.set_bind_group(1, &draw.indices[*i], &[]);
                        match sphere {
                            None => self.sky_box.draw(&mut p, 1),
                            Some(_) if std::ptr::eq(draw, &c.ground) => {
                                draw_mesh(&mut p, &c.ground_shader, &b.meshes[0], None)
                            }
                            Some(_) => draw_mesh(
                                &mut p,
                                proxy_shader,
                                &b.meshes[b.proxy.0],
                                Some(&b.instances[b.proxy.1]),
                            ),
                        }
                    }
                }
            }
            {
                let mut sh =
                    color_pass(&mut encoder, "city probe SH", &self.batch, None, None, true);
                sh.set_viewport(
                    0.,
                    batch_start as f32,
                    (9 * 6 * FACE_BLOCKS) as f32,
                    count as f32,
                    0.,
                    1.,
                );
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
            s.vs
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
        let sun = self.sun();
        if let Some(time) = self.pmrem_pending.take() {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            write_static(
                r,
                &self.capture.sky.object,
                PROBE_SKY.1,
                "objectStruct",
                &probe_sky_values(sun.direction),
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
        // probes.visible: the grid's intensity in the scene's materials.
        let gi = if self.params[3] > 0.5 { 1. } else { 0. };
        let shadow = render_for(&sun.shadow, &sun, self.time, gi);
        let frame = render_for(&main, &sun, self.time, gi);
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
                        ("nodeUniform2", direction.to_vec()),
                        (
                            "nodeUniform3",
                            vec![1. / level.0 as f64, 1. / level.1 as f64],
                        ),
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
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
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
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
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

/// The program of three.js examples/webgpu_generator_city.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_generator_city(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                20000.,
                Box::new(Demo::create(scene, camera, 449, renderer).await?),
            ))
        },
    )
    .await
}
