//! Two-dimensional shapes as the pinned sources build them: Path and ShapePath
//! curves, FontLoader typeface glyphs, Earcut with holes and ExtrudeGeometry.
use crate::{Error, Result};
use std::collections::HashMap;

fn bad(what: &str) -> Error {
    Error::Asset(what.to_string())
}
type P2 = [f64; 2];

// ---------------------------------------------------------------- Curves

#[derive(Clone)]
pub(super) enum Curve2 {
    Line(P2, P2),
    Quadratic(P2, P2, P2),
    Cubic(P2, P2, P2, P2),
}
impl Curve2 {
    fn point(&self, t: f64) -> P2 {
        match *self {
            // LineCurve.getPoint: t === 1 returns v2 itself.
            Curve2::Line(a, b) => {
                if t == 1. {
                    b
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
        }
    }
}
/// Path: curves appended from its current point.
#[derive(Clone, Default)]
pub(super) struct Path {
    pub curves: Vec<Curve2>,
    current: P2,
}
impl Path {
    fn move_to(&mut self, p: P2) {
        self.current = p;
    }
    fn line_to(&mut self, p: P2) {
        self.curves.push(Curve2::Line(self.current, p));
        self.current = p;
    }
    fn quadratic_to(&mut self, c: P2, p: P2) {
        self.curves.push(Curve2::Quadratic(self.current, c, p));
        self.current = p;
    }
    fn cubic_to(&mut self, c1: P2, c2: P2, p: P2) {
        self.curves.push(Curve2::Cubic(self.current, c1, c2, p));
        self.current = p;
    }
    /// CurvePath.getPoints( divisions ): one step per line, `divisions` per curve,
    /// consecutive duplicates skipped.
    pub fn points(&self, divisions: usize) -> Vec<P2> {
        let mut out: Vec<P2> = vec![];
        for c in &self.curves {
            let resolution = if matches!(c, Curve2::Line(..)) {
                1
            } else {
                divisions
            };
            for d in 0..=resolution {
                let p = c.point(d as f64 / resolution as f64);
                if out.last() == Some(&p) {
                    continue;
                }
                out.push(p);
            }
        }
        out
    }
}
#[derive(Clone, Default)]
pub(super) struct Shape {
    pub outline: Path,
    pub holes: Vec<Path>,
}
/// ShapeUtils.area.
pub(super) fn area(p: &[P2]) -> f64 {
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
pub(super) fn to_shapes(sub_paths: &[Path]) -> Vec<Shape> {
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
pub(super) struct Font {
    glyphs: HashMap<String, Glyph>,
    resolution: f64,
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingBox,
    #[serde(rename = "underlineThickness", default)]
    underline_thickness: f64,
}
impl Font {
    pub fn parse(json: &[u8]) -> Result<Self> {
        serde_json::from_slice(json).map_err(|e| bad(&format!("typeface: {e}")))
    }
    /// Font.generateShapes( text, size ): each glyph's ShapePath, left to right.
    pub fn shapes(&self, text: &str, size: f64) -> Vec<Shape> {
        let scale = size / self.resolution;
        let line_height =
            (self.bounding_box.y_max - self.bounding_box.y_min + self.underline_thickness) * scale;
        let (mut offset_x, mut offset_y) = (0., 0.);
        let mut shapes = vec![];
        for ch in text.chars() {
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
                let n = |s: &str| super::helpers_formats::formats::parse_float(s);
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
            offset_x += glyph.ha * scale;
            shapes.extend(to_shapes(&paths));
        }
        shapes
    }
}

// ---------------------------------------------------------------- Earcut

#[derive(Clone)]
struct Node {
    i: usize,
    x: f64,
    y: f64,
    prev: usize,
    next: usize,
    z: i32,
    prev_z: Option<usize>,
    next_z: Option<usize>,
    steiner: bool,
}
struct Earcut {
    n: Vec<Node>,
    out: Vec<usize>,
    min: P2,
    inv_size: f64,
}
impl Earcut {
    fn x(&self, p: usize) -> f64 {
        self.n[p].x
    }
    fn y(&self, p: usize) -> f64 {
        self.n[p].y
    }
    fn next(&self, p: usize) -> usize {
        self.n[p].next
    }
    fn prev(&self, p: usize) -> usize {
        self.n[p].prev
    }
    fn create(&mut self, i: usize, x: f64, y: f64) -> usize {
        self.n.push(Node {
            i,
            x,
            y,
            prev: 0,
            next: 0,
            z: 0,
            prev_z: None,
            next_z: None,
            steiner: false,
        });
        self.n.len() - 1
    }
    fn insert(&mut self, i: usize, x: f64, y: f64, last: Option<usize>) -> usize {
        let p = self.create(i, x, y);
        match last {
            None => {
                self.n[p].prev = p;
                self.n[p].next = p;
            }
            Some(last) => {
                let ln = self.next(last);
                self.n[p].next = ln;
                self.n[p].prev = last;
                self.n[ln].prev = p;
                self.n[last].next = p;
            }
        }
        p
    }
    fn remove(&mut self, p: usize) {
        let (prev, next) = (self.prev(p), self.next(p));
        self.n[next].prev = prev;
        self.n[prev].next = next;
        if let Some(pz) = self.n[p].prev_z {
            self.n[pz].next_z = self.n[p].next_z;
        }
        if let Some(nz) = self.n[p].next_z {
            self.n[nz].prev_z = self.n[p].prev_z;
        }
    }
    fn area(&self, p: usize, q: usize, r: usize) -> f64 {
        (self.y(q) - self.y(p)) * (self.x(r) - self.x(q))
            - (self.x(q) - self.x(p)) * (self.y(r) - self.y(q))
    }
    fn equals(&self, a: usize, b: usize) -> bool {
        self.x(a) == self.x(b) && self.y(a) == self.y(b)
    }
    fn linked_list(
        &mut self,
        data: &[f64],
        start: usize,
        end: usize,
        clockwise: bool,
    ) -> Option<usize> {
        let mut last = None;
        if clockwise == (signed_area(data, start, end) > 0.) {
            let mut i = start;
            while i < end {
                last = Some(self.insert(i / 2, data[i], data[i + 1], last));
                i += 2;
            }
        } else {
            let mut i = end as i64 - 2;
            while i >= start as i64 {
                let k = i as usize;
                last = Some(self.insert(k / 2, data[k], data[k + 1], last));
                i -= 2;
            }
        }
        if let Some(l) = last
            && self.equals(l, self.next(l))
        {
            self.remove(l);
            last = Some(self.next(l));
        }
        last
    }
    fn filter(&mut self, start: usize, end: Option<usize>) -> usize {
        let mut end = end.unwrap_or(start);
        let mut p = start;
        loop {
            let mut again = false;
            let (prev, next) = (self.prev(p), self.next(p));
            if !self.n[p].steiner && (self.equals(p, next) || self.area(prev, p, next) == 0.) {
                self.remove(p);
                p = prev;
                end = prev;
                if p == self.next(p) {
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
    fn earcut_linked(&mut self, ear: usize, pass: u8) {
        let mut ear = ear;
        if pass == 0 && self.inv_size != 0. {
            self.index_curve(ear);
        }
        let mut stop = ear;
        while self.prev(ear) != self.next(ear) {
            let (prev, next) = (self.prev(ear), self.next(ear));
            let is_ear = if self.inv_size != 0. {
                self.is_ear_hashed(ear)
            } else {
                self.is_ear(ear)
            };
            if is_ear {
                self.out
                    .extend([self.n[prev].i, self.n[ear].i, self.n[next].i]);
                self.remove(ear);
                ear = self.next(next);
                stop = ear;
                continue;
            }
            ear = next;
            if ear == stop {
                if pass == 0 {
                    let e = self.filter(ear, None);
                    self.earcut_linked(e, 1);
                } else if pass == 1 {
                    let e = self.filter(ear, None);
                    let e = self.cure_local_intersections(e);
                    self.earcut_linked(e, 2);
                } else {
                    self.split_earcut(ear);
                }
                break;
            }
        }
    }
    fn triangle_box(&self, ear: usize) -> (usize, usize, [f64; 6], [f64; 4]) {
        let (a, c) = (self.prev(ear), self.next(ear));
        let v = [
            self.x(a),
            self.y(a),
            self.x(ear),
            self.y(ear),
            self.x(c),
            self.y(c),
        ];
        let b = [
            v[0].min(v[2]).min(v[4]),
            v[1].min(v[3]).min(v[5]),
            v[0].max(v[2]).max(v[4]),
            v[1].max(v[3]).max(v[5]),
        ];
        (a, c, v, b)
    }
    fn blocks(&self, p: usize, v: &[f64; 6], b: &[f64; 4]) -> bool {
        let (px, py) = (self.x(p), self.y(p));
        px >= b[0]
            && px <= b[2]
            && py >= b[1]
            && py <= b[3]
            && !(v[0] == px && v[1] == py)
            && in_triangle(v[0], v[1], v[2], v[3], v[4], v[5], px, py)
            && self.area(self.prev(p), p, self.next(p)) >= 0.
    }
    fn is_ear(&self, ear: usize) -> bool {
        let (a, c, v, b) = self.triangle_box(ear);
        if self.area(a, ear, c) >= 0. {
            return false;
        }
        let mut p = self.next(c);
        while p != a {
            if self.blocks(p, &v, &b) {
                return false;
            }
            p = self.next(p);
        }
        true
    }
    fn is_ear_hashed(&self, ear: usize) -> bool {
        let (a, c, v, b) = self.triangle_box(ear);
        if self.area(a, ear, c) >= 0. {
            return false;
        }
        let min_z = z_order(b[0], b[1], self.min, self.inv_size);
        let max_z = z_order(b[2], b[3], self.min, self.inv_size);
        let (mut p, mut n) = (self.n[ear].prev_z, self.n[ear].next_z);
        while let (Some(pp), Some(nn)) = (p, n) {
            if self.n[pp].z < min_z || self.n[nn].z > max_z {
                break;
            }
            if pp != a && pp != c && self.blocks(pp, &v, &b) {
                return false;
            }
            p = self.n[pp].prev_z;
            if nn != a && nn != c && self.blocks(nn, &v, &b) {
                return false;
            }
            n = self.n[nn].next_z;
        }
        while let Some(pp) = p {
            if self.n[pp].z < min_z {
                break;
            }
            if pp != a && pp != c && self.blocks(pp, &v, &b) {
                return false;
            }
            p = self.n[pp].prev_z;
        }
        while let Some(nn) = n {
            if self.n[nn].z > max_z {
                break;
            }
            if nn != a && nn != c && self.blocks(nn, &v, &b) {
                return false;
            }
            n = self.n[nn].next_z;
        }
        true
    }
    fn cure_local_intersections(&mut self, start: usize) -> usize {
        let mut start = start;
        let mut p = start;
        loop {
            let a = self.prev(p);
            let b = self.next(self.next(p));
            if !self.equals(a, b)
                && self.intersects(a, p, self.next(p), b)
                && self.locally_inside(a, b)
                && self.locally_inside(b, a)
            {
                self.out.extend([self.n[a].i, self.n[p].i, self.n[b].i]);
                let pn = self.next(p);
                self.remove(p);
                self.remove(pn);
                p = b;
                start = b;
            }
            p = self.next(p);
            if p == start {
                break;
            }
        }
        self.filter(p, None)
    }
    fn split_earcut(&mut self, start: usize) {
        let mut a = start;
        loop {
            let mut b = self.next(self.next(a));
            while b != self.prev(a) {
                if self.n[a].i != self.n[b].i && self.is_valid_diagonal(a, b) {
                    let c = self.split_polygon(a, b);
                    let an = self.next(a);
                    let a2 = self.filter(a, Some(an));
                    let cn = self.next(c);
                    let c2 = self.filter(c, Some(cn));
                    self.earcut_linked(a2, 0);
                    self.earcut_linked(c2, 0);
                    return;
                }
                b = self.next(b);
            }
            a = self.next(a);
            if a == start {
                break;
            }
        }
    }
    fn is_valid_diagonal(&self, a: usize, b: usize) -> bool {
        let (an, ap, bp, bn) = (self.next(a), self.prev(a), self.prev(b), self.next(b));
        self.n[an].i != self.n[b].i
            && self.n[ap].i != self.n[b].i
            && !self.intersects_polygon(a, b)
            && ((self.locally_inside(a, b)
                && self.locally_inside(b, a)
                && self.middle_inside(a, b)
                && (self.area(ap, a, bp) != 0. || self.area(a, bp, b) != 0.))
                || (self.equals(a, b) && self.area(ap, a, an) > 0. && self.area(bp, b, bn) > 0.))
    }
    fn intersects(&self, p1: usize, q1: usize, p2: usize, q2: usize) -> bool {
        let sign = |v: f64| {
            if v > 0. {
                1
            } else if v < 0. {
                -1
            } else {
                0
            }
        };
        let o1 = sign(self.area(p1, q1, p2));
        let o2 = sign(self.area(p1, q1, q2));
        let o3 = sign(self.area(p2, q2, p1));
        let o4 = sign(self.area(p2, q2, q1));
        (o1 != o2 && o3 != o4)
            || (o1 == 0 && self.on_segment(p1, p2, q1))
            || (o2 == 0 && self.on_segment(p1, q2, q1))
            || (o3 == 0 && self.on_segment(p2, p1, q2))
            || (o4 == 0 && self.on_segment(p2, q1, q2))
    }
    fn on_segment(&self, p: usize, q: usize, r: usize) -> bool {
        self.x(q) <= self.x(p).max(self.x(r))
            && self.x(q) >= self.x(p).min(self.x(r))
            && self.y(q) <= self.y(p).max(self.y(r))
            && self.y(q) >= self.y(p).min(self.y(r))
    }
    fn intersects_polygon(&self, a: usize, b: usize) -> bool {
        let mut p = a;
        loop {
            let pn = self.next(p);
            if self.n[p].i != self.n[a].i
                && self.n[pn].i != self.n[a].i
                && self.n[p].i != self.n[b].i
                && self.n[pn].i != self.n[b].i
                && self.intersects(p, pn, a, b)
            {
                return true;
            }
            p = pn;
            if p == a {
                return false;
            }
        }
    }
    fn locally_inside(&self, a: usize, b: usize) -> bool {
        let (ap, an) = (self.prev(a), self.next(a));
        if self.area(ap, a, an) < 0. {
            self.area(a, b, an) >= 0. && self.area(a, ap, b) >= 0.
        } else {
            self.area(a, b, ap) < 0. || self.area(a, an, b) < 0.
        }
    }
    fn middle_inside(&self, a: usize, b: usize) -> bool {
        let mut p = a;
        let mut inside = false;
        let (px, py) = ((self.x(a) + self.x(b)) / 2., (self.y(a) + self.y(b)) / 2.);
        loop {
            let pn = self.next(p);
            if ((self.y(p) > py) != (self.y(pn) > py))
                && self.y(pn) != self.y(p)
                && px
                    < (self.x(pn) - self.x(p)) * (py - self.y(p)) / (self.y(pn) - self.y(p))
                        + self.x(p)
            {
                inside = !inside;
            }
            p = pn;
            if p == a {
                return inside;
            }
        }
    }
    fn split_polygon(&mut self, a: usize, b: usize) -> usize {
        let a2 = self.create(self.n[a].i, self.x(a), self.y(a));
        let b2 = self.create(self.n[b].i, self.x(b), self.y(b));
        let (an, bp) = (self.next(a), self.prev(b));
        self.n[a].next = b;
        self.n[b].prev = a;
        self.n[a2].next = an;
        self.n[an].prev = a2;
        self.n[b2].next = a2;
        self.n[a2].prev = b2;
        self.n[bp].next = b2;
        self.n[b2].prev = bp;
        b2
    }
    fn index_curve(&mut self, start: usize) {
        let mut p = start;
        loop {
            if self.n[p].z == 0 {
                self.n[p].z = z_order(self.x(p), self.y(p), self.min, self.inv_size);
            }
            self.n[p].prev_z = Some(self.prev(p));
            self.n[p].next_z = Some(self.next(p));
            p = self.next(p);
            if p == start {
                break;
            }
        }
        let pz = self.n[p].prev_z.expect("ring");
        self.n[pz].next_z = None;
        self.n[p].prev_z = None;
        self.sort_linked(p);
    }
    fn sort_linked(&mut self, list: usize) {
        let mut list = Some(list);
        let mut in_size = 1;
        loop {
            let mut p = list;
            list = None;
            let mut tail: Option<usize> = None;
            let mut merges = 0;
            while let Some(mut pp) = p {
                merges += 1;
                let mut q = Some(pp);
                let mut p_size = 0;
                for _ in 0..in_size {
                    p_size += 1;
                    q = q.and_then(|q| self.n[q].next_z);
                    if q.is_none() {
                        break;
                    }
                }
                let mut q_size = in_size;
                let mut p_opt = Some(pp);
                while p_size > 0 || (q_size > 0 && q.is_some()) {
                    let e;
                    let take_p = p_size != 0
                        && (q_size == 0
                            || q.is_none()
                            || self.n[p_opt.expect("p")].z <= self.n[q.expect("q")].z);
                    if take_p {
                        e = p_opt.expect("p");
                        p_opt = self.n[e].next_z;
                        p_size -= 1;
                    } else {
                        e = q.expect("q");
                        q = self.n[e].next_z;
                        q_size -= 1;
                    }
                    match tail {
                        Some(t) => self.n[t].next_z = Some(e),
                        None => list = Some(e),
                    }
                    self.n[e].prev_z = tail;
                    tail = Some(e);
                }
                pp = match q {
                    Some(q) => q,
                    None => {
                        p = None;
                        continue;
                    }
                };
                p = Some(pp);
            }
            if let Some(t) = tail {
                self.n[t].next_z = None;
            }
            in_size *= 2;
            if merges <= 1 {
                break;
            }
        }
    }
    fn eliminate_holes(&mut self, data: &[f64], holes: &[usize], mut outer: usize) -> usize {
        let mut queue = vec![];
        for (k, &h) in holes.iter().enumerate() {
            let start = h * 2;
            let end = if k + 1 < holes.len() {
                holes[k + 1] * 2
            } else {
                data.len()
            };
            if let Some(list) = self.linked_list(data, start, end, false) {
                if list == self.next(list) {
                    self.n[list].steiner = true;
                }
                queue.push(self.leftmost(list));
            }
        }
        queue.sort_by(|&a, &b| {
            let mut r = self.x(a) - self.x(b);
            if r == 0. {
                r = self.y(a) - self.y(b);
                if r == 0. {
                    let (an, bn) = (self.next(a), self.next(b));
                    let sa = (self.y(an) - self.y(a)) / (self.x(an) - self.x(a));
                    let sb = (self.y(bn) - self.y(b)) / (self.x(bn) - self.x(b));
                    r = sa - sb;
                }
            }
            r.partial_cmp(&0.).unwrap_or(std::cmp::Ordering::Equal)
        });
        for h in queue {
            outer = self.eliminate_hole(h, outer);
        }
        outer
    }
    fn eliminate_hole(&mut self, hole: usize, outer: usize) -> usize {
        let Some(bridge) = self.find_hole_bridge(hole, outer) else {
            return outer;
        };
        let reverse = self.split_polygon(bridge, hole);
        let rn = self.next(reverse);
        self.filter(reverse, Some(rn));
        let bn = self.next(bridge);
        self.filter(bridge, Some(bn))
    }
    fn find_hole_bridge(&self, hole: usize, outer: usize) -> Option<usize> {
        let mut p = outer;
        let (hx, hy) = (self.x(hole), self.y(hole));
        let mut qx = f64::NEG_INFINITY;
        let mut m = None;
        if self.equals(hole, p) {
            return Some(p);
        }
        loop {
            let pn = self.next(p);
            if self.equals(hole, pn) {
                return Some(pn);
            } else if hy <= self.y(p) && hy >= self.y(pn) && self.y(pn) != self.y(p) {
                let x = self.x(p)
                    + (hy - self.y(p)) * (self.x(pn) - self.x(p)) / (self.y(pn) - self.y(p));
                if x <= hx && x > qx {
                    qx = x;
                    let c = if self.x(p) < self.x(pn) { p } else { pn };
                    m = Some(c);
                    if x == hx {
                        return m;
                    }
                }
            }
            p = pn;
            if p == outer {
                break;
            }
        }
        let mut m = m?;
        let stop = m;
        let (mx, my) = (self.x(m), self.y(m));
        let mut tan_min = f64::INFINITY;
        p = m;
        loop {
            let (px, py) = (self.x(p), self.y(p));
            if hx >= px
                && px >= mx
                && hx != px
                && in_triangle(
                    if hy < my { hx } else { qx },
                    hy,
                    mx,
                    my,
                    if hy < my { qx } else { hx },
                    hy,
                    px,
                    py,
                )
            {
                let tan = (hy - py).abs() / (hx - px);
                if self.locally_inside(p, hole)
                    && (tan < tan_min
                        || (tan == tan_min
                            && (px > self.x(m) || (px == self.x(m) && self.sector_contains(m, p)))))
                {
                    m = p;
                    tan_min = tan;
                }
            }
            p = self.next(p);
            if p == stop {
                break;
            }
        }
        Some(m)
    }
    fn sector_contains(&self, m: usize, p: usize) -> bool {
        self.area(self.prev(m), m, self.prev(p)) < 0.
            && self.area(self.next(p), m, self.next(m)) < 0.
    }
    fn leftmost(&self, start: usize) -> usize {
        let (mut p, mut left) = (start, start);
        loop {
            if self.x(p) < self.x(left) || (self.x(p) == self.x(left) && self.y(p) < self.y(left)) {
                left = p;
            }
            p = self.next(p);
            if p == start {
                return left;
            }
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn in_triangle(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, px: f64, py: f64) -> bool {
    (cx - px) * (ay - py) >= (ax - px) * (cy - py)
        && (ax - px) * (by - py) >= (bx - px) * (ay - py)
        && (bx - px) * (cy - py) >= (cx - px) * (by - py)
}
fn signed_area(data: &[f64], start: usize, end: usize) -> f64 {
    let mut sum = 0.;
    let mut j = end - 2;
    let mut i = start;
    while i < end {
        sum += (data[j] - data[i]) * (data[i + 1] + data[j + 1]);
        j = i;
        i += 2;
    }
    sum
}
fn z_order(x: f64, y: f64, min: P2, inv: f64) -> i32 {
    let mut x = ((x - min[0]) * inv) as i32;
    let mut y = ((y - min[1]) * inv) as i32;
    x = (x | (x << 8)) & 0x00FF00FF;
    x = (x | (x << 4)) & 0x0F0F0F0F;
    x = (x | (x << 2)) & 0x33333333;
    x = (x | (x << 1)) & 0x55555555;
    y = (y | (y << 8)) & 0x00FF00FF;
    y = (y | (y << 4)) & 0x0F0F0F0F;
    y = (y | (y << 2)) & 0x33333333;
    y = (y | (y << 1)) & 0x55555555;
    x | (y << 1)
}
/// Earcut.triangulate( data, holeIndices ).
fn earcut(data: &[f64], holes: &[usize]) -> Vec<usize> {
    let outer_len = holes.first().map_or(data.len(), |h| h * 2);
    let mut e = Earcut {
        n: vec![],
        out: vec![],
        min: [0., 0.],
        inv_size: 0.,
    };
    let Some(mut outer) = e.linked_list(data, 0, outer_len, true) else {
        return vec![];
    };
    if e.next(outer) == e.prev(outer) {
        return vec![];
    }
    if !holes.is_empty() {
        outer = e.eliminate_holes(data, holes, outer);
    }
    if data.len() > 160 {
        let (mut min_x, mut min_y) = (data[0], data[1]);
        let (mut max_x, mut max_y) = (min_x, min_y);
        let mut i = 2;
        while i < outer_len {
            let (x, y) = (data[i], data[i + 1]);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            i += 2;
        }
        let size = (max_x - min_x).max(max_y - min_y);
        e.min = [min_x, min_y];
        e.inv_size = if size != 0. { 32767. / size } else { 0. };
    }
    e.earcut_linked(outer, 0);
    e.out
}
/// ShapeUtils.triangulateShape( contour, holes ), after removeDupEndPts.
fn triangulate(contour: &mut Vec<P2>, holes: &mut [Vec<P2>]) -> Vec<[usize; 3]> {
    let trim = |p: &mut Vec<P2>| {
        if p.len() > 2 && p[p.len() - 1] == p[0] {
            p.pop();
        }
    };
    trim(contour);
    let mut data: Vec<f64> = contour.iter().flatten().copied().collect();
    let mut index = contour.len();
    let mut hole_indices = vec![];
    for h in holes.iter_mut() {
        trim(h);
        hole_indices.push(index);
        index += h.len();
        data.extend(h.iter().flatten());
    }
    earcut(&data, &hole_indices)
        .chunks(3)
        .map(|t| [t[0], t[1], t[2]])
        .collect()
}

// ---------------------------------------------------------------- Extrude

pub(super) struct Extrude {
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
pub(super) fn extrude(shapes: &[Shape], o: &Extrude) -> (Vec<f32>, Vec<(usize, usize, usize)>) {
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
            for k in [f[2], f[1], f[0]] {
                push(&mut out, k);
            }
        }
        for f in &faces {
            for k in [f[0], f[1], f[2]] {
                push(&mut out, k + vlen * layers);
            }
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
                    for v in [a, b, d, b, c, d] {
                        push(&mut out, v);
                    }
                }
            }
            offset += len;
        }
        groups.push((start, out.len() / 3 - start, 1));
    }
    (out.into_iter().map(|v| v as f32).collect(), groups)
}
