//! bounce's GJK closest points ( `GjkModule.getClosestPoints` with the
//! closest-point-on-simplex helpers ), `PenetrationDepthModule
//! .getPenetrationDepthStepGjk` and `collideConvexVsConvex`, for the sphere
//! and box support shapes.
use super::epa;
use super::math::*;
use super::world::ShapeKind;

/// A support shape without its convex radius ( `SupportMode
/// .ExcludeConvexRadius` ) or with it ( `IncludeConvexRadius` ).
#[derive(Clone, Copy, Debug)]
pub enum Support {
    /// SphereNoConvex: the center.
    Point,
    /// SphereWithConvex: the full sphere.
    Sphere(f64),
    /// BoxSupport: the box's ( reduced ) extents.
    Box(Aabb),
}

impl Support {
    pub fn compute(&self, direction: Vec3) -> Vec3 {
        match *self {
            Support::Point => Vec3::ZERO,
            Support::Sphere(radius) => {
                let length = direction.length();
                let mut out = Vec3::ZERO;
                if length > 0. {
                    out.scale_vector(direction, radius / length);
                }
                out
            }
            Support::Box(aabb) => aabb.compute_support(direction),
        }
    }
}

/// `computeSupportShape( mode, 1 )` and its `getConvexRadius()`.
fn support_shape(kind: ShapeKind, include_convex_radius: bool) -> (Support, f64) {
    match kind {
        ShapeKind::Sphere { radius } => {
            if include_convex_radius {
                (Support::Sphere(radius), 0.)
            } else {
                (Support::Point, radius)
            }
        }
        ShapeKind::Box {
            width,
            height,
            depth,
            convex_radius,
        } => {
            let half = Vec3::new(width * 0.5, height * 0.5, depth * 0.5);
            let mut scaled = Vec3::ZERO;
            scaled.scale_vector(half, 1.);
            let mut aabb = Aabb::default();
            if include_convex_radius {
                aabb.min.negate_vector(scaled);
                aabb.max = scaled;
                (Support::Box(aabb), 0.)
            } else {
                let radius = min(convex_radius * 1f64.abs(), 0.05);
                let mut r = Vec3::ZERO;
                r.replicate(radius);
                let mut reduced = Vec3::ZERO;
                reduced.subtract_vectors(scaled, r);
                aabb.min.negate_vector(reduced);
                aabb.max = reduced;
                (Support::Box(aabb), radius)
            }
        }
    }
}

/// `TransformedConvexShape`: a support shape under a transform.
#[derive(Clone, Copy)]
pub struct Transformed {
    pub matrix: Mat4,
    pub object: Support,
}

impl Transformed {
    pub fn compute_support(&self, direction: Vec3) -> Vec3 {
        let local = self.matrix.multiply_3x3_transposed(direction);
        let mut out = self.object.compute(local);
        out.transform_by_mat4(&self.matrix);
        out
    }
}

#[derive(Clone, Copy, Default)]
struct Barycentric {
    u: f64,
    v: f64,
    w: f64,
}

fn barycentric_2d(a: Vec3, b: Vec3, squared_tolerance: f64) -> Barycentric {
    let mut ab = Vec3::ZERO;
    ab.subtract_vectors(b, a);
    let denominator = ab.squared_length();
    let mut out = Barycentric::default();
    if denominator < squared_tolerance {
        if a.squared_length() < b.squared_length() {
            out.u = 1.;
            out.v = 0.;
        } else {
            out.u = 0.;
            out.v = 1.;
        }
        return out;
    }
    out.v = -a.dot(ab) / denominator;
    out.u = 1. - out.v;
    out
}

fn barycentric_3d(a: Vec3, b: Vec3, c: Vec3, squared_tolerance: f64) -> Barycentric {
    let mut ab = Vec3::ZERO;
    ab.subtract_vectors(b, a);
    let mut ac = Vec3::ZERO;
    ac.subtract_vectors(c, a);
    let mut bc = Vec3::ZERO;
    bc.subtract_vectors(c, b);
    let d00 = ab.dot(ab);
    let d11 = ac.dot(ac);
    let d22 = bc.dot(bc);
    let mut out = Barycentric::default();
    if d00 <= d22 {
        let d01 = ab.dot(ac);
        let denominator = d00 * d11 - d01 * d01;
        if denominator.abs() < 1e-12 {
            if d00 > d11 {
                let o = barycentric_2d(a, b, squared_tolerance);
                out.u = o.u;
                out.v = o.v;
                out.w = 0.;
            } else {
                let o = barycentric_2d(a, c, squared_tolerance);
                out.u = o.u;
                out.w = o.v;
                out.v = 0.;
            }
            return out;
        }
        let a0 = a.dot(ab);
        let a1 = a.dot(ac);
        out.v = (d01 * a1 - d11 * a0) / denominator;
        out.w = (d01 * a0 - d00 * a1) / denominator;
        out.u = 1. - out.v - out.w;
    } else {
        let d12 = ac.dot(bc);
        let denominator = d11 * d22 - d12 * d12;
        if denominator.abs() < 1e-12 {
            if d11 > d22 {
                let o = barycentric_2d(a, c, squared_tolerance);
                out.u = o.u;
                out.w = o.v;
                out.v = 0.;
            } else {
                let o = barycentric_2d(b, c, squared_tolerance);
                out.v = o.u;
                out.w = o.v;
                out.u = 0.;
            }
            return out;
        }
        let c1 = c.dot(ac);
        let c2 = c.dot(bc);
        out.u = (d22 * c1 - d12 * c2) / denominator;
        out.v = (d11 * c2 - d12 * c1) / denominator;
        out.w = 1. - out.u - out.v;
    }
    out
}

/// A closest point and the simplex point set it lies on.
#[derive(Clone, Copy, Default)]
pub struct Closest {
    pub point: Vec3,
    pub point_set: u32,
}

fn closest_on_line(a: Vec3, b: Vec3, squared_tolerance: f64) -> Closest {
    let bary = barycentric_2d(a, b, squared_tolerance);
    if bary.v <= 0. {
        Closest {
            point: a,
            point_set: 1,
        }
    } else if bary.u <= 0. {
        Closest {
            point: b,
            point_set: 2,
        }
    } else {
        let mut p = Vec3::ZERO;
        p.add_scaled(a, bary.u);
        p.add_scaled(b, bary.v);
        Closest {
            point: p,
            point_set: 3,
        }
    }
}

pub fn closest_on_triangle(
    in_a: Vec3,
    in_b: Vec3,
    in_c: Vec3,
    must_include_c: bool,
    squared_tolerance: f64,
) -> Closest {
    let mut ac = Vec3::ZERO;
    ac.subtract_vectors(in_c, in_a);
    let mut bc = Vec3::ZERO;
    bc.subtract_vectors(in_c, in_b);
    let swap_ac = bc.dot(bc) < ac.dot(ac);
    let a = if swap_ac { in_c } else { in_a };
    let c = if swap_ac { in_a } else { in_c };
    let mut ab = Vec3::ZERO;
    ab.subtract_vectors(in_b, a);
    ac.subtract_vectors(c, a);
    let mut n = Vec3::ZERO;
    n.cross_vectors(ab, ac);
    let normal_length_squared = n.squared_length();
    if normal_length_squared < 1e-11 {
        let mut closest_set = 4;
        let mut closest = in_c;
        let mut best = in_c.squared_length();
        if !must_include_c {
            let al = in_a.squared_length();
            if al < best {
                closest_set = 1;
                closest = in_a;
                best = al;
            }
            let bl = in_b.squared_length();
            if bl < best {
                closest_set = 2;
                closest = in_b;
                best = bl;
            }
        }
        let acl = ac.squared_length();
        if acl > squared_tolerance {
            let v = clamp(-a.dot(ac) / acl, 0., 1.);
            let mut q = Vec3::ZERO;
            q.add_scaled_to_vector(a, ac, v);
            let d = q.squared_length();
            if d < best {
                closest_set = 5;
                closest = q;
                best = d;
            }
        }
        bc.subtract_vectors(in_c, in_b);
        let bcl = bc.squared_length();
        if bcl > squared_tolerance {
            let v = clamp(-in_b.dot(bc) / bcl, 0., 1.);
            let mut q = Vec3::ZERO;
            q.add_scaled_to_vector(in_b, bc, v);
            let d = q.squared_length();
            if d < best {
                closest_set = 6;
                closest = q;
                best = d;
            }
        }
        if !must_include_c {
            ab.subtract_vectors(in_b, in_a);
            let abl = ab.squared_length();
            if abl > squared_tolerance {
                let v = clamp(-in_a.dot(ab) / abl, 0., 1.);
                let mut q = Vec3::ZERO;
                q.add_scaled_to_vector(in_a, ab, v);
                let d = q.squared_length();
                if d < best {
                    closest_set = 3;
                    closest = q;
                }
            }
        }
        return Closest {
            point: closest,
            point_set: closest_set,
        };
    }
    let mut ap = Vec3::ZERO;
    ap.negate_vector(a);
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0. && d2 <= 0. {
        return Closest {
            point: a,
            point_set: if swap_ac { 4 } else { 1 },
        };
    }
    let mut bp = Vec3::ZERO;
    bp.negate_vector(in_b);
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0. && d4 <= d3 {
        return Closest {
            point: in_b,
            point_set: 2,
        };
    }
    if d1 * d4 <= d3 * d2 && d1 >= 0. && d3 <= 0. {
        let v = d1 / (d1 - d3);
        let mut p = Vec3::ZERO;
        p.add_scaled_to_vector(a, ab, v);
        return Closest {
            point: p,
            point_set: if swap_ac { 6 } else { 3 },
        };
    }
    let mut cp = Vec3::ZERO;
    cp.negate_vector(c);
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0. && d5 <= d6 {
        return Closest {
            point: c,
            point_set: if swap_ac { 1 } else { 4 },
        };
    }
    if d5 * d2 <= d1 * d6 && d2 >= 0. && d6 <= 0. {
        let w = d2 / (d2 - d6);
        let mut p = Vec3::ZERO;
        p.add_scaled_to_vector(a, ac, w);
        return Closest {
            point: p,
            point_set: 5,
        };
    }
    let diff_d4_d3 = d4 - d3;
    let diff_d5_d6 = d5 - d6;
    if d3 * d6 <= d5 * d4 && diff_d4_d3 >= 0. && diff_d5_d6 >= 0. {
        let w = diff_d4_d3 / (diff_d4_d3 + diff_d5_d6);
        let mut t = Vec3::ZERO;
        t.subtract_vectors(c, in_b);
        let mut p = Vec3::ZERO;
        p.add_scaled_to_vector(in_b, t, w);
        return Closest {
            point: p,
            point_set: if swap_ac { 3 } else { 6 },
        };
    }
    let mut t = Vec3::ZERO;
    t.add_vectors(a, in_b);
    let t0 = t;
    t.add_vectors(t0, c);
    let mut p = Vec3::ZERO;
    p.scale_vector(n, t.dot(n) / (3. * normal_length_squared));
    Closest {
        point: p,
        point_set: 7,
    }
}

fn origin_outside_of_triangle_planes(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    tolerance: f64,
) -> [bool; 4] {
    let sub = |p: Vec3, q: Vec3| {
        let mut v = Vec3::ZERO;
        v.subtract_vectors(p, q);
        v
    };
    let cross = |p: Vec3, q: Vec3| {
        let mut v = Vec3::ZERO;
        v.cross_vectors(p, q);
        v
    };
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ad = sub(d, a);
    let bd = sub(d, b);
    let bc = sub(c, b);
    let ab_cross_ac = cross(ab, ac);
    let ac_cross_ad = cross(ac, ad);
    let ad_cross_ab = cross(ad, ab);
    let bd_cross_bc = cross(bd, bc);
    let sp = [
        a.dot(ab_cross_ac),
        a.dot(ac_cross_ad),
        a.dot(ad_cross_ab),
        b.dot(bd_cross_bc),
    ];
    let sd = [
        ad.dot(ab_cross_ac),
        ab.dot(ac_cross_ad),
        ac.dot(ad_cross_ab),
        -ab.dot(bd_cross_bc),
    ];
    if sd.iter().all(|&s| s > 0.) {
        return sp.map(|s| s >= -tolerance);
    }
    if sd.iter().all(|&s| s < 0.) {
        return sp.map(|s| s <= tolerance);
    }
    [true; 4]
}

fn closest_on_tetrahedron(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    must_include_d: bool,
    tolerance: f64,
) -> Closest {
    let squared_tolerance = tolerance * tolerance;
    let mut result = Closest {
        point: Vec3::ZERO,
        point_set: 15,
    };
    let mut best = f64::INFINITY;
    let outside = origin_outside_of_triangle_planes(a, b, c, d, tolerance);
    if outside[0] {
        if must_include_d {
            result = Closest {
                point: a,
                point_set: 1,
            };
        } else {
            result = closest_on_triangle(a, b, c, false, squared_tolerance);
        }
        best = result.point.squared_length();
    }
    if outside[1] {
        let o = closest_on_triangle(a, c, d, must_include_d, squared_tolerance);
        let distance = o.point.squared_length();
        if distance < best {
            best = distance;
            result.point = o.point;
            result.point_set = (o.point_set & 1) + ((o.point_set & 6) << 1);
        }
    }
    if outside[2] {
        let o = closest_on_triangle(a, b, d, must_include_d, squared_tolerance);
        let distance = o.point.squared_length();
        if distance < best {
            best = distance;
            result.point = o.point;
            result.point_set = (o.point_set & 3) + ((o.point_set & 4) << 1);
        }
    }
    if outside[3] {
        let o = closest_on_triangle(b, c, d, must_include_d, squared_tolerance);
        if o.point.squared_length() < best {
            result.point = o.point;
            result.point_set = o.point_set << 1;
        }
    }
    result
}

/// `computeClosestPointToSimplex`: None when no closer point is found.
fn closest_point_to_simplex(previous: f64, simplex: &[Vec3]) -> Option<(Closest, f64)> {
    let closest = match simplex.len() {
        1 => Closest {
            point: simplex[0],
            point_set: 1,
        },
        2 => closest_on_line(simplex[0], simplex[1], 1e-10),
        3 => closest_on_triangle(simplex[0], simplex[1], simplex[2], true, 1e-10),
        _ => closest_on_tetrahedron(simplex[0], simplex[1], simplex[2], simplex[3], true, 1e-5),
    };
    let squared_distance = closest.point.squared_length();
    (squared_distance < previous).then_some((closest, squared_distance))
}

/// GJK's simplex: support points of the difference ( y ), A ( p ) and B ( q ).
#[derive(Clone, Default)]
pub struct Simplex {
    pub y: Vec<Vec3>,
    pub p: Vec<Vec3>,
    pub q: Vec<Vec3>,
}

impl Simplex {
    fn keep(&mut self, set: u32) {
        let mut n = 0;
        for i in 0..self.y.len() {
            if set & (1 << i) != 0 {
                self.y[n] = self.y[i];
                self.p[n] = self.p[i];
                self.q[n] = self.q[i];
                n += 1;
            }
        }
        self.y.truncate(n);
        self.p.truncate(n);
        self.q.truncate(n);
    }
    fn points(&self) -> (Vec3, Vec3) {
        let mut a = Vec3::ZERO;
        let mut b = Vec3::ZERO;
        match self.y.len() {
            1 => {
                a = self.p[0];
                b = self.q[0];
            }
            2 => {
                let bary = barycentric_2d(self.y[0], self.y[1], 1e-10);
                a.add_scaled(self.p[0], bary.u);
                a.add_scaled(self.p[1], bary.v);
                b.add_scaled(self.q[0], bary.u);
                b.add_scaled(self.q[1], bary.v);
            }
            3 => {
                let bary = barycentric_3d(self.y[0], self.y[1], self.y[2], 1e-10);
                a.add_scaled(self.p[0], bary.u);
                a.add_scaled(self.p[1], bary.v);
                a.add_scaled(self.p[2], bary.w);
                b.add_scaled(self.q[0], bary.u);
                b.add_scaled(self.q[1], bary.v);
                b.add_scaled(self.q[2], bary.w);
            }
            _ => {}
        }
        (a, b)
    }
}

pub struct GjkClosestPoints {
    pub squared_distance: f64,
    pub penetration_axis: Vec3,
    pub point_a: Vec3,
    pub point_b: Vec3,
}

/// `getClosestPoints( result, inA, inB, inTolerance, inMaxDistanceSquared,
/// inDirection )`; the simplex is left for EPA. An early out keeps the
/// previous call's points, as the original's module state does.
pub fn closest_points(
    a: &Transformed,
    b: &Transformed,
    tolerance: f64,
    max_distance_squared: f64,
    direction: Vec3,
    simplex: &mut Simplex,
    stale: &mut GjkClosestPoints,
) {
    let squared_tolerance = squared(tolerance);
    simplex.y.clear();
    simplex.p.clear();
    simplex.q.clear();
    let mut v_squared = direction.squared_length();
    let mut v = direction;
    let mut previous = f64::MAX;
    let mut iterations = 0;
    let gjk_tolerance = 1e-5;
    while {
        iterations += 1;
        iterations - 1 < 100
    } {
        let mut negated = Vec3::ZERO;
        negated.negate_vector(v);
        let p = a.compute_support(v);
        let q = b.compute_support(negated);
        let mut w = Vec3::ZERO;
        w.subtract_vectors(p, q);
        let dot = v.dot(w);
        if dot < 0. && dot * dot > v_squared * max_distance_squared {
            stale.squared_distance = f64::INFINITY;
            return;
        }
        simplex.y.push(w);
        simplex.p.push(p);
        simplex.q.push(q);
        let Some((closest, d)) = closest_point_to_simplex(previous, &simplex.y) else {
            simplex.y.pop();
            simplex.p.pop();
            simplex.q.pop();
            break;
        };
        v = closest.point;
        v_squared = d;
        if closest.point_set == 15 {
            v = Vec3::ZERO;
            v_squared = 0.;
            break;
        }
        simplex.keep(closest.point_set);
        if v_squared <= squared_tolerance {
            v = Vec3::ZERO;
            v_squared = 0.;
            break;
        }
        let max_y = simplex
            .y
            .iter()
            .skip(1)
            .fold(simplex.y[0].squared_length(), |m, y| {
                max(m, y.squared_length())
            });
        if v_squared <= gjk_tolerance * max_y {
            v = Vec3::ZERO;
            v_squared = 0.;
            break;
        }
        v.negate();
        if previous - v_squared <= gjk_tolerance * previous {
            break;
        }
        previous = v_squared;
    }
    let (pa, pb) = simplex.points();
    stale.point_a = pa;
    stale.point_b = pb;
    stale.squared_distance = v_squared;
    stale.penetration_axis = v;
}

pub enum Penetration {
    NotColliding,
    Colliding,
    Indeterminate,
}

/// `collideConvexVsConvex` for a body pair: ( penetration, contact points
/// and normal in A's space, with A's rotation applied ), and whether EPA ran.
pub fn collide_convex_vs_convex(
    kind_a: ShapeKind,
    kind_b: ShapeKind,
    iso_a: &Mat4,
    iso_b: &Mat4,
    max_separation: f64,
    collision_tolerance: f64,
    penetration_tolerance: f64,
) -> (Option<(f64, Vec3, Vec3, Vec3)>, bool) {
    thread_local! {
        static STATE: std::cell::RefCell<(Simplex, GjkClosestPoints)> = std::cell::RefCell::new((
            Simplex::default(),
            GjkClosestPoints {
                squared_distance: 0.,
                penetration_axis: Vec3::ZERO,
                point_a: Vec3::ZERO,
                point_b: Vec3::ZERO,
            },
        ));
    }
    let (support_a, radius_a) = support_shape(kind_a, false);
    let (support_b, radius_b) = support_shape(kind_b, false);
    let convex_radius_a = radius_a + max_separation;
    let convex_radius_b = radius_b;
    let mut identity = Mat4::default();
    identity.identity();
    let mut inverse_a = Mat4::default();
    inverse_a.invert_matrix(iso_a);
    let mut b_to_a = Mat4::default();
    b_to_a.multiply_matrices(&inverse_a, iso_b);
    let position_a = iso_a.get_translation();
    let position_b = b_to_a.get_translation();
    let mut ab = Vec3::ZERO;
    ab.subtract_vectors(position_b, position_a);
    let mut search = ab;
    search.normalize();
    if search.is_near_zero() {
        search = Vec3::new(0., -1., 0.);
    }
    let mut ta = Transformed {
        matrix: identity,
        object: support_a,
    };
    let tb = Transformed {
        matrix: b_to_a,
        object: support_b,
    };
    STATE.with(|state| {
        let (simplex, closest) = &mut *state.borrow_mut();
        // getPenetrationDepthStepGjk
        let combined = squared(convex_radius_a + convex_radius_b);
        closest_points(
            &ta,
            &tb,
            collision_tolerance,
            combined,
            search,
            simplex,
            closest,
        );
        let mut point_a = closest.point_a;
        let mut point_b = closest.point_b;
        let axis = closest.penetration_axis;
        let status = if closest.squared_distance > combined {
            Penetration::NotColliding
        } else if closest.squared_distance > 0. {
            let length = closest.squared_distance.sqrt();
            point_a.add_scaled(axis, convex_radius_a / length);
            point_b.add_scaled(axis, -convex_radius_b / length);
            Penetration::Colliding
        } else {
            Penetration::Indeterminate
        };
        let finish = |mut pa: Vec3, mut pb: Vec3, axis: Vec3| {
            let mut ab = Vec3::ZERO;
            ab.subtract_vectors(pb, pa);
            let depth = ab.length() - max_separation;
            // CollisionCollector.getEarlyOutFraction() is Infinity.
            if -depth >= f64::INFINITY {
                return None;
            }
            let axis_length = axis.length();
            if axis_length > 0. {
                pa.add_scaled(axis, -(max_separation / axis_length));
            }
            pa.transform_by_mat4(iso_a);
            pb.transform_by_mat4(iso_a);
            let normal = iso_a.multiply_3x3(axis);
            Some((depth, pa, pb, normal))
        };
        match status {
            Penetration::NotColliding => (None, false),
            Penetration::Colliding => (finish(point_a, point_b, axis), false),
            Penetration::Indeterminate => {
                let (with_a, _) = support_shape(kind_a, true);
                let (with_b, _) = support_shape(kind_b, true);
                ta.object = with_a;
                let tb = Transformed {
                    matrix: b_to_a,
                    object: with_b,
                };
                let result = epa::penetration_depth(
                    simplex,
                    &ta,
                    max_separation,
                    &tb,
                    penetration_tolerance,
                );
                match result {
                    Some((v, pa, pb)) => (finish(pa, pb, v), true),
                    None => (None, true),
                }
            }
        }
    })
}
