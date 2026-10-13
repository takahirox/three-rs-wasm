//! ConvexHull ( three/addons/math/ConvexHull.js, quickhull without face
//! merging ) and ConvexGeometry, with the original's arithmetic order so a
//! hull built from the same points has the same faces in the same order.
//! The lists are arenas indexed by vertex, face and half-edge ids.
use crate::attribute::BufferAttribute;
use crate::{Result, geometry::*, math::*};

const VISIBLE: bool = true;

struct VertexNode {
    point: Vector3,
    prev: Option<usize>,
    next: Option<usize>,
    face: Option<usize>,
}

struct Face {
    normal: Vector3,
    constant: f64,
    outside: Option<usize>,
    mark: bool,
    edge: usize,
}

struct HalfEdge {
    vertex: usize,
    prev: usize,
    next: usize,
    twin: usize,
    face: usize,
}

#[derive(Default, Clone, Copy)]
struct VertexList {
    head: Option<usize>,
    tail: Option<usize>,
}

/// Vector3.normalize(): divideScalar( length || 1 ).
pub(super) fn normalize(v: Vector3) -> Vector3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    v * (1. / if length == 0. { 1. } else { length })
}

pub(super) fn cross(a: Vector3, b: Vector3) -> Vector3 {
    Vector3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub(super) fn dot(a: Vector3, b: Vector3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Plane.setFromCoplanarPoints( a, b, c ): ( normal, constant ).
pub(super) fn plane_from_coplanar_points(a: Vector3, b: Vector3, c: Vector3) -> (Vector3, f64) {
    let normal = normalize(cross(c - b, a - b));
    (normal, -dot(a, normal))
}

#[derive(Default)]
struct Hull {
    tolerance: f64,
    faces: Vec<usize>,
    new_faces: Vec<usize>,
    assigned: VertexList,
    unassigned: VertexList,
    vertices: Vec<VertexNode>,
    face_store: Vec<Face>,
    edges: Vec<HalfEdge>,
}

impl Hull {
    // VertexList operations on the given list.
    fn list_append(&mut self, list: &mut VertexList, v: usize) {
        match list.head {
            None => list.head = Some(v),
            Some(_) => self.vertices[list.tail.unwrap()].next = Some(v),
        }
        self.vertices[v].prev = list.tail;
        self.vertices[v].next = None;
        list.tail = Some(v);
    }
    fn list_append_chain(&mut self, list: &mut VertexList, mut v: usize) {
        match list.head {
            None => list.head = Some(v),
            Some(_) => self.vertices[list.tail.unwrap()].next = Some(v),
        }
        self.vertices[v].prev = list.tail;
        while let Some(next) = self.vertices[v].next {
            v = next;
        }
        list.tail = Some(v);
    }
    fn list_insert_before(&mut self, list: &mut VertexList, target: usize, v: usize) {
        self.vertices[v].prev = self.vertices[target].prev;
        self.vertices[v].next = Some(target);
        match self.vertices[v].prev {
            None => list.head = Some(v),
            Some(prev) => self.vertices[prev].next = Some(v),
        }
        self.vertices[target].prev = Some(v);
    }
    fn list_remove(&mut self, list: &mut VertexList, v: usize) {
        let (prev, next) = (self.vertices[v].prev, self.vertices[v].next);
        match prev {
            None => list.head = next,
            Some(p) => self.vertices[p].next = next,
        }
        match next {
            None => list.tail = prev,
            Some(n) => self.vertices[n].prev = prev,
        }
    }
    fn list_remove_sub_list(&mut self, list: &mut VertexList, a: usize, b: usize) {
        let (prev, next) = (self.vertices[a].prev, self.vertices[b].next);
        match prev {
            None => list.head = next,
            Some(p) => self.vertices[p].next = next,
        }
        match next {
            None => list.tail = prev,
            Some(n) => self.vertices[n].prev = prev,
        }
    }

    /// Face.create( a, b, c ) and compute().
    fn create_face(&mut self, a: usize, b: usize, c: usize) -> usize {
        let face = self.face_store.len();
        let e = self.edges.len();
        for (k, vertex) in [a, b, c].into_iter().enumerate() {
            self.edges.push(HalfEdge {
                vertex,
                prev: e + (k + 2) % 3,
                next: e + (k + 1) % 3,
                twin: usize::MAX,
                face,
            });
        }
        // compute(): the triangle ( tail, head, next head ).
        let pa = self.vertices[c].point;
        let pb = self.vertices[a].point;
        let pc = self.vertices[b].point;
        let n = cross(pc - pb, pa - pb);
        let length_sq = dot(n, n);
        let normal = if length_sq > 0. {
            n * (1. / length_sq.sqrt())
        } else {
            Vector3::ZERO
        };
        let midpoint = (pa + pb + pc) * (1. / 3.);
        self.face_store.push(Face {
            normal,
            constant: dot(normal, midpoint),
            outside: None,
            mark: VISIBLE,
            edge: e,
        });
        face
    }
    fn get_edge(&self, face: usize, mut i: i32) -> usize {
        let mut edge = self.face_store[face].edge;
        while i > 0 {
            edge = self.edges[edge].next;
            i -= 1;
        }
        while i < 0 {
            edge = self.edges[edge].prev;
            i += 1;
        }
        edge
    }
    fn set_twin(&mut self, a: usize, b: usize) {
        self.edges[a].twin = b;
        self.edges[b].twin = a;
    }
    fn distance(&self, face: usize, point: Vector3) -> f64 {
        let f = &self.face_store[face];
        dot(f.normal, point) - f.constant
    }
    fn tail(&self, edge: usize) -> usize {
        self.edges[self.edges[edge].prev].vertex
    }

    fn add_vertex_to_face(&mut self, vertex: usize, face: usize) {
        self.vertices[vertex].face = Some(face);
        let mut assigned = self.assigned;
        match self.face_store[face].outside {
            None => self.list_append(&mut assigned, vertex),
            Some(outside) => self.list_insert_before(&mut assigned, outside, vertex),
        }
        self.assigned = assigned;
        self.face_store[face].outside = Some(vertex);
    }
    fn remove_vertex_from_face(&mut self, vertex: usize, face: usize) {
        if Some(vertex) == self.face_store[face].outside {
            self.face_store[face].outside = match self.vertices[vertex].next {
                Some(next) if self.vertices[next].face == Some(face) => Some(next),
                _ => None,
            };
        }
        let mut assigned = self.assigned;
        self.list_remove(&mut assigned, vertex);
        self.assigned = assigned;
    }
    fn remove_all_vertices_from_face(&mut self, face: usize) -> Option<usize> {
        let start = self.face_store[face].outside?;
        let mut end = start;
        while let Some(next) = self.vertices[end].next {
            if self.vertices[next].face != Some(face) {
                break;
            }
            end = next;
        }
        let mut assigned = self.assigned;
        self.list_remove_sub_list(&mut assigned, start, end);
        self.assigned = assigned;
        self.vertices[start].prev = None;
        self.vertices[end].next = None;
        self.face_store[face].outside = None;
        Some(start)
    }
    fn delete_face_vertices(&mut self, face: usize) {
        if let Some(vertices) = self.remove_all_vertices_from_face(face) {
            let mut unassigned = self.unassigned;
            self.list_append_chain(&mut unassigned, vertices);
            self.unassigned = unassigned;
        }
    }
    fn resolve_unassigned_points(&mut self) {
        let mut vertex = self.unassigned.head;
        while let Some(v) = vertex {
            let next_vertex = self.vertices[v].next;
            let mut max_distance = self.tolerance;
            let mut max_face = None;
            for i in 0..self.new_faces.len() {
                let face = self.new_faces[i];
                if self.face_store[face].mark == VISIBLE {
                    let distance = self.distance(face, self.vertices[v].point);
                    if distance > max_distance {
                        max_distance = distance;
                        max_face = Some(face);
                    }
                    if max_distance > 1000. * self.tolerance {
                        break;
                    }
                }
            }
            if let Some(face) = max_face {
                self.add_vertex_to_face(v, face);
            }
            vertex = next_vertex;
        }
    }
    fn compute_extremes(&mut self) -> ([usize; 3], [usize; 3]) {
        let (mut min_v, mut max_v) = ([0; 3], [0; 3]);
        let mut min = self.vertices[0].point;
        let mut max = min;
        for (i, vertex) in self.vertices.iter().enumerate() {
            let point = vertex.point;
            for j in 0..3 {
                if point[j] < min[j] {
                    min[j] = point[j];
                    min_v[j] = i;
                }
            }
            for j in 0..3 {
                if point[j] > max[j] {
                    max[j] = point[j];
                    max_v[j] = i;
                }
            }
        }
        self.tolerance = 3.
            * f64::EPSILON
            * (min.x.abs().max(max.x.abs())
                + min.y.abs().max(max.y.abs())
                + min.z.abs().max(max.z.abs()));
        (min_v, max_v)
    }
    fn compute_initial_hull(&mut self) {
        let (min, max) = self.compute_extremes();
        let mut max_distance = 0.;
        let mut index = 0;
        for i in 0..3 {
            let distance = self.vertices[max[i]].point[i] - self.vertices[min[i]].point[i];
            if distance > max_distance {
                max_distance = distance;
                index = i;
            }
        }
        let (v0, v1) = (min[index], max[index]);
        // The point farthest from the line v0 v1 ( Line3.closestPointToPoint, clamped ).
        let (start, end) = (self.vertices[v0].point, self.vertices[v1].point);
        let mut v2 = None;
        max_distance = 0.;
        for (i, vertex) in self.vertices.iter().enumerate() {
            if i != v0 && i != v1 {
                let start_p = vertex.point - start;
                let start_end = end - start;
                let t = (dot(start_end, start_p) / dot(start_end, start_end)).clamp(0., 1.);
                let closest = start_end * t + start;
                let d = closest - vertex.point;
                let distance = d.x * d.x + d.y * d.y + d.z * d.z;
                if distance > max_distance {
                    max_distance = distance;
                    v2 = Some(i);
                }
            }
        }
        let v2 = v2.unwrap_or(usize::MAX);
        let (normal, constant) = plane_from_coplanar_points(
            self.vertices[v0].point,
            self.vertices[v1].point,
            self.vertices.get(v2).map_or(Vector3::NAN, |v| v.point),
        );
        let mut v3 = usize::MAX;
        max_distance = -1.;
        for (i, vertex) in self.vertices.iter().enumerate() {
            if i != v0 && i != v1 && i != v2 {
                let distance = (dot(normal, vertex.point) + constant).abs();
                if distance > max_distance {
                    max_distance = distance;
                    v3 = i;
                }
            }
        }
        let mut faces = vec![];
        if dot(normal, self.vertices[v3].point) + constant < 0. {
            faces.push(self.create_face(v0, v1, v2));
            faces.push(self.create_face(v3, v1, v0));
            faces.push(self.create_face(v3, v2, v1));
            faces.push(self.create_face(v3, v0, v2));
            for i in 0..3 {
                let j = (i + 1) % 3;
                let (a, b) = (
                    self.get_edge(faces[i + 1], 2),
                    self.get_edge(faces[0], j as i32),
                );
                self.set_twin(a, b);
                let (a, b) = (
                    self.get_edge(faces[i + 1], 1),
                    self.get_edge(faces[j + 1], 0),
                );
                self.set_twin(a, b);
            }
        } else {
            faces.push(self.create_face(v0, v2, v1));
            faces.push(self.create_face(v3, v0, v1));
            faces.push(self.create_face(v3, v1, v2));
            faces.push(self.create_face(v3, v2, v0));
            for i in 0..3 {
                let j = (i + 1) % 3;
                let (a, b) = (
                    self.get_edge(faces[i + 1], 2),
                    self.get_edge(faces[0], ((3 - i) % 3) as i32),
                );
                self.set_twin(a, b);
                let (a, b) = (
                    self.get_edge(faces[i + 1], 0),
                    self.get_edge(faces[j + 1], 1),
                );
                self.set_twin(a, b);
            }
        }
        self.faces.extend_from_slice(&faces);
        for i in 0..self.vertices.len() {
            if i != v0 && i != v1 && i != v2 && i != v3 {
                let mut max_distance = self.tolerance;
                let mut max_face = None;
                for j in 0..4 {
                    let distance = self.distance(self.faces[j], self.vertices[i].point);
                    if distance > max_distance {
                        max_distance = distance;
                        max_face = Some(self.faces[j]);
                    }
                }
                if let Some(face) = max_face {
                    self.add_vertex_to_face(i, face);
                }
            }
        }
    }
    fn next_vertex_to_add(&self) -> Option<usize> {
        let first = self.assigned.head?;
        let eye_face = self.vertices[first].face?;
        let mut eye_vertex = None;
        let mut max_distance = 0.;
        let mut vertex = self.face_store[eye_face].outside;
        while let Some(v) = vertex {
            let distance = self.distance(eye_face, self.vertices[v].point);
            if distance > max_distance {
                max_distance = distance;
                eye_vertex = Some(v);
            }
            vertex = self.vertices[v].next;
            if vertex.is_some_and(|n| self.vertices[n].face != Some(eye_face)) {
                break;
            }
        }
        eye_vertex
    }
    fn compute_horizon(
        &mut self,
        eye_point: Vector3,
        cross_edge: Option<usize>,
        face: usize,
        horizon: &mut Vec<usize>,
    ) {
        self.delete_face_vertices(face);
        self.face_store[face].mark = !VISIBLE;
        let (mut edge, cross_edge) = match cross_edge {
            None => {
                let e = self.get_edge(face, 0);
                (e, e)
            }
            Some(c) => (self.edges[c].next, c),
        };
        loop {
            let twin_edge = self.edges[edge].twin;
            let opposite_face = self.edges[twin_edge].face;
            if self.face_store[opposite_face].mark == VISIBLE {
                if self.distance(opposite_face, eye_point) > self.tolerance {
                    self.compute_horizon(eye_point, Some(twin_edge), opposite_face, horizon);
                } else {
                    horizon.push(edge);
                }
            }
            edge = self.edges[edge].next;
            if edge == cross_edge {
                break;
            }
        }
    }
    fn add_adjoining_face(&mut self, eye_vertex: usize, horizon_edge: usize) -> usize {
        let face = self.create_face(
            eye_vertex,
            self.tail(horizon_edge),
            self.edges[horizon_edge].vertex,
        );
        self.faces.push(face);
        let (a, b) = (self.get_edge(face, -1), self.edges[horizon_edge].twin);
        self.set_twin(a, b);
        self.get_edge(face, 0)
    }
    fn add_new_faces(&mut self, eye_vertex: usize, horizon: &[usize]) {
        self.new_faces.clear();
        let mut first_side_edge = None;
        let mut previous_side_edge = 0;
        for &horizon_edge in horizon {
            let side_edge = self.add_adjoining_face(eye_vertex, horizon_edge);
            if first_side_edge.is_none() {
                first_side_edge = Some(side_edge);
            } else {
                let next = self.edges[side_edge].next;
                self.set_twin(next, previous_side_edge);
            }
            self.new_faces.push(self.edges[side_edge].face);
            previous_side_edge = side_edge;
        }
        let next = self.edges[first_side_edge.unwrap()].next;
        self.set_twin(next, previous_side_edge);
    }
    fn add_vertex_to_hull(&mut self, eye_vertex: usize) {
        let mut horizon = vec![];
        self.unassigned = VertexList::default();
        let face = self.vertices[eye_vertex].face.unwrap();
        self.remove_vertex_from_face(eye_vertex, face);
        self.compute_horizon(self.vertices[eye_vertex].point, None, face, &mut horizon);
        self.add_new_faces(eye_vertex, &horizon);
        self.resolve_unassigned_points();
    }
}

/// ConvexGeometry( points ): per hull face, its three vertices with the
/// face normal ( no index ). Fewer than four points give an empty geometry.
pub(super) fn convex_geometry(points: &[Vector3]) -> Result<BufferGeometry> {
    let mut hull = Hull::default();
    if points.len() >= 4 {
        hull.vertices = points
            .iter()
            .map(|&point| VertexNode {
                point,
                prev: None,
                next: None,
                face: None,
            })
            .collect();
        hull.compute_initial_hull();
        while let Some(vertex) = hull.next_vertex_to_add() {
            hull.add_vertex_to_hull(vertex);
        }
    }
    let (mut vertices, mut normals) = (vec![], vec![]);
    for &face in &hull.faces {
        let f = &hull.face_store[face];
        if f.mark != VISIBLE {
            continue;
        }
        let mut edge = f.edge;
        loop {
            let p = hull.vertices[hull.edges[edge].vertex].point;
            vertices.extend([p.x as f32, p.y as f32, p.z as f32]);
            normals.extend([f.normal.x as f32, f.normal.y as f32, f.normal.z as f32]);
            edge = hull.edges[edge].next;
            if edge == f.edge {
                break;
            }
        }
    }
    let mut geometry = BufferGeometry::default();
    geometry.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(vertices, 3, false)?),
    );
    geometry.set_attribute(
        "normal",
        Attribute::F32(BufferAttribute::new(normals, 3, false)?),
    );
    Ok(geometry)
}
