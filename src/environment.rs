//! Linear HDR environment data. Radiance HDR keeps values above one until presentation.
use crate::{Error, Result};
#[derive(Debug)]
pub struct EnvironmentMap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<half::f16>,
    /// A resident scene capture; when present no CPU pixel upload is needed.
    pub gpu: Option<PrefilteredEnvironment>,
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
            gpu: None,
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
            gpu: None,
        })
    }
}

/// GPU-only prefiltered scene lighting. Views keep their backing textures alive.
#[derive(Clone, Debug)]
pub struct PrefilteredEnvironment {
    pub(crate) view: wgpu::TextureView,
    pub(crate) source: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) max_mip: f32,
    pub(crate) source_is_cube_uv: bool,
}

impl EnvironmentMap {
    /// Prepare a resident GPU atlas once, also reusable by custom TSL environments.
    pub fn prefilter(
        &mut self,
        renderer: &crate::renderer::Renderer,
    ) -> Result<&PrefilteredEnvironment> {
        if self.gpu.is_none() {
            self.gpu = Some(crate::environment_gpu::build(
                &renderer.device,
                &renderer.queue,
                self,
            )?);
        }
        Ok(self.gpu.as_ref().unwrap())
    }
}
impl PrefilteredEnvironment {
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
    pub fn max_mip(&self) -> f32 {
        self.max_mip
    }
}
