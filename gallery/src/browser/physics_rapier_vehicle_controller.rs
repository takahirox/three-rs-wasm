//! physics_rapier_vehicle_controller: a red 2 × 1 × 4 box chassis ( mass 10 )
//! on four ray-cast wheels of Rapier's DynamicRayCastVehicleController,
//! driven with WASD / the arrows, braked with space and reset with R, on a
//! 100 × 100 gridded floor, with RapierHelper's outlines, a 2048² PCF shadow
//! map and OrbitControls following the car.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::{Helper, Interval, StepTimer, next_due};
use crate::physics::{PhysicsBody, PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use rapier3d::control::{DynamicRayCastVehicleController, WheelTuning};
use rapier3d::prelude::{Point, QueryFilter, QueryFilterFlags, RigidBodyHandle, Rotation, Vector};
use std::f64::consts::PI;
use std::sync::Arc;

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
/// The page's movement state.
#[derive(Default)]
struct Movement {
    forward: f64,
    right: f64,
    brake: f64,
    reset: bool,
    accelerate: f64,
    brake_force: f64,
}
pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    helper: Helper,
    car: Object3D,
    chassis: RigidBodyHandle,
    vehicle: DynamicRayCastVehicleController,
    wheels: Vec<Object3D>,
    movement: Movement,
    now: f64,
    intervals: [Interval; 1],
    timer: StepTimer,
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
        s.get_mut(c)?.position = Vector3::new(0., 4., 10.);
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
        n.shadow.map_size = Some(2048);
        n.shadow.extent = 40.;
        n.shadow.near = 1.;
        n.shadow.far = 50.;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        let mut grid =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(80., 80.);
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(100., 0.5, 100.)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                properties: MaterialProperties {
                    map: Some(Arc::new(grid)),
                    ..Default::default()
                },
                energy_conservation: true,
                ..Default::default()
            })),
        )));
        let n = s.get_mut(ground)?;
        n.receive_shadow = true;
        n.position = Vector3::new(0., -0.25, -20.);
        // initPhysics(): the helper, addScene( scene ), createCar().
        let mut physics = RapierPhysics::new();
        let helper = Helper::new(s)?;
        let floor = PhysicsShape::Box {
            width: 100.,
            height: 0.5,
            depth: 100.,
        };
        physics.add_mesh(s, ground, &floor, 0., 0.)?;
        let car = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(2., 1., 4.)?),
            standard(0xff0000),
        )));
        let n = s.get_mut(car)?;
        n.cast_shadow = true;
        n.position.y = 1.;
        let chassis_shape = PhysicsShape::Box {
            width: 2.,
            height: 1.,
            depth: 4.,
        };
        let Some(PhysicsBody::Single(chassis)) =
            physics.add_mesh(s, car, &chassis_shape, 10., 0.8)?
        else {
            return Err(crate::Error::Invalid(
                "physics_rapier_vehicle_controller chassis",
            ));
        };
        let mut vehicle = DynamicRayCastVehicleController::new(chassis);
        let mut geometry = CylinderGeometry::build(0.3, 0.3, 0.4, 16, 1, false, 0., 2. * PI)?;
        geometry.rotate_z(PI * 0.5)?;
        let geometry = Arc::new(geometry);
        let black = standard(0x000000);
        let mut wheels = vec![];
        for (x, z) in [(-1., -1.5), (1., -1.5), (-1., 1.5), (1., 1.5)] {
            // addWheel(): rest length 0.8, radius 0.3, the default tuning.
            let wheel = vehicle.add_wheel(
                Point::new(x as f32, 0., z as f32),
                Vector::new(0., -1., 0.),
                Vector::new(-1., 0., 0.),
                0.8,
                0.3,
                &WheelTuning::default(),
            );
            wheel.suspension_stiffness = 24.;
            wheel.friction_slip = 1000.;
            // setWheelSteering( index, pos.z < 0 ): the boolean as a number.
            wheel.steering = if z < 0. { 1. } else { 0. };
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), black.clone())));
            let n = s.get_mut(mesh)?;
            n.cast_shadow = true;
            n.position = Vector3::new(x, 0., z);
            s.add(car, mesh)?;
            wheels.push(mesh);
        }
        for w in &mut vehicle.wheels_mut()[..2] {
            w.steering = (PI / 4.) as f32;
        }
        Ok(Self {
            controls,
            physics,
            helper,
            car,
            chassis,
            vehicle,
            wheels,
            movement: Movement::default(),
            now: 0.,
            intervals: [Interval::new(0., 1000. / 60., 0)],
            timer: StepTimer::default(),
        })
    }
    /// updateCarControl(): the reset, or the engine force and the brake
    /// stepping toward their limits, and the front wheels' steering eased
    /// toward ±45°.
    fn control(&mut self) {
        let m = &mut self.movement;
        let p = &mut self.physics;
        if m.reset {
            let body = &mut p.bodies[self.chassis];
            body.set_translation(Vector::new(0., 1., 0.), true);
            body.set_rotation(Rotation::identity(), true);
            body.set_linvel(Vector::zeros(), true);
            body.set_angvel(Vector::zeros(), true);
            m.accelerate = 0.;
            m.brake_force = 0.;
            return;
        }
        let accelerate = if m.forward < 0. {
            (m.accelerate - 1.).max(-30.)
        } else if m.forward > 0. {
            (m.accelerate + 1.).min(30.)
        } else {
            if p.bodies[self.chassis].is_sleeping() {
                p.bodies[self.chassis].wake_up(true);
            }
            0.
        };
        m.accelerate = accelerate;
        let brake = if m.brake > 0. {
            (m.brake_force + 0.05).min(1.)
        } else {
            0.
        };
        m.brake_force = brake;
        let wheels = self.vehicle.wheels_mut();
        let current = f64::from(wheels[0].steering);
        let target = PI / 4. * m.right;
        // MathUtils.lerp( current, target, 0.25 ).
        let steering = ((1. - 0.25) * current + 0.25 * target) as f32;
        for w in &mut wheels[..2] {
            w.engine_force = accelerate as f32;
            w.steering = steering;
        }
        for w in wheels.iter_mut() {
            w.brake = (m.brake * brake) as f32;
        }
    }
    /// updateWheels(): each wheel at its suspension length, steered about y
    /// and turned about its axle.
    fn update_wheels(&self, s: &mut Scene) -> Result<()> {
        for (wheel, mesh) in self.vehicle.wheels().iter().zip(&self.wheels) {
            let axle = wheel.axle_cs;
            let axle = Vector3::new(f64::from(axle.x), f64::from(axle.y), f64::from(axle.z));
            // `value || 0`: zero for NaN.
            let or_zero = |v: f32| if v.is_nan() { 0. } else { f64::from(v) };
            let connection = or_zero(wheel.chassis_connection_point_cs.y);
            let suspension = or_zero(wheel.raycast_info().suspension_length);
            let steering = Quaternion::from_axis_angle(Vector3::Y, or_zero(wheel.steering));
            let rotation = Quaternion::from_axis_angle(axle, or_zero(wheel.rotation));
            let n = s.get_mut(*mesh)?;
            n.position.y = connection - suspension;
            n.quaternion = steering * rotation;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
        }
        Ok(())
    }
    /// The steps due by the clock, then animate(): the car's control and
    /// updateVehicle( 1 / 60 ), the wheels, the controls on the car, the helper.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        while let Some((_, at)) = next_due(&mut self.intervals, self.now) {
            let delta = self.timer.delta(at);
            self.physics.step(s, delta)?;
        }
        self.control();
        let p = &mut self.physics;
        let filter = QueryFilter {
            flags: QueryFilterFlags::empty(),
            groups: None,
            exclude_collider: None,
            exclude_rigid_body: Some(self.chassis),
            predicate: None,
        };
        self.vehicle.update_vehicle(
            (1. / 60.) as f32,
            &mut p.bodies,
            &p.colliders,
            &p.query_pipeline,
            filter,
        );
        self.update_wheels(s)?;
        let target = s.get(self.car)?.position;
        self.controls.set_target(target);
        self.controls.update(s, c)?;
        self.helper.update(s, &mut self.physics)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    /// keydown / keyup: W or ↑, S or ↓, A or ←, D or →, R, space.
    pub fn key(&mut self, code: u32, down: bool) {
        let m = &mut self.movement;
        match (code, down) {
            (87 | 38, true) => m.forward = -1.,
            (83 | 40, true) => m.forward = 1.,
            (65 | 37, true) => m.right = 1.,
            (68 | 39, true) => m.right = -1.,
            (82, true) => m.reset = true,
            (32, true) => m.brake = 1.,
            (87 | 83 | 38 | 40, false) => m.forward = 0.,
            (65 | 68 | 37 | 39, false) => m.right = 0.,
            (82, false) => m.reset = false,
            (32, false) => m.brake = 0.,
            _ => {}
        }
    }
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
        Err(crate::Error::Invalid(
            "physics_rapier_vehicle_controller parameter",
        ))
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
