use super::motion::MotionScene;
use super::*;
use crate::{postprocessing::Effect, tsl::temporal::TemporalAA};
pub(in crate::browser) struct Taau {
    viewer: OrbitViewer,
    orbit: Vector2,
    pan: Vector3,
    time: f64,
    mixer: crate::animation::AnimationMixer,
    motion: MotionScene,
    scene: RenderTarget,
    aa: TemporalAA,
    copy: Effect,
    sharpen: Effect,
    parameters: [f32; 4],
    started: bool,
}
impl Taau {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }));
        let mut viewer = OrbitViewer::from_camera(Vector3::new(-0.5, 0., 0.), 12.);
        viewer.fixture(0., 0., 1.8);
        viewer.update(s, c)?;
        s.background = Color::from_hex(0xbfe3dd);
        s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
        s.environment = Some(super::super::room_environment::environment(r)?);
        let (a, b, i) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/LittlestTokyo.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let root = s.insert(NodeKind::Group);
        s.get_mut(root)?.scale = Vector3::splat(0.01);
        for h in instance.roots {
            s.add(root, h)?;
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        mixer.update(s, 0.)?;
        for &h in &instance.meshes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
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
        let motion = MotionScene::new(r, s, &instance.meshes).await?;
        let scene = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                count: 2,
                ..Default::default()
            },
        )?;
        Ok(Self {
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            time: 0.,
            mixer,
            motion,
            scene,
            aa: TemporalAA::upscaling(r).await?,
            copy: effect(r, HDR, &tsl::Texture::Input.sample(uv())).await?,
            sharpen: effect(
                r,
                HDR,
                &tsl::fsr1::rcas(
                    tsl::Texture::Input,
                    uv(),
                    float(0.2),
                    float(0.).greater_than(float(1.)),
                ),
            )
            .await?,
            parameters: [1., 0.5, 1., 0.2],
            started: false,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 4 || !v.is_finite() {
            return Err(Error::Invalid("TAAU parameter"));
        }
        self.parameters[i] = match i {
            0 | 2 => {
                if v > 0.5 {
                    1.
                } else {
                    0.
                }
            }
            1 => (v * 4.).round().clamp(1., 4.) / 4.,
            _ => v.clamp(0., 2.),
        };
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer.orbit_pixels(
            self.orbit.x * 0.05,
            self.orbit.y * 0.05,
            0.,
            1.,
            0.,
            f64::INFINITY,
        );
        self.viewer.pan_world(self.pan * 0.05);
        self.orbit *= 0.95;
        self.pan *= 0.95;
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.;
        }
        for action in &mut self.mixer.actions {
            action.time = self.time;
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
        p: bool,
        h: f64,
    ) -> Result<()> {
        if p {
            self.pan += OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, h)?;
        } else {
            self.orbit += Vector2::new(dx, dy) / h.max(1.);
        }
        self.viewer.orbit_pixels(0., 0., w, h, 0., f64::INFINITY);
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let previous_size = [self.scene.width, self.scene.height];
        let scale = self.parameters[1];
        self.scene.set_size(
            &r.device,
            (out.width as f32 * scale).floor().max(1.) as u32,
            (out.height as f32 * scale).floor().max(1.) as u32,
        )?;
        let NodeKind::Camera(Camera::Perspective(camera)) = &s.get(c)?.kind else {
            return Err(Error::Invalid("TAAU camera"));
        };
        let original_aspect = camera.aspect;
        let mut jittered = camera.clone();
        if self.parameters[0] > 0.5 && self.started {
            jittered.aspect = previous_size[0] as f64 / previous_size[1] as f64;
            jittered.view = Some(self.aa.view_offset(previous_size[0], previous_size[1]));
        }
        let projection = jittered.projection_matrix()?;
        // r186 TAAU compiles VelocityNode before its projection override is set:
        // current clip uses camera jitter, previous clip remains unjittered.
        // Preserve that upstream example behavior; MotionVectors itself imposes
        // no jitter and can be used with correctly unjittered inputs elsewhere.
        self.motion.update(r, s, c, Some(projection))?;
        let world = s.get(c)?.matrix_world;
        if let NodeKind::Camera(Camera::Perspective(camera)) = &mut s.get_mut(c)?.kind {
            camera.view = jittered.view;
            camera.aspect = jittered.aspect;
        }
        let result = r.render(s, c, &self.scene);
        if let NodeKind::Camera(Camera::Perspective(camera)) = &mut s.get_mut(c)?.kind {
            camera.view = None;
            camera.aspect = original_aspect;
        }
        result?;
        if self.parameters[0] < 0.5 {
            self.copy.apply(r, &self.scene, None, out)?;
        } else {
            self.started = true;
            self.aa.set_output_size(out.width, out.height)?;
            self.aa
                .apply(r, &self.scene, world, projection, [0.1, 100.], false)?;
            if self.parameters[2] > 0.5 {
                // r186 SharpenNode captures its numeric sharpness as a ConstNode;
                // the GUI changes .value without invalidating the compiled material.
                self.sharpen.apply(r, self.aa.output(), None, out)?;
            } else {
                self.copy.apply(r, self.aa.output(), None, out)?;
            }
        }
        Ok(true)
    }
}
