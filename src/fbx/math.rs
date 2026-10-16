//! The three.js r186 math FBXLoader computes with, operation for operation,
//! so the loader's transforms and keyframes round as in the original.
//! Elementary functions are the host's `Math` in the browser ( see
//! `bounce::trig` ).

pub(crate) use crate::js_math;
use js_math::*;

/// MathUtils.DEG2RAD.
pub(crate) const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
pub(crate) fn deg_to_rad(d: f64) -> f64 {
    d * DEG2RAD
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl V3 {
    pub(crate) const ZERO: V3 = V3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub(crate) const ONE: V3 = V3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub(crate) fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub(crate) fn length_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub(crate) fn multiply_scalar(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
    /// normalize(): divideScalar( length || 1 ).
    pub(crate) fn normalize(self) -> Self {
        let l = self.length();
        let l = if l == 0.0 || l.is_nan() { 1.0 } else { l };
        self.multiply_scalar(1.0 / l)
    }
    pub(crate) fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    pub(crate) fn cross(a: Self, b: Self) -> Self {
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
    pub(crate) fn dot(self, o: Self) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub(crate) fn apply_matrix4(self, m: &M4) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let e = &m.0;
        let w = 1.0 / (e[3] * x + e[7] * y + e[11] * z + e[15]);
        Self::new(
            (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
            (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
            (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
        )
    }
    pub(crate) fn apply_matrix3(self, m: &M3) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let e = &m.0;
        Self::new(
            e[0] * x + e[3] * y + e[6] * z,
            e[1] * x + e[4] * y + e[7] * z,
            e[2] * x + e[5] * y + e[8] * z,
        )
    }
    pub(crate) fn from_matrix_column(m: &M4, i: usize) -> Self {
        Self::new(m.0[i * 4], m.0[i * 4 + 1], m.0[i * 4 + 2])
    }
    pub(crate) fn from_matrix_scale(m: &M4) -> Self {
        Self::new(
            Self::from_matrix_column(m, 0).length(),
            Self::from_matrix_column(m, 1).length(),
            Self::from_matrix_column(m, 2).length(),
        )
    }
    pub(crate) fn from_array(a: &[f64]) -> Self {
        let g = |i: usize| a.get(i).copied().unwrap_or(f64::NAN);
        Self::new(g(0), g(1), g(2))
    }
    pub(crate) fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Order {
    Xyz,
    Yxz,
    Zxy,
    Zyx,
    Yzx,
    Xzy,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Euler {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub order: Order,
}
impl Euler {
    pub(crate) fn new(x: f64, y: f64, z: f64, order: Order) -> Self {
        Self { x, y, z, order }
    }
    #[allow(unused_assignments)]
    pub(crate) fn from_rotation_matrix(m: &M4, order: Order) -> Self {
        let te = &m.0;
        let (m11, m12, m13) = (te[0], te[4], te[8]);
        let (m21, m22, m23) = (te[1], te[5], te[9]);
        let (m31, m32, m33) = (te[2], te[6], te[10]);
        let clamp = |v: f64| (-1.0f64).max(1.0f64.min(v));
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        match order {
            Order::Xyz => {
                y = asin(clamp(m13));
                if m13.abs() < 0.9999999 {
                    x = atan2(-m23, m33);
                    z = atan2(-m12, m11);
                } else {
                    x = atan2(m32, m22);
                    z = 0.0;
                }
            }
            Order::Yxz => {
                x = asin(-clamp(m23));
                if m23.abs() < 0.9999999 {
                    y = atan2(m13, m33);
                    z = atan2(m21, m22);
                } else {
                    y = atan2(-m31, m11);
                    z = 0.0;
                }
            }
            Order::Zxy => {
                x = asin(clamp(m32));
                if m32.abs() < 0.9999999 {
                    y = atan2(-m31, m33);
                    z = atan2(-m12, m22);
                } else {
                    y = 0.0;
                    z = atan2(m21, m11);
                }
            }
            Order::Zyx => {
                y = asin(-clamp(m31));
                if m31.abs() < 0.9999999 {
                    x = atan2(m32, m33);
                    z = atan2(m21, m11);
                } else {
                    x = 0.0;
                    z = atan2(-m12, m22);
                }
            }
            Order::Yzx => {
                z = asin(clamp(m21));
                if m21.abs() < 0.9999999 {
                    x = atan2(-m23, m22);
                    y = atan2(-m31, m11);
                } else {
                    x = 0.0;
                    y = atan2(m13, m33);
                }
            }
            Order::Xzy => {
                z = asin(-clamp(m12));
                if m12.abs() < 0.9999999 {
                    x = atan2(m32, m22);
                    y = atan2(m13, m11);
                } else {
                    x = atan2(-m23, m33);
                    y = 0.0;
                }
            }
        }
        Self { x, y, z, order }
    }
    pub(crate) fn from_quaternion(q: Q, order: Order) -> Self {
        Self::from_rotation_matrix(&M4::rotation_from_quaternion(q), order)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Q {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Q {
    pub(crate) const IDENTITY: Q = Q {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub(crate) fn from_euler(e: Euler) -> Self {
        let (x, y, z) = (e.x, e.y, e.z);
        let c1 = cos(x / 2.0);
        let c2 = cos(y / 2.0);
        let c3 = cos(z / 2.0);
        let s1 = sin(x / 2.0);
        let s2 = sin(y / 2.0);
        let s3 = sin(z / 2.0);
        let (qx, qy, qz, qw) = match e.order {
            Order::Xyz => (
                s1 * c2 * c3 + c1 * s2 * s3,
                c1 * s2 * c3 - s1 * c2 * s3,
                c1 * c2 * s3 + s1 * s2 * c3,
                c1 * c2 * c3 - s1 * s2 * s3,
            ),
            Order::Yxz => (
                s1 * c2 * c3 + c1 * s2 * s3,
                c1 * s2 * c3 - s1 * c2 * s3,
                c1 * c2 * s3 - s1 * s2 * c3,
                c1 * c2 * c3 + s1 * s2 * s3,
            ),
            Order::Zxy => (
                s1 * c2 * c3 - c1 * s2 * s3,
                c1 * s2 * c3 + s1 * c2 * s3,
                c1 * c2 * s3 + s1 * s2 * c3,
                c1 * c2 * c3 - s1 * s2 * s3,
            ),
            Order::Zyx => (
                s1 * c2 * c3 - c1 * s2 * s3,
                c1 * s2 * c3 + s1 * c2 * s3,
                c1 * c2 * s3 - s1 * s2 * c3,
                c1 * c2 * c3 + s1 * s2 * s3,
            ),
            Order::Yzx => (
                s1 * c2 * c3 + c1 * s2 * s3,
                c1 * s2 * c3 + s1 * c2 * s3,
                c1 * c2 * s3 - s1 * s2 * c3,
                c1 * c2 * c3 - s1 * s2 * s3,
            ),
            Order::Xzy => (
                s1 * c2 * c3 - c1 * s2 * s3,
                c1 * s2 * c3 - s1 * c2 * s3,
                c1 * c2 * s3 + s1 * s2 * c3,
                c1 * c2 * c3 + s1 * s2 * s3,
            ),
        };
        Self {
            x: qx,
            y: qy,
            z: qz,
            w: qw,
        }
    }
    pub(crate) fn from_rotation_matrix(m: &M4) -> Self {
        let te = &m.0;
        let (m11, m12, m13) = (te[0], te[4], te[8]);
        let (m21, m22, m23) = (te[1], te[5], te[9]);
        let (m31, m32, m33) = (te[2], te[6], te[10]);
        let trace = m11 + m22 + m33;
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Self {
                w: 0.25 / s,
                x: (m32 - m23) * s,
                y: (m13 - m31) * s,
                z: (m21 - m12) * s,
            }
        } else if m11 > m22 && m11 > m33 {
            let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
            Self {
                w: (m32 - m23) / s,
                x: 0.25 * s,
                y: (m12 + m21) / s,
                z: (m13 + m31) / s,
            }
        } else if m22 > m33 {
            let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
            Self {
                w: (m13 - m31) / s,
                x: (m12 + m21) / s,
                y: 0.25 * s,
                z: (m23 + m32) / s,
            }
        } else {
            let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
            Self {
                w: (m21 - m12) / s,
                x: (m13 + m31) / s,
                y: (m23 + m32) / s,
                z: 0.25 * s,
            }
        }
    }
    pub(crate) fn multiply(a: Q, b: Q) -> Q {
        let (qax, qay, qaz, qaw) = (a.x, a.y, a.z, a.w);
        let (qbx, qby, qbz, qbw) = (b.x, b.y, b.z, b.w);
        Q {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
    /// invert(): the conjugate.
    pub(crate) fn invert(self) -> Q {
        Q {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    pub(crate) fn dot(self, v: Q) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }
    pub(crate) fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub(crate) fn normalize(self) -> Q {
        let l = self.length();
        if l == 0.0 {
            Q::IDENTITY
        } else {
            let l = 1.0 / l;
            Q {
                x: self.x * l,
                y: self.y * l,
                z: self.z * l,
                w: self.w * l,
            }
        }
    }
    pub(crate) fn negate(self) -> Q {
        Q {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
    pub(crate) fn slerp(self, qb: Q, t: f64) -> Q {
        let (mut x, mut y, mut z, mut w) = (qb.x, qb.y, qb.z, qb.w);
        let mut dot = self.dot(qb);
        if dot < 0.0 {
            x = -x;
            y = -y;
            z = -z;
            w = -w;
            dot = -dot;
        }
        let mut s = 1.0 - t;
        let mut t = t;
        if dot < 0.9995 {
            let theta = acos(dot);
            let sine = sin(theta);
            s = sin(s * theta) / sine;
            t = sin(t * theta) / sine;
            Q {
                x: self.x * s + x * t,
                y: self.y * s + y * t,
                z: self.z * s + z * t,
                w: self.w * s + w * t,
            }
        } else {
            Q {
                x: self.x * s + x * t,
                y: self.y * s + y * t,
                z: self.z * s + z * t,
                w: self.w * s + w * t,
            }
            .normalize()
        }
    }
    pub(crate) fn to_array(self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

/// Matrix4 ( column-major elements ).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct M4(pub [f64; 16]);
impl Default for M4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}
impl M4 {
    pub(crate) const IDENTITY: M4 = M4([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);
    /// fromArray(): the first 16 values ( missing ones undefined → NaN ).
    pub(crate) fn from_array(a: &[f64]) -> Self {
        M4(std::array::from_fn(|i| {
            a.get(i).copied().unwrap_or(f64::NAN)
        }))
    }
    pub(crate) fn multiply_matrices(a: &M4, b: &M4) -> M4 {
        let ae = &a.0;
        let be = &b.0;
        let (a11, a12, a13, a14) = (ae[0], ae[4], ae[8], ae[12]);
        let (a21, a22, a23, a24) = (ae[1], ae[5], ae[9], ae[13]);
        let (a31, a32, a33, a34) = (ae[2], ae[6], ae[10], ae[14]);
        let (a41, a42, a43, a44) = (ae[3], ae[7], ae[11], ae[15]);
        let (b11, b12, b13, b14) = (be[0], be[4], be[8], be[12]);
        let (b21, b22, b23, b24) = (be[1], be[5], be[9], be[13]);
        let (b31, b32, b33, b34) = (be[2], be[6], be[10], be[14]);
        let (b41, b42, b43, b44) = (be[3], be[7], be[11], be[15]);
        let mut te = [0.0; 16];
        te[0] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        te[4] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        te[8] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        te[12] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;
        te[1] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        te[5] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        te[9] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        te[13] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;
        te[2] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        te[6] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        te[10] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        te[14] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;
        te[3] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        te[7] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        te[11] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        te[15] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;
        M4(te)
    }
    /// this.multiply( m ).
    pub(crate) fn multiply(&self, m: &M4) -> M4 {
        Self::multiply_matrices(self, m)
    }
    /// this.premultiply( m ).
    pub(crate) fn premultiply(&self, m: &M4) -> M4 {
        Self::multiply_matrices(m, self)
    }
    pub(crate) fn invert(&self) -> M4 {
        let te = &self.0;
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
        if det == 0.0 {
            return M4([0.0; 16]);
        }
        let d = 1.0 / det;
        M4([
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
        ])
    }
    pub(crate) fn compose(position: V3, q: Q, scale: V3) -> M4 {
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let (xx, xy, xz) = (x * x2, x * y2, x * z2);
        let (yy, yz, zz) = (y * y2, y * z2, z * z2);
        let (wx, wy, wz) = (w * x2, w * y2, w * z2);
        let (sx, sy, sz) = (scale.x, scale.y, scale.z);
        M4([
            (1.0 - (yy + zz)) * sx,
            (xy + wz) * sx,
            (xz - wy) * sx,
            0.0,
            (xy - wz) * sy,
            (1.0 - (xx + zz)) * sy,
            (yz + wx) * sy,
            0.0,
            (xz + wy) * sz,
            (yz - wx) * sz,
            (1.0 - (xx + yy)) * sz,
            0.0,
            position.x,
            position.y,
            position.z,
            1.0,
        ])
    }
    pub(crate) fn determinant_affine(&self) -> f64 {
        let te = &self.0;
        let (n11, n12, n13) = (te[0], te[4], te[8]);
        let (n21, n22, n23) = (te[1], te[5], te[9]);
        let (n31, n32, n33) = (te[2], te[6], te[10]);
        n11 * (n22 * n33 - n23 * n32) - n12 * (n21 * n33 - n23 * n31)
            + n13 * (n21 * n32 - n22 * n31)
    }
    /// decompose() into ( position, quaternion, scale ).
    pub(crate) fn decompose(&self) -> (V3, Q, V3) {
        let te = &self.0;
        let position = V3::new(te[12], te[13], te[14]);
        let det = self.determinant_affine();
        if det == 0.0 {
            return (position, Q::IDENTITY, V3::ONE);
        }
        let mut sx = V3::new(te[0], te[1], te[2]).length();
        let sy = V3::new(te[4], te[5], te[6]).length();
        let sz = V3::new(te[8], te[9], te[10]).length();
        if det < 0.0 {
            sx = -sx;
        }
        let mut m = *self;
        let (ix, iy, iz) = (1.0 / sx, 1.0 / sy, 1.0 / sz);
        m.0[0] *= ix;
        m.0[1] *= ix;
        m.0[2] *= ix;
        m.0[4] *= iy;
        m.0[5] *= iy;
        m.0[6] *= iy;
        m.0[8] *= iz;
        m.0[9] *= iz;
        m.0[10] *= iz;
        (position, Q::from_rotation_matrix(&m), V3::new(sx, sy, sz))
    }
    pub(crate) fn rotation_from_euler(e: Euler) -> M4 {
        let mut te = [0.0; 16];
        let (x, y, z) = (e.x, e.y, e.z);
        let (a, b) = (cos(x), sin(x));
        let (c, d) = (cos(y), sin(y));
        let (ee, f) = (cos(z), sin(z));
        match e.order {
            Order::Xyz => {
                let (ae, af, be, bf) = (a * ee, a * f, b * ee, b * f);
                te[0] = c * ee;
                te[4] = -c * f;
                te[8] = d;
                te[1] = af + be * d;
                te[5] = ae - bf * d;
                te[9] = -b * c;
                te[2] = bf - ae * d;
                te[6] = be + af * d;
                te[10] = a * c;
            }
            Order::Yxz => {
                let (ce, cf, de, df) = (c * ee, c * f, d * ee, d * f);
                te[0] = ce + df * b;
                te[4] = de * b - cf;
                te[8] = a * d;
                te[1] = a * f;
                te[5] = a * ee;
                te[9] = -b;
                te[2] = cf * b - de;
                te[6] = df + ce * b;
                te[10] = a * c;
            }
            Order::Zxy => {
                let (ce, cf, de, df) = (c * ee, c * f, d * ee, d * f);
                te[0] = ce - df * b;
                te[4] = -a * f;
                te[8] = de + cf * b;
                te[1] = cf + de * b;
                te[5] = a * ee;
                te[9] = df - ce * b;
                te[2] = -a * d;
                te[6] = b;
                te[10] = a * c;
            }
            Order::Zyx => {
                let (ae, af, be, bf) = (a * ee, a * f, b * ee, b * f);
                te[0] = c * ee;
                te[4] = be * d - af;
                te[8] = ae * d + bf;
                te[1] = c * f;
                te[5] = bf * d + ae;
                te[9] = af * d - be;
                te[2] = -d;
                te[6] = b * c;
                te[10] = a * c;
            }
            Order::Yzx => {
                let (ac, ad, bc, bd) = (a * c, a * d, b * c, b * d);
                te[0] = c * ee;
                te[4] = bd - ac * f;
                te[8] = bc * f + ad;
                te[1] = f;
                te[5] = a * ee;
                te[9] = -b * ee;
                te[2] = -d * ee;
                te[6] = ad * f + bc;
                te[10] = ac - bd * f;
            }
            Order::Xzy => {
                let (ac, ad, bc, bd) = (a * c, a * d, b * c, b * d);
                te[0] = c * ee;
                te[4] = -f;
                te[8] = d * ee;
                te[1] = ac * f + bd;
                te[5] = a * ee;
                te[9] = ad * f - bc;
                te[2] = bc * f - ad;
                te[6] = b * ee;
                te[10] = bd * f + ac;
            }
        }
        te[15] = 1.0;
        M4(te)
    }
    pub(crate) fn rotation_from_quaternion(q: Q) -> M4 {
        Self::compose(V3::ZERO, q, V3::ONE)
    }
    pub(crate) fn extract_rotation(m: &M4) -> M4 {
        if m.determinant_affine() == 0.0 {
            return M4::IDENTITY;
        }
        let me = &m.0;
        let sx = 1.0 / V3::from_matrix_column(m, 0).length();
        let sy = 1.0 / V3::from_matrix_column(m, 1).length();
        let sz = 1.0 / V3::from_matrix_column(m, 2).length();
        M4([
            me[0] * sx,
            me[1] * sx,
            me[2] * sx,
            0.0,
            me[4] * sy,
            me[5] * sy,
            me[6] * sy,
            0.0,
            me[8] * sz,
            me[9] * sz,
            me[10] * sz,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
    pub(crate) fn set_position(mut self, v: V3) -> M4 {
        self.0[12] = v.x;
        self.0[13] = v.y;
        self.0[14] = v.z;
        self
    }
    pub(crate) fn copy_position(mut self, m: &M4) -> M4 {
        self.0[12] = m.0[12];
        self.0[13] = m.0[13];
        self.0[14] = m.0[14];
        self
    }
    pub(crate) fn scale(mut self, v: V3) -> M4 {
        let te = &mut self.0;
        for i in [0, 1, 2, 3] {
            te[i] *= v.x;
            te[i + 4] *= v.y;
            te[i + 8] *= v.z;
        }
        self
    }
    /// lookAt( eye, target, up ): the rotation part only.
    pub(crate) fn look_at(mut self, eye: V3, target: V3, up: V3) -> M4 {
        let mut z = eye.sub(target);
        if z.length_sq() == 0.0 {
            z.z = 1.0;
        }
        z = z.normalize();
        let mut x = V3::cross(up, z);
        if x.length_sq() == 0.0 {
            if up.z.abs() == 1.0 {
                z.x += 0.0001;
            } else {
                z.z += 0.0001;
            }
            z = z.normalize();
            x = V3::cross(up, z);
        }
        x = x.normalize();
        let y = V3::cross(z, x);
        let te = &mut self.0;
        te[0] = x.x;
        te[4] = y.x;
        te[8] = z.x;
        te[1] = x.y;
        te[5] = y.y;
        te[9] = z.y;
        te[2] = x.z;
        te[6] = y.z;
        te[10] = z.z;
        self
    }
}

/// Matrix3 ( column-major ).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct M3(pub [f64; 9]);
impl M3 {
    /// getNormalMatrix( m ): the inverse transpose of the upper 3×3.
    pub(crate) fn normal_matrix(m: &M4) -> M3 {
        let me = &m.0;
        // setFromMatrix4 ( set() takes row-major arguments ).
        let te = [
            me[0], me[1], me[2], me[4], me[5], me[6], me[8], me[9], me[10],
        ];
        let (n11, n21, n31) = (te[0], te[1], te[2]);
        let (n12, n22, n32) = (te[3], te[4], te[5]);
        let (n13, n23, n33) = (te[6], te[7], te[8]);
        let t11 = n33 * n22 - n32 * n23;
        let t12 = n32 * n13 - n33 * n12;
        let t13 = n23 * n12 - n22 * n13;
        let det = n11 * t11 + n21 * t12 + n31 * t13;
        let inv = if det == 0.0 {
            [0.0; 9]
        } else {
            let d = 1.0 / det;
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
        // transpose()
        M3([
            inv[0], inv[3], inv[6], inv[1], inv[4], inv[7], inv[2], inv[5], inv[8],
        ])
    }
}

/// ColorManagement's SRGBToLinear.
pub(crate) fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        pow(c * 0.9478672986 + 0.0521327014, 2.4)
    }
}

/// `Math.fround`.
pub(crate) fn fround(v: f64) -> f64 {
    f64::from(v as f32)
}

#[allow(dead_code)]
pub(crate) fn js_atan(v: f64) -> f64 {
    atan(v)
}
