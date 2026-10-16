//! Lut ( examples/jsm/math/Lut.js ): the color maps and their 32-step lookup
//! tables, and the color bar updateCanvas() draws.
/// ColorMapKeywords.
pub const COLOR_MAPS: [(&str, [(f64, u32); 5]); 4] = [
    (
        "rainbow",
        [
            (0.0, 0x0000ff),
            (0.2, 0x00ffff),
            (0.5, 0x00ff00),
            (0.8, 0xffff00),
            (1.0, 0xff0000),
        ],
    ),
    (
        "cooltowarm",
        [
            (0.0, 0x3c4ec2),
            (0.2, 0x9bbcff),
            (0.5, 0xdcdcdc),
            (0.8, 0xf6a385),
            (1.0, 0xb40426),
        ],
    ),
    (
        "blackbody",
        [
            (0.0, 0x000000),
            (0.2, 0x780000),
            (0.5, 0xe63200),
            (0.8, 0xffff00),
            (1.0, 0xffffff),
        ],
    ),
    (
        "grayscale",
        [
            (0.0, 0x000000),
            (0.2, 0x404040),
            (0.5, 0x7f7f80),
            (0.8, 0xbfbfbf),
            (1.0, 0xffffff),
        ],
    ),
];

/// A Lut with its colors ( in the working color space ) and step count.
#[derive(Clone, Debug)]
pub struct Lut {
    pub map: [(f64, u32); 5],
    pub lut: Vec<[f64; 3]>,
    pub n: usize,
}

/// `setHex( hex, LinearSRGBColorSpace )`.
fn linear_hex(hex: u32) -> [f64; 3] {
    [
        f64::from((hex >> 16) & 255) / 255.,
        f64::from((hex >> 8) & 255) / 255.,
        f64::from(hex & 255) / 255.,
    ]
}
/// `new Color( hex )` ( sRGB to the linear working space ).
fn srgb_hex(hex: u32) -> [f64; 3] {
    linear_hex(hex).map(crate::math::srgb_to_linear)
}
/// Color.lerpColors( a, b, t ).
fn lerp(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

impl Lut {
    /// new Lut( colormap, count ): unknown names give the rainbow.
    pub fn new(colormap: &str, count: usize) -> Self {
        let map = COLOR_MAPS
            .iter()
            .find(|(n, _)| *n == colormap)
            .unwrap_or(&COLOR_MAPS[0])
            .1;
        let step = 1.0 / count as f64;
        let mut lut = vec![srgb_hex(map[0].1)];
        for i in 1..count {
            let alpha = i as f64 * step;
            for j in 0..map.len() - 1 {
                if alpha > map[j].0 && alpha <= map[j + 1].0 {
                    let (min, max) = (map[j].0, map[j + 1].0);
                    lut.push(lerp(
                        linear_hex(map[j].1),
                        linear_hex(map[j + 1].1),
                        (alpha - min) / (max - min),
                    ));
                }
            }
        }
        lut.push(srgb_hex(map[map.len() - 1].1));
        Self { map, lut, n: count }
    }
    /// updateCanvas(): the 1 × n color bar's RGBA rows, top ( 1 ) to bottom.
    pub fn canvas(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.n * 4];
        let step = 1.0 / self.n as f64;
        let mut k = 0;
        let mut i = 1.0;
        while i >= 0.0 {
            for j in (1..self.map.len()).rev() {
                if i < self.map[j].0 && i >= self.map[j - 1].0 {
                    let (min, max) = (self.map[j - 1].0, self.map[j].0);
                    let c = lerp(
                        linear_hex(self.map[j - 1].1),
                        linear_hex(self.map[j].1),
                        (i - min) / (max - min),
                    );
                    if k * 4 + 3 < data.len() {
                        for (o, v) in c.iter().enumerate() {
                            // Math.round, then Uint8ClampedArray.
                            data[k * 4 + o] = (v * 255. + 0.5).floor().clamp(0., 255.) as u8;
                        }
                        data[k * 4 + 3] = 255;
                    }
                    k += 1;
                }
            }
            i -= step;
        }
        data
    }
}
