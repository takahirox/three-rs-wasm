use crate::{Result, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;
pub(super) fn rect_area(scene: &mut Scene) -> Result<Vec<Object3D>> {
    let mut lights = Vec::new();
    for (i, color) in [0xff0000, 0x00ff00, 0x0000ff].into_iter().enumerate() {
        let color = Color::from_hex(color);
        let light = scene.insert(NodeKind::Light(Light::RectArea {
            color,
            intensity: 5.0,
            width: 4.0,
            height: 10.0,
        }));
        scene.get_mut(light)?.position = Vector3::new(i as f64 * 5.0 - 5.0, 6.0, 5.0);
        let mut border = BufferGeometry::default();
        border.set_from_points(&[
            Vector3::new(2.0, 5.0, 0.0),
            Vector3::new(-2.0, 5.0, 0.0),
            Vector3::new(-2.0, -5.0, 0.0),
            Vector3::new(2.0, -5.0, 0.0),
            Vector3::new(2.0, 5.0, 0.0),
        ])?;
        let mut material = LineBasicMaterial::default();
        material.properties.color = color;
        material.properties.fog = false;
        let helper = scene.insert(NodeKind::Line(Line {
            geometry: Arc::new(border),
            material: Arc::new(Material::Line(material)),
            segments: false,
        }));
        scene.add(light, helper)?;
        let mut material = Material::default();
        material.properties_mut().color = color;
        material.properties_mut().side = Side::Back;
        material.properties_mut().fog = false;
        let plane = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(4.0, 10.0, 1, 1)?),
            Arc::new(material),
        )));
        scene.add(light, plane)?;
        lights.push(light);
    }
    let mut checker = Texture::from_rgba(
        2,
        2,
        vec![
            255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255,
        ],
        false,
    )?;
    checker.filter = Filter::Nearest;
    checker.min_filter = Some(Filter::Linear);
    checker.mipmap_filter = Some(Filter::Linear);
    checker.repeat = Vector2::splat(400.0);
    checker.wrap_s = Wrapping::Repeat;
    checker.wrap_t = Wrapping::Repeat;
    let mut floor = MeshStandardMaterial::default();
    floor.properties.color = Color::from_hex(0x444444);
    floor.metallic_roughness_map = Some(Arc::new(checker));
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(2000.0, 0.1, 2000.0)?),
        Arc::new(Material::Standard(floor)),
    )));
    let knot = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(TorusKnotGeometry::build(1.5, 0.5, 200, 16, 2, 3)?),
        Arc::new(Material::Standard(MeshStandardMaterial {
            roughness: 0.0,
            metalness: 0.0,
            ..Default::default()
        })),
    )));
    scene.get_mut(knot)?.position = Vector3::new(0.0, 5.5, 0.0);
    Ok(lights)
}
