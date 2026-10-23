//! webgl_loader_3dm: Rhino_Logo.3dm as Rhino3dmLoader builds it in a Z-up
//! world: the brep render meshes in double-sided physical materials, 218 red
//! curves, and the hidden point cloud, SubD meshes, text dots and Rhino
//! lights, under the page's directional light. The file is decoded at load by
//! the Rhino3dmLoader port ( rhino3dm_loader.rs ) on rhino3dm 8.32.1, loaded
//! from jsDelivr as the page loads it, at the loader's subdivision level 3.
//! The text dots are drawn on a canvas as the loader draws them ( the font,
//! the dot colour and the device pixel ratio ) and shown as unlit sprites
//! without depth testing. The GUI toggles each layer's objects; OrbitControls
//! orbit around Z. The loader leaves the lights' targets out of the scene, so
//! the directional light shines toward the origin.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::tsl::{Texture as Tex, sprites::SpriteNodeMaterial, uv};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "assets/3dm-loader";

use super::rhino3dm_loader::{self as loader, LoaderMaterial, MaterialType, NodeKind as Kind};

fn rgb(c: [f64; 3]) -> Color {
    Color::linear(c[0], c[1], c[2])
}
fn material(m: &LoaderMaterial) -> Material {
    let properties = MaterialProperties {
        color: rgb(m.color),
        opacity: m.opacity,
        transparent: m.transparent,
        vertex_colors: m.vertex_colors,
        flat_shading: m.flat_shading,
        side: if m.double_sided {
            Side::Double
        } else {
            Side::Front
        },
        ..Default::default()
    };
    match m.kind {
        MaterialType::Physical => {
            let mut p = MeshPhysicalMaterial::default();
            p.base.properties = properties;
            p.base.emissive = rgb(m.emissive);
            p.base.metalness = m.metalness;
            p.base.roughness = m.roughness;
            p.ior = m.ior;
            p.specular_intensity = m.specular_intensity;
            p.specular_color = rgb(m.specular_color);
            p.clearcoat = m.clearcoat;
            p.clearcoat_roughness = m.clearcoat_roughness;
            p.sheen = m.sheen;
            p.sheen_color = rgb(m.sheen_color);
            p.thickness = m.thickness;
            p.anisotropy = m.anisotropy;
            Material::Physical(p)
        }
        MaterialType::Standard => Material::Standard(MeshStandardMaterial {
            properties,
            emissive: rgb(m.emissive),
            metalness: m.metalness,
            roughness: m.roughness,
            ..Default::default()
        }),
        MaterialType::LineBasic => Material::Line(LineBasicMaterial {
            properties,
            ..Default::default()
        }),
        MaterialType::Points => Material::Points(PointsMaterial {
            properties,
            size: m.size,
            size_attenuation: m.size_attenuation,
        }),
    }
}

/// The loader's text dot: the text centred in white on the dot colour, on a
/// canvas of the measured text width plus 10 by the font height plus 10 CSS
/// pixels at the device pixel ratio; the sprite is a tenth of that in size.
fn text_dot(
    text: &str,
    font_height: f64,
    font_face: &str,
    c: [f64; 4],
) -> Result<(Texture, f64, f64)> {
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
    let font = format!("{font_height}px {font_face}");
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
    let channel = |i: usize| c[i];
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
        // loader.load( 'models/3dm/Rhino_Logo.3dm' ): the worker's decode
        // and _createGeometry.
        let rhino = loader::library().await?;
        let decoded = loader::decode_objects(
            &rhino,
            &fetch(&format!("{ASSETS}/Rhino_Logo.3dm")).await?,
            3.0,
        )?;
        let tree = loader::create_geometry(&decoded)?;
        let materials: Vec<Arc<Material>> = tree
            .materials
            .iter()
            .map(|m| Arc::new(material(m)))
            .collect();
        let mut layers: Vec<(Vec<Object3D>, bool)> = decoded
            .layers
            .iter()
            .map(|l| {
                let visible = js_sys::Reflect::get(l, &"visible".into())
                    .map(|v| v.is_truthy())
                    .unwrap_or(true);
                (vec![], visible)
            })
            .collect();
        let order = loader::traverse(&tree);
        let mut handles: Vec<Option<Object3D>> = vec![None; tree.nodes.len()];
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let quad = Arc::new(PlaneGeometry::build(1., 1., 1, 1)?);
        for &i in &order {
            let n = &tree.nodes[i];
            let material = || {
                n.material
                    .and_then(|m| materials.get(m).cloned())
                    .ok_or(Error::Invalid("3dm node material"))
            };
            let geometry = || {
                n.geometry
                    .clone()
                    .ok_or(Error::Invalid("3dm node geometry"))
            };
            let mut sprite_scale = None;
            let kind = match &n.kind {
                Kind::Mesh => NodeKind::Mesh(Mesh {
                    geometry: geometry()?,
                    materials: vec![material()?],
                }),
                Kind::Line => NodeKind::Line(Line {
                    geometry: geometry()?,
                    material: material()?,
                    segments: false,
                }),
                Kind::Points => NodeKind::Points(Points {
                    geometry: geometry()?,
                    material: material()?,
                }),
                Kind::Sprite {
                    text,
                    font_height,
                    font_face,
                    color,
                } => {
                    let (texture, width, height) = text_dot(text, *font_height, font_face, *color)?;
                    let gpu = r.upload_texture(&Arc::new(texture))?;
                    let mut m = SpriteNodeMaterial::new(Tex::External(0).sample(uv()))
                        .build(r, &[], &[(&gpu.view, &sampler)])
                        .await?;
                    // SpriteMaterial: transparent, and the loader turns depth testing off.
                    m.properties.transparent = true;
                    m.properties.depth_test = false;
                    sprite_scale = Some(Vector3::new(width, height, 1.));
                    NodeKind::Mesh(Mesh::new(quad.clone(), Arc::new(Material::Shader(m))))
                }
                Kind::PointLight => {
                    let (color, intensity) = n.light.unwrap_or(([1.; 3], 1.));
                    NodeKind::Light(Light::Point {
                        color: rgb(color),
                        intensity,
                        distance: 0.,
                        decay: 2.,
                    })
                }
                Kind::DirectionalLight { .. } => {
                    let (color, intensity) = n.light.unwrap_or(([1.; 3], 1.));
                    NodeKind::Light(Light::Directional {
                        color: rgb(color),
                        intensity,
                        target: Vector3::ZERO,
                    })
                }
                Kind::SpotLight { .. } | Kind::RectAreaLight { .. } => {
                    return Err(Error::Invalid("3dm light type"));
                }
                Kind::Object3D => NodeKind::Group,
            };
            let h = s.insert(kind);
            handles[i] = Some(h);
            let node = s.get_mut(h)?;
            node.position = Vector3::from_array(n.position);
            let q = n.quaternion;
            node.quaternion = Quaternion::from_xyzw(q[0], q[1], q[2], q[3]);
            node.scale = sprite_scale.unwrap_or(Vector3::from_array(n.scale));
            if sprite_scale.is_some() {
                node.frustum_culled = false;
            }
            node.up = Vector3::Z;
            node.visible = n.visible;
            if let Some(parent) = n.parent.and_then(|p| handles[p]) {
                s.add(parent, h)?;
            }
            if let Some(layer) = n
                .layer_index
                .filter(|l| *l >= 0. && l.fract() == 0.)
                .and_then(|l| layers.get_mut(l as usize))
            {
                layer.0.push(h);
            }
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

/// The program of three.js examples/webgl_loader_3dm.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_3dm(
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
                1000.,
                Box::new(Demo::create(scene, camera, 440, renderer).await?),
            ))
        },
    )
    .await
}
