use crate::{Error, Result, math::*};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum Side {
    #[default]
    Front,
    Back,
    Double,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum Wrapping {
    #[default]
    Clamp,
    Repeat,
    Mirror,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum Filter {
    Nearest,
    #[default]
    Linear,
}
#[derive(Clone, Debug, Serialize)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub srgb: bool,
    pub wrap_s: Wrapping,
    pub wrap_t: Wrapping,
    pub filter: Filter,
    pub flip_y: bool,
    pub offset: Vector2,
    pub repeat: Vector2,
    pub rotation: f64,
    pub center: Vector2,
}
impl Texture {
    pub fn from_rgba(width: u32, height: u32, rgba: Vec<u8>, srgb: bool) -> Result<Self> {
        let size = (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(4));
        if width == 0 || height == 0 || size != Some(rgba.len()) {
            return Err(Error::Invalid("texture dimensions"));
        }
        Ok(Self {
            width,
            height,
            rgba,
            srgb,
            wrap_s: Wrapping::Clamp,
            wrap_t: Wrapping::Clamp,
            filter: Filter::Linear,
            flip_y: true,
            offset: Vector2::ZERO,
            repeat: Vector2::ONE,
            rotation: 0.0,
            center: Vector2::ZERO,
        })
    }
    pub fn from_image(bytes: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(bytes)
            .map_err(|e| Error::Asset(e.to_string()))?
            .to_rgba8();
        Self::from_rgba(image.width(), image.height(), image.into_raw(), true)
    }
    #[cfg(target_arch = "wasm32")]
    pub async fn load(url: &str) -> Result<Self> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        let window = web_sys::window().ok_or(Error::Asset("window unavailable".into()))?;
        let response = JsFuture::from(window.fetch_with_str(url))
            .await
            .map_err(|e| Error::Asset(format!("{e:?}")))?;
        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|_| Error::Asset("invalid response".into()))?;
        if !response.ok() {
            return Err(Error::Asset(format!("HTTP {}", response.status())));
        }
        let buffer = JsFuture::from(
            response
                .array_buffer()
                .map_err(|e| Error::Asset(format!("{e:?}")))?,
        )
        .await
        .map_err(|e| Error::Asset(format!("{e:?}")))?;
        Self::from_image(&js_sys::Uint8Array::new(&buffer).to_vec())
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct MaterialProperties {
    pub color: Color,
    pub opacity: f64,
    pub transparent: bool,
    pub side: Side,
    pub depth_test: bool,
    pub depth_write: bool,
    pub visible: bool,
    pub vertex_colors: bool,
    pub map: Option<Arc<Texture>>,
}
impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            opacity: 1.0,
            transparent: false,
            side: Side::Front,
            depth_test: true,
            depth_write: true,
            visible: true,
            vertex_colors: false,
            map: None,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct MeshBasicMaterial {
    pub properties: MaterialProperties,
}
#[derive(Clone, Debug, Serialize)]
pub struct MeshStandardMaterial {
    pub properties: MaterialProperties,
    pub roughness: f64,
    pub metalness: f64,
    pub emissive: Color,
}
impl Default for MeshStandardMaterial {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            roughness: 1.0,
            metalness: 0.0,
            emissive: Color::BLACK,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct LineBasicMaterial {
    pub properties: MaterialProperties,
    pub linewidth: f64,
}
impl Default for LineBasicMaterial {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            linewidth: 1.0,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct PointsMaterial {
    pub properties: MaterialProperties,
    pub size: f64,
    pub size_attenuation: bool,
}
impl Default for PointsMaterial {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            size: 1.0,
            size_attenuation: true,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub enum Material {
    Basic(MeshBasicMaterial),
    Standard(MeshStandardMaterial),
    Line(LineBasicMaterial),
    Points(PointsMaterial),
}
impl Default for Material {
    fn default() -> Self {
        Self::Basic(MeshBasicMaterial::default())
    }
}
impl Material {
    pub fn properties(&self) -> &MaterialProperties {
        match self {
            Self::Basic(m) => &m.properties,
            Self::Standard(m) => &m.properties,
            Self::Line(m) => &m.properties,
            Self::Points(m) => &m.properties,
        }
    }
    pub fn properties_mut(&mut self) -> &mut MaterialProperties {
        match self {
            Self::Basic(m) => &mut m.properties,
            Self::Standard(m) => &mut m.properties,
            Self::Line(m) => &mut m.properties,
            Self::Points(m) => &mut m.properties,
        }
    }
}
