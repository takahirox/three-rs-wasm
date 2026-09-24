//! File formats of the pinned loaders: PDBLoader, AMFLoader (with fflate's zip
//! reader and the DOMParser tree it reads), and TIFFLoader's UTIF decoder with its
//! bundled pdf.js baseline JPEG decoder.
use crate::{Error, Result};
use std::collections::HashMap;

fn bad(what: &str) -> Error {
    Error::Asset(what.to_string())
}
/// `parseFloat`: leading whitespace, then the longest decimal-literal prefix.
pub(super) fn parse_float(s: &str) -> f64 {
    let t = s.trim_start();
    let b = t.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    if t[i..].starts_with("Infinity") {
        return if b.first() == Some(&b'-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    let digits = |i: &mut usize| {
        let start = *i;
        while *i < b.len() && b[*i].is_ascii_digit() {
            *i += 1;
        }
        *i > start
    };
    let mut any = digits(&mut i);
    if i < b.len() && b[i] == b'.' {
        let mut j = i + 1;
        if digits(&mut j) || any {
            any = true;
            i = j;
        }
    }
    if !any {
        return f64::NAN;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        if digits(&mut j) {
            i = j;
        }
    }
    t[..i].parse().unwrap_or(f64::NAN)
}
/// `parseInt( s )` in base 10; None is NaN.
fn parse_int(s: &str) -> Option<i64> {
    let t = s.trim_start();
    let (sign, digits) = match t.as_bytes().first() {
        Some(b'-') => (-1, &t[1..]),
        Some(b'+') => (1, &t[1..]),
        _ => (1, t),
    };
    let end = digits
        .bytes()
        .position(|c| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    digits[..end].parse::<i64>().ok().map(|v| sign * v)
}
/// `String.prototype.slice` on ASCII text.
fn slice(line: &str, start: usize, end: usize) -> &str {
    let end = end.min(line.len());
    line.get(start.min(end)..end).unwrap_or("")
}

// ---------------------------------------------------------------- PDB

pub(super) struct Atom {
    pub position: [f64; 3],
    pub color: [u8; 3],
    pub label: String,
}
pub(super) struct Molecule {
    pub atoms: Vec<Atom>,
    /// Bond end positions, as the loader's bond geometry lists them.
    pub bonds: Vec<([f64; 3], [f64; 3])>,
}
include!("cpk.rs");
/// PDBLoader.parse: ATOM/HETATM records and CONECT bonds without duplicates.
pub(super) fn parse_pdb(text: &str) -> Result<Molecule> {
    let (mut atoms, mut bonds) = (vec![], vec![]);
    let mut map: HashMap<i64, usize> = HashMap::new();
    let mut hashes = std::collections::HashSet::new();
    for line in text.split('\n') {
        if slice(line, 0, 4) == "ATOM" || slice(line, 0, 6) == "HETATM" {
            let x = parse_float(slice(line, 30, 37));
            let y = parse_float(slice(line, 38, 45));
            let z = parse_float(slice(line, 46, 53));
            let index = parse_int(slice(line, 6, 11)).map(|v| v - 1);
            let mut e = slice(line, 76, 78).trim().to_lowercase();
            if e.is_empty() {
                e = slice(line, 12, 14).trim().to_lowercase();
            }
            let color = CPK
                .iter()
                .find(|(k, _)| *k == e)
                .map(|(_, c)| *c)
                .ok_or_else(|| bad("PDB: unknown element"))?;
            let mut label = e.clone();
            if let Some(first) = label.get(..1) {
                label = first.to_uppercase() + &label[1..];
            }
            if let Some(index) = index {
                map.insert(index, atoms.len());
            }
            atoms.push(Atom {
                position: [x, y, z],
                color,
                label,
            });
        } else if slice(line, 0, 6) == "CONECT" {
            let Some(s) = parse_int(slice(line, 6, 11)) else {
                continue;
            };
            for start in [11, 16, 21, 26] {
                if let Some(e) = parse_int(slice(line, start, start + 5)).filter(|&e| e != 0)
                    && hashes.insert((s.min(e), s.max(e)))
                {
                    bonds.push((s - 1, e - 1));
                }
            }
        }
    }
    let bonds = bonds
        .into_iter()
        .map(|(a, b)| {
            let atom = |i: i64| {
                map.get(&i)
                    .map(|&k| atoms[k].position)
                    .ok_or_else(|| bad("PDB: bond to a missing atom"))
            };
            Ok((atom(a)?, atom(b)?))
        })
        .collect::<Result<_>>()?;
    Ok(Molecule { atoms, bonds })
}

// ---------------------------------------------------------------- ZIP and XML

/// fflate `unzipSync`: every stored or deflated entry, in central-directory order.
pub(super) fn unzip(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let u16_at = |o: usize| -> Result<usize> {
        data.get(o..o + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)
            .ok_or_else(|| bad("zip: truncated"))
    };
    let u32_at = |o: usize| -> Result<usize> {
        data.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
            .ok_or_else(|| bad("zip: truncated"))
    };
    let end = (0..data.len().saturating_sub(21))
        .rev()
        .find(|&o| data[o..o + 4] == [0x50, 0x4b, 0x05, 0x06])
        .ok_or_else(|| bad("zip: no end record"))?;
    let (count, mut o) = (u16_at(end + 10)?, u32_at(end + 16)?);
    let mut files = vec![];
    for _ in 0..count {
        if u32_at(o)? != 0x0201_4b50 {
            return Err(bad("zip: bad central record"));
        }
        let method = u16_at(o + 10)?;
        let size = u32_at(o + 20)?;
        let (name_len, extra, comment) = (u16_at(o + 28)?, u16_at(o + 30)?, u16_at(o + 32)?);
        let local = u32_at(o + 42)?;
        let name = String::from_utf8_lossy(
            data.get(o + 46..o + 46 + name_len)
                .ok_or_else(|| bad("zip: name"))?,
        )
        .into_owned();
        let start = local + 30 + u16_at(local + 26)? + u16_at(local + 28)?;
        let raw = data
            .get(start..start + size)
            .ok_or_else(|| bad("zip: data"))?;
        let bytes = match method {
            0 => raw.to_vec(),
            8 => miniz_oxide::inflate::decompress_to_vec(raw).map_err(|_| bad("zip: inflate"))?,
            _ => return Err(bad("zip: unsupported method")),
        };
        files.push((name, bytes));
        o += 46 + name_len + extra + comment;
    }
    Ok(files)
}
pub(super) enum XmlNode {
    Element(Element),
    Text(String),
}
pub(super) struct Element {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<XmlNode>,
}
impl Element {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
    pub fn elements(&self) -> impl Iterator<Item = &Element> {
        self.children.iter().filter_map(|c| match c {
            XmlNode::Element(e) => Some(e),
            XmlNode::Text(_) => None,
        })
    }
    /// `textContent`: all descendant text, in document order.
    pub fn text(&self) -> String {
        let mut out = String::new();
        fn walk(e: &Element, out: &mut String) {
            for c in &e.children {
                match c {
                    XmlNode::Text(t) => out.push_str(t),
                    XmlNode::Element(e) => walk(e, out),
                }
            }
        }
        walk(self, &mut out);
        out
    }
    /// `getElementsByTagName( name )[ 0 ]`: the first descendant in document order.
    pub fn first_descendant(&self, name: &str) -> Option<&Element> {
        for e in self.elements() {
            if e.name == name {
                return Some(e);
            }
            if let Some(found) = e.first_descendant(name) {
                return Some(found);
            }
        }
        None
    }
}
fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let Some(end) = tail.find(';') else {
            out.push_str(tail);
            return out;
        };
        let entity = &tail[1..end];
        let c = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ if entity.starts_with("#x") => u32::from_str_radix(&entity[2..], 16)
                .ok()
                .and_then(char::from_u32),
            _ if entity.starts_with('#') => entity[1..].parse().ok().and_then(char::from_u32),
            _ => None,
        };
        match c {
            Some(c) => out.push(c),
            None => out.push_str(&tail[..=end]),
        }
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}
/// `DOMParser.parseFromString( text, 'application/xml' ).documentElement`.
pub(super) fn parse_xml(text: &str) -> Result<Element> {
    let mut stack = vec![Element {
        name: String::new(),
        attributes: vec![],
        children: vec![],
    }];
    let mut rest = text;
    while !rest.is_empty() {
        if let Some(body) = rest.strip_prefix("<!--") {
            let end = body.find("-->").ok_or_else(|| bad("XML: comment"))?;
            rest = &body[end + 3..];
        } else if let Some(body) = rest.strip_prefix("<![CDATA[") {
            let end = body.find("]]>").ok_or_else(|| bad("XML: CDATA"))?;
            let top = stack.last_mut().expect("root");
            top.children.push(XmlNode::Text(body[..end].to_string()));
            rest = &body[end + 3..];
        } else if rest.starts_with("<?") || rest.starts_with("<!") {
            let end = rest.find('>').ok_or_else(|| bad("XML: declaration"))?;
            rest = &rest[end + 1..];
        } else if let Some(body) = rest.strip_prefix("</") {
            let end = body.find('>').ok_or_else(|| bad("XML: end tag"))?;
            let element = stack.pop().ok_or_else(|| bad("XML: unbalanced"))?;
            if element.name != body[..end].trim() || stack.is_empty() {
                return Err(bad("XML: mismatched end tag"));
            }
            let top = stack.last_mut().expect("parent");
            top.children.push(XmlNode::Element(element));
            rest = &body[end + 1..];
        } else if let Some(body) = rest.strip_prefix('<') {
            let end = body.find('>').ok_or_else(|| bad("XML: start tag"))?;
            let mut tag = &body[..end];
            let closed = tag.ends_with('/');
            if closed {
                tag = &tag[..tag.len() - 1];
            }
            let name_end = tag
                .find(|c: char| c.is_ascii_whitespace())
                .unwrap_or(tag.len());
            let mut element = Element {
                name: tag[..name_end].to_string(),
                attributes: vec![],
                children: vec![],
            };
            let mut attrs = tag[name_end..].trim_start();
            while let Some(eq) = attrs.find('=') {
                let key = attrs[..eq].trim().to_string();
                let value = attrs[eq + 1..].trim_start();
                let quote = value.chars().next().ok_or_else(|| bad("XML: attribute"))?;
                let close = value[1..]
                    .find(quote)
                    .ok_or_else(|| bad("XML: attribute"))?;
                element
                    .attributes
                    .push((key, unescape(&value[1..1 + close])));
                attrs = value[close + 2..].trim_start();
            }
            if closed {
                let top = stack.last_mut().expect("parent");
                top.children.push(XmlNode::Element(element));
            } else {
                stack.push(element);
            }
            rest = &body[end + 1..];
        } else {
            let end = rest.find('<').unwrap_or(rest.len());
            let top = stack.last_mut().expect("parent");
            top.children.push(XmlNode::Text(unescape(&rest[..end])));
            rest = &rest[end..];
        }
    }
    let mut document = stack
        .pop()
        .filter(|_| stack.is_empty())
        .ok_or_else(|| bad("XML: unclosed element"))?;
    document
        .children
        .drain(..)
        .find_map(|c| match c {
            XmlNode::Element(e) => Some(e),
            XmlNode::Text(_) => None,
        })
        .ok_or_else(|| bad("XML: no document element"))
}

// ---------------------------------------------------------------- AMF

pub(super) struct AmfMaterial {
    /// `new Color( r, g, b )` from the text values: linear, unconverted.
    pub color: [f64; 3],
    pub opacity: Option<f64>,
}
pub(super) struct AmfVolume {
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub index: Vec<u32>,
    /// None uses the object's (or the loader's default) material.
    pub material: Option<AmfMaterial>,
    /// The mesh color, when the object has one.
    pub object_color: Option<AmfMaterial>,
}
fn amf_color(e: &Element) -> AmfMaterial {
    let mut c = [1., 1., 1., 1.];
    for child in e.elements() {
        let k = match child.name.as_str() {
            "r" => 0,
            "g" => 1,
            "b" => 2,
            "a" => 3,
            _ => continue,
        };
        c[k] = parse_float(&child.text());
    }
    AmfMaterial {
        color: [c[0], c[1], c[2]],
        opacity: (c[3] != 1.).then_some(c[3]),
    }
}
/// AMFLoader.parse: the zip entry ending in .amf, materials, then every object's
/// mesh volumes, scaled to millimeters.
pub(super) fn parse_amf(data: &[u8]) -> Result<Vec<Vec<AmfVolume>>> {
    let text = if data.starts_with(b"PK") {
        let files = unzip(data)?;
        let index = files
            .iter()
            .position(|(name, _)| name.to_lowercase().ends_with(".amf"))
            .unwrap_or(files.len().saturating_sub(1));
        String::from_utf8_lossy(&files.get(index).ok_or_else(|| bad("AMF: empty zip"))?.1)
            .into_owned()
    } else {
        String::from_utf8_lossy(data).into_owned()
    };
    let root = parse_xml(&text)?;
    if root.name.to_lowercase() != "amf" {
        return Err(bad("AMF: no AMF document"));
    }
    let unit = root.attribute("unit").map(str::to_lowercase);
    let scale = match unit.as_deref().unwrap_or("millimeter") {
        "inch" => 25.4,
        "feet" => 304.8,
        "meter" => 1000.,
        "micron" => 0.001,
        _ => 1.,
    };
    let mut materials: HashMap<String, AmfMaterial> = HashMap::new();
    let mut objects: Vec<(String, &Element)> = vec![];
    for child in root.elements() {
        match child.name.as_str() {
            "material" => {
                let id = child.attribute("id").unwrap_or_default().to_string();
                // loadMaterials: the last color child wins; no color is opaque white.
                let color = child
                    .elements()
                    .filter(|e| e.name == "color")
                    .last()
                    .map(amf_color)
                    .unwrap_or(AmfMaterial {
                        color: [1., 1., 1.],
                        opacity: None,
                    });
                materials.insert(id, color);
            }
            "object" => {
                let id = child.attribute("id").unwrap_or_default().to_string();
                // `amfObjects[ id ] = ...`: a repeated id keeps its first position.
                match objects.iter_mut().find(|(k, _)| *k == id) {
                    Some(entry) => entry.1 = child,
                    None => objects.push((id, child)),
                }
            }
            _ => {}
        }
    }
    // JavaScript objects list integer-like keys first, in ascending order.
    let integer = |k: &str| {
        k.parse::<u32>()
            .ok()
            .filter(|v| v.to_string() == k && *v != u32::MAX)
    };
    objects.sort_by_key(|(k, _)| integer(k).map_or((1, 0), |v| (0, v)));
    let mut result = vec![];
    for (_, object) in objects {
        let mut color = None;
        let mut volumes = vec![];
        for node in object.elements() {
            match node.name.as_str() {
                "color" => color = Some(amf_color(node)),
                "mesh" => {
                    let (mut positions, mut normals) = (vec![], vec![]);
                    let mut mesh_volumes = vec![];
                    for part in node.elements() {
                        if part.name == "vertices" {
                            for vertex in part.elements().filter(|e| e.name == "vertex") {
                                for v in vertex.elements() {
                                    let (keys, out) = match v.name.as_str() {
                                        "coordinates" => (["x", "y", "z"], &mut positions),
                                        "normal" => (["nx", "ny", "nz"], &mut normals),
                                        _ => continue,
                                    };
                                    for k in keys {
                                        let text = v
                                            .first_descendant(k)
                                            .ok_or_else(|| bad("AMF: coordinate"))?
                                            .text();
                                        out.push(parse_float(&text));
                                    }
                                }
                            }
                        } else if part.name == "volume" {
                            let mut index = vec![];
                            for t in part.elements().filter(|e| e.name == "triangle") {
                                for k in ["v1", "v2", "v3"] {
                                    let text = t
                                        .first_descendant(k)
                                        .ok_or_else(|| bad("AMF: triangle"))?
                                        .text();
                                    index.push(parse_float(&text) as u32);
                                }
                            }
                            mesh_volumes
                                .push((part.attribute("materialid").map(String::from), index));
                        }
                    }
                    // `geometry.scale( s, s, s )` on the Float32 attributes.
                    let positions: Vec<f32> = positions
                        .iter()
                        .map(|&v| (v as f32 as f64 * scale) as f32)
                        .collect();
                    // The scale's normal matrix renormalizes the Float32 normals.
                    let normals = (!normals.is_empty()).then(|| {
                        normals
                            .chunks(3)
                            .flat_map(|n| {
                                let v =
                                    [n[0] as f32 as f64, n[1] as f32 as f64, n[2] as f32 as f64];
                                let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                                let l = if l == 0. { 1. } else { l };
                                v.map(|c| (c / l) as f32)
                            })
                            .collect::<Vec<f32>>()
                    });
                    for (material, index) in mesh_volumes {
                        volumes.push(AmfVolume {
                            positions: positions.clone(),
                            normals: normals.clone(),
                            index,
                            material: material.and_then(|id| materials.get(&id)).map(|m| {
                                AmfMaterial {
                                    color: m.color,
                                    opacity: m.opacity,
                                }
                            }),
                            object_color: color.as_ref().map(|c| AmfMaterial {
                                color: c.color,
                                opacity: c.opacity,
                            }),
                        });
                    }
                }
                _ => {}
            }
        }
        result.push(volumes);
    }
    Ok(result)
}

// ---------------------------------------------------------------- TIFF

/// UTIF.decode, decodeImage and toRGBA8 for the first image: uncompressed, LZW
/// and new-style JPEG strips, the horizontal predictor, and palette or RGB color.
pub(super) fn decode_tiff(data: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let le = data.get(..2) == Some(b"II");
    let rd = |o: usize, n: usize| -> Result<u64> {
        let b = data.get(o..o + n).ok_or_else(|| bad("TIFF: truncated"))?;
        Ok(if le {
            b.iter().rev().fold(0, |a, &x| a << 8 | x as u64)
        } else {
            b.iter().fold(0, |a, &x| a << 8 | x as u64)
        })
    };
    let ifd = rd(4, 4)? as usize;
    let mut tags: HashMap<u16, Vec<u64>> = HashMap::new();
    let mut bytes_tags: HashMap<u16, Vec<u8>> = HashMap::new();
    for i in 0..rd(ifd, 2)? as usize {
        let o = ifd + 2 + i * 12;
        let (tag, ty, count) = (rd(o, 2)? as u16, rd(o + 2, 2)?, rd(o + 4, 4)? as usize);
        let size = match ty {
            1 | 2 | 6 | 7 => 1,
            3 | 8 => 2,
            4 | 9 => 4,
            _ => continue,
        };
        let at = if size * count > 4 {
            rd(o + 8, 4)? as usize
        } else {
            o + 8
        };
        if ty == 7 || ty == 1 || ty == 2 {
            bytes_tags.insert(
                tag,
                data.get(at..at + count)
                    .ok_or_else(|| bad("TIFF: tag"))?
                    .to_vec(),
            );
        }
        tags.insert(
            tag,
            (0..count)
                .map(|k| rd(at + k * size, size))
                .collect::<Result<_>>()?,
        );
    }
    let one = |t: u16, default: u64| {
        tags.get(&t)
            .and_then(|v| v.first().copied())
            .unwrap_or(default)
    };
    let (w, h) = (one(256, 0) as usize, one(257, 0) as usize);
    let cmpr = one(259, 1);
    let spp = one(277, 1) as usize;
    let bps = one(258, 1) as usize;
    let mut intp = one(262, 2);
    let bipl = (w * bps * spp).div_ceil(8) * 8;
    let bpl = bipl / 8;
    let soff = tags.get(&273).cloned().ok_or_else(|| bad("TIFF: strips"))?;
    let mut bcnt = tags.get(&279).cloned().unwrap_or_default();
    if cmpr == 1 && soff.len() == 1 {
        bcnt = vec![(h * bpl) as u64];
    }
    let rps = (one(278, h as u64) as usize).min(h);
    let mut bytes = vec![0u8; h * bpl];
    let mut lzw = Lzw::default();
    let mut bilen = 0;
    for (i, &off) in soff.iter().enumerate() {
        let (off, len) = (
            off as usize,
            *bcnt.get(i).ok_or_else(|| bad("TIFF: counts"))? as usize,
        );
        let toff = bilen / 8;
        match cmpr {
            1 => {
                for j in 0..len {
                    if let (Some(t), Some(&s)) = (bytes.get_mut(toff + j), data.get(off + j)) {
                        *t = s;
                    }
                }
            }
            5 => lzw.decode(data, off, len, &mut bytes, toff),
            7 => {
                let len = len.min(data.len() - off);
                let mut buff = vec![];
                if let Some(tables) = bytes_tags.get(&347) {
                    for k in 0..tables.len().saturating_sub(1) {
                        if tables[k] == 255 && tables[k + 1] == 0xD9 {
                            break;
                        }
                        buff.push(tables[k]);
                    }
                    if data[off] != 255 || data[off + 1] != 0xD8 {
                        buff.extend([data[off], data[off + 1]]);
                    }
                    buff.extend_from_slice(&data[off + 2..off + len]);
                } else {
                    buff.extend_from_slice(&data[off..off + len]);
                }
                let decoded = decode_jpeg(&buff)?;
                for (k, v) in decoded.into_iter().enumerate() {
                    if let Some(t) = bytes.get_mut(toff + k) {
                        *t = v;
                    }
                }
                if intp == 6 {
                    intp = 2;
                }
            }
            _ => return Err(bad("TIFF: unsupported compression")),
        }
        if one(317, 1) == 2 && bps == 8 {
            let bpp = spp;
            for y in 0..rps {
                let row = toff + y * bpl;
                if row + bpl > bytes.len() {
                    break;
                }
                for j in bpp..bpl {
                    bytes[row + j] = bytes[row + j].wrapping_add(bytes[row + j - bpp]);
                }
            }
        }
        bilen += bipl * rps;
    }
    let mut rgba = vec![0u8; w * h * 4];
    match (intp, bps) {
        (2, 8) => {
            for i in 0..w * h {
                let t = i * spp;
                rgba[i * 4..i * 4 + 3].copy_from_slice(&bytes[t..t + 3]);
                rgba[i * 4 + 3] = if spp >= 4 { bytes[t + 3] } else { 255 };
            }
        }
        (3, 8) => {
            let map = tags.get(&320).ok_or_else(|| bad("TIFF: palette"))?;
            let cn = 1 << bps;
            let alpha = spp > 1 && one(338, 0) != 0;
            for y in 0..h {
                for x in 0..w {
                    let (i, mi) = (y * w + x, bytes[y * bpl + x * spp] as usize);
                    rgba[i * 4] = (map[mi] >> 8) as u8;
                    rgba[i * 4 + 1] = (map[cn + mi] >> 8) as u8;
                    rgba[i * 4 + 2] = (map[2 * cn + mi] >> 8) as u8;
                    rgba[i * 4 + 3] = if alpha {
                        bytes[y * bpl + x * spp + 1]
                    } else {
                        255
                    };
                }
            }
        }
        _ => return Err(bad("TIFF: unsupported color")),
    }
    Ok((w as u32, h as u32, rgba))
}
/// UTIF.decode._decodeLZW, including its code table kept between strips.
struct Lzw {
    table: Vec<u32>,
    base: u32,
}
impl Default for Lzw {
    fn default() -> Self {
        Self {
            table: vec![0; 4096 * 4],
            base: 0,
        }
    }
}
impl Lzw {
    fn decode(&mut self, data: &[u8], off: usize, len: usize, out: &mut [u8], toff: usize) {
        let h = &mut self.table;
        let q = 8;
        if self.base != q {
            self.base = q;
            let n = (1 << q) + 1;
            for a in 0..(n + 1) as usize {
                h[4 * a] = a as u32;
                h[4 * a + 3] = a as u32;
                h[4 * a + 1] = 65535;
                h[4 * a + 2] = 1;
            }
        }
        let (clear, eoi) = (1u32 << q, (1u32 << q) + 1);
        let byte = |i: usize| data.get(i).copied().unwrap_or(0) as u32;
        let (mut e, end) = (off << 3, (off + len) << 3);
        let (mut width, mut next) = (q + 1, eoi + 1);
        let mut u = toff;
        let read = |e: &mut usize, width: u32| {
            let s = *e >> 3;
            let a = byte(s) << 16 | byte(s + 1) << 8 | byte(s + 2);
            let j = a >> (24 - (*e as u32 & 7) - width) & ((1 << width) - 1);
            *e += width as usize;
            j
        };
        let emit = |h: &[u32], code: u32, u: &mut usize, out: &mut [u8]| {
            let mut a = code << 2;
            let j = h[a as usize + 2] as usize;
            let mut pos = *u as isize + j as isize - 1;
            while a != 65535 {
                if let Some(t) = out.get_mut(pos as usize) {
                    *t = h[a as usize] as u8;
                }
                pos -= 1;
                a = h[a as usize + 1];
            }
            *u += j;
        };
        let add = |h: &mut [u32], s: u32, a: u32, next: &mut u32, width: &mut u32| {
            let (j, a4, s4) = ((*next << 2) as usize, (a << 2) as usize, s << 2);
            h[j] = h[a4 + 3];
            h[j + 1] = s4;
            h[j + 2] = h[s4 as usize + 2] + 1;
            h[j + 3] = h[s4 as usize + 3];
            *next += 1;
            if *next + 1 == 1 << *width && *width != 12 {
                *width += 1;
            }
        };
        let mut t = 0;
        while e < end {
            let mut code = read(&mut e, width);
            if code == eoi {
                break;
            }
            if code == clear {
                width = q + 1;
                next = eoi + 1;
                code = read(&mut e, width);
                if code == eoi {
                    break;
                }
                emit(h, code, &mut u, out);
            } else if code < next {
                emit(h, code, &mut u, out);
                add(h, t, code, &mut next, &mut width);
            } else {
                add(h, t, t, &mut next, &mut width);
                emit(h, next - 1, &mut u, out);
            }
            t = code;
        }
    }
}

// ---------------------------------------------------------------- JPEG

const ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
/// A canonical Huffman table, decoded bit by bit as the pdf.js code tree is.
#[derive(Clone, Default)]
struct Huffman {
    /// (length, code) → symbol.
    codes: HashMap<(u8, u16), u8>,
}
impl Huffman {
    fn new(counts: &[u8; 16], symbols: &[u8]) -> Self {
        let (mut codes, mut code, mut k) = (HashMap::new(), 0u16, 0);
        for (len, &n) in counts.iter().enumerate() {
            for _ in 0..n {
                if let Some(&s) = symbols.get(k) {
                    codes.insert((len as u8 + 1, code), s);
                }
                code += 1;
                k += 1;
            }
            code <<= 1;
        }
        Self { codes }
    }
}
struct Component {
    id: u8,
    h: usize,
    v: usize,
    quant: usize,
    blocks_per_line: usize,
    blocks_per_column: usize,
    data: Vec<i16>,
    pred: i32,
    dc: usize,
    ac: usize,
}
struct Bits<'a> {
    data: &'a [u8],
    offset: usize,
    value: u32,
    count: u32,
}
enum Stop {
    Eoi,
    Error(Error),
}
impl Bits<'_> {
    fn bit(&mut self) -> std::result::Result<u32, Stop> {
        if self.count > 0 {
            self.count -= 1;
            return Ok(self.value >> self.count & 1);
        }
        self.value = *self.data.get(self.offset).ok_or(Stop::Eoi)? as u32;
        self.offset += 1;
        if self.value == 255 {
            let next = self.data.get(self.offset).copied().unwrap_or(0);
            self.offset += 1;
            if next != 0 {
                return Err(if next == 0xD9 {
                    Stop::Eoi
                } else {
                    Stop::Error(bad("JPEG: unexpected marker"))
                });
            }
        }
        self.count = 7;
        Ok(self.value >> 7)
    }
    fn receive(&mut self, n: u32) -> std::result::Result<i32, Stop> {
        let mut v = 0i32;
        for _ in 0..n {
            v = v << 1 | self.bit()? as i32;
        }
        Ok(v)
    }
    fn extend(&mut self, n: u32) -> std::result::Result<i32, Stop> {
        if n == 1 {
            return Ok(if self.bit()? == 1 { 1 } else { -1 });
        }
        let v = self.receive(n)?;
        Ok(if v >= 1 << (n - 1) {
            v
        } else {
            v + (-1 << n) + 1
        })
    }
    fn huffman(&mut self, table: &Huffman) -> std::result::Result<u8, Stop> {
        let mut code = 0u16;
        for len in 1..=16u8 {
            code = code << 1 | self.bit()? as u16;
            if let Some(&s) = table.codes.get(&(len, code)) {
                return Ok(s);
            }
        }
        Err(Stop::Error(bad("JPEG: invalid huffman sequence")))
    }
}
fn be16(d: &[u8], o: usize) -> usize {
    (d.get(o).copied().unwrap_or(0) as usize) << 8 | d.get(o + 1).copied().unwrap_or(0) as usize
}
/// pdf.js `findNextFileMarker`.
fn next_marker(d: &[u8], pos: usize) -> Option<(usize, usize)> {
    let max = d.len().checked_sub(1)?;
    if pos >= max {
        return None;
    }
    let mut n = pos;
    loop {
        let m = be16(d, n);
        if (0xFFC0..=0xFFFE).contains(&m) {
            return Some((m, n));
        }
        n += 1;
        if n >= max {
            return None;
        }
    }
}
/// UTIF's pdf.js JpegDecoder for baseline frames: Huffman decoding, the integer
/// `quantizeAndInverse`, and `getData( forceRGB )` with its RGB-id rule.
pub(super) fn decode_jpeg(d: &[u8]) -> Result<Vec<u8>> {
    if be16(d, 0) != 0xFFD8 {
        return Err(bad("JPEG: SOI not found"));
    }
    let mut o = 2;
    let mut quant: Vec<[u16; 64]> = vec![[0; 64]; 4];
    let (mut dc, mut ac) = (vec![Huffman::default(); 4], vec![Huffman::default(); 4]);
    let mut components: Vec<Component> = vec![];
    let (mut width, mut height, mut max_h, mut max_v) = (0, 0, 1, 1);
    let (mut mcus_per_line, mut mcus_per_column) = (0, 0);
    let mut restart = 0;
    let mut adobe_transform: Option<u8> = None;
    let mut marker = be16(d, o);
    o += 2;
    'markers: while marker != 0xFFD9 {
        match marker {
            0xFFE0..=0xFFEF | 0xFFFE => {
                let len = be16(d, o);
                let start = o + 2;
                let end = start + len - 2;
                let seg = d.get(start..end.min(d.len())).unwrap_or(&[]);
                if marker == 0xFFEE && seg.starts_with(b"Adobe") && seg.len() > 11 {
                    adobe_transform = Some(seg[11]);
                }
                o = end;
            }
            0xFFDB => {
                let end = o + be16(d, o);
                o += 2;
                while o < end {
                    let spec = d[o];
                    o += 1;
                    let mut table = [0u16; 64];
                    for &z in &ZIGZAG {
                        if spec >> 4 == 0 {
                            table[z] = d[o] as u16;
                            o += 1;
                        } else {
                            table[z] = be16(d, o) as u16;
                            o += 2;
                        }
                    }
                    quant[(spec & 15) as usize] = table;
                }
            }
            0xFFC0 | 0xFFC1 => {
                o += 2;
                o += 1;
                height = be16(d, o);
                width = be16(d, o + 2);
                o += 4;
                let n = d[o] as usize;
                o += 1;
                for _ in 0..n {
                    let (h, v) = ((d[o + 1] >> 4) as usize, (d[o + 1] & 15) as usize);
                    max_h = max_h.max(h);
                    max_v = max_v.max(v);
                    components.push(Component {
                        id: d[o],
                        h,
                        v,
                        quant: d[o + 2] as usize,
                        blocks_per_line: 0,
                        blocks_per_column: 0,
                        data: vec![],
                        pred: 0,
                        dc: 0,
                        ac: 0,
                    });
                    o += 3;
                }
                mcus_per_line = (width as f64 / 8. / max_h as f64).ceil() as usize;
                mcus_per_column = (height as f64 / 8. / max_v as f64).ceil() as usize;
                for c in &mut components {
                    c.blocks_per_line =
                        ((width as f64 / 8.).ceil() * c.h as f64 / max_h as f64).ceil() as usize;
                    c.blocks_per_column =
                        ((height as f64 / 8.).ceil() * c.v as f64 / max_v as f64).ceil() as usize;
                    let (lines, columns) = (mcus_per_line * c.h, mcus_per_column * c.v);
                    c.data = vec![0; 64 * columns * (lines + 1)];
                }
            }
            0xFFC2 => return Err(bad("JPEG: progressive frames are not supported")),
            0xFFC4 => {
                let len = be16(d, o);
                o += 2;
                let mut k = 2;
                while k < len {
                    let spec = d[o];
                    o += 1;
                    let mut counts = [0u8; 16];
                    let mut total = 0;
                    for c in &mut counts {
                        *c = d[o];
                        total += d[o] as usize;
                        o += 1;
                    }
                    let symbols = d[o..o + total].to_vec();
                    o += total;
                    k += 17 + total;
                    let table = Huffman::new(&counts, &symbols);
                    if spec >> 4 == 0 {
                        dc[(spec & 15) as usize] = table;
                    } else {
                        ac[(spec & 15) as usize] = table;
                    }
                }
            }
            0xFFDD => {
                o += 2;
                restart = be16(d, o);
                o += 2;
            }
            0xFFDA => {
                o += 2;
                let n = d[o] as usize;
                o += 1;
                let mut scan = vec![];
                for _ in 0..n {
                    let id = d[o];
                    let tables = d[o + 1];
                    o += 2;
                    let index = components
                        .iter()
                        .position(|c| c.id == id)
                        .ok_or_else(|| bad("JPEG: scan component"))?;
                    components[index].dc = (tables >> 4) as usize;
                    components[index].ac = (tables & 15) as usize;
                    scan.push(index);
                }
                // Spectral selection and approximation: baseline scans ignore them.
                o += 3;
                match decode_scan(
                    d,
                    o,
                    &mut components,
                    &scan,
                    (&dc, &ac),
                    (mcus_per_line, mcus_per_column, restart),
                ) {
                    Ok(n) => o += n,
                    Err(Stop::Eoi) => break 'markers,
                    Err(Stop::Error(e)) => return Err(e),
                }
            }
            0xFFDC => o += 4,
            0xFFFF => {
                if d.get(o) != Some(&255) {
                    o -= 1;
                }
            }
            _ => {
                if let Some((_, at)) = next_marker(d, o - 2).filter(|&(_, at)| at != o - 2) {
                    o = at;
                } else if o >= d.len() - 1 {
                    break;
                } else {
                    return Err(bad("JPEG: unknown marker"));
                }
            }
        }
        marker = be16(d, o);
        o += 2;
    }
    // buildComponentData: quantizeAndInverse over every block.
    let mut tmp = [0i16; 64];
    for c in &mut components {
        let q = quant[c.quant];
        for row in 0..c.blocks_per_column {
            for col in 0..c.blocks_per_line {
                let offset = 64 * ((c.blocks_per_line + 1) * row + col);
                inverse(&q, &mut c.data[offset..offset + 64], &mut tmp);
            }
        }
    }
    // getData with forceRGB: interleave, then the YCbCr transform unless the
    // component ids spell R, G, B (or an Adobe marker says otherwise).
    let n = components.len();
    let mut out = vec![0u8; width * height * n];
    for (k, c) in components.iter().enumerate() {
        let (sx, sy) = (c.h as f64 / max_h as f64, c.v as f64 / max_v as f64);
        let line = (c.blocks_per_line + 1) << 3;
        let xs: Vec<usize> = (0..width)
            .map(|x| {
                let z = (x as f64 * sx) as usize;
                ((z & !7) << 3) | (z & 7)
            })
            .collect();
        for y in 0..height {
            let z = (y as f64 * sy) as usize;
            let base = (line * (z & !7)) | ((z & 7) << 3);
            for x in 0..width {
                out[(y * width + x) * n + k] = c.data[base + xs[x]].clamp(0, 255) as u8;
            }
        }
    }
    let transform = match adobe_transform {
        Some(t) => t != 0,
        None => {
            n == 3
                && !(components[0].id == b'R'
                    && components[1].id == b'G'
                    && components[2].id == b'B')
        }
    };
    if n == 1 {
        return Ok(out.iter().flat_map(|&v| [v, v, v]).collect());
    }
    if n == 3 && transform {
        for p in out.chunks_mut(3) {
            let (y, cb, cr) = (p[0] as f64, p[1] as f64, p[2] as f64);
            let clamp = |v: f64| v.round_ties_even().clamp(0., 255.) as u8;
            p[0] = clamp(y - 179.456 + 1.402 * cr);
            p[1] = clamp(y + 135.459 - 0.344 * cb - 0.714 * cr);
            p[2] = clamp(y - 226.816 + 1.772 * cb);
        }
    } else if n != 3 {
        return Err(bad("JPEG: unsupported color mode"));
    }
    Ok(out)
}
fn decode_scan(
    d: &[u8],
    start: usize,
    components: &mut [Component],
    scan: &[usize],
    (dc, ac): (&[Huffman], &[Huffman]),
    (mcus_per_line, mcus_per_column, restart): (usize, usize, usize),
) -> std::result::Result<usize, Stop> {
    let mut bits = Bits {
        data: d,
        offset: start,
        value: 0,
        count: 0,
    };
    let expected = if scan.len() == 1 {
        let c = &components[scan[0]];
        c.blocks_per_line * c.blocks_per_column
    } else {
        mcus_per_line * mcus_per_column
    };
    let block =
        |bits: &mut Bits, c: &mut Component, offset: usize| -> std::result::Result<(), Stop> {
            let t = bits.huffman(&dc[c.dc])?;
            let diff = if t == 0 { 0 } else { bits.extend(t as u32)? };
            c.pred += diff;
            c.data[offset] = c.pred as i16;
            let mut k = 1;
            while k < 64 {
                let rs = bits.huffman(&ac[c.ac])?;
                let (s, r) = ((rs & 15) as u32, (rs >> 4) as usize);
                if s == 0 {
                    if r < 15 {
                        break;
                    }
                    k += 16;
                    continue;
                }
                k += r;
                let value = bits.extend(s)?;
                if let Some(&z) = ZIGZAG.get(k) {
                    c.data[offset + z] = value as i16;
                }
                k += 1;
            }
            Ok(())
        };
    let mut mcu = 0;
    while mcu <= expected {
        let count = if restart > 0 {
            (expected - mcu).min(restart)
        } else {
            expected
        };
        if count > 0 {
            for &i in scan {
                components[i].pred = 0;
            }
            for _ in 0..count {
                if scan.len() == 1 {
                    let c = &mut components[scan[0]];
                    let (row, col) = (mcu / c.blocks_per_line, mcu % c.blocks_per_line);
                    let offset = 64 * ((c.blocks_per_line + 1) * row + col);
                    block(&mut bits, c, offset)?;
                } else {
                    for &i in scan {
                        let c = &mut components[i];
                        for v in 0..c.v {
                            for h in 0..c.h {
                                let row = (mcu / mcus_per_line) * c.v + v;
                                let col = (mcu % mcus_per_line) * c.h + h;
                                let offset = 64 * ((c.blocks_per_line + 1) * row + col);
                                if offset + 64 <= c.data.len() {
                                    block(&mut bits, c, offset)?;
                                }
                            }
                        }
                    }
                }
                mcu += 1;
            }
        }
        bits.count = 0;
        let Some((m, at)) = next_marker(d, bits.offset) else {
            break;
        };
        bits.offset = at;
        if (0xFFD0..=0xFFD7).contains(&m) {
            bits.offset += 2;
        } else {
            break;
        }
    }
    Ok(bits.offset - start)
}
/// pdf.js `quantizeAndInverse`: the integer row pass into an Int16 buffer, then
/// the column pass with its 0..255 clamps written back into the block.
fn inverse(q: &[u16; 64], block: &mut [i16], p: &mut [i16; 64]) {
    const C1: i32 = 4017;
    const S1: i32 = 799;
    const C3: i32 = 3406;
    const S3: i32 = 2276;
    const C6: i32 = 1567;
    const S6: i32 = 3784;
    const SQRT2: i32 = 5793;
    const SQRT1D2: i32 = 2896;
    let m = |a: i32, b: i32| a.wrapping_mul(b);
    for r in (0..64).step_by(8) {
        let mut v = [0i32; 8];
        for k in 0..8 {
            v[k] = block[r + k] as i32;
        }
        let mut p0 = m(v[0], q[r] as i32);
        if v[1..].iter().all(|&x| x == 0) {
            let t = (m(SQRT2, p0) + 512) >> 10;
            for k in 0..8 {
                p[r + k] = t as i16;
            }
            continue;
        }
        let p1 = m(v[1], q[r + 1] as i32);
        let p2 = m(v[2], q[r + 2] as i32);
        let p3 = m(v[3], q[r + 3] as i32);
        let p4 = m(v[4], q[r + 4] as i32);
        let p5 = m(v[5], q[r + 5] as i32);
        let p6 = m(v[6], q[r + 6] as i32);
        let p7 = m(v[7], q[r + 7] as i32);
        p0 = (m(SQRT2, p0) + 128) >> 8;
        let mut v1 = (m(SQRT2, p4) + 128) >> 8;
        let mut v2 = p2;
        let mut v3 = p6;
        let mut v4 = (m(SQRT1D2, p1 - p7) + 128) >> 8;
        let mut v7 = (m(SQRT1D2, p1 + p7) + 128) >> 8;
        let mut v5 = p3 << 4;
        let mut v6 = p5 << 4;
        let mut v0 = (p0 + v1 + 1) >> 1;
        v1 = v0 - v1;
        let t = (m(v2, S6) + m(v3, C6) + 128) >> 8;
        v2 = (m(v2, C6) - m(v3, S6) + 128) >> 8;
        v3 = t;
        v4 = (v4 + v6 + 1) >> 1;
        v6 = v4 - v6;
        v7 = (v7 + v5 + 1) >> 1;
        v5 = v7 - v5;
        v0 = (v0 + v3 + 1) >> 1;
        v3 = v0 - v3;
        v1 = (v1 + v2 + 1) >> 1;
        v2 = v1 - v2;
        let t = (m(v4, S3) + m(v7, C3) + 2048) >> 12;
        v4 = (m(v4, C3) - m(v7, S3) + 2048) >> 12;
        v7 = t;
        let t = (m(v5, S1) + m(v6, C1) + 2048) >> 12;
        v5 = (m(v5, C1) - m(v6, S1) + 2048) >> 12;
        v6 = t;
        p[r] = (v0 + v7) as i16;
        p[r + 7] = (v0 - v7) as i16;
        p[r + 1] = (v1 + v6) as i16;
        p[r + 6] = (v1 - v6) as i16;
        p[r + 2] = (v2 + v5) as i16;
        p[r + 5] = (v2 - v5) as i16;
        p[r + 3] = (v3 + v4) as i16;
        p[r + 4] = (v3 - v4) as i16;
    }
    let clamp = |v: i32| -> i16 {
        if v < 16 {
            0
        } else if v >= 4080 {
            255
        } else {
            (v >> 4) as i16
        }
    };
    for col in 0..8 {
        let v: [i32; 8] = std::array::from_fn(|k| p[col + 8 * k] as i32);
        if v[1..].iter().all(|&x| x == 0) {
            let mut t = (m(SQRT2, v[0]) + 8192) >> 14;
            t = if t < -2040 {
                0
            } else if t >= 2024 {
                255
            } else {
                (t + 2056) >> 4
            };
            for k in 0..8 {
                block[col + 8 * k] = t as i16;
            }
            continue;
        }
        let mut v0 = (m(SQRT2, v[0]) + 2048) >> 12;
        let mut v1 = (m(SQRT2, v[4]) + 2048) >> 12;
        let mut v2 = v[2];
        let mut v3 = v[6];
        let mut v4 = (m(SQRT1D2, v[1] - v[7]) + 2048) >> 12;
        let mut v7 = (m(SQRT1D2, v[1] + v[7]) + 2048) >> 12;
        let mut v5 = v[3];
        let mut v6 = v[5];
        v0 = ((v0 + v1 + 1) >> 1) + 4112;
        v1 = v0 - v1;
        let t = (m(v2, S6) + m(v3, C6) + 2048) >> 12;
        v2 = (m(v2, C6) - m(v3, S6) + 2048) >> 12;
        v3 = t;
        v4 = (v4 + v6 + 1) >> 1;
        v6 = v4 - v6;
        v7 = (v7 + v5 + 1) >> 1;
        v5 = v7 - v5;
        v0 = (v0 + v3 + 1) >> 1;
        v3 = v0 - v3;
        v1 = (v1 + v2 + 1) >> 1;
        v2 = v1 - v2;
        let t = (m(v4, S3) + m(v7, C3) + 2048) >> 12;
        v4 = (m(v4, C3) - m(v7, S3) + 2048) >> 12;
        v7 = t;
        let t = (m(v5, S1) + m(v6, C1) + 2048) >> 12;
        v5 = (m(v5, C1) - m(v6, S1) + 2048) >> 12;
        v6 = t;
        let out = [
            v0 + v7,
            v1 + v6,
            v2 + v5,
            v3 + v4,
            v3 - v4,
            v2 - v5,
            v1 - v6,
            v0 - v7,
        ];
        for k in 0..8 {
            block[col + 8 * k] = clamp(out[k]);
        }
    }
}
