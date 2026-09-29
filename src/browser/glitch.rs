//! webgl_postprocessing_glitch: 100 instanced flat-shaded spheres rendered
//! into a half-float target, then GlitchPass's DigitalGlitch shader (the
//! 64 × 64 random displacement map, RGB shift and snow) into the sRGB output.
//! GlitchPass draws from Math.random each frame; the port consumes the same
//! sequence in the same order, frame by frame.
use crate::attribute::BufferAttribute;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
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
    /// MathUtils.randFloat.
    fn float(&mut self, low: f64, high: f64) -> f64 {
        low + self.next() * (high - low)
    }
    /// MathUtils.randInt.
    fn int(&mut self, low: i64, high: i64) -> i64 {
        low + (self.next() * (high - low + 1) as f64).floor() as i64
    }
}
/// DigitalGlitch.fragmentShader. u.custom[0]: amount, angle, seed, seed_x;
/// [1]: seed_y, distortion_x, distortion_y, col_s; [2]: byp and the target
/// height (gl_FragCoord.y counts from the bottom row). The scene target is
/// stored from its top row, so its v is flipped.
const GLITCH: &str = "fn glitch_rand(co:vec2<f32>)->f32{return fract(sin(dot(co,vec2(12.9898,78.233)))*43758.5453);}fn glitch_diffuse(p:vec2<f32>)->vec4<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec2(p.x,1.0-p.y));}fn glitch()->vec4<f32>{let a=u.custom[0];let b=u.custom[1];let amount=a.x;let angle=a.y;let seed=a.z;let seed_x=a.w;let seed_y=b.x;let distortion_x=b.y;let distortion_y=b.z;let col_s=b.w;var p=fragment_surface.uv;let xs=floor(fragment_surface.clip.x/0.5);let ys=floor((u.custom[2].y-fragment_surface.clip.y)/0.5);let disp=textureSample(tsl_texture_1,tsl_sampler_1,p*seed*seed).r;let plain=glitch_diffuse(fragment_surface.uv);if p.y<distortion_x+col_s && p.y>distortion_x-col_s*seed {if seed_x>0.0 {p.y=1.0-(p.y+distortion_y);} else {p.y=distortion_y;}}if p.x<distortion_y+col_s && p.x>distortion_y-col_s*seed {if seed_y>0.0 {p.x=distortion_x;} else {p.x=1.0-(p.x+distortion_x);}}p.x+=disp*seed_x*(seed/5.0);p.y+=disp*seed_y*(seed/5.0);let offset=amount*vec2(cos(angle),sin(angle));let cr=glitch_diffuse(p+offset);let cga=glitch_diffuse(p);let cb=glitch_diffuse(p-offset);let snow=200.0*amount*vec4(glitch_rand(vec2(xs*seed,ys*seed*50.0))*0.2);if u.custom[2].x<1.0 {return vec4(cr.r,cga.g,cb.b,cga.a)+snow;}return plain;}";
/// FullScreenQuad's geometry: one triangle over the viewport, uv 0..2.
fn fullscreen_triangle() -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            vec![-1., 3., 0., -1., -1., 0., 3., -1., 0.],
            3,
            false,
        )?),
    );
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(
            vec![0., 2., 0., 0., 2., 0.],
            2,
            false,
        )?),
    );
    g.set_attribute(
        "normal",
        Attribute::F32(BufferAttribute::new(
            vec![0., 0., 1., 0., 0., 1., 0., 0., 1.],
            3,
            false,
        )?),
    );
    Ok(g)
}
pub(super) struct Demo {
    random: Random,
    object: Object3D,
    rotation: Vector2,
    /// GlitchPass state: _curF, _randX, goWild and the uniforms.
    frame: i64,
    trigger: i64,
    wild: bool,
    uniforms: [[f32; 4]; 3],
    post: Scene,
    post_camera: Object3D,
    quad: Object3D,
    target: Option<RenderTarget>,
    sampler: wgpu::Sampler,
    displacement: (wgpu::Texture, wgpu::Sampler),
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
        s.background = Color::BLACK;
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 1.,
            far: 1000.,
        });
        let mut random = Random(186);
        let object = s.insert(NodeKind::Group);
        let mut instances = vec![];
        for _ in 0..100 {
            let direction = Vector3::new(
                random.next() - 0.5,
                random.next() - 0.5,
                random.next() - 0.5,
            )
            .normalize();
            let position = direction * (random.next() * 400.);
            let rotation = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                random.next() * 2.,
                random.next() * 2.,
                random.next() * 2.,
            );
            let scale = random.next() * 50.;
            let hex = (0xffffff as f64 * random.next()).floor() as u32;
            instances.push(Instance {
                matrix: Matrix4::from_scale_rotation_translation(
                    Vector3::splat(scale),
                    rotation,
                    position,
                ),
                color: Color::from_hex(hex),
            });
        }
        let mut phong = MeshPhongMaterial::default();
        phong.properties.flat_shading = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(1., 4, 4)?),
            Arc::new(Material::Phong(phong)),
        )));
        let n = s.get_mut(mesh)?;
        n.instances = instances;
        n.frustum_culled = false;
        s.add(object, mesh)?;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 1., 1.);
        // GlitchPass(): the heightmap, then the first trigger.
        let heights: Vec<f32> = (0..64 * 64).map(|_| random.float(0., 1.) as f32).collect();
        let trigger = random.int(120, 240);
        let size = wgpu::Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        };
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glitch heightmap"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let bytes: Vec<u8> = heights.iter().flat_map(|h| h.to_le_bytes()).collect();
        r.queue.write_texture(
            texture.as_image_copy(),
            &bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(64 * 4),
                rows_per_image: Some(64),
            },
            size,
        );
        // DataTexture: nearest filtering, clamped.
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // The pass: a full-screen quad reading the scene target.
        let placeholder = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let view = placeholder.texture.create_view(&Default::default());
        let heightmap = texture.create_view(&Default::default());
        let node = WgslFn::new("glitch", GLITCH, &[], Type::Vec4)?.call(&[]);
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_sample_types(
                r,
                &NodeMaterial::new(node).wgsl_with_texture_types(&[Type::Texture, Type::Texture], &[])?,
                &[(&view, &sampler), (&heightmap, &nearest)],
                &[wgpu::TextureViewDimension::D2, wgpu::TextureViewDimension::D2],
                &[
                    wgpu::TextureSampleType::Float { filterable: true },
                    wgpu::TextureSampleType::Float { filterable: false },
                ],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy,0.0,1.0);return out;}",
            )
            .await?,
        ));
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        let mut post = Scene::new();
        post.background = Color::BLACK;
        let post_camera = post.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 1.,
            zoom: 1.,
            ..Default::default()
        })));
        let quad = post.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(fullscreen_triangle()?),
            Arc::new(Material::Shader(m)),
        )));
        post.get_mut(quad)?.frustum_culled = false;
        Ok(Self {
            random,
            object,
            rotation: Vector2::ZERO,
            frame: 0,
            trigger,
            wild: false,
            // DigitalGlitch's defaults: col_s 0.05.
            uniforms: [[0.; 4], [0., 0., 0., 0.05], [0.; 4]],
            post,
            post_camera,
            quad,
            target: None,
            sampler,
            displacement: (texture, nearest),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// animate(): the group turns once per frame, then GlitchPass.render's
    /// random draws and state.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.rotation += Vector2::new(0.005, 0.01);
        s.get_mut(self.object)?.quaternion =
            Quaternion::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, 0.);
        let r = &mut self.random;
        let [a, b, c] = &mut self.uniforms;
        a[2] = r.next() as f32;
        c[0] = 0.;
        if self.frame % self.trigger == 0 || self.wild {
            a[0] = (r.next() / 30.) as f32;
            a[1] = r.float(-PI, PI) as f32;
            a[3] = r.float(-1., 1.) as f32;
            b[0] = r.float(-1., 1.) as f32;
            b[1] = r.float(0., 1.) as f32;
            b[2] = r.float(0., 1.) as f32;
            self.frame = 0;
            self.trigger = r.int(120, 240);
        } else if ((self.frame % self.trigger) as f64) < self.trigger as f64 / 5. {
            a[0] = (r.next() / 90.) as f32;
            a[1] = r.float(-PI, PI) as f32;
            b[1] = r.float(0., 1.) as f32;
            b[2] = r.float(0., 1.) as f32;
            a[3] = r.float(-0.3, 0.3) as f32;
            b[0] = r.float(-0.3, 0.3) as f32;
        } else if !self.wild {
            c[0] = 1.;
        }
        self.frame += 1;
        Ok(())
    }
    /// composer.render(): the RenderPass into the half-float target, the
    /// glitch into the output (OutputPass: the target's sRGB encoding).
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            let target = RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    ..Default::default()
                },
            )?;
            let view = target.texture.create_view(&Default::default());
            let heightmap = self.displacement.0.create_view(&Default::default());
            if let NodeKind::Mesh(m) = &mut self.post.get_mut(self.quad)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                Arc::make_mut(&mut m.program).rebind(
                    r,
                    &[],
                    &[(&view, &self.sampler), (&heightmap, &self.displacement.1)],
                )?;
            }
            self.target = Some(target);
        }
        let target = self
            .target
            .as_ref()
            .ok_or(Error::Invalid("glitch target"))?;
        r.render(s, c, target)?;
        self.uniforms[2][1] = out.height as f32;
        if let NodeKind::Mesh(m) = &mut self.post.get_mut(self.quad)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0..3].copy_from_slice(&self.uniforms);
        }
        r.render(&mut self.post, self.post_camera, out)?;
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
    /// The page's "Glitch me wild" checkbox.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("glitch parameter"));
        }
        self.wild = value > 0.5;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
