//! VRMLLoader's parseTree(): the node builders, in the original's order of
//! object and material creation and with its JavaScript value coercions
//! ( a missing value is `undefined`, NaN once used arithmetically ).
use super::parse::{Field, Node, NodeRef, Tree, Value};
use super::{
    Options, VrmlGeometry, VrmlImage, VrmlMaterial, VrmlMaterialKind, VrmlNode, VrmlNodeKind,
    VrmlScene, VrmlTexture,
};
use crate::attribute::BufferAttribute;
use crate::fbx::js::{js_number_string, parse_int};
use crate::fbx::math::{M4, Q, V3, srgb_to_linear};
use crate::geometry::{Attribute, BoxGeometry, BufferGeometry, CylinderGeometry, SphereGeometry};
use crate::js_math::{acos, cos, sin};
use crate::{Error, Result};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::Rc;

const NAN: f64 = f64::NAN;
const DEFAULT_MATERIAL_NAME: &str = "__DEFAULT";
const REPEAT: u32 = 1000;
const CLAMP: u32 = 1001;
const NEAREST: u32 = 1003;
const LINEAR: u32 = 1006;
const LINEAR_MIPMAP_LINEAR: u32 = 1008;

fn type_error(what: &str) -> Error {
    Error::Asset(format!("THREE.VRMLLoader: {what}"))
}

/// The value as a number ( `undefined` → NaN ).
fn num(v: Option<&Value>) -> f64 {
    v.map_or(NAN, Value::number)
}
/// A constructor argument with a default ( `x = d` applies to `undefined` ).
fn num_or(v: Option<&Value>, d: f64) -> f64 {
    v.map_or(d, Value::number)
}
fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Number(n)) => *n != 0.0 && !n.is_nan(),
        Some(Value::String(s) | Value::Hex(s)) => !s.is_empty(),
        Some(Value::Bool(b)) => *b,
        Some(Value::Node(_) | Value::Use(_)) => true,
    }
}
fn is_bool(v: Option<&Value>, b: bool) -> bool {
    matches!(v, Some(Value::Bool(x)) if *x == b)
}
/// `a === b` on field values.
fn strict_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) | (Some(Value::Null), Some(Value::Null)) => true,
        (Some(Value::Number(a)), Some(Value::Number(b))) => a == b,
        (Some(Value::String(a)), Some(Value::String(b)))
        | (Some(Value::Hex(a)), Some(Value::Hex(b))) => a == b,
        (Some(Value::Bool(a)), Some(Value::Bool(b))) => a == b,
        (Some(Value::Node(a)), Some(Value::Node(b))) => Rc::ptr_eq(a, b),
        _ => false,
    }
}
/// `String( value )`.
fn string(v: &Value) -> String {
    match v {
        Value::String(s) | Value::Hex(s) => s.clone(),
        Value::Number(n) => js_number_string(*n),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".into(),
        Value::Node(_) | Value::Use(_) => "[object Object]".into(),
    }
}
fn numbers(values: &[Value]) -> Vec<f64> {
    values.iter().map(Value::number).collect()
}
/// `array[ i ]` used as a number.
fn at(d: &[f64], i: f64) -> f64 {
    if i >= 0.0 && i.fract() == 0.0 && i < d.len() as f64 {
        d[i as usize]
    } else {
        NAN
    }
}
/// Vector3.set( x, y, z ): an undefined z keeps the current one.
fn set3(v: &mut V3, values: &[Value]) {
    let z = values.get(2).map_or(v.z, Value::number);
    *v = V3::new(num(values.first()), num(values.get(1)), z);
}
fn srgb(r: f64, g: f64, b: f64) -> [f64; 3] {
    [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b)]
}
/// Color.setHex( hex ) with the sRGB conversion.
fn hex_color(hex: u32) -> [f64; 3] {
    let c = |s: u32| f64::from((hex >> s) & 255) / 255.0;
    srgb(c(16), c(8), c(0))
}
/// Quaternion.setFromAxisAngle( axis, angle ).
fn axis_angle(axis: V3, angle: f64) -> Q {
    let half = angle / 2.0;
    let s = sin(half);
    Q {
        x: axis.x * s,
        y: axis.y * s,
        z: axis.z * s,
        w: cos(half),
    }
}
/// Vector3.applyQuaternion( q ).
fn apply_quaternion(v: V3, q: Q) -> V3 {
    let tx = 2.0 * (q.y * v.z - q.z * v.y);
    let ty = 2.0 * (q.z * v.x - q.x * v.z);
    let tz = 2.0 * (q.x * v.y - q.y * v.x);
    V3::new(
        v.x + q.w * tx + q.y * tz - q.z * ty,
        v.y + q.w * ty + q.z * tx - q.x * tz,
        v.z + q.w * tz + q.x * ty - q.y * tx,
    )
}
/// Vector3.angleTo( v ).
fn angle_to(a: V3, b: V3) -> f64 {
    let denominator = (a.length_sq() * b.length_sq()).sqrt();
    if denominator == 0.0 {
        return PI / 2.0;
    }
    let theta = a.dot(b) / denominator;
    // MathUtils.clamp: Math.max( -1, Math.min( 1, theta ) ), NaN through.
    acos(if theta.is_nan() {
        theta
    } else {
        theta.clamp(-1.0, 1.0)
    })
}
fn f32s(v: &[f64]) -> Vec<f32> {
    v.iter().map(|&x| x as f32).collect()
}
fn attribute(array: Vec<f32>, item_size: usize) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(
        array, item_size, false,
    )?))
}

/// What getNode() returns for a node.
#[derive(Clone, Debug)]
enum Build {
    Undefined,
    Object(usize),
    Material(usize),
    Texture(usize),
    Geometry(usize),
    MaterialData(Rc<MaterialData>),
    Transform(Rc<TextureTransform>),
    /// A Color, Coordinate, Normal or TextureCoordinate node's values.
    Values(Rc<Vec<Value>>),
    /// WorldInfo's plain object.
    Plain,
}

#[derive(Debug, Default)]
struct MaterialData {
    diffuse: Option<[f64; 3]>,
    emissive: Option<[f64; 3]>,
    specular: Option<[f64; 3]>,
    shininess: Option<Value>,
    transparency: Option<Value>,
}

/// TextureTransform's data; the rotation stays the initial Vector2 ( NaN in
/// arithmetic ) unless set.
#[derive(Debug)]
struct TextureTransform {
    center: [f64; 2],
    rotation: f64,
    scale: [f64; 2],
    translation: [f64; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GeometryType {
    Mesh,
    Line,
    Points,
}
struct Geometry {
    geometry: BufferGeometry,
    /// `_solid` ( `None` when undefined ) as its truthiness.
    solid: Option<bool>,
    kind: Option<GeometryType>,
}

#[derive(Clone)]
struct Object {
    id: u32,
    name: String,
    kind: VrmlNodeKind,
    position: V3,
    quaternion: Q,
    scale: V3,
    render_order: f64,
    visible: bool,
    parent: Option<usize>,
    children: Vec<usize>,
}

struct Texture {
    texture: VrmlTexture,
    /// `__type`, the PixelTexture's components.
    kind: Option<u32>,
}

struct Builder<'a> {
    options: &'a Options,
    node_map: HashMap<String, NodeRef>,
    built: HashMap<*const Node, Build>,
    objects: Vec<Object>,
    materials: Vec<VrmlMaterial>,
    textures: Vec<Texture>,
    geometries: Vec<Geometry>,
    material_ids: u32,
    depth: usize,
}

pub(super) fn scene(tree: &Tree, options: &Options) -> Result<VrmlScene> {
    let mut b = Builder {
        options,
        node_map: HashMap::new(),
        built: HashMap::new(),
        objects: vec![],
        materials: vec![],
        textures: vec![],
        geometries: vec![],
        material_ids: 0,
        depth: 0,
    };
    let scene = b.object(VrmlNodeKind::Scene);
    for node in &tree.nodes {
        b.build_node_map(node);
    }
    for node in &tree.nodes {
        if let Build::Object(o) = b.get_node(&Value::Node(node.clone()))? {
            b.add(scene, o);
        }
    }
    Ok(b.finish(scene))
}

impl Builder<'_> {
    fn object(&mut self, kind: VrmlNodeKind) -> usize {
        self.objects.push(Object {
            id: self.objects.len() as u32,
            name: String::new(),
            kind,
            position: V3::ZERO,
            quaternion: Q::IDENTITY,
            scale: V3::ONE,
            render_order: 0.0,
            visible: true,
            parent: None,
            children: vec![],
        });
        self.objects.len() - 1
    }
    /// Object3D.add(): the child leaves its former parent.
    fn add(&mut self, parent: usize, child: usize) {
        if let Some(old) = self.objects[child].parent {
            self.objects[old].children.retain(|&c| c != child);
        }
        self.objects[child].parent = Some(parent);
        self.objects[parent].children.push(child);
    }
    fn material(&mut self, kind: VrmlMaterialKind) -> usize {
        let phong = kind == VrmlMaterialKind::MeshPhong;
        self.materials.push(VrmlMaterial {
            id: self.material_ids,
            kind,
            name: String::new(),
            color: [1.0; 3],
            emissive: [0.0; 3],
            specular: if phong { hex_color(0x111111) } else { [0.0; 3] },
            shininess: if phong { 30.0 } else { 0.0 },
            opacity: 1.0,
            transparent: false,
            side: 0,
            vertex_colors: false,
            depth_write: true,
            depth_test: true,
            fog: true,
            size: 1.0,
            size_attenuation: true,
            map: None,
        });
        self.material_ids += 1;
        self.materials.len() - 1
    }
    /// Material.clone().
    fn clone_material(&mut self, m: usize) -> usize {
        let mut copy = self.materials[m].clone();
        copy.id = self.material_ids;
        self.material_ids += 1;
        self.materials.push(copy);
        self.materials.len() - 1
    }
    /// Object3D.clone(): a new object of the type ( a mesh, line or points
    /// constructor creates its default material ), then the copy and the
    /// children's clones; geometries and materials stay shared.
    fn clone_object(&mut self, o: usize) -> usize {
        let source = self.objects[o].clone();
        if matches!(
            source.kind,
            VrmlNodeKind::Mesh { .. }
                | VrmlNodeKind::LineSegments { .. }
                | VrmlNodeKind::Points { .. }
        ) {
            self.material_ids += 1;
        }
        let n = self.object(source.kind.clone());
        let target = &mut self.objects[n];
        target.name = source.name;
        target.position = source.position;
        target.quaternion = source.quaternion;
        target.scale = source.scale;
        target.render_order = source.render_order;
        target.visible = source.visible;
        for c in source.children {
            let clone = self.clone_object(c);
            self.add(n, clone);
        }
        n
    }
    fn geometry(
        &mut self,
        geometry: BufferGeometry,
        solid: Option<bool>,
        kind: Option<GeometryType>,
    ) -> Build {
        self.geometries.push(Geometry {
            geometry,
            solid,
            kind,
        });
        Build::Geometry(self.geometries.len() - 1)
    }

    fn build_node_map(&mut self, node: &NodeRef) {
        if let Some(def) = &node.def {
            self.node_map.insert(def.clone(), node.clone());
        }
        for field in &node.fields {
            if field.kind == Some("node") {
                for v in &field.values {
                    if let Value::Node(n) = v {
                        self.build_node_map(n);
                    }
                }
            }
        }
    }

    /// getNode( node ): a USE resolves, a node builds once.
    fn get_node(&mut self, value: &Value) -> Result<Build> {
        match value {
            Value::Use(id) => self.resolve_use(id),
            Value::Node(node) => {
                let key = Rc::as_ptr(node);
                if let Some(b) = self.built.get(&key)
                    && !matches!(b, Build::Undefined)
                {
                    return Ok(b.clone());
                }
                self.depth += 1;
                if self.depth > 1000 {
                    return Err(type_error("Maximum call stack size exceeded"));
                }
                let build = self.build_node(node);
                self.depth -= 1;
                let build = build?;
                self.built.insert(key, build.clone());
                Ok(build)
            }
            _ => Err(type_error("a field value is not a node")),
        }
    }
    /// resolveUSE( identifier ): objects and materials are cloned.
    fn resolve_use(&mut self, id: &str) -> Result<Build> {
        let node = self
            .node_map
            .get(id)
            .cloned()
            .ok_or_else(|| type_error(&format!("no node named {id}")))?;
        Ok(match self.get_node(&Value::Node(node))? {
            Build::Object(o) => Build::Object(self.clone_object(o)),
            Build::Material(m) => Build::Material(self.clone_material(m)),
            Build::Undefined => return Err(type_error("USE of a node without a build")),
            b => b,
        })
    }
    /// getNode() of a field's first value, unless it is NULL; `undefined`
    /// ( an empty field ) is a TypeError in the original.
    fn first_node(&mut self, field: &Field) -> Result<Option<Build>> {
        match field.values.first() {
            Some(Value::Null) => Ok(None),
            Some(v) => self.get_node(v).map(Some),
            None => Err(type_error("Cannot read properties of undefined")),
        }
    }

    fn build_node(&mut self, node: &NodeRef) -> Result<Build> {
        let build = match node.name.as_str() {
            "Anchor" | "Group" | "Transform" | "Collision" => self.grouping(node)?,
            "Background" => self.background(node)?,
            "Shape" => self.shape(node)?,
            "Appearance" => self.appearance(node)?,
            "Material" => material_node(node),
            "ImageTexture" => self.image_texture(node),
            "PixelTexture" => self.pixel_texture(node)?,
            "TextureTransform" => texture_transform(node),
            "IndexedFaceSet" => self.indexed_face_set(node)?,
            "IndexedLineSet" => self.indexed_line_set(node)?,
            "PointSet" => self.point_set(node)?,
            "Box" => self.box_node(node)?,
            "Cone" => self.cone(node)?,
            "Cylinder" => self.cylinder(node)?,
            "Sphere" => self.sphere(node)?,
            "ElevationGrid" => self.elevation_grid(node)?,
            "Extrusion" => self.extrusion(node)?,
            "Color" | "Coordinate" | "Normal" | "TextureCoordinate" => Build::Values(Rc::new(
                node.fields
                    .first()
                    .ok_or_else(|| type_error("Cannot read properties of undefined"))?
                    .values
                    .clone(),
            )),
            "WorldInfo" => Build::Plain,
            "OrthographicCamera" | "PerspectiveCamera" => self.camera(node),
            _ => Build::Undefined,
        };
        if let Some(def) = &node.def {
            match build {
                Build::Object(o) => self.objects[o].name = def.clone(),
                Build::Material(m) => self.materials[m].name = def.clone(),
                Build::Texture(t) => self.textures[t].texture.name = def.clone(),
                Build::Geometry(g) => self.geometries[g].geometry.name = def.clone(),
                _ => {}
            }
        }
        Ok(build)
    }

    fn grouping(&mut self, node: &Node) -> Result<Build> {
        let object = self.object(VrmlNodeKind::Group);
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "children" => self.children(v, object)?,
                "rotation" => {
                    let axis = V3::new(
                        num_or(v.first(), 0.0),
                        num_or(v.get(1), 0.0),
                        num_or(v.get(2), 0.0),
                    )
                    .normalize();
                    self.objects[object].quaternion = axis_angle(axis, num(v.get(3)));
                }
                "scale" => set3(&mut self.objects[object].scale, v),
                "translation" => set3(&mut self.objects[object].position, v),
                _ => {}
            }
        }
        Ok(Build::Object(object))
    }
    /// parseFieldChildren( children, owner ).
    fn children(&mut self, children: &[Value], owner: usize) -> Result<()> {
        for c in children {
            if let Build::Object(o) = self.get_node(c)? {
                self.add(owner, o);
            }
        }
        Ok(())
    }

    fn background(&mut self, node: &Node) -> Result<Build> {
        let group = self.object(VrmlNodeKind::Group);
        let (mut ground_angle, mut ground_color, mut sky_angle, mut sky_color) =
            (None, None, None, None);
        for field in &node.fields {
            let v = Some(numbers(&field.values));
            match field.name.as_str() {
                "groundAngle" => ground_angle = v,
                "groundColor" => ground_color = v,
                "skyAngle" => sky_angle = v,
                "skyColor" => sky_color = v,
                _ => {}
            }
        }
        let radius = 10000.0;
        if let Some(sky) = &sky_color {
            let mut geometry = SphereGeometry::build(radius, 32, 16)?;
            let m = self.material(VrmlMaterialKind::MeshBasic);
            let material = &mut self.materials[m];
            material.fog = false;
            material.side = 1;
            material.depth_write = false;
            material.depth_test = false;
            if sky.len() > 3 {
                paint_faces(
                    &mut geometry,
                    radius,
                    sky_angle.as_deref(),
                    &color_array(sky),
                    true,
                )?;
                self.materials[m].vertex_colors = true;
            } else {
                let c = |i: usize| sky.get(i).copied().unwrap_or(NAN);
                self.materials[m].color = srgb(c(0), c(1), c(2));
            }
            let Build::Geometry(g) = self.geometry(geometry, None, None) else {
                unreachable!()
            };
            let sky = self.object(VrmlNodeKind::Mesh {
                geometry: g,
                material: m,
            });
            self.add(group, sky);
        }
        if let Some(ground) = &ground_color
            && !ground.is_empty()
        {
            let mut geometry =
                SphereGeometry::with_angles(radius, 32, 16, 0.0, 2.0 * PI, 0.5 * PI, 1.5 * PI)?;
            let m = self.material(VrmlMaterialKind::MeshBasic);
            let material = &mut self.materials[m];
            material.fog = false;
            material.side = 1;
            material.vertex_colors = true;
            material.depth_write = false;
            material.depth_test = false;
            paint_faces(
                &mut geometry,
                radius,
                ground_angle.as_deref(),
                &color_array(ground),
                false,
            )?;
            let Build::Geometry(g) = self.geometry(geometry, None, None) else {
                unreachable!()
            };
            let ground = self.object(VrmlNodeKind::Mesh {
                geometry: g,
                material: m,
            });
            self.add(group, ground);
        }
        self.objects[group].render_order = f64::NEG_INFINITY;
        Ok(Build::Object(group))
    }

    fn shape(&mut self, node: &Node) -> Result<Build> {
        let default = self.material(VrmlMaterialKind::MeshBasic);
        self.materials[default].name = DEFAULT_MATERIAL_NAME.into();
        self.materials[default].color = [0.0; 3];
        let mut material = Build::Material(default);
        let mut geometry = Build::Undefined;
        for field in &node.fields {
            match field.name.as_str() {
                "appearance" => {
                    if let Some(b) = self.first_node(field)? {
                        material = b;
                    }
                }
                "geometry" => {
                    if let Some(b) = self.first_node(field)? {
                        geometry = b;
                    }
                }
                _ => {}
            }
        }
        let geometry = match geometry {
            Build::Undefined => None,
            Build::Geometry(g)
                if self.geometries[g]
                    .geometry
                    .attributes
                    .contains_key("position") =>
            {
                Some(g)
            }
            Build::Geometry(_) => None,
            _ => {
                return Err(type_error(
                    "Cannot read properties of undefined ( reading 'position' )",
                ));
            }
        };
        let Some(g) = geometry else {
            let object = self.object(VrmlNodeKind::Object3D);
            self.objects[object].visible = false;
            return Ok(Build::Object(object));
        };
        let Build::Material(material) = material else {
            return Err(type_error("a Shape's appearance is not a material"));
        };
        let has_color = self.geometries[g].geometry.attributes.contains_key("color");
        let kind = self.geometries[g].kind;
        let object = match kind {
            Some(GeometryType::Points) | Some(GeometryType::Line) => {
                let points = kind == Some(GeometryType::Points);
                let m = self.material(if points {
                    VrmlMaterialKind::Points
                } else {
                    VrmlMaterialKind::LineBasic
                });
                let source = self.materials[material].clone();
                let target = &mut self.materials[m];
                target.name = DEFAULT_MATERIAL_NAME.into();
                target.opacity = source.opacity;
                target.transparent = source.transparent;
                if has_color {
                    target.vertex_colors = true;
                } else if source.kind == VrmlMaterialKind::MeshPhong {
                    target.color = source.emissive;
                }
                self.object(if points {
                    VrmlNodeKind::Points {
                        geometry: g,
                        material: m,
                    }
                } else {
                    VrmlNodeKind::LineSegments {
                        geometry: g,
                        material: m,
                    }
                })
            }
            _ => {
                if let Some(solid) = self.geometries[g].solid {
                    self.materials[material].side = if solid { 0 } else { 2 };
                }
                if has_color {
                    self.materials[material].vertex_colors = true;
                }
                self.object(VrmlNodeKind::Mesh {
                    geometry: g,
                    material,
                })
            }
        };
        Ok(Build::Object(object))
    }

    fn appearance(&mut self, node: &Node) -> Result<Build> {
        let mut material = self.material(VrmlMaterialKind::MeshPhong);
        let mut transform = Build::Undefined;
        for field in &node.fields {
            match field.name.as_str() {
                "material" => match self.first_node(field)? {
                    Some(Build::MaterialData(data)) => {
                        let m = &mut self.materials[material];
                        let phong = m.kind == VrmlMaterialKind::MeshPhong;
                        if let Some(c) = data.diffuse {
                            m.color = c;
                        }
                        if let Some(c) = data.emissive {
                            if !phong {
                                return Err(type_error(
                                    "Cannot read properties of undefined ( reading 'copy' )",
                                ));
                            }
                            m.emissive = c;
                        }
                        if truthy(data.shininess.as_ref()) {
                            m.shininess = num(data.shininess.as_ref());
                        }
                        if let Some(c) = data.specular {
                            if !phong {
                                return Err(type_error(
                                    "Cannot read properties of undefined ( reading 'copy' )",
                                ));
                            }
                            m.specular = c;
                        }
                        let t = data.transparency.as_ref();
                        if truthy(t) {
                            m.opacity = 1.0 - num(t);
                        }
                        if num(t) > 0.0 {
                            m.transparent = true;
                        }
                    }
                    Some(Build::Undefined) => {
                        return Err(type_error(
                            "Cannot read properties of undefined ( reading 'diffuseColor' )",
                        ));
                    }
                    Some(_) => {}
                    None => {
                        material = self.material(VrmlMaterialKind::MeshBasic);
                        self.materials[material].name = DEFAULT_MATERIAL_NAME.into();
                        self.materials[material].color = [0.0; 3];
                    }
                },
                "texture" => match field.values.first() {
                    None => {
                        return Err(type_error(
                            "Cannot read properties of undefined ( reading 'name' )",
                        ));
                    }
                    Some(v @ Value::Node(n))
                        if n.name == "ImageTexture" || n.name == "PixelTexture" =>
                    {
                        self.materials[material].map = match self.get_node(v)? {
                            Build::Texture(t) => Some(t),
                            _ => None,
                        };
                    }
                    Some(_) => {}
                },
                "textureTransform" => {
                    if let Some(b) = self.first_node(field)? {
                        transform = b;
                    }
                }
                _ => {}
            }
        }
        if let Some(t) = self.materials[material].map {
            if let Some(kind) = self.textures[t].kind.take() {
                let m = &mut self.materials[material];
                match kind {
                    2 => m.opacity = 1.0,
                    3 => m.color = [1.0; 3],
                    4 => {
                        m.color = [1.0; 3];
                        m.opacity = 1.0;
                    }
                    _ => {}
                }
            }
            match transform {
                Build::Undefined => {}
                Build::Transform(data) => {
                    let texture = &mut self.textures[t].texture;
                    texture.center = data.center;
                    texture.rotation = data.rotation;
                    texture.repeat = data.scale;
                    texture.offset = data.translation;
                    texture.matrix = uv_transform(texture);
                }
                _ => {
                    return Err(type_error(
                        "Cannot read properties of undefined ( reading 'x' )",
                    ));
                }
            }
        }
        Ok(Build::Material(material))
    }

    fn texture(&mut self, image: VrmlImage, data: bool) -> usize {
        let mut texture = VrmlTexture {
            name: String::new(),
            image,
            wrap_s: CLAMP,
            wrap_t: CLAMP,
            mag_filter: if data { NEAREST } else { LINEAR },
            min_filter: if data { NEAREST } else { LINEAR_MIPMAP_LINEAR },
            generate_mipmaps: !data,
            flip_y: !data,
            srgb: false,
            offset: [0.0; 2],
            repeat: [1.0; 2],
            center: [0.0; 2],
            rotation: 0.0,
            matrix: [0.0; 9],
        };
        texture.matrix = uv_transform(&texture);
        self.textures.push(Texture {
            texture,
            kind: None,
        });
        self.textures.len() - 1
    }
    fn pixel_texture(&mut self, node: &Node) -> Result<Build> {
        let mut texture = None;
        let (mut wrap_s, mut wrap_t) = (REPEAT, REPEAT);
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "image" => {
                    let width = num(v.first());
                    let height = num(v.get(1));
                    // getTextureType(): the switch's strict cases.
                    let kind = match v.get(2) {
                        Some(Value::Number(n)) if [1.0, 2.0, 3.0, 4.0].contains(n) => {
                            Some(*n as u32)
                        }
                        _ => None,
                    };
                    // new Uint8Array( 4 × width × height ): ToIndex.
                    let length = 4.0 * width * height;
                    let length = if length.is_nan() { 0.0 } else { length.trunc() };
                    if !(0.0..=1e9).contains(&length) {
                        return Err(Error::Asset(
                            "RangeError: Invalid typed array length".into(),
                        ));
                    }
                    let mut data = vec![0u8; length as usize];
                    let mut color = [0.0f64; 4];
                    for (k, hex) in v.iter().skip(3).enumerate() {
                        parse_hex_color(hex, kind, &mut color)?;
                        for (c, value) in color.iter().enumerate() {
                            if let Some(byte) = data.get_mut(k * 4 + c) {
                                *byte = to_uint8(*value);
                            }
                        }
                    }
                    let t = self.texture(
                        VrmlImage::Data {
                            width,
                            height,
                            data,
                        },
                        true,
                    );
                    self.textures[t].texture.srgb = true;
                    self.textures[t].kind = kind;
                    texture = Some(t);
                }
                "repeatS" if is_bool(v.first(), false) => {
                    wrap_s = CLAMP;
                }
                "repeatT" if is_bool(v.first(), false) => {
                    wrap_t = CLAMP;
                }
                _ => {}
            }
        }
        Ok(match texture {
            Some(t) => {
                self.textures[t].texture.wrap_s = wrap_s;
                self.textures[t].texture.wrap_t = wrap_t;
                Build::Texture(t)
            }
            None => Build::Undefined,
        })
    }
    fn image_texture(&mut self, node: &Node) -> Build {
        let mut texture = None;
        let (mut wrap_s, mut wrap_t) = (REPEAT, REPEAT);
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "url" => {
                    if truthy(v.first())
                        && let Some(url) = v.first()
                    {
                        let url = format!("{}{}", self.options.path, string(url));
                        texture = Some(self.texture(VrmlImage::Url(url), false));
                    }
                }
                "repeatS" if is_bool(v.first(), false) => {
                    wrap_s = CLAMP;
                }
                "repeatT" if is_bool(v.first(), false) => {
                    wrap_t = CLAMP;
                }
                _ => {}
            }
        }
        match texture {
            Some(t) => {
                let texture = &mut self.textures[t].texture;
                texture.wrap_s = wrap_s;
                texture.wrap_t = wrap_t;
                texture.srgb = true;
                Build::Texture(t)
            }
            None => Build::Undefined,
        }
    }

    fn camera(&mut self, node: &Node) -> Build {
        let perspective = node.name == "PerspectiveCamera";
        let camera = self.object(if perspective {
            VrmlNodeKind::PerspectiveCamera {
                fov: 50.0,
                aspect: 1.0,
                near: 0.1,
                far: 2000.0,
            }
        } else {
            VrmlNodeKind::OrthographicCamera {
                left: -1.0,
                right: 1.0,
                top: 1.0,
                bottom: -1.0,
                near: 0.1,
                far: 2000.0,
            }
        });
        let aspect = self.options.window_size.0 / self.options.window_size.1;
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "position" => {
                    // position.set( v0, v1, v0 ), as the original reads it
                    // ( an undefined z keeps the current one ).
                    let o = &mut self.objects[camera];
                    o.position = V3::new(
                        num(v.first()),
                        num(v.get(1)),
                        v.first().map_or(o.position.z, Value::number),
                    );
                }
                "orientation" => {
                    let axis = V3::new(
                        num_or(v.first(), 0.0),
                        num_or(v.get(1), 0.0),
                        num_or(v.get(2), 0.0),
                    )
                    .normalize();
                    self.objects[camera].quaternion = axis_angle(axis, num(v.get(3)));
                }
                "heightAngle" => {
                    if let VrmlNodeKind::PerspectiveCamera { fov, aspect: a, .. } =
                        &mut self.objects[camera].kind
                    {
                        *fov = num(v.first()) * (180.0 / PI);
                        *a = aspect;
                    }
                }
                "height" => {
                    if let VrmlNodeKind::OrthographicCamera {
                        left,
                        right,
                        top,
                        bottom,
                        ..
                    } = &mut self.objects[camera].kind
                    {
                        let half_height = num(v.first()) / 2.0;
                        let half_width = half_height * aspect;
                        *left = -half_width;
                        *right = half_width;
                        *top = half_height;
                        *bottom = -half_height;
                    }
                }
                _ => {}
            }
        }
        Build::Object(camera)
    }

    /// A geometric node's values ( `undefined` when unset ).
    fn data(&mut self, field: &Field) -> Result<Option<Vec<f64>>> {
        Ok(match self.first_node(field)? {
            None | Some(Build::Undefined) => None,
            Some(Build::Values(v)) => Some(numbers(&v)),
            Some(_) => {
                return Err(type_error(
                    "a geometric field is not a Color, Coordinate, Normal or TextureCoordinate",
                ));
            }
        })
    }

    fn indexed_face_set(&mut self, node: &Node) -> Result<Build> {
        let (mut color, mut coord, mut normal, mut tex_coord) = (None, None, None, None);
        let mut ccw = Some(Value::Bool(true));
        let mut solid = Some(Value::Bool(true));
        let mut crease = Some(Value::Number(0.0));
        let (mut color_index, mut coord_index, mut normal_index, mut tex_coord_index) =
            (None, None, None, None);
        let mut color_per_vertex = Some(Value::Bool(true));
        let mut normal_per_vertex = Some(Value::Bool(true));
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "color" if !matches!(v.first(), Some(Value::Null)) => {
                    color = self.data(field)?;
                }
                "coord" if !matches!(v.first(), Some(Value::Null)) => {
                    coord = self.data(field)?;
                }
                "normal" if !matches!(v.first(), Some(Value::Null)) => {
                    normal = self.data(field)?;
                }
                "texCoord" if !matches!(v.first(), Some(Value::Null)) => {
                    tex_coord = self.data(field)?;
                }
                "ccw" => ccw = v.first().cloned(),
                "colorIndex" => color_index = Some(numbers(v)),
                "colorPerVertex" => color_per_vertex = v.first().cloned(),
                "coordIndex" => coord_index = Some(numbers(v)),
                "creaseAngle" => crease = v.first().cloned(),
                "normalIndex" => normal_index = Some(numbers(v)),
                "normalPerVertex" => normal_per_vertex = v.first().cloned(),
                "solid" => solid = v.first().cloned(),
                "texCoordIndex" => tex_coord_index = Some(numbers(v)),
                _ => {}
            }
        }
        let Some(coord_index) = coord_index else {
            return Ok(self.geometry(BufferGeometry::default(), None, None));
        };
        let ccw = truthy(ccw.as_ref());
        let triangulated = triangulate_face_index(&coord_index, ccw);
        fn nonempty(i: &Option<Vec<f64>>) -> Option<&Vec<f64>> {
            i.as_ref().filter(|i| !i.is_empty())
        }
        let mut color_attribute = None;
        if let Some(color) = &color {
            let mut c = if is_bool(color_per_vertex.as_ref(), true) {
                match nonempty(&color_index) {
                    Some(ci) => {
                        indexed_data(&triangulated, &triangulate_face_index(ci, ccw), color, 3)
                    }
                    None => to_non_indexed(&triangulated, &f32s(color), 3),
                }
            } else {
                let faces = match nonempty(&color_index) {
                    Some(ci) => triangulate_face_data(&flatten_data(color, ci), &coord_index),
                    None => triangulate_face_data(color, &coord_index),
                };
                face_data(&triangulated, &faces)
            };
            to_linear(&mut c);
            color_attribute = Some(c);
        }
        let normal_attribute = match &normal {
            Some(normal) => {
                if is_bool(normal_per_vertex.as_ref(), true) {
                    match nonempty(&normal_index) {
                        Some(ni) => {
                            indexed_data(&triangulated, &triangulate_face_index(ni, ccw), normal, 3)
                        }
                        None => to_non_indexed(&triangulated, &f32s(normal), 3),
                    }
                } else {
                    let faces = match nonempty(&normal_index) {
                        Some(ni) => triangulate_face_data(&flatten_data(normal, ni), &coord_index),
                        None => triangulate_face_data(normal, &coord_index),
                    };
                    face_data(&triangulated, &faces)
                }
            }
            None => compute_normals(&triangulated, coord.as_deref(), crease.as_ref())?,
        };
        let uv_attribute = tex_coord
            .as_ref()
            .map(|t| match nonempty(&tex_coord_index) {
                Some(ti) => indexed_data(&triangulated, &triangulate_face_index(ti, ccw), t, 2),
                None => to_non_indexed(&triangulated, &f32s(t), 2),
            });
        let mut geometry = BufferGeometry::default();
        let coord = coord.unwrap_or_default();
        geometry.set_attribute(
            "position",
            attribute(to_non_indexed(&triangulated, &f32s(&coord), 3), 3)?,
        );
        geometry.set_attribute("normal", attribute(normal_attribute, 3)?);
        if let Some(c) = color_attribute {
            geometry.set_attribute("color", attribute(c, 3)?);
        }
        if let Some(uv) = uv_attribute {
            geometry.set_attribute("uv", attribute(uv, 2)?);
        }
        Ok(self.geometry(
            geometry,
            solid.as_ref().map(|s| truthy(Some(s))),
            Some(GeometryType::Mesh),
        ))
    }

    fn indexed_line_set(&mut self, node: &Node) -> Result<Build> {
        let (mut color, mut coord) = (None, None);
        let (mut color_index, mut coord_index) = (None, None);
        let mut color_per_vertex = Some(Value::Bool(true));
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "color" if !matches!(v.first(), Some(Value::Null)) => {
                    color = self.data(field)?;
                }
                "coord" if !matches!(v.first(), Some(Value::Null)) => {
                    coord = self.data(field)?;
                }
                "colorIndex" => color_index = Some(numbers(v)),
                "colorPerVertex" => color_per_vertex = v.first().cloned(),
                "coordIndex" => coord_index = Some(numbers(v)),
                _ => {}
            }
        }
        let coord_index = coord_index.ok_or_else(|| {
            type_error("Cannot read properties of undefined ( reading 'length' )")
        })?;
        let expanded = expand_line_index(&coord_index);
        let mut color_attribute = None;
        if let Some(color) = &color {
            let color_index = color_index.as_ref().ok_or_else(|| {
                type_error("Cannot read properties of undefined ( reading 'length' )")
            })?;
            let mut c = if is_bool(color_per_vertex.as_ref(), true) {
                if !color_index.is_empty() {
                    indexed_data(&expanded, &expand_line_index(color_index), color, 3)
                } else {
                    to_non_indexed(&expanded, &f32s(color), 3)
                }
            } else if !color_index.is_empty() {
                line_data(
                    &expanded,
                    &expand_line_data(&flatten_data(color, color_index), &coord_index),
                )
            } else {
                line_data(&expanded, &expand_line_data(color, &coord_index))
            };
            to_linear(&mut c);
            color_attribute = Some(c);
        }
        let mut geometry = BufferGeometry::default();
        let coord = coord.unwrap_or_default();
        geometry.set_attribute(
            "position",
            attribute(to_non_indexed(&expanded, &f32s(&coord), 3), 3)?,
        );
        if let Some(c) = color_attribute {
            geometry.set_attribute("color", attribute(c, 3)?);
        }
        Ok(self.geometry(geometry, None, Some(GeometryType::Line)))
    }

    fn point_set(&mut self, node: &Node) -> Result<Build> {
        let (mut color, mut coord) = (None, None);
        for field in &node.fields {
            match field.name.as_str() {
                "color" if !matches!(field.values.first(), Some(Value::Null)) => {
                    color = self.data(field)?;
                }
                "coord" if !matches!(field.values.first(), Some(Value::Null)) => {
                    coord = self.data(field)?;
                }
                _ => {}
            }
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute("position", attribute(f32s(&coord.unwrap_or_default()), 3)?);
        if let Some(color) = &color {
            let mut c = f32s(color);
            to_linear(&mut c);
            geometry.set_attribute("color", attribute(c, 3)?);
        }
        Ok(self.geometry(geometry, None, Some(GeometryType::Points)))
    }

    fn box_node(&mut self, node: &Node) -> Result<Build> {
        let mut size = [Some(2.0), Some(2.0), Some(2.0)];
        for field in &node.fields {
            if field.name == "size" {
                for (i, s) in size.iter_mut().enumerate() {
                    *s = field.values.get(i).map(Value::number);
                }
            }
        }
        let [x, y, z] = size.map(|s| s.unwrap_or(1.0));
        let g = BoxGeometry::build(x, y, z)?;
        Ok(self.geometry(g, None, None))
    }
    fn cone(&mut self, node: &Node) -> Result<Build> {
        let (mut radius, mut height, mut open) = (Some(1.0), Some(2.0), false);
        for field in &node.fields {
            let v = field.values.first();
            match field.name.as_str() {
                "bottom" => open = !truthy(v),
                "bottomRadius" => radius = v.map(Value::number),
                "height" => height = v.map(Value::number),
                _ => {}
            }
        }
        let g = CylinderGeometry::build(
            0.0,
            radius.unwrap_or(1.0),
            height.unwrap_or(1.0),
            16,
            1,
            open,
            0.0,
            2.0 * PI,
        )?;
        Ok(self.geometry(g, None, None))
    }
    fn cylinder(&mut self, node: &Node) -> Result<Build> {
        let (mut radius, mut height) = (Some(1.0), Some(2.0));
        for field in &node.fields {
            let v = field.values.first();
            match field.name.as_str() {
                "radius" => radius = v.map(Value::number),
                "height" => height = v.map(Value::number),
                _ => {}
            }
        }
        let r = radius.unwrap_or(1.0);
        let g = CylinderGeometry::build(r, r, height.unwrap_or(1.0), 16, 1, false, 0.0, 2.0 * PI)?;
        Ok(self.geometry(g, None, None))
    }
    fn sphere(&mut self, node: &Node) -> Result<Build> {
        let mut radius = Some(1.0);
        for field in &node.fields {
            if field.name == "radius" {
                radius = field.values.first().map(Value::number);
            }
        }
        let g = SphereGeometry::build(radius.unwrap_or(1.0), 16, 16)?;
        Ok(self.geometry(g, None, None))
    }

    fn elevation_grid(&mut self, node: &Node) -> Result<Build> {
        let (mut color, mut normal, mut tex_coord) = (None, None, None);
        let mut height: Option<Vec<f64>> = None;
        let mut color_per_vertex = Some(Value::Bool(true));
        let mut normal_per_vertex = Some(Value::Bool(true));
        let mut solid = Some(Value::Bool(true));
        let mut ccw = Some(Value::Bool(true));
        let mut crease = Some(Value::Number(0.0));
        let (mut x_dimension, mut z_dimension, mut x_spacing, mut z_spacing) = (2.0, 2.0, 1.0, 1.0);
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "color" if !matches!(v.first(), Some(Value::Null)) => {
                    color = self.data(field)?;
                }
                "normal" if !matches!(v.first(), Some(Value::Null)) => {
                    normal = self.data(field)?;
                }
                "texCoord" if !matches!(v.first(), Some(Value::Null)) => {
                    tex_coord = self.data(field)?;
                }
                "height" => height = Some(numbers(v)),
                "ccw" => ccw = v.first().cloned(),
                "colorPerVertex" => color_per_vertex = v.first().cloned(),
                "creaseAngle" => crease = v.first().cloned(),
                "normalPerVertex" => normal_per_vertex = v.first().cloned(),
                "solid" => solid = v.first().cloned(),
                "xDimension" => x_dimension = num(v.first()),
                "xSpacing" => x_spacing = num(v.first()),
                "zDimension" => z_dimension = num(v.first()),
                "zSpacing" => z_spacing = num(v.first()),
                _ => {}
            }
        }
        let per_vertex_colors = is_bool(color_per_vertex.as_ref(), true);
        let per_vertex_normals = is_bool(normal_per_vertex.as_ref(), true);
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        let mut i = 0.0;
        while i < z_dimension {
            let mut j = 0.0;
            while j < x_dimension {
                let index = i * x_dimension + j;
                let y = match &height {
                    Some(h) => at(h, index),
                    None => return Err(type_error("Cannot read properties of undefined")),
                };
                vertices.extend([x_spacing * i, y, z_spacing * j]);
                if let Some(c) = &color
                    && per_vertex_colors
                {
                    colors.extend((0..3).map(|k| at(c, index * 3.0 + k as f64)));
                }
                if let Some(n) = &normal
                    && per_vertex_normals
                {
                    normals.extend((0..3).map(|k| at(n, index * 3.0 + k as f64)));
                }
                match &tex_coord {
                    Some(t) => uvs.extend([at(t, index * 2.0), at(t, index * 2.0 + 1.0)]),
                    None => uvs.extend([i / (x_dimension - 1.0), j / (z_dimension - 1.0)]),
                }
                j += 1.0;
            }
            i += 1.0;
        }
        let ccw = is_bool(ccw.as_ref(), true);
        let mut indices: Vec<f64> = vec![];
        let mut i = 0.0;
        while i < x_dimension - 1.0 {
            let mut j = 0.0;
            while j < z_dimension - 1.0 {
                let a = i + j * x_dimension;
                let b = i + (j + 1.0) * x_dimension;
                let c = (i + 1.0) + (j + 1.0) * x_dimension;
                let d = (i + 1.0) + j * x_dimension;
                if ccw {
                    indices.extend([a, c, b, c, a, d]);
                } else {
                    indices.extend([a, b, c, c, d, a]);
                }
                j += 1.0;
            }
            i += 1.0;
        }
        let position = to_non_indexed(&indices, &f32s(&vertices), 3);
        let uv = to_non_indexed(&indices, &f32s(&uvs), 2);
        // One value per quad, six times.
        let per_quad = |data: &[f64], out: &mut Vec<f64>| {
            let mut i = 0.0;
            while i < x_dimension - 1.0 {
                let mut j = 0.0;
                while j < z_dimension - 1.0 {
                    let index = i + j * (x_dimension - 1.0);
                    let v: Vec<f64> = (0..3).map(|k| at(data, index * 3.0 + k as f64)).collect();
                    for _ in 0..6 {
                        out.extend(&v);
                    }
                    j += 1.0;
                }
                i += 1.0;
            }
        };
        let mut color_attribute = None;
        if let Some(c) = &color {
            let mut attribute = if is_bool(color_per_vertex.as_ref(), false) {
                per_quad(c, &mut colors);
                f32s(&colors)
            } else {
                to_non_indexed(&indices, &f32s(&colors), 3)
            };
            to_linear(&mut attribute);
            color_attribute = Some(attribute);
        }
        let normal_attribute = match &normal {
            Some(n) => {
                if is_bool(normal_per_vertex.as_ref(), false) {
                    per_quad(n, &mut normals);
                    f32s(&normals)
                } else {
                    to_non_indexed(&indices, &f32s(&normals), 3)
                }
            }
            None => compute_normals(&indices, Some(&vertices), crease.as_ref())?,
        };
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute("position", attribute(position, 3)?);
        geometry.set_attribute("normal", attribute(normal_attribute, 3)?);
        geometry.set_attribute("uv", attribute(uv, 2)?);
        if let Some(c) = color_attribute {
            geometry.set_attribute("color", attribute(c, 3)?);
        }
        Ok(self.geometry(
            geometry,
            solid.as_ref().map(|s| truthy(Some(s))),
            Some(GeometryType::Mesh),
        ))
    }

    fn extrusion(&mut self, node: &Node) -> Result<Build> {
        let n = |v: f64| Value::Number(v);
        let mut cross_section: Vec<Value> = [1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0]
            .into_iter()
            .map(n)
            .collect();
        let mut spine: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let (mut scale, mut orientation): (Option<Vec<f64>>, Option<Vec<f64>>) = (None, None);
        let mut begin_cap = Some(Value::Bool(true));
        let mut ccw = Some(Value::Bool(true));
        let mut crease = Some(Value::Number(0.0));
        let mut end_cap = Some(Value::Bool(true));
        let mut solid = Some(Value::Bool(true));
        for field in &node.fields {
            let v = &field.values;
            match field.name.as_str() {
                "beginCap" => begin_cap = v.first().cloned(),
                "ccw" => ccw = v.first().cloned(),
                "creaseAngle" => crease = v.first().cloned(),
                "crossSection" => cross_section = v.clone(),
                "endCap" => end_cap = v.first().cloned(),
                "orientation" => orientation = Some(numbers(v)),
                "scale" => scale = Some(numbers(v)),
                "solid" => solid = v.first().cloned(),
                "spine" => spine = numbers(v),
                _ => {}
            }
        }
        let len = cross_section.len();
        let closed = strict_eq(
            cross_section.first(),
            len.checked_sub(2).and_then(|i| cross_section.get(i)),
        ) && strict_eq(
            cross_section.get(1),
            len.checked_sub(1).and_then(|i| cross_section.get(i)),
        );
        let section = numbers(&cross_section);
        let mut vertices = vec![];
        let (mut i, mut j, mut o) = (0usize, 0usize, 0usize);
        while i < spine.len() {
            let spine_vector = V3::new(
                at(&spine, i as f64),
                at(&spine, i as f64 + 1.0),
                at(&spine, i as f64 + 2.0),
            );
            let scaling = V3::new(
                scale.as_ref().map_or(1.0, |s| at(s, j as f64)),
                1.0,
                scale.as_ref().map_or(1.0, |s| at(s, j as f64 + 1.0)),
            );
            let axis = match &orientation {
                Some(r) => V3::new(
                    at(r, o as f64),
                    at(r, o as f64 + 1.0),
                    at(r, o as f64 + 2.0),
                ),
                None => V3::new(0.0, 0.0, 1.0),
            };
            let angle = orientation.as_ref().map_or(0.0, |r| at(r, o as f64 + 3.0));
            let mut k = 0;
            while k < section.len() {
                let vertex = V3::new(
                    section[k] * scaling.x,
                    0.0 * scaling.y,
                    at(&section, k as f64 + 1.0) * scaling.z,
                );
                let vertex = apply_quaternion(vertex, axis_angle(axis, angle));
                vertices.extend([
                    vertex.x + spine_vector.x,
                    vertex.y + spine_vector.y,
                    vertex.z + spine_vector.z,
                ]);
                k += 2;
            }
            i += 3;
            j += 2;
            o += 4;
        }
        let spine_count = spine.len() as f64 / 3.0;
        let section_count = section.len() as f64 / 2.0;
        let ccw = is_bool(ccw.as_ref(), true);
        let mut indices: Vec<f64> = vec![];
        let mut i = 0.0;
        while i < spine_count - 1.0 {
            let mut j = 0.0;
            while j < section_count - 1.0 {
                let a = j + i * section_count;
                let mut b = (j + 1.0) + i * section_count;
                let c = j + (i + 1.0) * section_count;
                let mut d = (j + 1.0) + (i + 1.0) * section_count;
                if j == section_count - 2.0 && closed {
                    b = i * section_count;
                    d = (i + 1.0) * section_count;
                }
                if ccw {
                    indices.extend([a, b, c, c, b, d]);
                } else {
                    indices.extend([a, c, b, c, d, b]);
                }
                j += 1.0;
            }
            i += 1.0;
        }
        let begin = is_bool(begin_cap.as_ref(), true);
        let end = is_bool(end_cap.as_ref(), true);
        if begin || end {
            // new Vector2( x, y ): undefined components default to 0.
            let mut contour: Vec<[f64; 2]> = (0..section.len().div_ceil(2))
                .map(|p| {
                    let c = |i: usize| section.get(i).copied().unwrap_or(0.0);
                    [c(p * 2), c(p * 2 + 1)]
                })
                .collect();
            let faces = crate::shape_utils::triangulate(&mut contour, &mut []);
            let cap: Vec<f64> = faces.iter().flatten().map(|&i| i as f64).collect();
            if begin {
                for t in cap.as_chunks::<3>().0 {
                    if ccw {
                        indices.extend([t[0], t[1], t[2]]);
                    } else {
                        indices.extend([t[0], t[2], t[1]]);
                    }
                }
            }
            if end {
                let offset = section_count * (spine_count - 1.0);
                for t in cap.as_chunks::<3>().0 {
                    if ccw {
                        indices.extend([offset + t[0], offset + t[2], offset + t[1]]);
                    } else {
                        indices.extend([offset + t[0], offset + t[1], offset + t[2]]);
                    }
                }
            }
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            attribute(to_non_indexed(&indices, &f32s(&vertices), 3), 3)?,
        );
        geometry.set_attribute(
            "normal",
            attribute(
                compute_normals(&indices, Some(&vertices), crease.as_ref())?,
                3,
            )?,
        );
        Ok(self.geometry(
            geometry,
            solid.as_ref().map(|s| truthy(Some(s))),
            Some(GeometryType::Mesh),
        ))
    }

    /// The scene in traverse() order, with world matrices.
    fn finish(self, scene: usize) -> VrmlScene {
        let mut nodes = vec![];
        let mut stack = vec![(scene, None, M4::IDENTITY)];
        while let Some((o, parent, parent_world)) = stack.pop() {
            let object = &self.objects[o];
            let local = M4::compose(object.position, object.quaternion, object.scale);
            let world = if parent.is_some() {
                M4::multiply_matrices(&parent_world, &local)
            } else {
                local
            };
            let index = nodes.len();
            nodes.push(VrmlNode {
                id: object.id,
                name: object.name.clone(),
                parent,
                kind: object.kind.clone(),
                position: object.position.to_array(),
                quaternion: object.quaternion.to_array(),
                scale: object.scale.to_array(),
                render_order: object.render_order,
                visible: object.visible,
                matrix_world: world.0,
            });
            for &c in object.children.iter().rev() {
                stack.push((c, Some(index), world));
            }
        }
        VrmlScene {
            nodes,
            geometries: self
                .geometries
                .into_iter()
                .map(|g| VrmlGeometry {
                    geometry: g.geometry,
                })
                .collect(),
            materials: self.materials,
            textures: self.textures.into_iter().map(|t| t.texture).collect(),
        }
    }
}

fn material_node(node: &Node) -> Build {
    let mut data = MaterialData::default();
    for field in &node.fields {
        let v = &field.values;
        let color = || srgb(num(v.first()), num(v.get(1)), num(v.get(2)));
        match field.name.as_str() {
            "diffuseColor" => data.diffuse = Some(color()),
            "emissiveColor" => data.emissive = Some(color()),
            "shininess" => data.shininess = v.first().cloned(),
            "specularColor" => data.specular = Some(color()),
            "transparency" => data.transparency = v.first().cloned(),
            _ => {}
        }
    }
    Build::MaterialData(Rc::new(data))
}

fn texture_transform(node: &Node) -> Build {
    let mut data = TextureTransform {
        center: [0.0; 2],
        rotation: NAN,
        scale: [0.0; 2],
        translation: [0.0; 2],
    };
    for field in &node.fields {
        let v = &field.values;
        let pair = || [num(v.first()), num(v.get(1))];
        match field.name.as_str() {
            "center" => data.center = pair(),
            "rotation" => data.rotation = num(v.first()),
            "scale" => data.scale = pair(),
            "translation" => data.translation = pair(),
            _ => {}
        }
    }
    Build::Transform(Rc::new(data))
}

/// Matrix3.setUvTransform( offset, repeat, rotation, center ).
fn uv_transform(t: &VrmlTexture) -> [f64; 9] {
    let (tx, ty) = (t.offset[0], t.offset[1]);
    let (sx, sy) = (t.repeat[0], t.repeat[1]);
    let (cx, cy) = (t.center[0], t.center[1]);
    let c = cos(t.rotation);
    let s = sin(t.rotation);
    [
        sx * c,
        -sy * s,
        0.0,
        sx * s,
        sy * c,
        0.0,
        -sx * (c * cx + s * cy) + cx + tx,
        -sy * (-s * cx + c * cy) + cy + ty,
        1.0,
    ]
}

/// The ToUint8 a Uint8Array store applies.
fn to_uint8(v: f64) -> u8 {
    if !v.is_finite() {
        0
    } else {
        (v.trunc().rem_euclid(256.0)) as u8
    }
}

/// parseHexColor( hex, textureType, color ).
fn parse_hex_color(hex: &Value, kind: Option<u32>, color: &mut [f64; 4]) -> Result<()> {
    let text = || match hex {
        Value::Hex(s) | Value::String(s) => Ok(s.clone()),
        _ => Err(type_error("hex.substring is not a function")),
    };
    // String.prototype.substring( a, b ) on the ASCII literal.
    let sub = |s: &str, a: usize, b: usize| -> String {
        let chars: Vec<char> = s.chars().collect();
        chars[a.min(chars.len())..b.min(chars.len())]
            .iter()
            .collect()
    };
    let byte = |s: &str, a: usize| parse_int(&format!("0x{}", sub(s, a, a + 2)));
    match kind {
        Some(1) => {
            let value = parse_int(&string(hex));
            *color = [value, value, value, 1.0];
        }
        Some(2) => {
            let s = text()?;
            let value = byte(&s, 2);
            *color = [value, value, value, byte(&s, 4)];
        }
        Some(3) => {
            let s = text()?;
            *color = [byte(&s, 2), byte(&s, 4), byte(&s, 6), 1.0];
        }
        Some(4) => {
            let s = text()?;
            *color = [byte(&s, 2), byte(&s, 4), byte(&s, 6), byte(&s, 8)];
        }
        _ => {}
    }
    Ok(())
}

/// toColorArray( colors ): `new Color( r, g, b )` per triple ( a lone red
/// value reads as a hex ).
fn color_array(colors: &[f64]) -> Vec<[f64; 3]> {
    colors
        .chunks(3)
        .map(|c| match c {
            [r] => {
                let hex = r.floor();
                if hex.is_nan() {
                    [NAN; 3]
                } else {
                    hex_color(hex as i64 as u32)
                }
            }
            [r, g] => [*r, *g, NAN],
            [r, g, b] => [*r, *g, *b],
            _ => [1.0; 3],
        })
        .collect()
}

/// paintFaces( geometry, radius, angles, colors, topDown ).
fn paint_faces(
    geometry: &mut BufferGeometry,
    radius: f64,
    angles: Option<&[f64]>,
    colors: &[[f64; 3]],
    top_down: bool,
) -> Result<()> {
    let start = if top_down { 0.0 } else { PI };
    let mut thresholds = vec![];
    for i in 0..colors.len() {
        let mut angle = if i == 0 {
            0.0
        } else {
            angles
                .ok_or_else(|| type_error("Cannot read properties of undefined"))?
                .get(i - 1)
                .copied()
                .unwrap_or(NAN)
        };
        if !top_down {
            angle = start - angle;
        }
        // setFromSphericalCoords( radius, angle, 0 ).y
        thresholds.push(cos(angle) * radius);
    }
    let positions = match geometry.attributes.get("position") {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => return Err(Error::Invalid("sphere positions")),
    };
    let mut out = vec![0.0f32; positions.len()];
    let index = geometry.index.clone().unwrap_or_default();
    for &i in &index {
        let i = i as usize;
        let y = f64::from(positions[i * 3 + 1]);
        let (mut a, mut b) = (None, None);
        let mut t = 1.0;
        for j in 1..thresholds.len() {
            a = Some(j - 1);
            b = Some(j);
            let (ta, tb) = (thresholds[j - 1], thresholds[j]);
            let inside = if top_down {
                y <= ta && y > tb
            } else {
                y >= ta && y < tb
            };
            if inside {
                t = (ta - y).abs() / (ta - tb).abs();
                break;
            }
        }
        let (Some(a), Some(b)) = (a, b) else {
            return Err(type_error(
                "Cannot read properties of undefined ( reading 'r' )",
            ));
        };
        let (ca, cb) = (colors[a], colors[b]);
        for k in 0..3 {
            let c = ca[k] + (cb[k] - ca[k]) * t;
            out[i * 3 + k] = srgb_to_linear(c) as f32;
        }
    }
    geometry.set_attribute("color", attribute(out, 3)?);
    Ok(())
}

/// triangulateFaceIndex( index, ccw ); `undefined` entries are NaN.
fn triangulate_face_index(index: &[f64], ccw: bool) -> Vec<f64> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut out = vec![];
    let mut start = 0;
    let mut i = 0;
    while i < index.len() {
        let (o2, o3) = if ccw { (1, 2) } else { (2, 1) };
        out.extend([get(start), get(i + o2), get(i + o3)]);
        if get(i + 3) == -1.0 || i + 3 >= index.len() {
            i += 3;
            start = i + 1;
        }
        i += 1;
    }
    out
}
fn triangulate_face_data(data: &[f64], index: &[f64]) -> Vec<f64> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut out = vec![];
    let mut start = 0usize;
    let mut i = 0;
    while i < index.len() {
        let stride = (start * 3) as f64;
        out.extend([
            at(data, stride),
            at(data, stride + 1.0),
            at(data, stride + 2.0),
        ]);
        if get(i + 3) == -1.0 || i + 3 >= index.len() {
            i += 3;
            start += 1;
        }
        i += 1;
    }
    out
}
fn flatten_data(data: &[f64], index: &[f64]) -> Vec<f64> {
    index
        .iter()
        .flat_map(|&i| {
            let stride = i * 3.0;
            [
                at(data, stride),
                at(data, stride + 1.0),
                at(data, stride + 2.0),
            ]
        })
        .collect()
}
fn expand_line_index(index: &[f64]) -> Vec<f64> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut out = vec![];
    let mut i = 0;
    while i < index.len() {
        out.extend([get(i), get(i + 1)]);
        if get(i + 2) == -1.0 || i + 2 >= index.len() {
            i += 2;
        }
        i += 1;
    }
    out
}
fn expand_line_data(data: &[f64], index: &[f64]) -> Vec<f64> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut out = vec![];
    let mut start = 0usize;
    let mut i = 0;
    while i < index.len() {
        let stride = (start * 3) as f64;
        out.extend([
            at(data, stride),
            at(data, stride + 1.0),
            at(data, stride + 2.0),
        ]);
        if get(i + 2) == -1.0 || i + 2 >= index.len() {
            i += 2;
            start += 1;
        }
        i += 1;
    }
    out
}
/// computeAttributeFromIndexedData( coordIndex, index, data, itemSize ).
fn indexed_data(coord_index: &[f64], index: &[f64], data: &[f64], item_size: usize) -> Vec<f32> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut out = vec![];
    let mut i = 0;
    while i < coord_index.len() {
        for v in [get(i), get(i + 1), get(i + 2)] {
            let offset = v * item_size as f64;
            for k in 0..item_size {
                out.push(at(data, offset + k as f64) as f32);
            }
        }
        i += 3;
    }
    out
}
/// computeAttributeFromFaceData( index, faceData ).
fn face_data(index: &[f64], faces: &[f64]) -> Vec<f32> {
    let mut out = vec![];
    let (mut i, mut j) = (0, 0);
    while i < index.len() {
        let v = [0, 1, 2].map(|k| at(faces, (j * 3 + k) as f64) as f32);
        for _ in 0..3 {
            out.extend(v);
        }
        i += 3;
        j += 1;
    }
    out
}
/// computeAttributeFromLineData( index, lineData ).
fn line_data(index: &[f64], lines: &[f64]) -> Vec<f32> {
    let mut out = vec![];
    let (mut i, mut j) = (0, 0);
    while i < index.len() {
        let v = [0, 1, 2].map(|k| at(lines, (j * 3 + k) as f64) as f32);
        out.extend(v);
        out.extend(v);
        i += 2;
        j += 1;
    }
    out
}
/// toNonIndexedAttribute( indices, attribute ) of a Float32 attribute.
fn to_non_indexed(indices: &[f64], array: &[f32], item_size: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(indices.len() * item_size);
    for &i in indices {
        let base = i * item_size as f64;
        for k in 0..item_size {
            let at = base + k as f64;
            out.push(
                if at >= 0.0 && at.fract() == 0.0 && at < array.len() as f64 {
                    array[at as usize]
                } else {
                    f32::NAN
                },
            );
        }
    }
    out
}
/// convertColorsToLinearSRGB( attribute ).
fn to_linear(colors: &mut [f32]) {
    for c in colors.iter_mut() {
        *c = srgb_to_linear(f64::from(*c)) as f32;
    }
}
/// The vertex-normals key ( `String( index )` ).
fn key(i: f64) -> String {
    if i.is_nan() {
        "undefined".into()
    } else {
        crate::fbx::js::number_key(i)
    }
}
/// computeNormalAttribute( index, coord, creaseAngle ).
fn compute_normals(
    index: &[f64],
    coord: Option<&[f64]>,
    crease: Option<&Value>,
) -> Result<Vec<f32>> {
    let get = |i: usize| index.get(i).copied().unwrap_or(NAN);
    let mut faces = vec![];
    let mut vertex_normals: HashMap<String, Vec<usize>> = HashMap::new();
    let mut i = 0;
    while i < index.len() {
        let coord = coord.ok_or_else(|| type_error("Cannot read properties of undefined"))?;
        let (a, b, c) = (get(i), get(i + 1), get(i + 2));
        let v = |x: f64| {
            V3::new(
                at(coord, x * 3.0),
                at(coord, x * 3.0 + 1.0),
                at(coord, x * 3.0 + 2.0),
            )
        };
        let (va, vb, vc) = (v(a), v(b), v(c));
        let normal = V3::cross(vc.sub(vb), va.sub(vb)).normalize();
        for x in [a, b, c] {
            vertex_normals.entry(key(x)).or_default().push(faces.len());
        }
        faces.push((a, b, c, normal));
        i += 3;
    }
    let zero = matches!(crease, Some(Value::Number(n)) if *n == 0.0);
    let crease = crease.map_or(NAN, Value::number);
    let mut out = Vec::with_capacity(faces.len() * 9);
    for &(a, b, c, normal) in &faces {
        for x in [a, b, c] {
            let mut n = V3::ZERO;
            if zero {
                n = normal;
            } else {
                for &f in &vertex_normals[&key(x)] {
                    let other = faces[f].3;
                    if angle_to(other, normal) < crease {
                        n = V3::new(n.x + other.x, n.y + other.y, n.z + other.z);
                    }
                }
            }
            let n = n.normalize();
            out.extend([n.x as f32, n.y as f32, n.z as f32]);
        }
    }
    Ok(out)
}
