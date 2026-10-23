//! The MVP page's own scenes ( web/index.html ): the placeholder cube that
//! turns, is picked and is rebuilt, and the lights, texture, line and points
//! scene beside it.
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

/// The page's cube, turning on Y with the page's clock, until it is picked.
struct Cube {
    mesh: Object3D,
    rebuilds: u32,
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
        let time = web_sys::window()
            .and_then(|w| w.performance())
            .map_or(0.0, |p| p.now());
        Some(
            scene
                .get_mut(self.mesh)
                .map(|n| n.quaternion = Quaternion::from_rotation_y(time * 0.0003)),
        )
    }
    fn on_pick(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        ndc: Vector2,
    ) -> Option<Result<bool>> {
        Some(super::pick(scene, camera, self.mesh, ndc))
    }
    /// Rebuild a small scene fragment, exercising owned geometry replacement,
    /// groups/draw ranges, attribute edits and stale-handle protection.
    fn on_rebuild(&mut self, scene: &mut Scene) -> Option<Result<u32>> {
        Some((|| {
            let original = self.mesh;
            let mut geometry = PlaneGeometry::build(2.0, 2.0, 1, 1)?;
            geometry.clear_groups();
            geometry.add_group(0, 3, 0);
            geometry.add_group(3, 3, 1);
            self.rebuilds += 1;
            geometry.set_draw_range(
                0,
                Some(if self.rebuilds.is_multiple_of(2) {
                    6
                } else {
                    3
                }),
            );
            if let Some(position) = geometry.attributes.get_mut("position") {
                position.set_component(0, 0, -0.8)?;
            }
            let mut first = Material::default();
            first.properties_mut().color = Color::from_hex(0xff3366);
            let mut second = Material::default();
            second.properties_mut().color = Color::from_hex(0x33ccff);
            let replacement = scene.insert(NodeKind::Mesh(Mesh {
                geometry: Arc::new(geometry),
                materials: vec![Arc::new(first), Arc::new(second)],
            }));
            let group = scene.insert(NodeKind::Group);
            scene.add(group, replacement)?;
            scene.remove_from_parent(replacement)?;
            scene.dispose(group)?;
            scene.dispose(original)?;
            if scene.get(original).is_ok() {
                return Err(Error::Invalid("stale handle retained"));
            }
            self.mesh = replacement;
            Ok(self.rebuilds)
        })())
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

async fn lights_textures(page: &mut super::Page) -> Result<()> {
    let (scene, camera, mesh) = (&mut page.scene, page.camera, page.mesh);
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -3.0,
        right: 3.0,
        top: 3.0,
        bottom: -3.0,
        ..Default::default()
    }));
    scene.get_mut(mesh)?.position.x = -1.1;
    let group = scene.insert(NodeKind::Group);
    scene.get_mut(group)?.position = Vector3::new(0.8, 0.6, 0.0);
    let texture = Arc::new(Texture::load("textures/checker.png").await?);
    let mut standard = MeshStandardMaterial::default();
    standard.properties.map = Some(texture);
    standard.roughness = 0.7;
    let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(0.75, 24, 16)?),
        Arc::new(Material::Standard(standard)),
    )));
    scene.add(group, sphere)?;
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::WHITE,
        intensity: 0.3,
    }));
    let directional = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 2.0,
        target: Vector3::ZERO,
    }));
    scene.get_mut(directional)?.position = Vector3::new(2.0, 3.0, 4.0);
    let point = scene.insert(NodeKind::Light(Light::Point {
        color: Color::from_hex(0xff8040),
        intensity: 10.0,
        distance: 0.0,
        decay: 2.0,
    }));
    scene.get_mut(point)?.position = Vector3::new(-2.0, 1.0, 2.0);
    let mut line_geometry = BufferGeometry::default();
    line_geometry.set_from_points(&[
        Vector3::new(-2.0, -1.5, 0.0),
        Vector3::new(0.0, -0.9, 0.0),
        Vector3::new(2.0, -1.5, 0.0),
    ])?;
    let mut line_material = LineBasicMaterial::default();
    line_material.properties.color = Color::from_hex(0x44ff88);
    scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(line_geometry.clone()),
        material: Arc::new(Material::Line(line_material)),
        segments: false,
    }));
    let mut points_material = PointsMaterial::default();
    points_material.properties.color = Color::from_hex(0xffffff);
    points_material.size_attenuation = false;
    let points = scene.insert(NodeKind::Points(Points {
        geometry: Arc::new(line_geometry),
        material: Arc::new(Material::Points(points_material)),
    }));
    scene.get_mut(points)?.position.y = -0.4;
    Ok(())
}

/// The MVP page's first scene: the cube that turns and is picked.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn mvp_rotate_select(
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
            Ok(super::expanded::own(Box::new(Cube {
                mesh: page.mesh,
                rebuilds: 0,
            })))
        },
    )
    .await
}

/// The MVP page's lights, texture, line and points scene.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn mvp_lights_textures(
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
            lights_textures(page).await?;
            Ok(super::expanded::own(Box::new(Cube {
                mesh: page.mesh,
                rebuilds: 0,
            })))
        },
    )
    .await
}
