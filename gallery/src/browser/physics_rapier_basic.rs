//! physics_rapier_basic: a 10 × 10 floor, and every second a box, a sphere
//! or a rounded box with a random color dropped from 6–9 m, simulated by
//! RapierPhysics ( rapier3d 0.26.1, the crate of rapier.js 0.17.3 ) on its
//! 16 ms interval, with RapierHelper's collider outlines, a 1024² PCF shadow
//! map and damped OrbitControls. Meshes that fall below −10 are removed from
//! the scene and the world in animate().
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::gtao::rounded_box;
use super::rapier_common::{Helper, Interval, Random, StepTimer, next_due};
use crate::physics::{PhysicsShape, RapierPhysics};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

/// scene.children: animate() walks it with for…of while removing, so the
/// child after a removed one is skipped that frame.
enum Child {
    Other,
    Mesh(Object3D),
}
pub(super) struct Demo {
    controls: Controls,
    physics: RapierPhysics,
    random: Random,
    geometries: Vec<(Arc<BufferGeometry>, PhysicsShape)>,
    children: Vec<Child>,
    helper: Helper,
    /// The example clock in ms; the step and addBody intervals.
    now: f64,
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
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 3., 10.);
        s.background = Color::from_hex(0xbfd1e5);
        let mut children = vec![];
        let ambient = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x555555),
            ground: Color::from_hex(0xffffff),
            intensity: 1.,
        }));
        s.get_mut(ambient)?.position = Vector3::Y;
        children.push(Child::Other);
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
        children.push(Child::Other);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        // The floor: TextureLoader's grid ( flipY, no color space, mipmapped ),
        // repeated 20 × 20.
        let mut grid =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(20., 20.);
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(10., 0.5, 10.)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                properties: MaterialProperties {
                    map: Some(Arc::new(grid)),
                    ..Default::default()
                },
                energy_conservation: true,
                ..Default::default()
            })),
        )));
        let n = s.get_mut(floor)?;
        n.receive_shadow = true;
        n.position.y = -0.25;
        children.push(Child::Mesh(floor));
        // initPhysics(): addScene( scene ) takes the floor ( mass 0 ).
        let mut physics = RapierPhysics::new();
        physics.add_mesh(
            s,
            floor,
            &PhysicsShape::Box {
                width: 10.,
                height: 0.5,
                depth: 10.,
            },
            0.,
            0.,
        )?;
        let geometries = vec![
            (
                Arc::new(BoxGeometry::build(1., 1., 1.)?),
                PhysicsShape::Box {
                    width: 1.,
                    height: 1.,
                    depth: 1.,
                },
            ),
            (
                Arc::new(SphereGeometry::build(0.5, 32, 16)?),
                PhysicsShape::Ball { radius: 0.5 },
            ),
            (
                Arc::new(rounded_box(1., 1., 1., 2, 0.25)?),
                PhysicsShape::RoundedBox {
                    width: 1.,
                    height: 1.,
                    depth: 1.,
                    radius: 0.25,
                },
            ),
        ];
        let mut d = Self {
            controls,
            physics,
            random: Random(186),
            geometries,
            children,
            helper: Helper::new(s)?,
            now: 0.,
            // RapierPhysics()'s step interval, then the page's addBody one.
            intervals: [
                Interval::new(0., 1000. / 60., 0),
                Interval::new(0., 1000., 1),
            ],
            timer: StepTimer::default(),
        };
        d.add_body(s)?;
        d.children.push(Child::Other);
        Ok(d)
    }
    /// addBody(): a random geometry, color and position; mass 1, restitution 0.5.
    fn add_body(&mut self, s: &mut Scene) -> Result<()> {
        let k = (self.random.next() * self.geometries.len() as f64).floor() as usize;
        let (geometry, shape) = self.geometries[k].clone();
        let color = Color::from_hex((self.random.next() * 16777215.).floor() as u32);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Standard(MeshStandardMaterial {
                properties: MaterialProperties {
                    color,
                    ..Default::default()
                },
                energy_conservation: true,
                ..Default::default()
            })),
        )));
        let n = s.get_mut(mesh)?;
        n.cast_shadow = true;
        n.position = Vector3::new(
            self.random.next() * 2. - 1.,
            self.random.next() * 3. + 6.,
            self.random.next() * 2. - 1.,
        );
        self.physics.add_mesh(s, mesh, &shape, 1., 0.5)?;
        self.children.push(Child::Mesh(mesh));
        Ok(())
    }
    /// The intervals due by the clock, in order.
    fn advance(&mut self, s: &mut Scene) -> Result<()> {
        while let Some((i, at)) = next_due(&mut self.intervals, self.now) {
            if i == 0 {
                let delta = self.timer.delta(at);
                self.physics.step(s, delta)?;
            } else {
                self.add_body(s)?;
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
    /// The intervals due by the clock, then animate(): fallen meshes out,
    /// the helper, then controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.advance(s)?;
        let mut i = 0;
        while i < self.children.len() {
            if let Child::Mesh(m) = self.children[i]
                && s.get(m)?.position.y < -10.
            {
                self.children.remove(i);
                s.dispose(m)?;
                self.physics.remove_mesh(m);
                // for…of moves on to the next index: the shifted child is skipped.
                i += 1;
                continue;
            }
            i += 1;
        }
        self.helper.update(s, &mut self.physics)?;
        self.controls.update(s, c)
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
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(crate::Error::Invalid("physics_rapier_basic parameter"))
    }
    /// The next frame runs the intervals due by then; earlier times run none.
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
    }
}
