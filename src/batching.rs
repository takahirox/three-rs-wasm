//! CPU-built batches for opaque triangle meshes sharing one material.
//! Rebuild after changing entries; ordinary Mesh rendering submits the result once.
//! GPU-driven per-object culling and transparent object sorting are not provided.
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    geometry::*,
    material::Material,
    math::{Matrix3, Matrix4, Vector3},
    scene::Mesh,
};
use std::{collections::BTreeMap, sync::Arc};

pub struct BatchEntry {
    pub geometry: Arc<BufferGeometry>,
    pub matrix: Matrix4,
    pub visible: bool,
}
pub fn build(entries: &[BatchEntry], material: Arc<Material>) -> Result<Mesh> {
    if matches!(material.as_ref(), Material::Shader(_)) {
        return Err(Error::Invalid(
            "batch custom deformation requires evaluated geometry",
        ));
    }
    if material.properties().transparent {
        return Err(Error::Invalid("batch requires opaque material"));
    }
    let mut attributes: BTreeMap<String, (usize, Vec<f32>)> = BTreeMap::new();
    let mut indices = Vec::new();
    let mut fallback_normals = Vec::new();
    let mut vertex_offset = 0usize;
    for entry in entries.iter().filter(|e| e.visible) {
        let source = &entry.geometry;
        if !entry.matrix.is_finite()
            || entry.matrix.determinant() <= 0.0
            || entry.matrix.x_axis.w != 0.0
            || entry.matrix.y_axis.w != 0.0
            || entry.matrix.z_axis.w != 0.0
            || entry.matrix.w_axis.w != 1.0
        {
            return Err(Error::Invalid("batch affine transform"));
        }
        if !source.morph_attributes.is_empty()
            || source.attributes.contains_key("skinIndex")
            || source.indirect.is_some()
            || source.instance_count.is_some()
        {
            return Err(Error::Invalid(
                "batch requires evaluated non-instanced geometry",
            ));
        }
        if !source.attributes.contains_key("position") {
            return Err(Error::Invalid("batch position"));
        }
        if vertex_offset == 0 {
            for (name, a) in &source.attributes {
                attributes.insert(name.clone(), (a.item_size(), Vec::new()));
            }
        }
        if attributes.len() != source.attributes.len()
            || source.attributes.iter().any(|(name, a)| {
                attributes
                    .get(name)
                    .is_none_or(|(width, _)| *width != a.item_size())
                    || a.count() != source.vertex_count()
            })
        {
            return Err(Error::Invalid("batch attribute layout"));
        }
        if !source.attributes.contains_key("normal") {
            let normal = (Matrix3::from_mat4(entry.matrix).inverse().transpose() * Vector3::Z)
                .normalize_or_zero()
                .as_vec3()
                .to_array();
            for _ in 0..source.vertex_count() {
                fallback_normals.extend(normal);
            }
        }
        let mut geometry = source.as_ref().clone();
        geometry.apply_matrix4(entry.matrix)?;
        let start = source.draw_range.start.min(source.draw_count());
        let end = start
            .saturating_add(source.draw_range.count.unwrap_or(usize::MAX))
            .min(source.draw_count());
        if start % 3 != 0 || (end - start) % 3 != 0 {
            return Err(Error::Invalid("batch triangle range"));
        }
        for i in start..end {
            let index = vertex_offset
                .checked_add(source.vertex_index(i)?)
                .and_then(|i| u32::try_from(i).ok())
                .ok_or(Error::Invalid("batch index capacity"))?;
            indices.push(index);
        }
        for (name, a) in &geometry.attributes {
            let (_, values) = attributes.get_mut(name).unwrap();
            for i in 0..a.count() {
                for c in 0..a.item_size() {
                    values.push(a.get_component(i, c)? as f32);
                }
            }
        }
        vertex_offset += source.vertex_count();
    }
    if !attributes.contains_key("normal") && !fallback_normals.is_empty() {
        attributes.insert("normal".into(), (3, fallback_normals));
    }
    let mut geometry = BufferGeometry::default();
    for (name, (width, values)) in attributes {
        geometry.set_attribute(
            &name,
            Attribute::F32(BufferAttribute::new(values, width, false)?),
        );
    }
    geometry.set_index(Some(indices));
    if vertex_offset > 0 {
        geometry.compute_bounding_box()?;
        geometry.compute_bounding_sphere()?;
    }
    Ok(Mesh::new(Arc::new(geometry), material))
}
