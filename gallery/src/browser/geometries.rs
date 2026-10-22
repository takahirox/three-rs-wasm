//! three.js examples/webgl_geometries.html: the geometry builders on a
//! UV-grid Phong material, under a camera circling the grid.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct Geometries {
    shapes: Vec<Object3D>,
    elapsed: f64,
}

impl super::example::Example for Geometries {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if animate {
            self.elapsed += delta;
        }
        let t = self.elapsed * 0.1;
        Some((|| {
            scene.get_mut(camera)?.position = Vector3::new(t.cos() * 800.0, 500.0, t.sin() * 800.0);
            scene.look_at(camera, Vector3::ZERO)?;
            for &mesh in &self.shapes {
                scene.get_mut(mesh)?.quaternion =
                    Quaternion::from_euler(glam::EulerRot::XYZ, t * 5.0, t * 2.5, 0.0);
            }
            Ok(())
        })())
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

async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<super::expanded::Demo> {
    use std::f64::consts::{PI, TAU};
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.0,
        aspect,
        near: 1.0,
        far: 2000.0,
        ..Default::default()
    }));
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::from_hex(0xcccccc),
        intensity: 1.5,
    }));
    let point = scene.insert(NodeKind::Light(Light::Point {
        color: Color::WHITE,
        intensity: 2.5,
        distance: 0.0,
        decay: 0.0,
    }));
    scene.add(camera, point)?;
    let mut map = decode_image(&fetch("/web/gallery/assets/uv-grid.jpg").await?).await?;
    map.wrap_s = Wrapping::Repeat;
    map.wrap_t = Wrapping::Repeat;
    map.anisotropy = 16;
    map.srgb = true;
    map.mipmap_filter = Some(Filter::Linear);
    let mut material = MeshPhongMaterial::default();
    material.properties.map = Some(Arc::new(map));
    material.properties.side = Side::Double;
    let material = Arc::new(Material::Phong(material));
    let points = (0..50)
        .map(|i| {
            let i = i as f64;
            Vector2::new(
                (i * 0.2).sin() * (i * 0.1).sin() * 15.0 + 50.0,
                (i - 5.0) * 2.0,
            )
        })
        .collect::<Vec<_>>();
    let mut plane = ParametricGeometry::build(|u, v| Vector3::new(u, 0.0, v), 10, 10)?;
    plane.scale(Vector3::splat(100.0))?;
    plane.center()?;
    let klein = ParametricGeometry::build(
        |v, u| {
            let u = u * TAU;
            let v = v * TAU;
            let r = 2.0 * (1.0 - u.cos() / 2.0);
            let (x, z) = if u < PI {
                (
                    3.0 * u.cos() * (1.0 + u.sin()) + r * u.cos() * v.cos(),
                    -8.0 * u.sin() - r * u.sin() * v.cos(),
                )
            } else {
                (
                    3.0 * u.cos() * (1.0 + u.sin()) + r * (v + PI).cos(),
                    -8.0 * u.sin(),
                )
            };
            Vector3::new(x, -r * v.sin(), z)
        },
        20,
        20,
    )?;
    let mobius = ParametricGeometry::build(
        |u, t| {
            let u = u - 0.5;
            let v = t * TAU;
            Vector3::new(
                v.cos() * (2.0 + u * (v / 2.0).cos()),
                v.sin() * (2.0 + u * (v / 2.0).cos()),
                u * (v / 2.0).sin(),
            )
        },
        20,
        20,
    )?;
    let geometries = [
        SphereGeometry::build(75.0, 20, 10)?,
        IcosahedronGeometry::build(75.0, 0)?,
        OctahedronGeometry::build(75.0, 0)?,
        TetrahedronGeometry::build(75.0, 0)?,
        PlaneGeometry::build(100.0, 100.0, 4, 4)?,
        BoxGeometry::segmented(100.0, 100.0, 100.0, 4, 4, 4)?,
        CircleGeometry::build(50.0, 20, 0.0, TAU)?,
        RingGeometry::build(10.0, 50.0, 20, 5, 0.0, TAU)?,
        CylinderGeometry::build(25.0, 75.0, 100.0, 40, 5, false, 0.0, TAU)?,
        LatheGeometry::build(&points, 20, 0.0, TAU)?,
        TorusGeometry::build(50.0, 20.0, 20, 20, TAU, 0.0, TAU)?,
        TorusKnotGeometry::build(50.0, 10.0, 50, 20, 2, 3)?,
        CapsuleGeometry::build(20.0, 50.0, 4, 8, 1)?,
        plane,
        klein,
        mobius,
    ];
    let mut shapes = Vec::new();
    for (i, geometry) in geometries.into_iter().enumerate() {
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            material.clone(),
        )));
        scene.get_mut(mesh)?.position = Vector3::new(
            (i % 4) as f64 * 200.0 - 300.0,
            0.0,
            300.0 - (i / 4) as f64 * 200.0,
        );
        if i == 14 {
            scene.get_mut(mesh)?.scale = Vector3::splat(5.0);
        }
        if i == 15 {
            scene.get_mut(mesh)?.scale = Vector3::splat(30.0);
        }
        shapes.push(mesh);
    }
    Ok(super::expanded::module(
        OrbitViewer::from_camera(Vector3::ZERO, 800.0),
        1.0,
        2000.0,
        Box::new(Geometries {
            shapes,
            elapsed: 0.0,
        }),
    ))
}

/// The program of three.js examples/webgl_geometries.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometries(
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
