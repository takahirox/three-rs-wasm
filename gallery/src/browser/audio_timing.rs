//! webaudio_timing: five balls bouncing in a ring over a shadowed floor, each
//! playing its PositionalAudio when it turns from falling to rising. The
//! bounce detection and the listener and source placement run here; the page
//! drives the Web Audio graph (web/gallery/audio.js) from `audio_frame`.
use super::controls_attributes::{CameraState, Controls, camera_state};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

const COUNT: usize = 5;
const SPEED: f64 = 2.5;
const HEIGHT: f64 = 3.;
const OFFSET: f64 = 0.5;
pub(super) struct Demo {
    time: f64,
    /// The Play button: init() runs, and the timer starts, on its click.
    started: Option<f64>,
    start_requested: bool,
    root: Object3D,
    balls: [Object3D; COUNT],
    down: [bool; COUNT],
    /// audio.play() calls since the page last took them.
    plays: [u32; COUNT],
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(7., 3., 7.);
        n.quaternion = Quaternion::IDENTITY;
        // No scene background: the clear color is black.
        s.background = Color::BLACK;
        let root = s.insert(NodeKind::Group);
        let add = |s: &mut Scene, kind: NodeKind| -> Result<Object3D> {
            let h = s.insert(kind);
            s.add(root, h)?;
            Ok(h)
        };
        add(
            s,
            NodeKind::Light(Light::Ambient {
                color: Color::from_hex(0xcccccc),
                intensity: 1.,
            }),
        )?;
        let light = add(
            s,
            NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 2.5,
                target: Vector3::ZERO,
            }),
        )?;
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 5., 5.);
        n.cast_shadow = true;
        n.shadow.extent = 5.;
        n.shadow.near = 1.;
        n.shadow.far = 20.;
        n.shadow.map_size = Some(1024);
        let lambert = |hex: u32| {
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(hex);
            Arc::new(Material::Lambert(m))
        };
        let floor = add(
            s,
            NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(10., 10., 1, 1)?),
                lambert(0x4676b6),
            )),
        )?;
        let n = s.get_mut(floor)?;
        n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        n.receive_shadow = true;
        let mut ball = SphereGeometry::build(0.3, 32, 16)?;
        ball.translate(Vector3::new(0., 0.3, 0.))?;
        let ball = Arc::new(ball);
        let material = lambert(0xcccccc);
        let mut balls = vec![];
        for i in 0..COUNT {
            let angle = i as f64 / COUNT as f64 * std::f64::consts::TAU;
            let h = add(s, NodeKind::Mesh(Mesh::new(ball.clone(), material.clone())))?;
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.position = Vector3::new(3. * angle.cos(), 0., 3. * angle.sin());
            balls.push(h);
        }
        // Nothing is shown until the Play button runs init().
        s.get_mut(root)?.visible = false;
        // OrbitControls: minDistance 1, maxDistance 25; the constructor's update().
        let mut controls = Controls::new(None, (1., 25.), std::f64::consts::PI, true);
        controls.update(s, c)?;
        Ok(Self {
            time: 0.,
            started: None,
            start_requested: false,
            root,
            balls: [balls[0], balls[1], balls[2], balls[3], balls[4]],
            down: [false; COUNT],
            plays: [0; COUNT],
            controls,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the balls' heights from the elapsed time, and a play on each
    /// turn from falling to rising.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.start_requested) && self.started.is_none() {
            self.started = Some(self.time);
            s.get_mut(self.root)?.visible = true;
        }
        let Some(start) = self.started else {
            return Ok(());
        };
        let time = self.time - start;
        for (i, &h) in self.balls.iter().enumerate() {
            let n = s.get_mut(h)?;
            let previous = n.position.y;
            n.position.y = ((i as f64 * OFFSET + time * SPEED).sin() * HEIGHT).abs();
            if n.position.y < previous {
                self.down[i] = true;
            } else if self.down[i] {
                self.plays[i] += 1;
                self.down[i] = false;
            }
        }
        Ok(())
    }
    /// The listener (camera) and the sources' world placement, then the plays
    /// since the last call: [position, forward, up] + per ball [position,
    /// orientation, plays].
    pub fn audio_frame(&mut self, s: &mut Scene, c: Object3D) -> Result<Vec<f32>> {
        s.update_world_matrix(c, true, false)?;
        let camera = s.get(c)?;
        let (_, q, p) = camera.matrix_world.to_scale_rotation_translation();
        let mut out = vec![];
        for v in [p, q * Vector3::NEG_Z, q * Vector3::Y] {
            out.extend([v.x as f32, v.y as f32, v.z as f32]);
        }
        for (i, &h) in self.balls.iter().enumerate() {
            let n = s.get(h)?;
            let (_, q, p) = n.matrix_world.to_scale_rotation_translation();
            let o = q * Vector3::Z;
            out.extend([p.x, p.y, p.z, o.x, o.y, o.z].map(|v| v as f32));
            out.push(std::mem::take(&mut self.plays[i]) as f32);
        }
        Ok(out)
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// 0: the Play button, applied at the next frame.
    pub fn parameter(&mut self, index: usize, _value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("audio timing parameter"));
        }
        self.start_requested = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
