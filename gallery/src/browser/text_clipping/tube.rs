//! CurveExtras curves and TubeGeometry, as the extrude-splines example builds them.
use crate::{
    Result,
    curve::{CatmullRomCurve3, CatmullRomType, Curve},
    math::Vector3,
};
use std::f64::consts::PI;

pub(super) enum Spline {
    GrannyKnot,
    Heart(f64),
    Viviani(f64),
    Knot,
    Helix,
    Trefoil(f64),
    Torus(f64),
    Cinquefoil(f64),
    TrefoilPolynomial(f64),
    FigureEightPolynomial(f64),
    Decorated4a(f64),
    Decorated4b(f64),
    Decorated5a(f64),
    Decorated5c(f64),
    CatmullRom(CatmullRomCurve3),
}
/// The example's `splines` dictionary, in its key order.
pub(super) fn splines() -> Vec<Spline> {
    let v = Vector3::new;
    let pipe = CatmullRomCurve3::new(vec![
        v(0., 10., -10.),
        v(10., 0., -10.),
        v(20., 0., 0.),
        v(30., 0., 10.),
        v(30., 0., 20.),
        v(20., 0., 30.),
        v(10., 0., 30.),
        v(0., 0., 30.),
        v(-10., 10., 30.),
        v(-10., 20., 30.),
        v(0., 30., 30.),
        v(10., 30., 30.),
        v(20., 30., 15.),
        v(10., 30., 10.),
        v(0., 30., 10.),
        v(-10., 20., 10.),
        v(-10., 10., 10.),
        v(0., 0., 10.),
        v(10., -10., 10.),
        v(20., -15., 10.),
        v(30., -15., 10.),
        v(40., -15., 10.),
        v(50., -15., 10.),
        v(60., 0., 10.),
        v(70., 0., 0.),
        v(80., 0., 0.),
        v(90., 0., 0.),
        v(100., 0., 0.),
    ]);
    let mut closed = CatmullRomCurve3::new(vec![
        v(0., -40., -40.),
        v(0., 40., -40.),
        v(0., 140., -40.),
        v(0., 40., 40.),
        v(0., -40., 40.),
    ]);
    closed.curve_type = CatmullRomType::Uniform { tension: 0.5 };
    closed.closed = true;
    vec![
        Spline::GrannyKnot,
        Spline::Heart(3.5),
        Spline::Viviani(70.),
        Spline::Knot,
        Spline::Helix,
        Spline::Trefoil(10.),
        Spline::Torus(20.),
        Spline::Cinquefoil(20.),
        Spline::TrefoilPolynomial(14.),
        Spline::FigureEightPolynomial(1.),
        Spline::Decorated4a(40.),
        Spline::Decorated4b(40.),
        Spline::Decorated5a(40.),
        Spline::Decorated5c(40.),
        Spline::CatmullRom(pipe),
        Spline::CatmullRom(closed),
    ]
}
fn torus(p: f64, q: f64, t: f64, scale: f64) -> Vector3 {
    let t = t * PI * 2.;
    Vector3::new(
        (2. + (q * t).cos()) * (p * t).cos(),
        (2. + (q * t).cos()) * (p * t).sin(),
        (q * t).sin(),
    ) * scale
}
impl Curve for Spline {
    fn point(&self, t: f64) -> Result<Vector3> {
        let v = Vector3::new;
        Ok(match self {
            Spline::GrannyKnot => {
                let t = 2. * PI * t;
                v(
                    -0.22 * t.cos()
                        - 1.28 * t.sin()
                        - 0.44 * (3. * t).cos()
                        - 0.78 * (3. * t).sin(),
                    -0.1 * (2. * t).cos() - 0.27 * (2. * t).sin()
                        + 0.38 * (4. * t).cos()
                        + 0.46 * (4. * t).sin(),
                    0.7 * (3. * t).cos() - 0.4 * (3. * t).sin(),
                ) * 20.
            }
            Spline::Heart(scale) => {
                let t = t * 2. * PI;
                v(
                    16. * t.sin().powf(3.),
                    13. * t.cos() - 5. * (2. * t).cos() - 2. * (3. * t).cos() - (4. * t).cos(),
                    0.,
                ) * *scale
            }
            Spline::Viviani(scale) => {
                let t = t * 4. * PI;
                let a = scale / 2.;
                v(a * (1. + t.cos()), a * t.sin(), 2. * a * (t / 2.).sin())
            }
            Spline::Knot => {
                let t = t * 2. * PI;
                let (r, s) = (10., 50.);
                v(
                    s * t.sin(),
                    t.cos() * (r + s * t.cos()),
                    t.sin() * (r + s * t.cos()),
                )
            }
            Spline::Helix => {
                let (a, b) = (30., 150.);
                let t2 = 2. * PI * t * b / 30.;
                v(t2.cos() * a, t2.sin() * a, b * t)
            }
            Spline::Trefoil(scale) => {
                let t = t * PI * 2.;
                v(
                    (2. + (3. * t).cos()) * (2. * t).cos(),
                    (2. + (3. * t).cos()) * (2. * t).sin(),
                    (3. * t).sin(),
                ) * *scale
            }
            Spline::Torus(scale) => torus(3., 4., t, *scale),
            Spline::Cinquefoil(scale) => torus(2., 5., t, *scale),
            Spline::TrefoilPolynomial(scale) => {
                let t = t * 4. - 2.;
                v(
                    t.powf(3.) - 3. * t,
                    t.powf(4.) - 4. * t * t,
                    1. / 5. * t.powf(5.) - 2. * t,
                ) * *scale
            }
            Spline::FigureEightPolynomial(scale) => {
                let t = t * 8. - 4.;
                v(
                    2. / 5. * t * (t * t - 7.) * (t * t - 10.),
                    t.powf(4.) - 13. * t * t,
                    1. / 10. * t * (t * t - 4.) * (t * t - 9.) * (t * t - 12.),
                ) * *scale
            }
            Spline::Decorated4a(scale) => {
                let t = t * PI * 2.;
                let r = 1. + 0.6 * ((5. * t).cos() + 0.75 * (10. * t).cos());
                v(
                    (2. * t).cos() * r,
                    (2. * t).sin() * r,
                    0.35 * (5. * t).sin(),
                ) * *scale
            }
            Spline::Decorated4b(scale) => {
                let fi = t * PI * 2.;
                let r = 1. + 0.45 * (3. * fi).cos() + 0.4 * (9. * fi).cos();
                v(
                    (2. * fi).cos() * r,
                    (2. * fi).sin() * r,
                    0.2 * (9. * fi).sin(),
                ) * *scale
            }
            Spline::Decorated5a(scale) => {
                let fi = t * PI * 2.;
                let r = 1. + 0.3 * (5. * fi).cos() + 0.5 * (10. * fi).cos();
                v(
                    (3. * fi).cos() * r,
                    (3. * fi).sin() * r,
                    0.2 * (20. * fi).sin(),
                ) * *scale
            }
            Spline::Decorated5c(scale) => {
                let fi = t * PI * 2.;
                let r = 1. + 0.5 * ((5. * fi).cos() + 0.4 * (20. * fi).cos());
                v(
                    (4. * fi).cos() * r,
                    (4. * fi).sin() * r,
                    0.35 * (15. * fi).sin(),
                ) * *scale
            }
            Spline::CatmullRom(c) => c.point(t)?,
        })
    }
}
/// TubeGeometry( path, tubularSegments, radius, radialSegments, closed ) with the
/// Frenet frames it keeps for the camera animation.
pub(super) struct Tube {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub index: Vec<u32>,
    pub binormals: Vec<Vector3>,
    pub tangents: usize,
}
pub(super) fn tube(
    path: &dyn Curve,
    tubular: u32,
    radius: f64,
    radial: u32,
    closed: bool,
) -> Result<Tube> {
    let (tangents, normals, binormals) = path.frenet_frames(tubular, closed)?;
    let lengths = path.lengths(200)?;
    let (mut positions, mut out_normals) = (vec![], vec![]);
    let rows = (0..tubular).chain([if closed { 0 } else { tubular }]);
    for i in rows {
        let p = path.point(path.u_to_t(i as f64 / tubular as f64, &lengths))?;
        let (n, b) = (normals[i as usize], binormals[i as usize]);
        for j in 0..=radial {
            let v = j as f64 / radial as f64 * PI * 2.;
            let (sin, cos) = (v.sin(), -v.cos());
            let normal = (cos * n + sin * b).normalize_or_zero();
            out_normals.extend(normal.to_array().map(|x| x as f32));
            positions.extend((p + radius * normal).to_array().map(|x| x as f32));
        }
    }
    let mut index = vec![];
    for j in 1..=tubular {
        for i in 1..=radial {
            let a = (radial + 1) * (j - 1) + (i - 1);
            let b = (radial + 1) * j + (i - 1);
            let c = (radial + 1) * j + i;
            let d = (radial + 1) * (j - 1) + i;
            index.extend([a, b, d, b, c, d]);
        }
    }
    Ok(Tube {
        positions,
        normals: out_normals,
        index,
        binormals,
        tangents: tangents.len(),
    })
}
