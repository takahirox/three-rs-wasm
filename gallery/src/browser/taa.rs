//! webgl_postprocessing_taa: a wireframe box and a nearest-filtered brick box
//! through TAARenderPass (SSAARenderPass while moving; while still, the
//! 32 jittered samples accumulated one level per frame over the held frame)
//! or a plain RenderPass, then OutputPass. The example turns the boxes for
//! 200 frames, then holds them for 200, counting frames.
use super::gltf_viewer::{decode_texture_image, fetch};
use super::ssao::{Pass, TEXEL, half, pass, target};
use crate::tsl::Type;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// SSAARenderPass's jitter vectors, in sixteenths of a pixel.
const JITTER: [&[[f64; 2]]; 6] = [
    &[[0., 0.]],
    &[[4., 4.], [-4., -4.]],
    &[[-2., -6.], [6., -2.], [-6., 2.], [2., 6.]],
    &[
        [1., -3.],
        [-1., 3.],
        [5., 1.],
        [-3., -5.],
        [-5., 5.],
        [-7., -1.],
        [3., 7.],
        [7., -7.],
    ],
    &[
        [1., 1.],
        [-1., -3.],
        [-3., 2.],
        [4., -1.],
        [-5., -2.],
        [2., 5.],
        [5., 3.],
        [3., -5.],
        [-2., 6.],
        [0., -7.],
        [-4., -6.],
        [-6., 4.],
        [-8., 0.],
        [7., -4.],
        [6., 7.],
        [-7., -8.],
    ],
    &[
        [-4., -7.],
        [-7., -5.],
        [-3., -5.],
        [-5., -4.],
        [-1., -4.],
        [-2., -2.],
        [-6., -1.],
        [-4., 0.],
        [-7., 1.],
        [-1., 2.],
        [-6., 3.],
        [-3., 3.],
        [-7., 6.],
        [-3., 6.],
        [-5., 7.],
        [-1., 7.],
        [5., -7.],
        [1., -6.],
        [6., -5.],
        [4., -4.],
        [2., -3.],
        [7., -2.],
        [1., -1.],
        [4., -1.],
        [2., 1.],
        [6., 2.],
        [0., 4.],
        [4., 4.],
        [2., 5.],
        [7., 5.],
        [5., 6.],
        [3., 7.],
    ],
];
/// CopyShader: opacity (u.custom[0].x) × the texel under the fragment.
const COPY: &str = "fn taa_copy()->vec4<f32>{return u.custom[0].x*textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);}";
struct Targets {
    /// The composer's write buffer, SSAA/TAA's sample target and the held frame.
    write: RenderTarget,
    sample: RenderTarget,
    hold: RenderTarget,
}
pub(super) struct Demo {
    meshes: [Object3D; 2],
    /// The children's rotation.x and rotation.y.
    rotation: Vector2,
    /// animate()'s frame index; TAARenderPass.accumulate and accumulateIndex.
    index: u64,
    accumulate: bool,
    accumulate_index: i64,
    targets: Option<Targets>,
    /// Additive copies (premultiplied AdditiveBlending: One, One) from the
    /// sample, write and hold targets, and the output copy of the write target.
    from_sample: Pass,
    from_write: Pass,
    from_hold: Pass,
    output: Pass,
    nearest: wgpu::Sampler,
    /// TAAEnabled, TAASampleLevel.
    params: [f64; 2],
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
        n.position = Vector3::new(0., 0., 300.);
        n.quaternion = Quaternion::IDENTITY;
        // The passes clear to ( 0x000000, 0 ); only the color reaches the page.
        s.background = Color::BLACK;
        let geometry = Arc::new(BoxGeometry::build(120., 120., 120.)?);
        let mut wire = MeshBasicMaterial::default();
        wire.properties.wireframe = true;
        let mut brick = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-next/textures/brick_diffuse.jpg").await?,
        )
        .await?;
        brick.srgb = true;
        brick.filter = Filter::Nearest;
        brick.min_filter = Some(Filter::Nearest);
        brick.mipmap_filter = None;
        let mut basic = MeshBasicMaterial::default();
        basic.properties.map = Some(Arc::new(brick));
        let mut meshes = vec![];
        for (x, m) in [(-100., wire), (100., basic)] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(h)?.position.x = x;
            meshes.push(h);
        }
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let initial = target(r, 1, 1, false)?;
        let color = initial.texture.create_view(&Default::default());
        let copy = || async {
            let mut p = pass(
                r,
                "taa_copy",
                &format!("{TEXEL}{COPY}"),
                &[(&color, &nearest, Type::Texture, half())],
            )
            .await?;
            let m = p.material()?;
            m.properties.transparent = true;
            let one = wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            };
            m.properties.blending = Some(wgpu::BlendState {
                color: one,
                alpha: one,
            });
            Ok::<_, Error>(p)
        };
        let (from_sample, from_write, from_hold) = (copy().await?, copy().await?, copy().await?);
        let mut output = pass(
            r,
            "taa_copy",
            &format!("{TEXEL}{COPY}"),
            &[(&color, &nearest, Type::Texture, half())],
        )
        .await?;
        output.material()?.uniforms[0][0] = 1.;
        Ok(Self {
            meshes: meshes
                .try_into()
                .map_err(|_| Error::Invalid("taa meshes"))?,
            rotation: Vector2::ZERO,
            index: 0,
            accumulate: false,
            accumulate_index: -1,
            targets: None,
            from_sample,
            from_write,
            from_hold,
            output,
            nearest,
            params: [1., 0.],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// animate(): 200 frames turning (accumulate off), 200 still (on).
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.index += 1;
        if ((self.index as f64 / 200.).round() as u64).is_multiple_of(2) {
            self.rotation += Vector2::new(0.005, 0.01);
            let q =
                Quaternion::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, 0.);
            for &mesh in &self.meshes {
                s.get_mut(mesh)?.quaternion = q;
            }
            self.accumulate = false;
        } else {
            self.accumulate = true;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let targets = Targets {
            write: target(r, w, h, true)?,
            sample: target(r, w, h, true)?,
            hold: target(r, w, h, false)?,
        };
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        for (p, source) in [
            (&mut self.from_sample, &targets.sample),
            (&mut self.from_write, &targets.write),
            (&mut self.from_hold, &targets.hold),
            (&mut self.output, &targets.write),
        ] {
            let v = view(source);
            Arc::make_mut(&mut p.material()?.program).rebind(r, &[], &[(&v, &self.nearest)])?;
        }
        self.targets = Some(targets);
        Ok(())
    }
    /// The camera's setViewOffset for one jitter, or clearViewOffset.
    fn jitter(
        s: &mut Scene,
        c: Object3D,
        size: (u32, u32),
        offset: Option<[f64; 2]>,
    ) -> Result<()> {
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.view = offset.map(|[x, y]| ViewOffset {
                full_width: size.0 as f64,
                full_height: size.1 as f64,
                offset_x: x * 0.0625,
                offset_y: y * 0.0625,
                width: size.0 as f64,
                height: size.1 as f64,
            });
        }
        Ok(())
    }
    /// A pass copying additively (opacity) onto a target, optionally cleared first.
    fn add(
        r: &Renderer,
        pass: &mut Pass,
        target: &mut RenderTarget,
        opacity: f64,
        clear: bool,
    ) -> Result<()> {
        let mut uniforms = [[0f32; 4]; 16];
        uniforms[0][0] = opacity as f32;
        target.set_load_color(!clear);
        let result = pass.render(r, target, &uniforms);
        target.set_load_color(false);
        result
    }
    /// SSAARenderPass.render into `destination` (unbiased off): each jittered
    /// render added at 1 / samples over a cleared target.
    fn ssaa(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        t: &mut Targets,
        hold: bool,
    ) -> Result<()> {
        let level = (self.params[1].round() as usize).min(5);
        let jitter = JITTER[level];
        let size = (t.write.width, t.write.height);
        for (i, offset) in jitter.iter().enumerate() {
            Self::jitter(s, c, size, Some(*offset))?;
            r.render(s, c, &t.sample)?;
            let destination = if hold { &mut t.hold } else { &mut t.write };
            Self::add(
                r,
                &mut self.from_sample,
                destination,
                1. / jitter.len() as f64,
                i == 0,
            )?;
        }
        Self::jitter(s, c, size, None)
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
            .is_none_or(|t| t.write.width != out.width || t.write.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let Some(mut t) = self.targets.take() else {
            return Err(Error::Invalid("taa targets"));
        };
        let result = self.passes(r, s, c, &mut t);
        self.targets = Some(t);
        result?;
        self.output.render(r, out, &{
            let mut u = [[0f32; 4]; 16];
            u[0][0] = 1.;
            u
        })?;
        Ok(true)
    }
    fn passes(&mut self, r: &Renderer, s: &mut Scene, c: Object3D, t: &mut Targets) -> Result<()> {
        if self.params[0] < 0.5 {
            // RenderPass.
            return r.render(s, c, &t.write);
        }
        if !self.accumulate {
            self.ssaa(r, s, c, t, false)?;
            self.accumulate_index = -1;
            return Ok(());
        }
        let jitter = JITTER[5];
        let size = (t.write.width, t.write.height);
        if self.accumulate_index == -1 {
            self.ssaa(r, s, c, t, true)?;
            self.accumulate_index = 0;
        }
        let weight = 1. / jitter.len() as f64;
        if (0..jitter.len() as i64).contains(&self.accumulate_index) {
            let per_frame = 1usize << (self.params[1].round() as usize).min(5);
            for _ in 0..per_frame {
                Self::jitter(s, c, size, Some(jitter[self.accumulate_index as usize]))?;
                r.render(s, c, &t.write)?;
                let clear = self.accumulate_index == 0;
                Self::add(r, &mut self.from_write, &mut t.sample, weight, clear)?;
                self.accumulate_index += 1;
                if self.accumulate_index >= jitter.len() as i64 {
                    break;
                }
            }
            Self::jitter(s, c, size, None)?;
        }
        let accumulation = self.accumulate_index as f64 * weight;
        let mut cleared = false;
        if accumulation > 0. {
            Self::add(r, &mut self.from_sample, &mut t.write, 1., true)?;
            cleared = true;
        }
        if accumulation < 1. {
            Self::add(
                r,
                &mut self.from_hold,
                &mut t.write,
                1. - accumulation,
                !cleared,
            )?;
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
    /// TAAEnabled (TAA or the plain RenderPass) and TAASampleLevel.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("taa parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
