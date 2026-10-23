//! physics_ammo_rope: a ball hangs from a rope ( an ammo.js soft body ) tied
//! to an arm on a hinge, next to a wall of randomly colored bricks. Q and A
//! drive the hinge's motor. Physics runs in the page's ammo.js
//! ( `super::ammo::AmmoSoftWorld` ); each frame steps the world with the
//! Timer's delta, writes the rope's nodes into its line positions and copies
//! the dynamic bodies' motion states onto their meshes, as updatePhysics does.
use super::ammo::AmmoSoftWorld;
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::Random;
use crate::attribute::BufferAttribute;
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const MARGIN: f64 = 0.05;
const ROPE_SEGMENTS: u32 = 10;

pub(super) struct Demo {
    controls: Controls,
    physics: AmmoSoftWorld,
    /// The dynamic bodies' meshes, in the order of the physics' dynamic bodies.
    rigid: Vec<Object3D>,
    rope: Object3D,
    rope_body: u32,
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
        s.get_mut(c)?.position = Vector3::new(-7., 5., 8.);
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
        n.position = Vector3::new(-10., 10., 5.);
        n.cast_shadow = true;
        n.shadow.extent = 10.;
        n.shadow.near = 2.;
        n.shadow.far = 50.;
        n.shadow.map_size = Some(1024);
        // Ammo().then( init ): initPhysics().
        let physics = super::ammo::soft_world(-9.8).await?;
        // createObjects(): the ground with the grid ( sRGB, repeated 40 × 40 ).
        let mut grid = decode_texture_image(&fetch("assets/physics/grid.png").await?).await?;
        grid.srgb = true;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(40., 40.);
        let mut ground_material = MeshPhongMaterial::default();
        ground_material.properties.map = Some(Arc::new(grid));
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
        // The ball.
        let (ball_mass, ball_radius) = (1.2, 0.6);
        let ball = b.s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(ball_radius, 20, 20)?),
            phong(0x202020),
        )));
        let ball_position = Vector3::new(-3., 2., 0.);
        let ball_body = b.body(ball, (1, ball_radius, 0., 0.), ball_mass, ball_position)?;
        physics.set_friction(ball_body, 0.5);
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
        // The rope: LineSegments over 11 nodes, and its soft body.
        let (rope_length, rope_mass) = (4., 3.);
        let rope_pos = Vector3::new(
            ball_position.x,
            ball_position.y + ball_radius,
            ball_position.z,
        );
        let segment = rope_length / ROPE_SEGMENTS as f64;
        let positions: Vec<f32> = (0..=ROPE_SEGMENTS)
            .flat_map(|i| {
                [
                    rope_pos.x as f32,
                    (rope_pos.y + i as f64 * segment) as f32,
                    rope_pos.z as f32,
                ]
            })
            .collect();
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        geometry.set_index(Some((0..ROPE_SEGMENTS).flat_map(|i| [i, i + 1]).collect()));
        let material = LineBasicMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0x000000),
                ..Default::default()
            },
            ..Default::default()
        };
        let rope = b.s.insert(NodeKind::Line(Line {
            geometry: Arc::new(geometry),
            material: Arc::new(Material::Line(material)),
            segments: true,
        }));
        let n = b.s.get_mut(rope)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        // The bounding sphere is the first one's ( computeBoundingSphere once ).
        n.frustum_culled = false;
        let rope_body = physics.rope(
            rope_pos.x,
            rope_pos.y,
            rope_pos.z,
            rope_pos.x,
            rope_pos.y + rope_length,
            rope_pos.z,
            ROPE_SEGMENTS - 1,
            0,
            10,
            10,
            rope_mass,
            MARGIN * 3.,
        );
        // The base, the pylon and the arm.
        let (arm_mass, arm_length) = (2., 3.);
        let pylon_height = rope_pos.y + rope_length;
        let base_material = phong(0x606060);
        b.box_(
            (1., 0.2, 1.),
            0.,
            Vector3::new(rope_pos.x, 0.1, rope_pos.z - arm_length),
            base_material.clone(),
        )?;
        let (_, pylon) = b.box_(
            (0.4, pylon_height, 0.4),
            0.,
            Vector3::new(rope_pos.x, 0.5 * pylon_height, rope_pos.z - arm_length),
            base_material.clone(),
        )?;
        let (_, arm) = b.box_(
            (0.4, 0.4, arm_length + 0.4),
            arm_mass,
            Vector3::new(
                rope_pos.x,
                pylon_height + 0.2,
                rope_pos.z - 0.5 * arm_length,
            ),
            base_material,
        )?;
        // Glue the rope's ends to the ball and the arm; the arm's hinge.
        physics.append_anchor(rope_body, 0, ball_body, true, 1.);
        physics.append_anchor(rope_body, ROPE_SEGMENTS, arm, true, 1.);
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
            rope,
            rope_body,
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
            .enable_angular_motor(self.hinge, true, 1.5 * self.arm_movement, 50.);
        self.physics.step_simulation(delta, 10);
        let nodes = self
            .physics
            .nodes(self.rope_body, ROPE_SEGMENTS + 1)
            .to_vec();
        if let NodeKind::Line(l) = &mut s.get_mut(self.rope)?.kind {
            Arc::make_mut(&mut l.geometry).set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(nodes, 3, false)?),
            );
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

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/physics_ammo_rope.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn physics_ammo_rope(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.2,
                2000.,
                Box::new(Demo::create(scene, camera, 491, renderer).await?),
            ))
        },
    )
    .await
}
