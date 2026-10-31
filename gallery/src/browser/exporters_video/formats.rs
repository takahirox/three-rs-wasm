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
pub(super) const RLE_COMPRESSION: u8 = 1;
pub(super) const ZIPS_COMPRESSION: u8 = 2;
pub(super) const ZIP_COMPRESSION: u8 = 3;
pub(super) const PIZ_COMPRESSION: u8 = 4;

/// EXRExporter.parse: four channels written A, B, G, R per scanline, bottom row
/// first (the WebGL readback order), in blocks of 1 (NONE, RLE, ZIPS), 16 (ZIP)
/// or 32 (PIZ) lines. `half` selects HalfFloatType output over FloatType. A
/// block that does not shrink is stored uncompressed, as compressData does.
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
    let block_lines = match compression {
        ZIP_COMPRESSION => 16,
        PIZ_COMPRESSION => 32,
        _ => 1,
    };
    let blocks = height.div_ceil(block_lines);
    let size = width * 4 * block_lines * data_size;
    let chunks: Vec<Vec<u8>> = (0..blocks)
        .map(|i| {
            let block = &raw[(size * i).min(raw.len())..(size * (i + 1)).min(raw.len())];
            let packed = match compression {
                RLE_COMPRESSION => compress_rle(block),
                ZIPS_COMPRESSION | ZIP_COMPRESSION => compress_zip(block),
                PIZ_COMPRESSION => compress_piz(block, width, data_size, 4),
                _ => None,
            };
            match packed {
                Some(packed) if packed.len() < block.len() => packed,
                _ => block.to_vec(),
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
/// reorderAndPredict: interleave the even and odd bytes, then delta-predict.
fn reorder_and_predict(data: &[u8]) -> Vec<u8> {
    let mut tmp = vec![0u8; data.len()];
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
    if let Some(&first) = tmp.first() {
        let mut p = first;
        for t in tmp.iter_mut().skip(1) {
            let d = (*t as i32 - p as i32 + 128 + 256) as u8;
            p = *t;
            *t = d;
        }
    }
    tmp
}
/// compressZIP: reorderAndPredict, then zlib.
fn compress_zip(data: &[u8]) -> Option<Vec<u8>> {
    Some(miniz_oxide::deflate::compress_to_vec_zlib(
        &reorder_and_predict(data),
        6,
    ))
}
/// compressRLE: reorderAndPredict, then runs of at least three equal bytes as
/// ( count, byte ) and literal spans as ( −count, bytes ), at most 127 each.
fn compress_rle(data: &[u8]) -> Option<Vec<u8>> {
    let t = reorder_and_predict(data);
    let end = t.len();
    let mut out = Vec::with_capacity(end * 2);
    let (mut runs, mut rune) = (0usize, 1usize);
    while runs < end {
        let mut count = 0u8;
        while rune < end && t[runs] == t[rune] && count < 127 {
            rune += 1;
            count += 1;
        }
        if count >= 2 {
            out.push(count);
            out.push(t[runs]);
            runs = rune;
        } else {
            count += 1;
            while rune < end
                && ((rune + 1 >= end || t[rune] != t[rune + 1])
                    || (rune + 2 >= end || t[rune + 1] != t[rune + 2]))
                && count < 127
            {
                count += 1;
                rune += 1;
            }
            out.push(count.wrapping_neg());
            out.extend_from_slice(&t[runs..rune.max(runs)]);
            runs = runs.max(rune);
        }
        rune += 1;
    }
    Some(out)
}

const USHORT_RANGE: usize = 1 << 16;
const BITMAP_SIZE: usize = USHORT_RANGE >> 3;
const A_OFFSET: i32 = 1 << 15;
const M_OFFSET: i32 = 1 << 15;
const MOD_MASK: i32 = (1 << 16) - 1;
const HUF_ENCSIZE: usize = (1 << 16) + 1;
const SHORT_ZEROCODE_RUN: u64 = 59;
const LONG_ZEROCODE_RUN: u64 = 63;
const SHORTEST_LONG_RUN: usize = 2 + LONG_ZEROCODE_RUN as usize - SHORT_ZEROCODE_RUN as usize;
const LONGEST_LONG_RUN: usize = 255 + SHORTEST_LONG_RUN;

/// compressPIZ: the block's channel rows as 16-bit words, mapped through the
/// used-value bitmap, 2D Haar wavelet per channel and word lane, then Huffman.
/// `None` when the Huffman stream would not be smaller ( hufCompress's null ).
fn compress_piz(data: &[u8], width: usize, data_size: usize, channels: usize) -> Option<Vec<u8>> {
    let wcount = data_size / 2;
    let nx = width;
    let bpl = nx * data_size;
    let ny = data.len() / (bpl * channels);
    let ndata = data.len() / 2;
    let mut buffer = vec![0u16; ndata];
    for y in 0..ny {
        for c in 0..channels {
            let src = (y * channels + c) * bpl;
            let dst = (c * ny + y) * bpl / 2;
            for i in 0..bpl / 2 {
                buffer[dst + i] = u16::from_le_bytes([data[src + 2 * i], data[src + 2 * i + 1]]);
            }
        }
    }
    let mut bitmap = vec![0u8; BITMAP_SIZE];
    for &v in &buffer {
        bitmap[v as usize >> 3] |= 1 << (v & 7);
    }
    bitmap[0] &= !1;
    let (mut min_non_zero, mut max_non_zero) = (BITMAP_SIZE - 1, 0);
    for (i, &b) in bitmap.iter().enumerate() {
        if b != 0 {
            min_non_zero = min_non_zero.min(i);
            max_non_zero = max_non_zero.max(i);
        }
    }
    let mut lut = vec![0u16; USHORT_RANGE];
    let mut k = 0u32;
    for (i, entry) in lut.iter_mut().enumerate() {
        if i == 0 || bitmap[i >> 3] & (1 << (i & 7)) != 0 {
            *entry = k as u16;
            k += 1;
        }
    }
    let max_value = k - 1;
    for v in buffer.iter_mut() {
        *v = lut[*v as usize];
    }
    for c in 0..channels {
        for j in 0..wcount {
            wav2_encode(
                &mut buffer,
                c * nx * ny * wcount + j,
                nx,
                wcount,
                ny,
                wcount * nx,
                max_value,
            );
        }
    }
    let bitmap_length = if min_non_zero <= max_non_zero {
        max_non_zero - min_non_zero + 1
    } else {
        0
    };
    let huf = huf_compress(&buffer, data.len())?;
    let mut out = Vec::with_capacity(8 + bitmap_length + huf.len());
    out.extend((min_non_zero as u16).to_le_bytes());
    out.extend((max_non_zero as u16).to_le_bytes());
    out.extend(&bitmap[min_non_zero..min_non_zero + bitmap_length]);
    out.extend((huf.len() as u32).to_le_bytes());
    out.extend(huf);
    Some(out)
}

fn wenc14(a: u16, b: u16) -> (u16, u16) {
    let (a, b) = (a as i16 as i32, b as i16 as i32);
    (((a + b) >> 1) as u16, (a - b) as u16)
}
fn wenc16(a: u16, b: u16) -> (u16, u16) {
    let ao = (a as i32 + A_OFFSET) & MOD_MASK;
    let mut m = (ao + b as i32) >> 1;
    let d = ao - b as i32;
    if d < 0 {
        m = (m + M_OFFSET) & MOD_MASK;
    }
    (m as u16, (d & MOD_MASK) as u16)
}
/// wav2Encode: the in-place 2D wavelet over an nx × ny lane of `buffer`.
fn wav2_encode(b: &mut [u16], j: usize, nx: usize, ox: usize, ny: usize, oy: usize, mx: u32) {
    let wenc = if mx < 1 << 14 { wenc14 } else { wenc16 };
    let n = nx.min(ny);
    let (mut p, mut p2) = (1usize, 2usize);
    while p2 <= n {
        let mut py = j;
        let ey = j + oy * (ny - p2);
        let (oy1, oy2, ox1, ox2) = (oy * p, oy * p2, ox * p, ox * p2);
        while py <= ey {
            let mut px = py;
            let ex = py + ox * (nx - p2);
            while px <= ex {
                let p01 = px + ox1;
                let p10 = px + oy1;
                let p11 = p10 + ox1;
                let (i00, i01) = wenc(b[px], b[p01]);
                let (i10, i11) = wenc(b[p10], b[p11]);
                (b[px], b[p10]) = wenc(i00, i10);
                (b[p01], b[p11]) = wenc(i01, i11);
                px += ox2;
            }
            if nx & p != 0 {
                let p10 = px + oy1;
                (b[px], b[p10]) = wenc(b[px], b[p10]);
            }
            py += oy2;
        }
        if ny & p != 0 {
            let mut px = py;
            let ex = py + ox * (nx - p2);
            while px <= ex {
                let p01 = px + ox1;
                (b[px], b[p01]) = wenc(b[px], b[p01]);
                px += ox2;
            }
        }
        p = p2;
        p2 <<= 1;
    }
}

/// The Huffman writer: whole bytes into `out` past `offset`, `lc` pending bits in `c`.
struct BitWriter {
    out: Vec<u8>,
    offset: usize,
    c: u32,
    lc: u32,
    overflow: bool,
}
impl BitWriter {
    /// outputBits: `bits`' low `n` bits, most significant first, 16 at a time.
    fn output(&mut self, mut n: u32, bits: u64) {
        while n > 0 {
            let k = n.min(16);
            n -= k;
            self.c = (self.c << k) | ((bits >> n) as u32 & ((1 << k) - 1));
            self.lc += k;
            while self.lc >= 8 {
                self.lc -= 8;
                if self.offset < self.out.len() {
                    self.out[self.offset] = (self.c >> self.lc) as u8;
                    self.offset += 1;
                } else {
                    self.overflow = true;
                }
            }
            self.c &= (1 << self.lc) - 1;
        }
    }
}
struct EncTable {
    codes: Vec<u64>,
    lengths: Vec<u8>,
    im: usize,
    big_m: usize,
}
/// hufCompress: the packed code lengths and the run-length coded symbols after
/// a 20-byte header, or `None` when they would not fit in `max_length` bytes.
fn huf_compress(raw: &[u16], max_length: usize) -> Option<Vec<u8>> {
    let mut freq = vec![0u64; HUF_ENCSIZE];
    for &v in raw {
        freq[v as usize] += 1;
    }
    let table = huf_build_enc_table(&mut freq);
    let mut writer = BitWriter {
        out: vec![0; max_length],
        offset: 20,
        c: 0,
        lc: 0,
        overflow: false,
    };
    huf_pack_enc_table(&table, &mut writer);
    let table_length = writer.offset - 20;
    let data_start = writer.offset;
    writer.c = 0;
    writer.lc = 0;
    huf_encode(&table, raw, &mut writer);
    let n_bits = (writer.offset - data_start) * 8 + writer.lc as usize;
    let length = data_start + n_bits.div_ceil(8);
    if writer.overflow || length >= max_length {
        return None;
    }
    let mut out = writer.out;
    for (i, v) in [table.im, table.big_m, table_length, n_bits, 0]
        .into_iter()
        .enumerate()
    {
        out[i * 4..i * 4 + 4].copy_from_slice(&(v as u32).to_le_bytes());
    }
    out.truncate(length);
    Some(out)
}
/// hufBuildEncTable: code lengths from a min-heap of frequencies ( ties broken
/// by symbol, as the original's STL-style heap does ), then canonical codes.
fn huf_build_enc_table(frq: &mut [u64]) -> EncTable {
    let mut hlink = vec![0u32; HUF_ENCSIZE];
    let mut heap = Vec::new();
    let mut im = 0;
    let mut big_m = 0;
    while frq[im] == 0 {
        im += 1;
    }
    for i in im..HUF_ENCSIZE {
        hlink[i] = i as u32;
        if frq[i] != 0 {
            heap.push(i);
            big_m = i;
        }
    }
    big_m += 1;
    frq[big_m] = 1;
    heap.push(big_m);
    let mut nf = heap.len();
    let greater: Greater =
        |frq: &[u64], a: usize, b: usize| frq[a] > frq[b] || (frq[a] == frq[b] && a > b);
    make_heap(&mut heap, nf, frq, greater);
    let mut lengths = vec![0u8; HUF_ENCSIZE];
    while nf > 1 {
        let mm = heap[0];
        pop_heap(&mut heap, nf, frq, greater);
        nf -= 1;
        let m = heap[0];
        pop_heap(&mut heap, nf, frq, greater);
        frq[m] += frq[mm];
        push_heap(&mut heap, nf, frq, greater);
        let mut j = m;
        loop {
            lengths[j] = lengths[j].wrapping_add(1);
            if hlink[j] as usize == j {
                hlink[j] = mm as u32;
                break;
            }
            j = hlink[j] as usize;
        }
        let mut j = mm;
        loop {
            lengths[j] = lengths[j].wrapping_add(1);
            if hlink[j] as usize == j {
                break;
            }
            j = hlink[j] as usize;
        }
    }
    let mut n = [0u64; 59];
    for &l in &lengths {
        if let Some(count) = n.get_mut(l as usize) {
            *count += 1;
        }
    }
    let mut c = 0;
    for i in (1..=58).rev() {
        let nc = (c + n[i]) / 2;
        n[i] = c;
        c = nc;
    }
    let mut codes = vec![0u64; HUF_ENCSIZE];
    for (code, &l) in codes.iter_mut().zip(&lengths) {
        if l > 0
            && let Some(next) = n.get_mut(l as usize)
        {
            *code = *next;
            *next += 1;
        }
    }
    EncTable {
        codes,
        lengths,
        im,
        big_m,
    }
}
type Greater = fn(&[u64], usize, usize) -> bool;
fn push_heap_hole(
    heap: &mut [usize],
    mut hole: usize,
    top: usize,
    value: usize,
    frq: &[u64],
    greater: Greater,
) {
    while hole > top {
        let parent = (hole - 1) / 2;
        if !greater(frq, heap[parent], value) {
            break;
        }
        heap[hole] = heap[parent];
        hole = parent;
    }
    heap[hole] = value;
}
fn adjust_heap(
    heap: &mut [usize],
    mut hole: usize,
    len: usize,
    value: usize,
    frq: &[u64],
    greater: Greater,
) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if greater(frq, heap[second], heap[second - 1]) {
            second -= 1;
        }
        heap[hole] = heap[second];
        hole = second;
    }
    if len & 1 == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        heap[hole] = heap[second - 1];
        hole = second - 1;
    }
    push_heap_hole(heap, hole, top, value, frq, greater);
}
fn make_heap(heap: &mut [usize], len: usize, frq: &[u64], greater: Greater) {
    if len < 2 {
        return;
    }
    for parent in (0..=(len - 2) / 2).rev() {
        adjust_heap(heap, parent, len, heap[parent], frq, greater);
    }
}
fn push_heap(heap: &mut [usize], len: usize, frq: &[u64], greater: Greater) {
    push_heap_hole(heap, len - 1, 0, heap[len - 1], frq, greater);
}
fn pop_heap(heap: &mut [usize], len: usize, frq: &[u64], greater: Greater) {
    if len > 1 {
        let last = len - 1;
        let value = heap[last];
        heap[last] = heap[0];
        adjust_heap(heap, 0, last, value, frq, greater);
    }
}
/// hufPackEncTable: 6-bit code lengths, zero runs as short or long run codes.
fn huf_pack_enc_table(table: &EncTable, writer: &mut BitWriter) {
    let (lengths, big_m) = (&table.lengths, table.big_m);
    let mut im = table.im;
    while im <= big_m {
        let l = lengths[im];
        if l == 0 {
            let mut zerun = 1;
            while im < big_m && zerun < LONGEST_LONG_RUN {
                if lengths[im + 1] > 0 {
                    break;
                }
                im += 1;
                zerun += 1;
            }
            if zerun >= 2 {
                if zerun >= SHORTEST_LONG_RUN {
                    writer.output(6, LONG_ZEROCODE_RUN);
                    writer.output(8, (zerun - SHORTEST_LONG_RUN) as u64);
                } else {
                    writer.output(6, SHORT_ZEROCODE_RUN + zerun as u64 - 2);
                }
                im += 1;
                continue;
            }
        }
        writer.output(6, l as u64);
        im += 1;
    }
    if writer.lc > 0 {
        writer.output(8 - writer.lc, 0);
    }
}
/// hufEncode: each symbol's code, runs of up to 256 as symbol, run code, count.
fn huf_encode(table: &EncTable, raw: &[u16], writer: &mut BitWriter) {
    let (codes, lengths, rlc) = (&table.codes, &table.lengths, table.big_m);
    let send = |writer: &mut BitWriter, s: usize, run: u32| {
        let ls = lengths[s] as u32;
        if ls + lengths[rlc] as u32 + 8 < ls * run {
            writer.output(ls, codes[s]);
            writer.output(lengths[rlc] as u32, codes[rlc]);
            writer.output(8, run as u64);
        } else {
            for _ in 0..=run {
                writer.output(ls, codes[s]);
            }
        }
    };
    let Some(&first) = raw.first() else {
        return;
    };
    let (mut s, mut cs) = (first as usize, 0u32);
    for &ns in &raw[1..] {
        if cs == 255 || s != ns as usize {
            send(writer, s, cs);
            cs = 0;
            s = ns as usize;
        } else {
            cs += 1;
        }
    }
    send(writer, s, cs);
    if writer.lc > 0 {
        if writer.offset < writer.out.len() {
            writer.out[writer.offset] = (writer.c << (8 - writer.lc)) as u8;
        } else {
            writer.overflow = true;
        }
    }
}

/// VK formats and DFD values used by KTX2Exporter for RGBA data.
pub(super) enum Ktx2Kind {
    /// FloatType, NoColorSpace (a DataTexture).
    Float,
    /// HalfFloatType, NoColorSpace (the page's WebGLRenderTarget).
    Half,
}
/// KTX2Exporter.parse with ktx-parse's write( container, { keepWriter: true } ).
pub(super) fn ktx2(kind: Ktx2Kind, data: &[u8], width: u32, height: u32) -> Vec<u8> {
    const VK_FORMAT_R16G16B16A16_SFLOAT: u32 = 97;
    const VK_FORMAT_R32G32B32A32_SFLOAT: u32 = 109;
    let (vk_format, type_size, primaries) = match kind {
        Ktx2Kind::Float => (VK_FORMAT_R32G32B32A32_SFLOAT, 4u32, 0u8),
        Ktx2Kind::Half => (VK_FORMAT_R16G16B16A16_SFLOAT, 2, 0),
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
    let (key, value) = (b"KTXwriter".as_slice(), b"three.js 187".as_slice());
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
