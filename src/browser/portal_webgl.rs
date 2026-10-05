//! webgl_portal: two portals in the box room, each a plane showing the room
//! as seen through the other one. Each animation frame renders the left
//! portal's 256² sRGB target from the camera reflected through the left
//! portal and placed behind the right one, framed by the right portal's
//! corners ( CameraUtils.frameCorners ). The right portal's view is then
//! rendered the same way, showing the left target just rendered, while the
//! left target showed the right target of the previous frame. Both bouncing
//! icospheres are clipped at z = 0. The portal renders write sRGB without
//! tone mapping; the main render applies ACES Filmic.
use super::controls_attributes::{Controls, camera_state};
use super::mirror_webgl::gpu;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, float, uv, vec2};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// Animation frames due before the next render ( the page's animate() ).
    steps: u32,
    spheres: [Object3D; 2],
    /// The left and right portals: node, world matrix, target.
    portals: [(Object3D, Matrix4); 2],
    targets: [RenderTarget; 2],
    camera: Object3D,
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
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 75., 160.);
        let mut controls = Controls::new(None, (10., 400.), PI, true);
        controls.set_target(Vector3::new(0., 40., 0.));
        controls.update(s, c)?;
        s.background = Color::BLACK;
        let plane = Arc::new(PlaneGeometry::build(100.1, 100.1, 1, 1)?);
        let phong = |color: u32| {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(color);
            Arc::new(Material::Phong(m))
        };
        // The bouncing icospheres, clipped by the plane z = 0.
        let mut sphere = MeshPhongMaterial::default();
        sphere.properties.color = Color::WHITE;
        sphere.emissive = Color::from_hex(0x333333);
        sphere.properties.flat_shading = true;
        sphere.properties.clipping_planes = vec![Plane {
            normal: Vector3::Z,
            constant: 0.,
        }];
        sphere.properties.clip_shadows = true;
        let sphere = Arc::new(Material::Phong(sphere));
        let geometry = Arc::new(IcosahedronGeometry::build(5., 0)?);
        let spheres = [
            s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), sphere.clone()))),
            s.insert(NodeKind::Mesh(Mesh::new(geometry, sphere))),
        ];
        // The portals' targets: WebGLRenderTarget( 256, 256 ) with the
        // output color space, so rendering encodes sRGB and sampling decodes.
        let target = || {
            RenderTarget::with_options(
                &r.device,
                256,
                256,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    depth_buffer: true,
                    ..Default::default()
                },
            )
        };
        let targets = [target()?, target()?];
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut portals = vec![];
        for (x, t) in [(-30., &targets[0]), (30., &targets[1])] {
            // MeshBasicMaterial with the target as map ( sRGB, decoded by its
            // format ). Render targets are stored top row first: sample
            // ( u, 1 − v ) as WebGL's bottom-up render target texture.
            let view = t.texture.create_view(&Default::default());
            let program = ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(
                    crate::tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y())),
                )
                .wgsl(1)?,
                &[],
                &[(&view, &sampler)],
                PROJECTION,
            )
            .await?;
            let node = s.insert(NodeKind::Mesh(Mesh::new(
                plane.clone(),
                Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
            )));
            let n = s.get_mut(node)?;
            n.position = Vector3::new(x, 20., 0.);
            n.scale = Vector3::splat(0.35);
            portals.push((
                node,
                Matrix4::from_scale_rotation_translation(
                    Vector3::splat(0.35),
                    Quaternion::IDENTITY,
                    Vector3::new(x, 20., 0.),
                ),
            ));
        }
        let portals: [(Object3D, Matrix4); 2] =
            portals.try_into().map_err(|_| Error::Invalid("portals"))?;
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
            (
                0x7f7fff,
                Vector3::new(0., 50., 50.),
                Quaternion::from_rotation_y(PI),
            ),
            (0xff7fff, Vector3::new(0., 50., -50.), Quaternion::IDENTITY),
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
            let node = s.insert(NodeKind::Mesh(Mesh::new(plane.clone(), phong(color))));
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
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 500.,
            aspect: 1.,
            ..Default::default()
        })));
        Ok(Self {
            controls,
            time: 0.,
            steps: 1,
            spheres,
            portals,
            targets,
            camera,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// The spheres follow Date.now() * 0.01, the second π × 10 ahead.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let one = self.time * 10.;
        for (sphere, timer) in self.spheres.iter().zip([one, one + PI * 10.]) {
            let n = s.get_mut(*sphere)?;
            n.position = Vector3::new(
                (timer * 0.1).cos() * 30.,
                (timer * 0.2).cos().abs() * 20. + 5.,
                (timer * 0.1).sin() * 30.,
            );
            n.quaternion = Euler {
                angles: Vector3::new(0., PI / 2. - timer * 0.1, timer * 0.8),
                order: EulerOrder::XYZ,
            }
            .quaternion();
        }
        self.controls.update(s, c)
    }
    /// renderPortal(): the camera reflected through this portal into the
    /// other, framed by the other portal's corners, rendered into this
    /// portal's target with this portal hidden.
    fn render_portal(&self, r: &Renderer, s: &mut Scene, eye: Vector3, this: usize) -> Result<()> {
        let (node, model) = self.portals[this];
        let other = self.portals[1 - this].1;
        let mut local = model.inverse().transform_point3(eye);
        local.x *= -1.;
        local.z *= -1.;
        let position = other.transform_point3(local);
        // CameraUtils.frameCorners( camera, bottomLeft, bottomRight, topLeft, false ).
        let pa = other.transform_point3(Vector3::new(50.05, -50.05, 0.));
        let pb = other.transform_point3(Vector3::new(-50.05, -50.05, 0.));
        let pc = other.transform_point3(Vector3::new(50.05, 50.05, 0.));
        let vr = (pb - pa).normalize();
        let vu = (pc - pa).normalize();
        let vn = vr.cross(vu).normalize();
        let (va, vb, vc) = (pa - position, pb - position, pc - position);
        let d = -va.dot(vn);
        let (n, f) = (0.1, 500.);
        let l = vr.dot(va) * n / d;
        let rt = vr.dot(vb) * n / d;
        let b = vu.dot(va) * n / d;
        let t = vu.dot(vc) * n / d;
        let projection = Matrix4::from_cols_array(&[
            2. * n / (rt - l),
            0.,
            0.,
            0.,
            0.,
            2. * n / (t - b),
            0.,
            0.,
            (rt + l) / (rt - l),
            (t + b) / (t - b),
            (f + n) / (n - f),
            -1.,
            0.,
            0.,
            2. * f * n / (n - f),
            0.,
        ]);
        let camera = s.get_mut(self.camera)?;
        camera.position = position;
        camera.quaternion = Quaternion::from_mat3(&glam::DMat3::from_cols(vr, vu, vn));
        camera.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: n,
            far: f,
            aspect: 1.,
            projection_override: Some(gpu(projection)),
            ..Default::default()
        }));
        s.get_mut(node)?.visible = false;
        s.update()?;
        r.render(s, self.camera, &self.targets[this])?;
        s.get_mut(node)?.visible = true;
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        s.update()?;
        let eye = s.camera(c)?.1.w_axis.truncate();
        // A re-render without an animation frame draws the main view only:
        // the page renders its portals in animate().
        if std::mem::take(&mut self.steps) > 0 {
            s.tone_mapping = ToneMapping::None;
            self.render_portal(r, s, eye, 0)?;
            self.render_portal(r, s, eye, 1)?;
        }
        s.tone_mapping = ToneMapping::Aces;
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
        Err(Error::Invalid("portal parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.steps += 1;
    }
}
