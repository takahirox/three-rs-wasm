//! webgl_modifier_subdivision: three-subdivide's LoopSubdivision.modify
//! ( v1.1.5, MIT, Stephens Nunnally ) beside the source geometry: the
//! optional coplanar edge pre-split, then flat or Loop-smoothed 1 → 4
//! triangle iterations under maxTriangles, with the library's two-decimal
//! position hashes, neighbour, opposite and edge maps, per-attribute
//! subdivision ( UVs flat unless uvSmooth, normals averaged over coincident
//! points, preserveEdges ) in double precision stored to Float32 arrays as
//! the library computes them. It runs on the CPU when a parameter changes,
//! as the page does; the meshes then stay resident. Both meshes share a
//! MeshStandardMaterial ( uv_grid_opengl or grey, flat shading, polygon
//! offset 1, 1 ) with optional white wireframe overlays, lit by the
//! hemisphere light and two directional lights, under OrbitControls at
//! rotateSpeed 0.5 on demand.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::collections::{HashMap, HashSet};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

const GEOMETRIES: [&str; 15] = [
    "box",
    "capsule",
    "circle",
    "cone",
    "cylinder",
    "dodecahedron",
    "icosahedron",
    "lathe",
    "octahedron",
    "plane",
    "ring",
    "sphere",
    "tetrahedron",
    "torus",
    "torusknot",
];

type Hash = [i32; 3];
type V = [f64; 4];

/// hashFromNumber: round( num × 100 ), JS `( x ± 0.5 ) << 0`.
fn hash_number(n: f64) -> i32 {
    (n * 100. + if n * 100. > 0. { 0.5 } else { -0.5 }) as i32
}
fn hash(v: V) -> Hash {
    [hash_number(v[0]), hash_number(v[1]), hash_number(v[2])]
}
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}
fn scale(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s, a[3] * s]
}
fn half(a: V, b: V) -> V {
    let s = add(a, b);
    [s[0] / 2., s[1] / 2., s[2] / 2., s[3] / 2.]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        0.,
    ]
}
fn length(a: V) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

/// A Float32 attribute as the library reads it: fromBufferAttribute into a
/// Vector3 ( UVs read past their item too, which setTriangle never writes ).
#[derive(Clone)]
struct Attr {
    array: Vec<f32>,
    size: usize,
}
impl Attr {
    fn count(&self) -> usize {
        self.array.len() / self.size
    }
    fn get(&self, i: usize) -> V {
        let mut v = [0.; 4];
        for (c, value) in v.iter_mut().enumerate().take(self.size) {
            *value = f64::from(self.array[i * self.size + c]);
        }
        v
    }
    fn with_capacity(size: usize) -> Self {
        Self {
            array: Vec::new(),
            size,
        }
    }
    /// setTriangle.
    fn push(&mut self, vs: [V; 3]) {
        for v in vs {
            self.array.extend(v[..self.size].iter().map(|&x| x as f32));
        }
    }
}

/// The non-indexed position, normal and uv ( gatherAttributes ).
#[derive(Clone)]
struct Geo {
    attrs: Vec<(&'static str, Attr)>,
}
impl Geo {
    fn from(g: &BufferGeometry) -> Result<Self> {
        let g = if g.index.is_some() {
            g.to_non_indexed()?
        } else {
            g.clone()
        };
        let mut attrs = vec![];
        for name in ["position", "normal", "uv"] {
            if let Some(Attribute::F32(a)) = g.get_attribute(name) {
                attrs.push((
                    name,
                    Attr {
                        array: a.array().to_vec(),
                        size: a.item_size(),
                    },
                ));
            }
        }
        Ok(Self { attrs })
    }
    fn get(&self, name: &str) -> Option<&Attr> {
        self.attrs.iter().find(|(n, _)| *n == name).map(|(_, a)| a)
    }
    fn position(&self) -> Result<&Attr> {
        self.get("position")
            .ok_or(Error::Invalid("subdivision position"))
    }
    fn geometry(&self) -> Result<BufferGeometry> {
        let mut g = BufferGeometry::default();
        for (name, a) in &self.attrs {
            g.set_attribute(
                *name,
                Attribute::F32(BufferAttribute::new(a.array.clone(), a.size, false)?),
            );
        }
        Ok(g)
    }
}

/// Insertion-ordered map, as the library's object keys iterate.
#[derive(Default)]
struct Ordered<K, T> {
    keys: Vec<K>,
    map: HashMap<K, T>,
}
impl<K: std::hash::Hash + Eq + Copy, T: Default> Ordered<K, T> {
    fn entry(&mut self, k: K) -> &mut T {
        if !self.map.contains_key(&k) {
            self.keys.push(k);
        }
        self.map.entry(k).or_default()
    }
}

#[derive(Clone, Copy)]
struct Params {
    split: bool,
    uv_smooth: bool,
    preserve_edges: bool,
    flat_only: bool,
    max_triangles: f64,
}

fn modify(g: &BufferGeometry, iterations: u32, p: Params) -> Result<BufferGeometry> {
    let mut geo = Geo::from(g)?;
    if p.split {
        geo = edge_split(&geo)?;
    }
    for _ in 0..iterations {
        if ((geo.position()?.count() / 3) as f64) < p.max_triangles {
            geo = if p.flat_only {
                flat(&geo)
            } else {
                smooth(&geo, p)?
            };
        }
    }
    geo.geometry()
}

fn edge_split(existing: &Geo) -> Result<Geo> {
    let pos = existing.position()?;
    let count = pos.count();
    type Edge = (Hash, Hash, Hash);
    let mut edge_triangles: HashMap<Edge, usize> = HashMap::new();
    let mut edge_length: HashMap<Edge, f64> = HashMap::new();
    let mut triangle_edges: Vec<[Edge; 3]> = vec![];
    let mut exists = vec![];
    for i in (0..count).step_by(3) {
        let [v0, v1, v2] = [pos.get(i), pos.get(i + 1), pos.get(i + 2)];
        let (h0, h1, h2) = (hash(v0), hash(v1), hash(v2));
        // Triangle.getArea, then fuzzy( size, 0 ).
        let area = length(cross(sub(v2, v1), sub(v0, v1))) * 0.5;
        let valid = !(area < 0.00001 && area > -0.00001);
        exists.push(valid);
        if !valid {
            triangle_edges.push([([0; 3], [0; 3], [0; 3]); 3]);
            continue;
        }
        // calcNormal: ( v1 − v2 ) × ( v0 − v1 ), normalized.
        let n = cross(sub(v1, v2), sub(v0, v1));
        let l = length(n);
        let n = if l == 0. { n } else { scale(n, 1. / l) };
        let hn = hash(n);
        let hashes = [
            (h0, h1, hn),
            (h1, h0, hn),
            (h1, h2, hn),
            (h2, h1, hn),
            (h2, h0, hn),
            (h0, h2, hn),
        ];
        for (j, h) in hashes.iter().enumerate() {
            *edge_triangles.entry(*h).or_default() += 1;
            let entry = edge_length.entry(*h).or_insert(0.);
            if *entry == 0. {
                *entry = match j {
                    0 | 1 => length(sub(v0, v1)),
                    2 | 3 => length(sub(v1, v2)),
                    _ => length(sub(v2, v0)),
                };
            }
        }
        triangle_edges.push([hashes[0], hashes[2], hashes[4]]);
    }
    let mut out = Geo { attrs: vec![] };
    for (name, a) in &existing.attrs {
        let mut f = Attr::with_capacity(a.size);
        for i in (0..count).step_by(3) {
            if !exists[i / 3] {
                continue;
            }
            let [v0, v1, v2] = [a.get(i), a.get(i + 1), a.get(i + 2)];
            let [e01, e12, e20] = triangle_edges[i / 3];
            let [c01, c12, c20] = [
                edge_triangles[&e01],
                edge_triangles[&e12],
                edge_triangles[&e20],
            ];
            if c01 + c12 + c20 - 3 == 0 {
                f.push([v0, v1, v2]);
                continue;
            }
            let [l01, l12, l20] = [edge_length[&e01], edge_length[&e12], edge_length[&e20]];
            if (l01 > l12 || c12 <= 1) && (l01 > l20 || c20 <= 1) && c01 > 1 {
                let center = half(v0, v1);
                if c20 > 1 {
                    let mid = half(v2, v0);
                    f.push([v0, center, mid]);
                    f.push([center, v2, mid]);
                } else {
                    f.push([v0, center, v2]);
                }
                if c12 > 1 {
                    let mid = half(v1, v2);
                    f.push([center, v1, mid]);
                    f.push([mid, v2, center]);
                } else {
                    f.push([v1, v2, center]);
                }
            } else if (l12 > l20 || c20 <= 1) && c12 > 1 {
                let center = half(v1, v2);
                if c01 > 1 {
                    let mid = half(v0, v1);
                    f.push([center, mid, v1]);
                    f.push([mid, center, v0]);
                } else {
                    f.push([v1, center, v0]);
                }
                if c20 > 1 {
                    let mid = half(v2, v0);
                    f.push([center, v2, mid]);
                    f.push([mid, v0, center]);
                } else {
                    f.push([v2, v0, center]);
                }
            } else if c20 > 1 {
                let center = half(v2, v0);
                if c12 > 1 {
                    let mid = half(v1, v2);
                    f.push([v2, center, mid]);
                    f.push([center, v1, mid]);
                } else {
                    f.push([v2, center, v1]);
                }
                if c01 > 1 {
                    let mid = half(v0, v1);
                    f.push([v0, mid, center]);
                    f.push([mid, v1, center]);
                } else {
                    f.push([v0, v1, center]);
                }
            } else {
                f.push([v0, v1, v2]);
            }
        }
        out.attrs.push((name, f));
    }
    Ok(out)
}

fn flat_attribute(a: &Attr) -> Attr {
    let mut f = Attr::with_capacity(a.size);
    for i in (0..a.count()).step_by(3) {
        let [v0, v1, v2] = [a.get(i), a.get(i + 1), a.get(i + 2)];
        let (m01, m12, m20) = (half(v0, v1), half(v1, v2), half(v2, v0));
        f.push([v0, m01, m20]);
        f.push([v1, m12, m01]);
        f.push([v2, m20, m12]);
        f.push([m01, m12, m20]);
    }
    f
}

fn flat(existing: &Geo) -> Geo {
    Geo {
        attrs: existing
            .attrs
            .iter()
            .map(|(n, a)| (*n, flat_attribute(a)))
            .collect(),
    }
}

fn smooth(existing: &Geo, p: Params) -> Result<Geo> {
    let flat = flat(existing);
    let pos = existing.position()?;
    let flat_pos = flat.position()?;
    let mut neighbors: HashMap<Hash, Ordered<Hash, Vec<usize>>> = HashMap::new();
    let mut opposites: HashMap<Hash, Vec<usize>> = HashMap::new();
    let mut edges: HashMap<Hash, HashSet<Hash>> = HashMap::new();
    for i in (0..pos.count()).step_by(3) {
        let v = [pos.get(i), pos.get(i + 1), pos.get(i + 2)];
        let h = v.map(hash);
        for (a, b, index) in [
            (0, 1, 1),
            (0, 2, 2),
            (1, 0, 0),
            (1, 2, 2),
            (2, 0, 0),
            (2, 1, 1),
        ] {
            neighbors
                .entry(h[a])
                .or_default()
                .entry(h[b])
                .push(i + index);
        }
        let h01 = hash(half(v[0], v[1]));
        let h12 = hash(half(v[1], v[2]));
        let h20 = hash(half(v[2], v[0]));
        opposites.entry(h01).or_default().push(i + 2);
        opposites.entry(h12).or_default().push(i);
        opposites.entry(h20).or_default().push(i + 1);
        for (a, e) in [(0, h01), (0, h20), (1, h01), (1, h12), (2, h12), (2, h20)] {
            edges.entry(h[a]).or_default().insert(e);
        }
    }
    let mut hash_to_index: HashMap<Hash, Vec<usize>> = HashMap::new();
    for i in 0..flat_pos.count() {
        hash_to_index
            .entry(hash(flat_pos.get(i)))
            .or_default()
            .push(i);
    }
    let mut out = Geo { attrs: vec![] };
    for (name, existing_attr) in &existing.attrs {
        let Some(flattened) = flat.get(name) else {
            continue;
        };
        let mut f = Attr::with_capacity(flattened.size);
        for i in (0..flat_pos.count()).step_by(3) {
            let mut vs = [[0.; 4]; 3];
            for (v, out_v) in vs.iter_mut().enumerate() {
                let index = i + v;
                *out_v = flattened.get(index);
                if *name == "uv" && !p.uv_smooth {
                    continue;
                }
                let position_hash = hash(flat_pos.get(index));
                if *name == "normal" {
                    let positions = &hash_to_index[&position_hash];
                    let k = positions.len() as f64;
                    let beta = 0.75 / k;
                    let mut value = scale(*out_v, 1. - beta * k);
                    for &j in positions {
                        value = add(value, scale(flattened.get(j), beta));
                    }
                    *out_v = value;
                    continue;
                }
                if let Some(neighbors) = neighbors.get(&position_hash) {
                    if p.preserve_edges
                        && edges[&position_hash]
                            .iter()
                            .any(|e| !opposites[e].len().is_multiple_of(2))
                    {
                        continue;
                    }
                    let k = neighbors.keys.len() as f64;
                    let c = 3. / 8. + 1. / 4. * (2. * PI / k).cos();
                    let beta = 1. / k * (5. / 8. - c * c);
                    // ( 1 − weight ) × heavy + weight × beta with the library's weight 1:
                    // the heavy term vanishes.
                    let weight = beta;
                    let mut value = scale(*out_v, 1. - weight * k);
                    for key in &neighbors.keys {
                        let indices = &neighbors.map[key];
                        let mut average = [0.; 4];
                        for &j in indices {
                            average = add(average, existing_attr.get(j));
                        }
                        let n = indices.len() as f64;
                        average = [
                            average[0] / n,
                            average[1] / n,
                            average[2] / n,
                            average[3] / n,
                        ];
                        value = add(value, scale(average, weight));
                    }
                    *out_v = value;
                } else if let Some(o) = opposites.get(&position_hash)
                    && o.len() == 2
                {
                    let mut value = scale(*out_v, 1. - 0.125 * 2.);
                    for &j in o {
                        value = add(value, scale(existing_attr.get(j), 0.125));
                    }
                    *out_v = value;
                }
            }
            f.push(vs);
        }
        out.attrs.push((name, f));
    }
    Ok(out)
}

fn source_geometry(index: usize) -> Result<BufferGeometry> {
    Ok(match GEOMETRIES[index] {
        "box" => BoxGeometry::build(1., 1., 1.)?,
        "capsule" => CapsuleGeometry::build(0.5, 0.5, 3, 5, 1)?,
        "circle" => CircleGeometry::build(0.6, 10, 0., TAU)?,
        "cone" => CylinderGeometry::build(0., 0.6, 1.5, 5, 3, false, 0., TAU)?,
        "cylinder" => CylinderGeometry::build(0.5, 0.5, 1., 5, 4, false, 0., TAU)?,
        "dodecahedron" => {
            let t = (1. + 5f64.sqrt()) / 2.;
            let r = 1. / t;
            let vertices = [
                [-1., -1., -1.],
                [-1., -1., 1.],
                [-1., 1., -1.],
                [-1., 1., 1.],
                [1., -1., -1.],
                [1., -1., 1.],
                [1., 1., -1.],
                [1., 1., 1.],
                [0., -r, -t],
                [0., -r, t],
                [0., r, -t],
                [0., r, t],
                [-r, -t, 0.],
                [-r, t, 0.],
                [r, -t, 0.],
                [r, t, 0.],
                [-t, 0., -r],
                [t, 0., -r],
                [-t, 0., r],
                [t, 0., r],
            ]
            .map(Vector3::from_array);
            let faces = [
                3, 11, 7, 3, 7, 15, 3, 15, 13, 7, 19, 17, 7, 17, 6, 7, 6, 15, 17, 4, 8, 17, 8, 10,
                17, 10, 6, 8, 0, 16, 8, 16, 2, 8, 2, 10, 0, 12, 1, 0, 1, 18, 0, 18, 16, 6, 10, 2,
                6, 2, 13, 6, 13, 15, 2, 16, 18, 2, 18, 3, 2, 3, 13, 18, 1, 9, 18, 9, 11, 18, 11, 3,
                4, 14, 12, 4, 12, 0, 4, 0, 8, 11, 9, 5, 11, 5, 19, 11, 19, 7, 19, 5, 14, 19, 14, 4,
                19, 4, 17, 1, 12, 14, 1, 14, 5, 1, 5, 9,
            ];
            PolyhedronGeometry::build(&vertices, &faces, 0.6, 0)?
        }
        "icosahedron" => IcosahedronGeometry::build(0.6, 0)?,
        "lathe" => {
            // The sine wave profile, centered.
            let points: Vec<Vector2> = (0..65)
                .step_by(5)
                .map(|i| {
                    let i = f64::from(i);
                    let x = ((i * 0.2).sin() * (i * 0.1).sin() * 15. + 50.) * 1.2;
                    let y = (i - 5.) * 3.;
                    Vector2::new(x * 0.0075, y * 0.005)
                })
                .collect();
            let mut g = LatheGeometry::build(&points, 4, 0., TAU)?;
            g.center()?;
            g
        }
        "octahedron" => OctahedronGeometry::build(0.7, 0)?,
        "plane" => PlaneGeometry::build(1., 1., 1, 1)?,
        "ring" => RingGeometry::build(0.3, 0.6, 10, 1, 0., TAU)?,
        "sphere" => SphereGeometry::build(0.6, 8, 4)?,
        "tetrahedron" => TetrahedronGeometry::build(0.8, 0)?,
        "torus" => TorusGeometry::build(0.48, 0.24, 4, 6, TAU, 0., TAU)?,
        _ => TorusKnotGeometry::build(0.38, 0.18, 20, 4, 2, 3)?,
    })
}

pub(super) struct Demo {
    controls: Controls,
    texture: Arc<Texture>,
    meshes: [Object3D; 2],
    wires: [Object3D; 2],
    wire_material: Arc<Material>,
    geometry: usize,
    iterations: u32,
    params: Params,
    flat_shading: bool,
    textured: bool,
    wireframe: bool,
    dirty: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0.7, 2.1);
        let hemisphere = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x737373),
            intensity: 3.,
        }));
        s.get_mut(hemisphere)?.position = Vector3::Y;
        for z in [1., -1.] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 1.5,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = Vector3::new(0., 1., z);
        }
        let mut texture = decode_texture_image(
            &fetch("/web/gallery/assets/environment-materials/textures/uv_grid_opengl.jpg").await?,
        )
        .await?;
        texture.srgb = true;
        texture.wrap_s = Wrapping::Repeat;
        texture.wrap_t = Wrapping::Repeat;
        texture.mipmap_filter = Some(Filter::Linear);
        let mut wire = MeshBasicMaterial::default();
        wire.properties.wireframe = true;
        let wire_material = Arc::new(Material::Basic(wire));
        let empty = Arc::new(BufferGeometry::default());
        let mut make = |x: f64, material: Arc<Material>, visible: bool| -> Result<Object3D> {
            let h = s.insert(NodeKind::Mesh(Mesh {
                geometry: empty.clone(),
                materials: vec![material],
            }));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(x, 0., 0.);
            n.visible = visible;
            Ok(h)
        };
        let placeholder = Arc::new(Material::Standard(MeshStandardMaterial::default()));
        let meshes = [
            make(-0.7, placeholder.clone(), true)?,
            make(0.7, placeholder, true)?,
        ];
        let wires = [
            make(-0.7, wire_material.clone(), false)?,
            make(0.7, wire_material.clone(), false)?,
        ];
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            texture: Arc::new(texture),
            meshes,
            wires,
            wire_material,
            geometry: 0,
            iterations: 3,
            params: Params {
                split: true,
                uv_smooth: false,
                preserve_edges: false,
                flat_only: false,
                max_triangles: 25000.,
            },
            flat_shading: false,
            textured: true,
            wireframe: false,
            dirty: true,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// updateMeshes() and updateMaterial() after a parameter change.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.dirty) {
            let normal = source_geometry(self.geometry)?;
            let smooth = modify(&normal, self.iterations, self.params)?;
            let name = GEOMETRIES[self.geometry];
            let mut material = MeshStandardMaterial {
                energy_conservation: true,
                ..Default::default()
            };
            let p = &mut material.properties;
            p.color = Color::from_hex(if self.textured { 0xffffff } else { 0x808080 });
            p.flat_shading = self.flat_shading;
            p.map = self.textured.then(|| self.texture.clone());
            p.polygon_offset = Some((1., 1));
            if matches!(name, "circle" | "lathe" | "plane" | "ring") {
                p.side = Side::Double;
            }
            let material = Arc::new(Material::Standard(material));
            for (k, geometry) in [normal, smooth].into_iter().enumerate() {
                let geometry = Arc::new(geometry);
                if let NodeKind::Mesh(m) = &mut s.get_mut(self.meshes[k])?.kind {
                    m.geometry = geometry.clone();
                    m.materials = vec![material.clone()];
                }
                if let NodeKind::Mesh(m) = &mut s.get_mut(self.wires[k])?.kind {
                    m.geometry = geometry;
                    m.materials = vec![self.wire_material.clone()];
                }
            }
        }
        for h in self.wires {
            s.get_mut(h)?.visible = self.wireframe;
        }
        self.controls.update(s, c)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            // rotateSpeed = 0.5.
            self.controls.rotate(dx * 0.5, dy * 0.5, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value != 0.;
        match index {
            0 => {
                self.geometry = (value as usize).min(GEOMETRIES.len() - 1);
                let name = GEOMETRIES[self.geometry];
                self.params.split = matches!(name, "box" | "ring" | "plane");
                self.params.uv_smooth = matches!(name, "circle" | "plane" | "ring");
            }
            1 => self.iterations = value as u32,
            2 => self.params.split = on,
            3 => self.params.uv_smooth = on,
            4 => self.params.preserve_edges = on,
            5 => self.params.flat_only = on,
            6 => self.params.max_triangles = f64::from(value),
            7 => self.flat_shading = on,
            8 => self.textured = on,
            9 => {
                self.wireframe = on;
                return Ok(());
            }
            _ => return Err(Error::Invalid("subdivision parameter")),
        }
        self.dirty = true;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
