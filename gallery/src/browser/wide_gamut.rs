//! webgl_test_wide_gamut: the sRGB and Display P3 logos as contained
//! scene backgrounds either side of the slider, with LinearDisplayP3 as the
//! working color space and sRGB output (the display is not P3).
use super::controls_attributes::viewport_css;
use super::gltf_viewer::fetch;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{Error, Result, geometry::*, material::*, math::Color, renderer::*, scene::*};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/wide-gamut";
/// A full-viewport background plane (depth test off, rendered first).
const BACKGROUND: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy,1.0,1.0);return out;}";
/// background.glsl: the texel at uvTransform × uv (u.custom[0]: repeat,
/// offset; the image is stored from its top row), kept on its side of the
/// scissor split (u.custom[1]: split in device pixels, side). The texture
/// decodes to linear Display P3, the working space; linearToOutputTexel
/// applies `value.rgb * mat3( … )` with three's 4-decimal working-to-sRGB
/// matrix before the sRGB transfer the target encodes.
const FRAGMENT: &str = "fn wide_gamut()->vec4<f32>{let x=fragment_surface.clip.x;if (u.custom[1].y==0.0 && x>=u.custom[1].x) || (u.custom[1].y==1.0 && x<u.custom[1].x) {discard;}let uv=fragment_surface.uv*u.custom[0].xy+u.custom[0].zw;let c=textureSample(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y));return vec4(c.rgb*mat3x3<f32>(1.3305,0.0760,0.0143,-0.2901,0.9295,-0.1283,-0.0221,-0.0016,1.1053),c.a);}";
/// TextureLoader's image with WebGL's unpack: the sRGB logo is converted to
/// the Display P3 unpack color space by the browser; the Display P3 logo
/// matches the working primaries and uploads without conversion. Both then
/// decode their sRGB transfer when sampled (SRGB8_ALPHA8), with mipmaps.
async fn logo(r: &Renderer, name: &str, convert: bool) -> Result<(wgpu::Texture, wgpu::Sampler)> {
    use wasm_bindgen::JsCast;
    let fail = |e| Error::Asset(format!("wide gamut image: {e:?}"));
    let bytes = fetch(&format!("{ASSETS}/{name}")).await?;
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes.as_slice()));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(fail)?;
    let options = web_sys::ImageBitmapOptions::new();
    options.set_color_space_conversion(if convert {
        web_sys::ColorSpaceConversion::Default
    } else {
        web_sys::ColorSpaceConversion::None
    });
    let bitmap: web_sys::ImageBitmap = wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .create_image_bitmap_with_blob_and_image_bitmap_options(&blob, &options)
            .map_err(fail)?,
    )
    .await
    .map_err(fail)?
    .dyn_into()
    .map_err(fail)?;
    let (width, height) = (bitmap.width(), bitmap.height());
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(name),
        size,
        mip_level_count: width.max(height).ilog2() + 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    r.queue.copy_external_image_to_texture(
        &wgpu::CopyExternalImageSourceInfo {
            source: wgpu::ExternalImageSource::ImageBitmap(bitmap.clone()),
            origin: wgpu::Origin2d::ZERO,
            flip_y: false,
        },
        wgpu::CopyExternalImageDestInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
            color_space: if convert {
                wgpu::PredefinedColorSpace::DisplayP3
            } else {
                wgpu::PredefinedColorSpace::Srgb
            },
            premultiplied_alpha: false,
        },
        size,
    );
    bitmap.close();
    crate::mipmap::MipGenerator::new(&r.device, &texture)?.update(&r.device, &r.queue);
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    Ok((texture, sampler))
}
pub(super) struct Demo {
    planes: [Object3D; 2],
    /// sliderPos in CSS pixels.
    slider: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        s.background = Color::BLACK;
        let geometry = Arc::new(PlaneGeometry::build(2., 2., 1, 1)?);
        let mut planes = vec![];
        for (side, (name, convert)) in [("logo_srgb.png", true), ("logo_p3.png", false)]
            .into_iter()
            .enumerate()
        {
            let (texture, sampler) = logo(r, name, convert).await?;
            let view = texture.create_view(&Default::default());
            let node = WgslFn::new("wide_gamut", FRAGMENT, &[], Type::Vec4)?.call(&[]);
            let mut m = ShaderMaterial::new(Arc::new(
                ShaderProgram::with_projection_and_dimensions(
                    r,
                    &NodeMaterial::new(node).wgsl_with_texture_types(&[Type::Texture], &[])?,
                    &[(&view, &sampler)],
                    &[wgpu::TextureViewDimension::D2],
                    BACKGROUND,
                )
                .await?,
            ));
            m.properties.depth_test = false;
            m.properties.depth_write = false;
            m.uniforms[1][1] = side as f32;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Shader(m)),
            )));
            let n = s.get_mut(h)?;
            n.frustum_culled = false;
            n.render_order = -10000;
            planes.push(h);
        }
        let (w, _, _) = viewport_css();
        Ok(Self {
            planes: planes
                .try_into()
                .map_err(|_| Error::Invalid("wide gamut planes"))?,
            slider: (w / 2.).clamp(10., w - 10.),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// animate(): each scene inside its scissor, the backgrounds contained
    /// (TextureUtils.contain, for the square logos) at the window aspect.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let (w, h, dpr) = viewport_css();
        let aspect = w / h.max(1.);
        let (repeat, offset) = if 1. > aspect {
            ([1., 1. / aspect], [0., (1. - 1. / aspect) / 2.])
        } else {
            ([aspect, 1.], [(1. - aspect) / 2., 0.])
        };
        let split = (self.slider * dpr).round() as f32;
        for plane in self.planes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(plane)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0] = [
                    repeat[0] as f32,
                    repeat[1] as f32,
                    offset[0] as f32,
                    offset[1] as f32,
                ];
                m.uniforms[1][0] = split;
            }
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    /// The page's slider: sliderPos, clamped to 10 px inside the window.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("wide gamut parameter"));
        }
        let (w, _, _) = viewport_css();
        self.slider = (value as f64).clamp(10., w - 10.);
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
