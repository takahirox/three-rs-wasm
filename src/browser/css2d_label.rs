//! css2d_label: the Earth ( Phong with color, specular and normal maps ) and
//! the Moon circling it, each with a name and a mass label drawn by
//! CSS2DRenderer over the canvas, on camera layers 0 and 1 that the GUI
//! toggles; an AxesHelper and OrbitControls on the label renderer's element.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::css2d::{CSS2DObject, CSS2DRenderer, CSS2DRendererParameters};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

/// The page's `.label` rule, and the body of main.css that the labels inherit
/// ( the gallery's own stylesheet smooths fonts; the page's does not ).
const STYLE: &str = "#css2d-label-renderer{color:#fff;font-family:Monospace;font-size:13px;line-height:24px}#css2d-label-renderer *{-webkit-font-smoothing:auto;-moz-osx-font-smoothing:auto}.label{color:#FFF;font-family:sans-serif;padding:2px;background:rgba( 0, 0, 0, .6 )}";

/// TextureLoader: flipped, mipmapped, in the given color space.
async fn texture(path: &str, srgb: bool) -> Result<Arc<Texture>> {
    let mut t = decode_texture_image(&fetch(&format!("/web/gallery/assets/{path}")).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(Arc::new(t))
}
pub(super) struct Demo {
    controls: Controls,
    labels: CSS2DRenderer,
    moon: Object3D,
    camera: Object3D,
    time: f64,
    size: (f64, f64),
    /// GUI buttons pressed since the last frame.
    pending: Vec<usize>,
}
fn window_size() -> (f64, f64) {
    let w = web_sys::window();
    let get = |v: Option<wasm_bindgen::JsValue>| v.and_then(|v| v.as_f64()).unwrap_or(0.);
    (
        get(w.as_ref().and_then(|w| w.inner_width().ok())),
        get(w.as_ref().and_then(|w| w.inner_height().ok())),
    )
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let fail = |_| Error::Invalid("css2d_label page");
        let style = document.create_element("style").map_err(fail)?;
        style.set_text_content(Some(STYLE));
        document
            .body()
            .ok_or(Error::Invalid("body"))?
            .append_child(&style)
            .map_err(fail)?;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 200.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(10., 5., 20.);
        n.layers.enable_all();
        s.background = Color::BLACK;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 0., 1.);
        n.layers.enable_all();
        // AxesHelper( 5 ): x red, y green, z blue, fading toward the tips.
        let mut axes = BufferGeometry::default();
        axes.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(
                vec![
                    0., 0., 0., 5., 0., 0., 0., 0., 0., 0., 5., 0., 0., 0., 0., 0., 0., 5.,
                ],
                3,
                false,
            )?),
        );
        axes.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(
                vec![
                    1., 0., 0., 1., 0.6, 0., 0., 1., 0., 0.6, 1., 0., 0., 0., 1., 0., 0.6, 1.,
                ],
                3,
                false,
            )?),
        );
        let mut line = LineBasicMaterial::default();
        line.properties.vertex_colors = true;
        line.properties.tone_mapped = false;
        let axes = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(axes),
            material: Arc::new(Material::Line(line)),
            segments: true,
        }));
        s.get_mut(axes)?.layers.enable_all();
        let mut earth = MeshPhongMaterial {
            specular: Color::from_hex(0x333333),
            shininess: 5.,
            specular_map: Some(texture("earth_specular_2048.jpg", false).await?),
            normal_map: Some(texture("lights-probes/earth_normal_2048.jpg", false).await?),
            normal_scale: Vector2::new(0.85, 0.85),
            ..Default::default()
        };
        earth.properties.map = Some(texture("earth_atmos_2048.jpg", true).await?);
        let earth = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(1., 16, 16)?),
            Arc::new(Material::Phong(earth)),
        )));
        let mut moon = MeshPhongMaterial {
            shininess: 5.,
            ..Default::default()
        };
        moon.properties.map = Some(texture("lights-probes/moon_1024.jpg", true).await?);
        let moon = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.27, 16, 16)?),
            Arc::new(Material::Phong(moon)),
        )));
        s.get_mut(earth)?.layers.enable_all();
        s.get_mut(moon)?.layers.enable_all();
        // The name ( layer 0, pivot at its bottom left ) and mass ( layer 1,
        // pivot at its top left ) labels at 1.5 radii along x.
        for (body, radius, name, mass) in [
            (earth, 1., "Earth", "5.97237e24 kg"),
            (moon, 0.27, "Moon", "7.342e22 kg"),
        ] {
            for (text, center, layer) in [
                (name, Vector2::new(0., 1.), 0),
                (mass, Vector2::new(0., 0.), 1),
            ] {
                let div = document
                    .create_element("div")
                    .map_err(fail)?
                    .dyn_into::<web_sys::HtmlElement>()
                    .map_err(|_| Error::Invalid("label element"))?;
                div.set_class_name("label");
                div.set_text_content(Some(text));
                let _ = div.style().set_property("background-color", "transparent");
                let mut object = CSS2DObject::new(div);
                object.center = center;
                let label = s.insert(NodeKind::CSS2DObject(object));
                let n = s.get_mut(label)?;
                n.position = Vector3::new(1.5 * radius, 0., 0.);
                s.add(body, label)?;
                s.get_mut(label)?.layers.set(layer);
            }
        }
        // The label renderer over the canvas: position absolute at the top.
        let mut labels = CSS2DRenderer::new(CSS2DRendererParameters::default())?;
        let size = window_size();
        labels.set_size(size.0, size.1);
        let element = labels.dom_element();
        element.set_id("css2d-label-renderer");
        // The page is `<html lang="en">`: its generic sans-serif, not the gallery's Japanese one.
        let _ = element.set_attribute("lang", "en");
        let _ = element.style().set_property("position", "absolute");
        let _ = element.style().set_property("top", "0px");
        document
            .body()
            .ok_or(Error::Invalid("body"))?
            .append_child(element)
            .map_err(fail)?;
        let mut controls = Controls::new(None, (5., 100.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            labels,
            moon,
            camera: c,
            time: 0.,
            size,
            pending: vec![],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the Moon on its circle, then renderer.render() and
    /// labelRenderer.render(); onWindowResize() resizes the labels too.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.labels.set_size(size.0, size.1);
        }
        for index in std::mem::take(&mut self.pending) {
            let layers = &mut s.get_mut(self.camera)?.layers;
            match index {
                0 => layers.toggle(0),
                1 => layers.toggle(1),
                2 => layers.enable_all(),
                _ => layers.disable_all(),
            }
        }
        s.get_mut(self.moon)?.position =
            Vector3::new(self.time.sin() * 5., 0., self.time.cos() * 5.);
        self.labels.render(s, c)
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
    /// The GUI: Toggle Name, Toggle Mass, Enable All, Disable All.
    pub fn parameter(&mut self, index: usize, _value: f32) -> Result<()> {
        if index > 3 {
            return Err(Error::Invalid("css2d_label parameter"));
        }
        self.pending.push(index);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
