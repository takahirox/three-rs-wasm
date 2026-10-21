//! css3d_orthographic: four translucent coloured divs ( three sides of a box
//! and a floor ) placed by CSS3DRenderer under an orthographic camera, each
//! matched by a black wireframe plane on the canvas, with OrbitControls on
//! the CSS renderer's element ( zoom 0.5–2 ) and the GUI's view offset.
use super::controls_attributes::Controls;
use super::css3d_common::{add_style, element, overlay, page_css, view_offset_gui, window_size};
use crate::css3d::{CSS3DObject, CSS3DRenderer};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const FRUSTUM: f64 = 500.;
pub(super) struct Demo {
    controls: Controls,
    css: CSS3DRenderer,
    objects: Scene,
    pending: Vec<(usize, f64)>,
    size: (f64, f64),
}
/// The orthographic frustum for the window's aspect.
fn frustum(o: &mut OrthographicCamera, (w, h): (f64, f64)) {
    let aspect = w / h;
    o.left = -FRUSTUM * aspect / 2.;
    o.right = FRUSTUM * aspect / 2.;
    o.top = FRUSTUM / 2.;
    o.bottom = -FRUSTUM / 2.;
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        add_style(&page_css(&["#css3d-renderer"], "#fff"))?;
        let size = window_size();
        let mut camera = OrthographicCamera {
            near: 1.,
            far: 1000.,
            ..Default::default()
        };
        frustum(&mut camera, size);
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(camera));
        s.get_mut(c)?.position = Vector3::new(-200., 200., 200.);
        s.background = Color::from_hex(0xf0f0f0);
        let material = Arc::new(Material::Basic(MeshBasicMaterial {
            properties: MaterialProperties {
                color: Color::BLACK,
                wireframe: true,
                side: Side::Double,
                ..Default::default()
            },
        }));
        let mut objects = Scene::new();
        let deg = PI / 180.;
        for (width, height, color, position, rotation) in [
            (
                100.,
                100.,
                "chocolate",
                Vector3::new(-50., 0., 0.),
                Vector3::new(0., -90. * deg, 0.),
            ),
            (
                100.,
                100.,
                "saddlebrown",
                Vector3::new(0., 0., 50.),
                Vector3::ZERO,
            ),
            (
                100.,
                100.,
                "yellowgreen",
                Vector3::new(0., 50., 0.),
                Vector3::new(-90. * deg, 0., 0.),
            ),
            (
                300.,
                300.,
                "seagreen",
                Vector3::new(0., -50., 0.),
                Vector3::new(-90. * deg, 0., 0.),
            ),
        ] {
            let div = element("div")?;
            let style = div.style();
            let _ = style.set_property("width", &format!("{width}px"));
            let _ = style.set_property("height", &format!("{height}px"));
            let _ = style.set_property("opacity", "0.75");
            let _ = style.set_property("background", color);
            let quaternion = Euler {
                angles: rotation,
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let object = objects.insert(NodeKind::CSS3DObject(CSS3DObject::new(div)));
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(width, height, 1, 1)?),
                material.clone(),
            )));
            for (scene, h) in [(&mut objects, object), (&mut *s, mesh)] {
                let n = scene.get_mut(h)?;
                n.position = position;
                n.quaternion = quaternion;
            }
        }
        let css = overlay("css3d-renderer")?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            css,
            objects,
            pending: vec![],
            size,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// animate(): the canvas, then the CSS objects; onWindowResize() keeps the
    /// frustum's aspect.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
            if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                frustum(o, size);
            }
        }
        for (index, value) in std::mem::take(&mut self.pending) {
            view_offset_gui(s, c, index, value)?;
        }
        s.update()?;
        self.css.render_from(&mut self.objects, s, c)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// OrbitControls: an orthographic camera zooms ( minZoom 0.5, maxZoom 2 )
    /// instead of moving.
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
        if wheel != 0. {
            self.controls.wheel_scale(wheel);
        } else if pan {
            // pan(): the orthographic frustum's width and height over the element's.
            let (w, _) = window_size();
            let quaternion = s.get(c)?.quaternion;
            if let NodeKind::Camera(Camera::Orthographic(o)) = &s.get(c)?.kind {
                let left = dx * (o.right - o.left) / o.zoom / w;
                let up = dy * (o.top - o.bottom) / o.zoom / height;
                self.controls.pan_axes(quaternion, left, up);
            }
        } else {
            self.controls.rotate(dx, dy, height);
        }
        let scale = self.controls.take_scale();
        if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
            o.zoom = (o.zoom / scale).clamp(0.5, 2.);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index > 7 {
            return Err(Error::Invalid("css3d_orthographic parameter"));
        }
        self.pending.push((index, f64::from(value)));
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

/// The program of three.js examples/css3d_orthographic.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn css3d_orthographic(
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
                Box::new(Demo::create(scene, camera, 474, renderer).await?),
            ))
        },
    )
    .await
}
