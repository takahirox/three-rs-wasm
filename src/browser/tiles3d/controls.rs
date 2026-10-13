//! 3d-tiles-renderer 0.4.27's GlobeControls ( over EnvironmentControls,
//! PointerTracker and the controls utils ) for the page's perspective camera
//! with enableDamping, in three's f64 math: drag rotates the globe around its
//! center, right / shift drag rotates around the hit point, the wheel zooms
//! toward the pointer, and adjustCamera fits near / far to the globe. The
//! raycasts go through the tiles ( raycastTraverseFirstHit ) and fall back
//! to the ellipsoid. Flight keys stay off ( enableFlight's default ), and the
//! gallery hands the controls one mouse pointer, so the two-finger touch
//! gestures are not reachable.
use super::ellipsoid::Ellipsoid;
use super::volume::{
    M4, V3, apply4, cross, dot, invert, length, multiply, normalize, transform_direction,
};

pub(crate) type Q = [f64; 4];
const IDENTITY_Q: Q = [0., 0., 0., 1.];

fn sin(x: f64) -> f64 {
    crate::bounce::trig::sin(x)
}
fn cos(x: f64) -> f64 {
    crate::bounce::trig::cos(x)
}
fn acos(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::acos(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.acos()
}
fn tan(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::tan(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.tan()
}
fn atan(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::atan(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.atan()
}
fn sign(x: f64) -> f64 {
    if x > 0. {
        1.
    } else if x < 0. {
        -1.
    } else {
        x
    }
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn add_scaled(a: V3, b: V3, s: f64) -> V3 {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
fn length_sq(a: V3) -> f64 {
    dot(a, a)
}
fn distance(a: V3, b: V3) -> f64 {
    length(sub(a, b))
}
fn lerp3(a: V3, b: V3, t: f64) -> V3 {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}
fn map_linear(x: f64, a1: f64, a2: f64, b1: f64, b2: f64) -> f64 {
    b1 + (x - a1) * (b2 - b1) / (a2 - a1)
}
fn lerp(x: f64, y: f64, t: f64) -> f64 {
    (1. - t) * x + t * y
}
/// Vector3.angleTo.
fn angle_to(a: V3, b: V3) -> f64 {
    let denominator = (length_sq(a) * length_sq(b)).sqrt();
    if denominator == 0. {
        return std::f64::consts::PI / 2.;
    }
    acos((dot(a, b) / denominator).clamp(-1., 1.))
}

fn q_normalize(q: Q) -> Q {
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if l == 0. {
        return IDENTITY_Q;
    }
    let l = 1. / l;
    [q[0] * l, q[1] * l, q[2] * l, q[3] * l]
}
/// Quaternion.setFromAxisAngle.
fn q_axis_angle(axis: V3, angle: f64) -> Q {
    let half = angle / 2.;
    let s = sin(half);
    [axis[0] * s, axis[1] * s, axis[2] * s, cos(half)]
}
/// Quaternion.setFromUnitVectors.
fn q_unit_vectors(from: V3, to: V3) -> Q {
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
    q_normalize(q)
}
/// Quaternion.multiplyQuaternions( a, b ).
fn q_multiply(a: Q, b: Q) -> Q {
    let [qax, qay, qaz, qaw] = a;
    let [qbx, qby, qbz, qbw] = b;
    [
        qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
        qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
        qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
        qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
    ]
}
/// Quaternion.slerp( qb, t ) from `a`.
fn q_slerp(a: Q, qb: Q, mut t: f64) -> Q {
    let [mut x, mut y, mut z, mut w] = qb;
    let mut d = a[0] * x + a[1] * y + a[2] * z + a[3] * w;
    if d < 0. {
        (x, y, z, w) = (-x, -y, -z, -w);
        d = -d;
    }
    let mut s = 1. - t;
    if d < 0.9995 {
        let theta = acos(d);
        let sn = sin(theta);
        s = sin(s * theta) / sn;
        t = sin(t * theta) / sn;
        [
            a[0] * s + x * t,
            a[1] * s + y * t,
            a[2] * s + z * t,
            a[3] * s + w * t,
        ]
    } else {
        q_normalize([
            a[0] * s + x * t,
            a[1] * s + y * t,
            a[2] * s + z * t,
            a[3] * s + w * t,
        ])
    }
}
/// Vector3.applyQuaternion.
fn apply_q(v: V3, q: Q) -> V3 {
    let [vx, vy, vz] = v;
    let [qx, qy, qz, qw] = q;
    let tx = 2. * (qy * vz - qz * vy);
    let ty = 2. * (qz * vx - qx * vz);
    let tz = 2. * (qx * vy - qy * vx);
    [
        vx + qw * tx + qy * tz - qz * ty,
        vy + qw * ty + qz * tx - qx * tz,
        vz + qw * tz + qx * ty - qy * tx,
    ]
}
/// Matrix4.compose.
fn compose(p: V3, q: Q, s: V3) -> M4 {
    let [x, y, z, w] = q;
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
/// Quaternion.setFromRotationMatrix.
fn q_from_matrix(te: &M4) -> Q {
    let (m11, m12, m13) = (te[0], te[4], te[8]);
    let (m21, m22, m23) = (te[1], te[5], te[9]);
    let (m31, m32, m33) = (te[2], te[6], te[10]);
    let trace = m11 + m22 + m33;
    if trace > 0. {
        let s = 0.5 / (trace + 1.0).sqrt();
        [(m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s]
    } else if m11 > m22 && m11 > m33 {
        let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
        [0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s]
    } else if m22 > m33 {
        let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
        [(m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s]
    } else {
        let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
        [(m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s]
    }
}
/// Matrix4.decompose: ( position, quaternion, scale ).
pub(crate) fn decompose(te: &M4) -> (V3, Q, V3) {
    let position = [te[12], te[13], te[14]];
    let det = te[0] * (te[5] * te[10] - te[9] * te[6]) - te[4] * (te[1] * te[10] - te[9] * te[2])
        + te[8] * (te[1] * te[6] - te[5] * te[2]);
    if det == 0. {
        return (position, IDENTITY_Q, [1., 1., 1.]);
    }
    let mut sx = length([te[0], te[1], te[2]]);
    let sy = length([te[4], te[5], te[6]]);
    let sz = length([te[8], te[9], te[10]]);
    if det < 0. {
        sx = -sx;
    }
    let mut m = *te;
    let (ix, iy, iz) = (1. / sx, 1. / sy, 1. / sz);
    for k in 0..3 {
        m[k] *= ix;
        m[4 + k] *= iy;
        m[8 + k] *= iz;
    }
    (position, q_from_matrix(&m), [sx, sy, sz])
}
/// makeRotateAroundPoint( point, quat ).
fn rotate_around_point(point: V3, q: Q) -> M4 {
    let translate = |p: V3| {
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., p[0], p[1], p[2], 1.,
        ]
    };
    let mut target = translate(scale(point, -1.));
    target = multiply(&compose([0.; 3], q, [1., 1., 1.]), &target);
    multiply(&translate(point), &target)
}

/// three's Ray.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Ray {
    pub origin: V3,
    pub direction: V3,
}
impl Ray {
    pub fn at(&self, t: f64) -> V3 {
        add_scaled(self.origin, self.direction, t)
    }
    /// applyMatrix4: the origin, and the direction as a normalized direction.
    pub fn apply4(&self, m: &M4) -> Ray {
        Ray {
            origin: apply4(self.origin, m),
            direction: transform_direction(self.direction, m),
        }
    }
    pub fn intersect_sphere(&self, center: V3, radius: f64) -> Option<V3> {
        if radius < 0. {
            return None;
        }
        let v = sub(center, self.origin);
        let tca = dot(v, self.direction);
        let d2 = dot(v, v) - tca * tca;
        let radius2 = radius * radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let (t0, t1) = (tca - thc, tca + thc);
        if t1 < 0. {
            return None;
        }
        Some(self.at(if t0 < 0. { t1 } else { t0 }))
    }
    /// intersectPlane for a plane ( normal, constant ).
    pub fn intersect_box(&self, min: V3, max: V3) -> Option<V3> {
        let o = self.origin;
        let inv = [
            1. / self.direction[0],
            1. / self.direction[1],
            1. / self.direction[2],
        ];
        let span = |k: usize| {
            if inv[k] >= 0. {
                ((min[k] - o[k]) * inv[k], (max[k] - o[k]) * inv[k])
            } else {
                ((max[k] - o[k]) * inv[k], (min[k] - o[k]) * inv[k])
            }
        };
        let (mut tmin, mut tmax) = span(0);
        let (tymin, tymax) = span(1);
        if tmin > tymax || tymin > tmax {
            return None;
        }
        if tymin > tmin || tmin.is_nan() {
            tmin = tymin;
        }
        if tymax < tmax || tmax.is_nan() {
            tmax = tymax;
        }
        let (tzmin, tzmax) = span(2);
        if tmin > tzmax || tzmin > tmax {
            return None;
        }
        if tzmin > tmin || tmin.is_nan() {
            tmin = tzmin;
        }
        if tzmax < tmax || tmax.is_nan() {
            tmax = tzmax;
        }
        if tmax < 0. {
            return None;
        }
        Some(self.at(if tmin >= 0. { tmin } else { tmax }))
    }
    /// intersectTriangle ( r186's watertight test ).
    pub fn intersect_triangle(&self, a: V3, b: V3, c: V3, backface_culling: bool) -> Option<V3> {
        let o = self.origin;
        let [dx, dy, dz] = self.direction;
        let (ao, bo, co) = (sub(a, o), sub(b, o), sub(c, o));
        let (adx, ady, adz) = (dx.abs(), dy.abs(), dz.abs());
        // ( kx, ky, kz ) axis permutation.
        let (kx, ky, kz) = if adx >= ady && adx >= adz {
            if dx >= 0. { (1, 2, 0) } else { (2, 1, 0) }
        } else if ady >= adz {
            if dy >= 0. { (2, 0, 1) } else { (0, 2, 1) }
        } else if dz >= 0. {
            (0, 1, 2)
        } else {
            (1, 0, 2)
        };
        let d = [dx, dy, dz];
        let (dkx, dky, dkz) = (d[kx], d[ky], d[kz]);
        if dkz == 0. {
            return None;
        }
        let (sx, sy, sz) = (dkx / dkz, dky / dkz, 1. / dkz);
        let (ax, ay) = (ao[kx] - sx * ao[kz], ao[ky] - sy * ao[kz]);
        let (bx, by) = (bo[kx] - sx * bo[kz], bo[ky] - sy * bo[kz]);
        let (cx, cy) = (co[kx] - sx * co[kz], co[ky] - sy * co[kz]);
        let u = cx * by - cy * bx;
        let v = ax * cy - ay * cx;
        let w = bx * ay - by * ax;
        if backface_culling {
            if u < 0. || v < 0. || w < 0. {
                return None;
            }
        } else if (u < 0. || v < 0. || w < 0.) && (u > 0. || v > 0. || w > 0.) {
            return None;
        }
        let det = u + v + w;
        if det == 0. {
            return None;
        }
        let t_scaled = sz * (u * ao[kz] + v * bo[kz] + w * co[kz]);
        if if det > 0. {
            t_scaled < 0.
        } else {
            t_scaled > 0.
        } {
            return None;
        }
        Some(self.at(t_scaled / det))
    }
}

/// A raycast's nearest hit.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hit {
    pub point: V3,
    pub distance: f64,
}

/// The scene the controls raycast ( the tiles group, firstHitOnly ).
pub(crate) trait Scene {
    fn raycast(&self, ray: &Ray, near: f64, far: f64) -> Option<Hit>;
}

/// The PerspectiveCamera the controls move.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Camera {
    pub position: V3,
    pub quaternion: Q,
    pub scale: V3,
    pub world: M4,
    pub world_inverse: M4,
    pub fov: f64,
    pub aspect: f64,
    pub near: f64,
    pub far: f64,
    pub projection: M4,
    pub projection_inverse: M4,
}

impl Camera {
    pub fn from_matrix(matrix: &M4, fov: f64, aspect: f64, near: f64, far: f64) -> Self {
        let (position, quaternion, scale) = decompose(matrix);
        let mut c = Camera {
            position,
            quaternion,
            scale,
            world: *matrix,
            world_inverse: invert(matrix),
            fov,
            aspect,
            near,
            far,
            projection: [0.; 16],
            projection_inverse: [0.; 16],
        };
        c.update_matrix_world();
        c.update_projection_matrix();
        c
    }
    pub fn update_matrix_world(&mut self) {
        self.world = compose(self.position, self.quaternion, self.scale);
        self.world_inverse = invert(&self.world);
    }
    /// PerspectiveCamera.updateProjectionMatrix ( WebGL clip space ).
    pub fn update_projection_matrix(&mut self) {
        let near = self.near;
        let top = near * tan(std::f64::consts::PI / 180. * 0.5 * self.fov);
        let height = 2. * top;
        let width = self.aspect * height;
        let left = -0.5 * width;
        let (right, bottom) = (left + width, top - height);
        let (far, x, y) = (
            self.far,
            2. * near / (right - left),
            2. * near / (top - bottom),
        );
        let (a, b) = (
            (right + left) / (right - left),
            (top + bottom) / (top - bottom),
        );
        let c = -(far + near) / (far - near);
        let d = (-2. * far * near) / (far - near);
        self.projection = [x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.];
        self.projection_inverse = invert(&self.projection);
    }
    /// Copies matrixWorld's decomposition into the transform members.
    fn decompose_world(&mut self) {
        let (p, q, _) = decompose(&self.world);
        self.position = p;
        self.quaternion = q;
    }
    /// Vector3.unproject.
    fn unproject(&self, v: V3) -> V3 {
        apply4(apply4(v, &self.projection_inverse), &self.world)
    }
}

/// setRaycasterFromCamera: the ray from the near plane, normalized, and the
/// raycaster's ( near 0, far ).
fn ray_from_camera(coords: (f64, f64), camera: &Camera) -> (Ray, f64) {
    let origin = camera.unproject([coords.0, coords.1, -1.]);
    let direction = sub(camera.unproject([coords.0, coords.1, 1.]), origin);
    let far = length(direction);
    (
        Ray {
            origin,
            direction: normalize(direction),
        },
        far,
    )
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum State {
    None,
    Drag,
    Rotate,
}

/// PointerTracker for one mouse pointer and the hover position.
#[derive(Default)]
struct Pointer {
    down: Option<(f64, f64)>,
    previous: (f64, f64),
    buttons: u32,
    hover: Option<(f64, f64)>,
}

/// GlobeControls with enableDamping ( the page's settings ).
pub(crate) struct Controls {
    pub camera: Camera,
    ellipsoid: Ellipsoid,
    pub adjust_height: bool,
    state: State,
    pointer: Pointer,
    pub needs_update: bool,
    action_height_offset: f64,
    pub pivot_point: V3,
    /// The pivot mesh: shown while it is in the scene and visible.
    pub pivot_shown: bool,
    zoom_direction_set: bool,
    zoom_point_set: bool,
    zoom_point_was_set: bool,
    zoom_direction: V3,
    zoom_point: V3,
    zoom_delta: f64,
    rotation_inertia: [f64; 2],
    drag_inertia: V3,
    inertia_target_distance: f64,
    inertia_stable_frames: u32,
    up: V3,
    up_initialized: bool,
    drag_mode: i32,
    rotation_mode: i32,
    globe_inertia: Q,
    globe_inertia_factor: f64,
    scale_zoom_orientation_at_edges: bool,
    /// The element's client size.
    pub client: (f64, f64),
    pub shift: bool,
}

const CAMERA_RADIUS: f64 = 5.;
const MIN_ALTITUDE: f64 = 0.;
const MAX_ALTITUDE: f64 = 0.45 * std::f64::consts::PI;
const MIN_DISTANCE: f64 = 10.;
const MAX_DISTANCE: f64 = f64::INFINITY;
const DAMPING_FACTOR: f64 = 0.15;
const NEAR_MARGIN: f64 = 0.25;
const FAR_MARGIN: f64 = 0.;
const DRAG_PLANE_THRESHOLD: f64 = 0.05;
const DRAG_UP_THRESHOLD: f64 = 0.025;
const MIN_ELEVATION: f64 = 2550.;

impl Controls {
    pub fn new(camera: Camera) -> Self {
        Controls {
            camera,
            ellipsoid: Ellipsoid::default(),
            // "adjustHeight causes camera drift as tiles load": off until the first interaction.
            adjust_height: false,
            state: State::None,
            pointer: Pointer::default(),
            needs_update: true,
            action_height_offset: 0.,
            pivot_point: [0.; 3],
            pivot_shown: false,
            zoom_direction_set: false,
            zoom_point_set: false,
            zoom_point_was_set: false,
            zoom_direction: [0.; 3],
            zoom_point: [0.; 3],
            zoom_delta: 0.,
            rotation_inertia: [0.; 2],
            drag_inertia: [0.; 3],
            inertia_target_distance: f64::INFINITY,
            inertia_stable_frames: 0,
            up: [0., 1., 0.],
            up_initialized: false,
            drag_mode: 0,
            rotation_mode: 0,
            globe_inertia: IDENTITY_Q,
            globe_inertia_factor: 0.,
            scale_zoom_orientation_at_edges: false,
            client: (1., 1.),
            shift: false,
        }
    }

    fn coords(&self, p: (f64, f64)) -> (f64, f64) {
        // adjustedPointerToCoords.
        (
            (p.0 / self.client.0) * 2. - 1.,
            -(p.1 / self.client.1) * 2. + 1.,
        )
    }

    fn up_direction(&self, point: V3) -> V3 {
        // getPositionToNormal in the ellipsoid frame ( the identity here ).
        transform_direction(
            self.ellipsoid.position_normal(point),
            &super::volume::IDENTITY,
        )
    }

    fn max_world_radius(&self) -> f64 {
        let r = self.ellipsoid.radius;
        r[0].max(r[1]).max(r[2])
    }

    fn distance_to_center(&self) -> f64 {
        length(scale(self.camera.position, -1.))
    }

    fn transition_distance(&self, factor: f64) -> f64 {
        let radius = self.max_world_radius();
        let half = std::f64::consts::PI / 180. * self.camera.fov * 0.5;
        let fov_horizontal = 2. * atan(tan(half) * self.camera.aspect);
        let vertical = radius / tan(half);
        let horizontal = radius / tan(fov_horizontal * 0.5);
        factor * vertical.max(horizontal)
    }

    fn is_near_controls(&self) -> bool {
        self.distance_to_center() < self.transition_distance(1.)
    }

    /// _raycast: the scene, then the ellipsoid ( no fallback plane ).
    fn raycast(&self, scene: &dyn Scene, ray: &Ray, near: f64, far: f64) -> Option<Hit> {
        if let Some(hit) = scene.raycast(ray, near, far) {
            return Some(hit);
        }
        let point = self.ellipsoid_intersect(ray, self.ellipsoid.radius)?;
        Some(Hit {
            point,
            distance: distance(point, ray.origin),
        })
    }

    /// Ellipsoid.intersectRay: the unit sphere in the radius-scaled frame.
    fn ellipsoid_intersect(&self, ray: &Ray, radius: V3) -> Option<V3> {
        let s = [
            radius[0], 0., 0., 0., 0., radius[1], 0., 0., 0., 0., radius[2], 0., 0., 0., 0., 1.,
        ];
        let local = ray.apply4(&invert(&s));
        let p = local.intersect_sphere([0.; 3], 1.)?;
        Some(apply4(p, &s))
    }

    fn point_below_camera(&self, scene: &dyn Scene, point: V3, up: V3) -> Option<Hit> {
        let ray = Ray {
            origin: add_scaled(point, up, 1e5),
            direction: scale(up, -1.),
        };
        let mut hit = self.raycast(scene, &ray, 0., f64::INFINITY)?;
        hit.distance -= 1e5;
        Some(hit)
    }

    fn reset_state(&mut self) {
        self.state = State::None;
        self.pivot_shown = false;
        self.action_height_offset = 0.;
        // pointerTracker.reset() forgets the hover position too.
        self.pointer = Pointer::default();
    }

    fn set_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.drag_inertia = [0.; 3];
        self.rotation_inertia = [0.; 2];
        self.inertia_stable_frames = 0;
        self.state = state;
        self.drag_mode = 0;
        self.rotation_mode = 0;
    }

    /// pointerdown ( button 0 left, 2 right ) at element coordinates.
    pub fn pointer_down(&mut self, scene: &dyn Scene, button: u32, x: f64, y: f64) {
        // The page's enableAdjustHeight listener.
        self.adjust_height = true;
        self.pointer.down = Some((x, y));
        self.pointer.previous = (x, y);
        self.pointer.buttons = match button {
            0 => 1,
            2 => 2,
            1 => 4,
            _ => 0,
        };
        self.needs_update = true;
        let (ray, far) = ray_from_camera(self.coords((x, y)), &self.camera);
        let d = dot(ray.direction, self.up).abs();
        if d < DRAG_PLANE_THRESHOLD || d < DRAG_UP_THRESHOLD {
            return;
        }
        let Some(hit) = self.raycast(scene, &ray, 0., far) else {
            return;
        };
        let left = self.pointer.buttons & 1 != 0;
        let right = self.pointer.buttons & 2 != 0;
        if right || left && self.shift {
            self.pivot_point = hit.point;
            self.pivot_shown = true;
            self.set_state(State::Rotate);
        } else if left {
            self.pivot_point = hit.point;
            self.pivot_shown = true;
            self.set_state(State::Drag);
        }
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.zoom_direction_set = false;
        self.zoom_point_set = false;
        if self.state != State::None {
            self.needs_update = true;
        }
        self.pointer.hover = Some((x, y));
        if self.pointer.down.is_some() {
            self.pointer.down = Some((x, y));
        }
    }

    pub fn pointer_up(&mut self) {
        if self.pointer.down.is_none() {
            return;
        }
        self.reset_state();
        self.needs_update = true;
    }

    /// wheel ( deltaMode pixels ).
    pub fn wheel(&mut self, delta: f64, at: Option<(f64, f64)>) {
        self.adjust_height = true;
        if let Some(p) = at {
            self.pointer.hover = Some(p);
            if self.pointer.down.is_some() {
                self.pointer.down = Some(p);
            }
        }
        self.zoom_delta -= 0.25 * sign(delta) * delta.abs();
        self.needs_update = true;
    }

    fn latest_point(&self) -> Option<(f64, f64)> {
        self.pointer.down.or(self.pointer.hover)
    }

    fn move_distance(&self) -> f64 {
        let Some(p) = self.pointer.down else {
            return 0.;
        };
        ((p.0 - self.pointer.previous.0).powi(2) + (p.1 - self.pointer.previous.1).powi(2)).sqrt()
    }

    fn inertia_needs_update(&self) -> bool {
        self.rotation_inertia[0] * self.rotation_inertia[0]
            + self.rotation_inertia[1] * self.rotation_inertia[1]
            != 0.
            || length_sq(self.drag_inertia) != 0.
            || self.globe_inertia_factor != 0.
    }

    /// GlobeControls.update( deltaTime ), then the frame's adjustCamera.
    pub fn update(&mut self, scene: &dyn Scene, delta_time: f64, device_pixel_ratio: f64) {
        if delta_time == 0. {
            return;
        }
        if self.is_near_controls() {
            self.scale_zoom_orientation_at_edges = self.zoom_delta < 0.;
        } else {
            if self.state != State::None && self.drag_mode != 1 && self.rotation_mode != 1 {
                self.pivot_shown = false;
            }
            self.scale_zoom_orientation_at_edges = false;
        }
        let adjust_rotation = self.needs_update || self.inertia_needs_update();
        self.environment_update(scene, delta_time, device_pixel_ratio);
        self.adjust_camera(scene);
        if adjust_rotation && self.is_near_controls() {
            let up = self.up_direction(self.camera.position);
            self.align_camera_up(up, 1.);
            let up = self.up_direction(self.camera.position);
            self.clamp_rotation(up);
        }
    }

    /// EnvironmentControls.update.
    fn environment_update(&mut self, scene: &dyn Scene, delta_time: f64, dpr: f64) {
        self.camera.update_matrix_world();
        let local_up = self.up_direction(self.camera.position);
        if !self.up_initialized {
            self.up_initialized = true;
            self.up = local_up;
        }
        self.zoom_point_set = false;
        let inertia_needs_update = self.inertia_needs_update();
        if self.needs_update || inertia_needs_update {
            self.update_zoom(scene);
            self.update_position(scene, delta_time, dpr);
            self.update_rotation(delta_time, dpr);
            if matches!(self.state, State::Drag | State::Rotate) {
                let forward = transform_direction([0., 0., -1.], &self.camera.world);
                self.inertia_target_distance =
                    dot(sub(self.pivot_point, self.camera.position), forward);
            } else if self.state == State::None {
                self.update_inertia(delta_time);
            }
            self.needs_update = false;
        }
        let hit = if self.adjust_height {
            self.point_below_camera(scene, self.camera.position, self.up)
        } else {
            None
        };
        let local_up = self.up_direction(self.camera.position);
        self.set_frame(scene, local_up);
        let mut hit = hit;
        if matches!(self.state, State::Drag | State::Rotate) && self.action_height_offset != 0. {
            let offset = self.action_height_offset;
            self.camera.position = add_scaled(self.camera.position, self.up, -offset);
            self.pivot_point = add_scaled(self.pivot_point, self.up, -offset);
            if let Some(h) = &mut hit {
                h.distance -= offset;
            }
        }
        self.action_height_offset = 0.;
        if let Some(h) = hit
            && h.distance < CAMERA_RADIUS
        {
            let delta = CAMERA_RADIUS - h.distance;
            self.camera.position = add_scaled(self.camera.position, self.up, delta);
            self.pivot_point = add_scaled(self.pivot_point, self.up, delta);
            self.action_height_offset = delta;
        }
        // pointerTracker.updateFrame.
        if let Some(p) = self.pointer.down {
            self.pointer.previous = p;
        }
        // autoAdjustCameraRotation is false for GlobeControls.
    }

    /// GlobeControls.adjustCamera: the height above the tiles and near / far.
    fn adjust_camera(&mut self, scene: &dyn Scene) {
        if self.adjust_height {
            let up = self.up_direction(self.camera.position);
            if let Some(hit) = self.point_below_camera(scene, self.camera.position, up)
                && hit.distance < CAMERA_RADIUS
            {
                self.camera.position =
                    add_scaled(self.camera.position, up, CAMERA_RADIUS - hit.distance);
            }
        }
        let max_radius = self.max_world_radius();
        let distance_to_center = self.distance_to_center();
        let margin = NEAR_MARGIN * max_radius;
        let alpha = ((distance_to_center - max_radius) / margin).clamp(0., 1.);
        let min_near = lerp(1., 1000., alpha);
        self.camera.near = min_near.max(distance_to_center - max_radius - margin);
        let lat = self.ellipsoid.latitude(self.camera.position);
        let elevation = self
            .ellipsoid
            .elevation(self.camera.position)
            .max(MIN_ELEVATION);
        self.camera.far =
            self.ellipsoid.horizon_distance(lat, elevation) + 0.1 + max_radius * FAR_MARGIN;
        self.camera.update_projection_matrix();
    }

    fn update_inertia(&mut self, delta_time: f64) {
        // EnvironmentControls._updateInertia.
        if self.inertia_stable_frames > 1 {
            self.drag_inertia = [0.; 3];
            self.rotation_inertia = [0.; 2];
            self.globe_inertia_factor = 0.;
            self.globe_inertia = IDENTITY_Q;
            return;
        }
        let factor = 2f64.powf(-delta_time / DAMPING_FACTOR);
        let stable_distance = self
            .camera
            .near
            .max(CAMERA_RADIUS)
            .max(MIN_DISTANCE)
            .max(self.inertia_target_distance);
        let pixel_threshold = 0.25 * (2. / (2. * 1e3));
        let camera = self.camera;
        let stable_points = || {
            let at = |x: f64, y: f64| {
                let (ray, _) = ray_from_camera((x, y), &camera);
                let local = ray.apply4(&camera.world_inverse);
                let direction = normalize(local.direction);
                let recast = Ray {
                    origin: Ray {
                        origin: local.origin,
                        direction,
                    }
                    .at(-dot(direction, local.origin)),
                    direction,
                };
                apply4(recast.at(stable_distance / direction[2]), &camera.world)
            };
            (at(0., 0.), at(pixel_threshold, pixel_threshold))
        };
        let rotation_sq = self.rotation_inertia[0].powi(2) + self.rotation_inertia[1].powi(2);
        if rotation_sq > 0. {
            let (v, d) = stable_points();
            let v = normalize(sub(v, self.pivot_point));
            let d = normalize(sub(d, self.pivot_point));
            let threshold = angle_to(v, d) / delta_time;
            self.rotation_inertia = [
                self.rotation_inertia[0] * factor,
                self.rotation_inertia[1] * factor,
            ];
            if self.rotation_inertia[0].powi(2) + self.rotation_inertia[1].powi(2)
                < threshold.powi(2)
            {
                self.rotation_inertia = [0.; 2];
            }
        }
        if length_sq(self.drag_inertia) > 0. {
            let (v, d) = stable_points();
            let threshold = distance(v, d) / delta_time;
            self.drag_inertia = scale(self.drag_inertia, factor);
            if length_sq(self.drag_inertia) < threshold.powi(2) {
                self.drag_inertia = [0.; 3];
            }
        }
        if self.rotation_inertia[0].powi(2) + self.rotation_inertia[1].powi(2) > 0. {
            let pivot = self.pivot_point;
            self.apply_rotation(
                self.rotation_inertia[0] * delta_time,
                self.rotation_inertia[1] * delta_time,
                pivot,
            );
        }
        if length_sq(self.drag_inertia) > 0. {
            self.camera.position = add_scaled(self.camera.position, self.drag_inertia, delta_time);
            self.camera.update_matrix_world();
        }
        // GlobeControls._updateInertia.
        if self.inertia_stable_frames > 1 {
            self.globe_inertia_factor = 0.;
            self.globe_inertia = IDENTITY_Q;
            return;
        }
        let camera = self.camera;
        if self.globe_inertia_factor != 0. {
            let at = |x: f64, y: f64| {
                let (ray, _) = ray_from_camera((x, y), &camera);
                let local = ray.apply4(&camera.world_inverse);
                let direction = normalize(local.direction);
                let recast = Ray {
                    origin: Ray {
                        origin: local.origin,
                        direction,
                    }
                    .at(-dot(direction, local.origin)),
                    direction,
                };
                apply4(recast.at(stable_distance / direction[2]), &camera.world)
            };
            let v = normalize(at(0., 0.));
            let p = normalize(at(pixel_threshold, pixel_threshold));
            self.globe_inertia_factor *= factor;
            let threshold = angle_to(v, p) / delta_time;
            let globe_angle = 2. * acos(self.globe_inertia[3]) * self.globe_inertia_factor;
            if globe_angle < threshold {
                self.globe_inertia_factor = 0.;
                self.globe_inertia = IDENTITY_Q;
            }
        }
        if self.globe_inertia_factor != 0. {
            if self.globe_inertia[3] == 1.
                && (self.globe_inertia[0] != 0.
                    || self.globe_inertia[1] != 0.
                    || self.globe_inertia[2] != 0.)
            {
                self.globe_inertia[3] = self.globe_inertia[3].min(1. - 1e-9);
            }
            let q = q_slerp(
                IDENTITY_Q,
                self.globe_inertia,
                self.globe_inertia_factor * delta_time,
            );
            let m = rotate_around_point([0.; 3], q);
            self.camera.world = multiply(&m, &self.camera.world);
            self.camera.decompose_world();
        }
    }

    fn update_zoom_direction(&mut self) {
        if self.zoom_direction_set {
            return;
        }
        let Some(p) = self.latest_point() else { return };
        let (ray, _) = ray_from_camera(self.coords(p), &self.camera);
        self.zoom_direction = normalize(ray.direction);
        self.zoom_direction_set = true;
    }

    fn update_zoom_point(&mut self, scene: &dyn Scene) -> bool {
        self.zoom_point_was_set = false;
        if !self.zoom_direction_set {
            return false;
        }
        let ray = Ray {
            origin: self.camera.position,
            direction: self.zoom_direction,
        };
        if let Some(hit) = self.raycast(scene, &ray, 0., f64::INFINITY) {
            self.zoom_point = hit.point;
            self.zoom_point_set = true;
            self.zoom_point_was_set = true;
            return true;
        }
        false
    }

    /// GlobeControls._updateZoom.
    fn update_zoom(&mut self, scene: &dyn Scene) {
        let zoom_delta = self.zoom_delta;
        if zoom_delta == 0. {
            return;
        }
        self.rotation_inertia = [0.; 2];
        self.drag_inertia = [0.; 3];
        self.globe_inertia = IDENTITY_Q;
        self.globe_inertia_factor = 0.;
        let delta_alpha = map_linear(zoom_delta.abs(), 0., 20., 0., 1.).clamp(0., 1.);
        if self.is_near_controls() || zoom_delta > 0. {
            self.update_zoom_direction();
            if zoom_delta < 0. && (self.zoom_point_set || self.update_zoom_point(scene)) {
                let forward = normalize(transform_direction([0., 0., -1.], &self.camera.world));
                let to_center = scale(self.up, -1.);
                let zoom_point_up = self.up_direction(self.zoom_point);
                let up_alpha =
                    map_linear(-dot(zoom_point_up, to_center), 1., 0.95, 0., 1.).clamp(0., 1.);
                let forward_alpha = 1. - dot(forward, to_center);
                let adjusted = (delta_alpha * 3.).clamp(0., 1.);
                let alpha = (up_alpha * forward_alpha * adjusted).min(0.1);
                let to_center = normalize(lerp3(forward, to_center, alpha));
                let q = q_unit_vectors(forward, to_center);
                let m = rotate_around_point(self.zoom_point, q);
                self.camera.world = multiply(&m, &self.camera.world);
                self.camera.decompose_world();
                self.zoom_direction = normalize(sub(self.zoom_point, self.camera.position));
            }
            self.environment_zoom(scene);
        } else {
            let transition = self.transition_distance(1.);
            let max_distance = self.transition_distance(2.);
            let distance_alpha =
                map_linear(self.distance_to_center(), transition, max_distance, 0., 1.);
            self.tilt_towards_center(lerp(0., 0.4, distance_alpha * delta_alpha));
            self.align_camera_up_to_north(lerp(0., 0.2, distance_alpha * delta_alpha));
            let dist = self.distance_to_center() - self.max_world_radius();
            let s = zoom_delta * dist * 0.0025;
            let clamped = s.max((self.distance_to_center() - max_distance).min(0.));
            let to_center = normalize(scale(self.camera.position, -1.));
            self.camera.position = add_scaled(self.camera.position, to_center, clamped);
            self.camera.update_matrix_world();
            self.zoom_delta = 0.;
        }
    }

    /// EnvironmentControls._updateZoom ( perspective ).
    fn environment_zoom(&mut self, scene: &dyn Scene) {
        let mut s = self.zoom_delta;
        self.zoom_delta = 0.;
        if self.latest_point().is_none() || s == 0. {
            return;
        }
        self.rotation_inertia = [0.; 2];
        self.drag_inertia = [0.; 3];
        self.update_zoom_direction();
        if self.zoom_point_set || self.update_zoom_point(scene) {
            let dist = distance(self.zoom_point, self.camera.position);
            if s < 0. {
                let remaining = (dist - MAX_DISTANCE).min(0.);
                s = (s * dist * 0.0025).max(remaining);
            } else {
                let remaining = (dist - MIN_DISTANCE).max(0.);
                s = (s * (dist - MIN_DISTANCE).max(0.) * 0.0025).min(remaining);
            }
            self.camera.position = add_scaled(self.camera.position, self.zoom_direction, s);
            self.camera.update_matrix_world();
        } else if let Some(hit) = self.point_below_camera(scene, self.camera.position, self.up) {
            let direction = transform_direction([0., 0., -1.], &self.camera.world);
            self.camera.position =
                add_scaled(self.camera.position, direction, s * hit.distance * 0.01);
            self.camera.update_matrix_world();
        } else {
            self.camera.position = add_scaled(self.camera.position, self.zoom_direction, s);
            self.camera.update_matrix_world();
        }
    }

    /// GlobeControls._updatePosition: dragging rotates the globe.
    fn update_position(&mut self, _scene: &dyn Scene, delta_time: f64, dpr: f64) {
        if self.state != State::Drag {
            return;
        }
        if self.drag_mode == 0 {
            self.drag_mode = if self.is_near_controls() { 1 } else { -1 };
        }
        let Some(p) = self.pointer.down else { return };
        let (ray, _) = ray_from_camera(self.coords(p), &self.camera);
        let pivot_radius = length(self.pivot_point);
        let Some(hit) = self.ellipsoid_intersect(&ray, [pivot_radius; 3]) else {
            self.reset_state();
            self.update_inertia(delta_time);
            return;
        };
        let pivot_dir = normalize(self.pivot_point);
        let new_pivot_dir = normalize(hit);
        let q = q_unit_vectors(new_pivot_dir, pivot_dir);
        let m = rotate_around_point([0.; 3], q);
        self.camera.world = multiply(&m, &self.camera.world);
        self.camera.decompose_world();
        if self.move_distance() / delta_time < 2. * dpr {
            self.inertia_stable_frames += 1;
        } else {
            self.globe_inertia = q;
            self.globe_inertia_factor = 1. / delta_time;
            self.inertia_stable_frames = 0;
        }
    }

    /// GlobeControls._updateRotation: only near the globe.
    fn update_rotation(&mut self, delta_time: f64, dpr: f64) {
        if self.state != State::Rotate {
            return;
        }
        if self.rotation_mode == 1 || self.is_near_controls() {
            self.rotation_mode = 1;
        } else {
            self.pivot_shown = false;
            self.rotation_mode = -1;
            return;
        }
        let Some(p) = self.pointer.down else { return };
        let previous = self.pointer.previous;
        let k = 2. * std::f64::consts::PI / self.client.1;
        let delta = ((p.0 - previous.0) * k, (p.1 - previous.1) * k);
        let pivot = self.pivot_point;
        self.apply_rotation(delta.0, delta.1, pivot);
        if self.move_distance() / delta_time < 2. * dpr {
            self.inertia_stable_frames += 1;
        } else {
            self.rotation_inertia = [delta.0 / delta_time, delta.1 / delta_time];
            self.inertia_stable_frames = 0;
        }
    }

    fn apply_rotation(&mut self, x: f64, y: f64, pivot: V3) {
        if x == 0. && y == 0. {
            return;
        }
        let azimuth = -x;
        let mut altitude = y;
        let forward = transform_direction([0., 0., 1.], &self.camera.world);
        let right = transform_direction([1., 0., 0.], &self.camera.world);
        let local_up = self.up_direction(pivot);
        let angle = if dot(local_up, forward) > 1. - 1e-10 {
            0.
        } else {
            let v = normalize(cross(local_up, forward));
            sign(dot(v, right)) * angle_to(local_up, forward)
        };
        if altitude > 0. {
            altitude = (angle - MIN_ALTITUDE).min(altitude).max(0.);
        } else {
            altitude = (angle - MAX_ALTITUDE).max(altitude).min(0.);
        }
        let m = rotate_around_point(pivot, q_axis_angle(local_up, azimuth));
        self.camera.world = multiply(&m, &self.camera.world);
        let right = transform_direction([1., 0., 0.], &self.camera.world);
        let m = rotate_around_point(pivot, q_axis_angle(right, -altitude));
        self.camera.world = multiply(&m, &self.camera.world);
        self.camera.decompose_world();
    }

    /// _setFrame: reorient around the zoom point, then take the new up.
    fn set_frame(&mut self, scene: &dyn Scene, new_up: V3) {
        if self.zoom_direction_set && (self.zoom_point_set || self.update_zoom_point(scene)) {
            let mut q = q_unit_vectors(self.up, new_up);
            if self.scale_zoom_orientation_at_edges {
                let v = self.up_direction(self.zoom_point);
                let mut amount = (dot(v, self.up) - 0.6).max(0.) / 0.4;
                amount = map_linear(amount, 0., 0.5, 0., 1.).min(1.);
                q = q_slerp(q, IDENTITY_Q, 1. - amount);
            }
            let m = rotate_around_point(self.zoom_point, q);
            self.camera.update_matrix_world();
            self.camera.world = multiply(&m, &self.camera.world);
            self.camera.decompose_world();
            self.zoom_direction_set = false;
            self.update_zoom_direction();
        }
        self.up = new_up;
        self.camera.update_matrix_world();
    }

    fn fixed_point(&self) -> Option<V3> {
        if matches!(self.state, State::Drag | State::Rotate) {
            Some(self.pivot_point)
        } else if self.zoom_point_set {
            Some(self.zoom_point)
        } else {
            None
        }
    }

    /// Keeps the fixed point where it was on screen after a rotation change.
    fn hold_fixed_point(&mut self, before: &M4) {
        if let Some(fixed) = self.fixed_point() {
            let v = apply4(fixed, &invert(before));
            self.camera.update_matrix_world();
            let v = apply4(v, &self.camera.world);
            self.camera.position = add(self.camera.position, sub(fixed, v));
        }
        self.camera.update_matrix_world();
    }

    /// _alignCameraUp( up, alpha ).
    fn align_camera_up(&mut self, up: V3, alpha: f64) {
        self.camera.update_matrix_world();
        let forward = transform_direction([0., 0., -1.], &self.camera.world);
        let right = transform_direction([-1., 0., 0.], &self.camera.world);
        let multiplier = map_linear(1. - dot(forward, up).abs(), 0., 0.2, 0., 1.).clamp(0., 1.);
        let alpha = alpha * multiplier;
        let target_right = normalize(lerp3(cross(up, forward), right, 1. - alpha));
        let q = q_unit_vectors(right, target_right);
        self.camera.quaternion = q_multiply(q, self.camera.quaternion);
        let before = self.camera.world;
        self.hold_fixed_point(&before);
    }

    /// _clampRotation( up ).
    fn clamp_rotation(&mut self, up: V3) {
        self.camera.update_matrix_world();
        let forward = transform_direction([0., 0., 1.], &self.camera.world);
        let right = transform_direction([1., 0., 0.], &self.camera.world);
        let angle = if dot(up, forward) > 1. - 1e-10 {
            0.
        } else {
            sign(dot(cross(up, forward), right)) * angle_to(up, forward)
        };
        let target = if angle > MAX_ALTITUDE {
            MAX_ALTITUDE
        } else if angle < MIN_ALTITUDE {
            MIN_ALTITUDE
        } else {
            return;
        };
        let forward = normalize(apply_q(up, q_axis_angle(right, target)));
        let v = normalize(cross(forward, right));
        let basis = [
            right[0], right[1], right[2], 0., v[0], v[1], v[2], 0., forward[0], forward[1],
            forward[2], 0., 0., 0., 0., 1.,
        ];
        self.camera.quaternion = q_from_matrix(&basis);
        let before = self.camera.world;
        self.hold_fixed_point(&before);
    }

    fn align_camera_up_to_north(&mut self, alpha: f64) {
        self.align_camera_up([0., 0., 1.], alpha);
    }

    fn tilt_towards_center(&mut self, alpha: f64) {
        let forward = normalize(transform_direction([0., 0., -1.], &self.camera.world));
        let v = normalize(lerp3(
            normalize(scale(self.camera.position, -1.)),
            forward,
            1. - alpha,
        ));
        let q = q_unit_vectors(forward, v);
        self.camera.quaternion = q_multiply(q, self.camera.quaternion);
        self.camera.update_matrix_world();
    }

    /// The pivot mesh's world position, when shown ( Vector3.project for the ring ).
    pub fn pivot(&self) -> Option<V3> {
        (self.pivot_shown && self.state != State::None).then_some(self.pivot_point)
    }
}

/// The tiles group as the controls raycast it: TilesRenderer.raycast with
/// firstHitOnly ( raycastTraverseFirstHit ) over the tiles' Mesh.raycast.
pub(crate) struct TilesScene<'a> {
    pub tiles: &'a super::tiles::Tiles,
}

impl TilesScene<'_> {
    /// TileBoundingVolume.intersectRay: the farther of the sphere and box hits.
    fn volume_hit(volume: &super::volume::Volume, ray: &Ray) -> Option<V3> {
        let mut sphere_dist = f64::NEG_INFINITY;
        let mut obb_dist = f64::NEG_INFINITY;
        if let Some((center, radius)) = volume.sphere
            && let Some(p) = ray.intersect_sphere(center, radius)
        {
            sphere_dist = if length_sq(sub(ray.origin, center)) <= radius * radius {
                0.
            } else {
                length_sq(sub(ray.origin, p))
            };
        }
        if let Some(obb) = &volume.obb {
            let local = ray.apply4(&obb.inverse);
            if let Some(p) = local.intersect_box(obb.min, obb.max) {
                let p = apply4(p, &obb.transform);
                let o = apply4(ray.origin, &obb.inverse);
                let contains = (0..3).all(|k| o[k] >= obb.min[k] && o[k] <= obb.max[k]);
                obb_dist = if contains {
                    0.
                } else {
                    length_sq(sub(ray.origin, p))
                };
            }
        }
        let furthest = sphere_dist.max(obb_dist);
        (furthest != f64::NEG_INFINITY).then(|| ray.at(furthest.sqrt()))
    }

    /// raycaster.intersectObject( tile scene ): every mesh's front-face hits, the nearest.
    fn tile_hit(&self, t: super::tiles::TileId, ray: &Ray, near: f64, far: f64) -> Option<Hit> {
        let scene = self.tiles.tiles[t].scene.as_ref()?;
        let mut best: Option<Hit> = None;
        for ((matrix, geometry, _), (center, radius)) in scene.meshes.iter().zip(&scene.spheres) {
            // Mesh.raycast: the bounding sphere from the near point, then the triangles.
            let c = apply4(*center, matrix);
            let r = radius * super::volume::max_scale(matrix);
            let near_ray = Ray {
                origin: ray.at(near),
                direction: ray.direction,
            };
            if length_sq(sub(near_ray.origin, c)) > r * r {
                let Some(at) = near_ray.intersect_sphere(c, r) else {
                    continue;
                };
                if length_sq(sub(near_ray.origin, at)) > (far - near).powi(2) {
                    continue;
                }
            }
            let local = ray.apply4(&invert(matrix));
            let Some(crate::geometry::Attribute::F32(position)) =
                geometry.attributes.get("position")
            else {
                continue;
            };
            let p = position.array();
            for tri in p.as_chunks::<9>().0 {
                let v = |k: usize| {
                    [
                        tri[3 * k] as f64,
                        tri[3 * k + 1] as f64,
                        tri[3 * k + 2] as f64,
                    ]
                };
                // FrontSide: back faces culled.
                let Some(point) = local.intersect_triangle(v(0), v(1), v(2), true) else {
                    continue;
                };
                let world = apply4(point, matrix);
                let d = distance(ray.origin, world);
                if d < near || d > far {
                    continue;
                }
                if best.is_none_or(|b| d < b.distance) {
                    best = Some(Hit {
                        point: world,
                        distance: d,
                    });
                }
            }
        }
        best
    }

    fn first_hit(&self, t: super::tiles::TileId, ray: &Ray, near: f64, far: f64) -> Option<Hit> {
        let tiles = &self.tiles.tiles;
        let mut children: Vec<(f64, super::tiles::TileId)> = vec![];
        for &child in &tiles[t].children {
            let Some(tr) = &tiles[child].traversal else {
                continue;
            };
            if !tr.used {
                continue;
            }
            if let Some(p) = Self::volume_hit(&tiles[child].volume, ray) {
                children.push((length_sq(sub(p, ray.origin)), child));
            }
        }
        children.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut best: Option<Hit> = None;
        let mut best_sq = f64::INFINITY;
        if self.tiles.active_tiles.contains(&t)
            && let Some(hit) = self.tile_hit(t, ray, near, far)
        {
            best_sq = hit.distance * hit.distance;
            best = Some(hit);
        }
        for (d, child) in children {
            if d > best_sq {
                break;
            }
            if let Some(hit) = self.first_hit(child, ray, near, far) {
                let sq = hit.distance * hit.distance;
                if sq < best_sq {
                    best_sq = sq;
                    best = Some(hit);
                }
            }
        }
        best
    }
}

impl Scene for TilesScene<'_> {
    fn raycast(&self, ray: &Ray, near: f64, far: f64) -> Option<Hit> {
        self.first_hit(self.tiles.root?, ray, near, far)
    }
}
