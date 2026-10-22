use crate::{
    Result, attribute::BufferAttribute, camera::*, curve::CatmullRomCurve3, geometry::*,
    material::*, math::*, scene::*,
};
use std::sync::Arc;
// Ordering is the original GeometryUtils.hilbert3D recursion, including orientation.
pub(super) fn hilbert(
    center: Vector3,
    size: f64,
    iterations: u32,
    order: [usize; 8],
) -> Vec<Vector3> {
    let half = size / 2.0;
    let corners = [
        [-1.0, 1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, -1.0],
    ]
    .map(|p| center + Vector3::from_array(p) * half);
    let points = order.map(|i| corners[i]);
    if iterations == 0 {
        return points.to_vec();
    }
    let permutations = [
        [0, 3, 4, 7, 6, 5, 2, 1],
        [0, 7, 6, 1, 2, 5, 4, 3],
        [0, 7, 6, 1, 2, 5, 4, 3],
        [2, 3, 0, 1, 6, 7, 4, 5],
        [2, 3, 0, 1, 6, 7, 4, 5],
        [4, 3, 2, 5, 6, 1, 0, 7],
        [4, 3, 2, 5, 6, 1, 0, 7],
        [6, 5, 2, 1, 0, 3, 4, 7],
    ];
    points
        .into_iter()
        .zip(permutations)
        .flat_map(|(center, permutation)| {
            hilbert(center, half, iterations - 1, permutation.map(|i| order[i]))
        })
        .collect()
}
fn line(
    scene: &mut Scene,
    points: Vec<Vector3>,
    segments: bool,
    color: u32,
    dash: f64,
    gap: f64,
) -> Result<Object3D> {
    let mut geometry = BufferGeometry::default();
    geometry.set_from_points(&points)?;
    // Compute once from Float32 attributes, as Line.computeLineDistances does.
    let points = geometry.positions()?;
    let mut distances = vec![0.0; points.len()];
    for i in 1..points.len() {
        distances[i] = distances[i - 1]
            + if !segments || i % 2 == 1 {
                points[i].distance(points[i - 1])
            } else {
                0.0
            };
    }
    geometry.set_attribute(
        "lineDistance",
        Attribute::F32(BufferAttribute::new(
            distances.into_iter().map(|v| v as f32).collect(),
            1,
            false,
        )?),
    );
    let mut material = LineBasicMaterial::default();
    material.properties.color = Color::from_hex(color);
    material.dash = Some(LineDash {
        size: dash,
        gap,
        ..Default::default()
    });
    Ok(scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(geometry),
        material: Arc::new(Material::Line(material)),
        segments,
    })))
}
pub(super) fn dashed(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Vec<Object3D>> {
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 60.0,
        aspect,
        near: 1.0,
        far: 200.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 150.0);
    scene.look_at(camera, Vector3::ZERO)?;
    scene.background = Color::from_hex(0x111111);
    scene.fog = Some(Fog::Linear {
        color: scene.background,
        near: 150.0,
        far: 200.0,
    });
    let points = hilbert(Vector3::ZERO, 25.0, 1, [0, 1, 2, 3, 4, 5, 6, 7]);
    let count = points.len() as u32 * 6;
    let samples = CatmullRomCurve3::new(points).points(count)?;
    let spline = line(scene, samples, false, 0xffffff, 1.0, 0.5)?;
    let mut box_points = Vec::new();
    for z in [-25.0, 25.0] {
        let corners = [
            Vector3::new(-25.0, -25.0, z),
            Vector3::new(-25.0, 25.0, z),
            Vector3::new(25.0, 25.0, z),
            Vector3::new(25.0, -25.0, z),
        ];
        for i in 0..4 {
            box_points.extend([corners[i], corners[(i + 1) % 4]]);
        }
    }
    for (x, y) in [(-25.0, -25.0), (-25.0, 25.0), (25.0, 25.0), (25.0, -25.0)] {
        box_points.extend([Vector3::new(x, y, -25.0), Vector3::new(x, y, 25.0)]);
    }
    let box_line = line(scene, box_points, true, 0xffaa00, 3.0, 1.0)?;
    Ok(vec![spline, box_line])
}

pub(super) fn colors(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Vec<Object3D>> {
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 33.0,
        aspect,
        near: 1.0,
        far: 10000.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 1000.0);
    let points = hilbert(Vector3::ZERO, 200.0, 1, [0, 1, 2, 3, 4, 5, 6, 7]);
    let curve = CatmullRomCurve3::new(points.clone());
    let mut material = LineBasicMaterial::default();
    material.properties.vertex_colors = true;
    let material = Arc::new(Material::Line(material));
    let mut lines = Vec::new();
    for index in 0..6 {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let count = if index < 3 { 384 } else { 64 };
        let samples = if index < 3 {
            (0..count)
                .map(|i| curve.point(i as f64 / count as f64))
                .collect::<Result<Vec<_>>>()?
        } else {
            points.clone()
        };
        for (i, p) in samples.into_iter().enumerate() {
            positions.push(p);
            let (h, l) = match index {
                0 => (0.6, (-p.x / 200.0).max(0.0) + 0.5),
                1 => (0.9, (-p.y / 200.0).max(0.0) + 0.5),
                3 => (0.6, ((200.0 - p.x) / 400.0).max(0.0) * 0.5 + 0.5),
                4 => (0.3, ((200.0 + p.x) / 400.0).max(0.0) * 0.5),
                _ => (i as f64 / count as f64, 0.5),
            };
            let c = Color::from_hsl(h, 1.0, l).0;
            colors.extend(
                Color::from_srgb(c.x, c.y, c.z)
                    .0
                    .to_array()
                    .map(|v| v as f32),
            );
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_from_points(&positions)?;
        geometry.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(colors, 3, false)?),
        );
        let line = scene.insert(NodeKind::Line(Line {
            geometry: Arc::new(geometry),
            material: material.clone(),
            segments: false,
        }));
        scene.get_mut(line)?.scale = Vector3::splat(0.45);
        scene.get_mut(line)?.position = Vector3::new(
            (index % 3) as f64 * 225.0 - 225.0,
            if index < 3 { -112.5 } else { 112.5 },
            0.0,
        );
        lines.push(line);
    }
    Ok(lines)
}

/// webgl_lines_dashed's lines, turned by the elapsed time.
struct Dashed {
    objects: Vec<Object3D>,
    elapsed: f64,
}

impl super::example::Example for Dashed {
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
        let t = self.elapsed * 0.25;
        Some(self.objects.iter().try_for_each(|&object| {
            scene.get_mut(object)?.quaternion =
                Quaternion::from_euler(glam::EulerRot::XYZ, t, t, 0.0);
            Ok(())
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

/// webgl_lines_colors's curves, turned by the elapsed time, under a camera
/// that follows the pointer.
struct ColorLines {
    objects: Vec<Object3D>,
    pointer: Vector2,
    elapsed: f64,
}

impl super::example::Example for ColorLines {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        Some((|| {
            if animate {
                self.elapsed += delta;
                let p = &mut scene.get_mut(camera)?.position;
                p.x += (self.pointer.x - p.x) * 0.05;
                p.y += (-self.pointer.y + 200.0 - p.y) * 0.05;
            }
            scene.look_at(camera, Vector3::ZERO)?;
            for (i, &object) in self.objects.iter().enumerate() {
                scene.get_mut(object)?.quaternion = Quaternion::from_rotation_y(
                    self.elapsed * 0.5 * if i % 2 == 1 { 1.0 } else { -1.0 },
                );
            }
            Ok(())
        })())
    }
    fn on_pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
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

/// The program of three.js examples/webgl_lines_dashed.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_lines_dashed(
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
                super::gltf_viewer::OrbitViewer::from_camera(Vector3::ZERO, 150.0),
                1.0,
                200.0,
                Box::new(Dashed {
                    objects: dashed(scene, camera, aspect)?,
                    elapsed: 0.0,
                }),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_lines_colors.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_lines_colors(
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
                super::gltf_viewer::OrbitViewer::from_camera(Vector3::ZERO, 1000.0),
                1.0,
                10000.0,
                Box::new(ColorLines {
                    objects: colors(scene, camera, aspect)?,
                    pointer: Vector2::ZERO,
                    elapsed: 0.0,
                }),
            ))
        },
    )
    .await
}
