//! webgl_postprocessing_sao: 120 instanced standard spheres through
//! EffectComposer's RenderPass, SAOPass (view normals and depth, the
//! seven-sample spiral, the depth-limited vertical and horizontal blurs and
//! the multiplied composite, or the SAO and normal outputs) and OutputPass.
use super::ssao::{COPY, FULLSCREEN, NORMAL, PACKING, Pass, Random, TEXEL, half, pass, target};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// SAOShader: u.custom[8] (size, cameraNear, cameraFar), [9] (scale,
/// intensity, bias, kernelRadius), [10].x minResolution, [0..4] and
/// [4..8] the WebGL projection and its inverse. Textures: normals, depth.
/// randomSeed is 0; `rand` is common.glsl's, with `mod( dt, PI )`. Depth is
/// read at the 24-bit precision of the pass's UnsignedInt248 depth texture.
/// The spiral's rand() takes the exact pixel-center uv, as WebGL's
/// interpolated vUv is.
const SAO: &str = "fn sao_depth(p:vec2<i32>)->f32{return round(textureLoad(tsl_texture_1,p,0)*16777215.0)/16777215.0;}fn sao_rand(uv:vec2<f32>)->f32{let dt=dot(uv,vec2(12.9898,78.233));let pi=3.141592653589793;let sn=dt-pi*floor(dt/pi);return fract(sin(sn)*43758.5453);}fn sao_view_position(uv:vec2<f32>,depth:f32,view_z:f32)->vec3<f32>{let projection=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let inverse=mat4x4<f32>(u.custom[4],u.custom[5],u.custom[6],u.custom[7]);let clip_w=projection[2][3]*view_z+projection[3][3];let clip=vec4((vec3(uv,depth)-0.5)*2.0,1.0)*clip_w;return (inverse*clip).xyz;}fn sao()->vec4<f32>{let uv=fragment_surface.uv;let size=u.custom[8].xy;let near=u.custom[8].z;let far=u.custom[8].w;let scale=u.custom[9].x;let intensity=u.custom[9].y;let bias=u.custom[9].z;let kernel_radius=u.custom[9].w;let min_resolution=u.custom[10].x;let dims=textureDimensions(tsl_texture_1);let center_depth=sao_depth(gl_texel(dims,uv));if center_depth>=1.0-1e-6 {discard;}let view_position=sao_view_position(uv,center_depth,perspective_depth_to_view_z(center_depth,near,far));let normal=2.0*textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),uv),0).xyz-1.0;let scale_far=scale/far;let resolution_far=min_resolution*far;let pi2=6.283185307179586;var angle=sao_rand(vec2(fragment_surface.clip.x,size.y-fragment_surface.clip.y)/size)*pi2;var radius=vec2(kernel_radius*(1.0/7.0))/size;let step=radius;var occlusion=0.0;var weight=0.0;for(var i=0;i<7;i++){let sample_uv=uv+vec2(cos(angle),sin(angle))*radius;radius+=step;angle+=pi2*4.0/7.0;let sample_depth=sao_depth(gl_texel(dims,sample_uv));if sample_depth>=1.0-1e-6 {continue;}let sample_position=sao_view_position(sample_uv,sample_depth,perspective_depth_to_view_z(sample_depth,near,far));let delta=sample_position-view_position;let distance=length(delta);let scaled=scale_far*distance;occlusion+=max(0.0,(dot(normal,delta)-resolution_far)/scaled-bias)/(1.0+scaled*scaled);weight+=1.0;}if weight==0.0 {discard;}let ao=occlusion*(intensity/weight);return vec4(vec3(1.0-ao),1.0);}";
/// DepthLimitedBlurShader with BlurShaderUtils' gaussian weights:
/// u.custom[8] (size, near, far), [10].w depthCutoff, [11] (direction,
/// kernel radius, stdDev). Textures: the source, the depth.
const BLUR: &str = "fn sao_depth(p:vec2<i32>)->f32{return round(textureLoad(tsl_texture_1,p,0)*16777215.0)/16777215.0;}fn sao_weight(x:f32,std_dev:f32)->f32{return exp(-(x*x)/(2.0*(std_dev*std_dev)))/(sqrt(2.0*3.141592653589793)*std_dev);}fn sao_blur()->vec4<f32>{let uv=fragment_surface.uv;let near=u.custom[8].z;let far=u.custom[8].w;let inv=1.0/u.custom[8].xy;let cutoff=u.custom[10].w;let direction=u.custom[11].xy;let radius=i32(u.custom[11].z);let std_dev=u.custom[11].w;let dims=textureDimensions(tsl_texture_1);let source=textureDimensions(tsl_texture_0);let depth=sao_depth(gl_texel(dims,uv));if depth>=1.0-1e-6 {discard;}let center=-perspective_depth_to_view_z(depth,near,far);var r_break=false;var l_break=false;var weight_sum=sao_weight(0.0,std_dev);var sum=textureLoad(tsl_texture_0,gl_texel(source,uv),0)*weight_sum;for(var i=1;i<=radius;i++){let w=sao_weight(f32(i),std_dev);let offset=direction*f32(i)*inv;var sample_uv=uv+offset;var view_z=-perspective_depth_to_view_z(sao_depth(gl_texel(dims,sample_uv)),near,far);if abs(view_z-center)>cutoff {r_break=true;}if !r_break {sum+=textureLoad(tsl_texture_0,gl_texel(source,sample_uv),0)*w;weight_sum+=w;}sample_uv=uv-offset;view_z=-perspective_depth_to_view_z(sao_depth(gl_texel(dims,sample_uv)),near,far);if abs(view_z-center)>cutoff {l_break=true;}if !l_break {sum+=textureLoad(tsl_texture_0,gl_texel(source,sample_uv),0)*w;weight_sum+=w;}}return sum/weight_sum;}";
struct Targets {
    read: RenderTarget,
    normal: RenderTarget,
    sao: RenderTarget,
    blur: RenderTarget,
}
pub(super) struct Demo {
    group: Object3D,
    normal_scene: Scene,
    normal_group: Object3D,
    normal_camera: Object3D,
    time: f64,
    targets: Option<Targets>,
    sao: Pass,
    /// The vertical blur (into the intermediate target) and the horizontal
    /// blur (back into the SAO target).
    blurs: [Pass; 2],
    /// CopyShader: SAO multiplied (Default), SAO replaced, normals replaced.
    copies: [Pass; 3],
    output: Pass,
    nearest: wgpu::Sampler,
    comparison: wgpu::Sampler,
    /// output, saoBias, saoIntensity, saoScale, saoKernelRadius,
    /// saoMinResolution, saoBlur, saoBlurRadius, saoBlurStdDev,
    /// saoBlurDepthCutoff, enabled.
    params: [f64; 11],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let camera = Camera::Perspective(PerspectiveCamera {
            fov: 65.,
            near: 3.,
            far: 10.,
            aspect,
            ..Default::default()
        });
        s.get_mut(c)?.kind = NodeKind::Camera(camera.clone());
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 7.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        for (hex, x, y) in [
            (0xefffef, -10., -10.),
            (0xffefef, -10., 10.),
            (0xefefff, 10., -10.),
        ] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(hex),
                intensity: 500.,
                distance: 0.,
                decay: 2.,
            }));
            s.get_mut(light)?.position = Vector3::new(x, y, 10.);
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.2,
        }));
        let mut random = Random(186);
        let mut instances = vec![];
        for _ in 0..120 {
            let position = Vector3::new(
                random.next() * 4. - 2.,
                random.next() * 4. - 2.,
                random.next() * 4. - 2.,
            );
            let rotation = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                random.next(),
                random.next(),
                random.next(),
            );
            let scale = random.next() * 0.2 + 0.05;
            let color = Color::from_hsl(random.next(), 1., 0.3);
            instances.push(Instance {
                matrix: Matrix4::from_scale_rotation_translation(
                    Vector3::splat(scale),
                    rotation,
                    position,
                ),
                color,
            });
        }
        let geometry = Arc::new(SphereGeometry::build(3., 48, 24)?);
        let group = s.insert(NodeKind::Group);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Standard(MeshStandardMaterial {
                energy_conservation: true,
                roughness: 0.5,
                metalness: 0.,
                ..Default::default()
            })),
        )));
        let n = s.get_mut(mesh)?;
        n.instances = instances.clone();
        n.frustum_culled = false;
        s.add(group, mesh)?;
        // The override render: MeshNormalMaterial over the 0x7777ff clear
        // (the scene has no background to replace it).
        let mut normal_scene = Scene::new();
        normal_scene.background = Color::from_hex(0x7777ff);
        let normal_camera = normal_scene.insert(NodeKind::Camera(camera));
        normal_scene.get_mut(normal_camera)?.position = Vector3::new(0., 0., 7.);
        let node = WgslFn::new("ssao_normal", NORMAL, &[], Type::Vec4)?.call(&[]);
        let normal_material = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(node).wgsl(0)?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        ));
        let normal_group = normal_scene.insert(NodeKind::Group);
        let normal_mesh = normal_scene.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Shader(normal_material)),
        )));
        let n = normal_scene.get_mut(normal_mesh)?;
        n.instances = instances;
        n.frustum_culled = false;
        normal_scene.add(normal_group, normal_mesh)?;
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let initial = target(r, 1, 1, true)?;
        let color = initial.texture.create_view(&Default::default());
        let depth_view = initial
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("sao depth view"))?;
        let depth = || {
            (
                depth_view,
                &comparison,
                Type::DepthTexture,
                wgpu::TextureSampleType::Depth,
            )
        };
        let mut sao = pass(
            r,
            "sao",
            &format!("{TEXEL}{PACKING}{SAO}"),
            &[(&color, &nearest, Type::Texture, half()), depth()],
        )
        .await?;
        // The SAO and blur targets are cleared white; discarded fragments keep it.
        sao.scene.background = Color::WHITE;
        let mut blurs = vec![];
        for _ in 0..2 {
            let mut blur = pass(
                r,
                "sao_blur",
                &format!("{TEXEL}{PACKING}{BLUR}"),
                &[(&color, &nearest, Type::Texture, half()), depth()],
            )
            .await?;
            blur.scene.background = Color::WHITE;
            blurs.push(blur);
        }
        let mut copies = vec![];
        for multiply in [true, false, false] {
            let mut copy = pass(
                r,
                "ssao_copy",
                &format!("{TEXEL}{COPY}"),
                &[(&color, &nearest, Type::Texture, half())],
            )
            .await?;
            if multiply {
                // Default: CustomBlending ( DstColor, Zero ), alpha ( DstAlpha, Zero ).
                let m = copy.material()?;
                m.properties.transparent = true;
                m.properties.blending = Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::Dst,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::DstAlpha,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                });
            }
            copies.push(copy);
        }
        let output = pass(
            r,
            "ssao_copy",
            &format!("{TEXEL}{COPY}"),
            &[(&color, &nearest, Type::Texture, half())],
        )
        .await?;
        let _ = FULLSCREEN;
        Ok(Self {
            group,
            normal_scene,
            normal_group,
            normal_camera,
            time: 0.,
            targets: None,
            sao,
            blurs: blurs.try_into().map_err(|_| Error::Invalid("sao blurs"))?,
            copies: copies
                .try_into()
                .map_err(|_| Error::Invalid("sao copies"))?,
            output,
            nearest,
            comparison,
            params: [0., 0.5, 0.18, 1., 100., 0., 1., 8., 4., 0.01, 1.],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the group turns with performance.now().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let timer = self.time * 1000.;
        let q = Quaternion::from_euler(glam::EulerRot::XYZ, timer * 0.0002, timer * 0.0001, 0.);
        s.get_mut(self.group)?.quaternion = q;
        self.normal_scene.get_mut(self.normal_group)?.quaternion = q;
        let kind = s.get(c)?.kind.clone();
        self.normal_scene.get_mut(self.normal_camera)?.kind = kind;
        Ok(())
    }
    /// composer.setSize: every pass rebound to the targets at the output size.
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let targets = Targets {
            read: target(r, w, h, true)?,
            normal: target(r, w, h, true)?,
            sao: target(r, w, h, false)?,
            blur: target(r, w, h, false)?,
        };
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        let depth = targets
            .normal
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("sao depth view"))?;
        let (normal, sao, blur, read) = (
            view(&targets.normal),
            view(&targets.sao),
            view(&targets.blur),
            view(&targets.read),
        );
        let rebind =
            |p: &mut Pass, textures: &[(&wgpu::TextureView, &wgpu::Sampler)]| -> Result<()> {
                Arc::make_mut(&mut p.material()?.program).rebind(r, &[], textures)
            };
        rebind(
            &mut self.sao,
            &[(&normal, &self.nearest), (depth, &self.comparison)],
        )?;
        rebind(
            &mut self.blurs[0],
            &[(&sao, &self.nearest), (depth, &self.comparison)],
        )?;
        rebind(
            &mut self.blurs[1],
            &[(&blur, &self.nearest), (depth, &self.comparison)],
        )?;
        for (copy, source) in self.copies.iter_mut().zip([&sao, &sao, &normal]) {
            rebind(copy, &[(source, &self.nearest)])?;
        }
        rebind(&mut self.output, &[(&read, &self.nearest)])?;
        self.targets = Some(targets);
        Ok(())
    }
    /// composer.render(): RenderPass, SAOPass (when enabled), OutputPass.
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
            .is_none_or(|t| t.read.width != out.width || t.read.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let Some(mut targets) = self.targets.take() else {
            return Err(Error::Invalid("sao targets"));
        };
        let result = self.passes(r, s, c, out, &mut targets);
        self.targets = Some(targets);
        result?;
        Ok(true)
    }
    fn passes(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        t: &mut Targets,
    ) -> Result<()> {
        r.render(s, c, &t.read)?;
        let [
            output,
            bias,
            intensity,
            scale,
            kernel_radius,
            min_resolution,
            blur,
            blur_radius,
            std_dev,
            depth_cutoff,
            enabled,
        ] = self.params;
        if enabled > 0.5 {
            r.render(&mut self.normal_scene, self.normal_camera, &t.normal)?;
            let (near, far, projection) = match s.camera(c)?.0 {
                Camera::Perspective(p) => (
                    p.near,
                    p.far,
                    super::controls_attributes::webgl_perspective(p.fov, p.aspect, p.near, p.far),
                ),
                _ => return Err(Error::Invalid("sao camera")),
            };
            let columns = |m: Matrix4| m.to_cols_array_2d().map(|c| c.map(|v| v as f32));
            let mut uniforms = [[0f32; 4]; 16];
            uniforms[0..4].copy_from_slice(&columns(projection));
            uniforms[4..8].copy_from_slice(&columns(projection.inverse()));
            uniforms[8] = [out.width as f32, out.height as f32, near as f32, far as f32];
            uniforms[9] = [
                scale as f32,
                intensity as f32,
                bias as f32,
                kernel_radius as f32,
            ];
            // saoBlurRadius is floored; the cutoff scales by far − near.
            uniforms[10] = [
                min_resolution as f32,
                0.,
                0.,
                (depth_cutoff * (far - near)) as f32,
            ];
            self.sao.render(r, &t.sao, &uniforms)?;
            if blur > 0.5 {
                let radius = blur_radius.floor() as f32;
                uniforms[11] = [0., 1., radius, std_dev as f32];
                self.blurs[0].render(r, &t.blur, &uniforms)?;
                uniforms[11] = [1., 0., radius, std_dev as f32];
                self.blurs[1].render(r, &t.sao, &uniforms)?;
            }
            t.read.set_load_color(true);
            let result = match output.round() as u32 {
                1 => self.copies[1].render(r, &t.read, &uniforms),
                2 => self.copies[2].render(r, &t.read, &uniforms),
                _ => self.copies[0].render(r, &t.read, &uniforms),
            };
            t.read.set_load_color(false);
            result?;
        }
        self.output.render(r, out, &[[0.; 4]; 16])
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
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("sao parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
