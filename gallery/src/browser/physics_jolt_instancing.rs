//! physics_jolt_instancing: 400 instanced boxes and 400 instanced spheres
//! with random colors, one Jolt body per instance ( JoltPhysics.js,
//! `super::jolt` ), falling onto an invisible floor collider over a
//! ShadowMaterial plane, under WebGPURenderer. A 16 ms interval moves one
//! random box and one random sphere back above the center.
use super::controls_attributes::{Controls, camera_state};
use super::jolt::JoltWorld;
use super::rapier_common::{Interval, Random, StepTimer, next_due};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const COUNT: usize = 400;
pub(super) struct Demo {
    controls: Controls,
    physics: JoltWorld,
    random: Random,
    /// The instanced meshes and their AmmoPhysics mesh indices.
    meshes: [(Object3D, i32); 2],
    now: f64,
    /// JoltPhysics's step, then the page's reset interval.
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
        n.shadow.bias = -0.001;
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
        // JoltPhysics() resolves before init() builds the scene; addScene( scene )
        // then adds the floor collider, the boxes and the spheres.
        let physics = super::jolt::world().await?;
        let floor_shape = physics.box_shape(5., 2.5, 5.);
        physics.add_mesh(floor_shape, 0., 0., 0., -2.5, 0., 0., 0., 0., 1.);
        let instances = |s: &Scene, mesh: Object3D| -> Result<Vec<f32>> {
            Ok(s.get(mesh)?
                .instances
                .iter()
                .flat_map(|i| i.matrix.to_cols_array().map(|v| v as f32))
                .collect())
        };
        let box_shape = physics.box_shape(0.075 / 2., 0.075 / 2., 0.075 / 2.);
        let box_mesh = physics.add_instanced_mesh(box_shape, 1., 0., &instances(s, boxes)?);
        let sphere_shape = physics.sphere_shape(0.05);
        let sphere_mesh = physics.add_instanced_mesh(sphere_shape, 1., 0., &instances(s, spheres)?);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        Ok(Self {
            controls,
            physics,
            random,
            meshes: [(boxes, box_mesh), (spheres, sphere_mesh)],
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
        for (_, mesh) in self.meshes {
            let index = (self.random.next() * COUNT as f64).floor() as u32;
            let y = self.random.next() + 1.;
            self.physics.set_mesh_position(mesh, index, 0., y, 0.);
        }
    }
    fn advance(&mut self, s: &mut Scene) -> Result<()> {
        while let Some((i, at)) = next_due(&mut self.intervals, self.now) {
            if i == 0 {
                // step(): the Timer's delta, the poses into instanceMatrix.
                let delta = self.timer.delta(at);
                self.physics.step(delta);
                for (node, mesh) in self.meshes {
                    let poses = self.physics.poses(mesh).to_vec();
                    for (instance, pose) in s
                        .get_mut(node)?
                        .instances
                        .iter_mut()
                        .zip(poses.as_chunks::<16>().0)
                    {
                        instance.matrix = Matrix4::from_cols_array(&pose.map(f64::from));
                    }
                }
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
