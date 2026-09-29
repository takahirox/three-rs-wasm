//! PVRTC1 (2 and 4 bpp) block decoding, following Imagination's reference
//! decompressor (PVRTDecompress). WebGPU has no PVRTC formats, so the
//! WebGL PVRTC examples decode each level once at load to RGBA8 texels,
//! which are then uploaded and stay resident.

#[derive(Clone, Copy, Default)]
struct Color {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
}
impl std::ops::Sub for Color {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            r: self.r - o.r,
            g: self.g - o.g,
            b: self.b - o.b,
            a: self.a - o.a,
        }
    }
}
impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            r: self.r + o.r,
            g: self.g + o.g,
            b: self.b + o.b,
            a: self.a + o.a,
        }
    }
}
impl std::ops::Mul<i32> for Color {
    type Output = Self;
    fn mul(self, k: i32) -> Self {
        Self {
            r: self.r * k,
            g: self.g * k,
            b: self.b * k,
            a: self.a * k,
        }
    }
}
#[derive(Clone, Copy)]
struct Word {
    modulation: u32,
    color: u32,
}
/// getColorA: RGB554 (opaque) or ARGB3443, as 5-bit channels and 4-bit alpha.
fn color_a(d: u32) -> Color {
    if d & 0x8000 != 0 {
        Color {
            r: ((d & 0x7c00) >> 10) as i32,
            g: ((d & 0x3e0) >> 5) as i32,
            b: ((d & 0x1e) | ((d & 0x1e) >> 4)) as i32,
            a: 0xf,
        }
    } else {
        Color {
            r: (((d & 0xf00) >> 7) | ((d & 0xf00) >> 11)) as i32,
            g: (((d & 0xf0) >> 3) | ((d & 0xf0) >> 7)) as i32,
            b: (((d & 0xe) << 1) | ((d & 0xe) >> 2)) as i32,
            a: ((d & 0x7000) >> 11) as i32,
        }
    }
}
/// getColorB: RGB555 (opaque) or ARGB3444.
fn color_b(d: u32) -> Color {
    if d & 0x8000_0000 != 0 {
        Color {
            r: ((d & 0x7c00_0000) >> 26) as i32,
            g: ((d & 0x3e0_0000) >> 21) as i32,
            b: ((d & 0x1f_0000) >> 16) as i32,
            a: 0xf,
        }
    } else {
        Color {
            r: (((d & 0xf00_0000) >> 23) | ((d & 0xf00_0000) >> 27)) as i32,
            g: (((d & 0xf0_0000) >> 19) | ((d & 0xf0_0000) >> 23)) as i32,
            b: (((d & 0xf_0000) >> 15) | ((d & 0xf_0000) >> 19)) as i32,
            a: ((d & 0x7000_0000) >> 27) as i32,
        }
    }
}
/// interpolateColors: the bilinear upscale of the four words' colors over
/// one word-sized neighbourhood, expanded to 8 bits per channel.
fn interpolate(p: Color, q: Color, r: Color, s: Color, bpp: u32) -> [Color; 32] {
    let (ww, wh) = if bpp == 2 { (8, 4) } else { (4, 4) };
    let mut out = [Color::default(); 32];
    let q_p = q - p;
    let s_r = s - r;
    let mut hp = p * ww;
    let mut hr = r * ww;
    if bpp == 2 {
        for x in 0..ww {
            let mut result = hp * 4;
            let dy = hr - hp;
            for y in 0..wh {
                out[(y * ww + x) as usize] = Color {
                    r: (result.r >> 7) + (result.r >> 2),
                    g: (result.g >> 7) + (result.g >> 2),
                    b: (result.b >> 7) + (result.b >> 2),
                    a: (result.a >> 5) + (result.a >> 1),
                };
                result = result + dy;
            }
            hp = hp + q_p;
            hr = hr + s_r;
        }
    } else {
        for y in 0..wh {
            let mut result = hp * 4;
            let dy = hr - hp;
            for x in 0..ww {
                out[(y * ww + x) as usize] = Color {
                    r: (result.r >> 6) + (result.r >> 1),
                    g: (result.g >> 6) + (result.g >> 1),
                    b: (result.b >> 6) + (result.b >> 1),
                    a: (result.a >> 4) + result.a,
                };
                result = result + dy;
            }
            hp = hp + q_p;
            hr = hr + s_r;
        }
    }
    out
}
type Grid = [[i32; 8]; 16];
/// unpackModulations for one word placed at ( ox, oy ) of the 2 × 2 words.
fn unpack(word: Word, ox: usize, oy: usize, values: &mut Grid, modes: &mut Grid, bpp: u32) {
    let mut mode = word.color & 1;
    let mut bits = word.modulation;
    if bpp == 2 {
        if mode != 0 {
            if bits & 1 != 0 {
                mode = if bits & (1 << 20) != 0 { 3 } else { 2 };
                if bits & (1 << 21) != 0 {
                    bits |= 1 << 20;
                } else {
                    bits &= !(1 << 20);
                }
            }
            if bits & 2 != 0 {
                bits |= 1;
            } else {
                bits &= !1;
            }
            for y in 0..4 {
                for x in 0..8 {
                    modes[x + ox][y + oy] = mode as i32;
                    if (x ^ y) & 1 == 0 {
                        values[x + ox][y + oy] = (bits & 3) as i32;
                        bits >>= 2;
                    }
                }
            }
        } else {
            for y in 0..4 {
                for x in 0..8 {
                    modes[x + ox][y + oy] = mode as i32;
                    values[x + ox][y + oy] = if bits & 1 != 0 { 3 } else { 0 };
                    bits >>= 1;
                }
            }
        }
    } else if mode != 0 {
        for y in 0..4 {
            for x in 0..4 {
                values[y + oy][x + ox] = match bits & 3 {
                    1 => 4,
                    // +10: punch-through alpha.
                    2 => 14,
                    3 => 8,
                    _ => 0,
                };
                bits >>= 2;
            }
        }
    } else {
        for y in 0..4 {
            for x in 0..4 {
                let mut v = (bits & 3) as i32 * 3;
                if v > 3 {
                    v -= 1;
                }
                values[y + oy][x + ox] = v;
                bits >>= 2;
            }
        }
    }
}
/// getModulationValues.
fn modulation(values: &Grid, modes: &Grid, x: usize, y: usize, bpp: u32) -> i32 {
    if bpp != 2 {
        return values[x][y];
    }
    const REP: [i32; 4] = [0, 3, 5, 8];
    let v = |x: usize, y: usize| REP[values[x][y] as usize];
    if modes[x][y] == 0 || (x ^ y) & 1 == 0 {
        v(x, y)
    } else if modes[x][y] == 1 {
        (v(x, y - 1) + v(x, y + 1) + v(x - 1, y) + v(x + 1, y) + 2) / 4
    } else if modes[x][y] == 2 {
        (v(x - 1, y) + v(x + 1, y) + 1) / 2
    } else {
        (v(x, y - 1) + v(x, y + 1) + 1) / 2
    }
}
/// TwiddleUV: the Morton-ordered word index.
fn twiddle(xs: u32, ys: u32, x: u32, y: u32) -> u32 {
    let (min, mut max) = if ys < xs { (ys, x) } else { (xs, y) };
    let (mut twiddled, mut src, mut dst, mut shift) = (0, 1, 1, 0);
    while src < min {
        if y & src != 0 {
            twiddled |= dst;
        }
        if x & src != 0 {
            twiddled |= dst << 1;
        }
        src <<= 1;
        dst <<= 2;
        shift += 1;
    }
    max >>= shift;
    twiddled | (max << (2 * shift))
}
/// Decode one PVRTC1 level of `width` × `height` texels to RGBA8 rows.
/// Levels smaller than the 2 × 2 words minimum are decoded at that size and
/// cropped, as the reference decompressor does.
pub(super) fn decode(data: &[u8], width: u32, height: u32, bpp: u32) -> Vec<u8> {
    let (ww, wh) = if bpp == 2 { (8u32, 4u32) } else { (4, 4) };
    let (pw, ph) = (width.max(ww * 2), height.max(wh * 2));
    let (nx, ny) = ((pw / ww) as i32, (ph / wh) as i32);
    let word = |i: u32| -> Word {
        let at = |k: u32| {
            let o = (k * 4) as usize;
            data.get(o..o + 4)
                .map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        Word {
            modulation: at(i * 2),
            color: at(i * 2 + 1),
        }
    };
    let wrap = |n: i32, v: i32| ((v + n) % n) as u32;
    let mut out = vec![0u8; (pw * ph * 4) as usize];
    for wy in -1..ny - 1 {
        for wx in -1..nx - 1 {
            let p = (wrap(nx, wx), wrap(ny, wy));
            let q = (wrap(nx, wx + 1), wrap(ny, wy));
            let r = (wrap(nx, wx), wrap(ny, wy + 1));
            let s = (wrap(nx, wx + 1), wrap(ny, wy + 1));
            let words = [p, q, r, s].map(|(x, y)| word(twiddle(nx as u32, ny as u32, x, y)));
            let mut values = [[0; 8]; 16];
            let mut modes = [[0; 8]; 16];
            let (w, h) = (ww as usize, wh as usize);
            unpack(words[0], 0, 0, &mut values, &mut modes, bpp);
            unpack(words[1], w, 0, &mut values, &mut modes, bpp);
            unpack(words[2], 0, h, &mut values, &mut modes, bpp);
            unpack(words[3], w, h, &mut values, &mut modes, bpp);
            let a = interpolate(
                color_a(words[0].color),
                color_a(words[1].color),
                color_a(words[2].color),
                color_a(words[3].color),
                bpp,
            );
            let b = interpolate(
                color_b(words[0].color),
                color_b(words[1].color),
                color_b(words[2].color),
                color_b(words[3].color),
                bpp,
            );
            let mut pixels = [[0u8; 4]; 32];
            for y in 0..h {
                for x in 0..w {
                    let mut m = modulation(&values, &modes, x + w / 2, y + h / 2, bpp);
                    let punch = m > 10;
                    if punch {
                        m -= 10;
                    }
                    let (ca, cb) = (a[y * w + x], b[y * w + x]);
                    let mix = |a: i32, b: i32| (a * (8 - m) + b * m) / 8;
                    let alpha = if punch { 0 } else { mix(ca.a, cb.a) };
                    let texel =
                        [mix(ca.r, cb.r), mix(ca.g, cb.g), mix(ca.b, cb.b), alpha].map(|v| v as u8);
                    if bpp == 2 {
                        pixels[y * w + x] = texel;
                    } else {
                        pixels[y + x * h] = texel;
                    }
                }
            }
            // mapDecompressedData: each word's quadrant of the neighbourhood.
            let mut put = |row: u32, column: u32, texel: [u8; 4]| {
                let o = ((row * pw + column) * 4) as usize;
                out[o..o + 4].copy_from_slice(&texel);
            };
            for y in 0..wh / 2 {
                for x in 0..ww / 2 {
                    let (hw, hh) = (ww / 2, wh / 2);
                    let at = |yy: u32, xx: u32| pixels[(yy * ww + xx) as usize];
                    put(p.1 * wh + y + hh, p.0 * ww + x + hw, at(y, x));
                    put(q.1 * wh + y + hh, q.0 * ww + x, at(y, x + hw));
                    put(r.1 * wh + y, r.0 * ww + x + hw, at(y + hh, x));
                    put(s.1 * wh + y, s.0 * ww + x, at(y + hh, x + hw));
                }
            }
        }
    }
    if (pw, ph) == (width, height) {
        return out;
    }
    (0..height)
        .flat_map(|y| {
            let o = (y * pw * 4) as usize;
            out[o..o + (width * 4) as usize].to_vec()
        })
        .collect()
}
