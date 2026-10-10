//! bounce's EPA: `PenetrationDepthModule.getPenetrationDepthStepEPA` with the
//! `EpaConvexHullBuilder`, its triangle `ArrayPool` ( fixed objects whose
//! array slots swap on free ) and the binary-heap triangle queue. Triangle
//! objects are indices, so the reuse order follows the original's.
use super::gjk::{Simplex, Transformed};
use super::math::*;

const MAX_TRIANGLES: usize = 256;
const MAX_POINTS: usize = 128;
const MAX_EDGE_LENGTH: usize = 128;
const MIN_TRIANGLE_AREA: f64 = 1e-10;
const BARYCENTRIC_EPSILON: f64 = 0.001;
const MAX_POINTS_TO_INCLUDE_ORIGIN: usize = 32;

#[derive(Clone, Copy, Default)]
struct Edge {
    neighbour_triangle: Option<usize>,
    neighbour_edge: usize,
    start_index: usize,
}

#[derive(Clone, Copy, Default)]
struct Triangle {
    edge: [Edge; 3],
    normal: Vec3,
    centroid: Vec3,
    closest_length_sq: f64,
    lambda: [f64; 2],
    lambda_relative_to_0: bool,
    closest_point_interior: bool,
    removed: bool,
    in_queue: bool,
}

impl Triangle {
    fn is_facing(&self, p: Vec3) -> bool {
        let mut ab = Vec3::ZERO;
        ab.subtract_vectors(p, self.centroid);
        self.normal.dot(ab) > 0.
    }
    fn is_facing_origin(&self) -> bool {
        self.normal.dot(self.centroid) < 0.
    }
}

/// The convex shape EPA samples: `ConvexRadiusObject` over A ( its radius in
/// the support direction ) or B as is.
enum Shape<'a> {
    WithRadius(&'a Transformed, f64),
    Plain(&'a Transformed),
}

impl Shape<'_> {
    fn support(&self, direction: Vec3) -> Vec3 {
        match self {
            Shape::WithRadius(t, radius) => {
                let length = direction.length();
                let mut out = t.compute_support(direction);
                if length > 0. {
                    out.add_scaled(direction, radius / length);
                }
                out
            }
            Shape::Plain(t) => t.compute_support(direction),
        }
    }
}

struct Hull {
    triangles: Vec<Triangle>,
    /// ArrayPool: slot order of the triangle objects and each one's slot.
    slots: Vec<usize>,
    slot_of: Vec<usize>,
    count: usize,
    queue: Vec<usize>,
}

fn triangle_sorter(h: &Hull, a: usize, b: usize) -> bool {
    h.triangles[a].closest_length_sq > h.triangles[b].closest_length_sq
}

impl Hull {
    fn new() -> Self {
        Self {
            triangles: vec![Triangle::default(); MAX_TRIANGLES],
            slots: (0..MAX_TRIANGLES).collect(),
            slot_of: (0..MAX_TRIANGLES).collect(),
            count: 0,
            queue: vec![],
        }
    }
    fn create_triangle(
        &mut self,
        i0: usize,
        i1: usize,
        i2: usize,
        positions: &[Vec3],
    ) -> Option<usize> {
        if self.count >= MAX_TRIANGLES {
            return None;
        }
        let t = self.slots[self.count];
        self.slot_of[t] = self.count;
        self.count += 1;
        init_triangle(&mut self.triangles[t], i0, i1, i2, positions);
        Some(t)
    }
    fn free_triangle(&mut self, t: usize) {
        let index = self.slot_of[t];
        let last = self.count - 1;
        if index == last {
            self.count -= 1;
            return;
        }
        self.slots[index] = self.slots[last];
        self.slots[last] = t;
        let moved = self.slots[index];
        self.slot_of[moved] = index;
        self.slot_of[t] = last;
        self.count -= 1;
    }
    fn queue_push(&mut self, t: usize) {
        self.queue.push(t);
        self.triangles[t].in_queue = true;
        // binaryHeapPush
        let mut current = self.queue.len() as isize - 1;
        while current > 0 {
            let parent = (current - 1) >> 1;
            let (c, p) = (self.queue[current as usize], self.queue[parent as usize]);
            if triangle_sorter(self, p, c) {
                self.queue.swap(parent as usize, current as usize);
                current = parent;
            } else {
                break;
            }
        }
    }
    fn queue_pop(&mut self) -> usize {
        // binaryHeapPop over the whole queue, then popBack.
        let end = self.queue.len();
        self.queue.swap(end - 1, 0);
        let count = end - 1;
        let mut largest = 0;
        loop {
            let mut child = (largest << 1) + 1;
            if child >= count {
                break;
            }
            let previous = largest;
            if triangle_sorter(self, self.queue[largest], self.queue[child]) {
                largest = child;
            }
            child += 1;
            if child < count && triangle_sorter(self, self.queue[largest], self.queue[child]) {
                largest = child;
            }
            if previous == largest {
                break;
            }
            self.queue.swap(previous, largest);
        }
        self.queue.pop().expect("a queued triangle")
    }
    fn link(&mut self, t1: usize, e1: usize, t2: usize, e2: usize) {
        self.triangles[t1].edge[e1].neighbour_triangle = Some(t2);
        self.triangles[t1].edge[e1].neighbour_edge = e2;
        self.triangles[t2].edge[e2].neighbour_triangle = Some(t1);
        self.triangles[t2].edge[e2].neighbour_edge = e1;
    }
    fn initialize(&mut self, positions: &[Vec3]) -> Option<()> {
        self.count = 0;
        self.queue.clear();
        let t1 = self.create_triangle(0, 1, 2, positions)?;
        let t2 = self.create_triangle(0, 2, 1, positions)?;
        self.link(t1, 0, t2, 2);
        self.link(t1, 1, t2, 1);
        self.link(t1, 2, t2, 0);
        self.queue_push(t1);
        self.queue_push(t2);
        Some(())
    }
    fn find_facing_triangle(&self, p: Vec3) -> Option<usize> {
        let mut best = None;
        let mut best_dist_sq = 0.;
        for &t in &self.queue {
            let tri = &self.triangles[t];
            if tri.removed {
                continue;
            }
            let mut ab = Vec3::ZERO;
            ab.subtract_vectors(p, tri.centroid);
            let dot = tri.normal.dot(ab);
            if dot > 0. {
                let dist_sq = dot * dot / tri.normal.squared_length();
                if dist_sq > best_dist_sq {
                    best = Some(t);
                    best_dist_sq = dist_sq;
                }
            }
        }
        best
    }
    fn unlink(&mut self, t: usize) {
        for i in 0..3 {
            let e = self.triangles[t].edge[i];
            if let Some(n) = e.neighbour_triangle {
                self.triangles[n].edge[e.neighbour_edge].neighbour_triangle = None;
                self.triangles[t].edge[i].neighbour_triangle = None;
            }
        }
        if !self.triangles[t].in_queue {
            self.free_triangle(t);
        }
    }
    fn find_edge(&mut self, facing: usize, vertex: Vec3, out: &mut Vec<Edge>) -> bool {
        self.triangles[facing].removed = true;
        // ( triangle, edge, iter )
        let mut stack: Vec<(usize, usize, i32)> = vec![(facing, 0, -1)];
        let mut next_expected_start = -1i64;
        loop {
            let top = stack.len() - 1;
            stack[top].2 += 1;
            let (tri, edge, iter) = stack[top];
            if iter >= 3 {
                self.unlink(tri);
                stack.pop();
                if stack.is_empty() {
                    break;
                }
            } else {
                let e = self.triangles[tri].edge[(edge + iter as usize) % 3];
                if let Some(n) = e.neighbour_triangle
                    && !self.triangles[n].removed
                {
                    if self.triangles[n].is_facing(vertex) {
                        self.triangles[n].removed = true;
                        if stack.len() >= MAX_EDGE_LENGTH {
                            return false;
                        }
                        stack.push((n, e.neighbour_edge, 0));
                    } else {
                        if e.start_index as i64 != next_expected_start && next_expected_start != -1
                        {
                            return false;
                        }
                        next_expected_start =
                            self.triangles[n].edge[e.neighbour_edge].start_index as i64;
                        if out.len() >= MAX_EDGE_LENGTH {
                            return false;
                        }
                        out.push(e);
                    }
                }
            }
        }
        out.len() >= 3
    }
    fn add_point(
        &mut self,
        facing: usize,
        index: usize,
        closest_dist_sq: f64,
        positions: &[Vec3],
        out_triangles: &mut Vec<usize>,
    ) -> bool {
        let pos = positions[index];
        let mut edges = vec![];
        if !self.find_edge(facing, pos, &mut edges) {
            return false;
        }
        let n = edges.len();
        for i in 0..n {
            let Some(nt) = self.create_triangle(
                edges[i].start_index,
                edges[(i + 1) % n].start_index,
                index,
                positions,
            ) else {
                return false;
            };
            out_triangles.push(nt);
            let t = &self.triangles[nt];
            if t.closest_point_interior && t.closest_length_sq < closest_dist_sq
                || t.closest_length_sq < 0.
            {
                self.queue_push(nt);
            }
        }
        for i in 0..n {
            let neighbour = edges[i].neighbour_triangle.expect("a hull edge neighbour");
            self.link(out_triangles[i], 0, neighbour, edges[i].neighbour_edge);
            self.link(out_triangles[i], 1, out_triangles[(i + 1) % n], 2);
        }
        true
    }
}

fn init_triangle(m: &mut Triangle, i0: usize, i1: usize, i2: usize, positions: &[Vec3]) {
    m.closest_length_sq = f64::INFINITY;
    m.lambda = [0., 0.];
    m.lambda_relative_to_0 = false;
    m.closest_point_interior = false;
    m.removed = false;
    m.in_queue = false;
    m.edge[0].start_index = i0;
    m.edge[1].start_index = i1;
    m.edge[2].start_index = i2;
    for e in &mut m.edge {
        e.neighbour_triangle = None;
    }
    let (y0, y1, y2) = (positions[i0], positions[i1], positions[i2]);
    m.centroid = Vec3::ZERO;
    m.centroid.add_vector(y0);
    m.centroid.add_vector(y1);
    m.centroid.add_vector(y2);
    m.centroid.scale(1. / 3.);
    let mut y10 = Vec3::ZERO;
    y10.subtract_vectors(y1, y0);
    let mut y20 = Vec3::ZERO;
    y20.subtract_vectors(y2, y0);
    let mut y21 = Vec3::ZERO;
    y21.subtract_vectors(y2, y1);
    let y20_dot_y20 = y20.dot(y20);
    let y21_dot_y21 = y21.dot(y21);
    if y20_dot_y20 < y21_dot_y21 {
        m.normal.cross_vectors(y10, y20);
        let normal_len_sq = m.normal.squared_length();
        if normal_len_sq > MIN_TRIANGLE_AREA {
            let c_dot_n = m.centroid.dot(m.normal);
            m.closest_length_sq = c_dot_n.abs() * c_dot_n / normal_len_sq;
            let y10_dot_y10 = y10.squared_length();
            let y10_dot_y20 = y10.dot(y20);
            let determinant = y10_dot_y10 * y20_dot_y20 - y10_dot_y20 * y10_dot_y20;
            if determinant > 0. {
                let y0_dot_y10 = y0.dot(y10);
                let y0_dot_y20 = y0.dot(y20);
                let l0 = (y10_dot_y20 * y0_dot_y20 - y20_dot_y20 * y0_dot_y10) / determinant;
                let l1 = (y10_dot_y20 * y0_dot_y10 - y10_dot_y10 * y0_dot_y20) / determinant;
                m.lambda = [l0, l1];
                m.lambda_relative_to_0 = true;
                if l0 > -BARYCENTRIC_EPSILON
                    && l1 > -BARYCENTRIC_EPSILON
                    && l0 + l1 < 1. + BARYCENTRIC_EPSILON
                {
                    m.closest_point_interior = true;
                }
            }
        }
    } else {
        m.normal.cross_vectors(y10, y21);
        let normal_len_sq = m.normal.squared_length();
        if normal_len_sq > MIN_TRIANGLE_AREA {
            let c_dot_n = m.centroid.dot(m.normal);
            m.closest_length_sq = c_dot_n.abs() * c_dot_n / normal_len_sq;
            let y10_dot_y10 = y10.squared_length();
            let y10_dot_y21 = y10.dot(y21);
            let determinant = y10_dot_y10 * y21_dot_y21 - y10_dot_y21 * y10_dot_y21;
            if determinant > 0. {
                let y1_dot_y10 = y1.dot(y10);
                let y1_dot_y21 = y1.dot(y21);
                let l0 = (y21_dot_y21 * y1_dot_y10 - y10_dot_y21 * y1_dot_y21) / determinant;
                let l1 = (y10_dot_y21 * y1_dot_y10 - y10_dot_y10 * y1_dot_y21) / determinant;
                m.lambda = [l0, l1];
                m.lambda_relative_to_0 = false;
                if l0 > -BARYCENTRIC_EPSILON
                    && l1 > -BARYCENTRIC_EPSILON
                    && l0 + l1 < 1. + BARYCENTRIC_EPSILON
                {
                    m.closest_point_interior = true;
                }
            }
        }
    }
}

thread_local! {
    /// The module's hull builder: its triangle objects and slot order persist
    /// between calls.
    static HULL: std::cell::RefCell<Hull> = std::cell::RefCell::new(Hull::new());
}

/// `getPenetrationDepthStepEPA( A + maxSeparation radius, B, tolerance )`:
/// ( penetration axis, point on A, point on B ).
pub fn penetration_depth(
    simplex: &Simplex,
    a: &Transformed,
    radius_a: f64,
    b: &Transformed,
    tolerance: f64,
) -> Option<(Vec3, Vec3, Vec3)> {
    HULL.with(|hull| {
        step(
            &mut hull.borrow_mut(),
            simplex,
            &Shape::WithRadius(a, radius_a),
            &Shape::Plain(b),
            tolerance,
        )
    })
}

fn step(
    hull: &mut Hull,
    simplex: &Simplex,
    a: &Shape,
    b: &Shape,
    tolerance: f64,
) -> Option<(Vec3, Vec3, Vec3)> {
    let mut ys: Vec<Vec3> = simplex.y.clone();
    let mut ps: Vec<Vec3> = simplex.p.clone();
    let mut qs: Vec<Vec3> = simplex.q.clone();
    let add = |ys: &mut Vec<Vec3>,
               ps: &mut Vec<Vec3>,
               qs: &mut Vec<Vec3>,
               direction: Vec3|
     -> Option<usize> {
        if ys.len() >= MAX_POINTS {
            return None;
        }
        let mut negated = Vec3::ZERO;
        negated.negate_vector(direction);
        let p = a.support(direction);
        let q = b.support(negated);
        let mut y = Vec3::ZERO;
        y.subtract_vectors(p, q);
        ys.push(y);
        ps.push(p);
        qs.push(q);
        Some(ys.len() - 1)
    };
    match ys.len() {
        1 => {
            ys.pop();
            ps.pop();
            qs.pop();
            for d in [
                Vec3::new(0., 1., 0.),
                Vec3::new(-1., -1., -1.),
                Vec3::new(1., -1., -1.),
                Vec3::new(0., -1., 1.),
            ] {
                add(&mut ys, &mut ps, &mut qs, d)?;
            }
        }
        2 => {
            let mut axis = Vec3::ZERO;
            axis.subtract_vectors(ys[1], ys[0]);
            axis.normalize();
            let mut q = Quat::default();
            q.set_axis_angle(axis, degrees_to_radians(120.));
            let mut rotation = Mat4::default();
            rotation.from_quat(q);
            let mut dir1 = Vec3::ZERO;
            dir1.compute_normalized_perpendicular(axis);
            let dir2 = rotation.multiply_3x3(dir1);
            let dir3 = rotation.multiply_3x3(dir2);
            for d in [dir1, dir2, dir3] {
                add(&mut ys, &mut ps, &mut qs, d)?;
            }
        }
        _ => {}
    }
    hull.initialize(&ys)?;
    for i in 3..ys.len() {
        if let Some(t) = hull.find_facing_triangle(ys[i]) {
            let mut new_triangles = vec![];
            if !hull.add_point(t, i, f64::MAX, &ys, &mut new_triangles) {
                return None;
            }
        }
    }
    loop {
        let t = *hull.queue.first()?;
        if hull.triangles[t].removed {
            hull.queue_pop();
            if hull.queue.is_empty() {
                return None;
            }
            hull.free_triangle(t);
            continue;
        }
        if hull.triangles[t].closest_length_sq >= 0. {
            break;
        }
        hull.queue_pop();
        let normal = hull.triangles[t].normal;
        let new_index = add(&mut ys, &mut ps, &mut qs, normal)?;
        let w = ys[new_index];
        let mut new_triangles = vec![];
        if !hull.triangles[t].is_facing(w)
            || !hull.add_point(t, new_index, f64::MAX, &ys, &mut new_triangles)
        {
            return None;
        }
        hull.free_triangle(t);
        if hull.queue.is_empty() || ys.len() >= MAX_POINTS_TO_INCLUDE_ORIGIN {
            return None;
        }
    }
    let mut closest_dist_sq = f64::MAX;
    let mut last: Option<usize> = None;
    let mut flip_v_sign = false;
    'outer: loop {
        'body: {
            let t = hull.queue_pop();
            if hull.triangles[t].removed {
                hull.free_triangle(t);
                break 'body;
            }
            if hull.triangles[t].closest_length_sq >= closest_dist_sq {
                break 'outer;
            }
            if let Some(l) = last {
                hull.free_triangle(l);
            }
            last = Some(t);
            let normal = hull.triangles[t].normal;
            let new_index = add(&mut ys, &mut ps, &mut qs, normal)?;
            let w = ys[new_index];
            let dot = normal.dot(w);
            if dot < 0. {
                return None;
            }
            let dist_sq = squared(dot) / normal.squared_length();
            if dist_sq - hull.triangles[t].closest_length_sq
                < hull.triangles[t].closest_length_sq * tolerance
            {
                break 'outer;
            }
            closest_dist_sq = min(closest_dist_sq, dist_sq);
            if !hull.triangles[t].is_facing(w) {
                break 'outer;
            }
            let mut new_triangles = vec![];
            if !hull.add_point(t, new_index, closest_dist_sq, &ys, &mut new_triangles) {
                break 'outer;
            }
            let has_defect = new_triangles
                .iter()
                .any(|&nt| hull.triangles[nt].is_facing_origin());
            if has_defect {
                let mut negated = Vec3::ZERO;
                negated.negate_vector(normal);
                let p2 = a.support(negated);
                let q2 = b.support(normal);
                let mut w2 = Vec3::ZERO;
                w2.subtract_vectors(p2, q2);
                if negated.dot(w2) < dot {
                    flip_v_sign = !flip_v_sign;
                }
                break 'outer;
            }
        }
        if !(!hull.queue.is_empty() && ys.len() < MAX_POINTS) {
            break;
        }
    }
    let last = hull.triangles[last?];
    let mut v = last.normal;
    v.scale(last.centroid.dot(last.normal) / last.normal.squared_length());
    if v.is_near_zero() {
        return None;
    }
    if flip_v_sign {
        v.negate();
    }
    let (i0, i1, i2) = (
        last.edge[0].start_index,
        last.edge[1].start_index,
        last.edge[2].start_index,
    );
    let (xp0, xp1, xp2) = (ps[i0], ps[i1], ps[i2]);
    let (xq0, xq1, xq2) = (qs[i0], qs[i1], qs[i2]);
    let mut point_a;
    let mut point_b;
    if last.lambda_relative_to_0 {
        let sub = |p: Vec3, q: Vec3| {
            let mut v = Vec3::ZERO;
            v.subtract_vectors(p, q);
            v
        };
        let (p01, p02, q01, q02) = (sub(xp1, xp0), sub(xp2, xp0), sub(xq1, xq0), sub(xq2, xq0));
        point_a = xp0;
        point_a
            .add_scaled(p01, last.lambda[0])
            .add_scaled(p02, last.lambda[1]);
        point_b = xq0;
        point_b
            .add_scaled(q01, last.lambda[0])
            .add_scaled(q02, last.lambda[1]);
    } else {
        let sub = |p: Vec3, q: Vec3| {
            let mut v = Vec3::ZERO;
            v.subtract_vectors(p, q);
            v
        };
        let (p10, p12, q10, q12) = (sub(xp0, xp1), sub(xp2, xp1), sub(xq0, xq1), sub(xq2, xq1));
        point_a = xp1;
        point_a
            .add_scaled(p10, last.lambda[0])
            .add_scaled(p12, last.lambda[1]);
        point_b = xq1;
        point_b
            .add_scaled(q10, last.lambda[0])
            .add_scaled(q12, last.lambda[1]);
    }
    Some((v, point_a, point_b))
}
