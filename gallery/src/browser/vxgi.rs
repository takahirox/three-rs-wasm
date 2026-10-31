//! webgpu_vxgi: the Cornell box of webgpu_postprocessing_ssgi ( red and
//! green walls, white floor, back wall, ceiling and two boxes under a white
//! emitting cylinder and a shadow-casting point light ) with VXGINode's
//! voxel cone traced ambient occlusion and indirect light, resolved by TRAA.
//! VXGIVolume collects the scene's triangles on the CPU once ( split to the
//! sub-voxel edge, as the addon does ) and voxelizes them on the GPU into a
//! 72 × 56 × 72 opacity volume with its mips; the point light is injected
//! through its cube shadow and bounced once into the radiance volume, again
//! only when the light or the injection settings change. As r187's
//! computeSolidVoxels does, the interiors of the closed, outward-facing,
//! opaque single-sided meshes ( the two boxes and the emitter ) are filled on
//! the CPU once and OR'ed into the opacity by the resolve kernel. Each frame
//! the pre-pass writes packed normals and velocity, the cone tracer writes AO
//! and GI ( its cone rotation following frameId ), the scene pass reads them
//! in the materials' lighting, TRAA jitters, reprojects and blends, and the
//! output encodes sRGB. Every stage runs the WGSL three.js r187 generates
//! for the page (in `vxgi/`; the shadow is the fog_volume modules, TRAA's
//! vertex stage the volume_traa one and the outputs the hdr and sss ones,
//! byte-identical; the compute kernels drop the subgroup built-in they
//! declare but never use).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 1024;
const LIGHT: [f64; 3] = [0., 13., 0.];
/// Color( 0xaaaaaa ) in linear space: the background clear.
const BACKGROUND: f64 = 0.4019777798219466;
/// VXGINode's TEMPORAL_CYCLE.
const TEMPORAL_CYCLE: usize = 64;
/// VXGIVolume's MAX_EDGE_SUBVOXELS and TRIANGLE_STRIDE.
const MAX_EDGE_SUBVOXELS: f64 = 16.;
const TRIANGLE_STRIDE: usize = 20;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("vxgi/", $name, ".wgsl"))
    };
}
const SHADOW_VS: &str = include_str!("fog_volume/shadow_vs.wgsl");
const SHADOW_FS: &str = include_str!("fog_volume/shadow_fs.wgsl");
const TRAA_VS: &str = include_str!("volume_traa/traa_vs.wgsl");
const TRAA_FS: &str = wgsl!("traa_fs");
const OUTPUT_VS: &str = include_str!("hdr/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("hdr/output_fs.wgsl");
const AO_FS: &str = wgsl!("ao_output_fs");
const DIRECT_FS: &str = include_str!("sss/output_scene_fs.wgsl");
/// "nodeUniform0" … "nodeUniform35", to name uniforms by number.
const NAMES: [&str; 36] = [
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
];
/// PointShadowNode's WebGPU cube faces ( r187: CubeCamera's orientation,
/// sampled at ( -x, y, z ) ): directions and ups.
const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([-1., 0., 0.], [0., 1., 0.]),
    ([1., 0., 0.], [0., 1., 0.]),
    ([0., 1., 0.], [0., 0., -1.]),
    ([0., -1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., 1., 0.]),
    ([0., 0., -1.], [0., 1., 0.]),
];
/// TAAUtils' computeHaltonOffsets( 32 ): bases 2 and 3 from index 1.
pub(super) fn halton(index: usize) -> [f64; 2] {
    let h = |mut index: usize, base: usize| {
        let (mut fraction, mut result) = (1., 0.);
        while index > 0 {
            fraction /= base as f64;
            result += fraction * (index % base) as f64;
            index /= base;
        }
        result
    };
    [h(index % 32 + 1, 2), h(index % 32 + 1, 3)]
}
/// Object3D.updateMatrix(): compose( position, Euler XYZ, scale ).
fn compose(p: [f64; 3], r: [f64; 3], s: [f64; 3]) -> Matrix4 {
    let (c1, c2, c3) = ((r[0] / 2.).cos(), (r[1] / 2.).cos(), (r[2] / 2.).cos());
    let (s1, s2, s3) = ((r[0] / 2.).sin(), (r[1] / 2.).sin(), (r[2] / 2.).sin());
    let x = s1 * c2 * c3 + c1 * s2 * s3;
    let y = c1 * s2 * c3 - s1 * c2 * s3;
    let z = c1 * c2 * s3 + s1 * s2 * c3;
    let w = c1 * c2 * c3 - s1 * s2 * s3;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    let [sx, sy, sz] = s;
    Matrix4::from_cols_array(&[
        (1. - (yy + zz)) * sx,
        (xy + wz) * sx,
        (xz - wy) * sx,
        0.,
        (xy - wz) * sy,
        (1. - (xx + zz)) * sy,
        (yz + wx) * sy,
        0.,
        (xz + wy) * sz,
        (yz - wx) * sz,
        (1. - (xx + yy)) * sz,
        0.,
        p[0],
        p[1],
        p[2],
        1.,
    ])
}
/// Vector3.applyMatrix4 in three.js's operation order.
pub(super) fn apply(m: &Matrix4, v: [f64; 3]) -> [f64; 3] {
    let e = m.to_cols_array();
    let w = 1. / (e[3] * v[0] + e[7] * v[1] + e[11] * v[2] + e[15]);
    [
        (e[0] * v[0] + e[4] * v[1] + e[8] * v[2] + e[12]) * w,
        (e[1] * v[0] + e[5] * v[1] + e[9] * v[2] + e[13]) * w,
        (e[2] * v[0] + e[6] * v[1] + e[10] * v[2] + e[14]) * w,
    ]
}
/// An indexed mesh: normals, positions, index and local bounding sphere,
/// its model, color and material ( basic for the light source ).
struct Mesh {
    normals: wgpu::Buffer,
    positions: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
    color: [f64; 3],
    basic: bool,
    caster: bool,
    /// Pre-pass, scene and direct-view objects.
    objects: [wgpu::Buffer; 3],
    shadow: wgpu::Buffer,
}
impl Mesh {
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
    /// The scene and direct-view basic materials read only the position.
    fn draw(&self, pass: &mut wgpu::RenderPass, variant: usize) {
        if self.basic && variant >= 1 {
            pass.set_vertex_buffer(0, self.positions.slice(..));
        } else {
            pass.set_vertex_buffer(0, self.normals.slice(..));
            pass.set_vertex_buffer(1, self.positions.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
}
pub(super) fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("vxgi target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
pub(super) fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
pub(super) fn color(
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
pub(super) fn depth(
    view: &wgpu::TextureView,
) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// The previous frame's matrices VelocityNode reads.
#[derive(Clone, Copy)]
struct Motion {
    projection: Matrix4,
    view: Matrix4,
}
/// TRAANode's camera matrices from its last resolve.
#[derive(Clone, Copy)]
struct Resolved {
    world: Matrix4,
    projection_inverse: Matrix4,
}
/// A compute kernel: its pipeline, bind group and dispatch size.
type Kernel = (wgpu::ComputePipeline, wgpu::BindGroup, u32);
/// The voxel volume: its grid, storage, textures and kernels.
struct Volume {
    grid: [u32; 3],
    voxel_size: f64,
    bounds_min: Vector3,
    levels: u32,
    /// Clear, voxelize, resolve and the opacity mips.
    voxelize: Vec<Kernel>,
    /// Per ping-pong index: the inject kernel ( direct light into it ), its
    /// radiance mips, and the bounce into it from the other index.
    inject: [Kernel; 2],
    radiance_mips: [Vec<Kernel>; 2],
    bounce: [Kernel; 2],
    inject_objects: [wgpu::Buffer; 2],
    bounce_objects: [wgpu::Buffer; 2],
    opacity: wgpu::TextureView,
    radiance: wgpu::TextureView,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// The pre-pass MRT ( packed normal, velocity ) and its depth.
    prepass: [(wgpu::Texture, wgpu::TextureView); 2],
    prepass_depth: (wgpu::Texture, wgpu::TextureView),
    ao: wgpu::TextureView,
    gi: wgpu::TextureView,
    scene: (wgpu::Texture, wgpu::TextureView),
    scene_depth: wgpu::TextureView,
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    /// Per mesh: the pre-pass, scene and direct-view draws.
    draws: Vec<[Draw; 3]>,
    cone: Draw,
    /// The resolve; the previous depth is the history's depth texture, which
    /// r187's TRAANode reads from the first resolve ( zero until the first
    /// depth copy ).
    traa: (wgpu::RenderPipeline, wgpu::BindGroup),
    /// The outputs: the resolve, the scene, the direct scene, AO and GI.
    outputs: [Draw; 5],
}
pub(super) struct Demo {
    controls: Controls,
    /// output, voxel view, voxel view level, cone count, cone angle, step
    /// scale, max distance, normal offset, GI intensity, bounces,
    /// directional radiance and temporal filtering.
    params: [f64; 12],
    pending: bool,
    loaded: bool,
    frame_id: usize,
    jitter: usize,
    built: bool,
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    voxelized: bool,
    /// The injection key: bounces, step scale and max distance.
    lighting: Option<[f64; 3]>,
    meshes: Vec<Mesh>,
    volume: Volume,
    shadow_pipeline: wgpu::RenderPipeline,
    shadow_faces: Vec<(wgpu::TextureView, wgpu::TextureView, wgpu::BindGroup)>,
    shadow_objects: Vec<wgpu::BindGroup>,
    shadow_cube: wgpu::TextureView,
    /// Pre-pass, scene and direct-view render uniforms for the physical
    /// and basic materials.
    physical_renders: [wgpu::Buffer; 3],
    basic_renders: [wgpu::Buffer; 3],
    cone_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The pre-pass, scene and direct-view ( no GI context ) materials. The
/// velocity's current projection is TRAANode's unjittered one.
const PHYSICAL: [(&str, &str); 3] = [
    (wgsl!("prepass_physical_vs"), wgsl!("prepass_physical_fs")),
    (wgsl!("scene_physical_vs"), wgsl!("scene_physical_fs")),
    (wgsl!("direct_physical_vs"), wgsl!("direct_physical_fs")),
];
const BASIC: [(&str, &str); 3] = [
    (wgsl!("prepass_basic_vs"), wgsl!("prepass_basic_fs")),
    (wgsl!("scene_basic_vs"), wgsl!("scene_basic_fs")),
    (wgsl!("direct_basic_vs"), wgsl!("direct_basic_fs")),
];
/// collectSceneTriangles: the world-space triangles inside the volume,
/// split along their longest edge until no edge exceeds the limit.
/// A mesh's positions, index, world matrix, color and emissiveness.
type SceneMesh = (Vec<f32>, Vec<u32>, Matrix4, [f64; 3], bool);
fn collect_triangles(
    meshes: &[SceneMesh],
    bounds: (Vector3, Vector3),
    max_edge_sq: f64,
) -> Vec<f32> {
    let mut out = vec![];
    for (positions, index, model, color, basic) in meshes {
        let (albedo, emissive) = if *basic {
            ([0.; 3], *color)
        } else {
            (*color, [0.; 3])
        };
        let p = |i: u32| {
            let i = i as usize * 3;
            apply(
                model,
                [
                    positions[i] as f64,
                    positions[i + 1] as f64,
                    positions[i + 2] as f64,
                ],
            )
        };
        for t in index.chunks(3) {
            let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
            let lo = [0, 1, 2].map(|k| a[k].min(b[k]).min(c[k]));
            let hi = [0, 1, 2].map(|k| a[k].max(b[k]).max(c[k]));
            // Box3.intersectsBox.
            if hi[0] < bounds.0.x
                || lo[0] > bounds.1.x
                || hi[1] < bounds.0.y
                || lo[1] > bounds.1.y
                || hi[2] < bounds.0.z
                || lo[2] > bounds.1.z
            {
                continue;
            }
            let d = (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>();
            if d < 1e-14 {
                continue;
            }
            let mut stack: Vec<[f64; 9]> =
                vec![[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]];
            while let Some(v) = stack.pop() {
                let [ax, ay, az, bx, by, bz, cx, cy, cz] = v;
                let ab = (bx - ax).powi(2) + (by - ay).powi(2) + (bz - az).powi(2);
                let bc = (cx - bx).powi(2) + (cy - by).powi(2) + (cz - bz).powi(2);
                let ca = (ax - cx).powi(2) + (ay - cy).powi(2) + (az - cz).powi(2);
                let longest = ab.max(bc).max(ca);
                if longest > max_edge_sq {
                    if longest == ab {
                        let m = [(ax + bx) * 0.5, (ay + by) * 0.5, (az + bz) * 0.5];
                        stack.push([ax, ay, az, m[0], m[1], m[2], cx, cy, cz]);
                        stack.push([m[0], m[1], m[2], bx, by, bz, cx, cy, cz]);
                    } else if longest == bc {
                        let m = [(bx + cx) * 0.5, (by + cy) * 0.5, (bz + cz) * 0.5];
                        stack.push([ax, ay, az, bx, by, bz, m[0], m[1], m[2]]);
                        stack.push([ax, ay, az, m[0], m[1], m[2], cx, cy, cz]);
                    } else {
                        let m = [(cx + ax) * 0.5, (cy + ay) * 0.5, (cz + az) * 0.5];
                        stack.push([ax, ay, az, bx, by, bz, m[0], m[1], m[2]]);
                        stack.push([m[0], m[1], m[2], bx, by, bz, cx, cy, cz]);
                    }
                    continue;
                }
                for x in [
                    ax,
                    ay,
                    az,
                    0.,
                    bx,
                    by,
                    bz,
                    0.,
                    cx,
                    cy,
                    cz,
                    0.,
                    albedo[0],
                    albedo[1],
                    albedo[2],
                    0.,
                    emissive[0],
                    emissive[1],
                    emissive[2],
                    0.,
                ] {
                    out.push(x as f32);
                }
            }
        }
    }
    out
}
/// VXGISceneCollector's isClosed: every edge is shared by an even number of
/// triangles, vertices with the same quantized position being one vertex.
pub(super) fn is_closed(positions: &[f32], index: &[u32]) -> bool {
    let points: Vec<[f64; 3]> = positions
        .chunks(3)
        .map(|p| [p[0] as f64, p[1] as f64, p[2] as f64])
        .collect();
    if points.is_empty() {
        return false;
    }
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in &points {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    // POSITION_QUANTIZATION; Math.round.
    let scale = [0, 1, 2].map(|k| 131072. / (hi[k] - lo[k]).max(1e-9));
    let mut keys = std::collections::HashMap::new();
    let ids: Vec<usize> = points
        .iter()
        .map(|p| {
            let key = [0, 1, 2].map(|k| ((p[k] - lo[k]) * scale[k] + 0.5).floor() as i64);
            let next = keys.len();
            *keys.entry(key).or_insert(next)
        })
        .collect();
    let mut parity = std::collections::HashMap::new();
    let mut open = 0i64;
    for t in index.as_chunks::<3>().0 {
        let [a, b, c] = [0, 1, 2].map(|k| ids[t[k] as usize]);
        if a == b || b == c || c == a {
            continue;
        }
        for (x, y) in [(a, b), (b, c), (c, a)] {
            let odd = parity.entry((x.min(y), x.max(y))).or_insert(false);
            *odd = !*odd;
            open += if *odd { 1 } else { -1 };
        }
    }
    open == 0
}
/// VXGISceneCollector's facesOutwards: a positive signed volume.
pub(super) fn faces_outwards(positions: &[f32], index: &[u32]) -> bool {
    let o = [0, 1, 2].map(|k| positions[k] as f64);
    let p = |i: u32| {
        let i = i as usize * 3;
        [0, 1, 2].map(|k| positions[i + k] as f64 - o[k])
    };
    let volume: f64 = index
        .as_chunks::<3>()
        .0
        .iter()
        .map(|t| {
            let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
            let cross = [
                b[1] * c[2] - b[2] * c[1],
                b[2] * c[0] - b[0] * c[2],
                b[0] * c[1] - b[1] * c[0],
            ];
            a[0] * cross[0] + a[1] * cross[1] + a[2] * cross[2]
        })
        .sum();
    volume > 0.
}
/// computeSolidVoxels' fillInterior: marks the sub-voxels between pairs of
/// surface crossings along each sub-voxel column in x, one byte of 2 × 2 × 2
/// sub-voxel bits per voxel.
#[allow(clippy::too_many_arguments)]
pub(super) fn fill_interior(
    positions: &[f32],
    index: &[u32],
    model: &Matrix4,
    bounds_min: Vector3,
    voxel_size: f64,
    grid: [u32; 3],
    target: &mut [u8],
) {
    // The edge function at ( y, z ) with the vertices taken relative to it,
    // and the top-left rule for points on an edge.
    let edge = |a: [f64; 3], b: [f64; 3], y: f64, z: f64| {
        (a[1] - y) * (b[2] - z) - (a[2] - z) * (b[1] - y)
    };
    let inside = |w: f64, a: [f64; 3], b: [f64; 3]| {
        if w != 0. {
            return w > 0.;
        }
        let dz = b[2] - a[2];
        dz < 0. || (dz == 0. && b[1] > a[1])
    };
    let inv = 2. / voxel_size;
    let min = bounds_min.to_array();
    let (columns_y, columns_z, length) =
        (grid[1] as i64 * 2, grid[2] as i64 * 2, grid[0] as i64 * 2);
    let p = |i: u32| {
        let i = i as usize * 3;
        let w = apply(
            model,
            [
                positions[i] as f64,
                positions[i + 1] as f64,
                positions[i + 2] as f64,
            ],
        );
        [0, 1, 2].map(|k| (w[k] - min[k]) * inv)
    };
    let mut columns: std::collections::HashMap<i64, Vec<f64>> = Default::default();
    for t in index.as_chunks::<3>().0 {
        let (a, mut b, mut c) = (p(t[0]), p(t[1]), p(t[2]));
        let area = edge(a, b, c[1], c[2]);
        if area == 0. {
            continue;
        }
        if area < 0. {
            std::mem::swap(&mut b, &mut c);
        }
        let j0 = ((a[1].min(b[1]).min(c[1]) - 0.5).ceil() as i64).max(0);
        let j1 = ((a[1].max(b[1]).max(c[1]) - 0.5).floor() as i64).min(columns_y - 1);
        let k0 = ((a[2].min(b[2]).min(c[2]) - 0.5).ceil() as i64).max(0);
        let k1 = ((a[2].max(b[2]).max(c[2]) - 0.5).floor() as i64).min(columns_z - 1);
        for k in k0..=k1 {
            for j in j0..=j1 {
                let (y, z) = (j as f64 + 0.5, k as f64 + 0.5);
                let wa = edge(b, c, y, z);
                let wb = edge(c, a, y, z);
                let wc = edge(a, b, y, z);
                if inside(wa, b, c) && inside(wb, c, a) && inside(wc, a, b) {
                    columns
                        .entry(j + k * columns_y)
                        .or_default()
                        .push((wa * a[0] + wb * b[0] + wc * c[0]) / (wa + wb + wc));
                }
            }
        }
    }
    for (column, mut crossings) in columns {
        if crossings.len() % 2 != 0 {
            continue;
        }
        crossings.sort_by(f64::total_cmp);
        let (j, k) = (column % columns_y, column / columns_y);
        let offset = grid[0] as i64 * ((j >> 1) + grid[1] as i64 * (k >> 1));
        let bits_yz = ((j & 1) << 1) | ((k & 1) << 2);
        for pair in crossings.as_chunks::<2>().0 {
            let i0 = ((pair[0] - 0.5).ceil() as i64).max(0);
            let i1 = ((pair[1] - 0.5).ceil() as i64).min(length);
            for i in i0..i1 {
                target[(offset + (i >> 1)) as usize] |= 1 << ((i & 1) | bits_yz);
            }
        }
    }
}
/// computeSolidVoxels over the given meshes ( positions, index and world
/// matrix of each opaque, single-sided mesh ): the bytes of the solid
/// buffer, one per voxel, padded to whole u32 words.
pub(super) fn solid_voxels(
    meshes: &[(&[f32], &[u32], Matrix4)],
    bounds_min: Vector3,
    voxel_size: f64,
    grid: [u32; 3],
) -> Vec<u8> {
    let voxels = (grid[0] * grid[1] * grid[2]) as usize;
    let mut target = vec![0u8; voxels.div_ceil(4) * 4];
    for (positions, index, model) in meshes {
        if is_closed(positions, index) && faces_outwards(positions, index) {
            fill_interior(
                positions,
                index,
                model,
                bounds_min,
                voxel_size,
                grid,
                &mut target,
            );
        }
    }
    target
}
pub(super) fn compute_pipeline(r: &Renderer, label: &str, source: &str) -> wgpu::ComputePipeline {
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
impl Volume {
    #[allow(clippy::too_many_arguments)]
    fn new(
        r: &Renderer,
        triangles: &[f32],
        bounds: (Vector3, Vector3),
        shadow_cube: &wgpu::TextureView,
        solids: &[(&[f32], &[u32], Matrix4)],
    ) -> Result<Self> {
        // _voxelize: the longest axis gets 64 voxels; all axes are padded by
        // a voxel and rounded up to whole mips.
        let size = bounds.1 - bounds.0;
        let resolution = 64.;
        let voxel_size = size.x.max(size.y).max(size.z) / resolution;
        let mut levels = (f64::log2(resolution).floor() - 2.).clamp(1., 8.) as u32;
        let multiple = 2f64.powi(levels as i32 - 1);
        let axis = |s: f64| (((s / voxel_size).ceil() + 2.) / multiple).ceil() * multiple;
        let grid = [axis(size.x), axis(size.y), axis(size.z)].map(|v| v as u32);
        levels = levels.min((grid[0].max(grid[1]) as f64).log2().floor() as u32 + 1);
        let bounds_min = bounds.0 - Vector3::splat(voxel_size);
        let voxels = grid[0] * grid[1] * grid[2];
        let groups = |n: u32| n.div_ceil(64);
        let storage = |label, size: u64| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
        };
        let occupancy = storage("vxgi occupancy", voxels as u64 * 4);
        let triangle_ids = storage("vxgi triangle ids", voxels as u64 * 4);
        let solid = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vxgi solid"),
                contents: &solid_voxels(solids, bounds_min, voxel_size, grid),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let triangle_count = (triangles.len() / TRIANGLE_STRIDE) as u32;
        let triangle_buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vxgi triangles"),
                contents: bytemuck::cast_slice(triangles),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let mips = 32 - grid[0].max(grid[1]).max(grid[2]).leading_zeros();
        let volume_texture = |format| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("vxgi volume"),
                size: wgpu::Extent3d {
                    width: grid[0],
                    height: grid[1],
                    depth_or_array_layers: grid[2],
                },
                mip_level_count: mips,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
        };
        let level = |t: &wgpu::Texture, mip: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: mip,
                mip_level_count: Some(1),
                ..Default::default()
            })
        };
        let opacity_texture = volume_texture(BYTE);
        // Radiance index 0 ( the traced texture ) and 1 ( the ping-pong ).
        let radiance_textures = [volume_texture(HALF), volume_texture(HALF)];
        let direct_texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vxgi direct"),
            size: wgpu::Extent3d {
                width: grid[0],
                height: grid[1],
                depth_or_array_layers: grid[2],
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: HALF,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let direct = view(&direct_texture);
        let write = |source: &str, values: &[(&str, Vec<f64>)]| -> Result<wgpu::Buffer> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            Ok(r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vxgi kernel"),
                    contents: &pack(source, "objectStruct", &values)?,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }))
        };
        let v = voxels as f64;
        let min = bounds_min.to_array().to_vec();
        let res = wgpu::BindingResource::TextureView;
        let kernel = |source: &str,
                      object: &wgpu::Buffer,
                      entries: Vec<(u32, wgpu::BindingResource)>,
                      count: u32|
         -> Kernel {
            let p = compute_pipeline(r, "vxgi kernel", source);
            // The mip kernels take their uniforms at binding 2, the others at 0.
            let at = if entries.iter().any(|(b, _)| *b == 0) {
                2
            } else {
                0
            };
            let mut e = vec![(at, object.as_entire_binding())];
            e.extend(entries);
            let g = bind(r, p.get_bind_group_layout(0), &e);
            (p, g, count)
        };
        let mip_counts: Vec<u32> = (1..levels)
            .map(|m| (0..3).map(|k| (grid[k] >> m).max(1)).product())
            .collect();
        let mut voxelize = vec![];
        let clear_object = write(
            wgsl!("clear_cs"),
            &[("nodeUniform0", vec![v]), ("nodeUniform3", vec![v])],
        )?;
        voxelize.push(kernel(
            wgsl!("clear_cs"),
            &clear_object,
            vec![
                (1, occupancy.as_entire_binding()),
                (2, triangle_ids.as_entire_binding()),
            ],
            groups(voxels),
        ));
        if triangle_count > 0 {
            let object = write(
                wgsl!("voxelize_cs"),
                &[
                    ("nodeUniform0", vec![triangle_count as f64]),
                    ("nodeUniform2", min.clone()),
                    ("nodeUniform3", vec![voxel_size]),
                    ("nodeUniform4", vec![grid[0] as f64]),
                    ("nodeUniform5", vec![grid[1] as f64]),
                    ("nodeUniform8", vec![triangle_count as f64]),
                ],
            )?;
            voxelize.push(kernel(
                wgsl!("voxelize_cs"),
                &object,
                vec![
                    (1, triangle_buffer.as_entire_binding()),
                    (2, occupancy.as_entire_binding()),
                    (3, triangle_ids.as_entire_binding()),
                ],
                groups(triangle_count),
            ));
        }
        let resolve_object = write(
            wgsl!("resolve_cs"),
            &[("nodeUniform0", vec![v]), ("nodeUniform4", vec![v])],
        )?;
        let opacity_levels: Vec<wgpu::TextureView> =
            (0..levels).map(|m| level(&opacity_texture, m)).collect();
        voxelize.push(kernel(
            wgsl!("resolve_cs"),
            &resolve_object,
            vec![
                (1, occupancy.as_entire_binding()),
                (2, solid.as_entire_binding()),
                (3, res(&opacity_levels[0])),
            ],
            groups(voxels),
        ));
        let opacity_mips = [
            wgsl!("opacity_mip1_cs"),
            wgsl!("opacity_mip2_cs"),
            wgsl!("opacity_mip3_cs"),
        ];
        let radiance_mip_sources = [
            wgsl!("radiance_mip1_cs"),
            wgsl!("radiance_mip2_cs"),
            wgsl!("radiance_mip3_cs"),
        ];
        let mut objects = vec![];
        for m in 1..levels as usize {
            let count = mip_counts[m - 1];
            let source = opacity_mips[(m - 1).min(2)];
            let object = write(source, &[("nodeUniform2", vec![count as f64])])?;
            voxelize.push(kernel(
                source,
                &object,
                vec![
                    (0, res(&opacity_levels[m - 1])),
                    (1, res(&opacity_levels[m])),
                ],
                groups(count),
            ));
            objects.push(object);
        }
        // The lights: the point light at ( 0, 13, 0 ), its colour × 100,
        // distance 100 and decay 2; the other slots stay at Vector4().
        let mut lights = vec![0f32; 128];
        lights[..12].copy_from_slice(&[0., 13., 0., 1., 0., 0., 0., 100., 100., 100., 100., 2.]);
        for slot in 4..32 {
            lights[slot * 4 + 3] = 1.;
        }
        let lights = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vxgi lights"),
                contents: bytemuck::cast_slice(&lights),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let point_sampler = r.device.create_sampler(&Default::default());
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let (linear, trilinear) = (
            sampler(wgpu::FilterMode::Nearest),
            sampler(wgpu::FilterMode::Linear),
        );
        let radiance_levels: Vec<Vec<wgpu::TextureView>> = radiance_textures
            .iter()
            .map(|t| (0..levels).map(|m| level(t, m)).collect())
            .collect();
        let radiance_full: Vec<wgpu::TextureView> = radiance_textures.iter().map(view).collect();
        let inject_objects = [0, 1].map(|_| {
            write(
                wgsl!("inject_cs"),
                &[
                    ("nodeUniform0", vec![v]),
                    ("nodeUniform4", min.clone()),
                    ("nodeUniform5", vec![voxel_size]),
                    ("nodeUniform7", vec![0., 0.5, 100., 0.]),
                    ("nodeUniform11", vec![v]),
                ],
            )
        });
        let [o0, o1] = inject_objects;
        let inject_objects = [o0?, o1?];
        let inject = [0, 1].map(|i| {
            kernel(
                wgsl!("inject_cs"),
                &inject_objects[i],
                vec![
                    (1, occupancy.as_entire_binding()),
                    (2, triangle_ids.as_entire_binding()),
                    (3, triangle_buffer.as_entire_binding()),
                    (4, lights.as_entire_binding()),
                    (5, wgpu::BindingResource::Sampler(&point_sampler)),
                    (6, res(shadow_cube)),
                    (7, res(&direct)),
                    (8, res(&radiance_levels[i][0])),
                ],
                groups(voxels),
            )
        });
        let mut radiance_mips: [Vec<Kernel>; 2] = [vec![], vec![]];
        for (i, mips) in radiance_mips.iter_mut().enumerate() {
            for m in 1..levels as usize {
                let count = mip_counts[m - 1];
                let source = radiance_mip_sources[(m - 1).min(2)];
                let object = write(source, &[("nodeUniform2", vec![count as f64])])?;
                mips.push(kernel(
                    source,
                    &object,
                    vec![
                        (0, res(&radiance_levels[i][m])),
                        (1, res(&radiance_levels[i][m - 1])),
                    ],
                    groups(count),
                ));
                objects.push(object);
            }
        }
        let bounce_objects =
            [0, 1].map(|_| uniform(r, "vxgi bounce", wgsl!("bounce_cs"), "objectStruct"));
        let [b0, b1] = bounce_objects;
        let bounce_objects = [b0?, b1?];
        let opacity = view(&opacity_texture);
        // bounce[ target ] reads the other index and writes the target.
        let bounce = [0, 1].map(|target| {
            kernel(
                wgsl!("bounce_cs"),
                &bounce_objects[target],
                vec![
                    (1, occupancy.as_entire_binding()),
                    (2, triangle_ids.as_entire_binding()),
                    (3, triangle_buffer.as_entire_binding()),
                    (4, wgpu::BindingResource::Sampler(&linear)),
                    (5, res(&direct)),
                    (6, wgpu::BindingResource::Sampler(&trilinear)),
                    (7, res(&opacity)),
                    (8, wgpu::BindingResource::Sampler(&trilinear)),
                    (9, res(&radiance_full[1 - target])),
                    (10, res(&radiance_levels[target][0])),
                ],
                groups(voxels),
            )
        });
        Ok(Self {
            grid,
            voxel_size,
            bounds_min,
            levels,
            voxelize,
            inject,
            radiance_mips,
            bounce,
            inject_objects,
            bounce_objects,
            opacity,
            radiance: view(&radiance_textures[0]),
        })
    }
    fn volume_size(&self) -> Vector3 {
        Vector3::new(
            self.grid[0] as f64,
            self.grid[1] as f64,
            self.grid[2] as f64,
        ) * self.voxel_size
    }
    fn run(encoder: &mut wgpu::CommandEncoder, kernels: &[&Kernel]) {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        for (p, g, n) in kernels {
            pass.set_pipeline(p);
            pass.set_bind_group(0, g, &[]);
            pass.dispatch_workgroups(*n, 1, 1);
        }
    }
    /// _updateLighting: the direct light into the ping-pong index that ends
    /// in the traced texture, then each bounce with its mips.
    fn light(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        [bounces, step_scale, max_distance]: [f64; 3],
    ) -> Result<()> {
        let bounces = bounces.round().max(0.) as usize;
        for object in &self.bounce_objects {
            let source = wgsl!("bounce_cs");
            let v = (self.grid[0] * self.grid[1] * self.grid[2]) as f64;
            let values = [
                ("nodeUniform0", vec![v]),
                ("nodeUniform4", self.bounds_min.to_array().to_vec()),
                ("nodeUniform5", vec![self.voxel_size]),
                ("nodeUniform7", self.volume_size().to_array().to_vec()),
                ("nodeUniform8", vec![max_distance]),
                ("nodeUniform9", vec![60.]),
                ("nodeUniform10", vec![(self.levels - 1) as f64]),
                ("nodeUniform12", vec![step_scale]),
                ("nodeUniform15", vec![v]),
            ];
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(object, 0, &pack(source, "objectStruct", &values)?);
        }
        let mut index = bounces % 2;
        let mut kernels: Vec<&Kernel> = vec![&self.inject[index]];
        kernels.extend(self.radiance_mips[index].iter());
        for _ in 0..bounces {
            let target = 1 - index;
            kernels.push(&self.bounce[target]);
            kernels.extend(self.radiance_mips[target].iter());
            index = target;
        }
        Self::run(encoder, &kernels);
        let _ = &self.inject_objects;
        Ok(())
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 10., 30.);
        let mut controls = Controls::new(None, (1., 100.), PI, true);
        controls.set_target(Vector3::new(0., 7., 0.));
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
                .ok_or(Error::Invalid("vxgi attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let wall = PlaneGeometry::build(1., 1., 1, 1)?;
        let (red, green, white) = ([1., 0., 0.], [0., 1., 0.], [1.; 3]);
        let half = PI * 0.5;
        let scene: [(BufferGeometry, Matrix4, [f64; 3], bool, bool); 8] = [
            (
                wall.clone(),
                compose([-10., 7.5, 0.], [0., half, 0.], [20., 15., 1.]),
                red,
                false,
                false,
            ),
            (
                wall.clone(),
                compose([9.999, 7.5, 0.], [0., -half, 0.], [20., 15., 1.]),
                green,
                false,
                false,
            ),
            (
                wall.clone(),
                compose([0.; 3], [-half, 0., 0.], [20., 20., 1.]),
                white,
                false,
                false,
            ),
            (
                wall.clone(),
                compose([0., 7.5, -10.], [0., 0., -half], [15., 20., 1.]),
                white,
                false,
                false,
            ),
            (
                wall,
                compose([0., 14.999, 0.], [half, 0., 0.], [20., 20., 1.]),
                white,
                false,
                false,
            ),
            (
                BoxGeometry::build(5., 7., 5.)?,
                compose([-3., 3.5, -2.], [0., PI * 0.25, 0.], [1.; 3]),
                white,
                false,
                true,
            ),
            (
                BoxGeometry::build(4., 4., 4.)?,
                compose([4., 2., 4.], [0., PI * -0.1, 0.], [1.; 3]),
                white,
                false,
                true,
            ),
            (
                CylinderGeometry::build(2.5, 2.5, 1., 64, 1, false, 0., 2. * PI)?,
                compose([0., 15., 0.], [0.; 3], [1.; 3]),
                white,
                true,
                false,
            ),
        ];
        let mut meshes = vec![];
        let mut sources = vec![];
        let (mut lo, mut hi) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for (g, model, color, basic, caster) in scene {
            let positions = read(&g, "position")?;
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            // computeSceneBounds: Box3.expandByObject( mesh, true ).
            for p in &points {
                let w = Vector3::from_array(apply(&model, p.to_array()));
                lo = lo.min(w);
                hi = hi.max(w);
            }
            let (min, max) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (min + max) * 0.5;
            let index = g.index.clone().ok_or(Error::Invalid("vxgi index"))?;
            let mut shadow = vec![1f32, 0., 0., 0.];
            shadow.extend(model.to_cols_array().map(|v| v as f32));
            let sources_of = if basic { &BASIC } else { &PHYSICAL };
            let objects =
                [0, 1, 2].map(|i| uniform(r, "vxgi object", sources_of[i].1, "objectStruct"));
            let [a, b, c2] = objects;
            meshes.push(Mesh {
                normals: init(
                    "vxgi normals",
                    bytemuck::cast_slice(&read(&g, "normal")?),
                    wgpu::BufferUsages::VERTEX,
                ),
                positions: init(
                    "vxgi positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "vxgi index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
                model,
                color,
                basic,
                caster,
                objects: [a?, b?, c2?],
                shadow: init(
                    "vxgi shadow object",
                    bytemuck::cast_slice(&shadow),
                    wgpu::BufferUsages::UNIFORM,
                ),
            });
            sources.push((positions, index, model, color, basic));
        }
        // The point light's 1024² cube shadow ( near 0.5, far = distance 100 ).
        let position = [wgpu::vertex_attr_array![0 => Float32x3]];
        let shadow_pipeline = raw_pipeline(
            r,
            "vxgi shadow",
            SHADOW_VS,
            SHADOW_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position[0],
            }],
            BYTE,
            1,
            Some((wgpu::CompareFunction::LessEqual, true)),
            true,
        );
        let shadow_color = texture(r, (SHADOW, SHADOW, 6), BYTE);
        let shadow_depth = texture(r, (SHADOW, SHADOW, 6), DEPTH);
        let layer = |t: &wgpu::Texture, i: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            })
        };
        let light = Vector3::from_array(LIGHT);
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., 0.5, 100.);
        let shadow_faces = (0..6)
            .map(|i| {
                let (direction, up) = FACES[i];
                let view = Matrix4::look_at_rh(
                    light,
                    light + Vector3::from_array(direction),
                    Vector3::from_array(up),
                );
                let mut data: Vec<f32> = face_projection.to_cols_array().map(|v| v as f32).to_vec();
                data.extend(view.to_cols_array().map(|v| v as f32));
                let buffer = init(
                    "vxgi shadow camera",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::UNIFORM,
                );
                (
                    layer(&shadow_color, i as u32),
                    layer(&shadow_depth, i as u32),
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(0),
                        &[(0, buffer.as_entire_binding())],
                    ),
                )
            })
            .collect();
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
        let shadow_cube = shadow_depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        // The volume around the scene bounds; the triangles inside it.
        let probe = (hi - lo).max_element() / 64.;
        let bounds_min = lo - Vector3::splat(probe);
        let volume_box = {
            let size = hi - lo;
            let multiple = 8.;
            let axis = |s: f64| (((s / probe).ceil() + 2.) / multiple).ceil() * multiple;
            (
                bounds_min,
                bounds_min + Vector3::new(axis(size.x), axis(size.y), axis(size.z)) * probe,
            )
        };
        let triangles = collect_triangles(
            &sources,
            volume_box,
            (MAX_EDGE_SUBVOXELS * probe * 0.5).powi(2),
        );
        // computeSolidVoxels: every mesh's material is opaque and front-sided.
        let solids: Vec<(&[f32], &[u32], Matrix4)> = sources
            .iter()
            .map(|(positions, index, model, ..)| (&positions[..], &index[..], *model))
            .collect();
        let volume = Volume::new(r, &triangles, (lo, hi), &shadow_cube, &solids)?;
        let renders = |sources: &[(&str, &str); 3]| -> Result<[wgpu::Buffer; 3]> {
            let [a, b, c] =
                [0, 1, 2].map(|i| uniform(r, "vxgi render", sources[i].1, "renderStruct"));
            Ok([a?, b?, c?])
        };
        Ok(Self {
            controls,
            params: [0., 0., 0., 3., 40., 0.5, 0., 1.5, 1., 1., 0., 1.],
            pending: true,
            loaded: false,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            voxelized: false,
            lighting: None,
            meshes,
            volume,
            shadow_pipeline,
            shadow_faces,
            shadow_objects,
            shadow_cube,
            physical_renders: renders(&PHYSICAL)?,
            basic_renders: renders(&BASIC)?,
            cone_object: uniform(r, "vxgi cone", wgsl!("cone_fs"), "objectStruct")?,
            traa_object: uniform(r, "vxgi traa", TRAA_FS, "objectStruct")?,
            quad_uv: init(
                "vxgi quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height, 1);
        let make = |format| {
            let t = texture(r, size, format);
            let v = view(&t);
            (t, v)
        };
        let prepass = [make(HALF), make(HALF)];
        let prepass_depth = make(DEPTH);
        let ao = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let gi = view(&texture(r, size, HALF));
        let scene = make(HALF);
        let scene_depth = view(&texture(r, size, DEPTH));
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let lit = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        });
        let depth_state = Some((wgpu::CompareFunction::LessEqual, true));
        let triangles = (1, wgpu::PrimitiveTopology::TriangleList);
        let formats: [&[wgpu::TextureFormat]; 3] = [&[HALF, HALF], &[HALF], &[HALF]];
        let position_only = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[0],
        }];
        let pipelines = |sources: &[(&str, &str); 3], basic: bool| {
            [0, 1, 2].map(|i| {
                sampled_pipeline(
                    r,
                    "vxgi mesh",
                    sources[i],
                    if basic && i >= 1 {
                        &position_only
                    } else {
                        &lit
                    },
                    formats[i],
                    depth_state,
                    (false, false),
                    triangles,
                )
            })
        };
        let physical = pipelines(&PHYSICAL, false);
        let basic = pipelines(&BASIC, true);
        let draws = self
            .meshes
            .iter()
            .map(|m| {
                [0, 1, 2].map(|i| {
                    let (p, render) = if m.basic {
                        (&basic[i], &self.basic_renders[i])
                    } else {
                        (&physical[i], &self.physical_renders[i])
                    };
                    let object = m.objects[i].as_entire_binding();
                    let entries: Vec<(u32, wgpu::BindingResource)> = match (m.basic, i) {
                        (true, 1) => vec![
                            (0, object),
                            (1, sampler(&self.linear)),
                            (2, tex(&ao)),
                            (3, sampler(&self.linear)),
                            (4, tex(&gi)),
                        ],
                        (true, _) => vec![(0, object)],
                        (false, 1) => vec![
                            (0, object),
                            (1, sampler(&self.linear)),
                            (2, tex(&ao)),
                            (3, sampler(&self.linear)),
                            (4, tex(&r.dfg)),
                            (5, sampler(&self.compare)),
                            (6, tex(&self.shadow_cube)),
                            (7, sampler(&self.linear)),
                            (8, tex(&gi)),
                        ],
                        (false, _) => vec![
                            (0, object),
                            (1, sampler(&self.linear)),
                            (2, tex(&r.dfg)),
                            (3, sampler(&self.compare)),
                            (4, tex(&self.shadow_cube)),
                        ],
                    };
                    (
                        p.clone(),
                        vec![
                            bind(
                                r,
                                p.get_bind_group_layout(0),
                                &[(0, render.as_entire_binding())],
                            ),
                            bind(r, p.get_bind_group_layout(1), &entries),
                        ],
                    )
                })
            })
            .collect();
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen_pass = |label, vs, fs, formats: &[wgpu::TextureFormat]| {
            sampled_pipeline(
                r,
                label,
                (vs, fs),
                &quad,
                formats,
                None,
                (false, false),
                triangles,
            )
        };
        let trilinear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let cone_pipeline = screen_pass(
            "vxgi cone",
            wgsl!("cone_vs"),
            wgsl!("cone_fs"),
            &[wgpu::TextureFormat::R8Unorm, HALF],
        );
        let cone = (
            cone_pipeline.clone(),
            vec![bind(
                r,
                cone_pipeline.get_bind_group_layout(0),
                &[
                    (0, tex(&prepass_depth.1)),
                    (1, self.cone_object.as_entire_binding()),
                    (2, sampler(&self.linear)),
                    (3, tex(&prepass[0].1)),
                    (4, sampler(&trilinear)),
                    (5, tex(&self.volume.opacity)),
                    (6, sampler(&trilinear)),
                    (7, tex(&self.volume.radiance)),
                ],
            )],
        );
        let traa_pipeline = screen_pass("vxgi traa", TRAA_VS, TRAA_FS, &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_group = {
            bind(
                r,
                traa_pipeline.get_bind_group_layout(0),
                &[
                    // Velocity is read with textureLoad: the layout drops its sampler.
                    (1, tex(&prepass[1].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&scene.1)),
                    (4, tex(&prepass_depth.1)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(&history_depth_view)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        };
        let format = out.options.format;
        let single = |fs, source: &wgpu::TextureView| {
            let p = screen_pass("vxgi output", OUTPUT_VS, fs, &[format]);
            let g = bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(source)),
                ],
            );
            (p, vec![g])
        };
        let outputs = [
            single(OUTPUT_FS, &resolve.1),
            single(OUTPUT_FS, &scene.1),
            single(DIRECT_FS, &scene.1),
            single(AO_FS, &ao),
            single(wgsl!("gi_output_fs"), &gi),
        ];
        let screen = RenderTarget::with_options(
            &r.device,
            width,
            height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width,
            height,
            format,
            prepass,
            prepass_depth,
            ao,
            gi,
            scene,
            scene_depth,
            resolve,
            history,
            history_depth,
            screen,
            draws,
            cone,
            traa: (traa_pipeline, traa_group),
            outputs,
        });
        Ok(())
    }
    /// The output: 0 the resolve, 1 the scene without temporal filtering,
    /// 2 the direct scene, 3 AO and 4 GI ( the GUI lists Combined, Direct,
    /// AO and GI ).
    fn output_index(&self) -> usize {
        match self.params[0].round() as usize {
            1 => 2,
            2 => 3,
            3 => 4,
            _ if self.params[11] < 0.5 => 1,
            _ => 0,
        }
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("vxgi output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.outputs[self.output_index()]);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
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
            self.resolves = 0;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("vxgi targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::replace(&mut self.loaded, true) {
            self.frame_id += 1;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let output = self.output_index();
        let traa_on = output == 0;
        let gi_on = output != 2;
        let scene_on = output <= 2;
        let jittered = if traa_on && self.built {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        } else {
            projection
        };
        let motion = self.motion.unwrap_or(Motion { projection, view });
        self.motion = Some(Motion { projection, view });
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
        let light = Vector3::from_array(LIGHT);
        let ambient = Color::from_hex(0x0c0c0c).0.to_array().to_vec();
        let size = vec![t.width as f64, t.height as f64];
        let light_view = view.transform_point3(light).to_array().to_vec();
        let shadow_matrix = m4(Matrix4::from_translation(-light));
        // The point light ( colour × 100, distance 100, decay 2 ) and its
        // cube shadow ( near 0.5, far 100, bias 0, radius 1 ): the pre-pass
        // and the direct view share r187's physical uniform names, the scene
        // pass renumbers them. The names are the light position, colour,
        // shadow matrix, normal bias, far, near, bias, radius, map size, the
        // filter's far, near and bias, shadow intensity, distance and decay.
        let light_values = |n: [&'static str; 15]| {
            vec![
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                (n[0], light_view.clone()),
                (n[1], vec![100.; 3]),
                (n[2], shadow_matrix.clone()),
                (n[3], vec![0.]),
                (n[4], vec![100.]),
                (n[5], vec![0.5]),
                (n[6], vec![0.]),
                (n[7], vec![1.]),
                (n[8], vec![SHADOW as f64; 2]),
                (n[9], vec![100.]),
                (n[10], vec![0.5]),
                (n[11], vec![0.]),
                (n[12], vec![1.]),
                (n[13], vec![100.]),
                (n[14], vec![2.]),
            ]
        };
        let physical = [13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29];
        let scene = [15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 30, 31];
        let names = |numbers: [usize; 15]| numbers.map(|n| NAMES[n]);
        for i in [0, 2] {
            let mut values = light_values(names(physical));
            values.push(("nodeUniform30", ambient.clone()));
            if i == 0 {
                // VelocityNode: TRAANode's unjittered projection and the
                // previous frame's matrices.
                values.push(("nodeUniform31", m4(projection)));
                values.push(("nodeUniform33", m4(motion.projection)));
                values.push(("nodeUniform34", m4(motion.view)));
            }
            write(
                &self.physical_renders[i],
                PHYSICAL[i].1,
                "renderStruct",
                &values,
            )?;
            let mut basic = vec![
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform2", ambient.clone()),
            ];
            if i == 0 {
                basic.push(("nodeUniform5", m4(projection)));
                basic.push(("nodeUniform8", m4(motion.projection)));
                basic.push(("nodeUniform9", m4(motion.view)));
            }
            write(&self.basic_renders[i], BASIC[i].1, "renderStruct", &basic)?;
        }
        let mut values = light_values(names(scene));
        values.push(("nodeUniform32", ambient.clone()));
        values.push(("nodeUniform3", size.clone()));
        write(
            &self.physical_renders[1],
            PHYSICAL[1].1,
            "renderStruct",
            &values,
        )?;
        write(
            &self.basic_renders[1],
            BASIC[1].1,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform4", ambient),
                ("nodeUniform3", size.clone()),
            ],
        )?;
        for m in &self.meshes {
            let normal = m3(m.model.inverse().transpose());
            if m.basic {
                write(
                    &m.objects[0],
                    BASIC[0].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform4", normal.clone()),
                        ("nodeUniform7", m4(m.model)),
                        ("nodeUniform10", m4(m.model)),
                        ("nodeUniform12", m4(m.model)),
                    ],
                )?;
                write(
                    &m.objects[1],
                    BASIC[1].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform8", m4(m.model)),
                    ],
                )?;
                write(
                    &m.objects[2],
                    BASIC[2].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform5", m4(m.model)),
                    ],
                )?;
            } else {
                for i in [0, 2] {
                    write(
                        &m.objects[i],
                        PHYSICAL[i].1,
                        "objectStruct",
                        &[
                            ("nodeUniform0", m.color.to_vec()),
                            ("nodeUniform1", vec![1.]),
                            ("nodeUniform2", vec![0.]),
                            ("nodeUniform3", vec![1.]),
                            ("nodeUniform5", normal.clone()),
                            ("nodeUniform6", vec![1.5]),
                            ("nodeUniform7", vec![1.; 3]),
                            ("nodeUniform8", vec![1.]),
                            ("nodeUniform9", vec![0.; 3]),
                            ("nodeUniform10", vec![1.]),
                            ("nodeUniform12", m4(m.model)),
                            ("nodeUniform32", m4(m.model)),
                            ("nodeUniform35", m4(m.model)),
                        ],
                    )?;
                }
                write(
                    &m.objects[1],
                    PHYSICAL[1].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform4", vec![0.]),
                        ("nodeUniform5", vec![1.]),
                        ("nodeUniform7", normal),
                        ("nodeUniform8", vec![1.5]),
                        ("nodeUniform9", vec![1.; 3]),
                        ("nodeUniform10", vec![1.]),
                        ("nodeUniform11", vec![0.; 3]),
                        ("nodeUniform12", vec![1.]),
                        ("nodeUniform14", m4(m.model)),
                    ],
                )?;
            }
        }
        let [
            _,
            _,
            _,
            cone_count,
            cone_angle,
            step_scale,
            max_distance,
            normal_offset,
            gi_intensity,
            bounces,
            _,
            temporal,
        ] = self.params;
        let frame = if temporal > 0.5 {
            // NodeFrame.frameId: 1 on the loading frame ( the renderer's first
            // update ), then the fixture's restarted count.
            (self.frame_id.max(1) % TEMPORAL_CYCLE) as f64
        } else {
            0.
        };
        let volume = &self.volume;
        write(
            &self.cone_object,
            wgsl!("cone_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(jittered.inverse())),
                ("nodeUniform2", m4(world)),
                ("nodeUniform4", vec![frame]),
                ("nodeUniform5", vec![cone_count.round()]),
                ("nodeUniform6", vec![cone_angle]),
                ("nodeUniform7", vec![volume.voxel_size]),
                ("nodeUniform8", vec![normal_offset]),
                ("nodeUniform9", volume.bounds_min.to_array().to_vec()),
                ("nodeUniform10", volume.volume_size().to_array().to_vec()),
                ("nodeUniform11", vec![max_distance]),
                ("nodeUniform12", vec![1.]),
                ("nodeUniform13", vec![(volume.levels - 1) as f64]),
                ("nodeUniform15", vec![step_scale]),
                ("nodeUniform17", vec![gi_intensity]),
                ("nodeUniform18", vec![0.]),
                ("nodeUniform19", vec![1.]),
                ("nodeUniform20", vec![0.]),
                ("nodeUniform21", world.w_axis.truncate().to_array().to_vec()),
                ("nodeUniform22", vec![0.]),
                ("nodeUniform23", vec![1.]),
            ],
        )?;
        let previous = self.resolved.unwrap_or(Resolved {
            world: Matrix4::IDENTITY,
            projection_inverse: Matrix4::IDENTITY,
        });
        if traa_on {
            self.resolved = Some(Resolved {
                world,
                projection_inverse: jittered.inverse(),
            });
            write(
                &self.traa_object,
                TRAA_FS,
                "objectStruct",
                &[
                    ("nodeUniform3", vec![0.1, 100.]),
                    ("nodeUniform4", m4(view)),
                    ("nodeUniform5", m4(previous.world)),
                    ("nodeUniform6", m4(previous.projection_inverse)),
                    ("nodeUniform8", m3(Matrix4::IDENTITY)),
                    ("nodeUniform10", m3(Matrix4::IDENTITY)),
                    ("nodeUniform11", vec![1.]),
                ],
            )?;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The cube shadow, each face culling the casters.
        for (i, (color_view, depth_view, group)) in self.shadow_faces.iter().enumerate() {
            let (direction, up) = FACES[i];
            let face_view = Matrix4::look_at_rh(
                light,
                light + Vector3::from_array(direction),
                Vector3::from_array(up),
            );
            let frustum = Frustum::from_projection(
                Matrix4::perspective_rh(PI / 2., 1., 0.5, 100.) * face_view,
            );
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vxgi shadow"),
                color_attachments: &[color(color_view, wgpu::Color::BLACK)],
                depth_stencil_attachment: depth(depth_view),
                ..Default::default()
            });
            pass.set_pipeline(&self.shadow_pipeline);
            pass.set_bind_group(0, group, &[]);
            for (k, m) in self.meshes.iter().enumerate() {
                if m.caster && m.visible(&frustum) {
                    pass.set_bind_group(1, &self.shadow_objects[k], &[]);
                    pass.set_vertex_buffer(0, m.positions.slice(..));
                    pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..1);
                }
            }
        }
        let frustum = Frustum::from_projection(jittered * view);
        let background = wgpu::Color {
            r: BACKGROUND,
            g: BACKGROUND,
            b: BACKGROUND,
            a: 1.,
        };
        let black = wgpu::Color::BLACK;
        if gi_on {
            // The pre-pass: packed normals and velocity.
            let variant = 0;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vxgi pre-pass"),
                    color_attachments: &[
                        color(&t.prepass[0].1, background),
                        color(&t.prepass[1].1, black),
                    ],
                    depth_stencil_attachment: depth(&t.prepass_depth.1),
                    ..Default::default()
                });
                for (m, draws) in self.meshes.iter().zip(&t.draws) {
                    if m.visible(&frustum) {
                        set(&mut pass, &draws[variant]);
                        m.draw(&mut pass, variant);
                    }
                }
            }
            // VXGIVolume.update: voxelize once, then light when the
            // injection settings change.
            if !std::mem::replace(&mut self.voxelized, true) {
                let kernels: Vec<&Kernel> = self.volume.voxelize.iter().collect();
                Volume::run(&mut encoder, &kernels);
            }
            let key = [bounces.round(), step_scale, max_distance];
            if self.lighting != Some(key) {
                self.lighting = Some(key);
                self.volume.light(r, &mut encoder, key)?;
            }
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vxgi cone"),
                color_attachments: &[color(&t.ao, wgpu::Color::WHITE), color(&t.gi, black)],
                ..Default::default()
            });
            set(&mut pass, &t.cone);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        if scene_on {
            let variant = if output == 2 { 2 } else { 1 };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vxgi scene"),
                color_attachments: &[color(&t.scene.1, background)],
                depth_stencil_attachment: depth(&t.scene_depth),
                ..Default::default()
            });
            for (m, draws) in self.meshes.iter().zip(&t.draws) {
                if m.visible(&frustum) {
                    set(&mut pass, &draws[variant]);
                    m.draw(&mut pass, variant);
                }
            }
        }
        if traa_on {
            let full = wgpu::Extent3d {
                width: t.width,
                height: t.height,
                depth_or_array_layers: 1,
            };
            if self.resolves == 0 {
                encoder.copy_texture_to_texture(
                    t.scene.0.as_image_copy(),
                    t.history.as_image_copy(),
                    full,
                );
            }
            let (pipeline, group) = &t.traa;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vxgi traa"),
                    color_attachments: &[color(&t.resolve.1, black)],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
                pass.draw(0..3, 0..1);
            }
            encoder.copy_texture_to_texture(
                t.resolve.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
            encoder.copy_texture_to_texture(
                t.prepass_depth.0.as_image_copy(),
                t.history_depth.as_image_copy(),
                full,
            );
            self.resolves += 1;
            self.jitter = (self.jitter + 1) % 32;
            self.built = true;
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
    /// output, voxel view, voxel view level, cone count, cone angle, step
    /// scale, max distance, normal offset, GI intensity, bounces,
    /// directional radiance and temporal filtering. The voxel views and
    /// directional radiance are not reproduced.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            1 | 2 | 10 => return Err(Error::Invalid("vxgi parameter not reproduced")),
            // updatePostprocessing rebuilds the pipeline and the TRAA output.
            0 | 11 => self.built = false,
            _ => {}
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("vxgi parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
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

/// The program of three.js examples/webgpu_vxgi.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_vxgi(
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
                0.1,
                100.,
                Box::new(Demo::create(scene, camera, 407, renderer).await?),
            ))
        },
    )
    .await
}
