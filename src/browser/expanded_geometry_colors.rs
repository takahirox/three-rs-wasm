use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;
pub(super) fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<()> {
    scene.background = Color::WHITE;
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 20.0,
        aspect,
        near: 1.0,
        far: 10000.0,
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 1800.0);
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 3.0,
        target: Vector3::ZERO,
    }));
    scene.get_mut(light)?.position = Vector3::Z;
    let mut pixels = Vec::with_capacity(128 * 128 * 4);
    for y in 0..128 {
        for x in 0..128 {
            let distance =
                Vector2::new(x as f64 + 0.5 - 64.0, y as f64 + 0.5 - 64.0).length() / 64.0;
            let c = (210.0 + 45.0 * ((distance - 0.1) / 0.9).clamp(0.0, 1.0)).round() as u8;
            pixels.extend([c, c, c, 255]);
        }
    }
    let mut texture = Texture::from_rgba(128, 128, pixels, false)?;
    texture.mipmap_filter = Some(Filter::Linear);
    let mut shadow = Material::default();
    shadow.properties_mut().map = Some(Arc::new(texture));
    let shadow = Arc::new(shadow);
    let plane = Arc::new(PlaneGeometry::build(300.0, 300.0, 1, 1)?);
    for x in [0.0, -400.0, 400.0] {
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(plane.clone(), shadow.clone())));
        scene.get_mut(mesh)?.position = Vector3::new(x, -250.0, 0.0);
        scene.get_mut(mesh)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
    }
    let mut material = MeshPhongMaterial {
        shininess: 0.0,
        ..Default::default()
    };
    material.properties.flat_shading = true;
    material.properties.vertex_colors = true;
    let material = Arc::new(Material::Phong(material));
    let mut wire = Material::default();
    wire.properties_mut().color = Color::BLACK;
    wire.properties_mut().wireframe = true;
    wire.properties_mut().transparent = true;
    let wire = Arc::new(wire);
    for (i, x) in [-400.0, 400.0, 0.0].into_iter().enumerate() {
        let mut geometry = IcosahedronGeometry::build(200.0, 1)?;
        let mut colors = Vec::new();
        for p in geometry.positions()? {
            let ratio = (p.y / 200.0 + 1.0) / 2.0;
            let rgb = match i {
                0 => Color::from_hsl(ratio, 1.0, 0.5).0,
                1 => Color::from_hsl(0.0, ratio, 0.5).0,
                _ => Vector3::new(1.0, 0.8 - ratio, 0.0),
            };
            colors.extend(rgb.map(srgb_to_linear).to_array().map(half::f16::from_f64));
        }
        geometry.set_attribute(
            "color",
            Attribute::F16(BufferAttribute::new(colors, 3, false)?),
        );
        let geometry = Arc::new(geometry);
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            material.clone(),
        )));
        scene.get_mut(mesh)?.position.x = x;
        if i == 0 {
            scene.get_mut(mesh)?.quaternion = Quaternion::from_rotation_x(-1.87);
        }
        let overlay = scene.insert(NodeKind::Mesh(Mesh::new(geometry, wire.clone())));
        scene.add(mesh, overlay)?;
    }
    Ok(())
}
