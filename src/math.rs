//! Column-major, right-handed math. Three.js names map to glam value types.
pub use glam::{
    DMat3 as Matrix3, DMat4 as Matrix4, DQuat as Quaternion, DVec2 as Vector2, DVec3 as Vector3,
    DVec4 as Vector4,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum EulerOrder {
    #[default]
    XYZ,
    YXZ,
    ZXY,
    ZYX,
    YZX,
    XZY,
}
impl EulerOrder {
    fn axes(self) -> (glam::EulerRot, [usize; 3]) {
        match self {
            Self::XYZ => (glam::EulerRot::XYZ, [0, 1, 2]),
            Self::YXZ => (glam::EulerRot::YXZ, [1, 0, 2]),
            Self::ZXY => (glam::EulerRot::ZXY, [2, 0, 1]),
            Self::ZYX => (glam::EulerRot::ZYX, [2, 1, 0]),
            Self::YZX => (glam::EulerRot::YZX, [1, 2, 0]),
            Self::XZY => (glam::EulerRot::XZY, [0, 2, 1]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Euler {
    pub angles: Vector3,
    pub order: EulerOrder,
}

impl Default for Euler {
    fn default() -> Self {
        Self {
            angles: Vector3::ZERO,
            order: EulerOrder::XYZ,
        }
    }
}

impl Euler {
    pub fn quaternion(self) -> Quaternion {
        let (order, [a, b, c]) = self.order.axes();
        Quaternion::from_euler(order, self.angles[a], self.angles[b], self.angles[c])
    }
    pub fn from_quaternion(q: Quaternion, order: EulerOrder) -> Self {
        let (glam_order, [a, b, c]) = order.axes();
        let (x, y, z) = q.to_euler(glam_order);
        let mut angles = Vector3::ZERO;
        angles[a] = x;
        angles[b] = y;
        angles[c] = z;
        Self { angles, order }
    }
}

/// Linear-sRGB color. Hex input is decoded from sRGB like Three.js Color.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color(pub Vector3);

impl Default for Color {
    fn default() -> Self {
        Self(Vector3::ONE)
    }
}

impl Color {
    pub const BLACK: Self = Self(Vector3::ZERO);
    pub const WHITE: Self = Self(Vector3::ONE);
    pub fn linear(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3::new(r, g, b))
    }
    pub fn from_hex(hex: u32) -> Self {
        Self(
            Vector3::new(
                ((hex >> 16) & 255) as f64,
                ((hex >> 8) & 255) as f64,
                (hex & 255) as f64,
            )
            .map(|v| srgb_to_linear(v / 255.0)),
        )
    }
    pub fn from_srgb(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3::new(r, g, b).map(srgb_to_linear))
    }
    /// HSL in the working linear color space, as with Three's default setHSL.
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let h = h.rem_euclid(1.0);
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);
        if s == 0.0 {
            return Self(Vector3::splat(l));
        }
        let p = if l <= 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let q = 2.0 * l - p;
        let hue = |t: f64| {
            let t = t.rem_euclid(1.0);
            if t < 1.0 / 6.0 {
                q + (p - q) * 6.0 * t
            } else if t < 0.5 {
                p
            } else if t < 2.0 / 3.0 {
                q + (p - q) * 6.0 * (2.0 / 3.0 - t)
            } else {
                q
            }
        };
        Self::linear(hue(h + 1.0 / 3.0), hue(h), hue(h - 1.0 / 3.0))
    }
    pub fn to_hex(self) -> u32 {
        let c = self
            .0
            .map(|v| (linear_to_srgb(v).clamp(0.0, 1.0) * 255.0).round() as u32 as f64);
        ((c.x as u32) << 16) | ((c.y as u32) << 8) | c.z as u32
    }
}

pub fn srgb_to_linear(v: f64) -> f64 {
    if v < 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}
pub fn linear_to_srgb(v: f64) -> f64 {
    if v < 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Box2 {
    pub min: Vector2,
    pub max: Vector2,
}
impl Default for Box2 {
    fn default() -> Self {
        Self {
            min: Vector2::splat(f64::INFINITY),
            max: Vector2::splat(f64::NEG_INFINITY),
        }
    }
}
impl Box2 {
    pub fn expand_by_point(&mut self, p: Vector2) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }
    pub fn contains_point(self, p: Vector2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Box3 {
    pub min: Vector3,
    pub max: Vector3,
}
impl Default for Box3 {
    fn default() -> Self {
        Self {
            min: Vector3::splat(f64::INFINITY),
            max: Vector3::splat(f64::NEG_INFINITY),
        }
    }
}
impl Box3 {
    pub fn from_points(points: impl IntoIterator<Item = Vector3>) -> Self {
        let mut b = Self::default();
        for p in points {
            b.expand_by_point(p);
        }
        b
    }
    pub fn is_empty(self) -> bool {
        self.max.cmplt(self.min).any()
    }
    pub fn center(self) -> Vector3 {
        if self.is_empty() {
            Vector3::ZERO
        } else {
            (self.min + self.max) * 0.5
        }
    }
    pub fn size(self) -> Vector3 {
        if self.is_empty() {
            Vector3::ZERO
        } else {
            self.max - self.min
        }
    }
    pub fn expand_by_point(&mut self, p: Vector3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }
    pub fn expand_by_scalar(&mut self, n: f64) {
        self.min -= Vector3::splat(n);
        self.max += Vector3::splat(n);
    }
    pub fn union(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
    pub fn contains_point(self, p: Vector3) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }
    pub fn intersects_box(self, b: Self) -> bool {
        self.max.cmpge(b.min).all() && self.min.cmple(b.max).all()
    }
    pub fn distance_to_point(self, p: Vector3) -> f64 {
        p.distance(p.clamp(self.min, self.max))
    }
    pub fn transformed(self, m: Matrix4) -> Self {
        if self.is_empty() {
            return self;
        }
        Self::from_points((0..8).map(|i| {
            m.transform_point3(Vector3::new(
                if i & 1 == 0 { self.min.x } else { self.max.x },
                if i & 2 == 0 { self.min.y } else { self.max.y },
                if i & 4 == 0 { self.min.z } else { self.max.z },
            ))
        }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}
impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Vector3::ZERO,
            radius: -1.0,
        }
    }
}
impl Sphere {
    pub fn from_points(points: &[Vector3]) -> Self {
        let center = Box3::from_points(points.iter().copied()).center();
        let radius = points
            .iter()
            .map(|p| p.distance_squared(center))
            .fold(0.0, f64::max)
            .sqrt();
        Self { center, radius }
    }
    pub fn contains_point(self, p: Vector3) -> bool {
        self.radius >= 0.0 && p.distance_squared(self.center) <= self.radius * self.radius
    }
    pub fn transformed(self, m: Matrix4) -> Self {
        let scale = m
            .x_axis
            .truncate()
            .length_squared()
            .max(m.y_axis.truncate().length_squared())
            .max(m.z_axis.truncate().length_squared())
            .sqrt();
        Self {
            center: m.transform_point3(self.center),
            radius: self.radius * scale,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vector3,
    pub constant: f64,
}
impl Plane {
    pub fn normalize(self) -> Self {
        let length = self.normal.length();
        Self {
            normal: self.normal / length,
            constant: self.constant / length,
        }
    }
    pub fn distance_to_point(self, p: Vector3) -> f64 {
        self.normal.dot(p) + self.constant
    }
    pub fn from_normal_point(normal: Vector3, point: Vector3) -> Self {
        Self {
            normal,
            constant: -normal.dot(point),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}
impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vector3::ZERO,
            direction: Vector3::NEG_Z,
        }
    }
}
impl Ray {
    pub fn at(self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
    pub fn transformed(self, m: Matrix4) -> Self {
        Self {
            origin: m.transform_point3(self.origin),
            direction: m.transform_vector3(self.direction).normalize(),
        }
    }
    pub fn closest_point_to_point(self, p: Vector3) -> Vector3 {
        self.at((p - self.origin).dot(self.direction).max(0.0))
    }
    pub fn distance_squared_to_point(self, p: Vector3) -> f64 {
        p.distance_squared(self.closest_point_to_point(p))
    }
    pub fn intersect_plane(self, p: Plane) -> Option<Vector3> {
        let d = p.normal.dot(self.direction);
        let dist = p.distance_to_point(self.origin);
        if d == 0.0 {
            return if dist == 0.0 { Some(self.origin) } else { None };
        }
        let t = -dist / d;
        (t >= 0.0).then(|| self.at(t))
    }
    pub fn intersect_sphere(self, s: Sphere) -> Option<Vector3> {
        if s.radius < 0.0 {
            return None;
        }
        let offset = s.center - self.origin;
        let tca = offset.dot(self.direction);
        let d2 = offset.length_squared() - tca * tca;
        if d2 > s.radius * s.radius {
            return None;
        }
        let thc = (s.radius * s.radius - d2).sqrt();
        let t = if tca - thc >= 0.0 {
            tca - thc
        } else {
            tca + thc
        };
        (t >= 0.0).then(|| self.at(t))
    }
    pub fn intersect_box(self, b: Box3) -> Option<Vector3> {
        if b.is_empty() {
            return None;
        }
        let mut near: f64 = f64::NEG_INFINITY;
        let mut far: f64 = f64::INFINITY;
        for axis in 0..3 {
            let o = self.origin[axis];
            let d = self.direction[axis];
            if d == 0.0 {
                if o < b.min[axis] || o > b.max[axis] {
                    return None;
                }
                continue;
            }
            let t1 = (b.min[axis] - o) / d;
            let t2 = (b.max[axis] - o) / d;
            near = near.max(t1.min(t2));
            far = far.min(t1.max(t2));
            if near > far {
                return None;
            }
        }
        (far >= 0.0).then(|| self.at(if near >= 0.0 { near } else { far }))
    }
    /// Möller–Trumbore triangle intersection, with Three.js front-face semantics.
    pub fn intersect_triangle(
        self,
        a: Vector3,
        b: Vector3,
        c: Vector3,
        cull_back: bool,
    ) -> Option<Vector3> {
        let e1 = b - a;
        let e2 = c - a;
        let p = self.direction.cross(e2);
        let determinant = e1.dot(p);
        if determinant == 0.0 || (cull_back && determinant < 0.0) {
            return None;
        }
        let inverse = 1.0 / determinant;
        let t = self.origin - a;
        let u = t.dot(p) * inverse;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = t.cross(e1);
        let v = self.direction.dot(q) * inverse;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let distance = e2.dot(q) * inverse;
        (distance >= 0.0).then(|| self.at(distance))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Frustum {
    pub planes: [Plane; 6],
}
impl Frustum {
    /// WebGPU clip volume: -w <= x,y <= w, 0 <= z <= w.
    pub fn from_projection(m: Matrix4) -> Self {
        let r0 = m.row(0);
        let r1 = m.row(1);
        let r2 = m.row(2);
        let r3 = m.row(3);
        Self {
            planes: [r3 + r0, r3 - r0, r3 + r1, r3 - r1, r2, r3 - r2].map(|v| {
                Plane {
                    normal: v.truncate(),
                    constant: v.w,
                }
                .normalize()
            }),
        }
    }
    pub fn intersects_sphere(&self, s: Sphere) -> bool {
        self.planes
            .iter()
            .all(|p| p.distance_to_point(s.center) >= -s.radius)
    }
    pub fn intersects_box(&self, b: Box3) -> bool {
        !b.is_empty()
            && self.planes.iter().all(|p| {
                p.distance_to_point(Vector3::new(
                    if p.normal.x > 0.0 { b.max.x } else { b.min.x },
                    if p.normal.y > 0.0 { b.max.y } else { b.min.y },
                    if p.normal.z > 0.0 { b.max.z } else { b.min.z },
                )) >= 0.0
            })
    }
}
