//! three.js examples/webgl_buffergeometry_lines_indexed.html: Koch
//! snowflake curves as one indexed line segments object, turning.
use super::gallery_random::Random;
use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

struct Snowflake {
    positions: Vec<Vector3>,
    colors: Vec<f32>,
    indices: Vec<u32>,
    random: Random,
}
impl Snowflake {
    fn vertex(&mut self, p: Vector3) {
        self.positions.push(p);
        self.colors.extend([
            (self.random.next() * 0.5 + 0.5) as f32,
            (self.random.next() * 0.5 + 0.5) as f32,
            1.0,
        ]);
    }
    fn segment(&mut self, p0: Vector3, p4: Vector3, depth: u32) {
        if depth == 0 {
            let i = self.positions.len() as u32 - 1;
            self.vertex(p4);
            self.indices.extend([i, i + 1]);
            return;
        }
        let v = p4 - p0;
        let tier = v / 3.0;
        let p1 = p0 + tier;
        let angle = v.y.atan2(v.x) + std::f64::consts::PI / 3.0;
        let p2 = p1 + Vector3::new(angle.cos(), angle.sin(), 0.0) * tier.length();
        let p3 = p0 + tier * 2.0;
        for (a, b) in [(p0, p1), (p1, p2), (p2, p3), (p3, p4)] {
            self.segment(a, b, depth - 1);
        }
    }
    fn curve(&mut self, mut points: Vec<Vector3>, closed: bool) {
        for depth in 0..4 {
            self.vertex(points[0]);
            for pair in points.windows(2) {
                self.segment(pair[0], pair[1], depth);
            }
            if closed {
                self.segment(*points.last().unwrap(), points[0], depth);
            }
            for p in &mut points {
                p.x += 600.0;
            }
        }
    }
}

struct Snowflakes {
    root: Object3D,
    time: f64,
}

impl super::example::Example for Snowflakes {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        _camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if animate {
            self.time += delta;
        }
        Some(scene.get_mut(self.root).map(|n| {
            n.quaternion =
                Quaternion::from_rotation_z(if animate { self.time * 0.5 } else { 1.23 * 0.5 })
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
        far: 10000.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 9000.0);
    let mut snow = Snowflake {
        positions: vec![],
        colors: vec![],
        indices: vec![],
        random: Random(1),
    };
    let points = |values: &[(f64, f64)]| {
        values
            .iter()
            .map(|&(x, y)| Vector3::new(x, y, 0.0))
            .collect()
    };
    snow.curve(points(&[(0.0, 0.0), (500.0, 0.0)]), false);
    snow.curve(
        points(&[(0.0, 600.0), (250.0, 1000.0), (500.0, 600.0)]),
        true,
    );
    snow.curve(
        points(&[
            (0.0, 1200.0),
            (500.0, 1200.0),
            (500.0, 1700.0),
            (0.0, 1700.0),
        ]),
        true,
    );
    snow.curve(
        points(&[
            (250.0, 2200.0),
            (500.0, 2200.0),
            (250.0, 2200.0),
            (250.0, 2450.0),
            (250.0, 2200.0),
            (0.0, 2200.0),
            (250.0, 2200.0),
            (250.0, 1950.0),
            (250.0, 2200.0),
        ]),
        false,
    );
    let mut geometry = BufferGeometry::default();
    geometry.set_from_points(&snow.positions)?;
    geometry.set_attribute(
        "color",
        Attribute::F32(BufferAttribute::new(snow.colors, 3, false)?),
    );
    geometry.set_index(Some(snow.indices));
    let mut material = LineBasicMaterial::default();
    material.properties.vertex_colors = true;
    let line = scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(geometry),
        material: Arc::new(Material::Line(material)),
        segments: true,
    }));
    scene.get_mut(line)?.position = Vector3::new(-1200.0, -1200.0, 0.0);
    let root = scene.insert(NodeKind::Group);
    scene.add(root, line)?;
    Ok(super::expanded::own(Box::new(Snowflakes {
        root,
        time: 0.0,
    })))
}

/// The program of three.js examples/webgl_buffergeometry_lines_indexed.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry_lines_indexed(
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
