use std::sync::Arc;
use three_rs_wasm::{
    camera::*, geometry::*, material::*, math::*, raycast::Raycaster, renderer::*, scene::*,
};
#[test]
fn instance_transforms_colors_and_ray_ids_agree() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -2.0,
        right: 2.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.1,
        far: 10.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 2.0;
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap()),
        Arc::new(Material::default()),
    )));
    scene.get_mut(mesh).unwrap().instances = vec![
        Instance {
            matrix: Matrix4::from_translation(-Vector3::X),
            color: Color::linear(1.0, 0.0, 0.0),
        },
        Instance {
            matrix: Matrix4::from_translation(Vector3::X),
            color: Color::linear(0.0, 1.0, 0.0),
        },
    ];
    let target = RenderTarget::new(&renderer.device, 64, 32).unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(
        &pixels[(16 * 64 + 16) * 4..(16 * 64 + 16) * 4 + 4],
        &[255, 0, 0, 255]
    );
    assert_eq!(
        &pixels[(16 * 64 + 48) * 4..(16 * 64 + 48) * 4 + 4],
        &[0, 255, 0, 255]
    );
    let mut ray = Raycaster::default();
    ray.set(Vector3::new(1.0, 0.0, 2.0), -Vector3::Z);
    let hits = ray.intersect_object(&scene, mesh, false).unwrap();
    assert!(!hits.is_empty());
    assert!(hits.iter().all(|h| h.instance_index == Some(1)));
}
