//! css3d_periodictable: the 118 elements as CSS3DObject cards, tweened
//! ( Exponential.InOut, 2–4 s ) from random places into a table, a sphere,
//! a helix or a grid by the page's buttons, with TrackballControls
//! ( 500–6000 ) on the CSS renderer's element. The page has no canvas.
use super::css3d_common::{
    Random, Tween, add_style, element, exponential_in_out, overlay, page_css, window_size,
};
use super::periodic_table::{STYLE, TABLE};
use super::trackball_sprites::{Mode, Trackball};
use crate::css2d::js_number;
use crate::css3d::{CSS3DObject, CSS3DRenderer};
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::cell::Cell;
use std::f64::consts::PI;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

/// A layout: each card's position and Euler XYZ rotation.
type Targets = Vec<(Vector3, Vector3)>;
pub(super) struct Demo {
    controls: Trackball,
    css: CSS3DRenderer,
    objects: Scene,
    cards: Vec<Object3D>,
    /// The cards' Euler rotations, which the tweens animate.
    rotations: Vec<Vector3>,
    targets: [Targets; 4],
    random: Random,
    /// Position tweens ( object i ), rotation tweens ( object 1000 + i ), and
    /// the render tween ( None ).
    tweens: Vec<Tween>,
    /// A button clicked since the last frame.
    clicked: Rc<Cell<Option<usize>>>,
    time: f64,
    /// The clock of the last rendered frame, when a click lands ( ms ).
    last: f64,
    size: (f64, f64),
}
/// `Object3D.lookAt( target )` for a non-camera object: +z toward the
/// target, as Matrix4.lookAt( target, position, up ) builds it; then the
/// rotation as Euler XYZ, as Euler.setFromRotationMatrix reads it.
fn look_at_euler(position: Vector3, target: Vector3) -> Vector3 {
    let up = Vector3::Y;
    let mut z = target - position;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    // quaternion.setFromRotationMatrix(), then rotation.setFromQuaternion():
    // three.js's operations, so that a signed zero picks the same atan2 branch.
    let (m11, m12, m13) = (x.x, y.x, z.x);
    let (m21, m22, m23) = (x.y, y.y, z.y);
    let (m31, m32, m33) = (x.z, y.z, z.z);
    let trace = m11 + m22 + m33;
    let (qx, qy, qz, qw) = if trace > 0. {
        let s = 0.5 / (trace + 1.).sqrt();
        ((m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s)
    } else if m11 > m22 && m11 > m33 {
        let s = 2. * (1. + m11 - m22 - m33).sqrt();
        (0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s)
    } else if m22 > m33 {
        let s = 2. * (1. + m22 - m11 - m33).sqrt();
        ((m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s)
    } else {
        let s = 2. * (1. + m33 - m11 - m22).sqrt();
        ((m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s)
    };
    // makeRotationFromQuaternion(): compose( 0, q, 1 ).
    let (x2, y2, z2) = (qx + qx, qy + qy, qz + qz);
    let (xx, xy, xz) = (qx * x2, qx * y2, qx * z2);
    let (yy, yz, zz) = (qy * y2, qy * z2, qz * z2);
    let (wx, wy, wz) = (qw * x2, qw * y2, qw * z2);
    let n11 = 1. - (yy + zz);
    let n12 = xy - wz;
    let n13 = xz + wy;
    let n22 = 1. - (xx + zz);
    let n23 = yz - wx;
    let n32 = yz + wx;
    let n33 = 1. - (xx + yy);
    // Euler.setFromRotationMatrix( m, 'XYZ' ).
    let ry = n13.clamp(-1., 1.).asin();
    if n13.abs() < 0.9999999 {
        Vector3::new((-n23).atan2(n33), ry, (-n12).atan2(n11))
    } else {
        Vector3::new(n32.atan2(n22), ry, 0.)
    }
}
fn euler(r: Vector3) -> Quaternion {
    Euler {
        angles: r,
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        add_style(&format!(
            "{}{STYLE}",
            page_css(&["#css3d-renderer", "#menu"], "#fff")
        ))?;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 3000.);
        s.background = Color::BLACK;
        let mut random = Random(186);
        let mut objects = Scene::new();
        let mut cards = vec![];
        let mut table = vec![];
        for (i, (symbol, name, mass, column, row)) in TABLE.iter().enumerate() {
            let card = element("div")?;
            card.set_class_name("element");
            let alpha = random.next() * 0.5 + 0.25;
            let _ = card.style().set_property(
                "background-color",
                &format!("rgba(0,127,127,{})", js_number(alpha)),
            );
            for (class, html) in [
                ("number", (i + 1).to_string()),
                ("symbol", (*symbol).to_owned()),
                ("details", format!("{name}<br>{mass}")),
            ] {
                let part = element("div")?;
                part.set_class_name(class);
                if class == "details" {
                    part.set_inner_html(&html);
                } else {
                    part.set_text_content(Some(&html));
                }
                card.append_child(&part)
                    .map_err(|_| Error::Invalid("card"))?;
            }
            let h = objects.insert(NodeKind::CSS3DObject(CSS3DObject::new(card)));
            objects.get_mut(h)?.position = Vector3::new(
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
                random.next() * 4000. - 2000.,
            );
            cards.push(h);
            table.push((
                Vector3::new(
                    f64::from(*column) * 140. - 1330.,
                    -(f64::from(*row) * 180.) + 990.,
                    0.,
                ),
                Vector3::ZERO,
            ));
        }
        let l = cards.len() as f64;
        // sphere: setFromSphericalCoords( 800, φ, θ ), facing outward.
        let sphere = (0..cards.len())
            .map(|i| {
                let phi = (-1. + (2 * i) as f64 / l).acos();
                let theta = (l * PI).sqrt() * phi;
                let p = Vector3::new(
                    800. * phi.sin() * theta.sin(),
                    800. * phi.cos(),
                    800. * phi.sin() * theta.cos(),
                );
                (p, look_at_euler(p, p * 2.))
            })
            .collect();
        // helix: setFromCylindricalCoords( 900, θ, y ), facing outward.
        let helix = (0..cards.len())
            .map(|i| {
                let theta = i as f64 * 0.175 + PI;
                let y = -(i as f64 * 8.) + 450.;
                let p = Vector3::new(900. * theta.sin(), y, 900. * theta.cos());
                (p, look_at_euler(p, Vector3::new(p.x * 2., p.y, p.z * 2.)))
            })
            .collect();
        let grid = (0..cards.len())
            .map(|i| {
                (
                    Vector3::new(
                        (i % 5) as f64 * 400. - 800.,
                        -(((i / 5) % 5) as f64 * 400.) + 800.,
                        (i / 25) as f64 * 1000. - 2000.,
                    ),
                    Vector3::ZERO,
                )
            })
            .collect();
        let css = overlay("css3d-renderer")?;
        // The page's #menu buttons, below the renderer.
        let clicked = Rc::new(Cell::new(None));
        let menu = element("div")?;
        menu.set_id("menu");
        let _ = menu.set_attribute("lang", "en");
        for (i, (id, label)) in [
            ("table", "TABLE"),
            ("sphere", "SPHERE"),
            ("helix", "HELIX"),
            ("grid", "GRID"),
        ]
        .into_iter()
        .enumerate()
        {
            let button = element("button")?;
            button.set_id(id);
            button.set_text_content(Some(label));
            let slot = clicked.clone();
            let listener = Closure::<dyn FnMut()>::new(move || slot.set(Some(i)));
            button
                .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                .map_err(|_| Error::Invalid("button"))?;
            listener.forget();
            menu.append_child(&button)
                .map_err(|_| Error::Invalid("menu"))?;
            // The page's newline between the buttons.
            menu.append_with_str_1("\n\t\t\t")
                .map_err(|_| Error::Invalid("menu"))?;
        }
        super::css3d_common::document()?
            .body()
            .ok_or(Error::Invalid("body"))?
            .append_child(&menu)
            .map_err(|_| Error::Invalid("menu"))?;
        let size = window_size();
        let mut controls = Trackball::new(s, c, Vector2::new(size.0, size.1))?;
        controls.distance = (500., 6000.);
        let rotations = vec![Vector3::ZERO; cards.len()];
        let mut d = Self {
            controls,
            css,
            objects,
            cards,
            rotations,
            targets: [table, sphere, helix, grid],
            random,
            tweens: vec![],
            clicked,
            time: 0.,
            last: 0.,
            size,
        };
        d.transform(0, 0.)?;
        Ok(d)
    }
    /// transform( targets, 2000 ): TWEEN.removeAll(), then each card's
    /// position and rotation tween, and the 4 s render tween.
    fn transform(&mut self, layout: usize, time: f64) -> Result<()> {
        let duration = 2000.;
        self.tweens.clear();
        for i in 0..self.cards.len() {
            let (position, rotation) = self.targets[layout][i];
            let from = self.objects.get(self.cards[i])?.position.to_array();
            let d = self.random.next() * duration + duration;
            self.tweens.push(Tween {
                object: Some(i),
                from,
                to: position.to_array(),
                start: time,
                duration: d,
                easing: exponential_in_out,
            });
            let d = self.random.next() * duration + duration;
            self.tweens.push(Tween {
                object: Some(1000 + i),
                from: self.rotations[i].to_array(),
                to: rotation.to_array(),
                start: time,
                duration: d,
                easing: exponential_in_out,
            });
        }
        self.tweens.push(Tween {
            object: None,
            from: [0.; 3],
            to: [0.; 3],
            start: time,
            duration: duration * 2.,
            easing: |v| v,
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the buttons' transform(), TWEEN.update(), controls.update();
    /// render() on every tween update and controls change ( nothing else
    /// moves, so each frame renders ).
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
            self.controls.screen = Vector2::new(size.0, size.1);
        }
        let time = self.time * 1000.;
        // The click's listener ran between frames, at the page's last time.
        if let Some(layout) = self.clicked.take() {
            self.transform(layout, self.last)?;
        }
        self.last = time;
        let tweens = std::mem::take(&mut self.tweens);
        for tween in tweens {
            let (values, done) = tween.sample(time);
            if let (Some(v), Some(i)) = (values, tween.object) {
                if i >= 1000 {
                    self.rotations[i - 1000] = Vector3::from_array(v);
                } else {
                    self.objects.get_mut(self.cards[i])?.position = Vector3::from_array(v);
                }
            }
            if !done {
                self.tweens.push(tween);
            }
        }
        for (h, r) in self.cards.iter().zip(&self.rotations) {
            self.objects.get_mut(*h)?.quaternion = euler(*r);
        }
        self.controls.update(s)?;
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
        Err(Error::Invalid("css3d_periodictable parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
