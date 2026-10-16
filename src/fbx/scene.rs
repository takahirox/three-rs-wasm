//! FBXTreeParser: connections, images, textures, materials, deformers and
//! models into the scene graph, then the transforms, bind pose, skeleton
//! binding and clips.
use super::geometry::{self, Geo};
use super::js::{Obj, Value, number_key, parse_float_value, parse_int, parse_int_value};
use super::math::{Euler, M4, Order, Q, V3, deg_to_rad, srgb_to_linear};
use super::{
    FbxClip, FbxGeometry, FbxImage, FbxMaterial, FbxMaterialKind, FbxMesh, FbxNode, FbxNodeKind,
    FbxScene, FbxSkin, FbxTexture, Options,
};
use crate::math::{Color, Matrix4, Quaternion, Vector2, Vector3};
use crate::{Error, Result};
use std::collections::HashMap;

pub(super) fn bad(what: &str) -> Error {
    Error::Asset(format!("THREE.FBXLoader: {what}"))
}

/// A JavaScript Map key ( SameValueZero ) for an id value.
pub(super) fn map_key(v: &Value) -> String {
    match v {
        Value::Number(n) => format!("n{}", number_key(*n)),
        Value::String(s) => format!("s{s}"),
        Value::Undefined => "u".into(),
        other => format!("o{}", other.to_key()),
    }
}

#[derive(Clone, Debug)]
pub(super) struct Rel {
    pub id: Value,
    pub relationship: Value,
}
#[derive(Clone, Debug, Default)]
pub(super) struct Links {
    pub parents: Vec<Rel>,
    pub children: Vec<Rel>,
}
pub(super) struct Connections(HashMap<String, Links>);
impl Connections {
    pub(super) fn get(&self, id: &Value) -> Option<&Links> {
        self.0.get(&map_key(id))
    }
    pub(super) fn has(&self, id: &Value) -> bool {
        self.0.contains_key(&map_key(id))
    }
    /// `connections.get( id )` where the original dereferences the result.
    pub(super) fn links(&self, id: &Value) -> Result<&Links> {
        self.get(id)
            .ok_or_else(|| bad(&format!("missing connections for {}", id.to_key())))
    }
}

pub(super) struct Ctx<'a> {
    pub tree: &'a Obj,
    pub objects: Obj,
    pub connections: Connections,
}
impl Ctx<'_> {
    /// `fbxTree.Objects.<kind>` ( undefined when absent ).
    pub(super) fn objects(&self, kind: &str) -> Value {
        self.objects.get(kind)
    }
    /// `for ( const nodeID in fbxTree.Objects.<kind> )`.
    pub(super) fn nodes(&self, kind: &str) -> Vec<(String, Value)> {
        match self.objects(kind) {
            Value::Object(o) => o
                .entries()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            _ => vec![],
        }
    }
}

/// `.value` of a Properties70 entry.
pub(super) fn prop_value(node: &Value, name: &str) -> Value {
    node.get(name).get("value")
}

/// The numbers of an array value; a string ( an ASCII `Vector` property
/// left unparsed ) gives the numeric value of each character.
pub(super) fn numbers(v: &Value) -> Vec<f64> {
    match v {
        Value::String(s) => s
            .encode_utf16()
            .map(|u| super::js::string_to_number(&String::from_utf16_lossy(&[u])))
            .collect(),
        v => v.to_f64s().unwrap_or_default(),
    }
}

/// getEulerOrder( order ): the three.js intrinsic order for the FBX enum.
pub(super) fn euler_order(order: &Value) -> Option<Order> {
    let n = if order.truthy() {
        order.to_number()
    } else {
        0.0
    };
    const ENUMS: [Order; 6] = [
        Order::Zyx,
        Order::Yzx,
        Order::Xzy,
        Order::Zxy,
        Order::Yxz,
        Order::Xyz,
    ];
    if n == 6.0 {
        return Some(Order::Zyx);
    }
    if n.fract() == 0.0 && (0.0..6.0).contains(&n) {
        Some(ENUMS[n as usize])
    } else {
        None
    }
}

/// `ColorManagement.colorSpaceToWorking( new Color().fromArray( a ), SRGBColorSpace )`.
pub(super) fn srgb_color(a: &[f64]) -> [f64; 3] {
    let g = |i: usize| a.get(i).copied().unwrap_or(f64::NAN);
    [
        srgb_to_linear(g(0)),
        srgb_to_linear(g(1)),
        srgb_to_linear(g(2)),
    ]
}
/// `new Color( hex )`.
pub(super) fn hex_color(hex: u32) -> [f64; 3] {
    srgb_color(&[
        f64::from((hex >> 16) & 255) / 255.0,
        f64::from((hex >> 8) & 255) / 255.0,
        f64::from(hex & 255) / 255.0,
    ])
}

/// Transform data from a model node ( getTransformData and its geometric
/// counterpart ).
#[derive(Clone, Debug, Default)]
pub(super) struct TransformData {
    pub inherit_type: Option<f64>,
    pub euler_order: Option<Order>,
    pub translation: Option<Vec<f64>>,
    pub pre_rotation: Option<Vec<f64>>,
    pub rotation: Option<Vec<f64>>,
    pub post_rotation: Option<Vec<f64>>,
    pub scale: Option<Vec<f64>>,
    pub scaling_offset: Option<Vec<f64>>,
    pub scaling_pivot: Option<Vec<f64>>,
    pub rotation_offset: Option<Vec<f64>>,
    pub rotation_pivot: Option<Vec<f64>>,
    pub parent_matrix: Option<M4>,
    pub parent_matrix_world: Option<M4>,
}

/// A transform property ( present and truthy ).
pub(super) fn transform_prop(node: &Value, name: &str) -> Option<Vec<f64>> {
    if !node.has(name) {
        return None;
    }
    let v = prop_value(node, name);
    v.truthy().then(|| numbers(&v))
}

/// generateTransform( transformData ).
pub(super) fn generate_transform(t: &TransformData) -> M4 {
    let mut translation = M4::IDENTITY;
    let mut pre_rotation = M4::IDENTITY;
    let mut rotation = M4::IDENTITY;
    let mut post_rotation = M4::IDENTITY;
    let mut scaling = M4::IDENTITY;
    let mut scaling_pivot = M4::IDENTITY;
    let mut scaling_offset = M4::IDENTITY;
    let mut rotation_offset = M4::IDENTITY;
    let mut rotation_pivot = M4::IDENTITY;
    let mut parent_gx = M4::IDENTITY;
    let mut parent_lx = M4::IDENTITY;
    let inherit_type = t
        .inherit_type
        .filter(|v| *v != 0.0 && !v.is_nan())
        .unwrap_or(0.0);
    if let Some(v) = &t.translation {
        translation = translation.set_position(V3::from_array(v));
    }
    let default_order = Order::Zyx;
    let rad = |v: &[f64]| -> [f64; 3] {
        let g = |i: usize| v.get(i).copied().map_or(f64::NAN, deg_to_rad);
        [g(0), g(1), g(2)]
    };
    if let Some(v) = &t.pre_rotation {
        let [x, y, z] = rad(v);
        pre_rotation = M4::rotation_from_euler(Euler::new(x, y, z, default_order));
    }
    if let Some(v) = &t.rotation {
        let [x, y, z] = rad(v);
        rotation =
            M4::rotation_from_euler(Euler::new(x, y, z, t.euler_order.unwrap_or(default_order)));
    }
    if let Some(v) = &t.post_rotation {
        let [x, y, z] = rad(v);
        post_rotation = M4::rotation_from_euler(Euler::new(x, y, z, default_order)).invert();
    }
    if let Some(v) = &t.scale {
        scaling = scaling.scale(V3::from_array(v));
    }
    if let Some(v) = &t.scaling_offset {
        scaling_offset = scaling_offset.set_position(V3::from_array(v));
    }
    if let Some(v) = &t.scaling_pivot {
        scaling_pivot = scaling_pivot.set_position(V3::from_array(v));
    }
    if let Some(v) = &t.rotation_offset {
        rotation_offset = rotation_offset.set_position(V3::from_array(v));
    }
    if let Some(v) = &t.rotation_pivot {
        rotation_pivot = rotation_pivot.set_position(V3::from_array(v));
    }
    if let Some(world) = t.parent_matrix_world {
        parent_lx = t.parent_matrix.unwrap_or(M4::IDENTITY);
        parent_gx = world;
    }
    let lrm = pre_rotation.multiply(&rotation).multiply(&post_rotation);
    let parent_grm = M4::extract_rotation(&parent_gx);
    let parent_tm = M4::IDENTITY.copy_position(&parent_gx);
    let parent_grsm = parent_tm.invert().multiply(&parent_gx);
    let parent_gsm = parent_grm.invert().multiply(&parent_grsm);
    let lsm = scaling;
    let global_rs = if inherit_type == 0.0 {
        parent_grm
            .multiply(&lrm)
            .multiply(&parent_gsm)
            .multiply(&lsm)
    } else if inherit_type == 1.0 {
        parent_grm
            .multiply(&parent_gsm)
            .multiply(&lrm)
            .multiply(&lsm)
    } else {
        let parent_lsm = M4::IDENTITY.scale(V3::from_matrix_scale(&parent_lx));
        let parent_lsm_inv = parent_lsm.invert();
        let parent_gsm_no_local = parent_gsm.multiply(&parent_lsm_inv);
        parent_grm
            .multiply(&lrm)
            .multiply(&parent_gsm_no_local)
            .multiply(&lsm)
    };
    let rotation_pivot_inv = rotation_pivot.invert();
    let scaling_pivot_inv = scaling_pivot.invert();
    let transform = translation
        .multiply(&rotation_offset)
        .multiply(&rotation_pivot)
        .multiply(&pre_rotation)
        .multiply(&rotation)
        .multiply(&post_rotation)
        .multiply(&rotation_pivot_inv)
        .multiply(&scaling_offset)
        .multiply(&scaling_pivot)
        .multiply(&scaling)
        .multiply(&scaling_pivot_inv);
    let local_t = M4::IDENTITY.copy_position(&transform);
    let global_translation = parent_gx.multiply(&local_t);
    let global_t = M4::IDENTITY.copy_position(&global_translation);
    let transform = global_t.multiply(&global_rs);
    transform.premultiply(&parent_gx.invert())
}

/// PropertyBinding.sanitizeNodeName(): whitespace to `_`, `[ ] . : /` removed.
pub(super) fn sanitize(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '[' | ']' | '.' | ':' | '/'))
        .map(|c| if super::js::is_js_space(c) { '_' } else { c })
        .collect()
}
/// `node.attrName ? sanitizeNodeName( attrName ) : ''`.
pub(super) fn model_name(attr: &Value) -> String {
    if attr.truthy() {
        sanitize(&attr.to_key())
    } else {
        String::new()
    }
}

// ------------------------------------------------------------- scene graph

#[derive(Clone, Debug)]
pub(super) struct Light {
    pub kind: LightKind,
    pub color: [f64; 3],
    pub intensity: f64,
    pub distance: f64,
    pub decay: f64,
    pub angle: f64,
    pub penumbra: f64,
    pub target: Option<usize>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LightKind {
    Point,
    Directional,
    Spot,
}

#[derive(Clone, Debug)]
pub(super) enum Kind {
    Group,
    Object3D,
    Bone,
    Mesh {
        geometry: usize,
        materials: Vec<usize>,
        multi: bool,
        skinned: bool,
        skin: Option<(Vec<Option<usize>>, Vec<M4>, M4)>,
        morph_targets: Vec<(String, usize)>,
    },
    Line {
        geometry: Option<usize>,
        material: usize,
    },
    Camera {
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    },
    Light(Light),
    Ambient {
        color: [f64; 3],
        intensity: f64,
    },
}

#[derive(Clone, Debug)]
pub(super) struct Node {
    pub kind: Kind,
    pub name: String,
    pub id: Option<f64>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub position: V3,
    pub quaternion: Q,
    pub scale: V3,
    pub matrix: M4,
    pub matrix_world: M4,
    pub transform_data: Option<TransformData>,
    pub cast_shadow: bool,
}

#[derive(Default)]
pub(super) struct Graph {
    pub nodes: Vec<Node>,
}
impl Graph {
    pub(super) fn insert(&mut self, kind: Kind) -> usize {
        let position = match &kind {
            Kind::Light(l) if l.kind != LightKind::Point => V3::new(0.0, 1.0, 0.0),
            _ => V3::ZERO,
        };
        self.nodes.push(Node {
            kind,
            name: String::new(),
            id: None,
            parent: None,
            children: vec![],
            position,
            quaternion: Q::IDENTITY,
            scale: V3::ONE,
            matrix: M4::IDENTITY,
            matrix_world: M4::IDENTITY,
            transform_data: None,
            cast_shadow: false,
        });
        self.nodes.len() - 1
    }
    /// Object3D.add().
    pub(super) fn add(&mut self, parent: usize, child: usize) {
        if parent == child {
            return;
        }
        if let Some(old) = self.nodes[child].parent {
            self.nodes[old].children.retain(|&c| c != child);
        }
        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
    }
    pub(super) fn update_matrix(&mut self, i: usize) {
        let n = &mut self.nodes[i];
        n.matrix = M4::compose(n.position, n.quaternion, n.scale);
    }
    /// Object3D.applyMatrix4().
    pub(super) fn apply_matrix4(&mut self, i: usize, m: &M4) {
        self.update_matrix(i);
        let n = &mut self.nodes[i];
        n.matrix = n.matrix.premultiply(m);
        let (p, q, s) = n.matrix.decompose();
        n.position = p;
        n.quaternion = q;
        n.scale = s;
    }
    /// updateWorldMatrix( updateParents, false ).
    pub(super) fn update_world_matrix(&mut self, i: usize, parents: bool) {
        if parents && let Some(p) = self.nodes[i].parent {
            self.update_world_matrix(p, true);
        }
        self.update_matrix(i);
        let world = match self.nodes[i].parent {
            None => self.nodes[i].matrix,
            Some(p) => self.nodes[p].matrix_world.multiply(&self.nodes[i].matrix),
        };
        self.nodes[i].matrix_world = world;
    }
    /// updateMatrixWorld( true ).
    pub(super) fn update_matrix_world(&mut self, i: usize) {
        self.update_world_matrix(i, false);
        for c in self.nodes[i].children.clone() {
            self.update_matrix_world(c);
        }
    }
    /// traverse() order from a node.
    pub(super) fn traverse(&self, i: usize) -> Vec<usize> {
        let mut out = vec![i];
        for &c in &self.nodes[i].children {
            out.extend(self.traverse(c));
        }
        out
    }
    /// getObjectByName(): the node itself or the first descendant ( depth first ).
    pub(super) fn object_by_name(&self, i: usize, name: &str) -> Option<usize> {
        if self.nodes[i].name == name {
            return Some(i);
        }
        self.nodes[i]
            .children
            .iter()
            .find_map(|&c| self.object_by_name(c, name))
    }
    /// Object3D.lookAt( target ).
    fn look_at(&mut self, i: usize, target: V3) {
        let parent = self.nodes[i].parent;
        self.update_world_matrix(i, true);
        let w = self.nodes[i].matrix_world.0;
        let position = V3::new(w[12], w[13], w[14]);
        let camera_like = matches!(self.nodes[i].kind, Kind::Camera { .. } | Kind::Light(_));
        let up = V3::new(0.0, 1.0, 0.0);
        let m = if camera_like {
            M4::IDENTITY.look_at(position, target, up)
        } else {
            M4::IDENTITY.look_at(target, position, up)
        };
        let mut q = Q::from_rotation_matrix(&m);
        if let Some(p) = parent {
            let r = M4::extract_rotation(&self.nodes[p].matrix_world);
            q = Q::multiply(Q::from_rotation_matrix(&r).invert(), q);
        }
        self.nodes[i].quaternion = q;
    }
}

// ------------------------------------------------------------- deformers

#[derive(Clone, Debug)]
pub(super) struct RawBone {
    pub id: Value,
    pub indices: Vec<f64>,
    pub weights: Vec<f64>,
    pub transform_link: M4,
}
#[derive(Clone, Debug)]
pub(super) struct Skeleton {
    pub id: String,
    pub raw_bones: Vec<RawBone>,
    pub bones: Vec<Option<usize>>,
}
#[derive(Clone, Debug)]
pub(super) struct RawTarget {
    pub name: Value,
    pub geo_id: Value,
}
#[derive(Clone, Debug)]
pub(super) struct MorphTarget {
    pub raw_targets: Option<Vec<RawTarget>>,
}
#[derive(Default)]
pub(super) struct Deformers {
    /// By node id, in `for ( const ID in skeletons )` order.
    pub skeletons: Vec<(String, Skeleton)>,
    pub morph_targets: Vec<(String, MorphTarget)>,
}
impl Deformers {
    pub(super) fn skeleton(&self, id: &Value) -> Option<usize> {
        let key = id.to_key();
        self.skeletons.iter().position(|(k, _)| *k == key)
    }
    pub(super) fn morph_target(&self, id: &Value) -> Option<usize> {
        let key = id.to_key();
        self.morph_targets.iter().position(|(k, _)| *k == key)
    }
}

/// Order entries keyed by property name as a JavaScript object would.
fn js_ordered<T>(entries: Vec<(String, T)>) -> Vec<(String, T)> {
    let o = Obj::new();
    for (i, (k, _)) in entries.iter().enumerate() {
        if !o.has(k) {
            o.set(k, Value::Number(i as f64));
        }
    }
    let mut slots: Vec<Option<(String, T)>> = entries.into_iter().map(Some).collect();
    let mut out = vec![];
    for k in o.keys() {
        let i = o.get(&k).to_number() as usize;
        // A later duplicate replaces the value but keeps the first position.
        let last = slots
            .iter()
            .rposition(|s| s.as_ref().is_some_and(|(key, _)| key.as_str() == &*k))
            .unwrap_or(i);
        if let Some(entry) = slots[last].take() {
            out.push(entry);
        }
    }
    out
}

struct Parser<'a> {
    ctx: Ctx<'a>,
    options: &'a Options,
    graph: Graph,
    textures: Vec<FbxTexture>,
    materials: Vec<FbxMaterial>,
    /// textureMap and materialMap: id key → index.
    texture_map: HashMap<String, usize>,
    material_map: HashMap<String, usize>,
    pub(super) clips: Vec<FbxClip>,
    unit_scale: Option<f64>,
}

/// FBXTreeParser.parse().
pub(super) fn parse(tree: &Obj, options: &Options) -> Result<FbxScene> {
    let objects = tree
        .get("Objects")
        .as_obj()
        .cloned()
        .ok_or_else(|| bad("Cannot use 'in' operator to search in undefined Objects"))?;
    let connections = parse_connections(tree);
    let mut p = Parser {
        ctx: Ctx {
            tree,
            objects,
            connections,
        },
        options,
        graph: Graph::default(),
        textures: vec![],
        materials: vec![],
        texture_map: HashMap::new(),
        material_map: HashMap::new(),
        clips: vec![],
        unit_scale: None,
    };
    let images = p.parse_images()?;
    p.parse_textures(&images)?;
    p.parse_materials()?;
    let mut deformers = p.parse_deformers()?;
    let (mut geos, geometry_map) = geometry::parse(&p.ctx, &deformers)?;
    let root = p.parse_scene(&mut deformers, &mut geos, &geometry_map)?;
    p.finish(root, geos)
}

/// parseConnections().
fn parse_connections(tree: &Obj) -> Connections {
    let mut map: HashMap<String, Links> = HashMap::new();
    if let Some(c) = tree.get("Connections").as_obj()
        && let Value::Array(raw) = c.get("connections")
    {
        for raw in raw.borrow().iter() {
            let from = raw.get("0");
            let to = raw.get("1");
            let relationship = raw.get("2");
            map.entry(map_key(&from)).or_default().parents.push(Rel {
                id: to.clone(),
                relationship: relationship.clone(),
            });
            map.entry(map_key(&to)).or_default().children.push(Rel {
                id: from,
                relationship,
            });
        }
    }
    Connections(map)
}

/// An image reference: a file name, an embedded blob, or a data URI.
#[derive(Clone, Debug)]
enum ImageRef {
    Name(String),
    Blob { mime: String, data: Vec<u8> },
}

impl Parser<'_> {
    /// parseImages(): video id → file name or embedded image.
    fn parse_images(&self) -> Result<HashMap<String, Option<ImageRef>>> {
        let images = Obj::new();
        let mut names: Vec<(String, Value)> = vec![];
        let mut blobs: HashMap<String, Option<ImageRef>> = HashMap::new();
        if self.ctx.objects.has("Video") {
            for (node_id, video) in self.ctx.nodes("Video") {
                let id = parse_int(&node_id);
                let filename = {
                    let r = video.get("RelativeFilename");
                    if r.truthy() { r } else { video.get("Filename") }
                };
                images.set(&number_key(id), filename.clone());
                if video.has("Content") {
                    let content = video.get("Content");
                    let array = matches!(&content, Value::Bytes(b) if !b.is_empty());
                    let base64 = matches!(&content, Value::String(s) if !s.is_empty());
                    if array || base64 {
                        let image = parse_image(&video)?;
                        blobs.insert(filename.to_key(), image);
                    }
                }
            }
        }
        for key in images.keys() {
            names.push((key.to_string(), images.get(&key)));
        }
        let mut out = HashMap::new();
        for (id, filename) in names {
            let resolved = match blobs.get(&filename.to_key()) {
                Some(Some(blob)) => Some(blob.clone()),
                _ => {
                    let name = filename
                        .as_str()
                        .ok_or_else(|| bad("images[ id ].split is not a function"))?;
                    Some(ImageRef::Name(
                        name.rsplit('\\').next().unwrap_or(name).to_string(),
                    ))
                }
            };
            out.insert(id, resolved);
        }
        Ok(out)
    }

    /// parseTextures().
    fn parse_textures(&mut self, images: &HashMap<String, Option<ImageRef>>) -> Result<()> {
        for (node_id, node) in self.ctx.nodes("Texture") {
            let texture = self.parse_texture(&node, images)?;
            self.textures.push(texture);
            self.texture_map.insert(
                map_key(&Value::Number(parse_int(&node_id))),
                self.textures.len() - 1,
            );
        }
        Ok(())
    }

    fn parse_texture(
        &self,
        node: &Value,
        images: &HashMap<String, Option<ImageRef>>,
    ) -> Result<FbxTexture> {
        // loadTexture()
        if node.get("FileName").as_str().is_none() {
            return Err(bad("textureNode.FileName is undefined"));
        }
        let children = &self.ctx.connections.links(&node.get("id"))?.children;
        let mut image = FbxImage::None;
        if let Some(first) = children.first()
            && let Some(Some(file)) = images.get(&first.id.to_key())
        {
            image = match file {
                ImageRef::Name(name) if name.starts_with("blob:") || name.starts_with("data:") => {
                    FbxImage::Url(name.clone())
                }
                ImageRef::Name(name) => FbxImage::Url(format!("{}{name}", self.options.path)),
                ImageRef::Blob { mime, data } => FbxImage::Embedded {
                    mime: mime.clone(),
                    data: data.clone(),
                },
            };
        }
        let wrap_u = node.get("WrapModeU");
        let wrap_v = node.get("WrapModeV");
        let value_u = if wrap_u.is_undefined() {
            Value::Number(0.0)
        } else {
            wrap_u.get("value")
        };
        let value_v = if wrap_v.is_undefined() {
            Value::Number(0.0)
        } else {
            wrap_v.get("value")
        };
        let mut repeat = Vector2::ONE;
        let mut offset = Vector2::ZERO;
        let element = |v: &Value, i: usize| -> f64 {
            match v {
                Value::String(s) => s.encode_utf16().nth(i).map_or(f64::NAN, |u| {
                    super::js::string_to_number(&String::from_utf16_lossy(&[u]))
                }),
                v => v.get(&i.to_string()).to_number(),
            }
        };
        if node.has("Scaling") {
            let v = prop_value(node, "Scaling");
            repeat = Vector2::new(element(&v, 0), element(&v, 1));
        }
        if node.has("Translation") {
            let v = prop_value(node, "Translation");
            offset = Vector2::new(element(&v, 0), element(&v, 1));
        }
        let attr = node.get("attrName");
        Ok(FbxTexture {
            name: (!attr.is_undefined()).then(|| attr.to_key()),
            image,
            repeat_s: value_u.strict_eq(&Value::Number(0.0)),
            repeat_t: value_v.strict_eq(&Value::Number(0.0)),
            repeat,
            offset,
            srgb: false,
            equirectangular: false,
        })
    }

    /// parseMaterials().
    fn parse_materials(&mut self) -> Result<()> {
        for (node_id, node) in self.ctx.nodes("Material") {
            if let Some(m) = self.parse_material(&node)? {
                self.materials.push(m);
                self.material_map.insert(
                    map_key(&Value::Number(parse_int(&node_id))),
                    self.materials.len() - 1,
                );
            }
        }
        Ok(())
    }

    fn get_texture(&self, id: &Value) -> Result<Option<usize>> {
        let mut id = id.clone();
        if let Value::Object(layered) = self.ctx.objects("LayeredTexture")
            && layered.has(&id.to_key())
        {
            id = self
                .ctx
                .connections
                .links(&id)?
                .children
                .first()
                .map(|c| c.id.clone())
                .ok_or_else(|| bad("layered texture without layers"))?;
        }
        Ok(self.texture_map.get(&map_key(&id)).copied())
    }

    fn parse_material(&mut self, node: &Value) -> Result<Option<FbxMaterial>> {
        let id = node.get("id");
        let name = node.get("attrName");
        let mut shading = node.get("ShadingModel");
        if matches!(shading, Value::Object(_)) {
            shading = shading.get("value");
        }
        if !self.ctx.connections.has(&id) {
            return Ok(None);
        }
        let shading = shading
            .as_str()
            .ok_or_else(|| bad("type.toLowerCase is not a function"))?
            .to_lowercase();
        let kind = match shading.as_str() {
            "lambert" => FbxMaterialKind::Lambert,
            _ => FbxMaterialKind::Phong,
        };
        let mut m = default_material(kind);
        m.name = (!name.is_undefined()).then(|| name.to_key());
        // parseParameters(), applied by setValues() in the parameters' order.
        let lambert = kind == FbxMaterialKind::Lambert;
        if node.get("BumpFactor").truthy() {
            m.bump_scale = prop_value(node, "BumpFactor").to_number();
        }
        let color_prop = |name: &str| -> Option<[f64; 3]> {
            Some(srgb_color(&numbers(&prop_value(node, name))))
        };
        let typed = |name: &str, types: &[&str]| {
            let p = node.get(name);
            p.truthy()
                && types
                    .iter()
                    .any(|t| p.get("type").strict_eq(&Value::string(t)))
        };
        if node.get("Diffuse").truthy() {
            m.color = color(color_prop("Diffuse").unwrap_or_default());
        } else if typed("DiffuseColor", &["Color", "ColorRGB"]) {
            m.color = color(color_prop("DiffuseColor").unwrap_or_default());
        }
        if node.get("DisplacementFactor").truthy() {
            m.displacement_scale = prop_value(node, "DisplacementFactor").to_number();
        }
        if node.get("Emissive").truthy() {
            m.emissive = color(color_prop("Emissive").unwrap_or_default());
        } else if typed("EmissiveColor", &["Color", "ColorRGB"]) {
            m.emissive = color(color_prop("EmissiveColor").unwrap_or_default());
        }
        if node.get("EmissiveFactor").truthy() {
            m.emissive_intensity = parse_float_value(&prop_value(node, "EmissiveFactor"));
        }
        let mut opacity = 1.0
            - if node.get("TransparencyFactor").truthy() {
                parse_float_value(&prop_value(node, "TransparencyFactor"))
            } else {
                0.0
            };
        if opacity == 1.0 || opacity == 0.0 {
            opacity = if node.get("Opacity").truthy() {
                parse_float_value(&prop_value(node, "Opacity"))
            } else {
                1.0
            };
        }
        m.opacity = opacity;
        if opacity < 1.0 {
            m.transparent = true;
        }
        if node.get("ReflectionFactor").truthy() {
            m.reflectivity = prop_value(node, "ReflectionFactor").to_number();
        }
        if node.get("Shininess").truthy() && !lambert {
            m.shininess = prop_value(node, "Shininess").to_number();
        }
        if node.get("Specular").truthy() {
            if !lambert {
                m.specular = color(color_prop("Specular").unwrap_or_default());
            }
        } else if typed("SpecularColor", &["Color"]) && !lambert {
            m.specular = color(color_prop("SpecularColor").unwrap_or_default());
        }
        let children = self.ctx.connections.links(&id)?.children.clone();
        for child in children {
            let relationship = child.relationship.as_str().unwrap_or("");
            match relationship {
                "Bump" => m.bump_map = self.get_texture(&child.id)?,
                "Maya|TEX_ao_map" => m.ao_map = self.get_texture(&child.id)?,
                "DiffuseColor" | "Maya|TEX_color_map" => {
                    m.map = self.get_texture(&child.id)?;
                    if let Some(t) = m.map {
                        self.textures[t].srgb = true;
                    }
                }
                "DisplacementColor" => m.displacement_map = self.get_texture(&child.id)?,
                "EmissiveColor" => {
                    m.emissive_map = self.get_texture(&child.id)?;
                    if let Some(t) = m.emissive_map {
                        self.textures[t].srgb = true;
                    }
                }
                "NormalMap" | "Maya|TEX_normal_map" => {
                    m.normal_map = self.get_texture(&child.id)?;
                }
                "ReflectionColor" => {
                    m.env_map = self.get_texture(&child.id)?;
                    if let Some(t) = m.env_map {
                        self.textures[t].equirectangular = true;
                        self.textures[t].srgb = true;
                    }
                }
                "SpecularColor" => {
                    m.specular_map = self.get_texture(&child.id)?;
                    if let Some(t) = m.specular_map {
                        self.textures[t].srgb = true;
                    }
                }
                "TransparentColor" | "TransparencyFactor" => {
                    m.alpha_map = self.get_texture(&child.id)?;
                    m.transparent = true;
                }
                _ => {}
            }
        }
        Ok(Some(m))
    }

    /// parseDeformers().
    fn parse_deformers(&self) -> Result<Deformers> {
        let mut d = Deformers::default();
        let deformer_nodes = self.ctx.objects("Deformer");
        for (node_id, node) in self.ctx.nodes("Deformer") {
            let relationships = self
                .ctx
                .connections
                .get(&Value::Number(parse_int(&node_id)));
            let attr_type = node.get("attrType");
            if attr_type.strict_eq(&Value::string("Skin")) {
                let relationships = relationships.ok_or_else(|| bad("skin without connections"))?;
                let mut raw_bones = vec![];
                for child in &relationships.children {
                    let bone = deformer_nodes.get(&child.id.to_key());
                    if bone.is_undefined() {
                        return Err(bad("missing cluster deformer"));
                    }
                    if !bone.get("attrType").strict_eq(&Value::string("Cluster")) {
                        continue;
                    }
                    let link = bone.get("TransformLink").get("a");
                    let link = link.to_f64s().ok_or_else(|| bad("missing TransformLink"))?;
                    let (indices, weights) = if bone.has("Indexes") {
                        (
                            numbers(&bone.get("Indexes").get("a")),
                            numbers(&bone.get("Weights").get("a")),
                        )
                    } else {
                        (vec![], vec![])
                    };
                    raw_bones.push(RawBone {
                        id: child.id.clone(),
                        indices,
                        weights,
                        transform_link: M4::from_array(&link),
                    });
                }
                if relationships.parents.is_empty() {
                    return Err(bad("skeleton without a geometry"));
                }
                d.skeletons.push((
                    node_id.clone(),
                    Skeleton {
                        id: node_id,
                        raw_bones,
                        bones: vec![],
                    },
                ));
            } else if attr_type.strict_eq(&Value::string("BlendShape")) {
                let relationships =
                    relationships.ok_or_else(|| bad("blend shape without connections"))?;
                let mut raw = Some(vec![]);
                for child in &relationships.children {
                    let target = deformer_nodes.get(&child.id.to_key());
                    if target.is_undefined() {
                        return Err(bad("missing blend shape channel"));
                    }
                    if target.get("FullWeights").is_undefined() {
                        return Err(bad("missing FullWeights"));
                    }
                    if !target
                        .get("attrType")
                        .strict_eq(&Value::string("BlendShapeChannel"))
                    {
                        raw = None;
                        break;
                    }
                    let geo_id = self
                        .ctx
                        .connections
                        .links(&Value::Number(parse_int_value(&child.id)))?
                        .children
                        .iter()
                        .find(|c| c.relationship.is_undefined())
                        .map(|c| c.id.clone())
                        .ok_or_else(|| bad("blend shape channel without a shape"))?;
                    if let Some(r) = &mut raw {
                        r.push(RawTarget {
                            name: target.get("attrName"),
                            geo_id,
                        });
                    }
                }
                d.morph_targets
                    .push((node_id, MorphTarget { raw_targets: raw }));
            }
        }
        d.skeletons = js_ordered(d.skeletons);
        d.morph_targets = js_ordered(d.morph_targets);
        Ok(d)
    }

    /// parseScene(): the returned group's node index.
    fn parse_scene(
        &mut self,
        deformers: &mut Deformers,
        geos: &mut [Geo],
        geometry_map: &HashMap<String, Option<usize>>,
    ) -> Result<usize> {
        let mut root = self.graph.insert(Kind::Group);
        let model_map = self.parse_models(deformers, geos, geometry_map)?;
        let model_nodes = self.ctx.objects("Model");
        for &(_, model) in &model_map.order {
            let id = Value::Number(self.graph.nodes[model].id.unwrap_or(f64::NAN));
            let node = model_nodes.get(&id.to_key());
            self.set_look_at(root, model, &node)?;
            let parents = self.ctx.connections.links(&id)?.parents.clone();
            for c in parents {
                if let Some(parent) = model_map.get(&c.id) {
                    self.graph.add(parent, model);
                }
            }
            if self.graph.nodes[model].parent.is_none() {
                self.graph.add(root, model);
            }
        }
        let mut unit_scale = None;
        // addGlobalSceneSettings()
        if let Some(settings) = self.ctx.tree.get("GlobalSettings").as_obj() {
            if settings.has("AmbientColor") {
                let c = numbers(&settings.get("AmbientColor").get("value"));
                let g = |i: usize| c.get(i).copied().unwrap_or(f64::NAN);
                if g(0) != 0.0 || g(1) != 0.0 || g(2) != 0.0 {
                    let light = self.graph.insert(Kind::Ambient {
                        color: srgb_color(&c),
                        intensity: 1.0,
                    });
                    self.graph.add(root, light);
                }
            }
            if settings.has("UnitScaleFactor") {
                unit_scale = Some(settings.get("UnitScaleFactor").get("value").to_number());
            }
        }
        for i in self.graph.traverse(root) {
            if self.graph.nodes[i].transform_data.is_some() {
                if let Some(p) = self.graph.nodes[i].parent {
                    let (m, w) = (self.graph.nodes[p].matrix, self.graph.nodes[p].matrix_world);
                    let t = self.graph.nodes[i].transform_data.as_mut().unwrap();
                    t.parent_matrix = Some(m);
                    t.parent_matrix_world = Some(w);
                }
                let transform =
                    generate_transform(self.graph.nodes[i].transform_data.as_ref().unwrap());
                self.graph.apply_matrix4(i, &transform);
                self.graph.update_world_matrix(i, false);
            }
        }
        // The BindPose rest pose for bones outside skin clusters.
        let bind_poses = self.parse_pose_nodes()?;
        let mut cluster_bones = vec![];
        for (_, s) in &deformers.skeletons {
            for i in 0..s.raw_bones.len() {
                if let Some(Some(b)) = s.bones.get(i) {
                    cluster_bones.push(self.graph.nodes[*b].id);
                }
            }
        }
        for i in self.graph.traverse(root) {
            let n = &self.graph.nodes[i];
            if !matches!(n.kind, Kind::Bone) || n.id.is_none() || cluster_bones.contains(&n.id) {
                continue;
            }
            let Some(bind) = bind_poses.get(&number_key(n.id.unwrap_or(f64::NAN))) else {
                continue;
            };
            let m = match n.parent {
                Some(p) => self.graph.nodes[p].matrix_world.invert().multiply(bind),
                None => *bind,
            };
            let (pos, q, s) = m.decompose();
            let n = &mut self.graph.nodes[i];
            n.position = pos;
            n.quaternion = q;
            n.scale = s;
            self.graph.update_matrix(i);
            self.graph.nodes[i].matrix_world = *bind;
        }
        self.bind_skeleton(deformers, geometry_map, &model_map)?;
        let clips = super::animation::parse(
            &self.ctx,
            &self.graph,
            root,
            self.options.trim_animation_clips,
        )?;
        self.clips = clips;
        if self.graph.nodes[root].children.len() == 1
            && matches!(
                self.graph.nodes[self.graph.nodes[root].children[0]].kind,
                Kind::Group
            )
        {
            root = self.graph.nodes[root].children[0];
        }
        self.unit_scale = unit_scale;
        if let Some(settings) = self.ctx.tree.get("GlobalSettings").as_obj()
            && settings.has("UpAxis")
            && settings
                .get("UpAxis")
                .get("value")
                .strict_eq(&Value::Number(2.0))
        {
            self.graph.nodes[root].quaternion = Q::from_euler(Euler::new(
                -std::f64::consts::PI / 2.0,
                0.0,
                0.0,
                Order::Xyz,
            ));
        }
        Ok(root)
    }
}

/// modelMap: a Map from model id to node, in insertion order.
#[derive(Default)]
pub(super) struct ModelMap {
    pub order: Vec<(String, usize)>,
    index: HashMap<String, usize>,
}
impl ModelMap {
    fn set(&mut self, id: &Value, node: usize) {
        let key = map_key(id);
        if let Some(&i) = self.index.get(&key) {
            self.order[i].1 = node;
        } else {
            self.index.insert(key.clone(), self.order.len());
            self.order.push((key, node));
        }
    }
    pub(super) fn get(&self, id: &Value) -> Option<usize> {
        self.index.get(&map_key(id)).map(|&i| self.order[i].1)
    }
}

fn color(c: [f64; 3]) -> Color {
    Color::linear(c[0], c[1], c[2])
}

/// A new MeshPhongMaterial / MeshLambertMaterial / LineBasicMaterial.
fn default_material(kind: FbxMaterialKind) -> FbxMaterial {
    FbxMaterial {
        kind,
        name: Some(String::new()),
        color: Color::linear(1.0, 1.0, 1.0),
        specular: color(hex_color(0x111111)),
        shininess: 30.0,
        emissive: Color::linear(0.0, 0.0, 0.0),
        emissive_intensity: 1.0,
        opacity: 1.0,
        transparent: false,
        vertex_colors: false,
        bump_scale: 1.0,
        displacement_scale: 1.0,
        reflectivity: 1.0,
        map: None,
        alpha_map: None,
        ao_map: None,
        bump_map: None,
        displacement_map: None,
        emissive_map: None,
        env_map: None,
        normal_map: None,
        specular_map: None,
    }
}

/// parseImage(): an embedded image as a data URI ( ASCII ) or a blob ( binary ).
fn parse_image(video: &Value) -> Result<Option<ImageRef>> {
    let content = video.get("Content");
    let file = {
        let r = video.get("RelativeFilename");
        if r.truthy() { r } else { video.get("Filename") }
    };
    let file = file
        .as_str()
        .ok_or_else(|| bad("fileName.slice is not a function"))?;
    let extension = match file.rfind('.') {
        Some(i) => &file[i + 1..],
        None => file,
    }
    .to_lowercase();
    let mime = match extension.as_str() {
        "bmp" => "image/bmp",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "tif" => "image/tiff",
        "tga" => "image/tga",
        "webp" => "image/webp",
        _ => return Ok(None),
    };
    Ok(Some(match content {
        Value::String(s) => ImageRef::Name(format!("data:{mime};base64,{s}")),
        Value::Bytes(b) => ImageRef::Blob {
            mime: mime.to_string(),
            data: b.to_vec(),
        },
        _ => return Ok(None),
    }))
}

impl Parser<'_> {
    /// parseModels().
    fn parse_models(
        &mut self,
        deformers: &mut Deformers,
        geos: &mut [Geo],
        geometry_map: &HashMap<String, Option<usize>>,
    ) -> Result<ModelMap> {
        let mut map = ModelMap::default();
        for (node_id, node) in self.ctx.nodes("Model") {
            let id = Value::Number(parse_int(&node_id));
            let relationships = self.ctx.connections.links(&id)?.clone();
            let attr_name = node.get("attrName");
            let mut model = self.build_skeleton(&relationships, deformers, &id, &attr_name);
            if model.is_none() {
                let attr_type = node.get("attrType");
                let m = match attr_type.as_str() {
                    Some("Camera") => self.create_camera(&relationships),
                    Some("Light") => self.create_light(&relationships),
                    Some("Mesh") => {
                        self.create_mesh(&relationships, geos, geometry_map, deformers)?
                    }
                    Some("NurbsCurve") => self.create_curve(&relationships, geometry_map),
                    Some("LimbNode" | "Root") => self.graph.insert(Kind::Bone),
                    _ => self.graph.insert(Kind::Group),
                };
                self.graph.nodes[m].name = model_name(&attr_name);
                self.graph.nodes[m].id = Some(id.to_number());
                model = Some(m);
            }
            let model = model.unwrap_or_default();
            self.get_transform_data(model, &node);
            map.set(&id, model);
        }
        Ok(map)
    }

    fn build_skeleton(
        &mut self,
        relationships: &super::scene::Links,
        deformers: &mut Deformers,
        id: &Value,
        name: &Value,
    ) -> Option<usize> {
        let mut bone: Option<usize> = None;
        for parent in &relationships.parents {
            for s in 0..deformers.skeletons.len() {
                for i in 0..deformers.skeletons[s].1.raw_bones.len() {
                    let raw = &deformers.skeletons[s].1.raw_bones[i];
                    if !raw.id.strict_eq(&parent.id) {
                        continue;
                    }
                    let link = raw.transform_link;
                    let sub = bone;
                    let b = self.graph.insert(Kind::Bone);
                    self.graph.nodes[b].matrix_world = link;
                    self.graph.nodes[b].name = model_name(name);
                    self.graph.nodes[b].id = Some(id.to_number());
                    let bones = &mut deformers.skeletons[s].1.bones;
                    if bones.len() <= i {
                        bones.resize(i + 1, None);
                    }
                    bones[i] = Some(b);
                    if let Some(sub) = sub {
                        self.graph.add(b, sub);
                    }
                    bone = Some(b);
                }
            }
        }
        bone
    }

    fn node_attribute(&self, relationships: &super::scene::Links) -> Value {
        let attributes = self.ctx.objects("NodeAttribute");
        let mut found = Value::Undefined;
        for child in &relationships.children {
            let a = attributes.get(&child.id.to_key());
            if !a.is_undefined() {
                found = a;
            }
        }
        found
    }

    fn create_camera(&mut self, relationships: &super::scene::Links) -> usize {
        let a = self.node_attribute(relationships);
        if a.is_undefined() {
            return self.graph.insert(Kind::Object3D);
        }
        let projection = a.get("CameraProjectionType");
        if !projection.is_undefined() && projection.get("value").strict_eq(&Value::Number(1.0)) {
            return self.graph.insert(Kind::Object3D);
        }
        let value = |name: &str| a.get(name).get("value").to_number();
        let near = if a.get("NearPlane").is_undefined() {
            1.0
        } else {
            value("NearPlane") / 1000.0
        };
        let far = if a.get("FarPlane").is_undefined() {
            1000.0
        } else {
            value("FarPlane") / 1000.0
        };
        let (mut width, mut height) = self.options.window_size;
        if !a.get("AspectWidth").is_undefined() && !a.get("AspectHeight").is_undefined() {
            width = value("AspectWidth");
            height = value("AspectHeight");
        }
        let aspect = width / height;
        let mut fov = if a.get("FieldOfView").is_undefined() {
            45.0
        } else {
            value("FieldOfView")
        };
        if a.get("FocalLength").truthy() {
            // setFocalLength( focalLength ) with the 35 mm film gauge.
            let focal = value("FocalLength");
            let film_height = 35.0 / aspect.max(1.0);
            let slope = 0.5 * film_height / focal;
            fov = (180.0 / std::f64::consts::PI) * 2.0 * super::math::js_math::atan(slope);
        }
        self.graph.insert(Kind::Camera {
            fov,
            aspect,
            near,
            far,
        })
    }

    fn create_light(&mut self, relationships: &super::scene::Links) -> usize {
        let a = self.node_attribute(relationships);
        if a.is_undefined() {
            return self.graph.insert(Kind::Object3D);
        }
        let value = |name: &str| a.get(name).get("value");
        let kind = if a.get("LightType").is_undefined() {
            0.0
        } else {
            value("LightType").to_number()
        };
        let color = if a.get("Color").is_undefined() {
            [1.0, 1.0, 1.0]
        } else {
            srgb_color(&numbers(&value("Color")))
        };
        let mut intensity = if a.get("Intensity").is_undefined() {
            1.0
        } else {
            value("Intensity").to_number() / 100.0
        };
        if !a.get("CastLightOnObject").is_undefined()
            && value("CastLightOnObject").strict_eq(&Value::Number(0.0))
        {
            intensity = 0.0;
        }
        let mut distance = 0.0;
        if !a.get("FarAttenuationEnd").is_undefined() {
            distance = if !a.get("EnableFarAttenuation").is_undefined()
                && value("EnableFarAttenuation").strict_eq(&Value::Number(0.0))
            {
                0.0
            } else {
                value("FarAttenuationEnd").to_number()
            };
        }
        let mut light = Light {
            kind: LightKind::Point,
            color,
            intensity,
            distance,
            decay: 1.0,
            angle: std::f64::consts::PI / 3.0,
            penumbra: 0.0,
            target: None,
        };
        if kind == 0.0 {
        } else if kind == 1.0 {
            light.kind = LightKind::Directional;
        } else if kind == 2.0 {
            light.kind = LightKind::Spot;
            if !a.get("OuterAngle").is_undefined() {
                let outer = value("OuterAngle").to_number();
                light.angle = deg_to_rad(outer);
                if !a.get("InnerAngle").is_undefined() {
                    let inner = value("InnerAngle").to_number();
                    light.penumbra = (1.0 - inner / outer).max(0.0);
                }
            } else if !a.get("InnerAngle").is_undefined() {
                light.angle = deg_to_rad(value("InnerAngle").to_number());
            }
        } else {
            // PointLight( color, intensity ): the default distance and decay.
            light.distance = 0.0;
            light.decay = 2.0;
        }
        if light.kind != LightKind::Point {
            light.target = Some(self.graph.insert(Kind::Object3D));
        }
        let cast = !a.get("CastShadows").is_undefined()
            && value("CastShadows").strict_eq(&Value::Number(1.0));
        let i = self.graph.insert(Kind::Light(light));
        self.graph.nodes[i].cast_shadow = cast;
        i
    }

    fn create_mesh(
        &mut self,
        relationships: &super::scene::Links,
        geos: &mut [Geo],
        geometry_map: &HashMap<String, Option<usize>>,
        deformers: &Deformers,
    ) -> Result<usize> {
        let mut geometry: Option<Option<usize>> = None;
        let mut materials = vec![];
        for child in &relationships.children {
            if let Some(g) = geometry_map.get(&map_key(&child.id)) {
                geometry = Some(*g);
            }
            if let Some(&m) = self.material_map.get(&map_key(&child.id)) {
                materials.push(m);
            }
        }
        let multi = materials.len() > 1;
        if materials.is_empty() {
            let mut m = default_material(FbxMaterialKind::Phong);
            m.name = Some("__DEFAULT".into());
            m.color = color(hex_color(0xcccccc));
            self.materials.push(m);
            materials.push(self.materials.len() - 1);
        }
        let g = geometry
            .flatten()
            .ok_or_else(|| bad("Cannot use 'in' operator to search for 'color' in undefined"))?;
        let geo = &mut geos[g];
        if geo.has_attribute("color") {
            for &m in &materials {
                self.materials[m].vertex_colors = true;
            }
        }
        if !geo.groups.is_empty() {
            let mut needs_default = false;
            for group in &mut geo.groups {
                let n = materials.len() as f64;
                if let Some(index) = group.2
                    && (index < 0.0 || index >= n)
                {
                    group.2 = Some(n);
                    needs_default = true;
                }
            }
            if needs_default {
                self.materials
                    .push(default_material(FbxMaterialKind::Phong));
                materials.push(self.materials.len() - 1);
            }
        }
        let mesh_materials = if multi { materials } else { vec![materials[0]] };
        let skinned = geo.deformer.is_some();
        if skinned {
            geo.normalize_skin_weights();
        }
        let _ = deformers;
        let morph_targets = geo.morph_dictionary();
        Ok(self.graph.insert(Kind::Mesh {
            geometry: g,
            materials: mesh_materials,
            multi,
            skinned,
            skin: None,
            morph_targets,
        }))
    }

    fn create_curve(
        &mut self,
        relationships: &super::scene::Links,
        geometry_map: &HashMap<String, Option<usize>>,
    ) -> usize {
        let mut geometry = None;
        for child in &relationships.children {
            if let Some(g) = geometry_map.get(&map_key(&child.id)) {
                geometry = *g;
            }
        }
        let mut m = default_material(FbxMaterialKind::LineBasic);
        m.name = Some("__DEFAULT".into());
        m.color = color(hex_color(0x3300ff));
        self.materials.push(m);
        let material = self.materials.len() - 1;
        self.graph.insert(Kind::Line { geometry, material })
    }

    /// getTransformData().
    fn get_transform_data(&mut self, model: usize, node: &Value) {
        let mut t = TransformData::default();
        if node.has("InheritType") {
            t.inherit_type = Some(parse_int_value(&prop_value(node, "InheritType")));
        }
        t.euler_order = if node.has("RotationOrder") {
            euler_order(&prop_value(node, "RotationOrder"))
        } else {
            euler_order(&Value::Number(0.0))
        };
        t.translation = transform_prop(node, "Lcl_Translation");
        t.pre_rotation = transform_prop(node, "PreRotation");
        t.rotation = transform_prop(node, "Lcl_Rotation");
        t.post_rotation = transform_prop(node, "PostRotation");
        t.scale = transform_prop(node, "Lcl_Scaling");
        t.scaling_offset = transform_prop(node, "ScalingOffset");
        t.scaling_pivot = transform_prop(node, "ScalingPivot");
        t.rotation_offset = transform_prop(node, "RotationOffset");
        t.rotation_pivot = transform_prop(node, "RotationPivot");
        self.graph.nodes[model].transform_data = Some(t);
    }

    /// setLookAtProperties().
    fn set_look_at(&mut self, root: usize, model: usize, node: &Value) -> Result<()> {
        if !node.has("LookAtProperty") {
            return Ok(());
        }
        let id = Value::Number(self.graph.nodes[model].id.unwrap_or(f64::NAN));
        let children = self.ctx.connections.links(&id)?.children.clone();
        let models = self.ctx.objects("Model");
        for child in children {
            if !child
                .relationship
                .strict_eq(&Value::string("LookAtProperty"))
            {
                continue;
            }
            let target = models.get(&child.id.to_key());
            if !target.has("Lcl_Translation") {
                continue;
            }
            let pos = V3::from_array(&numbers(&prop_value(&target, "Lcl_Translation")));
            let light_target = match &self.graph.nodes[model].kind {
                Kind::Light(l) => l.target,
                _ => None,
            };
            if let Some(t) = light_target {
                self.graph.nodes[t].position = pos;
                self.graph.add(root, t);
            } else {
                self.graph.look_at(model, pos);
            }
        }
        Ok(())
    }

    /// bindSkeleton().
    fn bind_skeleton(
        &mut self,
        deformers: &Deformers,
        geometry_map: &HashMap<String, Option<usize>>,
        model_map: &ModelMap,
    ) -> Result<()> {
        for (_, skeleton) in &deformers.skeletons {
            let inverses: Vec<M4> = (0..skeleton.bones.len())
                .map(|i| match (skeleton.bones[i], skeleton.raw_bones.get(i)) {
                    (Some(_), Some(raw)) => raw.transform_link.invert(),
                    _ => M4::IDENTITY,
                })
                .collect();
            let id = Value::Number(parse_int(&skeleton.id));
            let parents = self.ctx.connections.links(&id)?.parents.clone();
            for parent in parents {
                if !geometry_map.contains_key(&map_key(&parent.id)) {
                    continue;
                }
                let geo_parents = self.ctx.connections.links(&parent.id)?.parents.clone();
                for p in geo_parents {
                    let Some(model) = model_map.get(&p.id) else {
                        continue;
                    };
                    self.graph.update_matrix_world(model);
                    let bind = self.graph.nodes[model].matrix_world;
                    match &mut self.graph.nodes[model].kind {
                        Kind::Mesh {
                            skinned: true,
                            skin,
                            ..
                        } => *skin = Some((skeleton.bones.clone(), inverses.clone(), bind)),
                        _ => return Err(bad("model.bind is not a function")),
                    }
                }
            }
        }
        Ok(())
    }

    /// parsePoseNodes(): node id → BindPose matrix.
    fn parse_pose_nodes(&self) -> Result<HashMap<String, M4>> {
        let mut out = HashMap::new();
        for (_, pose) in self.ctx.nodes("Pose") {
            if !pose.get("attrType").strict_eq(&Value::string("BindPose"))
                || pose.get("NbPoseNodes").to_number().partial_cmp(&0.0)
                    != Some(std::cmp::Ordering::Greater)
            {
                continue;
            }
            let pose_nodes = pose.get("PoseNode");
            let list = match &pose_nodes {
                Value::Array(a) => a.borrow().clone(),
                other => vec![other.clone()],
            };
            for p in list {
                let m = p
                    .get("Matrix")
                    .get("a")
                    .to_f64s()
                    .ok_or_else(|| bad("missing pose matrix"))?;
                out.insert(p.get("Node").to_key(), M4::from_array(&m));
            }
        }
        Ok(out)
    }

    /// The returned object as plain data.
    fn finish(self, root: usize, geos: Vec<Geo>) -> Result<FbxScene> {
        let order = self.graph.traverse(root);
        let mut index = HashMap::new();
        for (i, &n) in order.iter().enumerate() {
            index.insert(n, i);
        }
        let v3 = |v: V3| Vector3::new(v.x, v.y, v.z);
        let mut nodes = vec![];
        for &n in &order {
            let node = &self.graph.nodes[n];
            let kind = match &node.kind {
                Kind::Group => FbxNodeKind::Group,
                Kind::Object3D => FbxNodeKind::Object3D,
                Kind::Bone => FbxNodeKind::Bone,
                Kind::Mesh {
                    geometry,
                    materials,
                    multi,
                    skin,
                    morph_targets,
                    ..
                } => FbxNodeKind::Mesh(FbxMesh {
                    geometry: *geometry,
                    materials: materials.clone(),
                    multi_material: *multi,
                    morph_targets: morph_targets.clone(),
                    skin: skin.as_ref().map(|(bones, inverses, bind)| FbxSkin {
                        bones: bones
                            .iter()
                            .map(|b| b.and_then(|b| index.get(&b).copied()))
                            .collect(),
                        bone_inverses: inverses
                            .iter()
                            .map(|m| Matrix4::from_cols_array(&m.0))
                            .collect(),
                        bind_matrix: Matrix4::from_cols_array(&bind.0),
                    }),
                }),
                Kind::Line { geometry, material } => FbxNodeKind::Line {
                    geometry: *geometry,
                    material: *material,
                },
                Kind::Camera {
                    fov,
                    aspect,
                    near,
                    far,
                } => FbxNodeKind::PerspectiveCamera {
                    fov: *fov,
                    aspect: *aspect,
                    near: *near,
                    far: *far,
                },
                Kind::Light(l) => {
                    let target = l.target.and_then(|t| index.get(&t).copied());
                    let c = color(l.color);
                    match l.kind {
                        LightKind::Point => FbxNodeKind::PointLight {
                            color: c,
                            intensity: l.intensity,
                            distance: l.distance,
                            decay: l.decay,
                        },
                        LightKind::Directional => FbxNodeKind::DirectionalLight {
                            color: c,
                            intensity: l.intensity,
                            target,
                        },
                        LightKind::Spot => FbxNodeKind::SpotLight {
                            color: c,
                            intensity: l.intensity,
                            distance: l.distance,
                            angle: l.angle,
                            penumbra: l.penumbra,
                            decay: l.decay,
                            target,
                        },
                    }
                }
                Kind::Ambient {
                    color: c,
                    intensity,
                } => FbxNodeKind::AmbientLight {
                    color: color(*c),
                    intensity: *intensity,
                },
            };
            nodes.push(FbxNode {
                name: node.name.clone(),
                id: node.id,
                parent: if n == root {
                    None
                } else {
                    node.parent.and_then(|p| index.get(&p).copied())
                },
                kind,
                position: v3(node.position),
                quaternion: Quaternion::from_xyzw(
                    node.quaternion.x,
                    node.quaternion.y,
                    node.quaternion.z,
                    node.quaternion.w,
                ),
                scale: v3(node.scale),
                matrix_world: Matrix4::from_cols_array(&node.matrix_world.0),
                cast_shadow: node.cast_shadow,
            });
        }
        let geometries = geos
            .into_iter()
            .map(Geo::into_geometry)
            .collect::<Result<Vec<FbxGeometry>>>()?;
        Ok(FbxScene {
            nodes,
            geometries,
            materials: self.materials,
            textures: self.textures,
            clips: self.clips,
            unit_scale_factor: self.unit_scale,
        })
    }
}
