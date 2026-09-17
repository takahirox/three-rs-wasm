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
