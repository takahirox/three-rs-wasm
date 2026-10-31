//! webgpu_loader_materialx: 40 ShaderBall.glb models in rows of six, each
//! with one MaterialXLoader material ( twelve MaterialX standard-surface
//! samples and the page's 28 local test documents ), lit by the
//! san_giuseppe_bridge PMREM over the transparent grid ground; the
//! Geometry selector shows each material on its model's UV plane instead.
//! The documents are fetched and parsed at load by MaterialXLoader ported
//! to Rust ( the three-tsl crate's Wasm, web/build/tsl: r187's
//! examples/jsm/loaders/MaterialX* and the node system transpiled ), which
//! builds each material in the page's scene; the renderer's node builder
//! there generates the ground's and the materials' WGSL and their texture
//! bindings, which this module draws with. The documents' images decode as
//! the page's loaders read them ( ImageBitmapLoader, and the EXR, TIFF and
//! HDR loaders the page registers, ported ). The ShaderBall's meshes are
//! de-indexed with MikkTSpace tangents, as the page's computePrefabTangents
//! leaves them. The frame draws the opaque objects front to back, then
//! ( when a transmissive one is visible ) copies the frame into the back
//! sides' mipmapped transmission texture, draws the transmissive objects'
//! back sides, then the transparent list back to front, copying the frame
//! into the front sides' texture before its first transmissive object, and
//! the LinearToneMapping output ( exposure 0.5 ). The PMREM ( ../pmrem.rs ) and
//! output stages run the WGSL r187 generates for them.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, culled_pipeline, set};
use super::generator_city::geo::normal_matrix;
use super::generator_city::uniforms::{fields, inverse, uses};
use super::gltf_viewer::{decode_image, fetch};
use super::helpers_formats::formats::decode_tiff;
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::refraction_loaders::formats::decode_exr;
use super::retro::{mipmapped, target, uniform};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use super::trackball_sprites::parse_rgbe;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const OUTPUT_VS: &str = include_str!("loader_materialx/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("loader_materialx/output_fs.wgsl");
use super::pmrem;
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
const LOCAL_SAMPLES: [&str; 28] = [
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
    "standard_surface_image_exr_test",
    "standard_surface_image_tiff_test",
    "standard_surface_image_hdr_emission_test",
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
    "displacement",
];
const SAMPLE_PATH: &str = "https://raw.githubusercontent.com/materialx/MaterialX/main/resources/Materials/Examples/StandardSurface/";
const LOCAL_PATH: &str = "materialx/";

#[wasm_bindgen(inline_js = r#"
// MaterialXLoader and the renderer's node builder run on the three-tsl
// crate's Wasm ( web/build/tsl: three r187 and its addons transpiled to Rust ).
let tsl;
export async function materialxShaders( texts, paths ) {
	if ( tsl === undefined ) {
		const revision = 'tsl-3';
		tsl = await import( new URL( `../build/tsl/three_tsl.js?v=${ revision }`, document.baseURI ) );
		await tsl.default( { module_or_path: new URL( `../build/tsl/three_tsl_bg.wasm?v=${ revision }`, document.baseURI ) } );
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

/// A material's shaders on one geometry: its front side's WGSL ( its only
/// side's, unless it renders in two passes ), its back side's ( the
/// transmissive materials' ), and its texture bindings.
struct Shaders {
    vs: String,
    fs: String,
    back: Option<(String, String)>,
    textures: Vec<(u32, Tex)>,
}
/// One sample: its document's material on the ShaderBall and on the UV
/// plane, and how it renders.
struct Sample {
    name: String,
    kind: Kind,
    ball: Shaders,
    plane: Shaders,
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
/// The three.js examples' textures the local documents read
/// ( ../textures/… from examples/materialx/ ), where the gallery has them.
const TEXTURES: [(&str, &str); 6] = [
    ("textures/memorial.exr", "memorial.exr"),
    ("textures/memorial.hdr", "memorial.hdr"),
    (
        "textures/tiff/crate_uncompressed.tif",
        "tiff/crate_uncompressed.tif",
    ),
    (
        "textures/brick_diffuse.jpg",
        "tsl-next/textures/brick_diffuse.jpg",
    ),
    (
        "textures/brick_bump.jpg",
        "tsl-environment/textures/brick_bump.jpg",
    ),
    (
        "textures/brick_roughness.jpg",
        "materialx/textures/brick_roughness.jpg",
    ),
];
/// An image's path under assets/: the MaterialX repository's and the local
/// documents' own under materialx/, the examples' textures where listed.
fn image_asset(path: &str) -> String {
    match TEXTURES.iter().find(|(from, _)| *from == path) {
        Some((_, to)) => format!("assets/{to}"),
        None => format!("assets/materialx/{path}"),
    }
}
fn address(wrap: f64) -> Result<wgpu::AddressMode> {
    Ok(match wrap as u32 {
        1000 => wgpu::AddressMode::Repeat,
        1001 => wgpu::AddressMode::ClampToEdge,
        1002 => wgpu::AddressMode::MirrorRepeat,
        _ => return Err(Error::Invalid("materialx wrap")),
    })
}
/// One shader set of `materialx_shaders`' result: its stages, its back
/// side's, and its texture bindings, each image's path ( under the
/// documents' directory `dir` once the loader's `path` is taken off its
/// URI ) added to `images`.
fn shaders(
    set: &JsValue,
    path: &str,
    dir: &str,
    images: &mut Vec<String>,
) -> Result<(Shaders, bool)> {
    let mut textures = vec![];
    let mut transmissive = false;
    for b in js_sys::Array::from(&field(set, "textures")?).iter() {
        let binding = field(&b, "binding")?
            .as_f64()
            .ok_or(Error::Invalid("materialx binding"))? as u32;
        let tex = match text(&b, "source")?.as_str() {
            "image" => {
                // A handler's URI carries the document's path.
                let uri = text(&b, "uri")?;
                let path = resolve(dir, uri.strip_prefix(path).unwrap_or(&uri));
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
    let back = field(set, "back")?;
    let back = if back.is_null() || back.is_undefined() {
        None
    } else {
        Some((text(&back, "vertexShader")?, text(&back, "fragmentShader")?))
    };
    let fs = text(set, "fragmentShader")?;
    // The back side draws with the front's bindings.
    if let Some((_, back_fs)) = &back
        && fields(back_fs, "objectStruct") != fields(&fs, "objectStruct")
    {
        return Err(Error::Invalid("materialx back side"));
    }
    if transmissive != back.is_some() {
        return Err(Error::Invalid("materialx back side"));
    }
    Ok((
        Shaders {
            vs: text(set, "vertexShader")?,
            fs,
            back,
            textures,
        },
        transmissive,
    ))
}
/// The ground's shaders and the samples, as the page's MaterialXLoader
/// builds them from the documents, with the images their bindings sample
/// ( paths under assets/materialx/ ).
async fn load_samples() -> Result<((String, String), Vec<Sample>, Vec<String>)> {
    let texts = js_sys::Array::new();
    let paths = js_sys::Array::new();
    // ( name, the document's file in assets/materialx/, its directory there,
    // the loader's path )
    let mut documents = vec![];
    for name in SAMPLES {
        let dir = "resources/Materials/Examples/StandardSurface/";
        documents.push((
            name.to_string(),
            format!("{dir}standard_surface_{name}.mtlx"),
            dir,
            SAMPLE_PATH,
        ));
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
        documents.push((short, format!("three/{name}.mtlx"), "three/", LOCAL_PATH));
    }
    for (_, file, _, path) in &documents {
        let bytes = fetch(&format!("assets/materialx/{file}")).await?;
        let document =
            String::from_utf8(bytes).map_err(|_| Error::Invalid("materialx document"))?;
        texts.push(&JsValue::from_str(&document));
        paths.push(&JsValue::from_str(path));
    }
    let page = JsFuture::from(materialx_shaders(&texts, &paths).map_err(js_error)?)
        .await
        .map_err(js_error)?;
    let grid = field(&page, "grid")?;
    let grid = (text(&grid, "vertexShader")?, text(&grid, "fragmentShader")?);
    let materials = field(&page, "materials")?;
    let mut images: Vec<String> = vec![];
    let mut samples = vec![];
    for (i, (name, _, dir, path)) in documents.into_iter().enumerate() {
        let material = js_sys::Reflect::get_u32(&materials, i as u32).map_err(js_error)?;
        if let Some(e) = field(&material, "error")?.as_string() {
            return Err(Error::Asset(format!("THREE.MaterialXLoader: {name}: {e}")));
        }
        let (ball, transmissive) = shaders(&material, path, dir, &mut images)?;
        let (plane, _) = shaders(&field(&material, "plane")?, path, dir, &mut images)?;
        let kind = if transmissive {
            Transmissive
        } else if field(&material, "transparent")?.is_truthy() {
            Transparent
        } else {
            Opaque
        };
        samples.push(Sample {
            name,
            kind,
            ball,
            plane,
        });
    }
    Ok((grid, samples, images))
}
/// The ShaderBall's vertex attributes, each in its own buffer as
/// toNonIndexed leaves them and r187 uploads them; the UV plane's
/// PlaneGeometry has position, normal and uv.
#[derive(Clone, Copy, PartialEq)]
enum Attribute {
    Position,
    Normal,
    Uv,
    Tangent,
}
impl Attribute {
    /// Its stride and format on the ShaderBall, or on the plane.
    fn layout(self, plane: bool) -> (u64, wgpu::VertexFormat) {
        match (self, plane) {
            // Int16 items of three, padded to four for WebGPU.
            (Attribute::Position | Attribute::Normal, false) => (8, wgpu::VertexFormat::Snorm16x4),
            (Attribute::Uv, false) => (4, wgpu::VertexFormat::Unorm16x2),
            (Attribute::Position | Attribute::Normal, true) => (12, wgpu::VertexFormat::Float32x3),
            (Attribute::Uv, true) => (8, wgpu::VertexFormat::Float32x2),
            (Attribute::Tangent, _) => (16, wgpu::VertexFormat::Float32x4),
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
/// field: the model and normal matrices, the textures' uv transforms
/// ( identity ), the environment's rotation ( identity ), the PMREM's last
/// level `max_lod`, and the physical material's defaults ( opacity 1, ior 1.5,
/// metalness 0, specularIntensity 1, emissive 0 at intensity 1, attenuation
/// white at infinity, environment intensity 1 ). The loader sets thickness 1
/// for transmissive materials without a thickness input.
fn object_values(
    vs: &str,
    fs: &str,
    model: &[f64; 16],
    max_lod: u32,
) -> Result<Vec<(String, Vec<f64>)>> {
    let mut out = vec![];
    for (name, ty) in fields(fs, "objectStruct") {
        let lines = uses(&[fs, vs], "object", &name);
        let any = |p: &str| {
            let p = p.replace('#', &format!("object.{name}"));
            lines.iter().any(|l| l.contains(&p))
        };
        let v = match ty.as_str() {
            "mat3x3<f32>" if any("( # * vec3<f32>( vec2<f32>(") => {
                vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]
            }
            "mat3x3<f32>" => normal_matrix(model).to_vec(),
            // materialEnvRotation, on a direction; the model matrix otherwise.
            "mat4x4<f32>"
                if lines.iter().any(|l| {
                    l.contains(&format!("( object.{name} * vec4<f32>("))
                        && !l.contains("positionLocal")
                }) =>
            {
                m4(Matrix4::IDENTITY)
            }
            "mat4x4<f32>" => model.to_vec(),
            "vec3<f32>" if any("AttenuationColor = #") => vec![1.; 3],
            "vec3<f32>" if any("EmissiveColor = ( # *") => vec![0.; 3],
            "f32" if any("DiffuseColor.w * #") || any("* vec3<f32>( # ) )") => vec![1.],
            "f32" if any("IOR = #;") => vec![1.5],
            "f32" if any("Metalness = #;") => vec![0.],
            "f32" if any("Thickness = #;") => vec![1.],
            "f32" if any("AttenuationDistance = #;") => vec![f64::INFINITY],
            "f32" if any("SpecularColor = ( min(") => vec![1.],
            // roughnessToMip: maxLod × roughness × ( 2 - roughness ).
            "f32" if any(".yz ), ( ( # * ") => vec![max_lod as f64],
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
/// The UV plane: its position, normal and uv buffers and its index.
struct PlanePart {
    buffers: [wgpu::Buffer; 3],
    index: wgpu::Buffer,
    count: u32,
}
/// The part that is the UV plane.
const PLANE: usize = 2;
/// A model's mesh: its sample, part ( 0 calibration, 1 preview, 2 the UV
/// plane ), world model, object struct and render group.
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
    /// The back and front sides' transmission textures and their levels.
    transmission: [wgpu::Texture; 2],
    transmission_levels: [Vec<(wgpu::TextureView, wgpu::BindGroup)>; 2],
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
    /// The Geometry selector ( 0 Shader Ball, 1 UV Plane ), then the
    /// calibration and preview meshes' visibility.
    params: [f64; 3],
    parts: [Part; 2],
    plane: PlanePart,
    /// The documents' materials in the page's order.
    samples: Vec<Sample>,
    /// The ground's vertex and fragment WGSL.
    grid: (String, String),
    /// Per sample, its vertex inputs in location order on the ShaderBall
    /// and on the plane.
    attributes: Vec<(Vec<Attribute>, Vec<Attribute>)>,
    balls: Vec<Ball>,
    images: Vec<wgpu::TextureView>,
    ground: (wgpu::Buffer, wgpu::Buffer, u32, wgpu::Buffer),
    pmrem: pmrem::Pmrem,
    /// One render struct per distinct layout: its source and buffer.
    renders: Vec<(String, wgpu::Buffer)>,
    ground_render: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    pmrem_sampler: wgpu::Sampler,
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
        let asset =
            gltf::Gltf::from_slice_without_validation(&fetch("assets/gltf/ShaderBall.glb").await?)
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
        // The documents' images, each as its loader reads it. ImageBitmapLoader:
        // no color conversion or flip, mipmapped. EXRLoader ( rows from the
        // bottom ) and HDRLoader: half floats without mipmaps, rows from the
        // top once MaterialXLoader inverts the loader's flipY. TIFFLoader:
        // RGBA8 from the top. Their DataTextures generate no mipmaps.
        let mut mipmaps = Mipmaps::new(r);
        let (grid, samples, image_paths) = load_samples().await?;
        let mut images = vec![];
        for path in &image_paths {
            let bytes = fetch(&image_asset(path)).await?;
            let lower = path.to_lowercase();
            let single = |label: &str, format, width: u32, height: u32, texels: &[u8]| {
                r.device
                    .create_texture_with_data(
                        &r.queue,
                        &wgpu::TextureDescriptor {
                            label: Some(label),
                            size: wgpu::Extent3d {
                                width,
                                height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format,
                            usage: wgpu::TextureUsages::TEXTURE_BINDING,
                            view_formats: &[],
                        },
                        wgpu::util::TextureDataOrder::LayerMajor,
                        texels,
                    )
                    .create_view(&Default::default())
            };
            images.push(if lower.ends_with(".exr") {
                let (width, height, texels) = decode_exr(&bytes)?;
                let row = width as usize * 4;
                let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
                single(
                    "materialx exr",
                    HALF,
                    width,
                    height,
                    bytemuck::cast_slice(&texels),
                )
            } else if lower.ends_with(".hdr") {
                let (width, height, texels) = parse_rgbe(&bytes)?;
                single(
                    "materialx hdr",
                    HALF,
                    width,
                    height,
                    bytemuck::cast_slice(&texels),
                )
            } else if lower.ends_with(".tif") || lower.ends_with(".tiff") {
                let (width, height, rgba) = decode_tiff(&bytes)?;
                single(
                    "materialx tiff",
                    wgpu::TextureFormat::Rgba8Unorm,
                    width,
                    height,
                    &rgba,
                )
            } else {
                let mut image = decode_image(&bytes).await?;
                image.srgb = false;
                mipmapped(r, &mut mipmaps, &image, wgpu::TextureFormat::Rgba8Unorm)
            });
        }
        // scene.environment: the equirect HDR through PMREMGenerator.fromEquirectangular.
        let pmrem =
            pmrem::equirect_pmrem(r, "assets/tsl-procedural/san_giuseppe_bridge_2k.hdr").await?;
        // The UV plane: PlaneGeometry( 2, 2 ).
        let plane_geometry = PlaneGeometry::build(2., 2., 1, 1)?;
        let floats = |name: &str, size: usize| -> Result<Vec<f32>> {
            let a = plane_geometry
                .attributes
                .get(name)
                .ok_or(Error::Invalid("UV plane"))?;
            (0..a.count())
                .flat_map(|i| (0..size).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let plane_index = plane_geometry
            .index
            .clone()
            .ok_or(Error::Invalid("UV plane index"))?;
        let vertex_buffer = |label, data: &[f32]| {
            init(
                label,
                bytemuck::cast_slice(data),
                wgpu::BufferUsages::VERTEX,
            )
        };
        let plane = PlanePart {
            buffers: [
                vertex_buffer("UV plane position", &floats("position", 3)?),
                vertex_buffer("UV plane normal", &floats("normal", 3)?),
                vertex_buffer("UV plane uv", &floats("uv", 2)?),
            ],
            index: init(
                "UV plane index",
                bytemuck::cast_slice(&plane_index),
                wgpu::BufferUsages::INDEX,
            ),
            count: plane_index.len() as u32,
        };
        // Distinct render structs share one buffer.
        let mut renders: Vec<(String, wgpu::Buffer)> = vec![];
        let struct_body = |source: &str| fields(source, "renderStruct");
        let mut render_of = |fs: &str| -> Result<usize> {
            render_layout(fs)?;
            Ok(
                match renders
                    .iter()
                    .position(|(source, _)| struct_body(source) == struct_body(fs))
                {
                    Some(k) => k,
                    None => {
                        renders.push((
                            fs.to_string(),
                            uniform(r, "materialx render", fs, "renderStruct")?,
                        ));
                        renders.len() - 1
                    }
                },
            )
        };
        // updateModelsAlign(): six columns, 3 apart.
        let line_count = (samples.len() / 6) as f64 - 1.5;
        let (offset_x, offset_z) = (3. * 5. * -0.5, 3. * line_count * 0.5);
        let mut balls = vec![];
        for (i, sample) in samples.iter().enumerate() {
            let position = Vector3::new(
                (i % 6) as f64 * 3. + offset_x,
                0.,
                (i / 6) as f64 * -3. + offset_z,
            );
            // The calibration and preview meshes, then the UV plane
            // ( rotation.x -PI / 2, y 0.05 ).
            let nodes = [
                parts[0].node,
                parts[1].node,
                Matrix4::from_translation(Vector3::new(0., 0.05, 0.))
                    * Matrix4::from_rotation_x(-PI / 2.),
            ];
            for (p, node) in nodes.into_iter().enumerate() {
                let shaders = if p == PLANE {
                    &sample.plane
                } else {
                    &sample.ball
                };
                let model = Matrix4::from_translation(position) * node;
                let values = object_values(
                    &shaders.vs,
                    &shaders.fs,
                    &model.to_cols_array(),
                    pmrem.max_lod,
                )?;
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
                let object = init(
                    "materialx object",
                    &pack(&shaders.fs, "objectStruct", &values)?,
                    wgpu::BufferUsages::UNIFORM,
                );
                balls.push(Ball {
                    sample: i,
                    part: p,
                    model,
                    object,
                    render: render_of(&shaders.fs)?,
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
        // The PMREM's: its render target's LinearMipmapLinearFilter.
        let pmrem_sampler = sampler(
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Linear,
        );
        let mut image_samplers = vec![];
        for s in &samples {
            for &(_, t) in s.ball.textures.iter().chain(&s.plane.textures) {
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
            params: [0., 1., 1.],
            parts,
            plane,
            attributes: samples
                .iter()
                .map(|s| Ok((inputs(&s.ball.vs)?, inputs(&s.plane.vs)?)))
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
            pmrem_sampler,
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
        // The transmission ( viewport mip ) textures, the drawing buffer's
        // size: the back side's ( viewportBackSideTexture ) and the front
        // side's ( viewportOpaqueMipTexture ), each copied before its first
        // object in the frame.
        let transmission = ["back", "front"].map(|side| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("materialx transmission {side}")),
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
            })
        });
        let transmission_levels = transmission
            .each_ref()
            .map(|texture| self.mipmaps.levels(r, texture));
        let transmission_views = transmission
            .each_ref()
            .map(|texture| texture.create_view(&Default::default()));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        // Per sample, on the ShaderBall and on the plane, its front ( and
        // back side's ) pipeline: the back side's draws the back faces.
        type Pipelines = (wgpu::RenderPipeline, Option<wgpu::RenderPipeline>);
        let mut pipelines: Vec<[Pipelines; 2]> = vec![];
        for (sample, (ball, plane)) in self.samples.iter().zip(&self.attributes) {
            let pipelines_on = |shaders: &Shaders, attributes: &[Attribute], on_plane: bool| {
                let formats: Vec<[wgpu::VertexAttribute; 1]> = attributes
                    .iter()
                    .enumerate()
                    .map(|(location, a)| {
                        [wgpu::VertexAttribute {
                            format: a.layout(on_plane).1,
                            offset: 0,
                            shader_location: location as u32,
                        }]
                    })
                    .collect();
                let layouts: Vec<wgpu::VertexBufferLayout> = attributes
                    .iter()
                    .zip(&formats)
                    .map(|(a, f)| wgpu::VertexBufferLayout {
                        array_stride: a.layout(on_plane).0,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: f,
                    })
                    .collect();
                let pipeline = |(vs, fs): (&str, &str), cw| {
                    culled_pipeline(
                        r,
                        &sample.name,
                        (vs, fs),
                        &layouts,
                        &[HALF],
                        depth,
                        (cw, sample.kind != Opaque),
                        (samples, wgpu::PrimitiveTopology::TriangleList),
                        Some(wgpu::Face::Back),
                    )
                };
                (
                    pipeline((&shaders.vs, &shaders.fs), false),
                    shaders
                        .back
                        .as_ref()
                        .map(|(vs, fs)| pipeline((vs, fs), true)),
                )
            };
            pipelines.push([
                pipelines_on(&sample.ball, ball, false),
                pipelines_on(&sample.plane, plane, true),
            ]);
        }
        let mut balls = vec![];
        for b in &self.balls {
            let sample = &self.samples[b.sample];
            let shaders = if b.part == PLANE {
                &sample.plane
            } else {
                &sample.ball
            };
            // The material's bindings, with the back or front side's
            // transmission texture.
            let entries = |back: bool| -> Result<Vec<(u32, wgpu::BindingResource)>> {
                let mut entries =
                    vec![(object_binding(&shaders.fs)?, b.object.as_entire_binding())];
                for &(binding, t) in &shaders.textures {
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
                        Pmrem => (&self.pmrem_sampler, &self.pmrem.view),
                        Transmission => (
                            &self.viewport_sampler,
                            &transmission_views[(!back) as usize],
                        ),
                    };
                    entries.extend([(binding - 1, sampler(s)), (binding, tex(view))]);
                }
                Ok(entries)
            };
            let draw = |p: &wgpu::RenderPipeline, back: bool| -> Result<Draw> {
                Ok((
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, self.renders[b.render].1.as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries(back)?),
                    ],
                ))
            };
            let (front, back) = &pipelines[b.sample][(b.part == PLANE) as usize];
            balls.push((
                draw(front, false)?,
                back.as_ref().map(|p| draw(p, true)).transpose()?,
            ));
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
        // The Geometry selector: the ShaderBall's meshes ( each as its
        // checkbox says ) or the UV planes.
        let shader_ball = self.params[0] < 0.5;
        for (i, b) in self.balls.iter().enumerate() {
            let visible = match b.part {
                PLANE => !shader_ball,
                p => shader_ball && self.params[p + 1] >= 0.5,
            };
            // PlaneGeometry( 2, 2 )'s bounding sphere.
            let sphere = match b.part {
                PLANE => Sphere {
                    center: Vector3::ZERO,
                    radius: 2f64.sqrt(),
                },
                p => self.parts[p].sphere,
            };
            let (scale, _, translation) = b.model.to_scale_rotation_translation();
            if !visible
                || !frustum.intersects_sphere(Sphere {
                    center: b.model.transform_point3(sphere.center),
                    radius: sphere.radius * scale.max_element(),
                })
            {
                continue;
            }
            let z = projected_z(&screen, translation);
            match (self.samples[b.sample].kind, b.part) {
                (Opaque, _) => opaque.push((0, z, i, Some(i))),
                // The UV plane keeps renderOrder 0.
                (_, PLANE) => transparent.push((0, z, i, Some(i))),
                // Transparent materials' meshes take renderOrder 1 and 2.
                (_, p) => transparent.push((p as i32 + 1, z, i, Some(i))),
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
            if b.part == PLANE {
                for (slot, &a) in self.attributes[b.sample].1.iter().enumerate() {
                    // Pipelines exist only for the plane's own attributes.
                    let Some(buffer) = self.plane.buffers.get(a as usize) else {
                        return;
                    };
                    pass.set_vertex_buffer(slot as u32, buffer.slice(..));
                }
                pass.set_index_buffer(self.plane.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.plane.count, 0, 0..1);
                return;
            }
            let part = &self.parts[b.part];
            for (slot, &a) in self.attributes[b.sample].0.iter().enumerate() {
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
        // A transmission texture updates before its first object: the
        // resolved frame, copied and mipmapped.
        let update = |encoder: &mut wgpu::CommandEncoder, side: usize| {
            encoder.copy_texture_to_texture(
                t.color.as_image_copy(),
                t.transmission[side].as_image_copy(),
                t.color.size(),
            );
            self.mipmaps
                .encode(encoder, HALF, &t.transmission_levels[side]);
        };
        if !double_pass.is_empty() {
            drop(pass);
            update(&mut encoder, 0);
            pass = scene_pass(&mut encoder, false);
            for &i in &double_pass {
                draw(&mut pass, i, true);
            }
        }
        let mut front_updated = false;
        for e in &transparent {
            match e.3 {
                Some(i) => {
                    if !front_updated && self.samples[self.balls[i].sample].kind == Transmissive {
                        front_updated = true;
                        drop(pass);
                        update(&mut encoder, 1);
                        pass = scene_pass(&mut encoder, false);
                    }
                    draw(&mut pass, i, false)
                }
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
    /// The Geometry selector ( 0 Shader Ball, 1 UV Plane ), then the
    /// calibration and preview meshes' visibility.
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

/// The program of three.js examples/webgpu_loader_materialx.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_loader_materialx(
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
                0.25,
                200.,
                Box::new(Demo::create(scene, camera, 451, renderer).await?),
            ))
        },
    )
    .await
}
