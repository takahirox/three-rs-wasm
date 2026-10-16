//! Two-dimensional shapes as the pinned sources build them: Path and ShapePath
//! curves, FontLoader typeface glyphs ( and TTFLoader's ), Earcut with holes,
//! ExtrudeGeometry and TextGeometry.
use crate::{Error, Result};
use std::collections::HashMap;

fn bad(what: &str) -> Error {
    Error::Asset(what.to_string())
}
type P2 = [f64; 2];

// ---------------------------------------------------------------- Curves

/// The 2D curves of three.js's extras ( LineCurve, QuadraticBezierCurve,
/// CubicBezierCurve, EllipseCurve / ArcCurve, SplineCurve ).
#[derive(Clone, Debug)]
pub enum Curve2 {
    Line(P2, P2),
    Quadratic(P2, P2, P2),
    Cubic(P2, P2, P2, P2),
    Ellipse {
        center: P2,
        radius: P2,
        start: f64,
        end: f64,
        clockwise: bool,
        rotation: f64,
    },
    Spline(Vec<P2>),
}
/// Interpolations.CatmullRom.
fn catmull_rom(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let v0 = (p2 - p0) * 0.5;
    let v1 = (p3 - p1) * 0.5;
    let t2 = t * t;
    let t3 = t * t2;
    (2. * p1 - 2. * p2 + v0 + v1) * t3 + (-3. * p1 + 3. * p2 - 2. * v0 - v1) * t2 + v0 * t + p1
}
fn distance(a: P2, b: P2) -> f64 {
    let (dx, dy) = (a[0] - b[0], a[1] - b[1]);
    (dx * dx + dy * dy).sqrt()
}
impl Curve2 {
    /// getPoint( t ).
    pub fn point(&self, t: f64) -> P2 {
        match self {
            // LineCurve.getPoint: t === 1 returns v2 itself.
            Curve2::Line(a, b) => {
                if t == 1. {
                    *b
                } else {
                    [(b[0] - a[0]) * t + a[0], (b[1] - a[1]) * t + a[1]]
                }
            }
            Curve2::Quadratic(a, c, b) => {
                let k = 1. - t;
                let f = |i: usize| k * k * a[i] + 2. * (1. - t) * t * c[i] + t * t * b[i];
                [f(0), f(1)]
            }
            Curve2::Cubic(a, c1, c2, b) => {
                let k = 1. - t;
                let f = |i: usize| {
                    k * k * k * a[i]
                        + 3. * k * k * t * c1[i]
                        + 3. * (1. - t) * t * t * c2[i]
                        + t * t * t * b[i]
                };
                [f(0), f(1)]
            }
            Curve2::Ellipse {
                center,
                radius,
                start,
                end,
                clockwise,
                rotation,
            } => {
                let two_pi = std::f64::consts::PI * 2.;
                let mut delta = end - start;
                let same = delta.abs() < f64::EPSILON;
                while delta < 0. {
                    delta += two_pi;
                }
                while delta > two_pi {
                    delta -= two_pi;
                }
                if delta < f64::EPSILON {
                    delta = if same { 0. } else { two_pi };
                }
                if *clockwise && !same {
                    delta = if delta == two_pi {
                        -two_pi
                    } else {
                        delta - two_pi
                    };
                }
                let angle = start + t * delta;
                let mut x = center[0] + radius[0] * crate::js_math::cos(angle);
                let mut y = center[1] + radius[1] * crate::js_math::sin(angle);
                if *rotation != 0. {
                    let (c, s) = (
                        crate::js_math::cos(*rotation),
                        crate::js_math::sin(*rotation),
                    );
                    let (tx, ty) = (x - center[0], y - center[1]);
                    x = tx * c - ty * s + center[0];
                    y = tx * s + ty * c + center[1];
                }
                [x, y]
            }
            Curve2::Spline(points) => {
                let n = points.len();
                let p = (n - 1) as f64 * t;
                let int = p.floor();
                let weight = p - int;
                let i = int as usize;
                let p0 = points[if i == 0 { i } else { i - 1 }];
                let p1 = points[i.min(n - 1)];
                let p2 = points[if i + 2 > n { n - 1 } else { i + 1 }];
                let p3 = points[if i + 3 > n { n - 1 } else { i + 2 }];
                [
                    catmull_rom(weight, p0[0], p1[0], p2[0], p3[0]),
                    catmull_rom(weight, p0[1], p1[1], p2[1], p3[1]),
                ]
            }
        }
    }
    /// Curve.getLengths( 200 ): the cumulative chord lengths.
    fn lengths(&self) -> Vec<f64> {
        let divisions = 200;
        let mut cache = vec![0.];
        let mut last = self.point(0.);
        let mut sum = 0.;
        for p in 1..=divisions {
            let current = self.point(p as f64 / divisions as f64);
            sum += distance(current, last);
            cache.push(sum);
            last = current;
        }
        cache
    }
    /// getLength().
    pub fn length(&self) -> f64 {
        *self.lengths().last().unwrap_or(&0.)
    }
    /// getPointAt( u ) with the curve's arc lengths ( LineCurve: getPoint ).
    fn point_at_with(&self, u: f64, lengths: &[f64]) -> P2 {
        if matches!(self, Curve2::Line(..)) {
            return self.point(u);
        }
        self.point(u_to_t(lengths, u))
    }
    /// getPoints( divisions ).
    fn sample(&self, divisions: usize) -> Vec<P2> {
        (0..=divisions)
            .map(|d| self.point(d as f64 / divisions as f64))
            .collect()
    }
}
/// Curve.getUtoTmapping( u ).
fn u_to_t(arc: &[f64], u: f64) -> f64 {
    let il = arc.len();
    let target = u * arc[il - 1];
    let (mut low, mut high): (i64, i64) = (0, il as i64 - 1);
    while low <= high {
        let i = (low as f64 + (high - low) as f64 / 2.).floor() as i64;
        let comparison = arc[i as usize] - target;
        if comparison < 0. {
            low = i + 1;
        } else if comparison > 0. {
            high = i - 1;
        } else {
            high = i;
            break;
        }
    }
    let i = high.max(0) as usize;
    if arc[i] == target {
        return i as f64 / (il - 1) as f64;
    }
    let before = arc[i];
    let after = arc.get(i + 1).copied().unwrap_or(f64::NAN);
    let fraction = (target - before) / (after - before);
    (i as f64 + fraction) / (il - 1) as f64
}
/// Path: curves appended from its current point.
#[derive(Clone, Default)]
pub struct Path {
    pub curves: Vec<Curve2>,
    current: P2,
    /// CurvePath.autoClose.
    pub auto_close: bool,
}
impl Path {
    /// moveTo( first ), lineTo( each next ), closePath().
    pub fn polygon(points: &[P2]) -> Self {
        let mut path = Self::from_points(points);
        path.line_to(points[0]);
        path
    }
    /// new Path( points ): setFromPoints, moveTo( first ) then lineTo( each next ).
    pub fn from_points(points: &[P2]) -> Self {
        let mut path = Self::default();
        path.move_to(points[0]);
        for p in &points[1..] {
            path.line_to(*p);
        }
        path
    }
    pub fn move_to(&mut self, p: P2) {
        self.current = p;
    }
    pub fn line_to(&mut self, p: P2) {
        self.curves.push(Curve2::Line(self.current, p));
        self.current = p;
    }
    pub fn quadratic_to(&mut self, c: P2, p: P2) {
        self.curves.push(Curve2::Quadratic(self.current, c, p));
        self.current = p;
    }
    /// bezierCurveTo().
    pub fn cubic_to(&mut self, c1: P2, c2: P2, p: P2) {
        self.curves.push(Curve2::Cubic(self.current, c1, c2, p));
        self.current = p;
    }
    /// splineThru( points ): a SplineCurve from the current point.
    pub fn spline_thru(&mut self, points: &[P2]) {
        let mut all = vec![self.current];
        all.extend_from_slice(points);
        self.curves.push(Curve2::Spline(all));
        self.current = *points.last().unwrap_or(&self.current);
    }
    /// absarc( x, y, radius, start, end, clockwise ).
    pub fn absarc(&mut self, center: P2, radius: f64, start: f64, end: f64, clockwise: bool) {
        self.absellipse(center, [radius, radius], start, end, clockwise, 0.);
    }
    /// absellipse(): joined to the current point by a line when apart.
    pub fn absellipse(
        &mut self,
        center: P2,
        radius: P2,
        start: f64,
        end: f64,
        clockwise: bool,
        rotation: f64,
    ) {
        let curve = Curve2::Ellipse {
            center,
            radius,
            start,
            end,
            clockwise,
            rotation,
        };
        if !self.curves.is_empty() {
            let first = curve.point(0.);
            if first != self.current {
                self.line_to(first);
            }
        }
        let last = curve.point(1.);
        self.curves.push(curve);
        self.current = last;
    }
    /// CurvePath.getPoints( divisions ): one step per line, twice `divisions`
    /// per ellipse, `divisions` per spline point, `divisions` per other curve,
    /// consecutive duplicates skipped; closed when autoClose.
    pub fn points(&self, divisions: usize) -> Vec<P2> {
        let mut out: Vec<P2> = vec![];
        for c in &self.curves {
            let resolution = match c {
                Curve2::Line(..) => 1,
                Curve2::Ellipse { .. } => divisions * 2,
                Curve2::Spline(p) => divisions * p.len(),
                _ => divisions,
            };
            for p in c.sample(resolution) {
                if out.last() == Some(&p) {
                    continue;
                }
                out.push(p);
            }
        }
        if self.auto_close && out.len() > 1 && out.last() != out.first() {
            out.push(out[0]);
        }
        out
    }
    /// The lengths three.js caches for this path: each curve's arc lengths,
    /// the cumulative curve lengths and the path's own arc lengths.
    pub fn sampler(&self) -> PathSampler<'_> {
        let arcs: Vec<Vec<f64>> = self.curves.iter().map(Curve2::lengths).collect();
        let mut sum = 0.;
        let curve_lengths = arcs
            .iter()
            .map(|a| {
                sum += a.last().copied().unwrap_or(0.);
                sum
            })
            .collect();
        let mut sampler = PathSampler {
            path: self,
            arcs,
            curve_lengths,
            lengths: vec![],
        };
        // Curve.getLengths( 200 ) over CurvePath.getPoint.
        let divisions = 200;
        let mut lengths = vec![0.];
        if let Some(mut last) = sampler.point(0.) {
            let mut sum = 0.;
            for p in 1..=divisions {
                let Some(current) = sampler.point(p as f64 / divisions as f64) else {
                    break;
                };
                sum += distance(current, last);
                lengths.push(sum);
                last = current;
            }
        }
        sampler.lengths = lengths;
        sampler
    }
    /// CurvePath.getPoint( t ).
    pub fn point(&self, t: f64) -> Option<P2> {
        self.sampler().point(t)
    }
    /// CurvePath.getPointAt( u ).
    pub fn point_at(&self, u: f64) -> Option<P2> {
        self.sampler().point_at(u)
    }
    /// getSpacedPoints( divisions ): equidistant points, closed when autoClose.
    pub fn spaced_points(&self, divisions: usize) -> Vec<P2> {
        let sampler = self.sampler();
        let mut out: Vec<P2> = (0..=divisions)
            .filter_map(|i| sampler.point(i as f64 / divisions as f64))
            .collect();
        if self.auto_close && !out.is_empty() {
            out.push(out[0]);
        }
        out
    }
}
#[derive(Clone, Default)]
pub struct Shape {
    pub outline: Path,
    pub holes: Vec<Path>,
}
/// ShapeUtils.area.
pub fn area(p: &[P2]) -> f64 {
    let n = p.len();
    let mut a = 0.;
    let mut q = n.wrapping_sub(1);
    for i in 0..n {
        a += p[q][0] * p[i][1] - p[i][0] * p[q][1];
        q = i;
    }
    a * 0.5
}
/// ShapePath.toShapes: subpaths nested by containment under the non-zero rule,
/// outers with their holes.
pub fn to_shapes(sub_paths: &[Path]) -> Vec<Shape> {
    fn inside(p: P2, polygon: &[P2]) -> bool {
        let mut result = false;
        let n = polygon.len();
        let mut j = n - 1;
        for i in 0..n {
            let (a, b) = (polygon[i], polygon[j]);
            if (a[1] > p[1]) != (b[1] > p[1])
                && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
            {
                result = !result;
            }
            j = i;
        }
        result
    }
    struct Entry {
        path: usize,
        points: Vec<P2>,
        bounds: [f64; 4],
        interior: P2,
        abs_area: f64,
        winding: i32,
        container: Option<usize>,
        exclude: bool,
        outer: Option<bool>,
    }
    let mut entries = vec![];
    for (k, sub) in sub_paths.iter().enumerate() {
        let points = sub.points(12);
        if points.len() < 3 {
            continue;
        }
        let a = area(&points);
        if a == 0. {
            continue;
        }
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for p in &points {
            b = [
                b[0].min(p[0]),
                b[1].min(p[1]),
                b[2].max(p[0]),
                b[3].max(p[1]),
            ];
        }
        // getInteriorPoint: the box center, else the midpoint of its row's first crossings.
        let mut interior = [(b[0] + b[2]) * 0.5, (b[1] + b[3]) * 0.5];
        if !inside(interior, &points) {
            let y = interior[1];
            let n = points.len();
            let mut x = vec![];
            for i in 0..n {
                let (a, c) = (points[i], points[(i + 1) % n]);
                if (a[1] > y) != (c[1] > y) {
                    x.push(a[0] + (y - a[1]) * (c[0] - a[0]) / (c[1] - a[1]));
                }
            }
            if x.len() > 1 {
                x.sort_by(|a, b| a.total_cmp(b));
                interior[0] = (x[0] + x[1]) / 2.;
            }
        }
        entries.push(Entry {
            path: k,
            points,
            bounds: b,
            interior,
            abs_area: a.abs(),
            winding: if a < 0. { -1 } else { 1 },
            container: None,
            exclude: false,
            outer: None,
        });
    }
    // Stable sort, largest area first.
    entries.sort_by(|a, b| b.abs_area.total_cmp(&a.abs_area));
    for i in 0..entries.len() {
        let mut container_winding = 0;
        for j in (0..i).rev() {
            let (c, e) = (&entries[j], &entries[i]);
            let contains = c.bounds[0] <= e.bounds[0]
                && e.bounds[2] <= c.bounds[2]
                && c.bounds[1] <= e.bounds[1]
                && e.bounds[3] <= c.bounds[3];
            if !contains || !inside(e.interior, &c.points) {
                continue;
            }
            let container = if c.exclude { c.container } else { Some(j) };
            container_winding = c.winding;
            entries[i].container = container;
            entries[i].winding += container_winding;
            break;
        }
        if (entries[i].winding != 0) == (container_winding != 0) {
            entries[i].exclude = true;
        }
    }
    for i in 0..entries.len() {
        if entries[i].exclude {
            continue;
        }
        let outer = match entries[i].container {
            None => true,
            Some(c) => entries[c].outer == Some(false),
        };
        entries[i].outer = Some(outer);
    }
    let mut shapes = vec![];
    let mut by_entry: HashMap<usize, usize> = HashMap::new();
    for (i, e) in entries.iter().enumerate() {
        if e.exclude || e.outer != Some(true) {
            continue;
        }
        by_entry.insert(i, shapes.len());
        shapes.push(Shape {
            outline: sub_paths[e.path].clone(),
            holes: vec![],
        });
    }
    for e in &entries {
        if e.exclude || e.outer != Some(false) {
            continue;
        }
        if let Some(&s) = e.container.and_then(|c| by_entry.get(&c)) {
            shapes[s].holes.push(sub_paths[e.path].clone());
        }
    }
    shapes
}

// ---------------------------------------------------------------- Fonts

#[derive(serde::Deserialize)]
struct Glyph {
    ha: f64,
    #[serde(default)]
    o: Option<String>,
}
#[derive(serde::Deserialize)]
struct BoundingBox {
    #[serde(rename = "yMin")]
    y_min: f64,
    #[serde(rename = "yMax")]
    y_max: f64,
}
#[derive(serde::Deserialize)]
pub struct Font {
    glyphs: HashMap<String, Glyph>,
    resolution: f64,
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingBox,
    #[serde(rename = "underlineThickness", default)]
    underline_thickness: f64,
    #[serde(default)]
    ascender: f64,
}
impl Font {
    pub fn parse(json: &[u8]) -> Result<Self> {
        serde_json::from_slice(json).map_err(|e| bad(&format!("typeface: {e}")))
    }
    /// TTFLoader.parse(): opentype.js 1.3.4's TrueType outlines converted to the
    /// typeface layout (resolution 1000, coordinates scaled and rounded).
    pub fn from_ttf(data: &[u8]) -> Result<Self> {
        let err = || bad("ttf: truncated");
        let u16_at = |o: usize| -> Result<u16> {
            Ok(u16::from_be_bytes(
                data.get(o..o + 2)
                    .ok_or_else(err)?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let i16_at = |o: usize| -> Result<i16> { Ok(u16_at(o)? as i16) };
        let u32_at = |o: usize| -> Result<u32> {
            Ok(u32::from_be_bytes(
                data.get(o..o + 4)
                    .ok_or_else(err)?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let mut tables = HashMap::new();
        for i in 0..usize::from(u16_at(4)?) {
            let r = 12 + 16 * i;
            let tag = data.get(r..r + 4).ok_or_else(err)?;
            tables.insert(tag.to_vec(), u32_at(r + 8)? as usize);
        }
        let table = |tag: &[u8]| {
            tables
                .get(tag)
                .copied()
                .ok_or_else(|| bad("ttf: missing table"))
        };
        let head = table(b"head")?;
        let units_per_em = f64::from(u16_at(head + 18)?);
        let y_min = f64::from(i16_at(head + 38)?);
        let y_max = f64::from(i16_at(head + 42)?);
        let long_loca = i16_at(head + 50)? != 0;
        let glyph_count = usize::from(u16_at(table(b"maxp")? + 4)?);
        let hhea = table(b"hhea")?;
        let metrics = usize::from(u16_at(hhea + 34)?);
        let hmtx = table(b"hmtx")?;
        let advance = |g: usize| u16_at(hmtx + 4 * g.min(metrics.saturating_sub(1)));
        let loca = table(b"loca")?;
        let glyf = table(b"glyf")?;
        let location = |g: usize| -> Result<usize> {
            Ok(if long_loca {
                u32_at(loca + 4 * g)? as usize
            } else {
                usize::from(u16_at(loca + 2 * g)?) * 2
            })
        };
        let underline_thickness = f64::from(i16_at(table(b"post")? + 10)?);
        // parseCmapTable: the last platform 3 (0, 1, 10) or platform 0 subtable.
        let cmap = table(b"cmap")?;
        let mut sub = None;
        for i in (0..usize::from(u16_at(cmap + 2)?)).rev() {
            let (platform, encoding) = (u16_at(cmap + 4 + 8 * i)?, u16_at(cmap + 6 + 8 * i)?);
            if (platform == 3 && matches!(encoding, 0 | 1 | 10)) || (platform == 0 && encoding <= 4)
            {
                sub = Some(cmap + u32_at(cmap + 8 + 8 * i)? as usize);
                break;
            }
        }
        let sub = sub.ok_or_else(|| bad("ttf: no cmap"))?;
        let mut map = std::collections::BTreeMap::new();
        match u16_at(sub)? {
            4 => {
                let segments = usize::from(u16_at(sub + 6)? >> 1);
                for i in 0..segments.saturating_sub(1) {
                    let end = u16_at(sub + 14 + 2 * i)?;
                    let start = u16_at(sub + 16 + 2 * segments + 2 * i)?;
                    let delta = u16_at(sub + 16 + 4 * segments + 2 * i)?;
                    let range_at = sub + 16 + 6 * segments + 2 * i;
                    let range = usize::from(u16_at(range_at)?);
                    for c in start..=end {
                        let g = if range != 0 {
                            let g = u16_at(range_at + range + 2 * usize::from(c - start))?;
                            if g != 0 { g.wrapping_add(delta) } else { 0 }
                        } else {
                            c.wrapping_add(delta)
                        };
                        map.insert(u32::from(c), usize::from(g));
                    }
                }
            }
            12 => {
                for i in 0..u32_at(sub + 12)? as usize {
                    let r = sub + 16 + 12 * i;
                    let (first, last, glyph) = (u32_at(r)?, u32_at(r + 4)?, u32_at(r + 8)?);
                    for c in first..=last {
                        map.insert(c, (glyph + c - first) as usize);
                    }
                }
            }
            _ => return Err(bad("ttf: unsupported cmap format")),
        }
        let scale = 100000. / (units_per_em * 72.);
        let round = |v: f64| ((v * scale + 0.5).floor()) as i64;
        let mut outlines: HashMap<usize, String> = HashMap::new();
        let mut glyphs = HashMap::new();
        for (&c, &g) in &map {
            if g >= glyph_count {
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(slot) = outlines.entry(g) {
                let mut o = String::new();
                for contour in ttf_contours(data, glyf, &location, g, 0)? {
                    // getPath(): implied on-curve midpoints between off-curve points.
                    let n = contour.len();
                    let mid = |a: (f64, f64, bool), b: (f64, f64, bool)| {
                        ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5, true)
                    };
                    let (last, first) = (contour[n - 1], contour[0]);
                    let start = if last.2 {
                        last
                    } else if first.2 {
                        first
                    } else {
                        mid(last, first)
                    };
                    o += &format!("m {} {} ", round(start.0), round(start.1));
                    for i in 0..n {
                        let curr = contour[i];
                        let next = contour[(i + 1) % n];
                        if curr.2 {
                            o += &format!("l {} {} ", round(curr.0), round(curr.1));
                        } else {
                            let end = if next.2 { next } else { mid(curr, next) };
                            o += &format!(
                                "q {} {} {} {} ",
                                round(end.0),
                                round(end.1),
                                round(curr.0),
                                round(curr.1)
                            );
                        }
                    }
                    o += "z ";
                }
                slot.insert(o);
            }
            let Some(ch) = char::from_u32(c) else {
                continue;
            };
            glyphs.insert(
                ch.to_string(),
                Glyph {
                    ha: round(f64::from(advance(g)?)) as f64,
                    o: outlines.get(&g).cloned(),
                },
            );
        }
        Ok(Self {
            glyphs,
            resolution: 1000.,
            bounding_box: BoundingBox { y_min, y_max },
            underline_thickness,
            ascender: 0.,
        })
    }
    /// Font.generateShapes( text, size ): each glyph's ShapePath, left to right.
    pub fn shapes(&self, text: &str, size: f64) -> Vec<Shape> {
        self.shapes_in(text, size, "ltr")
    }
    /// Font.generateShapes( text, size, direction ): 'rtl' and 'tb' reverse the
    /// characters; 'tb' stacks them down by the ascender.
    pub fn shapes_in(&self, text: &str, size: f64, direction: &str) -> Vec<Shape> {
        let scale = size / self.resolution;
        let line_height =
            (self.bounding_box.y_max - self.bounding_box.y_min + self.underline_thickness) * scale;
        let (mut offset_x, mut offset_y) = (0., 0.);
        let mut shapes = vec![];
        let mut chars: Vec<char> = text.chars().collect();
        if direction == "rtl" || direction == "tb" {
            chars.reverse();
        }
        for ch in chars {
            if ch == '\n' {
                offset_x = 0.;
                offset_y -= line_height;
                continue;
            }
            let Some(glyph) = self
                .glyphs
                .get(&ch.to_string())
                .or_else(|| self.glyphs.get("?"))
            else {
                continue;
            };
            let mut paths: Vec<Path> = vec![];
            if let Some(o) = &glyph.o {
                let t: Vec<&str> = o.split(' ').collect();
                let n = |s: &str| crate::fbx::js::parse_float(s);
                let mut i = 0;
                let at = |i: usize, x: bool| {
                    n(t.get(i).copied().unwrap_or("")) * scale + if x { offset_x } else { offset_y }
                };
                while i < t.len() {
                    let action = t[i];
                    i += 1;
                    match action {
                        "m" => {
                            let mut p = Path::default();
                            p.move_to([at(i, true), at(i + 1, false)]);
                            paths.push(p);
                            i += 2;
                        }
                        "l" => {
                            if let Some(p) = paths.last_mut() {
                                p.line_to([at(i, true), at(i + 1, false)]);
                            }
                            i += 2;
                        }
                        "q" => {
                            let end = [at(i, true), at(i + 1, false)];
                            let c = [at(i + 2, true), at(i + 3, false)];
                            if let Some(p) = paths.last_mut() {
                                p.quadratic_to(c, end);
                            }
                            i += 4;
                        }
                        "b" => {
                            let end = [at(i, true), at(i + 1, false)];
                            let c1 = [at(i + 2, true), at(i + 3, false)];
                            let c2 = [at(i + 4, true), at(i + 5, false)];
                            if let Some(p) = paths.last_mut() {
                                p.cubic_to(c1, c2, end);
                            }
                            i += 6;
                        }
                        _ => {}
                    }
                }
            }
            if direction == "tb" {
                offset_x = 0.;
                offset_y += self.ascender * scale;
            } else {
                offset_x += glyph.ha * scale;
            }
            shapes.extend(to_shapes(&paths));
        }
        shapes
    }
}

/// A TrueType glyph's contours as (x, y, on-curve) points; composite glyphs
/// place their components by offset and 2×2 transform, as transformPoints.
type Contour = Vec<(f64, f64, bool)>;
fn ttf_contours(
    data: &[u8],
    glyf: usize,
    location: &dyn Fn(usize) -> Result<usize>,
    g: usize,
    depth: u32,
) -> Result<Vec<Contour>> {
    let err = || bad("ttf: truncated glyph");
    let byte = |o: usize| data.get(o).copied().ok_or_else(err);
    let u16_at =
        |o: usize| -> Result<u16> { Ok(u16::from(byte(o)?) << 8 | u16::from(byte(o + 1)?)) };
    let i16_at = |o: usize| -> Result<i16> { Ok(u16_at(o)? as i16) };
    let (start, end) = (location(g)?, location(g + 1)?);
    if end <= start || depth > 8 {
        return Ok(vec![]);
    }
    let at = glyf + start;
    let contours = i16_at(at)?;
    let mut p = at + 10;
    if contours >= 0 {
        let contours = contours as usize;
        let ends: Vec<usize> = (0..contours)
            .map(|i| u16_at(p + 2 * i).map(usize::from))
            .collect::<Result<_>>()?;
        p += 2 * contours;
        let count = ends.last().map_or(0, |e| e + 1);
        p += 2 + usize::from(u16_at(p)?);
        let mut flags = Vec::with_capacity(count);
        while flags.len() < count {
            let f = byte(p)?;
            p += 1;
            flags.push(f);
            if f & 8 != 0 {
                let repeat = byte(p)?;
                p += 1;
                for _ in 0..repeat {
                    flags.push(f);
                }
            }
        }
        let mut coordinate = |short: u8, same: u8| -> Result<Vec<f64>> {
            let mut v = 0i32;
            let mut out = Vec::with_capacity(count);
            for &f in &flags[..count] {
                if f & short != 0 {
                    let d = i32::from(byte(p)?);
                    p += 1;
                    v += if f & same != 0 { d } else { -d };
                } else if f & same == 0 {
                    v += i32::from(i16_at(p)?);
                    p += 2;
                }
                out.push(f64::from(v));
            }
            Ok(out)
        };
        let xs = coordinate(2, 16)?;
        let ys = coordinate(4, 32)?;
        let mut out = vec![];
        let mut first = 0;
        for e in ends {
            out.push(
                (first..=e)
                    .map(|i| (xs[i], ys[i], flags[i] & 1 != 0))
                    .collect(),
            );
            first = e + 1;
        }
        return Ok(out);
    }
    let mut out = vec![];
    loop {
        let flags = u16_at(p)?;
        let component = usize::from(u16_at(p + 2)?);
        p += 4;
        let (dx, dy) = if flags & 1 != 0 {
            let v = (f64::from(i16_at(p)?), f64::from(i16_at(p + 2)?));
            p += 4;
            v
        } else {
            let v = (f64::from(byte(p)? as i8), f64::from(byte(p + 1)? as i8));
            p += 2;
            v
        };
        let f2dot14 = |o: usize| -> Result<f64> { Ok(f64::from(i16_at(o)?) / 16384.) };
        let (mut a, mut b, mut c, mut d) = (1., 0., 0., 1.);
        if flags & 8 != 0 {
            a = f2dot14(p)?;
            d = a;
            p += 2;
        } else if flags & 64 != 0 {
            a = f2dot14(p)?;
            d = f2dot14(p + 2)?;
            p += 4;
        } else if flags & 128 != 0 {
            a = f2dot14(p)?;
            b = f2dot14(p + 2)?;
            c = f2dot14(p + 4)?;
            d = f2dot14(p + 6)?;
            p += 8;
        }
        for contour in ttf_contours(data, glyf, location, component, depth + 1)? {
            out.push(
                contour
                    .into_iter()
                    .map(|(x, y, on)| (a * x + b * y + dx, c * x + d * y + dy, on))
                    .collect(),
            );
        }
        if flags & 32 == 0 {
            break;
        }
    }
    Ok(out)
}

pub use crate::shape_utils::triangulate;

// ---------------------------------------------------------------- Extrude

pub struct Extrude {
    pub curve_segments: usize,
    pub steps: usize,
    pub depth: f64,
    /// ( thickness, size, segments ) when bevelEnabled.
    pub bevel: Option<(f64, f64, usize)>,
}
fn merge_overlapping(points: &mut Vec<P2>) {
    if points.is_empty() {
        return;
    }
    let threshold_sq = 1e-10 * 1e-10;
    let mut prev = points[0];
    let mut i = 1;
    while i <= points.len() {
        let ci = i % points.len();
        let cur = points[ci];
        let (dx, dy) = (cur[0] - prev[0], cur[1] - prev[1]);
        let s = cur[0]
            .abs()
            .max(cur[1].abs())
            .max(prev[0].abs())
            .max(prev[1].abs());
        if dx * dx + dy * dy <= threshold_sq * s * s {
            points.remove(ci);
            if points.is_empty() {
                return;
            }
            continue;
        }
        prev = cur;
        i += 1;
    }
}
fn bevel_vec(p: P2, prev: P2, next: P2) -> P2 {
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
        let same = if vpx > f64::EPSILON {
            vnx > f64::EPSILON
        } else if vpx < -f64::EPSILON {
            vnx < -f64::EPSILON
        } else {
            vpy.signum() == vny.signum()
        };
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
}
fn movements(ring: &[P2]) -> Vec<P2> {
    let n = ring.len();
    (0..n)
        .map(|i| bevel_vec(ring[i], ring[(i + n - 1) % n], ring[(i + 1) % n]))
        .collect()
}
/// ExtrudeGeometry( shapes, options ) without an extrude path: non-indexed Float32
/// positions and, per shape, the lid (material 0) and side (material 1) groups.
pub fn extrude(shapes: &[Shape], o: &Extrude) -> (Vec<f32>, Vec<(usize, usize, usize)>) {
    let (positions, _, groups) = extrude_uv(shapes, o);
    (positions, groups)
}

/// WorldUVGenerator.generateTopUV(): a lid triangle's x and y.
fn top_uv(v: &[f64], start: usize, uv: &mut Vec<f64>) {
    for k in 0..3 {
        let i = (start + k) * 3;
        uv.extend([v[i], v[i + 1]]);
    }
}
/// WorldUVGenerator.generateSideWallUV() for the quad f4 pushed as a, b, d,
/// b, c, d: x or y ( whichever runs along the wall ) and 1 − z.
fn side_uv(v: &[f64], start: usize, uv: &mut Vec<f64>) {
    let p = |k: usize| {
        [
            v[(start + k) * 3],
            v[(start + k) * 3 + 1],
            v[(start + k) * 3 + 2],
        ]
    };
    let (a, b, c, d) = (p(0), p(3), p(4), p(5));
    let along_x = (a[1] - b[1]).abs() < (a[0] - b[0]).abs();
    let w = |q: [f64; 3]| {
        if along_x {
            [q[0], 1. - q[2]]
        } else {
            [q[1], 1. - q[2]]
        }
    };
    let (ua, ub, uc, ud) = (w(a), w(b), w(c), w(d));
    for q in [ua, ub, ud, ub, uc, ud] {
        uv.extend(q);
    }
}

/// ( start, count, materialIndex ) of the geometry groups.
pub type Groups = Vec<(usize, usize, usize)>;

/// ExtrudeGeometry's positions, WorldUVGenerator uvs and groups.
pub fn extrude_uv(shapes: &[Shape], o: &Extrude) -> (Vec<f32>, Vec<f32>, Groups) {
    let mut uv: Vec<f64> = vec![];
    let mut out: Vec<f64> = vec![];
    let mut groups = vec![];
    let (bevel_thickness, bevel_size, bevel_segments) = o.bevel.unwrap_or((0., 0., 0));
    let bevel = o.bevel.is_some();
    for shape in shapes {
        let mut vertices = shape.outline.points(o.curve_segments);
        let mut holes: Vec<Vec<P2>> = shape
            .holes
            .iter()
            .map(|h| h.points(o.curve_segments))
            .collect();
        if area(&vertices) >= 0. {
            vertices.reverse();
            for h in &mut holes {
                if area(h) < 0. {
                    h.reverse();
                }
            }
        }
        merge_overlapping(&mut vertices);
        for h in &mut holes {
            merge_overlapping(h);
        }
        let contour = vertices.clone();
        let mut all = contour.clone();
        for h in &holes {
            all.extend(h);
        }
        let vlen = all.len();
        let contour_moves = movements(&contour);
        let hole_moves: Vec<Vec<P2>> = holes.iter().map(|h| movements(h)).collect();
        let mut all_moves = contour_moves.clone();
        for m in &hole_moves {
            all_moves.extend(m);
        }
        let scale = |p: P2, m: P2, s: f64| [p[0] + m[0] * s, p[1] + m[1] * s];
        let mut placeholder: Vec<f64> = vec![];
        let (mut contracted, mut expanded) = (vec![], vec![]);
        for b in 0..bevel_segments {
            let t = b as f64 / bevel_segments as f64;
            let z = bevel_thickness * (t * std::f64::consts::PI / 2.).cos();
            let bs = bevel_size * (t * std::f64::consts::PI / 2.).sin();
            for (p, m) in contour.iter().zip(&contour_moves) {
                let v = scale(*p, *m, bs);
                placeholder.extend([v[0], v[1], -z]);
                if t == 0. {
                    contracted.push(v);
                }
            }
            for (h, hm) in holes.iter().zip(&hole_moves) {
                let mut ring = vec![];
                for (p, m) in h.iter().zip(hm) {
                    let v = scale(*p, *m, bs);
                    placeholder.extend([v[0], v[1], -z]);
                    ring.push(v);
                }
                if t == 0. {
                    expanded.push(ring);
                }
            }
        }
        let faces = if bevel_segments == 0 {
            triangulate(&mut contour.clone(), &mut holes.clone())
        } else {
            triangulate(&mut contracted, &mut expanded)
        };
        for s in 0..=o.steps {
            for i in 0..vlen {
                let v = if bevel {
                    scale(all[i], all_moves[i], bevel_size)
                } else {
                    all[i]
                };
                placeholder.extend([v[0], v[1], o.depth / o.steps as f64 * s as f64]);
            }
        }
        for b in (0..bevel_segments).rev() {
            let t = b as f64 / bevel_segments as f64;
            let z = bevel_thickness * (t * std::f64::consts::PI / 2.).cos();
            let bs = bevel_size * (t * std::f64::consts::PI / 2.).sin();
            for (p, m) in contour.iter().zip(&contour_moves) {
                let v = scale(*p, *m, bs);
                placeholder.extend([v[0], v[1], o.depth + z]);
            }
            for (h, hm) in holes.iter().zip(&hole_moves) {
                for (p, m) in h.iter().zip(hm) {
                    let v = scale(*p, *m, bs);
                    placeholder.extend([v[0], v[1], o.depth + z]);
                }
            }
        }
        let push =
            |out: &mut Vec<f64>, k: usize| out.extend_from_slice(&placeholder[k * 3..k * 3 + 3]);
        let start = out.len() / 3;
        let layers = o.steps + bevel_segments * 2;
        for f in &faces {
            let at = out.len() / 3;
            for k in [f[2], f[1], f[0]] {
                push(&mut out, k);
            }
            top_uv(&out, at, &mut uv);
        }
        for f in &faces {
            let at = out.len() / 3;
            for k in [f[0], f[1], f[2]] {
                push(&mut out, k + vlen * layers);
            }
            top_uv(&out, at, &mut uv);
        }
        groups.push((start, out.len() / 3 - start, 0));
        let start = out.len() / 3;
        let mut offset = 0;
        let mut rings = vec![contour.len()];
        rings.extend(holes.iter().map(Vec::len));
        for len in rings {
            for i in (0..len).rev() {
                let (j, k) = (i, if i == 0 { len - 1 } else { i - 1 });
                for s in 0..layers {
                    let (s1, s2) = (vlen * s, vlen * (s + 1));
                    let (a, b, c, d) = (
                        offset + j + s1,
                        offset + k + s1,
                        offset + k + s2,
                        offset + j + s2,
                    );
                    let at = out.len() / 3;
                    for v in [a, b, d, b, c, d] {
                        push(&mut out, v);
                    }
                    side_uv(&out, at, &mut uv);
                }
            }
            offset += len;
        }
        groups.push((start, out.len() / 3 - start, 1));
    }
    (
        out.into_iter().map(|v| v as f32).collect(),
        uv.into_iter().map(|v| v as f32).collect(),
        groups,
    )
}

/// TextGeometry( text, { font, size, depth, curveSegments, bevel… } ):
/// ExtrudeGeometry of the font's shapes with position, uv and the normals
/// computeVertexNormals() gives.
pub fn text_geometry(
    font: &Font,
    text: &str,
    size: f64,
    o: &Extrude,
) -> Result<crate::geometry::BufferGeometry> {
    use crate::attribute::BufferAttribute;
    use crate::geometry::{Attribute, BufferGeometry};
    let (positions, uvs, groups) = extrude_uv(&font.shapes(text, size), o);
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(positions, 3, false)?),
    );
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
    for (start, count, material) in groups {
        g.add_group(start, count, material);
    }
    g.compute_vertex_normals()?;
    Ok(g)
}

/// ShapeUtils.isClockWise: a negative area.
pub fn is_clockwise(points: &[P2]) -> bool {
    area(points) < 0.
}

impl Shape {
    /// extractPoints( divisions ): the outline's and holes' points.
    pub fn extract_points(&self, divisions: usize) -> (Vec<P2>, Vec<Vec<P2>>) {
        (
            self.outline.points(divisions),
            self.holes.iter().map(|h| h.points(divisions)).collect(),
        )
    }
}

/// ShapeGeometry( shapes, curveSegments ): each shape's outline ( made
/// clockwise ) and holes ( made counter-clockwise ) triangulated at z = 0
/// with +z normals and world uvs, indexed; with `groups`, one group per
/// shape ( as an array of shapes gives ).
pub fn shape_geometry(
    shapes: &[Shape],
    curve_segments: usize,
    groups: bool,
) -> Result<crate::geometry::BufferGeometry> {
    use crate::attribute::BufferAttribute;
    use crate::geometry::{Attribute, BufferGeometry};
    let (mut vertices, mut normals, mut uvs, mut indices) = (vec![], vec![], vec![], vec![]);
    let mut g = BufferGeometry::default();
    let mut group_start = 0;
    for (i, shape) in shapes.iter().enumerate() {
        let offset = (vertices.len() / 3) as u32;
        let (mut outline, mut holes) = shape.extract_points(curve_segments);
        if !is_clockwise(&outline) {
            outline.reverse();
        }
        for h in &mut holes {
            if is_clockwise(h) {
                h.reverse();
            }
        }
        let faces = triangulate(&mut outline, &mut holes);
        for h in &holes {
            outline.extend_from_slice(h);
        }
        for v in &outline {
            vertices.extend([v[0] as f32, v[1] as f32, 0.]);
            normals.extend([0f32, 0., 1.]);
            uvs.extend([v[0] as f32, v[1] as f32]);
        }
        for f in &faces {
            indices.extend([
                f[0] as u32 + offset,
                f[1] as u32 + offset,
                f[2] as u32 + offset,
            ]);
        }
        if groups {
            g.add_group(group_start, faces.len() * 3, i);
            group_start += faces.len() * 3;
        }
    }
    g.set_index(Some(indices));
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(vertices, 3, false)?),
    );
    g.set_attribute(
        "normal",
        Attribute::F32(BufferAttribute::new(normals, 3, false)?),
    );
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
    Ok(g)
}

/// A path with its cached lengths ( getCurveLengths, getLengths ).
pub struct PathSampler<'a> {
    path: &'a Path,
    arcs: Vec<Vec<f64>>,
    curve_lengths: Vec<f64>,
    lengths: Vec<f64>,
}
impl PathSampler<'_> {
    /// CurvePath.getPoint( t ): by the curves' lengths.
    pub fn point(&self, t: f64) -> Option<P2> {
        let d = t * self.curve_lengths.last().copied().unwrap_or(0.);
        for (i, &l) in self.curve_lengths.iter().enumerate() {
            if l >= d {
                let diff = l - d;
                let segment = self.arcs[i].last().copied().unwrap_or(0.);
                let u = if segment == 0. {
                    0.
                } else {
                    1. - diff / segment
                };
                return Some(self.path.curves[i].point_at_with(u, &self.arcs[i]));
            }
        }
        None
    }
    /// CurvePath.getPointAt( u ).
    pub fn point_at(&self, u: f64) -> Option<P2> {
        self.point(u_to_t(&self.lengths, u))
    }
}
