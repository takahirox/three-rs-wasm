use crate::{Error, Result, attribute::*, math::*};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    collections::{BTreeMap, HashMap},
    sync::{Arc, RwLock},
};
type CloneBuffers = HashMap<(TypeId, usize), Arc<dyn Any + Send + Sync>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Attribute {
    I8(BufferAttribute<i8>),
    U8(BufferAttribute<u8>),
    Clamped(BufferAttribute<ClampedU8>),
    I16(BufferAttribute<i16>),
    U16(BufferAttribute<u16>),
    I32(BufferAttribute<i32>),
    U32(BufferAttribute<u32>),
    F16(BufferAttribute<half::f16>),
    F32(BufferAttribute<f32>),
    F64(BufferAttribute<f64>),
    InterleavedI8(InterleavedBufferAttribute<i8>),
    InterleavedU8(InterleavedBufferAttribute<u8>),
    InterleavedClamped(InterleavedBufferAttribute<ClampedU8>),
    InterleavedI16(InterleavedBufferAttribute<i16>),
    InterleavedU16(InterleavedBufferAttribute<u16>),
    InterleavedI32(InterleavedBufferAttribute<i32>),
    InterleavedU32(InterleavedBufferAttribute<u32>),
    InterleavedF16(InterleavedBufferAttribute<half::f16>),
    InterleavedF32(InterleavedBufferAttribute<f32>),
    InterleavedF64(InterleavedBufferAttribute<f64>),
}
macro_rules! dispatch {
    ($self:expr, $a:ident, $body:expr) => {
        match $self {
            Attribute::I8($a) => $body,
            Attribute::U8($a) => $body,
            Attribute::Clamped($a) => $body,
            Attribute::I16($a) => $body,
            Attribute::U16($a) => $body,
            Attribute::I32($a) => $body,
            Attribute::U32($a) => $body,
            Attribute::F16($a) => $body,
            Attribute::F32($a) => $body,
            Attribute::F64($a) => $body,
            Attribute::InterleavedI8($a) => $body,
            Attribute::InterleavedU8($a) => $body,
            Attribute::InterleavedClamped($a) => $body,
            Attribute::InterleavedI16($a) => $body,
            Attribute::InterleavedU16($a) => $body,
            Attribute::InterleavedI32($a) => $body,
            Attribute::InterleavedU32($a) => $body,
            Attribute::InterleavedF16($a) => $body,
            Attribute::InterleavedF32($a) => $body,
            Attribute::InterleavedF64($a) => $body,
        }
    };
}
impl Attribute {
    fn clone_with_buffers(&self, buffers: &mut CloneBuffers) -> Self {
        fn copy<T: Component>(
            view: &InterleavedBufferAttribute<T>,
            buffers: &mut CloneBuffers,
        ) -> InterleavedBufferAttribute<T> {
            let key = (TypeId::of::<T>(), Arc::as_ptr(&view.data) as usize);
            let data = buffers
                .entry(key)
                .or_insert_with(|| {
                    Arc::new(RwLock::new(
                        view.data.read().expect("interleaved lock poisoned").clone(),
                    ))
                })
                .clone();
            let data = data
                .downcast::<RwLock<InterleavedBuffer<T>>>()
                .expect("typed clone buffer");
            let mut clone = view.clone();
            clone.data = data;
            clone
        }
        match self {
            Self::InterleavedI8(a) => Self::InterleavedI8(copy(a, buffers)),
            Self::InterleavedU8(a) => Self::InterleavedU8(copy(a, buffers)),
            Self::InterleavedClamped(a) => Self::InterleavedClamped(copy(a, buffers)),
            Self::InterleavedI16(a) => Self::InterleavedI16(copy(a, buffers)),
            Self::InterleavedU16(a) => Self::InterleavedU16(copy(a, buffers)),
            Self::InterleavedI32(a) => Self::InterleavedI32(copy(a, buffers)),
            Self::InterleavedU32(a) => Self::InterleavedU32(copy(a, buffers)),
            Self::InterleavedF16(a) => Self::InterleavedF16(copy(a, buffers)),
            Self::InterleavedF32(a) => Self::InterleavedF32(copy(a, buffers)),
            Self::InterleavedF64(a) => Self::InterleavedF64(copy(a, buffers)),
            _ => self.clone(),
        }
    }

    pub fn notify_uploaded(&self) {
        dispatch!(self, a, a.notify_uploaded());
    }
    pub fn count(&self) -> usize {
        dispatch!(self, a, a.count())
    }
    pub fn item_size(&self) -> usize {
        dispatch!(self, a, a.item_size())
    }
    pub fn version(&self) -> u64 {
        dispatch!(self, a, a.version())
    }
    pub fn get_component(&self, i: usize, c: usize) -> Result<f64> {
        dispatch!(self, a, a.get_component(i, c))
    }
    pub fn set_component(&mut self, i: usize, c: usize, v: f64) -> Result<()> {
        dispatch!(self, a, a.set_component(i, c, v))
    }
    pub fn vector3(&self, i: usize) -> Result<Vector3> {
        dispatch!(self, a, a.vector3(i))
    }
    pub fn apply_matrix4(&mut self, m: Matrix4) -> Result<()> {
        dispatch!(self, a, a.apply_matrix4(m))
    }
    pub fn apply_normal_matrix(&mut self, m: Matrix3) -> Result<()> {
        dispatch!(self, a, a.apply_normal_matrix(m))
    }
    pub fn transform_direction(&mut self, m: Matrix4) -> Result<()> {
        dispatch!(self, a, a.transform_direction(m))
    }
    fn expanded(&self, indices: &[u32]) -> Result<Self> {
        fn expand<T: Component>(
            a: &BufferAttribute<T>,
            indices: &[u32],
        ) -> Result<BufferAttribute<T>> {
            let mut values = Vec::with_capacity(indices.len() * a.item_size());
            for &i in indices {
                let start = i as usize * a.item_size();
                values.extend_from_slice(
                    a.array()
                        .get(start..start + a.item_size())
                        .ok_or(Error::Invalid("geometry index"))?,
                );
            }
            BufferAttribute::new(values, a.item_size(), a.normalized)
        }
        Ok(match self {
            Self::I8(a) => Self::I8(expand(a, indices)?),
            Self::U8(a) => Self::U8(expand(a, indices)?),
            Self::Clamped(a) => Self::Clamped(expand(a, indices)?),
            Self::I16(a) => Self::I16(expand(a, indices)?),
            Self::U16(a) => Self::U16(expand(a, indices)?),
            Self::I32(a) => Self::I32(expand(a, indices)?),
            Self::U32(a) => Self::U32(expand(a, indices)?),
            Self::F16(a) => Self::F16(expand(a, indices)?),
            Self::F32(a) => Self::F32(expand(a, indices)?),
            Self::F64(a) => Self::F64(expand(a, indices)?),
            Self::InterleavedI8(a) => Self::I8(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedU8(a) => Self::U8(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedClamped(a) => Self::Clamped(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedI16(a) => Self::I16(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedU16(a) => Self::U16(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedI32(a) => Self::I32(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedU32(a) => Self::U32(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedF16(a) => Self::F16(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedF32(a) => Self::F32(expand(&a.to_attribute()?, indices)?),
            Self::InterleavedF64(a) => Self::F64(expand(&a.to_attribute()?, indices)?),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub start: usize,
    pub count: usize,
    pub material_index: usize,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawRange {
    pub start: usize,
    pub count: Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BufferGeometry {
    pub identity: crate::identity::Identity,
    pub name: String,
    pub attributes: BTreeMap<String, Attribute>,
    pub index: Option<Vec<u32>>,
    pub groups: Vec<Group>,
    pub draw_range: DrawRange,
    pub bounding_box: Option<Box3>,
    pub bounding_sphere: Option<Sphere>,
    pub morph_attributes: BTreeMap<String, Vec<Attribute>>,
    pub morph_targets_relative: bool,
    pub indirect: Option<Vec<u32>>,
    pub indirect_offset: usize,
    pub indirect_offsets: Vec<usize>,
    pub instance_count: Option<u32>,
    pub user_data: serde_json::Map<String, serde_json::Value>,
}
impl Clone for BufferGeometry {
    fn clone(&self) -> Self {
        let mut buffers = CloneBuffers::new();
        Self {
            identity: self.identity.clone(),
            name: self.name.clone(),
            attributes: self
                .attributes
                .iter()
                .map(|(name, a)| (name.clone(), a.clone_with_buffers(&mut buffers)))
                .collect(),
            index: self.index.clone(),
            groups: self.groups.clone(),
            draw_range: self.draw_range,
            bounding_box: self.bounding_box,
            bounding_sphere: self.bounding_sphere,
            morph_attributes: self
                .morph_attributes
                .iter()
                .map(|(name, attrs)| {
                    (
                        name.clone(),
                        attrs
                            .iter()
                            .map(|a| a.clone_with_buffers(&mut buffers))
                            .collect(),
                    )
                })
                .collect(),
            morph_targets_relative: self.morph_targets_relative,
            indirect: self.indirect.clone(),
            indirect_offset: self.indirect_offset,
            indirect_offsets: self.indirect_offsets.clone(),
            instance_count: self.instance_count,
            user_data: self.user_data.clone(),
        }
    }
}
impl BufferGeometry {
    pub fn copy_from(&mut self, source: &Self) {
        let identity = std::mem::take(&mut self.identity);
        *self = source.clone();
        self.identity = identity;
    }
    pub fn look_at(&mut self, target: Vector3) -> Result<()> {
        let mut scene = crate::scene::Scene::new();
        let node = scene.insert(crate::scene::NodeKind::Group);
        scene.look_at(node, target)?;
        self.apply_quaternion(scene.get(node)?.quaternion)
    }
    pub fn set_attribute(&mut self, name: impl Into<String>, attribute: Attribute) {
        self.attributes.insert(name.into(), attribute);
    }
    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }
    pub fn delete_attribute(&mut self, name: &str) -> Option<Attribute> {
        self.attributes.remove(name)
    }
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }
    pub fn set_index(&mut self, index: Option<Vec<u32>>) {
        self.index = index;
    }
    pub fn get_index(&self) -> Option<&[u32]> {
        self.index.as_deref()
    }
    pub fn set_indirect(&mut self, commands: Option<Vec<u32>>) {
        self.indirect = commands;
    }
    pub fn get_indirect(&self) -> Option<&[u32]> {
        self.indirect.as_deref()
    }
    pub fn add_group(&mut self, start: usize, count: usize, material_index: usize) {
        self.groups.push(Group {
            start,
            count,
            material_index,
        });
    }
    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }
    pub fn set_draw_range(&mut self, start: usize, count: Option<usize>) {
        self.draw_range = DrawRange { start, count };
    }
    pub fn set_from_points(&mut self, points: &[Vector3]) -> Result<()> {
        let values = points
            .iter()
            .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32])
            .collect();
        self.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(values, 3, false)?),
        );
        Ok(())
    }
    pub fn positions(&self) -> Result<Vec<Vector3>> {
        let a = self
            .attributes
            .get("position")
            .ok_or(Error::Invalid("missing position attribute"))?;
        (0..a.count()).map(|i| a.vector3(i)).collect()
    }
    pub fn vertex_count(&self) -> usize {
        self.attributes.get("position").map_or(0, Attribute::count)
    }
    pub fn draw_count(&self) -> usize {
        self.index.as_ref().map_or(self.vertex_count(), Vec::len)
    }
    pub fn vertex_index(&self, i: usize) -> Result<usize> {
        let index = match &self.index {
            Some(indices) => *indices.get(i).ok_or(Error::Invalid("index offset"))? as usize,
            None => i,
        };
        if index >= self.vertex_count() {
            return Err(Error::Invalid("geometry index"));
        }
        Ok(index)
    }
    pub fn compute_bounding_box(&mut self) -> Result<Box3> {
        let base = Box3::from_points(self.positions()?);
        let mut bounds = base;
        if let Some(morphs) = self.morph_attributes.get("position") {
            for morph in morphs {
                let points: Result<Vec<_>> = (0..morph.count()).map(|i| morph.vector3(i)).collect();
                let b = Box3::from_points(points?);
                if self.morph_targets_relative {
                    bounds.expand_by_point(base.min + b.min);
                    bounds.expand_by_point(base.max + b.max);
                } else {
                    bounds.union(b);
                }
            }
        }
        self.bounding_box = Some(bounds);
        Ok(bounds)
    }
    pub fn compute_bounding_sphere(&mut self) -> Result<Sphere> {
        let old_box = self.bounding_box;
        let center = self.compute_bounding_box()?.center();
        self.bounding_box = old_box;
        let positions = self.positions()?;
        let mut radius2 = positions
            .iter()
            .map(|p| p.distance_squared(center))
            .fold(0.0, f64::max);
        if let Some(morphs) = self.morph_attributes.get("position") {
            for morph in morphs {
                for (i, base) in positions.iter().enumerate() {
                    let p = morph.vector3(i)?
                        + if self.morph_targets_relative {
                            *base
                        } else {
                            Vector3::ZERO
                        };
                    radius2 = radius2.max(p.distance_squared(center));
                }
            }
        }
        let sphere = Sphere {
            center,
            radius: radius2.sqrt(),
        };
        self.bounding_sphere = Some(sphere);
        Ok(sphere)
    }
    pub fn apply_matrix4(&mut self, m: Matrix4) -> Result<()> {
        if let Some(a) = self.attributes.get_mut("position") {
            a.apply_matrix4(m)?;
        }
        if let Some(a) = self.attributes.get_mut("normal") {
            a.apply_normal_matrix(Matrix3::from_mat4(m).inverse().transpose())?;
        }
        if let Some(a) = self.attributes.get_mut("tangent") {
            a.transform_direction(m)?;
        }
        if self.bounding_box.is_some() {
            self.compute_bounding_box()?;
        }
        if self.bounding_sphere.is_some() {
            self.compute_bounding_sphere()?;
        }
        Ok(())
    }
    pub fn apply_quaternion(&mut self, q: Quaternion) -> Result<()> {
        self.apply_matrix4(Matrix4::from_quat(q))
    }
    pub fn translate(&mut self, v: Vector3) -> Result<()> {
        self.apply_matrix4(Matrix4::from_translation(v))
    }
    pub fn scale(&mut self, v: Vector3) -> Result<()> {
        self.apply_matrix4(Matrix4::from_scale(v))
    }
    pub fn rotate_x(&mut self, angle: f64) -> Result<()> {
        self.apply_matrix4(Matrix4::from_rotation_x(angle))
    }
    pub fn rotate_y(&mut self, angle: f64) -> Result<()> {
        self.apply_matrix4(Matrix4::from_rotation_y(angle))
    }
    pub fn rotate_z(&mut self, angle: f64) -> Result<()> {
        self.apply_matrix4(Matrix4::from_rotation_z(angle))
    }
    pub fn center(&mut self) -> Result<()> {
        let offset = -self.compute_bounding_box()?.center();
        self.translate(offset)
    }
    pub fn to_non_indexed(&self) -> Result<Self> {
        let Some(index) = &self.index else {
            return Ok(self.clone());
        };
        let mut geometry = Self {
            groups: self.groups.clone(),
            morph_targets_relative: self.morph_targets_relative,
            ..Default::default()
        };
        for (name, a) in &self.attributes {
            geometry.attributes.insert(name.clone(), a.expanded(index)?);
        }
        for (name, morphs) in &self.morph_attributes {
            geometry.morph_attributes.insert(
                name.clone(),
                morphs
                    .iter()
                    .map(|a| a.expanded(index))
                    .collect::<Result<_>>()?,
            );
        }
        Ok(geometry)
    }
    pub fn compute_vertex_normals(&mut self) -> Result<()> {
        let positions = self.positions()?;
        let mut normals = vec![Vector3::ZERO; positions.len()];
        for i in (0..self.draw_count().saturating_sub(2)).step_by(3) {
            let a = self.vertex_index(i)?;
            let b = self.vertex_index(i + 1)?;
            let c = self.vertex_index(i + 2)?;
            let n = (positions[c] - positions[b]).cross(positions[a] - positions[b]);
            normals[a] += n;
            normals[b] += n;
            normals[c] += n;
        }
        let values = normals
            .into_iter()
            .flat_map(|n| n.normalize_or_zero().to_array().map(|v| v as f32))
            .collect();
        self.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(values, 3, false)?),
        );
        Ok(())
    }
    pub fn normalize_normals(&mut self) -> Result<()> {
        let a = self
            .attributes
            .get_mut("normal")
            .ok_or(Error::Invalid("missing normal attribute"))?;
        for i in 0..a.count() {
            let n = a.vector3(i)?.normalize_or_zero();
            for c in 0..3 {
                a.set_component(i, c, n[c])?;
            }
        }
        Ok(())
    }
    pub fn compute_tangents(&mut self) -> Result<()> {
        let indices = self
            .index
            .as_ref()
            .ok_or(Error::Invalid("tangents require indexed geometry"))?;
        let positions = self.positions()?;
        let uv = self
            .attributes
            .get("uv")
            .ok_or(Error::Invalid("missing uv attribute"))?;
        let normal = self
            .attributes
            .get("normal")
            .ok_or(Error::Invalid("missing normal attribute"))?;
        let mut tan1 = vec![Vector3::ZERO; positions.len()];
        let mut tan2 = tan1.clone();
        let groups = if self.groups.is_empty() {
            vec![Group {
                start: 0,
                count: indices.len(),
                material_index: 0,
            }]
        } else {
            self.groups.clone()
        };
        for group in groups {
            for offset in (group.start
                ..group
                    .start
                    .saturating_add(group.count)
                    .min(indices.len())
                    .saturating_sub(2))
                .step_by(3)
            {
                let ids = [
                    self.vertex_index(offset)?,
                    self.vertex_index(offset + 1)?,
                    self.vertex_index(offset + 2)?,
                ];
                let [a, b, c] = ids;
                let e1 = positions[b] - positions[a];
                let e2 = positions[c] - positions[a];
                let s1 = uv.get_component(b, 0)? - uv.get_component(a, 0)?;
                let s2 = uv.get_component(c, 0)? - uv.get_component(a, 0)?;
                let t1 = uv.get_component(b, 1)? - uv.get_component(a, 1)?;
                let t2 = uv.get_component(c, 1)? - uv.get_component(a, 1)?;
                let r = 1.0 / (s1 * t2 - s2 * t1);
                if !r.is_finite() {
                    continue;
                }
                for i in ids {
                    tan1[i] += (e1 * t2 - e2 * t1) * r;
                    tan2[i] += (e2 * s1 - e1 * s2) * r;
                }
            }
        }
        let mut values = Vec::with_capacity(positions.len() * 4);
        for i in 0..positions.len() {
            let n = normal.vector3(i)?;
            let t = tan1[i];
            let tangent = (t - n * n.dot(t)).normalize_or_zero();
            values.extend(tangent.to_array().map(|v| v as f32));
            values.push(if n.cross(t).dot(tan2[i]) < 0.0 {
                -1.0
            } else {
                1.0
            });
        }
        self.set_attribute(
            "tangent",
            Attribute::F32(BufferAttribute::new(values, 4, false)?),
        );
        Ok(())
    }
    pub fn dispose(&mut self) {
        self.attributes.clear();
        self.index = None;
        self.morph_attributes.clear();
        self.indirect = None;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstancedBufferGeometry {
    pub geometry: BufferGeometry,
    pub instance_count: u32,
}
impl InstancedBufferGeometry {
    pub fn new(geometry: BufferGeometry, instance_count: u32) -> Self {
        Self {
            geometry,
            instance_count,
        }
    }
    pub fn into_geometry(mut self) -> BufferGeometry {
        self.geometry.instance_count = Some(self.instance_count);
        self.geometry
    }
}

pub struct PlaneGeometry;
impl PlaneGeometry {
    pub fn build(
        width: f64,
        height: f64,
        width_segments: u32,
        height_segments: u32,
    ) -> Result<BufferGeometry> {
        if width_segments == 0 || height_segments == 0 {
            return Err(Error::Invalid("plane segments"));
        }
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for y in 0..=height_segments {
            for x in 0..=width_segments {
                let u = x as f64 / width_segments as f64;
                let v = y as f64 / height_segments as f64;
                positions.extend([
                    (u * width - width / 2.0) as f32,
                    (-v * height + height / 2.0) as f32,
                    0.0,
                ]);
                normals.extend([0.0, 0.0, 1.0]);
                uvs.extend([u as f32, (1.0 - v) as f32]);
            }
        }
        for y in 0..height_segments {
            for x in 0..width_segments {
                let a = x + (width_segments + 1) * y;
                let b = a + width_segments + 1;
                indices.extend([a, b, a + 1, b, b + 1, a + 1]);
            }
        }
        primitive(positions, normals, uvs, indices)
    }
}
pub struct BoxGeometry;
impl BoxGeometry {
    pub fn build(width: f64, height: f64, depth: f64) -> Result<BufferGeometry> {
        let mut g = BufferGeometry::default();
        let mut p = Vec::new();
        let mut n = Vec::new();
        let mut uv = Vec::new();
        let mut indices = Vec::new();
        // Face order and vertex ordering match Three.js BoxGeometry.
        for (face, (u, v, w, udir, vdir, fw, fh, fd)) in [
            (2, 1, 0, -1.0, -1.0, depth, height, width),
            (2, 1, 0, 1.0, -1.0, depth, height, -width),
            (0, 2, 1, 1.0, 1.0, width, depth, height),
            (0, 2, 1, 1.0, -1.0, width, depth, -height),
            (0, 1, 2, 1.0, -1.0, width, height, depth),
            (0, 1, 2, -1.0, -1.0, width, height, -depth),
        ]
        .into_iter()
        .enumerate()
        {
            for y in 0..=1 {
                for x in 0..=1 {
                    let mut point = [0.0; 3];
                    let mut normal = [0.0; 3];
                    point[u] = (x as f64 * fw - fw / 2.0) * udir;
                    point[v] = (y as f64 * fh - fh / 2.0) * vdir;
                    point[w] = fd / 2.0;
                    normal[w] = if fd > 0.0 { 1.0 } else { -1.0 };
                    p.extend(point.map(|v| v as f32));
                    n.extend(normal.map(|v| v as f32));
                    uv.extend([x as f32, 1.0 - y as f32]);
                }
            }
            let a = (face * 4) as u32;
            indices.extend([a, a + 2, a + 1, a + 2, a + 3, a + 1]);
            g.add_group(face * 6, 6, face);
        }
        let mut result = primitive(p, n, uv, indices)?;
        result.groups = g.groups;
        Ok(result)
    }
}
pub struct SphereGeometry;
impl SphereGeometry {
    pub fn build(radius: f64, width_segments: u32, height_segments: u32) -> Result<BufferGeometry> {
        let width_segments = width_segments.max(3);
        let height_segments = height_segments.max(2);
        let mut p = Vec::new();
        let mut n = Vec::new();
        let mut uv = Vec::new();
        let mut indices = Vec::new();
        for y in 0..=height_segments {
            let v = y as f64 / height_segments as f64;
            let offset = if y == 0 {
                0.5 / width_segments as f64
            } else if y == height_segments {
                -0.5 / width_segments as f64
            } else {
                0.0
            };
            for x in 0..=width_segments {
                let u = x as f64 / width_segments as f64;
                let phi = u * std::f64::consts::TAU;
                let theta = v * std::f64::consts::PI;
                let position = Vector3::new(
                    -radius * phi.cos() * theta.sin(),
                    radius * theta.cos(),
                    radius * phi.sin() * theta.sin(),
                );
                p.extend(position.to_array().map(|v| v as f32));
                n.extend(position.normalize_or_zero().to_array().map(|v| v as f32));
                uv.extend([(u + offset) as f32, (1.0 - v) as f32]);
            }
        }
        for y in 0..height_segments {
            for x in 0..width_segments {
                let b = y * (width_segments + 1) + x;
                let a = b + 1;
                let c = b + width_segments + 1;
                let d = c + 1;
                if y != 0 {
                    indices.extend([a, b, d]);
                }
                if y != height_segments - 1 {
                    indices.extend([b, c, d]);
                }
            }
        }
        primitive(p, n, uv, indices)
    }
}
fn primitive(p: Vec<f32>, n: Vec<f32>, uv: Vec<f32>, index: Vec<u32>) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(p, 3, false)?),
    );
    g.set_attribute("normal", Attribute::F32(BufferAttribute::new(n, 3, false)?));
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
    g.index = Some(index);
    Ok(g)
}
