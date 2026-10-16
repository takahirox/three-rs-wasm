//! UltraHDRLoader ( examples/jsm/loaders/UltraHDRLoader.js ): a JPEG with a
//! gain map ( ISO 21496-1 / Adobe gain map XMP ) decoded to a half-float HDR
//! environment, as the loader decodes it at load.
use crate::{Error, Result, environment::EnvironmentMap};
use wasm_bindgen::JsCast;

/// UltraHDRLoader metadata from the gain map's XMP (`hdrgm:` attributes).
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
/// A JPEG decoded by the browser and drawn to a 2D canvas at `size`, as the
/// loader's `drawImage` / `getImageData` do (the gain map is scaled by it).
pub(super) async fn canvas_pixels(
    bytes: &[u8],
    size: Option<(u32, u32)>,
) -> Result<(u32, u32, Vec<u8>)> {
    let fail = |e: wasm_bindgen::JsValue| Error::Asset(format!("UltraHDR image: {e:?}"));
    let cast = |_| Error::Invalid("UltraHDR image type");
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(fail)?;
    let window = web_sys::window().ok_or(Error::Invalid("window"))?;
    let bitmap: web_sys::ImageBitmap = wasm_bindgen_futures::JsFuture::from(
        window.create_image_bitmap_with_blob(&blob).map_err(fail)?,
    )
    .await
    .map_err(fail)?
    .dyn_into()
    .map_err(|_| Error::Invalid("UltraHDR bitmap"))?;
    let (width, height) = size.unwrap_or((bitmap.width(), bitmap.height()));
    let document = window.document().ok_or(Error::Invalid("document"))?;
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(fail)?
        .dyn_into()
        .map_err(cast)?;
    canvas.set_width(width);
    canvas.set_height(height);
    let attributes = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&attributes, &"willReadFrequently".into(), &true.into());
    let _ = js_sys::Reflect::set(&attributes, &"colorSpace".into(), &"srgb".into());
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context_with_context_options("2d", &attributes)
        .map_err(fail)?
        .ok_or(Error::Invalid("2d context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("2d context"))?;
    context
        .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &bitmap,
            0.,
            0.,
            bitmap.width() as f64,
            bitmap.height() as f64,
            0.,
            0.,
            width as f64,
            height as f64,
        )
        .map_err(fail)?;
    let data = context
        .get_image_data(0., 0., width as f64, height as f64)
        .map_err(fail)?
        .data()
        .0;
    Ok((width, height, data))
}
/// UltraHDRLoader.parse: the MPF primary and gain-map JPEGs, the gain-map XMP,
/// and the HDR recovery formula with the loader's sRGB table, stored with
/// `DataUtils.toHalfFloat` (HalfFloatType, flipY: rows are top first).
pub(super) async fn ultra_hdr(bytes: &[u8]) -> Result<EnvironmentMap> {
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
                // MPF: primary and gain-map sizes and offsets after 60 bytes of tags.
                // Offsets are relative to the loader's DataView, two bytes into the section.
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
        if marker >= 0xc0 && marker != 0xd9 && !(0xd0..=0xd7).contains(&marker) && marker != 0xff {
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
    let (width, height, sdr) = canvas_pixels(primary, None).await?;
    let (_, _, gain) = canvas_pixels(gainmap, Some((width, height))).await?;
    let max_boost = 1.8f64.powf(meta.capacity_max * 0.5);
    let weight = ((max_boost.log2() - meta.capacity_min) / (meta.capacity_max - meta.capacity_min))
        .clamp(0., 1.);
    let srgb_to_linear = |v: f64| {
        if v < 10.31475 {
            v * 0.000303527
        } else if v < 1024. {
            (((v as i64) as f64) * 0.003717127 + 0.0521327014).powf(2.4)
        } else {
            (v * 0.003717127 + 0.0521327014).powf(2.4)
        }
    };
    let mut rgba = vec![half::f16::from_bits(15360); sdr.len()];
    for i in (0..sdr.len()).step_by(4) {
        for c in 0..3 {
            let g = gain[i + c] as f64 * 0.00392156862745098;
            let recovery = if meta.gamma == 1. {
                g
            } else {
                g.powf(1. / meta.gamma)
            };
            let boost = meta.gain_min + (meta.gain_max - meta.gain_min) * recovery;
            let factor = if boost * weight == 0. {
                1.
            } else {
                2f64.powf(boost * weight)
            };
            let hdr = (sdr[i + c] as f64 + meta.offset_sdr) * factor - meta.offset_hdr;
            let linear = srgb_to_linear(hdr).clamp(0., 65504.);
            rgba[i + c] = half::f16::from_bits(to_half_float(linear as f32));
        }
    }
    Ok(EnvironmentMap {
        width,
        height,
        rgba,
        gpu: None,
    })
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

/// An environment from a URL, as the pages load theirs: UltraHDRLoader for a
/// JPEG ( `.hdr.jpg` ), HDRLoader for Radiance RGBE.
pub(super) async fn environment(url: &str) -> Result<EnvironmentMap> {
    let bytes = super::gltf_viewer::fetch(url).await?;
    decode_environment(&bytes).await
}
pub(super) async fn decode_environment(bytes: &[u8]) -> Result<EnvironmentMap> {
    if bytes.starts_with(&[0xff, 0xd8]) {
        ultra_hdr(bytes).await
    } else {
        EnvironmentMap::from_hdr(bytes)
    }
}
