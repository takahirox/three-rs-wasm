//! BufferGeometry as the city generators use it: named Float32Array
//! attributes in insertion order and an optional index, with three.js's
//! transforms ( Matrix4 by Vector3.applyMatrix4, normals through the normal
//! matrix and renormalized ), computeVertexNormals, and
//! BufferGeometryUtils' mergeVertices and mergeGeometries, in f64 with the
//! Float32Array stores at the points three.js writes them.
use std::collections::HashMap;

pub(super) type V3 = [f64; 3];
/// A Matrix4's elements ( column-major ).
pub(super) type M4 = [f64; 16];
pub(super) const IDENTITY: M4 = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];
pub(super) fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub(super) fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub(super) fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub(super) fn length(a: V3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
/// Vector3.normalize(): divideScalar( length || 1 ).
pub(super) fn normalize(a: V3) -> V3 {
    let l = length(a);
    let inverse = 1. / if l == 0. { 1. } else { l };
    [a[0] * inverse, a[1] * inverse, a[2] * inverse]
}
/// A quaternion ( x, y, z, w ).
pub(super) type Q = [f64; 4];
/// Quaternion.normalize().
fn normalize_q(q: Q) -> Q {
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if l == 0. {
        return [0., 0., 0., 1.];
    }
    let l = 1. / l;
    [q[0] * l, q[1] * l, q[2] * l, q[3] * l]
}
/// Quaternion.setFromUnitVectors( from, to ).
pub(super) fn from_unit_vectors(from: V3, to: V3) -> Q {
    let r = dot(from, to) + 1.;
    let q = if r < 1e-8 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0., 0.]
        } else {
            [0., -from[2], from[1], 0.]
        }
    } else {
        [
            from[1] * to[2] - from[2] * to[1],
            from[2] * to[0] - from[0] * to[2],
            from[0] * to[1] - from[1] * to[0],
            r,
        ]
    };
    normalize_q(q)
}
/// Quaternion.setFromAxisAngle( axis, angle ).
pub(super) fn from_axis_angle(axis: V3, angle: f64) -> Q {
    let s = (angle / 2.).sin();
    [axis[0] * s, axis[1] * s, axis[2] * s, (angle / 2.).cos()]
}
/// Vector3.applyQuaternion( q ).
pub(super) fn apply_quaternion(v: V3, q: Q) -> V3 {
    let (vx, vy, vz) = (v[0], v[1], v[2]);
    let (qx, qy, qz, qw) = (q[0], q[1], q[2], q[3]);
    let tx = 2. * (qy * vz - qz * vy);
    let ty = 2. * (qz * vx - qx * vz);
    let tz = 2. * (qx * vy - qy * vx);
    [
        vx + qw * tx + qy * tz - qz * ty,
        vy + qw * ty + qz * tx - qx * tz,
        vz + qw * tz + qx * ty - qy * tx,
    ]
}
/// Matrix4.compose( position, quaternion, scale ).
pub(super) fn compose(p: V3, q: Q, s: V3) -> M4 {
    let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        (1. - (yy + zz)) * s[0],
        (xy + wz) * s[0],
        (xz - wy) * s[0],
        0.,
        (xy - wz) * s[1],
        (1. - (xx + zz)) * s[1],
        (yz + wx) * s[1],
        0.,
        (xz + wy) * s[2],
        (yz - wx) * s[2],
        (1. - (xx + yy)) * s[2],
        0.,
        p[0],
        p[1],
        p[2],
        1.,
    ]
}
/// Matrix4.makeBasis( x, y, z ).
pub(super) fn basis(x: V3, y: V3, z: V3) -> M4 {
    [
        x[0], x[1], x[2], 0., y[0], y[1], y[2], 0., z[0], z[1], z[2], 0., 0., 0., 0., 1.,
    ]
}
pub(super) fn set_position(mut m: M4, p: V3) -> M4 {
    m[12] = p[0];
    m[13] = p[1];
    m[14] = p[2];
    m
}
pub(super) fn translation(p: V3) -> M4 {
    set_position(IDENTITY, p)
}
pub(super) fn scaling(s: V3) -> M4 {
    [
        s[0], 0., 0., 0., 0., s[1], 0., 0., 0., 0., s[2], 0., 0., 0., 0., 1.,
    ]
}
/// Matrix4.makeRotationX/Y/Z( angle ).
pub(super) fn rotation(axis: usize, angle: f64) -> M4 {
    let (c, s) = (angle.cos(), angle.sin());
    match axis {
        0 => [1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.],
        1 => [c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.],
        _ => [c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.],
    }
}
/// Matrix4.multiplyMatrices( a, b ).
pub(super) fn multiply(a: &M4, b: &M4) -> M4 {
    let mut out = [0.; 16];
    for r in 0..4 {
        for c in 0..4 {
            out[c * 4 + r] = a[r] * b[c * 4]
                + a[4 + r] * b[c * 4 + 1]
                + a[8 + r] * b[c * 4 + 2]
                + a[12 + r] * b[c * 4 + 3];
        }
    }
    out
}
/// Matrix3.getNormalMatrix( m ): setFromMatrix4, invert, transpose.
fn normal_matrix(m: &M4) -> [f64; 9] {
    let (n11, n21, n31) = (m[0], m[1], m[2]);
    let (n12, n22, n32) = (m[4], m[5], m[6]);
    let (n13, n23, n33) = (m[8], m[9], m[10]);
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    if det == 0. {
        return [0.; 9];
    }
    let d = 1. / det;
    let te = [
        t11 * d,
        (n31 * n23 - n33 * n21) * d,
        (n32 * n21 - n31 * n22) * d,
        t12 * d,
        (n33 * n11 - n31 * n13) * d,
        (n31 * n12 - n32 * n11) * d,
        t13 * d,
        (n21 * n13 - n23 * n11) * d,
        (n22 * n11 - n21 * n12) * d,
    ];
    [
        te[0], te[3], te[6], te[1], te[4], te[7], te[2], te[5], te[8],
    ]
}
/// A BufferGeometry: attributes ( name, itemSize, values ) in insertion
/// order and an optional index.
#[derive(Clone, Default)]
pub(super) struct G {
    pub(super) attributes: Vec<(&'static str, usize, Vec<f32>)>,
    pub(super) index: Option<Vec<u32>>,
}
impl G {
    pub(super) fn get(&self, name: &str) -> Option<&Vec<f32>> {
        self.attributes.iter().find(|a| a.0 == name).map(|a| &a.2)
    }
    pub(super) fn get_mut(&mut self, name: &str) -> Option<&mut Vec<f32>> {
        self.attributes
            .iter_mut()
            .find(|a| a.0 == name)
            .map(|a| &mut a.2)
    }
    pub(super) fn set(&mut self, name: &'static str, size: usize, values: Vec<f32>) {
        match self.attributes.iter_mut().find(|a| a.0 == name) {
            Some(a) => *a = (name, size, values),
            None => self.attributes.push((name, size, values)),
        }
    }
    pub(super) fn delete(&mut self, name: &str) {
        self.attributes.retain(|a| a.0 != name);
    }
    pub(super) fn count(&self) -> usize {
        self.get("position").map_or(0, |p| p.len() / 3)
    }
    /// BufferGeometry.applyMatrix4( m ).
    pub(super) fn apply(mut self, m: &M4) -> Self {
        if let Some(p) = self.get_mut("position") {
            for p in p.chunks_mut(3) {
                let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64);
                let w = 1. / (m[3] * x + m[7] * y + m[11] * z + m[15]);
                p[0] = ((m[0] * x + m[4] * y + m[8] * z + m[12]) * w) as f32;
                p[1] = ((m[1] * x + m[5] * y + m[9] * z + m[13]) * w) as f32;
                p[2] = ((m[2] * x + m[6] * y + m[10] * z + m[14]) * w) as f32;
            }
        }
        if self.get("normal").is_some() {
            let n = normal_matrix(m);
            for v in self
                .get_mut("normal")
                .into_iter()
                .flat_map(|v| v.chunks_mut(3))
            {
                let (x, y, z) = (v[0] as f64, v[1] as f64, v[2] as f64);
                let t = normalize([
                    n[0] * x + n[3] * y + n[6] * z,
                    n[1] * x + n[4] * y + n[7] * z,
                    n[2] * x + n[5] * y + n[8] * z,
                ]);
                for k in 0..3 {
                    v[k] = t[k] as f32;
                }
            }
        }
        self
    }
    pub(super) fn translate(self, x: f64, y: f64, z: f64) -> Self {
        self.apply(&translation([x, y, z]))
    }
    pub(super) fn scale(self, x: f64, y: f64, z: f64) -> Self {
        self.apply(&scaling([x, y, z]))
    }
    pub(super) fn rotate_x(self, angle: f64) -> Self {
        self.apply(&rotation(0, angle))
    }
    pub(super) fn rotate_y(self, angle: f64) -> Self {
        self.apply(&rotation(1, angle))
    }
    pub(super) fn rotate_z(self, angle: f64) -> Self {
        self.apply(&rotation(2, angle))
    }
    /// applyQuaternion: makeRotationFromQuaternion, then applyMatrix4.
    pub(super) fn apply_quaternion(self, q: Q) -> Self {
        self.apply(&compose([0.; 3], q, [1.; 3]))
    }
    /// computeVertexNormals(): face cross products summed into the
    /// Float32Array normals ( reset first ) by index, or set per corner,
    /// then normalizeNormals().
    pub(super) fn compute_vertex_normals(&mut self) {
        let position = self.get("position").cloned().unwrap_or_default();
        let mut normal = vec![0f32; position.len()];
        let p = |i: usize| {
            [
                position[i * 3] as f64,
                position[i * 3 + 1] as f64,
                position[i * 3 + 2] as f64,
            ]
        };
        match &self.index {
            Some(index) => {
                for t in index.chunks(3) {
                    let (a, b, c) = (t[0] as usize, t[1] as usize, t[2] as usize);
                    let n = cross(sub(p(c), p(b)), sub(p(a), p(b)));
                    // nA, nB and nC are read before any is written back.
                    let sums: Vec<[f32; 3]> = [a, b, c]
                        .iter()
                        .map(|&v| [0, 1, 2].map(|k| (normal[v * 3 + k] as f64 + n[k]) as f32))
                        .collect();
                    for (v, sum) in [a, b, c].into_iter().zip(sums) {
                        normal[v * 3..v * 3 + 3].copy_from_slice(&sum);
                    }
                }
            }
            None => {
                for i in (0..position.len() / 3).step_by(3) {
                    let n = cross(sub(p(i + 2), p(i + 1)), sub(p(i), p(i + 1)));
                    for v in i..i + 3 {
                        for k in 0..3 {
                            normal[v * 3 + k] = n[k] as f32;
                        }
                    }
                }
            }
        }
        if self.get("normal").is_none() {
            self.set("normal", 3, vec![]);
        }
        *self.get_mut("normal").expect("normal") = normal;
        self.normalize_normals();
    }
    /// normalizeNormals().
    pub(super) fn normalize_normals(&mut self) {
        for n in self
            .get_mut("normal")
            .into_iter()
            .flat_map(|n| n.chunks_mut(3))
        {
            let v = normalize([n[0] as f64, n[1] as f64, n[2] as f64]);
            for k in 0..3 {
                n[k] = v[k] as f32;
            }
        }
    }
    /// computeBoundingSphere(): the box centre and the farthest vertex.
    pub(super) fn bounding_sphere(&self) -> (V3, f64) {
        let position = self.get("position").map_or(&[][..], |p| &p[..]);
        let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in position.chunks(3) {
            for k in 0..3 {
                min[k] = min[k].min(p[k] as f64);
                max[k] = max[k].max(p[k] as f64);
            }
        }
        let center = [0, 1, 2].map(|k| (min[k] + max[k]) * 0.5);
        let mut radius = 0f64;
        for p in position.chunks(3) {
            let d = sub([p[0] as f64, p[1] as f64, p[2] as f64], center);
            radius = radius.max(dot(d, d));
        }
        (center, radius.sqrt())
    }
}
/// BufferGeometryUtils.mergeVertices( geometry, 1e-4 ): every attribute
/// hashed to the tolerance, the first vertex of each hash kept.
pub(super) fn merge_vertices(g: &G) -> G {
    let tolerance = 1e-4f64;
    let half_tolerance = tolerance * 0.5;
    let exponent = (1. / tolerance).log10();
    let multiplier = 10f64.powf(exponent);
    let additive = half_tolerance * multiplier;
    let count = g.index.as_ref().map_or(g.count(), Vec::len);
    let mut map: HashMap<Vec<i64>, u32> = HashMap::new();
    let mut out: Vec<(&'static str, usize, Vec<f32>)> = g
        .attributes
        .iter()
        .map(|(n, s, _)| (*n, *s, vec![]))
        .collect();
    let mut index = Vec::with_capacity(count);
    let mut next = 0u32;
    for i in 0..count {
        let v = g.index.as_ref().map_or(i, |x| x[i] as usize);
        let mut hash = vec![];
        for (_, size, values) in &g.attributes {
            for k in 0..*size {
                hash.push((values[v * size + k] as f64 * multiplier + additive).trunc() as i64);
            }
        }
        match map.get(&hash) {
            Some(&id) => index.push(id),
            None => {
                for ((_, size, values), o) in g.attributes.iter().zip(&mut out) {
                    o.2.extend_from_slice(&values[v * size..v * size + size]);
                }
                map.insert(hash, next);
                index.push(next);
                next += 1;
            }
        }
    }
    G {
        attributes: out,
        index: Some(index),
    }
}
/// BufferGeometryUtils.mergeGeometries( geometries ): the attributes
/// concatenated in the first geometry's order and the indices offset.
pub(super) fn merge_geometries(parts: &[G]) -> G {
    let mut out = G {
        attributes: parts[0]
            .attributes
            .iter()
            .map(|(n, s, _)| (*n, *s, vec![]))
            .collect(),
        index: parts[0].index.as_ref().map(|_| vec![]),
    };
    let mut offset = 0;
    for p in parts {
        if let (Some(o), Some(i)) = (&mut out.index, &p.index) {
            o.extend(i.iter().map(|&v| v + offset));
        }
        for (name, _, values) in &mut out.attributes {
            values.extend_from_slice(p.get(name).expect("compatible attributes"));
        }
        offset += p.count() as u32;
    }
    out
}
/// CityGeneratorUtils.part( geometry, id ): indexed, tagged with its
/// material zone.
pub(super) fn part(g: G, id: f32) -> G {
    let mut g = if g.index.is_some() {
        g
    } else {
        merge_vertices(&g)
    };
    let n = g.count();
    g.set("partId", 1, vec![id; n]);
    g
}
