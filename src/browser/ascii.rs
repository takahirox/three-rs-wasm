//! webgl_effects_ascii: the bouncing flat-shaded sphere rendered at pixel
//! ratio 1, then AsciiEffect: the frame drawn into a canvas at 0.15 of its
//! size, each second row's brightness mapped to the inverted character set
//! and written into the page's table, with TrackballControls.
use super::controls_attributes::viewport_css;
use super::trackball_sprites::Trackball;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const RESOLUTION: f64 = 0.15;
const CHARS: [&str; 10] = [" ", ".", ":", "-", "+", "*", "=", "%", "@", "#"];
pub(super) struct Demo {
    time: f64,
    last: f64,
    sphere: Object3D,
    controls: Trackball,
    table: web_sys::Element,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    /// The effect's width and height (CSS pixels) and the ASCII canvas size.
    size: (f64, f64),
    ascii: (u32, u32),
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 150., 500.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        for (intensity, position) in [(3., Vector3::splat(500.)), (1., Vector3::splat(-500.))] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity,
                distance: 0.,
                decay: 0.,
            }));
            s.get_mut(light)?.position = position;
        }
        let mut phong = MeshPhongMaterial::default();
        phong.properties.flat_shading = true;
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(200., 20, 10)?),
            Arc::new(Material::Phong(phong)),
        )));
        let mut basic = MeshBasicMaterial::default();
        basic.properties.color = Color::from_hex(0xe0e0e0);
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(400., 400., 1, 1)?),
            Arc::new(Material::Basic(basic)),
        )));
        let n = s.get_mut(plane)?;
        n.position.y = -200.;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        // The effect's DOM: a div (white on black) holding the table.
        let fail = |_| Error::Invalid("ascii effect page");
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let div = document.create_element("div").map_err(fail)?;
        div.set_id("ascii-effect");
        div.set_attribute(
            "style",
            "cursor: default; color: white; background-color: black;",
        )
        .map_err(fail)?;
        let table = document.create_element("table").map_err(fail)?;
        div.append_child(&table).map_err(fail)?;
        document
            .body()
            .ok_or(Error::Invalid("body"))?
            .append_child(&div)
            .map_err(fail)?;
        let canvas: web_sys::HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(fail)?
            .dyn_into()
            .map_err(|_| Error::Invalid("ascii canvas"))?;
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("ascii context"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("ascii context"))?;
        let (w, _, _) = viewport_css();
        // TrackballControls reads its element's rectangle once, before the
        // first render has filled the table: the div is zero pixels tall.
        let controls = Trackball::new(s, c, Vector2::new(w, 0.))?;
        let mut demo = Self {
            time: 0.,
            last: 0.,
            sphere,
            controls,
            table,
            canvas,
            context,
            size: (0., 0.),
            ascii: (0, 0),
        };
        demo.set_size()?;
        Ok(demo)
    }
    /// setSize( innerWidth, innerHeight ) and initAsciiSize().
    fn set_size(&mut self) -> Result<()> {
        let (w, h, _) = viewport_css();
        if (w, h) == self.size {
            return Ok(());
        }
        self.size = (w, h);
        self.ascii = (
            (w * RESOLUTION).floor() as u32,
            (h * RESOLUTION).floor() as u32,
        );
        self.canvas.set_width(self.ascii.0);
        self.canvas.set_height(self.ascii.1);
        let fail = |_| Error::Invalid("ascii table");
        self.table.set_attribute("cellspacing", "0").map_err(fail)?;
        self.table.set_attribute("cellpadding", "0").map_err(fail)?;
        let font = 2. / RESOLUTION;
        self.table
            .set_attribute(
                "style",
                &format!(
                    "white-space: pre; margin: 0px; padding: 0px; letter-spacing: -1px; font-family: \"courier new\", monospace; font-size: {font}px; line-height: {font}px; text-align: left; text-decoration: none;"
                ),
            )
            .map_err(fail)?;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the sphere bounces and turns with the clock; the controls
    /// update once per 60 fps step.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.set_size()?;
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        let timer = self.time * 1000.;
        let n = s.get_mut(self.sphere)?;
        n.position.y = (timer * 0.002).sin().abs() * 150.;
        n.quaternion =
            Quaternion::from_euler(glam::EulerRot::XYZ, timer * 0.0003, 0., timer * 0.0002);
        for _ in 0..steps {
            self.controls.update(s)?;
        }
        Ok(())
    }
    /// asciifyImage(): runs after the frame, in the same task, so the canvas
    /// still holds it.
    pub fn presented(&mut self, source: &web_sys::HtmlCanvasElement) -> Result<()> {
        let (iw, ih) = self.ascii;
        if iw == 0 || ih == 0 {
            return Ok(());
        }
        let fail = |_| Error::Invalid("ascii readback");
        self.context.clear_rect(0., 0., iw as f64, ih as f64);
        self.context
            .draw_image_with_html_canvas_element_and_dw_and_dh(source, 0., 0., iw as f64, ih as f64)
            .map_err(fail)?;
        let data = self
            .context
            .get_image_data(0., 0., iw as f64, ih as f64)
            .map_err(fail)?
            .data();
        let max = (CHARS.len() - 1) as f64;
        let mut chars = String::new();
        for y in (0..ih).step_by(2) {
            for x in 0..iw {
                let o = ((y * iw + x) * 4) as usize;
                let (r, g, b, a) = (data[o], data[o + 1], data[o + 2], data[o + 3]);
                let mut brightness = (0.3 * r as f64 + 0.59 * g as f64 + 0.11 * b as f64) / 255.;
                if a == 0 {
                    brightness = 1.;
                }
                // Math.round, then the invert option.
                let index = ((1. - brightness) * max + 0.5).floor();
                let index = (max - index) as usize;
                match CHARS.get(index) {
                    Some(&" ") | None => chars += "&nbsp;",
                    Some(c) => chars += c,
                }
            }
            chars += "<br/>";
        }
        let (w, h) = self.size;
        self.table.set_inner_html(&format!(
            "<tr><td style=\"display:block;width:{w}px;height:{h}px;overflow:hidden\">{chars}</td></tr>"
        ));
        Ok(())
    }
    /// TrackballControls on the effect's element: absolute pointer events.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            10..=19 => self.controls.down(kind - 10, x, y),
            20..=29 => self.controls.state = super::trackball_sprites::Mode::None,
            _ => self.controls.moved(x, y),
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// The wheel: zoomStart.y −= deltaY × 0.00025.
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("ascii parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
