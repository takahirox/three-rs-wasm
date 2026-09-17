//! Three.js-compatible cubic curve sampling. Construction is CPU work; samples
//! are uploaded once when used as a static GPU geometry.
use crate::{Error, Result, math::Vector3};
#[derive(Clone, Copy, Debug)]
pub enum CatmullRomType {
    Centripetal,
    Chordal,
    Uniform { tension: f64 },
}
pub struct CatmullRomCurve3 {
    pub points: Vec<Vector3>,
    pub closed: bool,
    pub curve_type: CatmullRomType,
}
impl CatmullRomCurve3 {
    pub fn new(points: Vec<Vector3>) -> Self {
        Self {
            points,
            closed: false,
            curve_type: CatmullRomType::Centripetal,
        }
    }
    pub fn point(&self, t: f64) -> Result<Vector3> {
        let n = self.points.len();
        if n < 2 || !t.is_finite() || !(0.0..=1.0).contains(&t) {
            return Err(Error::Invalid("curve points/parameter"));
        }
        let p = (n - usize::from(!self.closed)) as f64 * t;
        let mut i = p.floor() as usize;
        let mut w = p - i as f64;
        if !self.closed && i == n - 1 && w == 0.0 {
            i = n - 2;
            w = 1.0;
        }
        let p0 = if self.closed || i > 0 {
            self.points[(i + n - 1) % n]
        } else {
            self.points[0] * 2.0 - self.points[1]
        };
        let p1 = self.points[i % n];
        let p2 = self.points[(i + 1) % n];
        let p3 = if self.closed || i + 2 < n {
            self.points[(i + 2) % n]
        } else {
            self.points[n - 1] * 2.0 - self.points[n - 2]
        };
        let (t1, t2) = match self.curve_type {
            CatmullRomType::Uniform { tension } => (tension * (p2 - p0), tension * (p3 - p1)),
            kind => {
                let exponent = if matches!(kind, CatmullRomType::Chordal) {
                    0.5
                } else {
                    0.25
                };
                let mut d0 = p0.distance_squared(p1).powf(exponent);
                let mut d1 = p1.distance_squared(p2).powf(exponent);
                let mut d2 = p2.distance_squared(p3).powf(exponent);
                if d1 < 1e-4 {
                    d1 = 1.0;
                }
                if d0 < 1e-4 {
                    d0 = d1;
                }
                if d2 < 1e-4 {
                    d2 = d1;
                }
                (
                    ((p1 - p0) / d0 - (p2 - p0) / (d0 + d1) + (p2 - p1) / d1) * d1,
                    ((p2 - p1) / d1 - (p3 - p1) / (d1 + d2) + (p3 - p2) / d2) * d1,
                )
            }
        };
        Ok(p1
            + t1 * w
            + (-3.0 * p1 + 3.0 * p2 - 2.0 * t1 - t2) * w * w
            + (2.0 * p1 - 2.0 * p2 + t1 + t2) * w * w * w)
    }
    pub fn points(&self, divisions: u32) -> Result<Vec<Vector3>> {
        if divisions == 0 {
            return Err(Error::Invalid("curve divisions"));
        }
        (0..=divisions)
            .map(|i| self.point(i as f64 / divisions as f64))
            .collect()
    }
}
