//! three.js math as 3d-tiles-renderer 0.4.27 uses it, in three's f64 order of
//! operations: Matrix4, Plane, Sphere, the OBB, ExtendedFrustum and
//! TileBoundingVolume ( sphere and box volumes; Google's tiles use boxes ).

pub(crate) type V3 = [f64; 3];
pub(crate) type M4 = [f64; 16];
pub(crate) const IDENTITY: M4 = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

pub(crate) fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub(crate) fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub(crate) fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub(crate) fn length(a: V3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
/// Vector3.normalize: divideScalar( length() || 1 ), which multiplies by the
/// reciprocal.
pub(crate) fn normalize(a: V3) -> V3 {
    let l = length(a);
    let inverse = 1. / if l == 0. || l.is_nan() { 1. } else { l };
    [a[0] * inverse, a[1] * inverse, a[2] * inverse]
}
pub(crate) fn distance(a: V3, b: V3) -> f64 {
    let d = sub(a, b);
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// Matrix4.multiplyMatrices( a, b ).
pub(crate) fn multiply(a: &M4, b: &M4) -> M4 {
    let (a11, a12, a13, a14) = (a[0], a[4], a[8], a[12]);
    let (a21, a22, a23, a24) = (a[1], a[5], a[9], a[13]);
    let (a31, a32, a33, a34) = (a[2], a[6], a[10], a[14]);
    let (a41, a42, a43, a44) = (a[3], a[7], a[11], a[15]);
    let (b11, b12, b13, b14) = (b[0], b[4], b[8], b[12]);
    let (b21, b22, b23, b24) = (b[1], b[5], b[9], b[13]);
    let (b31, b32, b33, b34) = (b[2], b[6], b[10], b[14]);
    let (b41, b42, b43, b44) = (b[3], b[7], b[11], b[15]);
    [
        a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41,
        a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41,
        a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41,
        a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41,
        a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42,
        a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42,
        a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42,
        a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42,
        a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43,
        a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43,
        a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43,
        a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43,
        a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44,
        a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44,
        a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44,
        a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44,
    ]
}
/// Matrix4.invert.
pub(crate) fn invert(te: &M4) -> M4 {
    let [
        n11,
        n21,
        n31,
        n41,
        n12,
        n22,
        n32,
        n42,
        n13,
        n23,
        n33,
        n43,
        n14,
        n24,
        n34,
        n44,
    ] = *te;
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
/// Vector3.applyMatrix4 ( with the perspective divide ).
pub(crate) fn apply4(v: V3, e: &M4) -> V3 {
    let [x, y, z] = v;
    let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
    [
        (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
        (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
        (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
    ]
}
/// Vector3.transformDirection.
pub(crate) fn transform_direction(v: V3, e: &M4) -> V3 {
    let [x, y, z] = v;
    normalize([
        e[0] * x + e[4] * y + e[8] * z,
        e[1] * x + e[5] * y + e[9] * z,
        e[2] * x + e[6] * y + e[10] * z,
    ])
}
/// Matrix4.getMaxScaleOnAxis.
pub(crate) fn max_scale(te: &M4) -> f64 {
    let x = te[0] * te[0] + te[1] * te[1] + te[2] * te[2];
    let y = te[4] * te[4] + te[5] * te[5] + te[6] * te[6];
    let z = te[8] * te[8] + te[9] * te[9] + te[10] * te[10];
    x.max(y).max(z).sqrt()
}

/// Plane: a normal and a constant.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Plane {
    pub normal: V3,
    pub constant: f64,
}
impl Plane {
    /// setComponents( ... ).normalize().
    fn normalized(x: f64, y: f64, z: f64, w: f64) -> Self {
        let inverse = 1.0 / length([x, y, z]);
        Plane {
            normal: [x * inverse, y * inverse, z * inverse],
            constant: w * inverse,
        }
    }
    fn from_normal_and_point(normal: V3, point: V3) -> Self {
        Plane {
            normal,
            constant: -dot(point, normal),
        }
    }
    fn negate(self) -> Self {
        Plane {
            normal: [-self.normal[0], -self.normal[1], -self.normal[2]],
            constant: -self.constant,
        }
    }
    pub fn distance_to_point(&self, p: V3) -> f64 {
        dot(self.normal, p) + self.constant
    }
}

/// ExtendedFrustum: three's Frustum ( WebGL clip space ) and its corners.
#[derive(Clone, Debug, Default)]
pub(crate) struct Frustum {
    pub planes: [Plane; 6],
    pub points: [V3; 8],
}
impl Frustum {
    pub fn from_projection(me: &M4) -> Self {
        let planes = [
            Plane::normalized(
                me[3] - me[0],
                me[7] - me[4],
                me[11] - me[8],
                me[15] - me[12],
            ),
            Plane::normalized(
                me[3] + me[0],
                me[7] + me[4],
                me[11] + me[8],
                me[15] + me[12],
            ),
            Plane::normalized(
                me[3] + me[1],
                me[7] + me[5],
                me[11] + me[9],
                me[15] + me[13],
            ),
            Plane::normalized(
                me[3] - me[1],
                me[7] - me[5],
                me[11] - me[9],
                me[15] - me[13],
            ),
            Plane::normalized(
                me[3] - me[2],
                me[7] - me[6],
                me[11] - me[10],
                me[15] - me[14],
            ),
            Plane::normalized(
                me[3] + me[2],
                me[7] + me[6],
                me[11] + me[10],
                me[15] + me[14],
            ),
        ];
        let corners = [
            [0, 3, 4],
            [1, 3, 4],
            [0, 2, 4],
            [1, 2, 4],
            [0, 3, 5],
            [1, 3, 5],
            [0, 2, 5],
            [1, 2, 5],
        ];
        let points = corners.map(|[a, b, c]| intersection(&planes[a], &planes[b], &planes[c]));
        Frustum { planes, points }
    }
    /// Frustum.intersectsSphere.
    pub fn intersects_sphere(&self, center: V3, radius: f64) -> bool {
        let negative = -radius;
        self.planes
            .iter()
            .all(|p| p.distance_to_point(center) >= negative)
    }
}
/// findIntersectionPoint: Matrix3( normals ).invert() applied to −constants.
fn intersection(p1: &Plane, p2: &Plane, p3: &Plane) -> V3 {
    // Matrix3.set takes rows; te is column-major.
    let te = [
        p1.normal[0],
        p2.normal[0],
        p3.normal[0],
        p1.normal[1],
        p2.normal[1],
        p3.normal[1],
        p1.normal[2],
        p2.normal[2],
        p3.normal[2],
    ];
    let inv = invert3(&te);
    let v = [-p1.constant, -p2.constant, -p3.constant];
    [
        inv[0] * v[0] + inv[3] * v[1] + inv[6] * v[2],
        inv[1] * v[0] + inv[4] * v[1] + inv[7] * v[2],
        inv[2] * v[0] + inv[5] * v[1] + inv[8] * v[2],
    ]
}
/// Matrix3.invert.
fn invert3(te: &[f64; 9]) -> [f64; 9] {
    let [n11, n21, n31, n12, n22, n32, n13, n23, n33] = *te;
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    if det == 0. {
        return [0.; 9];
    }
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
}

/// OBB: a box in a transform, its corners and face planes.
#[derive(Clone, Debug)]
pub(crate) struct Obb {
    pub min: V3,
    pub max: V3,
    pub transform: M4,
    pub inverse: M4,
    pub points: [V3; 8],
    pub planes: [Plane; 6],
}
impl Obb {
    pub fn new(min: V3, max: V3, transform: M4) -> Self {
        let inverse = invert(&transform);
        let mut points = [[0.; 3]; 8];
        let mut index = 0;
        for x in [-1, 1] {
            for y in [-1, 1] {
                for z in [-1, 1] {
                    points[index] = apply4(
                        [
                            if x < 0 { min[0] } else { max[0] },
                            if y < 0 { min[1] } else { max[1] },
                            if z < 0 { min[2] } else { max[2] },
                        ],
                        &transform,
                    );
                    index += 1;
                }
            }
        }
        let world_min = apply4(min, &transform);
        let world_max = apply4(max, &transform);
        let face = |axis: V3| {
            let n = transform_direction(axis, &transform);
            [
                Plane::from_normal_and_point(n, world_min),
                Plane::from_normal_and_point(n, world_max).negate(),
            ]
        };
        let [p0, p1] = face([0., 0., 1.]);
        let [p2, p3] = face([0., 1., 0.]);
        let [p4, p5] = face([1., 0., 0.]);
        Obb {
            min,
            max,
            transform,
            inverse,
            points,
            planes: [p0, p1, p2, p3, p4, p5],
        }
    }
    /// clampPoint: into the box's space, clamped, and back.
    fn clamp_point(&self, p: V3) -> V3 {
        let local = apply4(p, &self.inverse);
        let clamped = [0, 1, 2].map(|a| local[a].min(self.max[a]).max(self.min[a]));
        apply4(clamped, &self.transform)
    }
    pub fn distance_to_point(&self, p: V3) -> f64 {
        distance(self.clamp_point(p), p)
    }
    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        let separated = |planes: &[Plane; 6], points: &[V3; 8]| {
            planes.iter().any(|plane| {
                let mut max = f64::NEG_INFINITY;
                for &v in points {
                    let d = plane.distance_to_point(v);
                    max = if max < d { d } else { max };
                }
                max < 0.
            })
        };
        !separated(&frustum.planes, &self.points) && !separated(&self.planes, &frustum.points)
    }
}

/// TileBoundingVolume with a sphere and / or an OBB.
#[derive(Clone, Debug, Default)]
pub(crate) struct Volume {
    pub sphere: Option<(V3, f64)>,
    pub obb: Option<Obb>,
}
impl Volume {
    /// setObbData( box, transform ).
    pub fn set_box(&mut self, data: &[f64], transform: &M4) {
        let mut x = [data[3], data[4], data[5]];
        let mut y = [data[6], data[7], data[8]];
        let mut z = [data[9], data[10], data[11]];
        let (sx, sy, sz) = (length(x), length(y), length(z));
        x = normalize(x);
        y = normalize(y);
        z = normalize(z);
        if sx == 0. {
            x = cross(y, z);
        }
        if sy == 0. {
            y = cross(x, z);
        }
        if sz == 0. {
            z = cross(x, y);
        }
        // Matrix4.set takes rows.
        let local = [
            x[0], x[1], x[2], 0., y[0], y[1], y[2], 0., z[0], z[1], z[2], 0., data[0], data[1],
            data[2], 1.,
        ];
        let transform = multiply(transform, &local);
        self.obb = Some(Obb::new([-sx, -sy, -sz], [sx, sy, sz], transform));
    }
    /// setSphereData: Sphere.applyMatrix4.
    pub fn set_sphere(&mut self, data: &[f64], transform: &M4) {
        let center = apply4([data[0], data[1], data[2]], transform);
        self.sphere = Some((center, data[3] * max_scale(transform)));
    }
    pub fn distance_to_point(&self, p: V3) -> f64 {
        let mut sphere = f64::NEG_INFINITY;
        let mut obb = f64::NEG_INFINITY;
        if let Some((c, r)) = self.sphere {
            sphere = (distance(c, p) - r).max(0.);
        }
        if let Some(o) = &self.obb {
            obb = o.distance_to_point(p);
        }
        if sphere > obb { sphere } else { obb }
    }
    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        if let Some((c, r)) = self.sphere
            && !frustum.intersects_sphere(c, r)
        {
            return false;
        }
        if let Some(o) = &self.obb
            && !o.intersects_frustum(frustum)
        {
            return false;
        }
        self.sphere.is_some() || self.obb.is_some()
    }
}
