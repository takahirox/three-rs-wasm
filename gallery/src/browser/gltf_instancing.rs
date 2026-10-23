//! three.js examples/webgl_loader_gltf_instancing.html: the instanced
//! DamagedHelmet ( EXT_mesh_gpu_instancing ) under the royal esplanade.
use super::gltf_viewer::{OrbitViewer, load_asset};
use crate::{Result, camera::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct Instancing;

impl super::example::Example for Instancing {
    fn on_orbit_limits(&self) -> Option<(bool, f64, f64)> {
        Some((true, 0.2, 10.0))
    }
}

async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<super::expanded::Demo> {
    let (asset, buffers, images) =
        load_asset("models/DamagedHelmet/glTF-instancing/DamagedHelmetGpuInstancing.gltf").await?;
    crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
    // r186 GLTFLoader applies assignFinalMaterial twice to InstancedMesh,
    // flipping derivative-tangent normalScale.y twice. Match this example's
    // actual material without changing the glTF importer's convention.
    for root in scene.roots().to_vec() {
        for h in scene.traverse(root, true)? {
            if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
                for material in &mut mesh.materials {
                    if let Material::Standard(p) = Arc::make_mut(material) {
                        p.normal_scale.y *= -1.0;
                    }
                }
            }
        }
    }
    scene.environment = Some(Arc::new(
        super::ultra_hdr::environment("environments/royal_esplanade_2k.hdr.jpg").await?,
    ));
    scene.background_environment = true;
    scene.aces_tone_mapping = true;
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.0,
        aspect,
        near: 0.25,
        far: 20.0,
        ..Default::default()
    }));
    let center = Vector3::new(0.0, 0.25, 0.0);
    let offset = Vector3::new(-0.90, 0.41, -0.89) - center;
    let mut viewer = OrbitViewer::from_camera(center, offset.length());
    viewer.fixture(
        offset.x.atan2(offset.z),
        (offset.y / offset.length()).asin(),
        1.8,
    );
    Ok(super::expanded::module(
        viewer,
        0.25,
        20.0,
        Box::new(Instancing),
    ))
}

/// The program of three.js examples/webgl_loader_gltf_instancing.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_gltf_instancing(
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
