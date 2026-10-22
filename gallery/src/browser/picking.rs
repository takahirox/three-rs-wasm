//! three.js examples/webgl_interactive_lines.html and
//! webgl_interactive_raycasting_points.html: the raycaster's hits on lines or
//! points under the pointer, marked as the camera circles the scene.
use super::gallery_random::Random;
use crate::raycast::Raycaster;
use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

struct Picking {
    objects: Vec<Object3D>,
    markers: Vec<Object3D>,
    pointer: Vector2,
    angle: f64,
    points: bool,
    elapsed: f64,
    marker: usize,
}

impl super::example::Example for Picking {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        Some((|| {
            if animate {
                self.angle += if self.points {
                    0.005
                } else {
                    0.1_f64.to_radians()
                };
                self.elapsed += delta;
            }
            let (radius, y) = if self.points {
                (200.0_f64.sqrt(), 10.0)
            } else {
                (100.0, 100.0 * self.angle.sin())
            };
            let a = self.angle
                + if self.points {
                    std::f64::consts::FRAC_PI_4
                } else {
                    0.0
                };
            scene.get_mut(camera)?.position = Vector3::new(radius * a.sin(), y, radius * a.cos());
            scene.look_at(camera, Vector3::ZERO)?;
            scene.update()?;
            let mut ray = Raycaster::default();
            ray.params.line_threshold = 3.0;
            ray.params.points_threshold = 0.1;
            let (c, world) = scene.camera(camera)?;
            ray.set_from_camera(self.pointer, c, world)?;
            let hits = ray.intersect_objects(scene, &self.objects, true)?;
            if self.points {
                if self.elapsed > 0.02
                    && let Some(hit) = hits.first()
                {
                    let m = scene.get_mut(self.markers[self.marker])?;
                    m.position = hit.point;
                    m.scale = Vector3::ONE;
                    self.marker = (self.marker + 1) % self.markers.len();
                    self.elapsed = 0.0;
                }
                if animate {
                    for &m in self.markers.iter() {
                        let n = scene.get_mut(m)?;
                        n.scale = (n.scale * 0.98).clamp(Vector3::splat(0.01), Vector3::ONE);
                    }
                }
            } else {
                let m = scene.get_mut(self.markers[0])?;
                m.visible = !hits.is_empty();
                if let Some(hit) = hits.first() {
                    m.position = hit.point;
                }
            }

            Ok(())
        })())
    }
    fn on_pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    fn on_scene_input(
        &mut self,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _dragging: bool,
    ) -> Option<Result<()>> {
        Some(Ok(()))
    }
    #[allow(clippy::too_many_arguments)]
    fn on_input(
        &mut self,
        _scene: &mut Scene,
        _camera: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Option<Result<()>> {
        Some(Ok(()))
    }
}

fn create(
    scene: &mut Scene,
    camera: Object3D,
    aspect: f64,
    points: bool,
) -> Result<super::expanded::Demo> {
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: if points { 45.0 } else { 70.0 },
        aspect,
        near: 1.0,
        far: 10000.0,
        ..Default::default()
    }));
    let mut objects = vec![];
    if points {
        for (i, color) in [Vector3::X, Vector3::Y, Vector3::new(0.0, 1.0, 1.0)]
            .into_iter()
            .enumerate()
        {
            let mut positions = vec![];
            let mut colors = vec![];
            for u in 0..80 {
                for v in 0..160 {
                    let u = u as f64 / 80.0;
                    let v = v as f64 / 160.0;
                    let y = ((u * std::f64::consts::PI * 4.0).cos()
                        + (v * std::f64::consts::PI * 8.0).sin())
                        / 20.0;
                    positions.push(Vector3::new(u - 0.5, y, v - 0.5));
                    let c = color * ((y + 0.1) * 5.0);
                    colors.extend([c.x as f32, c.y as f32, c.z as f32]);
                }
            }
            let mut geometry = BufferGeometry::default();
            geometry.set_from_points(&positions)?;
            geometry.set_attribute(
                "color",
                Attribute::F32(BufferAttribute::new(colors, 3, false)?),
            );
            if i > 0 {
                geometry.set_index(Some((0..12800).collect()));
            }
            if i == 2 {
                geometry.add_group(0, 12800, 0);
            }
            let mut material = PointsMaterial {
                size: 0.05,
                ..Default::default()
            };
            material.properties.vertex_colors = true;
            let object = scene.insert(NodeKind::Points(Points {
                geometry: Arc::new(geometry),
                material: Arc::new(Material::Points(material)),
            }));
            scene.get_mut(object)?.scale = Vector3::new(5.0, 10.0, 10.0);
            scene.get_mut(object)?.position.x = i as f64 * 5.0 - 5.0;
            objects.push(object);
        }
    } else {
        scene.background = Color::from_hex(0xf0f0f0);
        let mut random = Random(1);
        let mut position = Vector3::ZERO;
        let mut direction = Vector3::ZERO;
        let mut positions = vec![];
        for _ in 0..50 {
            direction += Vector3::new(
                random.next() - 0.5,
                random.next() - 0.5,
                random.next() - 0.5,
            );
            direction = direction.normalize_or_zero() * 10.0;
            position += direction;
            positions.push(position);
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_from_points(&positions)?;
        let geometry = Arc::new(geometry);
        fn transform(
            scene: &mut Scene,
            object: Object3D,
            random: &mut Random,
            spread: f64,
        ) -> Result<()> {
            let n = scene.get_mut(object)?;
            n.position = Vector3::new(
                random.next() - 0.5,
                random.next() - 0.5,
                random.next() - 0.5,
            ) * spread;
            n.quaternion = Euler {
                angles: Vector3::new(random.next(), random.next(), random.next())
                    * std::f64::consts::TAU,
                order: EulerOrder::XYZ,
            }
            .quaternion();
            n.scale = Vector3::new(
                random.next() + 0.5,
                random.next() + 0.5,
                random.next() + 0.5,
            );
            Ok(())
        }
        let parent = scene.insert(NodeKind::Group);
        transform(scene, parent, &mut random, 40.0)?;
        for _ in 0..50 {
            let mut material = LineBasicMaterial::default();
            material.properties.color = Color::from_hex((random.next() * 0xffffff as f64) as u32);
            let segments = random.next() <= 0.5;
            let object = scene.insert(NodeKind::Line(Line {
                geometry: geometry.clone(),
                material: Arc::new(Material::Line(material)),
                segments,
            }));
            transform(scene, object, &mut random, 400.0)?;
            scene.add(parent, object)?;
        }
        objects.push(parent);
    }
    let geometry = Arc::new(SphereGeometry::build(
        if points { 0.1 } else { 5.0 },
        32,
        32,
    )?);
    let mut material = MeshBasicMaterial::default();
    material.properties.color = Color::from_hex(0xff0000);
    let material = Arc::new(Material::Basic(material));
    let mut markers = vec![];
    for _ in 0..if points { 40 } else { 1 } {
        markers.push(scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            material.clone(),
        ))));
    }
    Ok(super::expanded::own(Box::new(Picking {
        objects,
        markers,
        pointer: Vector2::ZERO,
        angle: 0.0,
        points,
        elapsed: 0.0,
        marker: 0,
    })))
}

/// The program of three.js examples/webgl_interactive_lines.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_interactive_lines(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            rtt: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, _renderer, aspect) = page.gallery_scene()?;
            create(scene, camera, aspect, false)
        },
    )
    .await
}

/// The program of three.js examples/webgl_interactive_raycasting_points.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_interactive_raycasting_points(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            rtt: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, _renderer, aspect) = page.gallery_scene()?;
            create(scene, camera, aspect, true)
        },
    )
    .await
}
