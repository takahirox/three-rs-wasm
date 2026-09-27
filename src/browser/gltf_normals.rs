//! misc_exporter_gltf_normals: two normal-mapped planes (OpenGL and DirectX
//! conventions) and GLTFExporter's binary export of them.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use js_sys::{Array, Object, Reflect};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;

const ASSETS: &str = "/web/gallery/assets";
const MAPS: [&str; 2] = ["NormalMapOpenGL.png", "NormalMapDirectX.png"];
/// The shared PlaneGeometry( 50, 50 ) and the two meshes' placement.
struct Plane {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u16>,
}
/// The exported `( filename, bytes )`, filled when the async encode finishes.
type Export = Rc<RefCell<Option<(String, Vec<u8>)>>>;
pub(super) struct Demo {
    time: f64,
    controls: Controls,
    plane: Plane,
    export: Export,
}
fn f32s(g: &BufferGeometry, name: &str) -> Vec<f32> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
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
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 150.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        for z in [1., -1.] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 4.,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = Vector3::new(0., 1., z);
        }
        let geometry = PlaneGeometry::build(50., 50., 1, 1)?;
        let plane = Plane {
            positions: f32s(&geometry, "position"),
            normals: f32s(&geometry, "normal"),
            uvs: f32s(&geometry, "uv"),
            indices: geometry
                .index
                .as_ref()
                .ok_or(Error::Invalid("plane index"))?
                .iter()
                .map(|&i| i as u16)
                .collect(),
        };
        let geometry = Arc::new(geometry);
        for (i, name) in MAPS.into_iter().enumerate() {
            let mut map =
                decode_texture_image(&fetch(&format!("{ASSETS}/gltf-normals/{name}")).await?)
                    .await?;
            map.srgb = false;
            map.mipmap_filter = Some(Filter::Linear);
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                roughness: 0.3,
                normal_map: Some(Arc::new(map)),
                // mesh2: normalScale.y *= -1.
                normal_scale: Vector2::new(0.5, if i == 0 { 0.5 } else { -0.5 }),
                ..Default::default()
            };
            m.properties.color = Color::from_hex(0x636389);
            m.properties.side = Side::Double;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Standard(m)),
            )));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(if i == 0 { -30. } else { 30. }, 0., 0.);
            n.name = format!("Mesh{}", i + 1);
        }
        // OrbitControls: minDistance 100, maxDistance 200, rendering on change.
        let mut controls = Controls::new(None, (100., 200.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            time: 0.,
            controls,
            plane,
            export: Rc::new(RefCell::new(None)),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
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
    /// The Export button: GLTFExporter.parse( [ mesh1, mesh2 ], { binary: true } ).
    pub fn parameter(&mut self, index: usize, _value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("glTF normals parameter"));
        }
        let slot = self.export.clone();
        let plane = Plane {
            positions: self.plane.positions.clone(),
            normals: self.plane.normals.clone(),
            uvs: self.plane.uvs.clone(),
            indices: self.plane.indices.clone(),
        };
        wasm_bindgen_futures::spawn_local(async move {
            match export_glb(&plane).await {
                Ok(bytes) => *slot.borrow_mut() = Some(("scene.glb".into(), bytes)),
                Err(e) => web_sys::console::error_1(&format!("glTF export: {e}").into()),
            }
        });
        Ok(())
    }
    pub fn take_export(&mut self) -> Option<(String, Vec<u8>)> {
        self.export.borrow_mut().take()
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn js(e: JsValue) -> Error {
    Error::Asset(format!("{e:?}"))
}
fn set(o: &Object, key: &str, value: impl Into<JsValue>) -> Result<()> {
    Reflect::set(o, &key.into(), &value.into()).map_err(js)?;
    Ok(())
}
fn numbers(values: impl IntoIterator<Item = f64>) -> Array {
    values.into_iter().map(JsValue::from_f64).collect()
}
fn padded(n: usize) -> usize {
    n.div_ceil(4) * 4
}
/// A 2D canvas as `getCanvas()` and `getContext( '2d', { willReadFrequently } )`.
fn canvas(
    width: u32,
    height: u32,
) -> Result<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))?;
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(js)?
        .dyn_into()
        .map_err(|_| Error::Invalid("canvas"))?;
    canvas.set_width(width);
    canvas.set_height(height);
    let options = Object::new();
    set(&options, "willReadFrequently", true)?;
    let context = canvas
        .get_context_with_context_options("2d", &options)
        .map_err(js)?
        .ok_or(Error::Invalid("2d context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("2d context"))?;
    Ok((canvas, context))
}
/// TextureLoader's HTMLImageElement.
async fn image(url: &str) -> Result<web_sys::HtmlImageElement> {
    let image = web_sys::HtmlImageElement::new().map_err(js)?;
    image.set_src(&super::asset_url(url)?);
    JsFuture::from(image.decode()).await.map_err(js)?;
    Ok(image)
}
/// processImage(): the image drawn flipped ( texture.flipY ) into a new canvas,
/// then `canvas.toBlob( 'image/png' )`.
async fn png(source: &JsValue, width: u32, height: u32) -> Result<Vec<u8>> {
    let (canvas, context) = canvas(width, height)?;
    context.translate(0., height as f64).map_err(js)?;
    context.scale(1., -1.).map_err(js)?;
    if let Some(image) = source.dyn_ref::<web_sys::HtmlImageElement>() {
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                image,
                0.,
                0.,
                width as f64,
                height as f64,
            )
            .map_err(js)?;
    } else {
        let source: &web_sys::HtmlCanvasElement =
            source.dyn_ref().ok_or(Error::Invalid("image source"))?;
        context
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                source,
                0.,
                0.,
                width as f64,
                height as f64,
            )
            .map_err(js)?;
    }
    let blob = js_sys::Promise::new(&mut |resolve, _| {
        let callback = Closure::once_into_js(move |blob: JsValue| {
            let _ = resolve.call1(&JsValue::NULL, &blob);
        });
        let _ = canvas.to_blob_with_type(callback.unchecked_ref(), "image/png");
    });
    let blob: web_sys::Blob = JsFuture::from(blob)
        .await
        .map_err(js)?
        .dyn_into()
        .map_err(|_| Error::Invalid("PNG blob"))?;
    let buffer = JsFuture::from(blob.array_buffer()).await.map_err(js)?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
/// The GLB GLTFExporter writes for [ mesh1, mesh2 ]: nodes with matrices, the
/// shared geometry's accessors, two materials, the flipped-green OpenGL normal
/// map and the DirectX one, and the two PNG images appended once encoded.
async fn export_glb(plane: &Plane) -> Result<Vec<u8>> {
    // Color( 0x636389 ) in the linear working space, with JavaScript's Math.pow.
    let linear = |c: u32| {
        let c = c as f64 / 255.;
        if c < 0.04045 {
            c * 0.0773993808
        } else {
            js_sys::Math::pow(c * 0.9478672986 + 0.0521327014, 2.4)
        }
    };
    let json = Object::new();
    let asset = Object::new();
    set(&asset, "version", "2.0")?;
    set(&asset, "generator", "THREE.GLTFExporter r186")?;
    set(&json, "asset", asset)?;
    let scene = Object::new();
    set(&scene, "name", "AuxScene")?;
    set(&json, "scenes", Array::of1(&scene))?;
    set(&json, "scene", 0)?;
    let nodes = Array::new();
    set(&json, "nodes", nodes.clone())?;
    let buffer_views = Array::new();
    let accessors = Array::new();
    let mut binary: Vec<u8> = vec![];
    // processAccessor / processBufferView for the geometry.
    let mut view = |data: Vec<u8>, stride: Option<usize>, target: u32| -> Result<usize> {
        let length = padded(data.len());
        let def = Object::new();
        set(&def, "buffer", 0)?;
        set(&def, "byteOffset", binary.len() as f64)?;
        set(&def, "byteLength", length as f64)?;
        set(&def, "target", target)?;
        if let Some(stride) = stride {
            set(&def, "byteStride", stride as f64)?;
        }
        binary.extend(&data);
        binary.resize(binary.len() + length - data.len(), 0);
        buffer_views.push(&def);
        Ok(buffer_views.length() as usize - 1)
    };
    let min_max = |values: &[f32], size: usize| {
        let mut min = vec![f64::INFINITY; size];
        let mut max = vec![f64::NEG_INFINITY; size];
        for v in values.chunks(size) {
            for k in 0..size {
                min[k] = js_sys::Math::min(min[k], v[k] as f64);
                max[k] = js_sys::Math::max(max[k], v[k] as f64);
            }
        }
        (min, max)
    };
    let attributes = Object::new();
    let mut first = true;
    for (name, values, size) in [
        ("POSITION", &plane.positions, 3),
        ("NORMAL", &plane.normals, 3),
        ("TEXCOORD_0", &plane.uvs, 2),
    ] {
        let id = view(bytemuck::cast_slice(values).to_vec(), Some(size * 4), 34962)?;
        if first {
            set(&json, "bufferViews", buffer_views.clone())?;
            let buffer = Object::new();
            set(&buffer, "byteLength", 0)?;
            set(&json, "buffers", Array::of1(&buffer))?;
            set(&json, "accessors", accessors.clone())?;
            first = false;
        }
        let (min, max) = min_max(values, size);
        let def = Object::new();
        set(&def, "bufferView", id as f64)?;
        set(&def, "componentType", 5126)?;
        set(&def, "count", (values.len() / size) as f64)?;
        set(&def, "max", numbers(max))?;
        set(&def, "min", numbers(min))?;
        set(&def, "type", if size == 3 { "VEC3" } else { "VEC2" })?;
        accessors.push(&def);
        set(&attributes, name, accessors.length() - 1)?;
    }
    let indices: Vec<u8> = bytemuck::cast_slice(&plane.indices).to_vec();
    let id = view(indices, None, 34963)?;
    let def = Object::new();
    set(&def, "bufferView", id as f64)?;
    set(&def, "componentType", 5123)?;
    set(&def, "count", plane.indices.len() as f64)?;
    let (min, max) = {
        let v: Vec<f32> = plane.indices.iter().map(|&i| i as f32).collect();
        min_max(&v, 1)
    };
    set(&def, "max", numbers(max))?;
    set(&def, "min", numbers(min))?;
    set(&def, "type", "SCALAR")?;
    accessors.push(&def);
    let index_accessor = accessors.length() - 1;
    // Materials, textures, images and samplers, in the exporter's creation order.
    let materials = Array::new();
    let textures = Array::new();
    let images = Array::new();
    let samplers = Array::new();
    let meshes = Array::new();
    let mut sources: Vec<(JsValue, u32, u32)> = vec![];
    for (i, map) in MAPS.into_iter().enumerate() {
        if i == 0 {
            set(&json, "materials", materials.clone())?;
        }
        let element = image(&format!("{ASSETS}/gltf-normals/{map}")).await?;
        let (width, height) = (element.natural_width(), element.natural_height());
        // mesh1: normalScale.y > 0 without tangents: the green channel is flipped.
        let source: JsValue = if i == 0 {
            let (canvas, context) = canvas(width, height)?;
            context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &element,
                    0.,
                    0.,
                    width as f64,
                    height as f64,
                )
                .map_err(js)?;
            let data = context
                .get_image_data(0., 0., width as f64, height as f64)
                .map_err(js)?;
            let mut pixels = data.data().0;
            for p in pixels.chunks_mut(4) {
                p[1] = 255 - p[1];
            }
            let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&pixels),
                width,
                height,
            )
            .map_err(js)?;
            context.put_image_data(&data, 0., 0.).map_err(js)?;
            canvas.into()
        } else {
            element.into()
        };
        if i == 0 {
            set(&json, "textures", textures.clone())?;
            set(&json, "images", images.clone())?;
        }
        let image_def = Object::new();
        set(&image_def, "mimeType", "image/png")?;
        images.push(&image_def);
        sources.push((source, width, height));
        let sampler = Object::new();
        set(&sampler, "magFilter", 9729)?;
        set(&sampler, "minFilter", 9987)?;
        set(&sampler, "wrapS", 33071)?;
        set(&sampler, "wrapT", 33071)?;
        if i == 0 {
            set(&json, "samplers", samplers.clone())?;
        }
        samplers.push(&sampler);
        let texture = Object::new();
        set(&texture, "sampler", samplers.length() - 1)?;
        set(&texture, "source", images.length() - 1)?;
        textures.push(&texture);
        let material = Object::new();
        let pbr = Object::new();
        set(
            &pbr,
            "baseColorFactor",
            numbers([linear(0x63), linear(0x63), linear(0x89), 1.]),
        )?;
        set(&pbr, "metallicFactor", 0)?;
        set(&pbr, "roughnessFactor", 0.3)?;
        set(&material, "pbrMetallicRoughness", pbr)?;
        let normal = Object::new();
        set(&normal, "index", textures.length() - 1)?;
        set(&normal, "texCoord", 0)?;
        set(&normal, "scale", 0.5)?;
        set(&material, "normalTexture", normal)?;
        set(&material, "doubleSided", true)?;
        materials.push(&material);
        let primitive = Object::new();
        set(&primitive, "mode", 4)?;
        set(&primitive, "attributes", attributes.clone())?;
        set(&primitive, "indices", index_accessor)?;
        set(&primitive, "material", materials.length() - 1)?;
        let mesh = Object::new();
        set(&mesh, "primitives", Array::of1(&primitive))?;
        if i == 0 {
            set(&json, "meshes", meshes.clone())?;
        }
        meshes.push(&mesh);
        let node = Object::new();
        let x = if i == 0 { -30. } else { 30. };
        set(
            &node,
            "matrix",
            numbers([
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, 0., 0., 1.,
            ]),
        )?;
        set(&node, "name", format!("Mesh{}", i + 1))?;
        set(&node, "mesh", meshes.length() - 1)?;
        nodes.push(&node);
    }
    set(&scene, "nodes", numbers([0., 1.]))?;
    // The pending images, appended as buffer views once encoded.
    for (i, (source, width, height)) in sources.iter().enumerate() {
        let data = png(source, *width, *height).await?;
        let length = padded(data.len());
        let def = Object::new();
        set(&def, "buffer", 0)?;
        set(&def, "byteOffset", binary.len() as f64)?;
        set(&def, "byteLength", length as f64)?;
        binary.extend(&data);
        binary.resize(binary.len() + length - data.len(), 0);
        buffer_views.push(&def);
        let image = images.get(i as u32);
        Reflect::set(
            &image,
            &"bufferView".into(),
            &JsValue::from(buffer_views.length() - 1),
        )
        .map_err(js)?;
    }
    let buffers: Array = Reflect::get(&json, &"buffers".into()).map_err(js)?.into();
    Reflect::set(
        &buffers.get(0),
        &"byteLength".into(),
        &JsValue::from_f64(binary.len() as f64),
    )
    .map_err(js)?;
    let text: String = js_sys::JSON::stringify(&json)
        .map_err(js)?
        .as_string()
        .ok_or(Error::Invalid("glTF JSON"))?;
    let mut chunk = text.into_bytes();
    chunk.resize(padded(chunk.len()), 0x20);
    let mut out = Vec::with_capacity(28 + chunk.len() + binary.len());
    out.extend(0x46546c67u32.to_le_bytes());
    out.extend(2u32.to_le_bytes());
    out.extend(((12 + 8 + chunk.len() + 8 + binary.len()) as u32).to_le_bytes());
    out.extend((chunk.len() as u32).to_le_bytes());
    out.extend(0x4e4f534au32.to_le_bytes());
    out.extend(chunk);
    out.extend((binary.len() as u32).to_le_bytes());
    out.extend(0x004e4942u32.to_le_bytes());
    out.extend(binary);
    Ok(out)
}
