//! webgl_loader_3dm: Rhino_Logo.3dm as Rhino3dmLoader builds it ( baked by
//! tools/tsl/prepare-3dm-loader.mjs: rhino3dm's decode and the loader's object
//! tree ) in a Z-up world: the brep render meshes in double-sided physical
//! materials, 218 red curves, and the hidden point cloud, SubD meshes, text
//! dots and Rhino lights, under the page's directional light. The text dots
//! are drawn on a canvas as the loader draws them ( the font, the dot colour
//! and the device pixel ratio ) and shown as unlit sprites without depth
//! testing. The GUI toggles each layer's objects; OrbitControls orbit around Z.
//! The loader leaves the lights' targets out of the scene, so the directional
//! light shines toward the origin.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::attribute::BufferAttribute;
use crate::tsl::{Texture as Tex, sprites::SpriteNodeMaterial, uv};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "/web/gallery/assets/3dm-loader";

type Json = serde_json::Value;
fn numbers(v: &Json) -> Vec<f64> {
    v.as_array()
        .map(|a| a.iter().filter_map(Json::as_f64).collect())
        .unwrap_or_default()
}
fn vector(v: &Json) -> Vector3 {
    let n = numbers(v);
    Vector3::new(
        n.first().copied().unwrap_or(0.),
        n.get(1).copied().unwrap_or(0.),
        n.get(2).copied().unwrap_or(0.),
    )
}
fn color(v: &Json) -> Color {
    let n = numbers(v);
    Color::linear(
        n.first().copied().unwrap_or(1.),
        n.get(1).copied().unwrap_or(1.),
        n.get(2).copied().unwrap_or(1.),
    )
}
fn bytes<'a>(bin: &'a [u8], v: &Json, size: usize) -> Result<&'a [u8]> {
    let offset = v["offset"].as_u64().ok_or(Error::Invalid("3dm offset"))? as usize;
    let length = v["length"].as_u64().ok_or(Error::Invalid("3dm length"))? as usize;
    bin.get(offset..offset + length * size)
        .ok_or(Error::Invalid("3dm range"))
}
fn geometry(bin: &[u8], n: &Json) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    if let Some(attributes) = n["attributes"].as_object() {
        for (name, a) in attributes {
            if a["type"].as_str() != Some("Float32Array") {
                return Err(Error::Invalid("3dm attribute type"));
            }
            let data = bytes(bin, a, 4)?
                .as_chunks::<4>()
                .0
                .iter()
                .map(|&c| f32::from_le_bytes(c))
                .collect();
            let size = a["itemSize"].as_u64().unwrap_or(3) as usize;
            g.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(
                    data,
                    size,
                    a["normalized"].as_bool().unwrap_or(false),
                )?),
            );
        }
    }
    if n["index"].is_object() {
        let index = match n["index"]["type"].as_str() {
            Some("Uint16Array") => bytes(bin, &n["index"], 2)?
                .as_chunks::<2>()
                .0
                .iter()
                .map(|&c| u32::from(u16::from_le_bytes(c)))
                .collect(),
            _ => bytes(bin, &n["index"], 4)?
                .as_chunks::<4>()
                .0
                .iter()
                .map(|&c| u32::from_le_bytes(c))
                .collect(),
        };
        g.set_index(Some(index));
    }
    Ok(g)
}
fn material(m: &Json) -> Result<Material> {
    let f = |k: &str, d: f64| m[k].as_f64().unwrap_or(d);
    let mut properties = MaterialProperties {
        color: color(&m["color"]),
        opacity: f("opacity", 1.),
        transparent: m["transparent"].as_bool().unwrap_or(false),
        vertex_colors: m["vertexColors"].as_bool().unwrap_or(false),
        flat_shading: m["flatShading"].as_bool().unwrap_or(false),
        ..Default::default()
    };
    properties.side = match m["side"].as_u64() {
        Some(1) => Side::Back,
        Some(2) => Side::Double,
        _ => Side::Front,
    };
    Ok(match m["type"].as_str() {
        Some("MeshPhysicalMaterial") => {
            let mut p = MeshPhysicalMaterial::default();
            p.base.properties = properties;
            p.base.emissive = color(&m["emissive"]);
            p.base.metalness = f("metalness", 0.);
            p.base.roughness = f("roughness", 1.);
            p.ior = f("ior", 1.5);
            p.specular_intensity = f("specularIntensity", 1.);
            p.specular_color = color(&m["specularColor"]);
            p.clearcoat = f("clearcoat", 0.);
            p.clearcoat_roughness = f("clearcoatRoughness", 0.);
            p.sheen = f("sheen", 0.);
            p.sheen_color = color(&m["sheenColor"]);
            p.thickness = f("thickness", 0.);
            p.anisotropy = f("anisotropy", 0.);
            Material::Physical(p)
        }
        Some("MeshStandardMaterial") => Material::Standard(MeshStandardMaterial {
            properties,
            emissive: color(&m["emissive"]),
            metalness: f("metalness", 0.),
            roughness: f("roughness", 1.),
            ..Default::default()
        }),
        Some("LineBasicMaterial") => Material::Line(LineBasicMaterial {
            properties,
            ..Default::default()
        }),
        Some("PointsMaterial") => Material::Points(PointsMaterial {
            properties,
            size: f("size", 1.),
            size_attenuation: m["sizeAttenuation"].as_bool().unwrap_or(true),
        }),
        _ => return Err(Error::Invalid("3dm material type")),
    })
}

/// The loader's text dot: the text centred in white on the dot colour, on a
/// canvas of the measured text width plus 10 by the font height plus 10 CSS
/// pixels at the device pixel ratio; the sprite is a tenth of that in size.
fn text_dot(dot: &Json) -> Result<(Texture, f64, f64)> {
    let window = web_sys::window().ok_or(Error::Invalid("window"))?;
    let canvas: web_sys::HtmlCanvasElement = window
        .document()
        .ok_or(Error::Invalid("document"))?
        .create_element("canvas")
        .map_err(|_| Error::Invalid("canvas"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("canvas"))?;
    let context =
        |canvas: &web_sys::HtmlCanvasElement| -> Result<web_sys::CanvasRenderingContext2d> {
            canvas
                .get_context("2d")
                .ok()
                .flatten()
                .and_then(|c| c.dyn_into().ok())
                .ok_or(Error::Invalid("canvas context"))
        };
    let text = dot["text"].as_str().unwrap_or_default();
    let font_height = dot["fontHeight"].as_f64().unwrap_or(12.);
    let font = format!(
        "{font_height}px {}",
        dot["fontFace"].as_str().unwrap_or("Arial")
    );
    let ctx = context(&canvas)?;
    ctx.set_font(&font);
    let width = ctx
        .measure_text(text)
        .map_err(|_| Error::Invalid("measureText"))?
        .width()
        + 10.;
    let height = font_height + 10.;
    let r = window.device_pixel_ratio();
    canvas.set_width((width * r) as u32);
    canvas.set_height((height * r) as u32);
    let ctx = context(&canvas)?;
    ctx.set_transform(r, 0., 0., r, 0., 0.)
        .map_err(|_| Error::Invalid("setTransform"))?;
    ctx.set_font(&font);
    ctx.set_text_baseline("middle");
    ctx.set_text_align("center");
    let c = numbers(&dot["color"]);
    let channel = |i: usize| c.get(i).copied().unwrap_or(0.);
    ctx.set_fill_style_str(&format!(
        "rgba({},{},{},{})",
        channel(0),
        channel(1),
        channel(2),
        channel(3)
    ));
    ctx.fill_rect(0., 0., width, height);
    ctx.set_fill_style_str("white");
    ctx.fill_text(text, width / 2., height / 2.)
        .map_err(|_| Error::Invalid("fillText"))?;
    let (w, h) = (canvas.width(), canvas.height());
    let data = ctx
        .get_image_data(0., 0., f64::from(w), f64::from(h))
        .map_err(|_| Error::Invalid("getImageData"))?
        .data()
        .0;
    // CanvasTexture's flipY: the canvas's top row at v = 1.
    let row = w as usize * 4;
    let rgba: Vec<u8> = data.chunks(row).rev().flatten().copied().collect();
    // NoColorSpace, linear filtering without mipmaps, clamped.
    let mut texture = Texture::from_rgba(w, h, rgba, false)?;
    texture.mipmap_filter = None;
    Ok((texture, width / 10., height / 10.))
}

pub struct Demo {
    controls: Controls,
    /// Per layer: the objects on it and its visibility.
    layers: Vec<(Vec<Object3D>, bool)>,
    dirty: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        // Object3D.DEFAULT_UP = ( 0, 0, 1 ) before the camera is made.
        let n = s.get_mut(c)?;
        n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        n.position = Vector3::new(26., -40., 5.);
        n.up = Vector3::Z;
        s.background = Color::BLACK;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 6.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 0., 2.);
        let json: Json =
            serde_json::from_slice(&fetch(&format!("{ASSETS}/Rhino_Logo.json")).await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let bin = fetch(&format!("{ASSETS}/Rhino_Logo.bin")).await?;
        let materials = json["materials"]
            .as_array()
            .ok_or(Error::Invalid("3dm materials"))?
            .iter()
            .map(|m| material(m).map(Arc::new))
            .collect::<Result<Vec<_>>>()?;
        let mut layers: Vec<(Vec<Object3D>, bool)> = json["layers"]
            .as_array()
            .ok_or(Error::Invalid("3dm layers"))?
            .iter()
            .map(|l| (vec![], l["visible"].as_bool().unwrap_or(true)))
            .collect();
        let nodes = json["nodes"]
            .as_array()
            .ok_or(Error::Invalid("3dm nodes"))?;
        let mut handles: Vec<Object3D> = Vec::with_capacity(nodes.len());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let quad = Arc::new(PlaneGeometry::build(1., 1., 1, 1)?);
        for n in nodes {
            let material = || {
                n["material"]
                    .as_u64()
                    .and_then(|i| materials.get(i as usize).cloned())
                    .ok_or(Error::Invalid("3dm node material"))
            };
            let kind = match n["type"].as_str() {
                Some("Mesh") => NodeKind::Mesh(Mesh {
                    geometry: Arc::new(geometry(&bin, n)?),
                    materials: vec![material()?],
                }),
                Some("Line") => NodeKind::Line(Line {
                    geometry: Arc::new(geometry(&bin, n)?),
                    material: material()?,
                    segments: false,
                }),
                Some("Points") => NodeKind::Points(Points {
                    geometry: Arc::new(geometry(&bin, n)?),
                    material: material()?,
                }),
                Some("Sprite") => {
                    let (texture, width, height) = text_dot(&n["textDot"])?;
                    let gpu = r.upload_texture(&Arc::new(texture))?;
                    let mut m = SpriteNodeMaterial::new(Tex::External(0).sample(uv()))
                        .build(r, &[], &[(&gpu.view, &sampler)])
                        .await?;
                    // SpriteMaterial: transparent, and the loader turns depth testing off.
                    m.properties.transparent = true;
                    m.properties.depth_test = false;
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        quad.clone(),
                        Arc::new(Material::Shader(m)),
                    )));
                    let node = s.get_mut(h)?;
                    node.scale = Vector3::new(width, height, 1.);
                    node.frustum_culled = false;
                    handles.push(h);
                    Self::place(s, h, n, &handles, &mut layers)?;
                    continue;
                }
                Some("PointLight") => {
                    let l = &n["light"];
                    NodeKind::Light(Light::Point {
                        color: color(&l["color"]),
                        intensity: l["intensity"].as_f64().unwrap_or(1.),
                        distance: l["distance"].as_f64().unwrap_or(0.),
                        decay: l["decay"].as_f64().unwrap_or(2.),
                    })
                }
                Some("DirectionalLight") => {
                    let l = &n["light"];
                    NodeKind::Light(Light::Directional {
                        color: color(&l["color"]),
                        intensity: l["intensity"].as_f64().unwrap_or(1.),
                        target: Vector3::ZERO,
                    })
                }
                Some("Object3D") => NodeKind::Group,
                _ => return Err(Error::Invalid("3dm node type")),
            };
            let h = s.insert(kind);
            handles.push(h);
            Self::place(s, h, n, &handles, &mut layers)?;
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.up = Vector3::Z;
        controls.update(s, c)?;
        Ok(Self {
            controls,
            layers,
            dirty: false,
        })
    }
    /// The node's parent, local transform, visibility and layer.
    fn place(
        s: &mut Scene,
        h: Object3D,
        n: &Json,
        handles: &[Object3D],
        layers: &mut [(Vec<Object3D>, bool)],
    ) -> Result<()> {
        let q = numbers(&n["quaternion"]);
        let node = s.get_mut(h)?;
        node.position = vector(&n["position"]);
        if q.len() == 4 {
            node.quaternion = Quaternion::from_xyzw(q[0], q[1], q[2], q[3]);
        }
        let scale = vector(&n["scale"]);
        if n["type"].as_str() != Some("Sprite") {
            node.scale = scale;
        }
        node.up = Vector3::Z;
        node.visible = n["visible"].as_bool().unwrap_or(true);
        if let Some(parent) = n["parent"].as_u64().and_then(|p| handles.get(p as usize)) {
            s.add(*parent, h)?;
        }
        if let Some(layer) = n["layerIndex"]
            .as_u64()
            .and_then(|i| layers.get_mut(i as usize))
        {
            layer.0.push(h);
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.dirty) {
            for (objects, visible) in &self.layers {
                for &h in objects {
                    s.get_mut(h)?.visible = *visible;
                }
            }
        }
        self.controls.update(s, c)
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
    /// The layer checkboxes, in the file's layer order.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if let Some(layer) = self.layers.get_mut(index) {
            layer.1 = value != 0.;
            self.dirty = true;
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
