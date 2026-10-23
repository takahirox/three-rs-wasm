//! three.js examples/webgl_materials_texture_rotation.html: a UV grid's
//! offset, repeat, rotation and center, from the page's controls.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

struct TextureRotation {
    mesh: Object3D,
    viewer: OrbitViewer,
}

impl super::example::Example for TextureRotation {
    fn on_update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        _delta: f64,
        _animate: bool,
    ) -> Option<Result<()>> {
        Some(self.viewer.update(scene, camera))
    }
    fn on_texture_transform(&mut self, scene: &mut Scene, values: &[f64]) -> Option<Result<()>> {
        Some((|| {
            if values.len() != 7 || !values.iter().all(|v| v.is_finite()) {
                return Ok(());
            }
            if let NodeKind::Mesh(m) = &mut scene.get_mut(self.mesh)?.kind {
                let material = Arc::make_mut(&mut m.materials[0]);
                if let Some(map) = &mut material.properties_mut().map {
                    let texture = Arc::make_mut(map);
                    texture.offset = Vector2::new(values[0], values[1]);
                    texture.repeat = Vector2::new(values[2], values[3]);
                    texture.rotation = values[4];
                    texture.center = Vector2::new(values[5], values[6]);
                }
            }
            Ok(())
        })())
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
        far: 1000.0,
        ..Default::default()
    }));
    let position = Vector3::new(10.0, 15.0, 25.0);
    let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
    viewer.fixture(
        position.x.atan2(position.z),
        (position.y / position.length()).asin(),
        1.8,
    );
    let mut texture = decode_image(&fetch("assets/uv-grid.jpg").await?).await?;
    texture.anisotropy = 16;
    texture.wrap_s = Wrapping::Repeat;
    texture.wrap_t = Wrapping::Repeat;
    texture.mipmap_filter = Some(Filter::Linear);
    texture.repeat = Vector2::splat(0.25);
    texture.center = Vector2::splat(0.5);
    texture.rotation = std::f64::consts::FRAC_PI_4;
    let mut material = MeshBasicMaterial::default();
    material.properties.map = Some(Arc::new(texture));
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(10.0, 10.0, 10.0)?),
        Arc::new(Material::Basic(material)),
    )));
    Ok(super::expanded::own(Box::new(TextureRotation {
        mesh,
        viewer,
    })))
}

/// The program of three.js examples/webgl_materials_texture_rotation.html: its renderer, its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_materials_texture_rotation(
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
