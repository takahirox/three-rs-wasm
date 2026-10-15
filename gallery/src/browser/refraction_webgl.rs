//! webgl_refraction: a Refractor plane ( WaterRefractionShader with the
//! scrolling waterdudv map ) in the middle of the box room, a bouncing
//! icosahedron and four point lights. In its onBeforeRender, unless seen
//! from behind, the Refractor renders the scene from the camera with its
//! near plane moved to the refractor ( an oblique projection culling what
//! lies in front ) into its 1024² 4× multisampled half-float target, hidden
//! itself; then it is drawn as a transparent object sampling that target
//! through its texture matrix, offset by the dudv distortion.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::mirror_webgl::{gl_perspective, gpu};
use super::retro::mipmapped;
use super::shadowmap_opacity::Mipmaps;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// WaterRefractionShader: u.custom[0..4] the texture matrix ( applied to the
/// object-space position ), [4].rgb the color, [4].w the time. Texture 0
/// the refraction ( stored from its top row ), 1 the dudv map ( flipped on
/// upload, as TextureLoader's flipY ).
const REFRACTION: &str = "fn refraction_overlay(base:f32,blend:f32)->f32{return select(1.0-2.0*(1.0-base)*(1.0-blend),2.0*base*blend,base<0.5);}fn refraction()->vec4<f32>{let wave_strength=0.5;let wave_speed=0.03;let time=u.custom[4].w;let v_uv=fragment_surface.uv;var distorted=textureSample(tsl_texture_1,tsl_sampler_1,vec2(v_uv.x+time*wave_speed,v_uv.y)).rg*wave_strength;distorted=v_uv+vec2(distorted.x,distorted.y+time*wave_speed);let distortion=(textureSample(tsl_texture_1,tsl_sampler_1,distorted).rg*2.0-1.0)*wave_strength;let m=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);var uv=m*vec4(fragment_surface.local_position,1.0);uv=vec4(uv.xy+distortion,uv.zw);let p=uv.xy/uv.w;let base=textureSample(tsl_texture_0,tsl_sampler_0,vec2(p.x,1.0-p.y));let c=u.custom[4].rgb;return vec4(refraction_overlay(base.r,c.r),refraction_overlay(base.g,c.g),refraction_overlay(base.b,c.b),1.0);}";
const PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    small: Object3D,
    refractor: Object3D,
    model: Matrix4,
    texture_matrix: Matrix4,
    /// The virtual camera and the refraction target.
    camera: Object3D,
    target: RenderTarget,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let perspective = PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 500.,
            aspect,
            ..Default::default()
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(perspective.clone()));
        s.get_mut(c)?.position = Vector3::new(0., 75., 160.);
        let mut controls = Controls::new(None, (10., 400.), PI, true);
        controls.set_target(Vector3::new(0., 40., 0.));
        controls.update(s, c)?;
        s.background = Color::BLACK;
        let target = RenderTarget::with_options(
            &r.device,
            1024,
            1024,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                depth_buffer: true,
                samples: 4,
                ..Default::default()
            },
        )?;
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // TextureLoader's dudv map: flipped ( flipY ), mipmapped, repeating.
        let mut image =
            decode_texture_image(&fetch("/web/gallery/assets/refraction/waterdudv.jpg").await?)
                .await?;
        image.srgb = false;
        let row = image.width as usize * 4;
        let image = crate::material::Texture {
            rgba: image.rgba.chunks(row).rev().flatten().copied().collect(),
            ..image
        };
        let mut mipmaps = Mipmaps::new(r);
        let dudv = mipmapped(r, &mut mipmaps, &image, wgpu::TextureFormat::Rgba8Unorm);
        let refraction = target.texture.create_view(&Default::default());
        let node = WgslFn::new("refraction", REFRACTION, &[], Type::Vec4)?.call(&[]);
        let wgsl = NodeMaterial::new(node)
            .wgsl_with_texture_types(&[Type::Texture, Type::Texture], &[])?;
        let filterable = wgpu::TextureSampleType::Float { filterable: true };
        let program = ShaderProgram::with_projection_and_sample_types(
            r,
            &wgsl,
            &[(&refraction, &linear), (&dudv, &repeat)],
            &[wgpu::TextureViewDimension::D2; 2],
            &[filterable, filterable],
            PROJECTION,
        )
        .await?;
        let mut material = ShaderMaterial::new(Arc::new(program));
        // transparent: true, so that refractors draw from farthest to closest.
        material.properties.transparent = true;
        let color = Color::from_hex(0xcbcbcb);
        material.uniforms[4] = [color.0.x as f32, color.0.y as f32, color.0.z as f32, 0.];
        let refractor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(90., 90., 1, 1)?),
            Arc::new(Material::Shader(material)),
        )));
        let position = Vector3::new(0., 50., 0.);
        s.get_mut(refractor)?.position = position;
        let phong = |color: u32, emissive: u32, flat: bool| {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(color);
            m.emissive = Color::from_hex(emissive);
            m.properties.flat_shading = flat;
            Arc::new(Material::Phong(m))
        };
        let small = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(IcosahedronGeometry::build(5., 0)?),
            phong(0xffffff, 0x333333, true),
        )));
        let plane = Arc::new(PlaneGeometry::build(100.1, 100.1, 1, 1)?);
        for (color, position, rotation) in [
            (
                0xffffff,
                Vector3::new(0., 100., 0.),
                Quaternion::from_rotation_x(PI / 2.),
            ),
            (
                0xffffff,
                Vector3::ZERO,
                Quaternion::from_rotation_x(-PI / 2.),
            ),
            (0x7f7fff, Vector3::new(0., 50., -50.), Quaternion::IDENTITY),
            (
                0x00ff00,
                Vector3::new(50., 50., 0.),
                Quaternion::from_rotation_y(-PI / 2.),
            ),
            (
                0xff0000,
                Vector3::new(-50., 50., 0.),
                Quaternion::from_rotation_y(PI / 2.),
            ),
        ] {
            let node = s.insert(NodeKind::Mesh(Mesh::new(
                plane.clone(),
                phong(color, 0, false),
            )));
            let n = s.get_mut(node)?;
            n.position = position;
            n.quaternion = rotation;
        }
        for (color, intensity, distance, position) in [
            (0xe7e7e7, 2.5, 250., Vector3::new(0., 60., 0.)),
            (0x00ff00, 0.5, 1000., Vector3::new(550., 50., 0.)),
            (0xff0000, 0.5, 1000., Vector3::new(-550., 50., 0.)),
            (0xbbbbfe, 0.5, 1000., Vector3::new(0., 50., 550.)),
        ] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance,
                decay: 0.,
            }));
            s.get_mut(light)?.position = position;
        }
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(perspective)));
        Ok(Self {
            controls,
            time: 0.,
            small,
            refractor,
            model: Matrix4::from_translation(position),
            texture_matrix: Matrix4::IDENTITY,
            camera,
            target,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the refractor's time and the icosahedron follow the timer.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let time = self.time;
        let n = s.get_mut(self.small)?;
        n.position = Vector3::new(
            time.cos() * 30.,
            (time * 2.).cos().abs() * 20. + 5.,
            time.sin() * 30.,
        );
        n.quaternion = Euler {
            angles: Vector3::new(0., PI / 2. - time, time * 8.),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        self.controls.update(s, c)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("refraction camera"));
        };
        let projection = gl_perspective(p.fov, p.aspect, p.near, p.far);
        let perspective = p.clone();
        let view = world.inverse();
        let position = self.model.w_axis.truncate();
        let normal = Vector3::Z;
        let screen = gpu(projection) * view;
        // onBeforeRender runs when the refractor is drawn ( in the frustum ) and
        // is not seen from behind.
        let drawn = Frustum::from_projection(screen).intersects_sphere(Sphere {
            center: position,
            radius: 45. * 2f64.sqrt(),
        });
        let facing = (position - world.w_axis.truncate()).dot(normal) < 0.;
        if drawn && facing {
            self.texture_matrix = Matrix4::from_cols_array(&[
                0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 0.5, 0., 0.5, 0.5, 0.5, 1.,
            ]) * projection
                * view
                * self.model;
            // The refractor plane, its normal flipped, in view space.
            let plane_normal = view.transform_vector3(-normal).normalize();
            let point = view.transform_point3(position);
            let mut clip = plane_normal.extend(-point.dot(plane_normal));
            let mut e = projection.to_cols_array();
            let q = Vector4::new(
                (clip.x.signum() + e[8]) / e[0],
                (clip.y.signum() + e[9]) / e[5],
                -1.,
                (1. + e[10]) / e[14],
            );
            clip *= 2. / clip.dot(q);
            e[2] = clip.x;
            e[6] = clip.y;
            e[10] = clip.z + 1.;
            e[14] = clip.w;
            let (_, rotation, translation) = world.to_scale_rotation_translation();
            let n = s.get_mut(self.camera)?;
            n.position = translation;
            n.quaternion = rotation;
            n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                projection_override: Some(gpu(Matrix4::from_cols_array(&e))),
                ..perspective
            }));
            s.get_mut(self.refractor)?.visible = false;
            s.update()?;
            r.render(s, self.camera, &self.target)?;
            s.get_mut(self.refractor)?.visible = true;
        }
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.refractor)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            let e = self.texture_matrix.to_cols_array();
            for k in 0..4 {
                m.uniforms[k] = [
                    e[k * 4] as f32,
                    e[k * 4 + 1] as f32,
                    e[k * 4 + 2] as f32,
                    e[k * 4 + 3] as f32,
                ];
            }
            m.uniforms[4][3] = self.time as f32;
        }
        r.render(s, c, out)?;
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("refraction parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
