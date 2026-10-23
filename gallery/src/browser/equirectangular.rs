//! three.js examples/webgpu_equirectangular.html: an equirectangular
//! panorama as the background, turning until the pointer drags it.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{Result, camera::*, math::*, scene::*};
use std::sync::Arc;

struct Equirectangular {
    viewer: OrbitViewer,
    dragging: bool,
}

impl super::example::Example for Equirectangular {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if animate && !self.dragging {
            self.viewer
                .orbit(delta * std::f64::consts::TAU / 60.0 / 0.008, 0.0, 0.0);
        }
        Some(self.viewer.update(scene, camera))
    }
    fn on_scene_input(
        &mut self,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> Option<Result<()>> {
        self.dragging = dragging;
        self.viewer.orbit(-dx * 0.125, -dy * 0.125, wheel);
        Some(Ok(()))
    }
    #[allow(clippy::too_many_arguments)]
    fn on_input(
        &mut self,
        _scene: &mut Scene,
        _camera: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Option<Result<()>> {
        Some(Ok(()))
    }
}

async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<super::expanded::Demo> {
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.0,
        aspect,
        near: 0.25,
        far: 20.0,
        ..Default::default()
    }));
    let texture = decode_image(&fetch("assets/panorama.jpg").await?).await?;
    scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_texture(
        &texture,
    )?));
    scene.background_environment = true;
    scene.background_equirectangular = true;
    let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 1.0);
    viewer.fixture(std::f64::consts::FRAC_PI_2, 0.0, 1.8);
    Ok(super::expanded::own(Box::new(Equirectangular {
        viewer,
        dragging: false,
    })))
}

/// The program of three.js examples/webgpu_equirectangular.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_equirectangular(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, _renderer, aspect) = page.gallery_scene()?;
            create(scene, camera, aspect).await
        },
    )
    .await
}
