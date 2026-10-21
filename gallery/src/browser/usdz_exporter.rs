//! misc_exporter_usdz: the CarbonFrameBike glTF ( Draco geometry, KTX2
//! textures with texture transforms ) and its looping animation under the
//! RoomEnvironment PMREM, ACES Filmic tone mapping and damped OrbitControls,
//! on the engine's glTF importer and renderer.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::load_asset;
use crate::{Result, camera::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;

pub(super) struct Demo {
    controls: Controls,
    mixer: Option<crate::animation::AnimationMixer>,
    time: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        // The page's KTX2Loader.
        crate::basis::install();
        // The page's DRACOLoader.
        crate::compression::install_draco();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.25,
            far: 20.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-2.5, 1.6, 3.);
        s.tone_mapping = ToneMapping::Aces;
        s.background = Color::from_hex(0xf0f0f0);
        s.environment = Some(super::room_environment::environment(r)?);
        let (a, b, i) = load_asset("/web/gallery/assets/gltf/CarbonFrameBike.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let mut mixer = None;
        if let Some(clip) = instance.clips.first() {
            let mut m = crate::animation::AnimationMixer::default();
            m.play(clip.clone())?;
            mixer = Some(m);
        }
        let mut controls = Controls::new(Some(0.05), (2., 10.), PI, true);
        controls.set_target(Vector3::new(0., 0.7, 0.));
        controls.update(s, c)?;
        Ok(Self {
            controls,
            mixer,
            time: 0.,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the mixer by the timer, then controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(m) = &mut self.mixer {
            for a in &mut m.actions {
                a.time = self.time;
            }
            m.update(s, 0.)?;
        }
        self.controls.update(s, c)
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
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/misc_exporter_usdz.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn misc_exporter_usdz(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.25,
                20.,
                Box::new(Demo::create(scene, camera, 424, renderer).await?),
            ))
        },
    )
    .await
}
