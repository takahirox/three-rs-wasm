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
    /// `Curve.getLengths( arcLengthDivisions = 200 )`: cumulative chord lengths.
    pub fn lengths(&self, divisions: u32) -> Result<Vec<f64>> {
        let mut out = vec![0.0];
        let mut last = self.point(0.0)?;
        let mut sum = 0.0;
        for p in 1..=divisions {
            let current = self.point(p as f64 / divisions as f64)?;
            sum += current.distance(last);
            out.push(sum);
            last = current;
        }
        Ok(out)
    }
    /// `Curve.getUtoTmapping( u )`: the parameter at arc-length fraction u.
    pub fn u_to_t(&self, u: f64, lengths: &[f64]) -> f64 {
        let il = lengths.len();
        let target = u * lengths[il - 1];
        let (mut low, mut high) = (0i64, il as i64 - 1);
        while low <= high {
            let i = low + (high - low) / 2;
            let comparison = lengths[i as usize] - target;
            if comparison < 0.0 {
                low = i + 1;
            } else if comparison > 0.0 {
                high = i - 1;
            } else {
                high = i;
                break;
            }
        }
        let i = high.max(0) as usize;
        if lengths[i] == target {
            return i as f64 / (il - 1) as f64;
        }
        let before = lengths[i];
        let segment = lengths[(i + 1).min(il - 1)] - before;
        (i as f64 + (target - before) / segment) / (il - 1) as f64
    }
    /// `Curve.getSpacedPoints( divisions )`: points at equal arc-length steps.
    pub fn spaced_points(&self, divisions: u32) -> Result<Vec<Vector3>> {
        let lengths = self.lengths(200)?;
        (0..=divisions)
            .map(|d| self.point(self.u_to_t(d as f64 / divisions as f64, &lengths)))
            .collect()
    }
    /// `Curve.getTangent( t )`: the normalized chord across ±0.0001.
    pub fn tangent(&self, t: f64) -> Result<Vector3> {
        let (t1, t2) = ((t - 0.0001).max(0.0), (t + 0.0001).min(1.0));
        Ok((self.point(t2)? - self.point(t1)?).normalize_or_zero())
    }
    /// `Curve.computeFrenetFrames( segments, closed )`: tangents, normals, binormals.
    #[allow(clippy::type_complexity)]
    pub fn frenet_frames(
        &self,
        segments: u32,
        closed: bool,
    ) -> Result<(Vec<Vector3>, Vec<Vector3>, Vec<Vector3>)> {
        let lengths = self.lengths(200)?;
        let tangents: Vec<Vector3> = (0..=segments)
            .map(|i| self.tangent(self.u_to_t(i as f64 / segments as f64, &lengths)))
            .collect::<Result<_>>()?;
        let rotate = |v: Vector3, axis: Vector3, angle: f64| {
            // makeRotationAxis, applied as a Matrix4 to a point.
            let (c, s) = (angle.cos(), angle.sin());
            let t = 1.0 - c;
            let (x, y, z) = (axis.x, axis.y, axis.z);
            let (tx, ty) = (t * x, t * y);
            Vector3::new(
                (tx * x + c) * v.x + (tx * y - s * z) * v.y + (tx * z + s * y) * v.z,
                (tx * y + s * z) * v.x + (ty * y + c) * v.y + (ty * z - s * x) * v.z,
                (tx * z - s * y) * v.x + (ty * z + s * x) * v.y + (t * z * z + c) * v.z,
            )
        };
        let t0 = tangents[0];
        let mut min = f64::MAX;
        let mut normal = Vector3::ZERO;
        let (tx, ty, tz) = (t0.x.abs(), t0.y.abs(), t0.z.abs());
        if tx <= min {
            min = tx;
            normal = Vector3::X;
        }
        if ty <= min {
            min = ty;
            normal = Vector3::Y;
        }
        if tz <= min {
            normal = Vector3::Z;
        }
        let vec = t0.cross(normal).normalize_or_zero();
        let mut normals = vec![t0.cross(vec)];
        let mut binormals = vec![t0.cross(normals[0])];
        for i in 1..=segments as usize {
            let mut n = normals[i - 1];
            let v = tangents[i - 1].cross(tangents[i]);
            if v.length() > f64::EPSILON {
                let theta = tangents[i - 1].dot(tangents[i]).clamp(-1.0, 1.0).acos();
                n = rotate(n, v.normalize(), theta);
            }
            normals.push(n);
            binormals.push(tangents[i].cross(n));
        }
        if closed {
            let last = segments as usize;
            let mut theta = normals[0].dot(normals[last]).clamp(-1.0, 1.0).acos() / segments as f64;
            if tangents[0].dot(normals[0].cross(normals[last])) > 0.0 {
                theta = -theta;
            }
            for i in 1..=last {
                normals[i] = rotate(normals[i], tangents[i], theta * i as f64);
                binormals[i] = tangents[i].cross(normals[i]);
            }
        }
        Ok((tangents, normals, binormals))
    }
}
