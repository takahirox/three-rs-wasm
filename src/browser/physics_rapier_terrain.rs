//! physics_rapier_terrain: a 128 × 128 sine-ripple heightfield ( a Rapier
//! heightfield collider and the displaced, gridded Phong plane ), and up to
//! 30 random spheres, boxes and cylinders dropped onto it, half a second
//! apart after the first three seconds. Each frame wakes the bodies above
//! y = 1 and pushes the slow ones down; meshes below −12 leave the scene but
//! stay in the world, as on the page.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::{Interval, Random, StepTimer, next_due};
use crate::physics::{PhysicsBody, PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use rapier3d::prelude::Vector;
use std::f64::consts::PI;
use std::sync::Arc;

const WIDTH: usize = 128;
const DEPTH: usize = 128;
const EXTENTS: f64 = 100.;
const MAX_HEIGHT: f64 = 8.;
const MIN_HEIGHT: f64 = -2.;
const PERIOD: f64 = 3.;
const MAX_OBJECTS: usize = 30;
const OBJECT_SIZE: f64 = 3.;

/// generateHeight(): concentric sine ripples between the heights.
fn heights() -> Vec<f32> {
    let range = MAX_HEIGHT - MIN_HEIGHT;
    let (w2, d2) = (WIDTH as f64 / 2., DEPTH as f64 / 2.);
    let mut data = Vec::with_capacity(WIDTH * DEPTH);
    for j in 0..DEPTH {
        for i in 0..WIDTH {
            let radius = (((i as f64 - w2) / w2).powf(2.) + ((j as f64 - d2) / d2).powf(2.)).sqrt();
            data.push((((radius * 12.).sin() + 1.) * 0.5 * range + MIN_HEIGHT) as f32);
        }
    }
    data
}
struct Dynamic {
    mesh: Object3D,
    body: rapier3d::prelude::RigidBodyHandle,
}
pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    random: Random,
    dynamics: Vec<Dynamic>,
    /// The page's Timer and its accumulated time, in seconds.
    time: f64,
    next_spawn: f64,
    last_frame: f64,
    now: f64,
    intervals: [Interval; 1],
    timer: StepTimer,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let data = heights();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.2,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.background = Color::from_hex(0xbfd1e5);
        let center = f64::from(data[WIDTH / 2 + DEPTH / 2 * WIDTH]);
        s.get_mut(c)?.position =
            Vector3::new(0., center * (MAX_HEIGHT - MIN_HEIGHT) + 5., EXTENTS / 2.);
        // OrbitControls without zoom, around the origin.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        // The terrain: the plane's vertices raised to the heights.
        let mut geometry =
            PlaneGeometry::build(EXTENTS, EXTENTS, WIDTH as u32 - 1, DEPTH as u32 - 1)?;
        geometry.rotate_x(-PI / 2.)?;
        if let Some(Attribute::F32(a)) = geometry.attributes.get_mut("position") {
            for (v, h) in a.array_mut().chunks_mut(3).zip(&data) {
                v[1] = *h;
            }
        }
        geometry.compute_vertex_normals()?;
        let mut grid =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(WIDTH as f64 - 1., DEPTH as f64 - 1.);
        let terrain = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Phong(MeshPhongMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0xc7c7c7),
                    map: Some(Arc::new(grid)),
                    ..Default::default()
                },
                ..Default::default()
            })),
        )));
        let n = s.get_mut(terrain)?;
        n.receive_shadow = true;
        n.cast_shadow = true;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xbbbbbb),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(100., 100., 50.);
        n.cast_shadow = true;
        n.shadow.extent = 50.;
        n.shadow.near = 200. / 30.;
        n.shadow.far = 200.;
        n.shadow.map_size = Some(2048);
        // initPhysics(): addHeightfield( terrainMesh, 127, 127, heightData, scale ).
        let mut physics = RapierPhysics::new();
        physics.add_heightfield(
            Vector3::ZERO,
            Quaternion::IDENTITY,
            WIDTH - 1,
            DEPTH - 1,
            data,
            Vector3::new(EXTENTS, 1., EXTENTS),
        )?;
        Ok(Self {
            controls,
            physics,
            random: Random(186),
            dynamics: vec![],
            time: 0.,
            next_spawn: PERIOD,
            last_frame: 0.,
            now: 0.,
            intervals: [Interval::new(0., 1000. / 60., 0)],
            timer: StepTimer::default(),
        })
    }
    fn phong(&mut self) -> Arc<Material> {
        let color = (self.random.next() * f64::from(1u32 << 24)).floor() as u32;
        Arc::new(Material::Phong(MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(color),
                ..Default::default()
            },
            ..Default::default()
        }))
    }
    /// generateObject(): a random sphere, box or cylinder high above the
    /// terrain; mass 15, restitution 0.3.
    fn generate(&mut self, s: &mut Scene) -> Result<()> {
        let kind = (self.random.next() * 3.).ceil() as u32;
        let (geometry, shape, material) = match kind {
            1 => {
                let radius = 1. + self.random.next() * OBJECT_SIZE;
                let g = SphereGeometry::build(radius, 20, 20)?;
                (g, PhysicsShape::Ball { radius }, self.phong())
            }
            2 => {
                let w = 1. + self.random.next() * OBJECT_SIZE;
                let h = 1. + self.random.next() * OBJECT_SIZE;
                let d = 1. + self.random.next() * OBJECT_SIZE;
                let g = BoxGeometry::build(w, h, d)?;
                (
                    g,
                    PhysicsShape::Box {
                        width: w,
                        height: h,
                        depth: d,
                    },
                    self.phong(),
                )
            }
            3 => {
                let radius = 1. + self.random.next() * OBJECT_SIZE;
                let height = 1. + self.random.next() * OBJECT_SIZE;
                let g = CylinderGeometry::build(radius, radius, height, 20, 1, false, 0., 2. * PI)?;
                (g, PhysicsShape::Cylinder { radius, height }, self.phong())
            }
            _ => return Err(crate::Error::Invalid("physics_rapier_terrain cone")),
        };
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(geometry), material)));
        let position = Vector3::new(
            (self.random.next() - 0.5) * WIDTH as f64 * 0.6,
            MAX_HEIGHT + OBJECT_SIZE + 15. + self.random.next() * 5.,
            (self.random.next() - 0.5) * DEPTH as f64 * 0.6,
        );
        let n = s.get_mut(mesh)?;
        n.position = position;
        n.receive_shadow = true;
        n.cast_shadow = true;
        let Some(PhysicsBody::Single(body)) =
            self.physics
                .add_mesh(s, mesh, &shape, OBJECT_SIZE * 5., 0.3)?
        else {
            return Err(crate::Error::Invalid("physics_rapier_terrain body"));
        };
        self.dynamics.push(Dynamic { mesh, body });
        self.next_spawn = self.time + 0.5;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
        }
        Ok(())
    }
    /// The steps due by the clock, then animate(): the Timer, a spawn when
    /// due, the fallen meshes out of the scene, updatePhysics().
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        while let Some((_, at)) = next_due(&mut self.intervals, self.now) {
            let delta = self.timer.delta(at);
            self.physics.step(s, delta)?;
        }
        let delta = (self.now - self.last_frame) / 1000.;
        self.last_frame = self.now;
        if self.dynamics.len() < MAX_OBJECTS && self.time > self.next_spawn {
            self.generate(s)?;
        }
        for i in (0..self.dynamics.len()).rev() {
            let mesh = self.dynamics[i].mesh;
            if s.get(mesh)?.position.y < MIN_HEIGHT - 10. {
                // scene.remove(): its body falls on in the world.
                s.get_mut(mesh)?.visible = false;
                self.dynamics.remove(i);
            }
        }
        for d in &self.dynamics {
            if s.get(d.mesh)?.position.y > 1. {
                let body = &mut self.physics.bodies[d.body];
                body.wake_up(true);
                let v = body.linvel();
                let (x, y, z) = (f64::from(v.x), f64::from(v.y), f64::from(v.z));
                if (x * x + y * y + z * z).sqrt() < 0.5 {
                    body.apply_impulse(Vector::new(0., -2., 0.), true);
                }
            }
        }
        self.time += delta;
        Ok(())
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
        // enableZoom = false.
        if wheel != 0. {
            return Ok(());
        }
        let camera = camera_state(s, c)?;
        if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(crate::Error::Invalid("physics_rapier_terrain parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
