//! bounce's `Vec3`, `Quat`, `Mat3`, `Mat4`, `Isometry` and `Aabb`, operation
//! for operation: every result is the double the JS engine computes. `sin`
//! and `cos` are correctly rounded, as V8's are, and `Math.min` /
//! `Math.max` keep JS's NaN and signed-zero rules.

/// `Math.min( a, b )`.
pub fn min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    if a == 0. && b == 0. {
        return if a.is_sign_negative() || b.is_sign_negative() {
            -0.
        } else {
            0.
        };
    }
    if a < b { a } else { b }
}

/// `Math.max( a, b )`.
pub fn max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    if a == 0. && b == 0. {
        return if a.is_sign_positive() || b.is_sign_positive() {
            0.
        } else {
            -0.
        };
    }
    if a > b { a } else { b }
}

/// scalarHelpers `clamp( value, min, max )`.
pub fn clamp(value: f64, lo: f64, hi: f64) -> f64 {
    min(max(lo, value), hi)
}

pub fn squared(v: f64) -> f64 {
    v * v
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.)
}

pub fn sin(v: f64) -> f64 {
    super::trig::sin(v)
}

pub fn cos(v: f64) -> f64 {
    super::trig::cos(v)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn add_vector(&mut self, a: Vec3) -> &mut Self {
        self.x += a.x;
        self.y += a.y;
        self.z += a.z;
        self
    }
    pub fn add_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
        self
    }
    pub fn subtract_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self.z = a.z - b.z;
        self
    }
    pub fn scale(&mut self, s: f64) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self
    }
    pub fn scale_vector(&mut self, o: Vec3, s: f64) -> &mut Self {
        self.x = o.x * s;
        self.y = o.y * s;
        self.z = o.z * s;
        self
    }
    pub fn add_scaled(&mut self, o: Vec3, s: f64) -> &mut Self {
        self.x += o.x * s;
        self.y += o.y * s;
        self.z += o.z * s;
        self
    }
    pub fn add_scaled_to_vector(&mut self, a: Vec3, b: Vec3, s: f64) -> &mut Self {
        self.x = a.x + b.x * s;
        self.y = a.y + b.y * s;
        self.z = a.z + b.z * s;
        self
    }
    pub fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
    pub fn negate_vector(&mut self, a: Vec3) -> &mut Self {
        self.x = -a.x;
        self.y = -a.y;
        self.z = -a.z;
        self
    }
    /// `normalize()`: `x * ( 1 / sqrt( length² ) )`.
    pub fn normalize(&mut self) -> &mut Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let mut length = x * x + y * y + z * z;
        if length > 0. {
            length = 1. / length.sqrt();
        }
        self.x *= length;
        self.y *= length;
        self.z *= length;
        self
    }
    pub fn normalize_vector(&mut self, a: Vec3) -> &mut Self {
        let (x, y, z) = (a.x, a.y, a.z);
        let mut length = x * x + y * y + z * z;
        if length > 0. {
            length = 1. / length.sqrt();
        }
        self.x = x * length;
        self.y = y * length;
        self.z = z * length;
        self
    }
    pub fn cross_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        let (ax, ay, az) = (a.x, a.y, a.z);
        let (bx, by, bz) = (b.x, b.y, b.z);
        self.x = ay * bz - az * by;
        self.y = az * bx - ax * bz;
        self.z = ax * by - ay * bx;
        self
    }
    pub fn zero(&mut self) -> &mut Self {
        *self = Vec3::ZERO;
        self
    }
    pub fn length(&self) -> f64 {
        let (x, y, z) = (self.x, self.y, self.z);
        (x * x + y * y + z * z).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        let (x, y, z) = (self.x, self.y, self.z);
        x * x + y * y + z * z
    }
    pub fn dot(&self, o: Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn transform_vector_from_mat3(&mut self, v: Vec3, m: &Mat3) -> &mut Self {
        let (x, y, z) = (v.x, v.y, v.z);
        self.x = x * m.e[0] + y * m.e[3] + z * m.e[6];
        self.y = x * m.e[1] + y * m.e[4] + z * m.e[7];
        self.z = x * m.e[2] + y * m.e[5] + z * m.e[8];
        self
    }
    /// `transformVectorFromMat4` / `transformByMat4`: with the perspective
    /// divide `w || 1`.
    pub fn transform_vector_from_mat4(&mut self, v: Vec3, m: &Mat4) -> &mut Self {
        let (x, y, z) = (v.x, v.y, v.z);
        let e = &m.e;
        let mut w = e[3] * x + e[7] * y + e[11] * z + e[15];
        if w == 0. || w.is_nan() {
            w = 1.;
        }
        self.x = (e[0] * x + e[4] * y + e[8] * z + e[12]) / w;
        self.y = (e[1] * x + e[5] * y + e[9] * z + e[13]) / w;
        self.z = (e[2] * x + e[6] * y + e[10] * z + e[14]) / w;
        self
    }
    pub fn transform_by_mat4(&mut self, m: &Mat4) -> &mut Self {
        let v = *self;
        self.transform_vector_from_mat4(v, m)
    }
    pub fn transform_vector_by_quat(&mut self, a: Vec3, q: Quat) -> &mut Self {
        let (qx, qy, qz, qw) = (q.x, q.y, q.z, q.w);
        let (x, y, z) = (a.x, a.y, a.z);
        let uvx = qy * z - qz * y;
        let uvy = qz * x - qx * z;
        let uvz = qx * y - qy * x;
        let uuvx = qy * uvz - qz * uvy;
        let uuvy = qz * uvx - qx * uvz;
        let uuvz = qx * uvy - qy * uvx;
        let w2 = qw * 2.;
        self.x = x + w2 * uvx + 2. * uuvx;
        self.y = y + w2 * uvy + 2. * uuvy;
        self.z = z + w2 * uvz + 2. * uuvz;
        self
    }
    pub fn transform_by_quat(&mut self, q: Quat) -> &mut Self {
        let v = *self;
        self.transform_vector_by_quat(v, q)
    }
    pub fn replicate(&mut self, v: f64) {
        self.x = v;
        self.y = v;
        self.z = v;
    }
    pub fn component(&self, index: usize) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
    pub fn compute_normalized_perpendicular(&mut self, v: Vec3) {
        if v.x.abs() > v.y.abs() {
            let len = (v.x * v.x + v.z * v.z).sqrt();
            self.x = v.z / len;
            self.y = 0.;
            self.z = -v.x / len;
        } else {
            let len = (v.y * v.y + v.z * v.z).sqrt();
            self.x = 0.;
            self.y = v.z / len;
            self.z = -v.y / len;
        }
    }
    /// `isClose( other, tolerance )`: the squared distance within tolerance.
    pub fn is_close(&self, o: Vec3, tolerance: f64) -> bool {
        squared_distance(*self, o) <= tolerance
    }
    pub fn is_near_zero(&self) -> bool {
        self.squared_length() <= 1e-6
    }
    pub fn is_near_zero_with(&self, squared_tolerance: f64) -> bool {
        self.squared_length() <= squared_tolerance
    }
    pub fn average_of_vectors(&mut self, a: Vec3, b: Vec3) {
        self.x = (a.x + b.x) * 0.5;
        self.y = (a.y + b.y) * 0.5;
        self.z = (a.z + b.z) * 0.5;
    }
    pub fn add_scalar_to_vector(&mut self, a: Vec3, s: f64) -> &mut Self {
        self.x = a.x + s;
        self.y = a.y + s;
        self.z = a.z + s;
        self
    }
    pub fn squared_distance(&self, b: Vec3) -> f64 {
        let x = b.x - self.x;
        let y = b.y - self.y;
        let z = b.z - self.z;
        x * x + y * y + z * z
    }
    pub fn not_equals(&self, o: Vec3) -> bool {
        let t = 1e-6;
        (self.x - o.x).abs() > t || (self.y - o.y).abs() > t || (self.z - o.z).abs() > t
    }
    pub fn min_vector(&mut self, a: Vec3) -> &mut Self {
        if a.x < self.x {
            self.x = a.x;
        }
        if a.y < self.y {
            self.y = a.y;
        }
        if a.z < self.z {
            self.z = a.z;
        }
        self
    }
    pub fn min_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        self.x = min(a.x, b.x);
        self.y = min(a.y, b.y);
        self.z = min(a.z, b.z);
        self
    }
    pub fn max_vector(&mut self, a: Vec3) -> &mut Self {
        if a.x > self.x {
            self.x = a.x;
        }
        if a.y > self.y {
            self.y = a.y;
        }
        if a.z > self.z {
            self.z = a.z;
        }
        self
    }
    pub fn max_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        self.x = max(a.x, b.x);
        self.y = max(a.y, b.y);
        self.z = max(a.z, b.z);
        self
    }
    pub fn multiply_vectors(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self.z = a.z * b.z;
        self
    }
}

/// `squaredDistance( a, b )`.
pub fn squared_distance(a: Vec3, b: Vec3) -> f64 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let z = b.z - a.z;
    x * x + y * y + z * z
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for Quat {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }
}

impl Quat {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    /// `normalize()`: a division by the length.
    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();
        if length > 0. {
            self.x /= length;
            self.y /= length;
            self.z /= length;
            self.w /= length;
        }
        self
    }
    pub fn identity(&mut self) {
        *self = Quat::default();
    }
    pub fn conjugate_quat(&mut self, a: Quat) -> &mut Self {
        self.x = -a.x;
        self.y = -a.y;
        self.z = -a.z;
        self.w = a.w;
        self
    }
    pub fn multiply_quats(&mut self, a: Quat, b: Quat) -> &mut Self {
        let (ax, ay, az, aw) = (a.x, a.y, a.z, a.w);
        let (bx, by, bz, bw) = (b.x, b.y, b.z, b.w);
        self.x = ax * bw + aw * bx + ay * bz - az * by;
        self.y = ay * bw + aw * by + az * bx - ax * bz;
        self.z = az * bw + aw * bz + ax * by - ay * bx;
        self.w = aw * bw - ax * bx - ay * by - az * bz;
        self
    }
    /// `normalizeQuat( a )`: a multiplication by the inverse length.
    pub fn normalize_quat(&mut self, a: Quat) -> &mut Self {
        let (x, y, z, w) = (a.x, a.y, a.z, a.w);
        let mut len = x * x + y * y + z * z + w * w;
        if len > 0. {
            len = 1. / len.sqrt();
        }
        self.x = x * len;
        self.y = y * len;
        self.z = z * len;
        self.w = w * len;
        self
    }
    pub fn set_axis_angle(&mut self, axis: Vec3, angle: f64) -> &mut Self {
        let angle = angle * 0.5;
        let s = sin(angle);
        self.x = s * axis.x;
        self.y = s * axis.y;
        self.z = s * axis.z;
        self.w = cos(angle);
        self
    }
    pub fn dot(&self, b: Quat) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
    pub fn not_equals(&self, o: Quat) -> bool {
        let t = 1e-6;
        (self.x - o.x).abs() >= t
            || (self.y - o.y).abs() >= t
            || (self.z - o.z).abs() >= t
            || (self.w - o.w).abs() >= t
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat3 {
    pub e: [f64; 9],
}

impl Mat3 {
    pub fn zero(&mut self) {
        self.e = [0.; 9];
    }
    /// `fromMat4( a )`: the upper-left 3 × 3.
    pub fn from_mat4(&mut self, a: &Mat4) -> &mut Self {
        let m = &a.e;
        self.e = [m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]];
        self
    }
    pub fn transpose(&mut self) -> &mut Self {
        let e = &mut self.e;
        let (a01, a02, a12) = (e[1], e[2], e[5]);
        e[1] = e[3];
        e[2] = e[6];
        e[3] = a01;
        e[5] = e[7];
        e[6] = a02;
        e[7] = a12;
        self
    }
    pub fn transpose_matrix(&mut self, a: &Mat3) -> &mut Self {
        let m = &a.e;
        self.e = [m[0], m[3], m[6], m[1], m[4], m[7], m[2], m[5], m[8]];
        self
    }
    /// `invert()`; a zero determinant leaves the matrix ( JS returns null ).
    pub fn invert(&mut self) -> &mut Self {
        let a = self.e;
        self.invert_from(&a);
        self
    }
    pub fn invert_mat3(&mut self, a: &Mat3) -> &mut Self {
        let a = a.e;
        self.invert_from(&a);
        self
    }
    fn invert_from(&mut self, a: &[f64; 9]) {
        let (a00, a01, a02, a10, a11, a12, a20, a21, a22) =
            (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]);
        let b01 = a22 * a11 - a12 * a21;
        let b11 = -a22 * a10 + a12 * a20;
        let b21 = a21 * a10 - a11 * a20;
        let mut det = a00 * b01 + a01 * b11 + a02 * b21;
        if det == 0. || det.is_nan() {
            return;
        }
        det = 1. / det;
        self.e = [
            b01 * det,
            (-a22 * a01 + a02 * a21) * det,
            (a12 * a01 - a02 * a11) * det,
            b11 * det,
            (a22 * a00 - a02 * a20) * det,
            (-a12 * a00 + a02 * a10) * det,
            b21 * det,
            (-a21 * a00 + a01 * a20) * det,
            (a11 * a00 - a01 * a10) * det,
        ];
    }
    pub fn multiply_mat3s(&mut self, a: &Mat3, b: &Mat3) -> &mut Self {
        let a = a.e;
        let b = b.e;
        let (a00, a01, a02, a10, a11, a12, a20, a21, a22) =
            (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]);
        let (b00, b01, b02, b10, b11, b12, b20, b21, b22) =
            (b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8]);
        self.e = [
            b00 * a00 + b01 * a10 + b02 * a20,
            b00 * a01 + b01 * a11 + b02 * a21,
            b00 * a02 + b01 * a12 + b02 * a22,
            b10 * a00 + b11 * a10 + b12 * a20,
            b10 * a01 + b11 * a11 + b12 * a21,
            b10 * a02 + b11 * a12 + b12 * a22,
            b20 * a00 + b21 * a10 + b22 * a20,
            b20 * a01 + b21 * a11 + b22 * a21,
            b20 * a02 + b21 * a12 + b22 * a22,
        ];
        self
    }
    pub fn from_quat(&mut self, q: Quat) -> &mut Self {
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        let e = &mut self.e;
        e[0] = 1. - yy - zz;
        e[3] = yx - wz;
        e[6] = zx + wy;
        e[1] = yx + wz;
        e[4] = 1. - xx - zz;
        e[7] = zy - wx;
        e[2] = zx - wy;
        e[5] = zy + wx;
        e[8] = 1. - xx - yy;
        self
    }
}

/// `transformTensor( out, tensor, orientation )`.
pub fn transform_tensor(out: &mut Mat3, tensor: &Mat3, orientation: Quat) {
    let mut normalized = Quat::default();
    normalized.normalize_quat(orientation);
    let mut local_to_world = Mat3::default();
    local_to_world.from_quat(normalized);
    let mut world_to_local = Mat3::default();
    world_to_local.invert_mat3(&local_to_world);
    out.multiply_mat3s(&local_to_world, tensor);
    let o = *out;
    out.multiply_mat3s(&o, &world_to_local);
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat4 {
    pub e: [f64; 16],
}

impl Mat4 {
    pub fn multiply_3x3(&self, a: Vec3) -> Vec3 {
        let e = &self.e;
        let (x, y, z) = (a.x, a.y, a.z);
        Vec3::new(
            e[0] * x + e[4] * y + e[8] * z,
            e[1] * x + e[5] * y + e[9] * z,
            e[2] * x + e[6] * y + e[10] * z,
        )
    }
    /// `multiply3x3Transposed( out, a )`: through the transposed Mat3.
    pub fn multiply_3x3_transposed(&self, a: Vec3) -> Vec3 {
        let mut m = Mat3::default();
        m.from_mat4(self);
        m.transpose();
        let mut out = Vec3::ZERO;
        out.transform_vector_from_mat3(a, &m);
        out
    }
    pub fn get_translation(&self) -> Vec3 {
        Vec3::new(self.e[12], self.e[13], self.e[14])
    }
    pub fn set_translation(&mut self, v: Vec3) {
        self.e[12] = v.x;
        self.e[13] = v.y;
        self.e[14] = v.z;
        self.e[15] = 1.;
    }
    pub fn post_translated(&mut self, t: Vec3) -> &mut Self {
        self.e[12] += t.x;
        self.e[13] += t.y;
        self.e[14] += t.z;
        self.e[15] = 1.;
        self
    }
    pub fn from_rotation_translation(&mut self, q: Quat, v: Vec3) -> &mut Self {
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        self.e = [
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
            v.x,
            v.y,
            v.z,
            1.,
        ];
        self
    }
    pub fn from_quat(&mut self, q: Quat) -> &mut Self {
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        self.e = [
            1. - yy - zz,
            yx + wz,
            zx - wy,
            0.,
            yx - wz,
            1. - xx - zz,
            zy + wx,
            0.,
            zx + wy,
            zy - wx,
            1. - xx - yy,
            0.,
            0.,
            0.,
            0.,
            1.,
        ];
        self
    }
    pub fn from_inverse_rotation_and_translation(
        &mut self,
        rotation: Quat,
        translation: Vec3,
    ) -> &mut Self {
        let mut conjugated = Quat::default();
        conjugated.conjugate_quat(rotation);
        self.from_quat(conjugated);
        let mut rotated = self.multiply_3x3(translation);
        rotated.negate();
        self.set_translation(rotated);
        self
    }
    /// `invertMatrix( a )`; a zero determinant leaves the matrix.
    pub fn invert_matrix(&mut self, a: &Mat4) -> &mut Self {
        let m = a.e;
        let (a00, a01, a02, a03) = (m[0], m[1], m[2], m[3]);
        let (a10, a11, a12, a13) = (m[4], m[5], m[6], m[7]);
        let (a20, a21, a22, a23) = (m[8], m[9], m[10], m[11]);
        let (a30, a31, a32, a33) = (m[12], m[13], m[14], m[15]);
        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det == 0. || det.is_nan() {
            return self;
        }
        det = 1. / det;
        self.e = [
            (a11 * b11 - a12 * b10 + a13 * b09) * det,
            (a02 * b10 - a01 * b11 - a03 * b09) * det,
            (a31 * b05 - a32 * b04 + a33 * b03) * det,
            (a22 * b04 - a21 * b05 - a23 * b03) * det,
            (a12 * b08 - a10 * b11 - a13 * b07) * det,
            (a00 * b11 - a02 * b08 + a03 * b07) * det,
            (a32 * b02 - a30 * b05 - a33 * b01) * det,
            (a20 * b05 - a22 * b02 + a23 * b01) * det,
            (a10 * b10 - a11 * b08 + a13 * b06) * det,
            (a01 * b08 - a00 * b10 - a03 * b06) * det,
            (a30 * b04 - a31 * b02 + a33 * b00) * det,
            (a21 * b02 - a20 * b04 - a23 * b00) * det,
            (a11 * b07 - a10 * b09 - a12 * b06) * det,
            (a00 * b09 - a01 * b07 + a02 * b06) * det,
            (a31 * b01 - a30 * b03 - a32 * b00) * det,
            (a20 * b03 - a21 * b01 + a22 * b00) * det,
        ];
        self
    }
    pub fn column3(&self, index: usize) -> Vec3 {
        let e = &self.e;
        match index {
            0 => Vec3::new(e[0], e[1], e[2]),
            1 => Vec3::new(e[4], e[5], e[6]),
            _ => Vec3::new(e[8], e[9], e[10]),
        }
    }
    pub fn multiply_matrices(&mut self, a: &Mat4, b: &Mat4) -> &mut Self {
        let a = a.e;
        let b = b.e;
        let mut out = [0.; 16];
        for col in 0..4 {
            let (b0, b1, b2, b3) = (b[col * 4], b[col * 4 + 1], b[col * 4 + 2], b[col * 4 + 3]);
            for row in 0..4 {
                out[col * 4 + row] =
                    b0 * a[row] + b1 * a[4 + row] + b2 * a[8 + row] + b3 * a[12 + row];
            }
        }
        self.e = out;
        self
    }
    pub fn identity(&mut self) -> &mut Self {
        self.e = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ];
        self
    }
}

/// `Isometry.fromRotationAndTranslation( rotation, translation )`.
pub fn isometry(rotation: Quat, translation: Vec3) -> Mat4 {
    let mut m = Mat4::default();
    m.from_rotation_translation(rotation, translation);
    m
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
    pub centroid: Vec3,
}

impl Aabb {
    pub fn compute_support(&self, d: Vec3) -> Vec3 {
        Vec3::new(
            if d.x < 0. { self.min.x } else { self.max.x },
            if d.y < 0. { self.min.y } else { self.max.y },
            if d.z < 0. { self.min.z } else { self.max.z },
        )
    }
    pub fn compute_centroid(&mut self) {
        self.centroid = Vec3::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            (self.min.z + self.max.z) * 0.5,
        );
    }
    pub fn translate_aabb(&mut self, aabb: &Aabb, t: Vec3) -> &mut Self {
        self.min.add_vectors(aabb.min, t);
        self.max.add_vectors(aabb.max, t);
        self.compute_centroid();
        self
    }
    pub fn transform_aabb(&mut self, aabb: &Aabb, transform: &Mat4) -> &mut Self {
        let mut new_min = transform.get_translation();
        let mut new_max = transform.get_translation();
        for c in 0..3 {
            let col = transform.column3(c);
            let mut a = Vec3::ZERO;
            let mut b = Vec3::ZERO;
            a.scale_vector(col, aabb.min.component(c));
            b.scale_vector(col, aabb.max.component(c));
            new_min.x += min(a.x, b.x);
            new_min.y += min(a.y, b.y);
            new_min.z += min(a.z, b.z);
            new_max.x += max(a.x, b.x);
            new_max.y += max(a.y, b.y);
            new_max.z += max(a.z, b.z);
        }
        self.min = new_min;
        self.max = new_max;
        self.compute_centroid();
        self
    }
    pub fn intersects_aabb(&self, o: &Aabb) -> bool {
        self.min.x <= o.max.x
            && self.max.x >= o.min.x
            && self.min.y <= o.max.y
            && self.max.y >= o.min.y
            && self.min.z <= o.max.z
            && self.max.z >= o.min.z
    }
    pub fn union_aabb(&mut self, o: &Aabb) -> &mut Self {
        let (a, b) = (self.min, self.max);
        self.min.min_vectors(a, o.min);
        self.max.max_vectors(b, o.max);
        self.compute_centroid();
        self
    }
    pub fn union_aabbs(&mut self, a: &Aabb, b: &Aabb) -> &mut Self {
        self.min.min_vectors(a.min, b.min);
        self.max.max_vectors(a.max, b.max);
        self.compute_centroid();
        self
    }
    pub fn expand_aabb(&mut self, a: &Aabb, v: f64) -> &mut Self {
        self.min = Vec3::new(a.min.x - v, a.min.y - v, a.min.z - v);
        self.max = Vec3::new(a.max.x + v, a.max.y + v, a.max.z + v);
        self.compute_centroid();
        self
    }
    pub fn surface_area(&self) -> f64 {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        2. * (dx * dy + dy * dz + dz * dx)
    }
    pub fn largest_axis(&self) -> usize {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        if dx >= dy && dx >= dz {
            return 0;
        }
        if dy >= dx && dy >= dz {
            return 1;
        }
        2
    }
    /// `expandToPoint( point )`: the centroid is left as it was.
    pub fn expand_to_point(&mut self, p: Vec3) {
        self.min.min_vector(p);
        self.max.max_vector(p);
    }
    pub fn encloses_aabb(&self, o: &Aabb) -> bool {
        self.min.x <= o.min.x
            && self.max.x >= o.max.x
            && self.min.y <= o.min.y
            && self.max.y >= o.max.y
            && self.min.z <= o.min.z
            && self.max.z >= o.max.z
    }
    /// `computeSupportingFace( out, inDirection )`: up to four vertices.
    pub fn supporting_face(&self, d: Vec3, out: &mut Vec<Vec3>) {
        let (ax, ay, az) = (d.x.abs(), d.y.abs(), d.z.abs());
        let axis = if ax > ay {
            if ax > az { 0 } else { 2 }
        } else if ay > az {
            1
        } else {
            2
        };
        let (n, x) = (self.min, self.max);
        let v = Vec3::new;
        let face: [Vec3; 4] = if d.component(axis) < 0. {
            match axis {
                0 => [
                    v(x.x, n.y, n.z),
                    v(x.x, x.y, n.z),
                    v(x.x, x.y, x.z),
                    v(x.x, n.y, x.z),
                ],
                1 => [
                    v(n.x, x.y, n.z),
                    v(n.x, x.y, x.z),
                    v(x.x, x.y, x.z),
                    v(x.x, x.y, n.z),
                ],
                _ => [
                    v(n.x, n.y, x.z),
                    v(x.x, n.y, x.z),
                    v(x.x, x.y, x.z),
                    v(n.x, x.y, x.z),
                ],
            }
        } else {
            match axis {
                0 => [
                    v(n.x, n.y, n.z),
                    v(n.x, n.y, x.z),
                    v(n.x, x.y, x.z),
                    v(n.x, x.y, n.z),
                ],
                1 => [
                    v(n.x, n.y, n.z),
                    v(x.x, n.y, n.z),
                    v(x.x, n.y, x.z),
                    v(n.x, n.y, x.z),
                ],
                _ => [
                    v(n.x, n.y, n.z),
                    v(n.x, x.y, n.z),
                    v(x.x, x.y, n.z),
                    v(x.x, n.y, n.z),
                ],
            }
        };
        out.extend(face);
    }
}
