//! three.js examples/webgpu_pmrem_test.html: spheres of every roughness
//! and metalness lit by the PMREM of an HDR environment, or by a directional
//! light in its place.
use super::gltf_viewer::{OrbitViewer, fetch};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct Calibration {
    viewer: OrbitViewer,
    light: Object3D,
}

impl super::example::Example for Calibration {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        _delta: f64,
        _animate: bool,
    ) -> Option<Result<()>> {
        Some((|| {
            self.viewer.update(scene, camera)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
                p.fov = 2.0
                    * ((20.0_f64.to_radians()).tan() / p.aspect)
                        .atan()
                        .to_degrees();
            }
            Ok(())
        })())
    }
    fn on_pmrem(&mut self, scene: &mut Scene, enabled: bool) -> Option<Result<()>> {
        scene.environment_intensity = if enabled { 1.0 } else { 0.0 };
        Some(scene.get_mut(self.light).map(|n| {
            if let NodeKind::Light(Light::Directional { intensity, .. }) = &mut n.kind {
                *intensity = if enabled { 0.0 } else { 1.0 };
            }
        }))
    }
    fn on_scene_input(
        &mut self,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> Option<Result<()>> {
        let _ = dragging;
        self.viewer.orbit(dx, dy, wheel);
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
        fov: 40.0,
        aspect,
        near: 1.0,
        far: 30.0,
        ..Default::default()
    }));
    scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
        &fetch("/web/gallery/assets/spot1Lux.hdr").await?,
    )?));
    scene.background_environment = true;
    scene.aces_tone_mapping = true;
    let geometry = Arc::new(SphereGeometry::build(0.4, 32, 32)?);
    for x in 0..=10 {
        for y in 0..=2 {
            let mut material = MeshStandardMaterial {
                roughness: x as f64 / 10.0,
                metalness: if y < 1 { 1.0 } else { 0.0 },
                ..Default::default()
            };
            material.properties.color = if y < 2 { Color::WHITE } else { Color::BLACK };
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Standard(material)),
            )));
            scene.get_mut(mesh)?.position = Vector3::new(x as f64 - 5.0, 1.0 - y as f64, 0.0);
        }
    }
    let theta = (597.0 + 0.5) * std::f64::consts::PI / 512.0;
    let phi = -(213.0 + 0.5) * std::f64::consts::PI / 512.0;
    let angle = std::f64::consts::FRAC_PI_2 - theta;
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 0.0,
        target: Vector3::ZERO,
    }));
    scene.get_mut(light)?.position =
        Vector3::new(phi.sin() * angle.sin(), phi.cos(), phi.sin() * angle.cos()) * 100.0;
    Ok(super::expanded::own(Box::new(Calibration {
        viewer: OrbitViewer::from_camera(Vector3::ZERO, 16.0),
        light,
    })))
}

/// The program of three.js examples/webgpu_pmrem_test.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_pmrem_test(
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
