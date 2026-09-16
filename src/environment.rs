//! Linear HDR environment data. Radiance HDR keeps values above one until presentation.
use crate::{Error, Result};
#[derive(Debug)]
pub struct EnvironmentMap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<half::f16>,
}
impl EnvironmentMap {
    pub fn from_hdr(bytes: &[u8]) -> Result<Self> {
        if image::guess_format(bytes).ok() != Some(image::ImageFormat::Hdr) {
            return Err(Error::Invalid("expected Radiance HDR environment"));
        }
        let image = image::load_from_memory(bytes)
            .map_err(|e| Error::Asset(e.to_string()))?
            .to_rgba32f();
        if image.width() < 64 || image.width() > 8192 || image.height() > 8192 {
            return Err(Error::Invalid(
                "HDR dimensions (width 64..8192, height at most 8192)",
            ));
        }
        let rgba = image
            .as_raw()
            .iter()
            .map(|&v| half::f16::from_f32(v.clamp(0.0, 65504.0)))
            .collect();
        Ok(Self {
            width: image.width(),
            height: image.height(),
            rgba,
        })
    }
}
