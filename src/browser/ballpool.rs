//! webgpu_postprocessing_ssgi_ballpool: a box of balls ( the bounce rigid-body
//! world ported in crate::bounce ) under a shadow-casting point light that
//! follows the pointer, with webgpu_postprocessing_ssgi's SSGI ( 2 slices, 8
//! steps ), composite and TRAA, then ACES Filmic tone mapping ( exposure 0.5 ).
//! The walls are boxes ( white, red and green MeshPhysicalMaterial; the front
//! wall has a body but no mesh ); the balls are one InstancedMesh with the
//! page's ten instance colors. Pointer movement pushes the balls near the
//! pointer ray and moves the light to the ray's hit on the front plane, a held
//! pointer ( two touches ) respawns five balls per frame and a resize rebuilds
//! the world, the box width and the ball count from the new aspect.
//!
//! Every frame steps the world with the page's `advanceTime( 1 / 60, dt )`
//! and uploads the instance matrices, as the page's `setMatrixAt` does; the
//! scene pass reads them, and the previous frame's for the velocity, from the
//! two buffers InstanceNode builds: uniform arrays up to 1,024 balls, else
//! instanced vertex attributes. Every stage runs the WGSL three.js r186
//! generates for the page ( `ballpool/`; SSGI, composite and TRAA are the
//! ssgi and volume_traa modules, which are byte-identical ). The page's camera,
//! pointer ray and pushes follow the three.js f64 operations, so the bodies
//! match the original's bit for bit ( tests/browser/bounce-physics.spec.js ).
use super::deferred::{Draw, sampled_pipeline, set};
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::uniform;
use crate::bounce::{Quat, Vec3, World};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::collections::BTreeSet;
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 1024;
const BALL_RADIUS: f64 = 0.4;
const FILL_RATIO: f64 = 0.4;
const PACKING: f64 = 0.6;
const BOX_HEIGHT: f64 = 6.;
const BOX_DEPTH: f64 = 8.;
const WALL_THICKNESS: f64 = 0.5;
const CAM_FOV: f64 = 45.;
const EASE_SPEED: f64 = 8.;
/// InstanceNode keeps the matrices in a uniform array while it fits the
/// 64 KiB uniform buffer limit.
const UNIFORM_MATRICES: usize = 1024;
const COLORS: [u32; 10] = [
    0xff4444, 0x44ff44, 0x4488ff, 0xffaa00, 0xff44ff, 0x44ffff, 0xffff44, 0xff8844, 0x8844ff,
    0x44ff88,
];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("ballpool/", $name, ".wgsl"))
    };
}
const SSGI_VS: &str = include_str!("ssgi/ssgi_vs.wgsl");
const SSGI_FS: &str = include_str!("ssgi/ssgi_fs.wgsl");
const COMPOSITE_FS: &str = include_str!("ssgi/composite_fs.wgsl");
const QUAD_VS: &str = include_str!("deferred/forward_output_vs.wgsl");
const TRAA_VS: &str = include_str!("volume_traa/traa_vs.wgsl");
const TRAA_FS: &str = include_str!("volume_traa/traa_fs.wgsl");
/// PointShadowNode's WebGPU cube faces: directions and ups.
const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., -1., 0.]),
    ([-1., 0., 0.], [0., -1., 0.]),
    ([0., -1., 0.], [0., 0., -1.]),
    ([0., 1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., -1., 0.]),
    ([0., 0., -1.], [0., -1., 0.]),
];
/// SSGINode's per-frame slice rotations and step offsets.
const TEMPORAL_ROTATIONS: [f64; 6] = [60., 300., 180., 240., 120., 0.];
const SPATIAL_OFFSETS: [f64; 4] = [0., 0.5, 0.25, 0.75];
/// TAAUtils' computeHaltonOffsets( 32 ): bases 2 and 3 from index 1.
fn halton(index: usize) -> [f64; 2] {
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

/// The host's Math.tan and Math.exp: the camera and the easing feed the
/// pointer pushes, which must match the original's bits.
fn tan(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::tan(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.tan()
}
fn exp(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::exp(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.exp()
}

/// three.js Vector3 operations in their order of evaluation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct V3 {
    x: f64,
    y: f64,
    z: f64,
}
impl V3 {
    const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    fn sub(self, v: V3) -> V3 {
        V3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
    fn add(self, v: V3) -> V3 {
        V3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
    fn dot(self, v: V3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    fn length_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(self) -> f64 {
        self.length_sq().sqrt()
    }
    fn scale(self, s: f64) -> V3 {
        V3::new(self.x * s, self.y * s, self.z * s)
    }
    /// divideScalar( length() || 1 ): a multiplication by the inverse.
    fn normalize(self) -> V3 {
        let l = self.length();
        self.scale(1. / if l == 0. { 1. } else { l })
    }
    fn cross(a: V3, b: V3) -> V3 {
        V3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
    fn add_scaled(self, v: V3, s: f64) -> V3 {
        V3::new(self.x + v.x * s, self.y + v.y * s, self.z + v.z * s)
    }
    fn lerp(self, v: V3, alpha: f64) -> V3 {
        V3::new(
            self.x + (v.x - self.x) * alpha,
            self.y + (v.y - self.y) * alpha,
            self.z + (v.z - self.z) * alpha,
        )
    }
    fn distance_to(self, v: V3) -> f64 {
        let (dx, dy, dz) = (self.x - v.x, self.y - v.y, self.z - v.z);
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    /// applyMatrix4 with the perspective divide.
    fn apply(self, e: &[f64; 16]) -> V3 {
        let (x, y, z) = (self.x, self.y, self.z);
        let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
        V3::new(
            (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
            (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
            (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
        )
    }
    fn glam(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

/// Matrix4.compose with a unit scale.
fn compose(p: V3, (x, y, z, w): (f64, f64, f64, f64)) -> [f64; 16] {
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        1. - (yy + zz),
        xy + wz,
        xz - wy,
        0.,
        xy - wz,
        1. - (xx + zz),
        yz + wx,
        0.,
        xz + wy,
        yz - wx,
        1. - (xx + yy),
        0.,
        p.x,
        p.y,
        p.z,
        1.,
    ]
}

/// Matrix4.invert.
fn invert(te: &[f64; 16]) -> [f64; 16] {
    let [
        n11,
        n21,
        n31,
        n41,
        n12,
        n22,
        n32,
        n42,
        n13,
        n23,
        n33,
        n43,
        n14,
        n24,
        n34,
        n44,
    ] = *te;
    let t1 = n11 * n22 - n21 * n12;
    let t2 = n11 * n32 - n31 * n12;
    let t3 = n11 * n42 - n41 * n12;
    let t4 = n21 * n32 - n31 * n22;
    let t5 = n21 * n42 - n41 * n22;
    let t6 = n31 * n42 - n41 * n32;
    let t7 = n13 * n24 - n23 * n14;
    let t8 = n13 * n34 - n33 * n14;
    let t9 = n13 * n44 - n43 * n14;
    let t10 = n23 * n34 - n33 * n24;
    let t11 = n23 * n44 - n43 * n24;
    let t12 = n33 * n44 - n43 * n34;
    let det = t1 * t12 - t2 * t11 + t3 * t10 + t4 * t9 - t5 * t8 + t6 * t7;
    if det == 0. {
        return [0.; 16];
    }
    let d = 1. / det;
    [
        (n22 * t12 - n32 * t11 + n42 * t10) * d,
        (n31 * t11 - n21 * t12 - n41 * t10) * d,
        (n24 * t6 - n34 * t5 + n44 * t4) * d,
        (n33 * t5 - n23 * t6 - n43 * t4) * d,
        (n32 * t9 - n12 * t12 - n42 * t8) * d,
        (n11 * t12 - n31 * t9 + n41 * t8) * d,
        (n34 * t3 - n14 * t6 - n44 * t2) * d,
        (n13 * t6 - n33 * t3 + n43 * t2) * d,
        (n12 * t11 - n22 * t9 + n42 * t7) * d,
        (n21 * t9 - n11 * t11 - n41 * t7) * d,
        (n14 * t5 - n24 * t3 + n44 * t1) * d,
        (n23 * t3 - n13 * t5 - n43 * t1) * d,
        (n22 * t8 - n12 * t10 - n32 * t7) * d,
        (n11 * t10 - n21 * t8 + n31 * t7) * d,
        (n24 * t2 - n14 * t4 - n34 * t1) * d,
        (n13 * t4 - n23 * t2 + n33 * t1) * d,
    ]
}

/// The page's PerspectiveCamera after fitCameraToBox: its world matrix
/// ( lookAt through Quaternion.setFromRotationMatrix ) and WebGPU projection.
#[derive(Clone, Copy)]
struct CameraState {
    position: V3,
    world: [f64; 16],
    projection: [f64; 16],
    projection_inverse: [f64; 16],
}
impl CameraState {
    fn fit(aspect: f64) -> Self {
        let v_fov = (CAM_FOV / 2.) * (PI / 180.);
        let dist = (BOX_HEIGHT / 2.) / tan(v_fov);
        let position = V3::new(0., BOX_HEIGHT / 2., dist + BOX_DEPTH / 2.);
        // Object3D.lookAt for a camera: Matrix4.lookAt( eye, target, up ).
        let target = V3::new(0., BOX_HEIGHT / 2., 0.);
        let up = V3::new(0., 1., 0.);
        let mut z = position.sub(target);
        if z.length_sq() == 0. {
            z.z = 1.;
        }
        z = z.normalize();
        let x = V3::cross(up, z).normalize();
        let y = V3::cross(z, x);
        let (m11, m12, m13, m21, m22, m23, m31, m32, m33) =
            (x.x, y.x, z.x, x.y, y.y, z.y, x.z, y.z, z.z);
        // Quaternion.setFromRotationMatrix.
        let trace = m11 + m22 + m33;
        let q = if trace > 0. {
            let s = 0.5 / (trace + 1.).sqrt();
            ((m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s)
        } else if m11 > m22 && m11 > m33 {
            let s = 2. * (1. + m11 - m22 - m33).sqrt();
            (0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s)
        } else if m22 > m33 {
            let s = 2. * (1. + m22 - m11 - m33).sqrt();
            ((m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s)
        } else {
            let s = 2. * (1. + m33 - m11 - m22).sqrt();
            ((m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s)
        };
        let world = compose(position, q);
        // updateProjectionMatrix and makePerspective in WebGPU coordinates.
        let (near, far) = (0.1, 100.);
        let top = near * tan((PI / 180.) * 0.5 * CAM_FOV);
        let height = 2. * top;
        let width = aspect * height;
        let left = -0.5 * width;
        let (right, bottom) = (left + width, top - height);
        let mut projection = [0.; 16];
        projection[0] = 2. * near / (right - left);
        projection[5] = 2. * near / (top - bottom);
        projection[8] = (right + left) / (right - left);
        projection[9] = (top + bottom) / (top - bottom);
        projection[10] = -far / (far - near);
        projection[14] = (-far * near) / (far - near);
        projection[11] = -1.;
        Self {
            position,
            world,
            projection,
            projection_inverse: invert(&projection),
        }
    }
    fn view(&self) -> Matrix4 {
        Matrix4::from_cols_array(&self.world).inverse()
    }
    fn projection(&self) -> Matrix4 {
        Matrix4::from_cols_array(&self.projection)
    }
    /// Raycaster.setFromCamera: the ray through normalized device coordinates.
    fn ray(&self, x: f64, y: f64) -> (V3, V3) {
        let origin = V3::new(self.world[12], self.world[13], self.world[14]);
        let direction = V3::new(x, y, 0.5)
            .apply(&self.projection_inverse)
            .apply(&self.world)
            .sub(origin)
            .normalize();
        (origin, direction)
    }
}

/// The fixture's seeded Math.random ( an LCG from seed 186 ).
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}

/// An indexed geometry's normals, positions and index.
struct Geometry {
    normals: wgpu::Buffer,
    positions: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    /// The local bounding sphere ( center, radius ).
    sphere: (V3, f64),
}
impl Geometry {
    fn new(r: &Renderer, g: &BufferGeometry) -> Result<Self> {
        let read = |name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("ballpool attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let positions = read("position")?;
        let index = g.index.clone().ok_or(Error::Invalid("ballpool index"))?;
        // BufferGeometry.computeBoundingSphere: the box center and the
        // largest distance to it.
        let points: Vec<V3> = positions
            .chunks(3)
            .map(|p| V3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        let (lo, hi) = points.iter().fold(
            (
                V3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                V3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            ),
            |(lo, hi), p| {
                (
                    V3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
                    V3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
                )
            },
        );
        let center = lo.add(hi).scale(0.5);
        let radius = points
            .iter()
            .map(|p| center.sub(*p).length_sq())
            .fold(0., f64::max)
            .sqrt();
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        Ok(Self {
            normals: init(
                "ballpool normals",
                bytemuck::cast_slice(&read("normal")?),
                wgpu::BufferUsages::VERTEX,
            ),
            positions: init(
                "ballpool positions",
                bytemuck::cast_slice(&positions),
                wgpu::BufferUsages::VERTEX,
            ),
            index: init(
                "ballpool index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
            sphere: (center, radius),
        })
    }
}

/// A wall: its box, translation, color and object uniforms.
struct Wall {
    geometry: Geometry,
    model: Matrix4,
    /// The material's uniforms but for the velocity's.
    constants: Vec<(String, Vec<f64>)>,
    object: wgpu::Buffer,
    group: wgpu::BindGroup,
}

/// One rebuildScene: the world, the box and the balls.
struct Pool {
    width: f64,
    count: usize,
    world: World,
    bodies: Vec<usize>,
    walls: Vec<Wall>,
    /// The instance matrices: this frame's and the previous frame's.
    matrices: Vec<f32>,
    previous: Option<Vec<f32>>,
    colors: wgpu::Buffer,
    current_buffer: wgpu::Buffer,
    previous_buffer: wgpu::Buffer,
    object: wgpu::Buffer,
    constants: Vec<(String, Vec<f64>)>,
    /// InstancedMesh.boundingSphere, computed on its first frustum test.
    bounds: Option<(V3, f64)>,
    camera: CameraState,
    ball: Draw,
    /// The shadow pipeline, the face cameras' groups and the balls' group.
    shadow: (wgpu::RenderPipeline, Vec<wgpu::BindGroup>, wgpu::BindGroup),
}

struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// The scene MRT: color, diffuse color, normal and velocity.
    scene: [(wgpu::Texture, wgpu::TextureView); 4],
    depth: (wgpu::Texture, wgpu::TextureView),
    ao: wgpu::TextureView,
    gi: wgpu::TextureView,
    composite: (wgpu::Texture, wgpu::TextureView),
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    ssgi: Draw,
    composite_draw: Draw,
    /// Resolve with the placeholder, or the history, previous depth.
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    output: Draw,
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

pub(super) struct Demo {
    random: Random,
    pool: Option<Pool>,
    sphere: Geometry,
    wall: Draw,
    /// The example clock ( ms ), Timer._currentTime and the pointer's stop.
    now: f64,
    timer: f64,
    stop_at: Option<f64>,
    mouse_moving: bool,
    pointer_down: bool,
    active: BTreeSet<i64>,
    ray_origin: V3,
    ray_direction: V3,
    ray_origin_target: V3,
    ray_direction_target: V3,
    light: V3,
    light_target: V3,
    /// A frame was requested; other redraws present the last frame again.
    pending: bool,
    /// The loading frame has rendered: the fixture restarts frameId after it.
    loaded: bool,
    frame_id: usize,
    jitter: usize,
    /// The frame that builds TRAA's output node renders unjittered.
    built: bool,
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    /// Per cube face: its color and depth layers and camera uniforms.
    shadow_faces: Vec<(wgpu::TextureView, wgpu::TextureView, wgpu::Buffer)>,
    shadow_cube: wgpu::TextureView,
    render: wgpu::Buffer,
    wall_render: wgpu::Buffer,
    output_render: wgpu::Buffer,
    ssgi_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}

fn texture(r: &Renderer, size: (u32, u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("ballpool target"),
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
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
fn color(
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
fn depth(view: &wgpu::TextureView) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// The uniform named `nodeUniform{n + offset}`: the generated modules number
/// the same uniforms from different starts.
fn u(n: usize, offset: usize) -> String {
    format!("nodeUniform{}", n + offset)
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: CAM_FOV,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // The point light's 1024² cube shadow.
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
        let shadow_faces = (0..6)
            .map(|i| {
                let buffer = uniform(
                    r,
                    "ballpool shadow camera",
                    wgsl!("shadow_vs"),
                    "renderStruct",
                )?;
                Ok((layer(&shadow_color, i), layer(&shadow_depth, i), buffer))
            })
            .collect::<Result<Vec<_>>>()?;
        let shadow_cube = shadow_depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        // TRAANode's DepthTexture( 1, 1 ) placeholder is never drawn: WebGPU
        // zero-initializes it, so its depth is the near plane.
        let placeholder = view(&texture(r, (1, 1, 1), DEPTH));
        let light = V3::new(0., BOX_HEIGHT / 2., BOX_DEPTH / 2.);
        let wall_render = uniform(r, "ballpool wall render", wgsl!("wall_fs"), "renderStruct")?;
        let wall_pipeline = wall_pipeline(r);
        let wall = (
            wall_pipeline.clone(),
            vec![bind(
                r,
                wall_pipeline.get_bind_group_layout(0),
                &[(0, wall_render.as_entire_binding())],
            )],
        );
        Ok(Self {
            random: Random(186),
            pool: None,
            sphere: Geometry::new(r, &SphereGeometry::build(BALL_RADIUS, 32, 16)?)?,
            wall,
            now: 0.,
            timer: 0.,
            stop_at: None,
            mouse_moving: false,
            pointer_down: false,
            active: BTreeSet::new(),
            ray_origin: V3::default(),
            ray_direction: V3::default(),
            ray_origin_target: V3::default(),
            ray_direction_target: V3::default(),
            light,
            light_target: light,
            pending: true,
            loaded: false,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            shadow_faces,
            shadow_cube,
            render: uniform(r, "ballpool render", wgsl!("ball_fs"), "renderStruct")?,
            wall_render,
            output_render: uniform(r, "ballpool output", wgsl!("output_fs"), "renderStruct")?,
            ssgi_object: uniform(r, "ballpool ssgi", SSGI_FS, "objectStruct")?,
            traa_object: uniform(r, "ballpool traa", TRAA_FS, "objectStruct")?,
            quad_uv: init(
                "ballpool quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            placeholder,
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

    /// The instance path's modules: uniform arrays sized to the count, or
    /// instanced attributes, and the offset of their uniform numbering.
    fn instanced_modules(
        count: usize,
    ) -> (String, &'static str, String, &'static str, usize, usize) {
        if count <= UNIFORM_MATRICES {
            let array = format!("array< mat4x4<f32>, {count} >");
            (
                wgsl!("ball_vs").replace("array< mat4x4<f32>, 257 >", &array),
                wgsl!("ball_fs"),
                wgsl!("shadow_vs").replace("array< mat4x4<f32>, 257 >", &array),
                wgsl!("shadow_fs"),
                0,
                0,
            )
        } else {
            (
                wgsl!("ball_attribute_vs").to_string(),
                wgsl!("ball_attribute_fs"),
                wgsl!("shadow_attribute_vs").to_string(),
                wgsl!("shadow_attribute_fs"),
                6,
                3,
            )
        }
    }

    /// rebuildScene: a new world, box and balls for the aspect.
    fn rebuild(&mut self, r: &Renderer, aspect: f64) -> Result<()> {
        let v_fov = (CAM_FOV / 2.) * (PI / 180.);
        let dist = (BOX_HEIGHT / 2.) / tan(v_fov);
        let width = tan(v_fov) * aspect * dist * 2.;
        let room = width * BOX_HEIGHT * BOX_DEPTH;
        let ball = (4. / 3.) * PI * BALL_RADIUS.powi(3);
        let count = (room * FILL_RATIO * PACKING / ball).floor() as usize;
        let mut world = World::new(crate::bounce::Options {
            gravity: Vec3::new(0., -9.81, 0.),
            solve_velocity_iterations: 6,
            solve_position_iterations: 2,
            linear_damping: 0.1,
            angular_damping: 0.1,
            restitution: 0.4,
            friction: 0.5,
        });
        let camera = CameraState::fit(aspect);
        // createBox: the walls' bodies, and meshes but for the front wall.
        let (hw, hh, hd, t) = (width / 2., BOX_HEIGHT / 2., BOX_DEPTH / 2., WALL_THICKNESS);
        let (white, red, green) = (
            Color::from_hex(0xeeeeee),
            Color::from_hex(0xff2222),
            Color::from_hex(0x22ff22),
        );
        let walls = [
            ([width, t, BOX_DEPTH], [0., -t / 2., 0.], Some(white)),
            (
                [width, t, BOX_DEPTH],
                [0., BOX_HEIGHT + t / 2., 0.],
                Some(white),
            ),
            ([width, BOX_HEIGHT, t], [0., hh, -hd - t / 2.], Some(white)),
            ([width, BOX_HEIGHT, t], [0., hh, hd + t / 2.], None),
            (
                [t, BOX_HEIGHT, BOX_DEPTH],
                [-hw - t / 2., hh, 0.],
                Some(red),
            ),
            (
                [t, BOX_HEIGHT, BOX_DEPTH],
                [hw + t / 2., hh, 0.],
                Some(green),
            ),
        ];
        let wall_layout = self.wall.0.get_bind_group_layout(1);
        let identity3 = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
        // The materials' constant uniforms: color, opacity, metalness,
        // roughness, normal matrix, IOR, specular color and intensity,
        // emissive and its intensity, and the model matrix.
        let material =
            |color: [f64; 3], metalness: f64, roughness: f64, model: Matrix4, offset: usize| {
                vec![
                    (u(0, offset), color.to_vec()),
                    (u(1, offset), vec![1.]),
                    (u(2, offset), vec![metalness]),
                    (u(3, offset), vec![roughness]),
                    (u(5, offset), identity3.clone()),
                    (u(6, offset), vec![1.5]),
                    (u(7, offset), vec![1.; 3]),
                    (u(8, offset), vec![1.]),
                    (u(9, offset), vec![0.; 3]),
                    (u(10, offset), vec![1.]),
                    (u(12, offset), m4(model)),
                ]
            };
        let mut meshes = vec![];
        for (size, position, wall_color) in walls {
            let shape = world.create_box(size[0], size[1], size[2]);
            world.create_static_body(shape, Vec3::new(position[0], position[1], position[2]));
            if let Some(material_color) = wall_color {
                let geometry = Geometry::new(r, &BoxGeometry::build(size[0], size[1], size[2])?)?;
                let object = uniform(r, "ballpool wall", wgsl!("wall_fs"), "objectStruct")?;
                let group = bind(
                    r,
                    wall_layout.clone(),
                    &[
                        (0, object.as_entire_binding()),
                        (1, wgpu::BindingResource::Sampler(&self.linear)),
                        (2, wgpu::BindingResource::TextureView(&r.dfg)),
                        (3, wgpu::BindingResource::Sampler(&self.compare)),
                        (4, wgpu::BindingResource::TextureView(&self.shadow_cube)),
                    ],
                );
                let model = Matrix4::from_translation(Vector3::from_array(position));
                meshes.push(Wall {
                    geometry,
                    model,
                    constants: material(material_color.0.to_array(), 0., 0.7, model, 0),
                    object,
                    group,
                });
            }
        }
        // createBalls: instance colors, then each ball's spawn.
        let sphere_shape = world.create_sphere(BALL_RADIUS);
        let colors: Vec<f32> = (0..count)
            .flat_map(|i| {
                Color::from_hex(COLORS[i % COLORS.len()])
                    .0
                    .to_array()
                    .map(|v| v as f32)
            })
            .collect();
        let (hw, hd) = (
            width / 2. - BALL_RADIUS - 0.1,
            BOX_DEPTH / 2. - BALL_RADIUS - 0.1,
        );
        let mut bodies = Vec::with_capacity(count);
        for _ in 0..count {
            let x = (self.random.next() - 0.5) * 2. * hw;
            let y = BALL_RADIUS + self.random.next() * (BOX_HEIGHT - BALL_RADIUS * 2.);
            let z = (self.random.next() - 0.5) * 2. * hd;
            bodies.push(world.create_dynamic_body(sphere_shape, Vec3::new(x, y, z), 1., 0.5, 0.4));
        }
        let (ball_vs, ball_fs, shadow_vs, shadow_fs, offset, shadow_offset) =
            Self::instanced_modules(count);
        let uniform_path = count <= UNIFORM_MATRICES;
        let usage = if uniform_path {
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        } else {
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        };
        let matrix_buffer = |label| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: (count * 64) as u64,
                usage,
                mapped_at_creation: false,
            })
        };
        let current_buffer = matrix_buffer("ballpool instance matrices");
        let previous_buffer = matrix_buffer("ballpool previous instance matrices");
        let colors = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ballpool instance colors"),
                contents: bytemuck::cast_slice(&colors),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let object = uniform(r, "ballpool balls", ball_fs, "objectStruct")?;
        let shadow_object = uniform(r, "ballpool shadow balls", shadow_fs, "objectStruct")?;
        let f3 = wgpu::VertexFormat::Float32x3;
        let f4 = wgpu::VertexFormat::Float32x4;
        let attribute = |format, offset, location| wgpu::VertexAttribute {
            format,
            offset,
            shader_location: location,
        };
        let vertex = |stride, step_mode, attributes| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode,
            attributes,
        };
        let (instance, per_vertex) = (wgpu::VertexStepMode::Instance, wgpu::VertexStepMode::Vertex);
        let position_attr = [attribute(f3, 0, 0)];
        let normal_attr = [attribute(f3, 0, 1)];
        let matrix_attr =
            |first: u32| [0, 1, 2, 3].map(|k| attribute(f4, 16 * k as u64, first + k));
        let (current_attr, previous_attr, shadow_matrix_attr) =
            (matrix_attr(2), matrix_attr(6), matrix_attr(2));
        let ball_color_attr = [attribute(f3, 0, if uniform_path { 2 } else { 10 })];
        let shadow_color_attr = [attribute(f3, 0, if uniform_path { 2 } else { 6 })];
        let mut ball_layouts = vec![
            vertex(12, per_vertex, &position_attr[..]),
            vertex(12, per_vertex, &normal_attr[..]),
        ];
        let mut shadow_layouts = ball_layouts.clone();
        if !uniform_path {
            ball_layouts.push(vertex(64, instance, &current_attr[..]));
            ball_layouts.push(vertex(64, instance, &previous_attr[..]));
            shadow_layouts.push(vertex(64, instance, &shadow_matrix_attr[..]));
        }
        ball_layouts.push(vertex(12, instance, &ball_color_attr[..]));
        shadow_layouts.push(vertex(12, instance, &shadow_color_attr[..]));
        let mrt = [HALF, BYTE, BYTE, HALF];
        let ball_pipeline = sampled_pipeline(
            r,
            "ballpool balls",
            (ball_vs.as_str(), ball_fs),
            &ball_layouts,
            &mrt,
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let mut ball_entries = vec![
            (0, object.as_entire_binding()),
            (1, wgpu::BindingResource::Sampler(&self.linear)),
            (2, wgpu::BindingResource::TextureView(&r.dfg)),
            (3, wgpu::BindingResource::Sampler(&self.compare)),
            (4, wgpu::BindingResource::TextureView(&self.shadow_cube)),
        ];
        if uniform_path {
            ball_entries.push((5, current_buffer.as_entire_binding()));
            ball_entries.push((6, previous_buffer.as_entire_binding()));
        }
        let ball = (
            ball_pipeline.clone(),
            vec![
                bind(
                    r,
                    ball_pipeline.get_bind_group_layout(0),
                    &[(0, self.render.as_entire_binding())],
                ),
                bind(r, ball_pipeline.get_bind_group_layout(1), &ball_entries),
            ],
        );
        // The cube faces flip the winding: the casters' front faces are clockwise.
        let shadow_pipeline = raw_pipeline(
            r,
            "ballpool shadow",
            &shadow_vs,
            shadow_fs,
            &shadow_layouts,
            BYTE,
            1,
            Some((wgpu::CompareFunction::LessEqual, true)),
            true,
        );
        let mut shadow_entries = vec![(0, shadow_object.as_entire_binding())];
        if uniform_path {
            shadow_entries.push((1, current_buffer.as_entire_binding()));
        }
        let faces = self
            .shadow_faces
            .iter()
            .map(|(_, _, buffer)| {
                bind(
                    r,
                    shadow_pipeline.get_bind_group_layout(0),
                    &[(0, buffer.as_entire_binding())],
                )
            })
            .collect();
        let shadow_group = bind(r, shadow_pipeline.get_bind_group_layout(1), &shadow_entries);
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(String, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> =
                values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        // The balls' InstancedMesh sits at the origin: its matrices are identities.
        let constants = material([1.; 3], 0.1, 0.3, Matrix4::IDENTITY, 3 + offset);
        write(
            &shadow_object,
            shadow_fs,
            "objectStruct",
            &[
                (u(2, shadow_offset), vec![1.]),
                (u(5, shadow_offset), m4(Matrix4::IDENTITY)),
            ],
        )?;
        self.pool = Some(Pool {
            width,
            count,
            world,
            bodies,
            walls: meshes,
            matrices: vec![0.; count * 16],
            previous: None,
            colors,
            current_buffer,
            previous_buffer,
            object,
            constants,
            bounds: None,
            camera,
            ball,
            shadow: (shadow_pipeline, faces, shadow_group),
        });
        Ok(())
    }
}

fn wall_pipeline(r: &Renderer) -> wgpu::RenderPipeline {
    let attrs = [
        wgpu::vertex_attr_array![0 => Float32x3],
        wgpu::vertex_attr_array![1 => Float32x3],
    ];
    let layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &attrs[i],
    });
    sampled_pipeline(
        r,
        "ballpool walls",
        (wgsl!("wall_vs"), wgsl!("wall_fs")),
        &layouts,
        &[HALF, BYTE, BYTE, HALF],
        Some((wgpu::CompareFunction::LessEqual, true)),
        (false, false),
        (1, wgpu::PrimitiveTopology::TriangleList),
    )
}

impl Demo {
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
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
        let scene = [make(HALF), make(BYTE), make(BYTE), make(HALF)];
        let depth_target = make(DEPTH);
        let ao = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let gi = view(&texture(r, size, wgpu::TextureFormat::Rg11b10Ufloat));
        let composite = make(HALF);
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
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
                (1, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let ssgi_pipeline = screen_pass(
            "ballpool ssgi",
            SSGI_VS,
            SSGI_FS,
            &[
                wgpu::TextureFormat::R8Unorm,
                wgpu::TextureFormat::Rg11b10Ufloat,
            ],
        );
        let ssgi = (
            ssgi_pipeline.clone(),
            vec![bind(
                r,
                ssgi_pipeline.get_bind_group_layout(0),
                &[
                    (0, tex(&depth_target.1)),
                    (1, self.ssgi_object.as_entire_binding()),
                    (2, sampler(&self.linear)),
                    (3, tex(&scene[2].1)),
                    (4, sampler(&self.linear)),
                    (5, tex(&scene[0].1)),
                ],
            )],
        );
        let composite_pipeline = screen_pass("ballpool composite", QUAD_VS, COMPOSITE_FS, &[HALF]);
        let composite_draw = (
            composite_pipeline.clone(),
            vec![bind(
                r,
                composite_pipeline.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&scene[0].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&ao)),
                    (4, sampler(&self.linear)),
                    (5, tex(&scene[1].1)),
                    (6, sampler(&self.linear)),
                    (7, tex(&gi)),
                ],
            )],
        );
        let traa_pipeline = screen_pass("ballpool traa", TRAA_VS, TRAA_FS, &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_groups = [&self.placeholder, &history_depth_view].map(|previous| {
            bind(
                r,
                traa_pipeline.get_bind_group_layout(0),
                &[
                    // Velocity is read with textureLoad: the layout drops its sampler.
                    (1, tex(&scene[3].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&composite.1)),
                    (4, tex(&depth_target.1)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(previous)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        });
        let format = out.options.format;
        let output_pipeline =
            screen_pass("ballpool output", QUAD_VS, wgsl!("output_fs"), &[format]);
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
                    &[(0, sampler(&self.linear)), (1, tex(&resolve.1))],
                ),
            ],
        );
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
            scene,
            depth: depth_target,
            ao,
            gi,
            composite,
            resolve,
            history,
            history_depth,
            screen,
            ssgi,
            composite_draw,
            traa: (traa_pipeline, traa_groups),
            output,
        });
        Ok(())
    }

    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ballpool output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
    }

    /// respawnBalls: five random balls back to the top, at rest.
    fn respawn(&mut self) {
        let Some(pool) = &mut self.pool else { return };
        let hw = pool.width / 2. - BALL_RADIUS - 0.1;
        let hd = BOX_DEPTH / 2. - BALL_RADIUS - 0.1;
        for _ in 0..5 {
            let body =
                pool.bodies[(self.random.next() * pool.bodies.len() as f64).floor() as usize];
            let x = (self.random.next() - 0.5) * 2. * hw;
            let y = BOX_HEIGHT - BALL_RADIUS - self.random.next() * 1.;
            let z = (self.random.next() - 0.5) * 2. * hd;
            pool.world.respawn(body, Vec3::new(x, y, z));
        }
    }

    /// The pointer ray's pushes: balls within 1.5 of the ray move away from it.
    fn push(&mut self) {
        let Some(pool) = &mut self.pool else { return };
        let (push_radius, push_strength) = (1.5, 15.);
        for &body in &pool.bodies {
            let p = pool.world.body(body).position;
            let ball = V3::new(p.x, p.y, p.z);
            // Ray.closestPointToPoint.
            let t = ball.sub(self.ray_origin).dot(self.ray_direction);
            let closest = if t < 0. {
                self.ray_origin
            } else {
                self.ray_origin.add_scaled(self.ray_direction, t)
            };
            let dist = closest.distance_to(ball);
            if dist < push_radius {
                let mut direction = ball.sub(closest);
                if direction.length_sq() < 0.001 {
                    direction = V3::new(0., 1., 0.);
                }
                let direction = direction.normalize();
                let strength = push_strength * (1. - dist / push_radius);
                pool.world.apply_linear_impulse(
                    body,
                    Vec3::new(
                        direction.x * strength,
                        direction.y * strength,
                        direction.z * strength,
                    ),
                );
            }
        }
    }

    /// The page's animate(): the timer, easing, interaction and the step.
    fn animate(&mut self, r: &Renderer) {
        // The pointer's stop timer runs on the example clock before the frame.
        if self.stop_at.is_some_and(|at| at <= self.now) {
            self.stop_at = None;
            self.mouse_moving = false;
        }
        let previous = std::mem::replace(&mut self.timer, self.now);
        let dt = ((self.timer - previous) / 1000.).min(1. / 30.);
        let ease = 1. - exp(-EASE_SPEED * dt);
        self.ray_origin = self.ray_origin.lerp(self.ray_origin_target, ease);
        self.ray_direction = self.ray_direction.lerp(self.ray_direction_target, ease);
        self.light = self.light.lerp(self.light_target, ease);
        if self.pointer_down {
            self.respawn();
        }
        if self.mouse_moving {
            self.push();
        }
        let Some(pool) = &mut self.pool else { return };
        // performance.now() is the example clock.
        pool.world.advance_time(1. / 60., dt, self.now / 1000.);
        for (i, &body) in pool.bodies.iter().enumerate() {
            let b = pool.world.body(body);
            let Quat { x, y, z, w } = b.orientation;
            let m = compose(
                V3::new(b.position.x, b.position.y, b.position.z),
                (x, y, z, w),
            );
            for (k, v) in m.iter().enumerate() {
                pool.matrices[i * 16 + k] = *v as f32;
            }
        }
        r.queue.write_buffer(
            &pool.current_buffer,
            0,
            bytemuck::cast_slice(&pool.matrices),
        );
        // InstanceNode's previous matrices: the array the last scene pass drew,
        // or this frame's for a new mesh.
        let previous = pool
            .previous
            .replace(pool.matrices.clone())
            .unwrap_or_else(|| pool.matrices.clone());
        r.queue
            .write_buffer(&pool.previous_buffer, 0, bytemuck::cast_slice(&previous));
        // InstancedMesh.computeBoundingSphere on the first frustum test.
        if pool.bounds.is_none() {
            let (local_center, local_radius) = self.sphere.sphere;
            let mut bounds: Option<(V3, f64)> = None;
            for m in pool.matrices.chunks(16) {
                let e: [f64; 16] = std::array::from_fn(|k| m[k] as f64);
                let center = local_center.apply(&e);
                let scale = [0, 4, 8]
                    .map(|c| e[c] * e[c] + e[c + 1] * e[c + 1] + e[c + 2] * e[c + 2])
                    .into_iter()
                    .fold(0., f64::max)
                    .sqrt();
                let radius = local_radius * scale;
                bounds = Some(match bounds {
                    None => (center, radius),
                    Some(b) => union(b, (center, radius)),
                });
            }
            pool.bounds = bounds;
        }
    }

    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let resized = self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        });
        if resized {
            self.resize(r, out)?;
            // New history targets: TRAA restarts from the beauty buffer.
            self.resolves = 0;
        }
        // The page rebuilds the scene on loading and on every resize.
        if self.pool.is_none() || resized {
            self.rebuild(r, out.width as f64 / out.height as f64)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("ballpool targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::replace(&mut self.loaded, true) {
            self.frame_id += 1;
        }
        self.animate(r);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("ballpool targets"))?;
        let pool = self.pool.as_ref().ok_or(Error::Invalid("ballpool scene"))?;
        let camera = pool.camera;
        let (projection, view) = (camera.projection(), camera.view());
        // Keep the gallery camera on the page's.
        if let Ok(node) = s.get_mut(c) {
            node.position = camera.position.glam();
        }
        // TRAANode.setViewOffset: the Halton offset in pixels.
        // The passes render when TRAA's resolve samples them, after its
        // updateBefore has set the offset, the building frame's included.
        let jittered = {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        };
        let motion = self.motion.unwrap_or(Motion { projection, view });
        self.motion = Some(Motion { projection, view });
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(String, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> =
                values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        let light = self.light.glam();
        // The point light ( intensity 80, no cutoff, decay 2 ) and its shadow
        // ( near 0.5, far 500, radius 20, no biases ).
        let lights = |offset: usize| {
            vec![
                (u(29, offset), m4(motion.projection)),
                ("cameraProjectionMatrix".into(), m4(jittered)),
                ("cameraViewMatrix".into(), m4(view)),
                (
                    u(13, offset),
                    view.transform_point3(light).to_array().to_vec(),
                ),
                (u(14, offset), vec![80.; 3]),
                (u(15, offset), m4(Matrix4::from_translation(-light))),
                (u(17, offset), vec![0.]),
                (u(18, offset), vec![500.]),
                (u(19, offset), vec![0.5]),
                (u(20, offset), vec![0.]),
                (u(22, offset), vec![20.]),
                (u(23, offset), vec![SHADOW as f64; 2]),
                (u(24, offset), vec![1.]),
                (u(25, offset), vec![0.]),
                (u(26, offset), vec![2.]),
            ]
        };
        let (_, ball_fs, _, _, offset, _) = Self::instanced_modules(pool.count);
        write(
            &self.wall_render,
            wgsl!("wall_fs"),
            "renderStruct",
            &lights(0),
        )?;
        write(&self.render, ball_fs, "renderStruct", &lights(3 + offset))?;
        let velocity = |offset: usize, model: Matrix4| {
            vec![
                (u(27, offset), m4(projection)),
                (u(28, offset), m4(model)),
                (u(30, offset), m4(motion.view)),
                (u(31, offset), m4(model)),
            ]
        };
        for wall in &pool.walls {
            let values = [wall.constants.clone(), velocity(0, wall.model)].concat();
            write(&wall.object, wgsl!("wall_fs"), "objectStruct", &values)?;
        }
        let values = [
            pool.constants.clone(),
            velocity(3 + offset, Matrix4::IDENTITY),
        ]
        .concat();
        write(&pool.object, ball_fs, "objectStruct", &values)?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform1".into(), vec![0.5])],
        )?;
        let (direction, step_offset) = (
            TEMPORAL_ROTATIONS[self.frame_id % 6] / 360.,
            SPATIAL_OFFSETS[self.frame_id % 4],
        );
        write(
            &self.ssgi_object,
            SSGI_FS,
            "objectStruct",
            &[
                ("nodeUniform1".into(), m4(jittered.inverse())),
                ("nodeUniform3".into(), vec![2.]),
                ("nodeUniform4".into(), vec![8.]),
                ("nodeUniform5".into(), vec![1.]),
                ("nodeUniform6".into(), vec![10.]),
                ("nodeUniform7".into(), vec![12.]),
                ("nodeUniform8".into(), vec![1.]),
                ("nodeUniform9".into(), vec![t.width as f64, t.height as f64]),
                (
                    "nodeUniform10".into(),
                    vec![t.height as f64 / (((CAM_FOV * PI / 180.) * 0.5).tan() * 2.) * 0.5],
                ),
                ("nodeUniform11".into(), vec![direction]),
                ("nodeUniform12".into(), vec![2.]),
                ("nodeUniform13".into(), vec![1.]),
                ("nodeUniform14".into(), vec![0.]),
                ("nodeUniform15".into(), vec![step_offset]),
                ("nodeUniform16".into(), vec![0.]),
                ("nodeUniform17".into(), vec![100.]),
                ("nodeUniform19".into(), vec![1.]),
            ],
        )?;
        // TRAANode.updateBefore: the previous camera matrices ( identity
        // before the first resolve ) and this frame's.
        let previous = self.resolved.unwrap_or(Resolved {
            world: Matrix4::IDENTITY,
            projection_inverse: Matrix4::IDENTITY,
        });
        // updateBefore keeps the camera before it sets the building frame's
        // offset.
        self.resolved = Some(Resolved {
            world: Matrix4::from_cols_array(&camera.world),
            projection_inverse: if self.built { jittered } else { projection }.inverse(),
        });
        let identity3 = vec![1., 0., 0., 0., 1., 0., 0., 0., 1.];
        write(
            &self.traa_object,
            TRAA_FS,
            "objectStruct",
            &[
                ("nodeUniform3".into(), vec![0.1, 100.]),
                ("nodeUniform4".into(), m4(view)),
                ("nodeUniform5".into(), m4(previous.world)),
                ("nodeUniform6".into(), m4(previous.projection_inverse)),
                ("nodeUniform8".into(), identity3.clone()),
                ("nodeUniform10".into(), identity3),
                ("nodeUniform11".into(), vec![1.]),
            ],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The cube shadow: each face draws the balls when their bounding
        // sphere meets the face's frustum.
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., 0.5, 500.);
        let bounds = pool.bounds.map(|(c, radius)| Sphere {
            center: c.glam(),
            radius,
        });
        let visible = |frustum: &Frustum| bounds.is_none_or(|b| frustum.intersects_sphere(b));
        for (i, (color_view, depth_view, buffer)) in self.shadow_faces.iter().enumerate() {
            let (direction, up) = FACES[i];
            let face_view = Matrix4::look_at_rh(
                light,
                light + Vector3::from_array(direction),
                Vector3::from_array(up),
            );
            write(
                buffer,
                wgsl!("shadow_vs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix".into(), m4(face_projection)),
                    ("cameraViewMatrix".into(), m4(face_view)),
                ],
            )?;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ballpool shadow"),
                color_attachments: &[color(color_view, wgpu::Color::BLACK)],
                depth_stencil_attachment: depth(depth_view),
                ..Default::default()
            });
            if visible(&Frustum::from_projection(face_projection * face_view)) {
                pass.set_pipeline(&pool.shadow.0);
                pass.set_bind_group(0, &pool.shadow.1[i], &[]);
                pass.set_bind_group(1, &pool.shadow.2, &[]);
                draw_balls(&mut pass, pool, &self.sphere, false);
            }
        }
        // The scene MRT.
        {
            let black = wgpu::Color::BLACK;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ballpool scene"),
                color_attachments: &[
                    color(&t.scene[0].1, black),
                    color(&t.scene[1].1, black),
                    color(&t.scene[2].1, black),
                    color(&t.scene[3].1, black),
                ],
                depth_stencil_attachment: depth(&t.depth.1),
                ..Default::default()
            });
            set(&mut pass, &self.wall);
            for wall in &pool.walls {
                pass.set_bind_group(1, &wall.group, &[]);
                pass.set_vertex_buffer(0, wall.geometry.normals.slice(..));
                pass.set_vertex_buffer(1, wall.geometry.positions.slice(..));
                pass.set_index_buffer(wall.geometry.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..wall.geometry.count, 0, 0..1);
            }
            if visible(&Frustum::from_projection(jittered * view)) {
                set(&mut pass, &pool.ball);
                draw_balls(&mut pass, pool, &self.sphere, true);
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    targets: &[Option<wgpu::RenderPassColorAttachment>],
                    draw: &Draw| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ballpool quad"),
                color_attachments: targets,
                ..Default::default()
            });
            set(&mut pass, draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        // AO clears white, GI black.
        quad(
            &mut encoder,
            &[
                color(&t.ao, wgpu::Color::WHITE),
                color(&t.gi, wgpu::Color::BLACK),
            ],
            &t.ssgi,
        );
        let full = wgpu::Extent3d {
            width: t.width,
            height: t.height,
            depth_or_array_layers: 1,
        };
        // TRAANode sizes and restarts its history from the composite's render
        // target before the resolve renders that target ( convertToTexture
        // updates when it is sampled ). Its first frame reads it at 1 × 1, so
        // the second restarts again, from the first frame's composite.
        if self.resolves == 1 {
            encoder.copy_texture_to_texture(
                t.composite.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
        }
        quad(
            &mut encoder,
            &[color(&t.composite.1, wgpu::Color::BLACK)],
            &t.composite_draw,
        );
        if self.resolves == 0 {
            encoder.copy_texture_to_texture(
                t.composite.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
        }
        // The first two resolves read TRAANode's 1 × 1 placeholder depth
        // ( the far plane: the first frame's history does not match the
        // drawing buffer, so it keeps no depth ); later ones the previous frame's.
        let (pipeline, groups) = &t.traa;
        let draw = (
            pipeline.clone(),
            vec![groups[usize::from(self.resolves >= 2)].clone()],
        );
        quad(
            &mut encoder,
            &[color(&t.resolve.1, wgpu::Color::BLACK)],
            &draw,
        );
        encoder.copy_texture_to_texture(
            t.resolve.0.as_image_copy(),
            t.history.as_image_copy(),
            full,
        );
        encoder.copy_texture_to_texture(
            t.depth.0.as_image_copy(),
            t.history_depth.as_image_copy(),
            full,
        );
        self.resolves += 1;
        self.jitter = (self.jitter + 1) % 32;
        self.built = true;
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }

    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }

    /// Pointer events: 0 a move to normalized device coordinates ( x, y ),
    /// 1 a press and 2 a release of pointer x ( y 1 for touch ).
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            0 => {
                let Some(pool) = &self.pool else { return };
                let (origin, direction) = pool.camera.ray(x, y);
                self.ray_origin_target = origin;
                self.ray_direction_target = direction;
                self.mouse_moving = true;
                self.stop_at = Some(self.now + 50.);
                // Ray.intersectPlane with the front plane ( 0, 0, 1 ), −d/2.
                let normal = V3::new(0., 0., 1.);
                let constant = -BOX_DEPTH / 2.;
                let denominator = normal.dot(direction);
                let t = if denominator == 0. {
                    (origin.dot(normal) + constant == 0.).then_some(0.)
                } else {
                    let t = -(origin.dot(normal) + constant) / denominator;
                    (t >= 0.).then_some(t)
                };
                if let Some(t) = t {
                    self.light_target = origin.add_scaled(direction, t);
                }
            }
            1 | 2 => {
                let id = x as i64;
                if kind == 1 {
                    self.active.insert(id);
                } else {
                    self.active.remove(&id);
                }
                self.pointer_down = if y > 0.5 {
                    self.active.len() >= 2
                } else {
                    kind == 1
                };
            }
            _ => {}
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("ballpool has no parameters"))
    }
    /// The example clock: the fixture renders each frame at a time.
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
        self.pending = true;
    }
}

/// The instanced spheres: positions, normals, then the instance buffers.
fn draw_balls(pass: &mut wgpu::RenderPass, pool: &Pool, sphere: &Geometry, scene: bool) {
    pass.set_vertex_buffer(0, sphere.positions.slice(..));
    pass.set_vertex_buffer(1, sphere.normals.slice(..));
    let mut slot = 2;
    if pool.count > UNIFORM_MATRICES {
        pass.set_vertex_buffer(slot, pool.current_buffer.slice(..));
        slot += 1;
        if scene {
            pass.set_vertex_buffer(slot, pool.previous_buffer.slice(..));
            slot += 1;
        }
    }
    pass.set_vertex_buffer(slot, pool.colors.slice(..));
    pass.set_index_buffer(sphere.index.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..sphere.count, 0, 0..pool.count as u32);
}

/// Sphere.union.
fn union((center, radius): (V3, f64), (c, r): (V3, f64)) -> (V3, f64) {
    if center == c && radius == r {
        return (center, radius);
    }
    let expand = |(center, radius): (V3, f64), point: V3| {
        let v = point.sub(center);
        let length_sq = v.length_sq();
        if length_sq > radius * radius {
            let length = length_sq.sqrt();
            let delta = (length - radius) * 0.5;
            (center.add_scaled(v, delta / length), radius + delta)
        } else {
            (center, radius)
        }
    };
    let d = c.sub(center);
    let l = d.length();
    let v = if l == 0. { d } else { d.scale(r / l) };
    let s = expand((center, radius), c.add(v));
    expand(s, c.sub(v))
}
