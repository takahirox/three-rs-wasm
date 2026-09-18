//! glTF scenes; unaccepted candidates remain hidden until fidelity/resource checks pass.
use super::gltf_viewer::{OrbitViewer, fetch, load_asset};
use crate::{
    Result, animation::AnimationMixer, camera::*, environment::EnvironmentMap, math::*, scene::*,
};
use std::sync::Arc;

pub(super) struct Demo {
    viewer: OrbitViewer,
    mixer: AnimationMixer,
    near: f64,
    far: f64,
    min: f64,
    max: f64,
    auto_speed: f64,
    damping: bool,
    orbit_delta: Vector2,
    pan_delta: Vector3,
    time: f64,
    seek: Option<f64>,
    dragging: bool,
}
impl Demo {
    pub async fn create(scene: &mut Scene, camera: Object3D, example: u32) -> Result<Self> {
        let (model, environment, fov, near, far, position, center, blur, exposure, min, max) =
            match example {
                29 => (
                    "IridescenceLamp.glb",
                    "/web/environments/venice_sunset_1k.hdr",
                    50.0,
                    0.05,
                    20.0,
                    Vector3::new(0.35, 0.05, 0.35),
                    Vector3::new(0.0, 0.2, 0.0),
                    0.0,
                    1.0,
                    0.0,
                    f64::INFINITY,
                ),
                30 => (
                    "AnisotropyBarnLamp.glb",
                    "/web/environments/royal_esplanade_2k.hdr",
                    40.0,
                    0.01,
                    10.0,
                    Vector3::new(-0.35, -0.2, 0.35),
                    Vector3::new(0.0, -0.08, 0.11),
                    0.5,
                    1.35,
                    0.1,
                    2.0,
                ),
                31 => (
                    "SheenChair.glb",
                    "/web/environments/royal_esplanade_2k.hdr",
                    45.0,
                    0.1,
                    20.0,
                    Vector3::new(-0.75, 0.7, 1.25),
                    Vector3::new(0.0, 0.35, 0.0),
                    0.0,
                    1.0,
                    1.0,
                    10.0,
                ),
                32 => (
                    "IridescentDishWithOlives.glb",
                    "/web/environments/royal_esplanade_2k.hdr",
                    45.0,
                    0.25,
                    20.0,
                    Vector3::new(0.0, 0.4, 0.7),
                    Vector3::new(0.0, 0.1, 0.0),
                    0.35,
                    1.0,
                    0.5,
                    1.0,
                ),
                _ => return Err(crate::Error::Invalid("glTF candidate id")),
            };
        let (asset, buffers, images) = load_asset(&format!("/web/models/{model}")).await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let mut mixer = AnimationMixer::default();
        if let Some(clip) = instance.clips.first() {
            mixer.play(clip.clone())?;
        }
        scene.environment = Some(Arc::new(EnvironmentMap::from_hdr(
            &fetch(environment).await?,
        )?));
        scene.background_environment = true;
        scene.background_blur = blur;
        scene.aces_tone_mapping = true;
        scene.exposure = exposure;
        let aspect = match scene.camera(camera)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.0,
        };
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            aspect,
            near,
            far,
            ..Default::default()
        }));
        let offset = position - center;
        let mut viewer = OrbitViewer::from_camera(center, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            viewer,
            mixer,
            near,
            far,
            min,
            max,
            auto_speed: match example {
                29 => -0.5,
                32 => -0.75,
                _ => 0.0,
            },
            damping: matches!(example, 31 | 32),
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
            time: 0.0,
            seek: None,
            dragging: false,
        })
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        let step = if let Some(seconds) = self.seek.take() {
            let step = seconds - self.time;
            self.time = seconds;
            self.viewer.orbit_pixels(
                step * self.auto_speed / 60.0,
                0.0,
                0.0,
                1.0,
                self.min,
                self.max,
            );
            self.orbit_delta = Vector2::ZERO;
            self.pan_delta = Vector3::ZERO;
            for action in &mut self.mixer.actions {
                action.time = seconds;
            }
            self.mixer.update(scene, 0.0)?;
            0.0
        } else if animate {
            self.mixer.update(scene, delta)?;
            // The original OrbitControls.update() omits deltaTime: one step per frame.
            self.time += 1.0 / 60.0;
            if self.dragging { 0.0 } else { 1.0 / 60.0 }
        } else {
            0.0
        };
        self.orbit_delta.x += step * self.auto_speed / 60.0;
        let damping = if self.damping { 0.05 } else { 1.0 };
        self.viewer.orbit_pixels(
            self.orbit_delta.x * damping,
            self.orbit_delta.y * damping,
            0.0,
            1.0,
            self.min,
            self.max,
        );
        self.viewer.pan_world(self.pan_delta * damping);
        self.orbit_delta *= 1.0 - damping;
        self.pan_delta *= 1.0 - damping;
        self.viewer.update(scene, camera)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
            p.near = self.near;
            p.far = self.far;
        }
        Ok(())
    }
    pub fn seek(&mut self, seconds: f64) {
        self.seek = Some(seconds);
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if pan {
            self.pan_delta +=
                OrbitViewer::pan_delta(scene, camera, self.viewer.radius(), dx, dy, height)?;
        } else {
            self.orbit_delta += Vector2::new(dx, dy) / height.max(1.0);
            self.viewer
                .orbit_pixels(0.0, 0.0, wheel, height, self.min, self.max);
        }
        Ok(())
    }
}
