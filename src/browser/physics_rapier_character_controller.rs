//! physics_rapier_character_controller: a 20 × 20 floor with ten random
//! fixed red boxes or dynamic green balls, and a blue capsule moved with
//! WASD / the arrows by Rapier's KinematicCharacterController ( offset 0.01,
//! pushing dynamic bodies with a character mass of 3 ) at 2.5 m/s, with
//! RapierHelper's outlines, a 1024² PCF shadow map and OrbitControls.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::{Helper, Interval, Random, StepTimer, next_due};
use crate::physics::{PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use rapier3d::control::{CharacterLength, KinematicCharacterController};
use rapier3d::prelude::{ColliderHandle, QueryFilter, QueryFilterFlags, SharedShape, Vector};
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
pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    helper: Helper,
    controller: KinematicCharacterController,
    collider: ColliderHandle,
    player: Object3D,
    /// movement.forward and movement.right.
    movement: (f64, f64),
    /// animate() moves the character after its render: the next frame moves
    /// it first, with the movement of the frame that rendered.
    pending: Option<(f64, f64)>,
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
        s.get_mut(c)?.position = Vector3::new(2., 5., 15.);
        s.background = Color::from_hex(0xbfd1e5);
        let ambient = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x555555),
            ground: Color::from_hex(0xffffff),
            intensity: 1.,
        }));
        s.get_mut(ambient)?.position = Vector3::Y;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
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
        let mut grid =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(20., 20.);
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(20., 0.5, 20.)?),
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
        n.position.y = -0.25;
        // addBody( Math.random() > 0.7 ) ten times: fixed boxes or light balls.
        let mut random = Random(186);
        let boxes = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let balls = Arc::new(SphereGeometry::build(0.25, 32, 16)?);
        let mut bodies = vec![];
        for _ in 0..10 {
            let fixed = random.next() > 0.7;
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                if fixed { boxes.clone() } else { balls.clone() },
                standard(if fixed { 0xff0000 } else { 0x00ff00 }),
            )));
            let x = random.next() * 20. - 10.;
            let z = random.next() * 20. - 10.;
            let n = s.get_mut(mesh)?;
            n.cast_shadow = true;
            n.position = Vector3::new(x, 0.5, z);
            bodies.push((mesh, fixed));
        }
        // initPhysics(): the helper, addScene( scene ), then the character.
        let mut physics = RapierPhysics::new();
        let helper = Helper::new(s)?;
        physics.add_mesh(
            s,
            ground,
            &PhysicsShape::Box {
                width: 20.,
                height: 0.5,
                depth: 20.,
            },
            0.,
            0.,
        )?;
        for (mesh, fixed) in bodies {
            if fixed {
                let shape = PhysicsShape::Box {
                    width: 1.,
                    height: 1.,
                    depth: 1.,
                };
                physics.add_mesh(s, mesh, &shape, 0., 0.)?;
            } else {
                physics.add_mesh(s, mesh, &PhysicsShape::Ball { radius: 0.25 }, 0.5, 0.3)?;
            }
        }
        let player = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(CapsuleGeometry::build(0.3, 1., 8, 8, 1)?),
            standard(0x0000ff),
        )));
        let n = s.get_mut(player)?;
        n.cast_shadow = true;
        n.position = Vector3::new(0., 0.8, 0.);
        // createCharacterController( 0.01 ), as RawKinematicCharacterController.new.
        let controller = KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            autostep: None,
            snap_to_ground: None,
            ..Default::default()
        };
        let p2 = rapier3d::prelude::Point::from(Vector::y() * 0.5);
        let collider = physics.insert_collider(
            RapierPhysics::default_collider(SharedShape::capsule(-p2, p2, 0.3))
                .translation(Vector::new(0., 0.8, 0.)),
        );
        Ok(Self {
            controls,
            physics,
            helper,
            controller,
            collider,
            player,
            movement: (0., 0.),
            pending: None,
            now: 0.,
            intervals: [Interval::new(0., 1000. / 60., 0)],
            timer: StepTimer::default(),
        })
    }
    /// computeColliderMovement( collider, ( right, 0, −forward ) × 2.5 / 60 ),
    /// pushing dynamic bodies, then the collider and the mesh move by the
    /// computed movement.
    fn move_character(&mut self, s: &mut Scene, (forward, right): (f64, f64)) -> Result<()> {
        let speed = 2.5 * (1. / 60.);
        let desired = Vector::new((right * speed) as f32, 0., (-forward * speed) as f32);
        let p = &mut self.physics;
        let Some(collider) = p.colliders.get(self.collider) else {
            return Ok(());
        };
        let filter = QueryFilter {
            flags: QueryFilterFlags::empty(),
            groups: None,
            exclude_collider: Some(self.collider),
            exclude_rigid_body: collider.parent(),
            predicate: None,
        };
        let dt = p.integration_parameters.dt;
        let mut events = vec![];
        let result = self.controller.move_shape(
            dt,
            &p.bodies,
            &p.colliders,
            &p.query_pipeline,
            collider.shape(),
            collider.position(),
            desired,
            filter,
            |event| events.push(event),
        );
        self.controller.solve_character_collision_impulses(
            dt,
            &mut p.bodies,
            &p.colliders,
            &p.query_pipeline,
            collider.shape(),
            3.,
            events.iter(),
            filter,
        );
        // The page adds the movement to the translation in doubles; the
        // collider takes them as floats, the mesh as they are.
        let t = collider.translation();
        let m = result.translation;
        let position = Vector3::new(
            f64::from(t.x) + f64::from(m.x),
            f64::from(t.y) + f64::from(m.y),
            f64::from(t.z) + f64::from(m.z),
        );
        if let Some(c) = p.colliders.get_mut(self.collider) {
            c.set_translation(Vector::new(
                position.x as f32,
                position.y as f32,
                position.z as f32,
            ));
        }
        s.get_mut(self.player)?.position = position;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
        }
        Ok(())
    }
    /// The last frame's movement, the steps due by the clock, then
    /// animate()'s helper before its render.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(movement) = self.pending.take() {
            self.move_character(s, movement)?;
        }
        while let Some((_, at)) = next_due(&mut self.intervals, self.now) {
            let delta = self.timer.delta(at);
            self.physics.step(s, delta)?;
        }
        self.helper.update(s, &mut self.physics)?;
        self.pending = Some(self.movement);
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    /// keydown / keyup: W or ↑, S or ↓, A or ←, D or →.
    pub fn key(&mut self, code: u32, down: bool) {
        let (forward, right) = &mut self.movement;
        match (code, down) {
            (87 | 38, true) => *forward = 1.,
            (83 | 40, true) => *forward = -1.,
            (65 | 37, true) => *right = -1.,
            (68 | 39, true) => *right = 1.,
            (87 | 83 | 38 | 40, false) => *forward = 0.,
            (65 | 68 | 37 | 39, false) => *right = 0.,
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
            "physics_rapier_character_controller parameter",
        ))
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
