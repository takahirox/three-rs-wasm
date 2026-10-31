//! three.js examples/webgl_geometry_cube.html: the crate texture on a cube
//! turning on two axes.
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct Cube {
    mesh: Object3D,
    rotation: Vector3,
}

impl super::example::Example for Cube {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        _camera: Object3D,
        _delta: f64,
        animate: bool,
    ) -> Option<Result<()>> {
        if !animate {
            return Some(Ok(()));
        }
        self.rotation.x += 0.005;
        self.rotation.y += 0.01;
        let rotation = self.rotation;
        Some(scene.get_mut(self.mesh).map(|n| {
            n.quaternion = Euler {
                angles: rotation,
                order: EulerOrder::XYZ,
            }
            .quaternion()
        }))
    }
    fn on_pick(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        ndc: Vector2,
    ) -> Option<Result<bool>> {
        Some(super::pick(scene, camera, self.mesh, ndc))
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

/// Three.js r187 examples/webgl_geometry_cube.html, ported to WebGPU.
async fn create(page: &mut super::Page) -> Result<()> {
    let (scene, camera, mesh) = (&mut page.scene, page.camera, page.mesh);
    let aspect = page.canvas.width() as f64 / page.canvas.height() as f64;
    scene.background = Color::BLACK;
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 70.0,
        aspect,
        near: 0.1,
        far: 100.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position.z = 2.0;
    let mut material = Material::default();
    material.properties_mut().map = Some(Arc::new(Texture::load("textures/crate.gif").await?));
    scene.get_mut(mesh)?.kind = NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?),
        Arc::new(material),
    ));
    if !page.animate {
        scene.get_mut(mesh)?.quaternion = Euler {
            angles: Vector3::new(0.4, 0.7, 0.0),
            order: EulerOrder::XYZ,
        }
        .quaternion();
    }
    Ok(())
}

/// The program of three.js examples/webgl_geometry_cube.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_cube(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 0,
            format: super::TargetFormat::CanvasSrgb,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            create(page).await?;
            Ok(super::expanded::own(Box::new(Cube {
                mesh: page.mesh,
                rotation: Vector3::ZERO,
            })))
        },
    )
    .await
}
