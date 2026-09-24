//! File formats of the pinned loaders: PLYLoader, EXRLoader's PIZ scanlines and
//! the static-mesh subset of ColladaLoader (also reached through KMZLoader).
use super::super::helpers_formats::formats::{Element, parse_float, parse_xml};
use crate::{Error, Result};
use std::collections::HashMap;

fn bad(what: &str) -> Error {
    Error::Asset(what.to_string())
}

// ---------------------------------------------------------------- PLY

/// PLYLoader.parse for vertex positions and triangle or quad faces, in ASCII or
/// binary form.
pub(super) fn parse_ply(data: &[u8]) -> Result<(Vec<f32>, Vec<u32>)> {
    // extractHeaderText: lines up to end_header, then one byte after it.
    let mut i = 0;
    let mut line = String::new();
    let mut lines = vec![];
    loop {
        let c = *data.get(i).ok_or_else(|| bad("PLY: header"))? as char;
        i += 1;
        if c != '\n' && c != '\r' {
            line.push(c);
        } else {
            let end = line == "end_header";
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            if end {
                break;
            }
        }
    }
    if data.starts_with(b"ply\r\n") {
        i += 1;
    }
    struct Property {
        name: String,
        kind: String,
        list: Option<(String, String)>,
    }
    struct ElementDesc {
        name: String,
        count: usize,
        properties: Vec<Property>,
    }
    let mut format = String::new();
    let mut elements: Vec<ElementDesc> = vec![];
    for l in &lines {
        let v: Vec<&str> = l.split_whitespace().collect();
        match v.first().copied() {
            Some("format") => format = v.get(1).copied().unwrap_or_default().to_string(),
            Some("element") => elements.push(ElementDesc {
                name: v.get(1).copied().unwrap_or_default().to_string(),
                count: v.get(2).and_then(|n| n.parse().ok()).unwrap_or(0),
                properties: vec![],
            }),
            Some("property") => {
                let p = if v.get(1) == Some(&"list") {
                    Property {
                        name: v.get(4).copied().unwrap_or_default().to_string(),
                        kind: "list".into(),
                        list: Some((v[2].to_string(), v[3].to_string())),
                    }
                } else {
                    Property {
                        name: v.get(2).copied().unwrap_or_default().to_string(),
                        kind: v.get(1).copied().unwrap_or_default().to_string(),
                        list: None,
                    }
                };
                elements
                    .last_mut()
                    .ok_or_else(|| bad("PLY: property"))?
                    .properties
                    .push(p);
            }
            _ => {}
        }
    }
    let (mut vertices, mut index) = (vec![], vec![]);
    let size = |t: &str| match t {
        "int8" | "char" | "uint8" | "uchar" => 1,
        "int16" | "short" | "uint16" | "ushort" => 2,
        "int32" | "int" | "uint32" | "uint" | "float32" | "float" => 4,
        _ => 8,
    };
    let little = format == "binary_little_endian";
    let read = |at: usize, t: &str| -> Result<f64> {
        let n = size(t);
        let b = data.get(at..at + n).ok_or_else(|| bad("PLY: truncated"))?;
        let mut a = [0u8; 8];
        a[..n].copy_from_slice(b);
        if !little {
            a[..n].reverse();
        }
        Ok(match t {
            "int8" | "char" => a[0] as i8 as f64,
            "uint8" | "uchar" => a[0] as f64,
            "int16" | "short" => i16::from_le_bytes([a[0], a[1]]) as f64,
            "uint16" | "ushort" => u16::from_le_bytes([a[0], a[1]]) as f64,
            "int32" | "int" => i32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
            "uint32" | "uint" => u32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
            "float32" | "float" => f32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
            _ => f64::from_le_bytes(a),
        })
    };
    let mut handle = |name: &str, values: &HashMap<String, Vec<f64>>| {
        if name == "vertex" {
            for k in [
                ["x", "px", "posx"],
                ["y", "py", "posy"],
                ["z", "pz", "posz"],
            ] {
                if let Some(v) = k.iter().find_map(|n| values.get(*n)) {
                    vertices.push(v[0] as f32);
                }
            }
        } else if name == "face" {
            let v = values
                .get("vertex_indices")
                .or_else(|| values.get("vertex_index"));
            if let Some(v) = v {
                let v: Vec<u32> = v.iter().map(|&i| i as u32).collect();
                if v.len() == 3 {
                    index.extend([v[0], v[1], v[2]]);
                } else if v.len() == 4 {
                    index.extend([v[0], v[1], v[3], v[1], v[2], v[3]]);
                }
            }
        }
    };
    if format == "ascii" {
        let text = String::from_utf8_lossy(data);
        let body = text
            .find("end_header")
            .map(|at| &text[at + "end_header".len()..])
            .unwrap_or("");
        let mut tokens = body.split_whitespace();
        'elements: for e in &elements {
            for _ in 0..e.count {
                let mut values = HashMap::new();
                for p in &e.properties {
                    let Some(t) = tokens.next() else {
                        break 'elements;
                    };
                    if let Some((_, item)) = &p.list {
                        let n = parse_float(t) as usize;
                        let mut list = vec![];
                        for _ in 0..n {
                            let Some(t) = tokens.next() else {
                                break 'elements;
                            };
                            list.push(ply_number(t, item));
                        }
                        values.insert(p.name.clone(), list);
                    } else {
                        values.insert(p.name.clone(), vec![ply_number(t, &p.kind)]);
                    }
                }
                handle(&e.name, &values);
            }
        }
    } else {
        let mut at = i;
        for e in &elements {
            for _ in 0..e.count {
                let mut values = HashMap::new();
                for p in &e.properties {
                    if let Some((count, item)) = &p.list {
                        let n = read(at, count)? as usize;
                        at += size(count);
                        let mut list = Vec::with_capacity(n);
                        for _ in 0..n {
                            list.push(read(at, item)?);
                            at += size(item);
                        }
                        values.insert(p.name.clone(), list);
                    } else {
                        values.insert(p.name.clone(), vec![read(at, &p.kind)?]);
                        at += size(&p.kind);
                    }
                }
                handle(&e.name, &values);
            }
        }
    }
    Ok((vertices, index))
}
/// parseASCIINumber: integers through parseInt, floats through parseFloat.
fn ply_number(t: &str, kind: &str) -> f64 {
    match kind {
        "float" | "double" | "float32" | "float64" => parse_float(t),
        _ => parse_float(t).trunc(),
    }
}

// ---------------------------------------------------------------- EXR

const USHORT_RANGE: usize = 1 << 16;
const BITMAP_SIZE: usize = USHORT_RANGE >> 3;
const HUF_ENCSIZE: usize = (1 << 16) + 1;
const HUF_DECBITS: i32 = 14;
const HUF_DECSIZE: usize = 1 << HUF_DECBITS;
const HUF_DECMASK: i32 = HUF_DECSIZE as i32 - 1;
const SHORT_ZEROCODE_RUN: i32 = 59;
const LONG_ZEROCODE_RUN: i32 = 63;
const SHORTEST_LONG_RUN: i32 = 2 + LONG_ZEROCODE_RUN - SHORT_ZEROCODE_RUN;

struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn u8(&mut self) -> u8 {
        let v = self.data.get(self.at).copied().unwrap_or(0);
        self.at += 1;
        v
    }
    fn u16(&mut self) -> u16 {
        u16::from_le_bytes([self.u8(), self.u8()])
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes([self.u8(), self.u8(), self.u8(), self.u8()])
    }
}
#[derive(Clone, Default)]
struct DecEntry {
    len: i32,
    lit: i32,
    p: Vec<i32>,
}
/// EXRLoader: getBits over a 32-bit accumulator, as JavaScript's `<<` wraps it.
fn get_bits(n: i32, c: &mut i32, lc: &mut i32, r: &mut Reader) -> i32 {
    while *lc < n {
        *c = c.wrapping_shl(8) | r.u8() as i32;
        *lc += 8;
    }
    *lc -= n;
    (*c >> *lc) & ((1 << n) - 1)
}
fn huf_unpack_enc_table(
    r: &mut Reader,
    mut im: usize,
    max: usize,
    hcode: &mut [i32],
) -> Result<()> {
    let (mut c, mut lc) = (0i32, 0i32);
    while im <= max {
        let l = get_bits(6, &mut c, &mut lc, r);
        hcode[im] = l;
        if l == LONG_ZEROCODE_RUN {
            let zerun = get_bits(8, &mut c, &mut lc, r) + SHORTEST_LONG_RUN;
            if im + zerun as usize > max + 1 {
                return Err(bad("EXR: bad Huffman table"));
            }
            for _ in 0..zerun {
                hcode[im] = 0;
                im += 1;
            }
            im -= 1;
        } else if l >= SHORT_ZEROCODE_RUN {
            let zerun = l - SHORT_ZEROCODE_RUN + 2;
            if im + zerun as usize > max + 1 {
                return Err(bad("EXR: bad Huffman table"));
            }
            for _ in 0..zerun {
                hcode[im] = 0;
                im += 1;
            }
            im -= 1;
        }
        im += 1;
    }
    // hufCanonicalCodeTable, with the codes packed as `l | code << 6` in 32 bits.
    let mut table = [0i32; 59];
    for &l in hcode.iter() {
        if (0..59).contains(&l) {
            table[l as usize] += 1;
        }
    }
    let mut c = 0;
    for i in (1..=58).rev() {
        let nc = (c + table[i]) >> 1;
        table[i] = c;
        c = nc;
    }
    for code in hcode.iter_mut() {
        let l = *code;
        if l > 0 {
            *code = l | table[l as usize].wrapping_shl(6);
            table[l as usize] += 1;
        }
    }
    Ok(())
}
fn huf_build_dec_table(hcode: &[i32], im: usize, max: usize, dec: &mut [DecEntry]) -> Result<()> {
    for (i, &code) in hcode.iter().enumerate().take(max + 1).skip(im) {
        let (c, l) = (code >> 6, code & 63);
        if (c >> l) != 0 {
            return Err(bad("EXR: invalid table entry"));
        }
        if l > HUF_DECBITS {
            let pl = &mut dec[(c >> (l - HUF_DECBITS)) as usize];
            if pl.len != 0 {
                return Err(bad("EXR: invalid table entry"));
            }
            pl.lit += 1;
            pl.p.push(i as i32);
        } else if l != 0 {
            let base = (c << (HUF_DECBITS - l)) as usize;
            for k in 0..(1usize << (HUF_DECBITS - l)) {
                let pl = &mut dec[base + k];
                if pl.len != 0 || !pl.p.is_empty() {
                    return Err(bad("EXR: invalid table entry"));
                }
                pl.len = l;
                pl.lit = i as i32;
            }
        }
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn get_code(
    po: i32,
    rlc: i32,
    c: &mut i32,
    lc: &mut i32,
    r: &mut Reader,
    out: &mut [u16],
    o: &mut usize,
    end: usize,
) {
    if po == rlc {
        if *lc < 8 {
            *c = c.wrapping_shl(8) | r.u8() as i32;
            *lc += 8;
        }
        *lc -= 8;
        let cs = ((*c >> *lc) & 255) as usize;
        if *o + cs > end {
            return;
        }
        let s = out[*o - 1];
        for _ in 0..cs {
            out[*o] = s;
            *o += 1;
        }
    } else if *o < end {
        out[*o] = po as u16;
        *o += 1;
    }
}
#[allow(clippy::too_many_arguments)]
fn huf_decode(
    hcode: &[i32],
    dec: &[DecEntry],
    r: &mut Reader,
    ni: i32,
    rlc: i32,
    no: usize,
    out: &mut [u16],
) -> Result<()> {
    let (mut c, mut lc) = (0i32, 0i32);
    let mut o = 0usize;
    let end_in = (r.at as f64 + (ni as f64 + 7.) / 8.).trunc() as usize;
    while r.at < end_in {
        c = c.wrapping_shl(8) | r.u8() as i32;
        lc += 8;
        while lc >= HUF_DECBITS {
            let pl = &dec[((c >> (lc - HUF_DECBITS)) & HUF_DECMASK) as usize];
            if pl.len != 0 {
                lc -= pl.len;
                get_code(pl.lit, rlc, &mut c, &mut lc, r, out, &mut o, no);
            } else {
                if pl.p.is_empty() {
                    return Err(bad("EXR: hufDecode"));
                }
                let mut found = false;
                for &p in &pl.p {
                    let l = hcode[p as usize] & 63;
                    while lc < l && r.at < end_in {
                        c = c.wrapping_shl(8) | r.u8() as i32;
                        lc += 8;
                    }
                    if lc >= l && (hcode[p as usize] >> 6) == ((c >> (lc - l)) & ((1 << l) - 1)) {
                        lc -= l;
                        get_code(p, rlc, &mut c, &mut lc, r, out, &mut o, no);
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(bad("EXR: hufDecode"));
                }
            }
        }
    }
    let i = (8 - ni) & 7;
    c >>= i;
    lc -= i;
    while lc > 0 {
        let pl = &dec[(c.wrapping_shl((HUF_DECBITS - lc) as u32) & HUF_DECMASK) as usize];
        if pl.len == 0 {
            return Err(bad("EXR: hufDecode"));
        }
        lc -= pl.len;
        get_code(pl.lit, rlc, &mut c, &mut lc, r, out, &mut o, no);
    }
    Ok(())
}
fn wdec14(l: u16, h: u16) -> (i32, i32) {
    let (ls, hs) = (l as i16 as i32, h as i16 as i32);
    let ai = ls + (hs & 1) + (hs >> 1);
    (ai, ai - hs)
}
fn wdec16(l: u16, h: u16) -> (i32, i32) {
    let (m, d) = (l as i32, h as i32);
    let bb = (m - (d >> 1)) & 0xffff;
    let aa = (d + bb - 0x8000) & 0xffff;
    (aa, bb)
}
/// wav2Decode: the inverse 2D Haar wavelet, in place.
fn wav2_decode(b: &mut [u16], j: usize, nx: usize, ox: usize, ny: usize, oy: usize, mx: u16) {
    let w14 = (mx as u32) < (1 << 14);
    let dec = |l: u16, h: u16| if w14 { wdec14(l, h) } else { wdec16(l, h) };
    let n = nx.min(ny);
    let mut p = 1;
    while p <= n {
        p <<= 1;
    }
    p >>= 1;
    let mut p2 = p;
    p >>= 1;
    while p >= 1 {
        let mut py = 0;
        let ey = oy * (ny - p2);
        let (oy1, oy2, ox1, ox2) = (oy * p, oy * p2, ox * p, ox * p2);
        while py <= ey {
            let mut px = py;
            let ex = py + ox * (nx - p2);
            while px <= ex {
                let (p01, p10) = (px + ox1, px + oy1);
                let p11 = p10 + ox1;
                let (i00, i10) = dec(b[px + j], b[p10 + j]);
                let (i01, i11) = dec(b[p01 + j], b[p11 + j]);
                let (a, c) = dec(i00 as u16, i01 as u16);
                b[px + j] = a as u16;
                b[p01 + j] = c as u16;
                let (a, c) = dec(i10 as u16, i11 as u16);
                b[p10 + j] = a as u16;
                b[p11 + j] = c as u16;
                px += ox2;
            }
            if nx & p != 0 {
                let p10 = px + oy1;
                let (a, c) = dec(b[px + j], b[p10 + j]);
                b[p10 + j] = c as u16;
                b[px + j] = a as u16;
            }
            py += oy2;
        }
        if ny & p != 0 {
            let mut px = py;
            let ex = py + ox * (nx - p2);
            while px <= ex {
                let p01 = px + ox1;
                let (a, c) = dec(b[px + j], b[p01 + j]);
                b[p01 + j] = c as u16;
                b[px + j] = a as u16;
                px += ox2;
            }
        }
        p2 = p;
        p >>= 1;
    }
}
/// uncompressPIZ: bitmap LUT, Huffman, wavelet, then the scanline interleave.
fn uncompress_piz(
    data: &[u8],
    at: usize,
    columns: usize,
    lines: usize,
    channels: usize,
    kind: usize,
) -> Result<Vec<u8>> {
    let mut r = Reader { data, at };
    let total = columns * lines * channels * kind;
    let mut out = vec![0u16; total];
    let min = r.u16() as usize;
    let max = r.u16() as usize;
    if max >= BITMAP_SIZE {
        return Err(bad("EXR: PIZ bitmap"));
    }
    let mut bitmap = vec![0u8; BITMAP_SIZE];
    if min <= max {
        for i in 0..=max - min {
            bitmap[i + min] = r.u8();
        }
    }
    // reverseLutFromBitmap
    let mut lut = vec![0u16; USHORT_RANGE];
    let mut k = 0;
    for i in 0..USHORT_RANGE {
        if i == 0 || bitmap[i >> 3] & (1 << (i & 7)) != 0 {
            lut[k] = i as u16;
            k += 1;
        }
    }
    let max_value = (k - 1) as u16;
    let length = r.u32() as usize;
    // hufUncompress
    let start = r.at;
    let im = r.u32() as usize;
    let imax = r.u32() as usize;
    r.at += 4;
    let bits = r.u32() as i32;
    r.at += 4;
    if im >= HUF_ENCSIZE || imax >= HUF_ENCSIZE {
        return Err(bad("EXR: HUF_ENCSIZE"));
    }
    let mut hcode = vec![0i32; HUF_ENCSIZE];
    huf_unpack_enc_table(&mut r, im, imax, &mut hcode)?;
    if bits as i64 > 8 * (length as i64 - (r.at - start) as i64) {
        return Err(bad("EXR: hufUncompress"));
    }
    let mut dec = vec![DecEntry::default(); HUF_DECSIZE];
    huf_build_dec_table(&hcode, im, imax, &mut dec)?;
    huf_decode(&hcode, &dec, &mut r, bits, imax as i32, total, &mut out)?;
    let plane = columns * lines * kind;
    for c in 0..channels {
        for j in 0..kind {
            wav2_decode(
                &mut out,
                c * plane + j,
                columns,
                kind,
                lines,
                columns * kind,
                max_value,
            );
        }
    }
    for v in &mut out {
        *v = lut[*v as usize];
    }
    let mut bytes = Vec::with_capacity(total * 2);
    let mut ends: Vec<usize> = (0..channels).map(|c| c * plane).collect();
    for _ in 0..lines {
        for end in &mut ends {
            let n = columns * kind;
            for v in &out[*end..*end + n] {
                bytes.extend(v.to_le_bytes());
            }
            *end += n;
        }
    }
    Ok(bytes)
}
/// DataUtils.toHalfFloat: clamped to ±65504, then the base/shift tables, which
/// truncate the mantissa.
pub(in crate::browser) fn to_half_float(v: f32) -> u16 {
    let v = v.clamp(-65504., 65504.);
    let f = v.to_bits();
    let e = ((f >> 23) & 0x1ff) as usize;
    let (i, sign) = (e & 0xff, e & 0x100 != 0);
    let exponent = i as i32 - 127;
    let (base, shift) = if exponent < -27 {
        (0u32, 24)
    } else if exponent < -14 {
        (0x0400 >> (-exponent - 14), -exponent - 1)
    } else if exponent <= 15 {
        (((exponent + 15) as u32) << 10, 13)
    } else if exponent < 128 {
        (0x7c00, 24)
    } else {
        (0x7c00, 13)
    };
    let base = if sign {
        if exponent < -27 {
            0x8000
        } else {
            base | 0x8000
        }
    } else {
        base
    };
    (base + ((f & 0x007fffff) >> shift)) as u16
}
/// uncompressZIP: zlib, the byte predictor, then interleaveScalar.
fn uncompress_zip(data: &[u8]) -> Result<Vec<u8>> {
    let mut raw = miniz_oxide::inflate::decompress_to_vec_zlib(data)
        .map_err(|_| bad("EXR: bad ZIP block"))?;
    for t in 1..raw.len() {
        raw[t] = raw[t - 1].wrapping_add(raw[t]).wrapping_sub(128);
    }
    let mut out = vec![0; raw.len()];
    let (mut t1, mut t2, mut s) = (0, raw.len().div_ceil(2), 0);
    let stop = raw.len() as isize - 1;
    loop {
        if s as isize > stop {
            break;
        }
        out[s] = raw[t1];
        s += 1;
        t1 += 1;
        if s as isize > stop {
            break;
        }
        out[s] = raw[t2];
        s += 1;
        t2 += 1;
    }
    Ok(out)
}
/// EXRLoader.parse for single-part scanline HALF or FLOAT images (no compression,
/// ZIPS, ZIP or PIZ), as HalfFloatType RGBA with rows from the bottom and alpha
/// filled with 1.
pub(in crate::browser) fn decode_exr(data: &[u8]) -> Result<(u32, u32, Vec<u16>)> {
    let mut r = Reader { data, at: 8 };
    let string = |r: &mut Reader| {
        let mut s = vec![];
        loop {
            let c = r.u8();
            if c == 0 || r.at > data.len() {
                break;
            }
            s.push(c);
        }
        String::from_utf8_lossy(&s).into_owned()
    };
    let (mut channels, mut compression, mut window) = (vec![], 0u8, [0i32; 4]);
    loop {
        let name = string(&mut r);
        if name.is_empty() {
            break;
        }
        let kind = string(&mut r);
        let size = r.u32() as usize;
        let at = r.at;
        match (name.as_str(), kind.as_str()) {
            ("channels", "chlist") => {
                while r.at < at + size - 1 {
                    let n = string(&mut r);
                    let pixel = r.u32() as usize;
                    r.at += 12;
                    channels.push((n, pixel));
                }
            }
            ("compression", _) => compression = r.u8(),
            ("dataWindow", _) => {
                for w in &mut window {
                    *w = r.u32() as i32;
                }
            }
            _ => {}
        }
        r.at = at + size;
    }
    let (w, h) = (
        (window[2] - window[0] + 1) as usize,
        (window[3] - window[1] + 1) as usize,
    );
    let block = match compression {
        0 | 2 => 1,
        3 => 16,
        4 => 32,
        _ => return Err(bad("EXR: unsupported compression")),
    };
    if channels.iter().any(|(_, p)| *p != 1 && *p != 2)
        || (compression == 4 && channels.iter().any(|(_, p)| *p != 1))
    {
        return Err(bad("EXR: only HALF and FLOAT channels are supported"));
    }
    // Byte size per sample: HALF 2, FLOAT 4.
    let bytes = |p: usize| if p == 1 { 2 } else { 4 };
    let slot = |n: &str| match n {
        "R" => Some(0),
        "G" => Some(1),
        "B" => Some(2),
        "A" => Some(3),
        _ => None,
    };
    let fill = !channels.iter().any(|(n, _)| n == "A");
    let mut out = vec![if fill { 0x3C00u16 } else { 0 }; w * h * 4];
    let total: usize = channels.iter().map(|(_, p)| bytes(*p)).sum();
    let mut offsets = vec![];
    let mut byte = 0;
    for (_, p) in &channels {
        offsets.push(byte);
        byte += bytes(*p);
    }
    r.at += h.div_ceil(block) * 8;
    for _ in 0..h / block {
        let line = r.u32() as i32 - window[1];
        let size = r.u32() as usize;
        let lines = if line as usize + block > h {
            h - line as usize
        } else {
            block
        };
        let per_line = w * total;
        let raw;
        let viewer: &[u8] = if size < lines * per_line {
            raw = if compression == 4 {
                uncompress_piz(data, r.at, w, lines, channels.len(), 1)?
            } else {
                uncompress_zip(
                    data.get(r.at..r.at + size)
                        .ok_or_else(|| bad("EXR: truncated"))?,
                )?
            };
            &raw
        } else {
            data.get(r.at..r.at + size)
                .ok_or_else(|| bad("EXR: truncated"))?
        };
        r.at += size;
        for y in 0..lines {
            let row = (h - 1 - (line as usize + y)) * w * 4;
            for (c, (n, p)) in channels.iter().enumerate() {
                let Some(s) = slot(n) else { continue };
                let mut at = y * per_line + offsets[c] * w;
                for x in 0..w {
                    out[row + x * 4 + s] = if *p == 1 {
                        u16::from_le_bytes([viewer[at], viewer[at + 1]])
                    } else {
                        to_half_float(f32::from_le_bytes([
                            viewer[at],
                            viewer[at + 1],
                            viewer[at + 2],
                            viewer[at + 3],
                        ]))
                    };
                    at += bytes(*p);
                }
            }
        }
    }
    Ok((w as u32, h as u32, out))
}

// ---------------------------------------------------------------- Collada

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Shading {
    Phong,
    Lambert,
    Basic,
}
#[derive(Clone)]
pub(super) struct DaeMaterial {
    pub shading: Shading,
    /// Diffuse, specular and emissive as the file gives them (sRGB, converted later).
    pub color: [f64; 3],
    pub specular: Option<[f64; 3]>,
    pub emissive: Option<[f64; 3]>,
    pub shininess: Option<f64>,
    /// The diffuse texture's image path, relative to the document.
    pub map: Option<String>,
    pub opacity: f64,
    pub transparent: bool,
    pub double_sided: bool,
}
pub(super) struct DaeMesh {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    /// ( start, count, material ) per primitive, as `addGroup` records them.
    pub groups: Vec<(usize, usize, usize)>,
    pub materials: Vec<DaeMaterial>,
    /// The node's matrix composed with its ancestors', column-major.
    pub matrix: [f64; 16],
}
pub(super) struct DaeScene {
    pub meshes: Vec<DaeMesh>,
    pub unit: f64,
    pub z_up: bool,
}
fn floats(text: &str) -> Vec<f64> {
    text.split_whitespace().map(parse_float).collect()
}
fn id(url: &str) -> &str {
    url.trim_start_matches('#')
}
fn children<'a>(e: &'a Element, name: &'a str) -> impl Iterator<Item = &'a Element> {
    e.elements().filter(move |c| c.name == name)
}
fn all<'a>(e: &'a Element, name: &str, out: &mut Vec<&'a Element>) {
    for c in e.elements() {
        if c.name == name {
            out.push(c);
        }
        all(c, name, out);
    }
}
fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    let mut m = [0.; 16];
    for c in 0..4 {
        for r in 0..4 {
            m[c * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[c * 4 + k]).sum();
        }
    }
    m
}
const IDENTITY: [f64; 16] = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];
/// ColladaParser and ColladaComposer for static geometry: effects (phong, blinn,
/// lambert, constant), triangles and polylists, node transforms and the asset
/// unit and up axis.
pub(super) fn parse_collada(text: &str) -> Result<DaeScene> {
    let root = parse_xml(text)?;
    let mut list = vec![];
    all(&root, "asset", &mut list);
    let asset = list.first().copied();
    let unit = asset
        .and_then(|a| {
            let mut u = vec![];
            all(a, "unit", &mut u);
            u.first()
                .and_then(|u| u.attribute("meter"))
                .map(parse_float)
        })
        .unwrap_or(1.);
    let z_up = asset
        .map(|a| {
            let mut u = vec![];
            all(a, "up_axis", &mut u);
            u.first().is_some_and(|u| u.text().trim() == "Z_UP")
        })
        .unwrap_or(false);
    let find_by_id = |tag: &str, key: &str| -> Option<&Element> {
        let mut v = vec![];
        all(&root, tag, &mut v);
        v.into_iter().find(|e| e.attribute("id") == Some(key))
    };
    let mut images: HashMap<String, String> = HashMap::new();
    let mut v = vec![];
    all(&root, "image", &mut v);
    for image in v {
        if let (Some(key), Some(init)) =
            (image.attribute("id"), image.first_descendant("init_from"))
        {
            images.insert(key.to_string(), init.text().trim().to_string());
        }
    }
    let material = |key: &str| -> Result<DaeMaterial> {
        let m = find_by_id("material", key).ok_or_else(|| bad("Collada: material"))?;
        let effect_url = m
            .first_descendant("instance_effect")
            .and_then(|e| e.attribute("url"))
            .ok_or_else(|| bad("Collada: effect"))?;
        let effect = find_by_id("effect", id(effect_url)).ok_or_else(|| bad("Collada: effect"))?;
        let profile = effect
            .first_descendant("profile_COMMON")
            .ok_or_else(|| bad("Collada: profile"))?;
        let (mut surfaces, mut samplers) = (HashMap::new(), HashMap::new());
        for p in children(profile, "newparam") {
            let sid = p.attribute("sid").unwrap_or_default().to_string();
            if let Some(s) = p.first_descendant("surface")
                && let Some(i) = s.first_descendant("init_from")
            {
                surfaces.insert(sid.clone(), i.text().trim().to_string());
            }
            if let Some(s) = p.first_descendant("sampler2D")
                && let Some(src) = s.first_descendant("source")
            {
                samplers.insert(sid, src.text().trim().to_string());
            }
        }
        let technique = profile
            .first_descendant("technique")
            .ok_or_else(|| bad("Collada: technique"))?;
        let body = technique
            .elements()
            .find(|e| ["phong", "blinn", "lambert", "constant"].contains(&e.name.as_str()))
            .ok_or_else(|| bad("Collada: shading"))?;
        let shading = match body.name.as_str() {
            "phong" | "blinn" => Shading::Phong,
            "lambert" => Shading::Lambert,
            _ => Shading::Basic,
        };
        let mut out = DaeMaterial {
            shading,
            color: [1., 1., 1.],
            specular: None,
            emissive: None,
            shininess: None,
            map: None,
            opacity: 1.,
            transparent: false,
            double_sided: false,
        };
        let color = |e: &Element| {
            e.first_descendant("color").map(|c| {
                let v = floats(&c.text());
                [v[0], v[1], v[2]]
            })
        };
        let (mut transparent, mut transparency) = (None, None);
        for p in body.elements() {
            match p.name.as_str() {
                "diffuse" => {
                    if let Some(c) = color(p) {
                        out.color = c;
                    }
                    if let Some(t) = p.first_descendant("texture") {
                        let tex = t.attribute("texture").unwrap_or_default();
                        // The sampler's surface image; an undefined sampler names the image.
                        let image = samplers
                            .get(tex)
                            .and_then(|s| surfaces.get(s))
                            .map(String::as_str)
                            .unwrap_or(tex);
                        out.map = images.get(image).cloned();
                    }
                }
                "specular" if shading == Shading::Phong => out.specular = color(p),
                "emission" if shading != Shading::Basic => out.emissive = color(p),
                "shininess" if shading == Shading::Phong => {
                    let v = p.first_descendant("float").map(|f| parse_float(&f.text()));
                    // `if ( parameter.float && material.shininess )`: zero keeps the default.
                    out.shininess = v.filter(|&v| v != 0.);
                }
                "transparent" => {
                    transparent = Some((
                        p.attribute("opaque").unwrap_or("A_ONE").to_string(),
                        color(p).map(|c| c.to_vec()),
                        p.first_descendant("texture").is_some(),
                        p.first_descendant("color").map(|c| floats(&c.text())),
                    ));
                }
                "transparency" => {
                    transparency = p.first_descendant("float").map(|f| parse_float(&f.text()));
                }
                _ => {}
            }
        }
        if transparency.is_none() && transparent.is_some() {
            transparency = Some(1.);
        }
        if transparent.is_none() && transparency.is_some() {
            transparent = Some(("A_ONE".into(), None, false, Some(vec![1., 1., 1., 1.])));
        }
        if let (Some((opaque, _, texture, color)), Some(f)) = (transparent, transparency) {
            if texture {
                out.transparent = true;
            } else {
                let c = color.unwrap_or(vec![1., 1., 1., 1.]);
                out.opacity = match opaque.as_str() {
                    "RGB_ZERO" => 1. - c[0] * f,
                    "A_ZERO" => 1. - c[3] * f,
                    "RGB_ONE" => c[0] * f,
                    _ => c[3] * f,
                };
                out.transparent = out.opacity < 1.;
            }
        }
        if let Some(extra) = technique.first_descendant("double_sided") {
            out.double_sided = parse_float(&extra.text()) == 1.;
        }
        Ok(out)
    };
    let mut materials_cache: HashMap<String, DaeMaterial> = HashMap::new();
    let geometry = |key: &str,
                    bindings: &HashMap<String, String>,
                    cache: &mut HashMap<String, DaeMaterial>,
                    matrix: [f64; 16]|
     -> Result<Vec<DaeMesh>> {
        let g = find_by_id("geometry", key).ok_or_else(|| bad("Collada: geometry"))?;
        let mesh = g
            .first_descendant("mesh")
            .ok_or_else(|| bad("Collada: mesh"))?;
        let mut sources: HashMap<String, (Vec<f64>, usize)> = HashMap::new();
        for s in children(mesh, "source") {
            let array = s
                .first_descendant("float_array")
                .map(|a| floats(&a.text()))
                .unwrap_or_default();
            let stride = s
                .first_descendant("accessor")
                .and_then(|a| a.attribute("stride"))
                .map(|v| parse_float(v) as usize)
                .unwrap_or(3);
            sources.insert(
                s.attribute("id").unwrap_or_default().to_string(),
                (array, stride),
            );
        }
        let mut vertices: Vec<(String, String)> = vec![];
        if let Some(v) = children(mesh, "vertices").next() {
            for i in children(v, "input") {
                vertices.push((
                    i.attribute("semantic").unwrap_or_default().to_string(),
                    id(i.attribute("source").unwrap_or_default()).to_string(),
                ));
            }
        }
        // groupPrimitives: one geometry per primitive type, in first-seen order.
        let mut types: Vec<(String, Vec<&Element>)> = vec![];
        for p in mesh.elements() {
            if ["triangles", "polylist", "polygons"].contains(&p.name.as_str()) {
                match types.iter_mut().find(|(t, _)| *t == p.name) {
                    Some(entry) => entry.1.push(p),
                    None => types.push((p.name.clone(), vec![p])),
                }
            }
        }
        let mut out = vec![];
        for (_, primitives) in types {
            let (mut position, mut normal, mut uv) = (vec![], vec![], vec![]);
            let (mut groups, mut keys) = (vec![], vec![]);
            let mut start = 0;
            for (pi, p) in primitives.iter().enumerate() {
                let mut inputs: Vec<(String, String, usize)> = vec![];
                let mut stride = 0;
                for i in children(p, "input") {
                    let semantic = i.attribute("semantic").unwrap_or_default();
                    let set = i.attribute("set").map(parse_float).unwrap_or(0.);
                    let name = if set > 0. {
                        format!("{semantic}{set}")
                    } else {
                        semantic.to_string()
                    };
                    let offset = i.attribute("offset").map(parse_float).unwrap_or(0.) as usize;
                    stride = stride.max(offset + 1);
                    inputs.push((
                        name,
                        id(i.attribute("source").unwrap_or_default()).to_string(),
                        offset,
                    ));
                }
                let indices: Vec<usize> = p
                    .first_descendant("p")
                    .map(|e| floats(&e.text()).into_iter().map(|v| v as usize).collect())
                    .unwrap_or_default();
                let count = p.attribute("count").map(parse_float).unwrap_or(0.) as usize;
                let vcount: Option<Vec<usize>> = match p.name.as_str() {
                    "polylist" => p
                        .first_descendant("vcount")
                        .map(|e| floats(&e.text()).into_iter().map(|v| v as usize).collect()),
                    "polygons" => Some(vec![indices.len() / stride.max(1)]),
                    _ => None,
                };
                let n = match &vcount {
                    None => count * 3,
                    Some(v) => v
                        .iter()
                        .take(count)
                        .map(|&c| {
                            if c == 3 {
                                3
                            } else if c == 4 {
                                6
                            } else {
                                (c.max(2) - 2) * 3
                            }
                        })
                        .sum(),
                };
                groups.push((start, n, pi));
                start += n;
                if let Some(m) = p.attribute("material") {
                    keys.push(m.to_string());
                }
                // buildGeometryData: expand each index tuple into the attribute arrays.
                let push = |array: &mut Vec<f32>,
                            (data, s): &(Vec<f64>, usize),
                            offset: usize|
                 -> Result<()> {
                    let mut vector = |i: usize| -> Result<()> {
                        let at = indices
                            .get(i + offset)
                            .ok_or_else(|| bad("Collada: index"))?
                            * s;
                        for k in at..at + s {
                            array.push(*data.get(k).ok_or_else(|| bad("Collada: source"))? as f32);
                        }
                        Ok(())
                    };
                    match &vcount {
                        None => {
                            let mut i = 0;
                            while i < indices.len() {
                                vector(i)?;
                                i += stride;
                            }
                        }
                        Some(v) => {
                            let mut index = 0;
                            for &c in v {
                                match c {
                                    3 => {
                                        for k in 0..3 {
                                            vector(index + stride * k)?;
                                        }
                                    }
                                    4 => {
                                        for k in [0, 1, 3, 1, 2, 3] {
                                            vector(index + stride * k)?;
                                        }
                                    }
                                    _ if c > 4 => {
                                        return Err(bad(
                                            "Collada: polygons above four vertices are not supported",
                                        ));
                                    }
                                    _ => {}
                                }
                                index += stride * c;
                            }
                        }
                    }
                    Ok(())
                };
                for (name, source, offset) in &inputs {
                    match name.as_str() {
                        "VERTEX" => {
                            for (semantic, source) in &vertices {
                                let s =
                                    sources.get(source).ok_or_else(|| bad("Collada: source"))?;
                                match semantic.as_str() {
                                    "POSITION" => push(&mut position, s, *offset)?,
                                    "NORMAL" => push(&mut normal, s, *offset)?,
                                    "TEXCOORD" => push(&mut uv, s, *offset)?,
                                    _ => {}
                                }
                            }
                        }
                        "NORMAL" => push(
                            &mut normal,
                            sources.get(source).ok_or_else(|| bad("Collada: source"))?,
                            *offset,
                        )?,
                        "TEXCOORD" => push(
                            &mut uv,
                            sources.get(source).ok_or_else(|| bad("Collada: source"))?,
                            *offset,
                        )?,
                        _ => {}
                    }
                }
            }
            let mut materials = vec![];
            for k in &keys {
                let target = bindings
                    .get(k)
                    .ok_or_else(|| bad("Collada: material binding"))?;
                if !cache.contains_key(target) {
                    cache.insert(target.clone(), material(target)?);
                }
                materials.push(cache[target].clone());
            }
            out.push(DaeMesh {
                positions: position,
                normals: normal,
                uvs: uv,
                groups,
                materials,
                matrix,
            });
        }
        Ok(out)
    };
    // The scene's visual scene, walked node by node.
    let mut scene_list = vec![];
    all(&root, "instance_visual_scene", &mut scene_list);
    let scene_url = scene_list
        .first()
        .and_then(|e| e.attribute("url"))
        .ok_or_else(|| bad("Collada: scene"))?;
    let visual =
        find_by_id("visual_scene", id(scene_url)).ok_or_else(|| bad("Collada: visual scene"))?;
    let mut meshes = vec![];
    let mut stack: Vec<(&Element, [f64; 16])> =
        children(visual, "node").map(|n| (n, IDENTITY)).collect();
    stack.reverse();
    while let Some((node, parent)) = stack.pop() {
        let mut matrix = IDENTITY;
        for c in node.elements() {
            let v = floats(&c.text());
            let m = match c.name.as_str() {
                // fromArray( array ).transpose(): the file lists rows.
                "matrix" => std::array::from_fn(|i| v[(i % 4) * 4 + i / 4]),
                "translate" => [
                    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., v[0], v[1], v[2], 1.,
                ],
                "scale" => [
                    v[0], 0., 0., 0., 0., v[1], 0., 0., 0., 0., v[2], 0., 0., 0., 0., 1.,
                ],
                "rotate" => {
                    // makeRotationAxis takes the axis as given.
                    let (x, y, z) = (v[0], v[1], v[2]);
                    let a = v[3].to_radians();
                    let (c, s, t) = (a.cos(), a.sin(), 1. - a.cos());
                    [
                        t * x * x + c,
                        t * x * y + s * z,
                        t * x * z - s * y,
                        0.,
                        t * x * y - s * z,
                        t * y * y + c,
                        t * y * z + s * x,
                        0.,
                        t * x * z + s * y,
                        t * y * z - s * x,
                        t * z * z + c,
                        0.,
                        0.,
                        0.,
                        0.,
                        1.,
                    ]
                }
                _ => continue,
            };
            matrix = multiply(&matrix, &m);
        }
        let world = multiply(&parent, &matrix);
        for instance in children(node, "instance_geometry") {
            let mut bindings = HashMap::new();
            let mut v = vec![];
            all(instance, "instance_material", &mut v);
            for m in v {
                if let (Some(symbol), Some(target)) = (m.attribute("symbol"), m.attribute("target"))
                {
                    bindings.insert(symbol.to_string(), id(target).to_string());
                }
            }
            let url = instance.attribute("url").unwrap_or_default();
            meshes.extend(geometry(id(url), &bindings, &mut materials_cache, world)?);
        }
        let mut kids: Vec<_> = children(node, "node").map(|n| (n, world)).collect();
        kids.reverse();
        stack.extend(kids);
    }
    Ok(DaeScene { meshes, unit, z_up })
}
