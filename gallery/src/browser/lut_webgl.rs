//! webgl_postprocessing_3dlut: the DamagedHelmet under the royal esplanade
//! UltraHDR background and environment, through RenderPass, OutputPass
//! ( ACES Filmic, sRGB ) and LUTPass with the page's nine lookup tables
//! ( five .CUBE, one .3dl and three image strips, parsed as the page's
//! loaders do into UnsignedByte 3D textures ). LUTPass is the composer's
//! last pass and writes the graded sRGB values to the canvas as they are;
//! with it disabled OutputPass draws to the canvas.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::probes_hdr::ultra_hdr;
use super::ssao::{COPY, FULLSCREEN, Pass, TEXEL, half, pass, target};
use crate::shader::ShaderProgram;
use crate::tsl::Type;
use crate::tsl::lut::Lut3D;
use crate::tsl::{NodeMaterial, WgslFn};
use crate::{
    Error, Result, camera::*, material::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const LUTS: [&str; 9] = [
    "Bourbon 64.CUBE",
    "Chemical 168.CUBE",
    "Clayton 33.CUBE",
    "Cubicle 99.CUBE",
    "Remy 24.CUBE",
    "Presetpro-Cinematic.3dl",
    "NeutralLUT.png",
    "B&WLUT.png",
    "NightLUT.png",
];
/// OutputPass: ACESFilmicToneMapping ( exposure 1 ) and sRGBTransferOETF.
const OUTPUT: &str = "fn lut_rrt(v:vec3<f32>)->vec3<f32>{let a=v*(v+0.0245786)-0.000090537;let b=v*(0.983729*v+0.4329510)+0.238081;return a/b;}fn lut_output()->vec4<f32>{let t=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let input=mat3x3<f32>(vec3(0.59719,0.07600,0.02840),vec3(0.35458,0.90834,0.13383),vec3(0.04823,0.01566,0.83777));let output=mat3x3<f32>(vec3(1.60475,-0.10208,-0.00327),vec3(-0.53108,1.10813,-0.07276),vec3(-0.07367,-0.00605,1.07602));var c=t.rgb*(1.0/0.6);c=input*c;c=lut_rrt(c);c=clamp(output*c,vec3(0.0),vec3(1.0));let s=select(pow(c,vec3(0.41666))*1.055-vec3(0.055),c*12.92,c<=vec3(0.0031308));return vec4(s,t.a);}";
/// LUTShader ( u.custom[0]: lutSize, intensity ): the sample pulled in by
/// half a texel, mixed by the intensity. Its output is the canvas value; the
/// output target encodes sRGB, so it is decoded first.
const LUT: &str = "fn lut_pass()->vec4<f32>{let val=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let size=u.custom[0].x;let pixel=1.0/size;let half_pixel=0.5/size;let uvw=vec3(half_pixel)+val.rgb*(1.0-pixel);let graded=vec4(textureSampleLevel(tsl_texture_1,tsl_sampler_1,uvw,0.0).rgb,val.a);let c=mix(val,graded,u.custom[0].y);let d=select(pow(c.rgb*0.9478672986+vec3(0.0521327014),vec3(2.4)),c.rgb*0.0773993808,c.rgb<=vec3(0.04045));return vec4(d,c.a);}";
struct Targets {
    width: u32,
    height: u32,
    read: RenderTarget,
    write: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    enabled: bool,
    lut: usize,
    intensity: f64,
    targets: Option<Targets>,
    /// OutputPass into the write buffer and to the canvas.
    output: Pass,
    screen: Pass,
    /// Per table: its size and the LUTPass bound to it.
    passes: Vec<(u32, Pass)>,
    nearest: wgpu::Sampler,
    linear: wgpu::Sampler,
    textures: Vec<wgpu::TextureView>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.25,
            far: 20.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-1.8, 0.6, 2.7);
        // OutputPass tone maps; the scene renders linear.
        s.tone_mapping = ToneMapping::None;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        let (a, b, i) = load_asset("/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf").await?;
        crate::gltf::import_decoded(&a, &b, &i)?.instantiate(s)?;
        for h in s.handles().collect::<Vec<_>>() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for material in &mut m.materials {
                    if let Material::Standard(m) = Arc::make_mut(material) {
                        // r186's WebGL physical shading: the DFG LUT's multiple scattering.
                        m.energy_conservation = true;
                    }
                }
            }
        }
        let mut controls = Controls::new(None, (2., 10.), PI, true);
        controls.set_target(Vector3::new(0., 0., -0.2));
        controls.update(s, c)?;
        let nearest = r.device.create_sampler(&Default::default());
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        let one = [(&view, &nearest, Type::Texture, half())];
        let mut textures = vec![];
        let mut passes = vec![];
        let lut_source = format!("{TEXEL}{LUT}");
        for name in LUTS {
            let data = fetch(&format!("{ASSETS}/tsl-next/luts/{name}")).await?;
            let text = || std::str::from_utf8(&data).map_err(|_| Error::Invalid("LUT text"));
            let lut = if name.ends_with(".CUBE") {
                Lut3D::from_cube(text()?)?
            } else if name.ends_with(".3dl") {
                Lut3D::from_3dl(text()?)?
            } else {
                let im = image::load_from_memory(&data)
                    .map_err(|e| Error::Asset(e.to_string()))?
                    .to_rgba8();
                Lut3D::from_strip(im.width(), im.height(), im.as_raw())?
            };
            let size = wgpu::Extent3d {
                width: lut.size,
                height: lut.size,
                depth_or_array_layers: lut.size,
            };
            let texture = r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("3D LUT"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            r.queue.write_texture(
                texture.as_image_copy(),
                &lut.rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(lut.size * 4),
                    rows_per_image: Some(lut.size),
                },
                size,
            );
            let lut_view = texture.create_view(&Default::default());
            let node = WgslFn::new("lut_pass", &lut_source, &[], Type::Vec4)?.call(&[]);
            let mut m = ShaderMaterial::new(Arc::new(
                ShaderProgram::with_projection_and_sample_types(
                    r,
                    &NodeMaterial::new(node)
                        .wgsl_with_texture_types(&[Type::Texture, Type::Texture3D], &[])?,
                    &[(&view, &nearest), (&lut_view, &linear)],
                    &[
                        wgpu::TextureViewDimension::D2,
                        wgpu::TextureViewDimension::D3,
                    ],
                    &[half(), wgpu::TextureSampleType::Float { filterable: true }],
                    FULLSCREEN,
                )
                .await?,
            ));
            m.properties.depth_test = false;
            m.properties.depth_write = false;
            let pass = Pass::new(m)?;
            passes.push((lut.size, pass));
            textures.push(lut_view);
        }
        let output_source = format!("{TEXEL}{OUTPUT}");
        let mut screen = pass(r, "ssao_copy", &format!("{TEXEL}{COPY}"), &one).await?;
        screen.scene.tone_mapping = ToneMapping::Aces;
        Ok(Self {
            controls,
            enabled: true,
            lut: 0,
            intensity: 1.,
            targets: None,
            output: pass(r, "lut_output", &output_source, &one).await?,
            screen,
            passes,
            nearest,
            linear,
            textures,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.update(s, c)
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let read = target(r, w, h, true)?;
        let write = target(r, w, h, false)?;
        let read_view = read.texture.create_view(&Default::default());
        let write_view = write.texture.create_view(&Default::default());
        Arc::make_mut(&mut self.output.material()?.program).rebind(
            r,
            &[],
            &[(&read_view, &self.nearest)],
        )?;
        Arc::make_mut(&mut self.screen.material()?.program).rebind(
            r,
            &[],
            &[(&read_view, &self.nearest)],
        )?;
        for ((_, p), lut) in self.passes.iter_mut().zip(&self.textures) {
            Arc::make_mut(&mut p.material()?.program).rebind(
                r,
                &[],
                &[(&write_view, &self.nearest), (lut, &self.linear)],
            )?;
        }
        self.targets = Some(Targets {
            width: w,
            height: h,
            read,
            write,
        });
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let t = self.targets.as_ref().ok_or(Error::Invalid("lut targets"))?;
        let mut uniforms = [[0f32; 4]; 16];
        r.render(s, c, &t.read)?;
        if self.enabled {
            self.output.render(r, &t.write, &uniforms)?;
            let (size, pass) = &mut self.passes[self.lut];
            uniforms[0] = [*size as f32, self.intensity as f32, 0., 0.];
            pass.render(r, out, &uniforms)?;
        } else {
            // OutputPass to the canvas: tone mapped here, encoded by the output.
            self.screen.render(r, out, &uniforms)?;
        }
        Ok(true)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The GUI: enabled, lut, intensity.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.enabled = value > 0.5,
            1 => self.lut = (value.round() as usize).min(LUTS.len() - 1),
            2 => self.intensity = value as f64,
            _ => return Err(Error::Invalid("lut parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
