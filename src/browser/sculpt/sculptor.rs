//! Sculptor ( three.js r186 addon, adapted from SculptGL ): the dynamic
//! topology sculpting mesh on the CPU, as the addon keeps it — welded
//! vertices with their vertex and face rings, an octree of faces, the
//! subdivision and decimation passes and the brush tools — with the
//! addon's Float32Array, Uint32Array and Int32Array stores. The stroke code
//! follows the addon's order of operations so the same pointer path makes
//! the same mesh.
use std::collections::HashMap;

const TRI_INDEX: u32 = u32::MAX;
const MAX_FLAG: i32 = 0x7fffffff;
const OCTREE_MAX_DEPTH: u32 = 8;
const OCTREE_MAX_FACES: usize = 100;
const RELATIVE_WELD_TOLERANCE: f64 = 1e-7;
const RAY_EPSILON: f64 = 1e-15;
const MAX_UPDATE_RANGES: usize = 8;
const MAX_UPDATE_GAP_COMPONENTS: usize = 96;
const STAMP_SPACING_RATIO: f64 = 0.15;
const TOPOLOGY_HYSTERESIS2: f64 = 2.05 * 2.05;
const CLAY_OFFSET_RATIO: f64 = 0.1;
type V3 = [f64; 3];

// ---------------------------------------------------------------- utils

/// Math.min / Math.max: NaN propagates.
fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a < b {
        a
    } else {
        b
    }
}
fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a > b {
        a
    } else {
        b
    }
}
/// V8's Math.hypot.
pub(super) fn hypot(values: &[f64]) -> f64 {
    let mut max = 0f64;
    for v in values {
        if v.is_infinite() {
            return f64::INFINITY;
        }
        if v.is_nan() {
            return f64::NAN;
        }
        max = max.max(v.abs());
    }
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
fn replace_element(array: &mut [u32], old: u32, new: u32) {
    if let Some(v) = array.iter_mut().find(|v| **v == old) {
        *v = new;
    }
}
fn remove_element(array: &mut Vec<u32>, value: u32) {
    if let Some(i) = array.iter().position(|&v| v == value) {
        let last = array[array.len() - 1];
        array[i] = last;
        array.pop();
    }
}
fn tidy(array: &mut Vec<u32>) {
    if array.len() < 2 {
        return;
    }
    array.sort_unstable();
    array.dedup();
}
fn sqr_dist(a: V3, b: V3) -> f64 {
    let (dx, dy, dz) = (a[0] - b[0], a[1] - b[1], a[2] - b[2]);
    dx * dx + dy * dy + dz * dz
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
// The comparisons keep NaN passing through, as the addon's do.
#[allow(clippy::manual_range_contains)]
fn intersection_ray_triangle_scaled(orig: V3, dir: V3, v1: V3, v2: V3, v3: V3) -> (f64, V3) {
    let mut edge1 = sub(v2, v1);
    let mut edge2 = sub(v3, v1);
    let scale = |v: V3| js_max(js_max(v[0].abs(), v[1].abs()), v[2].abs());
    let (edge1_scale, edge2_scale, dir_scale) = (scale(edge1), scale(edge2), scale(dir));
    if edge1_scale == 0.
        || edge2_scale == 0.
        || dir_scale == 0.
        || !edge1_scale.is_finite()
        || !edge2_scale.is_finite()
        || !dir_scale.is_finite()
    {
        return (-1., [0.; 3]);
    }
    for k in 0..3 {
        edge1[k] /= edge1_scale;
        edge2[k] /= edge2_scale;
    }
    let scaled_dir = [dir[0] / dir_scale, dir[1] / dir_scale, dir[2] / dir_scale];
    let pvec = cross(scaled_dir, edge2);
    let det = dot(edge1, pvec);
    let determinant_scale = hypot(&edge1) * hypot(&edge2) * hypot(&scaled_dir);
    if det.abs() <= RAY_EPSILON * determinant_scale {
        return (-1., [0.; 3]);
    }
    let inv_det = 1.0 / det;
    let mut tvec = sub(orig, v1);
    let translation_scale = scale(tvec);
    if !translation_scale.is_finite() {
        return (-1., [0.; 3]);
    }
    if translation_scale != 0. {
        for v in &mut tvec {
            *v /= translation_scale;
        }
    }
    let u = dot(tvec, pvec) * inv_det * translation_scale / edge1_scale;
    if u < -RAY_EPSILON || u > 1.0 + RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    let qvec = cross(tvec, edge1);
    let v = dot(scaled_dir, qvec) * inv_det * translation_scale / edge2_scale;
    if v < -RAY_EPSILON || u + v > 1.0 + RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    let t = dot(edge2, qvec) * inv_det * translation_scale / dir_scale;
    if t < -RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    (
        t,
        [
            orig[0] + dir[0] * t,
            orig[1] + dir[1] * t,
            orig[2] + dir[2] * t,
        ],
    )
}
/// intersectionRayTriangle: the distance ( or −1 ) and the hit point.
#[allow(clippy::manual_range_contains)]
fn intersection_ray_triangle(orig: V3, dir: V3, v1: V3, v2: V3, v3: V3) -> (f64, V3) {
    let edge1 = sub(v2, v1);
    let edge2 = sub(v3, v1);
    let pvec = cross(dir, edge2);
    let det = dot(edge1, pvec);
    let determinant_scale = hypot(&edge1) * hypot(&edge2) * hypot(&dir);
    let tolerance = RAY_EPSILON * determinant_scale;
    if !det.is_finite() || !determinant_scale.is_finite() || tolerance == 0. {
        return intersection_ray_triangle_scaled(orig, dir, v1, v2, v3);
    }
    if det.abs() <= tolerance {
        return (-1., [0.; 3]);
    }
    let inv_det = 1.0 / det;
    let tvec = sub(orig, v1);
    let u = dot(tvec, pvec) * inv_det;
    if !u.is_finite() {
        return intersection_ray_triangle_scaled(orig, dir, v1, v2, v3);
    }
    if u < -RAY_EPSILON || u > 1.0 + RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    let qvec = cross(tvec, edge1);
    let v = dot(dir, qvec) * inv_det;
    if !v.is_finite() {
        return intersection_ray_triangle_scaled(orig, dir, v1, v2, v3);
    }
    if v < -RAY_EPSILON || u + v > 1.0 + RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    let t = dot(edge2, qvec) * inv_det;
    if !t.is_finite() {
        return intersection_ray_triangle_scaled(orig, dir, v1, v2, v3);
    }
    if t < -RAY_EPSILON {
        return (-1., [0.; 3]);
    }
    (
        t,
        [
            orig[0] + dir[0] * t,
            orig[1] + dir[1] * t,
            orig[2] + dir[2] * t,
        ],
    )
}
fn distance_sq_to_segment(point: V3, v1: V3, v2: V3) -> f64 {
    let (ptx, pty, ptz) = (point[0] - v1[0], point[1] - v1[1], point[2] - v1[2]);
    let (vx, vy, vz) = (v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]);
    let length_squared = vx * vx + vy * vy + vz * vz;
    if length_squared == 0. {
        return ptx * ptx + pty * pty + ptz * ptz;
    }
    let t = (ptx * vx + pty * vy + ptz * vz) / length_squared;
    if t < 0. {
        return ptx * ptx + pty * pty + ptz * ptz;
    }
    if t > 1. {
        let (dx, dy, dz) = (point[0] - v2[0], point[1] - v2[1], point[2] - v2[2]);
        return dx * dx + dy * dy + dz * dz;
    }
    let rx = point[0] - v1[0] - t * vx;
    let ry = point[1] - v1[1] - t * vy;
    let rz = point[2] - v1[2] - t * vz;
    rx * rx + ry * ry + rz * rz
}
fn distance_sq_to_triangle(point: V3, v1: V3, v2: V3, v3: V3) -> f64 {
    let (abx, aby, abz) = (v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]);
    let (acx, acy, acz) = (v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]);
    let (bcx, bcy, bcz) = (v3[0] - v2[0], v3[1] - v2[1], v3[2] - v2[2]);
    let cross_x = aby * acz - abz * acy;
    let cross_y = abz * acx - abx * acz;
    let cross_z = abx * acy - aby * acx;
    let area_squared = cross_x * cross_x + cross_y * cross_y + cross_z * cross_z;
    let max_edge_squared = js_max(
        js_max(
            abx * abx + aby * aby + abz * abz,
            acx * acx + acy * acy + acz * acz,
        ),
        bcx * bcx + bcy * bcy + bcz * bcz,
    );
    if area_squared <= f64::EPSILON * max_edge_squared * max_edge_squared {
        return js_min(
            js_min(
                distance_sq_to_segment(point, v1, v2),
                distance_sq_to_segment(point, v2, v3),
            ),
            distance_sq_to_segment(point, v3, v1),
        );
    }
    let (apx, apy, apz) = (point[0] - v1[0], point[1] - v1[1], point[2] - v1[2]);
    let d1 = abx * apx + aby * apy + abz * apz;
    let d2 = acx * apx + acy * apy + acz * apz;
    if d1 <= 0. && d2 <= 0. {
        return apx * apx + apy * apy + apz * apz;
    }
    let (bpx, bpy, bpz) = (point[0] - v2[0], point[1] - v2[1], point[2] - v2[2]);
    let d3 = abx * bpx + aby * bpy + abz * bpz;
    let d4 = acx * bpx + acy * bpy + acz * bpz;
    if d3 >= 0. && d4 <= d3 {
        return bpx * bpx + bpy * bpy + bpz * bpz;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0. && d1 >= 0. && d3 <= 0. {
        let v = d1 / (d1 - d3);
        let (dx, dy, dz) = (apx - abx * v, apy - aby * v, apz - abz * v);
        return dx * dx + dy * dy + dz * dz;
    }
    let (cpx, cpy, cpz) = (point[0] - v3[0], point[1] - v3[1], point[2] - v3[2]);
    let d5 = abx * cpx + aby * cpy + abz * cpz;
    let d6 = acx * cpx + acy * cpy + acz * cpz;
    if d6 >= 0. && d5 <= d6 {
        return cpx * cpx + cpy * cpy + cpz * cpz;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0. && d2 >= 0. && d6 <= 0. {
        let w = d2 / (d2 - d6);
        let (dx, dy, dz) = (apx - acx * w, apy - acy * w, apz - acz * w);
        return dx * dx + dy * dy + dz * dz;
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0. && d4 - d3 >= 0. && d5 - d6 >= 0. {
        let (ex, ey, ez) = (v3[0] - v2[0], v3[1] - v2[1], v3[2] - v2[2]);
        let w = (d4 - d3) / (d4 - d3 + d5 - d6);
        let (dx, dy, dz) = (bpx - ex * w, bpy - ey * w, bpz - ez * w);
        return dx * dx + dy * dy + dz * dz;
    }
    let denominator = va + vb + vc;
    let inverse = 1.0 / denominator;
    let v = vb * inverse;
    let w = vc * inverse;
    let dx = apx - abx * v - acx * w;
    let dy = apy - aby * v - acy * w;
    let dz = apz - abz * v - acz * w;
    dx * dx + dy * dy + dz * dz
}
fn triangle_inside_sphere(point: V3, radius_squared: f64, v1: V3, v2: V3, v3: V3) -> bool {
    distance_sq_to_triangle(point, v1, v2, v3) < radius_squared
}
fn falloff(dist: f64) -> f64 {
    let d2 = dist * dist;
    3.0 * d2 * d2 - 4.0 * d2 * dist + 1.0
}
/// Math.imul-based cell hash.
fn hash_position(x: f64, y: f64, z: f64) -> u32 {
    let int = |v: f64| v as i64 as i32 as u32;
    let mut hash = int(x).wrapping_mul(0x85ebca6b);
    hash = (hash ^ (hash >> 13) ^ int(y)).wrapping_mul(0xc2b2ae35);
    hash = (hash ^ (hash >> 16) ^ int(z)).wrapping_mul(0x27d4eb2d);
    hash ^ (hash >> 15)
}

// ---------------------------------------------------------------- octree

struct Cell {
    parent: Option<usize>,
    depth: u32,
    children: Vec<usize>,
    loose: [f64; 6],
    split: [f64; 6],
    faces: Vec<u32>,
    queued: bool,
}
impl Cell {
    fn new(parent: Option<usize>, depth: u32) -> Self {
        let inf = f64::INFINITY;
        Self {
            parent,
            depth,
            children: vec![],
            loose: [inf, inf, inf, -inf, -inf, -inf],
            split: [inf, inf, inf, -inf, -inf, -inf],
            faces: vec![],
            queued: false,
        }
    }
}

// ---------------------------------------------------------------- mesh

/// SculptorMesh: the sculpting topology and its octree.
pub(super) struct Mesh {
    nb_vertices: usize,
    nb_faces: usize,
    pub(super) vertices: Vec<f32>,
    normals: Vec<f32>,
    pub(super) render_normals: Vec<f32>,
    faces: Vec<u32>,
    pub(super) triangles: Vec<u32>,
    vring_vert: Vec<Vec<u32>>,
    vring_face: Vec<Vec<u32>>,
    vert_on_edge: Vec<u8>,
    face_normals: Vec<f32>,
    face_boxes: Vec<f32>,
    face_centers: Vec<f32>,
    face_pos_in_leaf: Vec<u32>,
    face_leaf: Vec<Option<usize>>,
    vert_tag_flags: Vec<i32>,
    vert_sculpt_flags: Vec<i32>,
    faces_tag_flags: Vec<i32>,
    cells: Vec<Cell>,
    leaves_to_update: Vec<usize>,
    topology_version: u64,
    tag_flag: i32,
    sculpt_flag: i32,
    /// DecData: the decimated vertices and the deletions.
    decimated: Vec<u32>,
    tris_to_delete: Vec<u32>,
    verts_to_delete: Vec<u32>,
}
/// SubData and DecData's shared state.
struct Sub {
    vertices_map: HashMap<u64, u32>,
    edge_key_stride: u64,
    center: V3,
    radius2: f64,
    edge_max2: f64,
}
impl Mesh {
    pub(super) fn nb_vertices(&self) -> usize {
        self.nb_vertices
    }
    pub(super) fn nb_triangles(&self) -> usize {
        self.nb_faces
    }
    pub(super) fn topology_version(&self) -> u64 {
        self.topology_version
    }
    fn v(&self, i: u32) -> V3 {
        let o = i as usize * 3;
        [
            self.vertices[o] as f64,
            self.vertices[o + 1] as f64,
            self.vertices[o + 2] as f64,
        ]
    }
    fn next_tag_flag(&mut self) -> i32 {
        if self.tag_flag == MAX_FLAG {
            let reset = |flags: &mut Vec<i32>, active: usize| {
                for f in flags.iter_mut().take(active) {
                    if *f >= 0 {
                        *f = 0;
                    }
                }
                for f in flags.iter_mut().skip(active) {
                    *f = 0;
                }
            };
            reset(&mut self.vert_tag_flags, self.nb_vertices);
            reset(&mut self.faces_tag_flags, self.nb_faces);
            self.tag_flag = 1;
        } else {
            self.tag_flag += 1;
        }
        self.tag_flag
    }
    fn next_sculpt_flag(&mut self) -> i32 {
        if self.sculpt_flag == MAX_FLAG {
            self.vert_sculpt_flags.fill(0);
            self.sculpt_flag = 1;
        } else {
            self.sculpt_flag += 1;
        }
        self.sculpt_flag
    }
    fn add_nb_face(&mut self, nb: isize) {
        self.nb_faces = (self.nb_faces as isize + nb) as usize;
        self.topology_version += 1;
    }
    fn add_nb_vertice(&mut self, nb: isize) {
        self.nb_vertices = (self.nb_vertices as isize + nb) as usize;
    }
    /// initFromGeometry( geometry ) of a non-indexed position list.
    pub(super) fn new(source: &[f32]) -> Self {
        let count = source.len() / 3;
        let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in source.chunks(3) {
            for k in 0..3 {
                let v = p[k] as f64;
                if v < min[k] {
                    min[k] = v;
                }
                if v > max[k] {
                    max[k] = v;
                }
            }
        }
        // weldPositions.
        let extent = js_max(js_max(max[0] - min[0], max[1] - min[1]), max[2] - min[2]);
        let tolerance = extent * RELATIVE_WELD_TOLERANCE;
        let tolerance_squared = tolerance * tolerance;
        let inverse_cell = if tolerance > 0. { 0.5 / tolerance } else { 0. };
        let mut heads: HashMap<u32, usize> = HashMap::new();
        let mut next: Vec<Option<usize>> = vec![];
        let mut merged: Vec<f64> = vec![];
        let mut vertex_map = vec![0u32; count];
        let search = if inverse_cell > 0. { 2 } else { 1 };
        for i in 0..count {
            let (x, y, z) = (
                source[i * 3] as f64,
                source[i * 3 + 1] as f64,
                source[i * 3 + 2] as f64,
            );
            let grid = [
                (x - min[0]) * inverse_cell,
                (y - min[1]) * inverse_cell,
                (z - min[2]) * inverse_cell,
            ];
            let cell = grid.map(f64::floor);
            let neighbor =
                [0, 1, 2].map(|k| cell[k] + if grid[k] - cell[k] < 0.5 { -1. } else { 1. });
            let mut merged_index: Option<usize> = None;
            let mut closest = f64::INFINITY;
            'search: for iz in 0..search {
                let sz = if iz == 0 { cell[2] } else { neighbor[2] };
                for iy in 0..search {
                    let sy = if iy == 0 { cell[1] } else { neighbor[1] };
                    for ix in 0..search {
                        let sx = if ix == 0 { cell[0] } else { neighbor[0] };
                        let mut candidate = heads.get(&hash_position(sx, sy, sz)).copied();
                        while let Some(c) = candidate {
                            let dx = merged[c * 3] - x;
                            let dy = merged[c * 3 + 1] - y;
                            let dz = merged[c * 3 + 2] - z;
                            let d = dx * dx + dy * dy + dz * dz;
                            if d == 0. {
                                merged_index = Some(c);
                                break 'search;
                            }
                            if d <= tolerance_squared && d < closest {
                                merged_index = Some(c);
                                closest = d;
                            }
                            candidate = next[c];
                        }
                    }
                }
            }
            let index = match merged_index {
                Some(m) => m,
                None => {
                    let m = merged.len() / 3;
                    let hash = hash_position(cell[0], cell[1], cell[2]);
                    next.push(heads.get(&hash).copied());
                    heads.insert(hash, m);
                    merged.extend([x, y, z]);
                    m
                }
            };
            vertex_map[i] = index as u32;
        }
        let nb_vertices = merged.len() / 3;
        let nb_faces = count / 3;
        let mut faces = vec![0u32; nb_faces * 4];
        let mut triangles = vec![0u32; nb_faces * 3];
        for i in 0..nb_faces {
            let (a, b, c) = (
                vertex_map[i * 3],
                vertex_map[i * 3 + 1],
                vertex_map[i * 3 + 2],
            );
            faces[i * 4..i * 4 + 4].copy_from_slice(&[a, b, c, TRI_INDEX]);
            triangles[i * 3..i * 3 + 3].copy_from_slice(&[a, b, c]);
        }
        let mut mesh = Self {
            nb_vertices,
            nb_faces,
            vertices: merged.iter().map(|&v| v as f32).collect(),
            normals: vec![0.; nb_vertices * 3],
            render_normals: vec![0.; nb_vertices * 3],
            faces,
            triangles,
            vring_vert: vec![],
            vring_face: vec![],
            vert_on_edge: vec![0; nb_vertices],
            face_normals: vec![0.; nb_faces * 3],
            face_boxes: vec![0.; nb_faces * 6],
            face_centers: vec![0.; nb_faces * 3],
            face_pos_in_leaf: vec![0; nb_faces],
            face_leaf: vec![None; nb_faces],
            vert_tag_flags: vec![0; nb_vertices],
            vert_sculpt_flags: vec![0; nb_vertices],
            faces_tag_flags: vec![0; nb_faces],
            cells: vec![],
            leaves_to_update: vec![],
            topology_version: 0,
            tag_flag: 1,
            sculpt_flag: 1,
            decimated: vec![],
            tris_to_delete: vec![],
            verts_to_delete: vec![],
        };
        mesh.init_topology();
        mesh.update_geometry(None, None);
        mesh
    }
    fn init_topology(&mut self) {
        let n = self.nb_vertices;
        self.vring_vert = vec![vec![]; n];
        self.vring_face = vec![vec![]; n];
        for i in 0..self.nb_faces {
            for k in 0..3 {
                let v = self.triangles[i * 3 + k] as usize;
                self.vring_face[v].push(i as u32);
            }
        }
        for i in 0..n {
            self.compute_ring_vertices(i as u32);
            self.vert_on_edge[i] = u8::from(self.vring_face[i].len() != self.vring_vert[i].len());
        }
    }
    fn compute_ring_vertices(&mut self, vert: u32) {
        let tag = self.next_tag_flag();
        let mut ring = std::mem::take(&mut self.vring_vert[vert as usize]);
        ring.clear();
        for &face in &self.vring_face[vert as usize] {
            let ind = face as usize * 4;
            let mut v1 = self.faces[ind];
            let mut v2 = self.faces[ind + 1];
            if v1 == vert {
                v1 = self.faces[ind + 2];
            } else if v2 == vert {
                v2 = self.faces[ind + 2];
            }
            for v in [v1, v2] {
                if self.vert_tag_flags[v as usize] != tag {
                    self.vert_tag_flags[v as usize] = tag;
                    ring.push(v);
                }
            }
        }
        self.vring_vert[vert as usize] = ring;
    }
    fn update_geometry(&mut self, faces: Option<&[u32]>, verts: Option<&[u32]>) {
        let derived;
        let verts = match (verts, faces) {
            (None, Some(f)) => {
                derived = self.vertices_from_faces(f);
                Some(&derived[..])
            }
            (v, _) => v,
        };
        self.update_faces_aabb_and_normal(faces);
        self.update_vertices_normal(verts);
        self.update_octree(faces);
    }
    fn update_faces_aabb_and_normal(&mut self, faces: Option<&[u32]>) {
        let n = faces.map_or(self.nb_faces, <[u32]>::len);
        for i in 0..n {
            let ind = faces.map_or(i, |f| f[i] as usize);
            let (a, b, c) = (
                self.faces[ind * 4],
                self.faces[ind * 4 + 1],
                self.faces[ind * 4 + 2],
            );
            let (v1, v2, v3) = (self.v(a), self.v(b), self.v(c));
            let (ax, ay, az) = (v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]);
            let (bx, by, bz) = (v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]);
            self.face_normals[ind * 3] = (ay * bz - az * by) as f32;
            self.face_normals[ind * 3 + 1] = (az * bx - ax * bz) as f32;
            self.face_normals[ind * 3 + 2] = (ax * by - ay * bx) as f32;
            let mut bounds = [0f64; 6];
            for k in 0..3 {
                let (p, q, r) = (v1[k], v2[k], v3[k]);
                bounds[k] = if p < q {
                    if p < r { p } else { r }
                } else if q < r {
                    q
                } else {
                    r
                };
                bounds[k + 3] = if p > q {
                    if p > r { p } else { r }
                } else if q > r {
                    q
                } else {
                    r
                };
            }
            for (k, &b) in bounds.iter().enumerate() {
                self.face_boxes[ind * 6 + k] = b as f32;
            }
            for k in 0..3 {
                self.face_centers[ind * 3 + k] = ((bounds[k] + bounds[k + 3]) * 0.5) as f32;
            }
        }
    }
    fn update_vertices_normal(&mut self, verts: Option<&[u32]>) {
        let n = verts.map_or(self.nb_vertices, <[u32]>::len);
        for i in 0..n {
            let ind = verts.map_or(i, |v| v[i] as usize);
            let ring = &self.vring_face[ind];
            let (mut nx, mut ny, mut nz) = (0f64, 0f64, 0f64);
            for &f in ring {
                let id = f as usize * 3;
                nx += self.face_normals[id] as f64;
                ny += self.face_normals[id + 1] as f64;
                nz += self.face_normals[id + 2] as f64;
            }
            let inverse = if !ring.is_empty() {
                1.0 / ring.len() as f64
            } else {
                0.
            };
            nx *= inverse;
            ny *= inverse;
            nz *= inverse;
            let o = ind * 3;
            self.normals[o] = nx as f32;
            self.normals[o + 1] = ny as f32;
            self.normals[o + 2] = nz as f32;
            let length = (nx * nx + ny * ny + nz * nz).sqrt();
            let inv = if length > 0. { 1.0 / length } else { 0. };
            self.render_normals[o] = (nx * inv) as f32;
            self.render_normals[o + 1] = (ny * inv) as f32;
            self.render_normals[o + 2] = (nz * inv) as f32;
        }
    }
    fn update_octree(&mut self, faces: Option<&[u32]>) {
        match faces {
            None => self.compute_octree(),
            Some(f) => {
                let moved = self.octree_remove(f);
                self.octree_add(&moved);
            }
        }
    }
    fn compute_octree(&mut self) {
        let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for i in 0..self.nb_vertices {
            for k in 0..3 {
                let v = self.vertices[i * 3 + k] as f64;
                if v < min[k] {
                    min[k] = v;
                }
                if v > max[k] {
                    max[k] = v;
                }
            }
        }
        let d = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
        let thickness = hypot(&d) * 0.2;
        for k in 0..3 {
            if d[k] == 0. {
                min[k] -= thickness;
                max[k] += thickness;
            }
        }
        let mut root = Cell::new(None, 0);
        root.faces = (0..self.nb_faces as u32).collect();
        root.loose = [min[0], min[1], min[2], max[0], max[1], max[2]];
        root.split = [
            min[0] - d[0] * 0.3,
            min[1] - d[1] * 0.3,
            min[2] - d[2] * 0.3,
            max[0] + d[0] * 0.3,
            max[1] + d[1] * 0.3,
            max[2] + d[2] * 0.3,
        ];
        self.cells = vec![root];
        self.build(0);
        self.leaves_to_update.clear();
    }
    fn build(&mut self, start: usize) {
        let mut stack = vec![start];
        let mut leaves = vec![];
        while let Some(cell) = stack.pop() {
            let nb = self.cells[cell].faces.len();
            if nb > OCTREE_MAX_FACES && self.cells[cell].depth < OCTREE_MAX_DEPTH {
                self.construct_children(cell);
                stack.extend(self.cells[cell].children.iter().copied());
            } else if nb > 0 {
                leaves.push(cell);
            }
        }
        for leaf in leaves {
            self.construct_leaf(leaf);
        }
    }
    fn construct_leaf(&mut self, cell: usize) {
        let faces = std::mem::take(&mut self.cells[cell].faces);
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for (i, &id) in faces.iter().enumerate() {
            self.set_face_leaf(id as usize, Some(cell));
            self.face_pos_in_leaf[id as usize] = i as u32;
            let o = id as usize * 6;
            for k in 0..3 {
                let lo = self.face_boxes[o + k] as f64;
                let hi = self.face_boxes[o + 3 + k] as f64;
                if lo < b[k] {
                    b[k] = lo;
                }
                if hi > b[k + 3] {
                    b[k + 3] = hi;
                }
            }
        }
        self.cells[cell].faces = faces;
        self.expand_aabb_loose(cell, b);
    }
    fn construct_children(&mut self, cell: usize) {
        let s = self.cells[cell].split;
        let (xcen, ycen, zcen) = (
            (s[3] + s[0]) * 0.5,
            (s[4] + s[1]) * 0.5,
            (s[5] + s[2]) * 0.5,
        );
        let depth = self.cells[cell].depth + 1;
        let first = self.cells.len();
        for _ in 0..8 {
            self.cells.push(Cell::new(Some(cell), depth));
        }
        let faces = std::mem::take(&mut self.cells[cell].faces);
        for &face in &faces {
            let id = face as usize * 3;
            let (cx, cy, cz) = (
                self.face_centers[id] as f64,
                self.face_centers[id + 1] as f64,
                self.face_centers[id + 2] as f64,
            );
            let child = if cx > xcen {
                if cy > ycen {
                    if cz > zcen { 6 } else { 5 }
                } else if cz > zcen {
                    2
                } else {
                    1
                }
            } else if cy > ycen {
                if cz > zcen { 7 } else { 4 }
            } else if cz > zcen {
                3
            } else {
                0
            };
            self.cells[first + child].faces.push(face);
        }
        let splits = [
            [s[0], s[1], s[2], xcen, ycen, zcen],
            [xcen, s[1], s[2], s[3], ycen, zcen],
            [xcen, s[1], zcen, s[3], ycen, s[5]],
            [s[0], s[1], zcen, xcen, ycen, s[5]],
            [s[0], ycen, s[2], xcen, s[4], zcen],
            [xcen, ycen, s[2], s[3], s[4], zcen],
            [xcen, ycen, zcen, s[3], s[4], s[5]],
            [s[0], ycen, zcen, xcen, s[4], s[5]],
        ];
        for (i, split) in splits.into_iter().enumerate() {
            self.cells[first + i].split = split;
        }
        self.cells[cell].children = (first..first + 8).collect();
    }
    fn expand_aabb_loose(&mut self, cell: usize, b: [f64; 6]) {
        let mut parent = Some(cell);
        while let Some(p) = parent {
            let loose = &mut self.cells[p].loose;
            let mut proceed = false;
            for k in 0..3 {
                if b[k] < loose[k] {
                    loose[k] = b[k];
                    proceed = true;
                }
            }
            for k in 3..6 {
                if b[k] > loose[k] {
                    loose[k] = b[k];
                    proceed = true;
                }
            }
            parent = if proceed { self.cells[p].parent } else { None };
        }
    }
    fn prune_if_possible(&mut self, start: usize) {
        let mut cell = start;
        while let Some(parent) = self.cells[cell].parent {
            if self.cells[parent].children.is_empty() {
                return;
            }
            for i in 0..8 {
                let child = &self.cells[self.cells[parent].children[i]];
                if !child.faces.is_empty() || child.children.len() == 8 {
                    return;
                }
            }
            self.cells[parent].children.clear();
            cell = parent;
        }
    }
    fn queue_leaf(&mut self, leaf: usize) {
        if self.cells[leaf].queued {
            return;
        }
        self.cells[leaf].queued = true;
        self.leaves_to_update.push(leaf);
    }
    fn set_face_leaf(&mut self, face: usize, leaf: Option<usize>) {
        if face >= self.face_leaf.len() {
            self.face_leaf.resize(face + 1, None);
        }
        self.face_leaf[face] = leaf;
    }
    fn collect_ray(&self, near: V3, eye: V3) -> Vec<u32> {
        let ir = [1.0 / eye[0], 1.0 / eye[1], 1.0 / eye[2]];
        let mut out = vec![];
        let mut stack = vec![0usize];
        while let Some(cell) = stack.pop() {
            let l = self.cells[cell].loose;
            let t: [f64; 6] = std::array::from_fn(|k| (l[k] - near[k % 3]) * ir[k % 3]);
            let tmin = js_max(
                js_max(js_min(t[0], t[3]), js_min(t[1], t[4])),
                js_min(t[2], t[5]),
            );
            let tmax = js_min(
                js_min(js_max(t[0], t[3]), js_max(t[1], t[4])),
                js_max(t[2], t[5]),
            );
            if tmax < 0. || tmin > tmax {
                continue;
            }
            let c = &self.cells[cell];
            if c.children.len() == 8 {
                stack.extend(c.children.iter().copied());
            } else {
                out.extend_from_slice(&c.faces);
            }
        }
        out
    }
    fn collect_sphere(&mut self, vert: V3, radius_squared: f64, collect_leaves: bool) -> Vec<u32> {
        let mut out = vec![];
        let mut stack = vec![0usize];
        while let Some(cell) = stack.pop() {
            let l = self.cells[cell].loose;
            let mut d = [0f64; 3];
            for k in 0..3 {
                if l[k] > vert[k] {
                    d[k] = l[k] - vert[k];
                } else if l[k + 3] < vert[k] {
                    d[k] = l[k + 3] - vert[k];
                }
            }
            if d[0] * d[0] + d[1] * d[1] + d[2] * d[2] > radius_squared {
                continue;
            }
            if self.cells[cell].children.len() == 8 {
                let children = self.cells[cell].children.clone();
                stack.extend(children);
            } else {
                if collect_leaves {
                    self.queue_leaf(cell);
                }
                out.extend_from_slice(&self.cells[cell].faces);
            }
        }
        out
    }
    fn add_face(&mut self, face: u32, b: [f64; 6], c: V3) -> Option<usize> {
        let mut stack = vec![0usize];
        while let Some(cell) = stack.pop() {
            let s = self.cells[cell].split;
            if c[0] <= s[0]
                || c[1] <= s[1]
                || c[2] <= s[2]
                || c[0] > s[3]
                || c[1] > s[4]
                || c[2] > s[5]
            {
                continue;
            }
            let loose = &mut self.cells[cell].loose;
            for k in 0..3 {
                if b[k] < loose[k] {
                    loose[k] = b[k];
                }
            }
            for k in 3..6 {
                if b[k] > loose[k] {
                    loose[k] = b[k];
                }
            }
            if self.cells[cell].children.len() == 8 {
                let children = self.cells[cell].children.clone();
                stack.extend(children);
            } else {
                self.cells[cell].faces.push(face);
                return Some(cell);
            }
        }
        None
    }
    fn face_box(&self, face: usize) -> [f64; 6] {
        std::array::from_fn(|k| self.face_boxes[face * 6 + k] as f64)
    }
    fn face_center(&self, face: usize) -> V3 {
        std::array::from_fn(|k| self.face_centers[face * 3 + k] as f64)
    }
    fn octree_remove(&mut self, faces: &[u32]) -> Vec<u32> {
        let mut moved = vec![];
        for &face in faces {
            let f = face as usize;
            let Some(leaf) = self.face_leaf.get(f).copied().flatten() else {
                moved.push(face);
                continue;
            };
            let s = self.cells[leaf].split;
            let c = self.face_center(f);
            if c[0] <= s[0]
                || c[1] <= s[1]
                || c[2] <= s[2]
                || c[0] > s[3]
                || c[1] > s[4]
                || c[2] > s[5]
            {
                moved.push(face);
                let position = self.face_pos_in_leaf[f] as usize;
                let list = &mut self.cells[leaf].faces;
                let last = list[list.len() - 1];
                list[position] = last;
                self.face_pos_in_leaf[last as usize] = position as u32;
                self.cells[leaf].faces.pop();
                self.queue_leaf(leaf);
            } else {
                let b = self.face_box(f);
                self.expand_aabb_loose(leaf, b);
            }
        }
        moved
    }
    fn octree_add(&mut self, faces: &[u32]) {
        for &face in faces {
            let f = face as usize;
            let (b, c) = (self.face_box(f), self.face_center(f));
            let Some(leaf) = self.add_face(face, b, c) else {
                self.compute_octree();
                return;
            };
            self.set_face_leaf(f, Some(leaf));
            self.face_pos_in_leaf[f] = (self.cells[leaf].faces.len() - 1) as u32;
            self.queue_leaf(leaf);
        }
    }
    fn vertices_from_faces(&mut self, faces: &[u32]) -> Vec<u32> {
        let tag = self.next_tag_flag();
        let mut out = vec![];
        for &face in faces {
            let ind = face as usize * 4;
            for k in 0..3 {
                let v = self.faces[ind + k];
                if self.vert_tag_flags[v as usize] != tag {
                    self.vert_tag_flags[v as usize] = tag;
                    out.push(v);
                }
            }
        }
        out
    }
    fn faces_from_vertices(&mut self, verts: &[u32]) -> Vec<u32> {
        let tag = self.next_tag_flag();
        let mut out = vec![];
        for &v in verts {
            for &face in &self.vring_face[v as usize] {
                if self.faces_tag_flags[face as usize] != tag {
                    self.faces_tag_flags[face as usize] = tag;
                    out.push(face);
                }
            }
        }
        out
    }
    fn expands_faces(&mut self, faces: &[u32], mut n_ring: u32) -> Vec<u32> {
        let tag = self.next_tag_flag();
        let mut out = faces.to_vec();
        for &f in faces {
            self.faces_tag_flags[f as usize] = tag;
        }
        let mut begin = 0;
        let mut nb = faces.len();
        while n_ring > 0 {
            n_ring -= 1;
            for i in begin..nb {
                let ind = out[i] as usize * 4;
                for j in 0..3 {
                    let v = self.faces[ind + j] as usize;
                    for &id in &self.vring_face[v] {
                        if self.faces_tag_flags[id as usize] == tag {
                            continue;
                        }
                        self.faces_tag_flags[id as usize] = tag;
                        out.push(id);
                    }
                }
            }
            begin = nb;
            nb = out.len();
        }
        out
    }
    fn expands_vertices(&mut self, verts: &[u32], mut n_ring: u32) -> Vec<u32> {
        let tag = self.next_tag_flag();
        let mut out = verts.to_vec();
        for &v in verts {
            self.vert_tag_flags[v as usize] = tag;
        }
        let mut begin = 0;
        let mut nb = verts.len();
        while n_ring > 0 {
            n_ring -= 1;
            for i in begin..nb {
                let ring = &self.vring_vert[out[i] as usize];
                for &id in ring {
                    if self.vert_tag_flags[id as usize] == tag {
                        continue;
                    }
                    self.vert_tag_flags[id as usize] = tag;
                    out.push(id);
                }
            }
            begin = nb;
            nb = out.len();
        }
        out
    }
    fn update_topology(&mut self, faces: &[u32], verts: &[u32]) {
        for &id in faces {
            let id = id as usize;
            for k in 0..3 {
                self.triangles[id * 3 + k] = self.faces[id * 4 + k];
            }
        }
        for &id in verts {
            let id = id as usize;
            self.vert_on_edge[id] =
                u8::from(self.vring_vert[id].len() != self.vring_face[id].len());
        }
    }
    fn resize<T: Copy + Default>(array: &mut Vec<T>, required: usize) {
        let mut resized = vec![T::default(); required * 2];
        let n = array.len().min(resized.len());
        resized[..n].copy_from_slice(&array[..n]);
        *array = resized;
    }
    /// reAllocateArrays( nbAddElements ).
    fn reallocate(&mut self, add: usize) {
        let capacity = self.faces.len() / 4;
        let required = self.nb_faces + add;
        if capacity < required || capacity > required * 4 {
            Self::resize(&mut self.faces, required * 4);
            Self::resize(&mut self.triangles, required * 3);
            Self::resize(&mut self.face_boxes, required * 6);
            Self::resize(&mut self.face_normals, required * 3);
            Self::resize(&mut self.face_centers, required * 3);
            Self::resize(&mut self.faces_tag_flags, required);
            Self::resize(&mut self.face_pos_in_leaf, required);
        }
        let capacity = self.vertices.len() / 3;
        let required = self.nb_vertices + add;
        if capacity < required || capacity > required * 4 {
            Self::resize(&mut self.vertices, required * 3);
            Self::resize(&mut self.normals, required * 3);
            Self::resize(&mut self.render_normals, required * 3);
            Self::resize(&mut self.vert_on_edge, required);
            Self::resize(&mut self.vert_tag_flags, required);
            Self::resize(&mut self.vert_sculpt_flags, required);
        }
    }
    pub(super) fn balance_octree(&mut self) {
        let leaves = std::mem::take(&mut self.leaves_to_update);
        for leaf in leaves {
            self.cells[leaf].queued = false;
            if self.cells[leaf].faces.is_empty() {
                self.prune_if_possible(leaf);
            } else if self.cells[leaf].faces.len() > OCTREE_MAX_FACES
                && self.cells[leaf].depth < OCTREE_MAX_DEPTH
            {
                self.build(leaf);
            }
        }
    }
    fn set_ring(rings: &mut Vec<Vec<u32>>, index: usize, ring: Vec<u32>) {
        if index >= rings.len() {
            rings.resize(index + 1, vec![]);
        }
        rings[index] = ring;
    }
    fn ring_mut(rings: &mut Vec<Vec<u32>>, index: usize) -> &mut Vec<u32> {
        if index >= rings.len() {
            rings.resize(index + 1, vec![]);
        }
        &mut rings[index]
    }

    // -------------------------------------------------------- subdivision

    fn edge_key(sub: &Sub, a: u32, b: u32) -> u64 {
        let (low, high) = (a.min(b) as u64, a.max(b) as u64);
        low * sub.edge_key_stride + high
    }
    fn sub_fill_triangle(&mut self, tri: u32, iv1: u32, iv2: u32, iv3: u32, mid: u32) {
        let j = tri as usize * 4;
        self.faces[j..j + 4].copy_from_slice(&[iv1, mid, iv3, TRI_INDEX]);
        let leaf = self.face_leaf[tri as usize].unwrap_or(0);
        Self::ring_mut(&mut self.vring_vert, mid as usize).push(iv3);
        Self::ring_mut(&mut self.vring_vert, iv3 as usize).push(mid);
        let new_tri = self.nb_faces as u32;
        Self::ring_mut(&mut self.vring_face, mid as usize).extend([tri, new_tri]);
        let j = new_tri as usize * 4;
        self.faces[j..j + 4].copy_from_slice(&[mid, iv2, iv3, TRI_INDEX]);
        self.set_face_leaf(new_tri as usize, Some(leaf));
        self.face_pos_in_leaf[new_tri as usize] = self.cells[leaf].faces.len() as u32;
        Self::ring_mut(&mut self.vring_face, iv3 as usize).push(new_tri);
        replace_element(&mut self.vring_face[iv2 as usize], tri, new_tri);
        self.cells[leaf].faces.push(new_tri);
        self.add_nb_face(1);
    }
    fn sub_fill_triangles(&mut self, sub: &Sub, tris: &[u32]) -> Vec<u32> {
        let mut next = vec![];
        for &tri in tris {
            let j = tri as usize * 4;
            let (iv1, iv2, iv3) = (self.faces[j], self.faces[j + 1], self.faces[j + 2]);
            // A vertex index of 0 reads as no split, as the addon's truthiness test does.
            let get = |a: u32, b: u32| {
                sub.vertices_map
                    .get(&Self::edge_key(sub, a, b))
                    .copied()
                    .filter(|&v| v != 0)
            };
            let (val1, val2, val3) = (get(iv1, iv2), get(iv2, iv3), get(iv1, iv3));
            let num = |v: u32| self.vring_vert[v as usize].len();
            let (num1, num2, num3) = (num(iv1), num(iv2), num(iv3));
            let split = if val1.is_some() {
                if val2.is_some() {
                    if val3.is_some() {
                        if num1 < num2 && num1 < num3 {
                            2
                        } else if num2 < num3 {
                            3
                        } else {
                            1
                        }
                    } else if num1 < num3 {
                        2
                    } else {
                        1
                    }
                } else if val3.is_some() && num2 < num3 {
                    3
                } else {
                    1
                }
            } else if val2.is_some() {
                if val3.is_some() && num2 < num1 { 3 } else { 2 }
            } else if val3.is_some() {
                3
            } else {
                0
            };
            match split {
                1 => self.sub_fill_triangle(tri, iv1, iv2, iv3, val1.unwrap_or(0)),
                2 => self.sub_fill_triangle(tri, iv2, iv3, iv1, val2.unwrap_or(0)),
                3 => self.sub_fill_triangle(tri, iv3, iv1, iv2, val3.unwrap_or(0)),
                _ => continue,
            }
            next.push(tri);
            next.push(self.nb_faces as u32 - 1);
        }
        next
    }
    fn half_edge_split(&mut self, sub: &mut Sub, tri: u32, iv1: u32, iv2: u32, iv3: u32) {
        let key = Self::edge_key(sub, iv1, iv2);
        let (mid, is_new) = match sub.vertices_map.get(&key) {
            Some(&m) => (m, false),
            None => {
                let m = self.nb_vertices as u32;
                sub.vertices_map.insert(key, m);
                (m, true)
            }
        };
        Self::ring_mut(&mut self.vring_vert, iv3 as usize).push(mid);
        let id = tri as usize * 4;
        self.faces[id..id + 4].copy_from_slice(&[iv1, mid, iv3, TRI_INDEX]);
        let new_tri = self.nb_faces as u32;
        let id = new_tri as usize * 4;
        self.faces[id..id + 4].copy_from_slice(&[mid, iv2, iv3, TRI_INDEX]);
        Self::ring_mut(&mut self.vring_face, iv3 as usize).push(new_tri);
        replace_element(&mut self.vring_face[iv2 as usize], tri, new_tri);
        let leaf = self.face_leaf[tri as usize].unwrap_or(0);
        self.set_face_leaf(new_tri as usize, Some(leaf));
        self.face_pos_in_leaf[new_tri as usize] = self.cells[leaf].faces.len() as u32;
        self.cells[leaf].faces.push(new_tri);
        if !is_new {
            Self::ring_mut(&mut self.vring_vert, mid as usize).push(iv3);
            Self::ring_mut(&mut self.vring_face, mid as usize).extend([tri, new_tri]);
            self.add_nb_face(1);
            return;
        }
        let (id1, id2) = (iv1 as usize * 3, iv2 as usize * 3);
        let r = |a: &Vec<f32>, i: usize| a[i] as f64;
        let (v1x, v1y, v1z) = (
            r(&self.vertices, id1),
            r(&self.vertices, id1 + 1),
            r(&self.vertices, id1 + 2),
        );
        let (n1x, n1y, n1z) = (
            r(&self.normals, id1),
            r(&self.normals, id1 + 1),
            r(&self.normals, id1 + 2),
        );
        let (v2x, v2y, v2z) = (
            r(&self.vertices, id2),
            r(&self.vertices, id2 + 1),
            r(&self.vertices, id2 + 2),
        );
        let (n2x, n2y, n2z) = (
            r(&self.normals, id2),
            r(&self.normals, id2 + 1),
            r(&self.normals, id2 + 2),
        );
        let (n1n2x, n1n2y, n1n2z) = (n1x + n2x, n1y + n2y, n1z + n2z);
        let id = mid as usize * 3;
        self.normals[id] = (n1n2x * 0.5) as f32;
        self.normals[id + 1] = (n1n2y * 0.5) as f32;
        self.normals[id + 2] = (n1n2z * 0.5) as f32;
        let unit = |x: f64, y: f64, z: f64| {
            let len = x * x + y * y + z * z;
            if len == 0. {
                (1., y, z)
            } else {
                let len = 1. / len.sqrt();
                (x * len, y * len, z * len)
            }
        };
        let (nn1x, nn1y, nn1z) = unit(n1x, n1y, n1z);
        let (nn2x, nn2y, nn2z) = unit(n2x, n2y, n2z);
        let d = nn1x * nn2x + nn1y * nn2y + nn1z * nn2z;
        let angle = if d <= -1. {
            std::f64::consts::PI
        } else if d >= 1. {
            0.
        } else {
            d.acos()
        };
        let (ex, ey, ez) = (v1x - v2x, v1y - v2y, v1z - v2z);
        let mut offset = angle * 0.12 * (ex * ex + ey * ey + ez * ez).sqrt();
        let len = n1n2x * n1n2x + n1n2y * n1n2y + n1n2z * n1n2z;
        if len > 0. {
            offset /= len.sqrt();
        }
        if (ex * (nn1x - nn2x) + ey * (nn1y - nn2y) + ez * (nn1z - nn2z)) < 0. {
            offset = -offset;
        }
        self.vertices[id] = ((v1x + v2x) * 0.5 + n1n2x * offset) as f32;
        self.vertices[id + 1] = ((v1y + v2y) * 0.5 + n1n2y * offset) as f32;
        self.vertices[id + 2] = ((v1z + v2z) * 0.5 + n1n2z * offset) as f32;
        Self::set_ring(&mut self.vring_vert, mid as usize, vec![iv1, iv2, iv3]);
        Self::set_ring(&mut self.vring_face, mid as usize, vec![tri, new_tri]);
        replace_element(&mut self.vring_vert[iv1 as usize], iv2, mid);
        replace_element(&mut self.vring_vert[iv2 as usize], iv1, mid);
        self.add_nb_vertice(1);
        self.add_nb_face(1);
    }
    fn sub_find_split(&self, sub: &Sub, tri: u32, check_inside: bool) -> u8 {
        let id = tri as usize * 4;
        let (v1, v2, v3) = (
            self.v(self.faces[id]),
            self.v(self.faces[id + 1]),
            self.v(self.faces[id + 2]),
        );
        if check_inside && !triangle_inside_sphere(sub.center, sub.radius2, v1, v2, v3) {
            return 0;
        }
        let (l1, l2, l3) = (sqr_dist(v1, v2), sqr_dist(v2, v3), sqr_dist(v1, v3));
        if l1 > l2 && l1 > l3 {
            if l1 > sub.edge_max2 { 1 } else { 0 }
        } else if l2 > l3 {
            if l2 > sub.edge_max2 { 2 } else { 0 }
        } else if l3 > sub.edge_max2 {
            3
        } else {
            0
        }
    }
    fn subdivide(&mut self, sub: &mut Sub, tris: Vec<u32>) -> Vec<u32> {
        let nb_verts_init = self.nb_vertices;
        let nb_tris_init = self.nb_faces;
        sub.vertices_map.clear();
        let mut subd = vec![];
        let mut split_arr = vec![];
        for &tri in &tris {
            let split = self.sub_find_split(sub, tri, true);
            if split == 0 {
                continue;
            }
            split_arr.push(split);
            subd.push(tri);
        }
        if subd.is_empty() {
            self.reallocate(0);
            return tris;
        }
        if subd.len() > 5 {
            subd = self.expands_faces(&subd, 3);
            split_arr.resize(subd.len(), 0);
        }
        sub.edge_key_stride = (self.nb_vertices + subd.len() + 1) as u64;
        self.reallocate(split_arr.len());
        for (i, &tri) in subd.iter().enumerate() {
            let mut split = split_arr[i];
            if split == 0 {
                split = self.sub_find_split(sub, tri, false);
            }
            let ind = tri as usize * 4;
            let (a, b, c) = (self.faces[ind], self.faces[ind + 1], self.faces[ind + 2]);
            match split {
                1 => self.half_edge_split(sub, tri, a, b, c),
                2 => self.half_edge_split(sub, tri, b, c, a),
                3 => self.half_edge_split(sub, tri, c, a, b),
                _ => {}
            }
        }
        let nb_new = self.nb_faces - nb_tris_init;
        let mut new_triangles: Vec<u32> = (0..nb_new).map(|i| (nb_tris_init + i) as u32).collect();
        new_triangles = self.expands_faces(&new_triangles, 1);
        let mut all = tris;
        all.extend_from_slice(&new_triangles);
        let tag = self.next_tag_flag();
        let mut result = vec![];
        for &tri in &all {
            if self.faces_tag_flags[tri as usize] == tag {
                continue;
            }
            self.faces_tag_flags[tri as usize] = tag;
            result.push(tri);
        }
        let nb_old = self.nb_faces;
        while !new_triangles.is_empty() {
            self.reallocate(new_triangles.len());
            new_triangles = self.sub_fill_triangles(sub, &new_triangles);
        }
        let added = self.nb_faces - nb_old;
        result.extend((0..added).map(|i| (nb_old + i) as u32));
        let nb_v_new = self.nb_vertices - nb_verts_init;
        let v_new: Vec<u32> = (0..nb_v_new).map(|i| (nb_verts_init + i) as u32).collect();
        let v_new = self.expands_vertices(&v_new, 1);
        let expanded = v_new[nb_v_new..].to_vec();
        self.smooth_tangent_verts(&expanded, 1.0);
        let mask = self.sculpt_flag;
        for &ind in &v_new {
            let v = self.v(ind);
            let d = sqr_dist(v, sub.center);
            self.vert_sculpt_flags[ind as usize] = if d < sub.radius2 { mask } else { mask - 1 };
        }
        result
    }
    fn subdivision_pass(
        &mut self,
        tris: Vec<u32>,
        center: V3,
        radius2: f64,
        detail2: f64,
    ) -> Vec<u32> {
        let mut sub = Sub {
            vertices_map: HashMap::new(),
            edge_key_stride: 0,
            center,
            radius2,
            edge_max2: detail2,
        };
        let mut tris = tris;
        let mut nb = usize::MAX;
        while nb != self.nb_faces {
            nb = self.nb_faces;
            tris = self.subdivide(&mut sub, tris);
        }
        tris
    }

    // -------------------------------------------------------- decimation

    fn dec_delete_triangle(&mut self, tri: u32) {
        let t = tri as usize;
        let old = self.face_pos_in_leaf[t] as usize;
        let leaf = self.face_leaf[t].unwrap_or(0);
        let last_tri = *self.cells[leaf].faces.last().unwrap_or(&tri);
        if tri != last_tri {
            self.cells[leaf].faces[old] = last_tri;
            self.face_pos_in_leaf[last_tri as usize] = old as u32;
        }
        self.cells[leaf].faces.pop();
        let last = self.nb_faces - 1;
        if last == t {
            self.face_leaf.truncate(last);
            self.add_nb_face(-1);
            return;
        }
        let id = last * 4;
        let (iv1, iv2, iv3) = (self.faces[id], self.faces[id + 1], self.faces[id + 2]);
        for v in [iv1, iv2, iv3] {
            replace_element(&mut self.vring_face[v as usize], last as u32, tri);
        }
        let leaf_last = self.face_leaf[last].unwrap_or(0);
        let pil_last = self.face_pos_in_leaf[last];
        self.cells[leaf_last].faces[pil_last as usize] = tri;
        self.face_leaf[t] = Some(leaf_last);
        self.face_pos_in_leaf[t] = pil_last;
        self.faces_tag_flags[t] = self.faces_tag_flags[last];
        let j = t * 4;
        self.faces[j..j + 4].copy_from_slice(&[iv1, iv2, iv3, TRI_INDEX]);
        self.face_leaf.truncate(last);
        self.decimated.extend([iv1, iv2, iv3]);
        self.add_nb_face(-1);
    }
    fn dec_delete_vertex(&mut self, vert: u32) {
        let v = vert as usize;
        let last = self.nb_vertices - 1;
        if v == last {
            self.vring_vert.truncate(last);
            self.vring_face.truncate(last);
            self.add_nb_vertice(-1);
            return;
        }
        let tris = self.vring_face[last].clone();
        let ring = self.vring_vert[last].clone();
        for &t in &tris {
            let id = t as usize * 4;
            if self.faces[id] == last as u32 {
                self.faces[id] = vert;
            } else if self.faces[id + 1] == last as u32 {
                self.faces[id + 1] = vert;
            } else {
                self.faces[id + 2] = vert;
            }
        }
        for &r in &ring {
            replace_element(&mut self.vring_vert[r as usize], last as u32, vert);
        }
        self.vring_vert[v] = self.vring_vert[last].clone();
        self.vring_face[v] = self.vring_face[last].clone();
        self.vert_tag_flags[v] = self.vert_tag_flags[last];
        self.vert_sculpt_flags[v] = self.vert_sculpt_flags[last];
        for k in 0..3 {
            self.vertices[v * 3 + k] = self.vertices[last * 3 + k];
            self.normals[v * 3 + k] = self.normals[last * 3 + k];
        }
        self.vring_vert.truncate(last);
        self.vring_face.truncate(last);
        self.add_nb_vertice(-1);
    }
    #[allow(clippy::too_many_arguments)]
    fn dec_edge_collapse(
        &mut self,
        tri1: u32,
        tri2: u32,
        iv1: u32,
        iv2: u32,
        opp1: u32,
        opp2: u32,
        tris: &mut Vec<u32>,
    ) {
        let (i1, i2, o1, o2) = (iv1 as usize, iv2 as usize, opp1 as usize, opp2 as usize);
        if self.vring_vert[i1].len() != self.vring_face[i1].len()
            || self.vring_vert[i2].len() != self.vring_face[i2].len()
        {
            return;
        }
        if self.vring_vert[o1].len() != self.vring_face[o1].len()
            || self.vring_vert[o2].len() != self.vring_face[o2].len()
        {
            return;
        }
        if self.vring_vert[i1].len() == 3 && self.vring_vert[i2].len() == 3 {
            return;
        }
        self.vring_vert[i1].sort_unstable();
        self.vring_vert[i2].sort_unstable();
        if three_common(&self.vring_vert[i1], &self.vring_vert[i2]) {
            if self.vring_vert[o1].contains(&opp2) {
                return;
            }
            self.decimated.extend([iv1, iv2]);
            remove_element(&mut self.vring_face[i1], tri2);
            remove_element(&mut self.vring_face[i2], tri1);
            self.vring_face[o1].push(tri2);
            self.vring_face[o2].push(tri1);
            let id = tri1 as usize * 4;
            if self.faces[id] == iv2 {
                self.faces[id] = opp2;
            } else if self.faces[id + 1] == iv2 {
                self.faces[id + 1] = opp2;
            } else {
                self.faces[id + 2] = opp2;
            }
            let id = tri2 as usize * 4;
            if self.faces[id] == iv1 {
                self.faces[id] = opp1;
            } else if self.faces[id + 1] == iv1 {
                self.faces[id + 1] = opp1;
            } else {
                self.faces[id + 2] = opp1;
            }
            self.compute_ring_vertices(iv1);
            self.compute_ring_vertices(iv2);
            self.compute_ring_vertices(opp1);
            self.compute_ring_vertices(opp2);
            self.topology_version += 1;
            return;
        }
        self.decimated.extend([iv1, iv2]);
        let (id, id2) = (i1 * 3, i2 * 3);
        let n = |k: usize, o: usize| self.normals[o + k] as f64;
        let (mut nx, mut ny, mut nz) = (
            n(0, id) + n(0, id2),
            n(1, id) + n(1, id2),
            n(2, id) + n(2, id2),
        );
        let len = nx * nx + ny * ny + nz * nz;
        if len == 0. {
            nx = 1.;
        } else {
            let len = 1. / len.sqrt();
            nx *= len;
            ny *= len;
            nz *= len;
        }
        self.normals[id] = nx as f32;
        self.normals[id + 1] = ny as f32;
        self.normals[id + 2] = nz as f32;
        remove_element(&mut self.vring_face[i1], tri1);
        remove_element(&mut self.vring_face[i1], tri2);
        remove_element(&mut self.vring_face[i2], tri1);
        remove_element(&mut self.vring_face[i2], tri2);
        remove_element(&mut self.vring_face[o1], tri1);
        remove_element(&mut self.vring_face[o2], tri2);
        let tris2 = self.vring_face[i2].clone();
        for &t2 in &tris2 {
            self.vring_face[i1].push(t2);
            let idx = t2 as usize * 4;
            if self.faces[idx] == iv2 {
                self.faces[idx] = iv1;
            } else if self.faces[idx + 1] == iv2 {
                self.faces[idx + 1] = iv1;
            } else {
                self.faces[idx + 2] = iv1;
            }
        }
        let ring2 = self.vring_vert[i2].clone();
        self.vring_vert[i1].extend(ring2);
        self.compute_ring_vertices(iv1);
        let (mut mx, mut my, mut mz) = (0f64, 0f64, 0f64);
        let ring1 = self.vring_vert[i1].clone();
        let nb_ring1 = ring1.len();
        for &r in &ring1 {
            self.compute_ring_vertices(r);
            let p = self.v(r);
            mx += p[0];
            my += p[1];
            mz += p[2];
        }
        mx /= nb_ring1 as f64;
        my /= nb_ring1 as f64;
        mz /= nb_ring1 as f64;
        let p = self.v(iv1);
        let dot_n = nx * (mx - p[0]) + ny * (my - p[1]) + nz * (mz - p[2]);
        self.vertices[id] = (mx - nx * dot_n) as f32;
        self.vertices[id + 1] = (my - ny * dot_n) as f32;
        self.vertices[id + 2] = (mz - nz * dot_n) as f32;
        self.vert_tag_flags[i2] = -1;
        self.faces_tag_flags[tri1 as usize] = -1;
        self.faces_tag_flags[tri2 as usize] = -1;
        self.verts_to_delete.push(iv2);
        self.tris_to_delete.extend([tri1, tri2]);
        tris.extend_from_slice(&self.vring_face[i1]);
    }
    fn dec_decimate_triangles(&mut self, tri1: u32, tri2: i64, tris: &mut Vec<u32>) {
        if tri2 == -1 {
            return;
        }
        let tri2 = tri2 as u32;
        let (id1, id2) = (tri1 as usize * 4, tri2 as usize * 4);
        let (v11, v21, v31) = (self.faces[id1], self.faces[id1 + 1], self.faces[id1 + 2]);
        let (v12, v22, v32) = (self.faces[id2], self.faces[id2 + 1], self.faces[id2 + 2]);
        if v11 == v12 {
            if v21 == v32 {
                self.dec_edge_collapse(tri1, tri2, v11, v21, v31, v22, tris);
            } else {
                self.dec_edge_collapse(tri1, tri2, v11, v31, v21, v32, tris);
            }
        } else if v11 == v22 {
            if v21 == v12 {
                self.dec_edge_collapse(tri1, tri2, v11, v21, v31, v32, tris);
            } else {
                self.dec_edge_collapse(tri1, tri2, v11, v31, v21, v12, tris);
            }
        } else if v11 == v32 {
            if v21 == v22 {
                self.dec_edge_collapse(tri1, tri2, v11, v21, v31, v12, tris);
            } else {
                self.dec_edge_collapse(tri1, tri2, v11, v31, v21, v22, tris);
            }
        } else if v21 == v12 {
            self.dec_edge_collapse(tri1, tri2, v31, v21, v11, v22, tris);
        } else if v21 == v22 {
            self.dec_edge_collapse(tri1, tri2, v31, v21, v11, v32, tris);
        } else {
            self.dec_edge_collapse(tri1, tri2, v31, v21, v11, v12, tris);
        }
    }
    fn dec_find_opposite(&self, tri: u32, iv1: u32, iv2: u32) -> i64 {
        let (t1, t2) = (
            &self.vring_face[iv1 as usize],
            &self.vring_face[iv2 as usize],
        );
        let mut count = 0;
        let mut opposite = -1i64;
        for &candidate in t1 {
            if t2.contains(&candidate) {
                count += 1;
                if candidate != tri {
                    opposite = candidate as i64;
                }
            }
        }
        if count == 2 { opposite } else { -1 }
    }
    fn decimation_pass(
        &mut self,
        tris: Vec<u32>,
        center: V3,
        radius2: f64,
        detail2: f64,
    ) -> Vec<u32> {
        self.decimated.clear();
        self.tris_to_delete.clear();
        self.verts_to_delete.clear();
        let radius = radius2.sqrt();
        let mut dyn_arr = tris;
        let mut i = 0;
        while i < dyn_arr.len() {
            let tri = dyn_arr[i];
            i += 1;
            if self.faces_tag_flags[tri as usize] < 0 {
                continue;
            }
            let id = tri as usize * 4;
            let (iv1, iv2, iv3) = (self.faces[id], self.faces[id + 1], self.faces[id + 2]);
            let (v1, v2, v3) = (self.v(iv1), self.v(iv2), self.v(iv3));
            let dx = (v1[0] + v2[0] + v3[0]) / 3.0 - center[0];
            let dy = (v1[1] + v2[1] + v3[1]) / 3.0 - center[1];
            let dz = (v1[2] + v2[2] + v3[2]) / 3.0 - center[2];
            let mut fall = dx * dx + dy * dy + dz * dz;
            if fall < radius2 {
                fall = 1.0;
            } else if fall < radius2 * 2.0 {
                fall = (fall.sqrt() - radius) / (radius * std::f64::consts::SQRT_2 - radius);
                let f2 = fall * fall;
                fall = 3.0 * f2 * f2 - 4.0 * f2 * fall + 1.0;
            } else {
                continue;
            }
            let len1 = sqr_dist(v2, v1);
            let len2 = sqr_dist(v2, v3);
            let len3 = sqr_dist(v1, v3);
            if len1 < len2 && len1 < len3 {
                if len1 < detail2 * fall {
                    let opp = self.dec_find_opposite(tri, iv1, iv2);
                    self.dec_decimate_triangles(tri, opp, &mut dyn_arr);
                }
            } else if len2 < len3 {
                if len2 < detail2 * fall {
                    let opp = self.dec_find_opposite(tri, iv2, iv3);
                    self.dec_decimate_triangles(tri, opp, &mut dyn_arr);
                }
            } else if len3 < detail2 * fall {
                let opp = self.dec_find_opposite(tri, iv1, iv3);
                self.dec_decimate_triangles(tri, opp, &mut dyn_arr);
            }
        }
        let mut delete = std::mem::take(&mut self.tris_to_delete);
        tidy(&mut delete);
        for &t in delete.iter().rev() {
            self.dec_delete_triangle(t);
        }
        let mut delete = std::mem::take(&mut self.verts_to_delete);
        tidy(&mut delete);
        for &v in delete.iter().rev() {
            self.dec_delete_vertex(v);
        }
        let decimated = std::mem::take(&mut self.decimated);
        let nb_vertices = self.nb_vertices;
        let tag = self.next_tag_flag();
        let mut valid = vec![];
        for &v in &decimated {
            if v as usize >= nb_vertices || self.vert_tag_flags[v as usize] == tag {
                continue;
            }
            self.vert_tag_flags[v as usize] = tag;
            valid.push(v);
        }
        let new_tris = self.faces_from_vertices(&valid);
        let tag = self.next_tag_flag();
        let nb_triangles = self.nb_faces;
        let mut out = vec![];
        for &t in dyn_arr.iter().chain(&new_tris) {
            if t as usize >= nb_triangles || self.faces_tag_flags[t as usize] == tag {
                continue;
            }
            self.faces_tag_flags[t as usize] = tag;
            out.push(t);
        }
        out
    }

    // -------------------------------------------------------- smoothing

    /// laplacianSmooth into a Float32Array of the verts' averages.
    fn laplacian_smooth(&self, verts: &[u32]) -> Vec<f32> {
        let mut out = vec![0f32; verts.len() * 3];
        for (i, &id) in verts.iter().enumerate() {
            let ring = &self.vring_vert[id as usize];
            let count = ring.len();
            if count <= 2 {
                let p = self.v(id);
                for k in 0..3 {
                    out[i * 3 + k] = p[k] as f32;
                }
                continue;
            }
            let mut av = [0f64; 3];
            if self.vert_on_edge[id as usize] == 1 {
                let mut nb = 0;
                for &r in ring {
                    if self.vert_on_edge[r as usize] == 1 {
                        let p = self.v(r);
                        for k in 0..3 {
                            av[k] += p[k];
                        }
                        nb += 1;
                    }
                }
                if nb >= 2 {
                    for k in 0..3 {
                        out[i * 3 + k] = (av[k] / nb as f64) as f32;
                    }
                    continue;
                }
                av = [0.; 3];
            }
            for &r in ring {
                let p = self.v(r);
                for k in 0..3 {
                    av[k] += p[k];
                }
            }
            for k in 0..3 {
                out[i * 3 + k] = (av[k] / count as f64) as f32;
            }
        }
        out
    }
    fn smooth_tangent_verts(&mut self, verts: &[u32], strength: f64) {
        let intensity = js_min(strength, 1.0);
        let smooth = self.laplacian_smooth(verts);
        for (i, &v) in verts.iter().enumerate() {
            let ind = v as usize * 3;
            let p = self.v(v);
            let (mut nx, mut ny, mut nz) = (
                self.normals[ind] as f64,
                self.normals[ind + 1] as f64,
                self.normals[ind + 2] as f64,
            );
            let len = nx * nx + ny * ny + nz * nz;
            if len == 0. {
                continue;
            }
            let len = 1. / len.sqrt();
            nx *= len;
            ny *= len;
            nz *= len;
            let s = [
                smooth[i * 3] as f64,
                smooth[i * 3 + 1] as f64,
                smooth[i * 3 + 2] as f64,
            ];
            let d = nx * (s[0] - p[0]) + ny * (s[1] - p[1]) + nz * (s[2] - p[2]);
            self.vertices[ind] = (p[0] + (s[0] - nx * d - p[0]) * intensity) as f32;
            self.vertices[ind + 1] = (p[1] + (s[1] - ny * d - p[1]) * intensity) as f32;
            self.vertices[ind + 2] = (p[2] + (s[2] - nz * d - p[2]) * intensity) as f32;
        }
    }
}
/// hasAtLeastThreeCommonElements of two sorted rings.
fn three_common(a: &[u32], b: &[u32]) -> bool {
    let (mut ai, mut bi, mut count) = (0, 0, 0);
    while ai < a.len() && bi < b.len() {
        if a[ai] < b[bi] {
            ai += 1;
        } else if a[ai] > b[bi] {
            bi += 1;
        } else {
            count += 1;
            if count == 3 {
                return true;
            }
            ai += 1;
            bi += 1;
        }
    }
    false
}

// ---------------------------------------------------------------- tools

/// The sculpting tools, in the GUI's order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Tool {
    Clay,
    Brush,
    Inflate,
    Smooth,
    Flatten,
    Pinch,
    Crease,
    Drag,
    Scale,
}
pub(super) const TOOLS: [Tool; 9] = [
    Tool::Clay,
    Tool::Brush,
    Tool::Inflate,
    Tool::Smooth,
    Tool::Flatten,
    Tool::Pinch,
    Tool::Crease,
    Tool::Drag,
    Tool::Scale,
];
/// TOOL_DEFAULTS: size, strength and negative.
fn tool_defaults(tool: Tool) -> (f64, f64, bool) {
    match tool {
        Tool::Clay | Tool::Brush | Tool::Scale => (50., 0.5, false),
        Tool::Inflate => (50., 0.3, false),
        Tool::Smooth | Tool::Pinch => (50., 0.75, false),
        Tool::Flatten => (50., 0.75, true),
        Tool::Crease => (25., 0.75, true),
        Tool::Drag => (150., 0.5, false),
    }
}
impl Mesh {
    fn front_vertices(&self, verts: &[u32], eye: V3) -> Vec<u32> {
        verts
            .iter()
            .copied()
            .filter(|&id| {
                let j = id as usize * 3;
                self.normals[j] as f64 * eye[0]
                    + self.normals[j + 1] as f64 * eye[1]
                    + self.normals[j + 2] as f64 * eye[2]
                    <= 0.
            })
            .collect()
    }
    fn area_normal(&self, verts: &[u32]) -> Option<V3> {
        let mut a = [0f64; 3];
        for &v in verts {
            for (k, a) in a.iter_mut().enumerate() {
                *a += self.normals[v as usize * 3 + k] as f64;
            }
        }
        let len = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        if len == 0. {
            return None;
        }
        let inv = 1.0 / len;
        Some([a[0] * inv, a[1] * inv, a[2] * inv])
    }
    fn area_center(&self, verts: &[u32]) -> V3 {
        let mut a = [0f64; 3];
        for &v in verts {
            let p = self.v(v);
            for (a, p) in a.iter_mut().zip(p) {
                *a += p;
            }
        }
        let n = verts.len() as f64;
        [a[0] / n, a[1] / n, a[2] / n]
    }
    fn set_v(&mut self, v: u32, p: [f64; 3]) {
        let o = v as usize * 3;
        for (k, p) in p.iter().enumerate() {
            self.vertices[o + k] = *p as f32;
        }
    }
    fn tool_brush(
        &mut self,
        verts: &[u32],
        normal: V3,
        center: V3,
        radius_sq: f64,
        strength: f64,
        negative: bool,
    ) {
        let radius = radius_sq.sqrt();
        let mut deform = strength * radius * 0.1;
        if negative {
            deform = -deform;
        }
        for &v in verts {
            let p = self.v(v);
            let d = sub(p, center);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            if dist >= 1.0 {
                continue;
            }
            let f = falloff(dist) * deform;
            self.set_v(
                v,
                [
                    p[0] + normal[0] * f,
                    p[1] + normal[1] * f,
                    p[2] + normal[2] * f,
                ],
            );
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn tool_flatten(
        &mut self,
        verts: &[u32],
        normal: V3,
        plane: V3,
        center: V3,
        radius_sq: f64,
        strength: f64,
        negative: bool,
    ) {
        let radius = radius_sq.sqrt();
        let comp = if negative { -1. } else { 1. };
        for &v in verts {
            let p = self.v(v);
            let dist_to_plane = (p[0] - plane[0]) * normal[0]
                + (p[1] - plane[1]) * normal[1]
                + (p[2] - plane[2]) * normal[2];
            if dist_to_plane * comp > 0. {
                continue;
            }
            let d = sub(p, center);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            if dist >= 1.0 {
                continue;
            }
            let f = falloff(dist) * dist_to_plane * strength;
            self.set_v(
                v,
                [
                    p[0] - normal[0] * f,
                    p[1] - normal[1] * f,
                    p[2] - normal[2] * f,
                ],
            );
        }
    }
    fn tool_inflate(
        &mut self,
        verts: &[u32],
        center: V3,
        radius_sq: f64,
        strength: f64,
        negative: bool,
    ) {
        let radius = radius_sq.sqrt();
        let mut deform = strength * radius * 0.1;
        if negative {
            deform = -deform;
        }
        for &v in verts {
            let p = self.v(v);
            let d = sub(p, center);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            if dist >= 1.0 {
                continue;
            }
            let mut f = falloff(dist) * deform;
            let o = v as usize * 3;
            let n = [
                self.normals[o] as f64,
                self.normals[o + 1] as f64,
                self.normals[o + 2] as f64,
            ];
            let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if n_len > 0. {
                f /= n_len;
            }
            self.set_v(v, [p[0] + n[0] * f, p[1] + n[1] * f, p[2] + n[2] * f]);
        }
    }
    fn tool_smooth(&mut self, verts: &[u32], strength: f64) {
        let intensity = js_min(strength, 1.0);
        let comp = 1.0 - intensity;
        let smooth = self.laplacian_smooth(verts);
        for (i, &v) in verts.iter().enumerate() {
            let p = self.v(v);
            self.set_v(
                v,
                [0, 1, 2].map(|k| p[k] * comp + smooth[i * 3 + k] as f64 * intensity),
            );
        }
    }
    fn tool_pinch(
        &mut self,
        verts: &[u32],
        center: V3,
        radius_sq: f64,
        strength: f64,
        negative: bool,
    ) {
        let radius = radius_sq.sqrt();
        let mut deform = strength * 0.05;
        if negative {
            deform = -deform;
        }
        for &v in verts {
            let p = self.v(v);
            let d = sub(center, p);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            let f = falloff(dist) * deform;
            self.set_v(v, [p[0] + d[0] * f, p[1] + d[1] * f, p[2] + d[2] * f]);
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn tool_crease(
        &mut self,
        verts: &[u32],
        normal: V3,
        center: V3,
        radius_sq: f64,
        strength: f64,
        negative: bool,
    ) {
        let radius = radius_sq.sqrt();
        let deform = strength * 0.07;
        let mut brush = deform * radius;
        if negative {
            brush = -brush;
        }
        for &v in verts {
            let p = self.v(v);
            let d = sub(center, p);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            if dist >= 1.0 {
                continue;
            }
            let f = falloff(dist);
            let brush_mod = f.powf(5.) * brush;
            let pinch = f * deform;
            self.set_v(
                v,
                [0, 1, 2].map(|k| p[k] + d[k] * pinch + normal[k] * brush_mod),
            );
        }
    }
    fn tool_drag(&mut self, verts: &[u32], center: V3, radius_sq: f64, dir: V3) {
        let radius = radius_sq.sqrt();
        for &v in verts {
            let p = self.v(v);
            let d = sub(p, center);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            let f = falloff(dist);
            self.set_v(v, [0, 1, 2].map(|k| p[k] + dir[k] * f));
        }
    }
    fn tool_scale(&mut self, verts: &[u32], center: V3, radius_sq: f64, delta: f64) {
        let radius = radius_sq.sqrt();
        let scale = delta * 0.01;
        for &v in verts {
            let p = self.v(v);
            let d = sub(p, center);
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() / radius;
            let f = falloff(dist) * scale;
            self.set_v(v, [0, 1, 2].map(|k| p[k] + d[k] * f));
        }
    }
}

// ---------------------------------------------------------------- sculptor

/// The camera and canvas a stroke projects through: the projection,
/// its inverse, the camera world matrix and its inverse ( three.js
/// Matrix4 elements ), and the canvas rectangle.
#[derive(Clone, Copy)]
pub(super) struct View {
    pub(super) projection: [f64; 16],
    pub(super) projection_inverse: [f64; 16],
    pub(super) world: [f64; 16],
    pub(super) world_inverse: [f64; 16],
    pub(super) rect: [f64; 4],
}
/// Vector3.applyMatrix4 ( with the perspective divide ).
fn apply(m: &[f64; 16], v: V3) -> V3 {
    let w = 1. / (m[3] * v[0] + m[7] * v[1] + m[11] * v[2] + m[15]);
    [
        (m[0] * v[0] + m[4] * v[1] + m[8] * v[2] + m[12]) * w,
        (m[1] * v[0] + m[5] * v[1] + m[9] * v[2] + m[13]) * w,
        (m[2] * v[0] + m[6] * v[1] + m[10] * v[2] + m[14]) * w,
    ]
}
impl View {
    fn unproject(&self, client_x: f64, client_y: f64, z: f64) -> V3 {
        let r = self.rect;
        let x = ((client_x - r[0]) / r[2]) * 2. - 1.;
        let y = -((client_y - r[1]) / r[3]) * 2. + 1.;
        apply(&self.world, apply(&self.projection_inverse, [x, y, z]))
    }
    fn project(&self, p: V3) -> V3 {
        let v = apply(&self.projection, apply(&self.world_inverse, p));
        let r = self.rect;
        [
            (v[0] * 0.5 + 0.5) * r[2] + r[0],
            (-v[1] * 0.5 + 0.5) * r[3] + r[1],
            v[2],
        ]
    }
}
/// Matrix4.invert ( three.js r186 ).
pub(super) fn invert(te: &[f64; 16]) -> [f64; 16] {
    let (n11, n21, n31, n41) = (te[0], te[1], te[2], te[3]);
    let (n12, n22, n32, n42) = (te[4], te[5], te[6], te[7]);
    let (n13, n23, n33, n43) = (te[8], te[9], te[10], te[11]);
    let (n14, n24, n34, n44) = (te[12], te[13], te[14], te[15]);
    let t1 = n11 * n22 - n21 * n12;
    let t2 = n11 * n32 - n31 * n12;
    let t3 = n11 * n42 - n41 * n12;
    let t4 = n21 * n32 - n31 * n22;
    let t5 = n21 * n42 - n41 * n22;
    let t6 = n31 * n42 - n41 * n32;
    let t7 = n13 * n24 - n23 * n14;
    let t8 = n13 * n34 - n33 * n14;
    let t9 = n13 * n44 - n43 * n14;
    let t10 = n23 * n34 - n33 * n24;
    let t11 = n23 * n44 - n43 * n24;
    let t12 = n33 * n44 - n43 * n34;
    let det = t1 * t12 - t2 * t11 + t3 * t10 + t4 * t9 - t5 * t8 + t6 * t7;
    if det == 0. {
        return [0.; 16];
    }
    let d = 1. / det;
    [
        (n22 * t12 - n32 * t11 + n42 * t10) * d,
        (n31 * t11 - n21 * t12 - n41 * t10) * d,
        (n24 * t6 - n34 * t5 + n44 * t4) * d,
        (n33 * t5 - n23 * t6 - n43 * t4) * d,
        (n32 * t9 - n12 * t12 - n42 * t8) * d,
        (n11 * t12 - n31 * t9 + n41 * t8) * d,
        (n34 * t3 - n14 * t6 - n44 * t2) * d,
        (n13 * t6 - n33 * t3 + n43 * t2) * d,
        (n12 * t11 - n22 * t9 + n42 * t7) * d,
        (n21 * t9 - n11 * t11 - n41 * t7) * d,
        (n14 * t5 - n24 * t3 + n44 * t1) * d,
        (n23 * t3 - n13 * t5 - n43 * t1) * d,
        (n22 * t8 - n12 * t10 - n32 * t7) * d,
        (n11 * t10 - n21 * t8 + n31 * t7) * d,
        (n24 * t2 - n14 * t4 - n34 * t1) * d,
        (n13 * t4 - n23 * t2 + n33 * t1) * d,
    ]
}
/// The Sculptor's stroke state.
pub(super) struct Sculptor {
    pub(super) mesh: Mesh,
    settings: [(f64, f64, bool); 9],
    pub(super) tool: Tool,
    pub(super) size: f64,
    pub(super) strength: f64,
    pub(super) negative: bool,
    pub(super) detail: f64,
    pub(super) sculpting: bool,
    last_pointer: [f64; 2],
    hit_face: i64,
    ray_origin: V3,
    ray_direction: V3,
    pub(super) hit_point: V3,
    pub(super) hit_normal: V3,
    local_radius2: f64,
    pub(super) world_radius2: f64,
    drag_direction: V3,
    /// Vertices whose position and normal changed since the last sync.
    pub(super) dirty: Vec<u32>,
}
impl Sculptor {
    pub(super) fn new(source: &[f32]) -> Self {
        Self {
            mesh: Mesh::new(source),
            settings: TOOLS.map(tool_defaults),
            tool: Tool::Clay,
            size: 50.,
            strength: 0.5,
            negative: false,
            detail: 0.75,
            sculpting: false,
            last_pointer: [0.; 2],
            hit_face: -1,
            ray_origin: [0.; 3],
            ray_direction: [0.; 3],
            hit_point: [0.; 3],
            hit_normal: [0.; 3],
            local_radius2: 0.,
            world_radius2: 0.,
            drag_direction: [0.; 3],
            dirty: vec![],
        }
    }
    pub(super) fn set_tool(&mut self, tool: Tool) {
        if tool == self.tool {
            return;
        }
        let (size, strength, negative) = self.settings[tool as usize];
        self.tool = tool;
        self.size = size;
        self.strength = strength;
        self.negative = negative;
    }
    pub(super) fn set_size(&mut self, size: f64) {
        self.size = size;
        self.settings[self.tool as usize].0 = size;
    }
    pub(super) fn set_strength(&mut self, strength: f64) {
        self.strength = strength;
        self.settings[self.tool as usize].1 = strength;
    }
    pub(super) fn set_negative(&mut self, negative: bool) {
        self.negative = negative;
        self.settings[self.tool as usize].2 = negative;
    }
    pub(super) fn has_hit(&self) -> bool {
        self.hit_face >= 0
    }
    fn clear_hit(&mut self) {
        self.hit_face = -1;
        self.local_radius2 = 0.;
        self.world_radius2 = 0.;
        self.hit_point = [0.; 3];
        self.hit_normal = [0.; 3];
    }
    /// _updatePointerRay: the mesh's world matrix is the identity.
    fn update_pointer_ray(&mut self, view: &View, x: f64, y: f64) -> bool {
        let near = view.unproject(x, y, 0.);
        let far = view.unproject(x, y, 0.1);
        self.ray_origin = near;
        let mut d = sub(far, near);
        let length = hypot(&d);
        if length == 0. || !length.is_finite() {
            return false;
        }
        for v in &mut d {
            *v /= length;
        }
        self.ray_direction = d;
        true
    }
    fn pick_closest_face(&mut self) -> bool {
        let candidates = self.mesh.collect_ray(self.ray_origin, self.ray_direction);
        let mut distance = f64::INFINITY;
        self.hit_face = -1;
        for face in candidates {
            let o = face as usize * 4;
            let (v1, v2, v3) = (
                self.mesh.v(self.mesh.faces[o]),
                self.mesh.v(self.mesh.faces[o + 1]),
                self.mesh.v(self.mesh.faces[o + 2]),
            );
            let (t, hit) =
                intersection_ray_triangle(self.ray_origin, self.ray_direction, v1, v2, v3);
            if t >= 0. && t < distance {
                distance = t;
                self.hit_point = hit;
                self.hit_face = face as i64;
            }
        }
        self.hit_face != -1
    }
    fn intersection_ray_mesh(&mut self, view: &View, x: f64, y: f64) -> bool {
        let r = view.rect;
        if !x.is_finite() || !y.is_finite() || r[2] <= 0. || r[3] <= 0. {
            self.clear_hit();
            return false;
        }
        if !self.update_pointer_ray(view, x, y) {
            self.clear_hit();
            return false;
        }
        if !self.pick_closest_face() {
            self.clear_hit();
            return false;
        }
        self.update_radii(view);
        true
    }
    fn update_radii(&mut self, view: &View) {
        let world = self.hit_point;
        let screen = view.project(world);
        let radius_point = view.unproject(screen[0] + self.size, screen[1], screen[2]);
        self.world_radius2 = sqr_dist(world, radius_point);
        self.local_radius2 = self.world_radius2;
    }
    fn pick_vertices_in_sphere(&mut self, radius2: f64) -> Vec<u32> {
        let faces = self.mesh.collect_sphere(self.hit_point, radius2, true);
        let verts = self.mesh.vertices_from_faces(&faces);
        let flag = self.mesh.next_sculpt_flag();
        let mut picked = vec![];
        for v in verts {
            if sqr_dist(self.hit_point, self.mesh.v(v)) < radius2 {
                self.mesh.vert_sculpt_flags[v as usize] = flag;
                picked.push(v);
            }
        }
        picked
    }
    fn compute_picked_normal(&mut self) -> bool {
        if self.hit_face < 0 {
            return false;
        }
        let o = self.hit_face as usize * 4;
        let ids = [
            self.mesh.faces[o],
            self.mesh.faces[o + 1],
            self.mesh.faces[o + 2],
        ];
        let h = self.hit_point;
        let dist = ids.map(|v| hypot(&sub(h, self.mesh.v(v))));
        let n = |v: u32| {
            let o = v as usize * 3;
            [
                self.mesh.normals[o] as f64,
                self.mesh.normals[o + 1] as f64,
                self.mesh.normals[o + 2] as f64,
            ]
        };
        if dist.contains(&0.) {
            let v = if dist[0] == 0. {
                ids[0]
            } else if dist[1] == 0. {
                ids[1]
            } else {
                ids[2]
            };
            self.hit_normal = n(v);
        } else {
            let w = dist.map(|d| 1. / d);
            let inverse = 1. / (w[0] + w[1] + w[2]);
            let (a, b, c) = (n(ids[0]), n(ids[1]), n(ids[2]));
            self.hit_normal =
                [0, 1, 2].map(|k| (a[k] * w[0] + b[k] * w[1] + c[k] * w[2]) * inverse);
        }
        let length = hypot(&self.hit_normal);
        if length > 0. {
            for v in &mut self.hit_normal {
                *v /= length;
            }
        }
        true
    }
    fn dynamic_topology(&mut self, picked: Vec<u32>) -> Vec<u32> {
        let detail = self.detail;
        if detail == 0. {
            return picked;
        }
        let version = self.mesh.topology_version;
        let seeds = if picked.is_empty() {
            self.mesh.vertices_from_faces(&[self.hit_face as u32])
        } else {
            picked.clone()
        };
        let faces = self.mesh.faces_from_vertices(&seeds);
        let radius2 = self.local_radius2;
        let edge_max2 = radius2 * (1.1 - detail) * 0.2;
        let edge_min2 = edge_max2 / TOPOLOGY_HYSTERESIS2;
        let faces = self
            .mesh
            .subdivision_pass(faces, self.hit_point, radius2, edge_max2);
        let faces = self
            .mesh
            .decimation_pass(faces, self.hit_point, radius2, edge_min2);
        if self.mesh.topology_version == version {
            return picked;
        }
        let affected = self.mesh.vertices_from_faces(&faces);
        let faces = self.mesh.faces_from_vertices(&affected);
        let affected = self.mesh.vertices_from_faces(&faces);
        let flag = self.mesh.sculpt_flag;
        let result: Vec<u32> = affected
            .iter()
            .copied()
            .filter(|&v| self.mesh.vert_sculpt_flags[v as usize] == flag)
            .collect();
        self.mesh.update_topology(&faces, &affected);
        self.mesh.update_geometry(Some(&faces), Some(&affected));
        self.dirty.extend_from_slice(&affected);
        result
    }
    fn apply_stroke(&mut self, scale_delta: f64) {
        let tool = self.tool;
        let strength = self.strength;
        let deforms = strength != 0. || tool == Tool::Drag || tool == Tool::Scale;
        let remeshes = tool != Tool::Smooth && self.detail != 0.;
        if !deforms && !remeshes {
            return;
        }
        let radius2 = self.local_radius2;
        let mut picked = self.pick_vertices_in_sphere(radius2);
        if remeshes {
            picked = self.dynamic_topology(picked);
        }
        if !deforms || picked.is_empty() {
            return;
        }
        let hit = self.hit_point;
        let negative = self.negative;
        let m = &mut self.mesh;
        match tool {
            Tool::Clay | Tool::Flatten => {
                let front = m.front_vertices(&picked, self.ray_direction);
                let Some(normal) = m.area_normal(&front) else {
                    return;
                };
                let mut plane = m.area_center(&front);
                if tool == Tool::Clay {
                    let offset =
                        radius2.sqrt() * CLAY_OFFSET_RATIO * if negative { -1. } else { 1. };
                    for k in 0..3 {
                        plane[k] += normal[k] * offset;
                    }
                }
                m.tool_flatten(&picked, normal, plane, hit, radius2, strength, negative);
            }
            Tool::Brush => m.tool_brush(&picked, self.hit_normal, hit, radius2, strength, negative),
            Tool::Inflate => m.tool_inflate(&picked, hit, radius2, strength, negative),
            Tool::Smooth => m.tool_smooth(&picked, strength),
            Tool::Pinch => m.tool_pinch(&picked, hit, radius2, strength, negative),
            Tool::Crease => {
                m.tool_crease(&picked, self.hit_normal, hit, radius2, strength, negative)
            }
            Tool::Drag => m.tool_drag(&picked, hit, radius2, self.drag_direction),
            Tool::Scale => m.tool_scale(&picked, hit, radius2, scale_delta),
        }
        let faces = m.faces_from_vertices(&picked);
        let affected = m.vertices_from_faces(&faces);
        m.update_geometry(Some(&faces), Some(&affected));
        self.dirty.extend_from_slice(&affected);
    }
    fn make_stroke(&mut self, view: &View, x: f64, y: f64) -> bool {
        if !self.intersection_ray_mesh(view, x, y) {
            return false;
        }
        self.compute_picked_normal();
        self.apply_stroke(0.);
        true
    }
    fn sculpt_stroke(&mut self, view: &View, x: f64, y: f64) -> bool {
        let (dx, dy) = (x - self.last_pointer[0], y - self.last_pointer[1]);
        let distance = hypot(&[dx, dy]);
        let min_spacing = STAMP_SPACING_RATIO * self.size;
        if distance <= min_spacing {
            return false;
        }
        let count = (distance / min_spacing).floor();
        let (step_x, step_y) = (dx / count, dy / count);
        let (mut px, mut py) = (self.last_pointer[0] + step_x, self.last_pointer[1] + step_y);
        let mut sampled = false;
        let count = count as usize;
        for i in 0..count {
            sampled = i == count - 1;
            if !self.make_stroke(view, px, py) {
                break;
            }
            px += step_x;
            py += step_y;
        }
        self.last_pointer = [x, y];
        sampled
    }
    fn update_drag_direction(&mut self, view: &View, x: f64, y: f64) -> bool {
        if !self.update_pointer_ray(view, x, y) {
            return false;
        }
        let (h, o, d) = (self.hit_point, self.ray_origin, self.ray_direction);
        let p = sub(h, o);
        let denominator = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        let projection = if denominator > 0. {
            (d[0] * p[0] + d[1] * p[1] + d[2] * p[2]) / denominator
        } else {
            0.
        };
        let q = [
            o[0] + d[0] * projection,
            o[1] + d[1] * projection,
            o[2] + d[2] * projection,
        ];
        self.drag_direction = sub(q, h);
        self.hit_point = q;
        self.update_radii(view);
        true
    }
    fn sculpt_stroke_drag(&mut self, view: &View, x: f64, y: f64) {
        let (dx, dy) = (x - self.last_pointer[0], y - self.last_pointer[1]);
        let distance = hypot(&[dx, dy]);
        if distance == 0. {
            return;
        }
        let min_spacing = STAMP_SPACING_RATIO * self.size;
        let count = (distance / min_spacing).floor().max(1.);
        let (step_x, step_y) = (dx / count, dy / count);
        let (mut px, mut py) = (self.last_pointer[0] + step_x, self.last_pointer[1] + step_y);
        for _ in 0..count as usize {
            if !self.update_drag_direction(view, px, py) {
                break;
            }
            self.compute_picked_normal();
            self.apply_stroke(0.);
            px += step_x;
            py += step_y;
        }
        self.last_pointer = [x, y];
    }
    fn sculpt_stroke_scale(&mut self, x: f64, y: f64) {
        let delta = x - self.last_pointer[0];
        self.last_pointer = [x, y];
        if delta == 0. {
            return;
        }
        self.apply_stroke(delta);
    }
    /// _onPointerDown: a stroke starts on a hit.
    pub(super) fn pointer_down(&mut self, view: &View, x: f64, y: f64) -> bool {
        if self.sculpting || !self.intersection_ray_mesh(view, x, y) {
            return false;
        }
        self.compute_picked_normal();
        self.last_pointer = [x, y];
        self.sculpting = true;
        true
    }
    /// _onPointerMove during a stroke.
    pub(super) fn pointer_move(&mut self, view: &View, x: f64, y: f64) {
        if !self.sculpting {
            return;
        }
        match self.tool {
            Tool::Drag => self.sculpt_stroke_drag(view, x, y),
            Tool::Scale => self.sculpt_stroke_scale(x, y),
            _ => {
                let sampled = self.sculpt_stroke(view, x, y);
                if !sampled && self.intersection_ray_mesh(view, x, y) {
                    self.compute_picked_normal();
                }
            }
        }
    }
    /// endStroke().
    pub(super) fn end_stroke(&mut self) {
        if !self.sculpting {
            return;
        }
        self.sculpting = false;
        self.mesh.balance_octree();
    }
    /// pickFromPointer( x, y ).
    pub(super) fn pick(&mut self, view: &View, x: f64, y: f64) -> bool {
        if !self.intersection_ray_mesh(view, x, y) {
            return false;
        }
        self.compute_picked_normal();
        true
    }
}
/// addVertexUpdateRanges: the dirty vertices' component ranges, merged
/// across gaps of at most 96 components and capped at eight ranges.
pub(super) fn update_ranges(vertices: &[u32]) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut start = vertices[0] as usize;
    let mut previous = start;
    for &v in &vertices[1..] {
        let current = v as usize;
        if current == previous {
            continue;
        }
        if current != previous + 1 {
            ranges.push((start * 3, (previous - start + 1) * 3));
            start = current;
        }
        previous = current;
    }
    ranges.push((start * 3, (previous - start + 1) * 3));
    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(usize, usize)> = vec![ranges[0]];
    for &r in &ranges[1..] {
        let last = merged.len() - 1;
        let end = merged[last].0 + merged[last].1;
        if r.0 as isize - end as isize <= MAX_UPDATE_GAP_COMPONENTS as isize {
            merged[last].1 = (r.0 + r.1).max(end) - merged[last].0;
        } else {
            merged.push(r);
        }
    }
    if merged.len() <= MAX_UPDATE_RANGES {
        return merged;
    }
    let gap = |i: usize| merged[i].0 as isize - merged[i - 1].0 as isize - merged[i - 1].1 as isize;
    let mut splits: Vec<usize> = (1..merged.len()).collect();
    splits.sort_by(|&a, &b| gap(b).cmp(&gap(a)));
    splits.truncate(MAX_UPDATE_RANGES - 1);
    splits.sort_unstable();
    let mut out = vec![merged[0]];
    let mut split = 0;
    for (i, &r) in merged.iter().enumerate().skip(1) {
        if split < splits.len() && i == splits[split] {
            out.push(r);
            split += 1;
        } else {
            let last = out.len() - 1;
            out[last].1 = (out[last].0 + out[last].1).max(r.0 + r.1) - out[last].0;
        }
    }
    out
}
/// IcosahedronGeometry( radius, detail )'s non-indexed positions, by
/// PolyhedronGeometry's subdivision and applyRadius.
pub(super) fn icosahedron(radius: f64, detail: usize) -> Vec<f32> {
    let t = (1. + 5f64.sqrt()) / 2.;
    let vertices = [
        [-1., t, 0.],
        [1., t, 0.],
        [-1., -t, 0.],
        [1., -t, 0.],
        [0., -1., t],
        [0., 1., t],
        [0., -1., -t],
        [0., 1., -t],
        [t, 0., -1.],
        [t, 0., 1.],
        [-t, 0., -1.],
        [-t, 0., 1.],
    ];
    let indices = [
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7,
        1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9,
        8, 1,
    ];
    let lerp = |a: V3, b: V3, alpha: f64| [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * alpha);
    let mut buffer: Vec<V3> = vec![];
    let cols = detail + 1;
    for f in indices.chunks(3) {
        let (a, b, c) = (vertices[f[0]], vertices[f[1]], vertices[f[2]]);
        let mut v: Vec<Vec<V3>> = vec![];
        for i in 0..=cols {
            let aj = lerp(a, c, i as f64 / cols as f64);
            let bj = lerp(b, c, i as f64 / cols as f64);
            let rows = cols - i;
            let mut row = vec![];
            for j in 0..=rows {
                if j == 0 && i == cols {
                    row.push(aj);
                } else {
                    row.push(lerp(aj, bj, j as f64 / rows as f64));
                }
            }
            v.push(row);
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
    buffer
        .iter()
        .flat_map(|p| {
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            let inverse = 1. / if length == 0. { 1. } else { length };
            p.map(|c| (c * inverse * radius) as f32)
        })
        .collect()
}
