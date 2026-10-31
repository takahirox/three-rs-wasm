//! BufferGeometryUtils ( examples/jsm/utils/BufferGeometryUtils.js ).
use crate::attribute::BufferAttribute;
use crate::geometry::{Attribute, BufferGeometry};
use crate::{Error, Result};
use std::collections::HashMap;

/// mergeVertices( geometry, tolerance ): vertices whose attributes agree to
/// the tolerance ( `Math.trunc( v × 10^log10( 1 / tolerance ) + ½ tolerance ×
/// that )` per component ) share one index; the result is indexed, in first-
/// occurrence order, every attribute and morph attribute remapped
/// ( remapAttribute's raw copy ) to the first source vertex of each index.
/// Float32 attributes and morph attributes.
pub fn merge_vertices(geometry: &BufferGeometry, tolerance: f64) -> Result<BufferGeometry> {
    let tolerance = tolerance.max(f64::EPSILON);
    let positions = geometry
        .attributes
        .get("position")
        .ok_or(Error::Invalid("mergeVertices without positions"))?;
    let count = geometry
        .index
        .as_ref()
        .map_or(positions.count(), |i| i.len());
    fn f32s(a: &Attribute) -> Result<(&[f32], usize, bool)> {
        match a {
            Attribute::F32(a) => Ok((a.array(), a.item_size(), a.normalized)),
            _ => Err(Error::Invalid("mergeVertices supports Float32 attributes")),
        }
    }
    let names: Vec<String> = geometry.attributes.keys().cloned().collect();
    let sources = names
        .iter()
        .map(|n| f32s(&geometry.attributes[n]))
        .collect::<Result<Vec<_>>>()?;
    let half = tolerance * 0.5;
    let exponent = crate::js_math::log10(1.0 / tolerance);
    let multiplier = crate::js_math::pow(10.0, exponent);
    let additive = half * multiplier;
    // Math.trunc as the hash string prints it ( −0 as 0 ).
    let key = |v: f32| {
        let t = (f64::from(v) * multiplier + additive).trunc();
        if t == 0.0 {
            0.0f64.to_bits()
        } else {
            t.to_bits()
        }
    };
    let mut map: HashMap<Vec<u64>, u32> = HashMap::new();
    let mut source_indices = vec![];
    let mut indices = Vec::with_capacity(count);
    let mut next = 0u32;
    for i in 0..count {
        let index = match &geometry.index {
            Some(ix) => ix[i] as usize,
            None => i,
        };
        let mut hash = vec![];
        for (data, size, _) in &sources {
            for k in 0..*size {
                hash.push(key(data.get(index * size + k).copied().unwrap_or(f32::NAN)));
            }
        }
        if let Some(&id) = map.get(&hash) {
            indices.push(id);
            continue;
        }
        source_indices.push(index);
        map.insert(hash, next);
        indices.push(next);
        next += 1;
    }
    // remapAttribute: the source's raw entries in source-index order, NaN
    // past its end ( a Float32Array of undefined ).
    let remap = |a: &Attribute| -> Result<Attribute> {
        let (data, size, normalized) = f32s(a)?;
        let mut array = Vec::with_capacity(source_indices.len() * size);
        for &index in &source_indices {
            for k in 0..size {
                array.push(data.get(index * size + k).copied().unwrap_or(f32::NAN));
            }
        }
        Ok(Attribute::F32(BufferAttribute::new(
            array, size, normalized,
        )?))
    };
    let mut result = geometry.clone();
    for name in &names {
        result.set_attribute(name, remap(&geometry.attributes[name])?);
    }
    for (name, list) in &geometry.morph_attributes {
        let remapped = list.iter().map(remap).collect::<Result<Vec<_>>>()?;
        result.morph_attributes.insert(name.clone(), remapped);
    }
    result.set_index(Some(indices));
    Ok(result)
}

/// BufferGeometry.applyMatrix4( m ) with three.js's arithmetic: positions
/// through Vector3.applyMatrix4, normals through the normal matrix
/// ( getNormalMatrix ) and normalized, each stored back as Float32.
/// `m` is column-major ( Matrix4.elements ).
pub fn apply_matrix4(geometry: &mut BufferGeometry, m: &[f64; 16]) -> Result<()> {
    use crate::fbx::math::{M3, M4, V3};
    let m4 = M4(*m);
    if let Some(Attribute::F32(a)) = geometry.attributes.get_mut("position") {
        for p in a.array_mut().as_chunks_mut::<3>().0 {
            let v = V3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])).apply_matrix4(&m4);
            *p = [v.x as f32, v.y as f32, v.z as f32];
        }
    }
    let nm = M3::normal_matrix(&m4);
    if let Some(Attribute::F32(a)) = geometry.attributes.get_mut("normal") {
        for p in a.array_mut().as_chunks_mut::<3>().0 {
            let v = V3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]))
                .apply_matrix3(&nm)
                .normalize();
            *p = [v.x as f32, v.y as f32, v.z as f32];
        }
    }
    Ok(())
}

/// Matrix4.compose( position, quaternion, scale ) with three.js's
/// arithmetic, column-major.
pub fn compose(position: [f64; 3], quaternion: [f64; 4], scale: [f64; 3]) -> [f64; 16] {
    use crate::fbx::math::{M4, Q, V3};
    let [x, y, z, w] = quaternion;
    M4::compose(
        V3::new(position[0], position[1], position[2]),
        Q { x, y, z, w },
        V3::new(scale[0], scale[1], scale[2]),
    )
    .0
}
