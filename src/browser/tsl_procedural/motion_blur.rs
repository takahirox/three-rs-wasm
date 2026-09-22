use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::motion::MotionScene;
use super::*;
use crate::postprocessing::Effect;
pub(in crate::browser) struct MotionBlur {
    viewer: OrbitViewer,
    orbit: Vector2,
    pan: Vector3,
    pub dragging: bool,
    time: f64,
    animation_time: f64,
    seek: Option<f64>,
    parameters: [f32; 3],
    mixer: crate::animation::AnimationMixer,
    left: Object3D,
    right: Object3D,
    motion: MotionScene,
    target: RenderTarget,
    effect: Effect,
    sampler: wgpu::Sampler,
}
impl MotionBlur {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.25,
            far: 30.,
            ..Default::default()
        }));
        let target = Vector3::new(0., 1., 0.);
        let offset = Vector3::new(0., 0.5, 4.5);
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(0., (offset.y / offset.length()).asin(), 1.8);
        s.background = Color::BLACK;
        s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x0487e2),
            near: 7.,
            far: 25.,
        });
        s.shadow_map_size = 1024;
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::from_hex(0xffe499),
            intensity: 5.,
        }));
        {
            let n = s.get_mut(sun)?;
            n.position = Vector3::new(4., 4., 2.);
            n.cast_shadow = true;
            n.shadow.far = 10.;
        }
        for (sky, ground, intensity) in [(0x74ccf4, 0, 1.), (0x333366, 0x74ccf4, 5.)] {
            let h = s.insert(NodeKind::Light(Light::Hemisphere {
                sky: Color::from_hex(sky),
                ground: Color::from_hex(ground),
                intensity,
            }));
            s.get_mut(h)?.position = Vector3::Y;
        }
        let (a, b, i) = load_asset("/web/gallery/assets/tsl-procedural/Xbot.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.quaternion = Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2);
        for h in instance.roots {
            s.add(group, h)?;
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[3].clone())?;
        mixer.update(s, 0.)?;
        let mut objects = instance.meshes;
        for &h in &objects {
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            if let NodeKind::Mesh(m) = &mut n.kind {
                for material in &mut m.materials {
                    if let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) =
                        Arc::make_mut(material)
                    {
                        m.energy_conservation = true;
                    }
                }
            }
        }
        let mut floor = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-procedural/FloorsCheckerboard_S_Diffuse.jpg").await?,
        )
        .await?;
        floor.srgb = true;
        floor.wrap_s = Wrapping::Repeat;
        floor.wrap_t = Wrapping::Repeat;
        floor.repeat = Vector2::splat(5.);
        floor.mipmap_filter = Some(Filter::Linear);
        for walls in [false, true] {
            let mut m = MeshPhongMaterial::default();
            m.properties.map = Some(Arc::new(floor.clone()));
            m.properties.side = if walls { Side::Back } else { Side::Front };
            let h = mesh(
                s,
                Arc::new(BoxGeometry::build(
                    15.,
                    if walls { 15. } else { 0.001 },
                    15.,
                )?),
                Material::Phong(m),
            );
            s.get_mut(h)?.receive_shadow = !walls;
            objects.push(h);
        }
        let mut map = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-procedural/uv_grid_opengl.jpg").await?,
        )
        .await?;
        map.srgb = true;
        map.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(map));
        let g = Arc::new(TorusGeometry::build(
            0.8,
            0.4,
            12,
            48,
            std::f64::consts::TAU,
            0.,
            std::f64::consts::TAU,
        )?);
        let right = mesh(s, g.clone(), Material::Basic(m.clone()));
        s.get_mut(right)?.position = Vector3::new(3.5, 1.5, -4.);
        let left = mesh(s, g, Material::Basic(m));
        s.get_mut(left)?.position = Vector3::new(-3.5, 1.5, -4.);
        objects.extend([right, left]);
        let motion = MotionScene::new(r, s, &objects).await?;
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                count: 2,
                ..Default::default()
            },
        )?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let blur = tsl::display::motion_blur(
            tsl::Texture::Input,
            uv(),
            tsl::Texture::External(0).sample(uv()).swizzle("xy") * uniform(0, Type::Float),
            uint(16),
        );
        let vignette = float(1.)
            - (((uv() - float(0.5)).length() - float(0.6)) / float(0.4) * float(2.))
                .clamp(float(0.), float(1.));
        let effect = effect_with_textures(
            r,
            HDR,
            &vec4(blur.rgb() * vignette, blur.swizzle("w")),
            &[(&target.views[1], &sampler)],
        )
        .await?;
        Ok(Self {
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            dragging: false,
            time: 0.,
            animation_time: 0.,
            seek: None,
            parameters: [1., 1., 1.],
            mixer,
            left,
            right,
            motion,
            target,
            effect,
            sampler,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.seek = Some(t);
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 3 || !v.is_finite() {
            return Err(Error::Invalid("motion blur parameter"));
        }
        self.parameters[i] = v.clamp(0., [1., 3., 2.][i]);
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        let d = if let Some(t) = self.seek.take() {
            t - self.time
        } else if a {
            d
        } else {
            0.
        };
        self.time += d;
        self.animation_time += d * self.parameters[2] as f64;
        if self.parameters[0] > 0.5 && !self.dragging {
            self.orbit.x += d / 60.;
        }
        self.viewer
            .orbit_pixels(self.orbit.x * 0.05, self.orbit.y * 0.05, 0., 1., 1., 10.);
        self.viewer.pan_world(self.pan * 0.05);
        self.orbit *= 0.95;
        self.pan *= 0.95;
        self.viewer
            .limit_pitch(0., std::f64::consts::FRAC_PI_2 - 1e-6);
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.25;
            p.far = 30.;
        }
        s.get_mut(self.right)?.quaternion = Quaternion::from_rotation_y(self.animation_time * 4.);
        s.get_mut(self.left)?.scale =
            Vector3::splat(1. + (self.time * 10. * self.parameters[2] as f64).sin() * 0.2);
        for action in &mut self.mixer.actions {
            action.time = self.animation_time;
        }
        self.mixer.update(s, 0.)?;
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
        pan: bool,
        h: f64,
    ) -> Result<()> {
        if pan {
            self.pan += OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, h)?;
        } else {
            self.orbit += Vector2::new(dx, dy) / h.max(1.);
        }
        self.viewer.orbit_pixels(0., 0., w, h, 1., 10.);
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if (self.target.width, self.target.height) != (out.width, out.height) {
            self.target.set_size(&r.device, out.width, out.height)?;
            self.effect
                .set_textures(r, &[(&self.target.views[1], &self.sampler)])?;
        }
        self.motion.update(r, s, c, None)?;
        r.render(s, c, &self.target)?;
        self.effect.parameters[0][0] = self.parameters[1];
        self.effect.apply(r, &self.target, None, out)?;
        Ok(true)
    }
}
