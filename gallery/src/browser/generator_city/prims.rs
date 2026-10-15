//! The three.js r186 primitives the city's parts start from, with their
//! constructors' arithmetic, vertex order and Float32Array stores.
use super::geo::{G, V3, normalize};
use std::f64::consts::PI;

fn geometry(attributes: Vec<(&'static str, usize, Vec<f64>)>, index: Option<Vec<u32>>) -> G {
    G {
        attributes: attributes
            .into_iter()
            .map(|(n, s, v)| (n, s, v.into_iter().map(|x| x as f32).collect()))
            .collect(),
        index,
    }
}
/// CylinderGeometry( top, bottom, height, radial, heightSegments, openEnded ).
pub(super) fn cylinder(
    top: f64,
    bottom: f64,
    height: f64,
    radial: u32,
    rows: u32,
    open: bool,
) -> G {
    let (theta_start, theta_length) = (0., PI * 2.);
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    let mut next = 0u32;
    let half = height / 2.;
    let slope = (bottom - top) / height;
    let mut grid = vec![];
    for y in 0..=rows {
        let mut row = vec![];
        let v = y as f64 / rows as f64;
        let radius = v * (bottom - top) + top;
        for x in 0..=radial {
            let u = x as f64 / radial as f64;
            let theta = u * theta_length + theta_start;
            let (s, c) = (theta.sin(), theta.cos());
            p.extend([radius * s, -v * height + half, radius * c]);
            n.extend(normalize([s, slope, c]));
            uv.extend([u, 1. - v]);
            row.push(next);
            next += 1;
        }
        grid.push(row);
    }
    for x in 0..radial as usize {
        for y in 0..rows as usize {
            let (a, b) = (grid[y][x], grid[y + 1][x]);
            let (c, d) = (grid[y + 1][x + 1], grid[y][x + 1]);
            if top > 0. || y != 0 {
                index.extend([a, b, d]);
            }
            if bottom > 0. || y != rows as usize - 1 {
                index.extend([b, c, d]);
            }
        }
    }
    if !open {
        for is_top in [true, false] {
            let radius = if is_top { top } else { bottom };
            if radius <= 0. {
                continue;
            }
            let sign = if is_top { 1. } else { -1. };
            let center = next;
            for _ in 1..=radial {
                p.extend([0., half * sign, 0.]);
                n.extend([0., sign, 0.]);
                uv.extend([0.5, 0.5]);
                next += 1;
            }
            let end = next;
            for x in 0..=radial {
                let u = x as f64 / radial as f64;
                let theta = u * theta_length + theta_start;
                let (c, s) = (theta.cos(), theta.sin());
                p.extend([radius * s, half * sign, radius * c]);
                n.extend([0., sign, 0.]);
                uv.extend([c * 0.5 + 0.5, s * 0.5 * sign + 0.5]);
                next += 1;
            }
            for x in 0..radial {
                let (c, i) = (center + x, end + x);
                if is_top {
                    index.extend([i, i + 1, c]);
                } else {
                    index.extend([i + 1, i, c]);
                }
            }
        }
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// BoxGeometry( width, height, depth ).
pub(super) fn cuboid(width: f64, height: f64, depth: f64) -> G {
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    let mut vertices = 0u32;
    // buildPlane( u, v, w, udir, vdir, width, height, depth ).
    let planes = [
        (2, 1, 0, -1., -1., depth, height, width),
        (2, 1, 0, 1., -1., depth, height, -width),
        (0, 2, 1, 1., 1., width, depth, height),
        (0, 2, 1, 1., -1., width, depth, -height),
        (0, 1, 2, 1., -1., width, height, depth),
        (0, 1, 2, -1., -1., width, height, -depth),
    ];
    for (u, v, w, udir, vdir, pw, ph, pd) in planes {
        let (sw, sh) = (pw / 1., ph / 1.);
        for iy in 0..2 {
            let y = iy as f64 * sh - ph / 2.;
            for ix in 0..2 {
                let x = ix as f64 * sw - pw / 2.;
                let mut q = [0.; 3];
                q[u] = x * udir;
                q[v] = y * vdir;
                q[w] = pd / 2.;
                p.extend(q);
                let mut m = [0.; 3];
                m[w] = if pd > 0. { 1. } else { -1. };
                n.extend(m);
                uv.extend([ix as f64, 1. - iy as f64]);
            }
        }
        let (a, b, c, d) = (vertices, vertices + 2, vertices + 3, vertices + 1);
        index.extend([a, b, d, b, c, d]);
        vertices += 4;
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// SphereGeometry( radius, widthSegments, heightSegments, phiStart,
/// phiLength, thetaStart, thetaLength ).
pub(super) fn sphere(radius: f64, ws: u32, hs: u32, phi: (f64, f64), theta: (f64, f64)) -> G {
    let (ws, hs) = (ws.max(3), hs.max(2));
    let theta_end = (theta.0 + theta.1).min(PI);
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    for iy in 0..=hs {
        let v = iy as f64 / hs as f64;
        let t = theta.0 + v * theta.1;
        let y = radius * t.cos();
        let ring = (radius * radius - y * y).sqrt();
        let offset = if iy == 0 && theta.0 == 0. {
            0.5 / ws as f64
        } else if iy == hs && theta_end == PI {
            -0.5 / ws as f64
        } else {
            0.
        };
        for ix in 0..=ws {
            let u = ix as f64 / ws as f64;
            let f = phi.0 + u * phi.1;
            let q = [-ring * f.cos(), y, ring * f.sin()];
            p.extend(q);
            n.extend(normalize(q));
            uv.extend([u + offset, 1. - v]);
        }
    }
    let at = |iy: u32, ix: u32| iy * (ws + 1) + ix;
    for iy in 0..hs {
        for ix in 0..ws {
            let (a, b, c, d) = (
                at(iy, ix + 1),
                at(iy, ix),
                at(iy + 1, ix),
                at(iy + 1, ix + 1),
            );
            if iy != 0 || theta.0 > 0. {
                index.extend([a, b, d]);
            }
            if iy != hs - 1 || theta_end < PI {
                index.extend([b, c, d]);
            }
        }
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// CircleGeometry( radius, segments, thetaStart, thetaLength ).
pub(super) fn circle(radius: f64, segments: u32, start: f64, length: f64) -> G {
    let segments = segments.max(3);
    let (mut p, mut n, mut uv, mut index) = (vec![0.; 3], vec![0., 0., 1.], vec![0.5, 0.5], vec![]);
    for s in 0..=segments {
        let segment = start + s as f64 / segments as f64 * length;
        let (x, y) = (radius * segment.cos(), radius * segment.sin());
        p.extend([x, y, 0.]);
        n.extend([0., 0., 1.]);
        uv.extend([(x / radius + 1.) / 2., (y / radius + 1.) / 2.]);
    }
    for i in 1..=segments {
        index.extend([i, i + 1, 0]);
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// RingGeometry( inner, outer, thetaSegments, phiSegments ).
pub(super) fn ring(inner: f64, outer: f64, theta_segments: u32, phi_segments: u32) -> G {
    let (ts, ps) = (theta_segments.max(3), phi_segments.max(1));
    let (start, length) = (0., PI * 2.);
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    let mut radius = inner;
    let step = (outer - inner) / ps as f64;
    for _ in 0..=ps {
        for i in 0..=ts {
            let segment = start + i as f64 / ts as f64 * length;
            let (x, y) = (radius * segment.cos(), radius * segment.sin());
            p.extend([x, y, 0.]);
            n.extend([0., 0., 1.]);
            uv.extend([(x / outer + 1.) / 2., (y / outer + 1.) / 2.]);
        }
        radius += step;
    }
    for j in 0..ps {
        let level = j * (ts + 1);
        for i in 0..ts {
            let s = i + level;
            let (a, b, c, d) = (s, s + ts + 1, s + ts + 2, s + 1);
            index.extend([a, b, d, b, c, d]);
        }
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// TorusGeometry( radius, tube, radialSegments, tubularSegments, arc ).
pub(super) fn torus(radius: f64, tube: f64, radial: u32, tubular: u32, arc: f64) -> G {
    let (start, length) = (0., PI * 2.);
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    for j in 0..=radial {
        let v = start + (j as f64 / radial as f64) * length;
        for i in 0..=tubular {
            let u = i as f64 / tubular as f64 * arc;
            let q = [
                (radius + tube * v.cos()) * u.cos(),
                (radius + tube * v.cos()) * u.sin(),
                tube * v.sin(),
            ];
            p.extend(q);
            let center = [radius * u.cos(), radius * u.sin(), 0.];
            n.extend(normalize([
                q[0] - center[0],
                q[1] - center[1],
                q[2] - center[2],
            ]));
            uv.extend([i as f64 / tubular as f64, j as f64 / radial as f64]);
        }
    }
    for j in 1..=radial {
        for i in 1..=tubular {
            let a = (tubular + 1) * j + i - 1;
            let b = (tubular + 1) * (j - 1) + i - 1;
            let c = (tubular + 1) * (j - 1) + i;
            let d = (tubular + 1) * j + i;
            index.extend([a, b, d, b, c, d]);
        }
    }
    geometry(
        vec![("position", 3, p), ("normal", 3, n), ("uv", 2, uv)],
        Some(index),
    )
}
/// LatheGeometry( points, segments ).
pub(super) fn lathe(points: &[[f64; 2]], segments: u32) -> G {
    let (start, length) = (0., PI * 2.);
    let inverse = 1. / segments as f64;
    let last = points.len() - 1;
    let mut init: Vec<V3> = vec![];
    let mut prev = [0.; 3];
    for j in 0..=last {
        if j == 0 {
            let (dx, dy) = (points[1][0] - points[0][0], points[1][1] - points[0][1]);
            let normal = [dy * 1., -dx, dy * 0.];
            prev = normal;
            init.push(normalize(normal));
        } else if j == last {
            init.push(prev);
        } else {
            let (dx, dy) = (
                points[j + 1][0] - points[j][0],
                points[j + 1][1] - points[j][1],
            );
            let current = [dy * 1., -dx, dy * 0.];
            init.push(normalize([
                current[0] + prev[0],
                current[1] + prev[1],
                current[2] + prev[2],
            ]));
            prev = current;
        }
    }
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    for i in 0..=segments {
        let phi = start + i as f64 * inverse * length;
        let (s, c) = (phi.sin(), phi.cos());
        for (j, q) in points.iter().enumerate() {
            p.extend([q[0] * s, q[1], q[0] * c]);
            uv.extend([i as f64 / segments as f64, j as f64 / last as f64]);
            n.extend([init[j][0] * s, init[j][1], init[j][0] * c]);
        }
    }
    let count = points.len() as u32;
    for i in 0..segments {
        for j in 0..last as u32 {
            let base = j + i * count;
            let (a, b, c, d) = (base, base + count, base + count + 1, base + 1);
            index.extend([a, b, d, c, d, b]);
        }
    }
    geometry(
        vec![("position", 3, p), ("uv", 2, uv), ("normal", 3, n)],
        Some(index),
    )
}
/// IcosahedronGeometry( radius, detail ): PolyhedronGeometry, non-indexed.
pub(super) fn icosahedron(radius: f64, detail: usize) -> G {
    let t = (1. + 5f64.sqrt()) / 2.;
    let vertices = [
        -1., t, 0., 1., t, 0., -1., -t, 0., 1., -t, 0., 0., -1., t, 0., 1., t, 0., -1., -t, 0., 1.,
        -t, t, 0., -1., t, 0., 1., -t, 0., -1., -t, 0., 1.,
    ];
    let indices = [
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7,
        1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9,
        8, 1,
    ];
    let lerp = |a: V3, b: V3, t: f64| {
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    };
    let vertex = |i: usize| [vertices[i * 3], vertices[i * 3 + 1], vertices[i * 3 + 2]];
    let mut buffer: Vec<V3> = vec![];
    for f in indices.chunks(3) {
        let (a, b, c) = (vertex(f[0]), vertex(f[1]), vertex(f[2]));
        let cols = detail + 1;
        let mut v: Vec<Vec<V3>> = vec![];
        for i in 0..=cols {
            let aj = lerp(a, c, i as f64 / cols as f64);
            let bj = lerp(b, c, i as f64 / cols as f64);
            let rows = cols - i;
            v.push(
                (0..=rows)
                    .map(|j| {
                        if j == 0 && i == cols {
                            aj
                        } else {
                            lerp(aj, bj, j as f64 / rows as f64)
                        }
                    })
                    .collect(),
            );
        }
        for i in 0..cols {
            for j in 0..2 * (cols - i) - 1 {
                let k = j / 2;
                if j % 2 == 0 {
                    buffer.extend([v[i][k + 1], v[i + 1][k], v[i][k]]);
                } else {
                    buffer.extend([v[i][k + 1], v[i + 1][k + 1], v[i + 1][k]]);
                }
            }
        }
    }
    for q in &mut buffer {
        let n = normalize(*q);
        *q = [n[0] * radius, n[1] * radius, n[2] * radius];
    }
    let azimuth = |q: V3| q[2].atan2(-q[0]);
    let inclination = |q: V3| (-q[1]).atan2((q[0] * q[0] + q[2] * q[2]).sqrt());
    let mut uv: Vec<f64> = vec![];
    for q in &buffer {
        uv.extend([
            azimuth(*q) / 2. / PI + 0.5,
            1. - (inclination(*q) / PI + 0.5),
        ]);
    }
    // correctUVs().
    for (f, j) in buffer.chunks(3).zip((0..).step_by(6)) {
        let centroid = [0, 1, 2].map(|k| (f[0][k] + f[1][k] + f[2][k]) / 3.);
        let azi = azimuth(centroid);
        for (q, stride) in f.iter().zip([j, j + 2, j + 4]) {
            let x = uv[stride];
            if azi < 0. && x == 1. {
                uv[stride] = x - 1.;
            }
            if q[0] == 0. && q[2] == 0. {
                uv[stride] = azi / 2. / PI + 0.5;
            }
        }
    }
    // correctSeam().
    for i in (0..uv.len()).step_by(6) {
        let x = [uv[i], uv[i + 2], uv[i + 4]];
        let max = x[0].max(x[1]).max(x[2]);
        let min = x[0].min(x[1]).min(x[2]);
        if max > 0.9 && min < 0.1 {
            for k in 0..3 {
                if x[k] < 0.2 {
                    uv[i + k * 2] += 1.;
                }
            }
        }
    }
    let p: Vec<f64> = buffer.iter().flatten().copied().collect();
    let mut g = geometry(
        vec![("position", 3, p.clone()), ("normal", 3, p), ("uv", 2, uv)],
        None,
    );
    if detail == 0 {
        g.compute_vertex_normals();
    } else {
        g.normalize_normals();
    }
    g
}
/// LoftGeometry( sections, { closed, capStart, capEnd } ).
pub(super) fn loft(sections: &[Vec<V3>], closed: bool, cap_start: bool, cap_end: bool) -> G {
    let l = super::super::geometry_loft::shapes::loft(sections, closed, cap_start, cap_end);
    G {
        attributes: vec![
            ("position", 3, l.position),
            ("uv", 2, l.uv),
            ("normal", 3, l.normal),
        ],
        index: Some(l.index),
    }
}
