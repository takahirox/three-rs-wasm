//! webgpu_loader_materialx: 36 ShaderBall.glb models in a 6 × 6 grid, each
//! with one MaterialXLoader material ( twelve MaterialX standard-surface
//! samples and the page's 24 local test documents ), lit by the
//! san_giuseppe_bridge PMREM ( lodMax 9 ) over the transparent grid ground.
//! The documents are fetched and parsed at load by MaterialXLoader ported
//! to Rust ( the three-tsl crate's Wasm, web/tsl-pkg: r186's
//! examples/jsm/loaders/MaterialX* and the node system transpiled ), which
//! builds each material in the page's scene; the renderer's node builder
//! there generates the ground's and the materials' WGSL and their texture
//! bindings, which this module draws with. The ShaderBall's meshes are
//! de-indexed with MikkTSpace tangents, as the page's computePrefabTangents
//! leaves them. The frame draws the opaque balls front to back, then ( when
//! a transmissive ball is visible ) copies the frame into the mipmapped
//! transmission texture, draws the transmissive balls' back faces, then the
//! transparent list back to front, and the LinearToneMapping output
//! ( exposure 0.5 ). The PMREM, mipmap and output stages run the WGSL r186
//! generates for them ( the PMREM, mipmap and output vertex modules are the
//! deferred, shadowmap_opacity and compute_cloth ones, which are
//! byte-identical ).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, culled_pipeline, set};
use super::generator_city::geo::normal_matrix;
use super::generator_city::uniforms::{fields, inverse, uses};
use super::gltf_viewer::{decode_image, fetch};
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::{Pmrem, bind, raw_pipeline, rgbe_pmrem_at};
use super::retro::{mipmapped, target, uniform};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const OUTPUT_VS: &str = include_str!("compute_cloth/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("loader_materialx/output_fs.wgsl");
const GGX_9: (&str, &str) = (
    include_str!("deferred/pmrem_ggx_vs.wgsl"),
    include_str!("deferred/pmrem_ggx_fs.wgsl"),
);
/// How the material renders: opaque, transparent ( front faces ) or
/// transmissive ( double-sided, transparent ).
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Opaque,
    Transparent,
    Transmissive,
}
/// A texture binding: one of the documents' images ( with its wrapS and
/// wrapT ), the DFG LUT, the PMREM or the transmission ( viewport mip )
/// texture. Its sampler is bound at the binding before it.
#[derive(Clone, Copy)]
enum Tex {
    Image(usize, wgpu::AddressMode, wgpu::AddressMode),
    Dfg,
    Pmrem,
    Transmission,
}
use Kind::*;
use Tex::*;
/// The page's documents: the MaterialX repository's standard-surface
/// samples ( loaded from GitHub on the page, from the gallery's copy of its
/// resources here ), then its local test documents ( examples/materialx/ ).
const SAMPLES: [&str; 12] = [
    "brass_tiled",
    "brick_procedural",
    "carpaint",
    "chrome",
    "copper",
    "gold",
    "jade",
    "marble_solid",
    "metal_brushed",
    "plastic",
    "velvet",
    "wood_tiled",
];
const LOCAL_SAMPLES: [&str; 24] = [
    "standard_surface_heightnormal",
    "standard_surface_conditional_if_float",
    "standard_surface_image_transform",
    "standard_surface_color3_vec3_cm_test",
    "standard_surface_rotate2d_test",
    "standard_surface_rotate3d_test",
    "standard_surface_heighttonormal_normal_input",
    "standard_surface_roughness_test",
    "standard_surface_opacity_test",
    "standard_surface_opacity_only_test",
    "standard_surface_specular_test",
    "standard_surface_ior_test",
    "standard_surface_combined_test",
    "standard_surface_texture_opacity_test",
    "standard_surface_transmission_test",
    "standard_surface_transmission_only_test",
    "standard_surface_transmission_rough",
    "standard_surface_thin_film_rainbow_test",
    "standard_surface_thin_film_ior_clamp_test",
    "standard_surface_sheen_test",
    "gltf_pbr_glass_dispersion",
    "open_pbr_surface_velvet",
    "open_pbr_surface_pearl",
    "open_pbr_surface_honey",
];
const SAMPLE_PATH: &str = "https://raw.githubusercontent.com/materialx/MaterialX/main/resources/Materials/Examples/StandardSurface/";
const LOCAL_PATH: &str = "materialx/";

#[wasm_bindgen(inline_js = r#"
// MaterialXLoader and the renderer's node builder run on the three-tsl
// crate's Wasm ( web/tsl-pkg: three r186 and its addons transpiled to Rust ).
let tsl;
export async function materialxShaders( texts, paths ) {
	if ( tsl === undefined ) {
		const revision = 'tsl-2';
		tsl = await import( new URL( `../tsl-pkg/three_tsl.js?v=${ revision }`, document.baseURI ) );
		await tsl.default( { module_or_path: new URL( `../tsl-pkg/three_tsl_bg.wasm?v=${ revision }`, document.baseURI ) } );
	}
	return tsl.materialx_shaders( texts, paths, innerWidth, innerHeight );
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = materialxShaders)]
    fn materialx_shaders(
        texts: &js_sys::Array,
        paths: &js_sys::Array,
    ) -> std::result::Result<js_sys::Promise, JsValue>;
}

/// One sample: its document's material's WGSL, how it renders and its
/// texture bindings.
struct Sample {
    name: String,
    vs: String,
    fs: String,
    kind: Kind,
    textures: Vec<(u32, Tex)>,
}
fn js_error(e: JsValue) -> Error {
    Error::Asset(format!("THREE.MaterialXLoader: {e:?}"))
}
fn field(o: &JsValue, key: &str) -> Result<JsValue> {
    js_sys::Reflect::get(o, &JsValue::from_str(key)).map_err(js_error)
}
fn text(o: &JsValue, key: &str) -> Result<String> {
    field(o, key)?
        .as_string()
        .ok_or_else(|| Error::Asset(format!("THREE.MaterialXLoader: {key}")))
}
/// A path relative to `base` with its `..` segments resolved.
fn resolve(base: &str, uri: &str) -> String {
    let mut parts: Vec<&str> = base.split('/').filter(|p| !p.is_empty()).collect();
    for p in uri.split('/') {
        match p {
            ".." => {
                parts.pop();
            }
            "." | "" => {}
            p => parts.push(p),
        }
    }
    parts.join("/")
}
fn address(wrap: f64) -> Result<wgpu::AddressMode> {
    Ok(match wrap as u32 {
        1000 => wgpu::AddressMode::Repeat,
        1001 => wgpu::AddressMode::ClampToEdge,
        1002 => wgpu::AddressMode::MirrorRepeat,
        _ => return Err(Error::Invalid("materialx wrap")),
    })
}
/// The ground's shaders and the samples, as the page's MaterialXLoader
/// builds them from the documents, with the images their bindings sample
/// ( paths under assets/materialx/ ).
async fn load_samples() -> Result<((String, String), Vec<Sample>, Vec<String>)> {
    let texts = js_sys::Array::new();
    let paths = js_sys::Array::new();
    // ( name, the document's directory in assets/materialx/ )
    let mut documents = vec![];
    for name in SAMPLES {
        let dir = "resources/Materials/Examples/StandardSurface/";
        documents.push((
            name.to_string(),
            format!("{dir}standard_surface_{name}.mtlx"),
            dir,
        ));
        paths.push(&JsValue::from_str(SAMPLE_PATH));
    }
    for name in LOCAL_SAMPLES {
        let short = match (
            name.strip_prefix("standard_surface_"),
            name.strip_prefix("open_pbr_surface_"),
        ) {
            (Some(r), _) => r.to_string(),
            (_, Some(r)) => format!("open_pbr_{r}"),
            _ => name.to_string(),
        };
        documents.push((short, format!("three/{name}.mtlx"), "three/"));
        paths.push(&JsValue::from_str(LOCAL_PATH));
    }
    for (_, file, _) in &documents {
        let bytes = fetch(&format!("/web/gallery/assets/materialx/{file}")).await?;
        let document =
            String::from_utf8(bytes).map_err(|_| Error::Invalid("materialx document"))?;
        texts.push(&JsValue::from_str(&document));
    }
    let page = JsFuture::from(materialx_shaders(&texts, &paths).map_err(js_error)?)
        .await
        .map_err(js_error)?;
    let grid = field(&page, "grid")?;
    let grid = (text(&grid, "vertexShader")?, text(&grid, "fragmentShader")?);
    let materials = field(&page, "materials")?;
    let mut images: Vec<String> = vec![];
    let mut samples = vec![];
    for (i, (name, _, dir)) in documents.into_iter().enumerate() {
        let material = js_sys::Reflect::get_u32(&materials, i as u32).map_err(js_error)?;
        if let Some(e) = field(&material, "error")?.as_string() {
            return Err(Error::Asset(format!("THREE.MaterialXLoader: {name}: {e}")));
        }
        let mut textures = vec![];
        let mut transmissive = false;
        for b in js_sys::Array::from(&field(&material, "textures")?).iter() {
            let binding = field(&b, "binding")?
                .as_f64()
                .ok_or(Error::Invalid("materialx binding"))? as u32;
            let tex = match text(&b, "source")?.as_str() {
                "image" => {
                    let path = resolve(dir, &text(&b, "uri")?);
                    let k = match images.iter().position(|p| *p == path) {
                        Some(k) => k,
                        None => {
                            images.push(path);
                            images.len() - 1
                        }
                    };
                    let wrap = |key| address(field(&b, key)?.as_f64().unwrap_or(0.));
                    Image(k, wrap("wrapS")?, wrap("wrapT")?)
                }
                "dfg" => Dfg,
                "pmrem" => Pmrem,
                "transmission" => {
                    transmissive = true;
                    Transmission
                }
                _ => return Err(Error::Invalid("materialx texture")),
            };
            textures.push((binding, tex));
        }
        let kind = if transmissive {
            Transmissive
        } else if field(&material, "transparent")?.is_truthy() {
            Transparent
        } else {
            Opaque
        };
        samples.push(Sample {
            name,
            vs: text(&material, "vertexShader")?,
            fs: text(&material, "fragmentShader")?,
            kind,
            textures,
        });
    }
    Ok((grid, samples, images))
}
/// The ShaderBall's vertex attributes, each in its own buffer as
/// toNonIndexed leaves them and r186 uploads them.
#[derive(Clone, Copy)]
enum Attribute {
    Position,
    Normal,
    Uv,
    Tangent,
}
impl Attribute {
    fn layout(self) -> (u64, wgpu::VertexFormat) {
        match self {
            // Int16 items of three, padded to four for WebGPU.
            Attribute::Position | Attribute::Normal => (8, wgpu::VertexFormat::Snorm16x4),
            Attribute::Uv => (4, wgpu::VertexFormat::Unorm16x2),
            Attribute::Tangent => (16, wgpu::VertexFormat::Float32x4),
        }
    }
}
/// A vertex stage's inputs ( `@location( n ) name :` ) in location order.
fn inputs(vs: &str) -> Result<Vec<Attribute>> {
    let start = vs
        .find("fn main(")
        .ok_or(Error::Invalid("materialx vertex main"))?;
    let header = &vs[start..start + vs[start..].find("->").unwrap_or(0)];
    let mut out = vec![];
    for (location, part) in header.split("@location( ").skip(1).enumerate() {
        let (n, rest) = part
            .split_once(" ) ")
            .ok_or(Error::Invalid("materialx location"))?;
        if n.parse::<usize>().ok() != Some(location) {
            return Err(Error::Invalid("materialx location order"));
        }
        out.push(match rest.split(' ').next() {
            Some("position") => Attribute::Position,
            Some("normal") => Attribute::Normal,
            Some("uv") => Attribute::Uv,
            Some("tangent") => Attribute::Tangent,
            _ => return Err(Error::Invalid("materialx vertex input")),
        });
    }
    Ok(out)
}
/// The `var<uniform> object` binding of group 1.
fn object_binding(source: &str) -> Result<u32> {
    let end = source
        .find("var<uniform> object :")
        .ok_or(Error::Invalid("materialx object binding"))?;
    let start = source[..end]
        .rfind("@binding( ")
        .ok_or(Error::Invalid("materialx object binding"))?;
    source[start + 10..end]
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .ok_or(Error::Invalid("materialx object binding"))
}
/// The object struct's values, recognized by how the stages use each
/// field: the model and normal matrices, the PMREM's rotation, texel sizes
/// and lodMax, and the physical material's defaults ( opacity 1, ior 1.5,
/// metalness 0, specularIntensity 1, emissive 0 at intensity 1, attenuation
/// white at infinity, environment intensity 1 ). The loader sets thickness 1
/// for transmissive materials without a thickness input.
fn object_values(vs: &str, fs: &str, model: &[f64; 16]) -> Result<Vec<(String, Vec<f64>)>> {
    let mut out = vec![];
    for (name, ty) in fields(fs, "objectStruct") {
        let lines = uses(&[fs, vs], "object", &name);
        let any = |p: &str| {
            let p = p.replace('#', &format!("object.{name}"));
            lines.iter().any(|l| l.contains(&p))
        };
        let v = match ty.as_str() {
            "mat3x3<f32>" => normal_matrix(model).to_vec(),
            "mat4x4<f32>" if any("getFace( ( #") => m4(Matrix4::IDENTITY),
            "mat4x4<f32>" => model.to_vec(),
            "vec3<f32>" if any("AttenuationColor = #") => vec![1.; 3],
            "vec3<f32>" if any("EmissiveColor = ( # *") => vec![0.; 3],
            "f32" if any("DiffuseColor.w * #") || any("* vec3<f32>( # ) )") => vec![1.],
            "f32" if any("IOR = #;") => vec![1.5],
            "f32" if any("Metalness = #;") => vec![0.],
            "f32" if any("Thickness = #;") => vec![1.],
            "f32" if any("AttenuationDistance = #;") => vec![f64::INFINITY],
            "f32" if any("SpecularColor = ( min(") => vec![1.],
            "f32" if any("-2.0, # )") => vec![9.],
            "f32" if any(".x * # )") => vec![1. / 1536.],
            "f32" if any(".y * # )") => vec![1. / 2048.],
            _ => return Err(Error::Asset(format!("materialx object uniform {name}"))),
        };
        out.push((name, v));
    }
    Ok(out)
}
/// What a render struct field holds: the camera's matrices and position,
/// the drawing buffer size ( the transmission's viewport ) or TSL time.
#[derive(Clone, Copy)]
enum RenderSlot {
    Projection,
    View,
    World,
    Position,
    Viewport,
    Time,
}
fn render_layout(source: &str) -> Result<Vec<(String, RenderSlot)>> {
    fields(source, "renderStruct")
        .into_iter()
        .map(|(name, ty)| {
            let slot = match (name.as_str(), ty.as_str()) {
                ("cameraProjectionMatrix", _) => RenderSlot::Projection,
                ("cameraViewMatrix", _) => RenderSlot::View,
                ("cameraWorldMatrix", _) => RenderSlot::World,
                ("cameraPosition", _) => RenderSlot::Position,
                (_, "vec2<f32>") => RenderSlot::Viewport,
                (_, "f32") => RenderSlot::Time,
                _ => return Err(Error::Asset(format!("materialx render uniform {name}"))),
            };
            Ok((name, slot))
        })
        .collect()
}
/// One of the ShaderBall's two meshes, de-indexed: its attribute buffers,
/// vertex count, node transform and local bounding sphere.
struct Part {
    buffers: [wgpu::Buffer; 4],
    count: u32,
    node: Matrix4,
    sphere: Sphere,
}
/// computeMikkTSpaceTangents' input: the denormalized, de-indexed
/// attributes as Float32Arrays; its output, the tangents.
struct Tangents {
    position: Vec<f32>,
    normal: Vec<f32>,
    uv: Vec<f32>,
    out: Vec<f32>,
}
impl mikktspace::Geometry for Tangents {
    fn num_faces(&self) -> usize {
        self.position.len() / 9
    }
    fn num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }
    fn position(&self, face: usize, vert: usize) -> [f32; 3] {
        let i = (face * 3 + vert) * 3;
        [self.position[i], self.position[i + 1], self.position[i + 2]]
    }
    fn normal(&self, face: usize, vert: usize) -> [f32; 3] {
        let i = (face * 3 + vert) * 3;
        [self.normal[i], self.normal[i + 1], self.normal[i + 2]]
    }
    fn tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
        let i = (face * 3 + vert) * 2;
        [self.uv[i], self.uv[i + 1]]
    }
    fn set_tangent_encoded(&mut self, tangent: [f32; 4], face: usize, vert: usize) {
        let i = (face * 3 + vert) * 4;
        self.out[i..i + 4].copy_from_slice(&tangent);
    }
}
/// A ball's mesh: its model, part ( 0 calibration, 1 preview ), world
/// model, object struct and render group.
struct Ball {
    sample: usize,
    part: usize,
    model: Matrix4,
    object: wgpu::Buffer,
    render: usize,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::Texture,
    color_view: wgpu::TextureView,
    depth: wgpu::TextureView,
    transmission: wgpu::Texture,
    transmission_levels: Vec<(wgpu::TextureView, wgpu::BindGroup)>,
    /// Per ball, its front-face draw and ( transmissive ) back-face draw.
    balls: Vec<(Draw, Option<Draw>)>,
    ground: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// The example clock: TSL time.
    time: f64,
    /// Calibration and preview mesh visibility.
    params: [f64; 2],
    parts: [Part; 2],
    /// The documents' materials in the page's order.
    samples: Vec<Sample>,
    /// The ground's vertex and fragment WGSL.
    grid: (String, String),
    /// Per sample, its vertex inputs in location order.
    attributes: Vec<Vec<Attribute>>,
    balls: Vec<Ball>,
    images: Vec<wgpu::TextureView>,
    ground: (wgpu::Buffer, wgpu::Buffer, u32, wgpu::Buffer),
    pmrem: Pmrem,
    /// One render struct per distinct layout: its source and buffer.
    renders: Vec<(String, wgpu::Buffer)>,
    ground_render: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    /// The images' samplers by wrapS and wrapT.
    image_samplers: Vec<((wgpu::AddressMode, wgpu::AddressMode), wgpu::Sampler)>,
    viewport_sampler: wgpu::Sampler,
    mipmaps: Mipmaps,
    targets: Option<Targets>,
}
/// The glTF bufferView `view` of buffer 0.
fn view_bytes<'a>(asset: &gltf::Gltf, blob: &'a [u8], view: usize) -> Result<&'a [u8]> {
    let v = asset
        .views()
        .nth(view)
        .ok_or(Error::Invalid("ShaderBall view"))?;
    blob.get(v.offset()..v.offset() + v.length())
        .ok_or(Error::Invalid("ShaderBall buffer"))
}
/// Vector3.applyMatrix4's projected z.
fn projected_z(m: &Matrix4, p: Vector3) -> f64 {
    let e = m.to_cols_array();
    let w = 1. / (e[3] * p.x + e[7] * p.y + e[11] * p.z + e[15]);
    (e[2] * p.x + e[6] * p.y + e[10] * p.z + e[14]) * w
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.25,
            far: 200.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(10., 10., 20.);
        let mut controls = Controls::new(Some(0.05), (2., 40.), PI, true);
        controls.set_target(Vector3::ZERO);
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let asset = gltf::Gltf::from_slice_without_validation(
            &fetch("/web/gallery/assets/gltf/ShaderBall.glb").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let blob = asset
            .blob
            .clone()
            .ok_or(Error::Invalid("ShaderBall blob"))?;
        let mut parts = vec![];
        for name in ["Calibration_Mesh", "Preview_Mesh"] {
            let node = asset
                .nodes()
                .find(|n| n.name() == Some(name))
                .ok_or(Error::Invalid("ShaderBall node"))?;
            let primitive = node
                .mesh()
                .and_then(|m| m.primitives().next())
                .ok_or(Error::Invalid("ShaderBall mesh"))?;
            let position = primitive
                .get(&gltf::Semantic::Positions)
                .ok_or(Error::Invalid("ShaderBall position"))?;
            let view = position.view().ok_or(Error::Invalid("ShaderBall view"))?;
            // Position and normal snorm16 at 0 and 8, uv unorm16 at 16.
            let vertices = view_bytes(&asset, &blob, view.index())?;
            let vertices = vertices.as_chunks::<20>().0;
            let indices = primitive
                .indices()
                .ok_or(Error::Invalid("ShaderBall index"))?;
            let index_view = indices
                .view()
                .ok_or(Error::Invalid("ShaderBall index view"))?;
            let index_bytes = view_bytes(&asset, &blob, index_view.index())?;
            let start = indices.offset();
            let index: Vec<usize> = index_bytes[start..start + indices.count() * 2]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|b| u16::from_le_bytes(*b) as usize)
                .collect();
            let vertex = |i: usize| vertices.get(i).ok_or(Error::Invalid("ShaderBall vertex"));
            let short = |v: &[u8; 20], k: usize| i16::from_le_bytes([v[k], v[k + 1]]);
            let unsigned = |v: &[u8; 20], k: usize| u16::from_le_bytes([v[k], v[k + 1]]);
            // toNonIndexed: each attribute in its own array of the same type.
            let (mut positions, mut normals, mut uvs) = (vec![], vec![], vec![]);
            for &i in &index {
                let v = vertex(i)?;
                positions.extend([short(v, 0), short(v, 2), short(v, 4), 0]);
                normals.extend([short(v, 8), short(v, 10), short(v, 12), 0]);
                uvs.extend([unsigned(v, 16), unsigned(v, 18)]);
            }
            // getX() denormalizes into the Float32Arrays MikkTSpace reads.
            let signed = |a: &[i16]| -> Vec<f32> {
                a.chunks(4)
                    .flat_map(|c| c[..3].iter().map(|&x| (x as f64 / 32767.).max(-1.) as f32))
                    .collect()
            };
            let mut tangents = Tangents {
                position: signed(&positions),
                normal: signed(&normals),
                uv: uvs.iter().map(|&x| (x as f64 / 65535.) as f32).collect(),
                out: vec![0.; index.len() * 4],
            };
            if !mikktspace::generate_tangents(&mut tangents) {
                return Err(Error::Invalid("ShaderBall tangents"));
            }
            // negateSign.
            for w in tangents.out.iter_mut().skip(3).step_by(4) {
                *w = -*w;
            }
            // computeBoundingSphere over the denormalized positions.
            let points: Vec<Vector3> = tangents
                .position
                .chunks(3)
                .zip(positions.chunks(4))
                .map(|(_, p)| {
                    let c = |k: usize| (p[k] as f64 / 32767.).max(-1.);
                    Vector3::new(c(0), c(1), c(2))
                })
                .collect();
            let (lo, hi) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) * 0.5;
            let radius = points
                .iter()
                .map(|p| p.distance_squared(center))
                .fold(0., f64::max)
                .sqrt();
            let (t, _, sc) = node.transform().decomposed();
            let vertex_buffer = |label, data: &[u8]| init(label, data, wgpu::BufferUsages::VERTEX);
            parts.push(Part {
                buffers: [
                    vertex_buffer("ShaderBall position", bytemuck::cast_slice(&positions)),
                    vertex_buffer("ShaderBall normal", bytemuck::cast_slice(&normals)),
                    vertex_buffer("ShaderBall uv", bytemuck::cast_slice(&uvs)),
                    vertex_buffer("ShaderBall tangent", bytemuck::cast_slice(&tangents.out)),
                ],
                count: index.len() as u32,
                // The nodes carry a translation and a uniform scale.
                node: Matrix4::from_cols_array(&[
                    sc[0] as f64,
                    0.,
                    0.,
                    0.,
                    0.,
                    sc[1] as f64,
                    0.,
                    0.,
                    0.,
                    0.,
                    sc[2] as f64,
                    0.,
                    t[0] as f64,
                    t[1] as f64,
                    t[2] as f64,
                    1.,
                ]),
                sphere: Sphere { center, radius },
            });
        }
        let parts: [Part; 2] = parts
            .try_into()
            .map_err(|_| Error::Invalid("ShaderBall parts"))?;
        // The documents' images: decoded without color conversion or flip
        // ( ImageBitmapLoader with imageOrientation none ), mipmapped.
        let mut mipmaps = Mipmaps::new(r);
        let (grid, samples, image_paths) = load_samples().await?;
        let mut images = vec![];
        for path in &image_paths {
            let mut image =
                decode_image(&fetch(&format!("/web/gallery/assets/materialx/{path}")).await?)
                    .await?;
            image.srgb = false;
            images.push(mipmapped(
                r,
                &mut mipmaps,
                &image,
                wgpu::TextureFormat::Rgba8Unorm,
            ));
        }
        // Distinct render structs share one buffer.
        let mut renders: Vec<(String, wgpu::Buffer)> = vec![];
        let struct_body = |source: &str| fields(source, "renderStruct");
        // updateModelsAlign(): six columns, 3 apart.
        let line_count = (samples.len() / 6) as f64 - 1.5;
        let (offset_x, offset_z) = (3. * 5. * -0.5, 3. * line_count * 0.5);
        let mut balls = vec![];
        for (i, sample) in samples.iter().enumerate() {
            render_layout(&sample.fs)?;
            let render = match renders
                .iter()
                .position(|(source, _)| struct_body(source) == struct_body(&sample.fs))
            {
                Some(k) => k,
                None => {
                    renders.push((
                        sample.fs.clone(),
                        uniform(r, "materialx render", &sample.fs, "renderStruct")?,
                    ));
                    renders.len() - 1
                }
            };
            let position = Vector3::new(
                (i % 6) as f64 * 3. + offset_x,
                0.,
                (i / 6) as f64 * -3. + offset_z,
            );
            for (p, part) in parts.iter().enumerate() {
                let model = Matrix4::from_translation(position) * part.node;
                let values = object_values(&sample.vs, &sample.fs, &model.to_cols_array())?;
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
                let object = init(
                    "materialx object",
                    &pack(&sample.fs, "objectStruct", &values)?,
                    wgpu::BufferUsages::UNIFORM,
                );
                balls.push(Ball {
                    sample: i,
                    part: p,
                    model,
                    object,
                    render,
                });
            }
        }
        // The ground: CircleGeometry( 40 ) turned flat.
        let circle = CircleGeometry::build(40., 32, 0., 2. * PI)?;
        let circle_positions = circle
            .attributes
            .get("position")
            .ok_or(Error::Invalid("circle"))?;
        let circle_positions: Vec<f32> = (0..circle_positions.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| circle_positions.get_component(i, k).map(|v| v as f32))
            .collect::<Result<_>>()?;
        let circle_index = circle.index.clone().ok_or(Error::Invalid("circle index"))?;
        let ground_object = init(
            "materialx ground",
            &pack(
                &grid.1,
                "objectStruct",
                &[
                    ("nodeUniform0", &m4(Matrix4::from_rotation_x(-PI / 2.))),
                    ("nodeUniform1", &[1.]),
                ],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let ground = (
            init(
                "ground positions",
                bytemuck::cast_slice(&circle_positions),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "ground index",
                bytemuck::cast_slice(&circle_index),
                wgpu::BufferUsages::INDEX,
            ),
            circle_index.len() as u32,
            ground_object,
        );
        let sampler = |address, mag, mip| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: mag,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mip,
                ..Default::default()
            })
        };
        let clamp = sampler(
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Nearest,
        );
        // scene.environment: the equirect HDR through PMREMGenerator.fromEquirectangular.
        let (pmrem, _) = rgbe_pmrem_at(
            r,
            &clamp,
            "/web/gallery/assets/tsl-procedural/san_giuseppe_bridge_2k.hdr",
            Some((9, GGX_9)),
        )
        .await?;
        let mut image_samplers = vec![];
        for s in &samples {
            for &(_, t) in &s.textures {
                if let Image(_, u, v) = t
                    && !image_samplers.iter().any(|(w, _)| *w == (u, v))
                {
                    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
                        address_mode_u: u,
                        address_mode_v: v,
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Linear,
                        mipmap_filter: wgpu::FilterMode::Linear,
                        ..Default::default()
                    });
                    image_samplers.push(((u, v), sampler));
                }
            }
        }
        Ok(Self {
            controls,
            time: 0.,
            params: [1., 1.],
            parts,
            attributes: samples
                .iter()
                .map(|s| inputs(&s.vs))
                .collect::<Result<_>>()?,
            balls,
            images,
            ground,
            pmrem,
            renders,
            ground_render: uniform(r, "materialx ground render", &grid.0, "renderStruct")?,
            output_render: uniform(r, "materialx output render", OUTPUT_FS, "renderStruct")?,
            output_object: init(
                "materialx output object",
                &pack(
                    OUTPUT_VS,
                    "objectStruct",
                    &[("nodeUniform5", &m4(Matrix4::IDENTITY))],
                )?,
                wgpu::BufferUsages::UNIFORM,
            ),
            output_quad: init(
                "materialx quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            image_samplers,
            viewport_sampler: sampler(
                wgpu::AddressMode::ClampToEdge,
                wgpu::FilterMode::Nearest,
                wgpu::FilterMode::Linear,
            ),
            clamp,
            samples,
            grid,
            mipmaps,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.frame_update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let depth = Some((wgpu::CompareFunction::LessEqual, true));
        // The transmission ( viewport mip ) texture: the drawing buffer's size.
        let transmission = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("materialx transmission"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: mip_count(size),
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HALF,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let transmission_levels = self.mipmaps.levels(r, &transmission);
        let transmission_view = transmission.create_view(&Default::default());
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        // Per sample, its front ( and back ) pipeline.
        let mut pipelines = vec![];
        for (sample, attributes) in self.samples.iter().zip(&self.attributes) {
            let formats: Vec<[wgpu::VertexAttribute; 1]> = attributes
                .iter()
                .enumerate()
                .map(|(location, a)| {
                    [wgpu::VertexAttribute {
                        format: a.layout().1,
                        offset: 0,
                        shader_location: location as u32,
                    }]
                })
                .collect();
            let layouts: Vec<wgpu::VertexBufferLayout> = attributes
                .iter()
                .zip(&formats)
                .map(|(a, f)| wgpu::VertexBufferLayout {
                    array_stride: a.layout().0,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: f,
                })
                .collect();
            let pipeline = |cw| {
                culled_pipeline(
                    r,
                    &sample.name,
                    (&sample.vs, &sample.fs),
                    &layouts,
                    &[HALF],
                    depth,
                    (cw, sample.kind != Opaque),
                    (samples, wgpu::PrimitiveTopology::TriangleList),
                    Some(wgpu::Face::Back),
                )
            };
            pipelines.push((
                pipeline(false),
                (sample.kind == Transmissive).then(|| pipeline(true)),
            ));
        }
        let mut balls = vec![];
        for b in &self.balls {
            let sample = &self.samples[b.sample];
            let mut entries = vec![(object_binding(&sample.fs)?, b.object.as_entire_binding())];
            for &(binding, t) in &sample.textures {
                let (s, view) = match t {
                    Image(k, u, v) => (
                        self.image_samplers
                            .iter()
                            .find(|(w, _)| *w == (u, v))
                            .map(|(_, s)| s)
                            .ok_or(Error::Invalid("materialx sampler"))?,
                        self.images
                            .get(k)
                            .ok_or(Error::Invalid("materialx image"))?,
                    ),
                    Dfg => (&self.clamp, &r.dfg),
                    Pmrem => (&self.clamp, &self.pmrem.view),
                    Transmission => (&self.viewport_sampler, &transmission_view),
                };
                entries.extend([(binding - 1, sampler(s)), (binding, tex(view))]);
            }
            let draw = |p: &wgpu::RenderPipeline| -> Draw {
                (
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, self.renders[b.render].1.as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries),
                    ],
                )
            };
            let (front, back) = &pipelines[b.sample];
            balls.push((draw(front), back.as_ref().map(draw)));
        }
        let position = [wgpu::vertex_attr_array![0 => Float32x3]];
        let ground_pipeline = culled_pipeline(
            r,
            "materialx ground",
            (&self.grid.0, &self.grid.1),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position[0],
            }],
            &[HALF],
            depth,
            (false, true),
            (samples, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
        );
        let ground = (
            ground_pipeline.clone(),
            vec![
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(0),
                    &[(0, self.ground_render.as_entire_binding())],
                ),
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(1),
                    &[(0, self.ground.3.as_entire_binding())],
                ),
            ],
        );
        let color = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("materialx color"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HALF,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let color_view = color.create_view(&Default::default());
        let output_pipeline = raw_pipeline(
            r,
            "materialx output",
            OUTPUT_VS,
            OUTPUT_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position[0],
            }],
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
                        (0, sampler(&self.clamp)),
                        (1, tex(&color_view)),
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
            color_view,
            transmission,
            transmission_levels,
            balls,
            ground,
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
            .ok_or(Error::Invalid("materialx targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?;
        let view = Matrix4::from_cols_array(&inverse(&world.to_cols_array()));
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
        for (source, buffer) in &self.renders {
            let values: Vec<(String, Vec<f64>)> = render_layout(source)?
                .into_iter()
                .map(|(name, slot)| {
                    let v = match slot {
                        RenderSlot::Projection => m4(projection),
                        RenderSlot::View => m4(view),
                        RenderSlot::World => m4(world),
                        RenderSlot::Position => world.w_axis.truncate().to_array().to_vec(),
                        RenderSlot::Viewport => vec![t.width as f64, t.height as f64],
                        RenderSlot::Time => vec![self.time],
                    };
                    (name, v)
                })
                .collect();
            write(buffer, source, "renderStruct", &values)?;
        }
        write(
            &self.ground_render,
            &self.grid.0,
            "renderStruct",
            &[
                ("cameraProjectionMatrix".into(), m4(projection)),
                ("cameraViewMatrix".into(), m4(view)),
            ],
        )?;
        write(
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
                ("nodeUniform2".into(), vec![0.5]),
            ],
        )?;
        // The render lists: frustum culled by bounding sphere, with each
        // object's projected z.
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        // ( renderOrder, z, id, ball ) with the ground as None.
        let mut opaque = vec![];
        let mut transparent = vec![];
        for (i, b) in self.balls.iter().enumerate() {
            let part = &self.parts[b.part];
            let (scale, _, translation) = b.model.to_scale_rotation_translation();
            if self.params[b.part] < 0.5
                || !frustum.intersects_sphere(Sphere {
                    center: b.model.transform_point3(part.sphere.center),
                    radius: part.sphere.radius * scale.max_element(),
                })
            {
                continue;
            }
            let z = projected_z(&screen, translation);
            match self.samples[b.sample].kind {
                Opaque => opaque.push((0, z, i, Some(i))),
                // Transparent materials' meshes take renderOrder 1 and 2.
                _ => transparent.push((b.part as i32 + 1, z, i, Some(i))),
            }
        }
        if frustum.intersects_sphere(Sphere {
            center: Vector3::ZERO,
            radius: 40.,
        }) {
            transparent.push((-1, projected_z(&screen, Vector3::ZERO), usize::MAX, None));
        }
        let order = |a: &(i32, f64, usize, Option<usize>), b: &(i32, f64, usize, Option<usize>)| {
            a.0.cmp(&b.0)
                .then(a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .then(a.2.cmp(&b.2))
        };
        opaque.sort_by(order);
        transparent.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then(b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
                .then(a.2.cmp(&b.2))
        });
        let double_pass: Vec<usize> = transparent
            .iter()
            .filter_map(|e| e.3)
            .filter(|&i| self.samples[self.balls[i].sample].kind == Transmissive)
            .collect();
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let scene_pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("materialx scene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: t.msaa.as_ref().map_or(&t.color_view, |m| &m.0),
                        depth_slice: None,
                        resolve_target: t.msaa.as_ref().map(|_| &t.color_view),
                        ops: wgpu::Operations {
                            load: if clear {
                                wgpu::LoadOp::Clear(wgpu::Color::WHITE)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
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
        let draw = |pass: &mut wgpu::RenderPass, i: usize, back: bool| {
            let b = &self.balls[i];
            let (front, back_draw) = &t.balls[i];
            let Some(d) = (if back {
                back_draw.as_ref()
            } else {
                Some(front)
            }) else {
                return;
            };
            set(pass, d);
            let part = &self.parts[b.part];
            for (slot, &a) in self.attributes[b.sample].iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, part.buffers[a as usize].slice(..));
            }
            pass.draw(0..part.count, 0..1);
        };
        let mut pass = scene_pass(&mut encoder, true);
        for e in &opaque {
            if let Some(i) = e.3 {
                draw(&mut pass, i, false);
            }
        }
        if !double_pass.is_empty() {
            // The transmission's viewport mip texture updates before its
            // first object: the resolved frame, copied and mipmapped.
            drop(pass);
            encoder.copy_texture_to_texture(
                t.color.as_image_copy(),
                t.transmission.as_image_copy(),
                t.color.size(),
            );
            self.mipmaps
                .encode(&mut encoder, HALF, &t.transmission_levels);
            pass = scene_pass(&mut encoder, false);
            for &i in &double_pass {
                draw(&mut pass, i, true);
            }
        }
        for e in &transparent {
            match e.3 {
                Some(i) => draw(&mut pass, i, false),
                None => {
                    set(&mut pass, &t.ground);
                    pass.set_vertex_buffer(0, self.ground.0.slice(..));
                    pass.set_index_buffer(self.ground.1.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.ground.2, 0, 0..1);
                }
            }
        }
        drop(pass);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("materialx output"),
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
    /// Calibration and preview mesh visibility.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("materialx parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
