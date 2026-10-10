//! UltraHDRLoader.parse for the page's royal_esplanade_2k.hdr.jpg: the MPF
//! primary and gain-map JPEGs and the gain-map XMP, both images decoded and
//! drawn by the browser's 2D canvas as the loader does, then the HDR recovery
//! with the host's Math.pow / Math.log2 and DataUtils.toHalfFloat.
use super::textures::to_half;
use crate::{Error, Result};

fn pow(a: f64, b: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::pow(a, b);
    #[cfg(not(target_arch = "wasm32"))]
    a.powf(b)
}
/// Math.max / Math.min: NaN wins.
fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}
fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}
fn log2(a: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::log2(a);
    #[cfg(not(target_arch = "wasm32"))]
    a.log2()
}

#[derive(Default)]
struct GainMap {
    version: bool,
    gain_min: f64,
    gain_max: f64,
    gamma: f64,
    offset_sdr: f64,
    offset_hdr: f64,
    capacity_min: f64,
    capacity_max: f64,
}

/// The half-float RGBA image ( width, height, rows top first ).
pub(crate) async fn ultra_hdr(bytes: &[u8]) -> Result<(usize, usize, Vec<u16>)> {
    let mut meta = GainMap::default();
    let (mut primary, mut gainmap) = (None, None);
    let mut offset = 0;
    let bad = || Error::Asset("UltraHDR: truncated section".into());
    while offset + 1 < bytes.len() {
        if bytes[offset] != 0xff {
            offset += 1;
            continue;
        }
        let marker = bytes[offset + 1];
        if marker == 0xd8 {
            offset += 2;
            continue;
        }
        let length = || -> Result<usize> {
            Ok(((*bytes.get(offset + 2).ok_or_else(bad)? as usize) << 8)
                | *bytes.get(offset + 3).ok_or_else(bad)? as usize)
        };
        if matches!(marker, 0xe0..=0xe2) {
            let end = (offset + 2 + length()?).min(bytes.len());
            let section = &bytes[offset..end];
            let section_offset = offset + 2;
            if marker == 0xe1 {
                let text = String::from_utf8_lossy(section);
                if !text.contains("Container:Directory") && text.contains("hdrgm:Version") {
                    // getAttribute on the first rdf:Description, and parseFloat.
                    let attribute = |name: &str| -> Option<f64> {
                        let key = format!("hdrgm:{name}=\"");
                        let start = text.find(&key)? + key.len();
                        let end = start + text[start..].find('"')?;
                        text[start..end].parse().ok()
                    };
                    meta.version = true;
                    meta.gain_min = attribute("GainMapMin").unwrap_or(0.);
                    meta.gain_max = attribute("GainMapMax").unwrap_or(1.);
                    meta.gamma = attribute("Gamma").unwrap_or(1.);
                    meta.offset_sdr = attribute("OffsetSDR").unwrap_or(f64::NAN) / (1. / 64.);
                    meta.offset_hdr = attribute("OffsetHDR").unwrap_or(f64::NAN) / (1. / 64.);
                    meta.capacity_min = attribute("HDRCapacityMin").unwrap_or(0.);
                    meta.capacity_max = attribute("HDRCapacityMax").unwrap_or(1.);
                }
            } else if marker == 0xe2 && section.len() >= 8 && section[4..8] == [0x4d, 0x50, 0x46, 0]
            {
                // MPF: sizes and offsets after 60 bytes of tags, relative to the
                // loader's DataView two bytes into the section.
                let data = &section[2..];
                let little = data.get(6..10) == Some(&[0x49, 0x49, 0x2a, 0][..]);
                let u32_at = |i: usize| -> Result<usize> {
                    let b: [u8; 4] = data
                        .get(i..i + 4)
                        .ok_or_else(bad)?
                        .try_into()
                        .map_err(|_| bad())?;
                    Ok(if little {
                        u32::from_le_bytes(b)
                    } else {
                        u32::from_be_bytes(b)
                    } as usize)
                };
                let (primary_size, primary_offset) = (u32_at(60)?, u32_at(64)?);
                let (gain_size, gain_offset) = (u32_at(76)?, u32_at(80)? + section_offset + 6);
                primary = bytes.get(primary_offset..primary_offset + primary_size);
                gainmap = bytes.get(gain_offset..(gain_offset + gain_size).min(bytes.len()));
            }
            offset = end;
            continue;
        }
        if (0xc0..=0xfe).contains(&marker) && marker != 0xd9 && !(0xd0..=0xd7).contains(&marker) {
            offset += 2 + length()?;
            continue;
        }
        offset += 2;
    }
    if !meta.version {
        return Err(Error::Asset("UltraHDR: not a valid UltraHDR image".into()));
    }
    let (primary, gainmap) = primary
        .zip(gainmap)
        .ok_or_else(|| Error::Asset("UltraHDR: could not parse images".into()))?;
    let (width, height, sdr) = super::super::probes_hdr::canvas_pixels(primary, None).await?;
    let (_, _, gain) =
        super::super::probes_hdr::canvas_pixels(gainmap, Some((width, height))).await?;
    // SRGB_TO_LINEAR, computed at module load.
    let table: Vec<f64> = (0..1024)
        .map(|i| pow(i as f64 * 0.003717127 + 0.0521327014, 2.4))
        .collect();
    let srgb_to_linear = |v: f64| {
        if v < 10.31475 {
            v * 0.000303527
        } else if v < 1024. {
            table[(v as i64) as usize]
        } else {
            pow(v * 0.003717127 + 0.0521327014, 2.4)
        }
    };
    let max_boost = pow(1.8, meta.capacity_max * 0.5);
    let weight = js_min(
        js_max(
            (log2(max_boost) - meta.capacity_min) / (meta.capacity_max - meta.capacity_min),
            0.,
        ),
        1.,
    );
    let inv_gamma = 1.0 / meta.gamma;
    let mut out = vec![15360u16; sdr.len()];
    for i in (0..sdr.len()).step_by(4) {
        for c in 0..3 {
            let g = gain[i + c] as f64 * 0.00392156862745098;
            let recovery = if meta.gamma == 1. {
                g
            } else {
                pow(g, inv_gamma)
            };
            let boost = meta.gain_min + (meta.gain_max - meta.gain_min) * recovery;
            let factor = if boost * weight == 0. {
                1.
            } else {
                pow(2., boost * weight)
            };
            let hdr = (sdr[i + c] as f64 + meta.offset_sdr) * factor - meta.offset_hdr;
            let linear = js_min(js_max(srgb_to_linear(hdr), 0.), 65504.);
            out[i + c] = to_half(linear);
        }
    }
    Ok((width as usize, height as usize, out))
}
