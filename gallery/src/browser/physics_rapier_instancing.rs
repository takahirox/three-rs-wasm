//! physics_rapier_instancing: 400 instanced boxes and 400 instanced spheres
//! with random colors, one Rapier body per instance, falling onto an
//! invisible floor collider over a ShadowMaterial plane. A 16 ms interval
//! moves one random box and one random sphere back above the center, and
//! SHAKE gives every instance a random impulse.
use super::controls_attributes::{Controls, camera_state};
use super::rapier_common::{Interval, Random, StepTimer, next_due};
use crate::physics::{PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const COUNT: usize = 400;
pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    random: Random,
    boxes: Object3D,
    spheres: Object3D,
    now: f64,
    /// RapierPhysics's step, then the page's reset interval.
    intervals: [Interval; 2],
    timer: StepTimer,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-1., 1.5, 2.);
        s.background = Color::from_hex(0x666666);
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::WHITE,
            intensity: 1.,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(5., 5., 5.);
        n.cast_shadow = true;
        // shadow.camera.zoom = 2 on the default ±5 frustum.
        n.shadow.extent = 2.5;
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1)?),
            Arc::new(Material::Shadow(ShadowMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0x444444),
                    transparent: true,
                    ..Default::default()
                },
            })),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(10., 5., 10.)?),
            Arc::new(Material::Basic(MeshBasicMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0x666666),
                    ..Default::default()
                },
            })),
        )));
        let n = s.get_mut(floor)?;
        n.position.y = -2.5;
        n.visible = false;
        let material = Arc::new(Material::Lambert(MeshLambertMaterial::default()));
        let mut random = Random(186);
        let mut instanced = |s: &mut Scene, geometry: BufferGeometry| -> Result<Object3D> {
            let instances = (0..COUNT)
                .map(|_| {
                    let p =
                        Vector3::new(random.next() - 0.5, random.next() * 2., random.next() - 0.5);
                    Instance {
                        matrix: Matrix4::from_translation(p),
                        color: Color::from_hex((16777215. * random.next()).floor() as u32),
                    }
                })
                .collect();
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                material.clone(),
            )));
            let n = s.get_mut(mesh)?;
            n.instances = instances;
            n.cast_shadow = true;
            n.receive_shadow = true;
            // computeBoundingSphere() after each step covers every instance.
            n.frustum_culled = false;
            Ok(mesh)
        };
        let boxes = instanced(s, BoxGeometry::build(0.075, 0.075, 0.075)?)?;
        let spheres = instanced(s, IcosahedronGeometry::build(0.05, 4)?)?;
        // addScene( scene ): the floor collider, then the boxes and spheres.
        let mut physics = RapierPhysics::new();
        physics.add_mesh(
            s,
            floor,
            &PhysicsShape::Box {
                width: 10.,
                height: 5.,
                depth: 10.,
            },
            0.,
            0.,
        )?;
        let cube = PhysicsShape::Box {
            width: 0.075,
            height: 0.075,
            depth: 0.075,
        };
        physics.add_mesh(s, boxes, &cube, 1., 0.)?;
        physics.add_mesh(s, spheres, &PhysicsShape::Ball { radius: 0.05 }, 1., 0.)?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        Ok(Self {
            controls,
            physics,
            random,
            boxes,
            spheres,
            now: 0.,
            intervals: [
                Interval::new(0., 1000. / 60., 0),
                Interval::new(0., 1000. / 60., 1),
            ],
            timer: StepTimer::default(),
        })
    }
    /// The page's interval: a random box, then a random sphere, back to
    /// ( 0, 1–2, 0 ) at rest.
    fn reset_one(&mut self) {
        for mesh in [self.boxes, self.spheres] {
            let index = (self.random.next() * COUNT as f64).floor() as usize;
            let position = Vector3::new(0., self.random.next() + 1., 0.);
            self.physics.set_mesh_position(mesh, position, index);
        }
    }
    fn advance(&mut self, s: &mut Scene) -> Result<()> {
        while let Some((i, at)) = next_due(&mut self.intervals, self.now) {
            if i == 0 {
                let delta = self.timer.delta(at);
                self.physics.step(s, delta)?;
            } else {
                self.reset_one();
            }
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.advance(s)
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
    /// SHAKE: a random impulse for every sphere, then every box.
    pub fn parameter(&mut self, index: usize, _value: f32) -> Result<()> {
        if index != 0 {
            return Err(crate::Error::Invalid("physics_rapier_instancing parameter"));
        }
        for mesh in [self.spheres, self.boxes] {
            for i in 0..COUNT {
                let impulse = Vector3::new(
                    (self.random.next() - 0.5) * 5.,
                    self.random.next() * 5.,
                    (self.random.next() - 0.5) * 5.,
                );
                self.physics.apply_impulse(mesh, impulse, i);
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
