//! webgl_multisampled_renderbuffers: 50 red Lambert spheres, each with a
//! white wireframe copy ( the solid one pushed back by its polygon offset ),
//! under a hemisphere light and linear fog, in a group turning about y. Two
//! EffectComposers draw the scene each frame: the left half of the canvas
//! through a plain half-float target, the right half through the page's 4×
//! multisampled half-float target. Each composer's RenderPass renders the
//! whole target; only the OutputPass to the canvas is scissored, and the
//! column left of the right half shows the page's white body.
use super::fxaa_webgl::{OVER_WHITE, SHOW};
use super::ssao::{Pass, TEXEL, half, pass, target};
use crate::tsl::Type;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
struct Targets {
    width: u32,
    height: u32,
    /// composer1's read buffer and composer2's multisampled one.
    plain: RenderTarget,
    multisampled: RenderTarget,
    output: RenderTarget,
}
pub(super) struct Demo {
    group: Object3D,
    rotation: f64,
    animate: bool,
    /// Animation frames due before the next render ( the page's animate() ).
    steps: u32,
    targets: Option<Targets>,
    /// OutputPass to the canvas from each composer's buffer.
    outputs: [Pass; 2],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 10.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 500.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::WHITE;
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xcccccc),
            near: 100.,
            far: 1500.,
        });
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x222222),
            intensity: 5.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(1., 1., 1.);
        let group = s.insert(NodeKind::Group);
        let geometry = Arc::new(SphereGeometry::build(10., 64, 40)?);
        let mut solid = MeshLambertMaterial::default();
        solid.properties.color = Color::from_hex(0xee0808);
        solid.properties.polygon_offset = Some((1., 1));
        let solid = Arc::new(Material::Lambert(solid));
        let mut wire = MeshBasicMaterial::default();
        wire.properties.color = Color::WHITE;
        wire.properties.wireframe = true;
        let wire = Arc::new(Material::Basic(wire));
        let mut random = Random(186);
        for _ in 0..50 {
            let position = Vector3::new(
                random.next() * 600. - 300.,
                random.next() * 600. - 300.,
                random.next() * 600. - 300.,
            );
            let rotation = Euler {
                angles: Vector3::new(random.next(), 0., random.next()),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let scale = Vector3::splat(random.next() * 5. + 5.);
            for material in [&solid, &wire] {
                let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                    geometry.clone(),
                    material.clone(),
                )));
                let n = s.get_mut(mesh)?;
                n.position = position;
                n.quaternion = rotation;
                n.scale = scale;
                s.add(group, mesh)?;
            }
        }
        let nearest = r.device.create_sampler(&Default::default());
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        let source = format!("{TEXEL}{OVER_WHITE}{SHOW}");
        let textures = [(&view, &nearest, Type::Texture, half())];
        let output = || pass(r, "fxaa_show", &source, &textures);
        Ok(Self {
            group,
            rotation: 0.,
            animate: true,
            steps: 1,
            targets: None,
            outputs: [output().await?, output().await?],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// animate(): group.rotation.y += 0.002 per frame while animating.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.steps) {
            if self.animate {
                self.rotation += 0.002;
            }
        }
        s.get_mut(self.group)?.quaternion = Quaternion::from_rotation_y(self.rotation);
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (w, h) = (out.width, out.height);
        let plain = target(r, w, h, true)?;
        let multisampled = RenderTarget::with_options(
            &r.device,
            w,
            h,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                depth_buffer: true,
                samples: 4,
                ..Default::default()
            },
        )?;
        let nearest = r.device.create_sampler(&Default::default());
        for (pass, t) in self.outputs.iter_mut().zip([&plain, &multisampled]) {
            let view = t.texture.create_view(&Default::default());
            Arc::make_mut(&mut pass.material()?.program).rebind(r, &[], &[(&view, &nearest)])?;
            // The canvas is cleared each frame: the page shows through the gap.
            pass.scene.background = Color::WHITE;
        }
        let mut options = out.options.clone();
        options.load_color = false;
        self.targets = Some(Targets {
            width: w,
            height: h,
            plain,
            multisampled,
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
            .ok_or(Error::Invalid("multisampled targets"))?;
        let (w, h) = (t.width, t.height);
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.);
        // container.offsetWidth / 2 in CSS pixels, scaled and rounded as
        // setScissor does.
        let half_width = (w as f64 / dpr) / 2.;
        let left = ((half_width - 1.) * dpr).round() as u32;
        let right_x = ((half_width * dpr).round() as u32).min(w - 1);
        let uniforms = [[0f32; 4]; 16];
        r.render(s, c, &t.plain)?;
        t.output.scissor = Some([0, 0, left.max(1), h]);
        self.outputs[0].render(r, &t.output, &uniforms)?;
        r.render(s, c, &t.multisampled)?;
        t.output.set_load_color(true);
        t.output.scissor = Some([right_x, 0, w - right_x, h]);
        self.outputs[1].render(r, &t.output, &uniforms)?;
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
    /// The page's animate checkbox.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("multisampled parameter"));
        }
        self.animate = value > 0.5;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.steps += 1;
    }
}
