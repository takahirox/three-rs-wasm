//! CPU evaluation for explicit queries (raycasting, exports) and test oracles.
//! Rendering and shadow passes use deformation_gpu and never call this evaluator.
use crate::{Error, Result, attribute::BufferAttribute, geometry::*, math::*, scene::*};
use std::sync::Arc;
#[derive(Clone, Debug, serde::Serialize)]
pub struct Skin {
    pub joints: Vec<Object3D>,
    pub inverse_bind_matrices: Vec<Matrix4>,
}
pub fn evaluate(scene: &Scene, object: Object3D) -> Result<Option<Arc<BufferGeometry>>> {
    let node = scene.get(object)?;
    let Some(source) = node.geometry() else {
        return Ok(None);
    };
    if node.skin.is_none() && node.morph_weights.iter().all(|w| *w == 0.0) {
        return Ok(None);
    }
    if node.morph_weights.iter().any(|w| !w.is_finite()) {
        return Err(Error::Invalid("morph weight"));
    }
    let mut geometry = (**source).clone();
    for name in ["position", "normal", "color", "tangent"] {
        let Some(targets) = source.morph_attributes.get(name) else {
            continue;
        };
        if node.morph_weights.len() > targets.len() {
            return Err(Error::Invalid("morph weight count"));
        }
        let base = source
            .attributes
            .get(name)
            .ok_or(Error::Invalid("morph base attribute"))?;
        let mut values = Vec::with_capacity(base.count() * base.item_size());
        for vertex in 0..base.count() {
            for component in 0..base.item_size() {
                let initial = base.get_component(vertex, component)?;
                let mut value = initial;
                for (target, &weight) in targets.iter().zip(&node.morph_weights) {
                    if target.count() != base.count()
                        || target.item_size()
                            != (if name == "tangent" {
                                3
                            } else {
                                base.item_size()
                            })
                    {
                        return Err(Error::Invalid("morph attribute dimensions"));
                    }
                    if name == "tangent" && component == 3 {
                        continue;
                    }
                    let delta = target.get_component(vertex, component)?
                        - if source.morph_targets_relative {
                            0.0
                        } else {
                            initial
                        };
                    value += weight * delta;
                }
                values.push(value as f32);
            }
        }
        geometry.set_attribute(
            name,
            Attribute::F32(BufferAttribute::new(values, base.item_size(), false)?),
        );
    }
    if let Some(skin) = &node.skin {
        if skin.joints.len() != skin.inverse_bind_matrices.len()
            || node.matrix_world.determinant() == 0.0
        {
            return Err(Error::Invalid("skin joints/bind matrices"));
        }
        let inverse = node.matrix_world.inverse();
        let palette = skin
            .joints
            .iter()
            .zip(&skin.inverse_bind_matrices)
            .map(|(&joint, bind)| Ok(inverse * scene.get(joint)?.matrix_world * *bind))
            .collect::<Result<Vec<_>>>()?;
        let joints = source
            .attributes
            .get("skinIndex")
            .ok_or(Error::Invalid("skinIndex attribute"))?;
        let weights = source
            .attributes
            .get("skinWeight")
            .ok_or(Error::Invalid("skinWeight attribute"))?;
        if joints.item_size() != 4
            || weights.item_size() != 4
            || joints.count() != geometry.vertex_count()
            || weights.count() != geometry.vertex_count()
        {
            return Err(Error::Invalid("skin attribute dimensions"));
        }
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        for vertex in 0..geometry.vertex_count() {
            let mut matrix = Matrix4::ZERO;
            for c in 0..4 {
                let weight = weights.get_component(vertex, c)?;
                let joint = joints.get_component(vertex, c)?;
                if !weight.is_finite()
                    || weight < 0.0
                    || !joint.is_finite()
                    || joint < 0.0
                    || joint.fract() != 0.0
                {
                    return Err(Error::Invalid("skin influence"));
                }
                if weight == 0.0 {
                    continue;
                }
                matrix += *palette
                    .get(joint as usize)
                    .ok_or(Error::Invalid("skin joint index"))?
                    * weight;
            }
            let p = geometry.attributes["position"].vector3(vertex)?;
            positions.extend(matrix.transform_point3(p).as_vec3().to_array());
            if let Some(normal) = geometry.attributes.get("normal") {
                normals.extend(
                    matrix
                        .transform_vector3(normal.vector3(vertex)?)
                        .normalize_or_zero()
                        .as_vec3()
                        .to_array(),
                );
            }
            if let Some(tangent) = geometry.attributes.get("tangent") {
                tangents.extend(
                    matrix
                        .transform_vector3(tangent.vector3(vertex)?)
                        .normalize_or_zero()
                        .as_vec3()
                        .to_array(),
                );
                tangents.push(tangent.get_component(vertex, 3)? as f32);
            }
        }
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        if !normals.is_empty() {
            geometry.set_attribute(
                "normal",
                Attribute::F32(BufferAttribute::new(normals, 3, false)?),
            );
        }
        if !tangents.is_empty() {
            geometry.set_attribute(
                "tangent",
                Attribute::F32(BufferAttribute::new(tangents, 4, false)?),
            );
        }
    }
    geometry.morph_attributes.clear();
    geometry.bounding_box = None;
    geometry.bounding_sphere = None;
    Ok(Some(Arc::new(geometry)))
}
