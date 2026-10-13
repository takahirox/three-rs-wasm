//! physics_ammo_cloth: a cloth ( an ammo.js soft body patch ) hangs from an
//! arm on a hinge, next to a wall of randomly colored bricks. Q and A drive
//! the hinge's motor. Physics runs in the page's ammo.js
//! ( `super::ammo::AmmoSoftWorld` ); each frame steps the world with the
//! Timer's delta, writes the patch's nodes into the cloth's positions,
//! recomputes its vertex normals and copies the dynamic bodies' motion
//! states onto their meshes, as updatePhysics does.
use super::ammo::AmmoSoftWorld;
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::Random;
use crate::attribute::BufferAttribute;
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const MARGIN: f64 = 0.05;
const SEGMENTS_Z: u32 = 4 * 5;
const SEGMENTS_Y: u32 = 3 * 5;

pub(super) struct Demo {
    controls: Controls,
    physics: AmmoSoftWorld,
    /// The dynamic bodies' meshes, in the order of the physics' dynamic bodies.
    rigid: Vec<Object3D>,
    cloth: Object3D,
    cloth_body: u32,
    hinge: u32,
    arm_movement: f64,
    delta: f64,
    now: f64,
    pending: bool,
}

/// A mesh at a body's pose, its body in the world, and whether it moves.
struct Builder<'a> {
    s: &'a mut Scene,
    physics: &'a AmmoSoftWorld,
    rigid: Vec<Object3D>,
}

impl Builder<'_> {
    /// createRigidBody( object, shape, mass, pos, quat ) for a mesh.
    fn body(
        &mut self,
        mesh: Object3D,
        shape: (u32, f64, f64, f64),
        mass: f64,
        pos: Vector3,
    ) -> Result<u32> {
        let n = self.s.get_mut(mesh)?;
        n.position = pos;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let body = self.physics.rigid(
            shape.0, shape.1, shape.2, shape.3, MARGIN, mass, pos.x, pos.y, pos.z, 0., 0., 0., 1.,
        );
        if mass > 0. {
            self.rigid.push(mesh);
        }
        Ok(body)
    }
    /// createParalellepiped( sx, sy, sz, mass, pos, quat, material ).
    fn box_(
        &mut self,
        size: (f64, f64, f64),
        mass: f64,
        pos: Vector3,
        material: Arc<Material>,
    ) -> Result<(Object3D, u32)> {
        let mesh = self.s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(size.0, size.1, size.2)?),
            material,
        )));
        let body = self.body(
            mesh,
            (0, size.0 * 0.5, size.1 * 0.5, size.2 * 0.5),
            mass,
            pos,
        )?;
        Ok((mesh, body))
    }
}

fn phong(color: u32) -> Arc<Material> {
    let mut m = MeshPhongMaterial::default();
    m.properties.color = Color::from_hex(color);
    Arc::new(Material::Phong(m))
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        // initGraphics().
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.2,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-12., 7., 4.);
        s.background = Color::from_hex(0xbfd1e5);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
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
        n.position = Vector3::new(-7., 10., 15.);
        n.cast_shadow = true;
        n.shadow.extent = 10.;
        n.shadow.near = 2.;
        n.shadow.far = 50.;
        n.shadow.map_size = Some(1024);
        n.shadow.bias = -0.003;
        // Ammo().then( init ): initPhysics().
        let physics = super::ammo::soft_world(-9.8).await?;
        // createObjects(): the grid ( sRGB ) on the ground and on the cloth.
        let image =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        let grid = |repeat: Vector2| {
            let mut texture = image.clone();
            texture.srgb = true;
            texture.mipmap_filter = Some(Filter::Linear);
            texture.wrap_s = Wrapping::Repeat;
            texture.wrap_t = Wrapping::Repeat;
            texture.repeat = repeat;
            Arc::new(texture)
        };
        let mut ground_material = MeshPhongMaterial::default();
        ground_material.properties.map = Some(grid(Vector2::new(40., 40.)));
        let mut b = Builder {
            s,
            physics: &physics,
            rigid: vec![],
        };
        b.box_(
            (40., 1., 40.),
            0.,
            Vector3::new(0., -0.5, 0.),
            Arc::new(Material::Phong(ground_material)),
        )?;
        // The wall: Math.random colors, in creation order.
        let mut random = Random(186);
        let (brick_mass, brick_length, brick_depth) = (0.5, 1.2, 0.6);
        let brick_height = brick_length * 0.5;
        let (bricks_length, bricks_height) = (6, 8);
        let z0 = -(bricks_length as f64) * brick_length * 0.5;
        let mut pos = Vector3::new(0., brick_height * 0.5, z0);
        for j in 0..bricks_height {
            let odd = j % 2 == 1;
            pos.z = z0;
            if odd {
                pos.z -= 0.25 * brick_length;
            }
            let n_row = if odd {
                bricks_length + 1
            } else {
                bricks_length
            };
            for i in 0..n_row {
                let (mut length, mut mass) = (brick_length, brick_mass);
                if odd && (i == 0 || i == n_row - 1) {
                    length *= 0.5;
                    mass *= 0.5;
                }
                let color = (random.next() * (1 << 24) as f64).floor() as u32;
                b.box_((brick_depth, brick_height, length), mass, pos, phong(color))?;
                if odd && (i == 0 || i == n_row - 2) {
                    pos.z += 0.75 * brick_length;
                } else {
                    pos.z += brick_length;
                }
            }
            pos.y += brick_height;
        }
        // The cloth: the plane turned into x = −3, and its soft body patch.
        let (cloth_width, cloth_height) = (4., 3.);
        let cloth_pos = Vector3::new(-3., 3., 2.);
        let mut geometry = PlaneGeometry::build(cloth_width, cloth_height, SEGMENTS_Z, SEGMENTS_Y)?;
        geometry.rotate_y(PI * 0.5)?;
        geometry.translate(Vector3::new(
            cloth_pos.x,
            cloth_pos.y + cloth_height * 0.5,
            cloth_pos.z - cloth_width * 0.5,
        ))?;
        let mut cloth_material = MeshLambertMaterial::default();
        cloth_material.properties.side = Side::Double;
        cloth_material.properties.map =
            Some(grid(Vector2::new(SEGMENTS_Z as f64, SEGMENTS_Y as f64)));
        let cloth = b.s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Lambert(cloth_material)),
        )));
        let n = b.s.get_mut(cloth)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        // The bounding sphere stays the first one's.
        n.frustum_culled = false;
        let (top, bottom, x) = (cloth_pos.y + cloth_height, cloth_pos.y, cloth_pos.x);
        let (near, far) = (cloth_pos.z, cloth_pos.z - cloth_width);
        let cloth_body = physics.patch(
            x,
            top,
            near,
            x,
            top,
            far,
            x,
            bottom,
            near,
            x,
            bottom,
            far,
            SEGMENTS_Z + 1,
            SEGMENTS_Y + 1,
            0,
            true,
            10,
            10,
            0.9,
            MARGIN * 3.,
        );
        // The base, the pylon and the arm.
        let (arm_mass, arm_length) = (2., 3. + cloth_width);
        let pylon_height = cloth_pos.y + cloth_height;
        let base_material = phong(0x606060);
        b.box_(
            (1., 0.2, 1.),
            0.,
            Vector3::new(x, 0.1, cloth_pos.z - arm_length),
            base_material.clone(),
        )?;
        let (_, pylon) = b.box_(
            (0.4, pylon_height, 0.4),
            0.,
            Vector3::new(x, 0.5 * pylon_height, cloth_pos.z - arm_length),
            base_material.clone(),
        )?;
        let (_, arm) = b.box_(
            (0.4, 0.4, arm_length + 0.4),
            arm_mass,
            Vector3::new(x, pylon_height + 0.2, cloth_pos.z - 0.5 * arm_length),
            base_material,
        )?;
        // Glue the cloth to the arm; the arm's hinge.
        physics.append_anchor(cloth_body, 0, arm, false, 0.5);
        physics.append_anchor(cloth_body, SEGMENTS_Z, arm, false, 0.5);
        let hinge = physics.hinge(
            pylon,
            arm,
            0.,
            pylon_height * 0.5,
            0.,
            0.,
            -0.2,
            -arm_length * 0.5,
            0.,
            1.,
            0.,
        );
        let rigid = b.rigid;
        Ok(Self {
            controls,
            physics,
            rigid,
            cloth,
            cloth_body,
            hinge,
            arm_movement: 0.,
            delta: 0.,
            now: 0.,
            pending: false,
        })
    }

    /// updatePhysics( deltaTime ).
    fn update_physics(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        self.physics
            .enable_angular_motor(self.hinge, true, 0.8 * self.arm_movement, 50.);
        self.physics.step_simulation(delta, 10);
        let count = (SEGMENTS_Z + 1) * (SEGMENTS_Y + 1);
        let nodes = self.physics.nodes(self.cloth_body, count).to_vec();
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.cloth)?.kind {
            let g = Arc::make_mut(&mut m.geometry);
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(nodes, 3, false)?),
            );
            g.compute_vertex_normals()?;
        }
        let poses = self.physics.poses().to_vec();
        for (&mesh, p) in self.rigid.iter().zip(poses.as_chunks::<7>().0) {
            let n = s.get_mut(mesh)?;
            n.position = Vector3::new(p[0], p[1], p[2]);
            n.quaternion = Quaternion::from_xyzw(p[3], p[4], p[5], p[6]);
        }
        Ok(())
    }

    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
            self.delta += dt;
            self.pending = true;
        }
        Ok(())
    }
    /// animate(): timer.update(), then render() steps the world with the delta.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            let delta = std::mem::take(&mut self.delta);
            self.update_physics(s, delta)?;
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    /// keydown Q ( 81 ) and A ( 65 ) set the arm's movement; any keyup stops it.
    pub fn key(&mut self, code: u32, down: bool) {
        if !down {
            self.arm_movement = 0.;
        } else if code == 81 {
            self.arm_movement = 1.;
        } else if code == 65 {
            self.arm_movement = -1.;
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
        Ok(())
    }
    /// The page's clock set directly: the Timer's delta from the previous time.
    pub fn seek(&mut self, t: f64) {
        self.delta += (t * 1000. - self.now).max(0.) / 1000.;
        self.now = t * 1000.;
        self.pending = true;
    }
}
