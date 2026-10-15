//! games_fps: the Octree collision demo. The collision world glTF is split
//! into the Octree addon's tree ( eight triangles per leaf, sixteen levels,
//! the triangles popped into the children in reverse ), and each animation
//! frame runs five substeps of the page's player capsule and sphere physics
//! on the CPU, as the page does: WASD and jump controls with floor and wall
//! responses from the triangle–capsule tests, a hundred thrown spheres with
//! the triangle–sphere tests, sphere–sphere and sphere–player exchanges,
//! damping, gravity and the out-of-bounds teleport. The camera follows the
//! capsule and turns by the pointer while it is locked; releasing the button
//! throws the next sphere with the hold time's impulse. The scene renders
//! with VSM shadows from the directional light, the hemisphere fill, linear
//! fog, ACES tone mapping and the hidden OctreeHelper the debug toggle shows.
use super::gltf_viewer::load_asset;
use crate::attribute::BufferAttribute;
use crate::shadow::ShadowFilter;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

const GRAVITY: f64 = 30.;
const NUM_SPHERES: usize = 100;
const SPHERE_RADIUS: f64 = 0.2;
const STEPS_PER_FRAME: usize = 5;
const EPS: f64 = 1e-10;

type V = Vector3;

#[derive(Clone, Copy)]
struct Box3 {
    min: V,
    max: V,
}
impl Box3 {
    fn empty() -> Self {
        Self {
            min: V::splat(f64::INFINITY),
            max: V::splat(f64::NEG_INFINITY),
        }
    }
    /// Box3.intersectsTriangle: the separating axis test.
    fn intersects_triangle(&self, t: &[V; 3]) -> bool {
        if self.max.x < self.min.x || self.max.y < self.min.y || self.max.z < self.min.z {
            return false;
        }
        let center = (self.min + self.max) * 0.5;
        let extents = self.max - center;
        let (v0, v1, v2) = (t[0] - center, t[1] - center, t[2] - center);
        let (f0, f1, f2) = (v1 - v0, v2 - v1, v0 - v2);
        let sat = |axes: &[V]| {
            axes.iter().all(|a| {
                let r = extents.x * a.x.abs() + extents.y * a.y.abs() + extents.z * a.z.abs();
                let (p0, p1, p2) = (v0.dot(*a), v1.dot(*a), v2.dot(*a));
                // JavaScript's !( x > r ): a NaN separation does not separate.
                let x = (-p0.max(p1).max(p2)).max(p0.min(p1).min(p2));
                x <= r || x.is_nan()
            })
        };
        let axes = [
            V::new(0., -f0.z, f0.y),
            V::new(0., -f1.z, f1.y),
            V::new(0., -f2.z, f2.y),
            V::new(f0.z, 0., -f0.x),
            V::new(f1.z, 0., -f1.x),
            V::new(f2.z, 0., -f2.x),
            V::new(-f0.y, f0.x, 0.),
            V::new(-f1.y, f1.x, 0.),
            V::new(-f2.y, f2.x, 0.),
        ];
        sat(&axes) && sat(&[V::X, V::Y, V::Z]) && sat(&[f0.cross(f1)])
    }
}
#[derive(Clone, Copy)]
struct Capsule {
    start: V,
    end: V,
    radius: f64,
}
impl Capsule {
    fn translate(&mut self, v: V) {
        self.start += v;
        self.end += v;
    }
    fn center(&self) -> V {
        (self.end + self.start) * 0.5
    }
    fn intersects_box(&self, b: &Box3) -> bool {
        let check =
            |p1x: f64, p1y: f64, p2x: f64, p2y: f64, minx: f64, maxx: f64, miny: f64, maxy: f64| {
                (minx - p1x < self.radius || minx - p2x < self.radius)
                    && (p1x - maxx < self.radius || p2x - maxx < self.radius)
                    && (miny - p1y < self.radius || miny - p2y < self.radius)
                    && (p1y - maxy < self.radius || p2y - maxy < self.radius)
            };
        let (s, e) = (self.start, self.end);
        check(s.x, s.y, e.x, e.y, b.min.x, b.max.x, b.min.y, b.max.y)
            && check(s.x, s.z, e.x, e.z, b.min.x, b.max.x, b.min.z, b.max.z)
            && check(s.y, s.z, e.y, e.z, b.min.y, b.max.y, b.min.z, b.max.z)
    }
}
/// Vector3.normalize(): by the length, or by 1 when zero.
fn normalize(v: V) -> V {
    let l = v.length();
    if l == 0. { v } else { v / l }
}
/// Triangle.getPlane: ( normal, constant ).
fn plane(t: &[V; 3]) -> (V, f64) {
    let n = normalize((t[2] - t[1]).cross(t[0] - t[1]));
    (n, -t[0].dot(n))
}
/// Triangle.containsPoint through getBarycoord.
fn contains(t: &[V; 3], p: V) -> bool {
    let (v0, v1, v2) = (t[2] - t[0], t[1] - t[0], p - t[0]);
    let (d00, d01, d02, d11, d12) = (v0.dot(v0), v0.dot(v1), v0.dot(v2), v1.dot(v1), v1.dot(v2));
    let denom = d00 * d11 - d01 * d01;
    if denom == 0. {
        return false;
    }
    let inv = 1. / denom;
    let u = (d11 * d02 - d01 * d12) * inv;
    let v = (d00 * d12 - d01 * d02) * inv;
    let (x, y) = (1. - u - v, v);
    x >= 0. && y >= 0. && x + y <= 1.
}
fn line_to_line_closest(a0: V, a1: V, b0: V, b1: V) -> (V, V) {
    let r = a1 - a0;
    let s = b1 - b0;
    let w = b0 - a0;
    let (a, b, c, d, e) = (r.dot(s), r.dot(r), s.dot(s), s.dot(w), r.dot(w));
    let divisor = b * c - a * a;
    let (mut t1, mut t2);
    if divisor.abs() < EPS {
        let d1 = -d / c;
        let d2 = (a - d) / c;
        if (d1 - 0.5).abs() < (d2 - 0.5).abs() {
            t1 = 0.;
            t2 = d1;
        } else {
            t1 = 1.;
            t2 = d2;
        }
    } else {
        t1 = (d * a + e * c) / divisor;
        t2 = (t1 * a - d) / c;
    }
    t2 = 0f64.max(1f64.min(t2));
    t1 = 0f64.max(1f64.min(t1));
    (r * t1 + a0, s * t2 + b0)
}

struct Octree {
    box3: Box3,
    sub_trees: Vec<Octree>,
    triangles: Vec<usize>,
}
impl Octree {
    fn split(&mut self, level: usize, all: &[[V; 3]]) {
        let half = (self.box3.max - self.box3.min) * 0.5;
        let mut sub: Vec<Octree> = vec![];
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let min =
                        self.box3.min + V::new(f64::from(x), f64::from(y), f64::from(z)) * half;
                    sub.push(Octree {
                        box3: Box3 {
                            min,
                            max: min + half,
                        },
                        sub_trees: vec![],
                        triangles: vec![],
                    });
                }
            }
        }
        while let Some(t) = self.triangles.pop() {
            for s in &mut sub {
                if s.box3.intersects_triangle(&all[t]) {
                    s.triangles.push(t);
                }
            }
        }
        for mut s in sub {
            let len = s.triangles.len();
            if len > 8 && level < 16 {
                s.split(level + 1, all);
            }
            if len != 0 {
                self.sub_trees.push(s);
            }
        }
    }
    fn collect(&self, test: &dyn Fn(&Box3) -> bool, out: &mut Vec<usize>) {
        for s in &self.sub_trees {
            if !test(&s.box3) {
                continue;
            }
            if s.triangles.is_empty() {
                s.collect(test, out);
            } else {
                for &t in &s.triangles {
                    if !out.contains(&t) {
                        out.push(t);
                    }
                }
            }
        }
    }
    fn boxes(&self, out: &mut Vec<f32>) {
        for s in &self.sub_trees {
            let (n, x) = (s.box3.min, s.box3.max);
            let corners = [
                [x.x, x.y, x.z, n.x, x.y, x.z],
                [n.x, x.y, x.z, n.x, n.y, x.z],
                [n.x, n.y, x.z, x.x, n.y, x.z],
                [x.x, n.y, x.z, x.x, x.y, x.z],
                [x.x, x.y, n.z, n.x, x.y, n.z],
                [n.x, x.y, n.z, n.x, n.y, n.z],
                [n.x, n.y, n.z, x.x, n.y, n.z],
                [x.x, n.y, n.z, x.x, x.y, n.z],
                [x.x, x.y, x.z, x.x, x.y, n.z],
                [n.x, x.y, x.z, n.x, x.y, n.z],
                [n.x, n.y, x.z, n.x, n.y, n.z],
                [x.x, n.y, x.z, x.x, n.y, n.z],
            ];
            for c in corners {
                out.extend(c.map(|v| v as f32));
            }
            s.boxes(out);
        }
    }
}
struct World {
    triangles: Vec<[V; 3]>,
    root: Octree,
}
impl World {
    fn new(triangles: Vec<[V; 3]>) -> Self {
        let mut bounds = Box3::empty();
        for t in &triangles {
            bounds.min = bounds.min.min(t[0].min(t[1]).min(t[2]));
            bounds.max = bounds.max.max(t[0].max(t[1]).max(t[2]));
        }
        // calcBox(): the bounds, the minimum lowered by 0.01.
        let mut root = Octree {
            box3: Box3 {
                min: bounds.min - V::splat(0.01),
                max: bounds.max,
            },
            sub_trees: vec![],
            triangles: (0..triangles.len()).collect(),
        };
        root.split(0, &triangles);
        Self { triangles, root }
    }
    fn capsule_intersect(&self, capsule: &Capsule) -> Option<(V, f64)> {
        let mut c = *capsule;
        let mut found = vec![];
        self.root.collect(&|b| c.intersects_box(b), &mut found);
        let mut hit = false;
        for t in found {
            let tri = &self.triangles[t];
            if let Some((normal, depth)) = triangle_capsule(&c, tri) {
                hit = true;
                c.translate(normal * depth);
            }
        }
        hit.then(|| {
            let v = c.center() - capsule.center();
            (normalize(v), v.length())
        })
    }
    fn sphere_intersect(&self, center: V, radius: f64) -> Option<(V, f64)> {
        let mut c = center;
        let mut found = vec![];
        // Sphere.intersectsBox: the clamped center within the radius.
        self.root.collect(
            &|b| center.clamp(b.min, b.max).distance_squared(center) <= radius * radius,
            &mut found,
        );
        let mut hit = false;
        for t in found {
            if let Some((normal, depth)) = triangle_sphere(c, radius, &self.triangles[t]) {
                hit = true;
                c += normal * depth;
            }
        }
        hit.then(|| {
            let v = c - center;
            (normalize(v), v.length())
        })
    }
}
fn triangle_capsule(c: &Capsule, t: &[V; 3]) -> Option<(V, f64)> {
    let (n, k) = plane(t);
    let d1 = n.dot(c.start) + k - c.radius;
    let d2 = n.dot(c.end) + k - c.radius;
    if (d1 > 0. && d2 > 0.) || (d1 < -c.radius && d2 < -c.radius) {
        return None;
    }
    let delta = (d1 / (d1.abs() + d2.abs())).abs();
    let point = c.start.lerp(c.end, delta);
    if contains(t, point) {
        return Some((n, d1.min(d2).abs()));
    }
    let r2 = c.radius * c.radius;
    for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
        let (p1, p2) = line_to_line_closest(c.start, c.end, a, b);
        if p1.distance_squared(p2) < r2 {
            return Some((normalize(p1 - p2), c.radius - p1.distance(p2)));
        }
    }
    None
}
fn triangle_sphere(center: V, radius: f64, t: &[V; 3]) -> Option<(V, f64)> {
    let (n, k) = plane(t);
    let distance = n.dot(center) + k;
    if distance.abs() > radius {
        return None;
    }
    let depth = (distance - radius).abs();
    let r2 = radius * radius - depth * depth;
    let plain = center - n * distance;
    if contains(t, center) {
        return Some((n, depth));
    }
    for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
        // Line3.closestPointToPoint( plainPoint, true ).
        let ab = b - a;
        let tt = (ab.dot(plain - a) / ab.dot(ab)).clamp(0., 1.);
        let p = ab * tt + a;
        let d = p.distance_squared(center);
        if d < r2 {
            return Some((normalize(center - p), radius - d.sqrt()));
        }
    }
    None
}

struct Ball {
    mesh: Object3D,
    center: V,
    velocity: V,
}

pub(super) struct Demo {
    world: World,
    helper: Object3D,
    balls: Vec<Ball>,
    ball: usize,
    player: Capsule,
    velocity: V,
    on_floor: bool,
    yaw: f64,
    pitch: f64,
    keys: [bool; 5],
    mouse_time: f64,
    throws: usize,
    /// The example clock, and the clock the last frame stepped to.
    time: f64,
    stepped: f64,
    queue: Vec<(u32, bool)>,
    debug: Option<bool>,
}
fn now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or(0., |p| p.now())
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.background = Color::from_hex(0x88ccee);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x88ccee),
            near: 0.,
            far: 50.,
        });
        s.tone_mapping = ToneMapping::Aces;
        let fill = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x8dc1de),
            ground: Color::from_hex(0x00668d),
            intensity: 1.5,
        }));
        s.get_mut(fill)?.position = V::new(2., 1., 1.);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.5,
            target: V::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = V::new(-5., 25., -1.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            near: 0.01,
            far: 500.,
            extent: 30.,
            map_size: Some(1024),
            radius: 4.,
            bias: -0.00006,
            filter: ShadowFilter::Vsm,
            ..Default::default()
        };
        let geometry = Arc::new(IcosahedronGeometry::build(SPHERE_RADIUS, 5)?);
        let mut lambert = MeshLambertMaterial::default();
        lambert.properties.color = Color::from_hex(0xdede8d);
        let material = Arc::new(Material::Lambert(lambert));
        let mut balls = vec![];
        for _ in 0..NUM_SPHERES {
            let h = s.insert(NodeKind::Mesh(Mesh {
                geometry: geometry.clone(),
                materials: vec![material.clone()],
            }));
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            balls.push(Ball {
                mesh: h,
                center: V::new(0., -100., 0.),
                velocity: V::ZERO,
            });
        }
        // collision-world.glb: drawn, and its triangles in the octree.
        let (a, b, i) = load_asset("/web/gallery/assets/gltf/collision-world.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        s.update()?;
        let mut triangles = vec![];
        for &h in &instance.meshes {
            let node = s.get_mut(h)?;
            node.cast_shadow = true;
            node.receive_shadow = true;
            let world = node.matrix_world;
            let NodeKind::Mesh(mesh) = &mut node.kind else {
                continue;
            };
            for m in &mut mesh.materials {
                if let Some(map) = &m.properties().map {
                    let mut t = (**map).clone();
                    t.anisotropy = 4;
                    Arc::make_mut(m).properties_mut().map = Some(Arc::new(t));
                }
            }
            let g = if mesh.geometry.index.is_some() {
                mesh.geometry.to_non_indexed()?
            } else {
                (*mesh.geometry).clone()
            };
            let Some(Attribute::F32(p)) = g.get_attribute("position") else {
                return Err(Error::Invalid("collision world positions"));
            };
            let p = p.array();
            for t in p.chunks(9) {
                let v = |k: usize| {
                    world.transform_point3(V::new(
                        f64::from(t[k]),
                        f64::from(t[k + 1]),
                        f64::from(t[k + 2]),
                    ))
                };
                triangles.push([v(0), v(3), v(6)]);
            }
        }
        let world = World::new(triangles);
        let mut lines = vec![];
        world.root.boxes(&mut lines);
        let mut helper_geometry = BufferGeometry::default();
        helper_geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(lines, 3, false)?),
        );
        let mut helper_material = LineBasicMaterial::default();
        helper_material.properties.color = Color::from_hex(0xffff00);
        helper_material.properties.tone_mapped = false;
        let helper = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(helper_geometry),
            material: Arc::new(Material::Line(helper_material)),
            segments: true,
        }));
        s.get_mut(helper)?.visible = false;
        let player = Capsule {
            start: V::new(0., 0.35, 0.),
            end: V::new(0., 1., 0.),
            radius: 0.35,
        };
        let demo = Self {
            world,
            helper,
            balls,
            ball: 0,
            player,
            velocity: V::ZERO,
            on_floor: false,
            yaw: 0.,
            pitch: 0.,
            keys: [false; 5],
            mouse_time: 0.,
            throws: 0,
            time: 0.,
            stepped: 0.,
            queue: vec![],
            debug: None,
        };
        s.get_mut(c)?.position = V::ZERO;
        Ok(demo)
    }
    fn quaternion(&self) -> Quaternion {
        Quaternion::from_euler(glam::EulerRot::YXZ, self.yaw, self.pitch, 0.)
    }
    /// camera.getWorldDirection().
    fn direction(&self) -> V {
        normalize(-(self.quaternion() * V::Z))
    }
    fn forward(&self) -> V {
        let mut d = self.direction();
        d.y = 0.;
        normalize(d)
    }
    fn side(&self) -> V {
        self.forward().cross(V::Y)
    }
    fn controls(&mut self, dt: f64) {
        let speed = dt * if self.on_floor { 25. } else { 8. };
        let [w, sk, a, d, space] = self.keys;
        if w {
            self.velocity += self.forward() * speed;
        }
        if sk {
            self.velocity += self.forward() * -speed;
        }
        if a {
            self.velocity += self.side() * -speed;
        }
        if d {
            self.velocity += self.side() * speed;
        }
        if self.on_floor && space {
            self.velocity.y = 15.;
        }
    }
    fn update_player(&mut self, dt: f64) {
        let mut damping = (-4. * dt).exp() - 1.;
        if !self.on_floor {
            self.velocity.y -= GRAVITY * dt;
            damping *= 0.1;
        }
        self.velocity += self.velocity * damping;
        let delta = self.velocity * dt;
        self.player.translate(delta);
        // playerCollisions().
        self.on_floor = false;
        if let Some((normal, depth)) = self.world.capsule_intersect(&self.player) {
            self.on_floor = normal.y >= 0.15;
            if !self.on_floor {
                self.velocity += normal * -normal.dot(self.velocity);
            }
            if depth >= 1e-10 {
                self.player.translate(normal * depth);
            }
        }
    }
    fn update_spheres(&mut self, dt: f64) {
        for i in 0..self.balls.len() {
            let b = &mut self.balls[i];
            b.center += b.velocity * dt;
            if let Some((normal, depth)) = self.world.sphere_intersect(b.center, SPHERE_RADIUS) {
                b.velocity += normal * (-normal.dot(b.velocity) * 1.5);
                b.center += normal * depth;
            } else {
                b.velocity.y -= GRAVITY * dt;
            }
            let damping = (-1.5 * dt).exp() - 1.;
            b.velocity += b.velocity * damping;
            // playerSphereCollision(): the centre shares vector1 with the normal,
            // so after a hit at start or end the third point is that normal.
            let mut vector1 = (self.player.start + self.player.end) * 0.5;
            let r = self.player.radius + SPHERE_RADIUS;
            let r2 = r * r;
            for k in 0..3 {
                let point = [self.player.start, self.player.end, vector1][k];
                let d2 = point.distance_squared(b.center);
                if d2 < r2 {
                    let normal = normalize(point - b.center);
                    vector1 = normal;
                    let v1 = normal * normal.dot(self.velocity);
                    let v2 = normal * normal.dot(b.velocity);
                    self.velocity = self.velocity + v2 - v1;
                    b.velocity = b.velocity + v1 - v2;
                    let d = (r - d2.sqrt()) / 2.;
                    b.center += normal * -d;
                }
            }
        }
        // spheresCollisions().
        for i in 0..self.balls.len() {
            for j in i + 1..self.balls.len() {
                let (c1, c2) = (self.balls[i].center, self.balls[j].center);
                let d2 = c1.distance_squared(c2);
                let r = SPHERE_RADIUS * 2.;
                if d2 < r * r {
                    let normal = normalize(c1 - c2);
                    let v1 = normal * normal.dot(self.balls[i].velocity);
                    let v2 = normal * normal.dot(self.balls[j].velocity);
                    self.balls[i].velocity = self.balls[i].velocity + v2 - v1;
                    self.balls[j].velocity = self.balls[j].velocity + v1 - v2;
                    let d = (r - d2.sqrt()) / 2.;
                    self.balls[i].center += normal * d;
                    self.balls[j].center += normal * -d;
                }
            }
        }
    }
    fn throw(&mut self) {
        let dir = self.direction();
        let b = &mut self.balls[self.ball];
        b.center = self.player.end + dir * (self.player.radius * 1.5);
        let impulse = 15. + 30. * (1. - ((self.mouse_time - now()) * 0.001).exp());
        b.velocity = dir * impulse + self.velocity * 2.;
        self.ball = (self.ball + 1) % NUM_SPHERES;
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): five substeps of min( 0.05, delta ) / 5.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for (code, down) in std::mem::take(&mut self.queue) {
            match code {
                87 => self.keys[0] = down,
                83 => self.keys[1] = down,
                65 => self.keys[2] = down,
                68 => self.keys[3] = down,
                32 => self.keys[4] = down,
                _ => {}
            }
        }
        for _ in 0..std::mem::take(&mut self.throws) {
            self.throw();
        }
        // timer.getDelta(): the clock since the last frame.
        // Math.min( 0.05, timer.getDelta() ): a clock moved backward gives a negative step.
        let dt = (self.time - self.stepped).min(0.05) / STEPS_PER_FRAME as f64;
        self.stepped = self.time;
        for _ in 0..STEPS_PER_FRAME {
            self.controls(dt);
            self.update_player(dt);
            self.update_spheres(dt);
            // teleportPlayerIfOob().
            if self.player.end.y <= -25. {
                self.player = Capsule {
                    start: V::new(0., 0.35, 0.),
                    end: V::new(0., 1., 0.),
                    radius: 0.35,
                };
                self.yaw = 0.;
                self.pitch = 0.;
            }
        }
        if let Some(visible) = self.debug.take() {
            s.get_mut(self.helper)?.visible = visible;
        }
        for b in &self.balls {
            s.get_mut(b.mesh)?.position = b.center;
        }
        let q = self.quaternion();
        let n = s.get_mut(c)?;
        n.position = self.player.end;
        n.quaternion = q;
        Ok(())
    }
    /// The page's pointer: 10 the container's mousedown ( locking the
    /// pointer ), 20 the mouseup while locked.
    pub fn draw(&mut self, kind: u32, _x: f64, _y: f64) {
        match kind {
            10 => self.mouse_time = now(),
            20 => self.throws += 1,
            _ => {}
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.queue.push((code, down));
    }
    /// The locked pointer's movement turns the camera.
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        dx: f64,
        dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        self.yaw -= dx / 500.;
        self.pitch -= dy / 500.;
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("games fps parameter"));
        }
        self.debug = Some(value != 0.);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
