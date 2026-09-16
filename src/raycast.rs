use crate::{Result, camera::Camera, material::Side, math::*, scene::*};

#[derive(Clone, Debug)]
pub struct Intersection {
    pub distance: f64,
    pub point: Vector3,
    pub object: Object3D,
    pub face_index: Option<usize>,
    pub index: Option<usize>,
    pub uv: Option<Vector2>,
    pub normal: Option<Vector3>,
    pub material_index: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct RaycasterParams {
    pub line_threshold: f64,
    pub points_threshold: f64,
}
impl Default for RaycasterParams {
    fn default() -> Self {
        Self {
            line_threshold: 1.0,
            points_threshold: 1.0,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Raycaster {
    pub camera: Option<(Camera, Matrix4)>,
    pub ray: Ray,
    pub near: f64,
    pub far: f64,
    pub layers: Layers,
    pub params: RaycasterParams,
}
impl Default for Raycaster {
    fn default() -> Self {
        Self {
            camera: None,
            ray: Ray::default(),
            near: 0.0,
            far: f64::INFINITY,
            layers: Layers::default(),
            params: RaycasterParams::default(),
        }
    }
}
pub trait Raycast {
    fn raycast(
        &self,
        raycaster: &Raycaster,
        object: Object3D,
        hits: &mut Vec<Intersection>,
    ) -> Result<()>;
}
pub trait FrustumIntersect {
    fn intersects_frustum(&self, frustum: &Frustum) -> Result<bool>;
}
impl FrustumIntersect for Node {
    fn intersects_frustum(&self, frustum: &Frustum) -> Result<bool> {
        let Some(geometry) = self.geometry() else {
            return Ok(true);
        };
        let sphere = match geometry.bounding_sphere {
            Some(s) => s,
            None => {
                let mut geometry = (**geometry).clone();
                geometry.compute_bounding_sphere()?
            }
        };
        Ok(frustum.intersects_sphere(sphere.transformed(self.matrix_world)))
    }
}
impl Raycaster {
    pub fn set(&mut self, origin: Vector3, direction: Vector3) {
        self.ray = Ray { origin, direction };
    }
    pub fn set_from_camera(&mut self, ndc: Vector2, camera: &Camera, world: Matrix4) -> Result<()> {
        self.ray = camera.ray(ndc, world)?;
        self.camera = Some((camera.clone(), world));
        Ok(())
    }
    pub fn intersect_object(
        &self,
        scene: &Scene,
        object: Object3D,
        recursive: bool,
    ) -> Result<Vec<Intersection>> {
        self.intersect_objects(scene, &[object], recursive)
    }
    pub fn intersect_objects(
        &self,
        scene: &Scene,
        objects: &[Object3D],
        recursive: bool,
    ) -> Result<Vec<Intersection>> {
        let mut hits = Vec::new();
        for &object in objects {
            let descendants = if recursive {
                scene.traverse(object, false)?
            } else {
                vec![object]
            };
            for h in descendants {
                let n = scene.get(h)?;
                if n.layers.test(self.layers) {
                    n.raycast(self, h, &mut hits)?;
                }
            }
        }
        hits.sort_by(|a, b| a.distance.total_cmp(&b.distance));
        Ok(hits)
    }
    fn in_range(&self, distance: f64) -> bool {
        distance >= self.near && distance <= self.far
    }
}
impl Raycast for Node {
    fn raycast(&self, r: &Raycaster, object: Object3D, hits: &mut Vec<Intersection>) -> Result<()> {
        let Some(geometry) = self.geometry() else {
            return Ok(());
        };
        if self.matrix_world.determinant() == 0.0 {
            return Ok(());
        }
        let ray = r.ray.transformed(self.matrix_world.inverse());
        let positions = geometry.positions()?;
        let start = geometry.draw_range.start;
        let end = start
            .saturating_add(geometry.draw_range.count.unwrap_or(usize::MAX))
            .min(geometry.draw_count());
        match &self.kind {
            NodeKind::Mesh(mesh) => {
                let groups = if mesh.materials.len() > 1 {
                    geometry.groups.clone()
                } else {
                    vec![crate::geometry::Group {
                        start: 0,
                        count: geometry.draw_count(),
                        material_index: 0,
                    }]
                };
                for group in groups {
                    let Some(material) = mesh.materials.get(group.material_index) else {
                        continue;
                    };
                    let side = material.properties().side;
                    let first = start.max(group.start);
                    let last = end.min(group.start.saturating_add(group.count));
                    for offset in (first..last.saturating_sub(2)).step_by(3) {
                        let ia = geometry.vertex_index(offset)?;
                        let ib = geometry.vertex_index(offset + 1)?;
                        let ic = geometry.vertex_index(offset + 2)?;
                        let a = positions[ia];
                        let b = positions[ib];
                        let c = positions[ic];
                        let intersection = if side == Side::Back {
                            ray.intersect_triangle(c, b, a, true)
                        } else {
                            ray.intersect_triangle(a, b, c, side != Side::Double)
                        };
                        if let Some(local) = intersection {
                            let point = self.matrix_world.transform_point3(local);
                            let distance = r.ray.origin.distance(point);
                            if !r.in_range(distance) {
                                continue;
                            }
                            let e0 = b - a;
                            let e1 = c - a;
                            let e2 = local - a;
                            let d00 = e0.dot(e0);
                            let d01 = e0.dot(e1);
                            let d11 = e1.dot(e1);
                            let d20 = e2.dot(e0);
                            let d21 = e2.dot(e1);
                            let denom = d00 * d11 - d01 * d01;
                            let v = (d11 * d20 - d01 * d21) / denom;
                            let w = (d00 * d21 - d01 * d20) / denom;
                            let weights = [1.0 - v - w, v, w];
                            let ids = [ia, ib, ic];
                            let uv = if let Some(attr) = geometry.attributes.get("uv") {
                                let mut value = Vector2::ZERO;
                                for j in 0..3 {
                                    value += Vector2::new(
                                        attr.get_component(ids[j], 0)?,
                                        attr.get_component(ids[j], 1)?,
                                    ) * weights[j];
                                }
                                Some(value)
                            } else {
                                None
                            };
                            let normal = if let Some(attr) = geometry.attributes.get("normal") {
                                let mut value = Vector3::ZERO;
                                for j in 0..3 {
                                    value += attr.vector3(ids[j])? * weights[j];
                                }
                                if value.dot(ray.direction) > 0.0 {
                                    value = -value;
                                }
                                Some(value)
                            } else {
                                None
                            };
                            hits.push(Intersection {
                                distance,
                                point,
                                object,
                                face_index: Some(offset / 3),
                                index: None,
                                uv,
                                normal,
                                material_index: group.material_index,
                            });
                        }
                    }
                }
            }
            NodeKind::Points(_) => {
                let threshold = r.params.points_threshold
                    / ((self.scale.x + self.scale.y + self.scale.z) / 3.0);
                for i in start..end {
                    let index = geometry.vertex_index(i)?;
                    let p = positions[index];
                    if ray.distance_squared_to_point(p) < threshold * threshold {
                        let point = self
                            .matrix_world
                            .transform_point3(ray.closest_point_to_point(p));
                        let distance = r.ray.origin.distance(point);
                        if r.in_range(distance) {
                            hits.push(Intersection {
                                distance,
                                point,
                                object,
                                face_index: None,
                                index: Some(index),
                                uv: None,
                                normal: None,
                                material_index: 0,
                            });
                        }
                    }
                }
            }
            NodeKind::Line(line) => {
                let threshold =
                    r.params.line_threshold / ((self.scale.x + self.scale.y + self.scale.z) / 3.0);
                for i in (start..end.saturating_sub(1)).step_by(if line.segments { 2 } else { 1 }) {
                    let a = positions[geometry.vertex_index(i)?];
                    let b = positions[geometry.vertex_index(i + 1)?];
                    let (distance2, on_ray, on_segment) = ray_segment_distance(ray, a, b);
                    if distance2 > threshold * threshold {
                        continue;
                    }
                    let distance = r
                        .ray
                        .origin
                        .distance(self.matrix_world.transform_point3(on_ray));
                    if r.in_range(distance) {
                        hits.push(Intersection {
                            distance,
                            point: self.matrix_world.transform_point3(on_segment),
                            object,
                            face_index: None,
                            index: Some(i),
                            uv: None,
                            normal: None,
                            material_index: 0,
                        });
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
fn ray_segment_distance(ray: Ray, a: Vector3, b: Vector3) -> (f64, Vector3, Vector3) {
    let v = b - a;
    let w = ray.origin - a;
    let vv = v.length_squared();
    let dv = ray.direction.dot(v);
    let dw = ray.direction.dot(w);
    let vw = v.dot(w);
    let denom = vv - dv * dv;
    let mut candidates = vec![
        (
            0.0,
            if vv > 0.0 {
                (vw / vv).clamp(0.0, 1.0)
            } else {
                0.0
            },
        ),
        ((-dw).max(0.0), 0.0),
        ((dv - dw).max(0.0), 1.0),
    ];
    if denom > 0.0 {
        let t = (dv * vw - vv * dw) / denom;
        let s = (vw - dv * dw) / denom;
        if t >= 0.0 && (0.0..=1.0).contains(&s) {
            candidates.push((t, s));
        }
    }
    candidates
        .into_iter()
        .map(|(t, s)| {
            let p = ray.at(t);
            let q = a + v * s;
            (p.distance_squared(q), p, q)
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .expect("three boundary candidates")
}
