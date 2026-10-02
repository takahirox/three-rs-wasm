//! The page's procedural geometry: LoftGeometry ( three.js r186 addon )
//! over the sections each builder computes, and the CircleGeometry floor and
//! coffee surface, in f64 with the Float32Array stores, three.js's indexed
//! computeVertexNormals ( accumulated through the Float32Array ) and the
//! closed seam's averaged normals.
use super::super::text_shapes::{area, triangulate};
use std::f64::consts::PI;

type V3 = [f64; 3];
/// An indexed geometry's Float32Array attributes.
#[derive(Clone, Default)]
pub(super) struct Geometry {
    pub(super) position: Vec<f32>,
    pub(super) normal: Vec<f32>,
    pub(super) uv: Vec<f32>,
    pub(super) index: Vec<u32>,
    /// The page's section rings as line segments ( every `step`-th section
    /// and the last ), for the sections view.
    pub(super) skeleton: Vec<f32>,
}
impl Geometry {
    /// computeBoundingSphere(): the box centre and the farthest vertex.
    pub(super) fn bounding_sphere(&self) -> (V3, f64) {
        let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in self.position.chunks(3) {
            for k in 0..3 {
                min[k] = min[k].min(p[k] as f64);
                max[k] = max[k].max(p[k] as f64);
            }
        }
        let center = [0, 1, 2].map(|k| (min[k] + max[k]) / 2.);
        let mut radius = 0f64;
        for p in self.position.chunks(3) {
            let d: f64 = (0..3).map(|k| (p[k] as f64 - center[k]).powi(2)).sum();
            radius = radius.max(d);
        }
        (center, radius.sqrt())
    }
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn normalize(a: V3) -> V3 {
    let length = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    let inverse = 1. / if length == 0. { 1. } else { length };
    [a[0] * inverse, a[1] * inverse, a[2] * inverse]
}
fn distance(a: V3, b: V3) -> f64 {
    let d = sub(a, b);
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}
/// MathUtils.smoothstep( x, min, max ).
fn smoothstep(x: f64, min: f64, max: f64) -> f64 {
    if x <= min {
        return 0.;
    }
    if x >= max {
        return 1.;
    }
    let x = (x - min) / (max - min);
    x * x * (3. - 2. * x)
}
/// LoftGeometry( sections, { closed, capStart, capEnd } ).
pub(super) fn loft(sections: &[Vec<V3>], closed: bool, cap_start: bool, cap_end: bool) -> Geometry {
    let rows = sections.len();
    let columns = sections[0].len();
    let per_row = if closed { columns + 1 } else { columns };
    let (mut vertices, mut uvs, mut indices): (Vec<f64>, Vec<f64>, Vec<u32>) =
        (vec![], vec![], vec![]);
    let mut row_u = vec![0.];
    for i in 1..rows {
        let mut d = 0.;
        for (p, q) in sections[i].iter().zip(&sections[i - 1]).take(columns) {
            d += distance(*p, *q);
        }
        row_u.push(row_u[i - 1] + d / columns as f64);
    }
    let total_u = row_u[rows - 1];
    for (i, section) in sections.iter().enumerate() {
        let mut col_v = vec![0.];
        for j in 1..per_row {
            col_v.push(col_v[j - 1] + distance(section[j % columns], section[(j - 1) % columns]));
        }
        let total_v = col_v[per_row - 1];
        for j in 0..per_row {
            let p = section[j % columns];
            vertices.extend(p);
            uvs.push(if total_u > 0. {
                row_u[i] / total_u
            } else {
                i as f64 / (rows - 1) as f64
            });
            uvs.push(if total_v > 0. {
                col_v[j] / total_v
            } else {
                j as f64 / (per_row - 1) as f64
            });
        }
    }
    for i in 0..rows - 1 {
        for j in 0..per_row - 1 {
            let a = (i * per_row + j) as u32;
            let b = (i * per_row + j + 1) as u32;
            let c = ((i + 1) * per_row + j + 1) as u32;
            let d = ((i + 1) * per_row + j) as u32;
            indices.extend([a, b, d, b, c, d]);
        }
    }
    let mut cap = |index: usize| {
        let section = &sections[index];
        let mut centroid = [0.; 3];
        let mut normal = [0.; 3];
        for i in 0..columns {
            let (p, q) = (section[i], section[(i + 1) % columns]);
            for k in 0..3 {
                centroid[k] += p[k];
            }
            normal[0] += (p[1] - q[1]) * (p[2] + q[2]);
            normal[1] += (p[2] - q[2]) * (p[0] + q[0]);
            normal[2] += (p[0] - q[0]) * (p[1] + q[1]);
        }
        let inverse = 1. / columns as f64;
        centroid = centroid.map(|v| v * inverse);
        let mut normal = normalize(normal);
        let neighbor = &sections[if index == 0 { 1 } else { rows - 2 }];
        let mut v = [0.; 3];
        for p in neighbor {
            for k in 0..3 {
                v[k] += p[k];
            }
        }
        let v = sub(v.map(|c| c * inverse), centroid);
        if dot(normal, v) > 0. {
            normal = normal.map(|c| -c);
        }
        let mut tangent = [1., 0., 0.];
        if normal[0].abs() > 0.9 {
            tangent = [0., 1., 0.];
        }
        let bitangent = normalize(cross(normal, tangent));
        let tangent = cross(bitangent, normal);
        let mut points = section.clone();
        let mut contour: Vec<[f64; 2]> = points
            .iter()
            .map(|p| {
                let d = sub(*p, centroid);
                [dot(d, tangent), dot(d, bitangent)]
            })
            .collect();
        if area(&contour) < 0. {
            contour.reverse();
            points.reverse();
        }
        let faces = triangulate(&mut contour.clone(), &mut []);
        let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for c in &contour {
            for k in 0..2 {
                min[k] = min[k].min(c[k]);
                max[k] = max[k].max(c[k]);
            }
        }
        let width = (max[0] - min[0]).max(f64::EPSILON);
        let height = (max[1] - min[1]).max(f64::EPSILON);
        let offset = (vertices.len() / 3) as u32;
        for (p, c) in points.iter().zip(&contour) {
            vertices.extend(*p);
            uvs.extend([(c[0] - min[0]) / width, (c[1] - min[1]) / height]);
        }
        for f in faces {
            indices.extend(f.map(|i| offset + i as u32));
        }
    };
    if cap_start {
        cap(0);
    }
    if cap_end {
        cap(rows - 1);
    }
    let position: Vec<f32> = vertices.iter().map(|&v| v as f32).collect();
    let mut normal = vertex_normals(&position, &indices);
    if closed {
        for i in 0..rows {
            let (a, b) = (i * per_row, i * per_row + per_row - 1);
            let n =
                normalize([0, 1, 2].map(|k| normal[a * 3 + k] as f64 + normal[b * 3 + k] as f64));
            for k in 0..3 {
                normal[a * 3 + k] = n[k] as f32;
                normal[b * 3 + k] = n[k] as f32;
            }
        }
    }
    let step = ((rows as f64 / 20. + 0.5).floor() as usize).max(1);
    let mut skeleton = vec![];
    let mut add_ring = |ring: &[V3]| {
        let segments = if closed { ring.len() } else { ring.len() - 1 };
        for j in 0..segments {
            let (a, b) = (ring[j], ring[(j + 1) % ring.len()]);
            skeleton.extend(a.iter().chain(&b).map(|&v| v as f32));
        }
    };
    for section in sections.iter().step_by(step) {
        add_ring(section);
    }
    if !(rows - 1).is_multiple_of(step) {
        add_ring(&sections[rows - 1]);
    }
    Geometry {
        position,
        normal,
        uv: uvs.iter().map(|&v| v as f32).collect(),
        index: indices,
        skeleton,
    }
}
/// computeVertexNormals() of an indexed geometry: face cross products summed
/// into the Float32Array normals, then normalizeNormals().
fn vertex_normals(position: &[f32], index: &[u32]) -> Vec<f32> {
    let mut normal = vec![0f32; position.len()];
    let p = |i: u32| {
        let i = i as usize * 3;
        [
            position[i] as f64,
            position[i + 1] as f64,
            position[i + 2] as f64,
        ]
    };
    for t in index.chunks(3) {
        let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
        let n = cross(sub(c, b), sub(a, b));
        // nA, nB and nC are read before any is written back.
        let sums: Vec<[f32; 3]> = t
            .iter()
            .map(|&v| {
                let v = v as usize * 3;
                [0, 1, 2].map(|k| (normal[v + k] as f64 + n[k]) as f32)
            })
            .collect();
        for (&v, sum) in t.iter().zip(sums) {
            normal[v as usize * 3..v as usize * 3 + 3].copy_from_slice(&sum);
        }
    }
    for n in normal.chunks_mut(3) {
        let v = normalize([n[0] as f64, n[1] as f64, n[2] as f64]);
        for k in 0..3 {
            n[k] = v[k] as f32;
        }
    }
    normal
}
/// CircleGeometry( radius, segments ).rotateX( -π / 2 ).
pub(super) fn floor_circle(radius: f64, segments: usize) -> Geometry {
    let mut vertices = vec![[0., 0., 0.]];
    let mut uv = vec![0.5f32, 0.5];
    for s in 0..=segments {
        let segment = s as f64 / segments as f64 * (PI * 2.);
        let v = [radius * segment.cos(), radius * segment.sin(), 0.];
        uv.extend([
            ((v[0] / radius + 1.) / 2.) as f32,
            ((v[1] / radius + 1.) / 2.) as f32,
        ]);
        vertices.push(v);
    }
    let mut index = vec![];
    for i in 1..=segments as u32 {
        index.extend([i, i + 1, 0]);
    }
    // rotateX( -π / 2 ): makeRotationX, applied to the stored positions and
    // the renormalized normals.
    let (s, c) = (-PI / 2.).sin_cos();
    let mut position = vec![];
    for v in &vertices {
        let (x, y, z) = (v[0] as f32 as f64, v[1] as f32 as f64, v[2] as f32 as f64);
        position.extend([x as f32, (c * y - s * z) as f32, (s * y + c * z) as f32]);
    }
    // The normal matrix of a rotation: Matrix3 invert and transpose.
    let n = normal_of_rotation_x(c, s);
    let normal_value = normalize([n[6], n[7], n[8]]).map(|v| v as f32);
    let normal = vertices.iter().flat_map(|_| normal_value).collect();
    Geometry {
        position,
        normal,
        uv,
        index,
        skeleton: vec![],
    }
}
/// Matrix3.getNormalMatrix( makeRotationX ) by three.js's invert and
/// transpose ( column-major ).
fn normal_of_rotation_x(c: f64, s: f64) -> [f64; 9] {
    let (n11, n21, n31) = (1., 0., 0.);
    let (n12, n22, n32) = (0., c, s);
    let (n13, n23, n33) = (0., -s, c);
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    let d = 1. / det;
    let te = [
        t11 * d,
        (n31 * n23 - n33 * n21) * d,
        (n32 * n21 - n31 * n22) * d,
        t12 * d,
        (n33 * n11 - n31 * n13) * d,
        (n31 * n12 - n32 * n11) * d,
        t13 * d,
        (n21 * n13 - n23 * n11) * d,
        (n22 * n11 - n21 * n12) * d,
    ];
    [
        te[0], te[3], te[6], te[1], te[4], te[7], te[2], te[5], te[8],
    ]
}
/// CatmullRom( t, p0, p1, p2, p3 ).
fn catmull_rom(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let v0 = (p2 - p0) * 0.5;
    let v1 = (p3 - p1) * 0.5;
    let t2 = t * t;
    let t3 = t * t2;
    (2. * p1 - 2. * p2 + v0 + v1) * t3 + (-3. * p1 + 3. * p2 - 2. * v0 - v1) * t2 + v0 * t + p1
}
/// SplineCurve.getPoint( t ).
fn spline_point(points: &[[f64; 2]], t: f64) -> [f64; 2] {
    let n = points.len();
    let p = (n - 1) as f64 * t;
    let int = p.floor() as usize;
    let weight = p - int as f64;
    let p0 = points[if int == 0 { int } else { int - 1 }];
    let p1 = points[int];
    let p2 = points[if int > n - 2 { n - 1 } else { int + 1 }];
    let p3 = points[if int + 3 > n { n - 1 } else { int + 2 }];
    [0, 1].map(|k| catmull_rom(weight, p0[k], p1[k], p2[k], p3[k]))
}
/// SplineCurve.getPoints( divisions ).
fn spline_points(points: &[[f64; 2]], divisions: usize) -> Vec<[f64; 2]> {
    (0..=divisions)
        .map(|d| spline_point(points, d as f64 / divisions as f64))
        .collect()
}
/// Curve.getTangent( t ).
fn spline_tangent(points: &[[f64; 2]], t: f64) -> [f64; 2] {
    let delta = 0.0001;
    let t1 = (t - delta).max(0.);
    let t2 = (t + delta).min(1.);
    let (a, b) = (spline_point(points, t1), spline_point(points, t2));
    let d = [b[0] - a[0], b[1] - a[1]];
    let length = (d[0] * d[0] + d[1] * d[1]).sqrt();
    let inverse = 1. / if length == 0. { 1. } else { length };
    [d[0] * inverse, d[1] * inverse]
}
fn ring(segments: usize, f: impl Fn(f64) -> V3) -> Vec<V3> {
    (0..segments)
        .map(|j| f(j as f64 / segments as f64 * PI * 2.))
        .collect()
}
fn revolved(profile: &[[f64; 2]], divisions: usize, segments: usize) -> Vec<Vec<V3>> {
    spline_points(profile, divisions)
        .iter()
        .map(|p| ring(segments, |a| [a.sin() * p[0], p[1], a.cos() * p[0]]))
        .collect()
}
pub(super) fn pedestal(radius: f64, height: f64) -> Geometry {
    let r = radius;
    let h = height;
    let profile = [
        [0.2, 0.],
        [r * 1.06, 0.],
        [r * 1.06, h * 0.1],
        [r * 1.06, h * 0.1],
        [r * 0.98, h * 0.16],
        [r * 0.94, h * 0.55],
        [r * 0.97, h * 0.84],
        [r * 1.04, h * 0.88],
        [r * 1.04, h * 0.97],
        [r * 1.04, h * 0.97],
        [r * 0.98, h],
        [r * 0.98, h],
        [r * 0.5, h - 0.004],
        [0.2, h],
    ];
    let sections: Vec<Vec<V3>> = profile
        .iter()
        .map(|p| ring(48, |a| [a.sin() * p[0], p[1], a.cos() * p[0]]))
        .collect();
    loft(&sections, true, true, true)
}
pub(super) fn cup() -> Geometry {
    let profile = [
        [0.2, 0.],
        [0.7, 0.04],
        [1.05, 0.1],
        [1.75, 0.55],
        [2.25, 1.45],
        [2.36, 2.2],
        [2.3, 3.1],
        [2.22, 3.82],
        [2.18, 3.95],
        [2.06, 3.8],
        [2.18, 2.2],
        [1.5, 0.75],
        [0.9, 0.55],
        [0.2, 0.62],
    ];
    loft(&revolved(&profile, 120, 64), true, true, true)
}
pub(super) fn saucer() -> Geometry {
    let profile = [
        [0.2, 0.],
        [1.3, 0.08],
        [2.6, 0.35],
        [3.7, 0.8],
        [4.2, 1.],
        [3.4, 0.68],
        [2., 0.3],
        [1., 0.18],
        [0.2, 0.26],
    ];
    loft(&revolved(&profile, 120, 64), true, true, true)
}
pub(super) fn handle() -> Geometry {
    let path = [
        [2.2, 3.3],
        [2.9, 3.45],
        [3.6, 2.85],
        [3.65, 1.95],
        [3., 1.2],
        [1.78, 1.05],
    ];
    let divisions = 60;
    let points = spline_points(&path, divisions);
    let sections: Vec<Vec<V3>> = (0..=divisions)
        .map(|i| {
            let t = i as f64 / divisions as f64;
            let point = points[i];
            let tangent = spline_tangent(&path, t);
            let scale = (1. - 0.25 * t)
                * (0.28 + 0.72 * smoothstep(t, 0., 0.12))
                * (0.28 + 0.72 * (1. - smoothstep(t, 0.88, 1.)));
            let (a, b) = (0.22 * scale, 0.27 * scale);
            ring(16, |phi| {
                let radial = a * phi.cos();
                [
                    point[0] - radial * tangent[1],
                    point[1] + radial * tangent[0],
                    b * phi.sin(),
                ]
            })
        })
        .collect();
    loft(&sections, true, false, false)
}
pub(super) fn vase() -> Geometry {
    let profile = [
        [0.2, 0.],
        [1.05, 0.05],
        [1.5, 0.3],
        [2.1, 1.4],
        [2.2, 2.3],
        [1.8, 3.6],
        [1.2, 4.8],
        [0.85, 5.8],
        [0.72, 6.6],
        [0.8, 7.3],
        [1.1, 7.9],
        [1.3, 8.2],
    ];
    loft(&revolved(&profile, 100, 48), true, true, false)
}
pub(super) fn shell() -> Geometry {
    let turns = 3.;
    let growth = 0.18;
    let scale = (growth * turns * PI * 2.).exp();
    let sections: Vec<Vec<V3>> = (0..=150)
        .map(|i| {
            let t = i as f64 / 150.;
            let angle = turns * PI * 2. * t;
            let e = (growth * angle).exp() / scale;
            let path_radius = 3. * e;
            let section_radius = 2.4 * e;
            let (sin, cos) = (angle.sin(), angle.cos());
            ring(32, |phi| {
                let r = path_radius + section_radius * phi.cos();
                [
                    r * sin,
                    4.5 * (1. - e) + section_radius * phi.sin(),
                    r * cos,
                ]
            })
        })
        .collect();
    loft(&sections, true, false, false)
}
pub(super) fn star() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=60)
        .map(|i| {
            let t = i as f64 / 60.;
            let twist = t * PI / 3.;
            let scale = 1. - 0.35 * (t * PI).sin();
            ring(96, |angle| {
                let radius = (2.4 + 0.7 * (5. * angle).cos()) * scale;
                [
                    (angle + twist).sin() * radius,
                    t * 10.,
                    (angle + twist).cos() * radius,
                ]
            })
        })
        .collect();
    loft(&sections, true, true, true)
}
pub(super) fn ribbon() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=120)
        .map(|i| {
            let t = i as f64 / 120.;
            let angle = t * PI * 2. * 2.5;
            let (sin, cos) = (angle.sin(), angle.cos());
            vec![
                [3. * sin, t * 7.5, 3. * cos],
                [3. * sin, t * 7.5 + 2., 3. * cos],
            ]
        })
        .collect();
    loft(&sections, false, false, false)
}
pub(super) fn toothpaste() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=80)
        .map(|i| {
            let t = i as f64 / 80.;
            let radius = 0.5 + 0.28 * smoothstep(t, 0.08, 0.2);
            let crimp = smoothstep(t, 0.3, 0.95);
            let width = radius * (1. - crimp) + 1.15 * crimp;
            let depth = radius * (1. - crimp) + 0.05 * crimp;
            ring(48, |angle| {
                [angle.sin() * width, t * 4.2, angle.cos() * depth]
            })
        })
        .collect();
    loft(&sections, true, true, true)
}
pub(super) fn pumpkin() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=60)
        .map(|i| {
            let t = i as f64 / 60.;
            let angle = PI * (0.03 + 0.94 * t);
            let radius = 1.85 * angle.sin().powf(0.62);
            let creases = 0.15 * (PI * t).sin();
            let y = 2.05 * t - 0.75 * smoothstep(t, 0.8, 1.);
            ring(96, |theta| {
                let lobe = (3.5 * theta).cos().abs().powf(0.35);
                let r = radius * (1. - creases + creases * lobe);
                [theta.sin() * r, y, theta.cos() * r]
            })
        })
        .collect();
    loft(&sections, true, true, true)
}
pub(super) fn pumpkin_stem() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=30)
        .map(|i| {
            let t = i as f64 / 30.;
            let radius = 0.2 - 0.09 * t + 0.14 * (1. - t).powf(4.);
            let lean = 0.45 * t * t;
            ring(32, |angle| {
                let r = radius * (0.92 + 0.13 * (2.5 * angle).cos().abs().powf(0.5));
                [lean + angle.sin() * r, 1.3 + 1.15 * t, angle.cos() * r]
            })
        })
        .collect();
    loft(&sections, true, false, true)
}
pub(super) fn mushroom_cap() -> Geometry {
    let profile = [
        [0.35, 2.02],
        [1.1, 2.],
        [1.65, 2.15],
        [1.78, 2.4],
        [1.5, 2.85],
        [0.95, 3.18],
        [0.2, 3.32],
    ];
    loft(&revolved(&profile, 80, 48), true, false, true)
}
pub(super) fn mushroom_stem() -> Geometry {
    let profile = [
        [0.2, 0.],
        [0.55, 0.05],
        [0.45, 0.9],
        [0.4, 1.7],
        [0.42, 2.3],
    ];
    loft(&revolved(&profile, 60, 32), true, true, true)
}
pub(super) fn goblet() -> Geometry {
    let profile = [
        [0.2, 0.],
        [1.25, 0.05],
        [1.35, 0.2],
        [0.6, 0.5],
        [0.28, 0.9],
        [0.24, 1.7],
        [0.7, 2.15],
        [1.15, 2.8],
        [1.28, 3.5],
        [1.27, 3.62],
        [1.16, 3.5],
        [0.95, 2.85],
        [0.45, 2.25],
        [0.2, 2.32],
    ];
    loft(&revolved(&profile, 140, 48), true, true, true)
}
pub(super) fn stanchion() -> Geometry {
    let profile = [
        [0.16, 0.],
        [0.42, 0.04],
        [0.46, 0.12],
        [0.28, 0.22],
        [0.08, 0.38],
        [0.06, 1.],
        [0.06, 1.85],
        [0.11, 1.95],
        [0.19, 2.08],
        [0.2, 2.2],
        [0.11, 2.3],
        [0.04, 2.34],
    ];
    loft(&revolved(&profile, 80, 24), true, true, true)
}
pub(super) fn rope(length: f64) -> Geometry {
    let sag = 0.9;
    let sections: Vec<Vec<V3>> = (0..=40)
        .map(|i| {
            let t = i as f64 / 40.;
            let x = (t - 0.5) * length;
            let y = -sag * 4. * t * (1. - t);
            let tx = length;
            let ty = -sag * 4. * (1. - 2. * t);
            let tl = (tx * tx + ty * ty).sqrt();
            ring(16, |phi| {
                let radial = 0.08 * phi.cos();
                [x - radial * ty / tl, y + radial * tx / tl, 0.08 * phi.sin()]
            })
        })
        .collect();
    loft(&sections, true, false, false)
}
pub(super) fn curtain() -> Geometry {
    let sections: Vec<Vec<V3>> = (0..=30)
        .map(|i| {
            let t = i as f64 / 30.;
            let y = -5. + 25. * t;
            (0..480)
                .map(|j| {
                    let s = j as f64 / 480.;
                    let folds = (1.2 - 0.5 * t) * (s * PI * 2. * 48. + t * 2.).sin();
                    let sway = 0.5 * (s * PI * 2. * 5. + t * 3.).sin();
                    let theta = s * PI * 2.;
                    let r = 55. + folds + sway;
                    [theta.sin() * r, y, theta.cos() * r]
                })
                .collect()
        })
        .collect();
    loft(&sections, true, false, false)
}
