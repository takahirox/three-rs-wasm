//! bounce 1.8.1's `World` over sphere and box bodies: the step, the BVH
//! broadphase and its body pairs, the convex narrowphase ( sphere vs
//! sphere, GJK with EPA for sphere and box ), the contact manifold caches,
//! the contact constraint solver with warm starting, and sleeping. Pools,
//! linked lists and sets keep the original's orders, since those decide the
//! order the solver applies its impulses in.
use super::gjk;
use super::math::*;
use super::pool::{OrderedSet, Pool};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BodyType {
    Dynamic = 0,
    Kinematic = 1,
    Static = 2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeKind {
    Sphere {
        radius: f64,
    },
    Box {
        width: f64,
        height: f64,
        depth: f64,
        convex_radius: f64,
    },
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub kind: ShapeKind,
    pub center_of_mass: Vec3,
    pub volume: f64,
    pub aabb: Aabb,
}

impl Shape {
    /// `updateShape( shape )` after `create`.
    fn new(kind: ShapeKind) -> Self {
        let mut s = Shape {
            kind,
            center_of_mass: Vec3::ZERO,
            volume: 0.,
            aabb: Aabb::default(),
        };
        match kind {
            ShapeKind::Sphere { radius } => {
                s.volume = 4. / 3. * std::f64::consts::PI * radius * radius * radius;
                s.aabb.min.add_scalar_to_vector(s.center_of_mass, -radius);
                s.aabb.max.add_scalar_to_vector(s.center_of_mass, radius);
            }
            ShapeKind::Box {
                width,
                height,
                depth,
                ..
            } => {
                s.volume = width * height * depth;
                let half = Vec3::new(width * 0.5, height * 0.5, depth * 0.5);
                s.aabb.min.negate_vector(half);
                s.aabb.max = half;
            }
        }
        s
    }
    fn inverse_inertia(&self, mass: f64) -> Mat3 {
        let mut m = Mat3::default();
        match self.kind {
            ShapeKind::Sphere { radius } => {
                let inertia = 2. / 5. * mass * radius * radius;
                m.e = [inertia, 0., 0., 0., inertia, 0., 0., 0., inertia];
            }
            ShapeKind::Box {
                width,
                height,
                depth,
                ..
            } => {
                let (w2, h2, d2) = (width * width, height * height, depth * depth);
                m.e = [
                    mass * (h2 + d2) / 12.,
                    0.,
                    0.,
                    0.,
                    mass * (w2 + d2) / 12.,
                    0.,
                    0.,
                    0.,
                    mass * (w2 + h2) / 12.,
                ];
            }
        }
        m.invert();
        m
    }
    fn world_bounds(&self, translation: Vec3, rotation: Quat) -> Aabb {
        let mut out = Aabb::default();
        match self.kind {
            ShapeKind::Sphere { .. } => {
                out.translate_aabb(&self.aabb, translation);
            }
            ShapeKind::Box { .. } => {
                out.transform_aabb(&self.aabb, &isometry(rotation, translation));
            }
        }
        out
    }
}

/// The fields `copyForDiff` compares.
#[derive(Clone, Copy, Debug, Default)]
struct BodyCopy {
    position: Vec3,
    orientation: Quat,
    center_of_mass: Vec3,
    mass: f64,
    density: f64,
}

#[derive(Clone, Debug)]
pub struct Body {
    pub kind: BodyType,
    pub position: Vec3,
    pub orientation: Quat,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub friction: f64,
    pub restitution: f64,
    pub is_sleeping: bool,
    gravity_scale: f64,
    density: f64,
    mass: f64,
    linear_damping: f64,
    angular_damping: f64,
    time_without_moving: f64,
    pub center_of_mass: Vec3,
    bounds: Aabb,
    expanded_bounds: Aabb,
    shape: usize,
    sleep_visit_generation: u64,
    inverse_mass: f64,
    local_inverse_inertia: Mat3,
    world_inverse_inertia: Mat3,
    linear_forces: Vec3,
    angular_forces: Vec3,
    copy: BodyCopy,
    first_pair_edge: Option<usize>,
}

impl Body {
    fn compute_inverse_inertia_tensor(&mut self) -> Mat3 {
        let local = self.local_inverse_inertia;
        transform_tensor(&mut self.world_inverse_inertia, &local, self.orientation);
        self.world_inverse_inertia
    }
    fn velocity_of_point(&self, point: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        out.cross_vectors(self.angular_velocity, point);
        out.add_vector(self.linear_velocity);
        out
    }
    fn is_ready_to_sleep(&self, threshold: f64) -> bool {
        self.kind != BodyType::Dynamic || self.time_without_moving >= threshold
    }
    fn rotation_delta(&mut self, delta: Vec3, sign: f64) {
        let angle = delta.length();
        if angle <= 1e-6 {
            return;
        }
        let mut axis = Vec3::ZERO;
        axis.scale_vector(delta, 1. / angle);
        let mut q = Quat::default();
        q.set_axis_angle(axis, sign * angle);
        let o = self.orientation;
        self.orientation.multiply_quats(q, o);
        self.orientation.normalize();
    }
}

#[derive(Clone, Debug, Default)]
struct BvhNode {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    bounds: Aabb,
    height: f64,
    objects: Vec<usize>,
}

impl BvhNode {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

struct BvhTree {
    root: Option<usize>,
    expansion_margin: f64,
    max_depth: f64,
    max_objects_per_leaf: usize,
    body_to_node: Vec<Option<usize>>,
    dirty: OrderedSet,
}

#[derive(Clone, Debug, Default)]
struct PairNode {
    body_a: usize,
    body_b: usize,
    edge_a: usize,
    edge_b: usize,
}

#[derive(Clone, Debug, Default)]
struct PairEdge {
    node: usize,
    next: Option<usize>,
    prev: Option<usize>,
}

/// A contact manifold: its contact points live in reference lists that keep
/// their slots when the manifold is reused.
#[derive(Clone, Debug, Default)]
struct Manifold {
    body_a: usize,
    body_b: usize,
    next: Option<usize>,
    base_translation: Vec3,
    first_world_space_normal: Vec3,
    world_space_normal: Vec3,
    penetration_depth: f64,
    num_contacts: usize,
    points_a: Vec<Vec3>,
    points_b: Vec<Vec3>,
    lambdas: Vec<Vec3>,
}

#[derive(Clone, Debug, Default)]
struct ContactPair {
    first_manifold: Option<usize>,
    translation_ab: Vec3,
    rotation_ab: Quat,
}

#[derive(Default)]
struct ManifoldCache {
    manifolds: Pool<Manifold>,
    manifold_map: HashMap<String, usize>,
    pairs: Pool<ContactPair>,
    pair_map: HashMap<String, usize>,
}

impl ManifoldCache {
    fn clear(&mut self) {
        self.manifold_map.clear();
        self.manifolds.destroy_all();
        self.pair_map.clear();
        self.pairs.destroy_all();
    }
}

/// `DirectionalConstraint`: its fields persist in the pooled contact
/// constraint between uses, as the original's do.
#[derive(Clone, Copy, Debug, Default)]
struct Directional {
    r1_plus_u_x_axis: Vec3,
    r2_x_axis: Vec3,
    inv_i1_r1_plus_u_x_axis: Vec3,
    inv_i2_r2_x_axis: Vec3,
    effective_mass: f64,
    bias: f64,
    softness: f64,
    total_lambda: f64,
}

#[derive(Clone, Copy, Debug, Default)]
struct ContactConstraint {
    normal: Directional,
    tangent: Directional,
    bitangent: Directional,
    local_position_a: Vec3,
    local_position_b: Vec3,
}

#[derive(Clone, Debug, Default)]
struct ManifoldConstraint {
    body_a: usize,
    body_b: usize,
    friction: f64,
    restitution: f64,
    world_space_normal: Vec3,
    world_space_tangent: Vec3,
    world_space_bitangent: Vec3,
    inverse_mass_a: f64,
    inverse_mass_b: f64,
    contacts: Vec<ContactConstraint>,
    num_contacts: usize,
}

impl ManifoldConstraint {
    fn update_tangent_directions(&mut self) {
        let n = self.world_space_normal;
        self.world_space_tangent.compute_normalized_perpendicular(n);
        let t = self.world_space_tangent;
        self.world_space_bitangent.cross_vectors(n, t);
    }
}

/// The collision result the narrowphase hands the collector.
struct Hit {
    penetration: f64,
    contact_point_a: Vec3,
    contact_point_b: Vec3,
    normal_a: Vec3,
}

/// The world options the page sets, over bounce's defaults.
#[derive(Clone, Debug)]
pub struct Options {
    pub gravity: Vec3,
    pub restitution: f64,
    pub friction: f64,
    pub solve_velocity_iterations: usize,
    pub solve_position_iterations: usize,
    pub linear_damping: f64,
    pub angular_damping: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.80665, 0.),
            restitution: 0.2,
            friction: 0.5,
            solve_velocity_iterations: 6,
            solve_position_iterations: 2,
            linear_damping: 0.05,
            angular_damping: 0.05,
        }
    }
}

const MIN_VELOCITY_FOR_ELASTIC_CONTACT: f64 = 2.;
const BAUMGARTE: f64 = 0.2;
const PENETRATION_SLOP: f64 = 0.02;
const MAX_PENETRATION_DISTANCE: f64 = 0.2;
const SPECULATIVE_CONTACT_DISTANCE: f64 = 0.02;
const MAX_LINEAR_SPEED: f64 = 30.;
const MAX_ANGULAR_SPEED: f64 = 20.;
const COLLISION_TOLERANCE: f64 = 1e-4;
const PENETRATION_TOLERANCE: f64 = 1e-4;
const MAX_TIME_TO_SIMULATE: f64 = 0.2;
const SLEEP_VELOCITY_THRESHOLD: f64 = 0.03;
const SLEEP_SPIN_THRESHOLD: f64 = 0.03;
const SLEEP_TIME_THRESHOLD: f64 = 0.5;

pub struct World {
    options: Options,
    shapes: Vec<Shape>,
    pub bodies: Pool<Body>,
    pub dynamic_bodies: Vec<usize>,
    bvh_nodes: Pool<BvhNode>,
    dynamic_tree: BvhTree,
    static_tree: BvhTree,
    pair_nodes: Pool<PairNode>,
    pair_edges: Pool<PairEdge>,
    caches: [ManifoldCache; 2],
    current_cache: usize,
    collector_manifolds: Pool<Manifold>,
    manifold_constraints: Pool<ManifoldConstraint>,
    sleep_visit_generation: u64,
    accumulated: f64,
    last_advance: Option<f64>,
    previous_time_step: f64,
    pub step: u64,
    /// GJK found the shrunken shapes overlapping: EPA ran.
    pub epa_steps: u64,
}

fn contact_pair_key(a: &Body, a_index: usize, b: &Body, b_index: usize) -> String {
    let key_a = format!("{}:{}", a.kind as u8, a_index);
    let key_b = format!("{}:{}", b.kind as u8, b_index);
    if key_a < key_b {
        format!("{key_a}|{key_b}")
    } else {
        format!("{key_b}|{key_a}")
    }
}

impl World {
    pub fn new(options: Options) -> Self {
        let tree = || BvhTree {
            root: None,
            expansion_margin: 0.1,
            max_depth: 32.,
            max_objects_per_leaf: 8,
            body_to_node: vec![],
            dirty: OrderedSet::default(),
        };
        Self {
            options,
            shapes: vec![],
            bodies: Pool::default(),
            dynamic_bodies: vec![],
            bvh_nodes: Pool::default(),
            dynamic_tree: tree(),
            static_tree: tree(),
            pair_nodes: Pool::default(),
            pair_edges: Pool::default(),
            caches: [ManifoldCache::default(), ManifoldCache::default()],
            current_cache: 0,
            collector_manifolds: Pool::default(),
            manifold_constraints: Pool::default(),
            sleep_visit_generation: 0,
            accumulated: 0.,
            last_advance: None,
            previous_time_step: 1. / 60.,
            step: 0,
            epa_steps: 0,
        }
    }
    pub fn create_sphere(&mut self, radius: f64) -> usize {
        self.shapes.push(Shape::new(ShapeKind::Sphere { radius }));
        self.shapes.len() - 1
    }
    pub fn create_box(&mut self, width: f64, height: f64, depth: f64) -> usize {
        self.shapes.push(Shape::new(ShapeKind::Box {
            width,
            height,
            depth,
            convex_radius: 0.05,
        }));
        self.shapes.len() - 1
    }

    /// `Body.create( data, pool )` and `setShape`.
    fn create_body(
        &mut self,
        kind: BodyType,
        shape: usize,
        position: Vec3,
        mass: f64,
        restitution: Option<f64>,
        friction: Option<f64>,
    ) -> usize {
        let body = Body {
            kind,
            position,
            orientation: Quat::default(),
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            // monomorph writes each default into `data` while it constructs, so
            // Body.create's `"friction" in data` check always holds: a body
            // created without them keeps 0, not the world's values.
            friction: friction.unwrap_or(0.),
            restitution: restitution.unwrap_or(0.),
            is_sleeping: false,
            gravity_scale: 1.,
            density: 0.,
            mass,
            linear_damping: -1.,
            angular_damping: -1.,
            time_without_moving: 0.,
            center_of_mass: Vec3::ZERO,
            bounds: Aabb::default(),
            expanded_bounds: Aabb::default(),
            shape,
            sleep_visit_generation: 0,
            inverse_mass: 0.,
            local_inverse_inertia: Mat3::default(),
            world_inverse_inertia: Mat3::default(),
            linear_forces: Vec3::ZERO,
            angular_forces: Vec3::ZERO,
            copy: BodyCopy::default(),
            first_pair_edge: None,
        };
        let index = self
            .bodies
            .create(|_| unreachable!("bodies are never destroyed"), || body);
        self.bodies[index].orientation.normalize();
        self.update_body(index);
        // bodyCopy.copy( body )
        let b = &mut self.bodies[index];
        b.copy = BodyCopy {
            position: b.position,
            orientation: b.orientation,
            center_of_mass: b.center_of_mass,
            mass: b.mass,
            density: b.density,
        };
        index
    }
    pub fn create_static_body(&mut self, shape: usize, position: Vec3) -> usize {
        let index = self.create_body(BodyType::Static, shape, position, 0., None, None);
        self.update_center_of_mass_position(index);
        self.update_world_bounds(index);
        let b = &mut self.bodies[index];
        b.copy.position = b.position;
        b.copy.orientation = b.orientation;
        index
    }
    pub fn create_dynamic_body(
        &mut self,
        shape: usize,
        position: Vec3,
        mass: f64,
        restitution: f64,
        friction: f64,
    ) -> usize {
        let index = self.create_body(
            BodyType::Dynamic,
            shape,
            position,
            mass,
            Some(restitution),
            Some(friction),
        );
        self.dynamic_bodies.push(index);
        index
    }

    /// `updateBody( body )`.
    fn update_body(&mut self, index: usize) {
        self.update_mass_properties(index);
        self.update_center_of_mass_position(index);
        self.update_world_bounds(index);
        // markBodyAsDirty
        if self.bodies[index].kind == BodyType::Static {
            self.static_tree.dirty.add(index);
        } else {
            self.dynamic_tree.dirty.add(index);
        }
    }
    fn update_mass_properties(&mut self, index: usize) {
        let volume = self.shapes[self.bodies[index].shape].volume;
        let b = &mut self.bodies[index];
        if b.kind == BodyType::Static {
            b.density = f64::INFINITY;
            b.mass = f64::INFINITY;
            b.inverse_mass = 0.;
            b.local_inverse_inertia.zero();
            b.world_inverse_inertia.zero();
            return;
        }
        if b.mass == 0. && b.density == 0. {
            b.density = 1e3;
            b.mass = volume * b.density;
        } else if b.density == 0. {
            b.density = b.mass / volume;
        } else {
            b.mass = volume * b.density;
        }
        b.inverse_mass = 1. / b.mass;
        let shape = self.shapes[b.shape].clone();
        b.local_inverse_inertia = shape.inverse_inertia(b.mass);
        let local = b.local_inverse_inertia;
        transform_tensor(&mut b.world_inverse_inertia, &local, b.orientation);
    }
    fn update_center_of_mass_position(&mut self, index: usize) {
        let com = self.shapes[self.bodies[index].shape].center_of_mass;
        let b = &mut self.bodies[index];
        let mut local = com;
        local.transform_by_quat(b.orientation);
        let p = b.position;
        b.center_of_mass.add_vectors(p, local);
    }
    fn update_world_bounds(&mut self, index: usize) {
        let b = &self.bodies[index];
        let bounds = self.shapes[b.shape].world_bounds(b.center_of_mass, b.orientation);
        self.bodies[index].bounds = bounds;
    }

    /// The page's respawn: `body.position.set( p )`, zero velocities and
    /// `commitChanges()`.
    pub fn respawn(&mut self, index: usize, position: Vec3) {
        let b = &mut self.bodies[index];
        b.position = position;
        b.linear_velocity = Vec3::ZERO;
        b.angular_velocity = Vec3::ZERO;
        // hasChanged
        let changed = b.position.not_equals(b.copy.position)
            || b.orientation.not_equals(b.copy.orientation)
            || b.mass != b.copy.mass
            || b.density != b.copy.density;
        if changed {
            self.update_body(index);
        }
    }
    /// `body.applyLinearImpulse( impulse )`.
    pub fn apply_linear_impulse(&mut self, index: usize, impulse: Vec3) {
        let b = &mut self.bodies[index];
        if b.kind != BodyType::Dynamic {
            return;
        }
        let mut delta = Vec3::ZERO;
        delta.scale_vector(impulse, b.inverse_mass);
        let v = b.linear_velocity;
        b.linear_velocity.add_vectors(v, delta);
        clamp_speed(
            &mut b.linear_velocity,
            MAX_LINEAR_SPEED,
            MAX_LINEAR_SPEED * MAX_LINEAR_SPEED,
        );
        if b.is_sleeping {
            b.is_sleeping = false;
            b.time_without_moving = 0.;
        }
    }

    /// `advanceTime( timeStepSizeSeconds, timeToSimulateSeconds )`. A zero
    /// time to simulate after the first call reads the wall clock in the
    /// original; the port simulates nothing then.
    /// advanceTime( step, time ): a falsy time simulates the time since the
    /// first call on `now` ( performance.now() / 1000 ), or one step while that
    /// first call's time is falsy too.
    pub fn advance_time(&mut self, step: f64, time: f64, now: f64) {
        let falsy = |v: f64| v == 0. || v.is_nan();
        let mut time = time;
        if falsy(time) {
            time = match self.last_advance {
                Some(last) if !falsy(last) => now - last,
                _ => step,
            };
        }
        time = min(time, MAX_TIME_TO_SIMULATE);
        self.accumulated += time;
        while self.accumulated >= step {
            self.take_one_step(step);
            self.accumulated -= step;
        }
        self.last_advance = self.last_advance.or(Some(now));
    }

    fn take_one_step(&mut self, dt: f64) {
        // applyAccelerationIntegration
        let gravity = self.options.gravity;
        for &i in &self.dynamic_bodies {
            let b = &mut self.bodies[i];
            if b.is_sleeping {
                continue;
            }
            acceleration_integration(
                b,
                dt,
                gravity,
                self.options.linear_damping,
                self.options.angular_damping,
            );
        }
        // collideBodiesWithBroadphase
        self.current_cache ^= 1;
        self.caches[self.current_cache].clear();
        self.manifold_constraints.destroy_all();
        for (a, b) in self.iterate_pairs() {
            self.collide_bodies(a, b, dt);
        }
        // solveVelocityConstraintsInterleaved ( warm starting is enabled )
        for c in self.manifold_constraints.live() {
            self.warm_start(c);
        }
        for _ in 0..self.options.solve_velocity_iterations {
            for c in self.manifold_constraints.live() {
                self.solve_velocity_constraint(c);
            }
        }
        self.cache_lambdas();
        // applyVelocityIntegration
        for &i in &self.dynamic_bodies.clone() {
            if self.bodies[i].is_sleeping {
                continue;
            }
            let b = &mut self.bodies[i];
            b.copy.center_of_mass = b.center_of_mass;
            let v = b.linear_velocity;
            b.center_of_mass.add_scaled(v, dt);
            let mut rotation = Vec3::ZERO;
            rotation.scale_vector(b.angular_velocity, dt);
            let angle = rotation.length();
            if angle > 1e-6 {
                let mut axis = Vec3::ZERO;
                axis.scale_vector(rotation, 1. / angle);
                let mut q = Quat::default();
                q.set_axis_angle(axis, angle);
                b.copy.orientation = b.orientation;
                let o = b.orientation;
                b.orientation.multiply_quats(q, o);
                b.orientation.normalize();
            }
            self.dynamic_tree.dirty.add(i);
        }
        // solvePositionConstraintsInterleaved
        for _ in 0..self.options.solve_position_iterations {
            for c in self.manifold_constraints.live() {
                self.solve_position_constraint(c);
            }
        }
        // updateWorldInverseInertias
        for &i in &self.dynamic_bodies {
            let b = &mut self.bodies[i];
            let local = b.local_inverse_inertia;
            transform_tensor(&mut b.world_inverse_inertia, &local, b.orientation);
        }
        self.update_body_sleep_states(dt);
        // updateCachedBodyData over the whole body pool
        for i in self.bodies.live() {
            let b = &self.bodies[i];
            if b.center_of_mass.not_equals(b.copy.center_of_mass)
                || b.orientation.not_equals(b.copy.orientation)
            {
                let mut shape_com = self.shapes[b.shape].center_of_mass;
                shape_com.transform_by_quat(b.orientation);
                let b = &mut self.bodies[i];
                b.copy.position = b.position;
                let com = b.center_of_mass;
                b.position.subtract_vectors(com, shape_com);
                self.update_world_bounds(i);
            }
        }
        for &i in &self.dynamic_bodies {
            let b = &mut self.bodies[i];
            b.linear_forces = Vec3::ZERO;
            b.angular_forces = Vec3::ZERO;
        }
        self.step += 1;
        self.previous_time_step = dt;
    }

    // ---- broadphase ----

    fn tree(&mut self, dynamic: bool) -> &mut BvhTree {
        if dynamic {
            &mut self.dynamic_tree
        } else {
            &mut self.static_tree
        }
    }
    fn body_node(&self, dynamic: bool, body: usize) -> Option<usize> {
        let t = if dynamic {
            &self.dynamic_tree
        } else {
            &self.static_tree
        };
        t.body_to_node.get(body).copied().flatten()
    }
    fn set_body_node(&mut self, dynamic: bool, body: usize, node: Option<usize>) {
        let t = self.tree(dynamic);
        if t.body_to_node.len() <= body {
            t.body_to_node.resize(body + 1, None);
        }
        t.body_to_node[body] = node;
    }
    fn create_bvh_node(&mut self) -> usize {
        self.bvh_nodes.create(
            |n| {
                n.parent = None;
                n.left = None;
                n.right = None;
                n.bounds = Aabb::default();
                n.height = 0.;
            },
            BvhNode::default,
        )
    }
    fn rebalance(&mut self, dynamic: bool, node: usize) -> usize {
        let n = &self.bvh_nodes[node];
        if n.is_leaf() || n.height < 2. {
            return node;
        }
        let left = n.left.expect("an inner node");
        let right = n.right.expect("an inner node");
        let balance = self.bvh_nodes[right].height - self.bvh_nodes[left].height;
        // `pivot` rises above `node`; `other` is node's other child.
        let rotate = |w: &mut World, pivot: usize, other: usize, pivot_is_right: bool| -> usize {
            let pl = w.bvh_nodes[pivot].left.expect("a child");
            let pr = w.bvh_nodes[pivot].right.expect("a child");
            w.bvh_nodes[pivot].left = Some(node);
            let parent = w.bvh_nodes[node].parent;
            w.bvh_nodes[pivot].parent = parent;
            w.bvh_nodes[node].parent = Some(pivot);
            match parent {
                Some(p) => {
                    if w.bvh_nodes[p].left == Some(node) {
                        w.bvh_nodes[p].left = Some(pivot);
                    } else {
                        w.bvh_nodes[p].right = Some(pivot);
                    }
                }
                None => w.tree(dynamic).root = Some(pivot),
            }
            let (keep, give) = if w.bvh_nodes[pl].height > w.bvh_nodes[pr].height {
                (pl, pr)
            } else {
                (pr, pl)
            };
            w.bvh_nodes[pivot].right = Some(keep);
            if pivot_is_right {
                w.bvh_nodes[node].right = Some(give);
            } else {
                w.bvh_nodes[node].left = Some(give);
            }
            w.bvh_nodes[give].parent = Some(node);
            let (ob, gb) = (w.bvh_nodes[other].bounds, w.bvh_nodes[give].bounds);
            w.bvh_nodes[node].bounds.union_aabbs(&ob, &gb);
            let (nb, kb) = (w.bvh_nodes[node].bounds, w.bvh_nodes[keep].bounds);
            w.bvh_nodes[pivot].bounds.union_aabbs(&nb, &kb);
            w.bvh_nodes[node].height =
                max(w.bvh_nodes[other].height, w.bvh_nodes[give].height) + 1.;
            w.bvh_nodes[pivot].height =
                max(w.bvh_nodes[node].height, w.bvh_nodes[keep].height) + 1.;
            pivot
        };
        if balance > 1. {
            return rotate(self, right, left, true);
        }
        if balance < -1. {
            return rotate(self, left, right, false);
        }
        node
    }
    fn refit(&mut self, dynamic: bool, node: Option<usize>) -> bool {
        let mut did = false;
        let mut current = node;
        while let Some(n) = current {
            did = true;
            if self.bvh_nodes[n].is_leaf() {
                let objects = self.bvh_nodes[n].objects.clone();
                let node = &mut self.bvh_nodes[n];
                node.bounds.min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                node.bounds.max =
                    Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for o in objects {
                    let e = self.bodies[o].expanded_bounds;
                    self.bvh_nodes[n].bounds.union_aabb(&e);
                }
                self.bvh_nodes[n].height = 0.;
            } else {
                let left = self.bvh_nodes[n].left.expect("a child");
                let right = self.bvh_nodes[n].right.expect("a child");
                let (lb, rb) = (self.bvh_nodes[left].bounds, self.bvh_nodes[right].bounds);
                self.bvh_nodes[n].bounds.union_aabbs(&lb, &rb);
                self.bvh_nodes[n].height =
                    max(self.bvh_nodes[left].height, self.bvh_nodes[right].height) + 1.;
                if self.bvh_nodes[n].height > 2. {
                    self.rebalance(dynamic, n);
                }
            }
            current = self.bvh_nodes[n].parent;
        }
        did
    }
    fn split_objects(&mut self, dynamic: bool, left: usize, right: usize, node: usize) {
        let objects = self.bvh_nodes[node].objects.clone();
        let mut centroids = Aabb {
            min: Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            ..Default::default()
        };
        for &o in &objects {
            centroids.expand_to_point(self.bodies[o].bounds.centroid);
        }
        let axis = centroids.largest_axis();
        let mut sorted = objects;
        let key = |w: &World, o: usize| w.bodies[o].bounds.centroid.component(axis);
        // Array.prototype.sort is stable, as sort_by is.
        sorted.sort_by(|&a, &b| {
            let d = key(self, a) - key(self, b);
            if d < 0. {
                std::cmp::Ordering::Less
            } else if d > 0. {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let mut mid = sorted.len() / 2;
        if mid == 0 || mid >= sorted.len() {
            mid = 1;
        }
        for &o in &sorted[..mid] {
            self.bvh_nodes[left].objects.push(o);
            self.set_body_node(dynamic, o, Some(left));
        }
        for &o in &sorted[mid..] {
            self.bvh_nodes[right].objects.push(o);
            self.set_body_node(dynamic, o, Some(right));
        }
    }
    fn insert(&mut self, dynamic: bool, body: usize) -> bool {
        let margin = self.tree(dynamic).expansion_margin;
        let bounds = self.bodies[body].bounds;
        self.bodies[body]
            .expanded_bounds
            .expand_aabb(&bounds, margin);
        let Some(root) = self.tree(dynamic).root else {
            let root = self.create_bvh_node();
            self.bvh_nodes[root].bounds = self.bodies[body].expanded_bounds;
            self.bvh_nodes[root].objects.push(body);
            self.bvh_nodes[root].height = 0.;
            self.tree(dynamic).root = Some(root);
            self.set_body_node(dynamic, body, Some(root));
            return true;
        };
        let mut best = root;
        while !self.bvh_nodes[best].is_leaf() {
            let left = self.bvh_nodes[best].left.expect("a child");
            let right = self.bvh_nodes[best].right.expect("a child");
            let mut l = Aabb::default();
            l.union_aabbs(&self.bvh_nodes[best].bounds, &self.bvh_nodes[left].bounds);
            let mut r = Aabb::default();
            r.union_aabbs(&self.bvh_nodes[best].bounds, &self.bvh_nodes[right].bounds);
            best = if l.surface_area() < r.surface_area() {
                left
            } else {
                right
            };
        }
        let max_objects = self.tree(dynamic).max_objects_per_leaf;
        let max_depth = self.tree(dynamic).max_depth;
        if self.bvh_nodes[best].objects.len() < max_objects
            || self.bvh_nodes[best].height >= max_depth
        {
            self.bvh_nodes[best].objects.push(body);
            let e = self.bodies[body].expanded_bounds;
            self.bvh_nodes[best].bounds.union_aabb(&e);
            self.set_body_node(dynamic, body, Some(best));
            return self.refit(dynamic, Some(best));
        }
        let left = self.create_bvh_node();
        let right = self.create_bvh_node();
        self.bvh_nodes[best].objects.push(body);
        self.split_objects(dynamic, left, right, best);
        self.bvh_nodes[best].objects.clear();
        for child in [left, right] {
            let objects = self.bvh_nodes[child].objects.clone();
            self.bvh_nodes[child].bounds = self.bodies[objects[0]].expanded_bounds;
            for &o in &objects[1..] {
                let e = self.bodies[o].expanded_bounds;
                self.bvh_nodes[child].bounds.union_aabb(&e);
            }
            self.bvh_nodes[child].parent = Some(best);
            self.bvh_nodes[child].height = 0.;
        }
        self.bvh_nodes[best].left = Some(left);
        self.bvh_nodes[best].right = Some(right);
        let (lb, rb) = (self.bvh_nodes[left].bounds, self.bvh_nodes[right].bounds);
        self.bvh_nodes[best].bounds.union_aabbs(&lb, &rb);
        self.bvh_nodes[best].height = 1.;
        self.refit(dynamic, Some(best))
    }
    fn remove(&mut self, dynamic: bool, body: usize) {
        let Some(node) = self.body_node(dynamic, body) else {
            return;
        };
        // ReferenceList.remove: the last entry fills the gap.
        let objects = &mut self.bvh_nodes[node].objects;
        let Some(i) = objects.iter().position(|&o| o == body) else {
            return;
        };
        objects.swap_remove(i);
        self.set_body_node(dynamic, body, None);
        if !self.bvh_nodes[node].objects.is_empty() {
            self.refit(dynamic, Some(node));
            return;
        }
        let Some(parent) = self.bvh_nodes[node].parent else {
            self.bvh_nodes.destroy(node);
            self.tree(dynamic).root = None;
            return;
        };
        let sibling = if self.bvh_nodes[parent].left == Some(node) {
            self.bvh_nodes[parent].right
        } else {
            self.bvh_nodes[parent].left
        }
        .expect("a sibling");
        let grandparent = self.bvh_nodes[parent].parent;
        self.bvh_nodes[sibling].parent = grandparent;
        match grandparent {
            None => self.tree(dynamic).root = Some(sibling),
            Some(g) => {
                if self.bvh_nodes[g].left == Some(parent) {
                    self.bvh_nodes[g].left = Some(sibling);
                } else {
                    self.bvh_nodes[g].right = Some(sibling);
                }
            }
        }
        self.bvh_nodes.destroy(parent);
        self.bvh_nodes.destroy(node);
        self.refit(dynamic, Some(sibling));
    }
    fn update(&mut self, dynamic: bool, body: usize) {
        let Some(node) = self.body_node(dynamic, body) else {
            self.insert(dynamic, body);
            return;
        };
        if self.bvh_nodes[node]
            .bounds
            .encloses_aabb(&self.bodies[body].bounds)
        {
            return;
        }
        self.remove(dynamic, body);
        self.insert(dynamic, body);
    }
    /// `intersectBody( _createPairCallback, body, false )`.
    fn intersect_body(&mut self, dynamic: bool, body: usize) {
        let Some(root) = self.tree(dynamic).root else {
            return;
        };
        let mut stack = vec![Some(root)];
        while let Some(entry) = stack.pop() {
            let Some(node) = entry else { continue };
            if !self.bvh_nodes[node]
                .bounds
                .intersects_aabb(&self.bodies[body].bounds)
            {
                continue;
            }
            if !self.bvh_nodes[node].is_leaf() {
                stack.push(self.bvh_nodes[node].left);
                stack.push(self.bvh_nodes[node].right);
                continue;
            }
            for other in self.bvh_nodes[node].objects.clone() {
                if body == other
                    || !self.bodies[body]
                        .bounds
                        .intersects_aabb(&self.bodies[other].bounds)
                {
                    continue;
                }
                self.create_pair(body, other);
            }
        }
    }
    fn create_pair(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (a, b);
        if self.bodies[a].kind == self.bodies[b].kind && a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if self.bodies[a].kind != self.bodies[b].kind && self.bodies[a].kind != BodyType::Dynamic {
            std::mem::swap(&mut a, &mut b);
        }
        let node = self.pair_nodes.create(|_| {}, PairNode::default);
        let edge_a = self.pair_edges.create(|_| {}, PairEdge::default);
        let edge_b = self.pair_edges.create(|_| {}, PairEdge::default);
        self.pair_nodes[node] = PairNode {
            body_a: a,
            body_b: b,
            edge_a,
            edge_b,
        };
        for (edge, body) in [(edge_a, a), (edge_b, b)] {
            let first = self.bodies[body].first_pair_edge;
            self.pair_edges[edge] = PairEdge {
                node,
                next: first,
                prev: None,
            };
            if let Some(f) = first {
                self.pair_edges[f].prev = Some(edge);
            }
            self.bodies[body].first_pair_edge = Some(edge);
        }
    }
    fn destroy_all_pairs_of_one(&mut self, body: usize) {
        let mut edge = self.bodies[body].first_pair_edge;
        while let Some(e) = edge {
            let next = self.pair_edges[e].next;
            let node = self.pair_edges[e].node;
            let PairNode {
                body_a,
                body_b,
                edge_a,
                edge_b,
            } = self.pair_nodes[node].clone();
            let (other, other_edge) = if body_a == body {
                (body_b, edge_b)
            } else {
                (body_a, edge_a)
            };
            let (prev, next_other) = (
                self.pair_edges[other_edge].prev,
                self.pair_edges[other_edge].next,
            );
            match prev {
                Some(p) => self.pair_edges[p].next = next_other,
                None => self.bodies[other].first_pair_edge = next_other,
            }
            if let Some(n) = next_other {
                self.pair_edges[n].prev = prev;
            }
            self.pair_edges.destroy(edge_a);
            self.pair_edges.destroy(edge_b);
            self.pair_nodes.destroy(node);
            edge = next;
        }
        self.bodies[body].first_pair_edge = None;
    }
    fn pairs_of_one(&self, body: usize) -> Vec<usize> {
        let mut out = vec![];
        let mut edge = self.bodies[body].first_pair_edge;
        while let Some(e) = edge {
            out.push(self.pair_edges[e].node);
            edge = self.pair_edges[e].next;
        }
        out
    }
    /// `updateDirtyBodies()`.
    fn update_dirty_bodies(&mut self) {
        for body in self.dynamic_tree.dirty.items() {
            self.update(true, body);
        }
        for body in self.static_tree.dirty.items() {
            for pair in self.pairs_of_one(body) {
                let PairNode { body_a, body_b, .. } = self.pair_nodes[pair];
                let other = if body_a == body { body_b } else { body_a };
                self.dynamic_tree.dirty.add(other);
            }
            self.destroy_all_pairs_of_one(body);
            self.update(false, body);
        }
        self.static_tree.dirty.clear();
        for body in self.dynamic_tree.dirty.items() {
            self.destroy_all_pairs_of_one(body);
            self.intersect_body(true, body);
            self.intersect_body(false, body);
        }
        self.dynamic_tree.dirty.clear();
    }
    /// `broadphase.iteratePairs()`: the pair node pool in order.
    fn iterate_pairs(&mut self) -> Vec<(usize, usize)> {
        self.update_dirty_bodies();
        let mut out = vec![];
        for node in self.pair_nodes.live() {
            let PairNode { body_a, body_b, .. } = self.pair_nodes[node];
            let (a, b) = (&self.bodies[body_a], &self.bodies[body_b]);
            if a.kind != BodyType::Dynamic && b.kind != BodyType::Dynamic
                || b.kind == BodyType::Static && a.is_sleeping
            {
                continue;
            }
            out.push((body_a, body_b));
        }
        out
    }

    // ---- narrowphase ----

    /// `collideBodies( bodyA, bodyB, ... )`.
    fn collide_bodies(&mut self, a: usize, b: usize, dt: f64) {
        let (ba, bb) = (&self.bodies[a], &self.bodies[b]);
        let offset = ba.center_of_mass;
        let mut negated = Vec3::ZERO;
        negated.negate_vector(offset);
        let mut iso_a = Mat4::default();
        iso_a.from_quat(ba.orientation);
        let mut iso_b = isometry(bb.orientation, bb.center_of_mass);
        iso_b.post_translated(negated);
        // collector.init
        self.collector_manifolds.destroy_all();
        let mut hits = 0;
        let (shape_a, shape_b) = (self.shapes[ba.shape].clone(), self.shapes[bb.shape].clone());
        let hit = match (shape_a.kind, shape_b.kind) {
            (ShapeKind::Sphere { radius: ra }, ShapeKind::Sphere { radius: rb }) => {
                collide_sphere_vs_sphere(ra, rb, &iso_a, &iso_b)
            }
            (ka, kb) => {
                let (hit, epa) = gjk::collide_convex_vs_convex(
                    ka,
                    kb,
                    &iso_a,
                    &iso_b,
                    SPECULATIVE_CONTACT_DISTANCE,
                    COLLISION_TOLERANCE,
                    PENETRATION_TOLERANCE,
                );
                if epa {
                    self.epa_steps += 1;
                }
                hit.map(
                    |(penetration, contact_point_a, contact_point_b, normal_a)| Hit {
                        penetration,
                        contact_point_a,
                        contact_point_b,
                        normal_a,
                    },
                )
            }
        };
        if let Some(hit) = hit {
            hits += 1;
            self.add_hit(a, b, hit);
        }
        if hits < 1 {
            return;
        }
        let pair = self.add_contact_pair(a, b);
        let (ba, bb) = (&self.bodies[a], &self.bodies[b]);
        let mut inverse = Quat::default();
        inverse.conjugate_quat(ba.orientation);
        let mut t = Vec3::ZERO;
        t.subtract_vectors(bb.center_of_mass, ba.center_of_mass);
        let t0 = t;
        t.transform_vector_by_quat(t0, inverse);
        self.caches[self.current_cache].pairs[pair].translation_ab = t;
        for m in self.collector_manifolds.live() {
            self.collector_manifolds[m].world_space_normal.normalize();
            // A single sphere contact never needs pruning.
            self.add_contact_constraints(a, b, m, dt);
        }
    }

    /// `CollideBodiesCollector.addHit( result )`.
    fn add_hit(&mut self, a: usize, b: usize, hit: Hit) {
        let mut normal = Vec3::ZERO;
        normal.normalize_vector(hit.normal_a);
        let max_delta_cos_angle = cos(degrees_to_radians(5.));
        let mut target = None;
        for m in self.collector_manifolds.live() {
            if normal.dot(self.collector_manifolds[m].first_world_space_normal)
                >= max_delta_cos_angle
            {
                let manifold = &mut self.collector_manifolds[m];
                manifold.world_space_normal.add_vector(normal);
                manifold.penetration_depth = max(manifold.penetration_depth, hit.penetration);
                target = Some(m);
                break;
            }
        }
        let m = match target {
            Some(m) => m,
            None => self.collector_manifolds.create(
                |manifold| {
                    reset_manifold(manifold, a, b, normal, normal, hit.penetration);
                },
                || {
                    let mut manifold = Manifold {
                        points_a: vec![Vec3::ZERO; 64],
                        points_b: vec![Vec3::ZERO; 64],
                        lambdas: vec![Vec3::ZERO; 64],
                        ..Default::default()
                    };
                    reset_manifold(&mut manifold, a, b, normal, normal, hit.penetration);
                    manifold
                },
            ),
        };
        // The sphere's supporting face is empty, so manifoldBetweenTwoFaces
        // keeps the contact points ( relative to body A's center of mass ).
        let com = self.bodies[a].center_of_mass;
        let mut pa = hit.contact_point_a;
        pa.add_vector(com);
        let mut pb = hit.contact_point_b;
        pb.add_vector(com);
        let manifold = &mut self.collector_manifolds[m];
        let n = manifold.num_contacts;
        manifold.points_a[n] = pa;
        manifold.points_b[n] = pb;
        manifold.num_contacts += 1;
    }

    fn add_contact_pair(&mut self, a: usize, b: usize) -> usize {
        let key = contact_pair_key(&self.bodies[a], a, &self.bodies[b], b);
        let cache = &mut self.caches[self.current_cache];
        let pair = cache
            .pairs
            .create(|p| *p = ContactPair::default(), ContactPair::default);
        cache.pair_map.insert(key, pair);
        let (ba, bb) = (&self.bodies[a], &self.bodies[b]);
        let mut inverse = Quat::default();
        inverse.conjugate_quat(ba.orientation);
        let p = &mut cache.pairs[pair];
        p.translation_ab
            .subtract_vectors(bb.center_of_mass, ba.center_of_mass);
        p.translation_ab.transform_by_quat(inverse);
        p.rotation_ab.multiply_quats(inverse, bb.orientation);
        pair
    }

    /// `addContactConstraints( bodyA, bodyB, manifold, ... )`.
    fn add_contact_constraints(&mut self, a: usize, b: usize, m: usize, dt: f64) {
        self.collector_manifolds[m].world_space_normal.normalize();
        let source = self.collector_manifolds[m].clone();
        let key = contact_pair_key(&self.bodies[a], a, &self.bodies[b], b);
        let current = self.current_cache;
        let cached = self.caches[current].manifolds.create(
            |c| reset_cached_manifold(c, &source),
            || {
                let mut c = Manifold {
                    points_a: vec![Vec3::ZERO; 4],
                    points_b: vec![Vec3::ZERO; 4],
                    lambdas: vec![Vec3::ZERO; 4],
                    ..Default::default()
                };
                reset_cached_manifold(&mut c, &source);
                c
            },
        );
        self.caches[current]
            .manifold_map
            .insert(key.clone(), cached);
        let bb = &self.bodies[b];
        // Isometry.fromInverseRotationAndTranslation: the general inverse.
        let forward = isometry(bb.orientation, bb.center_of_mass);
        let mut inverse_b = forward;
        inverse_b.invert_matrix(&forward);
        let mut normal = inverse_b.multiply_3x3(source.world_space_normal);
        normal.normalize();
        self.caches[current].manifolds[cached].world_space_normal = normal;
        let previous = self.caches[1 - current].manifold_map.get(&key).copied();
        if let Some(p) = previous {
            let lambdas = self.caches[1 - current].manifolds[p].lambdas.clone();
            let c = &mut self.caches[current].manifolds[cached];
            let n = c.num_contacts;
            c.lambdas[..n].copy_from_slice(&lambdas[..n]);
        }
        if self.bodies[a].kind != BodyType::Dynamic && self.bodies[b].kind != BodyType::Dynamic {
            return;
        }
        let (fa, fb) = (self.bodies[a].friction, self.bodies[b].friction);
        let (ra, rb) = (self.bodies[a].restitution, self.bodies[b].restitution);
        let (ima, imb) = (self.bodies[a].inverse_mass, self.bodies[b].inverse_mass);
        let mc = self.manifold_constraints.create(
            |c| {
                reset_manifold_constraint(c, a, b, fa, fb, ra, rb, ima, imb);
            },
            || {
                let mut c = ManifoldConstraint {
                    contacts: vec![ContactConstraint::default(); 4],
                    ..Default::default()
                };
                reset_manifold_constraint(&mut c, a, b, fa, fb, ra, rb, ima, imb);
                c
            },
        );
        {
            let c = &mut self.manifold_constraints[mc];
            c.world_space_normal
                .normalize_vector(source.world_space_normal);
            c.update_tangent_directions();
            c.num_contacts = source.num_contacts;
        }
        let inertia_a = self.bodies[a].compute_inverse_inertia_tensor();
        let inertia_b = self.bodies[b].compute_inverse_inertia_tensor();
        let mut world_to_local_a = Mat4::default();
        world_to_local_a.from_inverse_rotation_and_translation(
            self.bodies[a].orientation,
            self.bodies[a].center_of_mass,
        );
        let mut world_to_local_b = Mat4::default();
        world_to_local_b.from_inverse_rotation_and_translation(
            self.bodies[b].orientation,
            self.bodies[b].center_of_mass,
        );
        for i in 0..source.num_contacts {
            self.add_contact_constraint(
                a,
                b,
                &source,
                mc,
                &inertia_a,
                &inertia_b,
                &world_to_local_a,
                &world_to_local_b,
                cached,
                i,
                dt,
                &key,
            );
        }
        let cache = &mut self.caches[current];
        let pair = *cache.pair_map.get(&key).expect("contact pair not found");
        if let Some(first) = cache.pairs[pair].first_manifold {
            cache.manifolds[cached].next = Some(first);
        }
        cache.pairs[pair].first_manifold = Some(cached);
        let c = &mut cache.manifolds[cached];
        for i in 0..c.num_contacts {
            c.lambdas[i] = Vec3::ZERO;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_contact_constraint(
        &mut self,
        a: usize,
        b: usize,
        manifold: &Manifold,
        mc: usize,
        inertia_a: &Mat3,
        inertia_b: &Mat3,
        world_to_local_a: &Mat4,
        world_to_local_b: &Mat4,
        cached: usize,
        index: usize,
        dt: f64,
        key: &str,
    ) {
        let mut world_a = manifold.points_a[index];
        world_a.add_vector(manifold.base_translation);
        let mut world_b = manifold.points_b[index];
        world_b.add_vector(manifold.base_translation);
        let mut local_a = Vec3::ZERO;
        local_a.transform_vector_from_mat4(world_a, world_to_local_a);
        let mut local_b = Vec3::ZERO;
        local_b.transform_vector_from_mat4(world_b, world_to_local_b);
        let warm = {
            let previous = self.caches[1 - self.current_cache]
                .manifold_map
                .get(key)
                .copied();
            // The original compares the old manifold's point at `index` for each
            // of its points.
            previous.and_then(|p| {
                let old = &self.caches[1 - self.current_cache].manifolds[p];
                (0..old.num_contacts)
                    .any(|_| {
                        local_a.is_close(old.points_a[index], squared(0.01))
                            && local_b.is_close(old.points_b[index], squared(0.01))
                    })
                    .then(|| old.lambdas[index])
            })
        };
        {
            let c = &mut self.manifold_constraints[mc].contacts[index];
            c.local_position_a = local_a;
            c.local_position_b = local_b;
            c.normal.total_lambda = 0.;
            c.tangent.total_lambda = 0.;
            c.bitangent.total_lambda = 0.;
            if let Some(l) = warm {
                c.normal.total_lambda = l.x;
                c.tangent.total_lambda = l.y;
                c.bitangent.total_lambda = l.z;
            }
        }
        {
            let cm = &mut self.caches[self.current_cache].manifolds[cached];
            cm.points_a[index] = local_a;
            cm.points_b[index] = local_b;
        }
        let mut combined = Vec3::ZERO;
        combined.average_of_vectors(world_a, world_b);
        let mut arm_a = Vec3::ZERO;
        arm_a.subtract_vectors(combined, self.bodies[a].center_of_mass);
        let mut arm_b = Vec3::ZERO;
        arm_b.subtract_vectors(combined, self.bodies[b].center_of_mass);
        let va = self.bodies[a].velocity_of_point(arm_a);
        let vb = self.bodies[b].velocity_of_point(arm_b);
        let mut vab = Vec3::ZERO;
        vab.subtract_vectors(vb, va);
        let (normal, tangent, bitangent, restitution, friction, ima, imb) = {
            let c = &self.manifold_constraints[mc];
            (
                c.world_space_normal,
                c.world_space_tangent,
                c.world_space_bitangent,
                c.restitution,
                c.friction,
                c.inverse_mass_a,
                c.inverse_mass_b,
            )
        };
        let normal_speed = vab.dot(normal);
        let mut penetration_vector = Vec3::ZERO;
        penetration_vector.subtract_vectors(world_a, world_b);
        let penetration = penetration_vector.dot(normal);
        let speculative = max(0., -penetration / dt);
        let normal_bias = if restitution > 0.
            && normal_speed < -MIN_VELOCITY_FOR_ELASTIC_CONTACT
            && normal_speed < -speculative
        {
            restitution * normal_speed
        } else {
            speculative
        };
        let (ka, kb) = (self.bodies[a].kind, self.bodies[b].kind);
        let c = &mut self.manifold_constraints[mc].contacts[index];
        initialize(
            &mut c.normal,
            ka,
            kb,
            ima,
            imb,
            inertia_a,
            inertia_b,
            arm_a,
            arm_b,
            normal,
            normal_bias,
        );
        if friction == 0. {
            c.tangent.effective_mass = 0.;
            c.tangent.total_lambda = 0.;
            c.bitangent.effective_mass = 0.;
            c.bitangent.total_lambda = 0.;
            return;
        }
        let surface = Vec3::ZERO;
        let tangent_speed = surface.dot(tangent);
        let bitangent_speed = surface.dot(bitangent);
        initialize(
            &mut c.tangent,
            ka,
            kb,
            ima,
            imb,
            inertia_a,
            inertia_b,
            arm_a,
            arm_b,
            tangent,
            tangent_speed,
        );
        initialize(
            &mut c.bitangent,
            ka,
            kb,
            ima,
            imb,
            inertia_a,
            inertia_b,
            arm_a,
            arm_b,
            bitangent,
            bitangent_speed,
        );
    }

    // ---- solver ----

    fn warm_start(&mut self, c: usize) {
        self.manifold_constraints[c].update_tangent_directions();
        let mc = self.manifold_constraints[c].clone();
        for i in 0..mc.num_contacts {
            let cc = mc.contacts[i];
            if is_active(&cc.tangent) || is_active(&cc.bitangent) {
                self.warm_start_directional(&mc, &cc.tangent, mc.world_space_tangent);
                self.warm_start_directional(&mc, &cc.bitangent, mc.world_space_bitangent);
            }
            self.warm_start_directional(&mc, &cc.normal, mc.world_space_normal);
        }
    }
    fn warm_start_directional(
        &mut self,
        mc: &ManifoldConstraint,
        d: &Directional,
        direction: Vec3,
    ) {
        let delta = d.total_lambda;
        if self.bodies[mc.body_a].kind == BodyType::Dynamic {
            let body = &mut self.bodies[mc.body_a];
            body.linear_velocity
                .add_scaled(direction, -delta * mc.inverse_mass_a);
            body.angular_velocity
                .add_scaled(d.inv_i1_r1_plus_u_x_axis, -delta);
        }
        if self.bodies[mc.body_b].kind == BodyType::Dynamic {
            let body = &mut self.bodies[mc.body_b];
            body.linear_velocity
                .add_scaled(direction, delta * mc.inverse_mass_b);
            body.angular_velocity.add_scaled(d.inv_i2_r2_x_axis, delta);
        }
    }
    fn total_lambda(&self, mc: &ManifoldConstraint, d: &Directional, direction: Vec3) -> f64 {
        let (ba, bb) = (&self.bodies[mc.body_a], &self.bodies[mc.body_b]);
        let mut jv = 0.;
        let mut vba = Vec3::ZERO;
        vba.subtract_vectors(ba.linear_velocity, bb.linear_velocity);
        jv += direction.dot(vba);
        if ba.kind != BodyType::Static {
            jv += d.r1_plus_u_x_axis.dot(ba.angular_velocity);
        }
        if bb.kind != BodyType::Static {
            jv -= d.r2_x_axis.dot(bb.angular_velocity);
        }
        let lambda = d.effective_mass * (jv - (d.softness * d.total_lambda + d.bias));
        d.total_lambda + lambda
    }
    fn apply_impulse(
        &mut self,
        mc: &ManifoldConstraint,
        d: &mut Directional,
        direction: Vec3,
        lambda: f64,
    ) {
        let delta = lambda - d.total_lambda;
        d.total_lambda = lambda;
        if delta == 0. {
            return;
        }
        if self.bodies[mc.body_a].kind == BodyType::Dynamic {
            let body = &mut self.bodies[mc.body_a];
            body.linear_velocity
                .add_scaled(direction, -delta * mc.inverse_mass_a);
            body.angular_velocity
                .add_scaled(d.inv_i1_r1_plus_u_x_axis, -delta);
        }
        if self.bodies[mc.body_b].kind == BodyType::Dynamic {
            let body = &mut self.bodies[mc.body_b];
            body.linear_velocity
                .add_scaled(direction, delta * mc.inverse_mass_b);
            body.angular_velocity.add_scaled(d.inv_i2_r2_x_axis, delta);
        }
    }
    fn solve_velocity_constraint(&mut self, c: usize) {
        // solveFrictionVelocityConstraints
        self.manifold_constraints[c].update_tangent_directions();
        let mc = self.manifold_constraints[c].clone();
        for i in 0..mc.num_contacts {
            let mut cc = self.manifold_constraints[c].contacts[i];
            if is_active(&cc.tangent) || is_active(&cc.bitangent) {
                let mut tangent = self.total_lambda(&mc, &cc.tangent, mc.world_space_tangent);
                let mut bitangent = self.total_lambda(&mc, &cc.bitangent, mc.world_space_bitangent);
                let total_squared = squared(tangent) + squared(bitangent);
                let max_lambda = mc.friction * cc.normal.total_lambda;
                if total_squared > squared(max_lambda) {
                    let scale = max_lambda / total_squared.sqrt();
                    tangent *= scale;
                    bitangent *= scale;
                }
                self.apply_impulse(&mc, &mut cc.tangent, mc.world_space_tangent, tangent);
                self.apply_impulse(&mc, &mut cc.bitangent, mc.world_space_bitangent, bitangent);
                self.manifold_constraints[c].contacts[i] = cc;
            }
        }
        // solveNonPenetrationVelocityConstraints
        for i in 0..mc.num_contacts {
            let mut cc = self.manifold_constraints[c].contacts[i];
            let total = clamp(
                self.total_lambda(&mc, &cc.normal, mc.world_space_normal),
                0.,
                f64::INFINITY,
            );
            self.apply_impulse(&mc, &mut cc.normal, mc.world_space_normal, total);
            self.manifold_constraints[c].contacts[i] = cc;
        }
    }
    fn solve_position_constraint(&mut self, c: usize) {
        let mc = self.manifold_constraints[c].clone();
        let to_world_a = isometry(
            self.bodies[mc.body_a].orientation,
            self.bodies[mc.body_a].center_of_mass,
        );
        let to_world_b = isometry(
            self.bodies[mc.body_b].orientation,
            self.bodies[mc.body_b].center_of_mass,
        );
        for i in 0..mc.num_contacts {
            let mut cc = self.manifold_constraints[c].contacts[i];
            let mut world_a = Vec3::ZERO;
            world_a.transform_vector_from_mat4(cc.local_position_a, &to_world_a);
            let mut world_b = Vec3::ZERO;
            world_b.transform_vector_from_mat4(cc.local_position_b, &to_world_b);
            let mut pv = Vec3::ZERO;
            pv.subtract_vectors(world_b, world_a);
            let separation = max(
                pv.dot(mc.world_space_normal) + PENETRATION_SLOP,
                -MAX_PENETRATION_DISTANCE,
            );
            if separation < 0. {
                // updateNonPenetrationConstraint
                let mut average = Vec3::ZERO;
                average.average_of_vectors(world_a, world_b);
                let mut arm_a = Vec3::ZERO;
                arm_a.subtract_vectors(average, self.bodies[mc.body_a].center_of_mass);
                let mut arm_b = Vec3::ZERO;
                arm_b.subtract_vectors(average, self.bodies[mc.body_b].center_of_mass);
                let inertia_a = self.bodies[mc.body_a].compute_inverse_inertia_tensor();
                let inertia_b = self.bodies[mc.body_b].compute_inverse_inertia_tensor();
                let (ka, kb) = (self.bodies[mc.body_a].kind, self.bodies[mc.body_b].kind);
                initialize(
                    &mut cc.normal,
                    ka,
                    kb,
                    mc.inverse_mass_a,
                    mc.inverse_mass_b,
                    &inertia_a,
                    &inertia_b,
                    arm_a,
                    arm_b,
                    mc.world_space_normal,
                    0.,
                );
                self.manifold_constraints[c].contacts[i] = cc;
                // solvePositionConstraintWithMassOverride
                if separation == 0. || cc.normal.softness != 0. {
                    continue;
                }
                let lambda = -cc.normal.effective_mass * BAUMGARTE * separation;
                if ka == BodyType::Dynamic {
                    let mut delta = Vec3::ZERO;
                    delta.scale_vector(mc.world_space_normal, -lambda * mc.inverse_mass_a);
                    self.dynamic_tree.dirty.add(mc.body_a);
                    let body = &mut self.bodies[mc.body_a];
                    let com = body.center_of_mass;
                    body.center_of_mass.add_vectors(com, delta);
                    let mut rotation = Vec3::ZERO;
                    rotation.scale_vector(cc.normal.inv_i1_r1_plus_u_x_axis, lambda);
                    body.rotation_delta(rotation, -1.);
                }
                if kb == BodyType::Dynamic {
                    let mut delta = Vec3::ZERO;
                    delta.scale_vector(mc.world_space_normal, lambda * mc.inverse_mass_b);
                    self.dynamic_tree.dirty.add(mc.body_b);
                    let body = &mut self.bodies[mc.body_b];
                    let com = body.center_of_mass;
                    body.center_of_mass.add_vectors(com, delta);
                    let mut rotation = Vec3::ZERO;
                    rotation.scale_vector(cc.normal.inv_i2_r2_x_axis, lambda);
                    body.rotation_delta(rotation, 1.);
                }
            }
        }
    }
    fn cache_lambdas(&mut self) {
        let current = self.current_cache;
        for c in self.manifold_constraints.live() {
            let mc = &self.manifold_constraints[c];
            let key = contact_pair_key(
                &self.bodies[mc.body_a],
                mc.body_a,
                &self.bodies[mc.body_b],
                mc.body_b,
            );
            let m = *self.caches[current]
                .manifold_map
                .get(&key)
                .unwrap_or_else(|| panic!("expected to find manifold {key} in cache"));
            let manifold = &mut self.caches[current].manifolds[m];
            for i in 0..manifold.num_contacts {
                let cc = &mc.contacts[i];
                manifold.lambdas[i] = Vec3::new(
                    cc.normal.total_lambda,
                    cc.tangent.total_lambda,
                    cc.bitangent.total_lambda,
                );
            }
        }
    }

    // ---- sleeping ----

    fn update_body_sleep_states(&mut self, dt: f64) {
        let velocity_threshold = SLEEP_VELOCITY_THRESHOLD * SLEEP_VELOCITY_THRESHOLD;
        let spin_threshold = SLEEP_SPIN_THRESHOLD * SLEEP_SPIN_THRESHOLD;
        for &i in &self.dynamic_bodies {
            let b = &mut self.bodies[i];
            let v = b.linear_velocity.squared_length();
            let s = b.angular_velocity.squared_length();
            if v < velocity_threshold && s < spin_threshold {
                b.time_without_moving += dt;
            } else {
                b.time_without_moving = 0.;
            }
        }
        // buildAndSleepIslands
        self.sleep_visit_generation += 1;
        let generation = self.sleep_visit_generation;
        for seed in self.dynamic_bodies.clone() {
            let s = &self.bodies[seed];
            if s.kind == BodyType::Static || s.sleep_visit_generation == generation || s.is_sleeping
            {
                continue;
            }
            let mut stack = vec![seed];
            let mut island = vec![];
            let mut all_ready = true;
            while let Some(body) = stack.pop() {
                if self.bodies[body].sleep_visit_generation == generation {
                    continue;
                }
                self.bodies[body].sleep_visit_generation = generation;
                island.push(body);
                if !self.bodies[body].is_ready_to_sleep(SLEEP_TIME_THRESHOLD) {
                    all_ready = false;
                }
                let mut e = self.bodies[body].first_pair_edge;
                while let Some(edge) = e {
                    let node = self.pair_edges[edge].node;
                    let PairNode { body_a, body_b, .. } = self.pair_nodes[node];
                    let other = if body_a == body { body_b } else { body_a };
                    let o = &self.bodies[other];
                    if o.kind != BodyType::Static && o.sleep_visit_generation != generation {
                        stack.push(other);
                    }
                    e = self.pair_edges[edge].next;
                }
            }
            for body in island {
                self.bodies[body].is_sleeping = all_ready;
            }
        }
    }

    pub fn body_count(&self) -> usize {
        self.bodies.length
    }
    pub fn body(&self, index: usize) -> &Body {
        &self.bodies[index]
    }
}

fn is_active(d: &Directional) -> bool {
    d.effective_mass != 0.
}

#[allow(clippy::too_many_arguments)]
fn initialize(
    d: &mut Directional,
    kind_a: BodyType,
    kind_b: BodyType,
    inverse_mass_a: f64,
    inverse_mass_b: f64,
    inertia_a: &Mat3,
    inertia_b: &Mat3,
    arm_a: Vec3,
    arm_b: Vec3,
    direction: Vec3,
    bias: f64,
) {
    // computeInverseEffectiveMass
    d.r1_plus_u_x_axis = Vec3::ZERO;
    d.r2_x_axis = Vec3::ZERO;
    if kind_a != BodyType::Static {
        d.r1_plus_u_x_axis.cross_vectors(arm_a, direction);
    }
    if kind_b != BodyType::Static {
        d.r2_x_axis.cross_vectors(arm_b, direction);
    }
    let mut inverse_effective_mass = 0.;
    if kind_a == BodyType::Dynamic {
        let r = d.r1_plus_u_x_axis;
        d.inv_i1_r1_plus_u_x_axis
            .transform_vector_from_mat3(r, inertia_a);
        inverse_effective_mass += inverse_mass_a + d.inv_i1_r1_plus_u_x_axis.dot(r);
    }
    if kind_b == BodyType::Dynamic {
        let r = d.r2_x_axis;
        d.inv_i2_r2_x_axis.transform_vector_from_mat3(r, inertia_b);
        inverse_effective_mass += inverse_mass_b + d.inv_i2_r2_x_axis.dot(r);
    }
    if inverse_effective_mass == 0. {
        d.effective_mass = 0.;
        d.total_lambda = 0.;
        return;
    }
    d.effective_mass = 1. / inverse_effective_mass;
    d.bias = bias;
    d.softness = 0.;
}

fn reset_manifold(
    m: &mut Manifold,
    a: usize,
    b: usize,
    first: Vec3,
    normal: Vec3,
    penetration: f64,
) {
    m.body_a = a;
    m.body_b = b;
    m.next = None;
    m.base_translation = Vec3::ZERO;
    m.first_world_space_normal = first;
    m.world_space_normal = normal;
    m.penetration_depth = penetration;
    m.num_contacts = 0;
}

fn reset_cached_manifold(c: &mut Manifold, source: &Manifold) {
    c.body_a = source.body_a;
    c.body_b = source.body_b;
    c.next = source.next;
    c.base_translation = Vec3::ZERO;
    c.first_world_space_normal = source.first_world_space_normal;
    c.world_space_normal = source.world_space_normal;
    c.penetration_depth = source.penetration_depth;
    c.num_contacts = source.num_contacts;
}

#[allow(clippy::too_many_arguments)]
fn reset_manifold_constraint(
    c: &mut ManifoldConstraint,
    a: usize,
    b: usize,
    fa: f64,
    fb: f64,
    ra: f64,
    rb: f64,
    ima: f64,
    imb: f64,
) {
    c.body_a = a;
    c.body_b = b;
    // getCombinedFriction / Restitution: both bodies use the average.
    c.friction = (fa + fb) * 0.5;
    c.restitution = (ra + rb) * 0.5;
    c.world_space_normal = Vec3::ZERO;
    c.world_space_tangent = Vec3::ZERO;
    c.world_space_bitangent = Vec3::ZERO;
    c.inverse_mass_a = ima;
    c.inverse_mass_b = imb;
    c.num_contacts = 0;
}

fn clamp_speed(v: &mut Vec3, max_speed: f64, max_speed_squared: f64) {
    let squared_speed = v.squared_length();
    if squared_speed > max_speed_squared {
        v.scale(max_speed / squared_speed.sqrt());
    }
}

/// `Body.applyAccelerationIntegration( ... )`.
fn acceleration_integration(
    b: &mut Body,
    dt: f64,
    gravity: Vec3,
    linear_damping: f64,
    angular_damping: f64,
) {
    let mut forces = Vec3::ZERO;
    forces.scale_vector(b.linear_forces, b.inverse_mass);
    let mut gravity_acceleration = Vec3::ZERO;
    gravity_acceleration.scale_vector(gravity, b.gravity_scale);
    let mut linear = Vec3::ZERO;
    linear.add_vectors(forces, gravity_acceleration);
    b.linear_velocity.add_scaled(linear, dt);
    let damping = if b.linear_damping < 0. {
        linear_damping
    } else {
        b.linear_damping
    };
    b.linear_velocity.scale(max(0., 1. - damping * dt));
    clamp_speed(
        &mut b.linear_velocity,
        MAX_LINEAR_SPEED,
        MAX_LINEAR_SPEED * MAX_LINEAR_SPEED,
    );
    // computeEffectiveInverseInertiaVector
    let mask = Vec3::new(1., 1., 1.);
    let mut masked = Vec3::ZERO;
    masked.multiply_vectors(b.angular_forces, mask);
    let mut rotation = Mat3::default();
    rotation.from_quat(b.orientation);
    let mut inverse = Mat3::default();
    inverse.transpose_matrix(&rotation);
    let mut rotated = Vec3::ZERO;
    rotated.transform_vector_from_mat3(masked, &inverse);
    let r = rotated;
    rotated.transform_vector_from_mat3(r, &b.local_inverse_inertia);
    let r = rotated;
    rotated.transform_vector_from_mat3(r, &rotation);
    let mut angular = Vec3::ZERO;
    angular.multiply_vectors(rotated, mask);
    b.angular_velocity.add_scaled(angular, dt);
    let damping = if b.angular_damping < 0. {
        angular_damping
    } else {
        b.angular_damping
    };
    b.angular_velocity.scale(max(0., 1. - damping * dt));
    clamp_speed(
        &mut b.angular_velocity,
        MAX_ANGULAR_SPEED,
        MAX_ANGULAR_SPEED * MAX_ANGULAR_SPEED,
    );
}

/// `collideSphereVsSphere( ... )`: the hit in body A's space.
fn collide_sphere_vs_sphere(ra: f64, rb: f64, iso_a: &Mat4, iso_b: &Mat4) -> Option<Hit> {
    let pa = iso_a.get_translation();
    let pb = iso_b.get_translation();
    let mut ab = Vec3::ZERO;
    ab.subtract_vectors(pb, pa);
    if ab.squared_length() > squared(ra + rb) {
        return None;
    }
    let mut normal_a = Vec3::ZERO;
    normal_a.normalize_vector(ab);
    let mut normal_b = Vec3::ZERO;
    normal_b.negate_vector(normal_a);
    let mut contact_a = Vec3::ZERO;
    contact_a.add_scaled_to_vector(pa, normal_a, ra);
    let mut contact_b = Vec3::ZERO;
    contact_b.add_scaled_to_vector(pb, normal_b, rb);
    let penetration = ra + rb - ab.length();
    let n = normal_a;
    normal_a.normalize_vector(n);
    Some(Hit {
        penetration,
        contact_point_a: contact_a,
        contact_point_b: contact_b,
        normal_a,
    })
}
