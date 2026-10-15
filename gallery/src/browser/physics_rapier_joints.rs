//! physics_rapier_joints: a fixed red sphere at y = 6 and three capsule
//! links hanging from it, each attached to the previous body by a Rapier
//! spherical impulse joint with angular damping 10, swinging down under
//! gravity, with RapierHelper's outlines, a 1024² PCF shadow map and
//! OrbitControls.
use super::controls_attributes::{Controls, camera_state};
use super::rapier_common::{Helper, Interval, StepTimer, next_due};
use crate::physics::{PhysicsBody, PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use rapier3d::prelude::{Point, RigidBodyHandle, SphericalJointBuilder};
use std::f64::consts::PI;
use std::sync::Arc;

pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    helper: Helper,
    now: f64,
    intervals: [Interval; 1],
    timer: StepTimer,
}
fn standard(color: u32) -> Arc<Material> {
    Arc::new(Material::Standard(MeshStandardMaterial {
        properties: MaterialProperties {
            color: Color::from_hex(color),
            ..Default::default()
        },
        energy_conservation: true,
        ..Default::default()
    }))
}
fn single(body: Option<PhysicsBody>) -> Result<RigidBodyHandle> {
    match body {
        Some(PhysicsBody::Single(h)) => Ok(h),
        _ => Err(crate::Error::Invalid("physics_rapier_joints body")),
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 3., 10.);
        s.background = Color::from_hex(0xbfd1e5);
        let ambient = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x555555),
            ground: Color::from_hex(0xffffff),
            intensity: 1.,
        }));
        s.get_mut(ambient)?.position = Vector3::Y;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 4.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 12.5, 12.5);
        n.cast_shadow = true;
        n.shadow.radius = 3.;
        n.shadow.blur_samples = 8;
        n.shadow.map_size = Some(1024);
        n.shadow.extent = 10.;
        n.shadow.near = 1.;
        n.shadow.far = 50.;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        // The pivot: addScene( scene ) takes it as a fixed body.
        let pivot = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.5, 32, 16)?),
            standard(0xff0000),
        )));
        s.get_mut(pivot)?.position.y = 6.;
        let mut physics = RapierPhysics::new();
        let mut previous =
            single(physics.add_mesh(s, pivot, &PhysicsShape::Ball { radius: 0.5 }, 0., 0.)?)?;
        let helper = Helper::new(s)?;
        // addLink( link, x ) for x = 0, 2, 4.
        let geometry = Arc::new(CapsuleGeometry::build(0.25, 1.8, 4, 8, 1)?);
        let material = standard(0xcccc00);
        for (k, x) in [0., 2., 4.].into_iter().enumerate() {
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                material.clone(),
            )));
            let n = s.get_mut(mesh)?;
            n.quaternion = Quaternion::from_rotation_z(PI * 0.5);
            n.position = Vector3::new(x + 0.9, 5.8, 0.);
            let body = single(physics.add_mesh(
                s,
                mesh,
                &PhysicsShape::Capsule {
                    radius: 0.25,
                    height: 1.8,
                },
                1.,
                0.5,
            )?)?;
            // JointData.spherical( anchor on the previous body, ( 0, 1.15, 0 ) ).
            let anchor1 = if k == 0 {
                Point::new(0., -0.5, 0.)
            } else {
                Point::new(0., -1.15, 0.)
            };
            let joint = SphericalJointBuilder::new()
                .local_anchor1(anchor1)
                .local_anchor2(Point::new(0., 1.15, 0.));
            physics.bodies[body].set_angular_damping(10.);
            physics.impulse_joints.insert(previous, body, joint, true);
            previous = body;
        }
        Ok(Self {
            controls,
            physics,
            helper,
            now: 0.,
            intervals: [Interval::new(0., 1000. / 60., 0)],
            timer: StepTimer::default(),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
        }
        Ok(())
    }
    /// The steps due by the clock, then animate(): the helper.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        while let Some((_, at)) = next_due(&mut self.intervals, self.now) {
            let delta = self.timer.delta(at);
            self.physics.step(s, delta)?;
        }
        self.helper.update(s, &mut self.physics)
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(crate::Error::Invalid("physics_rapier_joints parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
