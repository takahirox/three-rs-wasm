//! BufferGeometryLoader.parse( json ) ( src/loaders/BufferGeometryLoader.js ):
//! the index, attributes, morph attributes, groups and name of a
//! BufferGeometry JSON. Interleaved buffers are not supported.
use crate::attribute::BufferAttribute;
use crate::geometry::{Attribute, BufferGeometry};
use crate::{Error, Result};
use serde_json::Value;

fn bad(what: &str) -> Error {
    Error::Asset(format!("BufferGeometryLoader: {what}"))
}

/// getTypedArray( type, array ) as one of the library's attributes.
fn attribute(v: &Value) -> Result<Attribute> {
    if v["isInterleavedBufferAttribute"].as_bool() == Some(true) {
        return Err(bad("interleaved attributes are not supported"));
    }
    let size = v["itemSize"].as_u64().ok_or_else(|| bad("itemSize"))? as usize;
    let normalized = v["normalized"].as_bool().unwrap_or(false);
    let values = v["array"].as_array().ok_or_else(|| bad("array"))?;
    let numbers = || values.iter().map(|n| n.as_f64().unwrap_or(f64::NAN));
    Ok(match v["type"].as_str().unwrap_or("Float32Array") {
        "Float32Array" => Attribute::F32(BufferAttribute::new(
            numbers().map(|n| n as f32).collect(),
            size,
            normalized,
        )?),
        "Uint16Array" => Attribute::U16(BufferAttribute::new(
            numbers().map(|n| n as u16).collect(),
            size,
            normalized,
        )?),
        "Uint32Array" => Attribute::U32(BufferAttribute::new(
            numbers().map(|n| n as u32).collect(),
            size,
            normalized,
        )?),
        "Int16Array" => Attribute::I16(BufferAttribute::new(
            numbers().map(|n| n as i16).collect(),
            size,
            normalized,
        )?),
        "Uint8Array" => Attribute::U8(BufferAttribute::new(
            numbers().map(|n| n as u8).collect(),
            size,
            normalized,
        )?),
        "Int8Array" => Attribute::I8(BufferAttribute::new(
            numbers().map(|n| n as i8).collect(),
            size,
            normalized,
        )?),
        "Int32Array" => Attribute::I32(BufferAttribute::new(
            numbers().map(|n| n as i32).collect(),
            size,
            normalized,
        )?),
        other => return Err(bad(&format!("unsupported array type {other}"))),
    })
}

/// BufferGeometryLoader.parse( JSON.parse( text ) ).
pub fn parse(json: &[u8]) -> Result<BufferGeometry> {
    let json: Value = serde_json::from_slice(json).map_err(|e| bad(&e.to_string()))?;
    let data = &json["data"];
    let mut g = BufferGeometry::default();
    if let Some(index) = data.get("index") {
        let values = index["array"]
            .as_array()
            .ok_or_else(|| bad("index array"))?;
        g.set_index(Some(
            values
                .iter()
                .map(|v| v.as_f64().unwrap_or(0.) as u32)
                .collect(),
        ));
    }
    for (key, a) in data["attributes"]
        .as_object()
        .ok_or_else(|| bad("attributes"))?
    {
        g.set_attribute(key, attribute(a)?);
    }
    if let Some(morphs) = data.get("morphAttributes").and_then(Value::as_object) {
        for (key, list) in morphs {
            let list = list
                .as_array()
                .ok_or_else(|| bad("morph attributes"))?
                .iter()
                .map(attribute)
                .collect::<Result<Vec<_>>>()?;
            g.morph_attributes.insert(key.clone(), list);
        }
    }
    if data["morphTargetsRelative"].as_bool() == Some(true) {
        g.morph_targets_relative = true;
    }
    let groups = ["groups", "drawcalls", "offsets"]
        .iter()
        .find_map(|k| data.get(*k).and_then(Value::as_array));
    for group in groups.into_iter().flatten() {
        g.add_group(
            group["start"].as_u64().unwrap_or(0) as usize,
            group["count"].as_u64().unwrap_or(0) as usize,
            group["materialIndex"].as_u64().unwrap_or(0) as usize,
        );
    }
    if let Some(name) = json["name"].as_str()
        && !name.is_empty()
    {
        g.name = name.to_string();
    }
    Ok(g)
}
