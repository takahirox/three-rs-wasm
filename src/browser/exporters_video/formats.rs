//! EXRExporter and KTX2Exporter (ktx-parse `write`) byte layouts.
use crate::browser::refraction_loaders::formats::to_half_float;

/// RGBA pixel data as the exporters receive it: a Float32Array, or the
/// Uint16Array of half floats that a HalfFloatType readback returns.
pub(super) enum Pixels<'a> {
    Float(&'a [f32]),
    Half(&'a [u16]),
}
impl Pixels<'_> {
    fn get(&self, i: usize) -> f64 {
        match self {
            Pixels::Float(v) => v[i] as f64,
            Pixels::Half(v) => decode_float16(v[i]),
        }
    }
}
/// EXRExporter's decodeFloat16.
fn decode_float16(binary: u16) -> f64 {
    let exponent = (binary & 0x7c00) >> 10;
    let fraction = (binary & 0x03ff) as f64;
    let sign = if binary >> 15 != 0 { -1. } else { 1. };
    sign * if exponent != 0 {
        if exponent == 0x1f {
            if fraction != 0. {
                f64::NAN
            } else {
                f64::INFINITY
            }
        } else {
            2f64.powi(exponent as i32 - 15) * (1. + fraction / 1024.)
        }
    } else {
        6.103515625e-5 * (fraction / 1024.)
    }
}

pub(super) const NO_COMPRESSION: u8 = 0;
pub(super) const ZIPS_COMPRESSION: u8 = 2;
pub(super) const ZIP_COMPRESSION: u8 = 3;

/// EXRExporter.parse: four channels written A, B, G, R per scanline, bottom row
/// first (the WebGL readback order), in blocks of 1 (NONE, ZIPS) or 16 (ZIP)
/// lines. `half` selects HalfFloatType output over FloatType.
pub(super) fn exr(
    pixels: &Pixels,
    width: usize,
    height: usize,
    half: bool,
    compression: u8,
) -> Vec<u8> {
    let data_size = if half { 2 } else { 4 };
    let mut raw = vec![0u8; width * height * 4 * data_size];
    let mut put = |offset: usize, value: f64| {
        if half {
            raw[offset..offset + 2].copy_from_slice(&to_half_float(value as f32).to_le_bytes());
        } else {
            raw[offset..offset + 4].copy_from_slice(&(value as f32).to_le_bytes());
        }
    };
    for y in 0..height {
        for x in 0..width {
            let i = y * width * 4 + x * 4;
            let line = (height - y - 1) * width * 4 * data_size;
            for (channel, source) in [3, 2, 1, 0].into_iter().enumerate() {
                put(
                    line + channel * width * data_size + x * data_size,
                    pixels.get(i + source),
                );
            }
        }
    }
    let block_lines = if compression == ZIP_COMPRESSION {
        16
    } else {
        1
    };
    let blocks = height.div_ceil(block_lines);
    let size = width * 4 * block_lines * data_size;
    let chunks: Vec<Vec<u8>> = (0..blocks)
        .map(|i| {
            let block = &raw[(size * i).min(raw.len())..(size * (i + 1)).min(raw.len())];
            if compression == NO_COMPRESSION {
                block.to_vec()
            } else {
                compress_zip(block, size)
            }
        })
        .collect();
    let mut out = Vec::new();
    let u32le = |out: &mut Vec<u8>, v: u32| out.extend(v.to_le_bytes());
    let string = |out: &mut Vec<u8>, s: &str| {
        out.extend(s.as_bytes());
        out.push(0);
    };
    u32le(&mut out, 20000630);
    u32le(&mut out, 2);
    string(&mut out, "compression");
    string(&mut out, "compression");
    u32le(&mut out, 1);
    out.push(compression);
    string(&mut out, "screenWindowCenter");
    string(&mut out, "v2f");
    for v in [8, 0, 0] {
        u32le(&mut out, v);
    }
    for name in ["screenWindowWidth", "pixelAspectRatio"] {
        string(&mut out, name);
        string(&mut out, "float");
        u32le(&mut out, 4);
        out.extend(1f32.to_le_bytes());
    }
    string(&mut out, "lineOrder");
    string(&mut out, "lineOrder");
    u32le(&mut out, 1);
    out.push(0);
    for name in ["dataWindow", "displayWindow"] {
        string(&mut out, name);
        string(&mut out, "box2i");
        for v in [16, 0, 0, width as u32 - 1, height as u32 - 1] {
            u32le(&mut out, v);
        }
    }
    string(&mut out, "channels");
    string(&mut out, "chlist");
    u32le(&mut out, 4 * 18 + 1);
    for name in ["A", "B", "G", "R"] {
        string(&mut out, name);
        u32le(&mut out, if half { 1 } else { 2 });
        out.extend([0; 4]);
        u32le(&mut out, 1);
        u32le(&mut out, 1);
    }
    out.extend([0, 0]);
    let mut sum = (out.len() + blocks * 8) as u64;
    for chunk in &chunks {
        out.extend(sum.to_le_bytes());
        sum += chunk.len() as u64 + 8;
    }
    for (i, chunk) in chunks.iter().enumerate() {
        u32le(&mut out, (i * block_lines) as u32);
        u32le(&mut out, chunk.len() as u32);
        out.extend(chunk);
    }
    out
}
/// compressZIP: interleave the even and odd bytes, delta-predict, then zlib.
/// The exported heights are multiples of 16, so every block is full.
fn compress_zip(data: &[u8], size: usize) -> Vec<u8> {
    let mut tmp = vec![0u8; size];
    let (mut t1, mut t2) = (0, data.len().div_ceil(2));
    for (s, &byte) in data.iter().enumerate() {
        if s % 2 == 0 {
            tmp[t1] = byte;
            t1 += 1;
        } else {
            tmp[t2] = byte;
            t2 += 1;
        }
    }
    let mut p = tmp[0];
    for t in tmp.iter_mut().skip(1) {
        let d = (*t as i32 - p as i32 + 128 + 256) as u8;
        p = *t;
        *t = d;
    }
    miniz_oxide::deflate::compress_to_vec_zlib(&tmp, 6)
}

/// VK formats and DFD values used by KTX2Exporter for RGBA data.
pub(super) enum Ktx2Kind {
    /// FloatType, NoColorSpace (a DataTexture).
    Float,
    /// HalfFloatType, LinearSRGBColorSpace (a PMREM render target).
    HalfLinear,
}
/// KTX2Exporter.parse with ktx-parse's write( container, { keepWriter: true } ).
pub(super) fn ktx2(kind: Ktx2Kind, data: &[u8], width: u32, height: u32) -> Vec<u8> {
    const VK_FORMAT_R16G16B16A16_SFLOAT: u32 = 97;
    const VK_FORMAT_R32G32B32A32_SFLOAT: u32 = 109;
    let (vk_format, type_size, primaries) = match kind {
        Ktx2Kind::Float => (VK_FORMAT_R32G32B32A32_SFLOAT, 4u32, 0u8),
        Ktx2Kind::HalfLinear => (VK_FORMAT_R16G16B16A16_SFLOAT, 2, 1),
    };
    // Basic data format descriptor: RGBSDA, linear transfer, four float samples.
    let samples = 4;
    let mut dfd = Vec::new();
    dfd.extend((28 + 16 * samples as u32).to_le_bytes());
    dfd.extend(0u16.to_le_bytes());
    dfd.extend(0u16.to_le_bytes());
    dfd.extend(2u16.to_le_bytes());
    dfd.extend((24 + 16 * samples as u16).to_le_bytes());
    dfd.extend([1, primaries, 1, 0]);
    dfd.extend([0; 4]);
    dfd.extend([(type_size * 4) as u8, 0, 0, 0, 0, 0, 0, 0]);
    for (i, channel) in [0u8, 1, 2, 15].into_iter().enumerate() {
        // KHR_DF_SAMPLE_DATATYPE_FLOAT | KHR_DF_SAMPLE_DATATYPE_SIGNED.
        let channel_type = channel | 128 | 64;
        dfd.extend(((i as u32 * type_size * 8) as u16).to_le_bytes());
        dfd.push((type_size * 8 - 1) as u8);
        dfd.push(channel_type);
        dfd.extend([0; 4]);
        dfd.extend(0xbf800000u32.to_le_bytes());
        dfd.extend(0x3f800000u32.to_le_bytes());
    }
    // Key/value data: KTXwriter only.
    let mut kvd = Vec::new();
    let (key, value) = (b"KTXwriter".as_slice(), b"three.js 186".as_slice());
    let length = key.len() + 1 + value.len() + 1;
    kvd.extend((length as u32).to_le_bytes());
    kvd.extend(key);
    kvd.push(0);
    kvd.extend(value);
    kvd.push(0);
    kvd.extend(vec![0; length.div_ceil(4) * 4 - length]);
    const IDENTIFIER: [u8; 12] = [171, 75, 84, 88, 32, 50, 48, 187, 13, 10, 26, 10];
    let dfd_offset = IDENTIFIER.len() + 68 + 3 * 8;
    let kvd_offset = dfd_offset + dfd.len();
    // Level alignment: lcm( texel size, 4 ).
    let texel = (type_size * 4) as usize;
    let alignment = {
        let (e, n) = (texel.max(4), texel.min(4));
        let mut i = e;
        while i % n != 0 {
            i += e;
        }
        i
    };
    let mut x = kvd_offset + kvd.len();
    let padding = (x.div_ceil(alignment) * alignment) - x;
    x += padding;
    let level_offset = x as u64;
    let mut out = Vec::with_capacity(x + data.len());
    out.extend(IDENTIFIER);
    for v in [vk_format, type_size, width, height, 0, 0, 1, 1, 0] {
        out.extend(v.to_le_bytes());
    }
    for v in [dfd_offset, dfd.len(), kvd_offset, kvd.len()] {
        out.extend((v as u32).to_le_bytes());
    }
    out.extend(0u64.to_le_bytes());
    out.extend(0u64.to_le_bytes());
    for v in [level_offset, data.len() as u64, data.len() as u64] {
        out.extend(v.to_le_bytes());
    }
    out.extend(dfd);
    out.extend(kvd);
    out.extend(vec![0; padding]);
    out.extend(data);
    out
}
