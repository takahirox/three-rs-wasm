//! physics_ammo_break: two towers, a bridge, stones and a mountain break
//! into convex debris where thrown balls ( clicks ) hit them hard enough.
//! Physics runs in the page's ammo.js ( `super::ammo::AmmoSoftWorld` as a
//! rigid-only btDiscreteDynamicsWorld ); each frame steps the world, copies
//! the motion states onto the meshes and scans the contact manifolds. A
//! breakable object hit with an impulse over 250 is subdivided by
//! ConvexObjectBreaker ( ported below with `super::convex_hull` ) into
//! fragments with convex hull bodies, and the object is removed, as
//! updatePhysics does.
use super::ammo::AmmoSoftWorld;
use super::controls_attributes::{Controls, camera_state};
use super::convex_hull::{convex_geometry, dot, plane_from_coplanar_points};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::rapier_common::Random;
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const MARGIN: f64 = 0.05;
const BALL_MASS: f64 = 35.;
const BALL_RADIUS: f64 = 0.4;
const FRACTURE_IMPULSE: f64 = 250.;
/// ConvexObjectBreaker's minSizeForBreak and smallDelta.
const MIN_SIZE_FOR_BREAK: f64 = 1.4;
const SMALL_DELTA: f64 = 0.0001;

/// A breakable object: its mesh, geometry, material and userData.
#[derive(Clone)]
struct Piece {
    geometry: Arc<BufferGeometry>,
    material: Arc<Material>,
    position: Vector3,
    quaternion: Quaternion,
    mass: f64,
    velocity: Vector3,
    angular_velocity: Vector3,
    breakable: bool,
}

/// A debris object in the scene ( the user pointer's threeObject ).
struct Debris {
    piece: Piece,
    mesh: Object3D,
    body: u32,
    collided: bool,
}

pub(super) struct Demo {
    controls: Controls,
    physics: AmmoSoftWorld,
    /// The dynamic bodies' meshes, in the order of the physics' dynamic
    /// bodies; removed debris keep their place.
    rigid: Vec<Option<Object3D>>,
    debris: Vec<Debris>,
    random: Random,
    ball_geometry: Arc<BufferGeometry>,
    ball_material: Arc<Material>,
    clicks: Vec<Vector2>,
    balls: Vec<Ray>,
    delta: f64,
    now: f64,
    pending: bool,
}

fn phong(color: u32) -> Arc<Material> {
    let mut m = MeshPhongMaterial::default();
    m.properties.color = Color::from_hex(color);
    Arc::new(Material::Phong(m))
}

fn f32_array(g: &BufferGeometry, name: &str) -> Vec<f32> {
    match g.get_attribute(name) {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}

/// Vector3.applyAxisAngle( axis, angle ).
fn apply_axis_angle(v: Vector3, axis: Vector3, angle: f64) -> Vector3 {
    let s = (angle / 2.).sin();
    let (qx, qy, qz, qw) = (axis.x * s, axis.y * s, axis.z * s, (angle / 2.).cos());
    let tx = 2. * (qy * v.z - qz * v.y);
    let ty = 2. * (qz * v.x - qx * v.z);
    let tz = 2. * (qx * v.y - qy * v.x);
    Vector3::new(
        v.x + qw * tx + qy * tz - qz * ty,
        v.y + qw * ty + qz * tx - qx * tz,
        v.z + qw * tz + qx * ty - qy * tx,
    )
}

/// ConvexObjectBreaker.transformPlaneToLocalSpace( plane, object.matrix ).
fn plane_to_local_space(
    (normal, constant): (Vector3, f64),
    position: Vector3,
    q: Quaternion,
) -> (Vector3, f64) {
    // Matrix4.compose( position, quaternion, ( 1, 1, 1 ) ).
    let (x2, y2, z2) = (q.x + q.x, q.y + q.y, q.z + q.z);
    let (xx, xy, xz) = (q.x * x2, q.x * y2, q.x * z2);
    let (yy, yz, zz) = (q.y * y2, q.y * z2, q.z * z2);
    let (wx, wy, wz) = (q.w * x2, q.w * y2, q.w * z2);
    let e = [
        1. - (yy + zz),
        xy + wz,
        xz - wy,
        0.,
        xy - wz,
        1. - (xx + zz),
        yz + wx,
        0.,
        xz + wy,
        yz - wx,
        1. - (xx + yy),
        0.,
        position.x,
        position.y,
        position.z,
        1.,
    ];
    let p = normal * -constant;
    let reference = Vector3::new(
        e[0] * p.x + e[1] * p.y + e[2] * p.z - e[12],
        e[4] * p.x + e[5] * p.y + e[6] * p.z - e[13],
        e[8] * p.x + e[9] * p.y + e[10] * p.z - e[14],
    );
    let n = Vector3::new(
        e[0] * normal.x + e[1] * normal.y + e[2] * normal.z,
        e[4] * normal.x + e[5] * normal.y + e[6] * normal.z,
        e[8] * normal.x + e[9] * normal.y + e[10] * normal.z,
    );
    (n, -dot(reference, n))
}

/// ConvexObjectBreaker.cutByPlane( object, plane ): the two halves, each
/// when it has more than four points.
fn cut_by_plane(object: &Piece, plane: (Vector3, f64)) -> Result<(Option<Piece>, Option<Piece>)> {
    let geometry = &object.geometry;
    let coords = f32_array(geometry, "position");
    let normals = f32_array(geometry, "normal");
    let num_points = coords.len() / 3;
    let indices = geometry.get_index();
    let num_faces = indices.map_or(num_points / 3, |i| i.len() / 3);
    let vertex_index = |face: usize, vert: usize| {
        let idx = face * 3 + vert;
        indices.map_or(idx, |i| i[idx] as usize)
    };
    let mut points1: Vec<Vector3> = vec![];
    let mut points2: Vec<Vector3> = vec![];
    let delta = SMALL_DELTA;
    let mut segments = vec![false; (num_points * num_points).max(30 * 30)];
    // The page's normal reads ( normals[ a ], normals[ a ] + 1, normals[ a ] + 2 ).
    let normal_of = |a: usize| {
        let n = normals[a] as f64;
        Vector3::new(n, n + 1., n + 2.)
    };
    for i in 0..num_faces.saturating_sub(1) {
        let (a1, b1, c1) = (vertex_index(i, 0), vertex_index(i, 1), vertex_index(i, 2));
        let n0 = normal_of(a1);
        for j in i + 1..num_faces {
            let (a2, b2, c2) = (vertex_index(j, 0), vertex_index(j, 1), vertex_index(j, 2));
            let n1 = normal_of(a2);
            let coplanar = 1. - dot(n0, n1) < delta;
            if coplanar {
                if a1 == a2 || a1 == b2 || a1 == c2 {
                    if b1 == a2 || b1 == b2 || b1 == c2 {
                        segments[a1 * num_points + b1] = true;
                        segments[b1 * num_points + a1] = true;
                    } else {
                        segments[c1 * num_points + a1] = true;
                        segments[a1 * num_points + c1] = true;
                    }
                } else if b1 == a2 || b1 == b2 || b1 == c2 {
                    segments[c1 * num_points + b1] = true;
                    segments[b1 * num_points + c1] = true;
                }
            }
        }
    }
    let (local_normal, local_constant) =
        plane_to_local_space(plane, object.position, object.quaternion);
    let distance = |p: Vector3| dot(local_normal, p) + local_constant;
    let point = |i: usize| {
        Vector3::new(
            coords[3 * i] as f64,
            coords[3 * i + 1] as f64,
            coords[3 * i + 2] as f64,
        )
    };
    for i in 0..num_faces {
        let (va, vb, vc) = (vertex_index(i, 0), vertex_index(i, 1), vertex_index(i, 2));
        for segment in 0..3 {
            let (i0, i1) = [(va, vb), (vb, vc), (vc, va)][segment];
            if segments[i0 * num_points + i1] {
                continue;
            }
            segments[i0 * num_points + i1] = true;
            segments[i1 * num_points + i0] = true;
            let (p0, p1) = (point(i0), point(i1));
            let mut marks = [0; 2];
            for (mark, p) in marks.iter_mut().zip([p0, p1]) {
                let d = distance(p);
                *mark = if d > delta {
                    points2.push(p);
                    2
                } else if d < -delta {
                    points1.push(p);
                    1
                } else {
                    points1.push(p);
                    points2.push(p);
                    3
                };
            }
            if (marks[0] == 1 && marks[1] == 2) || (marks[0] == 2 && marks[1] == 1) {
                // Plane.intersectLine( line ), clamped to the segment.
                let direction = p1 - p0;
                let denominator = dot(local_normal, direction);
                let t = -(dot(p0, local_normal) + local_constant) / denominator;
                if denominator == 0. || !(0. ..=1.).contains(&t) {
                    return Ok((None, None));
                }
                let intersection = Vector3::new(
                    p0.x + direction.x * t,
                    p0.y + direction.y * t,
                    p0.z + direction.z * t,
                );
                points1.push(intersection);
                points2.push(intersection);
            }
        }
    }
    let new_mass = object.mass * 0.5;
    let half = |points: &mut Vec<Vector3>| -> Result<Option<Piece>> {
        let mut cm = Vector3::ZERO;
        let mut radius: f64 = 0.;
        if !points.is_empty() {
            for &p in points.iter() {
                cm += p;
            }
            cm /= points.len() as f64;
            for p in points.iter_mut() {
                *p -= cm;
                radius = radius.max(p.x).max(p.y).max(p.z);
            }
            cm += object.position;
        }
        if points.len() <= 4 {
            return Ok(None);
        }
        Ok(Some(Piece {
            geometry: Arc::new(convex_geometry(points)?),
            material: object.material.clone(),
            position: cm,
            quaternion: object.quaternion,
            mass: new_mass,
            velocity: object.velocity,
            angular_velocity: object.angular_velocity,
            breakable: 2. * radius > MIN_SIZE_FOR_BREAK,
        }))
    };
    Ok((half(&mut points1)?, half(&mut points2)?))
}

/// ConvexObjectBreaker.subdivideByImpact( object, pointOfImpact, normal,
/// 1, 2 ) with Math.random in the original's order.
fn subdivide_by_impact(
    random: &mut Random,
    object: &Piece,
    point_of_impact: Vector3,
    normal: Vector3,
) -> Result<Vec<Piece>> {
    let (max_radial_iterations, max_total_iterations) = (1, 3);
    let temp_vector3 = point_of_impact + normal;
    let plane1 = plane_from_coplanar_points(point_of_impact, object.position, temp_vector3);
    let mut debris = vec![];
    let mut stack = vec![(object.clone(), 0., 2. * PI, 0)];
    // subdivideRadial recurses into object1 then object2: a stack visits them in order.
    while let Some((sub, start_angle, end_angle, n)) = stack.pop() {
        if random.next() < n as f64 * 0.05 || n > max_total_iterations {
            debris.push(sub);
            continue;
        }
        let mut angle = PI;
        let plane2 = if n == 0 {
            plane1
        } else if n <= max_radial_iterations {
            angle = (end_angle - start_angle) * (0.2 + 0.6 * random.next()) + start_angle;
            let v2 = apply_axis_angle(object.position - point_of_impact, normal, angle)
                + point_of_impact;
            plane_from_coplanar_points(point_of_impact, temp_vector3, v2)
        } else {
            angle = ((0.5 * (n & 1) as f64) + 0.2 * (2. - random.next())) * PI;
            let v2 = apply_axis_angle(point_of_impact - sub.position, normal, angle) + sub.position;
            let v3 = normal + sub.position;
            plane_from_coplanar_points(sub.position, v3, v2)
        };
        let (obj1, obj2) = cut_by_plane(&sub, plane2)?;
        if let Some(obj2) = obj2 {
            stack.push((obj2, angle, end_angle, n + 1));
        }
        if let Some(obj1) = obj1 {
            stack.push((obj1, start_angle, angle, n + 1));
        }
    }
    Ok(debris)
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
        s.background = Color::from_hex(0xbfd1e5);
        s.get_mut(c)?.position = Vector3::new(-14., 8., 16.);
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
        n.position = Vector3::new(-10., 18., 5.);
        n.cast_shadow = true;
        n.shadow.extent = 14.;
        n.shadow.near = 2.;
        n.shadow.far = 50.;
        n.shadow.map_size = Some(1024);
        // initPhysics().
        let physics = super::ammo::rigid_world(-7.8).await?;
        // createObjects(): the ground with the grid ( no color space ).
        let mut grid =
            decode_texture_image(&fetch("/web/gallery/assets/physics/grid.png").await?).await?;
        grid.srgb = false;
        grid.mipmap_filter = Some(Filter::Linear);
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.repeat = Vector2::new(40., 40.);
        let mut ground_material = MeshPhongMaterial::default();
        ground_material.properties.map = Some(Arc::new(grid));
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(40., 1., 40.)?),
            Arc::new(Material::Phong(ground_material)),
        )));
        let n = s.get_mut(ground)?;
        n.position = Vector3::new(0., -0.5, 0.);
        n.receive_shadow = true;
        physics.break_body(
            0,
            20.,
            0.5,
            20.,
            &[],
            MARGIN,
            0.,
            0.,
            -0.5,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            -1,
        );
        let mut demo = Self {
            controls,
            physics,
            rigid: vec![],
            debris: vec![],
            random: Random(186),
            ball_geometry: Arc::new(SphereGeometry::build(BALL_RADIUS, 14, 10)?),
            ball_material: phong(0x202020),
            clicks: vec![],
            balls: vec![],
            delta: 0.,
            now: 0.,
            pending: false,
        };
        // createObject( mass, halfExtents, pos, quat, material ): towers, a bridge and stones.
        let breakable = |geometry, material, position, mass| Piece {
            geometry: Arc::new(geometry),
            material,
            position,
            quaternion: Quaternion::IDENTITY,
            mass,
            velocity: Vector3::ZERO,
            angular_velocity: Vector3::ZERO,
            breakable: true,
        };
        let tower = Vector3::new(2., 5., 2.);
        for (pos, color) in [
            (Vector3::new(-8., 5., 0.), 0xb03014),
            (Vector3::new(8., 5., 0.), 0xb03214),
        ] {
            let g = BoxGeometry::build(tower.x * 2., tower.y * 2., tower.z * 2.)?;
            demo.create_debris(s, breakable(g, phong(color), pos, 1000.))?;
        }
        // The bridge and the stones.
        let g = BoxGeometry::build(14., 0.4, 3.)?;
        demo.create_debris(
            s,
            breakable(g, phong(0xb3b865), Vector3::new(0., 10.2, 0.), 100.),
        )?;
        let num_stones = 8;
        for i in 0..num_stones {
            let pos = Vector3::new(0., 2., 15. * (0.5 - i as f64 / (num_stones + 1) as f64));
            let g = BoxGeometry::build(2., 4., 0.3)?;
            demo.create_debris(s, breakable(g, phong(0xb0b0b0), pos, 120.))?;
        }
        // The mountain: the convex hull of a square pyramid.
        let h = Vector3::new(4., 5., 4.);
        let points = [
            Vector3::new(h.x, -h.y, h.z),
            Vector3::new(-h.x, -h.y, h.z),
            Vector3::new(h.x, -h.y, -h.z),
            Vector3::new(-h.x, -h.y, -h.z),
            Vector3::new(0., h.y, 0.),
        ];
        demo.create_debris(
            s,
            breakable(
                convex_geometry(&points)?,
                phong(0xb03814),
                Vector3::new(5., h.y * 0.5, -7.),
                860.,
            ),
        )?;
        Ok(demo)
    }

    /// createDebrisFromBreakableObject( object ): the mesh and its convex hull body.
    fn create_debris(&mut self, s: &mut Scene, piece: Piece) -> Result<()> {
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            piece.geometry.clone(),
            piece.material.clone(),
        )));
        let n = s.get_mut(mesh)?;
        n.position = piece.position;
        n.quaternion = piece.quaternion;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let (p, q, v, w) = (
            piece.position,
            piece.quaternion,
            piece.velocity,
            piece.angular_velocity,
        );
        let body = self.physics.break_body(
            2,
            0.,
            0.,
            0.,
            &f32_array(&piece.geometry, "position"),
            MARGIN,
            piece.mass,
            p.x,
            p.y,
            p.z,
            q.x,
            q.y,
            q.z,
            q.w,
            v.x,
            v.y,
            v.z,
            w.x,
            w.y,
            w.z,
            self.debris.len() as i32,
        );
        if piece.mass > 0. {
            self.rigid.push(Some(mesh));
        }
        self.debris.push(Debris {
            piece,
            mesh,
            body,
            collided: false,
        });
        Ok(())
    }

    /// The window's pointerdown: a ball one unit along the pointer's ray,
    /// thrown at 24, with the camera of that moment.
    fn resolve_clicks(&mut self, s: &Scene, c: Object3D) -> Result<()> {
        if self.clicks.is_empty() {
            return Ok(());
        }
        let (camera, world) = s.camera(c)?;
        for ndc in std::mem::take(&mut self.clicks) {
            self.balls.push(camera.ray(ndc, world)?);
        }
        Ok(())
    }
    fn throw_balls(&mut self, s: &mut Scene) -> Result<()> {
        for ray in std::mem::take(&mut self.balls) {
            let pos = ray.origin + ray.direction;
            let ball = s.insert(NodeKind::Mesh(Mesh::new(
                self.ball_geometry.clone(),
                self.ball_material.clone(),
            )));
            let n = s.get_mut(ball)?;
            n.position = pos;
            n.cast_shadow = true;
            n.receive_shadow = true;
            let body = self.physics.break_body(
                1,
                BALL_RADIUS,
                0.,
                0.,
                &[],
                MARGIN,
                BALL_MASS,
                pos.x,
                pos.y,
                pos.z,
                0.,
                0.,
                0.,
                1.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                -1,
            );
            let v = ray.direction * 24.;
            self.physics.set_linear_velocity(body, v.x, v.y, v.z);
            self.rigid.push(Some(ball));
        }
        Ok(())
    }

    /// updatePhysics( deltaTime ).
    fn update_physics(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        self.physics.step_simulation(delta, 10);
        let poses = self.physics.poses().to_vec();
        for (mesh, p) in self.rigid.iter().zip(poses.as_chunks::<7>().0) {
            let Some(mesh) = *mesh else { continue };
            let n = s.get_mut(mesh)?;
            n.position = Vector3::new(p[0], p[1], p[2]);
            n.quaternion = Quaternion::from_xyzw(p[3], p[4], p[5], p[6]);
        }
        for d in &mut self.debris {
            d.collided = false;
            if let Ok(n) = s.get(d.mesh) {
                d.piece.position = n.position;
                d.piece.quaternion = n.quaternion;
            }
        }
        let contacts = self.physics.contacts().to_vec();
        let mut remove = vec![];
        for m in contacts.as_chunks::<21>().0 {
            let tags = [m[0], m[1]].map(|t| (t >= 0.).then_some(t as usize));
            let breakable = tags.map(|t| t.is_some_and(|t| self.debris[t].piece.breakable));
            let collided = tags.map(|t| t.is_some_and(|t| self.debris[t].collided));
            if (!breakable[0] && !breakable[1]) || (collided[0] && collided[1]) {
                continue;
            }
            let max_impulse = m[2];
            let impact_point = Vector3::new(m[3], m[4], m[5]);
            let impact_normal = Vector3::new(m[6], m[7], m[8]);
            for k in 0..2 {
                let Some(tag) = tags[k] else { continue };
                if breakable[k] && !self.debris[tag].collided && max_impulse > FRACTURE_IMPULSE {
                    let o = 9 + k * 6;
                    let object = self.debris[tag].piece.clone();
                    let debris = subdivide_by_impact(
                        &mut self.random,
                        &object,
                        impact_point,
                        impact_normal,
                    )?;
                    for mut fragment in debris {
                        fragment.velocity = Vector3::new(m[o], m[o + 1], m[o + 2]);
                        fragment.angular_velocity = Vector3::new(m[o + 3], m[o + 4], m[o + 5]);
                        self.create_debris(s, fragment)?;
                    }
                    remove.push(tag);
                    self.debris[tag].collided = true;
                }
            }
        }
        // removeDebris( object ).
        for tag in remove {
            let d = &self.debris[tag];
            s.dispose(d.mesh)?;
            self.physics.remove_body(d.body);
            for r in &mut self.rigid {
                if *r == Some(d.mesh) {
                    *r = None;
                }
            }
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
    /// animate(): the balls thrown since the last frame, then render().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.resolve_clicks(s, c)?;
        self.throw_balls(s)?;
        if std::mem::take(&mut self.pending) {
            let delta = std::mem::take(&mut self.delta);
            self.update_physics(s, delta)?;
        }
        Ok(())
    }
    /// The window's pointerdown ( kind 1 ) at window-relative NDC.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind == 1 {
            self.clicks.push(Vector2::new(x, y));
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
        // A pointerdown's ray uses the camera before the drag moves it.
        self.resolve_clicks(s, c)?;
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
