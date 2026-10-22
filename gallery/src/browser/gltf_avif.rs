//! three.js examples/webgl_loader_gltf_avif.html: an AVIF-textured,
//! Draco-compressed glTF, with the page's DRACOLoader.
use super::gltf_viewer::{OrbitViewer, load_asset};
use crate::{Result, camera::*, math::*, scene::*};

struct Avif;

impl super::example::Example for Avif {}

/// The page's DRACOLoader, kept out of line so the call stays a call into
/// this module.
#[inline(never)]
fn install() {
    crate::compression::install_draco();
}

async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<super::expanded::Demo> {
    install();
    let (asset, buffers, images) = load_asset("/web/models/AVIFTest/forest_house.glb").await?;
    crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
    scene.background = Color::from_hex(0xf6eedc);
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.0,
        aspect,
        near: 0.1,
        far: 100.0,
        ..Default::default()
    }));
    let center = Vector3::new(0.0, 2.0, 0.0);
    let offset = Vector3::new(1.5, 4.0, 9.0) - center;
    let mut viewer = OrbitViewer::from_camera(center, offset.length());
    viewer.fixture(
        offset.x.atan2(offset.z),
        (offset.y / offset.length()).asin(),
        1.8,
    );
    Ok(super::expanded::module(viewer, 0.1, 100.0, Box::new(Avif)))
}

/// The program of three.js examples/webgl_loader_gltf_avif.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_gltf_avif(
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
            on_demand: true,
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
