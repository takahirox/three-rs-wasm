//! css3d_sandbox: ten coloured 100 px divs ( half of them translucent ) placed
//! by CSS3DRenderer at random positions, rotations and scales, each matched
//! by a black wireframe plane rendered on the canvas beneath, with
//! TrackballControls on the CSS renderer's element and the GUI's camera
//! view offset.
use super::css3d_common::{
    Random, add_style, color_style, element, overlay, page_css, view_offset_gui, window_size,
};
use super::trackball_sprites::{Mode, Trackball};
use crate::css3d::{CSS3DObject, CSS3DRenderer};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

pub(super) struct Demo {
    controls: Trackball,
    css: CSS3DRenderer,
    /// scene2: the CSS objects.
    objects: Scene,
    pending: Vec<(usize, f64)>,
    size: (f64, f64),
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        add_style(&page_css(&["#css3d-renderer"], "#000"))?;
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
        s.get_mut(c)?.position = Vector3::new(200., 200., 200.);
        s.background = Color::from_hex(0xf0f0f0);
        let material = Arc::new(Material::Basic(MeshBasicMaterial {
            properties: MaterialProperties {
                color: Color::BLACK,
                wireframe: true,
                side: Side::Double,
                ..Default::default()
            },
        }));
        let geometry = Arc::new(PlaneGeometry::build(100., 100., 1, 1)?);
        let mut objects = Scene::new();
        let mut random = Random(186);
        for i in 0..10 {
            let div = element("div")?;
            let style = div.style();
            let _ = style.set_property("width", "100px");
            let _ = style.set_property("height", "100px");
            let _ = style.set_property("opacity", if i < 5 { "0.5" } else { "1" });
            let _ = style.set_property("background", &color_style(random.next() * 16777215.));
            let position = Vector3::new(
                random.next() * 200. - 100.,
                random.next() * 200. - 100.,
                random.next() * 200. - 100.,
            );
            let rotation = Euler {
                angles: Vector3::new(random.next(), random.next(), random.next()),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let scale = Vector3::new(random.next() + 0.5, random.next() + 0.5, 1.);
            let object = objects.insert(NodeKind::CSS3DObject(CSS3DObject::new(div)));
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                material.clone(),
            )));
            for (scene, h) in [(&mut objects, object), (&mut *s, mesh)] {
                let n = scene.get_mut(h)?;
                n.position = position;
                n.quaternion = rotation;
                n.scale = scale;
            }
        }
        let css = overlay("css3d-renderer")?;
        let size = window_size();
        let controls = Trackball::new(s, c, Vector2::new(size.0, size.1))?;
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
    /// animate(): controls.update(), the canvas, then the CSS objects.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
            self.controls.screen = Vector2::new(size.0, size.1);
        }
        for (index, value) in std::mem::take(&mut self.pending) {
            view_offset_gui(s, c, index, value)?;
            // updateCameraViewOffset() also sets camera.aspect from the window.
            if (1..=6).contains(&index) || index == 0 {
                let (w, h) = window_size();
                if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                    p.aspect = w / h;
                }
            }
        }
        self.controls.update(s)?;
        // renderer2.render( scene2, camera ): the camera node lives in the canvas scene.
        s.update()?;
        self.css.render_from(&mut self.objects, s, c)
    }
    /// TrackballControls on the CSS renderer's element: absolute pointer events.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            10..=19 => self.controls.down(kind - 10, x, y),
            20..=29 => self.controls.state = Mode::None,
            _ => self.controls.moved(x, y),
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        if wheel != 0. {
            self.controls.zoom_start.y -= wheel * 0.00025;
        }
        Ok(())
    }
    /// The GUI: setViewOffset, the six values, clearViewOffset.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index > 7 {
            return Err(Error::Invalid("css3d_sandbox parameter"));
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

/// The program of three.js examples/css3d_sandbox.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn css3d_sandbox(
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
                Box::new(Demo::create(scene, camera, 473, renderer).await?),
            ))
        },
    )
    .await
}
