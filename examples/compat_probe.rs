use three_rs_wasm::{attribute::*, camera::*, geometry::*, math::*, scene::*};
fn main() -> three_rs_wasm::Result<()> {
    let mut scene = Scene::new();
    let parent = scene.insert(NodeKind::Group);
    let child = scene.insert(NodeKind::Group);
    scene.get_mut(parent)?.position = Vector3::new(2.0, -1.0, 3.0);
    scene.get_mut(parent)?.quaternion = Euler {
        angles: Vector3::new(0.2, -0.4, 0.3),
        order: EulerOrder::XYZ,
    }
    .quaternion();
    scene.get_mut(child)?.position = Vector3::new(1.0, 2.0, -3.0);
    scene.get_mut(child)?.scale = Vector3::new(2.0, 1.0, 0.5);
    scene.add(parent, child)?;
    scene.update()?;
    let world = scene.get(child)?.matrix_world.to_cols_array();
    let other = scene.insert(NodeKind::Group);
    scene.get_mut(other)?.position = Vector3::new(-4.0, 2.0, 1.0);
    scene.attach(other, child)?;
    let attached = scene.get(child)?.matrix_world.to_cols_array();
    let mut geometry = BoxGeometry::build(2.0, 3.0, 4.0)?;
    geometry.rotate_y(0.3)?;
    geometry.translate(Vector3::new(1.0, 2.0, 3.0))?;
    let bounds = geometry.compute_bounding_box()?;
    let sphere = geometry.compute_bounding_sphere()?;
    let mut normalized = Int8BufferAttribute::new(vec![-128, -127, 0, 127], 1, true)?;
    let decoded: Vec<_> = (0..4)
        .map(|i| normalized.get_component(i, 0).unwrap())
        .collect();
    for (i, v) in [-1.0, -0.5, 0.5, 1.0].iter().enumerate() {
        normalized.set_component(i, 0, *v)?;
    }
    let plane = PlaneGeometry::build(3.0, 2.0, 2, 3)?;
    let sphere_geometry = SphereGeometry::build(2.0, 8, 4)?;
    let perspective = PerspectiveCamera {
        fov: 60.0,
        aspect: 1.5,
        near: 0.2,
        far: 100.0,
        ..Default::default()
    };
    let orthographic = OrthographicCamera {
        left: -2.0,
        right: 3.0,
        top: 4.0,
        bottom: -1.0,
        near: 0.1,
        far: 80.0,
        zoom: 1.5,
        ..Default::default()
    };
    let output = serde_json::json!({
        "world":world,"attached":attached,"bounds":[bounds.min.to_array(),bounds.max.to_array()],"sphere":[sphere.center.x,sphere.center.y,sphere.center.z,sphere.radius],
        "normalized_decoded":decoded,"normalized_encoded":normalized.array(),
        "plane_positions":plane.positions()?.iter().map(|p|p.to_array()).collect::<Vec<_>>(),"plane_index":plane.index,
        "sphere_positions":sphere_geometry.positions()?.iter().map(|p|p.to_array()).collect::<Vec<_>>(),"sphere_index":sphere_geometry.index,
        "perspective":perspective.projection_matrix()?.to_cols_array(),"orthographic":orthographic.projection_matrix()?.to_cols_array(),
        "color":Color::from_hex(0x348ac1).0.to_array()
    });
    println!("{output}");
    Ok(())
}
