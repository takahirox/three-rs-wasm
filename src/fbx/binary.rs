//! FBXLoader's BinaryParser and BinaryReader.
use super::js::{Obj, Value};
use crate::{Error, Result};
use std::cell::RefCell;
use std::rc::Rc;

fn bad(what: &str) -> Error {
    Error::Asset(format!("THREE.FBXLoader: {what}"))
}

struct Reader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        let b = self
            .data
            .get(self.offset..self.offset + n)
            .ok_or_else(|| bad("Offset is outside the bounds of the DataView"))?;
        self.offset += n;
        Ok(b)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }
    fn boolean(&mut self) -> Result<bool> {
        Ok(self.u8()? & 1 == 1)
    }
    fn i16(&mut self) -> Result<i16> {
        Ok(i16::from_le_bytes(
            self.bytes(2)?.try_into().unwrap_or_default(),
        ))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(
            self.bytes(4)?.try_into().unwrap_or_default(),
        ))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.bytes(4)?.try_into().unwrap_or_default(),
        ))
    }
    /// getInt64(): the two's complement value in a double.
    fn i64(&mut self) -> Result<f64> {
        let low = self.u32()?;
        let high = self.u32()?;
        if high & 0x8000_0000 != 0 {
            let mut high = !high;
            let low = !low;
            if low == 0xffff_ffff {
                high = high.wrapping_add(1);
            }
            let low = low.wrapping_add(1);
            return Ok(-(f64::from(high) * 4294967296.0 + f64::from(low)));
        }
        Ok(f64::from(high) * 4294967296.0 + f64::from(low))
    }
    fn u64(&mut self) -> Result<f64> {
        let low = self.u32()?;
        let high = self.u32()?;
        Ok(f64::from(high) * 4294967296.0 + f64::from(low))
    }
    fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(
            self.bytes(4)?.try_into().unwrap_or_default(),
        ))
    }
    fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(
            self.bytes(8)?.try_into().unwrap_or_default(),
        ))
    }
    /// getString(): UTF-8, cut at the first NUL.
    fn string(&mut self, n: usize) -> Result<String> {
        let b = self.bytes(n)?;
        let b = b.iter().position(|&c| c == 0).map_or(b, |i| &b[..i]);
        // TextDecoder drops a leading byte order mark.
        let b = b.strip_prefix(b"\xef\xbb\xbf").unwrap_or(b);
        Ok(String::from_utf8_lossy(b).into_owned())
    }
    fn array(&mut self, kind: u8, n: usize) -> Result<Value> {
        Ok(match kind {
            b'b' | b'c' => {
                let mut a = Vec::with_capacity(n);
                for _ in 0..n {
                    a.push(Value::Bool(self.boolean()?));
                }
                Value::Array(Rc::new(RefCell::new(a)))
            }
            b'd' => {
                let mut a = Vec::with_capacity(n);
                for _ in 0..n {
                    a.push(self.f64()?);
                }
                Value::numbers(a)
            }
            b'f' => {
                let mut a = Vec::with_capacity(n);
                for _ in 0..n {
                    a.push(f64::from(self.f32()?));
                }
                Value::numbers(a)
            }
            b'i' => {
                let mut a = Vec::with_capacity(n);
                for _ in 0..n {
                    a.push(f64::from(self.i32()?));
                }
                Value::numbers(a)
            }
            _ => {
                let mut a = Vec::with_capacity(n);
                for _ in 0..n {
                    a.push(self.i64()?);
                }
                Value::numbers(a)
            }
        })
    }
}

/// isFbxFormatBinary().
pub(crate) fn is_binary(data: &[u8]) -> bool {
    data.starts_with(b"Kaydara FBX Binary  \0")
}

/// BinaryParser.parse(): the FBXTree.
pub(crate) fn parse(data: &[u8]) -> Result<Obj> {
    let mut reader = Reader { data, offset: 0 };
    reader.bytes(23)?;
    let version = reader.u32()?;
    if version < 6400 {
        return Err(bad(&format!(
            "FBX version not supported, FileVersion: {version}"
        )));
    }
    let tree = Obj::new();
    while !end_of_content(&reader) {
        if let Some(node) = parse_node(&mut reader, version)? {
            let name = node.get("name").to_key();
            tree.set(&name, Value::Object(node));
        }
    }
    Ok(tree)
}

fn end_of_content(reader: &Reader) -> bool {
    let size = reader.data.len();
    if size.is_multiple_of(16) {
        ((reader.offset + 160 + 16) & !0xf) >= size
    } else {
        reader.offset + 160 + 16 >= size
    }
}

fn parse_node(reader: &mut Reader, version: u32) -> Result<Option<Obj>> {
    let node = Obj::new();
    let wide = version >= 7500;
    let read = |r: &mut Reader| {
        if wide {
            r.u64()
        } else {
            r.u32().map(f64::from)
        }
    };
    let end_offset = read(reader)?;
    let num_properties = read(reader)?;
    read(reader)?;
    let name_len = reader.u8()? as usize;
    let name = reader.string(name_len)?;
    if end_offset == 0.0 {
        return Ok(None);
    }
    let mut properties = vec![];
    for _ in 0..num_properties as u64 {
        properties.push(parse_property(reader)?);
    }
    let id = properties.first().cloned().unwrap_or(Value::string(""));
    let attr_name = properties.get(1).cloned().unwrap_or(Value::string(""));
    let attr_type = properties.get(2).cloned().unwrap_or(Value::string(""));
    let single = num_properties == 1.0 && reader.offset as f64 == end_offset;
    node.set("singleProperty", Value::Bool(single));
    while end_offset > reader.offset as f64 {
        if let Some(sub) = parse_node(reader, version)? {
            parse_sub_node(&name, &node, sub);
        }
    }
    node.set(
        "propertyList",
        Value::Array(Rc::new(RefCell::new(properties))),
    );
    if matches!(id, Value::Number(_)) {
        node.set("id", id);
    }
    if !attr_name.strict_eq(&Value::string("")) {
        node.set("attrName", attr_name);
    }
    if !attr_type.strict_eq(&Value::string("")) {
        node.set("attrType", attr_type);
    }
    if !name.is_empty() {
        node.set("name", Value::string(&name));
    }
    Ok(Some(node))
}

fn property_list(node: &Obj) -> Vec<Value> {
    match node.get("propertyList") {
        Value::Array(a) => a.borrow().clone(),
        _ => vec![],
    }
}

fn parse_sub_node(name: &str, node: &Obj, sub: Obj) {
    let sub_name = sub.get("name").to_key();
    let list = property_list(&sub);
    if sub.get("singleProperty").strict_eq(&Value::Bool(true)) {
        let value = list.first().cloned().unwrap_or_default();
        if value.is_array() {
            node.set(&sub_name, Value::Object(sub.clone()));
            sub.set("a", value);
        } else {
            node.set(&sub_name, value);
        }
    } else if name == "Connections" && sub_name == "C" {
        let array: Vec<Value> = list.into_iter().skip(1).collect();
        if node.get("connections").is_undefined() {
            node.set("connections", Value::Array(Rc::new(RefCell::new(vec![]))));
        }
        if let Value::Array(a) = node.get("connections") {
            a.borrow_mut()
                .push(Value::Array(Rc::new(RefCell::new(array))));
        }
    } else if sub_name == "Properties70" {
        for key in sub.keys() {
            node.set(&key, sub.get(&key));
        }
    } else if name == "Properties70" && sub_name == "P" {
        let mut inner_name = list.first().map(Value::to_key).unwrap_or_default();
        let mut type1 = list.get(1).map(Value::to_key).unwrap_or_default();
        let type2 = list.get(2).cloned().unwrap_or_default();
        let flag = list.get(3).cloned().unwrap_or_default();
        if inner_name.starts_with("Lcl ") {
            inner_name = inner_name.replacen("Lcl ", "Lcl_", 1);
        }
        if type1.starts_with("Lcl ") {
            type1 = type1.replacen("Lcl ", "Lcl_", 1);
        }
        let value = if matches!(type1.as_str(), "Color" | "ColorRGB" | "Vector" | "Vector3D")
            || type1.starts_with("Lcl_")
        {
            Value::Array(Rc::new(RefCell::new(vec![
                list.get(4).cloned().unwrap_or_default(),
                list.get(5).cloned().unwrap_or_default(),
                list.get(6).cloned().unwrap_or_default(),
            ])))
        } else {
            list.get(4).cloned().unwrap_or_default()
        };
        let p = Obj::new();
        p.set("type", Value::string(&type1));
        p.set("type2", type2);
        p.set("flag", flag);
        p.set("value", value);
        node.set(&inner_name, Value::Object(p));
    } else if node.get(&sub_name).is_undefined() {
        match sub.get("id") {
            Value::Number(id) => {
                let container = Obj::new();
                container.set(&super::js::number_key(id), Value::Object(sub.clone()));
                node.set(&sub_name, Value::Object(container));
            }
            _ => node.set(&sub_name, Value::Object(sub.clone())),
        }
    } else if sub_name == "PoseNode" {
        let existing = node.get(&sub_name);
        let list = match existing {
            Value::Array(a) => a,
            other => {
                let a = Rc::new(RefCell::new(vec![other]));
                node.set(&sub_name, Value::Array(a.clone()));
                a
            }
        };
        list.borrow_mut().push(Value::Object(sub.clone()));
    } else {
        let key = sub.get("id").to_key();
        let existing = node.get(&sub_name);
        if let Some(container) = existing.as_obj()
            && container.get(&key).is_undefined()
        {
            container.set(&key, Value::Object(sub.clone()));
        }
    }
}

fn parse_property(reader: &mut Reader) -> Result<Value> {
    let kind = reader.string(1)?;
    Ok(match kind.as_str() {
        "C" => Value::Bool(reader.boolean()?),
        "D" => Value::Number(reader.f64()?),
        "F" => Value::Number(f64::from(reader.f32()?)),
        "I" => Value::Number(f64::from(reader.i32()?)),
        "L" => Value::Number(reader.i64()?),
        "R" => {
            let n = reader.u32()? as usize;
            Value::Bytes(Rc::new(reader.bytes(n)?.to_vec()))
        }
        "S" => {
            let n = reader.u32()? as usize;
            Value::string(&reader.string(n)?)
        }
        "Y" => Value::Number(f64::from(reader.i16()?)),
        "b" | "c" | "d" | "f" | "i" | "l" => {
            let t = kind.as_bytes()[0];
            let length = reader.u32()? as usize;
            let encoding = reader.u32()?;
            let compressed = reader.u32()? as usize;
            if encoding == 0 {
                reader.array(t, length)?
            } else {
                let data = miniz_oxide::inflate::decompress_to_vec_zlib(reader.bytes(compressed)?)
                    .map_err(|_| bad("invalid zlib data"))?;
                Reader {
                    data: &data,
                    offset: 0,
                }
                .array(t, length)?
            }
        }
        other => return Err(bad(&format!("Unknown property type {other}"))),
    })
}
