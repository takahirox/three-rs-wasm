//! webgpu_geometry_loft: a turntable exhibition of LoftGeometry pieces —
//! coffee cup, saucer and handle, vase, nautilus shell, twisted star, gold
//! ribbon, toothpaste tube, pumpkin, mushroom and copper goblet on marble
//! pedestals, a stanchion-and-rope barrier and a theatre curtain — each
//! dressed by its procedural node material, lit by the SunLight addon's two
//! 4096² cascades and RoomEnvironment's PMREM over the screen-space vignette
//! background, with NeutralToneMapping and OrbitControls. The group turns by
//! 0.001 rad per frame, as the page's animate() does. The loft and circle
//! geometry is built once on the CPU, as the page's is; each mesh renders
//! with its own draw into both cascades and the 4× MSAA scene pass. The
//! sections view draws each loft's section rings as line segments instead.
//! Every stage runs the WGSL three.js r186 generates for the page ( in
//! `geometry_loft/`; the background vertex stage, the default shadow stages,
//! the line material and the output are the skinning_instances, fog_volume,
//! custom_fog, cubemap_dynamic and compute_cloth modules, byte-identical ).
pub(super) mod shapes;
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, culled_pipeline, set};
use super::fog_volume::{Cascade, cascades_sized};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use shapes::Geometry;
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The SunLight shadow: two 4096² cascades side by side.
const MAP: u32 = 4096;
/// The light's position ( it shines towards the origin ) and intensity.
const SUN: [f64; 3] = [18., 30., 12.];
const SUN_INTENSITY: f64 = 3.;
/// scene.environmentIntensity.
const ENVIRONMENT: f64 = 0.4;
/// The cube-UV texel sizes at lodMax 8.
const TEXEL: [f64; 2] = [1. / 768., 1. / 1024.];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("geometry_loft/", $name, ".wgsl"))
    };
}
const BACKGROUND_VS: &str = include_str!("skinning_instances/background_vs.wgsl");
const SHADOW_VS: &str = include_str!("fog_volume/shadow_vs.wgsl");
const SHADOW_FS: &str = include_str!("fog_volume/shadow_fs.wgsl");
const LINE_VS: &str = include_str!("custom_fog/box_vs.wgsl");
const LINE_FS: &str = include_str!("custom_fog/box_fs.wgsl");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("compute_cloth/output_fs.wgsl");
/// One node material: its scene and shadow stages, its side and the plain
/// uniforms it keeps ( colour, metalness, roughness ).
struct Kind {
    main: (&'static str, &'static str),
    shadow: Option<(&'static str, &'static str)>,
    double: bool,
    color: [f64; 3],
    metalness: f64,
    roughness: f64,
}
fn kind(
    main: (&'static str, &'static str),
    shadow: Option<(&'static str, &'static str)>,
    double: bool,
    color: u32,
    metalness: f64,
    roughness: f64,
) -> Kind {
    Kind {
        main,
        shadow,
        double,
        color: Color::from_hex(color).0.to_array(),
        metalness,
        roughness,
    }
}
const DEFAULT_SHADOW: Option<(&str, &str)> = Some((SHADOW_VS, SHADOW_FS));
/// The page's materials, in the order of `kinds()`.
#[derive(Clone, Copy)]
enum K {
    Floor,
    Curtain,
    Pedestal,
    Porcelain,
    Liquid,
    Vase,
    Shell,
    Star,
    Ribbon,
    Tube,
    Pumpkin,
    PumpkinStem,
    Cap,
    Stem,
    Goblet,
    Brass,
    Rope,
}
fn kinds() -> Vec<Kind> {
    vec![
        kind(
            (wgsl!("floor_vs"), wgsl!("floor_fs")),
            None,
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("curtain_vs"), wgsl!("curtain_fs")),
            Some((wgsl!("shadow_uv_vs"), wgsl!("curtain_shadow_fs"))),
            true,
            0xffffff,
            0.,
            0.9,
        ),
        kind(
            (wgsl!("pedestal_vs"), wgsl!("pedestal_fs")),
            Some((wgsl!("pedestal_shadow_vs"), wgsl!("pedestal_shadow_fs"))),
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("porcelain_vs"), wgsl!("porcelain_fs")),
            DEFAULT_SHADOW,
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("floor_vs"), wgsl!("liquid_fs")),
            None,
            false,
            0xffffff,
            0.,
            0.08,
        ),
        kind(
            (wgsl!("vase_vs"), wgsl!("vase_fs")),
            Some((wgsl!("shadow_local_vs"), wgsl!("vase_shadow_fs"))),
            true,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("shell_vs"), wgsl!("shell_fs")),
            Some((wgsl!("shadow_uv_vs"), wgsl!("shell_shadow_fs"))),
            true,
            0xffffff,
            0.,
            0.5,
        ),
        kind(
            (wgsl!("star_vs"), wgsl!("star_fs")),
            Some((wgsl!("shadow_local_vs"), wgsl!("star_shadow_fs"))),
            false,
            0xffffff,
            0.,
            0.6,
        ),
        kind(
            (wgsl!("ribbon_vs"), wgsl!("ribbon_fs")),
            DEFAULT_SHADOW,
            true,
            0xffcc44,
            1.,
            1.,
        ),
        kind(
            (wgsl!("curtain_vs"), wgsl!("tube_fs")),
            Some((wgsl!("shadow_uv_vs"), wgsl!("tube_shadow_fs"))),
            false,
            0xffffff,
            0.,
            0.25,
        ),
        kind(
            (wgsl!("pumpkin_vs"), wgsl!("pumpkin_fs")),
            Some((wgsl!("shadow_uv_vs"), wgsl!("pumpkin_shadow_fs"))),
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("brass_vs"), wgsl!("pumpkin_stem_fs")),
            DEFAULT_SHADOW,
            false,
            0x667744,
            0.,
            1.,
        ),
        kind(
            (wgsl!("cap_vs"), wgsl!("cap_fs")),
            Some((wgsl!("cap_shadow_vs"), wgsl!("cap_shadow_fs"))),
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("stem_vs"), wgsl!("stem_fs")),
            Some((wgsl!("shadow_uv_vs"), wgsl!("stem_shadow_fs"))),
            false,
            0xffffff,
            0.,
            1.,
        ),
        kind(
            (wgsl!("goblet_vs"), wgsl!("goblet_fs")),
            DEFAULT_SHADOW,
            false,
            0xb87333,
            1.,
            1.,
        ),
        kind(
            (wgsl!("brass_vs"), wgsl!("brass_fs")),
            DEFAULT_SHADOW,
            false,
            0xc9a86a,
            1.,
            1.,
        ),
        kind(
            (wgsl!("rope_vs"), wgsl!("rope_fs")),
            DEFAULT_SHADOW,
            false,
            0x8a2433,
            0.,
            0.65,
        ),
    ]
}
/// A struct's fields ( name, type ) in declaration order.
fn fields(source: &str, name: &str) -> Vec<(String, String)> {
    let Some(start) = source.find(&format!("struct {name} {{")) else {
        return vec![];
    };
    let body = &source[start..];
    let body = &body[body.find('{').unwrap_or(0) + 1..body.find('}').unwrap_or(0)];
    body.split(",\n")
        .filter_map(|f| {
            let (n, t) = f.split_once(':')?;
            Some((
                n.trim().to_string(),
                t.trim().trim_end_matches(',').to_string(),
            ))
        })
        .collect()
}
/// The first line using `object.<name>` ( not a longer name ).
fn first_use<'a>(sources: &[&'a str], name: &str) -> Option<&'a str> {
    let needle = format!("object.{name}");
    for source in sources {
        for line in source.lines() {
            let mut from = 0;
            while let Some(i) = line[from..].find(&needle) {
                let end = from + i + needle.len();
                if !line[end..].starts_with(|c: char| c.is_ascii_digit()) {
                    return Some(line);
                }
                from = end;
            }
        }
    }
    None
}
/// The object uniforms of a node material by how its shaders use them:
/// colour, opacity, metalness, roughness, emissive, the model and normal
/// matrices, and the environment's lodMax, rotation, texel size and
/// intensity.
fn object_values(
    vs: &str,
    fs: &str,
    k: Option<&Kind>,
    model: Matrix4,
) -> Result<Vec<(String, Vec<f64>)>> {
    let mut out = vec![];
    let mut matrices = 0;
    for (name, ty) in fields(fs, "objectStruct") {
        let value = match ty.as_str() {
            "mat3x3<f32>" => m3(model.inverse().transpose()),
            "mat4x4<f32>" => {
                matrices += 1;
                if matrices == 1 {
                    m4(model)
                } else {
                    m4(Matrix4::IDENTITY)
                }
            }
            _ => {
                let line = first_use(&[fs, vs], &name).ok_or(Error::Invalid("loft uniform use"))?;
                let used =
                    |pattern: &str| line.contains(&pattern.replace('#', &format!("object.{name}")));
                if used("DiffuseColor = vec4<f32>( #, 1.0 )") {
                    k.map_or(vec![1.; 3], |k| k.color.to_vec())
                } else if used("DiffuseColor.w * #") {
                    vec![1.]
                } else if used("Metalness = #") {
                    vec![k.map_or(0., |k| k.metalness)]
                } else if line.contains("Roughness = ") {
                    vec![k.map_or(1., |k| k.roughness)]
                } else if used("EmissiveColor = ( # *") {
                    vec![0.; 3]
                } else if used("vec3<f32>( # ) )") && line.contains("EmissiveColor") {
                    vec![1.]
                } else if used("-2.0, # )") {
                    vec![8.]
                } else if used(".x * # )") {
                    vec![TEXEL[0]]
                } else if used(".y * # )") {
                    vec![TEXEL[1]]
                } else if line.contains("radiance + (") || line.contains("iblIrradiance + (") {
                    vec![ENVIRONMENT]
                } else {
                    return Err(Error::Invalid("loft uniform"));
                }
            }
        };
        out.push((name, value));
    }
    Ok(out)
}
/// The camera and SunLight uniforms of a lit material, by field order:
/// the light's colour and direction, then each cascade's matrix and data
/// ( far cascade first ), the normal bias, each cascade's bias, radius and
/// map size, and the shadow intensity.
fn render_values(
    fs: &str,
    camera: &[(&str, Vec<f64>)],
    light: &[Vec<f64>],
) -> Result<Vec<(String, Vec<f64>)>> {
    let mut out = vec![];
    let mut k = 0;
    for (name, _) in fields(fs, "renderStruct") {
        if let Some((_, v)) = camera.iter().find(|(n, _)| *n == name) {
            out.push((name, v.clone()));
        } else {
            out.push((
                name,
                light
                    .get(k)
                    .ok_or(Error::Invalid("loft light uniform"))?
                    .clone(),
            ));
            k += 1;
        }
    }
    Ok(out)
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
fn named(values: &[(&str, Vec<f64>)]) -> Vec<(String, Vec<f64>)> {
    values
        .iter()
        .map(|(n, v)| (n.to_string(), v.clone()))
        .collect()
}
/// The vertex stage's attributes ( location, name, format ) from its main().
fn attributes(vs: &str) -> Vec<(u32, String, wgpu::VertexFormat)> {
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
            let format = if ty.contains("vec2") {
                wgpu::VertexFormat::Float32x2
            } else {
                wgpu::VertexFormat::Float32x3
            };
            Some((
                location.trim().parse().ok()?,
                name.trim().to_string(),
                format,
            ))
        })
        .collect()
}
fn pipeline(
    r: &Renderer,
    (vs, fs): (&str, &str),
    format: wgpu::TextureFormat,
    samples: u32,
    (cw, cull): (bool, Option<wgpu::Face>),
    topology: wgpu::PrimitiveTopology,
) -> wgpu::RenderPipeline {
    let attrs = attributes(vs);
    let lists: Vec<[wgpu::VertexAttribute; 1]> = attrs
        .iter()
        .map(|(location, _, format)| {
            [wgpu::VertexAttribute {
                format: *format,
                offset: 0,
                shader_location: *location,
            }]
        })
        .collect();
    let layouts: Vec<wgpu::VertexBufferLayout> = attrs
        .iter()
        .zip(&lists)
        .map(|((_, _, format), list)| wgpu::VertexBufferLayout {
            array_stride: format.size(),
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: list,
        })
        .collect();
    culled_pipeline(
        r,
        "geometry loft",
        (vs, fs),
        &layouts,
        &[format],
        Some((wgpu::CompareFunction::LessEqual, true)),
        (cw, false),
        (samples, topology),
        cull,
    )
}
/// A resident geometry: its attributes by name, index and bounds.
struct Mesh {
    position: wgpu::Buffer,
    normal: wgpu::Buffer,
    uv: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    skeleton: Option<(wgpu::Buffer, u32)>,
}
impl Mesh {
    fn new(r: &Renderer, g: &Geometry) -> Self {
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let (center, radius) = g.bounding_sphere();
        Self {
            position: init("loft position", bytemuck::cast_slice(&g.position), vertex),
            normal: init("loft normal", bytemuck::cast_slice(&g.normal), vertex),
            uv: init("loft uv", bytemuck::cast_slice(&g.uv), vertex),
            index: init(
                "loft index",
                bytemuck::cast_slice(&g.index),
                wgpu::BufferUsages::INDEX,
            ),
            count: g.index.len() as u32,
            sphere: Sphere {
                center: Vector3::from_array(center),
                radius,
            },
            skeleton: (!g.skeleton.is_empty()).then(|| {
                (
                    init("loft sections", bytemuck::cast_slice(&g.skeleton), vertex),
                    (g.skeleton.len() / 3) as u32,
                )
            }),
        }
    }
    fn draw(&self, pass: &mut wgpu::RenderPass, vs: &str) {
        for (slot, (_, name, _)) in attributes(vs).iter().enumerate() {
            let buffer = match name.as_str() {
                "normal" => &self.normal,
                "uv" => &self.uv,
                _ => &self.position,
            };
            pass.set_vertex_buffer(slot as u32, buffer.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
}
/// One scene mesh: its geometry, material, transform under the turning
/// group, and its uniforms.
struct Item {
    mesh: usize,
    kind: usize,
    local: Matrix4,
    object: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    line_object: wgpu::Buffer,
    shadow: Vec<Draw>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    background: Draw,
    items: Vec<Draw>,
    lines: Vec<Draw>,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    time: f64,
    /// group.rotation.y.
    rotation: f64,
    sections: bool,
    kinds: Vec<Kind>,
    meshes: Vec<Mesh>,
    items: Vec<Item>,
    /// One camera/light uniform buffer per material.
    renders: Vec<wgpu::Buffer>,
    shadow_renders: Vec<wgpu::Buffer>,
    line_render: wgpu::Buffer,
    background_mesh: (wgpu::Buffer, wgpu::Buffer, u32),
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    environment: wgpu::TextureView,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::from_translation(Vector3::new(x, y, z))
}
fn rotation_y(angle: f64) -> Matrix4 {
    Matrix4::from_quat(Quaternion::from_rotation_y(angle))
}
fn scale(s: f64) -> Matrix4 {
    Matrix4::from_scale(Vector3::splat(s))
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
        s.get_mut(c)?.position = Vector3::new(0., 15., 40.);
        let mut controls = Controls::new(None, (15., 50.), PI, true);
        controls.set_target(Vector3::new(0., -3., 0.));
        controls.update(s, c)?;
        let kinds = kinds();
        // The geometry, in the page's construction order.
        let mut geometries: Vec<Geometry> = vec![];
        let mut items: Vec<(usize, K, Matrix4, bool)> = vec![];
        let mut add = |g: Geometry| {
            geometries.push(g);
            geometries.len() - 1
        };
        let floor = add(shapes::floor_circle(58., 64));
        items.push((floor, K::Floor, translation(0., -5.01, 0.), false));
        let curtain = add(shapes::curtain());
        items.push((curtain, K::Curtain, Matrix4::IDENTITY, false));
        let pedestal = |items: &mut Vec<(usize, K, Matrix4, bool)>,
                        add: &mut dyn FnMut(Geometry) -> usize,
                        x: f64,
                        z: f64,
                        radius: f64,
                        height: f64| {
            let g = add(shapes::pedestal(radius, height));
            items.push((
                g,
                K::Pedestal,
                translation(x, -5., z) * rotation_y(x * 0.7 + z * 1.3),
                false,
            ));
            -5. + height
        };
        let top = pedestal(&mut items, &mut add, 0., 0., 3.6, 2.2);
        let coffee = translation(0., top, 0.) * scale(0.75);
        let saucer = add(shapes::saucer());
        items.push((saucer, K::Porcelain, coffee, false));
        let cup = add(shapes::cup());
        items.push((cup, K::Porcelain, coffee * translation(0., 0.3, 0.), false));
        let handle = add(shapes::handle());
        items.push((
            handle,
            K::Porcelain,
            coffee * translation(0., 0.3, 0.) * rotation_y(0.15),
            false,
        ));
        let liquid = add(shapes::floor_circle(2., 48));
        items.push((liquid, K::Liquid, coffee * translation(0., 3.6, 0.), true));
        let top = pedestal(&mut items, &mut add, -10.5, -10.5, 3.6, 1.1);
        let vase = add(shapes::vase());
        items.push((vase, K::Vase, translation(-10.5, top, -10.5), false));
        let top = pedestal(&mut items, &mut add, 10.5, -10.5, 3.6, 1.1) + 1.92;
        let shell = add(shapes::shell());
        items.push((
            shell,
            K::Shell,
            translation(10.5, top, -10.5) * rotation_y(-PI / 2.) * scale(0.8),
            false,
        ));
        let top = pedestal(&mut items, &mut add, -10.5, 10.5, 3.6, 1.1);
        let star = add(shapes::star());
        items.push((
            star,
            K::Star,
            translation(-10.5, top, 10.5) * scale(0.8),
            false,
        ));
        let top = pedestal(&mut items, &mut add, 10.5, 10.5, 3.6, 1.1);
        let ribbon = add(shapes::ribbon());
        items.push((ribbon, K::Ribbon, translation(10.5, top, 10.5), false));
        let top = pedestal(&mut items, &mut add, 7.8, 0., 2., 1.6);
        let tube = add(shapes::toothpaste());
        items.push((
            tube,
            K::Tube,
            translation(7.8, top, 0.) * rotation_y(0.5),
            false,
        ));
        let top = pedestal(&mut items, &mut add, 0., 7.8, 2., 1.6);
        let pumpkin = add(shapes::pumpkin());
        items.push((pumpkin, K::Pumpkin, translation(0., top, 7.8), false));
        let stem = add(shapes::pumpkin_stem());
        items.push((stem, K::PumpkinStem, translation(0., top, 7.8), false));
        let top = pedestal(&mut items, &mut add, -7.8, 0., 2., 1.6);
        let cap = add(shapes::mushroom_cap());
        items.push((cap, K::Cap, translation(-7.8, top, 0.), false));
        let stem = add(shapes::mushroom_stem());
        items.push((stem, K::Stem, translation(-7.8, top, 0.), false));
        let top = pedestal(&mut items, &mut add, 0., -7.8, 2., 1.6);
        let goblet = add(shapes::goblet());
        items.push((goblet, K::Goblet, translation(0., top, -7.8), false));
        let (posts, barrier) = (14., 20.);
        let stanchion = add(shapes::stanchion());
        let rope = add(shapes::rope(2. * barrier * (PI / posts).sin()));
        for i in 0..14 {
            let angle = (i as f64 + 0.5) / posts * PI * 2.;
            items.push((
                stanchion,
                K::Brass,
                translation(angle.sin() * barrier, -5., angle.cos() * barrier),
                false,
            ));
            let mid = angle + PI / posts;
            let mid_radius = barrier * (PI / posts).cos();
            items.push((
                rope,
                K::Rope,
                translation(mid.sin() * mid_radius, -5. + 2.05, mid.cos() * mid_radius)
                    * rotation_y(mid),
                false,
            ));
        }
        let meshes: Vec<Mesh> = geometries.iter().map(|g| Mesh::new(r, g)).collect();
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
        let texture = |size: (u32, u32), format, usage| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("geometry loft"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let attach = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let atlas = (MAP * 2, MAP);
        let shadow_color = texture(atlas, BYTE, attach);
        let shadow_depth = texture(atlas, DEPTH, attach | wgpu::TextureUsages::TEXTURE_BINDING);
        // scene.environment = fromScene( new RoomEnvironment(), 0.04 ).
        let room = super::room_environment::environment(r)?;
        let environment = room
            .gpu
            .as_ref()
            .ok_or(Error::Invalid("room environment"))?
            .view
            .clone();
        let shadow_renders = (0..2)
            .map(|_| uniform(r, "loft shadow", SHADOW_VS, "renderStruct"))
            .collect::<Result<Vec<_>>>()?;
        let shadow_pipelines: Vec<Option<wgpu::RenderPipeline>> = kinds
            .iter()
            .map(|k| {
                k.shadow.map(|shaders| {
                    let side = if k.double {
                        (false, None)
                    } else {
                        (true, Some(wgpu::Face::Back))
                    };
                    pipeline(
                        r,
                        shaders,
                        BYTE,
                        1,
                        side,
                        wgpu::PrimitiveTopology::TriangleList,
                    )
                })
            })
            .collect();
        let mut built = vec![];
        for (mesh, k, local, _) in items {
            let kind = k as usize;
            let fs = kinds[kind].main.1;
            let shadow_object = uniform(r, "loft shadow object", SHADOW_VS, "objectStruct")?;
            let shadow = match &shadow_pipelines[kind] {
                Some(p) => shadow_renders
                    .iter()
                    .map(|render| {
                        let groups = vec![
                            bind(
                                r,
                                p.get_bind_group_layout(0),
                                &[(0, render.as_entire_binding())],
                            ),
                            bind(
                                r,
                                p.get_bind_group_layout(1),
                                &[(0, shadow_object.as_entire_binding())],
                            ),
                        ];
                        (p.clone(), groups)
                    })
                    .collect(),
                None => vec![],
            };
            built.push(Item {
                mesh,
                kind,
                local,
                object: uniform(r, "loft object", fs, "objectStruct")?,
                shadow_object,
                line_object: uniform(r, "loft sections", LINE_FS, "objectStruct")?,
                shadow,
            });
        }
        let renders = kinds
            .iter()
            .map(|k| uniform(r, "loft render", k.main.1, "renderStruct"))
            .collect::<Result<Vec<_>>>()?;
        let sphere = SphereGeometry::build(1., 32, 32)?;
        let positions: Vec<f32> = {
            let a = sphere
                .attributes
                .get("position")
                .ok_or(Error::Invalid("background positions"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect::<Result<_>>()?
        };
        let index = sphere
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        Ok(Self {
            controls,
            pending: true,
            time: 0.,
            rotation: 0.,
            sections: false,
            kinds,
            meshes,
            items: built,
            renders,
            shadow_renders,
            line_render: uniform(r, "loft sections", LINE_VS, "renderStruct")?,
            background_mesh: (
                init(
                    "loft background",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                init(
                    "loft background index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ),
            background_render: uniform(
                r,
                "loft background",
                wgsl!("background_fs"),
                "renderStruct",
            )?,
            background_object: uniform(
                r,
                "loft background",
                wgsl!("background_fs"),
                "objectStruct",
            )?,
            output_render: uniform(r, "loft output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "loft output", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "loft quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            shadow_color,
            shadow_depth,
            environment,
            linear,
            compare,
            targets: None,
        })
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let target = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("geometry loft target"),
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
        let triangles = wgpu::PrimitiveTopology::TriangleList;
        let main_pipelines: Vec<wgpu::RenderPipeline> = self
            .kinds
            .iter()
            .map(|k| {
                let cull = if k.double {
                    None
                } else {
                    Some(wgpu::Face::Back)
                };
                pipeline(r, k.main, HALF, samples, (false, cull), triangles)
            })
            .collect();
        let mut items = vec![];
        for item in &self.items {
            let p = &main_pipelines[item.kind];
            let fs = self.kinds[item.kind].main.1;
            // The lit materials' textures: the DFG LUT, the shadow atlas and
            // the environment, in binding order.
            let mut entries = vec![(0, item.object.as_entire_binding())];
            let mut textures = 0;
            for line in fs.lines().filter(|l| l.contains("@group( 1 ) var")) {
                let binding: u32 = line
                    .split("@binding(")
                    .nth(1)
                    .and_then(|b| b.split(')').next())
                    .and_then(|b| b.trim().parse().ok())
                    .ok_or(Error::Invalid("loft binding"))?;
                let resource = if line.contains("sampler_comparison") {
                    sampler(&self.compare)
                } else if line.contains(": sampler") {
                    sampler(&self.linear)
                } else if line.contains("texture_depth_2d") {
                    tex(&self.shadow_depth)
                } else {
                    textures += 1;
                    tex(if textures == 1 {
                        &r.dfg
                    } else {
                        &self.environment
                    })
                };
                entries.push((binding, resource));
            }
            let groups = vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, self.renders[item.kind].as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), &entries),
            ];
            items.push((p.clone(), groups));
        }
        let line_pipeline = pipeline(
            r,
            (LINE_VS, LINE_FS),
            HALF,
            samples,
            (false, Some(wgpu::Face::Back)),
            wgpu::PrimitiveTopology::LineList,
        );
        let lines = self
            .items
            .iter()
            .map(|item| {
                (
                    line_pipeline.clone(),
                    vec![
                        bind(
                            r,
                            line_pipeline.get_bind_group_layout(0),
                            &[(0, self.line_render.as_entire_binding())],
                        ),
                        bind(
                            r,
                            line_pipeline.get_bind_group_layout(1),
                            &[(0, item.line_object.as_entire_binding())],
                        ),
                    ],
                )
            })
            .collect();
        let background_pipeline = culled_pipeline(
            r,
            "loft background",
            (BACKGROUND_VS, wgsl!("background_fs")),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            }],
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            (samples, triangles),
            Some(wgpu::Face::Back),
        );
        let background = (
            background_pipeline.clone(),
            vec![
                bind(
                    r,
                    background_pipeline.get_bind_group_layout(0),
                    &[(0, self.background_render.as_entire_binding())],
                ),
                bind(
                    r,
                    background_pipeline.get_bind_group_layout(1),
                    &[(0, self.background_object.as_entire_binding())],
                ),
            ],
        );
        let output_pipeline = culled_pipeline(
            r,
            "loft output",
            (OUTPUT_VS, OUTPUT_FS),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            }],
            &[out.options.format],
            None,
            (false, false),
            (1, triangles),
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
                        (0, sampler(&self.linear)),
                        (1, tex(resolve.as_ref().unwrap_or(&color))),
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
            color,
            resolve,
            depth,
            background,
            items,
            lines,
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
            label: Some("loft output"),
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
    /// Whether an item is drawn: the sections view hides the lofts and the
    /// coffee surface.
    fn shown(&self, item: &Item) -> bool {
        !self.sections || (item.kind == K::Floor as usize)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        // A resize redraws the frame without advancing the animation.
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("loft targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let advance = std::mem::take(&mut self.pending);
        if !advance && !resized {
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the group turns, then the scene renders.
        if advance {
            self.rotation += 0.001;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let sun = Vector3::from_array(SUN);
        let fitted: [Cascade; 2] = cascades_sized(sun, world, projection, (1., 1000., 110.), MAP);
        let group = rotation_y(self.rotation);
        let size = vec![(MAP * 2) as f64, MAP as f64];
        let light = vec![
            vec![SUN_INTENSITY; 3],
            SUN.to_vec(),
            m4(fitted[1].matrix),
            fitted[1].data.to_vec(),
            m4(fitted[0].matrix),
            fitted[0].data.to_vec(),
            vec![0.],
            vec![-0.0005],
            vec![1.],
            size.clone(),
            vec![-0.0005],
            vec![1.],
            size,
            vec![1.],
        ];
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("cameraWorldMatrix", m4(world)),
            (
                "cameraPosition",
                world.w_axis.truncate().to_array().to_vec(),
            ),
        ];
        for (k, render) in self.kinds.iter().zip(&self.renders) {
            write(
                r,
                render,
                k.main.1,
                "renderStruct",
                &render_values(k.main.1, &camera_values, &light)?,
            )?;
        }
        for (render, cascade) in self.shadow_renders.iter().zip(&fitted) {
            write(
                r,
                render,
                SHADOW_VS,
                "renderStruct",
                &named(&[
                    ("cameraProjectionMatrix", m4(cascade.projection)),
                    ("cameraViewMatrix", m4(cascade.view)),
                ]),
            )?;
        }
        write(
            r,
            &self.line_render,
            LINE_VS,
            "renderStruct",
            &named(&camera_values[..2]),
        )?;
        let line_color = Color::from_hex(0xaaccee).0.to_array().to_vec();
        for item in &self.items {
            let k = &self.kinds[item.kind];
            let model = group * item.local;
            write(
                r,
                &item.object,
                k.main.1,
                "objectStruct",
                &object_values(k.main.0, k.main.1, Some(k), model)?,
            )?;
            write(
                r,
                &item.shadow_object,
                SHADOW_VS,
                "objectStruct",
                &object_values(SHADOW_VS, SHADOW_FS, None, model)?,
            )?;
            write(
                r,
                &item.line_object,
                LINE_FS,
                "objectStruct",
                &named(&[
                    ("nodeUniform0", line_color.clone()),
                    ("nodeUniform1", vec![1.]),
                    ("nodeUniform4", m4(model)),
                ]),
            )?;
        }
        write(
            r,
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &named(&[
                ("nodeUniform1", vec![1.]),
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform0", vec![t.width as f64, t.height as f64]),
            ]),
        )?;
        write(
            r,
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &named(&[
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
            ]),
        )?;
        write(
            r,
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &named(&[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![1.]),
            ]),
        )?;
        write(
            r,
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &named(&[("nodeUniform5", m4(Matrix4::IDENTITY))]),
        )?;
        let world_sphere = |item: &Item| {
            let model = group * item.local;
            let sphere = self.meshes[item.mesh].sphere;
            let (scale, _, _) = model.to_scale_rotation_translation();
            Sphere {
                center: model.transform_point3(sphere.center),
                radius: sphere.radius * scale.max_element(),
            }
        };
        // The SunLight shadow: the atlas cleared, then each cascade's tile.
        let shadow_pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("loft shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.shadow_color,
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
                        view: &self.shadow_depth,
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
        drop(shadow_pass(&mut encoder, true));
        for (i, cascade) in fitted.iter().enumerate() {
            let mut pass = shadow_pass(&mut encoder, false);
            let [x, y, w, h] = cascade.viewport;
            pass.set_viewport(x, y, w, h, 0., 1.);
            let frustum = Frustum::from_projection(cascade.projection * cascade.view);
            for item in self.items.iter().filter(|item| self.shown(item)) {
                let Some(draw) = item.shadow.get(i) else {
                    continue;
                };
                if frustum.intersects_sphere(world_sphere(item)) {
                    set(&mut pass, draw);
                    let vs = self.kinds[item.kind].shadow.map_or(SHADOW_VS, |s| s.0);
                    self.meshes[item.mesh].draw(&mut pass, vs);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("loft scene"),
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
            pass.set_vertex_buffer(0, self.background_mesh.0.slice(..));
            pass.set_index_buffer(self.background_mesh.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background_mesh.2, 0, 0..1);
            let frustum = Frustum::from_projection(projection * view);
            for (item, draw) in self.items.iter().zip(&t.items) {
                if self.shown(item) && frustum.intersects_sphere(world_sphere(item)) {
                    set(&mut pass, draw);
                    self.meshes[item.mesh].draw(&mut pass, self.kinds[item.kind].main.0);
                }
            }
            if self.sections {
                for (item, draw) in self.items.iter().zip(&t.lines) {
                    let Some((buffer, count)) = &self.meshes[item.mesh].skeleton else {
                        continue;
                    };
                    set(&mut pass, draw);
                    pass.set_vertex_buffer(0, buffer.slice(..));
                    pass.draw(0..*count, 0..1);
                }
            }
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
        self.controls.update(s, c)
    }
    /// sections: the loft section rings in place of the lofts.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.sections = value > 0.5,
            _ => return Err(Error::Invalid("loft parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
