//! Pinned r186 procedural TSL scenes. Shading and skinning execute on the GPU.
use super::gltf_viewer::{OrbitViewer, load_asset};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, *},
};
use std::sync::Arc;
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn rgb(hex: u32) -> tsl::Node {
    let c = Color::from_hex(hex).0;
    vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32))
}
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn time() -> tsl::Node {
    uniform(0, Type::Float)
}
fn sky_direction() -> tsl::Node {
    WgslFn::new("procedural_sky_direction", "fn procedural_sky_direction(uv:vec2<f32>)->vec3<f32>{let p=uv*2.0-1.0;let direction=vec3(p.x/u.projection[0][0],p.y/u.projection[1][1],-1.0);return -normalize((transpose(u.view)*vec4(direction,0.0)).xyz);}",&[Type::Vec2],Type::Vec3).unwrap().call(&[uv()])
}
fn target(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: HDR,
            samples: 4,
            ..Default::default()
        },
    )
}
mod angular;
pub(in crate::browser) mod audio;
mod blurred_reflection;
mod mirror;
mod motion;
mod motion_blur;
mod noise;
mod pixel;
mod portal;
mod rain;
mod readback;
mod reflection;
mod retro;
mod shadowmap;
mod skinning_points;
mod snow;
mod taau;
mod terrain;
mod traa;
mod tree;
mod wood;
pub(super) enum Demo {
    Taau(Box<taau::Taau>),
    MotionBlur(Box<motion_blur::MotionBlur>),
    Traa(Box<traa::Traa>),
    Retro(Box<retro::Retro>),
    Rain(Box<rain::Rain>),
    Snow(Box<snow::Snow>),
    Pixel(Box<pixel::Pixel>),
    BlurredReflection(Box<blurred_reflection::BlurredReflection>),
    Audio(Box<audio::Audio>),
    Tree(Box<tree::Tree>),
    Terrain(Box<terrain::Terrain>),
    Readback(Box<readback::Readback>),
    ShadowMap(Box<shadowmap::ShadowMap>),
    Mirror(Box<mirror::Mirror>),
    Angular(Box<angular::Angular>),
    Wood(Box<wood::Wood>),
    SkinningPoints(Box<skinning_points::SkinningPoints>),
    Portal(Box<portal::Portal>),
    Noise(Box<noise::Noise>),
    Reflection(Box<reflection::Reflection>),
}
impl Demo {
    pub fn audio(&self) -> Result<&audio::Audio> {
        if let Self::Audio(a) = self {
            Ok(a)
        } else {
            Err(Error::Invalid("not an audio example"))
        }
    }
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        match id {
            146 => Ok(Self::Taau(Box::new(taau::Taau::create(s, c, r).await?))),
            147 => Ok(Self::MotionBlur(Box::new(
                motion_blur::MotionBlur::create(s, c, r).await?,
            ))),
            145 => Ok(Self::Traa(Box::new(traa::Traa::create(s, c, r).await?))),
            144 => Ok(Self::Retro(Box::new(retro::Retro::create(s, c, r).await?))),
            143 => Ok(Self::Rain(Box::new(rain::Rain::create(s, c, r).await?))),
            142 => Ok(Self::Snow(Box::new(snow::Snow::create(s, c, r).await?))),
            141 => Ok(Self::Pixel(Box::new(pixel::Pixel::create(s, c, r).await?))),
            140 => Ok(Self::BlurredReflection(Box::new(
                blurred_reflection::BlurredReflection::create(s, c, r).await?,
            ))),
            139 => Ok(Self::Audio(Box::new(audio::Audio::create(s, r).await?))),
            138 => Ok(Self::Tree(Box::new(tree::Tree::create(s, c, r).await?))),
            137 => Ok(Self::Mirror(Box::new(
                mirror::Mirror::create(s, c, r).await?,
            ))),
            136 => Ok(Self::ShadowMap(Box::new(
                shadowmap::ShadowMap::create(s, c, r).await?,
            ))),
            135 => Ok(Self::Readback(Box::new(
                readback::Readback::create(s, c, r).await?,
            ))),
            134 => Ok(Self::Terrain(Box::new(
                terrain::Terrain::create(s, c, r).await?,
            ))),
            133 => Ok(Self::Angular(Box::new(
                angular::Angular::create(s, c, r).await?,
            ))),
            132 => Ok(Self::Wood(Box::new(wood::Wood::create(s, c, r).await?))),
            131 => Ok(Self::SkinningPoints(Box::new(
                skinning_points::SkinningPoints::create(s, c, r).await?,
            ))),
            128 => Ok(Self::Portal(Box::new(
                portal::Portal::create(s, c, id, r).await?,
            ))),
            129 => Ok(Self::Noise(Box::new(noise::Noise::create(s, c, r).await?))),
            130 => Ok(Self::Reflection(Box::new(
                reflection::Reflection::create(s, c, r).await?,
            ))),
            _ => Err(Error::Invalid("procedural example")),
        }
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        match self {
            Self::MotionBlur(x) => x.update(s, c, d, a),
            Self::Taau(x) => x.update(s, c, d, a),
            Self::Traa(x) => x.update(s, c, d, a),
            Self::Retro(x) => x.update(s, c, d, a),
            Self::Rain(x) => x.update(s, c, d, a),
            Self::Snow(x) => x.update(s, c, d, a),
            Self::Pixel(x) => x.update(s, c, d, a),
            Self::Audio(_) => Ok(()),
            Self::BlurredReflection(x) => x.update(s, c, d, a),
            Self::Tree(x) => x.update(s, c, d, a),
            Self::SkinningPoints(x) => x.update(s, c, d, a),
            Self::Terrain(x) => x.update(s, c, d, a),
            Self::Readback(x) => x.update(s, c, d, a),
            Self::ShadowMap(x) => x.update(s, c, d, a),
            Self::Mirror(x) => x.update(s, c, d, a),
            Self::Angular(x) => x.update(s, c, d, a),
            Self::Wood(x) => x.update(s, c, d, a),
            Self::Portal(x) => x.update(s, c, d, a),
            Self::Noise(x) => x.update(s, c, d, a),
            Self::Reflection(x) => x.update(s, c, d, a),
        }
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        if let Self::Terrain(t) = self {
            t.pointer(x, y);
        }
    }
    pub fn dragging(&mut self, value: bool) {
        if let Self::MotionBlur(x) = self {
            x.dragging = value;
        }
        if let Self::Snow(x) = self {
            x.dragging = value;
        }
        if let Self::Tree(t) = self {
            t.dragging = value;
        }
        if let Self::Terrain(t) = self {
            t.dragging(value);
        }
    }
    pub fn seek(&mut self, t: f64) {
        match self {
            Self::MotionBlur(x) => x.seek(t),
            Self::Taau(x) => x.seek(t),
            Self::Traa(x) => x.seek(t),
            Self::Retro(x) => x.seek(t),
            Self::Rain(x) => x.seek(t),
            Self::Snow(x) => x.seek(t),
            Self::Pixel(x) => x.seek(t),
            Self::Audio(_) => {}
            Self::BlurredReflection(x) => x.seek(t),
            Self::Tree(x) => x.seek(t),
            Self::SkinningPoints(x) => x.seek(t),
            Self::Wood(_) | Self::Angular(_) | Self::Terrain(_) => {}
            Self::Readback(x) => x.seek(t),
            Self::ShadowMap(x) => x.seek(t),
            Self::Mirror(x) => x.seek(t),
            Self::Portal(x) => x.seek(t),
            Self::Noise(x) => x.seek(t),
            Self::Reflection(x) => x.seek(t),
        }
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        match self {
            Self::MotionBlur(x) => x.parameter(i, v),
            Self::Taau(x) => x.parameter(i, v),
            Self::Traa(_) => Err(Error::Invalid("TRAA has no parameters")),
            Self::Retro(x) => x.parameter(i, v),
            Self::Rain(x) => x.parameter(i, v),
            Self::Pixel(x) => x.parameter(i, v),
            Self::BlurredReflection(x) => x.parameter(i, v),
            Self::Audio(x) => x.parameter(i, v),
            Self::Terrain(x) => x.parameter(i, v),
            Self::Readback(x) => x.parameter(i, v),
            Self::Angular(x) => x.parameter(i, v),
            Self::Wood(x) => x.parameter(i, v),
            Self::Portal(x) => x.parameter(i, v),
            Self::Snow(_)
            | Self::Tree(_)
            | Self::Mirror(_)
            | Self::ShadowMap(_)
            | Self::Noise(_)
            | Self::Reflection(_)
            | Self::SkinningPoints(_) => Err(Error::Invalid("noise parameter")),
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        o: &RenderTarget,
    ) -> Result<bool> {
        match self {
            Self::MotionBlur(x) => x.render(r, s, c, o),
            Self::Taau(x) => x.render(r, s, c, o),
            Self::Traa(x) => x.render(r, s, c, o),
            Self::Retro(x) => x.render(r, s, c, o),
            Self::Rain(x) => x.render(r, s, c, o),
            Self::Snow(x) => x.render(r, s, c, o),
            Self::Pixel(x) => x.render(r, s, c, o),
            Self::Audio(_) => Ok(false),
            Self::BlurredReflection(x) => x.render(r, s, c, o),
            Self::Tree(x) => x.render(r, s, c, o),
            Self::SkinningPoints(x) => x.render(r, s, c, o),
            Self::Readback(x) => x.render(r, s, c, o),
            Self::Mirror(x) => x.render(r, s, c, o),
            Self::Portal(x) => x.render(r, s, c, o),
            Self::ShadowMap(_)
            | Self::Noise(_)
            | Self::Wood(_)
            | Self::Angular(_)
            | Self::Terrain(_) => Ok(false),
            Self::Reflection(x) => x.render(r, s, c, o),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        match self {
            Self::MotionBlur(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Taau(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Traa(_) => Ok(()),
            Self::Retro(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Rain(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Snow(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Pixel(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Audio(_) => Ok(()),
            Self::BlurredReflection(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Tree(x) => x.input(s, c, dx, dy, w, p, h),
            Self::SkinningPoints(_) => Ok(()),
            Self::Terrain(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Readback(x) => x.input(s, c, dx, dy, w, p, h),
            Self::ShadowMap(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Mirror(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Angular(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Wood(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Portal(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Noise(x) => x.input(s, c, dx, dy, w, p, h),
            Self::Reflection(x) => x.input(s, c, dx, dy, w, p, h),
        }
    }
}
