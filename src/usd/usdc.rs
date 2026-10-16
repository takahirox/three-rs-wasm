//! USDCParser ( examples/jsm/loaders/usd/USDCParser.js ): the binary crate
//! layer — its table of contents, LZ4-compressed token, field, field-set,
//! path and spec sections, and the typed values — as `specsByPath`. Reads
//! past the buffer fail as the DataView reads do; the decompressors keep the
//! original's handling of truncated input ( a missing byte reads as
//! `undefined` ).
use super::js::{Obj, Typed, Value};
use super::{Spec, Specs};
use crate::{Error, Result};
use std::collections::HashMap;

fn range_error(what: &str) -> Error {
    Error::Asset(format!("USDCParser: {what}"))
}

mod ty {
    pub const INVALID: u32 = 0;
    pub const BOOL: u32 = 1;
    pub const UCHAR: u32 = 2;
    pub const INT: u32 = 3;
    pub const UINT: u32 = 4;
    pub const INT64: u32 = 5;
    pub const UINT64: u32 = 6;
    pub const HALF: u32 = 7;
    pub const FLOAT: u32 = 8;
    pub const DOUBLE: u32 = 9;
    pub const STRING: u32 = 10;
    pub const TOKEN: u32 = 11;
    pub const ASSET_PATH: u32 = 12;
    pub const MATRIX2D: u32 = 13;
    pub const MATRIX3D: u32 = 14;
    pub const MATRIX4D: u32 = 15;
    pub const QUATD: u32 = 16;
    pub const QUATF: u32 = 17;
    pub const QUATH: u32 = 18;
    pub const VEC2D: u32 = 19;
    pub const VEC2F: u32 = 20;
    pub const VEC2H: u32 = 21;
    pub const VEC2I: u32 = 22;
    pub const VEC3D: u32 = 23;
    pub const VEC3F: u32 = 24;
    pub const VEC3H: u32 = 25;
    pub const VEC3I: u32 = 26;
    pub const VEC4D: u32 = 27;
    pub const VEC4F: u32 = 28;
    pub const VEC4I: u32 = 30;
    pub const DICTIONARY: u32 = 31;
    pub const TOKEN_LIST_OP: u32 = 32;
    pub const STRING_LIST_OP: u32 = 33;
    pub const PATH_LIST_OP: u32 = 34;
    pub const INT_LIST_OP: u32 = 36;
    pub const INT64_LIST_OP: u32 = 37;
    pub const UINT_LIST_OP: u32 = 38;
    pub const UINT64_LIST_OP: u32 = 39;
    pub const PATH_VECTOR: u32 = 40;
    pub const TOKEN_VECTOR: u32 = 41;
    pub const SPECIFIER: u32 = 42;
    pub const PERMISSION: u32 = 43;
    pub const VARIABILITY: u32 = 44;
    pub const VARIANT_SELECTION_MAP: u32 = 45;
    pub const TIME_SAMPLES: u32 = 46;
    pub const DOUBLE_VECTOR: u32 = 48;
}

const FIELD_SET_TERMINATOR: f64 = 4294967295.0;
const FLOAT_COMPRESSION_INT: i8 = 0x69;
const FLOAT_COMPRESSION_LUT: i8 = 0x74;

/// ToInt32.
fn to_i32(v: f64) -> i32 {
    if !v.is_finite() {
        0
    } else {
        v.trunc().rem_euclid(4294967296.0) as u32 as i32
    }
}
/// A Uint8Array store.
fn to_u8(v: f64) -> u8 {
    if !v.is_finite() {
        0
    } else {
        v.trunc().rem_euclid(256.0) as u8
    }
}

/// lz4DecompressBlock( input, inputOffset, inputEnd, output, outputOffset,
/// outputEnd ), with `input[ i ]` past the end `undefined` ( NaN ).
fn lz4_block(
    input: &[u8],
    mut io: f64,
    end: f64,
    output: &mut [u8],
    mut out: f64,
    out_end: f64,
) -> f64 {
    let get = |i: f64| -> f64 {
        if i >= 0.0 && i < input.len() as f64 {
            f64::from(input[i as usize])
        } else {
            f64::NAN
        }
    };
    let out_get = |output: &[u8], i: f64| -> f64 {
        if i >= 0.0 && i < output.len() as f64 {
            f64::from(output[i as usize])
        } else {
            f64::NAN
        }
    };
    let store = |output: &mut [u8], i: f64, v: f64| {
        if i >= 0.0 && i < output.len() as f64 {
            output[i as usize] = to_u8(v);
        }
    };
    while io < end {
        let token = get(io);
        io += 1.0;
        if io > end {
            break;
        }
        let mut literal = f64::from(to_i32(token) >> 4);
        if literal == 15.0 {
            loop {
                if io >= end {
                    break;
                }
                let b = get(io);
                io += 1.0;
                literal += b;
                if !(b == 255.0 && io < end) {
                    break;
                }
            }
        }
        if literal > 0.0 {
            if io + literal > end {
                literal = end - io;
            }
            let mut i = 0.0;
            while i < literal {
                if out >= out_end {
                    break;
                }
                let v = get(io);
                store(output, out, v);
                out += 1.0;
                io += 1.0;
                i += 1.0;
            }
        }
        if io >= end {
            break;
        }
        if io + 2.0 > end {
            break;
        }
        let offset = f64::from(to_i32(get(io)) | (to_i32(get(io + 1.0)) << 8));
        io += 2.0;
        if offset == 0.0 {
            break;
        }
        let mut length = f64::from(to_i32(token) & 0x0f) + 4.0;
        if length == 19.0 {
            loop {
                if io >= end {
                    break;
                }
                let b = get(io);
                io += 1.0;
                length += b;
                if !(b == 255.0 && io < end) {
                    break;
                }
            }
        }
        let pos = out - offset;
        if pos < 0.0 {
            break;
        }
        let mut i = 0.0;
        while i < length {
            if out >= out_end {
                break;
            }
            let v = out_get(output, pos + i);
            store(output, out, v);
            out += 1.0;
            i += 1.0;
        }
    }
    out
}

/// `new Uint8Array( length )`.
fn bytes_of(length: f64) -> Result<Vec<u8>> {
    if !(0.0..=4294967295.0).contains(&length) || length.fract() != 0.0 {
        return Err(Error::Asset(
            "RangeError: Invalid typed array length".into(),
        ));
    }
    Ok(vec![0; length as usize])
}

/// decompressLZ4( input, uncompressedSize ).
fn decompress_lz4(input: &[u8], size: f64) -> Result<Vec<u8>> {
    let mut output = bytes_of(size)?;
    let Some(&chunks) = input.first() else {
        return Ok(output);
    };
    if chunks == 0 {
        lz4_block(input, 1.0, input.len() as f64, &mut output, 0.0, size);
        return Ok(output);
    }
    let byte = |i: usize| u32::from(input.get(i).copied().unwrap_or(0));
    let mut header = 1;
    let mut sizes = vec![];
    for _ in 0..chunks {
        sizes.push(f64::from(
            byte(header)
                | (byte(header + 1) << 8)
                | (byte(header + 2) << 16)
                | (byte(header + 3) << 24),
        ));
        header += 4;
    }
    let mut io = header as f64;
    let mut out = 0.0;
    for s in sizes {
        let chunk_out = 65536f64.min(size - out);
        lz4_block(input, io, io + s, &mut output, out, out + chunk_out);
        io += s;
        out += chunk_out;
    }
    Ok(output)
}

/// decompressIntegers32( compressedData, numInts ): an Int32Array.
fn decompress_integers(compressed: &[u8], count: f64) -> Result<Vec<f64>> {
    let encoded_size = count * 4.0 + f64::from(to_i32(count * 2.0 + 7.0) >> 3) + 4.0;
    let data = decompress_lz4(compressed, encoded_size)?;
    decode_integers(&data, count)
}
fn decode_integers(data: &[u8], count: f64) -> Result<Vec<f64>> {
    let read = |at: usize, n: usize| -> Result<&[u8]> {
        data.get(at..at + n)
            .ok_or_else(|| range_error("Offset is outside the bounds of the DataView"))
    };
    let common = f64::from(i32::from_le_bytes(read(0, 4)?.try_into().unwrap_or([0; 4])));
    let codes_bytes = (to_i32(count * 2.0 + 7.0) >> 3).max(0) as usize;
    let mut codes = 4;
    let mut vints = 4 + codes_bytes;
    let n = bytes_of(count)?.len();
    let mut result = Vec::with_capacity(n);
    let mut prev = 0.0;
    let mut i = 0;
    while i < n {
        let code_byte = data.get(codes).map_or(0, |&b| u32::from(b));
        codes += 1;
        let mut j = 0;
        while j < 4 && i < n {
            let delta = match (code_byte >> (j * 2)) & 3 {
                0 => common,
                1 => {
                    let v = f64::from(read(vints, 1)?[0] as i8);
                    vints += 1;
                    v
                }
                2 => {
                    let v = f64::from(i16::from_le_bytes(
                        read(vints, 2)?.try_into().unwrap_or([0; 2]),
                    ));
                    vints += 2;
                    v
                }
                _ => {
                    let v = f64::from(i32::from_le_bytes(
                        read(vints, 4)?.try_into().unwrap_or([0; 4]),
                    ));
                    vints += 4;
                    v
                }
            };
            prev += delta;
            result.push(Typed::I32.store(prev));
            j += 1;
            i += 1;
        }
    }
    Ok(result)
}

struct Reader<'a> {
    buffer: &'a [u8],
    offset: f64,
}
impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8]> {
        let at = self.offset;
        if at < 0.0 || at.fract() != 0.0 || at + n as f64 > self.buffer.len() as f64 {
            return Err(range_error("Offset is outside the bounds of the DataView"));
        }
        self.offset += n as f64;
        Ok(&self.buffer[at as usize..at as usize + n])
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn i8(&mut self) -> Result<i8> {
        Ok(self.take(1)?[0] as i8)
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().unwrap_or([0; 2]),
        ))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().unwrap_or([0; 4]),
        ))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(
            self.take(4)?.try_into().unwrap_or([0; 4]),
        ))
    }
    fn u64(&mut self) -> Result<f64> {
        let lo = f64::from(self.u32()?);
        let hi = f64::from(self.u32()?);
        Ok(hi * 4294967296.0 + lo)
    }
    fn i64(&mut self) -> Result<f64> {
        let lo = f64::from(self.u32()?);
        let hi = f64::from(self.i32()?);
        Ok(hi * 4294967296.0 + lo)
    }
    fn f32(&mut self) -> Result<f64> {
        Ok(f64::from(f32::from_le_bytes(
            self.take(4)?.try_into().unwrap_or([0; 4]),
        )))
    }
    fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(
            self.take(8)?.try_into().unwrap_or([0; 8]),
        ))
    }
    /// readBytes( length ): `new Uint8Array( buffer, offset, length )`.
    fn bytes(&mut self, length: f64) -> Result<Vec<u8>> {
        if !(0.0..=4294967295.0).contains(&length) || length.fract() != 0.0 {
            return Err(range_error("Invalid typed array length"));
        }
        Ok(self.take(length as usize)?.to_vec())
    }
    fn string(&mut self, length: usize) -> Result<String> {
        let b = self.take(length)?;
        let end = b.iter().position(|&c| c == 0).unwrap_or(length);
        Ok(String::from_utf8_lossy(&b[..end]).into_owned())
    }
}

/// ValueRep: the low and high 32 bits.
#[derive(Clone, Copy)]
struct Rep {
    lo: f64,
    hi: f64,
}
impl Rep {
    fn hi(self) -> i32 {
        to_i32(self.hi)
    }
    fn is_array(self) -> bool {
        self.hi() as u32 & 0x8000_0000 != 0
    }
    fn is_inlined(self) -> bool {
        self.hi() as u32 & 0x4000_0000 != 0
    }
    fn is_compressed(self) -> bool {
        self.hi() as u32 & 0x2000_0000 != 0
    }
    fn type_enum(self) -> u32 {
        ((self.hi() >> 16) & 0xff) as u32
    }
    fn payload(self) -> f64 {
        self.lo + f64::from(self.hi() & 0xffff) * 4294967296.0
    }
}

fn half_to_float(h: u16) -> f64 {
    let sign = (h & 0x8000) >> 15;
    let exp = (h & 0x7c00) >> 10;
    let frac = f64::from(h & 0x03ff);
    let s = if sign != 0 { -1.0 } else { 1.0 };
    if exp == 0 {
        if frac == 0.0 {
            return if sign != 0 { -0.0 } else { 0.0 };
        }
        return s * 2f64.powi(-14) * (frac / 1024.0);
    } else if exp == 31 {
        return if frac != 0.0 {
            f64::NAN
        } else if sign != 0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    s * 2f64.powi(i32::from(exp) - 15) * (1.0 + frac / 1024.0)
}

struct Field {
    token: f64,
    rep: Rep,
}

struct Parser<'a> {
    reader: Reader<'a>,
    version: (u8, u8, u8),
    sections: HashMap<String, (f64, f64)>,
    tokens: Option<Vec<String>>,
    strings: Vec<f64>,
    fields: Option<Vec<Field>>,
    field_sets: Option<Vec<f64>>,
    paths: Option<Vec<Option<String>>>,
    specs: Option<Vec<(f64, f64, f64)>>,
}

impl Parser<'_> {
    fn legacy(&self, minor: u8) -> bool {
        self.version.0 == 0 && self.version.1 < minor
    }
    fn tokens(&self) -> Result<&Vec<String>> {
        self.tokens
            .as_ref()
            .ok_or_else(|| range_error("Cannot read properties of undefined"))
    }
    /// `this.tokens[ i ] || ''`.
    fn token(&self, i: f64) -> Result<String> {
        let t = self.tokens()?;
        Ok(if i >= 0.0 && i.fract() == 0.0 && (i as usize) < t.len() {
            t[i as usize].clone()
        } else {
            String::new()
        })
    }
    /// `this.tokens[ i ]`, undefined past the end.
    fn token_opt(&self, i: f64) -> Result<Option<String>> {
        let t = self.tokens()?;
        Ok((i >= 0.0 && i.fract() == 0.0 && (i as usize) < t.len()).then(|| t[i as usize].clone()))
    }
    fn string_index(&self, i: f64) -> f64 {
        if i >= 0.0 && i.fract() == 0.0 && (i as usize) < self.strings.len() {
            self.strings[i as usize]
        } else {
            f64::NAN
        }
    }
    fn path(&self, i: f64) -> Option<String> {
        let p = self.paths.as_ref()?;
        if i >= 0.0 && i.fract() == 0.0 && (i as usize) < p.len() {
            p[i as usize].clone()
        } else {
            None
        }
    }

    fn bootstrap(&mut self) -> Result<f64> {
        self.reader.offset = 0.0;
        if self.reader.string(8)? != "PXR-USDC" {
            return Err(Error::Asset(
                "THREE.USDCParser: Not a valid USDC file.".into(),
            ));
        }
        self.version = (self.reader.u8()?, self.reader.u8()?, self.reader.u8()?);
        self.reader.bytes(5.0)?;
        self.reader.u64()
    }
    fn toc(&mut self, offset: f64) -> Result<()> {
        self.reader.offset = offset;
        let n = self.reader.u64()?;
        let mut i = 0.0;
        while i < n {
            let name = self.reader.string(16)?;
            let start = self.reader.u64()?;
            let size = self.reader.u64()?;
            self.sections.insert(name, (start, size));
            i += 1.0;
        }
        Ok(())
    }
    fn split_tokens(data: &[u8], count: f64) -> Vec<String> {
        let mut out = vec![];
        let mut start = 0;
        let mut i = 0.0;
        while i < count {
            let mut end = start;
            while end < data.len() && data[end] != 0 {
                end += 1;
            }
            let s = data
                .get(start.min(data.len())..end.max(start.min(data.len())))
                .unwrap_or(&[]);
            out.push(String::from_utf8_lossy(s).into_owned());
            start = end + 1;
            i += 1.0;
        }
        out
    }
    fn read_tokens(&mut self) -> Result<()> {
        let Some(&(start, _)) = self.sections.get("TOKENS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let count = self.reader.u64()?;
        let data = if self.legacy(4) {
            let n = self.reader.u64()?;
            self.reader.bytes(n)?
        } else {
            let uncompressed = self.reader.u64()?;
            let compressed = self.reader.u64()?;
            let c = self.reader.bytes(compressed)?;
            decompress_lz4(&c, uncompressed)?
        };
        self.tokens = Some(Self::split_tokens(&data, count));
        Ok(())
    }
    fn read_strings(&mut self) -> Result<()> {
        let Some(&(start, _)) = self.sections.get("STRINGS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let n = self.reader.u64()?;
        let mut i = 0.0;
        while i < n {
            let v = f64::from(self.reader.u32()?);
            self.strings.push(v);
            i += 1.0;
        }
        Ok(())
    }
    /// A compressed integer section: the size, then the LZ4 bytes.
    fn compressed_ints(&mut self, count: f64) -> Result<Vec<f64>> {
        let size = self.reader.u64()?;
        let c = self.reader.bytes(size)?;
        decompress_integers(&c, count)
    }
    fn read_fields(&mut self) -> Result<()> {
        let Some(&(start, size)) = self.sections.get("FIELDS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let mut fields = vec![];
        if self.legacy(4) {
            let n = (size / 12.0).floor();
            let mut i = 0.0;
            while i < n {
                let token = f64::from(self.reader.u32()?);
                let lo = f64::from(self.reader.u32()?);
                let hi = f64::from(self.reader.u32()?);
                fields.push(Field {
                    token,
                    rep: Rep { lo, hi },
                });
                i += 1.0;
            }
        } else {
            let n = self.reader.u64()?;
            let tokens = self.compressed_ints(n)?;
            let reps_size = self.reader.u64()?;
            let reps = self.reader.bytes(reps_size)?;
            let data = decompress_lz4(&reps, n * 8.0)?;
            for (i, &token) in tokens.iter().enumerate() {
                let w = |at: usize| {
                    f64::from(u32::from_le_bytes(
                        data[at..at + 4].try_into().unwrap_or([0; 4]),
                    ))
                };
                fields.push(Field {
                    token,
                    rep: Rep {
                        lo: w(i * 8),
                        hi: w(i * 8 + 4),
                    },
                });
            }
        }
        self.fields = Some(fields);
        Ok(())
    }
    fn read_field_sets(&mut self) -> Result<()> {
        let Some(&(start, size)) = self.sections.get("FIELDSETS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let sets = if self.legacy(4) {
            let n = (size / 4.0).floor();
            let mut v = vec![];
            let mut i = 0.0;
            while i < n {
                v.push(f64::from(self.reader.u32()?));
                i += 1.0;
            }
            v
        } else {
            let n = self.reader.u64()?;
            self.compressed_ints(n)?
        };
        self.field_sets = Some(sets);
        Ok(())
    }
    fn set_path(&mut self, index: f64, path: String) {
        if let Some(paths) = &mut self.paths
            && index >= 0.0
            && index.fract() == 0.0
            && index < 4294967295.0
        {
            let i = index as usize;
            if i >= paths.len() {
                paths.resize(i + 1, None);
            }
            paths[i] = Some(path);
        }
    }
    fn read_paths(&mut self) -> Result<()> {
        let Some(&(start, _)) = self.sections.get("PATHS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let n = self.reader.u64()?;
        let count = bytes_of(n)
            .map_err(|_| range_error("Invalid array length"))?
            .len();
        self.paths = Some(vec![Some(String::new()); count]);
        if self.legacy(4) {
            self.paths_recursive("", 0)?;
        } else {
            self.reader.u64()?;
            let indices = self.compressed_ints(n)?;
            let tokens = self.compressed_ints(n)?;
            let jumps = self.compressed_ints(n)?;
            self.paths_from_compressed(&indices, &tokens, &jumps, 0, String::new())?;
        }
        Ok(())
    }
    fn paths_recursive(&mut self, parent: &str, depth: usize) -> Result<()> {
        if depth > 1000 {
            return Ok(());
        }
        let index = f64::from(self.reader.u32()?);
        let token = f64::from(self.reader.u32()?);
        let bits = self.reader.u8()?;
        let (child, sibling, property) = (bits & 1 != 0, bits & 2 != 0, bits & 4 != 0);
        let path = if parent.is_empty() {
            "/".to_string()
        } else {
            let elem = self.token(token)?;
            if property {
                format!("{parent}.{elem}")
            } else if parent == "/" {
                format!("/{elem}")
            } else {
                format!("{parent}/{elem}")
            }
        };
        self.set_path(index, path.clone());
        if child && sibling {
            let sibling_offset = self.reader.u64()?;
            self.paths_recursive(&path, depth + 1)?;
            self.reader.offset = sibling_offset;
            self.paths_recursive(parent, depth + 1)?;
        } else if child {
            self.paths_recursive(&path, depth + 1)?;
        } else if sibling {
            self.paths_recursive(parent, depth + 1)?;
        }
        Ok(())
    }
    fn paths_from_compressed(
        &mut self,
        indices: &[f64],
        tokens: &[f64],
        jumps: &[f64],
        start: usize,
        mut parent: String,
    ) -> Result<()> {
        let mut cur = start;
        while cur < indices.len() {
            let this = cur;
            cur += 1;
            let element = tokens.get(this).copied().unwrap_or(f64::NAN);
            let jump = jumps.get(this).copied().unwrap_or(f64::NAN);
            let path = if parent.is_empty() {
                parent = "/".into();
                "/".to_string()
            } else {
                let elem = self.token(element.abs())?;
                if element < 0.0 {
                    format!("{parent}.{elem}")
                } else if parent == "/" {
                    format!("/{elem}")
                } else {
                    format!("{parent}/{elem}")
                }
            };
            self.set_path(indices[this], path.clone());
            let child = jump > 0.0 || jump == -1.0;
            let sibling = jump >= 0.0;
            if child {
                if sibling {
                    let s = this as f64 + jump;
                    if s >= 0.0 {
                        self.paths_from_compressed(
                            indices,
                            tokens,
                            jumps,
                            s as usize,
                            parent.clone(),
                        )?;
                    }
                }
                parent = path;
            } else if !sibling {
                break;
            }
        }
        Ok(())
    }
    fn read_specs(&mut self) -> Result<()> {
        let Some(&(start, size)) = self.sections.get("SPECS") else {
            return Ok(());
        };
        self.reader.offset = start;
        let mut specs = vec![];
        if self.legacy(4) {
            let spec_size = if self.version.1 == 0 && self.version.2 == 1 {
                16.0
            } else {
                12.0
            };
            let n = (size / spec_size).floor();
            let mut i = 0.0;
            while i < n {
                let path = f64::from(self.reader.u32()?);
                let set = f64::from(self.reader.u32()?);
                let ty = f64::from(self.reader.u32()?);
                if spec_size == 16.0 {
                    self.reader.u32()?;
                }
                specs.push((path, set, ty));
                i += 1.0;
            }
        } else {
            let n = self.reader.u64()?;
            let paths = self.compressed_ints(n)?;
            let sets = self.compressed_ints(n)?;
            let types = self.compressed_ints(n)?;
            for i in 0..paths.len() {
                specs.push((paths[i], sets[i], types[i]));
            }
        }
        self.specs = Some(specs);
        Ok(())
    }

    fn read_value(&mut self, rep: Rep) -> Result<Value> {
        let ty = rep.type_enum();
        if ty == ty::TIME_SAMPLES {
            return self.time_samples(rep);
        }
        if rep.is_inlined() {
            return self.inlined(rep);
        }
        let offset = rep.payload();
        if offset == 0.0 && rep.is_array() {
            return Ok(Value::array(vec![]));
        }
        if offset < 0.0 || offset >= self.reader.buffer.len() as f64 {
            return Err(range_error(&format!(
                "Invalid payload offset {offset} for type {ty}."
            )));
        }
        let saved = self.reader.offset;
        self.reader.offset = offset;
        let value = if rep.is_array() {
            self.array(rep)?
        } else {
            self.scalar(ty)?
        };
        self.reader.offset = saved;
        Ok(value)
    }
    fn inlined(&self, rep: Rep) -> Result<Value> {
        let payload = rep.lo;
        let bytes = (payload as u32).to_le_bytes();
        let i8s =
            |n: usize| Value::numbers(bytes[..n].iter().map(|&b| f64::from(b as i8)).collect());
        Ok(match rep.type_enum() {
            ty::BOOL => Value::Bool(payload != 0.0),
            ty::UCHAR => Value::Number(f64::from(payload as u32 & 0xff)),
            ty::INT | ty::UINT => Value::Number(payload),
            ty::FLOAT | ty::DOUBLE => Value::Number(f64::from(f32::from_le_bytes(bytes))),
            ty::TOKEN | ty::ASSET_PATH => Value::string(&self.token(payload)?),
            ty::STRING => Value::string(&self.token(self.string_index(payload))?),
            ty::SPECIFIER | ty::PERMISSION | ty::VARIABILITY => Value::Number(payload),
            ty::VEC2H => Value::numbers(vec![
                half_to_float(u16::from_le_bytes([bytes[0], bytes[1]])),
                half_to_float(u16::from_le_bytes([bytes[2], bytes[3]])),
            ]),
            ty::VEC2F | ty::VEC2I => i8s(2),
            ty::VEC3F | ty::VEC3I => i8s(3),
            ty::VEC4F | ty::VEC4I => i8s(4),
            ty::MATRIX2D | ty::MATRIX3D | ty::MATRIX4D => {
                let n = match rep.type_enum() {
                    ty::MATRIX2D => 2,
                    ty::MATRIX3D => 3,
                    _ => 4,
                };
                let mut m = vec![0.0; n * n];
                for i in 0..n {
                    m[i * n + i] = f64::from(bytes[i] as i8);
                }
                Value::numbers(m)
            }
            _ => Value::Number(payload),
        })
    }
    fn time_samples(&mut self, rep: Rep) -> Result<Value> {
        let saved = self.reader.offset;
        let start = rep.payload();
        self.reader.offset = start;
        let rel = self.reader.i64()?;
        self.reader.offset = start + rel;
        let lo = f64::from(self.reader.u32()?);
        let hi = f64::from(self.reader.u32()?);
        let times = self.read_value(Rep { lo, hi })?;
        self.reader.offset = start + rel + 8.0;
        let values_start = self.reader.offset;
        let rel = self.reader.i64()?;
        self.reader.offset = values_start + rel;
        let n = self.reader.u64()?;
        let mut reps = vec![];
        let mut i = 0.0;
        while i < n {
            let lo = f64::from(self.reader.u32()?);
            let hi = f64::from(self.reader.u32()?);
            reps.push(Rep { lo, hi });
            i += 1.0;
        }
        let mut values = vec![];
        for r in reps {
            values.push(self.read_value(r)?);
        }
        self.reader.offset = saved;
        let times = match times {
            Value::TypedArray(super::js::Typed::F64, a) => Value::numbers(a.borrow().clone()),
            t @ Value::Array(_) => t,
            t => Value::array(vec![t]),
        };
        let o = Obj::new();
        o.set("times", times);
        o.set("values", Value::array(values));
        Ok(Value::Object(o))
    }
    fn scalar(&mut self, ty: u32) -> Result<Value> {
        let r = &mut self.reader;
        Ok(match ty {
            ty::INVALID => Value::Null,
            ty::BOOL => Value::Bool(r.u8()? != 0),
            ty::UCHAR => Value::Number(f64::from(r.u8()?)),
            ty::INT => Value::Number(f64::from(r.i32()?)),
            ty::UINT => Value::Number(f64::from(r.u32()?)),
            ty::INT64 => Value::Number(r.i64()?),
            ty::UINT64 => Value::Number(r.u64()?),
            ty::HALF => Value::Number(half_to_float(r.u16()?)),
            ty::FLOAT => Value::Number(r.f32()?),
            ty::DOUBLE => Value::Number(r.f64()?),
            ty::STRING | ty::TOKEN | ty::ASSET_PATH => {
                let i = f64::from(r.u32()?);
                Value::string(&self.token(i)?)
            }
            ty::VEC2F => Value::numbers(vec![r.f32()?, r.f32()?]),
            ty::VEC2D => Value::numbers(vec![r.f64()?, r.f64()?]),
            ty::VEC2I => Value::numbers(vec![f64::from(r.i32()?), f64::from(r.i32()?)]),
            ty::VEC3F => Value::numbers(vec![r.f32()?, r.f32()?, r.f32()?]),
            ty::VEC3D => Value::numbers(vec![r.f64()?, r.f64()?, r.f64()?]),
            ty::VEC3I => Value::numbers(vec![
                f64::from(r.i32()?),
                f64::from(r.i32()?),
                f64::from(r.i32()?),
            ]),
            ty::VEC4F | ty::QUATF => Value::numbers(vec![r.f32()?, r.f32()?, r.f32()?, r.f32()?]),
            ty::VEC4D | ty::QUATD => Value::numbers(vec![r.f64()?, r.f64()?, r.f64()?, r.f64()?]),
            ty::MATRIX4D => {
                let mut m = vec![];
                for _ in 0..16 {
                    m.push(r.f64()?);
                }
                Value::numbers(m)
            }
            ty::TOKEN_VECTOR => {
                let n = r.u64()?;
                let mut idx = vec![];
                let mut i = 0.0;
                while i < n {
                    idx.push(f64::from(self.reader.u32()?));
                    i += 1.0;
                }
                let mut out = vec![];
                for i in idx {
                    out.push(Value::string(&self.token(i)?));
                }
                Value::array(out)
            }
            ty::PATH_VECTOR => {
                let n = r.u64()?;
                let mut out = vec![];
                let mut i = 0.0;
                while i < n {
                    let p = f64::from(self.reader.u32()?);
                    out.push(Value::string(&self.path(p).unwrap_or_default()));
                    i += 1.0;
                }
                Value::array(out)
            }
            ty::DOUBLE_VECTOR => {
                let n = r.u64()?;
                let len = bytes_of(n)?.len();
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    v.push(self.reader.f64()?);
                }
                Value::typed(Typed::F64, v)
            }
            ty::DICTIONARY => {
                // The original reads `reader.position`, which BinaryReader
                // does not have: each entry consumes its key index, offset
                // and value rep, and a rep without its high word reads as
                // Invalid ( null ), so the dictionary stays empty.
                let n = r.u64()?;
                let mut i = 0.0;
                while i < n {
                    let key = f64::from(self.reader.u32()?);
                    self.token_opt(key)?;
                    self.reader.i64()?;
                    self.reader.u64()?;
                    i += 1.0;
                }
                Value::Object(Obj::new())
            }
            ty::TOKEN_LIST_OP
            | ty::STRING_LIST_OP
            | ty::INT_LIST_OP
            | ty::INT64_LIST_OP
            | ty::UINT_LIST_OP
            | ty::UINT64_LIST_OP => Value::Null,
            ty::PATH_LIST_OP => {
                let flags = r.u8()?;
                let list = |p: &mut Self| -> Result<Vec<Value>> {
                    let n = p.reader.u64()?;
                    let mut out = vec![];
                    let mut i = 0.0;
                    while i < n {
                        let idx = f64::from(p.reader.u32()?);
                        out.push(p.path(idx).map_or(Value::Undefined, |s| Value::string(&s)));
                        i += 1.0;
                    }
                    Ok(out)
                };
                let explicit = if flags & 0x02 != 0 {
                    Some(list(self)?)
                } else {
                    None
                };
                let add = if flags & 0x04 != 0 {
                    Some(list(self)?)
                } else {
                    None
                };
                let prepend = if flags & 0x20 != 0 {
                    Some(list(self)?)
                } else {
                    None
                };
                let append = if flags & 0x40 != 0 {
                    Some(list(self)?)
                } else {
                    None
                };
                if flags & 0x08 != 0 {
                    list(self)?;
                }
                if flags & 0x10 != 0 {
                    list(self)?;
                }
                for l in [prepend, explicit, append, add].into_iter().flatten() {
                    if !l.is_empty() {
                        return Ok(Value::array(l));
                    }
                }
                Value::Null
            }
            ty::VARIANT_SELECTION_MAP => {
                let n = r.u64()?;
                let map = Obj::new();
                let mut i = 0.0;
                while i < n {
                    let k = f64::from(self.reader.u32()?);
                    let v = f64::from(self.reader.u32()?);
                    let key = self.token_opt(self.string_index(k))?;
                    let value = self.token_opt(self.string_index(v))?;
                    if let (Some(key), Some(value)) = (key, value)
                        && !key.is_empty()
                        && !value.is_empty()
                    {
                        map.set(&key, Value::string(&value));
                    }
                    i += 1.0;
                }
                Value::Object(map)
            }
            _ => Value::Null,
        })
    }
    fn array(&mut self, rep: Rep) -> Result<Value> {
        let ty = rep.type_enum();
        let size = if self.legacy(7) {
            f64::from(self.reader.u32()?)
        } else {
            self.reader.u64()?
        };
        if !(size.fract() == 0.0 && size.abs() <= 9007199254740991.0) || size < 0.0 {
            return Err(range_error(&format!(
                "Invalid array size {size} for type {ty}."
            )));
        }
        if size > 2147483647.0 {
            return Err(range_error(&format!(
                "Array size {size} exceeds implementation limits."
            )));
        }
        if size == 0.0 {
            return Ok(Value::array(vec![]));
        }
        if rep.is_compressed() {
            return self.compressed_array(ty, size);
        }
        let n = size as usize;
        let r = &mut self.reader;
        let mut read =
            |count: usize, f: &mut dyn FnMut(&mut Reader) -> Result<f64>| -> Result<Vec<f64>> {
                let mut v = Vec::with_capacity(count.min(1 << 24));
                for _ in 0..count {
                    v.push(f(r)?);
                }
                Ok(v)
            };
        Ok(match ty {
            ty::INT => Value::typed(Typed::I32, read(n, &mut |r| Ok(f64::from(r.i32()?)))?),
            ty::UINT => Value::typed(Typed::U32, read(n, &mut |r| Ok(f64::from(r.u32()?)))?),
            ty::FLOAT => Value::typed(Typed::F32, read(n, &mut |r| r.f32())?),
            ty::DOUBLE => Value::typed(Typed::F64, read(n, &mut |r| r.f64())?),
            ty::VEC2F => Value::typed(Typed::F32, read(n * 2, &mut |r| r.f32())?),
            ty::VEC3F => Value::typed(Typed::F32, read(n * 3, &mut |r| r.f32())?),
            ty::VEC4F | ty::QUATF => Value::typed(Typed::F32, read(n * 4, &mut |r| r.f32())?),
            ty::VEC3H => Value::typed(
                Typed::F32,
                read(n * 3, &mut |r| Ok(half_to_float(r.u16()?)))?,
            ),
            ty::QUATH => Value::typed(
                Typed::F32,
                read(n * 4, &mut |r| Ok(half_to_float(r.u16()?)))?,
            ),
            ty::MATRIX4D => Value::typed(Typed::F64, read(n * 16, &mut |r| r.f64())?),
            ty::HALF => Value::typed(Typed::F32, read(n, &mut |r| Ok(half_to_float(r.u16()?)))?),
            ty::TOKEN => {
                let idx = read(n, &mut |r| Ok(f64::from(r.u32()?)))?;
                let mut out = vec![];
                for i in idx {
                    out.push(Value::string(&self.token(i)?));
                }
                Value::array(out)
            }
            _ => Value::array(vec![]),
        })
    }
    fn compressed_array(&mut self, ty: u32, size: f64) -> Result<Value> {
        Ok(match ty {
            ty::INT | ty::UINT => Value::typed(Typed::I32, self.compressed_ints(size)?),
            ty::FLOAT => {
                let code = self.reader.i8()?;
                if code == FLOAT_COMPRESSION_INT {
                    let ints = self.compressed_ints(size)?;
                    Value::typed(
                        Typed::F32,
                        ints.into_iter().map(|v| Typed::F32.store(v)).collect(),
                    )
                } else if code == FLOAT_COMPRESSION_LUT {
                    let lut_size = self.reader.u32()?;
                    let mut lut = vec![];
                    for _ in 0..lut_size {
                        lut.push(self.reader.f32()?);
                    }
                    let indices = self.compressed_ints(size)?;
                    Value::typed(
                        Typed::F32,
                        indices
                            .into_iter()
                            .map(|i| {
                                if i >= 0.0 && (i as usize) < lut.len() {
                                    lut[i as usize]
                                } else {
                                    f64::NAN
                                }
                            })
                            .collect(),
                    )
                } else {
                    Value::typed(Typed::F32, vec![0.0; size as usize])
                }
            }
            _ => Value::array(vec![]),
        })
    }
    fn fields_for_spec(&mut self, set_index: f64) -> Result<Obj> {
        let fields = Obj::new();
        let sets = self.field_sets.clone().ok_or_else(|| {
            range_error("Cannot read properties of undefined ( reading 'length' )")
        })?;
        let mut i = set_index;
        let mut iterations = 0;
        while i < sets.len() as f64 && iterations < 10000 {
            let field_index = if i >= 0.0 && i.fract() == 0.0 {
                sets[i as usize]
            } else {
                f64::NAN
            };
            if field_index == FIELD_SET_TERMINATOR || field_index == -1.0 {
                break;
            }
            let field = {
                let all = self
                    .fields
                    .as_ref()
                    .ok_or_else(|| range_error("Cannot read properties of undefined"))?;
                (field_index >= 0.0
                    && field_index.fract() == 0.0
                    && (field_index as usize) < all.len())
                .then(|| {
                    (
                        all[field_index as usize].token,
                        all[field_index as usize].rep,
                    )
                })
            };
            if let Some((token, rep)) = field {
                let name = self.token_opt(token)?.unwrap_or_else(|| "undefined".into());
                let value = self.read_value(rep)?;
                fields.set(&name, value);
            }
            i += 1.0;
            iterations += 1;
        }
        Ok(fields)
    }
}

/// parseData( buffer ).
pub(crate) fn parse(buffer: &[u8]) -> Result<Specs> {
    let mut p = Parser {
        reader: Reader {
            buffer,
            offset: 0.0,
        },
        version: (0, 0, 0),
        sections: HashMap::new(),
        tokens: None,
        strings: vec![],
        fields: None,
        field_sets: None,
        paths: None,
        specs: None,
    };
    let toc = p.bootstrap()?;
    p.toc(toc)?;
    p.read_tokens()?;
    p.read_strings()?;
    p.read_fields()?;
    p.read_field_sets()?;
    p.read_paths()?;
    p.read_specs()?;
    let mut specs = Specs::default();
    let all = p
        .specs
        .clone()
        .ok_or_else(|| range_error("this.specs is not iterable"))?;
    for (path_index, set_index, spec_type) in all {
        let Some(path) = p.path(path_index).filter(|s| !s.is_empty()) else {
            continue;
        };
        let fields = p.fields_for_spec(set_index)?;
        specs.set(&path, Spec::new(spec_type, fields));
    }
    Ok(specs)
}
