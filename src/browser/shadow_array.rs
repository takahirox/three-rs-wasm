//! webgpu_shadowmap_array: a field of instanced columns, cubes, spheres and
//! tori, a batched forest and a turning torus knot on a noise-coloured
//! ground, lit by an orbiting directional light whose shadow TileShadowNode
//! splits into 2 × 2 orthographic tiles of a 4096² depth array. Each frame
//! the tile cameras follow the light, each layer renders the shadow casters
//! for its tile camera ( three.js r186 records a render bundle per layer )
//! and the scene reads the minimum of the four tile lookups. The batched trees sort their draws front to back per camera on
//! the CPU into the indirect texture, as BatchedMesh does. The
//! TileShadowNodeHelper shows each layer as a screen overlay and each tile
//! camera's frustum as a CameraHelper, one frame behind as the page updates
//! it after rendering. Every stage runs the WGSL three.js r186 generates
//! for the page (in `shadow_array/`; the output is the cubemap_dynamic
//! module, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 4096;
const TILES: usize = 4;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("shadow_array/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
const TILE_SHADERS: [(&str, &str); 4] = [
    (wgsl!("tile0_vs"), wgsl!("tile0_fs")),
    (wgsl!("tile1_vs"), wgsl!("tile1_fs")),
    (wgsl!("tile2_vs"), wgsl!("tile2_fs")),
    (wgsl!("tile3_vs"), wgsl!("tile3_fs")),
];
/// The page's Math.random: the fixture's seeded LCG.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
/// Matrix4 in three.js's row-major `set` order.
fn rows(e: [f64; 16]) -> Matrix4 {
    Matrix4::from_cols_array(&[
        e[0], e[4], e[8], e[12], e[1], e[5], e[9], e[13], e[2], e[6], e[10], e[14], e[3], e[7],
        e[11], e[15],
    ])
}
fn rotation_x(t: f64) -> Matrix4 {
    let (s, c) = t.sin_cos();
    rows([1., 0., 0., 0., 0., c, -s, 0., 0., s, c, 0., 0., 0., 0., 1.])
}
fn rotation_y(t: f64) -> Matrix4 {
    let (s, c) = t.sin_cos();
    rows([c, 0., s, 0., 0., 1., 0., 0., -s, 0., c, 0., 0., 0., 0., 1.])
}
fn rotation_z(t: f64) -> Matrix4 {
    let (s, c) = t.sin_cos();
    rows([c, -s, 0., 0., s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.])
}
/// Matrix4.setPosition.
fn with_position(mut m: Matrix4, p: [f64; 3]) -> Matrix4 {
    m.w_axis = Vector4::new(p[0], p[1], p[2], 1.);
    m
}
/// compose( position, Euler XYZ, scale ).
fn compose(p: [f64; 3], r: [f64; 3], s: [f64; 3]) -> Matrix4 {
    let (c1, c2, c3) = ((r[0] / 2.).cos(), (r[1] / 2.).cos(), (r[2] / 2.).cos());
    let (s1, s2, s3) = ((r[0] / 2.).sin(), (r[1] / 2.).sin(), (r[2] / 2.).sin());
    let q = Quaternion::from_xyzw(
        s1 * c2 * c3 + c1 * s2 * s3,
        c1 * s2 * c3 - s1 * c2 * s3,
        c1 * c2 * s3 + s1 * s2 * c3,
        c1 * c2 * c3 - s1 * s2 * s3,
    );
    Matrix4::from_scale_rotation_translation(Vector3::from_array(s), q, Vector3::from_array(p))
}
/// Vertex buffers ( in each pipeline's slot order ) and index of a geometry.
struct Geometry {
    position: wgpu::Buffer,
    normal: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
}
fn upload(r: &Renderer, g: &BufferGeometry) -> Result<Geometry> {
    let read = |name: &str| -> Result<Vec<f32>> {
        let a = g
            .attributes
            .get(name)
            .ok_or(Error::Invalid("shadow array attribute"))?;
        (0..a.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    let init = |data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shadow array"),
                contents: data,
                usage,
            })
    };
    let positions = read("position")?;
    let index = g
        .index
        .clone()
        .ok_or(Error::Invalid("shadow array index"))?;
    Ok(Geometry {
        sphere: bounds(&positions),
        position: init(bytemuck::cast_slice(&positions), wgpu::BufferUsages::VERTEX),
        normal: init(
            bytemuck::cast_slice(&read("normal")?),
            wgpu::BufferUsages::VERTEX,
        ),
        index: init(bytemuck::cast_slice(&index), wgpu::BufferUsages::INDEX),
        count: index.len() as u32,
    })
}
fn bounds(positions: &[f32]) -> Sphere {
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
    Sphere {
        center,
        radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
    }
}
/// An InstancedMesh with a Phong material: the shared shading uniforms,
/// its instance matrices and its main and shadow draws.
struct Instanced {
    geometry: Geometry,
    count: u32,
    color: u32,
    shininess: f64,
    instances: wgpu::Buffer,
    object: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    vs: &'static str,
    shadow_vs: &'static str,
    /// The instances' world bounding sphere ( InstancedMesh.computeBoundingSphere ).
    sphere: Sphere,
}
/// An instanced group: its geometry, matrices, colour, roughness and stages.
type InstancedSpec = (
    BufferGeometry,
    Vec<Matrix4>,
    u32,
    f64,
    &'static str,
    &'static str,
);
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    background: Draw,
    knot: Draw,
    ground: Draw,
    instanced: Vec<Draw>,
    trees: Draw,
    helpers: Vec<Draw>,
    tiles: Vec<Draw>,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    time: f64,
    last: f64,
    knot_rotation: [f64; 3],
    background_mesh: Geometry,
    knot: Geometry,
    ground: Geometry,
    instanced: Vec<Instanced>,
    tree_geometry: Geometry,
    /// Per tree instance: geometry ( 0 trunk, 1 top ), world bounding sphere.
    tree_instances: Vec<(usize, Sphere)>,
    /// geometryInfo: the index start and count of the trunk and the top.
    tree_ranges: [(u32, u32); 2],
    tree_indirect: [wgpu::Texture; 2],
    tree_matrices: wgpu::TextureView,
    tree_colors: wgpu::TextureView,
    quad: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer),
    helper_geometry: (wgpu::Buffer, wgpu::Buffer),
    /// The helpers' world matrices of the previous frame ( None before the
    /// first TileShadowNodeHelper.update ).
    helper_models: Option<Vec<Matrix4>>,
    shadow_color: wgpu::Texture,
    shadow_depth: wgpu::Texture,
    shadow_array: wgpu::TextureView,
    shadow_projections: wgpu::Buffer,
    shadow_views: wgpu::Buffer,
    shadow_draws: Vec<Draw>,
    knot_shadow_object: wgpu::Buffer,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    knot_render: wgpu::Buffer,
    knot_object: wgpu::Buffer,
    ground_render: wgpu::Buffer,
    ground_object: wgpu::Buffer,
    instance_render: wgpu::Buffer,
    tree_render: wgpu::Buffer,
    tree_object: wgpu::Buffer,
    helper_render: wgpu::Buffer,
    helper_objects: Vec<wgpu::Buffer>,
    tile_renders: Vec<wgpu::Buffer>,
    tile_objects: Vec<wgpu::Buffer>,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    compare: wgpu::Sampler,
    linear: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The light's shadow camera and the tiles of TileShadowNode.generateTiles.
fn tile_projections() -> Vec<Matrix4> {
    let (left, right, top, bottom, near, far) = (-180., 180., 180., -160., 1., 200.);
    let (width, height) = (right - left, top - bottom);
    (0..TILES)
        .map(|i| {
            let (x, y) = (i % 2, i / 2);
            let (x0, x1) = (x as f64 * 0.5, (x + 1) as f64 * 0.5);
            let (y0, y1) = ((1 - y) as f64 * 0.5, (2 - y) as f64 * 0.5);
            let (l, r) = (left + x0 * width, left + x1 * width);
            let (b, t) = (bottom + y0 * height, bottom + y1 * height);
            // OrthographicCamera.updateProjectionMatrix ( WebGPU depth ).
            let (w, h, p) = (1. / (r - l), 1. / (t - b), 1. / (far - near));
            rows([
                2. * w,
                0.,
                0.,
                -(r + l) * w,
                0.,
                2. * h,
                0.,
                -(t + b) * h,
                0.,
                0.,
                -p,
                -near * p,
                0.,
                0.,
                0.,
                1.,
            ])
        })
        .collect()
}
/// CameraHelper's 50 points for an orthographic tile camera, unprojected
/// in its local space ( WebGPU depth: near 0, far 1 ).
fn helper_points(projection: Matrix4) -> Vec<f32> {
    let inverse = projection.inverse();
    let lines = [
        ("n1", "n2"),
        ("n2", "n4"),
        ("n4", "n3"),
        ("n3", "n1"),
        ("f1", "f2"),
        ("f2", "f4"),
        ("f4", "f3"),
        ("f3", "f1"),
        ("n1", "f1"),
        ("n2", "f2"),
        ("n3", "f3"),
        ("n4", "f4"),
        ("p", "n1"),
        ("p", "n2"),
        ("p", "n3"),
        ("p", "n4"),
        ("u1", "u2"),
        ("u2", "u3"),
        ("u3", "u1"),
        ("c", "t"),
        ("p", "c"),
        ("cn1", "cn2"),
        ("cn3", "cn4"),
        ("cf1", "cf2"),
        ("cf3", "cf4"),
    ];
    let point = |id: &str| -> [f64; 3] {
        let (x, y, z) = match id {
            "c" => (0., 0., 0.),
            "t" => (0., 0., 1.),
            "n1" => (-1., -1., 0.),
            "n2" => (1., -1., 0.),
            "n3" => (-1., 1., 0.),
            "n4" => (1., 1., 0.),
            "f1" => (-1., -1., 1.),
            "f2" => (1., -1., 1.),
            "f3" => (-1., 1., 1.),
            "f4" => (1., 1., 1.),
            "u1" => (0.7, 1.1, 0.),
            "u2" => (-0.7, 1.1, 0.),
            "u3" => (0., 2., 0.),
            "cf1" => (-1., 0., 1.),
            "cf2" => (1., 0., 1.),
            "cf3" => (0., -1., 1.),
            "cf4" => (0., 1., 1.),
            "cn1" => (-1., 0., 0.),
            "cn2" => (1., 0., 0.),
            "cn3" => (0., -1., 0.),
            "cn4" => (0., 1., 0.),
            // "p" is never set: it stays at the origin.
            _ => return [0.; 3],
        };
        inverse.project_point3(Vector3::new(x, y, z)).to_array()
    };
    lines
        .iter()
        .flat_map(|(a, b)| [point(a), point(b)])
        .flat_map(|p| p.map(|v| v as f32))
        .collect()
}
fn helper_colors() -> Vec<f32> {
    let c = |hex: u32| Color::from_hex(hex).0.to_array().map(|v| v as f32);
    let (frustum, cone, up, target, cross) = (
        c(0xffaa00),
        c(0xff0000),
        c(0x00aaff),
        c(0xffffff),
        c(0x333333),
    );
    let mut out = vec![];
    for i in 0..50 {
        let color = match i {
            0..=23 => frustum,
            24..=31 => cone,
            32..=37 => up,
            38..=39 => target,
            _ => cross,
        };
        out.extend(color);
    }
    out
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(45., 60., 100.);
        let mut controls = Controls::new(None, (0.01, 400.), PI / 2. - 0.1, true);
        controls.set_target(Vector3::new(0., 5., 0.));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // createScenery(): the page's Math.random draws, in order.
        let mut random = Random(186);
        let mut columns = vec![];
        let mut x = -100.;
        while x <= 100. {
            let mut z = -100.;
            while z <= 100. {
                if random.next() > 0.3 {
                    let height = 5. + random.next() * 10.;
                    let px = x + (random.next() * 10. - 5.);
                    let pz = z + (random.next() * 10. - 5.);
                    columns.push(with_position(
                        Matrix4::from_scale(Vector3::new(1., height, 1.)),
                        [px, height / 2., pz],
                    ));
                }
                z += 40.;
            }
            x += 40.;
        }
        let mut cubes = [vec![], vec![], vec![]];
        for i in 0..30 {
            let x = random.next() * 300. - 150.;
            let z = random.next() * 300. - 150.;
            let rotation = random.next() * PI * 2.;
            cubes[i % 3].push(with_position(rotation_y(rotation), [x, 1.5, z]));
        }
        let spheres: Vec<Matrix4> = (0..25)
            .map(|_| {
                let x = random.next() * 180. - 90.;
                let z = random.next() * 180. - 90.;
                with_position(Matrix4::IDENTITY, [x, 2., z])
            })
            .collect();
        let mut trees = vec![];
        for _ in 0..40 {
            let x = random.next() * 300. - 150.;
            let z = random.next() * 300. - 150.;
            trees.push((0, with_position(Matrix4::IDENTITY, [x, 1., z])));
            trees.push((1, with_position(Matrix4::IDENTITY, [x, 6., z])));
        }
        let tori: Vec<Matrix4> = (0..15)
            .map(|_| {
                let x = random.next() * 320. - 160.;
                let z = random.next() * 320. - 160.;
                let rotation = random.next() * PI * 2.;
                with_position(rotation_x(PI / 2.) * rotation_z(rotation), [x, 2., z])
            })
            .collect();
        let instanced_specs: [InstancedSpec; 6] = [
            (
                CylinderGeometry::build(0.8, 1., 1., 16, 1, false, 0., 2. * PI)?,
                columns,
                0xdddddd,
                20.,
                wgsl!("column_vs"),
                wgsl!("column_shadow_vs"),
            ),
            (
                BoxGeometry::build(3., 3., 3.)?,
                cubes[0].clone(),
                0x6699cc,
                20.,
                wgsl!("cube0_vs"),
                wgsl!("cube0_shadow_vs"),
            ),
            (
                BoxGeometry::build(3., 3., 3.)?,
                cubes[1].clone(),
                0xcc6666,
                20.,
                wgsl!("cube1_vs"),
                wgsl!("cube1_shadow_vs"),
            ),
            (
                BoxGeometry::build(3., 3., 3.)?,
                cubes[2].clone(),
                0xcccc66,
                20.,
                wgsl!("cube2_vs"),
                wgsl!("cube2_shadow_vs"),
            ),
            (
                SphereGeometry::build(2., 32, 32)?,
                spheres,
                0x88ccaa,
                40.,
                wgsl!("sphere_vs"),
                wgsl!("sphere_shadow_vs"),
            ),
            (
                TorusGeometry::build(3., 1., 16, 50, 2. * PI, 0., 2. * PI)?,
                tori,
                0xff99cc,
                30.,
                wgsl!("torus_vs"),
                wgsl!("torus_shadow_vs"),
            ),
        ];
        let mut instanced = vec![];
        for (g, matrices, color, shininess, vs, shadow_vs) in instanced_specs {
            let geometry = upload(r, &g)?;
            let data: Vec<f32> = matrices
                .iter()
                .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                .collect();
            let (lo, hi) = matrices.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), m| {
                    let c = m.transform_point3(geometry.sphere.center);
                    (lo.min(c), hi.max(c))
                },
            );
            let center = (lo + hi) * 0.5;
            let radius = matrices
                .iter()
                .map(|m| {
                    let scale = m.to_scale_rotation_translation().0.abs().max_element();
                    m.transform_point3(geometry.sphere.center).distance(center)
                        + geometry.sphere.radius * scale
                })
                .fold(0., f64::max);
            instanced.push(Instanced {
                geometry,
                count: matrices.len() as u32,
                color,
                shininess,
                instances: init(
                    "shadow array instances",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::UNIFORM,
                ),
                object: uniform(
                    r,
                    "shadow array instanced",
                    wgsl!("instance_fs"),
                    "objectStruct",
                )?,
                shadow_object: uniform(
                    r,
                    "shadow array instanced shadow",
                    wgsl!("instance_shadow_fs"),
                    "objectStruct",
                )?,
                vs,
                shadow_vs,
                sphere: Sphere { center, radius },
            });
        }
        // The batched trees: trunk then top geometry, 80 instances.
        let trunk = CylinderGeometry::build(0.5, 0.5, 2., 8, 1, false, 0., 2. * PI)?;
        let top = CylinderGeometry::build(0., 2., 8., 8, 1, false, 0., 2. * PI)?;
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("tree attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let (mut positions, mut normals) = (read(&trunk, "position")?, read(&trunk, "normal")?);
        let trunk_vertices = (positions.len() / 3) as u32;
        let trunk_index = trunk.index.clone().ok_or(Error::Invalid("trunk index"))?;
        let top_index = top.index.clone().ok_or(Error::Invalid("top index"))?;
        let top_positions = read(&top, "position")?;
        positions.extend(&top_positions);
        normals.extend(read(&top, "normal")?);
        let mut index = trunk_index.clone();
        index.extend(top_index.iter().map(|i| i + trunk_vertices));
        let tree_ranges = [
            (0, trunk_index.len() as u32),
            (trunk_index.len() as u32, top_index.len() as u32),
        ];
        let tree_spheres = [bounds(&read(&trunk, "position")?), bounds(&top_positions)];
        let tree_geometry = Geometry {
            sphere: bounds(&positions),
            position: init(
                "tree positions",
                bytemuck::cast_slice(&positions),
                wgpu::BufferUsages::VERTEX,
            ),
            normal: init(
                "tree normals",
                bytemuck::cast_slice(&normals),
                wgpu::BufferUsages::VERTEX,
            ),
            index: init(
                "tree index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
        };
        // BatchedMesh's matrices ( 20² RGBA32F ), colours and indirect ids ( 9² ).
        let mut matrix_texels = vec![0f32; 20 * 20 * 4];
        let mut color_texels = vec![1f32; 9 * 9 * 4];
        let (trunk_color, top_color) = (Color::from_hex(0x8b4513).0, Color::from_hex(0x336633).0);
        for (i, (kind, m)) in trees.iter().enumerate() {
            matrix_texels[i * 16..i * 16 + 16]
                .copy_from_slice(&m.to_cols_array().map(|v| v as f32));
            let color = if *kind == 0 { trunk_color } else { top_color };
            color_texels[i * 4..i * 4 + 3].copy_from_slice(&color.to_array().map(|v| v as f32));
        }
        let data_texture = |label, size: u32, format, data: &[u8]| {
            r.device.create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size,
                        height: size,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                data,
            )
        };
        let tree_matrices = data_texture(
            "tree matrices",
            20,
            wgpu::TextureFormat::Rgba32Float,
            bytemuck::cast_slice(&matrix_texels),
        )
        .create_view(&Default::default());
        let tree_colors = data_texture(
            "tree colors",
            9,
            wgpu::TextureFormat::Rgba32Float,
            bytemuck::cast_slice(&color_texels),
        )
        .create_view(&Default::default());
        let indirect = || {
            data_texture(
                "tree indirect",
                9,
                wgpu::TextureFormat::R32Uint,
                bytemuck::cast_slice(&[0u32; 81]),
            )
        };
        let tree_indirect = [indirect(), indirect()];
        let tree_instances = trees
            .iter()
            .map(|(kind, m)| {
                (
                    *kind,
                    Sphere {
                        center: m.transform_point3(tree_spheres[*kind].center),
                        radius: tree_spheres[*kind].radius,
                    },
                )
            })
            .collect();
        let mut plane = PlaneGeometry::build(1., 1., 1, 1)?;
        let read2 = |g: &BufferGeometry, name: &str, size: usize| -> Result<Vec<f32>> {
            let a = g.attributes.get(name).ok_or(Error::Invalid("plane"))?;
            (0..a.count())
                .flat_map(|i| (0..size).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let plane_index = plane.index.take().ok_or(Error::Invalid("plane index"))?;
        let quad = (
            init(
                "tile quad",
                bytemuck::cast_slice(&read2(&plane, "position", 3)?),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "tile quad uv",
                bytemuck::cast_slice(&read2(&plane, "uv", 2)?),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "tile quad index",
                bytemuck::cast_slice(&plane_index),
                wgpu::BufferUsages::INDEX,
            ),
        );
        let helper_geometry = (
            init(
                "camera helper colors",
                bytemuck::cast_slice(&helper_colors()),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "camera helper points",
                bytemuck::cast_slice(
                    &tile_projections()
                        .into_iter()
                        .flat_map(helper_points)
                        .collect::<Vec<f32>>(),
                ),
                wgpu::BufferUsages::VERTEX,
            ),
        );
        let texture = |format, usage| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("shadow array map"),
                size: wgpu::Extent3d {
                    width: SHADOW,
                    height: SHADOW,
                    depth_or_array_layers: TILES as u32,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        let attach = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let shadow_color = texture(wgpu::TextureFormat::R8Unorm, attach);
        let shadow_depth = texture(DEPTH, attach | wgpu::TextureUsages::TEXTURE_BINDING);
        let shadow_array = shadow_depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let shadow_projections = uniform_sized(r, 4 * 64);
        let shadow_views = uniform_sized(r, 4 * 64);
        // The shadow pipelines: the casters with the shadow material, each
        // drawn for the four tile cameras ( ArrayCamera ).
        let pair = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let pair_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &pair[i],
        });
        let shadow_pipeline = |vs, fs, layouts: &[wgpu::VertexBufferLayout]| {
            culled_pipeline(
                r,
                "shadow array shadow",
                (vs, fs),
                layouts,
                &[wgpu::TextureFormat::R8Unorm],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (true, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                Some(wgpu::Face::Back),
            )
        };
        let knot_shadow_object =
            uniform(r, "knot shadow", wgsl!("knot_shadow_fs"), "objectStruct")?;
        let tree_shadow_object =
            uniform(r, "tree shadow", wgsl!("tree_shadow_fs"), "objectStruct")?;
        let mut shadow_draws = vec![];
        let camera_buffers: Vec<wgpu::Buffer> = (0..TILES as u32)
            .map(|i| {
                init(
                    "camera index",
                    bytemuck::cast_slice(&[i, 0, 0, 0]),
                    wgpu::BufferUsages::UNIFORM,
                )
            })
            .collect();
        // Per pipeline ( automatic layouts ): the four camera-index groups.
        let indices = |p: &wgpu::RenderPipeline| -> Vec<wgpu::BindGroup> {
            camera_buffers
                .iter()
                .map(|b| bind(r, p.get_bind_group_layout(1), &[(0, b.as_entire_binding())]))
                .collect()
        };
        let tex = wgpu::BindingResource::TextureView;
        let cameras_group = |p: &wgpu::RenderPipeline| {
            bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, shadow_projections.as_entire_binding()),
                    (1, shadow_views.as_entire_binding()),
                ],
            )
        };
        for m in &instanced {
            let p = shadow_pipeline(m.shadow_vs, wgsl!("instance_shadow_fs"), &pair_layouts);
            let mut groups = vec![
                cameras_group(&p),
                bind(
                    r,
                    p.get_bind_group_layout(2),
                    &[
                        (0, m.shadow_object.as_entire_binding()),
                        (1, m.instances.as_entire_binding()),
                    ],
                ),
            ];
            groups.extend(indices(&p));
            shadow_draws.push((p.clone(), groups));
        }
        let tree_shadow = shadow_pipeline(
            wgsl!("tree_shadow_vs"),
            wgsl!("tree_shadow_fs"),
            &pair_layouts,
        );
        let tree_shadow_indirect = tree_indirect[0].create_view(&Default::default());
        let mut groups = vec![
            cameras_group(&tree_shadow),
            bind(
                r,
                tree_shadow.get_bind_group_layout(2),
                &[
                    (0, tree_shadow_object.as_entire_binding()),
                    (1, tex(&tree_shadow_indirect)),
                    (2, tex(&tree_matrices)),
                    (3, tex(&tree_colors)),
                ],
            ),
        ];
        groups.extend(indices(&tree_shadow));
        shadow_draws.push((tree_shadow.clone(), groups));
        let knot_shadow = shadow_pipeline(
            wgsl!("knot_shadow_vs"),
            wgsl!("knot_shadow_fs"),
            &pair_layouts[..1],
        );
        let mut groups = vec![
            cameras_group(&knot_shadow),
            bind(
                r,
                knot_shadow.get_bind_group_layout(2),
                &[(0, knot_shadow_object.as_entire_binding())],
            ),
        ];
        groups.extend(indices(&knot_shadow));
        shadow_draws.push((knot_shadow.clone(), groups));
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
        let id4 = m4(Matrix4::IDENTITY);
        for m in &instanced {
            write(
                &m.shadow_object,
                wgsl!("instance_shadow_fs"),
                "objectStruct",
                &[("nodeUniform1", vec![1.]), ("nodeUniform5", id4.clone())],
            )?;
        }
        write(
            &tree_shadow_object,
            wgsl!("tree_shadow_fs"),
            "objectStruct",
            &[("nodeUniform3", vec![1.]), ("nodeUniform7", id4.clone())],
        )?;
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::Less),
            ..Default::default()
        });
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let demo = Self {
            controls,
            pending: true,
            time: 0.,
            last: 0.,
            knot_rotation: [0.; 3],
            background_mesh: upload(r, &SphereGeometry::build(1., 32, 32)?)?,
            knot: upload(r, &TorusKnotGeometry::build(25., 8., 100, 30, 2, 3)?)?,
            ground: {
                let mut g = PlaneGeometry::build(1500., 1500., 2, 2)?;
                g.index = g.index.take();
                upload(r, &g)?
            },
            instanced,
            tree_geometry,
            tree_instances,
            tree_ranges,
            tree_indirect,
            tree_matrices,
            tree_colors,
            quad,
            helper_geometry,
            helper_models: None,
            shadow_color,
            shadow_depth,
            shadow_array,
            shadow_projections,
            shadow_views,
            shadow_draws,
            knot_shadow_object,
            background_render: uniform(r, "background", wgsl!("background_fs"), "renderStruct")?,
            background_object: uniform(r, "background", wgsl!("background_fs"), "objectStruct")?,
            knot_render: uniform(r, "knot", wgsl!("knot_fs"), "renderStruct")?,
            knot_object: uniform(r, "knot", wgsl!("knot_fs"), "objectStruct")?,
            ground_render: uniform(r, "ground", wgsl!("ground_fs"), "renderStruct")?,
            ground_object: uniform(r, "ground", wgsl!("ground_fs"), "objectStruct")?,
            instance_render: uniform(r, "instanced", wgsl!("instance_fs"), "renderStruct")?,
            tree_render: uniform(r, "trees", wgsl!("tree_fs"), "renderStruct")?,
            tree_object: uniform(r, "trees", wgsl!("tree_fs"), "objectStruct")?,
            helper_render: uniform(r, "helper", wgsl!("helper_fs"), "renderStruct")?,
            helper_objects: (0..TILES)
                .map(|_| uniform(r, "helper", wgsl!("helper_fs"), "objectStruct"))
                .collect::<Result<_>>()?,
            tile_renders: (0..TILES)
                .map(|i| uniform(r, "tile", TILE_SHADERS[i].1, "renderStruct"))
                .collect::<Result<_>>()?,
            tile_objects: (0..TILES)
                .map(|i| uniform(r, "tile", TILE_SHADERS[i].1, "objectStruct"))
                .collect::<Result<_>>()?,
            output_render: uniform(r, "output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "output", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "output quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            compare,
            linear,
            targets: None,
        };
        Ok(demo)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let target = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("shadow array target"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = target(HALF, samples);
        let resolve = (samples > 1).then(|| target(HALF, 1));
        let depth = target(DEPTH, samples);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x2],
        ];
        let layout = |stride, a| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: a,
        };
        let pair = [layout(12, &attrs[0][..]), layout(12, &attrs[1][..])];
        let quad_layouts = [layout(12, &attrs[0][..]), layout(8, &attrs[2][..])];
        let less = (wgpu::CompareFunction::LessEqual, true);
        let pipeline = |label,
                        shaders,
                        layouts: &[wgpu::VertexBufferLayout],
                        depth: (wgpu::CompareFunction, bool),
                        cw,
                        blended,
                        topology| {
            culled_pipeline(
                r,
                label,
                shaders,
                layouts,
                &[HALF],
                Some(depth),
                (cw, blended),
                (samples, topology),
                Some(wgpu::Face::Back),
            )
        };
        let triangles = wgpu::PrimitiveTopology::TriangleList;
        let two = |p: wgpu::RenderPipeline,
                   render: &wgpu::Buffer,
                   entries: &[(u32, wgpu::BindingResource)]|
         -> Draw {
            let groups = vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), entries),
            ];
            (p, groups)
        };
        let background = two(
            pipeline(
                "background",
                (wgsl!("background_vs"), wgsl!("background_fs")),
                &pair[..1],
                (wgpu::CompareFunction::Always, false),
                true,
                false,
                triangles,
            ),
            &self.background_render,
            &[(0, self.background_object.as_entire_binding())],
        );
        macro_rules! shadowed {
            ($object:expr) => {
                [
                    (0, $object.as_entire_binding()),
                    (1, sampler(&self.compare)),
                    (2, tex(&self.shadow_array)),
                ]
            };
        }
        let knot = two(
            pipeline(
                "knot",
                (wgsl!("knot_vs"), wgsl!("knot_fs")),
                &pair,
                less,
                false,
                false,
                triangles,
            ),
            &self.knot_render,
            &shadowed!(self.knot_object),
        );
        let ground = two(
            pipeline(
                "ground",
                (wgsl!("ground_vs"), wgsl!("ground_fs")),
                &pair,
                less,
                false,
                false,
                triangles,
            ),
            &self.ground_render,
            &shadowed!(self.ground_object),
        );
        let instanced = self
            .instanced
            .iter()
            .map(|m| {
                let mut entries = shadowed!(m.object).to_vec();
                entries.push((3, m.instances.as_entire_binding()));
                two(
                    pipeline(
                        "instanced",
                        (m.vs, wgsl!("instance_fs")),
                        &pair,
                        less,
                        false,
                        false,
                        triangles,
                    ),
                    &self.instance_render,
                    &entries,
                )
            })
            .collect();
        let tree_indirect = self.tree_indirect[1].create_view(&Default::default());
        let trees = two(
            pipeline(
                "trees",
                (wgsl!("tree_vs"), wgsl!("tree_fs")),
                &pair,
                less,
                false,
                false,
                triangles,
            ),
            &self.tree_render,
            &[
                (0, self.tree_object.as_entire_binding()),
                (1, tex(&tree_indirect)),
                (2, tex(&self.tree_matrices)),
                (3, tex(&self.tree_colors)),
            ],
        );
        let helper_pipeline = pipeline(
            "camera helper",
            (wgsl!("helper_vs"), wgsl!("helper_fs")),
            &pair,
            less,
            false,
            false,
            wgpu::PrimitiveTopology::LineList,
        );
        let helpers = self
            .helper_objects
            .iter()
            .map(|object| {
                two(
                    helper_pipeline.clone(),
                    &self.helper_render,
                    &[(0, object.as_entire_binding())],
                )
            })
            .collect();
        let tiles = (0..TILES)
            .map(|i| {
                two(
                    pipeline(
                        "tile",
                        TILE_SHADERS[i],
                        &quad_layouts,
                        (wgpu::CompareFunction::Always, false),
                        false,
                        true,
                        triangles,
                    ),
                    &self.tile_renders[i],
                    &shadowed!(self.tile_objects[i]),
                )
            })
            .collect();
        let output_pipeline = culled_pipeline(
            r,
            "shadow array output",
            (OUTPUT_VS, OUTPUT_FS),
            &pair[..1],
            &[out.options.format],
            None,
            (false, false),
            (1, triangles),
            Some(wgpu::Face::Back),
        );
        let output = two(
            output_pipeline,
            &self.output_render,
            &[
                (0, sampler(&self.linear)),
                (1, tex(resolve.as_ref().unwrap_or(&color))),
                (2, self.output_object.as_entire_binding()),
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
            background,
            knot,
            ground,
            instanced,
            trees,
            helpers,
            tiles,
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
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("shadow array output"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.screen.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.output_quad.slice(..));
        pass.draw(0..3, 0..1);
    }
    /// BatchedMesh.onBeforeRender: the instances sorted front to back along
    /// the camera's forward axis into the indirect texture.
    fn sort_trees(&self, r: &Renderer, which: usize, position: Vector3, forward: Vector3) {
        let mut list: Vec<(f64, usize)> = self
            .tree_instances
            .iter()
            .enumerate()
            .map(|(i, (_, sphere))| ((sphere.center - position).dot(forward), i))
            .collect();
        list.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut ids = [0u32; 81];
        for (k, (_, i)) in list.iter().enumerate() {
            ids[k] = *i as u32;
        }
        r.queue.write_texture(
            self.tree_indirect[which].as_image_copy(),
            bytemuck::cast_slice(&ids),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(9 * 4),
                rows_per_image: Some(9),
            },
            wgpu::Extent3d {
                width: 9,
                height: 9,
                depth_or_array_layers: 1,
            },
        );
    }
    fn tree_draws(&self, pass: &mut wgpu::RenderPass, sorted: &[usize]) {
        pass.set_vertex_buffer(0, self.tree_geometry.position.slice(..));
        pass.set_vertex_buffer(1, self.tree_geometry.normal.slice(..));
        pass.set_index_buffer(
            self.tree_geometry.index.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        for (k, &i) in sorted.iter().enumerate() {
            let (start, count) = self.tree_ranges[self.tree_instances[i].0];
            pass.draw_indexed(start..start + count, 0, k as u32..k as u32 + 1);
        }
    }
    fn sorted(&self, position: Vector3, forward: Vector3) -> Vec<usize> {
        let mut list: Vec<(f64, usize)> = self
            .tree_instances
            .iter()
            .enumerate()
            .map(|(i, (_, sphere))| ((sphere.center - position).dot(forward), i))
            .collect();
        list.sort_by(|a, b| a.0.total_cmp(&b.0));
        list.into_iter().map(|(_, i)| i).collect()
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
            self.pending = true;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("shadow array targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate( time ): the knot turns by the timer's delta; the light
        // orbits by the loop's time.
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.knot_rotation[0] += 0.25 * delta;
        self.knot_rotation[1] += 0.5 * delta;
        self.knot_rotation[2] += delta;
        let angle = self.time * 1000. * 0.0001;
        let light = Vector3::new(angle.sin() * 30., 80., angle.cos() * 30.);
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.w_axis.truncate();
        // TileShadowNode.update: the tile cameras at the light, looking at its target.
        let light_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let light_world = light_view.inverse();
        let projections = tile_projections();
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
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
        let to_bytes = |ms: &[Matrix4]| -> Vec<f32> {
            ms.iter()
                .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                .collect()
        };
        r.queue.write_buffer(
            &self.shadow_projections,
            0,
            bytemuck::cast_slice(&to_bytes(&projections)),
        );
        r.queue.write_buffer(
            &self.shadow_views,
            0,
            bytemuck::cast_slice(&to_bytes(&[light_view; TILES])),
        );
        let knot_model = compose([5., 5., 0.], self.knot_rotation, [1. / 18.; 3]);
        write(
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
            ],
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", vec![1.]),
                ("nodeUniform4", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let ambient = (Color::from_hex(0xccccff).0 * 3.).to_array().to_vec();
        let sun = (Color::from_hex(0xffffaa).0 * 5.).to_array().to_vec();
        let fog = Color::from_hex(0xccccff).0.to_array().to_vec();
        let shadow_matrices: Vec<Vec<f64>> = projections
            .iter()
            .map(|p| m4(bias * *p * light_view))
            .collect();
        // The Phong render uniforms: ambient, the sun's colour, position and
        // target, then per tile its shadow matrix, bias, normal bias and
        // intensity, then the fog.
        let phong = |names: [usize; 5], tiles: [[usize; 4]; 4], fog_at: usize| {
            let n = |k: usize| format!("nodeUniform{k}");
            let mut values: Vec<(String, Vec<f64>)> = vec![
                ("cameraProjectionMatrix".into(), m4(projection)),
                ("cameraViewMatrix".into(), m4(view)),
                (n(names[0]), ambient.clone()),
                (n(names[1]), sun.clone()),
                (n(names[2]), light.to_array().to_vec()),
                (n(names[3]), vec![0.; 3]),
            ];
            for (i, tile) in tiles.iter().enumerate() {
                values.push((n(tile[0]), shadow_matrices[i].clone()));
                values.push((n(tile[1]), vec![0.]));
                values.push((n(tile[2]), vec![0.]));
                values.push((n(tile[3]), vec![1.]));
            }
            values.push((n(fog_at), fog.clone()));
            values.push((n(fog_at + 1), vec![700.]));
            values.push((n(fog_at + 2), vec![1000.]));
            values
        };
        let pack_named =
            |buffer: &wgpu::Buffer, source: &str, values: Vec<(String, Vec<f64>)>| -> Result<()> {
                let values: Vec<(&str, Vec<f64>)> = values
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.clone()))
                    .collect();
                write(buffer, source, "renderStruct", &values)
            };
        pack_named(
            &self.knot_render,
            wgsl!("knot_fs"),
            phong(
                [6, 12, 10, 11, 0],
                [
                    [14, 15, 16, 18],
                    [19, 20, 21, 22],
                    [23, 24, 25, 26],
                    [27, 28, 29, 30],
                ],
                31,
            ),
        )?;
        pack_named(
            &self.ground_render,
            wgsl!("ground_fs"),
            phong(
                [6, 12, 10, 11, 0],
                [
                    [13, 14, 15, 17],
                    [18, 19, 20, 21],
                    [22, 23, 24, 25],
                    [26, 27, 28, 29],
                ],
                30,
            ),
        )?;
        pack_named(
            &self.instance_render,
            wgsl!("instance_fs"),
            phong(
                [7, 13, 11, 12, 0],
                [
                    [15, 16, 17, 19],
                    [20, 21, 22, 23],
                    [24, 25, 26, 27],
                    [28, 29, 30, 31],
                ],
                32,
            ),
        )?;
        write(
            &self.tree_render,
            wgsl!("tree_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform9", ambient.clone()),
                ("nodeUniform15", sun.clone()),
                ("nodeUniform13", light.to_array().to_vec()),
                ("nodeUniform14", vec![0.; 3]),
                ("nodeUniform17", fog.clone()),
                ("nodeUniform18", vec![700.]),
                ("nodeUniform19", vec![1000.]),
            ],
        )?;
        let specular = Color::from_hex(0x111111).0.to_array().to_vec();
        write(
            &self.knot_object,
            wgsl!("knot_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    Color::from_hex(0xff6347).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![30.]),
                ("nodeUniform3", specular.clone()),
                ("nodeUniform4", vec![0.; 3]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform8", m3(knot_model.inverse().transpose())),
                ("nodeUniform13", m4(knot_model)),
            ],
        )?;
        let ground_model = Matrix4::from_rotation_x(-PI / 2.);
        write(
            &self.ground_object,
            wgsl!("ground_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", m4(ground_model)),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![5.]),
                (
                    "nodeUniform3",
                    Color::from_hex(0x222222).0.to_array().to_vec(),
                ),
                ("nodeUniform4", vec![0.; 3]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform8", m3(ground_model.inverse().transpose())),
            ],
        )?;
        for m in &self.instanced {
            write(
                &m.object,
                wgsl!("instance_fs"),
                "objectStruct",
                &[
                    (
                        "nodeUniform1",
                        Color::from_hex(m.color).0.to_array().to_vec(),
                    ),
                    ("nodeUniform2", vec![1.]),
                    ("nodeUniform3", vec![m.shininess]),
                    ("nodeUniform4", specular.clone()),
                    ("nodeUniform5", vec![0.; 3]),
                    ("nodeUniform6", vec![1.]),
                    ("nodeUniform9", m3(Matrix4::IDENTITY)),
                    ("nodeUniform14", m4(Matrix4::IDENTITY)),
                ],
            )?;
        }
        write(
            &self.tree_object,
            wgsl!("tree_fs"),
            "objectStruct",
            &[
                ("nodeUniform3", vec![1.; 3]),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform5", vec![5.]),
                ("nodeUniform6", specular.clone()),
                ("nodeUniform7", vec![0.; 3]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform11", m3(Matrix4::IDENTITY)),
                ("nodeUniform16", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let knot_shadow_values = [("nodeUniform0", vec![1.]), ("nodeUniform4", m4(knot_model))];
        write(
            &self.knot_shadow_object,
            wgsl!("knot_shadow_fs"),
            "objectStruct",
            &knot_shadow_values,
        )?;
        write(
            &self.helper_render,
            wgsl!("helper_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform2", fog.clone()),
                ("nodeUniform3", vec![700.]),
                ("nodeUniform4", vec![1000.]),
            ],
        )?;
        if let Some(models) = &self.helper_models {
            for (object, model) in self.helper_objects.iter().zip(models) {
                write(
                    object,
                    wgsl!("helper_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", vec![1.; 3]),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform6", m4(*model)),
                    ],
                )?;
            }
        }
        for ((render, object), shaders) in self
            .tile_renders
            .iter()
            .zip(&self.tile_objects)
            .zip(&TILE_SHADERS)
        {
            write(
                render,
                shaders.1,
                "renderStruct",
                &[
                    ("nodeUniform1", fog.clone()),
                    ("nodeUniform2", vec![700.]),
                    ("nodeUniform3", vec![1000.]),
                    ("nodeUniform5", vec![t.width as f64, t.height as f64]),
                    ("cameraProjectionMatrixInverse", m4(projection.inverse())),
                ],
            )?;
            write(
                object,
                shaders.1,
                "objectStruct",
                &[("nodeUniform0", vec![1.])],
            )?;
        }
        write(
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
                ("nodeUniform2", vec![1.2]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        // The shadow: the trees sorted for the ArrayCamera ( at the origin,
        // looking down −z ); each layer draws the casters for its tile camera.
        self.sort_trees(r, 0, Vector3::ZERO, -Vector3::Z);
        let shadow_order = self.sorted(Vector3::ZERO, -Vector3::Z);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for layer in 0..TILES as u32 {
            let view = |t: &wgpu::Texture| {
                t.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_array_layer: layer,
                    array_layer_count: Some(1),
                    ..Default::default()
                })
            };
            let (color_view, depth_view) = (view(&self.shadow_color), view(&self.shadow_depth));
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shadow array tile"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            let n = self.instanced.len();
            for (k, draw) in self.shadow_draws.iter().enumerate() {
                pass.set_pipeline(&draw.0);
                pass.set_bind_group(0, &draw.1[0], &[]);
                pass.set_bind_group(2, &draw.1[1], &[]);
                {
                    pass.set_bind_group(1, &draw.1[2 + layer as usize], &[]);
                    if k < n {
                        let g = &self.instanced[k].geometry;
                        pass.set_vertex_buffer(0, g.position.slice(..));
                        pass.set_vertex_buffer(1, g.normal.slice(..));
                        pass.set_index_buffer(g.index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..g.count, 0, 0..self.instanced[k].count);
                    } else if k == n {
                        self.tree_draws(&mut pass, &shadow_order);
                    } else {
                        pass.set_vertex_buffer(0, self.knot.position.slice(..));
                        pass.set_index_buffer(self.knot.index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.knot.count, 0, 0..1);
                    }
                }
            }
        }
        r.queue.submit([encoder.finish()]);
        // The scene: the trees sorted for the main camera.
        let forward = world.transform_vector3(-Vector3::Z).normalize();
        self.sort_trees(r, 1, camera_position, forward);
        let main_order = self.sorted(camera_position, forward);
        let frustum = Frustum::from_projection(projection * view);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shadow array scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
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
            set(&mut pass, &t.background);
            pass.set_vertex_buffer(0, self.background_mesh.position.slice(..));
            pass.set_index_buffer(
                self.background_mesh.index.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..self.background_mesh.count, 0, 0..1);
            let draw =
                |pass: &mut wgpu::RenderPass, g: &Geometry, first_normal: bool, instances: u32| {
                    let (a, b) = if first_normal {
                        (&g.normal, &g.position)
                    } else {
                        (&g.position, &g.normal)
                    };
                    pass.set_vertex_buffer(0, a.slice(..));
                    pass.set_vertex_buffer(1, b.slice(..));
                    pass.set_index_buffer(g.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..g.count, 0, 0..instances);
                };
            let knot_sphere = Sphere {
                center: knot_model.transform_point3(self.knot.sphere.center),
                radius: self.knot.sphere.radius / 18.,
            };
            if frustum.intersects_sphere(knot_sphere) {
                set(&mut pass, &t.knot);
                draw(&mut pass, &self.knot, true, 1);
            }
            set(&mut pass, &t.ground);
            draw(&mut pass, &self.ground, false, 1);
            for (m, d) in self.instanced.iter().zip(&t.instanced) {
                if frustum.intersects_sphere(m.sphere) {
                    set(&mut pass, d);
                    draw(&mut pass, &m.geometry, false, m.count);
                }
            }
            set(&mut pass, &t.trees);
            self.tree_draws(&mut pass, &main_order);
            if self.helper_models.is_some() {
                for (i, d) in t.helpers.iter().enumerate() {
                    set(&mut pass, d);
                    pass.set_vertex_buffer(0, self.helper_geometry.0.slice(..));
                    pass.set_vertex_buffer(
                        1,
                        self.helper_geometry
                            .1
                            .slice(i as u64 * 600..(i as u64 + 1) * 600),
                    );
                    pass.draw(0..50, 0..1);
                }
                for d in &t.tiles {
                    set(&mut pass, d);
                    pass.set_vertex_buffer(0, self.quad.0.slice(..));
                    pass.set_vertex_buffer(1, self.quad.1.slice(..));
                    pass.set_index_buffer(self.quad.2.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..6, 0, 0..1);
                }
            }
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        // tsmHelper.update() after the render: the helpers take this frame's
        // tile cameras for the next frame.
        self.helper_models = Some(vec![light_world; TILES]);
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
        self.pending = true;
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("shadow array has no parameters"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
fn uniform_sized(r: &Renderer, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("shadow array cameras"),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
