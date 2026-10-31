//! GPU-resident geometry, invalidated by attribute versions or a new Arc allocation.
use crate::{
    Error, Result,
    geometry::{Attribute, BufferGeometry},
    math::*,
    renderer::Vertex,
};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
/// A vertex attribute of the renderer's shaders: its location, format and
/// its offset in a full `Vertex` record ( the default values' layout ).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Field {
    Position,
    Normal,
    Uv,
    Color,
    Corner,
    Tangent,
    Uv1,
    Uv2,
}
impl Field {
    pub const ALL: [Field; 8] = [
        Field::Position,
        Field::Normal,
        Field::Uv,
        Field::Color,
        Field::Corner,
        Field::Tangent,
        Field::Uv1,
        Field::Uv2,
    ];
    /// The bit of `Buffers::attributes` ( position is always present ).
    pub fn bit(self) -> u8 {
        1 << (self as u8)
    }
    pub fn floats(self) -> usize {
        match self {
            Field::Position | Field::Normal => 3,
            Field::Color | Field::Tangent => 4,
            _ => 2,
        }
    }
    pub fn format(self) -> wgpu::VertexFormat {
        match self.floats() {
            2 => wgpu::VertexFormat::Float32x2,
            3 => wgpu::VertexFormat::Float32x3,
            _ => wgpu::VertexFormat::Float32x4,
        }
    }
    /// Its offset in a full `Vertex`.
    pub fn full_offset(self) -> u64 {
        Field::ALL
            .iter()
            .take_while(|&&f| f != self)
            .map(|f| f.floats() as u64 * 4)
            .sum()
    }
}

/// A geometry's vertex layout: only the attributes it has, packed in `Field`
/// order, as three uploads only a geometry's own attributes.
pub(crate) fn packed_stride(attributes: u8) -> u64 {
    Field::ALL
        .iter()
        .filter(|f| attributes & f.bit() != 0)
        .map(|f| f.floats() as u64 * 4)
        .sum()
}
pub(crate) fn packed_offset(attributes: u8, field: Field) -> Option<u64> {
    (attributes & field.bit() != 0).then(|| {
        Field::ALL
            .iter()
            .take_while(|&&f| f != field)
            .filter(|f| attributes & f.bit() != 0)
            .map(|f| f.floats() as u64 * 4)
            .sum()
    })
}
/// The packed buffer's attributes a pipeline reads ( field, location ).
pub(crate) fn vertex_attributes(
    attributes: u8,
    wanted: &[(Field, u32)],
) -> Vec<wgpu::VertexAttribute> {
    wanted
        .iter()
        .filter_map(|&(field, location)| {
            packed_offset(attributes, field).map(|offset| wgpu::VertexAttribute {
                format: field.format(),
                offset,
                shader_location: location,
            })
        })
        .collect()
}
impl Field {
    /// The value a shader reads for a missing attribute: normal +Z, white
    /// vertex color, zero elsewhere.
    fn default_wgsl(self) -> &'static str {
        match self {
            Field::Normal => "vec3<f32>(0.0,0.0,1.0)",
            Field::Color => "vec4<f32>(1.0)",
            Field::Tangent => "vec4<f32>(0.0)",
            Field::Position => "vec3<f32>(0.0)",
            _ => "vec2<f32>(0.0)",
        }
    }
}
/// The `Field` bits of the optional attributes a vertex shader reads.
fn optional_mask(inputs: &[(Field, u32)]) -> u8 {
    inputs
        .iter()
        .filter(|(f, _)| *f != Field::Position)
        .fold(0, |m, (f, _)| m | f.bit())
}
/// The vertex entry point for a geometry with `attributes`.
pub(crate) fn entry_point(attributes: u8, inputs: &[(Field, u32)]) -> String {
    format!("vs_main_{}", attributes & optional_mask(inputs))
}
/// A vertex shader with one entry point per set of attributes: its
/// `@vertex fn vs_main( … )` becomes `vs_main_body`, and each `vs_main_<bits>`
/// takes the attributes of `inputs` the set holds and passes the others'
/// defaults. Pipelines then bind only a geometry's own attributes.
pub(crate) fn vertex_entry_points(source: &str, inputs: &[(Field, u32)]) -> String {
    const HEAD: &str = "@vertex fn vs_main(";
    let Some(start) = source.find(HEAD) else {
        return source.to_owned();
    };
    let open = start + HEAD.len();
    let mut depth = 1;
    let mut close = open;
    for (i, c) in source[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close = open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    let params_text = &source[open..close];
    let mut params = Vec::new();
    let mut depth = 0;
    let mut begin = 0;
    for (i, c) in params_text.char_indices() {
        match c {
            '(' | '<' => depth += 1,
            ')' | '>' => depth -= 1,
            ',' if depth == 0 => {
                params.push(params_text[begin..i].trim());
                begin = i + 1;
            }
            _ => {}
        }
    }
    if !params_text[begin..].trim().is_empty() {
        params.push(params_text[begin..].trim());
    }
    // ( attributes, name: type, name, the field of a location input )
    let parsed: Vec<(String, String, String, Option<Field>)> = params
        .iter()
        .map(|p| {
            let mut rest = *p;
            let mut attrs = String::new();
            while let Some(r) = rest.strip_prefix('@') {
                let end = r
                    .find(|c: char| c == ')' || c.is_whitespace())
                    .map(|e| if r.as_bytes()[e] == b')' { e + 1 } else { e })
                    .unwrap_or(r.len());
                attrs.push('@');
                attrs.push_str(&r[..end]);
                attrs.push(' ');
                rest = r[end..].trim_start();
            }
            let name = rest.split(':').next().unwrap_or("").trim().to_owned();
            let field = attrs
                .split("@location(")
                .nth(1)
                .and_then(|l| l.split(')').next())
                .and_then(|l| l.trim().parse::<u32>().ok())
                .and_then(|location| inputs.iter().find(|(_, l)| *l == location).map(|(f, _)| *f));
            (attrs, rest.to_owned(), name, field)
        })
        .collect();
    let after = &source[close + 1..];
    let body_start = after.find('{').unwrap_or(0);
    let signature_tail = &after[..body_start];
    let mut out = String::with_capacity(source.len() + 32 * 1024);
    out.push_str(&source[..start]);
    out.push_str("fn vs_main_body(");
    out.push_str(
        &parsed
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push(')');
    out.push_str(after);
    let optional = optional_mask(inputs);
    let mut mask: u8 = 0;
    loop {
        let mut wrapper_params = Vec::new();
        let mut args = Vec::new();
        for (attrs, typed, name, field) in &parsed {
            match field {
                Some(f) if *f != Field::Position && mask & f.bit() == 0 => {
                    args.push(f.default_wgsl().to_owned())
                }
                _ => {
                    wrapper_params.push(format!("{attrs}{typed}"));
                    args.push(name.clone());
                }
            }
        }
        out.push_str(&format!(
            "\n@vertex fn vs_main_{mask}({}){signature_tail}{{return vs_main_body({});}}",
            wrapper_params.join(","),
            args.join(",")
        ));
        // The next subset of the optional attributes.
        if mask == optional {
            break;
        }
        mask = (mask.wrapping_sub(optional)) & optional;
    }
    out
}

#[derive(Clone)]
pub(crate) struct Buffers {
    pub vertices: wgpu::Buffer,
    /// The attributes rewritten since the first upload ( `dynamic_attributes`
    /// bits ), packed apart in vertex buffer slot 2, so a frame that changes
    /// them writes only them, as WebGL's per-attribute bufferSubData does.
    pub dynamic: Option<wgpu::Buffer>,
    pub indices: Option<wgpu::Buffer>,
    /// `Field` bits of the attributes `vertices` and `dynamic` hold together.
    pub attributes: u8,
    /// `Field` bits of the attributes `dynamic` holds ( none of `vertices`' ).
    pub dynamic_attributes: u8,
    pub index_format: wgpu::IndexFormat,
}
/// The vertex buffer layouts of a geometry's `attributes` that a pipeline
/// reads through `inputs`: slot 0 packs those outside `dynamic`, slot 2 the
/// `dynamic` ones ( slot 1 is the instance buffer ).
pub(crate) struct VertexLayouts {
    main: (u64, Vec<wgpu::VertexAttribute>),
    dynamic: Option<(u64, Vec<wgpu::VertexAttribute>)>,
}
impl VertexLayouts {
    pub fn new(attributes: u8, dynamic: u8, inputs: &[(Field, u32)]) -> Self {
        let main = attributes & !dynamic;
        Self {
            main: (packed_stride(main), vertex_attributes(main, inputs)),
            dynamic: (dynamic != 0)
                .then(|| (packed_stride(dynamic), vertex_attributes(dynamic, inputs))),
        }
    }
    /// The pipeline's vertex buffers, with `instance` in slot 1.
    pub fn buffers<'a>(
        &'a self,
        instance: wgpu::VertexBufferLayout<'a>,
    ) -> Vec<wgpu::VertexBufferLayout<'a>> {
        let layout = |(stride, attributes): &'a (u64, Vec<wgpu::VertexAttribute>)| {
            wgpu::VertexBufferLayout {
                array_stride: *stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            }
        };
        let mut buffers = vec![layout(&self.main), instance];
        buffers.extend(self.dynamic.as_ref().map(layout));
        buffers
    }
}
impl Buffers {
    /// Bind the geometry's vertex buffers ( slot 1, the instances, is the caller's ).
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        if let Some(dynamic) = &self.dynamic {
            pass.set_vertex_buffer(2, dynamic.slice(..));
        }
    }
}
struct Entry {
    owner: Weak<BufferGeometry>,
    versions: Vec<(u64, usize)>,
    buffers: Buffers,
    /// The owner was dropped at the last prune: a geometry moved to a new
    /// allocation (Arc::make_mut with weak references) reclaims its entry by
    /// identity before the next prune frees it.
    orphaned: u8,
    /// Each `Field`'s source attribute ( version, storage ) at the upload.
    fields: [Option<(u64, usize)>; 8],
    /// `Field` bits whose attribute changed since the first upload.
    rewritten: u8,
    vertex_count: usize,
    /// The index `indices` was built from, kept once the geometry changes
    /// after its first upload: an unchanged index is not written again.
    index_source: Option<Option<Vec<u32>>>,
}
struct Bounds {
    owner: Weak<BufferGeometry>,
    versions: Vec<u64>,
    sphere: Sphere,
}
#[derive(Default)]
pub(crate) struct Cache {
    entries: HashMap<(usize, bool, bool, bool, Option<bool>), Entry>,
    bounds: HashMap<usize, Bounds>,
    pub uploads: u64,
    pub bytes: u64,
}
/// The attribute a `Field` reads on a plain draw ( neither points nor wide lines ).
fn source(geometry: &BufferGeometry, field: Field, vertex_colors: bool) -> Option<&Attribute> {
    let name = match field {
        Field::Position => "position",
        Field::Normal => "normal",
        Field::Uv => "uv",
        Field::Color if vertex_colors => "color",
        Field::Color => return None,
        Field::Corner => "lineDistance",
        Field::Tangent => "tangent",
        Field::Uv1 => "uv1",
        Field::Uv2 => "uv2",
    };
    geometry.attributes.get(name)
}
/// The `mask` fields of a plain draw's vertices, packed in `Field` order.
fn pack_plain(geometry: &BufferGeometry, mask: u8, vertex_colors: bool) -> Result<Vec<f32>> {
    let sources = Field::ALL
        .into_iter()
        .filter(|f| mask & f.bit() != 0)
        .map(|f| {
            source(geometry, f, vertex_colors)
                .map(|a| (f, a))
                .ok_or(Error::Invalid("vertex attribute"))
        })
        .collect::<Result<Vec<_>>>()?;
    let count = geometry.vertex_count();
    let mut out = Vec::with_capacity(count * packed_stride(mask) as usize / 4);
    for index in 0..count {
        for &(field, a) in &sources {
            match field {
                Field::Position | Field::Normal => {
                    out.extend_from_slice(&a.vector3(index)?.as_vec3().to_array());
                }
                Field::Uv | Field::Uv1 | Field::Uv2 => {
                    out.push(a.get_component(index, 0)? as f32);
                    out.push(a.get_component(index, 1)? as f32);
                }
                Field::Color => {
                    let mut color = [1.0; 4];
                    for (c, value) in color.iter_mut().enumerate().take(a.item_size().min(4)) {
                        *value = a.get_component(index, c)? as f32;
                    }
                    out.extend_from_slice(&color);
                }
                // Native one-pixel dashed lines carry the precomputed distance
                // in the otherwise unused expansion coordinate.
                Field::Corner => {
                    out.push(a.get_component(index, 0)? as f32);
                    out.push(0.0);
                }
                Field::Tangent => {
                    for c in 0..4 {
                        out.push(a.get_component(index, c)? as f32);
                    }
                }
            }
        }
    }
    Ok(out)
}
/// Points ( six corners a vertex ) and wide lines ( six corners a segment ),
/// packed whole, and their `Field` bits.
fn pack_expanded(
    geometry: &BufferGeometry,
    vertex_colors: bool,
    is_points: bool,
    wide: Option<bool>,
) -> Result<(Vec<f32>, u8)> {
    let mut vertices = Vec::new();
    let positions = geometry
        .attributes
        .get("position")
        .ok_or(Error::Invalid("position attribute"))?;
    for index in 0..geometry.vertex_count() {
        let position = positions.vector3(index)?.as_vec3().to_array();
        let normal = geometry
            .attributes
            .get("normal")
            .map(|a| a.vector3(index))
            .transpose()?
            .unwrap_or(Vector3::Z)
            .as_vec3()
            .to_array();
        let mut uv = [0.0; 2];
        if let Some(a) = geometry.attributes.get("uv") {
            uv = [
                a.get_component(index, 0)? as f32,
                a.get_component(index, 1)? as f32,
            ];
        }
        let uv1 = geometry
            .attributes
            .get("uv1")
            .map(|a| {
                Ok::<_, Error>([
                    a.get_component(index, 0)? as f32,
                    a.get_component(index, 1)? as f32,
                ])
            })
            .transpose()?
            .unwrap_or([0.0; 2]);
        let uv2 = geometry
            .attributes
            .get("uv2")
            .map(|a| {
                Ok::<_, Error>([
                    a.get_component(index, 0)? as f32,
                    a.get_component(index, 1)? as f32,
                ])
            })
            .transpose()?
            .unwrap_or([0.0; 2]);
        let mut color = [1.0; 4];
        if vertex_colors && let Some(a) = geometry.attributes.get("color") {
            for (c, value) in color.iter_mut().enumerate().take(a.item_size().min(4)) {
                *value = a.get_component(index, c)? as f32;
            }
        }
        let mut tangent = [0.0; 4];
        if let Some(a) = geometry.attributes.get("tangent") {
            for (c, value) in tangent.iter_mut().enumerate() {
                *value = a.get_component(index, c)? as f32;
            }
        }
        vertices.push(Vertex {
            position,
            normal,
            uv,
            color,
            corner: [
                geometry
                    .attributes
                    .get("lineDistance")
                    .map(|a| a.get_component(index, 0))
                    .transpose()?
                    .unwrap_or(0.0) as f32,
                0.0,
            ],
            tangent,
            uv1,
            uv2,
        });
    }
    if is_points {
        let centers = vertices;
        vertices = Vec::with_capacity(centers.len() * 6);
        for center in centers {
            for corner in [
                [-1.0, -1.0],
                [1.0, -1.0],
                [-1.0, 1.0],
                [-1.0, 1.0],
                [1.0, -1.0],
                [1.0, 1.0],
            ] {
                let mut vertex = center;
                vertex.corner = corner;
                if !geometry.has_attribute("uv") {
                    // Three uses (PointCoord.x, 1 - PointCoord.y): UVs
                    // increase upwards before the texture's flipY transform.
                    vertex.uv = [corner[0] * 0.5 + 0.5, corner[1] * 0.5 + 0.5];
                }
                vertices.push(vertex);
            }
        }
    }
    if let Some(segments) = wide {
        let source = vertices;
        vertices = Vec::new();
        let mut distance = 0.0f32;
        let step = if segments { 2 } else { 1 };
        for i in (0..geometry.draw_count().saturating_sub(1)).step_by(step) {
            let a = geometry.vertex_index(i)?;
            let b = geometry.vertex_index(i + 1)?;
            let p = source[a];
            let q = source[b];
            let da = geometry
                .attributes
                .get("lineDistance")
                .map(|d| d.get_component(a, 0).map(|v| v as f32))
                .transpose()?
                .unwrap_or(distance);
            distance +=
                (glam::Vec3::from_array(q.position) - glam::Vec3::from_array(p.position)).length();
            let db = geometry
                .attributes
                .get("lineDistance")
                .map(|d| d.get_component(b, 0).map(|v| v as f32))
                .transpose()?
                .unwrap_or(distance);
            for corner in [
                [0.0, -1.0],
                [1.0, -1.0],
                [0.0, 1.0],
                [0.0, 1.0],
                [1.0, -1.0],
                [1.0, 1.0],
            ] {
                let mut v = if corner[0] == 0.0 { p } else { q };
                v.position = p.position;
                v.normal = q.position;
                v.corner = corner;
                v.tangent = [da, db, a as f32, b as f32];
                vertices.push(v);
            }
        }
    }
    // The attributes this geometry has: three uploads only those.
    let has = |name: &str| geometry.attributes.contains_key(name);
    let mut attributes = Field::Position.bit();
    for (field, present) in [
        (Field::Normal, has("normal") || wide.is_some()),
        (Field::Uv, has("uv") || is_points),
        (Field::Color, vertex_colors && has("color")),
        (
            Field::Corner,
            has("lineDistance") || is_points || wide.is_some(),
        ),
        (Field::Tangent, has("tangent") || wide.is_some()),
        (Field::Uv1, has("uv1")),
        (Field::Uv2, has("uv2")),
    ] {
        if present {
            attributes |= field.bit();
        }
    }
    let full: &[f32] = bytemuck::cast_slice(&vertices);
    let per = std::mem::size_of::<Vertex>() / 4;
    let mut packed = Vec::with_capacity(vertices.len() * packed_stride(attributes) as usize / 4);
    for record in full.chunks_exact(per) {
        for field in Field::ALL {
            if attributes & field.bit() != 0 {
                let o = field.full_offset() as usize / 4;
                packed.extend_from_slice(&record[o..o + field.floats()]);
            }
        }
    }
    Ok((packed, attributes))
}
/// The draw's indices: wireframe edges, six corners a point, none for wide lines.
fn build_indices(
    geometry: &BufferGeometry,
    is_points: bool,
    wireframe: bool,
    wide: Option<bool>,
) -> Result<Option<Vec<u32>>> {
    Ok(if wide.is_some() {
        None
    } else if wireframe {
        let mut indices = Vec::with_capacity(geometry.draw_count() * 2);
        for i in (0..geometry.draw_count().saturating_sub(2)).step_by(3) {
            let a = geometry.vertex_index(i)? as u32;
            let b = geometry.vertex_index(i + 1)? as u32;
            let c = geometry.vertex_index(i + 2)? as u32;
            indices.extend([a, b, b, c, c, a]);
        }
        Some(indices)
    } else if let Some(indices) = &geometry.index {
        if indices
            .iter()
            .any(|&i| i as usize >= geometry.vertex_count())
        {
            return Err(Error::Invalid("geometry index"));
        }
        Some(if is_points {
            indices
                .iter()
                .flat_map(|i| (0..6).map(move |corner| i * 6 + corner))
                .collect()
        } else {
            indices.clone()
        })
    } else {
        None
    })
}
/// Write `bytes` into `previous` when there is one ( growing it if needed ),
/// else into a new buffer.
fn store(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    previous: Option<&wgpu::Buffer>,
    label: &str,
    bytes: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    match previous {
        Some(buffer) => rewrite(device, queue, buffer, label, bytes, usage),
        None => upload(device, queue, label, bytes, usage),
    }
}
impl Cache {
    pub fn prune(&mut self) {
        self.entries.retain(|_, e| {
            if e.owner.strong_count() > 0 {
                e.orphaned = 0;
                true
            } else {
                // Kept for a few prunes (several can run per frame).
                e.orphaned += 1;
                e.orphaned <= 4
            }
        });
        self.bounds.retain(|_, e| e.owner.strong_count() > 0);
    }
    pub fn sphere(&mut self, geometry: &Arc<BufferGeometry>) -> Result<Sphere> {
        if let Some(sphere) = geometry.bounding_sphere {
            return Ok(sphere);
        }
        let key = Arc::as_ptr(geometry) as usize;
        let versions = geometry
            .attributes
            .get("position")
            .into_iter()
            .chain(
                geometry
                    .morph_attributes
                    .get("position")
                    .into_iter()
                    .flatten(),
            )
            .map(|a| a.version())
            .collect::<Vec<_>>();
        if let Some(bounds) = self.bounds.get(&key)
            && bounds.owner.strong_count() > 0
            && bounds.versions == versions
        {
            return Ok(bounds.sphere);
        }
        let sphere = (**geometry).clone().compute_bounding_sphere()?;
        self.bounds.insert(
            key,
            Bounds {
                owner: Arc::downgrade(geometry),
                versions,
                sphere,
            },
        );
        Ok(sphere)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        geometry: &Arc<BufferGeometry>,
        vertex_colors: bool,
        is_points: bool,
        wireframe: bool,
        wide: Option<bool>,
    ) -> Result<Buffers> {
        // Keyed by the geometry's identity, which survives a move to a new
        // allocation; a moved geometry re-attaches its entry.
        let key = (
            geometry.identity.id as usize,
            vertex_colors,
            is_points,
            wireframe,
            wide,
        );
        if self
            .entries
            .get(&key)
            .is_some_and(|e| e.owner.strong_count() == 0)
        {
            // Every variant of the moved geometry follows it.
            for (k, entry) in &mut self.entries {
                if k.0 == key.0 && entry.owner.strong_count() == 0 {
                    entry.owner = Arc::downgrade(geometry);
                    entry.orphaned = 0;
                }
            }
        }
        let versions = geometry
            .attributes
            .values()
            .map(|a| (a.version(), a.storage_key()))
            .collect::<Vec<_>>();
        if let Some(entry) = self.entries.get(&key)
            && entry.owner.strong_count() > 0
            && entry.versions == versions
        {
            return Ok(entry.buffers.clone());
        }
        if !geometry.attributes.contains_key("position") {
            return Err(Error::Invalid("position attribute"));
        }
        let vertex_count = geometry.vertex_count();
        // A live geometry whose attributes changed writes into its resident
        // buffers, as WebGL's bufferSubData does; capacity grows only when the
        // new data does not fit.
        let previous = self
            .entries
            .get(&key)
            .filter(|e| e.owner.strong_count() > 0);
        let plain = !is_points && wide.is_none();
        let fields = Field::ALL
            .map(|f| source(geometry, f, vertex_colors).map(|a| (a.version(), a.storage_key())));
        let mut written = 0u64;
        let (vertices, dynamic, attributes, dynamic_attributes, rewritten) = if plain {
            // three uploads each attribute on its own, and only the changed
            // ones: an attribute rewritten after the first upload moves to
            // the dynamic buffer, which a frame writes without the others.
            let attributes = Field::ALL
                .into_iter()
                .zip(&fields)
                .filter(|(_, s)| s.is_some())
                .fold(0, |m, (f, _)| m | f.bit());
            let changed = previous.map_or(0, |e| {
                Field::ALL
                    .into_iter()
                    .zip(fields.iter().zip(&e.fields))
                    .filter(|(_, (a, b))| a != b)
                    .fold(0, |m, (f, _)| m | f.bit())
            });
            let rewritten = previous.map_or(0, |e| e.rewritten | changed) & attributes;
            // When every attribute changes, one buffer holds them all.
            let split = if rewritten == attributes {
                0
            } else {
                rewritten
            };
            let rebuild = previous.is_none_or(|e| {
                e.vertex_count != vertex_count
                    || e.buffers.attributes != attributes
                    || e.buffers.dynamic_attributes != split
            });
            let main = attributes & !split;
            let vertices = if rebuild || changed & main != 0 {
                let packed = pack_plain(geometry, main, vertex_colors)?;
                let bytes: &[u8] = bytemuck::cast_slice(&packed);
                written += bytes.len() as u64;
                store(
                    device,
                    queue,
                    previous.map(|e| &e.buffers.vertices),
                    "resident vertices",
                    bytes,
                    wgpu::BufferUsages::VERTEX,
                )
            } else {
                previous
                    .map(|e| e.buffers.vertices.clone())
                    .ok_or(Error::Invalid("resident vertices"))?
            };
            let dynamic = if split == 0 {
                None
            } else if rebuild || changed & split != 0 {
                let packed = pack_plain(geometry, split, vertex_colors)?;
                let bytes: &[u8] = bytemuck::cast_slice(&packed);
                written += bytes.len() as u64;
                Some(store(
                    device,
                    queue,
                    previous.and_then(|e| e.buffers.dynamic.as_ref()),
                    "dynamic vertices",
                    bytes,
                    wgpu::BufferUsages::VERTEX,
                ))
            } else {
                previous.and_then(|e| e.buffers.dynamic.clone())
            };
            (vertices, dynamic, attributes, split, rewritten)
        } else {
            let (packed, attributes) = pack_expanded(geometry, vertex_colors, is_points, wide)?;
            let bytes: &[u8] = bytemuck::cast_slice(&packed);
            written += bytes.len() as u64;
            let vertices = store(
                device,
                queue,
                previous.map(|e| &e.buffers.vertices),
                "resident vertices",
                bytes,
                wgpu::BufferUsages::VERTEX,
            );
            (vertices, None, attributes, 0, 0)
        };
        // The index is kept from the second upload on, and written again only
        // when it changed ( or the vertex count its validation used ).
        let keep_index = plain && previous.is_some();
        let index_unchanged = plain
            && previous.is_some_and(|e| {
                e.vertex_count == vertex_count && e.index_source.as_ref() == Some(&geometry.index)
            });
        let (indices, index_format) = match previous.filter(|_| index_unchanged) {
            Some(e) => (e.buffers.indices.clone(), e.buffers.index_format),
            None => {
                let indices = build_indices(geometry, is_points, wireframe, wide)?;
                // 16-bit indices when every index fits, as three's setIndex chooses.
                let index_format = if indices
                    .as_ref()
                    .is_some_and(|i| i.iter().all(|&i| i <= u16::MAX as u32))
                {
                    wgpu::IndexFormat::Uint16
                } else {
                    wgpu::IndexFormat::Uint32
                };
                let short: Option<Vec<u16>> = (index_format == wgpu::IndexFormat::Uint16)
                    .then(|| indices.iter().flatten().map(|&i| i as u16).collect());
                let index_bytes: Option<&[u8]> = match &short {
                    Some(s) => Some(bytemuck::cast_slice(s)),
                    None => indices.as_ref().map(|i| bytemuck::cast_slice(i)),
                };
                let buffer = index_bytes.map(|bytes| {
                    written += bytes.len() as u64;
                    store(
                        device,
                        queue,
                        previous.and_then(|e| e.buffers.indices.as_ref()),
                        "resident indices",
                        bytes,
                        wgpu::BufferUsages::INDEX,
                    )
                });
                (buffer, index_format)
            }
        };
        let index_source = keep_index.then(|| geometry.index.clone());
        for a in geometry.attributes.values() {
            a.notify_uploaded();
        }
        if written > 0 {
            self.uploads += 1;
        }
        self.bytes += written;
        let buffers = Buffers {
            vertices,
            dynamic,
            indices,
            attributes,
            dynamic_attributes,
            index_format,
        };
        self.entries.insert(
            key,
            Entry {
                owner: Arc::downgrade(geometry),
                versions,
                buffers: buffers.clone(),
                orphaned: 0,
                fields,
                rewritten,
                vertex_count,
                index_source,
            },
        );
        Ok(buffers)
    }
}

/// Write changed data into a resident buffer when it fits, else allocate
/// one of the next power-of-two size so growing data reallocates rarely.
fn rewrite(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    label: &str,
    bytes: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let size =
        (bytes.len() as u64).div_ceil(wgpu::COPY_BUFFER_ALIGNMENT) * wgpu::COPY_BUFFER_ALIGNMENT;
    if size <= buffer.size() {
        if bytes.len() as u64 == size {
            queue.write_buffer(buffer, 0, bytes);
        } else {
            let mut padded = bytes.to_vec();
            padded.resize(size as usize, 0);
            queue.write_buffer(buffer, 0, &padded);
        }
        return buffer.clone();
    }
    let grown = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size.next_power_of_two(),
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut padded = bytes.to_vec();
    padded.resize(size as usize, 0);
    queue.write_buffer(&grown, 0, &padded);
    grown
}
/// Queue uploads, not mapped-at-creation buffers: WebGPU's mapped range holds a
/// Wasm memory view until unmap, which allocator growth can detach.
fn upload(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    bytes: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let size =
        (bytes.len() as u64).div_ceil(wgpu::COPY_BUFFER_ALIGNMENT) * wgpu::COPY_BUFFER_ALIGNMENT;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size.max(wgpu::COPY_BUFFER_ALIGNMENT),
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if bytes.len() as u64 == size {
        queue.write_buffer(&buffer, 0, bytes);
    } else {
        let mut padded = bytes.to_vec();
        padded.resize(size as usize, 0);
        queue.write_buffer(&buffer, 0, &padded);
    }
    buffer
}
