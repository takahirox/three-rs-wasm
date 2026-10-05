//! webgl_postprocessing_sobel: a yellow Phong torus knot under an ambient
//! light and a point light carried by the camera, through RenderPass, the
//! LuminosityShader pass and the SobelOperatorShader pass. The Sobel pass is
//! the composer's last and draws to the canvas without an OutputPass, so it
//! writes the gradient magnitude as is. With enable off the page renders the
//! scene directly, encoded as usual.
use super::controls_attributes::{Controls, camera_state};
use super::ssao::{Pass, TEXEL, half, pass, target};
use crate::tsl::Type;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// LuminosityShader: luminance() of the texel, alpha kept.
const LUMINOSITY: &str = "fn sobel_luminosity()->vec4<f32>{let t=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let l=dot(t.rgb,vec3(0.2126,0.7152,0.0722));return vec4(l,l,l,t.w);}";
/// SobelOperatorShader. The resolution is the drawing buffer's, so its
/// offsets fall on texel centers: each tap is the neighbouring texel ( clamped
/// at the edges ). The canvas output encodes sRGB, so the raw magnitude the
/// page writes is decoded first.
const SOBEL: &str = "fn sobel_tap(c:vec2<i32>,dx:i32,dy:i32)->f32{let size=vec2<i32>(textureDimensions(tsl_texture_0));return textureLoad(tsl_texture_0,clamp(c+vec2(dx,-dy),vec2(0),size-1),0).r;}fn sobel()->vec4<f32>{let c=gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv);let tx0y0=sobel_tap(c,-1,-1);let tx0y1=sobel_tap(c,-1,0);let tx0y2=sobel_tap(c,-1,1);let tx1y0=sobel_tap(c,0,-1);let tx1y2=sobel_tap(c,0,1);let tx2y0=sobel_tap(c,1,-1);let tx2y1=sobel_tap(c,1,0);let tx2y2=sobel_tap(c,1,1);let gx=-1.0*tx0y0+1.0*tx2y0-2.0*tx0y1+2.0*tx2y1-1.0*tx0y2+1.0*tx2y2;let gy=-1.0*tx0y0-2.0*tx1y0-1.0*tx2y0+1.0*tx0y2+2.0*tx1y2+1.0*tx2y2;let g=sqrt(gx*gx+gy*gy);let d=select(pow(g*0.9478672986+0.0521327014,2.4),g*0.0773993808,g<=0.04045);return vec4(vec3(d),1.0);}";
struct Targets {
    width: u32,
    height: u32,
    /// The composer's read and write buffers.
    read: RenderTarget,
    write: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    enable: bool,
    targets: Option<Targets>,
    luminosity: Pass,
    sobel: Pass,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 1., 3.);
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::BLACK;
        let mut material = MeshPhongMaterial::default();
        material.properties.color = Color::from_hex(0xffff00);
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1., 0.3, 256, 32, 2, 3)?),
            Arc::new(Material::Phong(material)),
        )));
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xe7e7e7),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 20.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(c, light)?;
        // enableZoom false.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let nearest = r.device.create_sampler(&Default::default());
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        let textures = [(&view, &nearest, Type::Texture, half())];
        Ok(Self {
            controls,
            enable: true,
            targets: None,
            luminosity: pass(
                r,
                "sobel_luminosity",
                &format!("{TEXEL}{LUMINOSITY}"),
                &textures,
            )
            .await?,
            sobel: pass(r, "sobel", &format!("{TEXEL}{SOBEL}"), &textures).await?,
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
        let nearest = r.device.create_sampler(&Default::default());
        for (p, t) in [(&mut self.luminosity, &read), (&mut self.sobel, &write)] {
            let view = t.texture.create_view(&Default::default());
            Arc::make_mut(&mut p.material()?.program).rebind(r, &[], &[(&view, &nearest)])?;
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
        if !self.enable {
            r.render(s, c, out)?;
            return Ok(true);
        }
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("sobel targets"))?;
        let uniforms = [[0f32; 4]; 16];
        r.render(s, c, &t.read)?;
        self.luminosity.render(r, &t.write, &uniforms)?;
        self.sobel.render(r, out, &uniforms)?;
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
            // enableZoom is false.
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The enable checkbox.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("sobel parameter"));
        }
        self.enable = value > 0.5;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
