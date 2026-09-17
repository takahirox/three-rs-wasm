//! Linear HDR environment data. Radiance HDR keeps values above one until presentation.
use crate::{Error, Result};
#[derive(Debug)]
pub struct EnvironmentMap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<half::f16>,
}
impl EnvironmentMap {
    /// Decode an LDR equirectangular texture into the linear environment format.
    pub fn from_texture(texture: &crate::material::Texture) -> Result<Self> {
        if texture.width < 64
            || texture.width > 8192
            || texture.height == 0
            || texture.height > 8192
            || texture.rgba.len() != texture.width as usize * texture.height as usize * 4
        {
            return Err(Error::Invalid("environment texture dimensions/data"));
        }
        let rgba = texture
            .rgba
            .iter()
            .enumerate()
            .map(|(i, &byte)| {
                let value = byte as f64 / 255.0;
                half::f16::from_f64(if texture.srgb && i % 4 != 3 {
                    crate::math::srgb_to_linear(value)
                } else {
                    value
                })
            })
            .collect();
        Ok(Self {
            width: texture.width,
            height: texture.height,
            rgba,
        })
    }
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
