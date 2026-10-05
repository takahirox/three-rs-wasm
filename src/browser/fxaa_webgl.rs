//! webgl_postprocessing_fxaa: 100 flat-shaded tetrahedrons ( one instanced
//! mesh ) under a hemisphere and a directional light, drawn twice per frame
//! by two EffectComposers: the left half of the canvas shows RenderPass and
//! OutputPass, the right half adds FXAAPass after the OutputPass. Each
//! composer renders the scene into its own half-float target; only the last
//! pass to the screen is scissored, as WebGLRenderer applies the scissor to
//! the canvas and not to render targets. The column left of the right half
//! stays cleared, as on the page ( setScissor( 0, 0, halfWidth − 1, … ) ).
use super::controls_attributes::{Controls, camera_state};
use super::ssao::{Pass, TEXEL, half, pass, target};
use crate::tsl::Type;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
/// The page's transparent canvas over its white body: the premultiplied
/// sRGB color composited over white, decoded for the encoding output.
pub(super) const OVER_WHITE: &str = "fn fxaa_over_white(c:vec4<f32>)->vec4<f32>{let d=c.rgb+vec3(1.0-c.a);return vec4(select(pow(d*0.9478672986+vec3(0.0521327014),vec3(2.4)),d*0.0773993808,d<=vec3(0.04045)),1.0);}";
/// OutputPass to the canvas, shown over the page.
pub(super) const SHOW: &str = "fn fxaa_show()->vec4<f32>{let c=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let rgb=select(pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb*12.92,c.rgb<=vec3(0.0031308));return fxaa_over_white(vec4(rgb,c.a));}";
/// OutputPass into a render target: the linear color encoded to sRGB
/// ( no tone mapping ), stored in the half-float buffer FXAAPass reads.
const ENCODE: &str = "fn fxaa_encode()->vec4<f32>{let c=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let rgb=select(pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb*12.92,c.rgb<=vec3(0.0031308));return vec4(rgb,c.a);}";
/// FXAAShader ( u.custom[0].xy: resolution ). vUv is the fragment's uv with v
/// up, as in WebGL; the target is stored from its top row. The result is
/// already sRGB and shown over the page.
const FXAA: &str = "fn fxaa_sample(uv:vec2<f32>)->vec4<f32>{return textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y),0.0);}
fn fxaa_luma(uv:vec2<f32>)->f32{return dot(fxaa_sample(uv).rgb,vec3(0.3,0.59,0.11));}
fn fxaa_luma_at(t:vec2<f32>,uv:vec2<f32>,du:f32,dv:f32)->f32{return fxaa_luma(uv+t*vec2(du,dv));}
fn fxaa_apply(t:vec2<f32>,uv0:vec2<f32>)->vec4<f32>{
var uv=uv0;
let m=fxaa_luma(uv);let n=fxaa_luma_at(t,uv,0.0,1.0);let e=fxaa_luma_at(t,uv,1.0,0.0);let s=fxaa_luma_at(t,uv,0.0,-1.0);let w=fxaa_luma_at(t,uv,-1.0,0.0);
let ne=fxaa_luma_at(t,uv,1.0,1.0);let nw=fxaa_luma_at(t,uv,-1.0,1.0);let se=fxaa_luma_at(t,uv,1.0,-1.0);let sw=fxaa_luma_at(t,uv,-1.0,-1.0);
let highest=max(max(max(max(n,e),s),w),m);let lowest=min(min(min(min(n,e),s),w),m);let contrast=highest-lowest;
let threshold=max(0.0312,0.063*highest);
if contrast<threshold {return fxaa_sample(uv);}
var f=2.0*(n+e+s+w);f+=ne+nw+se+sw;f*=1.0/12.0;f=abs(f-m);f=clamp(f/contrast,0.0,1.0);
let blend_factor=smoothstep(0.0,1.0,f);let pixel_blend=blend_factor*blend_factor*1.0;
let horizontal=abs(n+s-2.0*m)*2.0+abs(ne+se-2.0*e)+abs(nw+sw-2.0*w);
let vertical=abs(e+w-2.0*m)*2.0+abs(ne+nw-2.0*n)+abs(se+sw-2.0*s);
let is_horizontal=horizontal>=vertical;
let p_lum=select(e,n,is_horizontal);let n_lum=select(w,s,is_horizontal);
let p_grad=abs(p_lum-m);let n_grad=abs(n_lum-m);
var pixel_step=select(t.x,t.y,is_horizontal);var opposite=p_lum;var gradient=p_grad;
if p_grad<n_grad {pixel_step=-pixel_step;opposite=n_lum;gradient=n_grad;}
var uv_edge=uv;var edge_step=vec2(0.0,t.y);
if is_horizontal {uv_edge.y+=pixel_step*0.5;edge_step=vec2(t.x,0.0);} else {uv_edge.x+=pixel_step*0.5;}
let edge_lum=(m+opposite)*0.5;let gradient_threshold=gradient*0.25;
let steps=array<f32,6>(1.0,1.5,2.0,2.0,2.0,4.0);
var puv=uv_edge+edge_step*steps[0];var p_delta=fxaa_luma(puv)-edge_lum;var p_end=abs(p_delta)>=gradient_threshold;
for(var i=1;i<6 && !p_end;i++){puv+=edge_step*steps[i];p_delta=fxaa_luma(puv)-edge_lum;p_end=abs(p_delta)>=gradient_threshold;}
if !p_end {puv+=edge_step*8.0;}
var nuv=uv_edge-edge_step*steps[0];var n_delta=fxaa_luma(nuv)-edge_lum;var n_end=abs(n_delta)>=gradient_threshold;
for(var i=1;i<6 && !n_end;i++){nuv-=edge_step*steps[i];n_delta=fxaa_luma(nuv)-edge_lum;n_end=abs(n_delta)>=gradient_threshold;}
if !n_end {nuv-=edge_step*8.0;}
var p_dist=puv.y-uv.y;var n_dist=uv.y-nuv.y;
if is_horizontal {p_dist=puv.x-uv.x;n_dist=uv.x-nuv.x;}
var shortest=n_dist;var delta_sign=n_delta>=0.0;
if p_dist<=n_dist {shortest=p_dist;delta_sign=p_delta>=0.0;}
var edge_blend=0.5-shortest/(p_dist+n_dist);
if delta_sign==(m-edge_lum>=0.0) {edge_blend=0.0;}
let final_blend=max(pixel_blend,edge_blend);
if is_horizontal {uv.y+=pixel_step*final_blend;} else {uv.x+=pixel_step*final_blend;}
return fxaa_sample(uv);}
fn fxaa()->vec4<f32>{return fxaa_over_white(fxaa_apply(u.custom[0].xy,fragment_surface.uv));}";
/// One EffectComposer's read buffer ( RenderPass ) and, for the FXAA
/// composer, its write buffer ( OutputPass ).
struct Targets {
    width: u32,
    height: u32,
    plain: RenderTarget,
    scene: RenderTarget,
    encoded: RenderTarget,
    output: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// The controls.update() calls due before the next render, each
    /// auto-rotating: one per animation frame ( the page's animate() ) and one
    /// per wheel event ( OrbitControls updates in its wheel handler ).
    /// Re-renders for other pointer input run none.
    steps: u32,
    targets: Option<Targets>,
    /// OutputPass to the screen, OutputPass into the write buffer, FXAAPass.
    output: Pass,
    encode: Pass,
    fxaa: Pass,
    sampler: wgpu::Sampler,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 500.);
        // RenderPass.clearAlpha 0 over no background.
        s.background = Color::BLACK;
        s.background_alpha = 0.;
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x8d8d8d),
            intensity: 1.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(0., 1000., 0.);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(-3000., 1000., -1000.);
        let mut random = Random(186);
        let mut instances = vec![];
        for _ in 0..100 {
            let position = Vector3::new(
                random.next() * 500. - 250.,
                random.next() * 500. - 250.,
                random.next() * 500. - 250.,
            );
            let scale = random.next() * 2. + 1.;
            let rotation = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                random.next() * PI,
                random.next() * PI,
                random.next() * PI,
            );
            instances.push(Instance {
                matrix: Matrix4::from_scale_rotation_translation(
                    Vector3::splat(scale),
                    rotation,
                    position,
                ),
                ..Default::default()
            });
        }
        let mut material = MeshStandardMaterial::default();
        material.properties.color = Color::from_hex(0xf73232);
        material.properties.flat_shading = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TetrahedronGeometry::build(10., 0)?),
            Arc::new(Material::Standard(material)),
        )));
        s.get_mut(mesh)?.instances = instances;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(2.);
        controls.update(s, c)?;
        let nearest = r.device.create_sampler(&Default::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        let texture = |s| [(&view, s, Type::Texture, half())];
        Ok(Self {
            controls,
            steps: 1,
            targets: None,
            output: pass(
                r,
                "fxaa_show",
                &format!("{TEXEL}{OVER_WHITE}{SHOW}"),
                &texture(&nearest),
            )
            .await?,
            encode: pass(
                r,
                "fxaa_encode",
                &format!("{TEXEL}{ENCODE}"),
                &texture(&nearest),
            )
            .await?,
            fxaa: pass(
                r,
                "fxaa",
                &format!("{OVER_WHITE}{FXAA}"),
                &texture(&sampler),
            )
            .await?,
            sampler,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.steps) {
            self.controls.frame_update(s, c)?;
        }
        self.controls.update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (w, h) = (out.width, out.height);
        let plain = target(r, w, h, true)?;
        let scene = target(r, w, h, true)?;
        let encoded = target(r, w, h, false)?;
        let nearest = r.device.create_sampler(&Default::default());
        let rebind = |p: &mut Pass, t: &RenderTarget, s: &wgpu::Sampler| -> Result<()> {
            let view = t.texture.create_view(&Default::default());
            Arc::make_mut(&mut p.material()?.program).rebind(r, &[], &[(&view, s)])
        };
        rebind(&mut self.output, &plain, &nearest)?;
        rebind(&mut self.encode, &scene, &nearest)?;
        rebind(&mut self.fxaa, &encoded, &self.sampler)?;
        let mut options = out.options.clone();
        options.load_color = false;
        self.targets = Some(Targets {
            width: w,
            height: h,
            plain,
            scene,
            encoded,
            output: RenderTarget::with_options(&r.device, w, h, options)?,
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
            self.resize(r, out)?;
        }
        let t = self
            .targets
            .as_mut()
            .ok_or(Error::Invalid("fxaa targets"))?;
        let (w, h) = (t.width, t.height);
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.);
        // container.offsetWidth / 2 in CSS pixels, then setScissor's
        // multiplyScalar( pixelRatio ).round().
        let half_width = (w as f64 / dpr) / 2.;
        let left = ((half_width - 1.) * dpr).round() as u32;
        let right_x = (half_width * dpr).round() as u32;
        let mut uniforms = [[0f32; 4]; 16];
        uniforms[0] = [1. / w as f32, 1. / h as f32, 0., 0.];
        // composer1: RenderPass, then OutputPass to the left of the canvas.
        r.render(s, c, &t.plain)?;
        t.output.scissor = Some([0, 0, left.max(1), h]);
        // The canvas is cleared each frame: the page shows through the gap.
        self.output.scene.background = Color::WHITE;
        self.output.render(r, &t.output, &uniforms)?;
        // composer2: RenderPass, OutputPass, then FXAAPass to the right.
        r.render(s, c, &t.scene)?;
        self.encode.render(r, &t.encoded, &uniforms)?;
        t.output.set_load_color(true);
        t.output.scissor = Some([right_x.min(w - 1), 0, w - right_x.min(w - 1), h]);
        self.fxaa.render(r, &t.output, &uniforms)?;
        t.output.set_load_color(false);
        t.output.scissor = None;
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.output)
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
            self.steps += 1;
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("fxaa parameter"))
    }
    pub fn seek(&mut self, _t: f64) {
        self.steps += 1;
    }
}
