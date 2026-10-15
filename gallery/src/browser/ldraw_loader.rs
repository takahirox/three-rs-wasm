//! A port of three.js r186's LDrawLoader for packed LDraw files ( every
//! subfile embedded with "0 FILE" ): the line parser, !COLOUR materials and
//! their edge and conditional-edge materials, BFC winding, the subobject
//! hierarchy ( parts become groups, primitives merge into their part ), the
//! smoothed normals across soft edges, createObject's per-material groups and
//! the building steps. JavaScript semantics the result depends on are kept:
//! the insertion order of the half-edge map, the shared normal wrappers and
//! the stable sort by color code.
use crate::{Error, Result, math::*};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

const MAIN: &str = "16";
const MAIN_EDGE: &str = "24";

/// A MeshStandardMaterial from a !COLOUR directive ( linear colors ).
#[derive(Clone, Debug)]
pub(super) struct FaceMaterial {
    pub code: Option<String>,
    pub color: Vector3,
    pub opacity: f64,
    pub transparent: bool,
    pub roughness: f64,
    pub metalness: f64,
    pub emissive: Vector3,
    /// edgeMaterialCache: its LineBasicMaterial.
    pub edge: usize,
}
/// A LineBasicMaterial and, through conditionalEdgeMaterialCache, its
/// conditional-line material ( the same color and opacity ).
#[derive(Clone, Debug)]
pub(super) struct EdgeMaterial {
    pub color: Vector3,
    pub opacity: f64,
    pub transparent: bool,
    pub conditional: Option<usize>,
}
/// A material slot: a resolved material or the color code awaiting a parent scope.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Slot {
    Code(String),
    Face(usize),
    Edge(usize),
    Conditional(usize),
}
/// createObject's geometry: positions ( and normals for faces, the control
/// points and directions for conditional lines ) with its material groups.
#[derive(Debug)]
pub(super) struct Geometry {
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub controls: Option<[Vec<f32>; 3]>,
    /// start, count, material index.
    pub groups: Vec<(usize, usize, usize)>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Kind {
    Mesh,
    Lines,
    Conditional,
}
#[derive(Clone, Debug)]
pub(super) struct Object {
    pub kind: Kind,
    pub geometry: Rc<Geometry>,
    pub materials: Vec<Slot>,
}
#[derive(Clone, Debug, Default)]
pub(super) struct Group {
    pub name: String,
    pub position: Vector3,
    pub quaternion: Quaternion,
    pub scale: Vector3,
    pub starting_step: bool,
    pub building_step: usize,
    pub children: Vec<Child>,
}
#[derive(Clone, Debug)]
pub(super) enum Child {
    Group(Group),
    Object(Object),
}
#[derive(Clone)]
struct Face {
    color: String,
    material: Option<usize>,
    vertices: Vec<Vector3>,
    /// Shared normal wrappers ( indices into the smoothing arena ).
    normals: Vec<Option<usize>>,
    face_normal: Option<Vector3>,
    double_sided: bool,
}
#[derive(Clone)]
struct Segment {
    color: String,
    material: Option<usize>,
    vertices: [Vector3; 2],
    controls: [Vector3; 2],
}
struct Subobject {
    color: String,
    matrix: [f64; 16],
    file: String,
    inverted: bool,
    starting_step: bool,
}
#[derive(Clone)]
struct Info {
    faces: Vec<Face>,
    conditional: Vec<Segment>,
    lines: Vec<Segment>,
    kind: String,
    subobjects: Rc<Vec<Subobject>>,
    total_faces: usize,
    materials: Rc<HashMap<String, usize>>,
}
/// JavaScript's parseFloat: the longest numeric prefix, else NaN.
fn parse_float(s: &str) -> f64 {
    let s = s.trim_start();
    let b = s.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    if s[i..].starts_with("Infinity") {
        return if b.first() == Some(&b'-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    let digits = |i: &mut usize| {
        let start = *i;
        while *i < b.len() && b[*i].is_ascii_digit() {
            *i += 1;
        }
        *i > start
    };
    let mut any = digits(&mut i);
    if i < b.len() && b[i] == b'.' {
        i += 1;
        any |= digits(&mut i);
    }
    if !any {
        return f64::NAN;
    }
    let mantissa = i;
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        if digits(&mut j) {
            i = j;
        }
    }
    s[..i]
        .parse()
        .or_else(|_| s[..mantissa].parse())
        .unwrap_or(f64::NAN)
}
/// JavaScript's parseInt without a radix ( "0x" reads hexadecimal ).
fn parse_int(s: &str) -> Option<i64> {
    let s = s.trim_start();
    let (negative, s) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let (radix, s) = if s.starts_with("0x") || s.starts_with("0X") {
        (16, &s[2..])
    } else {
        (10, s)
    };
    let end = s
        .char_indices()
        .find(|(_, c)| !c.is_digit(radix))
        .map_or(s.len(), |(i, _)| i);
    let v = i64::from_str_radix(&s[..end], radix).ok()?;
    Some(if negative { -v } else { v })
}
/// The loader's LineParser over UTF-16-like character positions.
struct LineParser {
    line: Vec<char>,
    index: usize,
    line_number: usize,
}
impl LineParser {
    fn new(line: &str, line_number: usize) -> Self {
        Self {
            line: line.chars().collect(),
            index: 0,
            line_number,
        }
    }
    fn seek_non_space(&mut self) {
        while self.index < self.line.len() {
            let c = self.line[self.index];
            if c != ' ' && c != '\t' {
                return;
            }
            self.index += 1;
        }
    }
    fn token(&mut self) -> String {
        let start = self.index;
        self.index += 1;
        while self.index < self.line.len() {
            let c = self.line[self.index];
            if c == ' ' || c == '\t' {
                break;
            }
            self.index += 1;
        }
        let end = self.index.min(self.line.len());
        self.seek_non_space();
        self.line[start.min(end)..end].iter().collect()
    }
    fn vector(&mut self) -> Vector3 {
        let x = parse_float(&self.token());
        let y = parse_float(&self.token());
        let z = parse_float(&self.token());
        Vector3::new(x, y, z)
    }
    fn remaining(&self) -> String {
        self.line[self.index.min(self.line.len())..]
            .iter()
            .collect()
    }
    fn at_end(&self) -> bool {
        self.index >= self.line.len()
    }
    fn invalid(&self, what: &str) -> Error {
        Error::Asset(format!("LDraw: {what} at line {}", self.line_number))
    }
}
/// Vector3.applyMatrix4.
fn apply(m: &[f64; 16], v: Vector3) -> Vector3 {
    let w = 1. / (m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15]);
    Vector3::new(
        (m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12]) * w,
        (m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13]) * w,
        (m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14]) * w,
    )
}
/// Matrix4.determinant.
fn determinant(te: &[f64; 16]) -> f64 {
    let (n11, n12, n13, n14) = (te[0], te[4], te[8], te[12]);
    let (n21, n22, n23, n24) = (te[1], te[5], te[9], te[13]);
    let (n31, n32, n33, n34) = (te[2], te[6], te[10], te[14]);
    let (n41, n42, n43, n44) = (te[3], te[7], te[11], te[15]);
    n41 * (n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34
        - n12 * n23 * n34)
        + n42
            * (n11 * n23 * n34 - n11 * n24 * n33 + n14 * n21 * n33 - n13 * n21 * n34
                + n13 * n24 * n31
                - n14 * n23 * n31)
        + n43
            * (n11 * n24 * n32 - n11 * n22 * n34 - n14 * n21 * n32
                + n12 * n21 * n34
                + n14 * n22 * n31
                - n12 * n24 * n31)
        + n44
            * (-n13 * n22 * n31 - n11 * n23 * n32 + n11 * n22 * n33 + n13 * n21 * n32
                - n12 * n21 * n33
                + n12 * n23 * n31)
}
/// Matrix4.decompose.
pub(super) fn decompose(te: &[f64; 16]) -> (Vector3, Quaternion, Vector3) {
    let length = |x: f64, y: f64, z: f64| (x * x + y * y + z * z).sqrt();
    let mut sx = length(te[0], te[1], te[2]);
    let sy = length(te[4], te[5], te[6]);
    let sz = length(te[8], te[9], te[10]);
    if determinant(te) < 0. {
        sx = -sx;
    }
    let position = Vector3::new(te[12], te[13], te[14]);
    let (ix, iy, iz) = (1. / sx, 1. / sy, 1. / sz);
    let m = [
        te[0] * ix,
        te[1] * ix,
        te[2] * ix,
        te[4] * iy,
        te[5] * iy,
        te[6] * iy,
        te[8] * iz,
        te[9] * iz,
        te[10] * iz,
    ];
    // Quaternion.setFromRotationMatrix.
    let (m11, m12, m13) = (m[0], m[3], m[6]);
    let (m21, m22, m23) = (m[1], m[4], m[7]);
    let (m31, m32, m33) = (m[2], m[5], m[8]);
    let trace = m11 + m22 + m33;
    let q = if trace > 0. {
        let s = 0.5 / (trace + 1.).sqrt();
        [(m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s]
    } else if m11 > m22 && m11 > m33 {
        let s = 2. * (1. + m11 - m22 - m33).sqrt();
        [0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s]
    } else if m22 > m33 {
        let s = 2. * (1. + m22 - m11 - m33).sqrt();
        [(m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s]
    } else {
        let s = 2. * (1. + m33 - m11 - m22).sqrt();
        [(m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s]
    };
    (
        position,
        Quaternion::from_xyzw(q[0], q[1], q[2], q[3]),
        Vector3::new(sx, sy, sz),
    )
}
/// The JavaScript object used as the half-edge list: insertion-ordered keys,
/// reassignment keeps a key's position.
struct OrderedMap<K, V> {
    map: HashMap<K, (u64, V)>,
    order: BTreeMap<u64, K>,
    next: u64,
}
impl<K: std::hash::Hash + Eq + Clone, V: Clone> OrderedMap<K, V> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: BTreeMap::new(),
            next: 0,
        }
    }
    fn set(&mut self, key: K, value: V) {
        if let Some(entry) = self.map.get_mut(&key) {
            entry.1 = value;
            return;
        }
        self.order.insert(self.next, key.clone());
        self.map.insert(key, (self.next, value));
        self.next += 1;
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|e| &e.1)
    }
    fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
    fn delete(&mut self, key: &K) {
        if let Some((order, _)) = self.map.remove(key) {
            self.order.remove(&order);
        }
    }
    fn first(&self) -> Option<V> {
        let (_, key) = self.order.iter().next()?;
        self.map.get(key).map(|e| e.1.clone())
    }
}
type VertexHash = (i32, i32, i32);
type EdgeHash = (VertexHash, VertexHash);
/// ~~( v * ( 1 + 1e-10 ) * 1e2 ): ToInt32 of the scaled component.
fn hash_component(v: f64) -> i32 {
    let t = (v * ((1. + 1e-10) * 1e2)).trunc();
    if !t.is_finite() {
        return 0;
    }
    t.rem_euclid(4294967296.) as u64 as u32 as i32
}
fn hash_vertex(v: Vector3) -> VertexHash {
    (
        hash_component(v.x),
        hash_component(v.y),
        hash_component(v.z),
    )
}
fn hash_edge(a: Vector3, b: Vector3) -> EdgeHash {
    (hash_vertex(a), hash_vertex(b))
}
/// toNormalizedRay: the direction and the origin projected onto the line through 0.
fn normalized_ray(a: Vector3, b: Vector3) -> (Vector3, Vector3) {
    let direction = normalize(b - a);
    let scalar = a.dot(direction);
    (a + direction * -scalar, direction)
}
fn face_normal(v: &[Vector3]) -> Vector3 {
    normalize((v[1] - v[0]).cross(v[2] - v[1]))
}
/// Vector3.normalize: divideScalar( length() || 1 ), a multiplication by the
/// inverse.
fn normalize(v: Vector3) -> Vector3 {
    let length = v.length();
    v * (1.
        / if length == 0. || length.is_nan() {
            1.
        } else {
            length
        })
}
/// smoothNormals: share normals across face edges that are neither hard
/// lines nor sharper than about 75 degrees. Returns the normal vectors the
/// faces' wrappers point to.
fn smooth_normals(faces: &mut [Face], lines: &[Segment], check_sub_segments: bool) -> Vec<Vector3> {
    let mut hard = HashSet::new();
    // Per ray hash: an index into the ray infos ( direction, segment extents ).
    let mut rays: HashMap<EdgeHash, usize> = HashMap::new();
    let mut infos: Vec<(Vector3, Vec<f64>)> = vec![];
    for ls in lines {
        let [v0, v1] = ls.vertices;
        hard.insert(hash_edge(v0, v1));
        hard.insert(hash_edge(v1, v0));
        if check_sub_segments {
            let (origin, direction) = normalized_ray(v0, v1);
            let rh1 = hash_edge(origin, direction);
            if !rays.contains_key(&rh1) {
                let (o2, d2) = normalized_ray(v1, v0);
                let rh2 = hash_edge(o2, d2);
                infos.push((direction, vec![]));
                rays.insert(rh1, infos.len() - 1);
                rays.insert(rh2, infos.len() - 1);
            }
            let info = &mut infos[rays[&rh1]];
            let (mut d0, mut d1) = (info.0.dot(v0), info.0.dot(v1));
            if d0 > d1 {
                std::mem::swap(&mut d0, &mut d1);
            }
            info.1.extend([d0, d1]);
        }
    }
    // Half edges: ( vertex index, face ).
    let mut half: OrderedMap<EdgeHash, (usize, usize)> = OrderedMap::new();
    for (f, tri) in faces.iter().enumerate() {
        let count = tri.vertices.len();
        for index in 0..count {
            let next = (index + 1) % count;
            let (v0, v1) = (tri.vertices[index], tri.vertices[next]);
            let hash = hash_edge(v0, v1);
            if hard.contains(&hash) {
                continue;
            }
            if check_sub_segments {
                let (origin, direction) = normalized_ray(v0, v1);
                if let Some(&i) = rays.get(&hash_edge(origin, direction)) {
                    let (direction, distances) = &infos[i];
                    let (mut d0, mut d1) = (direction.dot(v0), direction.dot(v1));
                    if d0 > d1 {
                        std::mem::swap(&mut d0, &mut d1);
                    }
                    if distances.chunks(2).any(|d| d0 >= d[0] && d1 <= d[1]) {
                        continue;
                    }
                }
            }
            half.set(hash, (index, f));
        }
    }
    // The wrappers' vectors, and the vectors in creation order.
    let mut wrappers: Vec<usize> = vec![];
    let mut vectors: Vec<Vector3> = vec![];
    let mut created: Vec<usize> = vec![];
    while let Some(start) = half.first() {
        let mut queue = vec![start];
        while let Some((_, f)) = queue.pop() {
            let count = faces[f].vertices.len();
            for index in 0..count {
                let next = (index + 1) % count;
                let (v0, v1) = (faces[f].vertices[index], faces[f].vertices[next]);
                half.delete(&hash_edge(v0, v1));
                let reverse = hash_edge(v1, v0);
                let Some(&(other_index, other)) = half.get(&reverse) else {
                    continue;
                };
                let face_normal = faces[f].face_normal.unwrap_or(Vector3::ZERO);
                let other_normal = faces[other].face_normal.unwrap_or(Vector3::ZERO);
                if other_normal.dot(face_normal).abs() < 0.25 {
                    continue;
                }
                if half.contains(&reverse) {
                    queue.push((other_index, other));
                    half.delete(&reverse);
                }
                let other_count = faces[other].normals.len();
                let other_next = (other_index + 1) % other_count;
                // Share the first normal, then the second.
                for (mine, theirs) in [(index, other_next), (next, other_index)] {
                    let a = faces[f].normals[mine];
                    let b = faces[other].normals[theirs];
                    if let (Some(a), Some(b)) = (a, b)
                        && a != b
                    {
                        let add = vectors[wrappers[a]];
                        vectors[wrappers[b]] += add;
                        wrappers[a] = wrappers[b];
                    }
                    let shared = match faces[f].normals[mine].or(faces[other].normals[theirs]) {
                        Some(w) => w,
                        None => {
                            vectors.push(Vector3::ZERO);
                            created.push(vectors.len() - 1);
                            wrappers.push(vectors.len() - 1);
                            wrappers.len() - 1
                        }
                    };
                    if faces[f].normals[mine].is_none() {
                        faces[f].normals[mine] = Some(shared);
                        vectors[wrappers[shared]] += face_normal;
                    }
                    if faces[other].normals[theirs].is_none() {
                        faces[other].normals[theirs] = Some(shared);
                        vectors[wrappers[shared]] += other_normal;
                    }
                }
            }
        }
    }
    for &v in &created {
        vectors[v] = normalize(vectors[v]);
    }
    // Resolve each face's wrappers to their final vectors.
    let resolved: Vec<Vector3> = wrappers.iter().map(|&v| vectors[v]).collect();
    resolved
}
pub(super) struct Loader {
    pub faces: Vec<FaceMaterial>,
    pub edges: Vec<EdgeMaterial>,
    /// Conditional-line materials: color and opacity, transparency.
    pub conditionals: Vec<EdgeMaterial>,
    library: HashMap<String, usize>,
    pub missing: usize,
    parsed: HashMap<String, Info>,
    parts: HashMap<String, Group>,
    pub smooth_normals: bool,
}
impl Loader {
    pub fn new(smooth_normals: bool) -> Self {
        let mut loader = Self {
            faces: vec![],
            edges: vec![],
            conditionals: vec![],
            library: HashMap::new(),
            missing: 0,
            parsed: HashMap::new(),
            parts: HashMap::new(),
            smooth_normals,
        };
        // missingColorMaterial, its edge and conditional edge materials.
        let magenta = Color::from_hex(0xff00ff).0;
        loader.conditionals.push(EdgeMaterial {
            color: magenta,
            opacity: 1.,
            transparent: false,
            conditional: None,
        });
        loader.edges.push(EdgeMaterial {
            color: magenta,
            opacity: 1.,
            transparent: false,
            conditional: Some(0),
        });
        loader.faces.push(FaceMaterial {
            code: None,
            color: magenta,
            opacity: 1.,
            transparent: false,
            roughness: 0.3,
            metalness: 0.,
            emissive: Vector3::ZERO,
            edge: 0,
        });
        loader
    }
    /// The '#RRGGBB' style in sRGB.
    fn style(color: &str) -> Vector3 {
        let hex = u32::from_str_radix(color.trim_start_matches('#'), 16).unwrap_or(0);
        Color::from_hex(hex).0
    }
    /// parseColorMetaDirective.
    fn parse_color(&mut self, lp: &mut LineParser) -> Result<usize> {
        let mut code = None;
        let mut fill = "#FF00FF".to_string();
        let mut edge_color = "#FF00FF".to_string();
        let mut alpha = 1.;
        let mut transparent = false;
        let mut luminance = 0.;
        let mut finish = (0.3, 0.);
        let mut edge = None;
        if lp.token().is_empty() {
            return Err(lp.invalid("material name expected"));
        }
        let luminance_of = |token: &str| -> Option<f64> {
            let v = if let Some(rest) = token.strip_prefix("LUMINANCE") {
                parse_int(rest)
            } else {
                parse_int(token)
            }?;
            Some((v as f64 / 255.).clamp(0., 1.))
        };
        loop {
            let token = lp.token();
            if token.is_empty() {
                break;
            }
            if let Some(l) = luminance_of(&token) {
                luminance = l;
                continue;
            }
            match token.to_uppercase().as_str() {
                "CODE" => code = Some(lp.token()),
                "VALUE" => {
                    fill = lp.token();
                    if let Some(hex) = fill.strip_prefix("0x") {
                        fill = format!("#{hex}");
                    } else if !fill.starts_with('#') {
                        return Err(lp.invalid("invalid color"));
                    }
                }
                "EDGE" => {
                    edge_color = lp.token();
                    if let Some(hex) = edge_color.strip_prefix("0x") {
                        edge_color = format!("#{hex}");
                    } else if !edge_color.starts_with('#') {
                        let material = self
                            .material(&edge_color)
                            .ok_or_else(|| lp.invalid("invalid edge color"))?;
                        edge = Some(self.faces[material].edge);
                    }
                }
                "ALPHA" => {
                    let a = parse_int(&lp.token()).ok_or_else(|| lp.invalid("invalid alpha"))?;
                    alpha = (a as f64 / 255.).clamp(0., 1.);
                    if alpha < 1. {
                        transparent = true;
                    }
                }
                "LUMINANCE" => {
                    luminance =
                        luminance_of(&lp.token()).ok_or_else(|| lp.invalid("invalid luminance"))?;
                }
                "CHROME" => finish = (0., 1.),
                "PEARLESCENT" => finish = (0.3, 0.25),
                "RUBBER" => finish = (0.9, 0.),
                "MATTE_METALLIC" => finish = (0.8, 0.4),
                "METAL" => finish = (0.2, 0.85),
                "MATERIAL" => lp.index = lp.line.len(),
                _ => return Err(lp.invalid("unknown material token")),
            }
        }
        let color = Self::style(&fill);
        let edge = match edge {
            Some(e) => e,
            None => {
                let edge_rgb = Self::style(&edge_color);
                self.conditionals.push(EdgeMaterial {
                    color: edge_rgb,
                    opacity: alpha,
                    transparent,
                    conditional: None,
                });
                self.edges.push(EdgeMaterial {
                    color: edge_rgb,
                    opacity: alpha,
                    transparent,
                    conditional: Some(self.conditionals.len() - 1),
                });
                self.edges.len() - 1
            }
        };
        self.faces.push(FaceMaterial {
            code: code.clone(),
            color,
            opacity: alpha,
            transparent,
            roughness: finish.0,
            metalness: finish.1,
            emissive: if luminance != 0. {
                color * luminance
            } else {
                Vector3::ZERO
            },
            edge,
        });
        let id = self.faces.len() - 1;
        // addMaterial: the first material of a code stays in the library.
        if let Some(code) = code
            && !self.library.contains_key(&code)
        {
            self.library.insert(code, id);
        }
        Ok(id)
    }
    /// getMaterial: the library, or a direct "0x2RRGGBB" color.
    fn material(&mut self, code: &str) -> Option<usize> {
        if let Some(hex) = code.strip_prefix("0x2") {
            let directive = format!("Direct_Color_{hex} CODE -1 VALUE #{hex} EDGE #{hex}");
            return self.parse_color(&mut LineParser::new(&directive, 0)).ok();
        }
        self.library.get(code).copied()
    }
    fn add_defaults(&mut self) -> Result<()> {
        for directive in [
            "Main_Colour CODE 16 VALUE #FF8080 EDGE #333333",
            "Edge_Colour CODE 24 VALUE #A0A0A0 EDGE #333333",
        ] {
            self.parse_color(&mut LineParser::new(directive, 0))?;
        }
        Ok(())
    }
    /// LDrawParsedCache.parse: one file's primitives, subobjects and materials;
    /// embedded files enter the cache.
    fn parse(&mut self, text: &str) -> Result<Info> {
        let text = if text.contains("\r\n") {
            text.replace("\r\n", "\n")
        } else {
            text.to_string()
        };
        let mut faces = vec![];
        let mut lines = vec![];
        let mut conditional = vec![];
        let mut subobjects = vec![];
        let mut materials: HashMap<String, usize> = HashMap::new();
        let mut kind = "Model".to_string();
        let mut total_faces = 0;
        let mut embedded: Option<(String, String)> = None;
        let (mut certified, mut ccw, mut inverted, mut cull) = (false, true, false, true);
        let mut starting_step = false;
        for (line_index, line) in text.split('\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            if let Some((name, body)) = &mut embedded {
                if let Some(next) = line.strip_prefix("0 FILE ") {
                    let (previous, body) = (std::mem::take(name), std::mem::take(body));
                    self.set_data(&previous, &body)?;
                    *name = next.to_string();
                } else {
                    body.push_str(line);
                    body.push('\n');
                }
                continue;
            }
            let mut lp = LineParser::new(line, line_index + 1);
            lp.seek_non_space();
            if lp.at_end() {
                continue;
            }
            let line_type = lp.token();
            let local =
                |materials: &HashMap<String, usize>, code: &str| materials.get(code).copied();
            match line_type.as_str() {
                "0" => {
                    let meta = lp.token();
                    match meta.as_str() {
                        "!LDRAW_ORG" => kind = lp.token(),
                        "!COLOUR" => {
                            let m = self.parse_color(&mut lp)?;
                            if let Some(code) = self.faces[m].code.clone() {
                                materials.insert(code, m);
                            }
                        }
                        "FILE" => {
                            if line_index > 0 {
                                embedded = Some((lp.remaining(), String::new()));
                                certified = false;
                                ccw = true;
                            }
                        }
                        "BFC" => {
                            while !lp.at_end() {
                                match lp.token().as_str() {
                                    t @ ("CERTIFY" | "NOCERTIFY") => {
                                        certified = t == "CERTIFY";
                                        ccw = true;
                                    }
                                    t @ ("CW" | "CCW") => ccw = t == "CCW",
                                    "INVERTNEXT" => inverted = true,
                                    t @ ("CLIP" | "NOCLIP") => cull = t == "CLIP",
                                    _ => {}
                                }
                            }
                        }
                        "STEP" => starting_step = true,
                        _ => {}
                    }
                }
                "1" => {
                    let color = lp.token();
                    let p: Vec<f64> = (0..12).map(|_| parse_float(&lp.token())).collect();
                    // Matrix4.set( m0, m1, m2, x, m3, m4, m5, y, m6, m7, m8, z, 0, 0, 0, 1 ).
                    let matrix = [
                        p[3], p[6], p[9], 0., p[4], p[7], p[10], 0., p[5], p[8], p[11], 0., p[0],
                        p[1], p[2], 1.,
                    ];
                    let mut file = lp.remaining().trim().replace('\\', "/");
                    if file.starts_with("s/") {
                        file = format!("parts/{file}");
                    } else if file.starts_with("48/") {
                        file = format!("p/{file}");
                    }
                    subobjects.push(Subobject {
                        color,
                        matrix,
                        file,
                        inverted,
                        starting_step,
                    });
                    starting_step = false;
                    inverted = false;
                }
                "2" | "5" => {
                    let color = lp.token();
                    let material = local(&materials, &color);
                    let v0 = lp.vector();
                    let v1 = lp.vector();
                    let mut segment = Segment {
                        color,
                        material,
                        vertices: [v0, v1],
                        controls: [Vector3::ZERO; 2],
                    };
                    if line_type == "5" {
                        segment.controls = [lp.vector(), lp.vector()];
                        conditional.push(segment);
                    } else {
                        lines.push(segment);
                    }
                }
                "3" | "4" => {
                    let color = lp.token();
                    let material = local(&materials, &color);
                    let double_sided = !certified || !cull;
                    let count = if line_type == "3" { 3 } else { 4 };
                    let mut vertices: Vec<Vector3> = (0..count).map(|_| lp.vector()).collect();
                    if !ccw {
                        vertices.reverse();
                    }
                    total_faces += if double_sided { 2 } else { 1 } * (count - 2);
                    faces.push(Face {
                        color,
                        material,
                        vertices,
                        normals: vec![None; count],
                        face_normal: None,
                        double_sided,
                    });
                }
                _ => return Err(lp.invalid("unknown line type")),
            }
        }
        if let Some((name, body)) = embedded {
            self.set_data(&name, &body)?;
        }
        Ok(Info {
            faces,
            conditional,
            lines,
            kind,
            subobjects: Rc::new(subobjects),
            total_faces,
            materials: Rc::new(materials),
        })
    }
    fn set_data(&mut self, name: &str, text: &str) -> Result<()> {
        let info = self.parse(text)?;
        self.parsed.insert(name.to_lowercase(), info);
        Ok(())
    }
    /// getData( fileName ): a copy whose vertices may be transformed.
    /// cloneResult copies neither `doubleSided` nor the normals: a copied
    /// face is one-sided, though `totalFaces` still counts both sides ( the
    /// unwritten tail of the geometry stays zero ).
    fn data(&self, name: &str) -> Result<Info> {
        let mut info = self
            .parsed
            .get(&name.to_lowercase())
            .cloned()
            .ok_or_else(|| {
                Error::Asset(format!("LDraw: subobject \"{name}\" could not be loaded"))
            })?;
        for face in &mut info.faces {
            face.normals.iter_mut().for_each(|n| *n = None);
            face.face_normal = None;
            face.double_sided = false;
        }
        Ok(info)
    }
    fn kind(&self, name: &str) -> Result<String> {
        self.parsed
            .get(&name.to_lowercase())
            .map(|i| i.kind.clone())
            .ok_or_else(|| Error::Asset(format!("LDraw: subobject \"{name}\" could not be loaded")))
    }
    /// getMaterialFromCode.
    fn material_from_code(
        code: &str,
        parent: &str,
        hierarchy: &HashMap<String, usize>,
        edge: bool,
    ) -> Option<usize> {
        let passthrough = (!edge && code == MAIN) || (edge && code == MAIN_EDGE);
        hierarchy
            .get(if passthrough { parent } else { code })
            .copied()
    }
    /// processInfoSubobjects: subobjects that are not primitives load as
    /// groups; primitives merge into this file's primitives.
    fn process_subobjects(
        &mut self,
        info: &mut Info,
        subobject: Option<(&str, &HashMap<String, usize>)>,
        face_materials: &mut HashSet<String>,
    ) -> Result<Group> {
        let mut group = Group {
            scale: Vector3::ONE,
            ..Default::default()
        };
        let subobjects = info.subobjects.clone();
        for sub in subobjects.iter() {
            let kind = self.kind(&sub.file)?;
            let primitive = kind.to_lowercase().contains("primitive") || kind == "Subpart";
            if !primitive {
                let mut child = self.load_model(&sub.file)?;
                let (position, quaternion, scale) = decompose(&sub.matrix);
                (child.position, child.quaternion, child.scale) = (position, quaternion, scale);
                child.starting_step = sub.starting_step;
                child.name = sub.file.clone();
                self.apply_materials(&mut child, &sub.color, &info.materials, false);
                group.children.push(Child::Group(child));
                continue;
            }
            let mut child = self.data(&sub.file)?;
            let child_group = self.process_subobjects(
                &mut child,
                Some((&sub.color, &info.materials)),
                face_materials,
            )?;
            if !child_group.children.is_empty() {
                group.children.push(Child::Group(child_group));
            }
            let matrix = &sub.matrix;
            let flip = (determinant(matrix) < 0.) != sub.inverted;
            let color = sub.color.as_str();
            let line_color = if color == MAIN { MAIN_EDGE } else { color };
            for mut ls in child.lines.drain(..) {
                ls.vertices = ls.vertices.map(|v| apply(matrix, v));
                if ls.color == MAIN_EDGE {
                    ls.color = line_color.to_string();
                }
                ls.material = ls.material.or_else(|| {
                    Self::material_from_code(&ls.color, &ls.color, &info.materials, true)
                });
                info.lines.push(ls);
            }
            for mut os in child.conditional.drain(..) {
                os.vertices = os.vertices.map(|v| apply(matrix, v));
                os.controls = os.controls.map(|v| apply(matrix, v));
                if os.color == MAIN_EDGE {
                    os.color = line_color.to_string();
                }
                os.material = os.material.or_else(|| {
                    Self::material_from_code(&os.color, &os.color, &info.materials, true)
                });
                info.conditional.push(os);
            }
            for mut tri in child.faces.drain(..) {
                tri.vertices.iter_mut().for_each(|v| *v = apply(matrix, *v));
                if tri.color == MAIN {
                    tri.color = color.to_string();
                }
                tri.material = tri.material.or_else(|| {
                    Self::material_from_code(&tri.color, color, &info.materials, false)
                });
                face_materials.insert(tri.color.clone());
                if flip {
                    tri.vertices.reverse();
                }
                info.faces.push(tri);
            }
            info.total_faces += child.total_faces;
        }
        if let Some((color, _)) = subobject {
            let materials = info.materials.clone();
            self.apply_materials(&mut group, color, &materials, false);
        }
        Ok(group)
    }
    /// processIntoMesh.
    fn process_into_mesh(&mut self, mut info: Info) -> Result<Group> {
        // The page's material-tracking loop compares the index with the array
        // and never runs: only the merged subobject faces count.
        let mut face_materials = HashSet::new();
        let mut group = self.process_subobjects(&mut info, None, &mut face_materials)?;
        let mut smoothed = vec![];
        if self.smooth_normals {
            for face in &mut info.faces {
                face.face_normal = Some(face_normal(&face.vertices));
            }
            smoothed = smooth_normals(&mut info.faces, &info.lines, face_materials.len() > 1);
        }
        if !info.faces.is_empty() {
            let total = info.total_faces;
            group.children.push(Child::Object(self.create_faces(
                &mut info.faces,
                total,
                &smoothed,
            )));
        }
        if !info.lines.is_empty() {
            group
                .children
                .push(Child::Object(self.create_lines(&mut info.lines, false)));
        }
        if !info.conditional.is_empty() {
            group.children.push(Child::Object(
                self.create_lines(&mut info.conditional, true),
            ));
        }
        Ok(group)
    }
    fn load_model(&mut self, name: &str) -> Result<Group> {
        let key = name.to_lowercase();
        if let Some(group) = self.parts.get(&key) {
            return Ok(group.clone());
        }
        let info = self.data(name)?;
        let part = info.kind == "Part" || info.kind == "Unofficial_Part";
        let group = self.process_into_mesh(info)?;
        if part {
            self.parts.insert(key, group.clone());
        }
        Ok(group)
    }
    /// The face object: two-sided faces repeat reversed with negated normals.
    fn create_faces(&mut self, faces: &mut [Face], total: usize, smoothed: &[Vector3]) -> Object {
        faces.sort_by(|a, b| a.color.cmp(&b.color));
        let mut positions = vec![0f32; 3 * total * 3];
        let mut normals = vec![0f32; 3 * total * 3];
        let mut builder = Groups::default();
        let mut offset = 0;
        for face in faces.iter_mut() {
            let quad = |v: &[Vector3]| -> Vec<Vector3> {
                if v.len() == 4 {
                    vec![v[0], v[1], v[2], v[0], v[2], v[3]]
                } else {
                    v.to_vec()
                }
            };
            let vertices = quad(&face.vertices);
            let sides = if face.double_sided { 2 } else { 1 };
            let n = vertices.len();
            for s in 0..sides {
                for j in 0..n {
                    let v = vertices[if s == 0 { j } else { n - 1 - j }];
                    let i = offset + (s * n + j) * 3;
                    positions[i..i + 3].copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
                }
            }
            let normal = *face
                .face_normal
                .get_or_insert_with(|| face_normal(&vertices));
            let wrappers: Vec<Option<usize>> = if face.normals.len() == 4 {
                let w = &face.normals;
                vec![w[0], w[1], w[2], w[0], w[2], w[3]]
            } else {
                face.normals.clone()
            };
            for s in 0..sides {
                let sign = if s == 0 { 1. } else { -1. };
                for j in 0..n {
                    let idx = if s == 0 { j } else { n - 1 - j };
                    let v = wrappers[idx].map_or(normal, |w| smoothed[w]);
                    let i = offset + (s * n + j) * 3;
                    normals[i..i + 3].copy_from_slice(&[
                        (sign * v.x) as f32,
                        (sign * v.y) as f32,
                        (sign * v.z) as f32,
                    ]);
                }
            }
            let slot = face
                .material
                .map_or(Slot::Code(face.color.clone()), Slot::Face);
            builder.push(&face.color, slot, offset / 3, n * sides);
            offset += 3 * n * sides;
        }
        let (groups, materials) = builder.finish();
        Object {
            kind: Kind::Mesh,
            geometry: Rc::new(Geometry {
                positions,
                normals: Some(normals),
                controls: None,
                groups,
            }),
            materials,
        }
    }
    /// The line or conditional-line object.
    fn create_lines(&mut self, segments: &mut [Segment], conditional: bool) -> Object {
        segments.sort_by(|a, b| a.color.cmp(&b.color));
        let mut positions = Vec::with_capacity(segments.len() * 6);
        let mut builder = Groups::default();
        let mut controls = [vec![], vec![], vec![]];
        for (i, ls) in segments.iter().enumerate() {
            for v in ls.vertices {
                positions.extend([v.x as f32, v.y as f32, v.z as f32]);
            }
            let slot = match ls.material {
                Some(m) => {
                    let edge = self.faces[m].edge;
                    if conditional {
                        self.edges[edge]
                            .conditional
                            .map_or(Slot::Code(ls.color.clone()), Slot::Conditional)
                    } else {
                        Slot::Edge(edge)
                    }
                }
                None => Slot::Code(ls.color.clone()),
            };
            builder.push(&ls.color, slot, i * 2, 2);
            if conditional {
                let [c0, c1] = ls.controls;
                let d = ls.vertices[1] - ls.vertices[0];
                for _ in 0..2 {
                    controls[0].extend([c0.x as f32, c0.y as f32, c0.z as f32]);
                    controls[1].extend([c1.x as f32, c1.y as f32, c1.z as f32]);
                    controls[2].extend([d.x as f32, d.y as f32, d.z as f32]);
                }
            }
        }
        let (groups, materials) = builder.finish();
        Object {
            kind: if conditional {
                Kind::Conditional
            } else {
                Kind::Lines
            },
            geometry: Rc::new(Geometry {
                positions,
                normals: None,
                controls: conditional.then_some(controls),
                groups,
            }),
            materials,
        }
    }
    /// applyMaterialsToMesh: color-code slots resolve from the scope's materials.
    fn apply_materials(
        &mut self,
        group: &mut Group,
        parent: &str,
        hierarchy: &HashMap<String, usize>,
        final_pass: bool,
    ) {
        let parent_passthrough = parent == MAIN;
        let mut objects = vec![];
        collect(group, &mut objects);
        for object in objects {
            let kind = object.kind;
            for slot in &mut object.materials {
                let Slot::Code(code) = slot else { continue };
                if parent_passthrough && !hierarchy.contains_key(code.as_str()) && !final_pass {
                    continue;
                }
                let edge = kind != Kind::Mesh;
                let passthrough = (!edge && code == MAIN) || (edge && code == MAIN_EDGE);
                let code = if passthrough {
                    parent.to_string()
                } else {
                    code.clone()
                };
                let material = if let Some(&m) = hierarchy.get(&code) {
                    m
                } else if final_pass {
                    self.material(&code).unwrap_or(self.missing)
                } else {
                    *slot = Slot::Code(code);
                    continue;
                };
                *slot = match kind {
                    Kind::Mesh => Slot::Face(material),
                    Kind::Lines => Slot::Edge(self.faces[material].edge),
                    Kind::Conditional => {
                        let edge = self.faces[material].edge;
                        self.edges[edge]
                            .conditional
                            .map_or(Slot::Code(code), Slot::Conditional)
                    }
                };
            }
        }
    }
    /// LDrawLoader.load: the defaults, the model, the final material pass
    /// and the building steps; returns the group and the step count.
    pub fn load(&mut self, text: &str) -> Result<(Group, usize)> {
        self.add_defaults()?;
        let info = self.parse(text)?;
        let mut group = self.process_into_mesh(info)?;
        let library = self.library.clone();
        self.apply_materials(&mut group, MAIN, &library, true);
        let mut step = 0;
        fn steps(g: &mut Group, step: &mut usize) {
            if g.starting_step {
                *step += 1;
            }
            g.building_step = *step;
            for c in &mut g.children {
                if let Child::Group(c) = c {
                    steps(c, step);
                }
            }
        }
        steps(&mut group, &mut step);
        Ok((group, step + 1))
    }
}
/// Every mesh and line object under a group.
fn collect<'a>(group: &'a mut Group, out: &mut Vec<&'a mut Object>) {
    for c in &mut group.children {
        match c {
            Child::Group(g) => collect(g, out),
            Child::Object(o) => out.push(o),
        }
    }
}
/// createObject's material groups: consecutive elements of one color code.
#[derive(Default)]
struct Groups {
    previous: Option<String>,
    start: usize,
    count: usize,
    groups: Vec<(usize, usize, usize)>,
    materials: Vec<Slot>,
}
impl Groups {
    fn push(&mut self, color: &str, slot: Slot, start: usize, count: usize) {
        if self.previous.as_deref() != Some(color) {
            if self.previous.is_some() {
                self.groups
                    .push((self.start, self.count, self.materials.len() - 1));
            }
            self.materials.push(slot);
            self.previous = Some(color.to_string());
            self.start = start;
            self.count = count;
        } else {
            self.count += count;
        }
    }
    fn finish(mut self) -> (Vec<(usize, usize, usize)>, Vec<Slot>) {
        if self.count > 0 {
            self.groups
                .push((self.start, self.count, self.materials.len() - 1));
        }
        (self.groups, self.materials)
    }
}
