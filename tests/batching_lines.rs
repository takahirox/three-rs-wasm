use std::sync::Arc;
use three_rs_wasm::{
    batching::*, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*,
};
#[test]
fn batch_preserves_transformed_bounds_indices_and_source() {
    let geometry = Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap());
    let entries = [
        BatchEntry {
            geometry: geometry.clone(),
            matrix: Matrix4::from_translation(-Vector3::X),
            visible: true,
        },
        BatchEntry {
            geometry: geometry.clone(),
            matrix: Matrix4::from_translation(Vector3::X),
            visible: true,
        },
    ];
    let mesh = build(&entries, Arc::new(Material::default())).unwrap();
    assert_eq!(mesh.geometry.draw_count(), 12);
    assert_eq!(mesh.geometry.vertex_count(), 8);
    let bounds = mesh.geometry.bounding_box.unwrap();
    assert_eq!(bounds.min.x, -1.5);
    assert_eq!(bounds.max.x, 1.5);
    assert!(
        geometry
            .positions()
            .unwrap()
            .iter()
            .all(|p| p.x.abs() <= 0.5)
    );
    assert!(
        build(
            &[BatchEntry {
                geometry,
                matrix: Matrix4::from_scale(Vector3::new(-1.0, 1.0, 1.0)),
                visible: true
            }],
            Arc::new(Material::default())
        )
        .is_err()
    );
}
#[test]
fn wide_dashed_line_has_screen_width_and_distance_gaps() {
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
    let mut geometry = BufferGeometry::default();
    geometry.set_attribute(
        "position",
        Attribute::F32(
            three_rs_wasm::attribute::BufferAttribute::new(
                vec![-2.0, 0.0, 0.0, 2.0, 0.0, 0.0],
                3,
                false,
            )
            .unwrap(),
        ),
    );
    let material = LineBasicMaterial {
        linewidth: 8.0,
        dash: Some(LineDash {
            size: 0.5,
            gap: 0.5,
            scale: 1.0,
            offset: 0.0,
        }),
        ..Default::default()
    };
    let line = scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(geometry),
        material: Arc::new(Material::Line(material)),
        segments: false,
    }));
    let target = RenderTarget::new(&renderer.device, 64, 32).unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    for x in [4, 20, 36, 52] {
        for y in 12..20 {
            assert_eq!(pixels[(y * 64 + x) * 4], 255);
        }
    }
    for x in [12, 28, 44, 60] {
        assert_eq!(pixels[(16 * 64 + x) * 4], 0);
    }
    for y in [10, 21] {
        assert_eq!(pixels[(y * 64 + 4) * 4], 0);
    }
    let uploads = renderer.transfer_counts();
    scene.get_mut(camera).unwrap().position.x = 0.125;
    let NodeKind::Line(line) = &mut scene.get_mut(line).unwrap().kind else {
        panic!()
    };
    let Material::Line(material) = Arc::make_mut(&mut line.material) else {
        panic!()
    };
    material.linewidth = 14.0;
    renderer.render(&mut scene, camera, &target).unwrap();
    assert_eq!(
        renderer.transfer_counts(),
        uploads,
        "camera/width changes must not rebuild endpoint geometry"
    );
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(pixels[(10 * 64 + 3) * 4], 255);
}

#[test]
fn nonindexed_wireframe_draws_edges_without_filling_triangle() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.1,
        far: 10.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 2.0;
    let mut geometry = BufferGeometry::default();
    geometry.set_attribute(
        "position",
        Attribute::F32(
            three_rs_wasm::attribute::BufferAttribute::new(
                vec![-0.8, -0.8, 0.0, 0.8, -0.8, 0.0, 0.0, 0.8, 0.0],
                3,
                false,
            )
            .unwrap(),
        ),
    );
    let mut material = Material::default();
    material.properties_mut().wireframe = true;
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(material),
    )));
    let target = RenderTarget::new(&renderer.device, 32, 32).unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(pixels[(16 * 32 + 16) * 4], 0);
    let visible = pixels
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|p| p[0] > 128)
        .count();
    assert!((40..100).contains(&visible));
}
