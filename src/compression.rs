//! Decode glTF geometry extensions into ordinary buffer views before importing.
//! All three codecs run as Rust on native targets and wasm32-unknown-unknown.
use crate::{Error, Result, material::Texture};
use serde_json::{Value, json};

pub struct PreparedGltf {
    pub asset: gltf::Gltf,
    pub buffers: Vec<Vec<u8>>,
}
fn number(value: &Value, key: &str) -> Result<usize> {
    value[key]
        .as_u64()
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Invalid("compressed glTF integer"))
}
fn slice(buffers: &[Vec<u8>], buffer: usize, offset: usize, length: usize) -> Result<&[u8]> {
    buffers
        .get(buffer)
        .and_then(|b| {
            offset
                .checked_add(length)
                .and_then(|end| b.get(offset..end))
        })
        .ok_or(Error::Invalid("compressed glTF buffer range"))
}
fn append_view(document: &mut Value, buffers: &mut Vec<Vec<u8>>, data: Vec<u8>) -> Result<usize> {
    let index = buffers.len();
    document["buffers"]
        .as_array_mut()
        .ok_or(Error::Invalid("glTF buffers"))?
        .push(json!({"byteLength":data.len()}));
    let views = document["bufferViews"]
        .as_array_mut()
        .ok_or(Error::Invalid("glTF buffer views"))?;
    let view = views.len();
    views.push(json!({"buffer":index,"byteLength":data.len()}));
    buffers.push(data);
    Ok(view)
}
/// `buffers` contains resolved external buffers (or the GLB BIN chunk) in source
/// index order. Missing URI-less meshopt fallback buffers may be empty.
pub fn prepare_gltf(bytes: &[u8], buffers: &[Vec<u8>]) -> Result<PreparedGltf> {
    let mut document: Value = if bytes.starts_with(b"glTF") {
        let glb = gltf::binary::Glb::from_slice(bytes).map_err(|e| Error::Asset(e.to_string()))?;
        serde_json::from_slice(&glb.json).map_err(|e| Error::Asset(e.to_string()))?
    } else {
        serde_json::from_slice(bytes).map_err(|e| Error::Asset(e.to_string()))?
    };
    let mut buffers = buffers.to_vec();
    let count = document["buffers"]
        .as_array()
        .ok_or(Error::Invalid("glTF buffers"))?
        .len();
    if buffers.len() > count {
        return Err(Error::Invalid("glTF buffer count"));
    }
    buffers.resize_with(count, Vec::new);
    if document.get("bufferViews").is_none() {
        document["bufferViews"] = json!([]);
    }
    let view_count = document["bufferViews"]
        .as_array()
        .ok_or(Error::Invalid("glTF views"))?
        .len();
    for i in 0..view_count {
        let ext = document["bufferViews"][i]["extensions"]["EXT_meshopt_compression"].clone();
        let ext = if ext.is_null() {
            document["bufferViews"][i]["extensions"]["KHR_meshopt_compression"].clone()
        } else {
            ext
        };
        if ext.is_null() {
            continue;
        }
        let count = number(&ext, "count")?;
        let stride = number(&ext, "byteStride")?;
        let size = count
            .checked_mul(stride)
            .filter(|&s| s <= 256 * 1024 * 1024)
            .ok_or(Error::Invalid("meshopt decoded size"))?;
        let source = slice(
            &buffers,
            number(&ext, "buffer")?,
            ext["byteOffset"].as_u64().unwrap_or(0) as usize,
            number(&ext, "byteLength")?,
        )?;
        let mut output = vec![0u8; size];
        match ext["mode"].as_str() {
            Some("ATTRIBUTES") => {
                if stride == 0 || stride > 256 || !stride.is_multiple_of(4) {
                    return Err(Error::Invalid("meshopt vertex stride"));
                }
                optimesh::vertexcodec::decode_vertex_buffer(&mut output, count, stride, source)
                    .map_err(|e| Error::Asset(format!("meshopt: {e:?}")))?;
            }
            Some("TRIANGLES" | "INDICES") => {
                if ![2, 4].contains(&stride)
                    || (ext["mode"] == "TRIANGLES" && !count.is_multiple_of(3))
                {
                    return Err(Error::Invalid("meshopt index layout"));
                }
                let mut indices = vec![0u32; count];
                if ext["mode"] == "TRIANGLES" {
                    optimesh::indexcodec::decode_index_buffer(&mut indices, source)
                } else {
                    optimesh::indexcodec::decode_index_sequence(&mut indices, source)
                }
                .map_err(|e| Error::Asset(format!("meshopt: {e:?}")))?;
                for (i, index) in indices.into_iter().enumerate() {
                    if stride == 2 && index > u16::MAX as u32 {
                        return Err(Error::Invalid("meshopt index overflow"));
                    }
                    output[i * stride..(i + 1) * stride]
                        .copy_from_slice(&index.to_le_bytes()[..stride]);
                }
            }
            _ => return Err(Error::Invalid("meshopt mode")),
        }
        match ext["filter"].as_str().unwrap_or("NONE") {
            "NONE" => {}
            "OCTAHEDRAL" if stride == 4 => {
                let mut values = output.iter().map(|&v| v as i8).collect::<Vec<_>>();
                optimesh::vertexfilter::decode_filter_oct8(&mut values);
                for (out, v) in output.iter_mut().zip(values) {
                    *out = v as u8;
                }
            }
            "OCTAHEDRAL" | "QUATERNION" if stride == 8 => {
                let mut values = output
                    .chunks_exact(2)
                    .map(|v| i16::from_le_bytes([v[0], v[1]]))
                    .collect::<Vec<_>>();
                if ext["filter"] == "QUATERNION" {
                    optimesh::vertexfilter::decode_filter_quat(&mut values);
                } else {
                    optimesh::vertexfilter::decode_filter_oct16(&mut values);
                }
                for (out, v) in output.chunks_exact_mut(2).zip(values) {
                    out.copy_from_slice(&v.to_le_bytes());
                }
            }
            "EXPONENTIAL" if stride.is_multiple_of(4) => {
                let mut values = output
                    .chunks_exact(4)
                    .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
                    .collect::<Vec<_>>();
                optimesh::vertexfilter::decode_filter_exp(&mut values);
                for (out, v) in output.chunks_exact_mut(4).zip(values) {
                    out.copy_from_slice(&v.to_le_bytes());
                }
            }
            _ => return Err(Error::Invalid("meshopt filter/stride")),
        }
        let buffer = buffers.len();
        document["buffers"]
            .as_array_mut()
            .ok_or(Error::Invalid("glTF buffers"))?
            .push(json!({"byteLength":output.len()}));
        buffers.push(output);
        let view = &mut document["bufferViews"][i];
        view["buffer"] = json!(buffer);
        view["byteOffset"] = json!(0);
        view["byteLength"] = json!(size);
        if let Some(extensions) = view["extensions"].as_object_mut() {
            extensions.remove("EXT_meshopt_compression");
            extensions.remove("KHR_meshopt_compression");
        }
    }
    let mesh_count = document["meshes"].as_array().map_or(0, Vec::len);
    for mesh_index in 0..mesh_count {
        let primitives = document["meshes"][mesh_index]["primitives"]
            .as_array()
            .ok_or(Error::Invalid("glTF primitives"))?
            .len();
        for primitive_index in 0..primitives {
            let primitive = document["meshes"][mesh_index]["primitives"][primitive_index].clone();
            let ext = &primitive["extensions"]["KHR_draco_mesh_compression"];
            if ext.is_null() {
                continue;
            }
            let view = &document["bufferViews"][number(ext, "bufferView")?];
            let data = slice(
                &buffers,
                number(view, "buffer")?,
                view["byteOffset"].as_u64().unwrap_or(0) as usize,
                number(view, "byteLength")?,
            )?;
            let mut mesh = draco_core::Mesh::new();
            draco_core::MeshDecoder::new()
                .decode(&mut draco_core::DecoderBuffer::new(data), &mut mesh)
                .map_err(|e| Error::Asset(format!("Draco: {e:?}")))?;
            if mesh.num_points() > 16_000_000 {
                return Err(Error::Invalid("Draco vertex count"));
            }
            for (semantic, id) in ext["attributes"]
                .as_object()
                .ok_or(Error::Invalid("Draco attributes"))?
            {
                let accessor_index = primitive["attributes"][semantic]
                    .as_u64()
                    .ok_or(Error::Invalid("Draco accessor"))?
                    as usize;
                let attribute = mesh
                    .attribute_by_unique_id(
                        id.as_u64().ok_or(Error::Invalid("Draco attribute ID"))? as u32,
                    )
                    .ok_or(Error::Invalid("Draco missing attribute"))?;
                let count = attribute.num_components() as usize;
                let values = attribute.read_f32s(mesh.num_points(), count);
                let accessor = &document["accessors"][accessor_index];
                let component = number(accessor, "componentType")?;
                let mut data = Vec::new();
                for value in values {
                    match component {
                        5120 => data.push(value as i8 as u8),
                        5121 => data.push(value as u8),
                        5122 => data.extend((value as i16).to_le_bytes()),
                        5123 => data.extend((value as u16).to_le_bytes()),
                        5125 => data.extend((value as u32).to_le_bytes()),
                        5126 => data.extend(value.to_le_bytes()),
                        _ => return Err(Error::Invalid("Draco component type")),
                    }
                }
                let view = append_view(&mut document, &mut buffers, data)?;
                let accessor = &mut document["accessors"][accessor_index];
                accessor["bufferView"] = json!(view);
                accessor["byteOffset"] = json!(0);
                accessor["count"] = json!(mesh.num_points());
            }
            {
                let accessor_index = if let Some(index) = primitive["indices"].as_u64() {
                    index as usize
                } else {
                    let list = document["accessors"]
                        .as_array_mut()
                        .ok_or(Error::Invalid("Draco accessors"))?;
                    let index = list.len();
                    list.push(json!({"type":"SCALAR","componentType":5125,"count":0}));
                    document["meshes"][mesh_index]["primitives"][primitive_index]["indices"] =
                        json!(index);
                    index
                };
                let mut indices = Vec::new();
                for i in 0..mesh.num_faces() {
                    for index in mesh.face(draco_core::geometry_indices::FaceIndex(i as u32)) {
                        indices.extend(index.0.to_le_bytes());
                    }
                }
                let view = append_view(&mut document, &mut buffers, indices)?;
                let accessor = &mut document["accessors"][accessor_index];
                accessor["bufferView"] = json!(view);
                accessor["byteOffset"] = json!(0);
                accessor["componentType"] = json!(5125);
                accessor["count"] = json!(mesh.num_faces() * 3);
            }
            document["meshes"][mesh_index]["primitives"][primitive_index]["extensions"]
                .as_object_mut()
                .unwrap()
                .remove("KHR_draco_mesh_compression");
        }
    }
    // gltf-rs exposes positions/normals/tangents as f32. Expand quantized
    // accessor scalars explicitly, retaining node transforms and normalized semantics.
    let mut accessors = std::collections::BTreeSet::new();
    if let Some(meshes) = document["meshes"].as_array() {
        for mesh in meshes {
            if let Some(primitives) = mesh["primitives"].as_array() {
                for primitive in primitives {
                    let mut sets = vec![&primitive["attributes"]];
                    if let Some(targets) = primitive["targets"].as_array() {
                        sets.extend(targets);
                    }
                    for set in sets {
                        if let Some(attributes) = set.as_object() {
                            for (name, index) in attributes {
                                if (matches!(name.as_str(), "POSITION" | "NORMAL" | "TANGENT")
                                    || name.starts_with("TEXCOORD_"))
                                    && let Some(index) = index.as_u64()
                                {
                                    accessors.insert(index as usize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for index in accessors {
        let accessor = document["accessors"][index].clone();
        let component = number(&accessor, "componentType")?;
        if component == 5126 {
            continue;
        }
        if accessor.get("sparse").is_some() {
            return Err(Error::Invalid("quantized sparse accessor"));
        }
        let count = number(&accessor, "count")?;
        let components = match accessor["type"].as_str() {
            Some("VEC2") => 2,
            Some("VEC3") => 3,
            Some("VEC4") => 4,
            _ => return Err(Error::Invalid("quantized accessor dimensions")),
        };
        let width = match component {
            5120 | 5121 => 1,
            5122 | 5123 => 2,
            _ => return Err(Error::Invalid("quantized component type")),
        };
        let view = &document["bufferViews"][number(&accessor, "bufferView")?];
        let data = slice(
            &buffers,
            number(view, "buffer")?,
            view["byteOffset"].as_u64().unwrap_or(0) as usize,
            number(view, "byteLength")?,
        )?;
        let stride = view["byteStride"]
            .as_u64()
            .unwrap_or((width * components) as u64) as usize;
        let offset = accessor["byteOffset"].as_u64().unwrap_or(0) as usize;
        if count
            .checked_mul(components * 4)
            .is_none_or(|n| n > 256 * 1024 * 1024)
        {
            return Err(Error::Invalid("quantized decoded size"));
        }
        let mut decoded = Vec::with_capacity(count * components * 4);
        let mut min = vec![f32::INFINITY; components];
        let mut max = vec![f32::NEG_INFINITY; components];
        for vertex in 0..count {
            for c in 0..components {
                let start = offset
                    .checked_add(
                        vertex
                            .checked_mul(stride)
                            .ok_or(Error::Invalid("accessor stride"))?,
                    )
                    .and_then(|v| v.checked_add(c * width))
                    .ok_or(Error::Invalid("accessor offset"))?;
                let value = data
                    .get(start..start + width)
                    .ok_or(Error::Invalid("quantized accessor bounds"))?;
                let mut value = match component {
                    5120 => value[0] as i8 as f32,
                    5121 => value[0] as f32,
                    5122 => i16::from_le_bytes([value[0], value[1]]) as f32,
                    _ => u16::from_le_bytes([value[0], value[1]]) as f32,
                };
                if accessor["normalized"].as_bool().unwrap_or(false) {
                    value = (value
                        / match component {
                            5120 => 127.0,
                            5121 => 255.0,
                            5122 => 32767.0,
                            _ => 65535.0,
                        })
                    .max(-1.0);
                }
                min[c] = min[c].min(value);
                max[c] = max[c].max(value);
                decoded.extend(value.to_le_bytes());
            }
        }
        let view = append_view(&mut document, &mut buffers, decoded)?;
        let a = &mut document["accessors"][index];
        a["bufferView"] = json!(view);
        a["byteOffset"] = json!(0);
        a["componentType"] = json!(5126);
        a.as_object_mut().unwrap().remove("normalized");
        if a.get("min").is_some() {
            a["min"] = json!(min);
        }
        if a.get("max").is_some() {
            a["max"] = json!(max);
        }
    }
    if let Some(textures) = document.get_mut("textures").and_then(Value::as_array_mut) {
        for texture in textures {
            for name in ["KHR_texture_basisu", "EXT_texture_webp", "EXT_texture_avif"] {
                if let Some(source) = texture["extensions"][name]["source"].as_u64() {
                    texture["source"] = json!(source);
                    texture["extensions"].as_object_mut().unwrap().remove(name);
                }
            }
        }
    }
    for key in ["extensionsRequired", "extensionsUsed"] {
        if let Some(extensions) = document.get_mut(key).and_then(Value::as_array_mut) {
            extensions.retain(|v| {
                !matches!(
                    v.as_str(),
                    Some(
                        "EXT_meshopt_compression"
                            | "KHR_meshopt_compression"
                            | "KHR_draco_mesh_compression"
                            | "KHR_texture_basisu"
                            | "EXT_texture_webp"
                            | "EXT_texture_avif"
                            | "KHR_mesh_quantization"
                    )
                )
            });
        }
    }
    // Retain the node payload for the animated importer, which creates GPU instances.
    if let Some(required) = document
        .get_mut("extensionsRequired")
        .and_then(Value::as_array_mut)
    {
        required.retain(|v| v.as_str() != Some("EXT_mesh_gpu_instancing"));
    }
    // Discard unused fallback buffers while retaining their indices; no allocation
    // is needed for the original placeholder decompressed byteLength.
    for i in 0..document["buffers"]
        .as_array()
        .ok_or(Error::Invalid("glTF buffers"))?
        .len()
    {
        if !document["bufferViews"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["buffer"].as_u64() == Some(i as u64))
        {
            document["buffers"][i]["byteLength"] = json!(1);
            if buffers.len() <= i {
                buffers.resize_with(i + 1, Vec::new);
            }
            if buffers[i].is_empty() {
                buffers[i].push(0);
            }
        }
    }
    let asset = gltf::Gltf::from_slice(
        &serde_json::to_vec(&document).map_err(|e| Error::Asset(e.to_string()))?,
    )
    .map_err(|e| Error::Asset(e.to_string()))?;
    Ok(PreparedGltf { asset, buffers })
}

/// Transcode a 2D KTX2/Basis image to the renderer's RGBA representation.
pub fn decode_basis(bytes: &[u8], srgb: bool) -> Result<Texture> {
    let transcoder =
        basisu::Transcoder::new(bytes).map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
    if transcoder.face_count() != 1 || transcoder.layer_count() > 1 || transcoder.is_video() {
        return Err(Error::Invalid("layered/cube/video Basis image"));
    }
    let (width, height) = transcoder.base_dimensions();
    if width == 0 || height == 0 || u64::from(width) * u64::from(height) > 64 * 1024 * 1024 {
        return Err(Error::Invalid("Basis image dimensions"));
    }
    let rgba = transcoder
        .transcode(0, basisu::TargetFormat::Rgba32, basisu::DecodeFlags::NONE)
        .map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
    Texture::from_rgba(width, height, rgba, srgb)
}
