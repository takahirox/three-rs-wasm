//! three.js Matrix4 / Matrix3 / Vector3 operations in their f64 order of
//! evaluation, so that the transformed vertices round to the original's
//! Float32Array values.

pub(crate) type M4 = [f64; 16];
pub(crate) type M3 = [f64; 9];
pub(crate) const IDENTITY: M4 = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

/// The host's Math.sin / Math.cos ( Euler rotations ).
pub(crate) fn sin(x: f64) -> f64 {
    crate::bounce::trig::sin(x)
}
pub(crate) fn cos(x: f64) -> f64 {
    crate::bounce::trig::cos(x)
}

/// Matrix4.compose( position, quaternion, scale ).
pub(crate) fn compose(p: [f64; 3], q: [f64; 4], s: [f64; 3]) -> M4 {
    let [x, y, z, w] = q;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    let [sx, sy, sz] = s;
    [
        (1. - (yy + zz)) * sx,
        (xy + wz) * sx,
        (xz - wy) * sx,
        0.,
        (xy - wz) * sy,
        (1. - (xx + zz)) * sy,
        (yz + wx) * sy,
        0.,
        (xz + wy) * sz,
        (yz - wx) * sz,
        (1. - (xx + yy)) * sz,
        0.,
        p[0],
        p[1],
        p[2],
        1.,
    ]
}

/// Quaternion.setFromEuler for an XYZ rotation about X only.
pub(crate) fn rotation_x(angle: f64) -> [f64; 4] {
    let (c1, s1) = (cos(angle / 2.), sin(angle / 2.));
    let (c2, s2, c3, s3) = (cos(0. / 2.), sin(0. / 2.), cos(0. / 2.), sin(0. / 2.));
    [
        s1 * c2 * c3 + c1 * s2 * s3,
        c1 * s2 * c3 - s1 * c2 * s3,
        c1 * c2 * s3 + s1 * s2 * c3,
        c1 * c2 * c3 - s1 * s2 * s3,
    ]
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

/// Matrix4.determinant.
pub(crate) fn determinant(te: &M4) -> f64 {
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

/// Matrix3.getNormalMatrix: setFromMatrix4, invert, transpose.
pub(crate) fn normal_matrix(m: &M4) -> M3 {
    let te = [m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]];
    let [n11, n21, n31, n12, n22, n32, n13, n23, n33] = te;
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
    // transpose
    [
        inv[0], inv[3], inv[6], inv[1], inv[4], inv[7], inv[2], inv[5], inv[8],
    ]
}

/// Vector3.applyMatrix4 ( with the perspective divide ).
pub(crate) fn apply4(v: [f64; 3], e: &M4) -> [f64; 3] {
    let [x, y, z] = v;
    let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
    [
        (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
        (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
        (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
    ]
}

/// Vector3.normalize: divideScalar( length() || 1 ).
pub(crate) fn normalize(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let s = 1. / if l == 0. || l.is_nan() { 1. } else { l };
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Vector3.applyNormalMatrix: applyMatrix3 then normalize.
pub(crate) fn apply_normal(v: [f64; 3], e: &M3) -> [f64; 3] {
    let [x, y, z] = v;
    normalize([
        e[0] * x + e[3] * y + e[6] * z,
        e[1] * x + e[4] * y + e[7] * z,
        e[2] * x + e[5] * y + e[8] * z,
    ])
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
