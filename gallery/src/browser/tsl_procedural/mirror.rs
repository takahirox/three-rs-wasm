use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
use crate::shader::ShaderProgram;
pub(in crate::browser) struct Mirror {
    viewer: OrbitViewer,
    time: f64,
    group: Object3D,
    small: Object3D,
    planes: [Object3D; 2],
    cameras: [Object3D; 4],
    targets: Vec<RenderTarget>,
    views: Vec<wgpu::TextureView>,
    programs: Vec<[Arc<ShaderProgram>; 2]>,
    textures: Vec<GpuTexture>,
    sampler: wgpu::Sampler,
    has_output: [bool; 4],
}
impl Mirror {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            aspect,
            near: 1.,
            far: 500.,
            ..Default::default()
        }));
        let center = Vector3::new(0., 40., 0.);
        let offset = Vector3::new(0., 75., 160.) - center;
        let mut viewer = OrbitViewer::from_camera(center, offset.length());
        viewer.fixture(0., (offset.y / offset.length()).asin(), 1.8);
        s.background = Color::BLACK;
        s.tone_mapping = ToneMapping::None;
        let group = s.insert(NodeKind::Group);
        let mut material = MeshPhongMaterial::default();
        material.properties.color = Color::WHITE;
        material.emissive = Color::from_hex(0x8d8d8d);
        let half = mesh(
            s,
            Arc::new(SphereGeometry::with_angles(
                15.,
                24,
                24,
                std::f64::consts::FRAC_PI_2,
                std::f64::consts::TAU,
                0.,
                120_f64.to_radians(),
            )?),
            Material::Phong(material.clone()),
        );
        {
            let n = s.get_mut(half)?;
            n.position.y = 15.;
            n.quaternion = Quaternion::from_rotation_x(-135_f64.to_radians())
                * Quaternion::from_rotation_z(-20_f64.to_radians());
        }
        s.add(group, half)?;
        let cap = mesh(
            s,
            Arc::new(CylinderGeometry::build(
                0.1,
                15. * 30_f64.to_radians().cos(),
                0.1,
                24,
                1,
                false,
                0.,
                std::f64::consts::TAU,
            )?),
            Material::Phong(material),
        );
        s.get_mut(cap)?.position.y = -7.55;
        s.get_mut(cap)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::PI);
        s.add(half, cap)?;
        let mut material = MeshPhongMaterial {
            emissive: Color::from_hex(0x7b7b7b),
            ..Default::default()
        };
        material.properties.flat_shading = true;
        let small = mesh(
            s,
            Arc::new(IcosahedronGeometry::build(5., 0)?),
            Material::Phong(material),
        );
        let mut textures = vec![];
        for (url, srgb, repeat) in [
            (
                "/web/gallery/assets/tsl-procedural/decal-diffuse.png",
                true,
                false,
            ),
            (
                "/web/gallery/assets/tsl-procedural/decal-normal.jpg",
                false,
                false,
            ),
            (
                "/web/gallery/assets/tsl-viewport/textures/floors/FloorsCheckerboard_S_Normal.jpg",
                false,
                true,
            ),
        ] {
            let mut t = decode_texture_image(&fetch(url).await?).await?;
            t.srgb = srgb;
            t.mipmap_filter = Some(Filter::Linear);
            if repeat {
                t.wrap_s = Wrapping::Repeat;
                t.wrap_t = Wrapping::Repeat;
            }
            textures.push(r.upload_texture(&Arc::new(t))?);
        }
        let mut targets = vec![];
        for _ in 0..4 {
            targets.push(RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format: HDR,
                    ..Default::default()
                },
            )?);
        }
        let views: Vec<_> = targets
            .iter()
            .map(|t| t.texture.create_view(&Default::default()))
            .collect();
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut planes = vec![];
        let mut programs = vec![];
        let plane_geo = Arc::new(PlaneGeometry::build(100.1, 100.1, 1, 1)?);
        for i in 0..2 {
            let coordinate = uv() * float(if i == 0 { 1. } else { 5. });
            let coordinate = vec2(coordinate.x(), float(1.) - coordinate.y());
            let sample = tsl::Texture::External(if i == 0 { 2 } else { 3 }).sample(coordinate);
            let offset = (sample.swizzle("xy") * float(2.) - float(1.))
                * float(if i == 0 { -0.08 } else { 0.1 });
            let screen = tsl::viewport::screen_uv();
            let reflected =
                tsl::Texture::External(0).sample(vec2(float(1.) - screen.x(), screen.y()) + offset);
            let color = if i == 0 {
                let alpha = tsl::Texture::External(1)
                    .sample(vec2(uv().x(), float(1.) - uv().y()))
                    .swizzle("w");
                mix(rgb(0xffffff), reflected.rgb(), alpha)
            } else {
                rgb(0x0000ff) * float(0.1) + reflected.rgb()
            };
            let source = SurfaceNodes {
                color: Some(color),
                ..Default::default()
            };
            let program = source
                .build(
                    r,
                    &[],
                    &[
                        (&views[i], &sampler),
                        (&textures[0].view, &textures[0].sampler),
                        (&textures[1].view, &textures[1].sampler),
                        (&textures[2].view, &textures[2].sampler),
                    ],
                )
                .await?;
            let mut nested = program.clone();
            nested.rebind(
                r,
                &[],
                &[
                    (&views[i + 2], &sampler),
                    (&textures[0].view, &textures[0].sampler),
                    (&textures[1].view, &textures[1].sampler),
                    (&textures[2].view, &textures[2].sampler),
                ],
            )?;
            let programs_pair = [Arc::new(program), Arc::new(nested)];
            let mut material = MeshPhongMaterial::default();
            material.properties.vertex_program = Some(programs_pair[0].clone());
            let h = mesh(s, plane_geo.clone(), Material::Phong(material));
            if i == 0 {
                s.get_mut(h)?.quaternion =
                    Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
            } else {
                s.get_mut(h)?.position = Vector3::new(0., 50., -50.);
            }
            planes.push(h);
            programs.push(programs_pair);
        }
        for (color, p, q) in [
            (
                0xffffff,
                Vector3::new(0., 100., 0.),
                Quaternion::from_rotation_x(std::f64::consts::FRAC_PI_2),
            ),
            (
                0x7f7fff,
                Vector3::new(0., 50., 50.),
                Quaternion::from_rotation_y(std::f64::consts::PI),
            ),
            (
                0x00ff00,
                Vector3::new(50., 50., 0.),
                Quaternion::from_rotation_y(-std::f64::consts::FRAC_PI_2),
            ),
            (
                0xff0000,
                Vector3::new(-50., 50., 0.),
                Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2),
            ),
        ] {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(color);
            let h = mesh(s, plane_geo.clone(), Material::Phong(m));
            let n = s.get_mut(h)?;
            n.position = p;
            n.quaternion = q;
        }
        for (color, intensity, distance, p) in [
            (0xe7e7e7, 2.5, 250., Vector3::new(0., 60., 0.)),
            (0x00ff00, 0.5, 1000., Vector3::new(550., 50., 0.)),
            (0xff0000, 0.5, 1000., Vector3::new(-550., 50., 0.)),
            (0xbbbbfe, 0.5, 1000., Vector3::new(0., 50., 550.)),
        ] {
            let h = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance,
                decay: 0.,
            }));
            s.get_mut(h)?.position = p;
        }
        let mut cameras = vec![];
        for _ in 0..4 {
            let h = s.insert(s.get(c)?.kind.clone());
            s.get_mut(h)?.matrix_auto_update = false;
            cameras.push(h);
        }
        Ok(Self {
            viewer,
            time: 0.,
            group,
            small,
            planes: planes.try_into().unwrap(),
            cameras: cameras.try_into().unwrap(),
            targets,
            views,
            programs,
            textures,
            sampler,
            has_output: [false; 4],
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 1.;
            p.far = 500.;
        }
        s.get_mut(self.group)?.quaternion = Quaternion::from_rotation_y(-self.time * 0.12);
        let n = s.get_mut(self.small)?;
        n.position = Vector3::new(
            self.time.cos() * 30.,
            (self.time * 2.).cos().abs() * 20. + 5.,
            self.time.sin() * 30.,
        );
        n.quaternion = Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2 - self.time)
            * Quaternion::from_rotation_z(self.time * 8.);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    fn bind(&self, s: &mut Scene, i: usize, nested: bool) -> Result<()> {
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.planes[i])?.kind {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_program = Some(self.programs[i][usize::from(nested)].clone());
        }
        Ok(())
    }
    fn reflected(&self, s: &mut Scene, c: Object3D, i: usize, destination: usize) -> Result<bool> {
        s.update_world_matrix(c, true, false)?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(camera) = camera else {
            return Err(Error::Invalid("mirror camera"));
        };
        let plane = if i == 0 {
            Vector4::new(0., 1., 0., 0.)
        } else {
            Vector4::new(0., 0., 1., 50.)
        };
        if let Some((camera, matrix)) = crate::reflection::planar_camera(camera, world, plane)? {
            let n = s.get_mut(self.cameras[destination])?;
            n.kind = NodeKind::Camera(Camera::Perspective(camera));
            n.matrix = matrix;
            n.matrix_world_needs_update = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets[0].width != out.width || self.targets[0].height != out.height {
            for (i, t) in self.targets.iter_mut().enumerate() {
                t.set_size(&r.device, out.width, out.height)?;
                self.views[i] = t.texture.create_view(&Default::default());
            }
            for i in 0..2 {
                for j in 0..2 {
                    Arc::make_mut(&mut self.programs[i][j]).rebind(
                        r,
                        &[],
                        &[
                            (&self.views[i + j * 2], &self.sampler),
                            (&self.textures[0].view, &self.textures[0].sampler),
                            (&self.textures[1].view, &self.textures[1].sampler),
                            (&self.textures[2].view, &self.textures[2].sampler),
                        ],
                    )?;
                }
            }
            self.has_output = [false; 4];
        }
        for i in 0..2 {
            if !self.reflected(s, c, i, i)? {
                self.clear(r, i);
                continue;
            }
            let other = 1 - i;
            s.get_mut(self.planes[i])?.visible = false;
            let result = (|| -> Result<()> {
                if self.reflected(s, self.cameras[i], other, other + 2)? {
                    s.get_mut(self.planes[other])?.visible = false;
                    let result = r.render(s, self.cameras[other + 2], &self.targets[other + 2]);
                    s.get_mut(self.planes[other])?.visible = true;
                    result?;
                    self.has_output[other + 2] = true;
                } else {
                    self.clear(r, other + 2);
                }
                self.bind(s, other, true)?;
                let result = r.render(s, self.cameras[i], &self.targets[i]);
                self.bind(s, other, false)?;
                result?;
                self.has_output[i] = true;
                Ok(())
            })();
            s.get_mut(self.planes[i])?.visible = true;
            result?;
        }
        self.bind(s, 0, false)?;
        self.bind(s, 1, false)?;
        r.render(s, c, out)?;
        Ok(true)
    }
    fn clear(&mut self, r: &Renderer, i: usize) {
        if !self.has_output[i] {
            return;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear back-facing reflector"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.views[i],
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        r.queue.submit([encoder.finish()]);
        self.has_output[i] = false;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 10., 400.);
        }
        Ok(())
    }
}
