//! A port of three-bvh-csg 0.0.18's Evaluator ( the default
//! LegacyTriangleSplitter ) over three-mesh-bvh 0.9.10's MeshBVH, as
//! webgl_geometry_csg uses them: BVHs built with the CENTER strategy, three
//! triangles per leaf and an indirect buffer; bvhcast collecting the
//! intersecting triangle pairs; the split triangles classified by a ray cast
//! from their midpoints, the whole triangles flood-filled over half edges.
//! Floating point follows the libraries: single-precision BVH bounds and
//! output attributes, three.js's vector, triangle, plane and matrix formulas.
use crate::math::Vector3 as V3;
use std::collections::{HashMap, HashSet};

pub(super) const ADDITION: u32 = 0;
pub(super) const SUBTRACTION: u32 = 1;
pub(super) const INTERSECTION: u32 = 3;

fn normalize(v: V3) -> V3 {
    let l = v.length();
    v * (1. / if l == 0. { 1. } else { l })
}
fn angle_to(a: V3, b: V3) -> f64 {
    let d = (a.length_squared() * b.length_squared()).sqrt();
    if d == 0. {
        return std::f64::consts::FRAC_PI_2;
    }
    (a.dot(b) / d).clamp(-1., 1.).acos()
}
/// A column-major three.js Matrix4.
pub(super) type M4 = [f64; 16];
pub(super) const IDENTITY: M4 = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];
pub(super) fn multiply(a: &M4, b: &M4) -> M4 {
    let mut t = [0.; 16];
    for r in 0..4 {
        for c in 0..4 {
            t[c * 4 + r] = a[r] * b[c * 4]
                + a[4 + r] * b[c * 4 + 1]
                + a[8 + r] * b[c * 4 + 2]
                + a[12 + r] * b[c * 4 + 3];
        }
    }
    t
}
pub(super) fn invert(te: &M4) -> M4 {
    let (n11, n21, n31, n41) = (te[0], te[1], te[2], te[3]);
    let (n12, n22, n32, n42) = (te[4], te[5], te[6], te[7]);
    let (n13, n23, n33, n43) = (te[8], te[9], te[10], te[11]);
    let (n14, n24, n34, n44) = (te[12], te[13], te[14], te[15]);
    let t1 = n11 * n22 - n21 * n12;
    let t2 = n11 * n32 - n31 * n12;
    let t3 = n11 * n42 - n41 * n12;
    let t4 = n21 * n32 - n31 * n22;
    let t5 = n21 * n42 - n41 * n22;
    let t6 = n31 * n42 - n41 * n32;
    let t7 = n13 * n24 - n23 * n14;
    let t8 = n13 * n34 - n33 * n14;
    let t9 = n13 * n44 - n43 * n14;
    let t10 = n23 * n34 - n33 * n24;
    let t11 = n23 * n44 - n43 * n24;
    let t12 = n33 * n44 - n43 * n34;
    let det = t1 * t12 - t2 * t11 + t3 * t10 + t4 * t9 - t5 * t8 + t6 * t7;
    if det == 0. {
        return [0.; 16];
    }
    let d = 1. / det;
    [
        (n22 * t12 - n32 * t11 + n42 * t10) * d,
        (n31 * t11 - n21 * t12 - n41 * t10) * d,
        (n24 * t6 - n34 * t5 + n44 * t4) * d,
        (n33 * t5 - n23 * t6 - n43 * t4) * d,
        (n32 * t9 - n12 * t12 - n42 * t8) * d,
        (n11 * t12 - n31 * t9 + n41 * t8) * d,
        (n34 * t3 - n14 * t6 - n44 * t2) * d,
        (n13 * t6 - n33 * t3 + n43 * t2) * d,
        (n12 * t11 - n22 * t9 + n42 * t7) * d,
        (n21 * t9 - n11 * t11 - n41 * t7) * d,
        (n14 * t5 - n24 * t3 + n44 * t1) * d,
        (n23 * t3 - n13 * t5 - n43 * t1) * d,
        (n22 * t8 - n12 * t10 - n32 * t7) * d,
        (n11 * t10 - n21 * t8 + n31 * t7) * d,
        (n24 * t2 - n14 * t4 - n34 * t1) * d,
        (n13 * t4 - n23 * t2 + n33 * t1) * d,
    ]
}
pub(super) fn determinant(te: &M4) -> f64 {
    let (n11, n12, n13, n14) = (te[0], te[4], te[8], te[12]);
    let (n21, n22, n23, n24) = (te[1], te[5], te[9], te[13]);
    let (n31, n32, n33, n34) = (te[2], te[6], te[10], te[14]);
    let (n41, n42, n43, n44) = (te[3], te[7], te[11], te[15]);
    n41 * (n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34
        - n12 * n23 * n34)
        + n42
            * (n11 * n23 * n34 - n11 * n24 * n33 + n14 * n21 * n33 - n13 * n21 * n34
                + n13 * n24 * n31
                - n14 * n23 * n31)
        + n43
            * (n11 * n24 * n32 - n11 * n22 * n34 - n14 * n21 * n32
                + n12 * n21 * n34
                + n14 * n22 * n31
                - n12 * n24 * n31)
        + n44
            * (-n13 * n22 * n31 - n11 * n23 * n32 + n11 * n22 * n33 + n13 * n21 * n32
                - n12 * n21 * n33
                + n12 * n23 * n31)
}
/// Vector3.applyMatrix4.
pub(super) fn apply(m: &M4, v: V3) -> V3 {
    let w = 1. / (m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15]);
    V3::new(
        (m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12]) * w,
        (m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13]) * w,
        (m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14]) * w,
    )
}
/// Vector3.transformDirection.
fn transform_direction(m: &M4, v: V3) -> V3 {
    normalize(V3::new(
        m[0] * v.x + m[4] * v.y + m[8] * v.z,
        m[1] * v.x + m[5] * v.y + m[9] * v.z,
        m[2] * v.x + m[6] * v.y + m[10] * v.z,
    ))
}
/// Matrix3.getNormalMatrix( m ), optionally negated.
fn normal_matrix(m: &M4, negate: bool) -> [f64; 9] {
    // setFromMatrix4 then invert ( Matrix3's formula ) then transpose.
    let te = [m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]];
    let (n11, n21, n31, n12, n22, n32, n13, n23, n33) = (
        te[0], te[1], te[2], te[3], te[4], te[5], te[6], te[7], te[8],
    );
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    let inv = if det == 0. {
        [0.; 9]
    } else {
        let d = 1. / det;
        [
            t11 * d,
            (n31 * n23 - n33 * n21) * d,
            (n32 * n21 - n31 * n22) * d,
            t12 * d,
            (n33 * n11 - n31 * n13) * d,
            (n31 * n12 - n32 * n11) * d,
            t13 * d,
            (n21 * n13 - n23 * n11) * d,
            (n22 * n11 - n21 * n12) * d,
        ]
    };
    let s = if negate { -1. } else { 1. };
    // transpose
    [
        inv[0] * s,
        inv[3] * s,
        inv[6] * s,
        inv[1] * s,
        inv[4] * s,
        inv[7] * s,
        inv[2] * s,
        inv[5] * s,
        inv[8] * s,
    ]
}
/// Vector3.applyNormalMatrix.
fn apply_normal(m: &[f64; 9], v: V3) -> V3 {
    normalize(V3::new(
        m[0] * v.x + m[3] * v.y + m[6] * v.z,
        m[1] * v.x + m[4] * v.y + m[7] * v.z,
        m[2] * v.x + m[5] * v.y + m[8] * v.z,
    ))
}
#[derive(Clone, Copy, Debug)]
pub(super) struct Tri {
    pub a: V3,
    pub b: V3,
    pub c: V3,
}
impl Tri {
    fn normal(&self) -> V3 {
        let t = (self.c - self.b).cross(self.a - self.b);
        let l = t.length_squared();
        if l > 0. {
            t * (1. / l.sqrt())
        } else {
            V3::ZERO
        }
    }
    fn plane(&self) -> Plane {
        let n = normalize((self.c - self.b).cross(self.a - self.b));
        Plane::from_normal_point(n, self.a)
    }
    fn midpoint(&self) -> V3 {
        (self.a + self.b + self.c) * (1. / 3.)
    }
    fn barycoord(&self, p: V3) -> Option<V3> {
        let v0 = self.c - self.a;
        let v1 = self.b - self.a;
        let v2 = p - self.a;
        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);
        let denom = dot00 * dot11 - dot01 * dot01;
        if denom == 0. {
            return None;
        }
        let inv = 1. / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv;
        let v = (dot00 * dot12 - dot01 * dot02) * inv;
        Some(V3::new(1. - u - v, v, u))
    }
    fn barycoord_or_zero(&self, p: V3) -> V3 {
        self.barycoord(p).unwrap_or(V3::ZERO)
    }
    fn contains(&self, p: V3) -> bool {
        self.barycoord(p)
            .is_some_and(|v| v.x >= 0. && v.y >= 0. && v.x + v.y <= 1.)
    }
    fn transformed(&self, m: &M4) -> Tri {
        Tri {
            a: apply(m, self.a),
            b: apply(m, self.b),
            c: apply(m, self.c),
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct Plane {
    n: V3,
    c: f64,
}
impl Plane {
    fn from_normal_point(n: V3, p: V3) -> Self {
        Self { n, c: -p.dot(n) }
    }
    fn distance(&self, p: V3) -> f64 {
        self.n.dot(p) + self.c
    }
    /// Plane.intersectLine.
    fn intersect_line(&self, s: V3, e: V3) -> Option<V3> {
        let d = e - s;
        let denominator = self.n.dot(d);
        if denominator == 0. {
            return (self.distance(s) == 0.).then_some(s);
        }
        let t = -(s.dot(self.n) + self.c) / denominator;
        if !(0. ..=1.).contains(&t) {
            return None;
        }
        Some(s + d * t)
    }
}
/// Line3.closestPointToPoint( point, true ).
fn closest_on_segment(s: V3, e: V3, p: V3) -> V3 {
    let se = e - s;
    let t = (se.dot(p - s) / se.dot(se)).clamp(0., 1.);
    (e - s) * t + s
}
const ZERO_EPSILON: f64 = 1e-15;
fn near_zero(v: f64) -> bool {
    v.abs() < ZERO_EPSILON
}
/// three-mesh-bvh's ExtendedTriangle with its cached separating axes.
struct Ext {
    t: Tri,
    axes: [V3; 4],
    bounds: [(f64, f64); 4],
    plane: Plane,
    point: bool,
    segment: Option<(V3, V3)>,
}
fn sat(axis: V3, points: &[V3; 3]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for p in points {
        let v = axis.dot(*p);
        min = if v < min { v } else { min };
        max = if v > max { v } else { max };
    }
    (min, max)
}
fn separated(a: (f64, f64), b: (f64, f64)) -> bool {
    a.0 > b.1 || b.0 > a.1
}
impl Ext {
    fn new(t: Tri) -> Self {
        let points = [t.a, t.b, t.c];
        let axis0 = t.normal();
        let axis1 = t.a - t.b;
        let axis2 = t.b - t.c;
        let axis3 = t.c - t.a;
        let axes = [axis0, axis1, axis2, axis3];
        let bounds = axes.map(|a| sat(a, &points));
        let (ab, bc, ca) = (axis1.length(), axis2.length(), axis3.length());
        let (mut point, mut segment) = (false, None);
        if ab < ZERO_EPSILON {
            if bc < ZERO_EPSILON || ca < ZERO_EPSILON {
                point = true;
            } else {
                segment = Some((t.a, t.c));
            }
        } else if bc < ZERO_EPSILON {
            if ca < ZERO_EPSILON {
                point = true;
            } else {
                segment = Some((t.b, t.a));
            }
        } else if ca < ZERO_EPSILON {
            segment = Some((t.c, t.b));
        }
        Self {
            t,
            axes,
            bounds,
            plane: Plane::from_normal_point(axis0, t.a),
            point,
            segment,
        }
    }
    fn points(&self) -> [V3; 3] {
        [self.t.a, self.t.b, self.t.c]
    }
    fn coplanar_intersects(&self, other: &Ext, target: &mut (V3, V3)) -> bool {
        let n = if !self.point && self.segment.is_none() {
            self.plane.n
        } else {
            other.plane.n
        };
        let (sp, op) = (self.points(), other.points());
        for i in 1..4 {
            if separated(self.bounds[i], sat(self.axes[i], &op)) {
                return false;
            }
            let d = n.cross(self.axes[i]);
            if separated(sat(d, &sp), sat(d, &op)) {
                return false;
            }
        }
        for i in 1..4 {
            if separated(other.bounds[i], sat(other.axes[i], &sp)) {
                return false;
            }
            let d = n.cross(other.axes[i]);
            if separated(sat(d, &sp), sat(d, &op)) {
                return false;
            }
        }
        *target = (V3::ZERO, V3::ZERO);
        true
    }
    fn triangle_segment(tri: &Ext, degenerate: &Ext, target: &mut (V3, V3)) -> bool {
        let (s, e) = degenerate
            .segment
            .unwrap_or((degenerate.t.a, degenerate.t.a));
        let sd = tri.plane.distance(s);
        let ed = tri.plane.distance(e);
        if near_zero(sd) {
            if near_zero(ed) {
                return tri.coplanar_intersects(degenerate, target);
            }
            *target = (s, s);
            return tri.t.contains(s);
        } else if near_zero(ed) {
            *target = (e, e);
            return tri.t.contains(e);
        }
        match tri.plane.intersect_line(s, e) {
            Some(p) => {
                *target = (p, p);
                tri.t.contains(p)
            }
            None => false,
        }
    }
    fn triangle_point(tri: &Ext, degenerate: &Ext, target: &mut (V3, V3)) -> bool {
        let p = degenerate.t.a;
        if near_zero(tri.plane.distance(p)) && tri.t.contains(p) {
            *target = (p, p);
            return true;
        }
        false
    }
    fn segment_point(seg: &Ext, point: &Ext, target: &mut (V3, V3)) -> bool {
        let (s, e) = seg.segment.unwrap_or((seg.t.a, seg.t.a));
        let p = point.t.a;
        let q = closest_on_segment(s, e, p);
        if p.distance_squared(q) < ZERO_EPSILON * ZERO_EPSILON {
            *target = (p, p);
            return true;
        }
        false
    }
    fn degenerate(&self, other: &Ext, target: &mut (V3, V3)) -> Option<bool> {
        if let Some((s1, e1)) = self.segment {
            if let Some((s2, e2)) = other.segment {
                let d1 = e1 - s1;
                let d2 = e2 - s2;
                let sd = s2 - s1;
                let denom = d1.x * d2.y - d1.y * d2.x;
                if near_zero(denom) {
                    return Some(false);
                }
                let t = (sd.x * d2.y - sd.y * d2.x) / denom;
                let u = -(d1.x * sd.y - d1.y * sd.x) / denom;
                if !(0. ..=1.).contains(&t) || !(0. ..=1.).contains(&u) {
                    return Some(false);
                }
                let z1 = s1.z + d1.z * t;
                let z2 = s2.z + d2.z * u;
                if near_zero(z1 - z2) {
                    let p = s1 + d1 * t;
                    *target = (p, p);
                    return Some(true);
                }
                return Some(false);
            } else if other.point {
                return Some(Self::segment_point(self, other, target));
            }
            return Some(Self::triangle_segment(other, self, target));
        } else if self.point {
            if other.point {
                if other.t.a.distance_squared(self.t.a) < ZERO_EPSILON * ZERO_EPSILON {
                    *target = (self.t.a, self.t.a);
                    return Some(true);
                }
                return Some(false);
            } else if other.segment.is_some() {
                return Some(Self::segment_point(other, self, target));
            }
            return Some(Self::triangle_point(other, self, target));
        } else if other.point {
            return Some(Self::triangle_point(self, other, target));
        } else if other.segment.is_some() {
            return Some(Self::triangle_segment(self, other, target));
        }
        None
    }
    /// ExtendedTriangle.intersectsTriangle ( Moller with SAT for coplanar ).
    fn intersects(&self, other: &Ext, target: &mut (V3, V3)) -> bool {
        if let Some(r) = self.degenerate(other, target) {
            return r;
        }
        let z = |v: f64| if near_zero(v) { 0. } else { v };
        let (a1, b1, c1) = (
            z(other.plane.distance(self.t.a)),
            z(other.plane.distance(self.t.b)),
            z(other.plane.distance(self.t.c)),
        );
        let (a1b1, a1c1) = (a1 * b1, a1 * c1);
        if a1b1 > 0. && a1c1 > 0. {
            return false;
        }
        let (a2, b2, c2) = (
            z(self.plane.distance(other.t.a)),
            z(self.plane.distance(other.t.b)),
            z(self.plane.distance(other.t.c)),
        );
        let (a2b2, a2c2) = (a2 * b2, a2 * c2);
        if a2b2 > 0. && a2c2 > 0. {
            return false;
        }
        let line = self.plane.n.cross(other.plane.n);
        let mut index = 0;
        let mut max = line.x.abs();
        if line.y.abs() > max {
            max = line.y.abs();
            index = 1;
        }
        if line.z.abs() > max {
            index = 2;
        }
        let key = |v: V3| [v.x, v.y, v.z][index];
        let single = |a: V3, b: V3, c: V3, ap: f64, bp: f64, cp: f64, ad: f64, bd: f64, cd: f64| {
            let t = ad / (ad - bd);
            let bx = ap + (bp - ap) * t;
            let s = (b - a) * t + a;
            let t = ad / (ad - cd);
            let by = ap + (cp - ap) * t;
            let e = (c - a) * t + a;
            ((bx, by), (s, e))
        };
        let line_bounds =
            |t: &Tri, ap: f64, bp: f64, cp: f64, ab: f64, ac: f64, ad: f64, bd: f64, cd: f64| {
                if ab > 0. {
                    Some(single(t.c, t.a, t.b, cp, ap, bp, cd, ad, bd))
                } else if ac > 0. {
                    Some(single(t.b, t.a, t.c, bp, ap, cp, bd, ad, cd))
                } else if bd * cd > 0. || ad != 0. {
                    Some(single(t.a, t.b, t.c, ap, bp, cp, ad, bd, cd))
                } else if bd != 0. {
                    Some(single(t.b, t.a, t.c, bp, ap, cp, bd, ad, cd))
                } else if cd != 0. {
                    Some(single(t.c, t.a, t.b, cp, ap, bp, cd, ad, bd))
                } else {
                    None
                }
            };
        let Some((mut bounds1, mut edge1)) = line_bounds(
            &self.t,
            key(self.t.a),
            key(self.t.b),
            key(self.t.c),
            a1b1,
            a1c1,
            a1,
            b1,
            c1,
        ) else {
            return self.coplanar_intersects(other, target);
        };
        let Some((mut bounds2, mut edge2)) = line_bounds(
            &other.t,
            key(other.t.a),
            key(other.t.b),
            key(other.t.c),
            a2b2,
            a2c2,
            a2,
            b2,
            c2,
        ) else {
            return self.coplanar_intersects(other, target);
        };
        if bounds1.1 < bounds1.0 {
            bounds1 = (bounds1.1, bounds1.0);
            edge1 = (edge1.1, edge1.0);
        }
        if bounds2.1 < bounds2.0 {
            bounds2 = (bounds2.1, bounds2.0);
            edge2 = (edge2.1, edge2.0);
        }
        if bounds1.1 < bounds2.0 || bounds2.1 < bounds1.0 {
            return false;
        }
        let start = if bounds2.0 > bounds1.0 {
            edge2.0
        } else {
            edge1.0
        };
        let end = if bounds2.1 < bounds1.1 {
            edge2.1
        } else {
            edge1.1
        };
        *target = (start, end);
        true
    }
}
/// The source geometry of a brush ( Float32 attributes ).
pub(super) struct Source {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub index: Option<Vec<u32>>,
    /// start, count, material index.
    pub groups: Vec<(usize, usize, usize)>,
}
impl Source {
    fn tri_count(&self) -> usize {
        self.index
            .as_ref()
            .map_or(self.positions.len() / 3, |i| i.len())
            / 3
    }
    fn vertex(&self, i: usize) -> usize {
        self.index.as_ref().map_or(i, |x| x[i] as usize)
    }
    fn position(&self, v: usize) -> V3 {
        let p = &self.positions[v * 3..v * 3 + 3];
        V3::new(p[0] as f64, p[1] as f64, p[2] as f64)
    }
    fn triangle(&self, t: usize) -> Tri {
        Tri {
            a: self.position(self.vertex(t * 3)),
            b: self.position(self.vertex(t * 3 + 1)),
            c: self.position(self.vertex(t * 3 + 2)),
        }
    }
}
struct Node {
    /// min x, y, z, max x, y, z ( Float32 ).
    bounds: [f32; 6],
    /// Children ( split axis ), or a leaf's range in the indirect buffer.
    inner: Option<(usize, usize, usize)>,
    offset: usize,
    count: usize,
}
/// MeshBVH( geometry, { maxLeafSize: 3, indirect: true } ).
struct Bvh {
    indirect: Vec<u32>,
    nodes: Vec<Node>,
}
const FLOAT32_EPSILON: f64 = 5.960464477539063e-8;
impl Bvh {
    fn new(g: &Source) -> Self {
        let count = g.tri_count();
        let mut indirect: Vec<u32> = (0..count as u32).collect();
        // computePrimitiveBounds: center and half extent per axis ( Float32 ).
        let mut bounds = vec![0f32; count * 6];
        for (i, &tri) in indirect.iter().enumerate() {
            let t = g.triangle(tri as usize);
            for el in 0..3 {
                let (a, b, c) = (t.a[el], t.b[el], t.c[el]);
                let mut min = a;
                if b < min {
                    min = b;
                }
                if c < min {
                    min = c;
                }
                let mut max = a;
                if b > max {
                    max = b;
                }
                if c > max {
                    max = c;
                }
                let half = (max - min) / 2.;
                bounds[i * 6 + el * 2] = (min + half) as f32;
                bounds[i * 6 + el * 2 + 1] = (half + (min.abs() + half) * FLOAT32_EPSILON) as f32;
            }
        }
        let mut bvh = Self {
            indirect: vec![],
            nodes: vec![],
        };
        let (node_bounds, centroid) = Self::bounds(&bounds, 0, count);
        bvh.split(
            &mut indirect,
            &mut bounds,
            0,
            count,
            node_bounds,
            centroid,
            0,
        );
        bvh.indirect = indirect;
        bvh
    }
    /// getBounds: the node bounds and the centroid bounds, in Float32.
    fn bounds(b: &[f32], offset: usize, count: usize) -> ([f32; 6], [f32; 6]) {
        let mut t = [
            f64::INFINITY,
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        let mut c = t;
        for i in offset..offset + count {
            for d in 0..3 {
                let cx = b[i * 6 + d * 2] as f64;
                let hx = b[i * 6 + d * 2 + 1] as f64;
                let (l, r) = (cx - hx, cx + hx);
                if l < t[d] {
                    t[d] = l;
                }
                if r > t[d + 3] {
                    t[d + 3] = r;
                }
                if cx < c[d] {
                    c[d] = cx;
                }
                if cx > c[d + 3] {
                    c[d + 3] = cx;
                }
            }
        }
        (t.map(|v| v as f32), c.map(|v| v as f32))
    }
    #[allow(clippy::too_many_arguments)]
    fn split(
        &mut self,
        indirect: &mut [u32],
        b: &mut [f32],
        offset: usize,
        count: usize,
        bounds: [f32; 6],
        centroid: [f32; 6],
        depth: usize,
    ) -> usize {
        let node = self.nodes.len();
        self.nodes.push(Node {
            bounds,
            inner: None,
            offset,
            count,
        });
        if count <= 3 || depth >= 40 {
            return node;
        }
        // CENTER: the longest centroid axis, split at its middle.
        let mut axis = None;
        let mut dist = f64::NEG_INFINITY;
        for i in 0..3 {
            let d = centroid[i + 3] as f64 - centroid[i] as f64;
            if d > dist {
                dist = d;
                axis = Some(i);
            }
        }
        let Some(axis) = axis else { return node };
        let pos = (centroid[axis] as f64 + centroid[axis + 3] as f64) / 2.;
        // Hoare partition: centers on the plane go right.
        let (mut left, mut right) = (offset as isize, (offset + count) as isize - 1);
        let split = loop {
            while left <= right && (b[left as usize * 6 + axis * 2] as f64) < pos {
                left += 1;
            }
            while left <= right && (b[right as usize * 6 + axis * 2] as f64) >= pos {
                right -= 1;
            }
            if left < right {
                indirect.swap(left as usize, right as usize);
                for i in 0..6 {
                    b.swap(left as usize * 6 + i, right as usize * 6 + i);
                }
                left += 1;
                right -= 1;
            } else {
                break left as usize;
            }
        };
        if split == offset || split == offset + count {
            return node;
        }
        let lcount = split - offset;
        let (lb, lc) = Self::bounds(b, offset, lcount);
        let l = self.split(indirect, b, offset, lcount, lb, lc, depth + 1);
        let (rb, rc) = Self::bounds(b, split, count - lcount);
        let r = self.split(indirect, b, split, count - lcount, rb, rc, depth + 1);
        self.nodes[node].inner = Some((l, r, axis));
        node
    }
    fn node_box(&self, n: usize) -> Box3 {
        let b = self.nodes[n].bounds.map(f64::from);
        Box3 {
            min: V3::new(b[0], b[1], b[2]),
            max: V3::new(b[3], b[4], b[5]),
        }
    }
    /// raycastFirst( ray, DoubleSide ): the closest hit and its face normal.
    fn raycast_first(&self, g: &Source, origin: V3, dir: V3) -> Option<(f64, V3, V3)> {
        self.raycast_node(0, g, origin, dir)
    }
    fn hits_node(&self, n: usize, o: V3, d: V3) -> bool {
        let b = self.nodes[n].bounds.map(f64::from);
        let (ix, iy, iz) = (1. / d.x, 1. / d.y, 1. / d.z);
        let (mut tmin, mut tmax) = if ix >= 0. {
            ((b[0] - o.x) * ix, (b[3] - o.x) * ix)
        } else {
            ((b[3] - o.x) * ix, (b[0] - o.x) * ix)
        };
        let (tymin, tymax) = if iy >= 0. {
            ((b[1] - o.y) * iy, (b[4] - o.y) * iy)
        } else {
            ((b[4] - o.y) * iy, (b[1] - o.y) * iy)
        };
        if tmin > tymax || tymin > tmax {
            return false;
        }
        if tymin > tmin || tmin.is_nan() {
            tmin = tymin;
        }
        if tymax < tmax || tmax.is_nan() {
            tmax = tymax;
        }
        let (tzmin, tzmax) = if iz >= 0. {
            ((b[2] - o.z) * iz, (b[5] - o.z) * iz)
        } else {
            ((b[5] - o.z) * iz, (b[2] - o.z) * iz)
        };
        if tmin > tzmax || tzmin > tmax {
            return false;
        }
        if tzmin > tmin || tmin.is_nan() {
            tmin = tzmin;
        }
        if tzmax < tmax || tmax.is_nan() {
            tmax = tzmax;
        }
        tmin <= f64::INFINITY && tmax >= 0.
    }
    fn raycast_node(&self, n: usize, g: &Source, o: V3, d: V3) -> Option<(f64, V3, V3)> {
        let node = &self.nodes[n];
        let Some((l, r, axis)) = node.inner else {
            let mut best: Option<(f64, V3, V3)> = None;
            for i in node.offset..node.offset + node.count {
                let t = g.triangle(self.indirect[i] as usize);
                if let Some(p) = ray_triangle(o, d, t.a, t.b, t.c, false) {
                    let distance = o.distance(p);
                    if distance >= 0. && best.is_none_or(|b| distance < b.0) {
                        best = Some((distance, p, t.normal()));
                    }
                }
            }
            return best;
        };
        let forward = d[axis] >= 0.;
        let (c1, c2) = if forward { (l, r) } else { (r, l) };
        let r1 = if self.hits_node(c1, o, d) {
            self.raycast_node(c1, g, o, d)
        } else {
            None
        };
        if let Some(h) = r1 {
            let p = h.1[axis];
            let b2 = self.nodes[c2].bounds;
            let outside = if forward {
                p <= b2[axis] as f64
            } else {
                p >= b2[axis + 3] as f64
            };
            if outside {
                return r1;
            }
        }
        let r2 = if self.hits_node(c2, o, d) {
            self.raycast_node(c2, g, o, d)
        } else {
            None
        };
        match (r1, r2) {
            (Some(a), Some(b)) => Some(if a.0 <= b.0 { a } else { b }),
            (a, b) => a.or(b),
        }
    }
}
/// Ray.intersectTriangle.
fn ray_triangle(o: V3, d: V3, a: V3, b: V3, c: V3, cull: bool) -> Option<V3> {
    let edge1 = b - a;
    let edge2 = c - a;
    let normal = edge1.cross(edge2);
    let mut ddn = d.dot(normal);
    let sign;
    if ddn > 0. {
        if cull {
            return None;
        }
        sign = 1.;
    } else if ddn < 0. {
        sign = -1.;
        ddn = -ddn;
    } else {
        return None;
    }
    let diff = o - a;
    let dqxe2 = sign * d.dot(diff.cross(edge2));
    if dqxe2 < 0. {
        return None;
    }
    let de1xq = sign * d.dot(edge1.cross(diff));
    if de1xq < 0. || dqxe2 + de1xq > ddn {
        return None;
    }
    let qdn = -sign * diff.dot(normal);
    if qdn < 0. {
        return None;
    }
    Some(o + d * (qdn / ddn))
}
#[derive(Clone, Copy)]
struct Box3 {
    min: V3,
    max: V3,
}
impl Box3 {
    fn transformed(&self, m: &M4) -> Box3 {
        let mut lo = V3::splat(f64::INFINITY);
        let mut hi = V3::splat(f64::NEG_INFINITY);
        for k in 0..8 {
            let p = V3::new(
                if k & 4 != 0 { self.max.x } else { self.min.x },
                if k & 2 != 0 { self.max.y } else { self.min.y },
                if k & 1 != 0 { self.max.z } else { self.min.z },
            );
            let q = apply(m, p);
            lo = lo.min(q);
            hi = hi.max(q);
        }
        Box3 { min: lo, max: hi }
    }
    fn intersects(&self, b: &Box3) -> bool {
        !(b.max.x < self.min.x
            || b.min.x > self.max.x
            || b.max.y < self.min.y
            || b.min.y > self.max.y
            || b.max.z < self.min.z
            || b.min.z > self.max.z)
    }
}
/// A brush: its geometry, world matrix and the data prepareGeometry caches.
pub(super) struct Brush {
    pub source: Source,
    pub matrix: M4,
    bvh: Bvh,
    /// HalfEdgeMap.data: the sibling half edge, or -1.
    half_edges: Vec<i32>,
    group_indices: Vec<usize>,
}
const HASH_WIDTH: f64 = 1e-6;
/// A half edge's two hashed vertices.
type EdgeHash = ((i32, i32, i32), (i32, i32, i32));
fn hash_number(v: f64, multiplier: f64, addition: f64) -> i32 {
    let t = (v * multiplier + addition).trunc();
    if !t.is_finite() {
        return 0;
    }
    t.rem_euclid(4294967296.) as u64 as u32 as i32
}
impl Brush {
    pub fn new(source: Source) -> Self {
        let bvh = Bvh::new(&source);
        // HalfEdgeMap.updateFrom with position hashes.
        let multiplier = 10f64.powf(-HASH_WIDTH.log10());
        let addition = HASH_WIDTH * 0.5 * multiplier;
        let tris = source.tri_count();
        let mut data = vec![-1i32; tris * 3];
        let mut map: HashMap<EdgeHash, usize> = HashMap::new();
        let hash = |v: V3| {
            (
                hash_number(v.x, multiplier, addition),
                hash_number(v.y, multiplier, addition),
                hash_number(v.z, multiplier, addition),
            )
        };
        for t in 0..tris {
            let h: Vec<_> = (0..3)
                .map(|e| hash(source.position(source.vertex(t * 3 + e))))
                .collect();
            for e in 0..3 {
                let next = (e + 1) % 3;
                let index = t * 3 + e;
                if let Some(other) = map.remove(&(h[next], h[e])) {
                    data[index] = other as i32;
                    data[other] = index as i32;
                } else {
                    map.insert((h[e], h[next]), index);
                }
            }
        }
        let mut group_indices = vec![0; tris];
        for (i, &(start, count, _)) in source.groups.iter().enumerate() {
            for g in start / 3..(start + count) / 3 {
                if let Some(x) = group_indices.get_mut(g) {
                    *x = i;
                }
            }
        }
        Self {
            source,
            matrix: IDENTITY,
            bvh,
            half_edges: data,
            group_indices,
        }
    }
}
fn is_degenerate(t: &Tri) -> bool {
    let eps = 1e-14;
    let ab = t.b - t.a;
    let ac = t.c - t.a;
    let cb = t.b - t.c;
    let angle1 = angle_to(ab, ac);
    let angle2 = angle_to(ab, cb);
    let angle3 = std::f64::consts::PI - angle1 - angle2;
    angle1.abs() < eps
        || angle2.abs() < eps
        || angle3.abs() < eps
        || t.a.distance_squared(t.b) < eps
        || t.a.distance_squared(t.c) < eps
        || t.b.distance_squared(t.c) < eps
}
/// IntersectionMap: per triangle, its intersecting triangles in discovery order.
#[derive(Default)]
struct Intersections {
    ids: Vec<usize>,
    sets: HashMap<usize, Vec<usize>>,
    coplanar: HashMap<usize, Vec<usize>>,
}
impl Intersections {
    fn add(&mut self, id: usize, other: usize, coplanar: bool) {
        let set = self.sets.entry(id).or_insert_with(|| {
            self.ids.push(id);
            vec![]
        });
        set.push(other);
        if coplanar {
            let c = self.coplanar.entry(id).or_default();
            if !c.contains(&other) {
                c.push(other);
            }
        }
    }
}
const COPLANAR_NORMAL_EPSILON: f64 = 1e-10;
const COPLANAR_DISTANCE_EPSILON: f64 = 1e-10;
const CLIP_EPSILON: f64 = 1e-10;
const PARALLEL_EPSILON: f64 = 1e-15;
fn coplanar(a: &Tri, b: &Tri) -> bool {
    let (na, nb) = (a.normal(), b.normal());
    if (1. - na.dot(nb).abs()).abs() >= COPLANAR_NORMAL_EPSILON {
        return false;
    }
    (na.dot(a.a) - na.dot(b.a)).abs() < COPLANAR_DISTANCE_EPSILON
}
/// clipSegmentToTriangle ( Cyrus-Beck ).
fn clip_segment(s: V3, e: V3, tri: &Tri, normal: V3) -> Option<(V3, V3)> {
    let (mut tmin, mut tmax) = (0f64, 1f64);
    let dir = e - s;
    let verts = [tri.a, tri.b, tri.c];
    for i in 0..3 {
        let (v0, v1) = (verts[i], verts[(i + 1) % 3]);
        let edge_normal = normal.cross(v1 - v0);
        let plane = Plane::from_normal_point(edge_normal, v0);
        let dist = plane.distance(s);
        let denom = plane.n.dot(dir);
        if denom.abs() < PARALLEL_EPSILON {
            if dist < -CLIP_EPSILON {
                return None;
            }
            continue;
        }
        let t = -dist / denom;
        if denom > 0. {
            tmin = tmin.max(t);
        } else {
            tmax = tmax.min(t);
        }
        if tmin > tmax + CLIP_EPSILON {
            return None;
        }
    }
    if tmax - tmin < CLIP_EPSILON {
        return None;
    }
    Some((dir * tmin + s, dir * tmax + s))
}
fn coplanar_edge_count(a: &Tri, b: &Tri) -> usize {
    let (na, nb) = (a.normal(), b.normal());
    let bv = [b.a, b.b, b.c];
    let av = [a.a, a.b, a.c];
    (0..3)
        .filter(|&i| clip_segment(bv[i], bv[(i + 1) % 3], a, na).is_some())
        .count()
        + (0..3)
            .filter(|&i| clip_segment(av[i], av[(i + 1) % 3], b, nb).is_some())
            .count()
}
/// collectIntersectingTriangles: bvhcast of B's BVH into A's frame.
fn collect(a: &Brush, b: &Brush) -> (Intersections, Intersections) {
    let mut ai = Intersections::default();
    let mut bi = Intersections::default();
    let to_local = multiply(&invert(&a.matrix), &b.matrix);
    let inv = invert(&to_local);
    let mut visit = |o1: usize, c1: usize, o2: usize, c2: usize| {
        for i2 in o2..o2 + c2 {
            let tb = b
                .source
                .triangle(b.bvh.indirect[i2] as usize)
                .transformed(&to_local);
            let eb = Ext::new(tb);
            for i1 in o1..o1 + c1 {
                let ta = a.source.triangle(a.bvh.indirect[i1] as usize);
                if is_degenerate(&ta) || is_degenerate(&tb) {
                    continue;
                }
                let count = if coplanar(&ta, &tb) {
                    coplanar_edge_count(&ta, &tb)
                } else {
                    0
                };
                let is_coplanar = count > 2;
                let mut edge = (V3::ZERO, V3::ZERO);
                let intersected = is_coplanar || Ext::new(ta).intersects(&eb, &mut edge);
                if intersected {
                    let va = a.bvh.indirect[i1] as usize;
                    let vb = b.bvh.indirect[i2] as usize;
                    ai.add(va, vb, is_coplanar);
                    bi.add(vb, va, is_coplanar);
                }
            }
        }
    };
    let root = a.bvh.node_box(0).transformed(&inv);
    traverse(
        &a.bvh, &b.bvh, 0, 0, &to_local, &inv, root, false, &mut visit,
    );
    (ai, bi)
}
/// bvhcast's _traverse: the boxes alternate between the trees' frames.
#[allow(clippy::too_many_arguments)]
fn traverse(
    t1: &Bvh,
    t2: &Bvh,
    n1: usize,
    n2: usize,
    m2to1: &M4,
    m1to2: &M4,
    curr: Box3,
    reversed: bool,
    visit: &mut impl FnMut(usize, usize, usize, usize),
) {
    let (a, b) = (&t1.nodes[n1], &t2.nodes[n2]);
    let (leaf1, leaf2) = (a.inner.is_none(), b.inner.is_none());
    if leaf1 && leaf2 {
        if reversed {
            visit(b.offset, b.count, a.offset, a.count);
        } else {
            visit(a.offset, a.count, b.offset, b.count);
        }
        return;
    }
    if leaf2 {
        let new_box = t2.node_box(n2).transformed(m2to1);
        let (cl1, cr1, _) = a.inner.unwrap_or((0, 0, 0));
        let l = new_box.intersects(&t1.node_box(cl1));
        let r = new_box.intersects(&t1.node_box(cr1));
        if l {
            traverse(t2, t1, n2, cl1, m1to2, m2to1, new_box, !reversed, visit);
        }
        if r {
            traverse(t2, t1, n2, cr1, m1to2, m2to1, new_box, !reversed, visit);
        }
        return;
    }
    let (cl2, cr2, _) = b.inner.unwrap_or((0, 0, 0));
    let li = curr.intersects(&t2.node_box(cl2));
    let ri = curr.intersects(&t2.node_box(cr2));
    if li && ri {
        traverse(t1, t2, n1, cl2, m2to1, m1to2, curr, reversed, visit);
        traverse(t1, t2, n1, cr2, m2to1, m1to2, curr, reversed, visit);
    } else if li || ri {
        let c2 = if li { cl2 } else { cr2 };
        if leaf1 {
            traverse(t1, t2, n1, c2, m2to1, m1to2, curr, reversed, visit);
        } else {
            let new_box = t2.node_box(c2).transformed(m2to1);
            let (cl1, cr1, _) = a.inner.unwrap_or((0, 0, 0));
            let l = new_box.intersects(&t1.node_box(cl1));
            let r = new_box.intersects(&t1.node_box(cr1));
            if l {
                traverse(t2, t1, c2, cl1, m1to2, m2to1, new_box, !reversed, visit);
            }
            if r {
                traverse(t2, t1, c2, cr1, m1to2, m2to1, new_box, !reversed, visit);
            }
        }
    }
}
const BACK_SIDE: i32 = -1;
const FRONT_SIDE: i32 = 1;
const COPLANAR_OPPOSITE: i32 = -2;
const COPLANAR_ALIGNED: i32 = 2;
#[derive(Clone, Copy, PartialEq)]
enum Action {
    Invert,
    Add,
    Skip,
}
fn action(operation: u32, side: i32, invert: bool) -> Action {
    match operation {
        ADDITION if side == FRONT_SIDE || (side == COPLANAR_ALIGNED && !invert) => Action::Add,
        SUBTRACTION if invert && side == BACK_SIDE => Action::Invert,
        SUBTRACTION if !invert && (side == FRONT_SIDE || side == COPLANAR_OPPOSITE) => Action::Add,
        INTERSECTION if side == BACK_SIDE || (side == COPLANAR_ALIGNED && !invert) => Action::Add,
        _ => Action::Skip,
    }
}
/// getHitSide: a ray from the midpoint along the normal into the other BVH.
fn hit_side(tri: &Tri, other: &Brush, matrix: Option<&M4>) -> i32 {
    let mut origin = tri.midpoint();
    let mut dir = tri.normal();
    if let Some(m) = matrix {
        origin = apply(m, origin);
        dir = transform_direction(m, dir);
    }
    match other.bvh.raycast_first(&other.source, origin, dir) {
        Some((_, _, normal)) if dir.dot(normal) > 0. => BACK_SIDE,
        _ => FRONT_SIDE,
    }
}
/// LegacyTriangleSplitter.
struct Splitter {
    triangles: Vec<Tri>,
}
const EPSILON: f64 = 1e-10;
impl Splitter {
    fn split_by_triangle(&mut self, triangle: &Tri, coplanar: bool) {
        if coplanar {
            let arr = [triangle.a, triangle.b, triangle.c];
            for i in 0..3 {
                let (v0, v1) = (arr[i], arr[(i + 1) % 3]);
                let n = normalize(triangle.normal());
                let e = normalize(v1 - v0);
                let plane = Plane::from_normal_point(n.cross(e), v0);
                self.split_by_plane(&plane, triangle);
            }
        } else {
            self.split_by_plane(&triangle.plane(), triangle);
        }
    }
    fn split_by_plane(&mut self, plane: &Plane, clipping: &Tri) {
        let clip = Ext::new(*clipping);
        let mut i = 0;
        let mut l = self.triangles.len();
        while i < l {
            let tri = self.triangles[i];
            let mut edge = (V3::ZERO, V3::ZERO);
            if !clip.intersects(&Ext::new(tri), &mut edge) {
                i += 1;
                continue;
            }
            let arr = [tri.a, tri.b, tri.c];
            let mut intersects = 0;
            let mut vertex_split_end: i32 = -1;
            let mut coplanar_edge = false;
            let mut pos = vec![];
            let mut neg = vec![];
            let mut found = (V3::ZERO, V3::ZERO);
            for t in 0..3 {
                let (s, e) = (arr[t], arr[(t + 1) % 3]);
                let sd = plane.distance(s);
                let ed = plane.distance(e);
                if sd.abs() < EPSILON && ed.abs() < EPSILON {
                    coplanar_edge = true;
                    break;
                }
                if sd > 0. {
                    pos.push(t);
                } else {
                    neg.push(t);
                }
                if sd.abs() < EPSILON {
                    continue;
                }
                let mut hit = plane.intersect_line(s, e);
                if hit.is_none() && ed.abs() < EPSILON {
                    hit = Some(e);
                }
                if let Some(v) = hit
                    && v.distance(s) >= EPSILON
                {
                    if v.distance(e) < EPSILON {
                        vertex_split_end = t as i32;
                    }
                    if intersects == 0 {
                        found.0 = v;
                    } else {
                        found.1 = v;
                    }
                    intersects += 1;
                }
            }
            if !coplanar_edge && intersects == 2 && found.0.distance(found.1) > EPSILON {
                if vertex_split_end != -1 {
                    let split = ((vertex_split_end + 1) % 3) as usize;
                    let mut other1 = 0;
                    if other1 == split {
                        other1 = (other1 + 1) % 3;
                    }
                    let mut other2 = other1 + 1;
                    if other2 == split {
                        other2 = (other2 + 1) % 3;
                    }
                    let next = Tri {
                        a: arr[other2],
                        b: found.1,
                        c: found.0,
                    };
                    if !is_degenerate(&next) {
                        self.triangles.push(next);
                    }
                    let t = Tri {
                        a: arr[other1],
                        b: found.0,
                        c: found.1,
                    };
                    self.triangles[i] = t;
                    if is_degenerate(&t) {
                        self.triangles.remove(i);
                        l -= 1;
                        continue;
                    }
                } else {
                    let single = if pos.len() >= 2 {
                        neg.first().copied().unwrap_or(0)
                    } else {
                        pos.first().copied().unwrap_or(0)
                    };
                    if single == 0 {
                        found = (found.1, found.0);
                    }
                    let (n1, n2) = ((single + 1) % 3, (single + 2) % 3);
                    let (t1, t2) =
                        if arr[n1].distance_squared(found.0) < arr[n2].distance_squared(found.1) {
                            (
                                Tri {
                                    a: arr[n1],
                                    b: found.0,
                                    c: found.1,
                                },
                                Tri {
                                    a: arr[n1],
                                    b: arr[n2],
                                    c: found.0,
                                },
                            )
                        } else {
                            (
                                Tri {
                                    a: arr[n2],
                                    b: found.0,
                                    c: found.1,
                                },
                                Tri {
                                    a: arr[n1],
                                    b: arr[n2],
                                    c: found.1,
                                },
                            )
                        };
                    let t = Tri {
                        a: arr[single],
                        b: found.1,
                        c: found.0,
                    };
                    if !is_degenerate(&t1) {
                        self.triangles.push(t1);
                    }
                    if !is_degenerate(&t2) {
                        self.triangles.push(t2);
                    }
                    self.triangles[i] = t;
                    if is_degenerate(&t) {
                        self.triangles.remove(i);
                        l -= 1;
                        continue;
                    }
                }
            }
            i += 1;
        }
    }
}
/// GeometryBuilder: Float32 attributes and per-group indices.
#[derive(Default)]
pub(super) struct Builder {
    pub positions: Vec<f32>,
    pub uvs: Vec<f32>,
    pub normals: Vec<f32>,
    pub groups: Vec<Vec<u32>>,
    forward: HashMap<Option<usize>, u32>,
    inverted: HashMap<Option<usize>, u32>,
    fields: Option<[[V3; 3]; 3]>,
    fields_uv: [[f64; 2]; 3],
}
impl Builder {
    fn clear(&mut self) {
        self.positions.clear();
        self.uvs.clear();
        self.normals.clear();
        for g in &mut self.groups {
            g.clear();
        }
        self.clear_index_map();
        self.fields = None;
    }
    fn clear_index_map(&mut self) {
        self.forward.clear();
        self.inverted.clear();
    }
    fn count(&self) -> u32 {
        (self.positions.len() / 3) as u32
    }
    fn group(&mut self, g: usize) -> &mut Vec<u32> {
        while self.groups.len() <= g {
            self.groups.push(vec![]);
        }
        &mut self.groups[g]
    }
    fn init_interpolated(&mut self, s: &Source, m: &M4, nm: &[f64; 9], i: [usize; 3]) {
        let p = i.map(|v| apply(m, s.position(v)));
        let n = i.map(|v| {
            apply_normal(
                nm,
                V3::new(
                    s.normals[v * 3] as f64,
                    s.normals[v * 3 + 1] as f64,
                    s.normals[v * 3 + 2] as f64,
                ),
            )
        });
        self.fields = Some([p, n, [V3::ZERO; 3]]);
        self.fields_uv = i.map(|v| [s.uvs[v * 2] as f64, s.uvs[v * 2 + 1] as f64]);
    }
    /// appendInterpolatedAttributeData. Directions are blended but not
    /// normalized: the library's Vector4 gets an undefined w, its length is NaN
    /// and normalize() divides by 1.
    fn append_interpolated(&mut self, group: usize, bary: V3, invert: bool) {
        let count = self.count();
        self.group(group).push(count);
        let Some([p, n, _]) = self.fields else { return };
        let blend = |f: &[V3; 3]| f[0] * bary.x + f[1] * bary.y + f[2] * bary.z;
        let pos = blend(&p);
        let uv = [0, 1].map(|k| {
            0. + self.fields_uv[0][k] * bary.x
                + self.fields_uv[1][k] * bary.y
                + self.fields_uv[2][k] * bary.z
        });
        let mut normal = blend(&n);
        if invert {
            normal *= -1.;
        }
        self.positions
            .extend([pos.x as f32, pos.y as f32, pos.z as f32]);
        self.uvs.extend([uv[0] as f32, uv[1] as f32]);
        self.normals
            .extend([normal.x as f32, normal.y as f32, normal.z as f32]);
        let map = if invert {
            &mut self.inverted
        } else {
            &mut self.forward
        };
        map.insert(None, count);
    }
    /// appendIndexFromGeometry.
    fn append_index(
        &mut self,
        s: &Source,
        m: &M4,
        nm: &[f64; 9],
        group: usize,
        index: usize,
        invert: bool,
    ) {
        let map = if invert {
            &self.inverted
        } else {
            &self.forward
        };
        if let Some(&v) = map.get(&Some(index)) {
            self.group(group).push(v);
            return;
        }
        let count = self.count();
        if invert {
            self.inverted.insert(Some(index), count);
        } else {
            self.forward.insert(Some(index), count);
        }
        self.group(group).push(count);
        let p = apply(m, s.position(index));
        let mut n = apply_normal(
            nm,
            V3::new(
                s.normals[index * 3] as f64,
                s.normals[index * 3 + 1] as f64,
                s.normals[index * 3 + 2] as f64,
            ),
        );
        if invert {
            n *= -1.;
        }
        self.positions.extend([p.x as f32, p.y as f32, p.z as f32]);
        self.uvs.extend([s.uvs[index * 2], s.uvs[index * 2 + 1]]);
        self.normals.extend([n.x as f32, n.y as f32, n.z as f32]);
    }
}
/// performWholeTriangleOperations.
#[allow(clippy::too_many_arguments)]
fn whole(
    a: &Brush,
    b: &Brush,
    split: &Intersections,
    operation: u32,
    second: bool,
    builder: &mut Builder,
    group_offset: i64,
) {
    let matrix = multiply(&invert(&b.matrix), &a.matrix);
    let builder_matrix = if second { matrix } else { IDENTITY };
    let inverted = determinant(&builder_matrix) < 0.;
    let nm = normal_matrix(&builder_matrix, inverted);
    let tris = a.source.tri_count();
    let mut traversed: HashSet<usize> = split.ids.iter().copied().collect();
    let mut stack = vec![];
    for id in 0..tris {
        if traversed.len() == tris {
            break;
        }
        if traversed.contains(&id) {
            continue;
        }
        traversed.insert(id);
        stack.push(id);
        let mut tri = a.source.triangle(id);
        if second {
            tri = tri.transformed(&matrix);
        }
        let side = hit_side(&tri, b, if second { None } else { Some(&matrix) });
        let act = action(operation, side, second);
        while let Some(curr) = stack.pop() {
            for i in 0..3 {
                let other = a.half_edges[curr * 3 + i];
                let sid = if other == -1 { -1 } else { other / 3 };
                if sid != -1 && !traversed.contains(&(sid as usize)) {
                    stack.push(sid as usize);
                    traversed.insert(sid as usize);
                }
            }
            if act == Action::Skip {
                continue;
            }
            let i = [0, 1, 2].map(|k| a.source.vertex(curr * 3 + k));
            let group = if group_offset == -1 {
                0
            } else {
                (a.group_indices[curr] as i64 + group_offset) as usize
            };
            let t = a.source.triangle(curr);
            if is_degenerate(&t) {
                continue;
            }
            let flip = (act == Action::Invert) != inverted;
            builder.append_index(&a.source, &builder_matrix, &nm, group, i[0], flip);
            if flip {
                builder.append_index(&a.source, &builder_matrix, &nm, group, i[2], flip);
                builder.append_index(&a.source, &builder_matrix, &nm, group, i[1], flip);
            } else {
                builder.append_index(&a.source, &builder_matrix, &nm, group, i[1], flip);
                builder.append_index(&a.source, &builder_matrix, &nm, group, i[2], flip);
            }
        }
    }
}
/// performSplitTriangleOperations.
#[allow(clippy::too_many_arguments)]
fn split_ops(
    a: &Brush,
    b: &Brush,
    map: &Intersections,
    operation: u32,
    second: bool,
    builder: &mut Builder,
    group_offset: i64,
) {
    let matrix = multiply(&invert(&b.matrix), &a.matrix);
    let inverse = invert(&matrix);
    let builder_matrix = if second { matrix } else { IDENTITY };
    let inverted = determinant(&builder_matrix) < 0.;
    let nm = normal_matrix(&builder_matrix, inverted);
    for &ia in &map.ids {
        let group = if group_offset == -1 {
            0
        } else {
            (a.group_indices[ia] as i64 + group_offset) as usize
        };
        let i = [0, 1, 2].map(|k| a.source.vertex(ia * 3 + k));
        let mut tri_a = a.source.triangle(ia);
        if second {
            tri_a = tri_a.transformed(&matrix);
        }
        let mut splitter = Splitter {
            triangles: vec![tri_a],
        };
        let normal = tri_a.normal();
        let coplanar_ids = map.coplanar.get(&ia);
        let coplanar_tris: Vec<Tri> = coplanar_ids
            .map(|ids| {
                ids.iter()
                    .map(|&index| {
                        let t = b.source.triangle(index);
                        if second { t } else { t.transformed(&inverse) }
                    })
                    .collect()
            })
            .unwrap_or_default();
        for &index in &map.sets[&ia] {
            let is_coplanar = coplanar_ids.is_some_and(|c| c.contains(&index));
            let mut tb = b.source.triangle(index);
            if !second {
                tb = tb.transformed(&inverse);
            }
            splitter.split_by_triangle(&tb, is_coplanar);
        }
        builder.init_interpolated(&a.source, &builder_matrix, &nm, i);
        for clipped in &splitter.triangles {
            let mid = clipped.midpoint();
            let mut side = None;
            for cpt in &coplanar_tris {
                if cpt.contains(mid) {
                    side = Some(if normal.dot(cpt.normal()) > 0. {
                        COPLANAR_ALIGNED
                    } else {
                        COPLANAR_OPPOSITE
                    });
                    break;
                }
            }
            let side = side
                .unwrap_or_else(|| hit_side(clipped, b, if second { None } else { Some(&matrix) }));
            let act = action(operation, side, second);
            if act == Action::Skip {
                continue;
            }
            let bary = [clipped.a, clipped.b, clipped.c].map(|p| tri_a.barycoord_or_zero(p));
            let flip = inverted != (act == Action::Invert);
            builder.append_interpolated(group, bary[0], flip);
            if flip {
                builder.append_interpolated(group, bary[2], flip);
                builder.append_interpolated(group, bary[1], flip);
            } else {
                builder.append_interpolated(group, bary[1], flip);
                builder.append_interpolated(group, bary[2], flip);
            }
        }
    }
}
/// The evaluated result: Float32 attributes, the index and its material groups.
pub(super) struct Output {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub index: Vec<u32>,
    /// start, count, material ( 0 for A, 1 for B ).
    pub groups: Vec<(usize, usize, usize)>,
}
/// Evaluator.evaluate( a, b, operation ) with consolidateGroups and
/// removeUnusedMaterials; `use_groups` keeps A's and B's materials apart.
pub(super) fn evaluate(
    a: &Brush,
    b: &Brush,
    operation: u32,
    use_groups: bool,
    builder: &mut Builder,
) -> Output {
    builder.clear();
    let (ai, bi) = collect(a, b);
    let offset_a = if use_groups { 0 } else { -1 };
    whole(a, b, &ai, operation, false, builder, offset_a);
    split_ops(a, b, &ai, operation, false, builder, offset_a);
    builder.clear_index_map();
    let offset_b = if use_groups {
        a.source.groups.len().max(1) as i64
    } else {
        -1
    };
    whole(b, a, &bi, operation, true, builder, offset_b);
    split_ops(b, a, &bi, operation, true, builder, offset_b);
    builder.clear_index_map();
    // Groups: A's ranges then B's, materials made common ( A's single
    // material, B's single material ), sorted by material and joined.
    let a_groups = if !use_groups || a.source.groups.is_empty() {
        vec![0]
    } else {
        (0..a.source.groups.len()).collect()
    };
    let b_groups = if !use_groups || b.source.groups.is_empty() {
        vec![0]
    } else {
        (0..b.source.groups.len()).collect()
    };
    let mut order: Vec<(usize, usize)> = vec![];
    if use_groups {
        for (i, _) in a_groups.iter().enumerate() {
            order.push((i, 0));
        }
        for (j, _) in b_groups.iter().enumerate() {
            order.push((a_groups.len() + j, 1));
        }
        order.sort_by_key(|g| g.1);
    } else {
        order.push((0, 0));
    }
    let mut index = vec![];
    let mut groups: Vec<(usize, usize, usize)> = vec![];
    for &(g, material) in order.iter().take(order.len().min(builder.groups.len())) {
        let data = &builder.groups[g];
        if !data.is_empty() {
            groups.push((index.len(), data.len(), material));
            index.extend_from_slice(data);
        }
    }
    if use_groups {
        // joinGroups, then removeUnusedMaterials.
        let mut i = 0;
        while i + 1 < groups.len() {
            if groups[i].2 == groups[i + 1].2 {
                let start = groups[i].0;
                let end = groups[i + 1].0 + groups[i + 1].1;
                groups[i + 1] = (start, end - start, groups[i + 1].2);
                groups.remove(i);
            } else {
                i += 1;
            }
        }
    }
    Output {
        positions: builder.positions.clone(),
        normals: builder.normals.clone(),
        uvs: builder.uvs.clone(),
        index,
        groups,
    }
}
