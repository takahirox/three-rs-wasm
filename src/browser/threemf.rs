//! ThreeMFLoader: the seven 3MF samples (colors, textures, beam lattices,
//! components) and the textured, shadowed truck.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_image, fetch};
use super::helpers_formats::formats::{Element, parse_xml, unzip};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const SAMPLES: [&str; 7] = [
    "cube_gears",
    "facecolors",
    "multipletextures",
    "pyramid",
    "spinal implant",
    "variable voronoi",
    "vertexcolors",
];
fn bad(what: &str) -> Error {
    Error::Asset(format!("3MF: {what}"))
}
fn local(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}
/// querySelectorAll by local name, in document order.
fn descendants<'a>(e: &'a Element, name: &str, out: &mut Vec<&'a Element>) {
    for c in e.elements() {
        if local(&c.name) == name {
            out.push(c);
        }
        descendants(c, name, out);
    }
}
fn all<'a>(e: &'a Element, name: &str) -> Vec<&'a Element> {
    let mut out = vec![];
    descendants(e, name, &mut out);
    out
}
fn first<'a>(e: &'a Element, name: &str) -> Option<&'a Element> {
    all(e, name).into_iter().next()
}
/// An attribute by local name ( getAttributeNS for prefixed ones ).
fn attr<'a>(e: &'a Element, name: &str) -> Option<&'a str> {
    e.attributes
        .iter()
        .find(|(k, _)| local(k) == name)
        .map(|(_, v)| v.as_str())
}
fn float(s: Option<&str>) -> f64 {
    s.and_then(|v| v.trim().parse().ok()).unwrap_or(f64::NAN)
}
/// Object.keys order: integer-like keys ascending, then the others as inserted.
fn js_key_order(keys: &mut Vec<String>) {
    let (mut ints, others): (Vec<String>, Vec<String>) = keys
        .drain(..)
        .partition(|k| k.parse::<u32>().is_ok_and(|v| v.to_string() == *k));
    ints.sort_by_key(|k| k.parse::<u32>().unwrap_or(0));
    keys.extend(ints);
    keys.extend(others);
}
/// parseTransform: the 3 × 4 column list into a Matrix4.
fn transform(s: &str) -> [f64; 16] {
    let t: Vec<f64> = s
        .split(' ')
        .map(|v| v.parse().unwrap_or(f64::NAN))
        .collect();
    let g = |i: usize| t.get(i).copied().unwrap_or(f64::NAN);
    [
        g(0),
        g(1),
        g(2),
        0.,
        g(3),
        g(4),
        g(5),
        0.,
        g(6),
        g(7),
        g(8),
        0.,
        g(9),
        g(10),
        g(11),
        1.,
    ]
}
struct Base {
    name: String,
    color: String,
    display: Option<String>,
}
struct Texture2d {
    path: String,
    tile_u: Option<String>,
    tile_v: Option<String>,
    filter: Option<String>,
}
#[derive(Clone)]
struct Triangle {
    v: [usize; 3],
    p: [Option<usize>; 3],
    pid: Option<String>,
}
struct Beam {
    v1: usize,
    v2: usize,
    r1: Option<f64>,
    r2: Option<f64>,
}
struct Lattice {
    radius: f64,
    min_length: f64,
    cap: String,
    ball_mode: String,
    ball_radius: f64,
    beams: Vec<Beam>,
    balls: Vec<(usize, Option<f64>)>,
}
struct MeshData {
    vertices: Vec<f32>,
    triangles: Vec<Triangle>,
    lattice: Option<Lattice>,
}
struct Object {
    name: Option<String>,
    pid: Option<String>,
    pindex: Option<usize>,
    mesh: Option<MeshData>,
    components: Vec<(String, Option<[f64; 16]>)>,
}
struct Model {
    basematerials: HashMap<String, Vec<Base>>,
    texture2d: HashMap<String, Texture2d>,
    colorgroup: HashMap<String, Vec<f32>>,
    metallic: HashMap<String, Vec<(f64, f64)>>,
    texture2dgroup: HashMap<String, (String, Vec<f32>)>,
    objects: Vec<(String, Object)>,
    build: Vec<(String, Option<[f64; 16]>)>,
}
fn parse_model(model: &Element) -> Result<Model> {
    let resources = first(model, "resources").ok_or_else(|| bad("resources"))?;
    let mut m = Model {
        basematerials: HashMap::new(),
        texture2d: HashMap::new(),
        colorgroup: HashMap::new(),
        metallic: HashMap::new(),
        texture2dgroup: HashMap::new(),
        objects: vec![],
        build: vec![],
    };
    for b in all(resources, "basematerials") {
        let bases = all(b, "base")
            .into_iter()
            .map(|e| Base {
                name: attr(e, "name").unwrap_or_default().into(),
                color: attr(e, "displaycolor").unwrap_or_default().into(),
                display: attr(e, "displaypropertiesid").map(str::to_string),
            })
            .collect();
        m.basematerials
            .insert(attr(b, "id").unwrap_or_default().into(), bases);
    }
    for t in all(resources, "texture2d") {
        m.texture2d.insert(
            attr(t, "id").unwrap_or_default().into(),
            Texture2d {
                path: attr(t, "path").unwrap_or_default().into(),
                tile_u: attr(t, "tilestyleu").map(str::to_string),
                tile_v: attr(t, "tilestylev").map(str::to_string),
                filter: attr(t, "filter").map(str::to_string),
            },
        );
    }
    for g in all(resources, "colorgroup") {
        let mut colors = vec![];
        for c in all(g, "color") {
            let hex = attr(c, "color").unwrap_or_default();
            let v = u32::from_str_radix(hex.get(1..7).unwrap_or("000000"), 16).unwrap_or(0);
            let c = Color::from_hex(v).0;
            colors.extend([c.x as f32, c.y as f32, c.z as f32]);
        }
        m.colorgroup
            .insert(attr(g, "id").unwrap_or_default().into(), colors);
    }
    for p in all(resources, "pbmetallicdisplayproperties") {
        let data = all(p, "pbmetallic")
            .into_iter()
            .map(|e| (float(attr(e, "metallicness")), float(attr(e, "roughness"))))
            .collect();
        m.metallic
            .insert(attr(p, "id").unwrap_or_default().into(), data);
    }
    for g in all(resources, "texture2dgroup") {
        let mut uvs = vec![];
        for c in all(g, "tex2coord") {
            uvs.extend([float(attr(c, "u")) as f32, float(attr(c, "v")) as f32]);
        }
        m.texture2dgroup.insert(
            attr(g, "id").unwrap_or_default().into(),
            (attr(g, "texid").unwrap_or_default().into(), uvs),
        );
    }
    let mut objects = vec![];
    for o in all(resources, "object") {
        let mesh = first(o, "mesh").map(|mesh| {
            let mut vertices = vec![];
            for v in all(mesh, "vertices")
                .into_iter()
                .flat_map(|e| all(e, "vertex"))
            {
                vertices.extend(["x", "y", "z"].map(|k| float(attr(v, k)) as f32));
            }
            let mut triangles = vec![];
            for t in all(mesh, "triangles")
                .into_iter()
                .flat_map(|e| all(e, "triangle"))
            {
                let int = |k: &str| attr(t, k).and_then(|v| v.parse::<usize>().ok());
                triangles.push(Triangle {
                    v: [
                        int("v1").unwrap_or(0),
                        int("v2").unwrap_or(0),
                        int("v3").unwrap_or(0),
                    ],
                    p: [int("p1"), int("p2"), int("p3")],
                    pid: attr(t, "pid").map(str::to_string),
                });
            }
            let lattice = first(mesh, "beamlattice").map(|b| Lattice {
                radius: float(attr(b, "radius")),
                min_length: float(attr(b, "minlength")),
                cap: attr(b, "cap").unwrap_or("sphere").into(),
                ball_mode: attr(b, "ballmode").unwrap_or("none").into(),
                ball_radius: float(attr(b, "ballradius")),
                beams: all(b, "beam")
                    .into_iter()
                    .map(|e| Beam {
                        v1: attr(e, "v1").and_then(|v| v.parse().ok()).unwrap_or(0),
                        v2: attr(e, "v2").and_then(|v| v.parse().ok()).unwrap_or(0),
                        r1: attr(e, "r1").map(|v| float(Some(v))),
                        r2: attr(e, "r2").map(|v| float(Some(v))),
                    })
                    .collect(),
                balls: all(b, "ball")
                    .into_iter()
                    .map(|e| {
                        (
                            attr(e, "vindex").and_then(|v| v.parse().ok()).unwrap_or(0),
                            attr(e, "r").map(|v| float(Some(v))),
                        )
                    })
                    .collect(),
            });
            MeshData {
                vertices,
                triangles,
                lattice,
            }
        });
        let components = first(o, "components")
            .map(|c| {
                all(c, "component")
                    .into_iter()
                    .map(|e| {
                        (
                            attr(e, "objectid").unwrap_or_default().to_string(),
                            attr(e, "transform").map(transform),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        objects.push((
            attr(o, "id").unwrap_or_default().to_string(),
            Object {
                name: attr(o, "name").map(str::to_string),
                pid: attr(o, "pid").map(str::to_string),
                pindex: attr(o, "pindex").and_then(|v| v.parse().ok()),
                mesh,
                components,
            },
        ));
    }
    // resourcesData.object is keyed by id: Object.keys order.
    let mut keys: Vec<String> = objects.iter().map(|(k, _)| k.clone()).collect();
    keys.dedup();
    js_key_order(&mut keys);
    let mut map: HashMap<String, Object> = objects.into_iter().collect();
    m.objects = keys
        .into_iter()
        .filter_map(|k| map.remove(&k).map(|o| (k, o)))
        .collect();
    if let Some(b) = first(model, "build") {
        for item in all(b, "item") {
            m.build.push((
                attr(item, "objectid").unwrap_or_default().to_string(),
                attr(item, "transform").map(transform),
            ));
        }
    }
    Ok(m)
}
/// A built object: a group of meshes or of transformed component clones.
#[derive(Clone)]
enum Built {
    Group {
        name: Option<String>,
        children: Vec<Built>,
        matrix: Option<[f64; 16]>,
    },
    Mesh {
        name: Option<String>,
        geometry: Arc<BufferGeometry>,
        material: Arc<Material>,
    },
}
fn positions(vertices: &[f32], triangles: &[Triangle]) -> Vec<f32> {
    let mut out = Vec::with_capacity(triangles.len() * 9);
    for t in triangles {
        for &v in &t.v {
            out.extend_from_slice(&vertices[v * 3..v * 3 + 3]);
        }
    }
    out
}
fn attribute(data: Vec<f32>, size: usize) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, size, false)?))
}
fn phong(color: Color, flat: bool) -> MeshPhongMaterial {
    let mut m = MeshPhongMaterial::default();
    m.properties.color = color;
    m.properties.flat_shading = flat;
    m
}
struct Builder<'a> {
    model: &'a Model,
    textures: &'a HashMap<String, Arc<Texture>>,
    materials: HashMap<(String, usize), Arc<Material>>,
    maps: HashMap<String, Arc<Texture>>,
    built: HashMap<String, Built>,
}
impl Builder<'_> {
    fn basematerial(&mut self, id: &str, index: usize) -> Result<Arc<Material>> {
        if let Some(m) = self.materials.get(&(id.to_string(), index)) {
            return Ok(m.clone());
        }
        let base = self.model.basematerials[id]
            .get(index)
            .ok_or_else(|| bad("basematerial index"))?;
        let hex = u32::from_str_radix(base.color.get(1..7).unwrap_or("ffffff"), 16).unwrap_or(0);
        let color = Color::from_hex(hex);
        let opacity = if base.color.len() == 9 {
            u32::from_str_radix(&base.color[7..9], 16).unwrap_or(255) as f64 / 255.
        } else {
            1.
        };
        let material = match base
            .display
            .as_ref()
            .and_then(|d| self.model.metallic.get(d))
            .and_then(|d| d.get(index))
        {
            Some(&(metalness, roughness)) => {
                let mut m = MeshStandardMaterial {
                    metalness,
                    roughness,
                    ..Default::default()
                };
                m.properties.color = color;
                m.properties.flat_shading = true;
                m.properties.opacity = opacity;
                Material::Standard(m)
            }
            None => {
                let mut m = phong(color, true);
                m.properties.opacity = opacity;
                Material::Phong(m)
            }
        };
        let _ = &base.name;
        let m = Arc::new(material);
        self.materials.insert((id.to_string(), index), m.clone());
        Ok(m)
    }
    /// buildTexture: the image with the tile styles and filter.
    fn texture(&mut self, group: &str) -> Result<Option<Arc<Texture>>> {
        if let Some(t) = self.maps.get(group) {
            return Ok(Some(t.clone()));
        }
        let (texid, _) = &self.model.texture2dgroup[group];
        let Some(t2d) = self.model.texture2d.get(texid) else {
            return Ok(None);
        };
        let Some(source) = self.textures.get(&t2d.path) else {
            return Ok(None);
        };
        let mut t = (**source).clone();
        let wrap = |s: &Option<String>| match s.as_deref() {
            Some("mirror") => Wrapping::Mirror,
            Some("none") | Some("clamp") => Wrapping::Clamp,
            _ => Wrapping::Repeat,
        };
        t.wrap_s = wrap(&t2d.tile_u);
        t.wrap_t = wrap(&t2d.tile_v);
        match t2d.filter.as_deref() {
            Some("linear") => {
                t.filter = Filter::Linear;
                t.min_filter = Some(Filter::Linear);
                t.mipmap_filter = None;
            }
            Some("nearest") => {
                t.filter = Filter::Nearest;
                t.min_filter = Some(Filter::Nearest);
                t.mipmap_filter = None;
            }
            _ => {
                t.filter = Filter::Linear;
                t.mipmap_filter = Some(Filter::Linear);
            }
        }
        let t = Arc::new(t);
        self.maps.insert(group.to_string(), t.clone());
        Ok(Some(t))
    }
    fn mesh_group(&mut self, object: &Object) -> Result<Built> {
        let mesh = object.mesh.as_ref().ok_or_else(|| bad("mesh"))?;
        // analyzeObject: triangles by resource id, in Object.keys order.
        let mut map: HashMap<String, Vec<Triangle>> = HashMap::new();
        let mut order = vec![];
        for t in &mesh.triangles {
            let pid = t
                .pid
                .clone()
                .or_else(|| object.pid.clone())
                .unwrap_or_else(|| "default".into());
            if !map.contains_key(&pid) {
                order.push(pid.clone());
            }
            map.entry(pid).or_default().push(t.clone());
        }
        js_key_order(&mut order);
        let mut meshes = vec![];
        for pid in order {
            let triangles = &map[&pid];
            if self.model.texture2dgroup.contains_key(&pid) {
                let uvs = &self.model.texture2dgroup[&pid].1;
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    attribute(positions(&mesh.vertices, triangles), 3)?,
                );
                let mut uv = vec![];
                for t in triangles {
                    for p in t.p {
                        let p = p.unwrap_or(0);
                        uv.extend([uvs[p * 2], uvs[p * 2 + 1]]);
                    }
                }
                g.set_attribute("uv", attribute(uv, 2)?);
                let mut m = phong(Color::WHITE, true);
                m.properties.map = self.texture(&pid)?;
                meshes.push((Arc::new(g), Arc::new(Material::Phong(m))));
            } else if self.model.basematerials.contains_key(&pid) {
                // buildBasematerialsMeshes: one mesh per material index, Object.keys order.
                let mut by: HashMap<String, Vec<&Triangle>> = HashMap::new();
                let mut keys = vec![];
                for t in triangles {
                    let index = t.p[0]
                        .or(object.pindex)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "undefined".into());
                    if !by.contains_key(&index) {
                        keys.push(index.clone());
                    }
                    by.entry(index).or_default().push(t);
                }
                js_key_order(&mut keys);
                for key in keys {
                    let list: Vec<Triangle> = by[&key].iter().map(|t| (*t).clone()).collect();
                    let index = key.parse::<usize>().map_err(|_| bad("material index"))?;
                    let material = self.basematerial(&pid, index)?;
                    let mut g = BufferGeometry::default();
                    g.set_attribute("position", attribute(positions(&mesh.vertices, &list), 3)?);
                    meshes.push((Arc::new(g), material));
                }
            } else if self.model.colorgroup.contains_key(&pid) {
                let colors = &self.model.colorgroup[&pid];
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    attribute(positions(&mesh.vertices, triangles), 3)?,
                );
                let mut c = vec![];
                for t in triangles {
                    let p1 = t.p[0].or(object.pindex).unwrap_or(0);
                    for p in [p1, t.p[1].unwrap_or(p1), t.p[2].unwrap_or(p1)] {
                        c.extend_from_slice(&colors[p * 3..p * 3 + 3]);
                    }
                }
                g.set_attribute("color", attribute(c, 3)?);
                let mut m = phong(Color::WHITE, true);
                m.properties.vertex_colors = true;
                meshes.push((Arc::new(g), Arc::new(Material::Phong(m))));
            } else if pid == "default" {
                let mut g = BufferGeometry::default();
                g.set_attribute("position", attribute(mesh.vertices.clone(), 3)?);
                g.set_index(Some(
                    mesh.triangles
                        .iter()
                        .flat_map(|t| t.v.map(|v| v as u32))
                        .collect(),
                ));
                meshes.push((
                    Arc::new(g),
                    Arc::new(Material::Phong(phong(Color::WHITE, true))),
                ));
            }
        }
        if let Some(lattice) = &mesh.lattice {
            meshes.push((
                Arc::new(lattice_geometry(&mesh.vertices, lattice)?),
                Arc::new(Material::Phong(phong(Color::WHITE, false))),
            ));
        }
        Ok(Built::Group {
            name: object.name.clone(),
            children: meshes
                .into_iter()
                .map(|(geometry, material)| Built::Mesh {
                    name: object.name.clone(),
                    geometry,
                    material,
                })
                .collect(),
            matrix: None,
        })
    }
    fn object(&mut self, id: &str) -> Result<Built> {
        if let Some(b) = self.built.get(id) {
            return Ok(b.clone());
        }
        let object = &self
            .model
            .objects
            .iter()
            .find(|(k, _)| k == id)
            .ok_or_else(|| bad("object id"))?
            .1;
        let built = if object.mesh.is_some() {
            self.mesh_group(object)?
        } else {
            // buildComposite: clones of the components with their transforms.
            let mut children = vec![];
            for (child, matrix) in &object.components {
                let mut b = self.object(child)?;
                if let (Some(m), Built::Group { matrix: slot, .. }) = (matrix, &mut b) {
                    *slot = Some(*m);
                }
                children.push(b);
            }
            Built::Group {
                name: object.name.clone(),
                children,
                matrix: None,
            }
        };
        self.built.insert(id.to_string(), built.clone());
        Ok(built)
    }
}
/// buildLatticeMesh: a cylinder per beam and a sphere per ball, merged.
fn lattice_geometry(vertices: &[f32], l: &Lattice) -> Result<BufferGeometry> {
    let vertex = |i: usize| {
        Vector3::new(
            vertices[i * 3] as f64,
            vertices[i * 3 + 1] as f64,
            vertices[i * 3 + 2] as f64,
        )
    };
    let open = l.cap != "butt";
    let mut parts = vec![];
    let mut radii: Vec<(usize, f64)> = vec![];
    let set =
        |radii: &mut Vec<(usize, f64)>, v: usize, r: f64| match radii.iter_mut().find(|e| e.0 == v)
        {
            Some(e) => e.1 = r.max(e.1),
            None => radii.push((v, r.max(0.))),
        };
    for beam in &l.beams {
        let (p1, p2) = (vertex(beam.v1), vertex(beam.v2));
        let length = p1.distance(p2);
        if length < l.min_length {
            continue;
        }
        let r1 = beam.r1.unwrap_or(l.radius);
        let r2 = beam.r2.unwrap_or(r1);
        let mut g = CylinderGeometry::build(r2, r1, length, 8, 1, open, 0., 2. * PI)?;
        let direction = (p2 - p1) / length;
        let q = Quaternion::from_rotation_arc(Vector3::Y, direction);
        g.apply_matrix4(Matrix4::from_rotation_translation(q, (p1 + p2) * 0.5))?;
        parts.push(g);
        if open {
            set(&mut radii, beam.v1, r1);
            set(&mut radii, beam.v2, r2);
        }
        if l.ball_mode == "all" {
            set(&mut radii, beam.v1, l.ball_radius);
            set(&mut radii, beam.v2, l.ball_radius);
        }
    }
    for &(v, r) in &l.balls {
        set(&mut radii, v, r.unwrap_or(l.ball_radius));
    }
    for (v, r) in radii {
        let mut g = SphereGeometry::build(r, 8, 6)?;
        g.translate(vertex(v))?;
        parts.push(g);
    }
    merge(parts)
}
/// mergeGeometries for indexed position/normal/uv geometries.
fn merge(parts: Vec<BufferGeometry>) -> Result<BufferGeometry> {
    let (mut p, mut n, mut uv, mut index) = (vec![], vec![], vec![], vec![]);
    for g in parts {
        let base = (p.len() / 3) as u32;
        let get = |name: &str| match g.attributes.get(name) {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => vec![],
        };
        p.extend(get("position"));
        n.extend(get("normal"));
        uv.extend(get("uv"));
        match &g.index {
            Some(i) => index.extend(i.iter().map(|&v| v + base)),
            None => return Err(bad("merge of non-indexed geometry")),
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", attribute(p, 3)?);
    g.set_attribute("normal", attribute(n, 3)?);
    g.set_attribute("uv", attribute(uv, 2)?);
    g.set_index(Some(index));
    Ok(g)
}
/// The loaded document: the root model's build items as built objects.
async fn load(path: &str) -> Result<Vec<(Built, Option<[f64; 16]>)>> {
    let files = unzip(&fetch(path).await?)?;
    let text = |name: &str| {
        files
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, b)| String::from_utf8_lossy(b).into_owned())
    };
    let root = files
        .iter()
        .map(|(n, _)| n.as_str())
        .find(|n| n.starts_with("3D/") && n.ends_with(".model") && n.matches('/').count() == 1)
        .ok_or_else(|| bad("root model"))?
        .to_string();
    let document = parse_xml(&text(&root).ok_or_else(|| bad("model"))?)?;
    let model_element = if local(&document.name) == "model" {
        &document
    } else {
        first(&document, "model").ok_or_else(|| bad("model"))?
    };
    let model = parse_model(model_element)?;
    // Textures referenced by the model relationships, decoded as TextureLoader does.
    let mut textures = HashMap::new();
    for t in model.texture2d.values() {
        let key = t.path.trim_start_matches('/');
        if let Some((_, bytes)) = files.iter().find(|(n, _)| n == key) {
            // WebGL uploads the blob image without its embedded ICC profile (measured).
            let mut texture = decode_image(bytes).await?;
            texture.srgb = true;
            texture.mipmap_filter = Some(Filter::Linear);
            textures.insert(t.path.clone(), Arc::new(texture));
        }
    }
    let mut builder = Builder {
        model: &model,
        textures: &textures,
        materials: HashMap::new(),
        maps: HashMap::new(),
        built: HashMap::new(),
    };
    for (id, _) in &model.objects {
        builder.object(id)?;
    }
    let mut items = vec![];
    for (id, matrix) in &model.build {
        items.push((builder.object(id)?, *matrix));
    }
    Ok(items)
}
fn instantiate(s: &mut Scene, b: &Built, extra: Option<[f64; 16]>) -> Result<Object3D> {
    let h = match b {
        Built::Group {
            name,
            children,
            matrix,
        } => {
            let g = s.insert(NodeKind::Group);
            if let Some(name) = name {
                s.get_mut(g)?.name = name.clone();
            }
            for c in children {
                let c = instantiate(s, c, None)?;
                s.add(g, c)?;
            }
            if let Some(m) = matrix {
                apply(s, g, m)?;
            }
            g
        }
        Built::Mesh {
            name,
            geometry,
            material,
        } => {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                material.clone(),
            )));
            if let Some(name) = name {
                s.get_mut(h)?.name = name.clone();
            }
            h
        }
    };
    if let Some(m) = extra {
        apply(s, h, &m)?;
    }
    Ok(h)
}
/// object.applyMatrix4( matrix ).
fn apply(s: &mut Scene, h: Object3D, m: &[f64; 16]) -> Result<()> {
    let n = s.get_mut(h)?;
    n.update_matrix();
    let (scale, rotation, translation) =
        (Matrix4::from_cols_array(m) * n.matrix).to_scale_rotation_translation();
    (n.position, n.quaternion, n.scale) = (translation, rotation, scale);
    Ok(())
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    controls: Option<Controls>,
    /// The loaded sample documents, and the shown object.
    samples: Vec<Vec<(Built, Option<[f64; 16]>)>>,
    object: Option<Object3D>,
    asset: Option<usize>,
    pending: Option<usize>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            318 => (35., 1., 500., Vector3::new(-100., -250., 100.)),
            _ => (35., 1., 500., Vector3::new(-50., 40., 50.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        let mut d = Self {
            id,
            time: 0.,
            controls: None,
            samples: vec![],
            object: None,
            asset: None,
            pending: Some(0),
        };
        match id {
            318 => d.samples_scene(s, c).await?,
            _ => d.truck_scene(s, c).await?,
        }
        Ok(d)
    }
    async fn samples_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x333333);
        s.get_mut(c)?.up = Vector3::Z;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.6,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.,
            target: Vector3::ZERO,
        }));
        // The light is a child of the scene; the camera is added too.
        s.get_mut(light)?.position = Vector3::new(-1., -2.5, 1.);
        for name in SAMPLES {
            self.samples
                .push(load(&format!("{ASSETS}/threemf/{name}.3mf")).await?);
        }
        let mut controls = Controls::new(None, (50., 400.), PI, true);
        controls.up = Vector3::Z;
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.show(s, c, 0)?;
        Ok(())
    }
    async fn truck_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0xa0a0a0);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xa0a0a0),
            near: 10.,
            far: 500.,
        });
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x8d8d8d),
            intensity: 3.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(0., 100., 0.);
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 3.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(0., 40., 50.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            far: 100.,
            ..Default::default()
        };
        let items = load(&format!("{ASSETS}/threemf/truck.3mf")).await?;
        let group = s.insert(NodeKind::Group);
        for (b, m) in &items {
            let h = instantiate(s, b, *m)?;
            s.add(group, h)?;
        }
        s.get_mut(group)?.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        for h in s.traverse(group, false)? {
            s.get_mut(h)?.cast_shadow = true;
        }
        let mut m = MeshPhongMaterial::default();
        m.properties.color = Color::from_hex(0xcbcbcb);
        m.properties.depth_write = false;
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1000., 1000., 1, 1)?),
            Arc::new(Material::Phong(m)),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = 11.;
        n.receive_shadow = true;
        let mut controls = Controls::new(None, (50., 200.), PI, true);
        controls.set_target(Vector3::new(0., 20., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.pending = None;
        Ok(())
    }
    /// loadAsset, then manager.onLoad: the new object centered by its world
    /// box, the previous one removed and the controls reset.
    fn show(&mut self, s: &mut Scene, c: Object3D, index: usize) -> Result<()> {
        let group = s.insert(NodeKind::Group);
        for (b, m) in &self.samples[index] {
            let h = instantiate(s, b, *m)?;
            s.add(group, h)?;
        }
        s.update_world_matrix(group, false, true)?;
        let mut min = Vector3::splat(f64::INFINITY);
        let mut max = Vector3::splat(f64::NEG_INFINITY);
        for h in s.traverse(group, false)? {
            let n = s.get(h)?;
            let Some(g) = n.geometry() else { continue };
            let mut g = (**g).clone();
            let b = g.compute_bounding_box()?;
            // Box3.applyMatrix4: the eight transformed corners.
            for k in 0..8 {
                let p = Vector3::new(
                    if k & 1 == 0 { b.min.x } else { b.max.x },
                    if k & 2 == 0 { b.min.y } else { b.max.y },
                    if k & 4 == 0 { b.min.z } else { b.max.z },
                );
                let p = n.matrix_world.transform_point3(p);
                min = min.min(p);
                max = max.max(p);
            }
        }
        let center = (min + max) * 0.5;
        s.get_mut(group)?.position = -center;
        if let Some(old) = self.object.replace(group) {
            s.dispose(old)?;
        }
        self.asset = Some(index);
        // controls.reset(): the saved position and target, then update().
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-100., -250., 100.);
        if let Some(controls) = &mut self.controls {
            controls.set_target(Vector3::ZERO);
            controls.update(s, c)?;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(index) = self.pending.take()
            && self.asset != Some(index)
            && !self.samples.is_empty()
        {
            self.show(s, c, index)?;
        }
        Ok(())
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            // enablePan = false.
            return Ok(());
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    /// The asset menu.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (318, 0) => self.pending = Some((value as usize).min(SAMPLES.len() - 1)),
            _ => return Err(Error::Invalid("3MF parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
