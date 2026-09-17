//! Direct Rust port of the static sphere grid in webgpu_pmrem_equirectangular.
use super::gltf_viewer::{OrbitViewer, fetch};
use crate::{
    Result, camera::*, environment::EnvironmentMap, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

pub(super) async fn pmrem_grid(
    scene: &mut Scene,
    camera: Object3D,
    placeholder: Object3D,
) -> Result<OrbitViewer> {
    scene.dispose(placeholder)?;
    scene.environment = Some(Arc::new(EnvironmentMap::from_hdr(
        &fetch("/web/environments/royal_esplanade_2k.hdr").await?,
    )?));
    scene.background_environment = true;
    scene.background_blur = 0.5;
    scene.aces_tone_mapping = true;
    let geometry = Arc::new(SphereGeometry::build(0.4, 64, 64)?);
    for i in 0..6 {
        for j in 0..5 {
            let material = MeshStandardMaterial {
                roughness: i as f64 / 5.0,
                metalness: j as f64 / 4.0,
                ..Default::default()
            };
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Standard(material)),
            )));
            scene.get_mut(mesh)?.position = Vector3::new(i as f64 - 2.5, j as f64 - 2.0, 0.0);
        }
    }
    if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
        p.fov = 45.0;
    }
    Ok(OrbitViewer::from_camera(Vector3::ZERO, 8.0))
}
