//! three.js examples/webgpu_morphtargets.html: a box morphed into a sphere
//! and twisted, by the page's sliders.
use super::gltf_viewer::OrbitViewer;
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct Morph(Object3D);

impl super::example::Example for Morph {
    fn on_morph(&mut self, scene: &mut Scene, weights: [f64; 2]) -> Option<Result<()>> {
        Some(
            scene
                .get_mut(self.0)
                .map(|n| n.morph_weights.copy_from_slice(&weights)),
        )
    }
    fn on_orbit_limits(&self) -> Option<(bool, f64, f64)> {
        Some((false, 0.0, f64::INFINITY))
    }
}

async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<super::expanded::Demo> {
    scene.background = Color::from_hex(0x8fbcd4);
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.0,
        aspect,
        near: 1.0,
        far: 20.0,
        ..Default::default()
    }));
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::from_hex(0x8fbcd4),
        intensity: 1.5,
    }));
    let light = scene.insert(NodeKind::Light(Light::Point {
        color: Color::WHITE,
        intensity: 200.0,
        distance: 0.0,
        decay: 2.0,
    }));
    scene.add(camera, light)?;
    let mut geometry = BoxGeometry::segmented(2.0, 2.0, 2.0, 32, 32, 32)?;
    let mut sphere = Vec::new();
    let mut twist = Vec::new();
    for p in geometry.positions()? {
        let Vector3 { x, y, z } = p;
        sphere.extend(
            [
                x * (1.0 - y * y / 2.0 - z * z / 2.0 + y * y * z * z / 3.0).sqrt(),
                y * (1.0 - z * z / 2.0 - x * x / 2.0 + z * z * x * x / 3.0).sqrt(),
                z * (1.0 - x * x / 2.0 - y * y / 2.0 + x * x * y * y / 3.0).sqrt(),
            ]
            .map(|v| v as f32),
        );
        let q = Quaternion::from_axis_angle(Vector3::X, std::f64::consts::PI * x / 2.0);
        twist.extend(
            (q * Vector3::new(x * 2.0, y, z))
                .to_array()
                .map(|v| v as f32),
        );
    }
    geometry.morph_attributes.insert(
        "position".into(),
        vec![
            Attribute::F32(crate::attribute::BufferAttribute::new(sphere, 3, false)?),
            Attribute::F32(crate::attribute::BufferAttribute::new(twist, 3, false)?),
        ],
    );
    let mut material = MeshPhongMaterial::default();
    material.properties.color = Color::from_hex(0xff0000);
    material.properties.flat_shading = true;
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(Material::Phong(material)),
    )));
    scene.get_mut(mesh)?.morph_weights = vec![0.0, 0.0];
    Ok(super::expanded::module(
        OrbitViewer::from_camera(Vector3::ZERO, 10.0),
        1.0,
        20.0,
        Box::new(Morph(mesh)),
    ))
}

/// The program of three.js examples/webgpu_morphtargets.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_morphtargets(
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
