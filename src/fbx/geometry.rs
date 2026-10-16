//! FBXLoader's GeometryParser: meshes ( n-gons triangulated with earcut, skin
//! weights, vertex colors, uv sets, material groups, morph targets ) and
//! NURBS curves.
use super::FbxGeometry;
use super::js::{Obj, Value, number_key, parse_int, parse_int_value};
use super::math::{M3, M4, V3};
use super::scene::{
    Ctx, Deformers, Links, TransformData, bad, euler_order, generate_transform, map_key,
    prop_value, srgb_color,
};
use crate::attribute::BufferAttribute;
use crate::geometry::{Attribute, BufferGeometry};
use crate::{Result, shape_utils};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(super) enum Attr {
    F32 { data: Vec<f32>, size: usize },
    U16 { data: Vec<u16>, size: usize },
}

/// A parsed BufferGeometry, before it becomes the library's.
#[derive(Clone, Debug, Default)]
pub(super) struct Geo {
    pub name: Option<String>,
    pub attributes: Vec<(String, Attr)>,
    /// ( start, count, materialIndex ).
    pub groups: Vec<(f64, f64, Option<f64>)>,
    pub morph_positions: Vec<(Option<String>, Vec<f32>)>,
    pub morph_relative: bool,
    /// `FBX_Deformer`: the skeleton's index.
    pub deformer: Option<usize>,
}

impl Geo {
    pub(super) fn has_attribute(&self, name: &str) -> bool {
        self.attributes.iter().any(|(n, _)| n == name)
    }
    fn set_attribute(&mut self, name: &str, a: Attr) {
        if let Some(slot) = self.attributes.iter_mut().find(|(n, _)| n == name) {
            slot.1 = a;
        } else {
            self.attributes.push((name.to_string(), a));
        }
    }
    fn count(&self) -> usize {
        match self.attributes.iter().find(|(n, _)| n == "position") {
            Some((_, Attr::F32 { data, size })) => data.len() / size,
            _ => 0,
        }
    }
    /// SkinnedMesh.normalizeSkinWeights().
    pub(super) fn normalize_skin_weights(&mut self) {
        if let Some((_, Attr::F32 { data, .. })) =
            self.attributes.iter_mut().find(|(n, _)| n == "skinWeight")
        {
            for w in data.chunks_mut(4) {
                let v: Vec<f64> = w.iter().map(|&x| f64::from(x)).collect();
                let scale = 1.0 / v.iter().map(|x| x.abs()).sum::<f64>();
                if scale != f64::INFINITY {
                    for (o, x) in w.iter_mut().zip(&v) {
                        *o = (x * scale) as f32;
                    }
                } else {
                    w.copy_from_slice(&[1.0, 0.0, 0.0, 0.0]);
                }
            }
        }
    }
    /// Mesh.updateMorphTargets(): the dictionary of the position morphs.
    pub(super) fn morph_dictionary(&self) -> Vec<(String, usize)> {
        let o = Obj::new();
        for (i, (name, _)) in self.morph_positions.iter().enumerate() {
            let name = match name {
                Some(n) if !n.is_empty() => n.clone(),
                _ => i.to_string(),
            };
            o.set(&name, Value::Number(i as f64));
        }
        o.entries()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_number() as usize))
            .collect()
    }
    pub(super) fn into_geometry(self) -> Result<FbxGeometry> {
        let mut g = BufferGeometry {
            name: self.name.unwrap_or_default(),
            ..Default::default()
        };
        for (name, a) in self.attributes {
            g.set_attribute(
                &name,
                match a {
                    Attr::F32 { data, size } => {
                        Attribute::F32(BufferAttribute::new(data, size, false)?)
                    }
                    Attr::U16 { data, size } => {
                        Attribute::U16(BufferAttribute::new(data, size, false)?)
                    }
                },
            );
        }
        for (start, count, material) in self.groups {
            g.add_group(
                start as usize,
                count as usize,
                material.unwrap_or(0.0).max(0.0) as usize,
            );
        }
        let mut names = vec![];
        if !self.morph_positions.is_empty() {
            let mut list = vec![];
            for (name, data) in self.morph_positions {
                names.push(name.unwrap_or_default());
                list.push(Attribute::F32(BufferAttribute::new(data, 3, false)?));
            }
            g.morph_attributes.insert("position".into(), list);
        }
        g.morph_targets_relative = self.morph_relative;
        Ok(FbxGeometry {
            geometry: g,
            morph_names: names,
        })
    }
}

/// Mapping information for one layer element ( getData's infoObject ).
struct Info {
    data_size: usize,
    buffer: Vec<f64>,
    indices: Vec<f64>,
    mapping: String,
    reference: String,
}

#[derive(Default)]
struct GeoInfo {
    vertex_positions: Vec<f64>,
    vertex_indices: Vec<f64>,
    base_positions: Option<Vec<f64>>,
    color: Option<Info>,
    material: Option<Info>,
    normal: Option<Info>,
    uv: Option<Vec<Info>>,
    /// weightTable: vertex index → ( bone, weight ).
    weights: Option<HashMap<String, Vec<(f64, f64)>>>,
}

#[derive(Default)]
struct Buffers {
    vertex: Vec<f64>,
    normal: Vec<f64>,
    colors: Vec<f64>,
    uvs: Vec<Vec<f64>>,
    material_index: Vec<Option<f64>>,
    vertex_weights: Vec<f64>,
    weights_indices: Vec<f64>,
}

/// getData(): the element's values for a polygon vertex. `data` is the
/// shared dataArray the original keeps between calls.
fn get_data<'a>(
    data: &'a mut Vec<Option<f64>>,
    polygon_vertex: usize,
    polygon: usize,
    vertex: f64,
    info: &Info,
) -> &'a [Option<f64>] {
    let mut index = match info.mapping.as_str() {
        "ByPolygonVertex" => Some(polygon_vertex as f64),
        "ByPolygon" => Some(polygon as f64),
        "ByVertice" => Some(vertex),
        "AllSame" => info.indices.first().copied(),
        _ => None,
    };
    if info.reference == "IndexToDirect" {
        index = index.and_then(|i| {
            if i >= 0.0 && i.fract() == 0.0 {
                info.indices.get(i as usize).copied()
            } else {
                None
            }
        });
    }
    if let Some(index) = index {
        let from = index * info.data_size as f64;
        let to = from + info.data_size as f64;
        let mut j = 0;
        let mut i = from;
        while i < to {
            let v = if i >= 0.0 && i.fract() == 0.0 {
                info.buffer.get(i as usize).copied()
            } else {
                None
            };
            if data.len() <= j {
                data.resize(j + 1, None);
            }
            data[j] = v;
            i += 1.0;
            j += 1;
        }
    }
    data
}

/// geometryMap: id key → geometry index ( `None` for an undefined entry ).
pub(super) type GeometryMap = HashMap<String, Option<usize>>;

/// GeometryParser.parse(): the geometries and geometryMap ( id key →
/// geometry, `None` for the undefined entries the original stores ).
pub(super) fn parse(ctx: &Ctx, deformers: &Deformers) -> Result<(Vec<Geo>, GeometryMap)> {
    let mut geos = vec![];
    let mut map = HashMap::new();
    if ctx.objects.has("Geometry") {
        let mut data = vec![];
        for (node_id, node) in ctx.nodes("Geometry") {
            let id = Value::Number(parse_int(&node_id));
            let relationships = ctx.connections.get(&id);
            let geo = match node.get("attrType").as_str() {
                Some("Mesh") => {
                    let r = relationships.ok_or_else(|| bad("geometry without connections"))?;
                    parse_mesh(ctx, r, &node, deformers, &mut data)?
                }
                Some("NurbsCurve") => Some(parse_nurbs(&node)?),
                _ => None,
            };
            let index = geo.map(|g| {
                geos.push(g);
                geos.len() - 1
            });
            map.insert(map_key(&id), index);
        }
    }
    Ok((geos, map))
}

fn parse_mesh(
    ctx: &Ctx,
    relationships: &Links,
    node: &Value,
    deformers: &Deformers,
    data: &mut Vec<Option<f64>>,
) -> Result<Option<Geo>> {
    let models = ctx.objects("Model");
    let model_nodes: Vec<Value> = relationships
        .parents
        .iter()
        .map(|p| models.get(&p.id.to_key()))
        .collect();
    if model_nodes.is_empty() {
        return Ok(None);
    }
    let mut skeleton = None;
    for child in &relationships.children {
        if let Some(s) = deformers.skeleton(&child.id) {
            skeleton = Some(s);
        }
    }
    let mut morph_targets = vec![];
    for child in &relationships.children {
        if let Some(m) = deformers.morph_target(&child.id) {
            morph_targets.push(m);
        }
    }
    let model = &model_nodes[0];
    if model.is_undefined() {
        return Err(bad(
            "Cannot use 'in' operator to search for 'RotationOrder' in undefined",
        ));
    }
    let mut t = TransformData::default();
    if model.has("RotationOrder") {
        t.euler_order = euler_order(&prop_value(model, "RotationOrder"));
    }
    if model.has("InheritType") {
        t.inherit_type = Some(parse_int_value(&prop_value(model, "InheritType")));
    }
    let raw = |name: &str| {
        model
            .has(name)
            .then(|| prop_value(model, name))
            .filter(Value::truthy)
            .map(|v| super::scene::numbers(&v))
    };
    t.translation = raw("GeometricTranslation");
    t.rotation = raw("GeometricRotation");
    t.scale = raw("GeometricScaling");
    let transform = generate_transform(&t);
    gen_geometry(
        ctx,
        node,
        skeleton,
        deformers,
        &morph_targets,
        &transform,
        data,
    )
    .map(Some)
}

fn layer(node: &Value, name: &str, index: usize) -> Value {
    node.get(name).get(&index.to_string())
}

fn gen_geometry(
    ctx: &Ctx,
    node: &Value,
    skeleton: Option<usize>,
    deformers: &Deformers,
    morph_targets: &[usize],
    pre: &M4,
    data: &mut Vec<Option<f64>>,
) -> Result<Geo> {
    let mut geo = Geo::default();
    let attr_name = node.get("attrName");
    if attr_name.truthy() {
        geo.name = Some(attr_name.to_key());
    }
    // parseGeoNode()
    let mut info = GeoInfo {
        vertex_positions: node.get("Vertices").get("a").to_f64s().unwrap_or_default(),
        vertex_indices: node
            .get("PolygonVertexIndex")
            .get("a")
            .to_f64s()
            .unwrap_or_default(),
        ..Default::default()
    };
    if node.get("Vertices").is_undefined() {
        info.vertex_positions = vec![];
    }
    let color_layer = layer(node, "LayerElementColor", 0);
    if node.get("LayerElementColor").truthy() && color_layer.get("Colors").truthy() {
        info.color = Some(vertex_colors(&color_layer));
    }
    if node.get("LayerElementMaterial").truthy() {
        let m = layer(node, "LayerElementMaterial", 0);
        if m.is_undefined() {
            return Err(bad("MaterialNode is undefined"));
        }
        info.material = Some(material_indices(&m));
    }
    if node.get("LayerElementNormal").truthy() {
        let n = layer(node, "LayerElementNormal", 0);
        if n.is_undefined() {
            return Err(bad("NormalNode is undefined"));
        }
        info.normal = Some(normals(&n));
    }
    if node.get("LayerElementUV").truthy() {
        let mut uvs = vec![];
        let mut i = 0;
        loop {
            let l = layer(node, "LayerElementUV", i);
            if !l.truthy() {
                break;
            }
            if l.get("UV").truthy() {
                uvs.push(uv_info(&l));
            }
            i += 1;
        }
        info.uv = Some(uvs);
    }
    if let Some(s) = skeleton {
        let mut table: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
        for (i, bone) in deformers.skeletons[s].1.raw_bones.iter().enumerate() {
            for (j, &index) in bone.indices.iter().enumerate() {
                table
                    .entry(number_key(index))
                    .or_default()
                    .push((i as f64, bone.weights.get(j).copied().unwrap_or(f64::NAN)));
            }
        }
        info.weights = Some(table);
    }
    let buffers = gen_buffers(&info, data);
    let positions: Vec<f32> = buffers.vertex.iter().map(|&v| v as f32).collect();
    geo.set_attribute(
        "position",
        Attr::F32 {
            data: apply_matrix4(&positions, pre),
            size: 3,
        },
    );
    if !buffers.colors.is_empty() {
        geo.set_attribute(
            "color",
            Attr::F32 {
                data: buffers.colors.iter().map(|&v| v as f32).collect(),
                size: 3,
            },
        );
    }
    if let Some(s) = skeleton {
        geo.set_attribute(
            "skinIndex",
            Attr::U16 {
                data: buffers
                    .weights_indices
                    .iter()
                    .map(|&v| to_uint16(v))
                    .collect(),
                size: 4,
            },
        );
        geo.set_attribute(
            "skinWeight",
            Attr::F32 {
                data: buffers.vertex_weights.iter().map(|&v| v as f32).collect(),
                size: 4,
            },
        );
        geo.deformer = Some(s);
    }
    if !buffers.normal.is_empty() {
        let nm = M3::normal_matrix(pre);
        let n: Vec<f32> = buffers.normal.iter().map(|&v| v as f32).collect();
        let mut out = Vec::with_capacity(n.len());
        for c in n.chunks(3) {
            let v = V3::new(
                f64::from(c[0]),
                f64::from(*c.get(1).unwrap_or(&f32::NAN)),
                f64::from(*c.get(2).unwrap_or(&f32::NAN)),
            )
            .apply_matrix3(&nm)
            .normalize();
            out.extend([v.x as f32, v.y as f32, v.z as f32]);
        }
        out.truncate(n.len() / 3 * 3);
        geo.set_attribute("normal", Attr::F32 { data: out, size: 3 });
    }
    for (i, uv) in buffers.uvs.iter().enumerate() {
        let name = if i == 0 {
            "uv".to_string()
        } else {
            format!("uv{i}")
        };
        geo.set_attribute(
            &name,
            Attr::F32 {
                data: uv.iter().map(|&v| v as f32).collect(),
                size: 2,
            },
        );
    }
    if let Some(m) = &info.material
        && m.mapping != "AllSame"
    {
        let indices = &buffers.material_index;
        let mut prev = indices.first().copied().flatten();
        let mut start = 0;
        for (i, &current) in indices.iter().enumerate() {
            if !same(current, prev) {
                geo.groups
                    .push((start as f64, (i - start) as f64, Some(prev.unwrap_or(0.0))));
                prev = current;
                start = i;
            }
        }
        if let Some(&(s, c, _)) = geo.groups.last() {
            let last = s + c;
            if last != indices.len() as f64 {
                geo.groups
                    .push((last, indices.len() as f64 - last, Some(prev.unwrap_or(0.0))));
            }
        }
        if geo.groups.is_empty() {
            geo.groups.push((
                0.0,
                indices.len() as f64,
                Some(indices.first().copied().flatten().unwrap_or(0.0)),
            ));
        }
    }
    add_morph_targets(ctx, &mut geo, node, morph_targets, deformers, pre, data)?;
    Ok(geo)
}

/// `a !== b` negated for possibly undefined numbers.
fn same(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn to_uint16(v: f64) -> u16 {
    if !v.is_finite() {
        return 0;
    }
    (v.trunc().rem_euclid(65536.0)) as u16
}

/// BufferAttribute.applyMatrix4() on Float32 positions.
fn apply_matrix4(data: &[f32], m: &M4) -> Vec<f32> {
    let mut out = Vec::with_capacity(data.len());
    for c in data.chunks(3) {
        if c.len() < 3 {
            break;
        }
        let v = V3::new(f64::from(c[0]), f64::from(c[1]), f64::from(c[2])).apply_matrix4(m);
        out.extend([v.x as f32, v.y as f32, v.z as f32]);
    }
    out
}

fn info(node: &Value, data: &str, index: Option<&str>, size: usize) -> Info {
    let reference = node.get("ReferenceInformationType").to_key();
    let indices = if reference == "IndexToDirect" {
        index
            .map(|i| node.get(i).get("a").to_f64s().unwrap_or_default())
            .unwrap_or_default()
    } else {
        vec![]
    };
    Info {
        data_size: size,
        buffer: node.get(data).get("a").to_f64s().unwrap_or_default(),
        indices,
        mapping: node.get("MappingInformationType").to_key(),
        reference,
    }
}

/// parseNormals().
fn normals(node: &Value) -> Info {
    let index = if node.has("NormalIndex") {
        Some("NormalIndex")
    } else if node.has("NormalsIndex") {
        Some("NormalsIndex")
    } else {
        None
    };
    info(node, "Normals", index, 3)
}

/// parseUVs().
fn uv_info(node: &Value) -> Info {
    info(node, "UV", Some("UVIndex"), 2)
}

/// parseVertexColors(): the colors converted to the working space in place.
fn vertex_colors(node: &Value) -> Info {
    if let Value::Numbers(a) = &node.get("Colors").get("a") {
        let mut b = a.borrow_mut();
        let mut k = 0;
        while k < b.len() {
            let g = |j: usize| b.get(k + j).copied().unwrap_or(f64::NAN);
            let c = srgb_color(&[g(0), g(1), g(2)]);
            for (j, v) in c.iter().enumerate() {
                if k + j < b.len() {
                    b[k + j] = *v;
                } else {
                    b.push(*v);
                }
            }
            k += 4;
        }
    }
    info(node, "Colors", Some("ColorIndex"), 4)
}

/// parseMaterialIndices().
fn material_indices(node: &Value) -> Info {
    let mapping = node.get("MappingInformationType").to_key();
    let reference = node.get("ReferenceInformationType").to_key();
    if mapping == "NoMappingInformation" {
        return Info {
            data_size: 1,
            buffer: vec![0.0],
            indices: vec![0.0],
            mapping: "AllSame".into(),
            reference,
        };
    }
    let buffer = node.get("Materials").get("a").to_f64s().unwrap_or_default();
    let n = buffer.len();
    Info {
        data_size: 1,
        buffer,
        indices: (0..n).map(|i| i as f64).collect(),
        mapping,
        reference,
    }
}

/// genBuffers().
fn gen_buffers(info: &GeoInfo, data: &mut Vec<Option<f64>>) -> Buffers {
    let mut b = Buffers::default();
    let mut polygon = 0;
    let mut face_length = 0;
    let mut face_positions: Vec<f64> = vec![];
    let mut face_normals: Vec<Option<f64>> = vec![];
    let mut face_colors: Vec<Option<f64>> = vec![];
    let mut face_uvs: Vec<Vec<Option<f64>>> = vec![];
    let mut face_weights: Vec<f64> = vec![];
    let mut face_weight_indices: Vec<f64> = vec![];
    let colors = info.color.as_ref();
    for (polygon_vertex, &raw) in info.vertex_indices.iter().enumerate() {
        let mut vertex = raw;
        let mut material_index = None;
        let mut end = false;
        if vertex < 0.0 {
            vertex = f64::from(to_int32(vertex) ^ -1);
            end = true;
        }
        face_positions.extend([vertex * 3.0, vertex * 3.0 + 1.0, vertex * 3.0 + 2.0]);
        if let Some(c) = colors {
            let d = get_data(data, polygon_vertex, polygon, vertex, c);
            face_colors.extend([d.first(), d.get(1), d.get(2)].map(|v| v.copied().flatten()));
        }
        if let Some(table) = &info.weights {
            let mut weights = vec![];
            let mut indices = vec![];
            if let Some(list) = table.get(&number_key(vertex)) {
                for &(id, w) in list {
                    weights.push(w);
                    indices.push(id);
                }
            }
            if weights.len() > 4 {
                let mut w_index = [0.0; 4];
                let mut weight = [0.0; 4];
                for (wi, &w) in weights.iter().enumerate() {
                    let mut current_weight = w;
                    let mut current_index = indices[wi];
                    for k in 0..4 {
                        if current_weight > weight[k] {
                            std::mem::swap(&mut weight[k], &mut current_weight);
                            std::mem::swap(&mut w_index[k], &mut current_index);
                        }
                    }
                }
                indices = w_index.to_vec();
                weights = weight.to_vec();
            }
            while weights.len() < 4 {
                weights.push(0.0);
                indices.push(0.0);
            }
            for i in 0..4 {
                face_weights.push(weights[i]);
                face_weight_indices.push(indices[i]);
            }
        }
        if let Some(n) = &info.normal {
            let d = get_data(data, polygon_vertex, polygon, vertex, n);
            face_normals.extend([d.first(), d.get(1), d.get(2)].map(|v| v.copied().flatten()));
        }
        if let Some(m) = &info.material
            && m.mapping != "AllSame"
        {
            let d = get_data(data, polygon_vertex, polygon, vertex, m);
            material_index = d.first().copied().flatten();
            if material_index.is_some_and(|v| v < 0.0) {
                material_index = Some(0.0);
            }
        }
        if let Some(uvs) = &info.uv {
            for (i, uv) in uvs.iter().enumerate() {
                let d = get_data(data, polygon_vertex, polygon, vertex, uv);
                let pair = [d.first(), d.get(1)].map(|v| v.copied().flatten());
                if face_uvs.len() <= i {
                    face_uvs.resize(i + 1, vec![]);
                }
                face_uvs[i].extend(pair);
            }
        }
        face_length += 1;
        if end {
            gen_face(
                &mut b,
                info,
                &face_positions,
                material_index,
                &face_normals,
                &face_colors,
                &face_uvs,
                &face_weights,
                &face_weight_indices,
                face_length,
            );
            polygon += 1;
            face_length = 0;
            face_positions.clear();
            face_normals.clear();
            face_colors.clear();
            face_uvs.clear();
            face_weights.clear();
            face_weight_indices.clear();
        }
    }
    b
}

/// ToInt32.
fn to_int32(v: f64) -> i32 {
    if !v.is_finite() {
        return 0;
    }
    (v.trunc().rem_euclid(4294967296.0) as u64 as u32) as i32
}

fn at(a: &[f64], i: f64) -> f64 {
    if i >= 0.0 && i.fract() == 0.0 {
        a.get(i as usize).copied().unwrap_or(f64::NAN)
    } else {
        f64::NAN
    }
}

/// getNormalNewell().
fn newell(vertices: &[V3]) -> V3 {
    let mut n = V3::ZERO;
    for i in 0..vertices.len() {
        let c = vertices[i];
        let next = vertices[(i + 1) % vertices.len()];
        n.x += (c.y - next.y) * (c.z + next.z);
        n.y += (c.z - next.z) * (c.x + next.x);
        n.z += (c.x - next.x) * (c.y + next.y);
    }
    n.normalize()
}

#[allow(clippy::too_many_arguments)]
fn gen_face(
    b: &mut Buffers,
    info: &GeoInfo,
    positions_index: &[f64],
    material_index: Option<f64>,
    normals: &[Option<f64>],
    colors: &[Option<f64>],
    uvs: &[Vec<Option<f64>>],
    weights: &[f64],
    weight_indices: &[f64],
    face_length: usize,
) {
    let triangles: Vec<[usize; 3]> = if face_length > 3 {
        let positions = info
            .base_positions
            .as_ref()
            .unwrap_or(&info.vertex_positions);
        let vertices: Vec<V3> = positions_index
            .chunks(3)
            .map(|c| {
                V3::new(
                    at(positions, c[0]),
                    at(positions, c[1]),
                    at(positions, c[2]),
                )
            })
            .collect();
        let normal = newell(&vertices);
        let up = if normal.z.abs() > 0.5 {
            V3::new(0.0, 1.0, 0.0)
        } else {
            V3::new(0.0, 0.0, 1.0)
        };
        let tangent = V3::cross(up, normal).normalize();
        let bitangent = V3::cross(normal, tangent).normalize();
        let mut contour: Vec<[f64; 2]> = vertices
            .iter()
            .map(|v| [v.dot(tangent), v.dot(bitangent)])
            .collect();
        shape_utils::triangulate(&mut contour, &mut [])
    } else {
        vec![[0, 1, 2]]
    };
    let opt = |a: &[Option<f64>], i: usize| a.get(i).copied().flatten().unwrap_or(f64::NAN);
    let num = |a: &[f64], i: usize| a.get(i).copied().unwrap_or(f64::NAN);
    for t in triangles {
        for &i in &t {
            for k in 0..3 {
                b.vertex
                    .push(at(&info.vertex_positions, num(positions_index, i * 3 + k)));
            }
        }
        if info.weights.is_some() {
            for &i in &t {
                for k in 0..4 {
                    b.vertex_weights.push(num(weights, i * 4 + k));
                }
            }
            for &i in &t {
                for k in 0..4 {
                    b.weights_indices.push(num(weight_indices, i * 4 + k));
                }
            }
        }
        if info.color.is_some() {
            for &i in &t {
                for k in 0..3 {
                    b.colors.push(opt(colors, i * 3 + k));
                }
            }
        }
        if let Some(m) = &info.material
            && m.mapping != "AllSame"
        {
            b.material_index.extend([material_index; 3]);
        }
        if info.normal.is_some() {
            for &i in &t {
                for k in 0..3 {
                    b.normal.push(opt(normals, i * 3 + k));
                }
            }
        }
        if let Some(layers) = &info.uv {
            for j in 0..layers.len() {
                if b.uvs.len() <= j {
                    b.uvs.resize(j + 1, vec![]);
                }
                let face = uvs.get(j).map_or(&[][..], |v| v.as_slice());
                for &i in &t {
                    b.uvs[j].push(opt(face, i * 2));
                    b.uvs[j].push(opt(face, i * 2 + 1));
                }
            }
        }
    }
}

/// addMorphTargets().
fn add_morph_targets(
    ctx: &Ctx,
    geo: &mut Geo,
    parent_node: &Value,
    morph_targets: &[usize],
    deformers: &Deformers,
    pre: &M4,
    data: &mut Vec<Option<f64>>,
) -> Result<()> {
    if morph_targets.is_empty() {
        return Ok(());
    }
    geo.morph_relative = true;
    let morph_pre = pre.set_position(V3::ZERO);
    let geometries = ctx.objects("Geometry");
    for &m in morph_targets {
        let raw = deformers.morph_targets[m]
            .1
            .raw_targets
            .as_ref()
            .ok_or_else(|| bad("morphTarget.rawTargets is undefined"))?;
        for target in raw {
            let morph_node = geometries.get(&target.geo_id.to_key());
            if morph_node.is_undefined() {
                continue;
            }
            // genMorphGeometry()
            let base_positions = parent_node
                .get("Vertices")
                .get("a")
                .to_f64s()
                .unwrap_or_default();
            let base_indices = parent_node
                .get("PolygonVertexIndex")
                .get("a")
                .to_f64s()
                .unwrap_or_default();
            let sparse = morph_node
                .get("Vertices")
                .get("a")
                .to_f64s()
                .unwrap_or_default();
            let morph_indices = morph_node
                .get("Indexes")
                .get("a")
                .to_f64s()
                .unwrap_or_default();
            let length = geo.count() * 3;
            let mut morph_positions = vec![0f32; length];
            for (i, &mi) in morph_indices.iter().enumerate() {
                let index = mi * 3.0;
                for k in 0..3 {
                    let at = index + k as f64;
                    if at >= 0.0 && at.fract() == 0.0 && (at as usize) < length {
                        morph_positions[at as usize] =
                            sparse.get(i * 3 + k).copied().unwrap_or(f64::NAN) as f32;
                    }
                }
            }
            let morph_info = GeoInfo {
                vertex_indices: base_indices,
                vertex_positions: morph_positions.iter().map(|&v| f64::from(v)).collect(),
                base_positions: Some(base_positions),
                ..Default::default()
            };
            let buffers = gen_buffers(&morph_info, data);
            let positions: Vec<f32> = buffers.vertex.iter().map(|&v| v as f32).collect();
            let name = if target.name.truthy() {
                target.name.clone()
            } else {
                morph_node.get("attrName")
            };
            let name = (!name.is_undefined()).then(|| name.to_key());
            geo.morph_positions
                .push((name, apply_matrix4(&positions, &morph_pre)));
        }
    }
    Ok(())
}

/// parseNurbsGeometry(): the curve sampled at 12 points per control point.
fn parse_nurbs(node: &Value) -> Result<Geo> {
    let order = parse_int_value(&node.get("Order"));
    let mut geo = Geo::default();
    if order.is_nan() {
        return Ok(geo);
    }
    let degree = order - 1.0;
    let knots = node
        .get("KnotVector")
        .get("a")
        .to_f64s()
        .unwrap_or_default();
    let values = node.get("Points").get("a").to_f64s().unwrap_or_default();
    let mut points: Vec<[f64; 4]> = vec![];
    let mut i = 0;
    while i < values.len() {
        let g = |j: usize| values.get(i + j).copied().unwrap_or(f64::NAN);
        points.push([g(0), g(1), g(2), g(3)]);
        i += 4;
    }
    let (mut start, mut end) = (None, None);
    let form = node.get("Form");
    if form.strict_eq(&Value::string("Closed")) {
        if let Some(&p) = points.first() {
            points.push(p);
        }
    } else if form.strict_eq(&Value::string("Periodic")) {
        start = Some(degree);
        end = Some(knots.len() as f64 - 1.0 - degree);
        for i in 0..degree.max(0.0) as usize {
            if let Some(&p) = points.get(i) {
                points.push(p);
            }
        }
    }
    let curve = NurbsCurve::new(degree, knots, points, start, end);
    let divisions = curve.points.len() * 12;
    let mut position = Vec::with_capacity((divisions + 1) * 3);
    for d in 0..=divisions {
        let p = curve.point(d as f64 / divisions as f64);
        let z = if p.z == 0.0 || p.z.is_nan() { 0.0 } else { p.z };
        position.extend([p.x as f32, p.y as f32, z as f32]);
    }
    geo.set_attribute(
        "position",
        Attr::F32 {
            data: position,
            size: 3,
        },
    );
    Ok(geo)
}

/// NURBSCurve ( examples/jsm/curves ) with NURBSUtils' point evaluation.
struct NurbsCurve {
    degree: f64,
    knots: Vec<f64>,
    points: Vec<[f64; 4]>,
    start: usize,
    end: usize,
}
impl NurbsCurve {
    fn new(
        degree: f64,
        knots: Vec<f64>,
        points: Vec<[f64; 4]>,
        start: Option<f64>,
        end: Option<f64>,
    ) -> Self {
        let knots_length = knots.len().saturating_sub(1);
        let index = |v: Option<f64>, default: usize| match v {
            Some(v) if v != 0.0 && !v.is_nan() && v > 0.0 => v as usize,
            _ => default,
        };
        Self {
            degree,
            start: index(start, 0),
            end: index(end, knots_length),
            knots,
            points,
        }
    }
    fn knot(&self, i: usize) -> f64 {
        self.knots.get(i).copied().unwrap_or(f64::NAN)
    }
    fn point(&self, t: f64) -> V3 {
        let u = self.knot(self.start) + t * (self.knot(self.end) - self.knot(self.start));
        let h = self.bspline_point(u);
        let h = if h[3] != 1.0 {
            let s = 1.0 / h[3];
            [h[0] * s, h[1] * s, h[2] * s, h[3] * s]
        } else {
            h
        };
        V3::new(h[0], h[1], h[2])
    }
    /// findSpan()
    fn find_span(&self, u: f64) -> usize {
        let p = self.degree as usize;
        let n = self.knots.len() - p - 1;
        if u >= self.knot(n) {
            return n - 1;
        }
        if u <= self.knot(p) {
            return p;
        }
        let (mut low, mut high) = (p, n);
        let mut mid = (low + high) / 2;
        while u < self.knot(mid) || u >= self.knot(mid + 1) {
            if u < self.knot(mid) {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }
    /// calcBSplinePoint() with calcBasisFunctions().
    fn bspline_point(&self, u: f64) -> [f64; 4] {
        let p = self.degree as usize;
        let span = self.find_span(u);
        let mut n = vec![0.0; p + 1];
        let mut left = vec![0.0; p + 1];
        let mut right = vec![0.0; p + 1];
        n[0] = 1.0;
        for j in 1..=p {
            left[j] = u - self.knot(span + 1 - j);
            right[j] = self.knot(span + j) - u;
            let mut saved = 0.0;
            for r in 0..j {
                let rv = right[r + 1];
                let lv = left[j - r];
                let temp = n[r] / (rv + lv);
                n[r] = saved + rv * temp;
                saved = lv * temp;
            }
            n[j] = saved;
        }
        let mut c = [0.0; 4];
        for (j, nj) in n.iter().enumerate() {
            let point = self
                .points
                .get(span - p + j)
                .copied()
                .unwrap_or([f64::NAN; 4]);
            let w = point[3] * nj;
            c[0] += point[0] * w;
            c[1] += point[1] * w;
            c[2] += point[2] * w;
            c[3] += point[3] * nj;
        }
        c
    }
}
