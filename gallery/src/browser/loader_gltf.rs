//! three.js examples/webgpu_loader_gltf.html, and the MVP page's BoomBox: a
//! glTF model under an HDR environment, in an orbiting viewer whose model the
//! page switches.
use super::gltf_viewer::OrbitViewer;
use crate::{Result, camera::*, scene::*};

struct Viewer(OrbitViewer);

impl super::example::Example for Viewer {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        _delta: f64,
        _animate: bool,
    ) -> Option<Result<()>> {
        Some(self.0.update(scene, camera))
    }
    fn on_orbit(&mut self, dx: f64, dy: f64, zoom: f64) {
        self.0.orbit(dx, dy, zoom);
    }
    fn on_gltf_view(&mut self, yaw: f64, pitch: f64, distance: f64) {
        self.0.fixture(yaw, pitch, distance);
    }
    fn on_load_model(
        &self,
        model: u32,
        environment: Option<std::sync::Arc<crate::environment::EnvironmentMap>>,
    ) -> Option<super::example::LoadModel> {
        Some(Box::pin(async move {
            if ![4, 5].contains(&model) {
                return Err(crate::Error::Invalid("unknown glTF example"));
            }
            let mut scene = Scene::new();
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
                PerspectiveCamera::default(),
            )));
            let placeholder = scene.insert(NodeKind::Group);
            let viewer =
                OrbitViewer::create(&mut scene, camera, placeholder, model, environment).await?;
            Ok((
                scene,
                camera,
                Box::new(Viewer(viewer)) as Box<dyn super::example::Example>,
            ))
        }))
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

/// The glTF viewer with model `model` ( 4 the helmet, 5 the BoomBox ).
async fn create(page: &mut super::Page, model: u32) -> Result<super::expanded::Demo> {
    let viewer = OrbitViewer::create(&mut page.scene, page.camera, page.mesh, model, None).await?;
    let _ = page
        .canvas
        .set_attribute("data-triangles", &viewer.triangles.to_string());
    let _ = page
        .canvas
        .set_attribute("data-meshes", &viewer.meshes.to_string());
    Ok(super::expanded::own(Box::new(Viewer(viewer))))
}

/// The program of three.js examples/webgpu_loader_gltf.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_loader_gltf(
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
        async |page| create(page, 4).await,
    )
    .await
}

/// The MVP page's BoomBox glTF.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn mvp_boombox(
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
        async |page| create(page, 5).await,
    )
    .await
}
