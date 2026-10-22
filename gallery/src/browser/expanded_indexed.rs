use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

pub(super) fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Object3D> {
    scene.background = Color::from_hex(0x050505);
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 27.0,
        aspect,
        near: 1.0,
        far: 3500.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 64.0);
    let light = scene.insert(NodeKind::Light(Light::Hemisphere {
        sky: Color::WHITE,
        ground: Color::WHITE,
        intensity: 3.0,
    }));
    scene.get_mut(light)?.position = Vector3::Y;
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    for i in 0..=10 {
        let y = i as f64 * 2.0 - 10.0;
        for j in 0..=10 {
            let x = j as f64 * 2.0 - 10.0;
            vertices.extend([x as f32, -y as f32, 0.0]);
            normals.extend([0.0, 0.0, 1.0]);
            colors.extend(
                Color::from_srgb(x / 20.0 + 0.5, y / 20.0 + 0.5, 1.0)
                    .0
                    .to_array()
                    .map(|v| v as f32),
            );
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            let b = i * 11 + j;
            indices.extend([b + 1, b, b + 12, b, b + 11, b + 12]);
        }
    }
    let mut geometry = BufferGeometry::default();
    for (name, data) in [
        ("position", vertices),
        ("normal", normals),
        ("color", colors),
    ] {
        geometry.set_attribute(name, Attribute::F32(BufferAttribute::new(data, 3, false)?));
    }
    geometry.set_index(Some(indices));
    let mut material = MeshPhongMaterial::default();
    material.properties.vertex_colors = true;
    material.properties.side = Side::Double;
    Ok(scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(Material::Phong(material)),
    ))))
}

/// webgl_buffergeometry_indexed's mesh, turned by the elapsed time.
struct Indexed {
    mesh: Object3D,
    elapsed: f64,
}

impl super::example::Example for Indexed {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        _camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if animate {
            self.elapsed += delta;
        }
        Some(scene.get_mut(self.mesh).map(|n| {
            n.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                self.elapsed * 0.25,
                self.elapsed * 0.5,
                0.0,
            )
        }))
    }
    fn on_wireframe(&mut self, scene: &mut Scene, enabled: bool) -> Option<Result<()>> {
        Some(scene.get_mut(self.mesh).map(|n| {
            if let NodeKind::Mesh(mesh) = &mut n.kind {
                Arc::make_mut(&mut mesh.materials[0])
                    .properties_mut()
                    .wireframe = enabled;
            }
        }))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.elapsed = seconds;
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

/// The program of three.js examples/webgl_buffergeometry_indexed.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry_indexed(
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
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(Vector3::ZERO, 64.0),
                1.0,
                3500.0,
                Box::new(Indexed {
                    mesh: create(scene, camera, aspect)?,
                    elapsed: 0.0,
                }),
            ))
        },
    )
    .await
}
