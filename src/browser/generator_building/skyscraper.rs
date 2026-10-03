//! SkyscraperGenerator ( three.js r186 addon ): the seeded tripartite tower,
//! its authored modules and shells instanced over the footprint faces and
//! baked into one non-indexed geometry with a per-vertex partId and the
//! glass panes' interior rooms. Computed in f64 with the Float32Array stores
//! and three.js's geometry transforms ( normal matrices, renormalized
//! normals, computeVertexNormals ) at the points the generator applies them.
use super::super::text_shapes::{Path, Shape, area, triangulate};

type V3 = [f64; 3];
/// A Matrix4's elements ( column-major ).
type M4 = [f64; 16];
const IDENTITY: M4 = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];
const WALL: f32 = 0.;
const PIER: f32 = 1.;
const FRAME: f32 = 2.;
const ORNAMENT: f32 = 3.;
const GLASS: f32 = 4.;
const AC: f32 = 5.;
const SHOPGLASS: f32 = 6.;
const STORE: f32 = 7.;
const AWNING: f32 = 8.;
const WINDOW_HEIGHT_RATIO: f64 = 0.62;
const WINDOW_BORDER: f64 = 0.1;
const BRICK_HEIGHT: f64 = 0.3;
const BRICK_LENGTH: f64 = 0.6;
/// The masonry palette pickBuildingColor draws from.
const PALETTE: [u32; 17] = [
    0xa8553c, 0x9c4a34, 0x8a6a52, 0x7d6450, 0xc4a370, 0xb89a6f, 0xc2b183, 0xc6c0b2, 0xc6c0b2,
    0xbdb7a8, 0xd1ccbe, 0xb4afa1, 0x9a988f, 0x8b8983, 0xa5a39a, 0xdbd6cb, 0x7c868d,
];
/// Math.round: halves round up.
fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}
/// createRandom( seed ): mulberry32.
fn create_random(seed: u32) -> impl FnMut() -> f64 {
    let mut s = if seed == 0 { 1 } else { seed };
    move || {
        s = s.wrapping_add(0x6D2B79F5);
        let mut t = (s ^ (s >> 15)).wrapping_mul(1 | s);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        (t ^ (t >> 14)) as f64 / 4294967296.
    }
}
/// V8's Math.hypot: scaled by the largest magnitude, Kahan-summed.
fn hypot(values: &[f64]) -> f64 {
    let max = values.iter().fold(0f64, |m, v| m.max(v.abs()));
    if max == 0. {
        return 0.;
    }
    let (mut sum, mut compensation) = (0f64, 0f64);
    for v in values {
        let n = v.abs() / max;
        let summand = n * n - compensation;
        let preliminary = sum + summand;
        compensation = (preliminary - sum) - summand;
        sum = preliminary;
    }
    sum.sqrt() * max
}
/// pickBuildingColor( seed ).
pub(in crate::browser) fn pick_building_color(seed: f64) -> u32 {
    let h = ((seed * 12.9898).sin() * 43758.5453).abs();
    PALETTE[((h - h.floor()) * PALETTE.len() as f64).floor() as usize]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scaled(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
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
/// Vector3.normalize(): divideScalar( length || 1 ).
fn normalize(a: V3) -> V3 {
    let length = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    let inverse = 1. / if length == 0. { 1. } else { length };
    scaled(a, inverse)
}
/// Matrix4.makeBasis( u, v, n ).
fn basis(u: V3, v: V3, n: V3) -> M4 {
    [
        u[0], u[1], u[2], 0., v[0], v[1], v[2], 0., n[0], n[1], n[2], 0., 0., 0., 0., 1.,
    ]
}
fn set_position(mut m: M4, p: V3) -> M4 {
    m[12] = p[0];
    m[13] = p[1];
    m[14] = p[2];
    m
}
/// Matrix4.scale( v ).
fn scale(mut m: M4, s: V3) -> M4 {
    for (c, k) in s.iter().enumerate() {
        for r in 0..4 {
            m[c * 4 + r] *= k;
        }
    }
    m
}
/// Matrix3.getNormalMatrix( m ): setFromMatrix4, invert, transpose.
fn normal_matrix(m: &M4) -> [f64; 9] {
    let (n11, n21, n31) = (m[0], m[1], m[2]);
    let (n12, n22, n32) = (m[4], m[5], m[6]);
    let (n13, n23, n33) = (m[8], m[9], m[10]);
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    if det == 0. {
        return [0.; 9];
    }
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
fn rotation_x(angle: f64) -> M4 {
    let (s, c) = angle.sin_cos();
    [1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.]
}
fn rotation_y(angle: f64) -> M4 {
    let (s, c) = angle.sin_cos();
    [c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.]
}
fn translation(p: V3) -> M4 {
    set_position(IDENTITY, p)
}
/// A non-indexed geometry's Float32Array attributes.
#[derive(Clone, Default)]
pub(in crate::browser) struct Geo {
    pub(in crate::browser) position: Vec<f32>,
    pub(in crate::browser) normal: Vec<f32>,
    pub(in crate::browser) uv: Vec<f32>,
}
impl Geo {
    /// An indexed geometry's toNonIndexed().
    fn expand(position: &[f32], normal: &[f32], uv: &[f32], index: &[u32]) -> Self {
        let mut g = Self::default();
        for &i in index {
            let i = i as usize;
            g.position.extend_from_slice(&position[i * 3..i * 3 + 3]);
            g.normal.extend_from_slice(&normal[i * 3..i * 3 + 3]);
            g.uv.extend_from_slice(&uv[i * 2..i * 2 + 2]);
        }
        g
    }
    /// BufferGeometry.applyMatrix4: positions by the matrix, normals by its
    /// normal matrix and renormalized.
    fn apply(&mut self, m: &M4) {
        let n = normal_matrix(m);
        for p in self.position.chunks_mut(3) {
            let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64);
            let w = 1. / (m[3] * x + m[7] * y + m[11] * z + m[15]);
            p[0] = ((m[0] * x + m[4] * y + m[8] * z + m[12]) * w) as f32;
            p[1] = ((m[1] * x + m[5] * y + m[9] * z + m[13]) * w) as f32;
            p[2] = ((m[2] * x + m[6] * y + m[10] * z + m[14]) * w) as f32;
        }
        for v in self.normal.chunks_mut(3) {
            let (x, y, z) = (v[0] as f64, v[1] as f64, v[2] as f64);
            let t = normalize([
                n[0] * x + n[3] * y + n[6] * z,
                n[1] * x + n[4] * y + n[7] * z,
                n[2] * x + n[5] * y + n[8] * z,
            ]);
            v[0] = t[0] as f32;
            v[1] = t[1] as f32;
            v[2] = t[2] as f32;
        }
    }
    fn count(&self) -> usize {
        self.position.len() / 3
    }
}
/// mergeGeometries over non-indexed geometries.
fn merge(parts: Vec<Geo>) -> Geo {
    let mut out = Geo::default();
    for p in parts {
        out.position.extend(p.position);
        out.normal.extend(p.normal);
        out.uv.extend(p.uv);
    }
    out
}
/// PlaneGeometry( width, height ), non-indexed.
fn plane(width: f64, height: f64) -> Geo {
    let (hw, hh) = (width / 2., height / 2.);
    let (mut position, mut normal, mut uv) = (vec![], vec![], vec![]);
    for iy in 0..2 {
        let y = iy as f64 * height - hh;
        for ix in 0..2 {
            let x = ix as f64 * width - hw;
            position.extend([x as f32, -y as f32, 0.]);
            normal.extend([0., 0., 1.]);
            uv.extend([ix as f32, (1. - iy as f64) as f32]);
        }
    }
    Geo::expand(&position, &normal, &uv, &[0, 2, 1, 2, 3, 1])
}
/// buildPlane's axes ( u, v, w ), directions and extents.
type Plane = (usize, usize, usize, f64, f64, f64, f64, f64);
/// BoxGeometry( width, height, depth ), non-indexed.
fn boxed(width: f64, height: f64, depth: f64) -> Geo {
    let (mut position, mut normal, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    let mut vertices = 0u32;
    // buildPlane( u, v, w, udir, vdir, width, height, depth ).
    let planes: [Plane; 6] = [
        (2, 1, 0, -1., -1., depth, height, width),
        (2, 1, 0, 1., -1., depth, height, -width),
        (0, 2, 1, 1., 1., width, depth, height),
        (0, 2, 1, 1., -1., width, depth, -height),
        (0, 1, 2, 1., -1., width, height, depth),
        (0, 1, 2, -1., -1., width, height, -depth),
    ];
    for (u, v, w, udir, vdir, pw, ph, pd) in planes {
        for iy in 0..2 {
            let y = iy as f64 * ph - ph / 2.;
            for ix in 0..2 {
                let x = ix as f64 * pw - pw / 2.;
                let mut p = [0f64; 3];
                p[u] = x * udir;
                p[v] = y * vdir;
                p[w] = pd / 2.;
                position.extend(p.map(|c| c as f32));
                let mut n = [0f32; 3];
                n[w] = if pd > 0. { 1. } else { -1. };
                normal.extend(n);
                uv.extend([ix as f32, (1. - iy as f64) as f32]);
            }
        }
        let (a, b, c, d) = (vertices, vertices + 2, vertices + 3, vertices + 1);
        index.extend([a, b, d, b, c, d]);
        vertices += 4;
    }
    Geo::expand(&position, &normal, &uv, &index)
}
/// ShapeGeometry( shape, curveSegments ), non-indexed.
fn shape_geometry(shape: &Shape, curve_segments: usize) -> Geo {
    let mut vertices = shape.outline.points(curve_segments);
    let mut holes: Vec<Vec<[f64; 2]>> = shape
        .holes
        .iter()
        .map(|h| h.points(curve_segments))
        .collect();
    if area(&vertices) >= 0. {
        vertices.reverse();
    }
    for h in &mut holes {
        if area(h) < 0. {
            h.reverse();
        }
    }
    let faces = triangulate(&mut vertices, &mut holes);
    for h in &holes {
        vertices.extend(h);
    }
    let position: Vec<f32> = vertices
        .iter()
        .flat_map(|p| [p[0] as f32, p[1] as f32, 0.])
        .collect();
    let normal: Vec<f32> = vertices.iter().flat_map(|_| [0., 0., 1.]).collect();
    let uv: Vec<f32> = vertices
        .iter()
        .flat_map(|p| [p[0] as f32, p[1] as f32])
        .collect();
    let index: Vec<u32> = faces.iter().flatten().map(|&i| i as u32).collect();
    Geo::expand(&position, &normal, &uv, &index)
}
/// computeVertexNormals() of a non-indexed geometry: each face's cross
/// product stored, then normalizeNormals().
fn face_normals(position: &[f32]) -> Vec<f32> {
    let mut normal = vec![];
    for t in position.chunks(9) {
        let p = |k: usize| [t[k * 3] as f64, t[k * 3 + 1] as f64, t[k * 3 + 2] as f64];
        let (a, b, c) = (p(0), p(1), p(2));
        let cb = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
        let ab = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        let n = cross(cb, ab).map(|v| v as f32);
        let n = normalize(n.map(f64::from)).map(|v| v as f32);
        for _ in 0..3 {
            normal.extend(n);
        }
    }
    normal
}
/// ExtrudeGeometry( shape, { depth, bevelEnabled: false } ) of a simple
/// polygon ( curveSegments 12, one step ), with WorldUVGenerator's uvs.
fn extrude(points: &[[f64; 2]], depth: f64) -> Geo {
    let mut vertices = points.to_vec();
    if area(&vertices) >= 0. {
        vertices.reverse();
    }
    // mergeOverlappingPoints, including the wrap to the first point.
    let threshold_sq = 1e-10 * 1e-10;
    let mut i = 1;
    let mut prev = vertices[0];
    while i <= vertices.len() {
        let current_index = i % vertices.len();
        let current = vertices[current_index];
        let (dx, dy) = (current[0] - prev[0], current[1] - prev[1]);
        let scaling = current[0]
            .abs()
            .max(current[1].abs())
            .max(prev[0].abs())
            .max(prev[1].abs());
        if dx * dx + dy * dy <= threshold_sq * scaling * scaling {
            vertices.remove(current_index);
            continue;
        }
        prev = current;
        i += 1;
    }
    let contour = vertices.clone();
    let faces = triangulate(&mut contour.clone(), &mut []);
    let vlen = contour.len();
    let mut placeholder: Vec<V3> = vec![];
    for s in 0..=1 {
        for p in &contour {
            placeholder.push([p[0], p[1], depth / 1. * s as f64]);
        }
    }
    let (mut position, mut uv): (Vec<V3>, Vec<[f64; 2]>) = (vec![], vec![]);
    let f3 = |position: &mut Vec<V3>, uv: &mut Vec<[f64; 2]>, k: [usize; 3]| {
        for i in k {
            let v = placeholder[i];
            position.push(v);
            uv.push([v[0], v[1]]);
        }
    };
    for f in &faces {
        f3(&mut position, &mut uv, [f[2], f[1], f[0]]);
    }
    for f in &faces {
        f3(
            &mut position,
            &mut uv,
            [f[0] + vlen, f[1] + vlen, f[2] + vlen],
        );
    }
    for i in (0..vlen).rev() {
        let (j, k) = (i, if i == 0 { vlen - 1 } else { i - 1 });
        let (a, b, c, d) = (j, k, k + vlen, j + vlen);
        let [pa, pb, pc, pd] = [a, b, c, d].map(|i| placeholder[i]);
        let side = |p: V3, x: bool| [if x { p[0] } else { p[1] }, 1. - p[2]];
        let x = (pa[1] - pb[1]).abs() < (pa[0] - pb[0]).abs();
        let uvs = [side(pa, x), side(pb, x), side(pc, x), side(pd, x)];
        for (v, t) in [(pa, 0), (pb, 1), (pd, 3), (pb, 1), (pc, 2), (pd, 3)] {
            position.push(v);
            uv.push(uvs[t]);
        }
    }
    let position: Vec<f32> = position.iter().flatten().map(|&v| v as f32).collect();
    Geo {
        normal: face_normals(&position),
        position,
        uv: uv.iter().flatten().map(|&v| v as f32).collect(),
    }
}
/// LatheGeometry( points, segments ), non-indexed.
fn lathe(points: &[[f64; 2]], segments: usize) -> Geo {
    let mut init = vec![];
    let mut prev = [0f64; 3];
    let last = points.len() - 1;
    for j in 0..=last {
        if j == last {
            init.push(prev);
            break;
        }
        let (dx, dy) = (
            points[j + 1][0] - points[j][0],
            points[j + 1][1] - points[j][1],
        );
        let current = [dy * 1., -dx, dy * 0.];
        let n = if j == 0 { current } else { add(current, prev) };
        init.push(normalize(n));
        prev = current;
    }
    let (mut position, mut normal, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    let inverse = 1. / segments as f64;
    for i in 0..=segments {
        let phi = i as f64 * inverse * (std::f64::consts::PI * 2.);
        let (sin, cos) = (phi.sin(), phi.cos());
        for (j, p) in points.iter().enumerate() {
            position.extend([(p[0] * sin) as f32, p[1] as f32, (p[0] * cos) as f32]);
            uv.extend([
                (i as f64 / segments as f64) as f32,
                (j as f64 / last as f64) as f32,
            ]);
            normal.extend([
                (init[j][0] * sin) as f32,
                init[j][1] as f32,
                (init[j][0] * cos) as f32,
            ]);
        }
    }
    let count = points.len() as u32;
    for i in 0..segments as u32 {
        for j in 0..last as u32 {
            let base = j + i * count;
            let (a, b, c, d) = (base, base + count, base + count + 1, base + 1);
            index.extend([a, b, d, c, d, b]);
        }
    }
    Geo::expand(&position, &normal, &uv, &index)
}
/// The generator parameters after the seeded style and the snapping.
#[derive(Clone, Copy)]
struct Params {
    total_height: f64,
    footprint: [f64; 2],
    floor_height: f64,
    window_height: f64,
    bay_width: f64,
    string_course_every: usize,
    chamfer_width: f64,
    setback_depth: f64,
    ac_chance: f64,
    tier_base: f64,
    tier_crown: f64,
    pier_width: f64,
    pier_depth: f64,
    window_reveal: f64,
    string_course_height: f64,
    arch_bay_width_ratio: f64,
    arch_rise: f64,
    arcade: bool,
}
/// The page's GUI parameters.
#[derive(Clone, Copy)]
pub(in crate::browser) struct Settings {
    pub(in crate::browser) seed: f64,
    pub(in crate::browser) height: f64,
    pub(in crate::browser) width: f64,
    pub(in crate::browser) depth: f64,
    pub(in crate::browser) floor_height: f64,
    pub(in crate::browser) bay_width: f64,
    pub(in crate::browser) chamfer: f64,
    pub(in crate::browser) setback: f64,
    /// The caller's pierWidth and pierDepth over the seeded style's.
    pub(in crate::browser) pier: Option<(f64, f64)>,
    /// chamferCornerX and chamferCornerZ: the corner the chamfer cuts.
    pub(in crate::browser) chamfer_corner: (f64, f64),
    pub(in crate::browser) string_course_every: usize,
}
#[derive(Clone)]
struct Frame {
    origin: V3,
    u: V3,
    v: V3,
    n: V3,
    length: f64,
}
impl Frame {
    fn point(&self, u: f64, v: f64, w: f64) -> V3 {
        let mut p = self.origin;
        for (axis, s) in [(self.u, u), (self.v, v), (self.n, w)] {
            for k in 0..3 {
                p[k] += axis[k] * s;
            }
        }
        p
    }
    fn matrix(&self, u: f64, v: f64, w: f64) -> M4 {
        set_position(basis(self.u, self.v, self.n), self.point(u, v, w))
    }
    /// ( count, margin, width ).
    fn bays(&self, bay_width: f64) -> (usize, f64, f64) {
        let count = (self.length / bay_width).floor().max(1.);
        (
            count as usize,
            (self.length - count * bay_width) / 2.,
            bay_width,
        )
    }
}
fn box_matrix(frame: &Frame, u: f64, v: f64, w: f64, size: V3) -> M4 {
    set_position(
        scale(basis(frame.u, frame.v, frame.n), size),
        frame.point(u, v, w),
    )
}
fn footprint(width: f64, depth: f64, chamfer: f64, corner: (f64, f64)) -> Vec<[f64; 2]> {
    let (hw, hd) = (width / 2., depth / 2.);
    let c = chamfer.min(hw).min(hd);
    let corners = [[hw, hd], [-hw, hd], [-hw, -hd], [hw, -hd]];
    let (corner_x, corner_z) = corner;
    let sign = |v: f64| {
        if v > 0. {
            1.
        } else if v < 0. {
            -1.
        } else {
            v
        }
    };
    let mut points = vec![];
    for i in 0..4 {
        let corner = corners[i];
        if c > 0. && sign(corner[0]) == corner_x && sign(corner[1]) == corner_z {
            for other in [corners[(i + 3) % 4], corners[(i + 1) % 4]] {
                let (dx, dy) = (corner[0] - other[0], corner[1] - other[1]);
                let distance = (dx * dx + dy * dy).sqrt();
                let alpha = c / distance;
                points.push([
                    corner[0] + (other[0] - corner[0]) * alpha,
                    corner[1] + (other[1] - corner[1]) * alpha,
                ]);
            }
        } else {
            points.push(corner);
        }
    }
    points
}
fn faces(points: &[[f64; 2]]) -> Vec<Frame> {
    let up = [0., 1., 0.];
    let mut out = vec![];
    for i in 0..points.len() {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        let mut n = normalize([b[1] - a[1], 0., -(b[0] - a[0])]);
        let mid = [(a[0] + b[0]) / 2., 0., (a[1] + b[1]) / 2.];
        if dot(n, mid) < 0. {
            n = n.map(|v| -v);
        }
        let u = normalize(cross(up, n));
        let (pa, pb) = ([a[0], 0., a[1]], [b[0], 0., b[1]]);
        let d = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let origin = if dot(d, u) > 0. { pa } else { pb };
        out.push(Frame {
            origin,
            u,
            v: up,
            n,
            length,
        });
    }
    out
}
fn add_wall(
    target: &mut Vec<M4>,
    frame: &Frame,
    bottom: f64,
    top: f64,
    thickness: f64,
    front: f64,
) {
    let h = top - bottom;
    target.push(box_matrix(
        frame,
        frame.length / 2.,
        bottom + h / 2.,
        front - thickness / 2.,
        [frame.length + thickness * 2., h, thickness],
    ));
}
fn add_spandrel_bands(target: &mut Vec<M4>, frame: &Frame, bottom: f64, height: f64, p: &Params) {
    let floors = round(height / p.floor_height).max(1.);
    let fh = height / floors;
    let band_height = p.floor_height - p.window_height;
    let band_length = (frame.length - 0.6).max(0.2);
    let v_top = bottom + height;
    for f in 0..=floors as usize {
        let center = bottom + f as f64 * fh;
        let top = (center + band_height / 2.).min(v_top);
        let low = (center - band_height / 2.).max(bottom);
        let h = top - low;
        if h <= 0. {
            continue;
        }
        target.push(box_matrix(
            frame,
            frame.length / 2.,
            (top + low) / 2.,
            -0.3,
            [band_length, h, 0.6],
        ));
    }
}
fn slab(points: &[[f64; 2]], y: f64, thickness: f64) -> Geo {
    let inset = 0.8;
    let (mut cx, mut cz) = (0., 0.);
    for p in points {
        cx += p[0];
        cz += p[1];
    }
    cx /= points.len() as f64;
    cz /= points.len() as f64;
    let mut signed = 0.;
    for i in 0..points.len() {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        signed += a[0] * b[1] - b[0] * a[1];
    }
    let mut pts = points.to_vec();
    if signed < 0. {
        pts.reverse();
    }
    let outline: Vec<[f64; 2]> = pts
        .iter()
        .map(|p| {
            let (dx, dz) = (cx - p[0], cz - p[1]);
            let d = hypot(&[dx, dz]);
            let d = if d == 0. { 1. } else { d };
            [p[0] + dx / d * inset, p[1] + dz / d * inset]
        })
        .collect();
    let mut g = extrude(&outline, thickness);
    g.apply(&rotation_x(std::f64::consts::PI / 2.));
    g.apply(&translation([0., y - 0.2, 0.]));
    g
}
fn add_cornice(target: &mut Vec<M4>, frame: &Frame, bottom: f64, height: f64, depth: f64) {
    let l = frame.length;
    target.push(box_matrix(
        frame,
        l / 2.,
        bottom + height * 0.275,
        depth / 2.,
        [l, height * 0.55, depth],
    ));
    target.push(box_matrix(
        frame,
        l / 2.,
        bottom + height * 0.775,
        depth * 0.85,
        [l, height * 0.45, depth * 1.7],
    ));
}
fn add_parapet(target: &mut Vec<M4>, frame: &Frame, top: f64, p: &Params) {
    let height = 1.4;
    target.push(box_matrix(
        frame,
        frame.length / 2.,
        top + height / 2.,
        p.pier_depth * 0.4,
        [frame.length, height, p.pier_depth * 0.8],
    ));
}
fn arch_reveals(holes: &[Path], depth: f64, curve_segments: usize) -> Geo {
    let (mut position, mut normal, mut uv) = (vec![], vec![], vec![]);
    for hole in holes {
        let points = hole.points(curve_segments);
        for w in points.windows(2) {
            let (a, b) = (w[0], w[1]);
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let h = hypot(&[dx, dy]);
            let inv = 1. / if h == 0. { 1. } else { h };
            let (nx, ny) = (dy * inv, -dx * inv);
            position.extend([
                a[0], a[1], 0., a[0], a[1], -depth, b[0], b[1], -depth, a[0], a[1], 0., b[0], b[1],
                -depth, b[0], b[1], 0.,
            ]);
            for _ in 0..6 {
                normal.extend([nx, ny, 0.]);
                uv.extend([0., 0.]);
            }
        }
    }
    let f = |v: Vec<f64>| v.into_iter().map(|x| x as f32).collect();
    Geo {
        position: f(position),
        normal: f(normal),
        uv: f(uv),
    }
}
fn add_arcade(target: &mut Vec<Geo>, frame: &Frame, height: f64, p: &Params) {
    let arch_width = p.bay_width * p.arch_bay_width_ratio;
    let (count, margin, _) = frame.bays(arch_width);
    let sill = height * 0.04;
    let spring = height * 0.55;
    let apex = (height * 0.96).min(spring + (arch_width / 2.) * (0.8 + p.arch_rise));
    let mut shape = Shape::default();
    shape.outline.move_to([0., 0.]);
    shape.outline.line_to([frame.length, 0.]);
    shape.outline.line_to([frame.length, height]);
    shape.outline.line_to([0., height]);
    shape.outline.line_to([0., 0.]);
    for i in 0..count {
        let cx = margin + (i as f64 + 0.5) * arch_width;
        let hw = arch_width * 0.34;
        let mut hole = Path::default();
        hole.move_to([cx - hw, sill]);
        hole.line_to([cx - hw, spring]);
        hole.quadratic_to([cx - hw, apex], [cx, apex]);
        hole.quadratic_to([cx + hw, apex], [cx + hw, spring]);
        hole.line_to([cx + hw, sill]);
        hole.line_to([cx - hw, sill]);
        shape.holes.push(hole);
    }
    let thickness = 1.1;
    let front = shape_geometry(&shape, 8);
    let reveals = arch_reveals(&shape.holes, thickness, 8);
    let mut g = merge(vec![front, reveals]);
    g.apply(&frame.matrix(0., 0., 0.));
    target.push(g);
    let mut back = plane(frame.length, height);
    back.apply(&frame.matrix(frame.length / 2., height / 2., -thickness - 0.4));
    target.push(back);
}
#[derive(Default)]
struct Parts {
    windows: Vec<M4>,
    glass: Vec<M4>,
    glass_rooms: Vec<(V3, [f64; 2])>,
    back_walls: Vec<M4>,
    bands: Vec<M4>,
    shop_glass: Vec<M4>,
    shop_rooms: Vec<(V3, [f64; 2])>,
    mullions: Vec<M4>,
    store_bands: Vec<M4>,
    awnings: Vec<M4>,
    /// Pier height key ( × 1000 ) → matrices, in insertion order.
    piers: Vec<(i64, Vec<M4>)>,
    trim: Vec<M4>,
    ac_units: Vec<M4>,
    finials: Vec<M4>,
    extras: Vec<Geo>,
}
impl Parts {
    fn add_pier(&mut self, frame: &Frame, u: f64, bottom: f64, height: f64) {
        let key = round(height * 1000.) as i64;
        let m = frame.matrix(u, bottom, 0.);
        match self.piers.iter_mut().find(|(k, _)| *k == key) {
            Some((_, list)) => list.push(m),
            None => self.piers.push((key, vec![m])),
        }
    }
}
fn add_storefront(frame: &Frame, height: f64, p: &Params, parts: &mut Parts) {
    let bulkhead = 0.5;
    let fascia = (height * 0.22).min(0.9);
    let glass_bottom = bulkhead;
    let glass_height = height - fascia - bulkhead;
    let len = frame.length;
    parts.store_bands.push(box_matrix(
        frame,
        len / 2.,
        bulkhead / 2.,
        0.,
        [len, bulkhead, 0.55],
    ));
    parts.store_bands.push(box_matrix(
        frame,
        len / 2.,
        height - fascia / 2.,
        0.08,
        [len, fascia, 0.72],
    ));
    add_wall(&mut parts.back_walls, frame, 0., height, 0.8, -0.6);
    let shop_width = (p.bay_width * 2.).max(4.);
    let (count, margin, width) = frame.bays(shop_width);
    for i in 0..count {
        let x0 = margin + i as f64 * width;
        let cx = x0 + width / 2.;
        parts.add_pier(frame, x0, 0., height);
        let reveal = 0.18;
        let gw = width - p.pier_width - 0.12;
        let cy = glass_bottom + glass_height / 2.;
        parts
            .shop_glass
            .push(box_matrix(frame, cx, cy, -reveal, [gw, glass_height, 1.]));
        parts
            .shop_rooms
            .push((frame.point(cx, cy, -reveal), [gw, glass_height]));
        parts.mullions.push(box_matrix(
            frame,
            x0 + width / 3.,
            glass_bottom + glass_height / 2.,
            0.,
            [0.08, glass_height, 0.16],
        ));
        parts.mullions.push(box_matrix(
            frame,
            x0 + width * 2. / 3.,
            glass_bottom + glass_height / 2.,
            0.,
            [0.08, glass_height, 0.16],
        ));
        let r =
            (i as f64 * 23.7 + frame.origin[0] * 0.21 + frame.origin[2] * 0.11).sin() * 43758.5453;
        if r - r.floor() < 0.5 {
            let depth = 1.3;
            parts.awnings.push(box_matrix(
                frame,
                cx,
                height - fascia - 0.12,
                depth / 2. + 0.05,
                [width - 0.3, 0.14, depth],
            ));
        }
    }
}
fn floor_hash(f: usize, frame: &Frame, k: f64) -> f64 {
    let s = (f as f64 * 12.9898 + frame.origin[0] * 0.07 + frame.origin[2] * 0.131 + k).sin()
        * 43758.5453;
    s - s.floor()
}
fn add_windows(frame: &Frame, parts: &mut Parts, ac: bool, bottom: f64, height: f64, p: &Params) {
    let (count, margin, width) = frame.bays(p.bay_width);
    let floors = round(height / p.floor_height).max(1.) as usize;
    let fh = height / floors as f64;
    let ac_w = ((p.bay_width - p.pier_width) * 0.55).min(0.66);
    let ac_h = ac_w * 0.6;
    let ac_d = ac_w * 0.5;
    let ac_v = -p.window_height / 2. + ac_h / 2. + WINDOW_BORDER;
    let ac_fits = ac_w >= (width - p.pier_width) * 0.34;
    for f in 0..floors {
        let cy = bottom + (f as f64 + 0.5) * fh;
        let room_bays = if floor_hash(f, frame, 0.) > 0.5 { 3 } else { 2 };
        let room_phase = (floor_hash(f, frame, 1.) * room_bays as f64).floor() as usize;
        for b in 0..count {
            let cx = margin + (b as f64 + 0.5) * width;
            parts.windows.push(frame.matrix(cx, cy, 0.));
            parts.glass.push(frame.matrix(cx, cy, -p.window_reveal));
            let room = (b + room_phase) / room_bays;
            let start = (room * room_bays).saturating_sub(room_phase);
            let end = count.min((room + 1) * room_bays - room_phase);
            let span = (end - start) as f64;
            parts.glass_rooms.push((
                frame.point(
                    margin + (start as f64 + span / 2.) * width,
                    cy,
                    -p.window_reveal,
                ),
                [span * width, fh - 1.],
            ));
            if ac && ac_fits {
                let r = (f as f64 * 41.3
                    + b as f64 * 12.7
                    + frame.origin[0] * 0.13
                    + frame.origin[2] * 0.31)
                    .sin()
                    * 43758.5453;
                let w0 = ac_d / 2. - p.window_reveal + 0.04;
                if r - r.floor() < p.ac_chance {
                    parts
                        .ac_units
                        .push(box_matrix(frame, cx, cy + ac_v, w0, [ac_w, ac_h, ac_d]));
                }
            }
        }
    }
}
fn pier_geometry(p: &Params, height: f64) -> Geo {
    let mut back = boxed(p.pier_width, height, p.pier_depth * 0.6);
    back.apply(&translation([0., height / 2., p.pier_depth * 0.3]));
    let pilaster = (height - 0.6).max(1.);
    let mut front = boxed(p.pier_width * 0.55, pilaster, p.pier_depth * 0.45);
    front.apply(&translation([
        0.,
        pilaster / 2.,
        p.pier_depth * 0.6 + p.pier_depth * 0.225,
    ]));
    merge(vec![back, front])
}
fn window_geometry(p: &Params) -> Geo {
    let w = p.bay_width - p.pier_width;
    let h = p.window_height;
    let depth = p.window_reveal;
    let iw = w / 2. - WINDOW_BORDER;
    let ih = h / 2. - WINDOW_BORDER;
    let mut shape = Shape::default();
    for (k, pt) in [
        [-w / 2., -h / 2.],
        [w / 2., -h / 2.],
        [w / 2., h / 2.],
        [-w / 2., h / 2.],
        [-w / 2., -h / 2.],
    ]
    .into_iter()
    .enumerate()
    {
        if k == 0 {
            shape.outline.move_to(pt);
        } else {
            shape.outline.line_to(pt);
        }
    }
    let mut hole = Path::default();
    for (k, pt) in [[-iw, -ih], [-iw, ih], [iw, ih], [iw, -ih], [-iw, -ih]]
        .into_iter()
        .enumerate()
    {
        if k == 0 {
            hole.move_to(pt);
        } else {
            hole.line_to(pt);
        }
    }
    shape.holes.push(hole);
    let front = shape_geometry(&shape, 12);
    let wall = |x: f64, y: f64, rx: f64, ry: f64, sw: f64, sh: f64| {
        let mut pl = plane(sw, sh);
        pl.apply(&rotation_x(rx));
        pl.apply(&rotation_y(ry));
        pl.apply(&translation([x, y, -depth / 2.]));
        pl
    };
    let half = std::f64::consts::PI / 2.;
    let left = wall(-iw, 0., 0., half, depth, ih * 2.);
    let right = wall(iw, 0., 0., -half, depth, ih * 2.);
    let sill = wall(0., -ih, -half, 0., iw * 2., depth);
    let head = wall(0., ih, half, 0., iw * 2., depth);
    let mut transom = plane(iw * 2., 0.05);
    transom.apply(&translation([0., h * 0.04, -depth + 0.02]));
    merge(vec![front, left, right, sill, head, transom])
}
fn glass_geometry(p: &Params) -> Geo {
    plane(
        p.bay_width - p.pier_width - WINDOW_BORDER * 2.,
        p.window_height - WINDOW_BORDER * 2.,
    )
}
fn finial_geometry(p: &Params) -> Geo {
    let s = p.pier_width;
    lathe(
        &[
            [0., 0.],
            [s * 0.9, 0.],
            [s * 0.9, s * 0.4],
            [s * 0.55, s * 1.],
            [0., s * 3.2],
        ],
        8,
    )
}
/// The baked building: the vertex attributes and the bounding sphere.
pub(in crate::browser) struct Building {
    pub(in crate::browser) position: Vec<f32>,
    pub(in crate::browser) normal: Vec<f32>,
    pub(in crate::browser) uv: Vec<f32>,
    pub(in crate::browser) part_id: Vec<f32>,
    pub(in crate::browser) room_center: Vec<f32>,
    pub(in crate::browser) room_size: Vec<f32>,
    pub(in crate::browser) center: V3,
    pub(in crate::browser) radius: f64,
}
struct Group {
    geometry: Geo,
    matrices: Vec<M4>,
    part: f32,
    rooms: Option<Vec<(V3, [f64; 2])>>,
    rigid: bool,
}
fn bake(groups: &[Group]) -> Building {
    let total: usize = groups
        .iter()
        .map(|g| g.geometry.count() * g.matrices.len())
        .sum();
    let mut b = Building {
        position: Vec::with_capacity(total * 3),
        normal: Vec::with_capacity(total * 3),
        uv: Vec::with_capacity(total * 2),
        part_id: Vec::with_capacity(total),
        room_center: Vec::with_capacity(total * 3),
        room_size: Vec::with_capacity(total * 2),
        center: [0.; 3],
        radius: 0.,
    };
    let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for g in groups {
        let (pos, nor, uv) = (&g.geometry.position, &g.geometry.normal, &g.geometry.uv);
        for (i, e) in g.matrices.iter().enumerate() {
            let room = g.rooms.as_ref().map(|r| r[i]);
            let n: [f64; 9] = if g.rigid {
                [e[0], e[1], e[2], e[4], e[5], e[6], e[8], e[9], e[10]]
            } else {
                normal_matrix(e)
            };
            for v in 0..g.geometry.count() {
                let (x, y, z) = (
                    pos[v * 3] as f64,
                    pos[v * 3 + 1] as f64,
                    pos[v * 3 + 2] as f64,
                );
                let world = [
                    e[0] * x + e[4] * y + e[8] * z + e[12],
                    e[1] * x + e[5] * y + e[9] * z + e[13],
                    e[2] * x + e[6] * y + e[10] * z + e[14],
                ];
                // The AABB reads the f64 positions before the Float32Array store.
                for k in 0..3 {
                    min[k] = min[k].min(world[k]);
                    max[k] = max[k].max(world[k]);
                }
                b.position.extend(world.map(|c| c as f32));
                let (nx, ny, nz) = (
                    nor[v * 3] as f64,
                    nor[v * 3 + 1] as f64,
                    nor[v * 3 + 2] as f64,
                );
                let t = [
                    n[0] * nx + n[3] * ny + n[6] * nz,
                    n[1] * nx + n[4] * ny + n[7] * nz,
                    n[2] * nx + n[5] * ny + n[8] * nz,
                ];
                let length = (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt();
                let inv = 1. / if length == 0. { 1. } else { length };
                b.normal.extend(t.map(|c| (c * inv) as f32));
                b.uv.extend_from_slice(&uv[v * 2..v * 2 + 2]);
                b.part_id.push(g.part);
                let (center, size) = room.unwrap_or(([0.; 3], [0.; 2]));
                b.room_center.extend(center.map(|c| c as f32));
                b.room_size.extend(size.map(|c| c as f32));
            }
        }
    }
    b.center = [0, 1, 2].map(|k| (min[k] + max[k]) / 2.);
    b.radius = hypot(&[max[0] - min[0], max[1] - min[1], max[2] - min[2]]) / 2.;
    b
}
/// SkyscraperGenerator( parameters ).build().
pub(in crate::browser) fn build(s: &Settings) -> Building {
    let mut random = create_random(s.seed as u32);
    // randomStyle( random ), in its property order.
    let tier_base = 0.10 + random() * 0.07;
    let tier_crown = 0.08 + random() * 0.08;
    let _style_width = 26. + random() * 18.;
    let _style_depth = 20. + random() * 14.;
    let mut pier_width = 0.4 + random() * 0.4;
    let mut pier_depth = 0.3 + random() * 0.3;
    if let Some((w, d)) = s.pier {
        (pier_width, pier_depth) = (w, d);
    }
    let window_reveal = 0.12 + random() * 0.1;
    let string_course_height = 0.5 + random() * 0.5;
    let arch_bay_width_ratio = round(1.5 + random() * 1.5);
    let arch_rise = 0.4 + random() * 0.5;
    let arcade = random() < 0.22;
    let v_module = BRICK_HEIGHT * 2.;
    let floor_height = (v_module * 3.).max(round(s.floor_height / v_module) * v_module);
    let mut p = Params {
        total_height: s.height,
        footprint: [s.width, s.depth],
        floor_height,
        window_height: round(floor_height * WINDOW_HEIGHT_RATIO / v_module) * v_module,
        bay_width: (BRICK_LENGTH * 3.).max(round(s.bay_width / BRICK_LENGTH) * BRICK_LENGTH),
        string_course_every: s.string_course_every,
        chamfer_width: s.chamfer,
        setback_depth: s.setback,
        ac_chance: 0.12,
        tier_base,
        tier_crown,
        pier_width: BRICK_LENGTH.max(round(pier_width / BRICK_LENGTH) * BRICK_LENGTH),
        pier_depth,
        window_reveal,
        string_course_height,
        arch_bay_width_ratio,
        arch_rise,
        arcade,
    };
    let floors = round(p.total_height / p.floor_height).max(3.);
    let base_floors = round(floors * p.tier_base).max(1.);
    let crown_floors = round(floors * p.tier_crown).max(1.);
    let shaft_floors = (floors - base_floors - crown_floors).max(1.);
    let base_height = base_floors * p.floor_height;
    let crown_height = crown_floors * p.floor_height;
    let shaft_height = shaft_floors * p.floor_height;
    p.total_height = base_height + shaft_height + crown_height;
    let base_top = base_height;
    let shaft_top = base_height + shaft_height;
    let mut parts = Parts::default();
    let full = footprint(
        p.footprint[0],
        p.footprint[1],
        p.chamfer_width,
        s.chamfer_corner,
    );
    let full_faces = faces(&full);
    let inset = p.setback_depth * p.bay_width;
    let crown = footprint(
        (p.footprint[0] - inset * 2.).max(p.bay_width * 2.),
        (p.footprint[1] - inset * 2.).max(p.bay_width * 2.),
        (p.chamfer_width - inset).max(0.),
        s.chamfer_corner,
    );
    let crown_faces = faces(&crown);
    let crown_cornice = p.string_course_height * 1.6;
    let ground_height = p.floor_height;
    let use_arcade = p.arcade && base_height > ground_height * 1.5;
    // ( faces, bottom, height, pier height, AC units ).
    let mut tiers = vec![
        (&full_faces, base_top, shaft_height, shaft_height, true),
        (
            &crown_faces,
            shaft_top,
            crown_height,
            crown_height - crown_cornice,
            false,
        ),
    ];
    if !use_arcade && base_height > ground_height + 0.1 {
        tiers.push((
            &full_faces,
            ground_height,
            base_height - ground_height,
            base_height - ground_height,
            false,
        ));
    }
    for (tier_faces, bottom, height, pier_height, ac) in tiers {
        for frame in tier_faces {
            add_windows(frame, &mut parts, ac, bottom, height, &p);
            add_wall(
                &mut parts.back_walls,
                frame,
                bottom,
                bottom + height,
                0.8,
                -0.6,
            );
            add_spandrel_bands(&mut parts.bands, frame, bottom, height, &p);
            let (count, margin, width) = frame.bays(p.bay_width);
            for i in 0..count {
                parts.add_pier(frame, margin + i as f64 * width, bottom, pier_height);
            }
        }
    }
    for frame in &full_faces {
        if use_arcade {
            add_arcade(&mut parts.extras, frame, base_height, &p);
        } else {
            add_storefront(frame, ground_height, &p, &mut parts);
        }
        add_cornice(
            &mut parts.trim,
            frame,
            base_top - p.string_course_height,
            p.string_course_height,
            0.5,
        );
    }
    if p.string_course_every > 0 {
        let mut f = p.string_course_every;
        while (f as f64) < shaft_floors {
            for frame in &full_faces {
                add_cornice(
                    &mut parts.trim,
                    frame,
                    base_top + f as f64 * p.floor_height - p.string_course_height * 0.5,
                    p.string_course_height,
                    0.3,
                );
            }
            f += p.string_course_every;
        }
    }
    for frame in &crown_faces {
        add_cornice(
            &mut parts.trim,
            frame,
            p.total_height - crown_cornice,
            crown_cornice,
            0.9,
        );
        add_parapet(&mut parts.trim, frame, p.total_height, &p);
        let (count, margin, width) = frame.bays(p.bay_width);
        let top = shaft_top + crown_height;
        for i in 0..count {
            parts.finials.push(translation(frame.point(
                margin + i as f64 * width,
                top,
                p.pier_depth * 0.5,
            )));
        }
    }
    parts.extras.push(slab(&full, shaft_top, 0.6));
    parts.extras.push(slab(&crown, p.total_height, 0.6));
    let unit_box = boxed(1., 1., 1.);
    let group = |geometry: Geo, matrices: Vec<M4>, part: f32, rigid: bool| Group {
        geometry,
        matrices,
        part,
        rooms: None,
        rigid,
    };
    let mut groups = vec![
        group(window_geometry(&p), parts.windows, FRAME, true),
        Group {
            geometry: glass_geometry(&p),
            matrices: parts.glass,
            part: GLASS,
            rooms: Some(parts.glass_rooms),
            rigid: true,
        },
        Group {
            geometry: plane(1., 1.),
            matrices: parts.shop_glass,
            part: SHOPGLASS,
            rooms: Some(parts.shop_rooms),
            rigid: false,
        },
        group(unit_box.clone(), parts.mullions, FRAME, false),
        group(unit_box.clone(), parts.store_bands, STORE, false),
        group(unit_box.clone(), parts.awnings, AWNING, false),
        group(unit_box.clone(), parts.bands, WALL, false),
    ];
    for (key, matrices) in parts.piers {
        groups.push(group(
            pier_geometry(&p, key as f64 / 1000.),
            matrices,
            PIER,
            true,
        ));
    }
    groups.push(group(unit_box.clone(), parts.trim, WALL, false));
    groups.push(group(unit_box.clone(), parts.ac_units, AC, false));
    groups.push(group(finial_geometry(&p), parts.finials, ORNAMENT, true));
    for g in parts.extras {
        groups.push(group(g, vec![IDENTITY], WALL, true));
    }
    groups.push(group(unit_box, parts.back_walls, WALL, false));
    bake(&groups)
}
/// The page's shadow-catching ground: PlaneGeometry( size, size )
/// .rotateX( -π / 2 ), non-indexed.
pub(in crate::browser) fn ground(size: f64) -> Geo {
    let mut g = plane(size, size);
    g.apply(&rotation_x(-std::f64::consts::PI / 2.));
    g
}
