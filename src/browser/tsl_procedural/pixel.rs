use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
use crate::postprocessing::Effect;
pub(in crate::browser) struct Pixel {
    viewer: OrbitViewer,
    time: f64,
    crystal: Object3D,
    target: RenderTarget,
    effect: Effect,
    sampler: wgpu::Sampler,
    parameters: [f32; 4],
    zoom: f64,
}
fn target(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: HDR,
            count: 2,
            ..Default::default()
        },
    )
}
impl Pixel {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 0.1,
            far: 10.,
            ..Default::default()
        }));
        let p = Vector3::new(0., 2. * (std::f64::consts::PI / 6.).tan(), 2.);
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, p.length());
        viewer.fixture(0., (p.y / p.length()).asin(), 1.8);
        s.background = Color::from_hex(0x151729);
        s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
        s.shadow_map_size = 2048;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x757f8e),
            intensity: 3.,
        }));
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::from_hex(0xfffecd),
            intensity: 1.5,
        }));
        {
            let n = s.get_mut(sun)?;
            n.position = Vector3::splat(100.);
            n.cast_shadow = true;
            n.shadow.far = 10.;
            n.shadow.filter = crate::shadow::ShadowFilter::Basic;
        }
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::from_hex(0xffc100),
            intensity: 10.,
            distance: 10.,
            angle: std::f64::consts::PI / 16.,
            penumbra: 0.02,
            decay: 2.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(spot)?;
            n.position = Vector3::new(2., 2., 0.);
            n.shadow.map_size = Some(512);
            n.cast_shadow = true;
            n.shadow.far = 10.;
            n.shadow.filter = crate::shadow::ShadowFilter::Basic;
        }
        let program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &[output(), normal_view()], &[], &[])
                .await?,
        );
        let shadow = Arc::new(shadow_program(r, None, None).await?);
        let mut checker =
            decode_texture_image(&fetch("/web/gallery/assets/tsl-procedural/checker.png").await?)
                .await?;
        checker.srgb = true;
        checker.min_filter = Some(Filter::Nearest);
        checker.filter = Filter::Nearest;
        checker.mipmap_filter = None;
        checker.wrap_s = Wrapping::Repeat;
        checker.wrap_t = Wrapping::Repeat;
        for (size, x, z, repeat) in [(0.4, 0., 0., 1.5), (0.5, -0.5, -0.5, 1.5), (2., 0., 0., 3.)] {
            let plane = size == 2.;
            let mut tex = checker.clone();
            tex.repeat = Vector2::splat(repeat);
            let mut m = MeshPhongMaterial::default();
            m.properties.map = Some(Arc::new(tex));
            m.properties.vertex_program = Some(program.clone());
            m.properties.shadow_program = Some(shadow.clone());
            let g = if plane {
                PlaneGeometry::build(size, size, 1, 1)?
            } else {
                BoxGeometry::build(size, size, size)?
            };
            let h = mesh(s, Arc::new(g), Material::Phong(m));
            let n = s.get_mut(h)?;
            n.receive_shadow = true;
            n.cast_shadow = !plane;
            n.position = Vector3::new(x, if plane { 0. } else { size / 2. + 0.0001 }, z);
            n.quaternion = if plane {
                Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2)
            } else {
                Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_4)
            };
        }
        let mut m = MeshPhongMaterial {
            shininess: 10.,
            specular: Color::WHITE,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x68b7e9);
        m.properties.vertex_program = Some(program);
        m.properties.shadow_program = Some(shadow);
        m.emissive = Color::from_hex(0x4f7e8b);
        let crystal = mesh(
            s,
            Arc::new(IcosahedronGeometry::build(0.2, 0)?),
            Material::Phong(m),
        );
        s.get_mut(crystal)?.cast_shadow = true;
        s.get_mut(crystal)?.receive_shadow = true;
        let target = target(r, 1, 1)?;
        let sampler = r.device.create_sampler(&Default::default());
        let color = tsl::pixelation::pixelation(
            tsl::Texture::External(0),
            uniform(0, Type::Vec2),
            uniform(1, Type::Float),
            uniform(2, Type::Float),
        );
        let effect = depth_effect_with_textures(
            r,
            HDR,
            &color,
            target.depth_view.as_ref().unwrap(),
            &[(&target.views[1], &sampler)],
        )
        .await?;
        Ok(Self {
            viewer,
            time: 0.,
            crystal,
            target,
            effect,
            sampler,
            parameters: [6., 0.3, 0.4, 1.],
            zoom: 1.,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 4 || !v.is_finite() {
            return Err(Error::Invalid("pixelation parameter"));
        }
        self.parameters[i] = match i {
            0 => v.round().clamp(1., 16.),
            1 => v.clamp(0., 2.),
            _ => v.clamp(0., 1.),
        };
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer.update(s, c)?;
        let n = s.get_mut(self.crystal)?;
        n.position.y = 0.7 + (self.time * 2.).sin() * 0.05;
        let cycle = (self.time / 4.).floor();
        let x = ((self.time - cycle * 4. - 2.) / 2.).clamp(0., 1.);
        n.quaternion =
            Quaternion::from_rotation_y((cycle + x * x * (3. - 2. * x)) * std::f64::consts::TAU);
        if let NodeKind::Mesh(m) = &mut n.kind
            && let Material::Phong(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.emissive = Color(Color::from_hex(0x4f7e8b).0 * ((self.time * 3.).sin() * 0.5 + 0.5));
        }
        Ok(())
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
            let q = s.get(c)?.quaternion;
            self.viewer.pan_world(
                (q * Vector3::X * (-dx) + q * Vector3::Y * dy) * (2. / self.zoom / h.max(1.)),
            );
        } else {
            self.viewer.orbit_pixels(dx, dy, 0., h, 0., f64::INFINITY);
            self.zoom = (self.zoom * 0.95_f64.powf(w * 0.01)).clamp(0.0001, 2.);
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        o: &RenderTarget,
    ) -> Result<bool> {
        let aspect = o.width as f64 / o.height as f64;
        let w = (o.width / self.parameters[0] as u32).max(1);
        let h = (o.height / self.parameters[0] as u32).max(1);
        let n = s.get_mut(c)?;
        let q = n.quaternion;
        let position = n.position;
        let Camera::Orthographic(p) = (match &mut n.kind {
            NodeKind::Camera(p) => p,
            _ => return Err(Error::Invalid("pixel camera")),
        }) else {
            return Err(Error::Invalid("pixel camera"));
        };
        p.zoom = self.zoom;
        p.left = -aspect;
        p.right = aspect;
        p.top = 1.;
        p.bottom = -1.;
        if self.parameters[3] > 0.5 {
            let px = 2. * aspect / self.zoom / w as f64;
            let py = 2. / self.zoom / h as f64;
            let x = position.dot(q * Vector3::X) / px;
            let y = position.dot(q * Vector3::Y) / py;
            let dx = (x - (x + 0.5).floor()) * px;
            let dy = (y - (y + 0.5).floor()) * py;
            p.left -= dx;
            p.right -= dx;
            p.top -= dy;
            p.bottom -= dy;
        }
        if self.target.width != w || self.target.height != h {
            self.target = target(r, w, h)?;
            self.effect.set_depth_and_textures(
                r,
                self.target.depth_view.as_ref().unwrap(),
                &[(&self.target.views[1], &self.sampler)],
            )?;
        }
        self.effect.parameters[0] = [1. / w as f32, 1. / h as f32, 0., 0.];
        self.effect.parameters[1][0] = self.parameters[1];
        self.effect.parameters[2][0] = self.parameters[2];
        r.render(s, c, &self.target)?;
        self.effect.apply(r, &self.target, None, o)?;
        Ok(true)
    }
}
