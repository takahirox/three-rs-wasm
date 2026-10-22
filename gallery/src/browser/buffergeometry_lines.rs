//! three.js examples/webgl_buffergeometry_lines.html: ten thousand random
//! points as one line, morphed to a second set and turning.
use super::gallery_random::Random;
use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

struct MorphLine {
    object: Object3D,
    time: f64,
}

impl super::example::Example for MorphLine {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        _camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if animate {
            self.time += delta;
        } else {
            self.time = 1.23;
        }
        let time = self.time;
        let weight = (time * 0.5).sin().abs();
        Some(scene.get_mut(self.object).map(|n| {
            n.quaternion = Euler {
                angles: Vector3::new(time * 0.25, time * 0.5, 0.0),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            n.morph_weights = vec![weight];
        }))
    }
    fn on_scene_input(
        &mut self,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _dragging: bool,
    ) -> Option<Result<()>> {
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
        fov: 27.0,
        aspect,
        near: 1.0,
        far: 4000.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 2750.0);
    let mut random = Random(1);
    let mut base = Vec::with_capacity(10000);
    let mut colors = Vec::with_capacity(30000);
    for _ in 0..10000 {
        let p = Vector3::new(random.next(), random.next(), random.next()) * 800.0
            - Vector3::splat(400.0);
        base.push(p);
        colors.extend([
            (p.x / 800.0 + 0.5) as f32,
            (p.y / 800.0 + 0.5) as f32,
            (p.z / 800.0 + 0.5) as f32,
        ]);
    }
    let target: Vec<Vector3> = (0..10000)
        .map(|_| {
            Vector3::new(random.next(), random.next(), random.next()) * 800.0
                - Vector3::splat(400.0)
        })
        .collect();
    // Morph attributes in the original are float32, including their base.
    for p in &mut base {
        *p = p.as_vec3().as_dvec3();
    }
    let target: Vec<Vector3> = target
        .into_iter()
        .map(|p: Vector3| p.as_vec3().as_dvec3())
        .collect();
    let mut geometry = BufferGeometry::default();
    geometry.set_from_points(&base)?;
    geometry.set_attribute(
        "color",
        Attribute::F32(BufferAttribute::new(colors, 3, false)?),
    );
    geometry.morph_attributes.insert(
        "position".into(),
        vec![Attribute::F32(BufferAttribute::new(
            target.iter().flat_map(|p| p.as_vec3().to_array()).collect(),
            3,
            false,
        )?)],
    );
    let mut material = LineBasicMaterial::default();
    material.properties.vertex_colors = true;
    let object = scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(geometry),
        material: Arc::new(Material::Line(material)),
        segments: false,
    }));
    Ok(super::expanded::own(Box::new(MorphLine {
        object,
        time: 0.0,
    })))
}

/// The program of three.js examples/webgl_buffergeometry_lines.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry_lines(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
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
