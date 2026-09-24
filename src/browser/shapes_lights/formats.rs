//! STLLoader, Earcut for simple polygons and ExtrudeGeometry, as the pinned
//! sources compute them.
use super::super::helpers_formats::formats::parse_float;
use crate::{Error, Result, curve::CatmullRomCurve3, math::Vector3};

fn bad(what: &str) -> Error {
    Error::Asset(what.to_string())
}

// ---------------------------------------------------------------- STL

pub(super) struct Stl {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    /// Face colors in linear space and the header's alpha, when the file has them.
    pub colors: Option<(Vec<f32>, f64)>,
}
/// STLLoader.parse: binary (with the Materialise COLOR= header) or ASCII solids.
pub(super) fn parse_stl(data: &[u8]) -> Result<Stl> {
    let u32_at = |o: usize| -> Result<u32> {
        data.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| bad("STL: truncated"))
    };
    let f32_at = |o: usize| -> Result<f32> { Ok(f32::from_bits(u32_at(o)?)) };
    let binary = data.len() >= 84 && {
        let faces = u32_at(80)? as usize;
        80 + 4 + faces * 50 == data.len()
            || !(0..5).any(|off| data.get(off..off + 5) == Some(b"solid"))
    };
    if binary {
        let faces = u32_at(80)? as usize;
        let mut header = None;
        for index in 0..70 {
            if data[index..index + 4] == *b"COLO"
                && data[index + 4] == b'R'
                && data[index + 5] == b'='
            {
                header = Some([6, 7, 8, 9].map(|k| data[index + k] as f64 / 255.));
            }
        }
        let (mut positions, mut normals) = (vec![], vec![]);
        let mut colors = header.map(|_| vec![]);
        for face in 0..faces {
            let start = 84 + face * 50;
            let n = [f32_at(start)?, f32_at(start + 4)?, f32_at(start + 8)?];
            let rgb = header.map(|h| {
                let packed = u16::from_le_bytes([data[start + 48], data[start + 49]]);
                if packed & 0x8000 == 0 {
                    [packed & 0x1f, (packed >> 5) & 0x1f, (packed >> 10) & 0x1f]
                        .map(|c| c as f64 / 31.)
                } else {
                    [h[0], h[1], h[2]]
                }
            });
            for i in 1..=3 {
                let v = start + i * 12;
                positions.extend([f32_at(v)?, f32_at(v + 4)?, f32_at(v + 8)?]);
                normals.extend(n);
                if let (Some(colors), Some(rgb)) = (&mut colors, rgb) {
                    // color.setRGB( r, g, b, SRGBColorSpace ), stored as Float32.
                    let c = crate::math::Color::from_srgb(rgb[0], rgb[1], rgb[2]).0;
                    colors.extend([c.x as f32, c.y as f32, c.z as f32]);
                }
            }
        }
        return Ok(Stl {
            positions,
            normals,
            colors: colors.zip(header.map(|h| h[3])),
        });
    }
    let text = String::from_utf8_lossy(data);
    let (mut positions, mut normals) = (vec![], vec![]);
    let mut normal = [0f32; 3];
    let mut tokens = text.split_whitespace().peekable();
    while let Some(t) = tokens.next() {
        let three = |tokens: &mut std::iter::Peekable<std::str::SplitWhitespace>| {
            [0; 3].map(|_| tokens.next().map(parse_float).unwrap_or(f64::NAN) as f32)
        };
        match t {
            "normal" => normal = three(&mut tokens),
            "vertex" => {
                positions.extend(three(&mut tokens));
                normals.extend(normal);
            }
            _ => {}
        }
    }
    Ok(Stl {
        positions,
        normals,
        colors: None,
    })
}

// ---------------------------------------------------------------- Earcut

#[derive(Clone)]
struct Node {
    i: usize,
    x: f64,
    y: f64,
    prev: usize,
    next: usize,
}
struct Ring {
    nodes: Vec<Node>,
}
impl Ring {
    fn insert(&mut self, i: usize, x: f64, y: f64, last: Option<usize>) -> usize {
        let p = self.nodes.len();
        self.nodes.push(Node {
            i,
            x,
            y,
            prev: p,
            next: p,
        });
        if let Some(last) = last {
            let ln = self.nodes[last].next;
            self.nodes[p].next = ln;
            self.nodes[p].prev = last;
            self.nodes[ln].prev = p;
            self.nodes[last].next = p;
        }
        p
    }
    fn remove(&mut self, p: usize) {
        let (prev, next) = (self.nodes[p].prev, self.nodes[p].next);
        self.nodes[next].prev = prev;
        self.nodes[prev].next = next;
    }
    fn area(&self, p: usize, q: usize, r: usize) -> f64 {
        let (p, q, r) = (&self.nodes[p], &self.nodes[q], &self.nodes[r]);
        (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
    }
    fn equals(&self, a: usize, b: usize) -> bool {
        self.nodes[a].x == self.nodes[b].x && self.nodes[a].y == self.nodes[b].y
    }
    fn filter(&mut self, start: usize, end: Option<usize>) -> usize {
        let mut end = end.unwrap_or(start);
        let mut p = start;
        loop {
            let mut again = false;
            let (prev, next) = (self.nodes[p].prev, self.nodes[p].next);
            if self.equals(p, next) || self.area(prev, p, next) == 0. {
                self.remove(p);
                p = prev;
                end = prev;
                if p == self.nodes[p].next {
                    break;
                }
                again = true;
            } else {
                p = next;
            }
            if !again && p == end {
                break;
            }
        }
        end
    }
    fn is_ear(&self, ear: usize) -> bool {
        let (a, c) = (self.nodes[ear].prev, self.nodes[ear].next);
        if self.area(a, ear, c) >= 0. {
            return false;
        }
        let (na, nb, nc) = (&self.nodes[a], &self.nodes[ear], &self.nodes[c]);
        let (x0, y0) = (na.x.min(nb.x).min(nc.x), na.y.min(nb.y).min(nc.y));
        let (x1, y1) = (na.x.max(nb.x).max(nc.x), na.y.max(nb.y).max(nc.y));
        let mut p = nc.next;
        while p != a {
            let n = &self.nodes[p];
            if n.x >= x0
                && n.x <= x1
                && n.y >= y0
                && n.y <= y1
                && !(na.x == n.x && na.y == n.y)
                && inside(na.x, na.y, nb.x, nb.y, nc.x, nc.y, n.x, n.y)
                && self.area(n.prev, p, n.next) >= 0.
            {
                return false;
            }
            p = n.next;
        }
        true
    }
    fn earcut(&mut self, ear: usize, out: &mut Vec<usize>, pass: u8) -> Result<()> {
        let mut ear = ear;
        let mut stop = ear;
        while self.nodes[ear].prev != self.nodes[ear].next {
            let (prev, next) = (self.nodes[ear].prev, self.nodes[ear].next);
            if self.is_ear(ear) {
                out.extend([self.nodes[prev].i, self.nodes[ear].i, self.nodes[next].i]);
                self.remove(ear);
                ear = self.nodes[next].next;
                stop = ear;
                continue;
            }
            ear = next;
            if ear == stop {
                if pass == 0 {
                    let e = self.filter(ear, None);
                    self.earcut(e, out, 1)?;
                } else {
                    // Self-intersecting inputs take earcut's cure and split passes,
                    // which the examples' simple shapes never reach.
                    return Err(bad("Earcut: polygon needs the intersection passes"));
                }
                break;
            }
        }
        Ok(())
    }
}
#[allow(clippy::too_many_arguments)]
fn inside(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, px: f64, py: f64) -> bool {
    (cx - px) * (ay - py) >= (ax - px) * (cy - py)
        && (ax - px) * (by - py) >= (bx - px) * (ay - py)
        && (bx - px) * (cy - py) >= (cx - px) * (by - py)
}
/// ShapeUtils.triangulateShape( contour, [] ) through Earcut, for a simple contour
/// of at most 80 points (earcut's unhashed path).
pub(super) fn triangulate(contour: &mut Vec<[f64; 2]>) -> Result<Vec<[usize; 3]>> {
    // removeDupEndPts
    if contour.len() > 2 && contour[contour.len() - 1] == contour[0] {
        contour.pop();
    }
    if contour.len() > 80 {
        return Err(bad("Earcut: the hashed path is not ported"));
    }
    let n = contour.len();
    // signedArea over the flat data, then the clockwise-ordered linked list.
    let mut sum = 0.;
    let mut j = n - 1;
    for i in 0..n {
        sum += (contour[j][0] - contour[i][0]) * (contour[i][1] + contour[j][1]);
        j = i;
    }
    let mut ring = Ring { nodes: vec![] };
    let mut last = None;
    if sum > 0. {
        for (i, p) in contour.iter().enumerate() {
            last = Some(ring.insert(i, p[0], p[1], last));
        }
    } else {
        for i in (0..n).rev() {
            last = Some(ring.insert(i, contour[i][0], contour[i][1], last));
        }
    }
    let mut last = last.ok_or_else(|| bad("Earcut: empty"))?;
    if ring.equals(last, ring.nodes[last].next) {
        ring.remove(last);
        last = ring.nodes[last].next;
    }
    let mut out = vec![];
    if ring.nodes[last].next != ring.nodes[last].prev {
        ring.earcut(last, &mut out, 0)?;
    }
    Ok(out.chunks(3).map(|t| [t[0], t[1], t[2]]).collect())
}

// ---------------------------------------------------------------- Extrude

/// ShapeUtils.area.
fn area(p: &[[f64; 2]]) -> f64 {
    let n = p.len();
    let mut a = 0.;
    let mut q = n - 1;
    for i in 0..n {
        a += p[q][0] * p[i][1] - p[i][0] * p[q][1];
        q = i;
    }
    a * 0.5
}
/// The lid and side-wall ( start, count ) ranges.
pub(super) type Groups = [(usize, usize); 2];
pub(super) struct ExtrudeOptions<'a> {
    pub steps: usize,
    pub depth: f64,
    pub bevel: Option<(f64, f64, usize)>,
    pub path: Option<&'a CatmullRomCurve3>,
}
/// ExtrudeGeometry for one hole-free shape of straight segments: the vertex
/// layers, bevel movements, lid faces (group 0) and side walls (group 1), as
/// non-indexed Float32 positions with the group ranges.
pub(super) fn extrude(shape: &[[f64; 2]], o: &ExtrudeOptions) -> Result<(Vec<f32>, Groups)> {
    // Shape( pts ).extractPoints: the line segments' points, consecutive duplicates removed.
    let mut vertices: Vec<[f64; 2]> = vec![];
    for p in shape {
        if vertices.last() != Some(p) {
            vertices.push(*p);
        }
    }
    let (steps, depth) = (o.steps, o.depth);
    let (mut bevel_thickness, mut bevel_size, mut bevel_segments) = o.bevel.unwrap_or((0., 0., 0));
    let mut spline = None;
    if let Some(path) = o.path {
        let points = path.spaced_points(steps as u32)?;
        let (_, normals, binormals) = path.frenet_frames(steps as u32, path.closed)?;
        spline = Some((points, normals, binormals));
        bevel_thickness = 0.;
        bevel_size = 0.;
        bevel_segments = 0;
    }
    let bevel_enabled = o.bevel.is_some() && o.path.is_none();
    if area(&vertices) >= 0. {
        vertices.reverse();
    }
    // mergeOverlappingPoints
    {
        let threshold_sq = 1e-10 * 1e-10;
        let mut prev = vertices[0];
        let mut i = 1;
        while i <= vertices.len() {
            let ci = i % vertices.len();
            let cur = vertices[ci];
            let (dx, dy) = (cur[0] - prev[0], cur[1] - prev[1]);
            let s = cur[0]
                .abs()
                .max(cur[1].abs())
                .max(prev[0].abs())
                .max(prev[1].abs());
            if dx * dx + dy * dy <= threshold_sq * s * s {
                vertices.remove(ci);
                continue;
            }
            prev = cur;
            i += 1;
        }
    }
    let contour = vertices.clone();
    let vlen = vertices.len();
    let bevel_vec = |p: [f64; 2], prev: [f64; 2], next: [f64; 2]| -> [f64; 2] {
        let (vpx, vpy) = (p[0] - prev[0], p[1] - prev[1]);
        let (vnx, vny) = (next[0] - p[0], next[1] - p[1]);
        let vp_lensq = vpx * vpx + vpy * vpy;
        let collinear = vpx * vny - vpy * vnx;
        let (tx, ty, shrink);
        if collinear.abs() > f64::EPSILON {
            let vp_len = vp_lensq.sqrt();
            let vn_len = (vnx * vnx + vny * vny).sqrt();
            let (psx, psy) = (prev[0] - vpy / vp_len, prev[1] + vpx / vp_len);
            let (nsx, nsy) = (next[0] - vny / vn_len, next[1] + vnx / vn_len);
            let sf = ((nsx - psx) * vny - (nsy - psy) * vnx) / (vpx * vny - vpy * vnx);
            let (x, y) = (psx + vpx * sf - p[0], psy + vpy * sf - p[1]);
            let lensq = x * x + y * y;
            if lensq <= 2. {
                return [x, y];
            }
            tx = x;
            ty = y;
            shrink = (lensq / 2.).sqrt();
        } else {
            let mut same = false;
            if vpx > f64::EPSILON {
                same = vnx > f64::EPSILON;
            } else if vpx < -f64::EPSILON {
                same = vnx < -f64::EPSILON;
            } else if vpy.signum() == vny.signum() {
                same = true;
            }
            if same {
                tx = -vpy;
                ty = vpx;
                shrink = vp_lensq.sqrt();
            } else {
                tx = vpx;
                ty = vpy;
                shrink = (vp_lensq / 2.).sqrt();
            }
        }
        [tx / shrink, ty / shrink]
    };
    let moves: Vec<[f64; 2]> = (0..vlen)
        .map(|i| {
            bevel_vec(
                contour[i],
                contour[(i + vlen - 1) % vlen],
                contour[(i + 1) % vlen],
            )
        })
        .collect();
    let scale = |p: [f64; 2], m: [f64; 2], s: f64| [p[0] + m[0] * s, p[1] + m[1] * s];
    let mut placeholder: Vec<f64> = vec![];
    let mut contracted = vec![];
    for b in 0..bevel_segments {
        let t = b as f64 / bevel_segments as f64;
        let z = bevel_thickness * (t * std::f64::consts::PI / 2.).cos();
        let bs = bevel_size * (t * std::f64::consts::PI / 2.).sin();
        for i in 0..vlen {
            let v = scale(contour[i], moves[i], bs);
            placeholder.extend([v[0], v[1], -z]);
            if t == 0. {
                contracted.push(v);
            }
        }
    }
    let mut faces = if bevel_segments == 0 {
        triangulate(&mut contour.clone())?
    } else {
        triangulate(&mut contracted)?
    };
    let bs = bevel_size;
    for s in 0..=steps {
        for i in 0..vlen {
            let v = if bevel_enabled {
                scale(vertices[i], moves[i], bs)
            } else {
                vertices[i]
            };
            match &spline {
                None => placeholder.extend([
                    v[0],
                    v[1],
                    if s == 0 {
                        0.
                    } else {
                        depth / steps as f64 * s as f64
                    },
                ]),
                Some((points, normals, binormals)) => {
                    let p: Vector3 = points[s] + normals[s] * v[0] + binormals[s] * v[1];
                    placeholder.extend([p.x, p.y, p.z]);
                }
            }
        }
    }
    for b in (0..bevel_segments).rev() {
        let t = b as f64 / bevel_segments as f64;
        let z = bevel_thickness * (t * std::f64::consts::PI / 2.).cos();
        let bs = bevel_size * (t * std::f64::consts::PI / 2.).sin();
        for i in 0..vlen {
            let v = scale(contour[i], moves[i], bs);
            placeholder.extend([v[0], v[1], depth + z]);
        }
    }
    let mut out: Vec<f64> = vec![];
    let push = |out: &mut Vec<f64>, k: usize| out.extend_from_slice(&placeholder[k * 3..k * 3 + 3]);
    // buildLidFaces
    let layers = if bevel_enabled {
        steps + bevel_segments * 2
    } else {
        steps
    };
    for f in &faces {
        for k in [f[2], f[1], f[0]] {
            push(&mut out, k);
        }
    }
    for f in &mut faces {
        for k in [f[0], f[1], f[2]] {
            push(&mut out, k + vlen * layers);
        }
    }
    let lids = out.len() / 3;
    // buildSideFaces: sidewalls over the contour, from its last point down.
    for i in (0..vlen).rev() {
        let (j, k) = (i, if i == 0 { vlen - 1 } else { i - 1 });
        for s in 0..steps + bevel_segments * 2 {
            let (s1, s2) = (vlen * s, vlen * (s + 1));
            let (a, b, c, d) = (j + s1, k + s1, k + s2, j + s2);
            for v in [a, b, d, b, c, d] {
                push(&mut out, v);
            }
        }
    }
    let total = out.len() / 3;
    Ok((
        out.into_iter().map(|v| v as f32).collect(),
        [(0, lids), (lids, total - lids)],
    ))
}
