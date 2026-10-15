//! 3d-tiles-renderer's WGS84 Ellipsoid: the east-north-up frame and
//! getObjectFrame, with the host's Math.sin / Math.cos as three evaluates them.
use super::volume::{M4, V3, cross, dot, multiply, normalize};

pub(crate) const WGS84_RADIUS: f64 = 6378137.;
pub(crate) const WGS84_FLATTENING: f64 = 1. / 298.257223563;
pub(crate) const CAMERA_FRAME: u32 = 1;

fn sin(x: f64) -> f64 {
    crate::bounce::trig::sin(x)
}
fn cos(x: f64) -> f64 {
    crate::bounce::trig::cos(x)
}
fn asin(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::asin(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.asin()
}

pub(crate) struct Ellipsoid {
    pub radius: V3,
}
impl Default for Ellipsoid {
    fn default() -> Self {
        let height = -(WGS84_FLATTENING * WGS84_RADIUS - WGS84_RADIUS);
        Ellipsoid {
            radius: [WGS84_RADIUS, WGS84_RADIUS, height],
        }
    }
}

/// Matrix4.makeRotationFromEuler for the orders getObjectFrame uses.
pub(crate) fn rotation_from_euler(x: f64, y: f64, z: f64, order: &str) -> M4 {
    let (a, b) = (cos(x), sin(x));
    let (c, d) = (cos(y), sin(y));
    let (e, f) = (cos(z), sin(z));
    let mut te = [0.; 16];
    match order {
        "XYZ" => {
            let (ae, af, be, bf) = (a * e, a * f, b * e, b * f);
            te[0] = c * e;
            te[4] = -c * f;
            te[8] = d;
            te[1] = af + be * d;
            te[5] = ae - bf * d;
            te[9] = -b * c;
            te[2] = bf - ae * d;
            te[6] = be + af * d;
            te[10] = a * c;
        }
        _ => {
            // ZXY
            let (ce, cf, de, df) = (c * e, c * f, d * e, d * f);
            te[0] = ce - df * b;
            te[4] = -a * f;
            te[8] = de + cf * b;
            te[1] = cf + de * b;
            te[5] = a * e;
            te[9] = df - ce * b;
            te[2] = -a * d;
            te[6] = b;
            te[10] = a * c;
        }
    }
    te[15] = 1.;
    te
}

impl Ellipsoid {
    /// getCartographicToNormal: the spherical direction swapped to the geo frame.
    fn normal(&self, lat: f64, lon: f64) -> V3 {
        let phi = -lat + std::f64::consts::PI / 2.;
        let sin_phi_radius = sin(phi) * 1.;
        let v = normalize([
            sin_phi_radius * sin(lon),
            cos(phi) * 1.,
            sin_phi_radius * cos(lon),
        ]);
        // swapToGeoFrame: ( z, x, y ).
        [v[2], v[0], v[1]]
    }
    /// getCartographicToPosition.
    pub fn position(&self, lat: f64, lon: f64, height: f64) -> V3 {
        let n = self.normal(lat, lon);
        let r = self.radius;
        let v = [
            n[0] * r[0].powi(2),
            n[1] * r[1].powi(2),
            n[2] * r[2].powi(2),
        ];
        let gamma = dot(n, v).sqrt();
        let v = [v[0] / gamma, v[1] / gamma, v[2] / gamma];
        [
            v[0] + n[0] * height,
            v[1] + n[1] * height,
            v[2] + n[2] * height,
        ]
    }
    /// getEastNorthUpFrame: makeBasis( east, north, up ).setPosition( position ).
    fn east_north_up(&self, lat: f64, lon: f64, height: f64) -> M4 {
        let point = self.position(lat, lon, 0.);
        let up = self.normal(lat, lon);
        let east = normalize([-point[1], point[0], 0.]);
        let north = normalize(cross(up, east));
        let p = self.position(lat, lon, height);
        [
            east[0], east[1], east[2], 0., north[0], north[1], north[2], 0., up[0], up[1], up[2],
            0., p[0], p[1], p[2], 1.,
        ]
    }
    /// getObjectFrame( lat, lon, height, az, el, roll, target, frame ).
    #[allow(clippy::too_many_arguments)]
    pub fn object_frame(
        &self,
        lat: f64,
        lon: f64,
        height: f64,
        az: f64,
        el: f64,
        roll: f64,
        frame: u32,
    ) -> M4 {
        let enu = self.east_north_up(lat, lon, height);
        let mut target = multiply(&enu, &rotation_from_euler(el, roll, -az, "ZXY"));
        if frame == CAMERA_FRAME {
            target = multiply(
                &target,
                &rotation_from_euler(std::f64::consts::PI / 2., 0., 0., "XYZ"),
            );
        }
        target
    }
}

impl Ellipsoid {
    /// getPositionToSurfacePoint: Newton iterations onto the surface.
    pub fn surface_point(&self, pos: V3) -> Option<V3> {
        let r = self.radius;
        let (ix, iy, iz) = (1. / r[0].powi(2), 1. / r[1].powi(2), 1. / r[2].powi(2));
        let (x2, y2, z2) = (
            pos[0] * pos[0] * ix,
            pos[1] * pos[1] * iy,
            pos[2] * pos[2] * iz,
        );
        let squared_norm = x2 + y2 + z2;
        let ratio = (1.0 / squared_norm).sqrt();
        let intersection = [pos[0] * ratio, pos[1] * ratio, pos[2] * ratio];
        if squared_norm < 0.1 {
            return ratio.is_finite().then_some(intersection);
        }
        let gradient = [
            intersection[0] * ix * 2.0,
            intersection[1] * iy * 2.0,
            intersection[2] * iz * 2.0,
        ];
        let length = |v: V3| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        let mut lambda = (1.0 - ratio) * length(pos) / (0.5 * length(gradient));
        let mut correction = 0.0;
        let (mut xm, mut ym, mut zm);
        loop {
            lambda -= correction;
            xm = 1.0 / (1.0 + lambda * ix);
            ym = 1.0 / (1.0 + lambda * iy);
            zm = 1.0 / (1.0 + lambda * iz);
            let (xm2, ym2, zm2) = (xm * xm, ym * ym, zm * zm);
            let (xm3, ym3, zm3) = (xm2 * xm, ym2 * ym, zm2 * zm);
            let func = x2 * xm2 + y2 * ym2 + z2 * zm2 - 1.0;
            let denominator = x2 * xm3 * ix + y2 * ym3 * iy + z2 * zm3 * iz;
            let derivative = -2.0 * denominator;
            correction = func / derivative;
            if func.abs() <= 1e-12 {
                break;
            }
        }
        Some([pos[0] * xm, pos[1] * ym, pos[2] * zm])
    }
    /// getPositionToNormal.
    pub fn position_normal(&self, pos: V3) -> V3 {
        let r = self.radius;
        normalize([
            pos[0] / r[0].powi(2),
            pos[1] / r[1].powi(2),
            pos[2] / r[2].powi(2),
        ])
    }
    /// getPositionToCartographic's latitude.
    pub fn latitude(&self, pos: V3) -> f64 {
        let surface = self.surface_point(pos).unwrap_or(pos);
        asin(self.position_normal(surface)[2])
    }
    /// getPositionElevation.
    pub fn elevation(&self, pos: V3) -> f64 {
        let surface = self.surface_point(pos).unwrap_or(pos);
        let delta = [
            pos[0] - surface[0],
            pos[1] - surface[1],
            pos[2] - surface[2],
        ];
        let sign = dot(delta, pos).signum_js();
        sign * (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt()
    }
    /// calculateHorizonDistance ( calculateEffectiveRadius reads degrees ).
    pub fn horizon_distance(&self, latitude: f64, elevation: f64) -> f64 {
        let (a, b) = (self.radius[0], self.radius[2]);
        let e2 = 1. - (b.powi(2) / a.powi(2));
        let phi = latitude * (std::f64::consts::PI / 180.);
        let s = sin(phi).powi(2);
        let n = a / (1. - e2 * s).sqrt();
        (2. * n * elevation + elevation.powi(2)).sqrt()
    }
}

trait SignumJs {
    fn signum_js(self) -> f64;
}
impl SignumJs for f64 {
    fn signum_js(self) -> f64 {
        if self > 0. {
            1.
        } else if self < 0. {
            -1.
        } else {
            self
        }
    }
}
