//! three.js examples/webgpu_pmrem_equirectangular.html: the static grid of
//! spheres of every roughness and metalness under the PMREM of an HDR
//! environment.
use super::gltf_viewer::OrbitViewer;
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

async fn pmrem_grid(
    scene: &mut Scene,
    camera: Object3D,
    placeholder: Object3D,
) -> Result<OrbitViewer> {
    scene.dispose(placeholder)?;
    scene.environment = Some(Arc::new(
        super::ultra_hdr::environment("/web/environments/royal_esplanade_2k.hdr.jpg").await?,
    ));
    scene.background_environment = true;
    scene.background_blur = 0.5;
    scene.aces_tone_mapping = true;
    let geometry = Arc::new(SphereGeometry::build(0.4, 64, 64)?);
    for i in 0..6 {
        for j in 0..5 {
            let material = MeshStandardMaterial {
                roughness: i as f64 / 5.0,
                metalness: j as f64 / 4.0,
                ..Default::default()
            };
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Standard(material)),
            )));
            scene.get_mut(mesh)?.position = Vector3::new(i as f64 - 2.5, j as f64 - 2.0, 0.0);
        }
    }
    if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
        p.fov = 45.0;
    }
    Ok(OrbitViewer::from_camera(Vector3::ZERO, 8.0))
}

struct Grid(OrbitViewer);

impl super::example::Example for Grid {
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

/// The program of three.js examples/webgpu_pmrem_equirectangular.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_pmrem_equirectangular(
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
            let grid = pmrem_grid(&mut page.scene, page.camera, page.mesh).await?;
            let _ = page.canvas.set_attribute("data-meshes", "30");
            Ok(super::expanded::own(Box::new(Grid(grid))))
        },
    )
    .await
}
