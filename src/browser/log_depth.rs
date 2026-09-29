//! webgpu_camera_logarithmicdepthbuffer: fifteen text labels with their dots,
//! from 1 µm to 1000 light years, seen by two cameras (near 1e-6, far 1e27)
//! side by side — a normal z-buffer left of the draggable border and a
//! logarithmic one right of it — while the camera zooms outward each frame.
use super::gltf_viewer::fetch;
use super::shapes_lights::hsl;
use super::text_clipping::text_geometry;
use super::text_shapes::{Extrude, Font};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::{E, PI};
use std::sync::Arc;

const NEAR: f64 = 1e-6;
const FAR: f64 = 1e27;
/// labeldata: ( size, scale, label ).
const LABELS: [(f64, f64, &str); 15] = [
    (0.01, 0.0001, "microscopic (1µm)"),
    (0.01, 0.1, "minuscule (1mm)"),
    (0.01, 1.0, "tiny (1cm)"),
    (1., 1.0, "child-sized (1m)"),
    (10., 1.0, "tree-sized (10m)"),
    (100., 1.0, "building-sized (100m)"),
    (1000., 1.0, "medium (1km)"),
    (10000., 1.0, "city-sized (10km)"),
    (3400000., 1.0, "moon-sized (3,400 Km)"),
    (12000000., 1.0, "planet-sized (12,000 km)"),
    (1400000000., 1.0, "sun-sized (1,400,000 km)"),
    (7.47e12, 1.0, "solar system-sized (50Au)"),
    (9.4605284e15, 1.0, "gargantuan (1 light year)"),
    (3.08567758e16, 1.0, "ludicrous (1 parsec)"),
    (1e19, 1.0, "mind boggling (1000 light years)"),
];
pub(super) struct Demo {
    normal: Object3D,
    logarithmic: Object3D,
    target: Option<RenderTarget>,
    screensplit: f64,
    mouse: Vector2,
    zoompos: f64,
    minzoomspeed: f64,
    zoomspeed: f64,
    /// Frames stepped from the page, applied before the next render.
    pending: u32,
}
impl Demo {
    pub async fn create(s: &mut Scene, _c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let font = Font::parse(
            &fetch("/web/gallery/assets/fonts/helvetiker_regular.typeface.json").await?,
        )?;
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x777777),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(100., 100., 100.);
        let sphere = Arc::new(SphereGeometry::build(0.5, 24, 12)?);
        let mut seed = 186u32;
        let mut random = move || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        for (size, scale, label) in LABELS {
            let mut g = text_geometry(
                &font,
                label,
                size,
                &Extrude {
                    curve_segments: 12,
                    steps: 1,
                    depth: size / 2.,
                    bevel: None,
                },
            )?;
            // Center the text by the bounding sphere's radius.
            let radius = g.compute_bounding_sphere()?.radius;
            g.translate(Vector3::new(-radius, 0., 0.))?;
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(0x050505),
                shininess: 50.,
                emissive: Color::BLACK,
                ..Default::default()
            };
            m.properties.color = hsl(random(), 0.5, 0.5);
            let material = Arc::new(Material::Phong(m));
            let group = s.insert(NodeKind::Group);
            s.get_mut(group)?.position.z = -size * scale;
            let text = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), material.clone())));
            let n = s.get_mut(text)?;
            n.scale = Vector3::splat(scale);
            n.position = Vector3::new(0., size / 4. * scale, -size * scale);
            s.add(group, text)?;
            let dot = s.insert(NodeKind::Mesh(Mesh::new(sphere.clone(), material)));
            let n = s.get_mut(dot)?;
            n.position.y = -size / 4. * scale;
            n.scale = Vector3::splat(size * scale);
            s.add(group, dot)?;
        }
        let camera = |s: &mut Scene| {
            s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 50.,
                aspect: 1.,
                near: NEAR,
                far: FAR,
                ..Default::default()
            })))
        };
        let normal = camera(s);
        let logarithmic = camera(s);
        let mut demo = Self {
            normal,
            logarithmic,
            target: None,
            screensplit: 0.25,
            mouse: Vector2::splat(0.5),
            zoompos: -100.,
            minzoomspeed: 0.015,
            zoomspeed: 0.015,
            pending: 0,
        };
        // Place the cameras as the first frame would, keeping the zoom state
        // for that frame.
        let zoom = (demo.zoompos, demo.zoomspeed);
        demo.step(s)?;
        (demo.zoompos, demo.zoomspeed) = zoom;
        Ok(demo)
    }
    /// One animate() frame: the damped zoom and the mouse-orbiting camera.
    fn step(&mut self, s: &mut Scene) -> Result<()> {
        let (first, last) = (LABELS[0], LABELS[LABELS.len() - 1]);
        let minzoom = first.0 * first.1;
        let maxzoom = last.0 * last.1 * 100.;
        let mut damping = if self.zoomspeed.abs() > self.minzoomspeed {
            0.95
        } else {
            1.
        };
        let zoom = E.powf(self.zoompos).clamp(minzoom, maxzoom);
        self.zoompos = zoom.ln();
        if (zoom == minzoom && self.zoomspeed < 0.) || (zoom == maxzoom && self.zoomspeed > 0.) {
            damping = 0.85;
        }
        self.zoompos += self.zoomspeed;
        self.zoomspeed *= damping;
        let position = Vector3::new(
            (0.5 * PI * (self.mouse.x - 0.5)).sin() * zoom,
            (0.25 * PI * (self.mouse.y - 0.5)).sin() * zoom,
            (0.5 * PI * (self.mouse.x - 0.5)).cos() * zoom,
        );
        s.get_mut(self.normal)?.position = position;
        s.look_at(self.normal, Vector3::ZERO)?;
        let quaternion = s.get(self.normal)?.quaternion;
        let n = s.get_mut(self.logarithmic)?;
        n.position = position;
        n.quaternion = quaternion;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.step(s)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.pending) {
            self.step(s)?;
        }
        Ok(())
    }
    /// The two renderers: the normal view in the left screensplit × width of
    /// the canvas and the logarithmic view in the rest, each camera's view
    /// offset cutting its part of the full frame.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.target = Some(RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                out.options.clone(),
            )?);
        }
        let target = self.target.as_mut().ok_or(Error::Invalid("log target"))?;
        let dpr = web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .device_pixel_ratio();
        let (w, h) = (out.width as f64 / dpr, out.height as f64 / dpr);
        let split = self.screensplit;
        let views = [
            (self.normal, false, 0., split),
            (self.logarithmic, true, split, 1. - split),
        ];
        for (i, (camera, logarithmic, offset, fraction)) in views.into_iter().enumerate() {
            // setSize( fraction × width, height ) at the pixel ratio.
            let x = (offset * w * dpr).round().min(out.width as f64) as u32;
            let width = ((fraction * w * dpr).floor() as u32).min(out.width - x);
            if width == 0 {
                continue;
            }
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(camera)?.kind {
                // setViewOffset() sets aspect to fullWidth / fullHeight.
                p.aspect = w / h;
                p.view = Some(ViewOffset {
                    full_width: w,
                    full_height: h,
                    offset_x: offset * w,
                    offset_y: 0.,
                    width: fraction * w,
                    height: h,
                });
            }
            let area = [x, 0, width, out.height];
            target.viewport = area;
            target.scissor = Some(area);
            target.set_load_color(i > 0);
            r.logarithmic_depth.set(logarithmic);
            let result = r.render(s, camera, target);
            r.logarithmic_depth.set(false);
            result?;
        }
        target.viewport = [0, 0, target.width, target.height];
        target.scissor = None;
        target.set_load_color(false);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.target.as_ref()
    }
    /// The page's input: kind 1 steps x frames, 2 is a wheel of deltaY x, 3 a
    /// mouse move to ( x, y ) of the window and 4 the border dragged to x.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            1 => self.pending += x.max(0.) as u32,
            2 if x != 0. => {
                self.zoomspeed = x.signum() / 10.;
                self.minzoomspeed = 0.001;
            }
            3 => self.mouse = Vector2::new(x, y),
            4 => self.screensplit = x.clamp(0., 1.),
            _ => {}
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
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("log depth parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
