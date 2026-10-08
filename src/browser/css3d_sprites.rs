//! css3d_sprites: 512 CSS3DSprite images tweened ( Exponential.InOut, 2–4 s )
//! every 6 s through a wave plane, a cube, a random cloud and a sphere,
//! pulsing in scale, with TrackballControls on the CSS renderer's element.
//! The page has no canvas: the gallery's canvas only clears to the page's
//! white body beneath the CSS renderer.
use super::css3d_common::{
    Random, Tween, add_style, element, exponential_in_out, overlay, page_css, window_size,
};
use super::trackball_sprites::{Mode, Trackball};
use crate::css3d::{CSS3DRenderer, CSS3DSprite};
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use wasm_bindgen::JsCast;

const TOTAL: usize = 512;
const DURATION: f64 = 2000.;
pub(super) struct Demo {
    controls: Trackball,
    css: CSS3DRenderer,
    objects: Scene,
    sprites: Vec<Object3D>,
    /// The four layouts, three values per sprite each.
    positions: Vec<f64>,
    current: usize,
    random: Random,
    tweens: Vec<Tween>,
    time: f64,
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
            fov: 75.,
            near: 1.,
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(600., 400., 1500.);
        s.look_at(c, Vector3::ZERO)?;
        // The page's white body.
        s.background = Color::WHITE;
        let mut random = Random(186);
        let mut positions = Vec::with_capacity(TOTAL * 12);
        // Plane
        let (amount_x, amount_z, separation) = (16, 32, 150.);
        let offset_x = (amount_x - 1) as f64 * separation / 2.;
        let offset_z = (amount_z - 1) as f64 * separation / 2.;
        for i in 0..TOTAL {
            let x = (i % amount_x) as f64 * separation;
            let z = (i / amount_x) as f64 * separation;
            let y = ((x * 0.5).sin() + (z * 0.5).sin()) * 200.;
            positions.extend([x - offset_x, y, z - offset_z]);
        }
        // Cube
        let amount = 8;
        let offset = (amount - 1) as f64 * separation / 2.;
        for i in 0..TOTAL {
            let x = (i % amount) as f64 * separation;
            let y = ((i / amount) % amount) as f64 * separation;
            let z = (i / (amount * amount)) as f64 * separation;
            positions.extend([x - offset, y - offset, z - offset]);
        }
        // Random
        for _ in 0..TOTAL {
            positions.extend([
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
            ]);
        }
        // Sphere
        let radius = 750.;
        for i in 0..TOTAL {
            let phi = (-1. + (2 * i) as f64 / TOTAL as f64).acos();
            let theta = (TOTAL as f64 * PI).sqrt() * phi;
            positions.extend([
                radius * theta.cos() * phi.sin(),
                radius * theta.sin() * phi.sin(),
                radius * phi.cos(),
            ]);
        }
        let css = overlay("css3d-renderer")?;
        let size = window_size();
        let controls = Trackball::new(s, c, Vector2::new(size.0, size.1))?;
        // The image's load listener: the sprites at random positions, then transition().
        let image = element("img")?;
        image
            .set_attribute("src", "/web/gallery/assets/css3d/sprite.png")
            .map_err(|_| Error::Invalid("sprite image"))?;
        let mut objects = Scene::new();
        let mut sprites = vec![];
        for _ in 0..TOTAL {
            let clone = image
                .clone_node()
                .map_err(|_| Error::Invalid("sprite image"))?
                .dyn_into::<web_sys::HtmlElement>()
                .map_err(|_| Error::Invalid("sprite image"))?;
            let sprite = objects.insert(NodeKind::CSS3DSprite(CSS3DSprite::new(clone)));
            objects.get_mut(sprite)?.position = Vector3::new(
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
            );
            sprites.push(sprite);
        }
        let mut d = Self {
            controls,
            css,
            objects,
            sprites,
            positions,
            current: 0,
            random,
            tweens: vec![],
            time: 0.,
            size,
        };
        d.transition(0.)?;
        Ok(d)
    }
    /// transition(): each sprite toward the next layout over 2–4 s, and a 6 s
    /// tween whose completion starts the next transition.
    fn transition(&mut self, time: f64) -> Result<()> {
        let offset = self.current * TOTAL * 3;
        for i in 0..TOTAL {
            let j = offset + i * 3;
            let from = self.objects.get(self.sprites[i])?.position.to_array();
            let duration = self.random.next() * DURATION + DURATION;
            self.tweens.push(Tween {
                object: Some(i),
                from,
                to: [
                    self.positions[j],
                    self.positions[j + 1],
                    self.positions[j + 2],
                ],
                start: time,
                duration,
                easing: exponential_in_out,
            });
        }
        self.tweens.push(Tween {
            object: None,
            from: [0.; 3],
            to: [0.; 3],
            start: time,
            duration: DURATION * 3.,
            easing: |v| v,
        });
        self.current = (self.current + 1) % 4;
        Ok(())
    }
    /// TWEEN.update( time ): in start order; a completed tween leaves, and
    /// the timer's completion starts the next transition at the same time.
    fn tween(&mut self, time: f64) -> Result<()> {
        let tweens = std::mem::take(&mut self.tweens);
        let mut kept = Vec::with_capacity(tweens.len());
        let mut transitions = 0;
        for tween in tweens {
            let (values, done) = tween.sample(time);
            if let (Some(v), Some(i)) = (values, tween.object) {
                self.objects.get_mut(self.sprites[i])?.position = Vector3::from_array(v);
            }
            if done {
                if tween.object.is_none() {
                    transitions += 1;
                }
            } else {
                kept.push(tween);
            }
        }
        self.tweens = kept;
        for _ in 0..transitions {
            self.transition(time)?;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): TWEEN.update(), controls.update(), each sprite's pulse,
    /// then the CSS render.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
            self.controls.screen = Vector2::new(size.0, size.1);
        }
        let time = self.time * 1000.;
        self.tween(time)?;
        self.controls.update(s)?;
        for &h in &self.sprites {
            let n = self.objects.get_mut(h)?;
            let scale = ((n.position.x.floor() + time) * 0.002).sin() * 0.3 + 1.;
            n.scale = Vector3::splat(scale);
        }
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("css3d_sprites parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
