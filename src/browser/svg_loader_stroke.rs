// SVGLoader.pointsToStroke: the stroke triangles of one polyline with its
// joins and caps, as SVGLoader.pointsToStrokeWithBuffers emits them.

fn sub(a: P2, b: P2) -> P2 {
    [a[0] - b[0], a[1] - b[1]]
}
fn add2(a: P2, b: P2) -> P2 {
    [a[0] + b[0], a[1] + b[1]]
}
fn mul(a: P2, s: f64) -> P2 {
    [a[0] * s, a[1] * s]
}
fn dot(a: P2, b: P2) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
fn len(a: P2) -> f64 {
    (a[0] * a[0] + a[1] * a[1]).sqrt()
}
/// Vector2.normalize(): divided by the length, or by 1 when zero.
fn normalize(a: P2) -> P2 {
    let l = len(a);
    let l = if l == 0. { 1. } else { l };
    [a[0] / l, a[1] / l]
}
fn set_length(a: P2, l: f64) -> P2 {
    mul(normalize(a), l)
}
fn distance(a: P2, b: P2) -> f64 {
    len(sub(a, b))
}

struct Stroke<'a> {
    style: &'a Style,
    arc_divisions: usize,
    vertices: Vec<f64>,
    normals: Vec<f64>,
    uvs: Vec<f64>,
    num_vertices: usize,
    last_l: P2,
    last_r: P2,
    current_l: P2,
    current_r: P2,
    next_l: P2,
    next_r: P2,
    inner: P2,
    outer: P2,
    u0: f64,
    u1: f64,
}
impl Stroke<'_> {
    fn vertex(&mut self, p: P2, u: f64, v: f64) {
        self.vertices.extend([p[0], p[1], 0.]);
        self.normals.extend([0., 0., 1.]);
        self.uvs.extend([u, v]);
        self.num_vertices += 3;
    }
    fn circular_sector(&mut self, center: P2, p1: P2, p2: P2, u: f64, v: f64) {
        let t1 = normalize(sub(p1, center));
        let t2 = normalize(sub(p2, center));
        let mut angle = PI;
        let d = dot(t1, t2);
        if d.abs() < 1. {
            angle = d.acos().abs();
        }
        angle /= self.arc_divisions as f64;
        let mut t3 = p1;
        for _ in 0..self.arc_divisions.saturating_sub(1) {
            // Vector2.rotateAround( center, angle ).
            let (c, s) = (angle.cos(), angle.sin());
            let (x, y) = (t3[0] - center[0], t3[1] - center[1]);
            let t4 = [x * c - y * s + center[0], x * s + y * c + center[1]];
            self.vertex(t3, u, v);
            self.vertex(t4, u, v);
            self.vertex(center, u, 0.5);
            t3 = t4;
        }
        self.vertex(t3, u, v);
        self.vertex(p2, u, v);
        self.vertex(center, u, 0.5);
    }
    fn segment_triangles(&mut self) {
        let (u0, u1) = (self.u0, self.u1);
        let (lr, ll, cl, cr) = (self.last_r, self.last_l, self.current_l, self.current_r);
        self.vertex(lr, u0, 1.);
        self.vertex(ll, u0, 0.);
        self.vertex(cl, u1, 0.);
        self.vertex(lr, u0, 1.);
        self.vertex(cl, u1, 0.);
        self.vertex(cr, u1, 1.);
    }
    fn bevel_join(&mut self, left: bool, modified: bool, u: f64, current: P2) {
        let (u0, u1) = (self.u0, self.u1);
        let (lr, ll, cl, cr, nl, nr, inner) = (self.last_r, self.last_l, self.current_l, self.current_r, self.next_l, self.next_r, self.inner);
        if modified {
            if left {
                self.vertex(lr, u0, 1.);
                self.vertex(ll, u0, 0.);
                self.vertex(cl, u1, 0.);
                self.vertex(lr, u0, 1.);
                self.vertex(cl, u1, 0.);
                self.vertex(inner, u1, 1.);
                self.vertex(cl, u, 0.);
                self.vertex(nl, u, 0.);
                self.vertex(inner, u, 0.5);
            } else {
                self.vertex(lr, u0, 1.);
                self.vertex(ll, u0, 0.);
                self.vertex(cr, u1, 1.);
                self.vertex(ll, u0, 0.);
                self.vertex(inner, u1, 0.);
                self.vertex(cr, u1, 1.);
                self.vertex(cr, u, 1.);
                self.vertex(inner, u, 0.);
                self.vertex(nr, u, 1.);
            }
        } else if left {
            self.vertex(cl, u, 0.);
            self.vertex(nl, u, 0.);
            self.vertex(current, u, 0.5);
        } else {
            self.vertex(cr, u, 1.);
            self.vertex(nr, u, 0.);
            self.vertex(current, u, 0.5);
        }
    }
    fn middle_section(&mut self, left: bool, modified: bool, current: P2) {
        if !modified {
            return;
        }
        let (u0, u1) = (self.u0, self.u1);
        let (lr, ll, cl, cr, nl, nr, inner) = (self.last_r, self.last_l, self.current_l, self.current_r, self.next_l, self.next_r, self.inner);
        if left {
            self.vertex(lr, u0, 1.);
            self.vertex(ll, u0, 0.);
            self.vertex(cl, u1, 0.);
            self.vertex(lr, u0, 1.);
            self.vertex(cl, u1, 0.);
            self.vertex(inner, u1, 1.);
            self.vertex(cl, u0, 0.);
            self.vertex(current, u1, 0.5);
            self.vertex(inner, u1, 1.);
            self.vertex(current, u1, 0.5);
            self.vertex(nl, u0, 0.);
            self.vertex(inner, u1, 1.);
        } else {
            self.vertex(lr, u0, 1.);
            self.vertex(ll, u0, 0.);
            self.vertex(cr, u1, 1.);
            self.vertex(ll, u0, 0.);
            self.vertex(inner, u1, 0.);
            self.vertex(cr, u1, 1.);
            self.vertex(cr, u0, 1.);
            self.vertex(inner, u1, 0.);
            self.vertex(current, u1, 0.5);
            self.vertex(current, u1, 0.5);
            self.vertex(inner, u1, 0.);
            self.vertex(nr, u0, 1.);
        }
    }
    /// Vector2.toArray( vertices, offset ): x and y only.
    fn write(&mut self, p: P2, offset: usize) {
        if offset + 1 < self.vertices.len() {
            self.vertices[offset] = p[0];
            self.vertices[offset + 1] = p[1];
        }
    }
    fn cap(&mut self, center: P2, p1: P2, p2: P2, left: bool, start: bool, u: f64) {
        match self.style.line_cap.as_str() {
            "round" => {
                if start {
                    self.circular_sector(center, p2, p1, u, 0.5);
                } else {
                    self.circular_sector(center, p1, p2, u, 0.5);
                }
            }
            "square" => {
                if start {
                    let t1 = sub(p1, center);
                    let t2 = [t1[1], -t1[0]];
                    let t3 = add2(add2(t1, t2), center);
                    let t4 = add2(sub(t2, t1), center);
                    if left {
                        self.write(t3, 3);
                        self.write(t4, 0);
                        self.write(t4, 9);
                    } else {
                        self.write(t3, 3);
                        if self.uvs.get(7) == Some(&1.) {
                            self.write(t4, 9);
                        } else {
                            self.write(t3, 9);
                        }
                        self.write(t4, 0);
                    }
                } else {
                    let t1 = sub(p2, center);
                    let t2 = [t1[1], -t1[0]];
                    let t3 = add2(add2(t1, t2), center);
                    let t4 = add2(sub(t2, t1), center);
                    let vl = self.vertices.len();
                    if left {
                        self.write(t3, vl.wrapping_sub(3));
                        self.write(t4, vl.wrapping_sub(6));
                        self.write(t4, vl.wrapping_sub(12));
                    } else {
                        self.write(t4, vl.wrapping_sub(6));
                        self.write(t3, vl.wrapping_sub(3));
                        self.write(t4, vl.wrapping_sub(12));
                    }
                }
            }
            _ => {}
        }
    }
}

fn points_to_stroke(points: &[P2], style: &Style, arc_divisions: usize, min_distance: f64) -> Result<Option<BufferGeometry>> {
    // removeDuplicatedPoints.
    let mut points = points.to_vec();
    let n = points.len();
    if n > 2 && (1..n - 1).any(|i| distance(points[i], points[i + 1]) < min_distance) {
        let mut kept = vec![points[0]];
        for i in 1..n - 1 {
            if distance(points[i], points[i + 1]) >= min_distance {
                kept.push(points[i]);
            }
        }
        kept.push(points[n - 1]);
        points = kept;
    }
    let num = points.len();
    if num < 2 {
        return Ok(None);
    }
    let closed = points[0] == points[num - 1];
    let w2 = style.stroke_width / 2.;
    let delta_u = 1. / (num - 1) as f64;
    let normal = |p1: P2, p2: P2| {
        let d = sub(p2, p1);
        normalize([-d[1], d[0]])
    };
    let mut k = Stroke {
        style,
        arc_divisions,
        vertices: vec![],
        normals: vec![],
        uvs: vec![],
        num_vertices: 0,
        last_l: [0.; 2],
        last_r: [0.; 2],
        current_l: [0.; 2],
        current_r: [0.; 2],
        next_l: [0.; 2],
        next_r: [0.; 2],
        inner: [0.; 2],
        outer: [0.; 2],
        u0: 0.,
        u1: 0.,
    };
    let t1 = mul(normal(points[0], points[1]), w2);
    k.last_l = sub(points[0], t1);
    k.last_r = add2(points[0], t1);
    let (point0_l, point0_r) = (k.last_l, k.last_r);
    let mut previous = points[0];
    let mut current = points[0];
    let mut inner_modified = false;
    let mut left = false;
    let mut is_miter = false;
    let mut initial_left = false;
    for i in 1..num {
        current = points[i];
        let next = if i == num - 1 {
            closed.then(|| points[1])
        } else {
            Some(points[i + 1])
        };
        let normal1 = normal(previous, current);
        let t3 = mul(normal1, w2);
        k.current_l = sub(current, t3);
        k.current_r = add2(current, t3);
        k.u1 = k.u0 + delta_u;
        inner_modified = false;
        if let Some(next) = next {
            let t3 = mul(normal(current, next), w2);
            k.next_l = sub(current, t3);
            k.next_r = add2(current, t3);
            // JavaScript's !( d < 0 ): a NaN counts as left.
            let d = dot(normal1, sub(next, previous));
            left = d >= 0. || d.is_nan();
            if i == 1 {
                initial_left = left;
            }
            let dir = normalize(sub(next, current));
            let d = dot(normal1, dir).abs();
            if d > EPSILON {
                let miter_side = w2 / d;
                let t3 = mul(dir, -miter_side);
                let t4 = sub(current, previous);
                let t5 = add2(set_length(t4, miter_side), t3);
                k.inner = mul(t5, -1.);
                let miter_length2 = len(t5);
                let prev_len = len(t4);
                let t4 = [t4[0] / prev_len, t4[1] / prev_len];
                let t6 = sub(next, current);
                let next_len = len(t6);
                let t6 = [t6[0] / next_len, t6[1] / next_len];
                if dot(t4, k.inner) < prev_len && dot(t6, k.inner) < next_len {
                    inner_modified = true;
                }
                k.outer = add2(t5, current);
                k.inner = add2(k.inner, current);
                if inner_modified {
                    let r = if left { k.last_r } else { k.last_l };
                    let fold = (k.outer[0] - r[0]) * (k.inner[1] - r[1]) - (k.outer[1] - r[1]) * (k.inner[0] - r[0]);
                    if (left && fold < 0.) || (!left && fold > 0.) {
                        k.inner = r;
                    }
                }
                is_miter = false;
                if inner_modified {
                    if left {
                        k.next_r = k.inner;
                        k.current_r = k.inner;
                    } else {
                        k.next_l = k.inner;
                        k.current_l = k.inner;
                    }
                } else {
                    k.segment_triangles();
                }
                let u1 = k.u1;
                match style.line_join.as_str() {
                    "bevel" => k.bevel_join(left, inner_modified, u1, current),
                    "round" => {
                        k.middle_section(left, inner_modified, current);
                        if left {
                            let (cl, nl) = (k.current_l, k.next_l);
                            k.circular_sector(current, cl, nl, u1, 0.);
                        } else {
                            let (nr, cr) = (k.next_r, k.current_r);
                            k.circular_sector(current, nr, cr, u1, 1.);
                        }
                    }
                    join => {
                        let fraction = (w2 * style.miter_limit) / miter_length2;
                        if fraction < 1. {
                            if join != "miter-clip" {
                                k.bevel_join(left, inner_modified, u1, current);
                            } else {
                                k.middle_section(left, inner_modified, current);
                                let (c, n, v) = if left { (k.current_l, k.next_l, 0.) } else { (k.current_r, k.next_r, 1.) };
                                let t6 = add2(mul(sub(k.outer, c), fraction), c);
                                let t7 = add2(mul(sub(k.outer, n), fraction), n);
                                k.vertex(c, u1, v);
                                k.vertex(t6, u1, v);
                                k.vertex(current, u1, 0.5);
                                k.vertex(current, u1, 0.5);
                                k.vertex(t6, u1, v);
                                k.vertex(t7, u1, v);
                                k.vertex(current, u1, 0.5);
                                k.vertex(t7, u1, v);
                                k.vertex(n, u1, v);
                            }
                        } else {
                            let u0 = k.u0;
                            let (lr, ll, outer, inner) = (k.last_r, k.last_l, k.outer, k.inner);
                            if inner_modified {
                                if left {
                                    k.vertex(lr, u0, 1.);
                                    k.vertex(ll, u0, 0.);
                                    k.vertex(outer, u1, 0.);
                                    k.vertex(lr, u0, 1.);
                                    k.vertex(outer, u1, 0.);
                                    k.vertex(inner, u1, 1.);
                                    k.next_l = outer;
                                } else {
                                    k.vertex(lr, u0, 1.);
                                    k.vertex(ll, u0, 0.);
                                    k.vertex(outer, u1, 1.);
                                    k.vertex(ll, u0, 0.);
                                    k.vertex(inner, u1, 0.);
                                    k.vertex(outer, u1, 1.);
                                    k.next_r = outer;
                                }
                            } else if left {
                                let (cl, nl) = (k.current_l, k.next_l);
                                k.vertex(cl, u1, 0.);
                                k.vertex(outer, u1, 0.);
                                k.vertex(current, u1, 0.5);
                                k.vertex(current, u1, 0.5);
                                k.vertex(outer, u1, 0.);
                                k.vertex(nl, u1, 0.);
                            } else {
                                let (cr, nr) = (k.current_r, k.next_r);
                                k.vertex(cr, u1, 1.);
                                k.vertex(outer, u1, 1.);
                                k.vertex(current, u1, 0.5);
                                k.vertex(current, u1, 0.5);
                                k.vertex(outer, u1, 1.);
                                k.vertex(nr, u1, 1.);
                            }
                            is_miter = true;
                        }
                    }
                }
            } else {
                k.segment_triangles();
            }
        } else {
            k.segment_triangles();
        }
        if !closed && i == num - 1 {
            let u0 = k.u0;
            k.cap(points[0], point0_l, point0_r, left, true, u0);
        }
        k.u0 = k.u1;
        previous = current;
        k.last_l = k.next_l;
        k.last_r = k.next_r;
    }
    if !closed {
        let (cl, cr, u1) = (k.current_l, k.current_r, k.u1);
        k.cap(current, cl, cr, left, false, u1);
    } else if inner_modified {
        let (mut last_outer, mut last_inner) = (k.outer, k.inner);
        if initial_left != left {
            (last_outer, last_inner) = (k.inner, k.outer);
        }
        if left {
            if is_miter || initial_left {
                k.write(last_inner, 0);
                k.write(last_inner, 9);
                if is_miter {
                    k.write(last_outer, 3);
                }
            }
        } else if is_miter || !initial_left {
            k.write(last_inner, 3);
            k.write(last_inner, 9);
            if is_miter {
                k.write(last_outer, 0);
            }
        }
    }
    // Triangles wound clockwise get their second vertex replaced by the first.
    let mut t = 0;
    while t + 8 < k.vertices.len() {
        let v = &k.vertices;
        let tri = [[v[t], v[t + 1]], [v[t + 3], v[t + 4]], [v[t + 6], v[t + 7]]];
        if area(&tri) < 0. {
            k.vertices[t + 3] = tri[0][0];
            k.vertices[t + 4] = tri[0][1];
        }
        t += 9;
    }
    if k.num_vertices == 0 {
        return Ok(None);
    }
    let mut g = BufferGeometry::default();
    let f = |v: &[f64]| v.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    g.set_attribute("position", Attribute::F32(BufferAttribute::new(f(&k.vertices), 3, false)?));
    g.set_attribute("normal", Attribute::F32(BufferAttribute::new(f(&k.normals), 3, false)?));
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(f(&k.uvs), 2, false)?));
    Ok(Some(g))
}
