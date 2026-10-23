//! physics_ammo_volume: two soft volumes ( ammo.js soft bodies made from a
//! sphere's and a box's triangles, inflated by pressure ) fall on a ramp;
//! clicking shoots a ball along the pointer's ray. Physics runs in the
//! page's ammo.js ( `super::ammo::AmmoSoftWorld` ); each frame steps the
//! world with the Timer's delta, writes the soft bodies' node positions and
//! normals into the meshes' vertices through processGeometry's
//! associations, copies the dynamic bodies' motion states onto their meshes
//! and then processes a pending click, as render() does.
use super::ammo::AmmoSoftWorld;
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

const MARGIN: f64 = 0.05;
const BALL_MASS: f64 = 3.;
const BALL_RADIUS: f64 = 0.4;

/// A soft volume's mesh, its body and, per body node, the mesh's vertex
/// offsets at that node's position ( ammoIndexAssociation ).
struct Volume {
    mesh: Object3D,
    body: u32,
    association: Vec<Vec<usize>>,
}

pub(super) struct Demo {
    controls: Controls,
    physics: AmmoSoftWorld,
    /// The dynamic bodies' meshes, in the order of the physics' dynamic bodies.
    rigid: Vec<Object3D>,
    volumes: Vec<Volume>,
    ball_geometry: Arc<BufferGeometry>,
    ball_material: Arc<Material>,
    click: Option<Vector2>,
    delta: f64,
    now: f64,
    pending: bool,
}

fn phong(color: u32) -> MeshPhongMaterial {
    let mut m = MeshPhongMaterial::default();
    m.properties.color = Color::from_hex(color);
    m
}

fn f32_array(g: &BufferGeometry, name: &str) -> Result<Vec<f32>> {
    match g.get_attribute(name) {
        Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
        _ => Err(crate::Error::Invalid("float attribute")),
    }
}

/// The merged positions, the merged triangles and the associations.
type Processed = (Vec<f32>, Vec<u32>, Vec<Vec<usize>>);

/// processGeometry( bufGeometry ): BufferGeometryUtils.mergeVertices over the
/// positions and index alone, then mapIndices.
fn process_geometry(g: &BufferGeometry) -> Result<Processed> {
    let vertices = f32_array(g, "position")?;
    let count = vertices.len() / 3;
    let index: Vec<u32> = match g.get_index() {
        Some(index) => index.to_vec(),
        None => (0..count as u32).collect(),
    };
    // mergeVertices( geometry, 1e-4 ): the hash truncates each component.
    let tolerance = 1e-4_f64;
    let multiplier = 10_f64.powf((1. / tolerance).log10());
    let additive = tolerance * 0.5 * multiplier;
    let hash = |v: f32| (v as f64 * multiplier + additive).trunc() as i64;
    let mut hash_to_index = HashMap::new();
    let (mut merged, mut indices) = (vec![], vec![]);
    for &i in &index {
        let p = &vertices[i as usize * 3..i as usize * 3 + 3];
        let key = (hash(p[0]), hash(p[1]), hash(p[2]));
        let next = *hash_to_index.entry(key).or_insert_with(|| {
            merged.extend_from_slice(p);
            merged.len() as u32 / 3 - 1
        });
        indices.push(next);
    }
    // mapIndices: every mesh vertex within 1e-6 of each merged vertex.
    let equal = |a: &[f32], b: &[f32]| (0..3).all(|k| ((b[k] - a[k]) as f64).abs() < 1e-6);
    let association = merged
        .as_chunks::<3>()
        .0
        .iter()
        .map(|m| {
            (0..count)
                .filter(|&j| equal(m, &vertices[j * 3..j * 3 + 3]))
                .map(|j| j * 3)
                .collect()
        })
        .collect();
    Ok((merged, indices, association))
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
        n.shadow.extent = 20.;
        n.shadow.near = 2.;
        n.shadow.far = 50.;
        n.shadow.map_size = Some(1024);
        // Ammo().then( init ): initPhysics().
        let physics = super::ammo::soft_world(-9.8).await?;
        // createObjects(): the ground with the grid ( sRGB ).
        let mut grid = decode_texture_image(&fetch("assets/physics/grid.png").await?).await?;
        grid.srgb = true;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(40., 40.);
        let mut ground_material = phong(0xffffff);
        ground_material.properties.map = Some(Arc::new(grid));
        let paralellepiped = |s: &mut Scene,
                              size: (f64, f64, f64),
                              pos: Vector3,
                              quat: Quaternion,
                              material: MeshPhongMaterial|
         -> Result<()> {
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(size.0, size.1, size.2)?),
                Arc::new(Material::Phong(material)),
            )));
            let n = s.get_mut(mesh)?;
            n.position = pos;
            n.quaternion = quat;
            n.cast_shadow = true;
            n.receive_shadow = true;
            physics.rigid(
                0,
                size.0 * 0.5,
                size.1 * 0.5,
                size.2 * 0.5,
                MARGIN,
                0.,
                pos.x,
                pos.y,
                pos.z,
                quat.x,
                quat.y,
                quat.z,
                quat.w,
            );
            Ok(())
        };
        paralellepiped(
            s,
            (40., 1., 40.),
            Vector3::new(0., -0.5, 0.),
            Quaternion::IDENTITY,
            ground_material,
        )?;
        // The soft volumes, mapped with colors.png ( no color space ).
        let mut colors =
            decode_texture_image(&fetch("assets/spot-skinning/colors.png").await?).await?;
        colors.srgb = false;
        colors.mipmap_filter = Some(Filter::Linear);
        let colors = Arc::new(colors);
        let mut volumes = vec![];
        let mut sphere = SphereGeometry::build(1.5, 40, 25)?;
        sphere.translate(Vector3::new(5., 5., 0.))?;
        let mut box_ = BoxGeometry::segmented(1., 1., 5., 4, 4, 20)?;
        box_.translate(Vector3::new(-2., 5., 0.))?;
        for (geometry, pressure) in [(sphere, 250.), (box_, 120.)] {
            let (vertices, indices, association) = process_geometry(&geometry)?;
            let mut material = phong(0xffffff);
            material.properties.map = Some(colors.clone());
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                Arc::new(Material::Phong(material)),
            )));
            let n = s.get_mut(mesh)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            n.frustum_culled = false;
            let body = physics.tri_mesh(
                &vertices, &indices, true, 40, 40, 0x11, 0.1, 0.01, pressure, 0.9, 0.9, 15., MARGIN,
            );
            volumes.push(Volume {
                mesh,
                body,
                association,
            });
        }
        // The ramp.
        paralellepiped(
            s,
            (10., 1., 4.),
            Vector3::new(3., 1., 0.),
            Quaternion::from_axis_angle(Vector3::Z, 30. * PI / 180.),
            phong(0x606060),
        )?;
        Ok(Self {
            controls,
            physics,
            rigid: vec![],
            volumes,
            ball_geometry: Arc::new(SphereGeometry::build(BALL_RADIUS, 18, 16)?),
            ball_material: Arc::new(Material::Phong(phong(0x202020))),
            click: None,
            delta: 0.,
            now: 0.,
            pending: false,
        })
    }

    /// updatePhysics( deltaTime ).
    fn update_physics(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        self.physics.step_simulation(delta, 10);
        for volume in &self.volumes {
            let count = volume.association.len();
            let nodes = self
                .physics
                .nodes_with_normals(volume.body, count as u32)
                .to_vec();
            let (positions, normals) = nodes.split_at(count * 3);
            let NodeKind::Mesh(m) = &mut s.get_mut(volume.mesh)?.kind else {
                continue;
            };
            let g = Arc::make_mut(&mut m.geometry);
            for (name, values) in [("position", positions), ("normal", normals)] {
                let Some(Attribute::F32(a)) = g.attributes.get_mut(name) else {
                    continue;
                };
                let array = a.array_mut();
                for (offsets, v) in volume.association.iter().zip(values.as_chunks::<3>().0) {
                    for &o in offsets {
                        array[o..o + 3].copy_from_slice(v);
                    }
                }
            }
        }
        let poses = self.physics.poses().to_vec();
        for (&mesh, p) in self.rigid.iter().zip(poses.as_chunks::<7>().0) {
            let n = s.get_mut(mesh)?;
            n.position = Vector3::new(p[0], p[1], p[2]);
            n.quaternion = Quaternion::from_xyzw(p[3], p[4], p[5], p[6]);
        }
        Ok(())
    }

    /// processClick(): a ball one unit along the pointer's ray, thrown at 14.
    fn process_click(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let Some(ndc) = self.click.take() else {
            return Ok(());
        };
        let (camera, world) = s.camera(c)?;
        let ray = camera.ray(ndc, world)?;
        let pos = ray.origin + ray.direction;
        let ball = s.insert(NodeKind::Mesh(Mesh::new(
            self.ball_geometry.clone(),
            self.ball_material.clone(),
        )));
        let n = s.get_mut(ball)?;
        n.position = pos;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let body = self.physics.rigid(
            1,
            BALL_RADIUS,
            0.,
            0.,
            MARGIN,
            BALL_MASS,
            pos.x,
            pos.y,
            pos.z,
            0.,
            0.,
            0.,
            1.,
        );
        self.physics.set_friction(body, 0.5);
        let v = ray.direction * 14.;
        self.physics.set_linear_velocity(body, v.x, v.y, v.z);
        self.rigid.push(ball);
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
    /// animate(): timer.update(), then render() steps the world with the
    /// delta and processes the click.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            let delta = std::mem::take(&mut self.delta);
            self.update_physics(s, delta)?;
            self.process_click(s, c)?;
        }
        Ok(())
    }
    /// The window's pointerdown ( kind 1 ) at window-relative NDC, unless a
    /// click is already pending.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind == 1 && self.click.is_none() {
            self.click = Some(Vector2::new(x, y));
        }
    }
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

/// The program of three.js examples/physics_ammo_volume.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn physics_ammo_volume(
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
                Box::new(Demo::create(scene, camera, 493, renderer).await?),
            ))
        },
    )
    .await
}
