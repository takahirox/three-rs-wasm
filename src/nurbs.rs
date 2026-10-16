//! NURBSCurve, NURBSSurface and NURBSVolume ( examples/jsm/curves ) with
//! NURBSUtils' evaluation, operation for operation.
use crate::math::Vector3;

/// A homogeneous control point ( x, y, z, w ).
pub type Point4 = [f64; 4];

/// findSpan( p, u, U ).
fn find_span(p: usize, u: f64, knots: &[f64]) -> usize {
    let k = |i: usize| knots.get(i).copied().unwrap_or(f64::NAN);
    let n = knots.len() - p - 1;
    if u >= k(n) {
        return n - 1;
    }
    if u <= k(p) {
        return p;
    }
    let (mut low, mut high) = (p, n);
    let mut mid = (low + high) / 2;
    while u < k(mid) || u >= k(mid + 1) {
        if u < k(mid) {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}

/// calcBasisFunctions( span, u, p, U ).
fn basis(span: usize, u: f64, p: usize, knots: &[f64]) -> Vec<f64> {
    let k = |i: usize| knots.get(i).copied().unwrap_or(f64::NAN);
    let mut n = vec![0.0; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - k(span + 1 - j);
        right[j] = k(span + j) - u;
        let mut saved = 0.0;
        for r in 0..j {
            let rv = right[r + 1];
            let lv = left[j - r];
            let temp = n[r] / (rv + lv);
            n[r] = saved + rv * temp;
            saved = lv * temp;
        }
        n[j] = saved;
    }
    n
}

fn add(a: Point4, b: Point4) -> Point4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}
fn scale(a: Point4, s: f64) -> Point4 {
    [a[0] * s, a[1] * s, a[2] * s, a[3] * s]
}
/// `point.clone()` with x, y, z multiplied by its weight.
fn weighted(p: Point4) -> Point4 {
    [p[0] * p[3], p[1] * p[3], p[2] * p[3], p[3]]
}
fn project(h: Point4) -> Vector3 {
    let s = scale(h, 1.0 / h[3]);
    Vector3::new(s[0], s[1], s[2])
}

/// NURBSCurve( degree, knots, controlPoints, startKnot, endKnot ).
#[derive(Clone, Debug)]
pub struct NurbsCurve {
    pub degree: usize,
    pub knots: Vec<f64>,
    pub control_points: Vec<Point4>,
    pub start_knot: usize,
    pub end_knot: usize,
}
impl NurbsCurve {
    /// `startKnot || 0` and `endKnot || knots.length − 1`.
    pub fn new(
        degree: usize,
        knots: Vec<f64>,
        control_points: Vec<Point4>,
        start_knot: Option<usize>,
        end_knot: Option<usize>,
    ) -> Self {
        let last = knots.len().saturating_sub(1);
        Self {
            degree,
            start_knot: start_knot.filter(|&k| k != 0).unwrap_or(0),
            end_knot: end_knot.filter(|&k| k != 0).unwrap_or(last),
            knots,
            control_points,
        }
    }
    fn knot(&self, i: usize) -> f64 {
        self.knots.get(i).copied().unwrap_or(f64::NAN)
    }
    /// getPoint( t ): calcBSplinePoint at the mapped u, projected when w ≠ 1.
    pub fn point(&self, t: f64) -> Vector3 {
        let u = self.knot(self.start_knot)
            + t * (self.knot(self.end_knot) - self.knot(self.start_knot));
        let p = self.degree;
        let span = find_span(p, u, &self.knots);
        let n = basis(span, u, p, &self.knots);
        let mut c = [0.0; 4];
        for (j, nj) in n.iter().enumerate() {
            let point = self
                .control_points
                .get(span - p + j)
                .copied()
                .unwrap_or([f64::NAN; 4]);
            let w = point[3] * nj;
            c[0] += point[0] * w;
            c[1] += point[1] * w;
            c[2] += point[2] * w;
            c[3] += point[3] * nj;
        }
        if c[3] != 1.0 {
            project(c)
        } else {
            Vector3::new(c[0], c[1], c[2])
        }
    }
    /// Curve.getPoints( divisions ).
    pub fn points(&self, divisions: usize) -> Vec<Vector3> {
        (0..=divisions)
            .map(|d| self.point(d as f64 / divisions as f64))
            .collect()
    }
}

/// NURBSSurface( degree1, degree2, knots1, knots2, controlPoints ).
#[derive(Clone, Debug)]
pub struct NurbsSurface {
    pub degrees: [usize; 2],
    pub knots: [Vec<f64>; 2],
    pub control_points: Vec<Vec<Point4>>,
}
impl NurbsSurface {
    /// getPoint( t1, t2 ): calcSurfacePoint at the mapped ( u, v ).
    pub fn point(&self, t1: f64, t2: f64) -> Vector3 {
        let map = |k: &[f64], t: f64| k[0] + t * (k[k.len() - 1] - k[0]);
        let (u, v) = (map(&self.knots[0], t1), map(&self.knots[1], t2));
        let [p, q] = self.degrees;
        let us = find_span(p, u, &self.knots[0]);
        let vs = find_span(q, v, &self.knots[1]);
        let nu = basis(us, u, p, &self.knots[0]);
        let nv = basis(vs, v, q, &self.knots[1]);
        let mut sw = [0.0; 4];
        let mut temp = vec![[0.0; 4]; q + 1];
        for (l, t) in temp.iter_mut().enumerate() {
            for (k, nk) in nu.iter().enumerate() {
                let point = weighted(self.control_points[us - p + k][vs - q + l]);
                *t = add(*t, scale(point, *nk));
            }
        }
        for (l, t) in temp.iter().enumerate() {
            sw = add(sw, scale(*t, nv[l]));
        }
        project(sw)
    }
}

/// NURBSVolume( degree1, degree2, degree3, knots1, knots2, knots3, controlPoints ).
#[derive(Clone, Debug)]
pub struct NurbsVolume {
    pub degrees: [usize; 3],
    pub knots: [Vec<f64>; 3],
    pub control_points: Vec<Vec<Vec<Point4>>>,
}
impl NurbsVolume {
    /// getPoint( t1, t2, t3 ): calcVolumePoint at the mapped ( u, v, w ).
    pub fn point(&self, t1: f64, t2: f64, t3: f64) -> Vector3 {
        let map = |k: &[f64], t: f64| k[0] + t * (k[k.len() - 1] - k[0]);
        let (u, v, w) = (
            map(&self.knots[0], t1),
            map(&self.knots[1], t2),
            map(&self.knots[2], t3),
        );
        let [p, q, r] = self.degrees;
        let us = find_span(p, u, &self.knots[0]);
        let vs = find_span(q, v, &self.knots[1]);
        let ws = find_span(r, w, &self.knots[2]);
        let nu = basis(us, u, p, &self.knots[0]);
        let nv = basis(vs, v, q, &self.knots[1]);
        let nw = basis(ws, w, r, &self.knots[2]);
        let mut temp = vec![vec![[0.0; 4]; q + 1]; r + 1];
        for (m, row) in temp.iter_mut().enumerate() {
            for (l, t) in row.iter_mut().enumerate() {
                for (k, nk) in nu.iter().enumerate() {
                    let point = weighted(self.control_points[us - p + k][vs - q + l][ws - r + m]);
                    *t = add(*t, scale(point, *nk));
                }
            }
        }
        let mut sw = [0.0; 4];
        for (m, row) in temp.iter().enumerate() {
            for (l, t) in row.iter().enumerate() {
                sw = add(sw, scale(scale(*t, nw[m]), nv[l]));
            }
        }
        project(sw)
    }
}
