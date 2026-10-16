//! USDComposer ( examples/jsm/loaders/usd/USDComposer.js ) and USDLoader's
//! parse(): `specsByPath` into the loader's group, step by step as the
//! original builds it — the same object, material and texture creation,
//! JavaScript's value coercions, and three.js's math.
use super::js::{Obj, Typed, Value};
use super::{
    Spec, Specs, UsdClip, UsdImage, UsdMaterial, UsdNode, UsdNodeKind, UsdScene, UsdSkeleton,
    UsdTexture, UsdTrack, UsdTrackKind,
};
use crate::attribute::BufferAttribute;
use crate::fbx::math::{Euler, M3, M4, Order, Q, V3, srgb_to_linear};
use crate::geometry::{
    Attribute, BoxGeometry, BufferGeometry, CapsuleGeometry, CylinderGeometry, SphereGeometry,
};
use crate::js_math::{atan, cos, log, pow, sin};
use crate::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::rc::Rc;

const PRIM: f64 = 6.0;
const ATTRIBUTE: f64 = 1.0;
const RELATIONSHIP: f64 = 8.0;
const REPEAT: u32 = 1000;
const CLAMP: u32 = 1001;
const MIRROR: u32 = 1002;
const EPSILON: f64 = f64::EPSILON;

fn type_error(what: &str) -> Error {
    Error::Asset(format!("THREE.USDLoader: {what}"))
}
/// `Math.max( a, b )` ( NaN through ).
fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}
fn num(v: &Value) -> f64 {
    v.to_number()
}
fn s(v: &str) -> Value {
    Value::string(v)
}
fn is_str(v: &Value, s: &str) -> bool {
    v.as_str() == Some(s)
}
/// `str.split( sep ).pop()`.
fn last_part(s: &str, sep: char) -> &str {
    s.rsplit(sep).next().unwrap_or("")
}
/// `/<|>/g` removed.
fn strip_angles(s: &str) -> String {
    s.replace(['<', '>'], "")
}

/// The archive's files: parsed layers and raw image bytes, in the zip's
/// order ( the keys of `unzipSync`'s object ).
#[derive(Clone)]
enum Asset {
    Layer(Rc<Specs>),
    Bytes(Rc<Vec<u8>>),
}
#[derive(Clone, Default)]
struct Assets {
    order: Vec<String>,
    map: HashMap<String, Asset>,
}
impl Assets {
    fn get(&self, k: &str) -> Option<&Asset> {
        self.map.get(k)
    }
    fn set(&mut self, k: &str, v: Asset) {
        if !self.map.contains_key(k) {
            self.order.push(k.into());
        }
        self.map.insert(k.into(), v);
    }
}

#[derive(Clone, Debug)]
enum Kind {
    Group,
    Object3D,
    Bone,
    Mesh {
        geometry: usize,
        materials: Vec<usize>,
        multi: bool,
        skin: Option<(Option<usize>, M4)>,
    },
    Camera(UsdNodeKind),
    Light(UsdNodeKind),
}

#[derive(Clone, Debug)]
struct Object {
    name: String,
    kind: Kind,
    position: V3,
    quaternion: Q,
    rotation: Euler,
    scale: V3,
    pivot: Option<V3>,
    parent: Option<usize>,
    children: Vec<usize>,
}

/// MeshPhysicalMaterial's properties the composer writes, with the values
/// it assigns.
#[derive(Clone, Debug)]
struct Material {
    color: [f64; 3],
    emissive: [f64; 3],
    specular_color: [f64; 3],
    attenuation_color: [f64; 3],
    emissive_intensity: Value,
    roughness: Value,
    metalness: Value,
    ior: Value,
    specular_intensity: Value,
    clearcoat: Value,
    clearcoat_roughness: Value,
    transmission: Value,
    thickness: Value,
    attenuation_distance: Value,
    iridescence: Value,
    iridescence_ior: Value,
    iridescence_thickness_range: [Value; 2],
    anisotropy: Value,
    opacity: Value,
    transparent: bool,
    alpha_test: Value,
    normal_scale: [f64; 2],
    maps: HashMap<&'static str, usize>,
}
impl Material {
    fn new() -> Self {
        Material {
            color: [1.0; 3],
            emissive: [0.0; 3],
            specular_color: [1.0; 3],
            attenuation_color: [1.0; 3],
            emissive_intensity: Value::Number(1.0),
            roughness: Value::Number(1.0),
            metalness: Value::Number(0.0),
            ior: Value::Number(1.5),
            specular_intensity: Value::Number(1.0),
            clearcoat: Value::Number(0.0),
            clearcoat_roughness: Value::Number(0.0),
            transmission: Value::Number(0.0),
            thickness: Value::Number(0.0),
            attenuation_distance: Value::Number(f64::INFINITY),
            iridescence: Value::Number(0.0),
            iridescence_ior: Value::Number(1.3),
            iridescence_thickness_range: [Value::Number(100.0), Value::Number(400.0)],
            anisotropy: Value::Number(0.0),
            opacity: Value::Number(1.0),
            transparent: false,
            alpha_test: Value::Number(0.0),
            normal_scale: [1.0, 1.0],
            maps: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Texture {
    image: UsdImage,
    srgb: bool,
    wrap: [u32; 2],
    repeat: [Value; 2],
    offset: [Value; 2],
    rotation: Value,
    channel: u32,
    scale: Option<Value>,
    bias: Option<Value>,
}

/// The objects, geometries, materials and textures every composer of a
/// parse creates.
#[derive(Default)]
struct World {
    objects: Vec<Object>,
    geometries: Vec<BufferGeometry>,
    materials: Vec<Material>,
    textures: Vec<Texture>,
    skeletons: Vec<(Vec<usize>, Vec<M4>)>,
}
impl World {
    fn object(&mut self, kind: Kind) -> usize {
        // DirectionalLight and SpotLight start at Object3D.DEFAULT_UP.
        let position = match &kind {
            Kind::Light(UsdNodeKind::DirectionalLight { .. } | UsdNodeKind::SpotLight { .. }) => {
                V3::new(0.0, 1.0, 0.0)
            }
            _ => V3::ZERO,
        };
        self.objects.push(Object {
            name: String::new(),
            kind,
            position,
            quaternion: Q::IDENTITY,
            rotation: Euler::new(0.0, 0.0, 0.0, Order::Xyz),
            scale: V3::ONE,
            pivot: None,
            parent: None,
            children: vec![],
        });
        self.objects.len() - 1
    }
    /// Object3D.add(): the child leaves its former parent.
    fn add(&mut self, parent: usize, child: usize) {
        if parent == child {
            return;
        }
        self.remove(child);
        self.objects[child].parent = Some(parent);
        self.objects[parent].children.push(child);
    }
    fn remove(&mut self, child: usize) {
        if let Some(old) = self.objects[child].parent.take() {
            self.objects[old].children.retain(|&c| c != child);
        }
    }
    /// quaternion.set(): the rotation follows ( setFromQuaternion ).
    fn set_quaternion(&mut self, o: usize, q: Q) {
        let obj = &mut self.objects[o];
        obj.quaternion = q;
        obj.rotation = Euler::from_quaternion(q, obj.rotation.order);
    }
    /// rotation.set(): the quaternion follows ( setFromEuler ).
    fn set_rotation(&mut self, o: usize, e: Euler) {
        let obj = &mut self.objects[o];
        obj.rotation = e;
        obj.quaternion = Q::from_euler(e);
    }
    fn is_mesh(&self, o: usize) -> bool {
        matches!(self.objects[o].kind, Kind::Mesh { .. })
    }
}

/// Matrix4.set( n11, n12, … ) from a USD row-major array ( the transpose ).
fn usd_matrix(m: &Value) -> M4 {
    let e = |i: usize| num(&m.at(i));
    M4([
        e(0),
        e(1),
        e(2),
        e(3),
        e(4),
        e(5),
        e(6),
        e(7),
        e(8),
        e(9),
        e(10),
        e(11),
        e(12),
        e(13),
        e(14),
        e(15),
    ])
}
fn translation(x: f64, y: f64, z: f64) -> M4 {
    M4([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
    ])
}
fn scaling(x: f64, y: f64, z: f64) -> M4 {
    M4([
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}
fn rotation_x(t: f64) -> M4 {
    let (c, s) = (cos(t), sin(t));
    M4([
        1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}
fn rotation_y(t: f64) -> M4 {
    let (c, s) = (cos(t), sin(t));
    M4([
        c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}
fn rotation_z(t: f64) -> M4 {
    let (c, s) = (cos(t), sin(t));
    M4([
        c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}

/// LoaderUtils.resolveURL( url, path ).
fn resolve_url(url: &str, path: &str) -> String {
    if url.is_empty() {
        return String::new();
    }
    let lower = url.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || url.starts_with("//") {
        return url.into();
    }
    if lower.starts_with("data:") && url[5..].contains(',') && !url.contains(['\n', '\r']) {
        return url.into();
    }
    if lower.starts_with("blob:") && !url.contains(['\n', '\r']) {
        return url.into();
    }
    let mut path = path.to_string();
    let lp = path.to_ascii_lowercase();
    if (lp.starts_with("http://") || lp.starts_with("https://")) && url.starts_with('/') {
        let scheme = if lp.starts_with("https://") { 8 } else { 7 };
        let host_end = path[scheme..].find('/').map_or(path.len(), |i| scheme + i);
        path.truncate(host_end);
    }
    path + url
}

/// The first `@([^@]+)@(?:<([^>]+)>)?` match from `from`: its range and
/// groups.
fn reference_match(s: &str, from: usize) -> Option<(usize, usize, String, Option<String>)> {
    let b = s.as_bytes();
    let mut i = from;
    while i < b.len() {
        if b[i] == b'@' {
            let rest = &s[i + 1..];
            if let Some(n) = rest.find('@')
                && n > 0
            {
                let file = rest[..n].to_string();
                let mut end = i + 1 + n + 1;
                let mut prim = None;
                let after = &s[end..];
                if let Some(t) = after.strip_prefix('<')
                    && let Some(m) = t.find('>')
                    && m > 0
                {
                    prim = Some(t[..m].to_string());
                    end += 1 + m + 1;
                }
                return Some((i, end, file, prim));
            }
        }
        i += s[i..].chars().next().map_or(1, char::len_utf8);
    }
    None
}

/// VARIANT_PATH_REGEX `^(.+?)\/\{(\w+)=(\w+)\}\/(.+)$`: the base and
/// relative paths.
fn variant_path_match(path: &str) -> Option<(&str, &str)> {
    let word = |s: &str| {
        s.char_indices()
            .find(|&(_, c)| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(s.len(), |(i, _)| i)
    };
    if path.contains(['\n', '\r', '\u{2028}', '\u{2029}']) {
        return None;
    }
    for (k, _) in path.char_indices().skip(1) {
        let rest = &path[k..];
        let Some(r) = rest.strip_prefix("/{") else {
            continue;
        };
        let w1 = word(r);
        if w1 == 0 || !r[w1..].starts_with('=') {
            continue;
        }
        let r2 = &r[w1 + 1..];
        let w2 = word(r2);
        if w2 == 0 {
            continue;
        }
        let Some(tail) = r2[w2..].strip_prefix("}/") else {
            continue;
        };
        if tail.is_empty() {
            continue;
        }
        return Some((&path[..k], tail));
    }
    None
}

/// An ordered map with Map's insertion order.
#[derive(Clone, Debug)]
struct OMap<V> {
    order: Vec<String>,
    map: HashMap<String, V>,
}
impl<V> Default for OMap<V> {
    fn default() -> Self {
        OMap {
            order: vec![],
            map: HashMap::new(),
        }
    }
}
impl<V> OMap<V> {
    fn get(&self, k: &str) -> Option<&V> {
        self.map.get(k)
    }
    fn set(&mut self, k: &str, v: V) {
        if !self.map.contains_key(k) {
            self.order.push(k.into());
        }
        self.map.insert(k.into(), v);
    }
    fn entry(&mut self, k: &str, f: impl FnOnce() -> V) -> &mut V {
        if !self.map.contains_key(k) {
            self.order.push(k.into());
            self.map.insert(k.into(), f());
        }
        self.map.get_mut(k).unwrap_or_else(|| unreachable!())
    }
    fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.order
            .iter()
            .filter_map(|k| self.map.get(k).map(|v| (k, v)))
    }
    fn len(&self) -> usize {
        self.order.len()
    }
}

struct SkinRecord {
    mesh: usize,
    skeleton_path: Value,
    local_joints: Value,
    geom_bind_transform: Value,
}
struct SkeletonData {
    skeleton: usize,
    joints: Value,
    root_bones: Vec<usize>,
}

/// The `{ indices, pattern }` of a triangulation, with undefined entries
/// as `Value::Undefined`.
struct Triangulation {
    indices: Vec<Value>,
    pattern: Vec<Value>,
}
#[derive(Default)]
struct HoleMap {
    parent_to_holes: Vec<(Value, Vec<Value>)>,
    hole_faces: Vec<Value>,
}
impl HoleMap {
    fn holes(&self, face: f64) -> Option<&Vec<Value>> {
        let f = Value::Number(face);
        self.parent_to_holes
            .iter()
            .find(|(k, _)| k.same_value_zero(&f))
            .map(|(_, v)| v)
    }
    fn is_hole(&self, face: f64) -> bool {
        let f = Value::Number(face);
        self.hole_faces.iter().any(|k| k.same_value_zero(&f))
    }
}

struct Composer {
    specs: Rc<Specs>,
    assets: Rc<Assets>,
    external_variants: Obj,
    base_path: String,
    skinned: Vec<SkinRecord>,
    skeletons: OMap<SkeletonData>,
    texture_cache: std::cell::RefCell<HashMap<String, usize>>,
    fps: Value,
    children_by_path: OMap<Vec<(String, String)>>,
    attributes_by_prim: OMap<OMap<Rc<Spec>>>,
    materials_by_root: OMap<Vec<String>>,
    shaders_by_material: OMap<Vec<String>>,
    subsets_by_mesh: OMap<Vec<String>>,
}

/// The value callback of _applyTextureOrValue.
type ValueCallback<'a> = Option<&'a dyn Fn(&mut Material, &Value)>;

impl Composer {
    fn new(specs: Rc<Specs>, assets: Rc<Assets>, variants: Obj, base_path: &str) -> Self {
        Composer {
            specs,
            assets,
            external_variants: variants,
            base_path: base_path.into(),
            skinned: vec![],
            skeletons: OMap::default(),
            texture_cache: std::cell::RefCell::new(HashMap::new()),
            fps: Value::Number(24.0),
            children_by_path: OMap::default(),
            attributes_by_prim: OMap::default(),
            materials_by_root: OMap::default(),
            shaders_by_material: OMap::default(),
            subsets_by_mesh: OMap::default(),
        }
    }
    fn spec(&self, path: &str) -> Option<Rc<Spec>> {
        self.specs.get(path)
    }
    fn fields(&self, path: &str) -> Value {
        self.spec(path)
            .map_or(Value::Undefined, |s| Value::Object(s.fields.clone()))
    }

    /// compose( parsedData, assets, variantSelections, basePath ).
    fn compose(&mut self, w: &mut World) -> Result<(usize, Vec<UsdClip>, Option<usize>)> {
        self.build_indexes();
        let root = self.fields("/");
        let root = if root.is_undefined() {
            Value::Object(Obj::new())
        } else {
            root
        };
        let tcps = root.get("timeCodesPerSecond");
        let fps = root.get("framesPerSecond");
        self.fps = if tcps.truthy() {
            tcps
        } else if fps.truthy() {
            fps
        } else {
            Value::Number(24.0)
        };
        let group = w.object(Kind::Group);
        self.build_hierarchy(w, group, "/", 0)?;
        self.bind_skeletons(w)?;
        let skeleton = (self.skeletons.len() == 1)
            .then(|| self.skeletons.iter().next().map(|(_, d)| d.skeleton))
            .flatten();
        let clips = self.build_animations()?;
        let mpu = root.get("metersPerUnit");
        if !mpu.is_undefined() && !mpu.strict_eq(&Value::Number(1.0)) {
            let m = num(&mpu);
            w.objects[group].scale = V3::new(m, m, m);
        }
        if self.spec("/").is_some() && is_str(&root.get("upAxis"), "Z") {
            let mut r = w.objects[group].rotation;
            r.x = -PI / 2.0;
            w.set_rotation(group, r);
        }
        Ok((group, clips, skeleton))
    }

    fn build_indexes(&mut self) {
        for path in self.specs.paths() {
            let Some(spec) = self.spec(&path) else {
                continue;
            };
            if spec.spec_type == PRIM {
                let last = path.rfind('/');
                if let Some(ls) = last.filter(|&l| l > 0) {
                    let parent = &path[..ls];
                    let child = &path[ls + 1..];
                    self.children_by_path
                        .entry(parent, Vec::new)
                        .push((child.into(), path.to_string()));
                } else if last == Some(0) && path.len() > 1 {
                    self.children_by_path
                        .entry("/", Vec::new)
                        .push((path[1..].into(), path.to_string()));
                }
                let type_name = spec.fields.get("typeName");
                if is_str(&type_name, "Material") {
                    let parts: Vec<&str> = path.split('/').collect();
                    let root = if parts.len() > 1 {
                        format!("/{}", parts[1])
                    } else {
                        "/".into()
                    };
                    self.materials_by_root
                        .entry(&root, Vec::new)
                        .push(path.to_string());
                }
                if is_str(&type_name, "Shader")
                    && let Some(ls) = last.filter(|&l| l > 0)
                {
                    let mut ancestor = path[..ls].to_string();
                    while !ancestor.is_empty() {
                        if let Some(a) = self.spec(&ancestor)
                            && a.spec_type == PRIM
                            && is_str(&a.fields.get("typeName"), "Material")
                        {
                            self.shaders_by_material
                                .entry(&ancestor, Vec::new)
                                .push(path.to_string());
                            break;
                        }
                        match ancestor.rfind('/') {
                            Some(sl) if sl > 0 => ancestor.truncate(sl),
                            _ => break,
                        }
                    }
                }
                if is_str(&type_name, "GeomSubset")
                    && let Some(ls) = last.filter(|&l| l > 0)
                {
                    self.subsets_by_mesh
                        .entry(&path[..ls], Vec::new)
                        .push(path.to_string());
                }
            } else if (spec.spec_type == ATTRIBUTE || spec.spec_type == RELATIONSHIP)
                && let Some(dot) = path.rfind('.').filter(|&d| d > 0)
            {
                let prim = &path[..dot];
                let attr = &path[dot + 1..];
                self.attributes_by_prim
                    .entry(prim, OMap::default)
                    .set(attr, spec.clone());
            }
        }
    }

    /// _getVariantPaths( parentPath ).
    fn variant_paths(&self, parent: &str) -> Result<Vec<String>> {
        let spec_fields = self.fields(parent);
        let children = spec_fields.get("variantSetChildren");
        let mut out = vec![];
        if !children.truthy() || children.length() == Some(0) {
            return Ok(out);
        }
        for set in children.items() {
            let set_name = set.to_key();
            let mut selected = self.external_variants.get(&set_name);
            if !selected.truthy() {
                let selection = spec_fields.get("variantSelection");
                selected = if selection.truthy() {
                    selection.get(&set_name)
                } else {
                    Value::Null
                };
            }
            if !selected.truthy() {
                let set_spec = self.fields(&format!("{parent}/{{{set_name}=}}"));
                let vc = set_spec.get("variantChildren");
                if vc.truthy() {
                    selected = vc.at(0);
                }
            }
            if selected.truthy() {
                out.push(format!(
                    "{parent}/{{{set_name}={}}}",
                    selected.to_js_string()
                ));
            }
        }
        Ok(out)
    }

    /// _resolveAttributeValue( path, visited ).
    fn resolve_attribute_value(&self, path: &str, visited: &HashSet<String>) -> Value {
        let clean = strip_angles(path);
        if visited.contains(&clean) {
            return Value::Undefined;
        }
        let fields = self.fields(&clean);
        if fields.is_undefined() {
            return Value::Undefined;
        }
        let mut next = visited.clone();
        next.insert(clean);
        let conn = fields.get("connectionPaths");
        if conn.truthy() {
            for c in conn.items() {
                let v = self.resolve_attribute_value(&c.to_js_string(), &next);
                if !v.is_undefined() {
                    return v;
                }
            }
        }
        let default = fields.get("default");
        if !default.is_undefined() {
            return default;
        }
        let ts = fields.get("timeSamples");
        let (times, values) = if ts.truthy() {
            (ts.get("times"), ts.get("values"))
        } else {
            (Value::Undefined, Value::Undefined)
        };
        if times.truthy() && values.truthy() && times.len_number() > 0.0 {
            let zero = Value::Number(0.0);
            let index = times.items().iter().position(|t| t.strict_eq(&zero));
            return values.at(index.unwrap_or(0));
        }
        Value::Undefined
    }

    /// _getAttributes( path ).
    fn attributes(&self, path: &str) -> Result<Obj> {
        let attrs = Obj::new();
        self.collect_attributes(path, &attrs);
        if let Some((base, relative)) = variant_path_match(path) {
            for vp in self.variant_paths(base)? {
                if path.starts_with(&vp) {
                    continue;
                }
                self.collect_attributes(&format!("{vp}/{relative}"), &attrs);
            }
        } else {
            let parts: Vec<&str> = path.split('/').collect();
            for i in 1..parts.len().saturating_sub(1) {
                let ancestor = parts[..=i].join("/");
                let relative = parts[i + 1..].join("/");
                for vp in self.variant_paths(&ancestor)? {
                    self.collect_attributes(&format!("{vp}/{relative}"), &attrs);
                }
            }
        }
        Ok(attrs)
    }
    fn collect_attributes(&self, path: &str, attrs: &Obj) {
        let Some(map) = self.attributes_by_prim.get(path) else {
            return;
        };
        for (name, spec) in map.iter() {
            let v = self.resolve_attribute_value(&format!("{path}.{name}"), &HashSet::new());
            if !v.is_undefined() {
                attrs.set(name, v);
            }
            let es = spec.fields.get("elementSize");
            if !es.is_undefined() {
                attrs.set(&format!("{name}:elementSize"), es);
            }
            let tn = spec.fields.get("typeName");
            if name.starts_with("primvars:") && !tn.is_undefined() {
                attrs.set(&format!("{name}:typeName"), tn);
            }
        }
    }

    /// applyTransform( obj, fields, attrs ).
    fn apply_transform(&self, w: &mut World, o: usize, fields: &Obj, attrs: &Obj) -> Result<()> {
        let data = Obj::new();
        fields.spread(&data);
        attrs.spread(&data);
        let order = data.get("xformOpOrder");
        if order.truthy() && order.len_number() > 0.0 {
            let mut matrix = M4::IDENTITY;
            let mut scale_values: Option<[Value; 3]> = None;
            for op in order.items() {
                let Value::String(op) = op else {
                    return Err(type_error("op.startsWith is not a function"));
                };
                let inverse = op.starts_with("!invert!");
                let name = if inverse { &op[8..] } else { &op[..] };
                let mut apply = |m: M4| {
                    let m = if inverse { m.invert() } else { m };
                    matrix = matrix.multiply(&m);
                };
                match name {
                    "xformOp:transform" => {
                        let m = data.get("xformOp:transform");
                        if m.truthy() && m.length() == Some(16) {
                            apply(usd_matrix(&m));
                        }
                    }
                    "xformOp:translate" | "xformOp:translate:pivot" => {
                        let t = data.get(name);
                        if t.truthy() {
                            apply(translation(num(&t.at(0)), num(&t.at(1)), num(&t.at(2))));
                        }
                    }
                    "xformOp:scale" => {
                        let v = data.get("xformOp:scale");
                        if v.truthy() {
                            let sv = if v.is_array() {
                                [v.at(0), v.at(1), v.at(2)]
                            } else {
                                [v.clone(), v.clone(), v.clone()]
                            };
                            apply(scaling(num(&sv[0]), num(&sv[1]), num(&sv[2])));
                            scale_values = Some(sv);
                        }
                    }
                    "xformOp:rotateXYZ" => {
                        let r = data.get("xformOp:rotateXYZ");
                        if r.truthy() {
                            let a = |i: usize| num(&r.at(i)) * PI / 180.0;
                            apply(M4::rotation_from_euler(Euler::new(
                                a(0),
                                a(1),
                                a(2),
                                Order::Zyx,
                            )));
                        }
                    }
                    "xformOp:rotateX" | "xformOp:rotateY" | "xformOp:rotateZ" => {
                        let r = data.get(name);
                        if !r.is_undefined() {
                            let t = num(&r) * PI / 180.0;
                            apply(match name {
                                "xformOp:rotateX" => rotation_x(t),
                                "xformOp:rotateY" => rotation_y(t),
                                _ => rotation_z(t),
                            });
                        }
                    }
                    "xformOp:orient" => {
                        let q = data.get("xformOp:orient");
                        if q.truthy() && q.length() == Some(4) {
                            apply(M4::rotation_from_quaternion(Q {
                                x: num(&q.at(0)),
                                y: num(&q.at(1)),
                                z: num(&q.at(2)),
                                w: num(&q.at(3)),
                            }));
                        }
                    }
                    _ => {}
                }
            }
            let (p, q, sc) = matrix.decompose();
            w.objects[o].position = p;
            w.objects[o].scale = sc;
            w.set_quaternion(o, q);
            if let Some(sv) = scale_values {
                let neg = sv.iter().filter(|v| v.lt(&Value::Number(0.0))).count();
                if neg == 3 {
                    w.objects[o].scale = V3::new(num(&sv[0]), num(&sv[1]), num(&sv[2]));
                    let q = w.objects[o].quaternion;
                    w.set_quaternion(
                        o,
                        Q {
                            x: q.x,
                            y: -q.y,
                            z: q.z,
                            w: -q.w,
                        },
                    );
                }
            }
            return Ok(());
        }
        // Vector3.set( x, y, z ): an undefined z keeps the current one.
        let set3 = |old: V3, x: Value, y: Value, z: Value| {
            V3::new(
                num(&x),
                num(&y),
                if z.is_undefined() { old.z } else { num(&z) },
            )
        };
        let t = data.get("xformOp:translate");
        if t.truthy() {
            w.objects[o].position = set3(w.objects[o].position, t.at(0), t.at(1), t.at(2));
        }
        let p = data.get("xformOp:translate:pivot");
        if p.truthy() {
            let d = |v: Value| if v.is_undefined() { 0.0 } else { num(&v) };
            w.objects[o].pivot = Some(V3::new(d(p.at(0)), d(p.at(1)), d(p.at(2))));
        }
        let sc = data.get("xformOp:scale");
        if sc.truthy() {
            w.objects[o].scale = if sc.is_array() {
                set3(w.objects[o].scale, sc.at(0), sc.at(1), sc.at(2))
            } else {
                let v = num(&sc);
                V3::new(v, v, v)
            };
        }
        let r = data.get("xformOp:rotateXYZ");
        if r.truthy() {
            let a = |i: usize| num(&r.at(i)) * PI / 180.0;
            let order = w.objects[o].rotation.order;
            w.set_rotation(o, Euler::new(a(0), a(1), a(2), order));
        }
        let q = data.get("xformOp:orient");
        if q.truthy() && q.length() == Some(4) {
            w.set_quaternion(
                o,
                Q {
                    x: num(&q.at(0)),
                    y: num(&q.at(1)),
                    z: num(&q.at(2)),
                    w: num(&q.at(3)),
                },
            );
        }
        Ok(())
    }

    fn references(&self, spec: &Spec) -> Vec<String> {
        let mut out = vec![];
        let refs = spec.fields.get("references");
        if refs.truthy() && refs.len_number() > 0.0 {
            let r = refs.at(0);
            if let Value::String(r) = &r {
                let mut from = 0;
                while let Some((a, b, _, _)) = reference_match(r, from) {
                    out.push(r[a..b].to_string());
                    from = b;
                }
            } else if r.get("assetPath").truthy() {
                out.push(format!("@{}@", r.get("assetPath").to_js_string()));
            }
        }
        let payload = spec.fields.get("payload");
        if out.is_empty() && payload.truthy() {
            if let Value::String(p) = &payload {
                out.push(p.to_string());
            } else if payload.get("assetPath").truthy() {
                out.push(format!("@{}@", payload.get("assetPath").to_js_string()));
            }
        }
        out
    }
    fn resolve_file_path(&self, r: &str) -> String {
        let clean = r.strip_prefix("./").unwrap_or(r);
        if self.base_path.is_empty() {
            return clean.into();
        }
        let base = if self.base_path.ends_with('/') {
            self.base_path.clone()
        } else {
            format!("{}/", self.base_path)
        };
        resolve_url(clean, &base)
    }
    /// _resolveReference( refValue, localVariants ): a group, or None.
    fn resolve_reference(&self, w: &mut World, value: &str, local: &Obj) -> Result<Option<usize>> {
        if value.is_empty() {
            return Ok(None);
        }
        let Some((_, _, file, prim)) = reference_match(value, 0) else {
            return Ok(None);
        };
        let resolved = self.resolve_file_path(&file);
        let merged = Obj::new();
        local.spread(&merged);
        self.external_variants.spread(&merged);
        let Some(Asset::Layer(specs)) = self.assets.get(&resolved).cloned() else {
            return Ok(None);
        };
        let base = resolved
            .rfind('/')
            .map_or(String::new(), |l| resolved[..l].to_string());
        let mut composer = Composer::new(specs, self.assets.clone(), merged, &base);
        let (group, _, _) = composer.compose(w)?;
        if let Some(prim) = prim {
            let name = last_part(&prim, '/');
            let target = w.objects[group]
                .children
                .iter()
                .copied()
                .find(|&c| w.objects[c].name == name);
            if let Some(t) = target {
                w.remove(t);
                let wrapper = w.object(Kind::Group);
                w.add(wrapper, t);
                return Ok(Some(wrapper));
            }
        }
        Ok(Some(group))
    }
    fn find_single_mesh(&self, w: &mut World, group: usize) -> Option<usize> {
        let children = w.objects[group].children.clone();
        for c in children {
            if w.is_mesh(c) {
                w.remove(c);
                return Some(c);
            }
        }
        if w.objects[group].children.len() == 1 {
            let child = w.objects[group].children[0];
            if w.objects[child].children.len() == 1 {
                let grand = w.objects[child].children[0];
                let o = &w.objects[child];
                let identity = o.position == V3::ZERO
                    && o.rotation.x == 0.0
                    && o.rotation.y == 0.0
                    && o.rotation.z == 0.0
                    && o.scale == V3::ONE;
                if w.is_mesh(grand) && identity {
                    w.remove(grand);
                    return Some(grand);
                }
            }
        }
        None
    }

    /// _buildHierarchy( parent, parentPath ).
    fn build_hierarchy(
        &mut self,
        w: &mut World,
        parent: usize,
        parent_path: &str,
        depth: usize,
    ) -> Result<()> {
        if depth > 2000 {
            return Err(type_error("Maximum call stack size exceeded"));
        }
        let mut entries: Vec<(String, String)> = vec![];
        let mut seen = HashSet::new();
        if let Some(direct) = self.children_by_path.get(parent_path) {
            for c in direct {
                if seen.insert(c.1.clone()) {
                    entries.push(c.clone());
                }
            }
        }
        for vp in self.variant_paths(parent_path)? {
            if let Some(vc) = self.children_by_path.get(&vp) {
                for c in vc {
                    if seen.insert(c.1.clone()) {
                        entries.push(c.clone());
                    }
                }
            }
        }
        for (name, path) in entries {
            let Some(spec) = self.spec(&path) else {
                continue;
            };
            if spec.spec_type != PRIM {
                continue;
            }
            let type_name = spec.fields.get("typeName");
            let refs = self.references(&spec);
            if !refs.is_empty() {
                let local = Obj::new();
                let vs = spec.fields.get("variantSelection");
                if let Value::Object(v) = &vs {
                    v.spread(&local);
                }
                let mut groups = vec![];
                for r in &refs {
                    if let Some(g) = self.resolve_reference(w, r, &local)? {
                        groups.push(g);
                    }
                }
                if !groups.is_empty() {
                    let attrs = self.attributes(&path)?;
                    if groups.len() == 1
                        && let Some(mesh) = self.find_single_mesh(w, groups[0])
                        && (is_str(&type_name, "Xform") || !type_name.truthy())
                    {
                        w.objects[mesh].name = name.clone();
                        self.apply_transform(w, mesh, &spec.fields, &attrs)?;
                        self.apply_material_binding(w, mesh, &path)?;
                        w.add(parent, mesh);
                        self.build_hierarchy(w, mesh, &path, depth + 1)?;
                        continue;
                    }
                    let obj = w.object(Kind::Object3D);
                    w.objects[obj].name = name.clone();
                    self.apply_transform(w, obj, &spec.fields, &attrs)?;
                    for g in groups {
                        while let Some(&c) = w.objects[g].children.first() {
                            w.add(obj, c);
                        }
                    }
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                    continue;
                }
            }
            let t = type_name.as_str().unwrap_or("");
            let typed = type_name.as_str().is_some();
            match t {
                "SkelRoot" if typed => {
                    let obj = w.object(Kind::Object3D);
                    w.objects[obj].name = name.clone();
                    let attrs = self.attributes(&path)?;
                    self.apply_transform(w, obj, &spec.fields, &attrs)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
                "Skeleton" if typed => {
                    if let Some(skel) = self.build_skeleton(w, &path)? {
                        self.skeletons.set(&path, skel);
                    }
                    self.build_hierarchy(w, parent, &path, depth + 1)?;
                }
                "SkelAnimation" if typed => {}
                "Mesh" if typed => {
                    let obj = self.build_mesh(w, &path, &spec)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
                "Camera" if typed => {
                    let obj = self.build_camera(w, &path)?;
                    w.objects[obj].name = name.clone();
                    let attrs = self.attributes(&path)?;
                    self.apply_transform(w, obj, &spec.fields, &attrs)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
                "DistantLight" | "SphereLight" | "RectLight" | "DiskLight" if typed => {
                    let obj = self.build_light(w, &path, t)?;
                    w.objects[obj].name = name.clone();
                    let attrs = self.attributes(&path)?;
                    self.apply_transform(w, obj, &spec.fields, &attrs)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
                "Cube" | "Sphere" | "Cylinder" | "Cone" | "Capsule" if typed => {
                    let obj = self.build_geom_primitive(w, &path, &spec, t)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
                "Material" | "Shader" | "GeomSubset" if typed => {}
                _ => {
                    let obj = w.object(Kind::Object3D);
                    w.objects[obj].name = name.clone();
                    let attrs = self.attributes(&path)?;
                    self.apply_transform(w, obj, &spec.fields, &attrs)?;
                    w.add(parent, obj);
                    self.build_hierarchy(w, obj, &path, depth + 1)?;
                }
            }
        }
        Ok(())
    }

    /// `attrs[ name ] || fallback` as a number.
    fn or_number(attrs: &Obj, name: &str, fallback: f64) -> f64 {
        let v = attrs.get(name);
        if v.truthy() { num(&v) } else { fallback }
    }
    fn build_geom_primitive(
        &mut self,
        w: &mut World,
        path: &str,
        spec: &Spec,
        t: &str,
    ) -> Result<usize> {
        let attrs = self.attributes(path)?;
        let name = last_part(path, '/').to_string();
        let mut g = match t {
            "Cube" => {
                let size = Self::or_number(&attrs, "size", 2.0);
                BoxGeometry::build(size, size, size)?
            }
            "Sphere" => SphereGeometry::build(Self::or_number(&attrs, "radius", 1.0), 32, 16)?,
            "Cylinder" => {
                let h = Self::or_number(&attrs, "height", 2.0);
                let r = Self::or_number(&attrs, "radius", 1.0);
                CylinderGeometry::build(r, r, h, 32, 1, false, 0.0, 2.0 * PI)?
            }
            "Cone" => {
                let h = Self::or_number(&attrs, "height", 2.0);
                let r = Self::or_number(&attrs, "radius", 1.0);
                CylinderGeometry::build(0.0, r, h, 32, 1, false, 0.0, 2.0 * PI)?
            }
            _ => {
                let h = Self::or_number(&attrs, "height", 1.0);
                let r = Self::or_number(&attrs, "radius", 0.5);
                CapsuleGeometry::build(r, h, 16, 32, 1)?
            }
        };
        let axis = attrs.get("axis");
        let axis = if axis.truthy() { axis } else { s("Z") };
        if is_str(&axis, "X") {
            apply_matrix4(&mut g, &rotation_z(-PI / 2.0))?;
        } else if is_str(&axis, "Z") {
            apply_matrix4(&mut g, &rotation_x(PI / 2.0))?;
        }
        let material = self.build_material(w, path, &spec.fields)?;
        w.geometries.push(g);
        let mesh = w.object(Kind::Mesh {
            geometry: w.geometries.len() - 1,
            materials: vec![material],
            multi: false,
            skin: None,
        });
        w.objects[mesh].name = name;
        self.apply_transform(w, mesh, &spec.fields, &attrs)?;
        Ok(mesh)
    }

    fn build_mesh(&mut self, w: &mut World, path: &str, spec: &Spec) -> Result<usize> {
        let attrs = self.attributes(path)?;
        let ji = attrs.get("primvars:skel:jointIndices");
        let jw = attrs.get("primvars:skel:jointWeights");
        let skinning = ji.truthy() && jw.truthy() && ji.len_number() > 0.0 && jw.len_number() > 0.0;
        let subsets = self.geom_subsets(path)?;
        let (geometry, materials, multi) = if !subsets.is_empty() {
            let g = self.build_geometry_with_subsets(&attrs, &subsets, skinning)?;
            let mesh_material = self.material_path(path, &spec.fields)?;
            let mut ms = vec![];
            for subset in &subsets {
                let m = if subset.1.truthy() {
                    subset.1.clone()
                } else {
                    mesh_material.clone()
                };
                ms.push(self.build_material_for_path(w, &m)?);
            }
            (g, ms, true)
        } else {
            let g = self.build_geometry(&attrs, skinning)?;
            let m = self.build_material(w, path, &spec.fields)?;
            (g, vec![m], false)
        };
        let display = attrs.get("primvars:displayColor");
        if display.truthy() && display.len_number() >= 3.0 {
            for &m in &materials {
                let mat = &mut w.materials[m];
                if mat.color == [1.0; 3] && !mat.maps.contains_key("map") {
                    mat.color = srgb(&display.at(0), &display.at(1), &display.at(2));
                }
            }
        }
        let opacity = attrs.get("primvars:displayOpacity");
        if opacity.truthy() && opacity.length() == Some(1) && subsets.is_empty() {
            let o = opacity.at(0);
            for &m in &materials {
                let mat = &mut w.materials[m];
                if o.lt(&Value::Number(1.0))
                    && mat.opacity.strict_eq(&Value::Number(1.0))
                    && !mat.transparent
                {
                    mat.opacity = o.clone();
                    mat.transparent = true;
                }
            }
        }
        w.geometries.push(geometry);
        let gi = w.geometries.len() - 1;
        let mesh = w.object(Kind::Mesh {
            geometry: gi,
            materials,
            multi,
            skin: skinning.then_some((None, M4::IDENTITY)),
        });
        if skinning {
            let mut binding = self.spec(&format!("{path}.skel:skeleton"));
            if binding.is_none() {
                binding = self.spec(&format!("{path}.rel skel:skeleton"));
            }
            let mut skeleton_path = Value::Null;
            if let Some(b) = binding {
                let tp = b.fields.get("targetPaths");
                let d = b.fields.get("default");
                if tp.truthy() && tp.len_number() > 0.0 {
                    skeleton_path = tp.at(0);
                } else if d.truthy() {
                    let Value::String(d) = d else {
                        return Err(type_error("replace is not a function"));
                    };
                    skeleton_path = s(&strip_angles(&d));
                }
            }
            self.skinned.push(SkinRecord {
                mesh,
                skeleton_path,
                local_joints: attrs.get("skel:joints"),
                geom_bind_transform: attrs.get("primvars:skel:geomBindTransform"),
            });
        }
        w.objects[mesh].name = last_part(path, '/').to_string();
        self.apply_transform(w, mesh, &spec.fields, &attrs)?;
        Ok(mesh)
    }

    fn parse_number(v: &Value, fallback: f64) -> f64 {
        let n = num(v);
        if n.is_finite() { n } else { fallback }
    }
    fn build_camera(&self, w: &mut World, path: &str) -> Result<usize> {
        let a = self.attributes(path)?;
        let projection = match a.get("projection") {
            Value::String(p) => p.to_lowercase(),
            _ => "perspective".into(),
        };
        let cr = a.get("clippingRange");
        let cr = if cr.truthy() {
            cr
        } else {
            Value::numbers(vec![1.0, 1000000.0])
        };
        let near = js_max(EPSILON, Self::parse_number(&cr.get("0"), 1.0));
        let far = js_max(near + EPSILON, Self::parse_number(&cr.get("1"), 1000000.0));
        let ha = Self::parse_number(&a.get("horizontalAperture"), 20.955);
        let va = Self::parse_number(&a.get("verticalAperture"), 15.2908);
        let hao = Self::parse_number(&a.get("horizontalApertureOffset"), 0.0);
        let vao = Self::parse_number(&a.get("verticalApertureOffset"), 0.0);
        let fl = Self::parse_number(&a.get("focalLength"), 50.0);
        let fd = Self::parse_number(&a.get("focusDistance"), 0.0);
        let _ = vao;
        let kind = if projection == "orthographic" {
            let (width, height) = (ha / 10.0, va / 10.0);
            let (ox, oy) = (hao / 10.0, vao / 10.0);
            UsdNodeKind::OrthographicCamera {
                left: ox - width * 0.5,
                right: ox + width * 0.5,
                top: oy + height * 0.5,
                bottom: oy - height * 0.5,
                near,
                far,
            }
        } else {
            let sva = js_max(EPSILON, va);
            let sfl = js_max(EPSILON, fl);
            let aspect = ha / sva;
            let film_gauge = js_max(ha, va);
            // setFocalLength(): fov from the film height.
            let slope = 0.5 * (film_gauge / js_max(aspect, 1.0)) / sfl;
            let fov = (180.0 / PI) * 2.0 * atan(slope);
            UsdNodeKind::PerspectiveCamera {
                fov,
                aspect,
                near,
                far,
                film_gauge,
                film_offset: hao,
                focus: fd,
            }
        };
        Ok(w.object(Kind::Camera(kind)))
    }
    fn build_light(&self, w: &mut World, path: &str, t: &str) -> Result<usize> {
        let a = self.attributes(path)?;
        let intensity = Self::parse_number(&a.get("inputs:intensity"), 1.0);
        let base = a.get("inputs:color");
        let base = if base.truthy() {
            base
        } else {
            Value::numbers(vec![1.0; 3])
        };
        let temperature = a
            .get("inputs:enableColorTemperature")
            .strict_eq(&Value::Bool(true));
        let kelvin = Self::parse_number(&a.get("inputs:colorTemperature"), 6500.0);
        let mut color = [num(&base.at(0)), num(&base.at(1)), num(&base.at(2))];
        if temperature {
            let k = color_temperature(kelvin);
            for i in 0..3 {
                color[i] *= k[i];
            }
        }
        let kind = match t {
            "DistantLight" => UsdNodeKind::DirectionalLight { color, intensity },
            "SphereLight" => {
                let cone = Self::parse_number(&a.get("shaping:cone:angle"), 0.0);
                if cone > 0.0 {
                    UsdNodeKind::SpotLight {
                        color,
                        intensity,
                        angle: cone * PI / 180.0,
                        penumbra: Self::parse_number(&a.get("shaping:cone:softness"), 0.0),
                    }
                } else {
                    UsdNodeKind::PointLight { color, intensity }
                }
            }
            "RectLight" => UsdNodeKind::RectAreaLight {
                color,
                intensity,
                width: Self::parse_number(&a.get("inputs:width"), 1.0),
                height: Self::parse_number(&a.get("inputs:height"), 1.0),
            },
            _ => {
                let side = Self::parse_number(&a.get("inputs:radius"), 0.5) * 2.0;
                UsdNodeKind::RectAreaLight {
                    color,
                    intensity,
                    width: side,
                    height: side,
                }
            }
        };
        Ok(w.object(Kind::Light(kind)))
    }

    /// _getGeomSubsets( meshPath ): ( name, indices, materialPath ).
    fn geom_subsets(&self, mesh: &str) -> Result<Vec<(Value, Value)>> {
        let mut out = vec![];
        let Some(paths) = self.subsets_by_mesh.get(mesh) else {
            return Ok(out);
        };
        for p in paths.clone() {
            let attrs = self.attributes(&p)?;
            let indices = attrs.get("indices");
            if !indices.truthy() || indices.length() == Some(0) {
                continue;
            }
            let material = self.material_binding_target(&p)?;
            out.push((indices, material));
        }
        Ok(out)
    }
    fn material_binding_spec(&self, prim: &str) -> Result<Option<Rc<Spec>>> {
        let name = "material:binding";
        let mut binding = self.spec(&format!("{prim}.{name}"));
        let parts: Vec<&str> = prim.split('/').collect();
        for i in 1..parts.len() {
            let ancestor = parts[..=i].join("/");
            let relative = parts[i + 1..].join("/");
            for vp in self.variant_paths(&ancestor)? {
                let path = if relative.is_empty() {
                    format!("{vp}.{name}")
                } else {
                    format!("{vp}/{relative}.{name}")
                };
                if let Some(o) = self.spec(&path)
                    && o.fields.get("targetPaths").len_number() > 0.0
                {
                    binding = Some(o);
                }
            }
        }
        Ok(binding)
    }
    /// _getMaterialBindingTarget( primPath ): the path, or null.
    fn material_binding_target(&self, prim: &str) -> Result<Value> {
        let parts: Vec<&str> = prim.split('/').filter(|p| !p.is_empty()).collect();
        let mut resolved: Option<Rc<Spec>> = None;
        for i in 0..parts.len() {
            let ancestor = format!("/{}", parts[..=i].join("/"));
            let Some(b) = self.material_binding_spec(&ancestor)? else {
                continue;
            };
            let tp = b.fields.get("targetPaths");
            if !tp.length().is_some_and(|l| l > 0) {
                continue;
            }
            let stronger = resolved.as_ref().is_some_and(|r| {
                is_str(&r.fields.get("bindMaterialAs"), "strongerThanDescendants")
            });
            if resolved.is_none() || !stronger {
                resolved = Some(b);
            }
        }
        Ok(match resolved {
            Some(r) => {
                let t = r.fields.get("targetPaths").at(0);
                if t.truthy() { t } else { Value::Null }
            }
            None => Value::Null,
        })
    }

    fn build_geometry(&self, f: &Obj, skinning: bool) -> Result<BufferGeometry> {
        let mut geometry = BufferGeometry::default();
        let points = f.get("points");
        if !points.truthy() || points.length() == Some(0) {
            return Ok(geometry);
        }
        let fvi = f.get("faceVertexIndices");
        let fvc = f.get("faceVertexCounts");
        let holes = build_hole_map(&f.get("primvars:arnold:polygon_holes"));
        let mut indices: Option<Vec<Value>> = fvi.is_list().then(|| fvi.items());
        let indices_value = fvi.clone();
        let mut pattern: Option<Vec<Value>> = None;
        if fvc.truthy() && fvc.len_number() > 0.0 {
            let t = triangulate_with_pattern(&fvi, &fvc, Some(&points), &holes)?;
            indices = Some(t.indices);
            pattern = Some(t.pattern);
        }
        let has_indices = match &pattern {
            Some(_) => indices.as_ref().is_some_and(|i| !i.is_empty()),
            None => indices_value.truthy() && indices_value.len_number() > 0.0,
        };
        let idx = || indices.clone().unwrap_or_default();
        let positions: Vec<Value> = if has_indices {
            expand(&points, &idx(), 3)
        } else {
            points.items()
        };
        let vertices = positions.len() as f64 / 3.0;
        set_f32(&mut geometry, "position", &positions, 3)?;
        let mut normals = f.get("normals");
        if !normals.truthy() {
            normals = f.get("primvars:normals");
        }
        let mut normal_indices = f.get("normals:indices");
        if !normal_indices.truthy() {
            normal_indices = f.get("primvars:normals:indices");
        }
        if normals.truthy() && normals.len_number() > 0.0 {
            let mut data = normals.items();
            if normal_indices.truthy() && normal_indices.len_number() > 0.0 && pattern.is_some() {
                let t = apply_pattern(&normal_indices, pattern.as_deref().unwrap_or(&[]));
                data = expand(&normals, &t, 3);
            } else if normals.length() == points.length() {
                if has_indices {
                    data = expand(&normals, &idx(), 3);
                }
            } else if let Some(p) = &pattern {
                let n = normals.len_number() / 3.0;
                let ident = identity(n)?;
                data = expand(&normals, &apply_pattern(&ident, p), 3);
            }
            set_f32(&mut geometry, "normal", &data, 3)?;
        } else {
            let vn = compute_vertex_normals(&points, &indices_value_or(&indices, &fvi))?;
            let vn = Value::typed(Typed::F32, vn);
            set_f32(
                &mut geometry,
                "normal",
                &expand(&vn, &indices_value_or(&indices, &fvi).items(), 3),
                3,
            )?;
        }
        let (uvs, uv_indices) = find_uv_primvar(f);
        let face_vertices = if fvi.truthy() { fvi.len_number() } else { 0.0 };
        if uvs.truthy() && uvs.len_number() > 0.0 {
            let mut data = uvs.items();
            if uv_indices.truthy() && uv_indices.len_number() > 0.0 && pattern.is_some() {
                data = expand(
                    &uvs,
                    &apply_pattern(&uv_indices, pattern.as_deref().unwrap_or(&[])),
                    2,
                );
            } else if indices_value_or(&indices, &fvi).truthy()
                && uvs.len_number() / 2.0 == points.len_number() / 3.0
            {
                data = expand(&uvs, &idx(), 2);
            } else if let Some(p) = pattern
                .as_ref()
                .filter(|_| uvs.len_number() / 2.0 == face_vertices)
            {
                data = expand(&uvs, &apply_pattern(&identity(face_vertices)?, p), 2);
            }
            set_f32(&mut geometry, "uv", &data, 2)?;
        }
        let uvs2 = f.get("primvars:st1");
        let uv2_indices = f.get("primvars:st1:indices");
        if uvs2.truthy() && uvs2.len_number() > 0.0 {
            let mut data = uvs2.items();
            if uv2_indices.truthy() && uv2_indices.len_number() > 0.0 && pattern.is_some() {
                data = expand(
                    &uvs2,
                    &apply_pattern(&uv2_indices, pattern.as_deref().unwrap_or(&[])),
                    2,
                );
            } else if indices_value_or(&indices, &fvi).truthy()
                && uvs2.len_number() / 2.0 == points.len_number() / 3.0
            {
                data = expand(&uvs2, &idx(), 2);
            } else if let Some(p) = pattern
                .as_ref()
                .filter(|_| uvs2.len_number() / 2.0 == face_vertices)
            {
                data = expand(&uvs2, &apply_pattern(&identity(face_vertices)?, p), 2);
            }
            set_f32(&mut geometry, "uv1", &data, 2)?;
        }
        if skinning {
            let ji = f.get("primvars:skel:jointIndices");
            let jw = f.get("primvars:skel:jointWeights");
            let es = f.get("primvars:skel:jointIndices:elementSize");
            let es = if es.truthy() { num(&es) } else { 4.0 };
            if ji.truthy() && jw.truthy() {
                let (si, sw) = if has_indices {
                    (
                        Value::array(expand_n(&ji, &idx(), es)),
                        Value::array(expand_n(&jw, &idx(), es)),
                    )
                } else {
                    (ji.clone(), jw.clone())
                };
                let n = typed_len(vertices * 4.0)?;
                let mut out_i = vec![0.0; n];
                let mut out_w = vec![0.0; n];
                select_top_weights(&si, &sw, es, vertices, &mut out_i, &mut out_w);
                geometry.set_attribute(
                    "skinIndex",
                    Attribute::U16(BufferAttribute::new(
                        out_i.iter().map(|&v| v as u16).collect(),
                        4,
                        false,
                    )?),
                );
                set_f32(
                    &mut geometry,
                    "skinWeight",
                    &out_w.into_iter().map(Value::Number).collect::<Vec<_>>(),
                    4,
                )?;
            }
        }
        Ok(geometry)
    }

    fn build_geometry_with_subsets(
        &self,
        f: &Obj,
        subsets: &[(Value, Value)],
        skinning: bool,
    ) -> Result<BufferGeometry> {
        let mut geometry = BufferGeometry::default();
        let points = f.get("points");
        if !points.truthy() || points.length() == Some(0) {
            return Ok(geometry);
        }
        let fvi = f.get("faceVertexIndices");
        let fvc = f.get("faceVertexCounts");
        if !fvc.truthy() || fvc.length() == Some(0) {
            return Ok(geometry);
        }
        let holes = build_hole_map(&f.get("primvars:arnold:polygon_holes"));
        let (uvs, uv_indices) = find_uv_primvar(f);
        let uvs2 = f.get("primvars:st1");
        let uv2_indices = f.get("primvars:st1:indices");
        let mut normals = f.get("normals");
        if !normals.truthy() {
            normals = f.get("primvars:normals");
        }
        let mut normal_indices = f.get("normals:indices");
        if !normal_indices.truthy() {
            normal_indices = f.get("primvars:normals:indices");
        }
        let ji = if skinning {
            f.get("primvars:skel:jointIndices")
        } else {
            Value::Null
        };
        let jw = if skinning {
            f.get("primvars:skel:jointWeights")
        } else {
            Value::Null
        };
        let es = f.get("primvars:skel:jointIndices:elementSize");
        let es = if es.truthy() { num(&es) } else { 4.0 };
        let counts = fvc.items();
        let mut offsets = vec![];
        let mut triangles = 0.0;
        for (i, c) in counts.iter().enumerate() {
            offsets.push(triangles);
            if holes.is_hole(i as f64) {
                continue;
            }
            let count = num(c);
            if let Some(h) = holes.holes(i as f64).filter(|h| !h.is_empty()) {
                let mut total = count;
                for hi in h {
                    total += num(&fvc.at_number(num(hi)));
                }
                triangles += total - 2.0;
            } else if count >= 3.0 {
                triangles += count - 2.0;
            }
        }
        let tri_n = typed_len(triangles)?;
        let mut to_subset = vec![-1.0f64; tri_n];
        for (si, subset) in subsets.iter().enumerate() {
            for face in subset.0.items() {
                let face = num(&face);
                if face >= counts.len() as f64 {
                    continue;
                }
                let start = if face >= 0.0 && face.fract() == 0.0 {
                    offsets.get(face as usize).copied().unwrap_or(f64::NAN)
                } else {
                    f64::NAN
                };
                let tri_count = num(&fvc.at_number(face)) - 2.0;
                let mut t = 0.0;
                while t < tri_count {
                    let at = start + t;
                    if at >= 0.0 && at.fract() == 0.0 && (at as usize) < tri_n {
                        to_subset[at as usize] = si as f64;
                    }
                    t += 1.0;
                }
            }
        }
        // A stable sort by subset.
        let mut sorted: Vec<(usize, f64)> = (0..tri_n).map(|t| (t, to_subset[t])).collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut current = sorted.first().map_or(-1.0, |t| t.1);
        let mut group_start = 0;
        for (i, t) in sorted.iter().enumerate() {
            if t.1 != current {
                if current >= 0.0 {
                    geometry.add_group(group_start * 3, (i - group_start) * 3, current as usize);
                }
                current = t.1;
                group_start = i;
            }
        }
        if current >= 0.0 && sorted.len() > group_start {
            geometry.add_group(
                group_start * 3,
                (sorted.len() - group_start) * 3,
                current as usize,
            );
        }
        let tri = triangulate_with_pattern(&fvi, &fvc, Some(&points), &holes)?;
        let face_vertices: f64 = counts.iter().map(num).sum();
        let ident = || identity(face_vertices);
        let face_varying = if (uvs.truthy()
            && !uv_indices.truthy()
            && uvs.len_number() / 2.0 == face_vertices)
            || (uvs2.truthy() && !uv2_indices.truthy() && uvs2.len_number() / 2.0 == face_vertices)
        {
            Some(apply_pattern(&ident()?, &tri.pattern))
        } else {
            None
        };
        let orig_uv = if uv_indices.truthy() {
            Some(apply_pattern(&uv_indices, &tri.pattern))
        } else if uvs.truthy() && uvs.len_number() / 2.0 == face_vertices {
            face_varying.clone()
        } else {
            None
        };
        let orig_uv2 = if uv2_indices.truthy() {
            Some(apply_pattern(&uv2_indices, &tri.pattern))
        } else if uvs2.truthy() && uvs2.len_number() / 2.0 == face_vertices {
            face_varying.clone()
        } else {
            None
        };
        let indexed_normals =
            normals.truthy() && normal_indices.truthy() && normal_indices.len_number() > 0.0;
        let face_varying_normals = normals.truthy() && normals.len_number() / 3.0 == face_vertices;
        let orig_normal = if indexed_normals {
            Some(apply_pattern(&normal_indices, &tri.pattern))
        } else if face_varying_normals {
            Some(apply_pattern(&ident()?, &tri.pattern))
        } else {
            None
        };
        let orig = tri.indices;
        let vertex_normals = if !normals.truthy() && !orig.is_empty() {
            Some(compute_vertex_normals(
                &points,
                &Value::array(orig.clone()),
            )?)
        } else {
            None
        };
        let vc = tri_n * 3;
        let f32_get = |v: &[f64], i: f64| -> f64 {
            if i >= 0.0 && i.fract() == 0.0 && (i as usize) < v.len() {
                v[i as usize]
            } else {
                f64::NAN
            }
        };
        let mut positions = vec![0.0f64; vc * 3];
        let mut uv_data = uvs.truthy().then(|| vec![0.0f64; vc * 2]);
        let mut uv1_data = uvs2.truthy().then(|| vec![0.0f64; vc * 2]);
        let mut normal_data =
            (normals.truthy() || vertex_normals.is_some()).then(|| vec![0.0f64; vc * 3]);
        let es_n = typed_len(vc as f64 * es)?;
        let mut skin_i = ji.truthy().then(|| vec![0.0f64; es_n]);
        let mut skin_w = jw.truthy().then(|| vec![0.0f64; es_n]);
        let store = |v: &mut [f64], i: usize, x: f64, t: Typed| {
            if i < v.len() {
                v[i] = t.store(x);
            }
        };
        let get = |v: &Value, i: f64| num(&v.at_number(i));
        for (i, &(orig_tri, _)) in sorted.iter().enumerate() {
            for k in 0..3 {
                let oi = orig_tri * 3 + k;
                let ni = i * 3 + k;
                let p = num(orig.get(oi).unwrap_or(&Value::Undefined));
                for c in 0..3 {
                    store(
                        &mut positions,
                        ni * 3 + c,
                        get(&points, p * 3.0 + c as f64),
                        Typed::F32,
                    );
                }
                if let Some(d) = &mut uv_data {
                    if let Some(o) = &orig_uv {
                        let u = num(o.get(oi).unwrap_or(&Value::Undefined));
                        store(d, ni * 2, get(&uvs, u * 2.0), Typed::F32);
                        store(d, ni * 2 + 1, get(&uvs, u * 2.0 + 1.0), Typed::F32);
                    } else if uvs.len_number() / 2.0 == points.len_number() / 3.0 {
                        store(d, ni * 2, get(&uvs, p * 2.0), Typed::F32);
                        store(d, ni * 2 + 1, get(&uvs, p * 2.0 + 1.0), Typed::F32);
                    }
                }
                if let Some(d) = &mut uv1_data {
                    if let Some(o) = &orig_uv2 {
                        let u = num(o.get(oi).unwrap_or(&Value::Undefined));
                        store(d, ni * 2, get(&uvs2, u * 2.0), Typed::F32);
                        store(d, ni * 2 + 1, get(&uvs2, u * 2.0 + 1.0), Typed::F32);
                    } else if uvs2.len_number() / 2.0 == points.len_number() / 3.0 {
                        store(d, ni * 2, get(&uvs2, p * 2.0), Typed::F32);
                        store(d, ni * 2 + 1, get(&uvs2, p * 2.0 + 1.0), Typed::F32);
                    }
                }
                if let Some(d) = &mut normal_data {
                    if normals.truthy()
                        && let Some(o) = &orig_normal
                    {
                        let n = num(o.get(oi).unwrap_or(&Value::Undefined));
                        for c in 0..3 {
                            store(d, ni * 3 + c, get(&normals, n * 3.0 + c as f64), Typed::F32);
                        }
                    } else if normals.truthy() && normals.length() == points.length() {
                        for c in 0..3 {
                            store(d, ni * 3 + c, get(&normals, p * 3.0 + c as f64), Typed::F32);
                        }
                    } else if let Some(vn) = &vertex_normals {
                        for c in 0..3 {
                            store(d, ni * 3 + c, f32_get(vn, p * 3.0 + c as f64), Typed::F32);
                        }
                    }
                }
                if let (Some(si), Some(sw)) = (&mut skin_i, &mut skin_w)
                    && ji.truthy()
                    && jw.truthy()
                {
                    let mut j = 0.0;
                    while j < es {
                        let at = ni as f64 * es + j;
                        let src = p * es + j;
                        let iv = ji.at_number(src);
                        let wv = jw.at_number(src);
                        let iv = if iv.truthy() { num(&iv) } else { 0.0 };
                        let wv = if wv.truthy() { num(&wv) } else { 0.0 };
                        if at >= 0.0 && at.fract() == 0.0 {
                            store(si, at as usize, iv, Typed::U16);
                            store(sw, at as usize, wv, Typed::F32);
                        }
                        j += 1.0;
                    }
                }
            }
        }
        let to_values = |v: Vec<f64>| v.into_iter().map(Value::Number).collect::<Vec<_>>();
        set_f32(&mut geometry, "position", &to_values(positions), 3)?;
        if let Some(d) = uv_data {
            set_f32(&mut geometry, "uv", &to_values(d), 2)?;
        }
        if let Some(d) = uv1_data {
            set_f32(&mut geometry, "uv1", &to_values(d), 2)?;
        }
        let Some(nd) = normal_data else {
            return Err(type_error(
                "Cannot read properties of null ( reading 'length' )",
            ));
        };
        set_f32(&mut geometry, "normal", &to_values(nd), 3)?;
        if let (Some(si), Some(sw)) = (skin_i, skin_w) {
            let n = typed_len(vc as f64 * 4.0)?;
            let mut out_i = vec![0.0; n];
            let mut out_w = vec![0.0; n];
            select_top_weights(
                &Value::typed(Typed::U16, si),
                &Value::typed(Typed::F32, sw),
                es,
                vc as f64,
                &mut out_i,
                &mut out_w,
            );
            geometry.set_attribute(
                "skinIndex",
                Attribute::U16(BufferAttribute::new(
                    out_i.iter().map(|&v| v as u16).collect(),
                    4,
                    false,
                )?),
            );
            set_f32(&mut geometry, "skinWeight", &to_values(out_w), 4)?;
        }
        Ok(geometry)
    }

    /// _getMaterialPath( meshPath, fields ).
    fn material_path(&self, mesh: &str, fields: &Obj) -> Result<Value> {
        let mut path = Value::Null;
        let binding = fields.get("material:binding");
        if binding.truthy() {
            path = if binding.is_array() {
                binding.at(0)
            } else {
                binding
            };
        }
        if !path.truthy() {
            path = self.material_binding_target(mesh)?;
        }
        Ok(path)
    }
    fn build_material(&self, w: &mut World, mesh: &str, fields: &Obj) -> Result<usize> {
        w.materials.push(Material::new());
        let m = w.materials.len() - 1;
        let mut path = self.material_path(mesh, fields)?;
        if !path.truthy() {
            let mut paths = vec![];
            let prefix = format!("{mesh}/");
            for p in self.specs.paths() {
                if !p.starts_with(&prefix) || !p.ends_with(".material:binding") {
                    continue;
                }
                let Some(b) = self.spec(&p) else {
                    continue;
                };
                let tp = b.fields.get("targetPaths");
                if tp.truthy() && tp.len_number() > 0.0 {
                    paths.push(tp.at(0));
                }
            }
            if !paths.is_empty() {
                path = self.pick_best_material(&paths)?;
            }
        }
        if !path.truthy() {
            let parts: Vec<&str> = mesh.split('/').collect();
            let root = format!("/{}", parts.get(1).copied().unwrap_or("undefined"));
            if let Some(ms) = self.materials_by_root.get(&root) {
                for p in ms {
                    if p.starts_with(&format!("{root}/Looks/"))
                        || p.starts_with(&format!("{root}/Materials/"))
                    {
                        path = s(p);
                        break;
                    }
                }
            }
        }
        if path.truthy() {
            self.apply_material(w, m, &path.to_js_string())?;
        }
        Ok(m)
    }
    fn build_material_for_path(&self, w: &mut World, path: &Value) -> Result<usize> {
        w.materials.push(Material::new());
        let m = w.materials.len() - 1;
        if path.truthy() {
            self.apply_material(w, m, &path.to_js_string())?;
        }
        Ok(m)
    }
    fn apply_material_binding(&self, w: &mut World, mesh: usize, prim: &str) -> Result<()> {
        let path = self.material_binding_target(prim)?;
        if !path.truthy() {
            return Ok(());
        }
        let p = path.to_js_string();
        let p = p.strip_prefix('<').unwrap_or(&p);
        let p = p.strip_suffix('>').unwrap_or(p);
        w.materials.push(Material::new());
        let m = w.materials.len() - 1;
        self.apply_material(w, m, p)?;
        if let Kind::Mesh {
            materials, multi, ..
        } = &mut w.objects[mesh].kind
        {
            *materials = vec![m];
            *multi = false;
        }
        Ok(())
    }
    fn pick_best_material(&self, paths: &[Value]) -> Result<Value> {
        for mp in paths {
            let Some(shaders) = self.shaders_by_material.get(&mp.to_key()) else {
                continue;
            };
            for p in shaders.clone() {
                let a = self.attributes(&p)?;
                if is_str(&a.get("info:id"), "UsdUVTexture") && a.get("inputs:file").truthy() {
                    return Ok(mp.clone());
                }
            }
        }
        Ok(paths[0].clone())
    }
    fn apply_material(&self, w: &mut World, m: usize, path: &str) -> Result<()> {
        if self.spec(path).is_none() {
            return Ok(());
        }
        let Some(shaders) = self.shaders_by_material.get(path).cloned() else {
            return Ok(());
        };
        for p in shaders {
            let Some(spec) = self.spec(&p) else {
                continue;
            };
            let attrs = self.attributes(&p)?;
            let id = attrs.get("info:id");
            let id = if id.truthy() {
                id
            } else {
                spec.fields.get("info:id")
            };
            if is_str(&id, "UsdPreviewSurface") || is_str(&id, "ND_UsdPreviewSurface_surfaceshader")
            {
                self.apply_preview_surface(w, m, &p)?;
            } else if is_str(&id, "arnold:openpbr_surface") {
                self.apply_openpbr_surface(w, m, &p)?;
            }
        }
        Ok(())
    }

    /// _applyTextureOrValue(): the texture through the connection, else the
    /// value through the callback; whether a texture was applied.
    #[allow(clippy::too_many_arguments)]
    fn texture_or_value(
        &self,
        w: &mut World,
        m: usize,
        shader: &str,
        fields: &Obj,
        attr: &str,
        property: &'static str,
        srgb: bool,
        value: ValueCallback,
        openpbr: bool,
    ) -> Result<bool> {
        if let Some(spec) = self.spec(&format!("{shader}.{attr}")) {
            let conn = spec.fields.get("connectionPaths");
            if conn.truthy() && conn.len_number() > 0.0 {
                let paths = if openpbr {
                    conn.items()
                } else {
                    vec![conn.at(0)]
                };
                for c in paths {
                    let t = if openpbr {
                        self.texture_from_openpbr(w, &c.to_js_string(), 0)?
                    } else {
                        self.texture_from_connection(w, &c.to_js_string())?
                    };
                    if let Some(t) = t {
                        w.textures[t].srgb = srgb;
                        w.materials[m].maps.insert(property, t);
                        return Ok(true);
                    }
                }
            }
        }
        let v = fields.get(attr);
        if !v.is_undefined()
            && let Some(cb) = value
        {
            cb(&mut w.materials[m], &v);
        }
        Ok(false)
    }
    fn texture_scale(w: &World, m: usize, property: &str) -> Option<Value> {
        let t = *w.materials[m].maps.get(property)?;
        w.textures[t].scale.clone().filter(Value::truthy)
    }
    fn apply_preview_surface(&self, w: &mut World, m: usize, shader: &str) -> Result<()> {
        let f = self.attributes(shader)?;
        let color3 = |v: &Value| v.is_array() && v.len_number() >= 3.0;
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:diffuseColor",
            "map",
            true,
            Some(&|mat, c| {
                if color3(c) {
                    mat.color = srgb(&c.at(0), &c.at(1), &c.at(2));
                }
            }),
            false,
        )?;
        if let Some(sc) = Self::texture_scale(w, m, "map")
            && color3(&sc)
        {
            w.materials[m].color = srgb(&sc.at(0), &sc.at(1), &sc.at(2));
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:emissiveColor",
            "emissiveMap",
            true,
            Some(&|mat, c| {
                if color3(c) {
                    mat.emissive = srgb(&c.at(0), &c.at(1), &c.at(2));
                }
            }),
            false,
        )?;
        if w.materials[m].maps.contains_key("emissiveMap") {
            match Self::texture_scale(w, m, "emissiveMap") {
                Some(sc) => {
                    if color3(&sc) {
                        w.materials[m].emissive = srgb(&sc.at(0), &sc.at(1), &sc.at(2));
                    }
                }
                None => w.materials[m].emissive = [1.0; 3],
            }
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:normal",
            "normalMap",
            false,
            None,
            false,
        )?;
        if let Some(sc) = Self::texture_scale(w, m, "normalMap") {
            let d = |v: Value| if v.is_undefined() { 0.0 } else { num(&v) };
            w.materials[m].normal_scale = [d(sc.at(0)), d(sc.at(1))];
        }
        if self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:roughness",
            "roughnessMap",
            false,
            Some(&|mat, v| {
                mat.roughness = v.clone();
            }),
            false,
        )? {
            w.materials[m].roughness = Value::Number(1.0);
        }
        if self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:metallic",
            "metalnessMap",
            false,
            Some(&|mat, v| {
                mat.metalness = v.clone();
            }),
            false,
        )? {
            w.materials[m].metalness = Value::Number(1.0);
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:occlusion",
            "aoMap",
            false,
            None,
            false,
        )?;
        let ior = f.get("inputs:ior");
        if !ior.is_undefined() {
            w.materials[m].ior = ior;
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:specularColor",
            "specularColorMap",
            true,
            Some(&|mat, c| {
                if color3(c) {
                    mat.specular_color = srgb(&c.at(0), &c.at(1), &c.at(2));
                }
            }),
            false,
        )?;
        if let Some(sc) = Self::texture_scale(w, m, "specularColorMap")
            && color3(&sc)
        {
            w.materials[m].specular_color = srgb(&sc.at(0), &sc.at(1), &sc.at(2));
        }
        let cc = f.get("inputs:clearcoat");
        if !cc.is_undefined() {
            w.materials[m].clearcoat = cc;
        }
        let ccr = f.get("inputs:clearcoatRoughness");
        if !ccr.is_undefined() {
            w.materials[m].clearcoat_roughness = ccr;
        }
        let threshold = f.get("inputs:opacityThreshold");
        let threshold = if threshold.is_undefined() {
            Value::Number(0.0)
        } else {
            threshold
        };
        let opacity_spec = self.spec(&format!("{shader}.inputs:opacity"));
        let connected =
            opacity_spec.is_some_and(|s| s.fields.get("connectionPaths").len_number() > 0.0);
        if connected {
            if threshold.gt(&Value::Number(0.0)) {
                w.materials[m].alpha_test = threshold;
                w.materials[m].transparent = false;
            } else {
                w.materials[m].transparent = true;
            }
        } else {
            let o = f.get("inputs:opacity");
            let o = if o.is_undefined() {
                Value::Number(1.0)
            } else {
                o
            };
            if o.lt(&Value::Number(1.0)) {
                w.materials[m].transparent = true;
                w.materials[m].opacity = o;
            }
        }
        Ok(())
    }
    fn apply_openpbr_surface(&self, w: &mut World, m: usize, shader: &str) -> Result<()> {
        let f = self.attributes(shader)?;
        let color3 = |v: &Value| v.is_array() && v.len_number() >= 3.0;
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:base_color",
            "map",
            true,
            Some(&|mat, c| {
                if color3(c) {
                    mat.color = srgb(&c.at(0), &c.at(1), &c.at(2));
                }
            }),
            true,
        )?;
        if let Some(sc) = Self::texture_scale(w, m, "map")
            && color3(&sc)
        {
            w.materials[m].color = srgb(&sc.at(0), &sc.at(1), &sc.at(2));
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:base_metalness",
            "metalnessMap",
            false,
            Some(&|mat, v| {
                if v.is_number() {
                    mat.metalness = v.clone();
                }
            }),
            true,
        )?;
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:specular_roughness",
            "roughnessMap",
            false,
            Some(&|mat, v| {
                if v.is_number() {
                    mat.roughness = v.clone();
                }
            }),
            true,
        )?;
        let has_emission = self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:emission_color",
            "emissiveMap",
            true,
            Some(&|mat, c| {
                if color3(c) {
                    mat.emissive = srgb(&c.at(0), &c.at(1), &c.at(2));
                }
            }),
            true,
        )?;
        let zero = Value::Number(0.0);
        let one = Value::Number(1.0);
        let lum = f.get("inputs:emission_luminance");
        if !lum.is_undefined() && lum.gt(&zero) {
            if has_emission {
                w.materials[m].emissive_intensity = lum;
            } else {
                let l = num(&lum);
                for c in &mut w.materials[m].emissive {
                    *c *= l;
                }
            }
        }
        let tw = f.get("inputs:transmission_weight");
        if !tw.is_undefined() && tw.gt(&zero) {
            w.materials[m].transmission = tw;
            let depth = f.get("inputs:transmission_depth");
            if !depth.is_undefined() {
                w.materials[m].thickness = depth.clone();
            }
            let tc = f.get("inputs:transmission_color");
            if !tc.is_undefined() && tc.is_array() {
                w.materials[m].attenuation_color = [num(&tc.at(0)), num(&tc.at(1)), num(&tc.at(2))];
                w.materials[m].attenuation_distance = if depth.truthy() {
                    depth
                } else {
                    Value::Number(1.0)
                };
            }
        }
        let go = f.get("inputs:geometry_opacity");
        if !go.is_undefined() && go.lt(&one) {
            w.materials[m].opacity = go;
            w.materials[m].transparent = true;
        }
        let ior = f.get("inputs:specular_ior");
        if !ior.is_undefined() {
            w.materials[m].ior = ior;
        }
        let cw = f.get("inputs:coat_weight");
        if !cw.is_undefined() && cw.gt(&zero) {
            w.materials[m].clearcoat = cw;
            let cr = f.get("inputs:coat_roughness");
            if !cr.is_undefined() {
                w.materials[m].clearcoat_roughness = cr;
            }
        }
        let tfw = f.get("inputs:thin_film_weight");
        if !tfw.is_undefined() && tfw.gt(&zero) {
            w.materials[m].iridescence = tfw;
            let ti = f.get("inputs:thin_film_ior");
            if !ti.is_undefined() {
                w.materials[m].iridescence_ior = ti;
            }
            let tt = f.get("inputs:thin_film_thickness");
            if !tt.is_undefined() {
                let nm = Value::Number(num(&tt) * 1000.0);
                w.materials[m].iridescence_thickness_range = [nm.clone(), nm];
            }
        }
        let sw = f.get("inputs:specular_weight");
        if !sw.is_undefined() {
            w.materials[m].specular_intensity = sw;
        }
        let sc = f.get("inputs:specular_color");
        if !sc.is_undefined() && sc.is_array() {
            w.materials[m].specular_color = [num(&sc.at(0)), num(&sc.at(1)), num(&sc.at(2))];
        }
        let an = f.get("inputs:specular_roughness_anisotropy");
        if !an.is_undefined() && an.gt(&zero) {
            w.materials[m].anisotropy = an;
        }
        self.texture_or_value(
            w,
            m,
            shader,
            &f,
            "inputs:geometry_normal",
            "normalMap",
            false,
            None,
            true,
        )?;
        Ok(())
    }

    /// The prim of a connection ( `path.split( '.' )[ 0 ]` ) has a spec.
    fn info_id(&self, path: &str, attrs: &Obj) -> Value {
        let id = attrs.get("info:id");
        if id.truthy() {
            id
        } else {
            self.spec(path)
                .map_or(Value::Undefined, |s| s.fields.get("info:id"))
        }
    }
    /// The first connection path of `path`'s spec, when it has one.
    fn first_connection(&self, path: &str) -> Option<String> {
        let spec = self.spec(path)?;
        let c = spec.fields.get("connectionPaths");
        (c.len_number() > 0.0).then(|| c.at(0).to_js_string())
    }
    fn texture_from_openpbr(
        &self,
        w: &mut World,
        conn: &str,
        depth: usize,
    ) -> Result<Option<usize>> {
        if depth > 1000 {
            return Err(type_error("Maximum call stack size exceeded"));
        }
        let clean = strip_angles(conn);
        let shader = clean.split('.').next().unwrap_or("").to_string();
        let Some(spec) = self.spec(&shader) else {
            return Ok(None);
        };
        let attrs = self.attributes(&shader)?;
        let id = self.info_id(&shader, &attrs);
        let id_str = id.as_str().unwrap_or("");
        let typed = id.as_str().is_some();
        if is_str(&spec.fields.get("typeName"), "NodeGraph") {
            let output = clean
                .split('.')
                .nth(1)
                .map_or("undefined".to_string(), str::to_string);
            if let Some(c) = self.first_connection(&format!("{shader}.{output}")) {
                return self.texture_from_openpbr(w, &c, depth + 1);
            }
            return Ok(None);
        }
        if id_str == "arnold:image" && typed {
            let file = attrs.get("inputs:filename");
            if !file.truthy() {
                return Ok(None);
            }
            return self.load_texture_from_path(w, &file.to_js_string());
        }
        if typed && id_str.starts_with("ND_image_") {
            let file = attrs.get("inputs:file");
            if !file.truthy() {
                return Ok(None);
            }
            return self.load_texture_from_path(w, &file.to_js_string());
        }
        let follow =
            |input: &str| -> Option<String> { self.first_connection(&format!("{shader}.{input}")) };
        if id_str == "MayaND_fileTexture_color4" && typed {
            return match follow("inputs:inColor") {
                Some(c) => self.texture_from_openpbr(w, &c, depth + 1),
                None => Ok(None),
            };
        }
        if typed && id_str.starts_with("ND_convert_") {
            return match follow("inputs:in") {
                Some(c) => self.texture_from_openpbr(w, &c, depth + 1),
                None => Ok(None),
            };
        }
        if id_str == "arnold:bump2d" && typed {
            return match follow("inputs:bump_map") {
                Some(c) => self.texture_from_openpbr(w, &c, depth + 1),
                None => Ok(None),
            };
        }
        if id_str == "arnold:color_correct" && typed {
            return match follow("inputs:input") {
                Some(c) => self.texture_from_openpbr(w, &c, depth + 1),
                None => Ok(None),
            };
        }
        // substring( 0, lastIndexOf( '/' ) ): the whole path when it has no
        // slash ( -1 clamps to 0 ).
        let parent = shader
            .rfind('/')
            .map_or(String::new(), |i| shader[..i].to_string());
        if !parent.is_empty() && self.spec(&parent).is_some() {
            let pa = self.attributes(&parent)?;
            if is_str(&self.info_id(&parent, &pa), "arnold:image") {
                let file = pa.get("inputs:filename");
                if file.truthy() {
                    return self.load_texture_from_path(w, &file.to_js_string());
                }
            }
        }
        Ok(None)
    }
    fn load_texture_from_path(&self, w: &mut World, file: &str) -> Result<Option<usize>> {
        if file.is_empty() {
            return Ok(None);
        }
        // The cache is per composer; a borrow of self cannot mutate it, so
        // the cached texture is looked up and stored through a cell.
        if let Some(&t) = self.cache().get(file) {
            return Ok(Some(t));
        }
        let t = self.load_texture(w, file, None, None)?;
        if let Some(t) = t {
            self.cache_set(file, t);
        }
        Ok(t)
    }
    fn texture_from_connection(&self, w: &mut World, conn: &str) -> Result<Option<usize>> {
        let shader = conn.split('.').next().unwrap_or("").to_string();
        if self.spec(&shader).is_none() {
            return Ok(None);
        }
        let attrs = self.attributes(&shader)?;
        if !is_str(&self.info_id(&shader, &attrs), "UsdUVTexture") {
            return Ok(None);
        }
        let file = attrs.get("inputs:file");
        if !file.truthy() {
            return Ok(None);
        }
        let file = file.to_js_string();
        let mut transform: Option<Obj> = None;
        let mut channel = 0;
        if let Some(st_conn) = self.first_connection(&format!("{shader}.inputs:st")) {
            let st_path = strip_angles(&st_conn)
                .split('.')
                .next()
                .unwrap_or("")
                .to_string();
            if self.spec(&st_path).is_some() {
                let st = self.attributes(&st_path)?;
                let st_id = self.info_id(&st_path, &st);
                if is_str(&st_id, "UsdTransform2d") {
                    transform = Some(st.clone());
                    if let Some(in_conn) = self.first_connection(&format!("{st_path}.inputs:in")) {
                        let pv_path = strip_angles(&in_conn)
                            .split('.')
                            .next()
                            .unwrap_or("")
                            .to_string();
                        let pv = self.attributes(&pv_path)?;
                        let var = pv.get("inputs:varname");
                        if is_str(&var, "st1") {
                            channel = 1;
                        } else if is_str(&var, "st2") {
                            channel = 2;
                        }
                    }
                } else if is_str(&st_id, "UsdPrimvarReader_float2") {
                    let var = st.get("inputs:varname");
                    if is_str(&var, "st1") {
                        channel = 1;
                    } else if is_str(&var, "st2") {
                        channel = 2;
                    }
                }
            }
        }
        let scale = attrs.get("inputs:scale");
        let bias = attrs.get("inputs:bias");
        let join = |v: &Value| -> Result<String> {
            if v.is_list() {
                Ok(v.join(","))
            } else {
                Err(type_error("join is not a function"))
            }
        };
        let mut key = file.clone();
        if scale.truthy() {
            key += &format!(":s{}", join(&scale)?);
        }
        if bias.truthy() {
            key += &format!(":b{}", join(&bias)?);
        }
        if let Some(&t) = self.cache().get(&key) {
            return Ok(Some(t));
        }
        let t = self.load_texture(w, &file, Some(&attrs), transform.as_ref())?;
        if let Some(t) = t {
            if scale.truthy() {
                w.textures[t].scale = Some(scale);
            }
            if bias.truthy() {
                w.textures[t].bias = Some(bias);
            }
            if channel != 0 {
                w.textures[t].channel = channel;
            }
            self.cache_set(&key, t);
        }
        Ok(t)
    }
    fn load_texture(
        &self,
        w: &mut World,
        file: &str,
        attrs: Option<&Obj>,
        transform: Option<&Obj>,
    ) -> Result<Option<usize>> {
        let clean = file.strip_prefix('@').unwrap_or(file);
        let clean = clean.strip_suffix('@').unwrap_or(clean);
        let resolved = self.resolve_file_path(clean);
        let mut data = self.assets.get(&resolved).cloned();
        if data.is_none() {
            data = self.assets.get(clean).cloned();
        }
        if data.is_none() {
            let base = last_part(clean, '/');
            for key in &self.assets.order {
                if key.ends_with(base) || key.ends_with(&format!("/{base}")) {
                    let d = self.assets.get(key).cloned();
                    return Ok(self.texture_from_data(w, key, d, None, attrs, transform));
                }
            }
            if !self.base_path.is_empty() {
                return Ok(self.texture_from_data(
                    w,
                    &resolved,
                    None,
                    Some(resolved.clone()),
                    attrs,
                    transform,
                ));
            }
            return Ok(None);
        }
        let name = if self.assets.get(&resolved).is_some() {
            resolved.clone()
        } else {
            clean.to_string()
        };
        Ok(self.texture_from_data(w, &name, data, None, attrs, transform))
    }
    /// _createTextureFromData( data, textureAttrs, transformAttrs ), with
    /// the image's onload applied ( the loader resolves after it ).
    fn texture_from_data(
        &self,
        w: &mut World,
        name: &str,
        data: Option<Asset>,
        url: Option<String>,
        attrs: Option<&Obj>,
        transform: Option<&Obj>,
    ) -> Option<usize> {
        let image = match (data, url) {
            (_, Some(u)) if !u.is_empty() => UsdImage::Url(u),
            (Some(Asset::Bytes(b)), _) => UsdImage::Asset {
                name: name.into(),
                data: b,
            },
            _ => return None,
        };
        let mut t = Texture {
            image,
            srgb: false,
            wrap: [CLAMP, CLAMP],
            repeat: [Value::Number(1.0), Value::Number(1.0)],
            offset: [Value::Number(0.0), Value::Number(0.0)],
            rotation: Value::Number(0.0),
            channel: 0,
            scale: None,
            bias: None,
        };
        if let Some(a) = attrs {
            t.wrap = [
                wrap_mode(&a.get("inputs:wrapS")),
                wrap_mode(&a.get("inputs:wrapT")),
            ];
        }
        if let Some(tr) = transform {
            let sc = tr.get("inputs:scale");
            if sc.truthy() && sc.is_array() && sc.len_number() >= 2.0 {
                t.repeat = [sc.at(0), sc.at(1)];
            }
            let tl = tr.get("inputs:translation");
            if tl.truthy() && tl.is_array() && tl.len_number() >= 2.0 {
                t.offset = [tl.at(0), tl.at(1)];
            }
            let r = tr.get("inputs:rotation");
            if r.is_number() {
                t.rotation = Value::Number(num(&r) * PI / 180.0);
            }
        }
        w.textures.push(t);
        Some(w.textures.len() - 1)
    }

    fn build_skeleton(&self, w: &mut World, path: &str) -> Result<Option<SkeletonData>> {
        let attrs = self.attributes(path)?;
        let joints = attrs.get("joints");
        if !joints.truthy() || joints.length() == Some(0) {
            return Ok(None);
        }
        let n = joints.len_number();
        let bind = flatten_matrix_array(&attrs.get("bindTransforms"), n);
        let rest = flatten_matrix_array(&attrs.get("restTransforms"), n);
        let mut bones = vec![];
        let mut by_path: HashMap<String, usize> = HashMap::new();
        let mut inverses = vec![];
        let joint_list = joints.items();
        for (i, j) in joint_list.iter().enumerate() {
            let Value::String(jp) = j else {
                return Err(type_error("jointPath.split is not a function"));
            };
            let bone = w.object(Kind::Bone);
            w.objects[bone].name = last_part(jp, '/').to_string();
            bones.push(bone);
            by_path.insert(jp.to_string(), bone);
            match &bind {
                Some(b) if b.len_number() >= ((i + 1) * 16) as f64 => {
                    let m = Value::array((0..16).map(|k| b.at(i * 16 + k)).collect());
                    inverses.push(usd_matrix(&m).invert());
                }
                _ => inverses.push(M4::IDENTITY),
            }
        }
        for (i, j) in joint_list.iter().enumerate() {
            let jp = j.to_js_string();
            let parts: Vec<&str> = jp.split('/').collect();
            if parts.len() > 1 {
                let parent = parts[..parts.len() - 1].join("/");
                if let Some(&p) = by_path.get(&parent) {
                    w.add(p, bones[i]);
                }
            }
        }
        if let Some(r) = &rest
            && r.len_number() >= n * 16.0
        {
            for (i, &b) in bones.iter().enumerate() {
                let m = Value::array((0..16).map(|k| r.at(i * 16 + k)).collect());
                let (p, q, sc) = usd_matrix(&m).decompose();
                w.objects[b].position = p;
                w.objects[b].scale = sc;
                w.set_quaternion(b, q);
            }
        }
        let roots: Vec<usize> = bones
            .iter()
            .copied()
            .filter(|&b| {
                w.objects[b]
                    .parent
                    .is_none_or(|p| !matches!(w.objects[p].kind, Kind::Bone))
            })
            .collect();
        w.skeletons.push((bones, inverses));
        Ok(Some(SkeletonData {
            skeleton: w.skeletons.len() - 1,
            joints,
            root_bones: roots,
        }))
    }
    fn bind_skeletons(&self, w: &mut World) -> Result<()> {
        for rec in &self.skinned {
            let mut data: Option<&SkeletonData> = None;
            let sp = &rec.skeleton_path;
            if sp.truthy() {
                data = self.skeletons.get(&sp.to_key());
            }
            if data.is_none() {
                for (path, d) in self.skeletons.iter() {
                    if let Value::String(spath) = sp
                        && (spath.contains(path.as_str()) || path.contains(&**spath))
                    {
                        data = Some(d);
                        break;
                    }
                }
            }
            if data.is_none() {
                data = self.skeletons.iter().next().map(|(_, d)| d);
            }
            let Some(d) = data else {
                continue;
            };
            let mesh = rec.mesh;
            if rec.local_joints.truthy() && rec.local_joints.len_number() > 0.0 {
                let geometry = match &w.objects[mesh].kind {
                    Kind::Mesh { geometry, .. } => *geometry,
                    _ => unreachable!(),
                };
                if let Some(Attribute::U16(a)) =
                    w.geometries[geometry].attributes.get_mut("skinIndex")
                {
                    let joints = d.joints.items();
                    let local: Vec<f64> = rec
                        .local_joints
                        .items()
                        .iter()
                        .map(|j| {
                            joints
                                .iter()
                                .position(|g| g.strict_eq(j))
                                .map_or(0.0, |p| p as f64)
                        })
                        .collect();
                    let arr = a.array_mut();
                    for v in arr.iter_mut() {
                        let l = f64::from(*v);
                        if l < local.len() as f64 {
                            *v = Typed::U16.store(local[l as usize]) as u16;
                        }
                    }
                }
            }
            for &root in &d.root_bones {
                w.add(mesh, root);
            }
            let g = &rec.geom_bind_transform;
            let bind = if g.truthy() && g.length() == Some(16) {
                usd_matrix(g)
            } else {
                M4::IDENTITY
            };
            if let Kind::Mesh { skin, .. } = &mut w.objects[mesh].kind {
                *skin = Some((Some(d.skeleton), bind));
            }
        }
        Ok(())
    }

    fn build_animations(&self) -> Result<Vec<UsdClip>> {
        let mut clips = vec![];
        for path in self.specs.paths() {
            let Some(spec) = self.spec(&path) else {
                continue;
            };
            if spec.spec_type != PRIM || !is_str(&spec.fields.get("typeName"), "SkelAnimation") {
                continue;
            }
            if let Some(c) = self.animation_clip(&path)? {
                clips.push(c);
            }
        }
        let tracks = self.transform_animations()?;
        if !tracks.is_empty() {
            clips.push(clip("TransformAnimation", tracks));
        }
        Ok(clips)
    }
    fn time_samples(&self, path: &str) -> Option<(Value, Value)> {
        let ts = self.spec(path)?.fields.get("timeSamples");
        ts.truthy().then(|| (ts.get("times"), ts.get("values")))
    }
    fn transform_animations(&self) -> Result<Vec<UsdTrack>> {
        let mut tracks = vec![];
        let fps = num(&self.fps);
        for path in self.specs.paths() {
            let Some(spec) = self.spec(&path) else {
                continue;
            };
            if spec.spec_type != PRIM {
                continue;
            }
            let t = spec.fields.get("typeName");
            if !(is_str(&t, "Xform") || is_str(&t, "Scope") || is_str(&t, "Mesh")) {
                continue;
            }
            let name = last_part(&path, '/').to_string();
            let mut simple =
                |op: &str, property: &str, n: usize, convert: &dyn Fn(&Value) -> Vec<f64>, kind| {
                    if let Some((times, values)) = self.time_samples(&format!("{path}.{op}")) {
                        let mut kt = vec![];
                        let mut kv = vec![];
                        let mut i = 0.0;
                        while i < times.len_number() {
                            kt.push(num(&times.at_number(i)) / fps);
                            kv.extend(convert(&values.at_number(i)));
                            i += 1.0;
                        }
                        let _ = n;
                        if !kt.is_empty() {
                            tracks.push(track(&format!("{name}.{property}"), kind, &kt, &kv));
                        }
                    }
                };
            let comps =
                |n: usize| move |v: &Value| (0..n).map(|k| num(&v.at(k))).collect::<Vec<f64>>();
            simple(
                "xformOp:orient",
                "quaternion",
                4,
                &comps(4),
                UsdTrackKind::Quaternion,
            );
            simple(
                "xformOp:rotateXYZ",
                "quaternion",
                4,
                &|r: &Value| {
                    let a = |i: usize| num(&r.at(i)) * PI / 180.0;
                    let q = Q::from_euler(Euler::new(a(0), a(1), a(2), Order::Zyx));
                    vec![q.x, q.y, q.z, q.w]
                },
                UsdTrackKind::Quaternion,
            );
            simple(
                "xformOp:translate",
                "position",
                3,
                &comps(3),
                UsdTrackKind::Vector,
            );
            simple("xformOp:scale", "scale", 3, &comps(3), UsdTrackKind::Vector);
            let props = spec.fields.get("properties");
            for prop in props.items() {
                let Value::String(prop) = prop else {
                    return Err(type_error("prop.startsWith is not a function"));
                };
                if !prop.starts_with("xformOp:transform") {
                    continue;
                }
                let Some((times, values)) = self.time_samples(&format!("{path}.{prop}")) else {
                    continue;
                };
                let (mut pt, mut pv, mut qv, mut sv) = (vec![], vec![], vec![], vec![]);
                let mut i = 0.0;
                while i < times.len_number() {
                    let m = values.at_number(i);
                    if !m.truthy() || m.len_number() < 16.0 {
                        i += 1.0;
                        continue;
                    }
                    let t = num(&times.at_number(i)) / fps;
                    let (p, q, s) = usd_matrix(&m).decompose();
                    pt.push(t);
                    pv.extend([p.x, p.y, p.z]);
                    qv.extend([q.x, q.y, q.z, q.w]);
                    sv.extend([s.x, s.y, s.z]);
                    i += 1.0;
                }
                if !pt.is_empty() {
                    tracks.push(track(
                        &format!("{name}.position"),
                        UsdTrackKind::Vector,
                        &pt,
                        &pv,
                    ));
                    tracks.push(track(
                        &format!("{name}.quaternion"),
                        UsdTrackKind::Quaternion,
                        &pt,
                        &qv,
                    ));
                    tracks.push(track(
                        &format!("{name}.scale"),
                        UsdTrackKind::Vector,
                        &pt,
                        &sv,
                    ));
                }
                break;
            }
        }
        Ok(tracks)
    }
    fn animation_clip(&self, path: &str) -> Result<Option<UsdClip>> {
        let attrs = self.attributes(path)?;
        let joints = attrs.get("joints");
        if !joints.truthy() || joints.length() == Some(0) {
            return Ok(None);
        }
        let fps = num(&self.fps);
        let mut tracks = vec![];
        for (attr, property, n, kind) in [
            ("rotations", "quaternion", 4usize, UsdTrackKind::Quaternion),
            ("translations", "position", 3, UsdTrackKind::Vector),
            ("scales", "scale", 3, UsdTrackKind::Vector),
        ] {
            let Some((times, values)) = self.time_samples(&format!("{path}.{attr}")) else {
                continue;
            };
            if !times.truthy() || !values.truthy() {
                continue;
            }
            for (j, joint) in joints.items().iter().enumerate() {
                let Value::String(jn) = joint else {
                    return Err(type_error("split is not a function"));
                };
                let name = last_part(jn, '/');
                let (mut kt, mut kv) = (vec![], vec![]);
                let mut t = 0.0;
                while t < times.len_number() {
                    let d = values.at_number(t);
                    if !d.truthy() || d.len_number() < ((j + 1) * n) as f64 {
                        t += 1.0;
                        continue;
                    }
                    kt.push(num(&times.at_number(t)) / fps);
                    for k in 0..n {
                        kv.push(num(&d.at(j * n + k)));
                    }
                    t += 1.0;
                }
                if !kt.is_empty() {
                    tracks.push(track(&format!("{name}.{property}"), kind, &kt, &kv));
                }
            }
        }
        if tracks.is_empty() {
            return Ok(None);
        }
        Ok(Some(clip(last_part(path, '/'), tracks)))
    }

    fn cache(&self) -> std::cell::Ref<'_, HashMap<String, usize>> {
        self.texture_cache.borrow()
    }
    fn cache_set(&self, k: &str, t: usize) {
        self.texture_cache.borrow_mut().insert(k.into(), t);
    }
}

fn clip(name: &str, tracks: Vec<UsdTrack>) -> UsdClip {
    // resetDuration(): Math.max over the tracks' last times.
    let mut duration = 0.0;
    for t in &tracks {
        duration = js_max(duration, t.times.last().map_or(f64::NAN, |&v| f64::from(v)));
    }
    UsdClip {
        name: name.into(),
        duration,
        tracks,
    }
}
fn track(name: &str, kind: UsdTrackKind, times: &[f64], values: &[f64]) -> UsdTrack {
    UsdTrack {
        name: name.into(),
        kind,
        times: times.iter().map(|&v| v as f32).collect(),
        values: values.iter().map(|&v| v as f32).collect(),
    }
}

fn srgb(r: &Value, g: &Value, b: &Value) -> [f64; 3] {
    [
        srgb_to_linear(num(r)),
        srgb_to_linear(num(g)),
        srgb_to_linear(num(b)),
    ]
}
fn wrap_mode(v: &Value) -> u32 {
    match v.as_str() {
        Some("repeat") => REPEAT,
        Some("mirror") => MIRROR,
        Some("clamp") => CLAMP,
        _ => REPEAT,
    }
}
#[allow(clippy::excessive_precision)]
fn color_temperature(kelvin: f64) -> [f64; 3] {
    let t = kelvin / 100.0;
    let (r, g) = if t <= 66.0 {
        (
            1.0,
            0.39008157876901960784 * log(t) - 0.63184144378862745098,
        )
    } else {
        (
            1.29293618606274509804 * pow(t - 60.0, -0.1332047592),
            1.12989086089529411765 * pow(t - 60.0, -0.0755148492),
        )
    };
    let b = if t >= 66.0 {
        1.0
    } else if t <= 19.0 {
        0.0
    } else {
        0.54320678911019607843 * log(t - 10.0) - 1.19625408914
    };
    // Math.min( Math.max( v, 0 ), 1 ), NaN through.
    let c = |v: f64| if v.is_nan() { v } else { v.clamp(0.0, 1.0) };
    [c(r), c(g), c(b)]
}

/// `new Float32Array( n )`'s length check.
fn typed_len(n: f64) -> Result<usize> {
    let n = if n.is_nan() { 0.0 } else { n.trunc() };
    if !(0.0..=4294967295.0).contains(&n) {
        return Err(Error::Asset(
            "RangeError: Invalid typed array length".into(),
        ));
    }
    Ok(n as usize)
}
/// `Array.from( { length: n }, ( _, i ) => i )`.
fn identity(n: f64) -> Result<Value> {
    if !(n >= 0.0 && n.fract() == 0.0 && n <= 4294967295.0) {
        return Err(Error::Asset("RangeError: Invalid array length".into()));
    }
    Ok(Value::numbers((0..n as usize).map(|i| i as f64).collect()))
}
fn indices_value_or(indices: &Option<Vec<Value>>, fallback: &Value) -> Value {
    match indices {
        Some(i) => Value::array(i.clone()),
        None => fallback.clone(),
    }
}
/// The geometry's Float32 attribute from values ( `new Float32Array( v )` ).
fn set_f32(g: &mut BufferGeometry, name: &str, values: &[Value], item: usize) -> Result<()> {
    let data: Vec<f32> = values.iter().map(|v| num(v) as f32).collect();
    // BufferAttribute's count is length / itemSize; a ragged tail keeps the
    // values ( the renderer reads whole items ).
    let whole = data.len() - data.len() % item;
    let mut data = data;
    if whole != data.len() {
        data.truncate(whole);
    }
    g.set_attribute(
        name,
        Attribute::F32(BufferAttribute::new(data, item, false)?),
    );
    Ok(())
}
/// _expandAttribute( data, indices, itemSize ).
fn expand(data: &Value, indices: &[Value], item: usize) -> Vec<Value> {
    let mut out = Vec::with_capacity(indices.len() * item);
    for i in indices {
        let src = num(i);
        for j in 0..item {
            out.push(data.at_number(src * item as f64 + j as f64));
        }
    }
    out
}
/// _expandAttribute with a numeric item size ( elementSize ).
fn expand_n(data: &Value, indices: &[Value], item: f64) -> Vec<Value> {
    let n = if item.is_nan() { 0.0 } else { item };
    let mut out = vec![];
    for i in indices {
        let src = num(i);
        let mut j = 0.0;
        while j < n {
            out.push(data.at_number(src * item + j));
            j += 1.0;
        }
    }
    out
}
/// _applyTriangulationPattern( indices, pattern ).
fn apply_pattern(indices: &Value, pattern: &[Value]) -> Vec<Value> {
    pattern.iter().map(|p| indices.at_number(num(p))).collect()
}
fn build_hole_map(holes: &Value) -> HoleMap {
    let mut m = HoleMap::default();
    if !holes.truthy() || holes.length() == Some(0) {
        return m;
    }
    let mut i = 0.0;
    while i < holes.len_number() {
        let hole = holes.at_number(i);
        let parent = holes.at_number(i + 1.0);
        if !m.hole_faces.iter().any(|h| h.same_value_zero(&hole)) {
            m.hole_faces.push(hole.clone());
        }
        match m
            .parent_to_holes
            .iter_mut()
            .find(|(k, _)| k.same_value_zero(&parent))
        {
            Some((_, v)) => v.push(hole),
            None => m.parent_to_holes.push((parent, vec![hole])),
        }
        i += 2.0;
    }
    m
}
/// The Map key of a value ( SameValueZero ).
fn key_bits(v: &Value) -> String {
    match v {
        Value::Number(n) if *n == 0.0 => "n0".into(),
        Value::Number(n) if n.is_nan() => "nNaN".into(),
        Value::Number(n) => format!("n{}", n.to_bits()),
        Value::Undefined => "u".into(),
        v => format!("s{}", v.to_js_string()),
    }
}
/// _triangulateIndicesWithPattern( indices, counts, points, holeMap ).
fn triangulate_with_pattern(
    indices: &Value,
    counts: &Value,
    points: Option<&Value>,
    holes: &HoleMap,
) -> Result<Triangulation> {
    let mut tri = vec![];
    let mut pattern = vec![];
    let counts_v = counts.items();
    let mut face_offsets = vec![];
    let mut acc = 0.0;
    for c in &counts_v {
        face_offsets.push(acc);
        acc += num(c);
    }
    let have_points = points.is_some_and(|p| p.len_number() > 0.0);
    let idx = |i: f64| indices.at_number(i);
    let mut offset = 0.0;
    for (i, c) in counts_v.iter().enumerate() {
        let count = num(c);
        if holes.is_hole(i as f64) {
            offset += count;
            continue;
        }
        let face_holes = holes.holes(i as f64);
        if let (Some(h), true) = (face_holes.filter(|h| !h.is_empty()), have_points) {
            let mut map: HashMap<String, f64> = HashMap::new();
            let mut face = vec![];
            let mut j = 0.0;
            while j < count {
                let v = idx(offset + j);
                map.insert(key_bits(&v), offset + j);
                face.push(v);
                j += 1.0;
            }
            let mut contours = vec![];
            for hf in h {
                let hfn = num(hf);
                let ho = if hfn >= 0.0 && hfn.fract() == 0.0 {
                    face_offsets.get(hfn as usize).copied().unwrap_or(f64::NAN)
                } else {
                    f64::NAN
                };
                let hc = num(&counts.at_number(hfn));
                let mut hole = vec![];
                let mut j = 0.0;
                while j < hc {
                    let v = idx(ho + j);
                    map.insert(key_bits(&v), ho + j);
                    hole.push(v);
                    j += 1.0;
                }
                contours.push(hole);
            }
            for t in
                triangulate_ngon_with_holes(&face, &contours, points.unwrap_or(&Value::Undefined))
            {
                for v in t {
                    pattern.push(
                        map.get(&key_bits(&v))
                            .map_or(Value::Undefined, |&p| Value::Number(p)),
                    );
                    tri.push(v);
                }
            }
        } else if count == 3.0 {
            tri.extend([idx(offset), idx(offset + 1.0), idx(offset + 2.0)]);
            pattern.extend([offset, offset + 1.0, offset + 2.0].map(Value::Number));
        } else if count == 4.0 {
            tri.extend([
                idx(offset),
                idx(offset + 1.0),
                idx(offset + 2.0),
                idx(offset),
                idx(offset + 2.0),
                idx(offset + 3.0),
            ]);
            pattern.extend(
                [
                    offset,
                    offset + 1.0,
                    offset + 2.0,
                    offset,
                    offset + 2.0,
                    offset + 3.0,
                ]
                .map(Value::Number),
            );
        } else if count > 4.0 {
            if have_points {
                let mut face = vec![];
                let mut j = 0.0;
                while j < count {
                    face.push(idx(offset + j));
                    j += 1.0;
                }
                for t in triangulate_ngon(&face, points.unwrap_or(&Value::Undefined)) {
                    for v in &t {
                        let local = face
                            .iter()
                            .position(|f| f.strict_eq(v))
                            .map_or(-1.0, |p| p as f64);
                        pattern.push(Value::Number(offset + local));
                    }
                    tri.extend(t);
                }
            } else {
                let mut j = 1.0;
                while j < count - 1.0 {
                    tri.extend([idx(offset), idx(offset + j), idx(offset + j + 1.0)]);
                    pattern.extend([offset, offset + j, offset + j + 1.0].map(Value::Number));
                    j += 1.0;
                }
            }
        }
        offset += count;
    }
    Ok(Triangulation {
        indices: tri,
        pattern,
    })
}
fn point(points: &Value, idx: &Value) -> V3 {
    let i = num(idx);
    // new Vector3( x, y, z ): undefined components default to 0.
    let c = |k: f64| {
        let v = points.at_number(i * 3.0 + k);
        if v.is_undefined() { 0.0 } else { num(&v) }
    };
    V3::new(c(0.0), c(1.0), c(2.0))
}
/// The projection basis of a polygon ( Newell's normal ).
fn basis(contour: &[V3]) -> (V3, V3) {
    let mut n = V3::ZERO;
    for i in 0..contour.len() {
        let (c, x) = (contour[i], contour[(i + 1) % contour.len()]);
        n.x += (c.y - x.y) * (c.z + x.z);
        n.y += (c.z - x.z) * (c.x + x.x);
        n.z += (c.x - x.x) * (c.y + x.y);
    }
    let n = n.normalize();
    let mut tangent = if n.y.abs() > 0.9 {
        V3::new(1.0, 0.0, 0.0)
    } else {
        V3::new(0.0, 1.0, 0.0)
    };
    let bitangent = V3::cross(n, tangent).normalize();
    tangent = V3::cross(bitangent, n).normalize();
    (tangent, bitangent)
}
fn triangulate_ngon(face: &[Value], points: &Value) -> Vec<Vec<Value>> {
    let contour: Vec<V3> = face.iter().map(|i| point(points, i)).collect();
    let (t, b) = basis(&contour);
    let mut c2: Vec<[f64; 2]> = contour.iter().map(|p| [p.dot(t), p.dot(b)]).collect();
    crate::shape_utils::triangulate(&mut c2, &mut [])
        .into_iter()
        .map(|tri| {
            tri.iter()
                .map(|&k| face.get(k).cloned().unwrap_or_default())
                .collect()
        })
        .collect()
}
fn triangulate_ngon_with_holes(
    outer: &[Value],
    holes: &[Vec<Value>],
    points: &Value,
) -> Vec<Vec<Value>> {
    let outer3: Vec<V3> = outer.iter().map(|i| point(points, i)).collect();
    let (t, b) = basis(&outer3);
    let mut o2: Vec<[f64; 2]> = outer3.iter().map(|p| [p.dot(t), p.dot(b)]).collect();
    let mut h2: Vec<Vec<[f64; 2]>> = holes
        .iter()
        .map(|h| {
            h.iter()
                .map(|i| point(points, i))
                .map(|p| [p.dot(t), p.dot(b)])
                .collect()
        })
        .collect();
    let mut all: Vec<Value> = outer.to_vec();
    for h in holes {
        all.extend(h.iter().cloned());
    }
    crate::shape_utils::triangulate(&mut o2, &mut h2)
        .into_iter()
        .map(|tri| {
            tri.iter()
                .map(|&k| all.get(k).cloned().unwrap_or_default())
                .collect()
        })
        .collect()
}
/// _computeVertexNormals( points, indices ): a Float32Array.
fn compute_vertex_normals(points: &Value, indices: &Value) -> Result<Vec<f64>> {
    let vertices = points.len_number() / 3.0;
    let mut normals = vec![0.0f64; typed_len(vertices * 3.0)?];
    let p = |i: f64| num(&points.at_number(i));
    let add = |n: &mut Vec<f64>, i: f64, v: f64| {
        if i >= 0.0 && i.fract() == 0.0 && (i as usize) < n.len() {
            let k = i as usize;
            n[k] = Typed::F32.store(n[k] + v);
        }
    };
    let mut i = 0.0;
    while i < indices.len_number() {
        let (a, b, c) = (
            num(&indices.at_number(i)),
            num(&indices.at_number(i + 1.0)),
            num(&indices.at_number(i + 2.0)),
        );
        let (ax, ay, az) = (p(a * 3.0), p(a * 3.0 + 1.0), p(a * 3.0 + 2.0));
        let (bx, by, bz) = (p(b * 3.0), p(b * 3.0 + 1.0), p(b * 3.0 + 2.0));
        let (cx, cy, cz) = (p(c * 3.0), p(c * 3.0 + 1.0), p(c * 3.0 + 2.0));
        let (e1x, e1y, e1z) = (bx - ax, by - ay, bz - az);
        let (e2x, e2y, e2z) = (cx - ax, cy - ay, cz - az);
        let nx = e1y * e2z - e1z * e2y;
        let ny = e1z * e2x - e1x * e2z;
        let nz = e1x * e2y - e1y * e2x;
        for v in [a, b, c] {
            add(&mut normals, v * 3.0, nx);
            add(&mut normals, v * 3.0 + 1.0, ny);
            add(&mut normals, v * 3.0 + 2.0, nz);
        }
        i += 3.0;
    }
    let n = normals.len() / 3;
    for i in 0..(vertices.max(0.0).ceil() as usize).min(n) {
        let (x, y, z) = (normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]);
        let len = (x * x + y * y + z * z).sqrt();
        if len > 0.0 {
            for k in 0..3 {
                normals[i * 3 + k] = Typed::F32.store(normals[i * 3 + k] / len);
            }
        }
    }
    Ok(normals)
}
/// _selectTopWeights( srcIndices, srcWeights, elementSize, numVertices,
/// dstIndices, dstWeights ), into Uint16 and Float32 stores.
#[allow(clippy::needless_range_loop)]
fn select_top_weights(
    si: &Value,
    sw: &Value,
    es: f64,
    vertices: f64,
    di: &mut [f64],
    dw: &mut [f64],
) {
    let or0 = |v: Value| if v.truthy() { num(&v) } else { 0.0 };
    let store = |d: &mut [f64], i: f64, v: f64, t: Typed| {
        if i >= 0.0 && i.fract() == 0.0 && (i as usize) < d.len() {
            d[i as usize] = t.store(v);
        }
    };
    if es <= 4.0 {
        let mut i = 0.0;
        while i < vertices {
            for j in 0..4 {
                let j = j as f64;
                if j < es {
                    store(di, i * 4.0 + j, or0(si.at_number(i * es + j)), Typed::U16);
                    store(dw, i * 4.0 + j, or0(sw.at_number(i * es + j)), Typed::F32);
                } else {
                    store(di, i * 4.0 + j, 0.0, Typed::U16);
                    store(dw, i * 4.0 + j, 0.0, Typed::F32);
                }
            }
            i += 1.0;
        }
        return;
    }
    let n = es as usize;
    let mut order = vec![0usize; n];
    let mut i = 0.0;
    while i < vertices {
        let base = i * es;
        for (j, o) in order.iter_mut().enumerate() {
            *o = j;
        }
        for k in 0..4.min(n) {
            let mut max_idx = k;
            let mut max_w = or0(sw.at_number(base + order[k] as f64));
            for j in k + 1..n {
                let w = or0(sw.at_number(base + order[j] as f64));
                if w > max_w {
                    max_w = w;
                    max_idx = j;
                }
            }
            if max_idx != k {
                order.swap(k, max_idx);
            }
        }
        let mut total = 0.0;
        for &o in order.iter().take(4) {
            total += or0(sw.at_number(base + o as f64));
        }
        for j in 0..4 {
            let at = i * 4.0 + j as f64;
            let s = order.get(j).copied().unwrap_or(0) as f64;
            if total > 0.0 {
                store(di, at, or0(si.at_number(base + s)), Typed::U16);
                store(dw, at, or0(sw.at_number(base + s)) / total, Typed::F32);
            } else {
                store(di, at, 0.0, Typed::U16);
                store(dw, at, 0.0, Typed::F32);
            }
        }
        i += 1.0;
    }
}
fn find_uv_primvar(f: &Obj) -> (Value, Value) {
    for key in f.keys() {
        if !key.starts_with("primvars:") {
            continue;
        }
        if key.ends_with(":typeName") || key.ends_with(":elementSize") || key.ends_with(":indices")
        {
            continue;
        }
        if key.contains("skel:") {
            continue;
        }
        let t = f.get(&format!("{key}:typeName"));
        if t.truthy()
            && let Value::String(ts) = &t
            && ts.contains("texCoord")
        {
            return (f.get(&key), f.get(&format!("{key}:indices")));
        }
    }
    let mut uvs = f.get("primvars:st");
    if !uvs.truthy() {
        uvs = f.get("primvars:UVMap");
    }
    (uvs, f.get("primvars:st:indices"))
}
/// _flattenMatrixArray( matrices, numMatrices ).
fn flatten_matrix_array(m: &Value, n: f64) -> Option<Value> {
    if !m.truthy() || m.length() == Some(0) {
        return None;
    }
    if m.at(0).is_number() {
        return Some(m.clone());
    }
    let mut flat = vec![];
    let mut k = 0.0;
    while k < n {
        for row in 0..4 {
            let r = m.at_number(k * 4.0 + row as f64);
            if r.truthy() && r.length() == Some(4) {
                flat.extend([r.at(0), r.at(1), r.at(2), r.at(3)]);
            } else {
                flat.extend((0..4).map(|c| Value::Number(if c == row { 1.0 } else { 0.0 })));
            }
        }
        k += 1.0;
    }
    Some(Value::array(flat))
}
/// BufferGeometry.applyMatrix4( m ): positions, and normals through the
/// normal matrix ( normalized ).
fn apply_matrix4(g: &mut BufferGeometry, m: &M4) -> Result<()> {
    if let Some(Attribute::F32(a)) = g.attributes.get_mut("position") {
        for p in a.array_mut().as_chunks_mut::<3>().0 {
            let v = V3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])).apply_matrix4(m);
            p[0] = v.x as f32;
            p[1] = v.y as f32;
            p[2] = v.z as f32;
        }
    }
    let nm = M3::normal_matrix(m);
    if let Some(Attribute::F32(a)) = g.attributes.get_mut("normal") {
        for p in a.array_mut().as_chunks_mut::<3>().0 {
            let v = V3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]))
                .apply_matrix3(&nm)
                .normalize();
            p[0] = v.x as f32;
            p[1] = v.y as f32;
            p[2] = v.z as f32;
        }
    }
    Ok(())
}

/// USDLoader.parse( buffer, path ) for a USDZ package, a USDC crate or USDA
/// text.
pub(super) fn load(bytes: &[u8], path: &str) -> Result<UsdScene> {
    let crate_file = |b: &[u8]| b.starts_with(b"PXR-USDC");
    let ext = |name: &str| -> String {
        let Some(dot) = name.rfind('.') else {
            return String::new();
        };
        if name.rfind('/').is_some_and(|s| s > dot) {
            return String::new();
        }
        name[dot + 1..].to_lowercase()
    };
    let decode = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    let mut w = World::default();
    let (specs, assets, base) = if crate_file(bytes) {
        (
            Rc::new(super::usdc::parse(bytes)?),
            Assets::default(),
            path.to_string(),
        )
    } else if bytes.starts_with(b"PK") {
        let zip = crate::compression::unzip(bytes)?;
        let mut assets = Assets::default();
        for (name, data) in &zip {
            let e = ext(name);
            if matches!(e.as_str(), "png" | "jpg" | "jpeg" | "avif") {
                assets.set(name, Asset::Bytes(Rc::new(data.clone())));
                continue;
            }
            if !matches!(e.as_str(), "usd" | "usda" | "usdc") {
                continue;
            }
            let specs = if crate_file(data) {
                super::usdc::parse(data)?
            } else {
                super::usda::parse(&decode(data))?
            };
            assets.set(name, Asset::Layer(Rc::new(specs)));
        }
        // findUSD(): the first entry is the root layer.
        let Some((first, data)) = zip.first() else {
            return Err(Error::Asset(
                "THREE.USDLoader: Invalid USDZ package. The first ZIP entry must be a USD layer (.usd/.usda/.usdc).".into(),
            ));
        };
        let e = ext(first);
        if !matches!(e.as_str(), "usda" | "usdc" | "usd") {
            return Err(Error::Asset(
                "THREE.USDLoader: Invalid USDZ package. The first ZIP entry must be a USD layer (.usd/.usda/.usdc).".into(),
            ));
        }
        let _ = data;
        let base = first
            .rfind('/')
            .map_or(String::new(), |s| first[..s].to_string());
        let Some(Asset::Layer(root)) = assets.get(first).cloned() else {
            return Err(Error::Asset(format!(
                "THREE.USDLoader: Failed to parse root layer \"{first}\"."
            )));
        };
        (root, assets, base)
    } else {
        (
            Rc::new(super::usda::parse(&decode(bytes))?),
            Assets::default(),
            path.to_string(),
        )
    };
    let mut composer = Composer::new(specs, Rc::new(assets), Obj::new(), &base);
    let (group, clips, skeleton) = composer.compose(&mut w)?;
    Ok(finish(w, group, clips, skeleton))
}

/// The group in traverse() order, with world matrices.
fn finish(w: World, group: usize, clips: Vec<UsdClip>, skeleton: Option<usize>) -> UsdScene {
    let mut nodes = vec![];
    let mut index_of: HashMap<usize, usize> = HashMap::new();
    let mut stack = vec![(group, None, M4::IDENTITY)];
    let mut mesh_refs = vec![];
    while let Some((o, parent, parent_world)) = stack.pop() {
        let obj = &w.objects[o];
        let mut local = M4::compose(obj.position, obj.quaternion, obj.scale);
        if let Some(p) = obj.pivot {
            let te = &mut local.0;
            te[12] += p.x - te[0] * p.x - te[4] * p.y - te[8] * p.z;
            te[13] += p.y - te[1] * p.x - te[5] * p.y - te[9] * p.z;
            te[14] += p.z - te[2] * p.x - te[6] * p.y - te[10] * p.z;
        }
        let world = if parent.is_some() {
            M4::multiply_matrices(&parent_world, &local)
        } else {
            local
        };
        let index = nodes.len();
        index_of.insert(o, index);
        let kind = match &obj.kind {
            Kind::Group => UsdNodeKind::Group,
            Kind::Object3D => UsdNodeKind::Object3D,
            Kind::Bone => UsdNodeKind::Bone,
            Kind::Camera(k) | Kind::Light(k) => k.clone(),
            Kind::Mesh { .. } => {
                mesh_refs.push((index, o));
                UsdNodeKind::Object3D
            }
        };
        nodes.push(UsdNode {
            name: obj.name.clone(),
            parent,
            kind,
            position: obj.position.to_array(),
            quaternion: obj.quaternion.to_array(),
            scale: obj.scale.to_array(),
            pivot: obj.pivot.map(V3::to_array),
            matrix_world: world.0,
        });
        for &c in obj.children.iter().rev() {
            stack.push((c, Some(index), world));
        }
    }
    for (index, o) in mesh_refs {
        if let Kind::Mesh {
            geometry,
            materials,
            multi,
            skin,
        } = &w.objects[o].kind
        {
            nodes[index].kind = UsdNodeKind::Mesh {
                geometry: *geometry,
                materials: materials.clone(),
                multi: *multi,
                skin: skin.map(|(s, m)| (s, m.0)),
            };
        }
    }
    let n = |v: &Value| num(v);
    UsdScene {
        nodes,
        geometries: w.geometries,
        materials: w
            .materials
            .iter()
            .map(|m| UsdMaterial {
                color: m.color,
                emissive: m.emissive,
                emissive_intensity: n(&m.emissive_intensity),
                roughness: n(&m.roughness),
                metalness: n(&m.metalness),
                ior: n(&m.ior),
                specular_intensity: n(&m.specular_intensity),
                specular_color: m.specular_color,
                clearcoat: n(&m.clearcoat),
                clearcoat_roughness: n(&m.clearcoat_roughness),
                sheen: 0.0,
                transmission: n(&m.transmission),
                thickness: n(&m.thickness),
                attenuation_color: m.attenuation_color,
                attenuation_distance: n(&m.attenuation_distance),
                iridescence: n(&m.iridescence),
                iridescence_ior: n(&m.iridescence_ior),
                iridescence_thickness_range: [
                    n(&m.iridescence_thickness_range[0]),
                    n(&m.iridescence_thickness_range[1]),
                ],
                anisotropy: n(&m.anisotropy),
                opacity: n(&m.opacity),
                transparent: m.transparent,
                alpha_test: n(&m.alpha_test),
                normal_scale: m.normal_scale,
                map: m.maps.get("map").copied(),
                emissive_map: m.maps.get("emissiveMap").copied(),
                normal_map: m.maps.get("normalMap").copied(),
                roughness_map: m.maps.get("roughnessMap").copied(),
                metalness_map: m.maps.get("metalnessMap").copied(),
                ao_map: m.maps.get("aoMap").copied(),
                specular_color_map: m.maps.get("specularColorMap").copied(),
            })
            .collect(),
        textures: w
            .textures
            .iter()
            .map(|t| UsdTexture {
                image: t.image.clone(),
                srgb: t.srgb,
                wrap_s: t.wrap[0],
                wrap_t: t.wrap[1],
                repeat: [n(&t.repeat[0]), n(&t.repeat[1])],
                offset: [n(&t.offset[0]), n(&t.offset[1])],
                rotation: n(&t.rotation),
                channel: t.channel,
                scale: t
                    .scale
                    .as_ref()
                    .map(|v| v.items().iter().map(num).collect()),
                bias: t.bias.as_ref().map(|v| v.items().iter().map(num).collect()),
            })
            .collect(),
        skeletons: w
            .skeletons
            .iter()
            .map(|(bones, inv)| UsdSkeleton {
                bones: bones.iter().map(|b| index_of.get(b).copied()).collect(),
                bone_inverses: inv.iter().map(|m| m.0).collect(),
            })
            .collect(),
        clips,
        skeleton,
    }
}
