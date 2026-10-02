//! webgpu_postprocessing_ao: a small gallery ( checkerboard floor, two
//! walls, pedestals, columns, a torus knot, a lathed vase, an armchair of
//! rounded boxes, a side table, a rug, extruded picture frames and the Draco
//! tennyson bust ) under three spot lights and the RoomEnvironment PMREM,
//! with GTAONode's ambient occlusion feeding the materials' ambient term
//! and TRAA resolving its temporal noise. The pre-pass writes packed normals
//! and velocity; GTAO traces at half resolution ( its slice rotation and
//! step offset following frameId ) over a 5 × 5 magic-square noise; the
//! scene pass reads the AO; TRAA ( without subpixel correction ) jitters,
//! reprojects and blends; the output tone maps ( Neutral ) and encodes
//! sRGB. Every stage runs the WGSL three.js r186 generates for the page (in
//! `gtao/`; the output is the volume_traa module and the AO-only vertex
//! stage the lightprobes one, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{mipmapped_raw, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::text_shapes::{Extrude, Path, Shape, extrude};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// Color( 0x666666 ) in linear space: the background clear.
const BACKGROUND: f64 = 0.13286832154414627;
/// SpotLight( 0xffe09e, 40 ), angle 0.6, penumbra 1, distance 6.5, decay 2.
const SPOTS: [([f64; 3], [f64; 3]); 3] = [
    ([-2.5, 5., -4.8], [-2.5, -2., -4.8]),
    ([2.5, 5., -4.8], [2.5, -2., -4.8]),
    ([-5.3, 5., 0.], [-5.3, -2., 0.]),
];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("gtao/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("volume_traa/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("volume_traa/output_fs.wgsl");
const FULLSCREEN_VS: &str = include_str!("lightprobes/repack_vs.wgsl");
/// GTAONode's per-frame slice rotations and step offsets.
const TEMPORAL_ROTATIONS: [f64; 6] = [60., 300., 180., 240., 120., 0.];
const SPATIAL_OFFSETS: [f64; 4] = [0., 0.5, 0.25, 0.75];
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
fn at(p: [f64; 3]) -> Matrix4 {
    compose(p, [0.; 3], [1.; 3])
}
/// MeshStandardMaterial: colour ( sRGB hex ), roughness and metalness; the
/// floor's material also has its map.
#[derive(Clone, Copy)]
struct Standard {
    color: [f64; 3],
    roughness: f64,
    metalness: f64,
    map: bool,
}
fn standard(hex: u32, roughness: f64, metalness: f64) -> Standard {
    Standard {
        color: Color::from_hex(hex).0.to_array(),
        roughness,
        metalness,
        map: false,
    }
}
/// RoundedBoxGeometry( width, height, depth, segments, radius ): the
/// non-indexed box with its vertices pushed onto the rounded corners.
fn rounded_box(
    width: f64,
    height: f64,
    depth: f64,
    segments: u32,
    radius: f64,
) -> Result<BufferGeometry> {
    let total = segments * 2 + 1;
    let radius = radius.min(width / 2.).min(height / 2.).min(depth / 2.);
    let source = BoxGeometry::segmented(1., 1., 1., total, total, total)?.to_non_indexed()?;
    let position = source
        .attributes
        .get("position")
        .ok_or(Error::Invalid("rounded box position"))?;
    let half_segment = 0.5 / total as f64;
    let boxed = [
        width / 2. - radius,
        height / 2. - radius,
        depth / 2. - radius,
    ];
    let sign = |v: f64| {
        if v > 0. {
            1.
        } else if v < 0. {
            -1.
        } else {
            v
        }
    };
    let mut positions = vec![];
    let mut normals = vec![];
    for i in 0..position.count() {
        let p = [
            position.get_component(i, 0)? as f32 as f64,
            position.get_component(i, 1)? as f32 as f64,
            position.get_component(i, 2)? as f32 as f64,
        ];
        let mut n = [0, 1, 2].map(|k| p[k] - sign(p[k]) * half_segment);
        let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        let inv = 1. / if length == 0. { 1. } else { length };
        n = n.map(|v| v * inv);
        for k in 0..3 {
            positions.push((boxed[k] * sign(p[k]) + n[k] * radius) as f32);
            normals.push(n[k] as f32);
        }
    }
    geometry_from(&positions, Some(&normals), None)
}
fn geometry_from(
    positions: &[f32],
    normals: Option<&[f32]>,
    index: Option<Vec<u32>>,
) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    let v3 = |d: &[f32]| -> Result<_> {
        Ok(Attribute::F32(crate::attribute::BufferAttribute::new(
            d.to_vec(),
            3,
            false,
        )?))
    };
    g.set_attribute("position", v3(positions)?);
    if let Some(n) = normals {
        g.set_attribute("normal", v3(n)?);
    } else {
        g.compute_vertex_normals()?;
    }
    g.index = index;
    Ok(g)
}
/// A frame of addFrame(): the outline with the picture's hole, extruded
/// 0.12 with a 0.02 bevel in two segments.
fn frame_geometry(width: f64, height: f64) -> Result<BufferGeometry> {
    let t = 0.1;
    let (ow, oh) = (width + t * 2., height + t * 2.);
    let shape = Shape {
        outline: Path::polygon(&[
            [-ow / 2., -oh / 2.],
            [ow / 2., -oh / 2.],
            [ow / 2., oh / 2.],
            [-ow / 2., oh / 2.],
        ]),
        holes: vec![Path::polygon(&[
            [-width / 2., -height / 2.],
            [width / 2., -height / 2.],
            [width / 2., height / 2.],
            [-width / 2., height / 2.],
        ])],
    };
    let (positions, _) = extrude(
        &[shape],
        &Extrude {
            curve_segments: 12,
            steps: 1,
            depth: 0.12,
            bevel: Some((0.02, 0.02, 2)),
        },
    );
    geometry_from(&positions, None, None)
}
/// One drawn mesh: its buffers, bounds, world matrix and material.
struct Mesh {
    /// Uv ( floor only ), normal and position.
    buffers: Vec<wgpu::Buffer>,
    index: Option<wgpu::Buffer>,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
    material: Standard,
    /// The pre-pass and scene objects.
    objects: [wgpu::Buffer; 2],
}
impl Mesh {
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self
            .model
            .to_scale_rotation_translation()
            .0
            .abs()
            .max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (i, b) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(i as u32, b.slice(..));
        }
        match &self.index {
            Some(index) => {
                pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.count, 0, 0..1);
            }
            None => pass.draw(0..self.count, 0..1),
        }
    }
}
fn texture(r: &Renderer, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("gtao target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
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
#[derive(Clone, Copy)]
struct Motion {
    projection: Matrix4,
    view: Matrix4,
}
#[derive(Clone, Copy)]
struct Resolved {
    world: Matrix4,
    projection_inverse: Matrix4,
}
/// The AO target and its draw for one resolution scale.
struct AoTarget {
    size: (u32, u32),
    view: wgpu::TextureView,
    /// Scene draws ( per mesh ) and the AO-only output reading it.
    scene: Vec<Draw>,
    ao_only: Draw,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    prepass: [(wgpu::Texture, wgpu::TextureView); 2],
    prepass_depth: (wgpu::Texture, wgpu::TextureView),
    scene: (wgpu::Texture, wgpu::TextureView),
    scene_depth: wgpu::TextureView,
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    prepass_draws: Vec<Draw>,
    /// The GTAO draw and its target per visited resolution scale.
    ao: Vec<(f64, AoTarget, Draw)>,
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    output: Draw,
}
pub(super) struct Demo {
    controls: Controls,
    /// AO type, resolution, samples, radius, scale, thickness, temporal
    /// filtering, intensity, bias, blur, blur sharpness, transparent mesh,
    /// its opacity and AO only.
    params: [f64; 14],
    pending: bool,
    loaded: bool,
    frame_id: usize,
    jitter: usize,
    built: bool,
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    meshes: Vec<Mesh>,
    floor_map: wgpu::TextureView,
    noise: wgpu::TextureView,
    environment: wgpu::TextureView,
    /// Pre-pass and scene render uniforms for the plain and mapped materials.
    renders: [[wgpu::Buffer; 2]; 2],
    gtao_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    ao_only_render: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
    nearest: wgpu::Sampler,
    repeat: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The pre-pass and scene shaders of the plain and mapped materials.
const SHADERS: [[(&str, &str); 2]; 2] = [
    [
        (wgsl!("prepass_vs"), wgsl!("prepass_fs")),
        (wgsl!("prepass_floor_vs"), wgsl!("prepass_floor_fs")),
    ],
    [
        (wgsl!("scene_vs"), wgsl!("scene_fs")),
        (wgsl!("scene_floor_vs"), wgsl!("scene_floor_fs")),
    ],
];
/// generateMagicSquareNoise( 5 ): rotation vectors in a 5 × 5 magic square.
fn magic_square_noise() -> Vec<u8> {
    let n = 5i32;
    let count = (n * n) as usize;
    let mut square = vec![0usize; count];
    let (mut i, mut j) = (n / 2, n - 1);
    let mut num = 1;
    while num <= count {
        if i == -1 && j == n {
            j = n - 2;
            i = 0;
        } else {
            if j == n {
                j = 0;
            }
            if i < 0 {
                i = n - 1;
            }
        }
        let at = (i * n + j) as usize;
        if square[at] != 0 {
            j -= 2;
            i += 1;
            continue;
        }
        square[at] = num;
        num += 1;
        j += 1;
        i -= 1;
    }
    let mut data = vec![0u8; count * 4];
    for (k, &value) in square.iter().enumerate() {
        let angle = 2. * PI * value as f64 / count as f64;
        let (x, y) = (angle.cos(), angle.sin());
        let length = (x * x + y * y).sqrt();
        let inv = 1. / length;
        data[k * 4] = ((x * inv * 0.5 + 0.5) * 255.) as u8;
        data[k * 4 + 1] = ((y * inv * 0.5 + 0.5) * 255.) as u8;
        data[k * 4 + 2] = 127;
        data[k * 4 + 3] = 255;
    }
    data
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 50.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(1., 3., 7.);
        let mut controls = Controls::new(Some(0.05), (2., 16.), PI, true);
        controls.set_target(Vector3::new(0., 1.2, 0.));
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // The scene in the page's creation order: geometry, world matrix and material.
        let mut scene: Vec<(BufferGeometry, Matrix4, Standard)> = vec![];
        let cylinder = |top, bottom, height, radial| {
            CylinderGeometry::build(top, bottom, height, radial, 1, false, 0., 2. * PI)
        };
        let mut floor_material = standard(0xffffff, 0.7, 0.05);
        floor_material.map = true;
        scene.push((
            PlaneGeometry::build(16., 16., 1, 1)?,
            compose([0., -2., 0.], [-PI / 2., 0., 0.], [1.; 3]),
            floor_material,
        ));
        let wall = standard(0xe0d8d0, 0.9, 0.);
        scene.push((
            PlaneGeometry::build(16., 10., 1, 1)?,
            at([0., 3., -5.]),
            wall,
        ));
        scene.push((
            PlaneGeometry::build(16., 10., 1, 1)?,
            compose([-5.5, 3., 0.], [0., PI / 2., 0.], [1.; 3]),
            wall,
        ));
        let pedestal = standard(0xf0ece8, 0.4, 0.05);
        scene.push((cylinder(0.9, 1., 0.2, 32)?, at([0., -1.9, 0.]), pedestal));
        scene.push((
            cylinder(0.55, 0.65, 1.5, 32)?,
            at([0., -1.05, 0.]),
            pedestal,
        ));
        scene.push((cylinder(0.8, 0.7, 0.25, 32)?, at([0., -0.18, 0.]), pedestal));
        scene.push((
            TorusKnotGeometry::build(0.5, 0.17, 128, 32, 2, 3)?,
            at([0., 0.82, 0.]),
            standard(0xc0a060, 0.45, 0.),
        ));
        let column = standard(0xe8e4de, 0.5, 0.);
        for (x, z) in [(-5.05, -4.55), (4.5, -4.55), (-5.05, 3.)] {
            let group = compose([x, 1., z], [0.; 3], [1.5; 3]);
            scene.push((
                BoxGeometry::build(0.6, 0.2, 0.6)?,
                group * at([0., -1.9, 0.]),
                column,
            ));
            scene.push((
                cylinder(0.18, 0.22, 5., 16)?,
                group * at([0., 0.7, 0.]),
                column,
            ));
            scene.push((
                BoxGeometry::build(0.55, 0.25, 0.55)?,
                group * at([0., 3.35, 0.]),
                column,
            ));
        }
        scene.push((
            cylinder(0.6, 0.7, 0.15, 32)?,
            at([-5., -1.925, -2.]),
            pedestal,
        ));
        scene.push((
            cylinder(0.35, 0.42, 1., 32)?,
            at([-5., -1.35, -2.]),
            pedestal,
        ));
        scene.push((
            cylinder(0.55, 0.48, 0.15, 32)?,
            at([-5., -0.775, -2.]),
            pedestal,
        ));
        let profile: Vec<Vector2> = [
            (0., 0.5),
            (0.12, 0.5),
            (0.18, 0.7),
            (0.28, 0.9),
            (0.32, 1.),
            (0.3, 1.1),
            (0.22, 1.15),
            (0.2, 1.2),
            (0.22, 1.25),
            (0., 1.25),
        ]
        .iter()
        .map(|&(x, y)| Vector2::new(x, y))
        .collect();
        scene.push((
            LatheGeometry::build(&profile, 24, 0., 2. * PI)?,
            compose([-5., -1.6, -2.], [0.; 3], [1.8; 3]),
            standard(0xd4806a, 0.6, 0.02),
        ));
        let wood = standard(0x8b6840, 0.8, 0.);
        let fabric = standard(0x8b3a3a, 0.9, 0.);
        let chair = compose([4., 0.948, -2.5], [0., -0.6, 0.], [1.76; 3]);
        scene.push((
            rounded_box(0.9, 0.25, 0.8, 4, 0.06)?,
            chair * at([0., -1.35, 0.]),
            fabric,
        ));
        scene.push((
            rounded_box(0.9, 0.6, 0.12, 4, 0.04)?,
            chair * compose([0., -0.95, -0.4], [-0.3, 0., 0.], [1.; 3]),
            fabric,
        ));
        for side in [-1., 1.] {
            scene.push((
                BoxGeometry::build(0.1, 0.25, 0.7)?,
                chair * at([side * 0.5, -1.2, 0.]),
                wood,
            ));
            scene.push((
                BoxGeometry::build(0.14, 0.06, 0.8)?,
                chair * at([side * 0.5, -1.07, 0.]),
                wood,
            ));
        }
        for (lx, lz) in [(-0.38, -0.32), (-0.38, 0.32), (0.38, -0.32), (0.38, 0.32)] {
            scene.push((
                cylinder(0.03, 0.035, 0.2, 8)?,
                chair * at([lx, -1.575, lz]),
                wood,
            ));
        }
        let table = compose([2., 2.29, -4.], [0.; 3], [2.2; 3]);
        scene.push((
            cylinder(0.4, 0.4, 0.05, 24)?,
            table * at([0., -1.05, 0.]),
            wood,
        ));
        scene.push((
            cylinder(0.04, 0.06, 0.9, 8)?,
            table * at([0., -1.5, 0.]),
            wood,
        ));
        scene.push((
            cylinder(0.25, 0.28, 0.06, 24)?,
            table * at([0., -1.92, 0.]),
            wood,
        ));
        scene.push((
            cylinder(0.1, 0.08, 0.2, 16)?,
            table * compose([0.15, -0.98, 0.], [0.; 3], [1. / 2.2; 3]),
            standard(0xf0ece0, 0.4, 0.05),
        ));
        scene.push((
            BoxGeometry::build(6., 0.02, 5.)?,
            at([0., -1.99, 0.5]),
            standard(0xc8a0a8, 0.95, 0.),
        ));
        scene.push((
            BoxGeometry::build(6.3, 0.015, 5.3)?,
            at([0., -1.9925, 0.5]),
            standard(0xd4b880, 0.95, 0.),
        ));
        for (x, y, z, width, height, paint, rotation) in [
            (-3.2, 2., -4.9, 1.8, 1.2, 0xe8a8a0, 0.),
            (-0.5, 2.6, -4.9, 1.1, 1.6, 0xa0c0e0, 0.),
            (2., 1.8, -4.9, 2., 1.4, 0xa0d0a8, 0.),
            (-5.4, 2.2, -3., 1.5, 1.1, 0xd0b0d8, PI / 2.),
            (-5.4, 1.8, 1., 1.8, 1.3, 0xe0c8a0, PI / 2.),
        ] {
            let group = compose([x, y, z], [0., rotation, 0.], [1.; 3]);
            scene.push((
                frame_geometry(width, height)?,
                group,
                standard(0x8b6840, 0.7, 0.),
            ));
            scene.push((
                PlaneGeometry::build(width, height, 1, 1)?,
                group * at([0., 0., 0.001]),
                standard(paint, 0.95, 0.),
            ));
        }
        let (bx, bz) = (-3., -3.2);
        scene.push((
            cylinder(0.6, 0.7, 0.15, 32)?,
            at([bx, -1.925, bz]),
            pedestal,
        ));
        scene.push((cylinder(0.35, 0.42, 1., 32)?, at([bx, -1.35, bz]), pedestal));
        scene.push((
            cylinder(0.55, 0.48, 0.15, 32)?,
            at([bx, -0.775, bz]),
            pedestal,
        ));
        // The bust: rotated π, scaled to 2.64 tall and stood on its pedestal
        // by Box3.setFromObject ( the geometry box's corners ).
        let (asset, buffers, _) = load_asset("/web/gallery/assets/gltf/tennyson-bust.glb").await?;
        let node = asset.nodes().next().ok_or(Error::Invalid("bust node"))?;
        let primitive = node
            .mesh()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("bust mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("bust positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("bust normals"))?
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("bust index"))?
            .into_u32()
            .collect();
        let node_scale = node.transform().decomposed().2;
        let node_matrix = compose(
            [0.; 3],
            [0.; 3],
            [
                node_scale[0] as f64,
                node_scale[1] as f64,
                node_scale[2] as f64,
            ],
        );
        let (lo, hi) = positions.chunks(3).fold(
            ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
            |(lo, hi), p| {
                (
                    [0, 1, 2].map(|k| lo[k].min(p[k] as f64)),
                    [0, 1, 2].map(|k| hi[k].max(p[k] as f64)),
                )
            },
        );
        let world_box = |m: Matrix4| {
            let mut out = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
            for corner in 0..8 {
                let c = Vector3::new(
                    if corner & 1 == 0 { lo[0] } else { hi[0] },
                    if corner & 2 == 0 { lo[1] } else { hi[1] },
                    if corner & 4 == 0 { lo[2] } else { hi[2] },
                );
                let w = m.transform_point3(c).to_array();
                for (k, &v) in w.iter().enumerate() {
                    out.0[k] = out.0[k].min(v);
                    out.1[k] = out.1[k].max(v);
                }
            }
            out
        };
        let rotation = compose([0.; 3], [0., PI, 0.], [1.; 3]);
        let size_box = world_box(rotation * node_matrix);
        let scale = 2.64 / (size_box.1[1] - size_box.0[1]);
        let scaled = compose([0.; 3], [0., PI, 0.], [scale; 3]);
        let fit = world_box(scaled * node_matrix);
        let center = [0, 1, 2].map(|k| (fit.0[k] + fit.1[k]) * 0.5);
        let bust_root = compose(
            [bx - center[0], -0.7 - fit.0[1], bz - center[2] + 0.1],
            [0., PI, 0.],
            [scale; 3],
        );
        scene.push((
            geometry_from(&positions, Some(&normals), Some(index))?,
            bust_root * node_matrix,
            standard(0xffffff, 1., 1.),
        ));
        let read = |g: &BufferGeometry, name: &str, size: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("gtao attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..size).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let mut meshes = vec![];
        for (g, model, material) in scene {
            let positions = read(&g, "position", 3)?;
            let mut buffers = vec![];
            if material.map {
                buffers.push(init(
                    "gtao uv",
                    bytemuck::cast_slice(&read(&g, "uv", 2)?),
                    wgpu::BufferUsages::VERTEX,
                ));
            }
            buffers.push(init(
                "gtao normal",
                bytemuck::cast_slice(&read(&g, "normal", 3)?),
                wgpu::BufferUsages::VERTEX,
            ));
            buffers.push(init(
                "gtao position",
                bytemuck::cast_slice(&positions),
                wgpu::BufferUsages::VERTEX,
            ));
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let (min, max) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (min + max) * 0.5;
            let (index, count) = match &g.index {
                Some(index) => (
                    Some(init(
                        "gtao index",
                        bytemuck::cast_slice(index),
                        wgpu::BufferUsages::INDEX,
                    )),
                    index.len() as u32,
                ),
                None => (None, points.len() as u32),
            };
            let kind = usize::from(material.map);
            let objects =
                [0, 1].map(|pass| uniform(r, "gtao object", SHADERS[pass][kind].1, "objectStruct"));
            let [a, b] = objects;
            meshes.push(Mesh {
                buffers,
                index,
                count,
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
                model,
                material,
                objects: [a?, b?],
            });
        }
        // The checkerboard: 16 × 16 tiles of 32 px drawn on a canvas, sRGB,
        // flipped and mipmapped.
        let mut texels = vec![0u8; 512 * 512 * 4];
        for y in 0..512 {
            for x in 0..512 {
                let (tx, tz) = (x / 32, y / 32);
                let c = if (tx + tz) % 2 == 0 {
                    [0xd8, 0xd0, 0xc8]
                } else {
                    [0xb8, 0xb0, 0xa8]
                };
                // CanvasTexture's flipY: row y of the canvas is row 511 − y.
                let at = ((511 - y) * 512 + x) * 4;
                texels[at..at + 4].copy_from_slice(&[c[0], c[1], c[2], 255]);
            }
        }
        let mut mipmaps = Mipmaps::new(r);
        let floor_map = mipmapped_raw(
            r,
            &mut mipmaps,
            &texels,
            (512, 512),
            4,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        );
        let noise = r
            .device
            .create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some("gtao noise"),
                    size: wgpu::Extent3d {
                        width: 5,
                        height: 5,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &magic_square_noise(),
            )
            .create_view(&Default::default());
        let room = super::room_environment::environment(r)?;
        let environment = room
            .gpu
            .as_ref()
            .ok_or(Error::Invalid("room environment"))?
            .view
            .clone();
        let placeholder = view(&texture(r, (1, 1), DEPTH));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("gtao placeholder"),
            depth_stencil_attachment: depth(&placeholder),
            ..Default::default()
        });
        r.queue.submit([encoder.finish()]);
        let sampler = |address, mag, mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: mag,
                min_filter: mag,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        use wgpu::{AddressMode::*, FilterMode::*};
        let renders = [0, 1].map(|pass| {
            [0, 1].map(|kind| uniform(r, "gtao render", SHADERS[pass][kind].1, "renderStruct"))
        });
        let [[a, b], [c2, d]] = renders;
        Ok(Self {
            controls,
            params: [
                0., 0.5, 16., 0.4, 0.8, 1., 1., 2., 0.025, 1., 2., 0., 0.3, 0.,
            ],
            pending: true,
            loaded: false,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            meshes,
            floor_map,
            noise,
            environment,
            renders: [[a?, b?], [c2?, d?]],
            gtao_object: uniform(r, "gtao", wgsl!("gtao_fs"), "objectStruct")?,
            traa_object: uniform(r, "gtao traa", wgsl!("traa_fs"), "objectStruct")?,
            output_render: uniform(r, "gtao output", OUTPUT_FS, "renderStruct")?,
            ao_only_render: uniform(r, "gtao ao only", wgsl!("ao_only_fs"), "renderStruct")?,
            quad_uv: init(
                "gtao quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            placeholder,
            linear: sampler(ClampToEdge, Linear, Nearest),
            nearest: sampler(ClampToEdge, Nearest, Nearest),
            repeat: sampler(Repeat, Linear, Linear),
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
    fn mesh_pipeline(r: &Renderer, pass: usize, kind: usize) -> wgpu::RenderPipeline {
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![2 => Float32x3],
        ];
        let layout = |stride, attributes| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
        let layouts: Vec<wgpu::VertexBufferLayout> = if kind == 1 {
            vec![
                layout(8, &attrs[0][..]),
                layout(12, &attrs[2][..]),
                layout(12, &attrs[4][..]),
            ]
        } else {
            vec![layout(12, &attrs[1][..]), layout(12, &attrs[3][..])]
        };
        let formats: &[wgpu::TextureFormat] = if pass == 0 { &[HALF, HALF] } else { &[HALF] };
        sampled_pipeline(
            r,
            "gtao mesh",
            SHADERS[pass][kind],
            &layouts,
            formats,
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        )
    }
    /// The GTAO target for a resolution scale ( setSize rounds the scaled
    /// drawing buffer ), with the scene draws and AO-only output reading it.
    fn ao_target(&self, r: &Renderer, t: &Targets, scale: f64) -> (AoTarget, Draw) {
        let size = (
            ((t.width as f64 * scale).round() as u32).max(1),
            ((t.height as f64 * scale).round() as u32).max(1),
        );
        let ao = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let scene_pipelines = [Self::mesh_pipeline(r, 1, 0), Self::mesh_pipeline(r, 1, 1)];
        let scene = self
            .meshes
            .iter()
            .map(|m| {
                let kind = usize::from(m.material.map);
                let p = &scene_pipelines[kind];
                let mut entries = vec![(0, m.objects[1].as_entire_binding())];
                let mut binding = 1;
                if kind == 1 {
                    entries.push((1, sampler(&self.repeat)));
                    entries.push((2, tex(&self.floor_map)));
                    binding = 3;
                }
                for resource in [&ao, &r.dfg, &self.environment] {
                    entries.push((binding, sampler(&self.linear)));
                    entries.push((binding + 1, tex(resource)));
                    binding += 2;
                }
                (
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, self.renders[1][kind].as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries),
                    ],
                )
            })
            .collect();
        let ao_only_pipeline = sampled_pipeline(
            r,
            "gtao ao only",
            (FULLSCREEN_VS, wgsl!("ao_only_fs")),
            &[],
            &[t.format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let ao_only = (
            ao_only_pipeline.clone(),
            vec![
                bind(
                    r,
                    ao_only_pipeline.get_bind_group_layout(0),
                    &[(0, self.ao_only_render.as_entire_binding())],
                ),
                bind(
                    r,
                    ao_only_pipeline.get_bind_group_layout(1),
                    &[(0, sampler(&self.linear)), (1, tex(&ao))],
                ),
            ],
        );
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];
        let gtao_pipeline = sampled_pipeline(
            r,
            "gtao",
            (wgsl!("gtao_vs"), wgsl!("gtao_fs")),
            &quad,
            &[wgpu::TextureFormat::R8Unorm],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let gtao = (
            gtao_pipeline.clone(),
            vec![bind(
                r,
                gtao_pipeline.get_bind_group_layout(0),
                &[
                    (0, self.gtao_object.as_entire_binding()),
                    (1, sampler(&self.nearest)),
                    (2, tex(&t.prepass_depth.1)),
                    (3, sampler(&self.linear)),
                    (4, tex(&t.prepass[0].1)),
                    (5, tex(&self.noise)),
                ],
            )],
        );
        (
            AoTarget {
                size,
                view: ao,
                scene,
                ao_only,
            },
            gtao,
        )
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height);
        let make = |format| {
            let t = texture(r, size, format);
            let v = view(&t);
            (t, v)
        };
        let prepass = [make(HALF), make(HALF)];
        let prepass_depth = make(DEPTH);
        let scene = make(HALF);
        let scene_depth = view(&texture(r, size, DEPTH));
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let prepass_pipelines = [Self::mesh_pipeline(r, 0, 0), Self::mesh_pipeline(r, 0, 1)];
        let prepass_draws = self
            .meshes
            .iter()
            .map(|m| {
                let kind = usize::from(m.material.map);
                let p = &prepass_pipelines[kind];
                let mut entries = vec![(0, m.objects[0].as_entire_binding())];
                let mut binding = 1;
                if kind == 1 {
                    entries.push((1, sampler(&self.repeat)));
                    entries.push((2, tex(&self.floor_map)));
                    binding = 3;
                }
                for resource in [&r.dfg, &self.environment] {
                    entries.push((binding, sampler(&self.linear)));
                    entries.push((binding + 1, tex(resource)));
                    binding += 2;
                }
                (
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, self.renders[0][kind].as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries),
                    ],
                )
            })
            .collect();
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
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
        let traa_pipeline = screen_pass("gtao traa", wgsl!("traa_vs"), wgsl!("traa_fs"), &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_groups = [&self.placeholder, &history_depth_view].map(|previous| {
            bind(
                r,
                traa_pipeline.get_bind_group_layout(0),
                &[
                    (1, tex(&prepass[1].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&scene.1)),
                    (4, tex(&prepass_depth.1)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(previous)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        });
        let output_pipeline =
            screen_pass("gtao output", OUTPUT_VS, OUTPUT_FS, &[out.options.format]);
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
            format: out.options.format,
            prepass,
            prepass_depth,
            scene,
            scene_depth,
            resolve,
            history,
            history_depth,
            screen,
            prepass_draws,
            ao: vec![],
            traa: (traa_pipeline, traa_groups),
            output,
        });
        Ok(())
    }
    fn ao_index(&self) -> usize {
        self.targets
            .as_ref()
            .and_then(|t| t.ao.iter().position(|(s, _, _)| *s == self.params[1]))
            .unwrap_or(0)
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("gtao output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        if self.params[13] > 0.5 {
            set(&mut pass, &t.ao[self.ao_index()].1.ao_only);
            pass.draw(0..3, 0..1);
        } else {
            set(&mut pass, &t.output);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
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
        // The AO target of this resolution scale ( kept for revisits ).
        let scale = self.params[1];
        let missing = self
            .targets
            .as_ref()
            .is_some_and(|t| !t.ao.iter().any(|(s, _, _)| *s == scale));
        if missing {
            let t = self
                .targets
                .as_ref()
                .ok_or(Error::Invalid("gtao targets"))?;
            let (target, draw) = self.ao_target(r, t, scale);
            if let Some(t) = self.targets.as_mut() {
                t.ao.push((scale, target, draw));
            }
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("gtao targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::replace(&mut self.loaded, true) {
            self.frame_id += 1;
        }
        // animate(): the damped controls, then the pipeline.
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let ao_only = self.params[13] > 0.5;
        let traa_on = !ao_only;
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
        let spot_color = (Color::from_hex(0xffe09e).0 * 40.).to_array().to_vec();
        // Per pass and material: the spot uniforms ( cone, penumbra,
        // distance, decay, colour ) and positions ( view, world, target ),
        // and the extra names.
        // The first light's block leaves one number free.
        let numbering = |first: usize| {
            let n = |k: usize| format!("nodeUniform{k}");
            let (b, c) = (first + 9, first + 17);
            (
                [
                    [
                        n(first + 2),
                        n(first + 3),
                        n(first + 7),
                        n(first + 8),
                        n(first + 1),
                    ],
                    [n(b + 2), n(b + 3), n(b + 6), n(b + 7), n(b + 1)],
                    [n(c + 2), n(c + 3), n(c + 6), n(c + 7), n(c + 1)],
                ],
                [
                    [n(first), n(first + 5), n(first + 6)],
                    [n(b), n(b + 4), n(b + 5)],
                    [n(c), n(c + 4), n(c + 5)],
                ],
            )
        };
        let size = vec![t.width as f64, t.height as f64];
        for (pass, first) in [(0, [10, 12]), (1, [12, 14])] {
            for kind in 0..2 {
                let (spots, places) = numbering(first[kind]);
                let mut values: Vec<(String, Vec<f64>)> = vec![
                    ("cameraProjectionMatrix".into(), m4(jittered)),
                    ("cameraViewMatrix".into(), m4(view)),
                    ("cameraWorldMatrix".into(), m4(world)),
                ];
                for (i, (position, target)) in SPOTS.iter().enumerate() {
                    let p = Vector3::from_array(*position);
                    values.push((spots[i][0].clone(), vec![0.6f64.cos()]));
                    values.push((spots[i][1].clone(), vec![1.]));
                    values.push((spots[i][2].clone(), vec![6.5]));
                    values.push((spots[i][3].clone(), vec![2.]));
                    values.push((spots[i][4].clone(), spot_color.clone()));
                    values.push((
                        places[i][0].clone(),
                        view.transform_point3(p).to_array().to_vec(),
                    ));
                    values.push((places[i][1].clone(), position.to_vec()));
                    values.push((places[i][2].clone(), target.to_vec()));
                }
                if pass == 0 {
                    values.push((
                        format!("nodeUniform{}", [44, 46][kind]),
                        m4(motion.projection),
                    ));
                } else {
                    values.push((format!("nodeUniform{}", [3, 5][kind]), size.clone()));
                }
                let values: Vec<(&str, Vec<f64>)> = values
                    .iter()
                    .map(|(n, v)| (n.as_str(), v.clone()))
                    .collect();
                write(
                    &self.renders[pass][kind],
                    SHADERS[pass][kind].1,
                    "renderStruct",
                    &values,
                )?;
            }
        }
        let id3 = m3(Matrix4::IDENTITY);
        let id4 = m4(Matrix4::IDENTITY);
        for m in &self.meshes {
            let normal = m3(m.model.inverse().transpose());
            let mat = m.material;
            if mat.map {
                let common = |metal: usize, normal_at: usize, model_at: usize, env: usize| {
                    vec![
                        ("nodeUniform0", mat.color.to_vec()),
                        ("nodeUniform2", id3.clone()),
                        ("nodeUniform3", vec![1.]),
                        (["nodeUniform4", "nodeUniform6"][metal], vec![mat.metalness]),
                        (["nodeUniform5", "nodeUniform7"][metal], vec![mat.roughness]),
                        (["nodeUniform7", "nodeUniform9"][normal_at], normal.clone()),
                        (["nodeUniform8", "nodeUniform10"][normal_at], vec![0.; 3]),
                        (["nodeUniform9", "nodeUniform11"][normal_at], vec![1.]),
                        (["nodeUniform11", "nodeUniform13"][model_at], m4(m.model)),
                        (["nodeUniform37", "nodeUniform39"][env], vec![8.]),
                        (["nodeUniform38", "nodeUniform40"][env], id4.clone()),
                        (["nodeUniform40", "nodeUniform42"][env], vec![1. / 768.]),
                        (["nodeUniform41", "nodeUniform43"][env], vec![1. / 1024.]),
                        (["nodeUniform43", "nodeUniform45"][env], vec![0.3]),
                    ]
                };
                let mut pre = common(0, 0, 0, 0);
                pre.extend([
                    ("nodeUniform44", m4(projection)),
                    ("nodeUniform45", m4(m.model)),
                    ("nodeUniform47", m4(motion.view)),
                    ("nodeUniform48", m4(m.model)),
                ]);
                write(&m.objects[0], SHADERS[0][1].1, "objectStruct", &pre)?;
                write(
                    &m.objects[1],
                    SHADERS[1][1].1,
                    "objectStruct",
                    &common(1, 1, 1, 1),
                )?;
            } else {
                write(
                    &m.objects[0],
                    SHADERS[0][0].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", mat.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform2", vec![mat.metalness]),
                        ("nodeUniform3", vec![mat.roughness]),
                        ("nodeUniform5", normal.clone()),
                        ("nodeUniform6", vec![0.; 3]),
                        ("nodeUniform7", vec![1.]),
                        ("nodeUniform9", m4(m.model)),
                        ("nodeUniform35", vec![8.]),
                        ("nodeUniform36", id4.clone()),
                        ("nodeUniform38", vec![1. / 768.]),
                        ("nodeUniform39", vec![1. / 1024.]),
                        ("nodeUniform41", vec![0.3]),
                        ("nodeUniform42", m4(projection)),
                        ("nodeUniform43", m4(m.model)),
                        ("nodeUniform45", m4(motion.view)),
                        ("nodeUniform46", m4(m.model)),
                    ],
                )?;
                write(
                    &m.objects[1],
                    SHADERS[1][0].1,
                    "objectStruct",
                    &[
                        ("nodeUniform0", mat.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform4", vec![mat.metalness]),
                        ("nodeUniform5", vec![mat.roughness]),
                        ("nodeUniform7", normal),
                        ("nodeUniform8", vec![0.; 3]),
                        ("nodeUniform9", vec![1.]),
                        ("nodeUniform11", m4(m.model)),
                        ("nodeUniform37", vec![8.]),
                        ("nodeUniform38", id4.clone()),
                        ("nodeUniform40", vec![1. / 768.]),
                        ("nodeUniform41", vec![1. / 1024.]),
                        ("nodeUniform43", vec![0.3]),
                    ],
                )?;
            }
        }
        let ao = &t.ao[self.ao_index()];
        let temporal = self.params[6] > 0.5;
        let (direction, offset) = if temporal {
            (
                TEMPORAL_ROTATIONS[self.frame_id % 6] / 360.,
                SPATIAL_OFFSETS[self.frame_id % 4],
            )
        } else {
            (0., 1.)
        };
        write(
            &self.gtao_object,
            wgsl!("gtao_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![scale]),
                ("nodeUniform2", m4(jittered.inverse())),
                ("nodeUniform4", vec![self.params[3]]),
                ("nodeUniform5", m4(jittered)),
                ("nodeUniform6", vec![direction]),
                ("nodeUniform8", id3.clone()),
                ("nodeUniform9", vec![ao.1.size.0 as f64, ao.1.size.1 as f64]),
                ("nodeUniform10", vec![offset]),
                ("nodeUniform11", vec![self.params[5]]),
                ("nodeUniform12", vec![self.params[4]]),
            ],
        )?;
        write(
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[("nodeUniform1", vec![1.])],
        )?;
        write(
            &self.ao_only_render,
            wgsl!("ao_only_fs"),
            "renderStruct",
            &[("nodeUniform1", size.clone())],
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
                wgsl!("traa_fs"),
                "objectStruct",
                &[
                    ("nodeUniform3", vec![0.1, 50.]),
                    ("nodeUniform4", m4(view)),
                    ("nodeUniform5", m4(previous.world)),
                    ("nodeUniform6", m4(previous.projection_inverse)),
                    ("nodeUniform8", id3.clone()),
                    ("nodeUniform10", id3),
                    ("nodeUniform11", vec![1.]),
                ],
            )?;
        }
        let frustum = Frustum::from_projection(jittered * view);
        // painterSortStable: front to back by clip-space z, then creation order.
        let projection_view = jittered * view;
        let mut order: Vec<(f64, usize)> = self
            .meshes
            .iter()
            .enumerate()
            .filter(|(_, m)| m.visible(&frustum))
            .map(|(i, m)| ((projection_view * m.model.w_axis).z, i))
            .collect();
        order.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let background = wgpu::Color {
            r: BACKGROUND,
            g: BACKGROUND,
            b: BACKGROUND,
            a: 1.,
        };
        let black = wgpu::Color::BLACK;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gtao pre-pass"),
                color_attachments: &[
                    color(&t.prepass[0].1, background),
                    color(&t.prepass[1].1, black),
                ],
                depth_stencil_attachment: depth(&t.prepass_depth.1),
                ..Default::default()
            });
            for &(_, i) in &order {
                set(&mut pass, &t.prepass_draws[i]);
                self.meshes[i].draw(&mut pass);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gtao"),
                color_attachments: &[color(&ao.1.view, wgpu::Color::WHITE)],
                ..Default::default()
            });
            set(&mut pass, &ao.2);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        if traa_on {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("gtao scene"),
                    color_attachments: &[color(&t.scene.1, background)],
                    depth_stencil_attachment: depth(&t.scene_depth),
                    ..Default::default()
                });
                for &(_, i) in &order {
                    set(&mut pass, &ao.1.scene[i]);
                    self.meshes[i].draw(&mut pass);
                }
            }
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
            let (pipeline, groups) = &t.traa;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("gtao traa"),
                    color_attachments: &[color(&t.resolve.1, black)],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &groups[usize::from(self.resolves >= 2)], &[]);
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
        Ok(())
    }
    /// AO type, resolution, samples, radius, scale, thickness, temporal
    /// filtering, the SSAO settings, the transparent mesh and AO only. SSAO,
    /// the sample count ( a shader constant ) and the transparent mesh are
    /// not reproduced.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 | 2 | 7..=12 => return Err(Error::Invalid("gtao parameter not reproduced")),
            // updateOutput rebuilds the output node and TRAA.
            13 => self.built = false,
            _ => {}
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("gtao parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
