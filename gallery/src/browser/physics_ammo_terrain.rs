//! physics_ammo_terrain: random spheres, boxes, cylinders and cones drop
//! every three seconds onto a sine-wave heightfield terrain. Physics runs in
//! the page's ammo.js ( `super::ammo::AmmoSoftWorld` as a rigid-only
//! btDiscreteDynamicsWorld with a btHeightfieldTerrainShape ); each frame
//! spawns an object when due, steps the world with the Timer's delta and
//! copies the objects' motion states onto their meshes, as render() does.
use super::ammo::AmmoSoftWorld;
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::Random;
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const TERRAIN_WIDTH_EXTENTS: f64 = 100.;
const TERRAIN_DEPTH_EXTENTS: f64 = 100.;
const TERRAIN_WIDTH: u32 = 128;
const TERRAIN_DEPTH: u32 = 128;
const TERRAIN_MAX_HEIGHT: f64 = 8.;
const TERRAIN_MIN_HEIGHT: f64 = -2.;
const OBJECT_TIME_PERIOD: f64 = 3.;
const MAX_NUM_OBJECTS: usize = 30;

pub(super) struct Demo {
    controls: Controls,
    physics: AmmoSoftWorld,
    /// The dynamic objects' meshes, in the order of the physics' dynamic bodies.
    objects: Vec<Object3D>,
    random: Random,
    time: f64,
    time_next_spawn: f64,
    delta: f64,
    now: f64,
    pending: bool,
}

/// generateHeight( width, depth, minHeight, maxHeight ): a sine wave over
/// the distance from the centre.
fn generate_height(width: u32, depth: u32, min_height: f64, max_height: f64) -> Vec<f32> {
    let h_range = max_height - min_height;
    let (w2, d2) = (width as f64 / 2., depth as f64 / 2.);
    let phase_mult = 12.;
    let mut data = Vec::with_capacity((width * depth) as usize);
    for j in 0..depth {
        for i in 0..width {
            let (x, z) = ((i as f64 - w2) / w2, (j as f64 - d2) / d2);
            let radius = (x * x + z * z).sqrt();
            let height = ((radius * phase_mult).sin() + 1.) * 0.5 * h_range + min_height;
            data.push(height as f32);
        }
    }
    data
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let height_data = generate_height(
            TERRAIN_WIDTH,
            TERRAIN_DEPTH,
            TERRAIN_MIN_HEIGHT,
            TERRAIN_MAX_HEIGHT,
        );
        // initGraphics().
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
        let centre = (TERRAIN_WIDTH / 2 + TERRAIN_DEPTH / 2 * TERRAIN_WIDTH) as usize;
        s.get_mut(c)?.position = Vector3::new(
            0.,
            height_data[centre] as f64 * (TERRAIN_MAX_HEIGHT - TERRAIN_MIN_HEIGHT) + 5.,
            TERRAIN_DEPTH_EXTENTS / 2.,
        );
        s.look_at(c, Vector3::ZERO)?;
        // OrbitControls ( enableZoom = false ) around the origin.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let mut geometry = PlaneGeometry::build(
            TERRAIN_WIDTH_EXTENTS,
            TERRAIN_DEPTH_EXTENTS,
            TERRAIN_WIDTH - 1,
            TERRAIN_DEPTH - 1,
        )?;
        geometry.rotate_x(-PI / 2.)?;
        if let Some(Attribute::F32(a)) = geometry.attributes.get_mut("position") {
            for (v, &h) in a
                .array_mut()
                .as_chunks_mut::<3>()
                .0
                .iter_mut()
                .zip(&height_data)
            {
                v[1] = h;
            }
        }
        geometry.compute_vertex_normals()?;
        // The grid ( no color space ), one cell per quad.
        let mut grid = decode_texture_image(&fetch("assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new((TERRAIN_WIDTH - 1) as f64, (TERRAIN_DEPTH - 1) as f64);
        let mut ground_material = MeshPhongMaterial::default();
        ground_material.properties.color = Color::from_hex(0xc7c7c7);
        ground_material.properties.map = Some(Arc::new(grid));
        let terrain = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Phong(ground_material)),
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
        let d_light = 200.;
        let n = s.get_mut(light)?;
        n.position = Vector3::new(100., 100., 50.);
        n.cast_shadow = true;
        n.shadow.extent = d_light * 0.25;
        n.shadow.near = d_light / 30.;
        n.shadow.far = d_light;
        n.shadow.map_size = Some(1024 * 2);
        // initPhysics(): the terrain body, shifted since bullet re-centres it.
        let physics = super::ammo::rigid_world(-6.).await?;
        physics.heightfield(
            TERRAIN_WIDTH,
            TERRAIN_DEPTH,
            &height_data,
            TERRAIN_MIN_HEIGHT,
            TERRAIN_MAX_HEIGHT,
            TERRAIN_WIDTH_EXTENTS / (TERRAIN_WIDTH - 1) as f64,
            TERRAIN_DEPTH_EXTENTS / (TERRAIN_DEPTH - 1) as f64,
            0.05,
            (TERRAIN_MAX_HEIGHT + TERRAIN_MIN_HEIGHT) / 2.,
        );
        Ok(Self {
            controls,
            physics,
            objects: vec![],
            random: Random(186),
            time: 0.,
            time_next_spawn: OBJECT_TIME_PERIOD,
            delta: 0.,
            now: 0.,
            pending: false,
        })
    }

    /// generateObject(): Math.random picks the type, the size, the color
    /// ( createObjectMaterial ) and the position, in the page's order.
    fn generate_object(&mut self, s: &mut Scene) -> Result<()> {
        let object_type = (self.random.next() * 4.).ceil() as u32;
        let object_size = 3.;
        let margin = 0.05;
        let r = &mut self.random;
        let (geometry, shape) = match object_type {
            1 => {
                let radius = 1. + r.next() * object_size;
                (
                    SphereGeometry::build(radius, 20, 20)?,
                    (1, radius, 0., 0., margin),
                )
            }
            2 => {
                let sx = 1. + r.next() * object_size;
                let sy = 1. + r.next() * object_size;
                let sz = 1. + r.next() * object_size;
                (
                    BoxGeometry::build(sx, sy, sz)?,
                    (0, sx * 0.5, sy * 0.5, sz * 0.5, margin),
                )
            }
            3 => {
                let radius = 1. + r.next() * object_size;
                let height = 1. + r.next() * object_size;
                (
                    CylinderGeometry::build(radius, radius, height, 20, 1, false, 0., 2. * PI)?,
                    (2, radius, height * 0.5, radius, margin),
                )
            }
            _ => {
                // The cone keeps its shape's default margin.
                let radius = 1. + r.next() * object_size;
                let height = 2. + r.next() * object_size;
                (
                    CylinderGeometry::build(0., radius, height, 20, 2, false, 0., 2. * PI)?,
                    (3, radius, height, 0., -1.),
                )
            }
        };
        let mut material = MeshPhongMaterial::default();
        material.properties.color = Color::from_hex((r.next() * (1 << 24) as f64).floor() as u32);
        let pos = Vector3::new(
            (r.next() - 0.5) * TERRAIN_WIDTH as f64 * 0.6,
            TERRAIN_MAX_HEIGHT + object_size + 2.,
            (r.next() - 0.5) * TERRAIN_DEPTH as f64 * 0.6,
        );
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Phong(material)),
        )));
        let n = s.get_mut(mesh)?;
        n.position = pos;
        n.receive_shadow = true;
        n.cast_shadow = true;
        self.physics.body(
            shape.0,
            shape.1,
            shape.2,
            shape.3,
            shape.4,
            object_size * 5.,
            pos.x,
            pos.y,
            pos.z,
            0.,
            0.,
            0.,
            1.,
            false,
        );
        self.objects.push(mesh);
        Ok(())
    }

    /// render(): spawn when due, updatePhysics( deltaTime ), then advance time.
    fn render(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        if self.objects.len() < MAX_NUM_OBJECTS && self.time > self.time_next_spawn {
            self.generate_object(s)?;
            self.time_next_spawn = self.time + OBJECT_TIME_PERIOD;
        }
        self.physics.step_simulation(delta, 10);
        let poses = self.physics.poses().to_vec();
        for (&mesh, p) in self.objects.iter().zip(poses.as_chunks::<7>().0) {
            let n = s.get_mut(mesh)?;
            n.position = Vector3::new(p[0], p[1], p[2]);
            n.quaternion = Quaternion::from_xyzw(p[3], p[4], p[5], p[6]);
        }
        self.time += delta;
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
    /// animate(): timer.update(), then render() with the delta.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            let delta = std::mem::take(&mut self.delta);
            self.render(s, delta)?;
        }
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
        Ok(())
    }
    /// The page's clock set directly: the Timer's delta from the previous time,
    /// negative when the clock goes back ( Timer.update does not clamp it ).
    pub fn seek(&mut self, t: f64) {
        self.delta += (t * 1000. - self.now) / 1000.;
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

/// The program of three.js examples/physics_ammo_terrain.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn physics_ammo_terrain(
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
                Box::new(Demo::create(scene, camera, 494, renderer).await?),
            ))
        },
    )
    .await
}
