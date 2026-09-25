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
    /// Basis payload retained for device-specific block transcoding at GPU upload.
    #[serde(skip)]
    pub basis: Option<Arc<Vec<u8>>>,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    pub(crate) bitmap: Option<Arc<BrowserBitmap>>,
    pub srgb: bool,
    pub wrap_s: Wrapping,
    pub wrap_t: Wrapping,
    pub filter: Filter,
    pub min_filter: Option<Filter>,
    pub mipmap_filter: Option<Filter>,
    /// Anisotropic sampling, clamped to 1..=16 when all filters are linear.
    pub anisotropy: u16,
    pub flip_y: bool,
    pub offset: Vector2,
    pub repeat: Vector2,
    pub rotation: f64,
    pub center: Vector2,
    pub tex_coord: u32,
    /// Explicit UV matrix, used by formats whose transform order differs.
    pub matrix: Option<Matrix3>,
}
#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) struct BrowserBitmap(pub web_sys::ImageBitmap);
#[cfg(target_arch = "wasm32")]
impl Drop for BrowserBitmap {
    fn drop(&mut self) {
        self.0.close();
    }
}
impl Texture {
    /// Retain native decoded pixels for a direct WebGPU upload. Canvas readback
    /// destroys RGB beneath zero alpha, which changes filtered AVIF textures.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_bitmap(bitmap: web_sys::ImageBitmap) -> Result<Self> {
        let mut texture = Self::from_rgba(1, 1, vec![0; 4], true)?;
        texture.width = bitmap.width();
        texture.height = bitmap.height();
        texture.rgba.clear();
        texture.bitmap = Some(Arc::new(BrowserBitmap(bitmap)));
        Ok(texture)
    }

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
            basis: None,
            #[cfg(target_arch = "wasm32")]
            bitmap: None,
            srgb,
            wrap_s: Wrapping::Clamp,
            wrap_t: Wrapping::Clamp,
            filter: Filter::Linear,
            min_filter: None,
            mipmap_filter: None,
            anisotropy: 1,
            flip_y: true,
            offset: Vector2::ZERO,
            repeat: Vector2::ONE,
            rotation: 0.0,
            center: Vector2::ZERO,
            tex_coord: 0,
            matrix: None,
        })
    }
    pub fn from_basis_compressed(bytes: Vec<u8>, srgb: bool) -> Result<Self> {
        let transcoder =
            basisu::Transcoder::new(&bytes).map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
        let (width, height) = transcoder.base_dimensions();
        if width == 0
            || height == 0
            || transcoder.face_count() != 1
            || transcoder.layer_count() > 1
            || transcoder.is_video()
            || transcoder.level_count() == 0
            || transcoder.level_count() > width.max(height).ilog2() + 1
        {
            return Err(Error::Invalid("compressed 2D texture dimensions/levels"));
        }
        let mut texture = Self::from_rgba(1, 1, vec![0; 4], srgb)?;
        texture.width = width;
        texture.height = height;
        texture.rgba.clear();
        texture.basis = Some(Arc::new(bytes));
        texture.mipmap_filter = Some(Filter::Linear);
        Ok(texture)
    }
    pub fn from_image(bytes: &[u8]) -> Result<Self> {
        if bytes.starts_with(b"\xabKTX 20\xbb\r\n\x1a\n") || bytes.starts_with(b"sB") {
            return crate::compression::decode_basis(bytes, true);
        }
        let image = image::load_from_memory(bytes)
            .map_err(|e| Error::Asset(e.to_string()))?
            .to_rgba8();
        Self::from_rgba(image.width(), image.height(), image.into_raw(), true)
    }
    pub fn uv_matrix(&self) -> Matrix3 {
        let matrix = self.matrix.unwrap_or_else(|| {
            let (s, c) = self.rotation.sin_cos();
            let tx = self.center.x + self.offset.x
                - self.repeat.x * (c * self.center.x + s * self.center.y);
            let ty = self.center.y + self.offset.y
                - self.repeat.y * (-s * self.center.x + c * self.center.y);
            Matrix3::from_cols_array(&[
                c * self.repeat.x,
                -s * self.repeat.y,
                0.0,
                s * self.repeat.x,
                c * self.repeat.y,
                0.0,
                tx,
                ty,
                1.0,
            ])
        });
        if self.flip_y {
            Matrix3::from_cols_array(&[1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.0]) * matrix
        } else {
            matrix
        }
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
    pub alpha_test: f64,
    pub alpha_to_coverage: bool,
    pub transparent: bool,
    /// Draw transparent double-sided surfaces once (ShaderMaterial defaults to true).
    pub force_single_pass: bool,
    pub side: Side,
    /// Override shadow caster faces; None uses the opposite of the visible side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_side: Option<Side>,
    pub depth_test: bool,
    pub depth_write: bool,
    pub visible: bool,
    pub vertex_colors: bool,
    pub flat_shading: bool,
    pub wireframe: bool,
    /// Material.toneMapped: false skips the renderer's tone mapping for this material.
    pub tone_mapped: bool,
    /// Material.polygonOffset: ( factor, units ), a depth bias of units × the
    /// depth resolution plus factor × the depth slope.
    pub polygon_offset: Option<(f32, i32)>,
    pub fog: bool,
    /// Optional scene-light selection for this material. None uses all visible lights.
    #[serde(skip)]
    pub lights: Option<Vec<crate::scene::Object3D>>,
    pub clipping_planes: Vec<Plane>,
    pub clip_intersection: bool,
    pub clip_shadows: bool,
    /// None selects opaque replacement or normal alpha blending according to transparent.
    #[serde(skip)]
    pub blending: Option<wgpu::BlendState>,
    /// Per-MRT blend overrides; empty uses `blending` for every attachment.
    #[serde(skip)]
    pub attachment_blending: Vec<Option<wgpu::BlendState>>,
    #[serde(skip)]
    pub stencil: Option<wgpu::StencilState>,
    pub stencil_reference: u32,
    pub color_write: bool,
    pub map: Option<Arc<Texture>>,
    #[serde(skip)]
    /// Optional vertex/output hooks; output-only programs retain standard lighting.
    pub vertex_program: Option<Arc<crate::shader::ShaderProgram>>,
    /// GPU displacement/mask for shadow depth passes; shares vertex_uniforms.
    #[serde(skip)]
    pub shadow_program: Option<Arc<crate::shadow::ShadowProgram>>,
    pub vertex_uniforms: [[f32; 4]; 16],
}
impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            opacity: 1.0,
            alpha_test: 0.0,
            alpha_to_coverage: false,
            transparent: false,
            force_single_pass: false,
            side: Side::Front,
            shadow_side: None,
            depth_test: true,
            depth_write: true,
            visible: true,
            vertex_colors: false,
            flat_shading: false,
            wireframe: false,
            tone_mapped: true,
            polygon_offset: None,
            fog: true,
            lights: None,
            clipping_planes: Vec::new(),
            clip_intersection: false,
            clip_shadows: false,
            blending: None,
            attachment_blending: Vec::new(),
            stencil: None,
            stencil_reference: 0,
            color_write: true,
            map: None,
            vertex_program: None,
            shadow_program: None,
            vertex_uniforms: [[0.0; 4]; 16],
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
    /// r186 WebGPU punctual/ambient lighting: Fresnel diffuse attenuation and
    /// multiple-scattering specular compensation. False retains legacy lighting.
    pub energy_conservation: bool,
    pub roughness: f64,
    pub metalness: f64,
    pub emissive: Color,
    pub metallic_roughness_map: Option<Arc<Texture>>,
    pub normal_map: Option<Arc<Texture>>,
    pub normal_scale: Vector2,
    pub occlusion_map: Option<Arc<Texture>>,
    pub occlusion_strength: f64,
    pub emissive_map: Option<Arc<Texture>>,
}
impl Default for MeshStandardMaterial {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            energy_conservation: false,
            roughness: 1.0,
            metalness: 0.0,
            emissive: Color::BLACK,
            metallic_roughness_map: None,
            normal_map: None,
            normal_scale: Vector2::ONE,
            occlusion_map: None,
            occlusion_strength: 1.0,
            emissive_map: None,
        }
    }
}
/// Classic normalized Blinn-Phong lighting. Specular colors are linear.
#[derive(Clone, Debug, Serialize)]
pub struct MeshPhongMaterial {
    pub properties: MaterialProperties,
    pub emissive: Color,
    pub specular: Color,
    pub shininess: f64,
    pub normal_map: Option<Arc<Texture>>,
    pub normal_scale: Vector2,
    pub specular_map: Option<Arc<Texture>>,
    pub emissive_map: Option<Arc<Texture>>,
}
impl Default for MeshPhongMaterial {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            emissive: Color::BLACK,
            specular: Color::from_hex(0x111111),
            shininess: 30.0,
            normal_map: None,
            normal_scale: Vector2::ONE,
            specular_map: None,
            emissive_map: None,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct MeshLambertMaterial {
    pub properties: MaterialProperties,
    pub emissive: Color,
    pub normal_map: Option<Arc<Texture>>,
    pub normal_scale: Vector2,
    pub emissive_map: Option<Arc<Texture>>,
}
impl Default for MeshLambertMaterial {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            emissive: Color::BLACK,
            normal_map: None,
            normal_scale: Vector2::ONE,
            emissive_map: None,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct MeshNormalMaterial {
    pub properties: MaterialProperties,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct MeshToonMaterial {
    pub base: MeshLambertMaterial,
    pub gradient_map: Option<Arc<Texture>>,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct MeshMatcapMaterial {
    pub base: MeshLambertMaterial,
    pub matcap: Option<Arc<Texture>>,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct MeshDepthMaterial {
    pub properties: MaterialProperties,
}
/// Layered PBR with independent base and extension texture maps.
#[derive(Clone, Debug, Serialize)]
pub struct MeshPhysicalMaterial {
    /// Blend the direct specular lobe toward the light source (r186 retroreflection).
    pub retroreflectivity: f64,
    pub base: MeshStandardMaterial,
    pub ior: f64,
    pub specular_color: Color,
    pub specular_intensity: f64,
    pub clearcoat: f64,
    pub clearcoat_roughness: f64,
    pub sheen_color: Color,
    pub sheen: f64,
    pub sheen_roughness: f64,
    pub anisotropy: f64,
    pub anisotropy_rotation: f64,
    pub transmission: f64,
    pub thickness: f64,
    pub attenuation_color: Color,
    pub attenuation_distance: f64,
    pub dispersion: f64,
    pub iridescence: f64,
    pub iridescence_ior: f64,
    pub iridescence_thickness_range: [f64; 2],
    pub iridescence_map: Option<Arc<Texture>>,
    pub iridescence_thickness_map: Option<Arc<Texture>>,
    pub clearcoat_normal_scale: Vector2,
    pub clearcoat_map: Option<Arc<Texture>>,
    pub clearcoat_roughness_map: Option<Arc<Texture>>,
    pub clearcoat_normal_map: Option<Arc<Texture>>,
    pub sheen_color_map: Option<Arc<Texture>>,
    pub sheen_roughness_map: Option<Arc<Texture>>,
    pub anisotropy_map: Option<Arc<Texture>>,
    pub specular_intensity_map: Option<Arc<Texture>>,
    pub specular_color_map: Option<Arc<Texture>>,
    pub transmission_map: Option<Arc<Texture>>,
    pub thickness_map: Option<Arc<Texture>>,
}
impl Default for MeshPhysicalMaterial {
    fn default() -> Self {
        Self {
            retroreflectivity: 0.0,
            base: Default::default(),
            ior: 1.5,
            specular_color: Color::WHITE,
            specular_intensity: 1.0,
            clearcoat: 0.0,
            clearcoat_roughness: 0.0,
            sheen_color: Color::BLACK,
            sheen: 0.0,
            sheen_roughness: 1.0,
            anisotropy: 0.0,
            anisotropy_rotation: 0.0,
            transmission: 0.0,
            thickness: 0.0,
            attenuation_color: Color::WHITE,
            attenuation_distance: f64::INFINITY,
            dispersion: 0.0,
            iridescence: 0.0,
            iridescence_ior: 1.3,
            iridescence_thickness_range: [100.0, 400.0],
            iridescence_map: None,
            iridescence_thickness_map: None,
            clearcoat_normal_scale: Vector2::ONE,
            clearcoat_map: None,
            clearcoat_roughness_map: None,
            clearcoat_normal_map: None,
            sheen_color_map: None,
            sheen_roughness_map: None,
            anisotropy_map: None,
            specular_intensity_map: None,
            specular_color_map: None,
            transmission_map: None,
            thickness_map: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize)]
pub struct LineDash {
    pub size: f64,
    pub gap: f64,
    pub scale: f64,
    pub offset: f64,
}
impl Default for LineDash {
    fn default() -> Self {
        Self {
            size: 3.0,
            gap: 1.0,
            scale: 1.0,
            offset: 0.0,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct LineBasicMaterial {
    pub properties: MaterialProperties,
    pub linewidth: f64,
    pub dash: Option<LineDash>,
}
impl Default for LineBasicMaterial {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            linewidth: 1.0,
            dash: None,
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
    Shader(ShaderMaterial),
    Basic(MeshBasicMaterial),
    Standard(MeshStandardMaterial),
    Physical(MeshPhysicalMaterial),
    Phong(MeshPhongMaterial),
    Lambert(MeshLambertMaterial),
    Normal(MeshNormalMaterial),
    Toon(MeshToonMaterial),
    Matcap(MeshMatcapMaterial),
    Depth(MeshDepthMaterial),
    Line(LineBasicMaterial),
    Points(PointsMaterial),
}
impl Default for Material {
    fn default() -> Self {
        Self::Basic(MeshBasicMaterial::default())
    }
}
impl Material {
    pub(crate) fn texture_maps(&self) -> [Option<&Arc<Texture>>; 5] {
        let mut maps = [self.properties().map.as_ref(), None, None, None, None];
        match self {
            Self::Standard(m) | Self::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                maps[1] = m.metallic_roughness_map.as_ref();
                maps[2] = m.normal_map.as_ref();
                maps[3] = m.occlusion_map.as_ref();
                maps[4] = m.emissive_map.as_ref();
            }
            Self::Phong(m) => {
                maps[1] = m.specular_map.as_ref();
                maps[2] = m.normal_map.as_ref();
                maps[4] = m.emissive_map.as_ref();
            }
            Self::Toon(m) => {
                maps[1] = m.gradient_map.as_ref();
                maps[2] = m.base.normal_map.as_ref();
                maps[4] = m.base.emissive_map.as_ref();
            }
            Self::Matcap(m) => {
                maps[1] = m.matcap.as_ref();
                maps[2] = m.base.normal_map.as_ref();
            }
            Self::Lambert(m) => {
                maps[2] = m.normal_map.as_ref();
                maps[4] = m.emissive_map.as_ref();
            }
            _ => {}
        }
        maps
    }
    pub fn properties(&self) -> &MaterialProperties {
        match self {
            Self::Shader(m) => &m.properties,
            Self::Basic(m) => &m.properties,
            Self::Standard(m) => &m.properties,
            Self::Physical(m) => &m.base.properties,
            Self::Phong(m) => &m.properties,
            Self::Lambert(m) => &m.properties,
            Self::Normal(m) => &m.properties,
            Self::Toon(m) => &m.base.properties,
            Self::Matcap(m) => &m.base.properties,
            Self::Depth(m) => &m.properties,
            Self::Line(m) => &m.properties,
            Self::Points(m) => &m.properties,
        }
    }
    pub fn properties_mut(&mut self) -> &mut MaterialProperties {
        match self {
            Self::Shader(m) => &mut m.properties,
            Self::Basic(m) => &mut m.properties,
            Self::Standard(m) => &mut m.properties,
            Self::Physical(m) => &mut m.base.properties,
            Self::Phong(m) => &mut m.properties,
            Self::Lambert(m) => &mut m.properties,
            Self::Normal(m) => &mut m.properties,
            Self::Toon(m) => &mut m.base.properties,
            Self::Matcap(m) => &mut m.base.properties,
            Self::Depth(m) => &mut m.properties,
            Self::Line(m) => &mut m.properties,
            Self::Points(m) => &mut m.properties,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ShaderMaterial {
    pub properties: MaterialProperties,
    #[serde(skip)]
    pub program: Arc<crate::shader::ShaderProgram>,
    pub uniforms: [[f32; 4]; 16],
}
impl ShaderMaterial {
    pub fn new(program: Arc<crate::shader::ShaderProgram>) -> Self {
        Self {
            properties: MaterialProperties {
                force_single_pass: true,
                ..Default::default()
            },
            program,
            uniforms: [[0.0; 4]; 16],
        }
    }
}

impl MeshPhysicalMaterial {
    pub(crate) fn extension_maps(&self) -> [Option<&Arc<Texture>>; 12] {
        [
            self.clearcoat_map.as_ref(),
            self.clearcoat_roughness_map.as_ref(),
            self.clearcoat_normal_map.as_ref(),
            self.sheen_color_map.as_ref(),
            self.sheen_roughness_map.as_ref(),
            self.anisotropy_map.as_ref(),
            self.specular_intensity_map.as_ref(),
            self.specular_color_map.as_ref(),
            self.transmission_map.as_ref(),
            self.thickness_map.as_ref(),
            self.iridescence_map.as_ref(),
            self.iridescence_thickness_map.as_ref(),
        ]
    }
}
