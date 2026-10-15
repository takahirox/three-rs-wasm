//! webgl_postprocessing_afterimage: a turning MeshNormalMaterial box in
//! black fog, through AfterimagePass and OutputPass. Each animation frame
//! renders the scene ( RenderPass ), composites it with the previous
//! composite ( AfterimageShader: the old texel damped, dropped at 0.1 or
//! below, then the per-channel maximum with the new one ), copies the
//! composite to the composer's write buffer and swaps the two half-float
//! history targets. A re-render without an animation frame ( pointer input,
//! a resize ) shows the last result again, as the page draws only in animate().
use super::ssao::{COPY, Pass, TEXEL, half, pass, target};
use crate::tsl::Type;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// AfterimageShader ( u.custom[0].x: damp; textures tOld, tNew ).
const AFTERIMAGE: &str = "fn afterimage()->vec4<f32>{let uv=fragment_surface.uv;var old=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),uv),0);let fresh=textureLoad(tsl_texture_1,gl_texel(textureDimensions(tsl_texture_1),uv),0);old*=u.custom[0].x*max(sign(old-vec4(0.1)),vec4(0.0));return max(fresh,old);}";
struct Targets {
    width: u32,
    height: u32,
    /// The composer's read and write buffers.
    read: RenderTarget,
    write: RenderTarget,
    /// AfterimagePass's _textureComp and _textureOld, by parity.
    history: [RenderTarget; 2],
}
pub(super) struct Demo {
    mesh: Object3D,
    rotation: Vector2,
    damp: f64,
    enable: bool,
    /// Animation frames due at the next render.
    steps: u32,
    /// Which history target holds _textureOld.
    parity: usize,
    /// The last frame's OutputPass read the write buffer ( afterimage on ).
    from_write: bool,
    targets: Option<Targets>,
    /// Per parity: the composite into the other target, and its copy into
    /// the write buffer.
    composite: [Pass; 2],
    copy: [Pass; 2],
    /// OutputPass from the write buffer and from the read buffer.
    output: [Pass; 2],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 400.);
        n.quaternion = Quaternion::IDENTITY;
        // No background: RenderPass clears to the renderer's opaque black.
        s.background = Color::BLACK;
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 1.,
            far: 1000.,
        });
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::segmented(150., 150., 150., 2, 2, 2)?),
            Arc::new(Material::Normal(MeshNormalMaterial::default())),
        )));
        let nearest = r.device.create_sampler(&Default::default());
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        let one = [(&view, &nearest, Type::Texture, half())];
        let two = [one[0], (&view, &nearest, Type::Texture, half())];
        let copy_source = format!("{TEXEL}{COPY}");
        let source = format!("{TEXEL}{AFTERIMAGE}");
        let copy = || pass(r, "ssao_copy", &copy_source, &one);
        let composite = || pass(r, "afterimage", &source, &two);
        Ok(Self {
            mesh,
            rotation: Vector2::ZERO,
            damp: 0.96,
            enable: true,
            steps: 1,
            parity: 0,
            from_write: true,
            targets: None,
            composite: [composite().await?, composite().await?],
            copy: [copy().await?, copy().await?],
            output: [copy().await?, copy().await?],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let read = target(r, w, h, true)?;
        let write = target(r, w, h, false)?;
        // setSize disposes the history targets: their contents restart at zero.
        let history = [target(r, w, h, false)?, target(r, w, h, false)?];
        let nearest = r.device.create_sampler(&Default::default());
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        let bind = |p: &mut Pass, views: &[&wgpu::TextureView]| -> Result<()> {
            let bindings: Vec<_> = views.iter().map(|v| (*v, &nearest)).collect();
            Arc::make_mut(&mut p.material()?.program).rebind(r, &[], &bindings)
        };
        let (read_view, write_view) = (view(&read), view(&write));
        for parity in 0..2 {
            let (old, comp) = (view(&history[parity]), view(&history[1 - parity]));
            bind(&mut self.composite[parity], &[&old, &read_view])?;
            bind(&mut self.copy[parity], &[&comp])?;
        }
        bind(&mut self.output[0], &[&write_view])?;
        bind(&mut self.output[1], &[&read_view])?;
        self.parity = 0;
        self.targets = Some(Targets {
            width: w,
            height: h,
            read,
            write,
            history,
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("afterimage targets"))?;
        let mut uniforms = [[0f32; 4]; 16];
        uniforms[0][0] = self.damp as f32;
        for _ in 0..std::mem::take(&mut self.steps) {
            // animate(): the turn, then composer.render().
            self.rotation += Vector2::new(0.005, 0.01);
            s.get_mut(self.mesh)?.quaternion = Euler {
                angles: Vector3::new(self.rotation.x, self.rotation.y, 0.),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            r.render(s, c, &t.read)?;
            self.from_write = self.enable;
            if self.enable {
                let parity = self.parity;
                self.composite[parity].render(r, &t.history[1 - parity], &uniforms)?;
                self.copy[parity].render(r, &t.write, &uniforms)?;
                self.parity = 1 - parity;
            }
        }
        self.output[usize::from(!self.from_write)].render(r, out, &uniforms)?;
        Ok(true)
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
    /// The GUI: damp, enable.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.damp = value as f64,
            1 => self.enable = value > 0.5,
            _ => return Err(Error::Invalid("afterimage parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.steps += 1;
    }
}
