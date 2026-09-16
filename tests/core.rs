use std::{cell::RefCell, rc::Rc, sync::Arc};
use three_rs_wasm::{
    attribute::*, camera::*, geometry::*, material::*, math::*, raycast::*, scene::*,
};

#[test]
fn interleaved_views_reject_replaced_incompatible_layouts() {
    use std::sync::RwLock;
    let data = Arc::new(RwLock::new(
        InterleavedBuffer::new(vec![1.0_f32, 2.0, 3.0, 4.0], 4).unwrap(),
    ));
    let mut view = InterleavedBufferAttribute::new(data.clone(), 1, 3, false).unwrap();
    data.write()
        .unwrap()
        .storage
        .copy_from(&BufferAttribute::new(vec![5.0, 6.0], 1, false).unwrap());
    assert!(view.get_component(0, 0).is_err());
    assert!(view.set_component(0, 0, 7.0).is_err());
    assert!(view.to_attribute().is_err());
}

#[test]
fn typed_array_wrapping_half_float_and_euler_orders() {
    assert_eq!(i8::encode(128.0, false), -128);
    assert_eq!(u8::encode(-1.0, false), 255);
    assert_eq!(i16::encode(f64::INFINITY, false), 0);
    assert_eq!(half::f16::encode(1.0009, false).to_f64(), 1.0);
    assert_eq!(half::f16::encode(1e10, false).to_f64(), 65504.0);
    for order in [
        EulerOrder::XYZ,
        EulerOrder::YXZ,
        EulerOrder::ZXY,
        EulerOrder::ZYX,
        EulerOrder::YZX,
        EulerOrder::XZY,
    ] {
        let angles = Vector3::new(0.2, -0.4, 0.7);
        let q = Euler { angles, order }.quaternion();
        let back = Euler::from_quaternion(q, order);
        assert!((angles - back.angles).length() < 1e-12);
    }
}

#[test]
fn event_removal_during_dispatch_uses_a_snapshot() {
    use three_rs_wasm::event::EventDispatcher;
    let events = EventDispatcher::<i32>::default();
    let sum = Rc::new(RefCell::new(0));
    let second = Rc::new(RefCell::new(None));
    let second_ref = second.clone();
    events.add_event_listener(move |_, dispatcher| {
        dispatcher.remove_event_listener(second_ref.borrow().unwrap());
    });
    let sum_ref = sum.clone();
    *second.borrow_mut() = Some(events.add_event_listener(move |v, _| *sum_ref.borrow_mut() += v));
    events.dispatch_event(&3);
    events.dispatch_event(&7);
    assert_eq!(*sum.borrow(), 3);
    assert!(!events.has_event_listener(second.borrow().unwrap()));
    let other = EventDispatcher::<i32>::default();
    let foreign = other.add_event_listener(|_, _| {});
    assert!(!events.has_event_listener(foreign));
}

#[test]
fn timers_use_repeatable_delta_queries() {
    use three_rs_wasm::time::{Clock, Timer};
    let mut clock = Clock::default();
    assert_eq!(clock.delta_at(10.0), 0.0);
    assert_eq!(clock.delta_at(10.25), 0.25);
    let mut timer = Timer::default();
    timer.update_at(1.0);
    timer.set_timescale(0.5);
    timer.update_at(3.0);
    assert_eq!(timer.get_delta(), 1.0);
    assert_eq!(timer.get_delta(), 1.0);
    assert_eq!(timer.get_elapsed(), 2.0);
}

#[test]
fn geometry_clone_preserves_internal_aliasing_but_not_source_storage() {
    use std::sync::RwLock;
    let data = Arc::new(RwLock::new(
        InterleavedBuffer::new(vec![1.0_f32, 2.0, 3.0, 4.0], 4).unwrap(),
    ));
    let position = InterleavedBufferAttribute::new(data.clone(), 3, 0, false).unwrap();
    let extra = InterleavedBufferAttribute::new(data.clone(), 1, 3, false).unwrap();
    let mut original = BufferGeometry::default();
    original.set_attribute("position", Attribute::InterleavedF32(position));
    original.set_attribute("extra", Attribute::InterleavedF32(extra));
    let mut copied = original.clone();
    copied
        .attributes
        .get_mut("position")
        .unwrap()
        .set_component(0, 0, 9.0)
        .unwrap();
    assert_eq!(
        original.attributes["position"].get_component(0, 0).unwrap(),
        1.0
    );
    assert_ne!(original.identity.uuid, copied.identity.uuid);
    if let (Attribute::InterleavedF32(position), Attribute::InterleavedF32(extra)) =
        (&copied.attributes["position"], &copied.attributes["extra"])
    {
        assert!(Arc::ptr_eq(&position.data, &extra.data));
        assert!(!Arc::ptr_eq(&position.data, &data));
    } else {
        panic!("interleaved layout was lost");
    }
}

#[test]
fn manual_world_matrix_update_respects_r186_dirty_and_force_flags() {
    let mut scene = Scene::new();
    let h = scene.insert(NodeKind::Group);
    let node = scene.get_mut(h).unwrap();
    node.matrix_auto_update = false;
    node.matrix = Matrix4::from_translation(Vector3::X);
    scene.update_world_matrix(h, false, false).unwrap();
    assert_eq!(scene.get(h).unwrap().world_position(), Vector3::ZERO);
    scene
        .update_world_matrix_force(h, false, false, true)
        .unwrap();
    assert_eq!(scene.get(h).unwrap().world_position(), Vector3::X);
    assert!(!scene.get(h).unwrap().matrix_world_needs_update);
}

#[test]
fn deep_scene_update_and_dispose_do_not_use_the_call_stack() {
    let mut scene = Scene::new();
    let root = scene.insert(NodeKind::Group);
    let mut parent = root;
    for _ in 0..2000 {
        let child = scene.insert(NodeKind::Group);
        scene.get_mut(child).unwrap().position.x = 1.0;
        scene.add(parent, child).unwrap();
        parent = child;
    }
    scene.update().unwrap();
    assert_eq!(scene.get(parent).unwrap().world_position().x, 2000.0);
    scene.dispose(root).unwrap();
    assert!(scene.is_empty());
}

#[test]
fn invalid_camera_inputs_fail_without_nan_projection() {
    let bad = PerspectiveCamera {
        aspect: f64::NAN,
        ..Default::default()
    };
    assert!(bad.projection_matrix().is_err());
    let infinite = PerspectiveCamera {
        far: f64::INFINITY,
        ..Default::default()
    };
    assert!(infinite.projection_matrix().unwrap().is_finite());
}

#[test]
fn empty_geometry_bounds_and_existing_points_follow_three() {
    let mut geometry = BufferGeometry::default();
    assert!(geometry.compute_bounding_box().unwrap().is_empty());
    assert_eq!(geometry.compute_bounding_sphere().unwrap().radius, -1.0);
    geometry.set_from_points(&[Vector3::X, Vector3::Y]).unwrap();
    geometry.set_from_points(&[Vector3::Z]).unwrap();
    assert_eq!(geometry.positions().unwrap(), vec![Vector3::Z, Vector3::Y]);
    let mut indexed = PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap();
    indexed.name = "source".into();
    indexed.set_draw_range(3, Some(3));
    indexed.compute_bounding_box().unwrap();
    let expanded = indexed.to_non_indexed().unwrap();
    assert!(expanded.name.is_empty());
    assert_eq!(expanded.draw_range, DrawRange::default());
    assert!(expanded.bounding_box.is_none());
}

#[test]
fn graph_reparent_cycles_stale_and_cross_scene_handles() {
    let mut scene = Scene::new();
    let a = scene.insert(NodeKind::Group);
    let b = scene.insert(NodeKind::Group);
    let c = scene.insert(NodeKind::Group);
    scene.add(a, b).unwrap();
    scene.add(b, c).unwrap();
    assert!(scene.add(c, a).is_err());
    assert_eq!(scene.traverse(a, false).unwrap(), vec![a, b, c]);
    scene.get_mut(b).unwrap().visible = false;
    assert_eq!(scene.traverse(a, true).unwrap(), vec![a]);
    scene.add(a, c).unwrap();
    assert!(scene.get(b).unwrap().children().is_empty());
    let mut foreign = Scene::new();
    let f = foreign.insert(NodeKind::Group);
    assert!(scene.get(f).is_err());
    scene.dispose(b).unwrap();
    let reused = scene.insert(NodeKind::Group);
    assert_ne!(b, reused);
    assert!(scene.get(b).is_err());
    let copy = scene.clone_subtree(a, true).unwrap();
    assert_eq!(scene.traverse(copy, false).unwrap().len(), 2);
}

#[test]
fn resource_lifecycle_reuses_slots_and_releases_shared_assets() {
    let mut scene = Scene::new();
    let geometry = Arc::new(BoxGeometry::build(1.0, 1.0, 1.0).unwrap());
    let material = Arc::new(Material::default());
    for _ in 0..1000 {
        let root = scene.insert(NodeKind::Group);
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            material.clone(),
        )));
        scene.add(root, mesh).unwrap();
        scene.dispose(root).unwrap();
        assert!(scene.is_empty());
    }
    assert_eq!(scene.allocated_slots(), 2);
    assert_eq!(Arc::strong_count(&geometry), 1);
    assert_eq!(Arc::strong_count(&material), 1);
}

#[test]
fn attribute_validation_normalization_interleaving_and_versions() {
    assert!(Float32BufferAttribute::new(vec![1.0], 0, false).is_err());
    assert!(Float32BufferAttribute::new(vec![1.0], 3, false).is_err());
    let mut a = Uint8ClampedBufferAttribute::new(vec![ClampedU8(0); 4], 1, false).unwrap();
    for (i, v) in [-1.0, 2.5, 3.5, 300.0].into_iter().enumerate() {
        a.set_component(i, 0, v).unwrap();
    }
    assert_eq!(
        a.array().iter().map(|v| v.0).collect::<Vec<_>>(),
        vec![0, 2, 4, 255]
    );
    let data = Arc::new(std::sync::RwLock::new(
        InterleavedBuffer::new(vec![1.0_f32, 2.0, 3.0, 9.0, 4.0, 5.0, 6.0, 8.0], 4).unwrap(),
    ));
    let mut position = InterleavedBufferAttribute::new(data.clone(), 3, 0, false).unwrap();
    let extra = InterleavedBufferAttribute::new(data.clone(), 1, 3, false).unwrap();
    position
        .apply_matrix4(Matrix4::from_translation(Vector3::ONE))
        .unwrap();
    assert_eq!(position.get_component(0, 0).unwrap(), 2.0);
    assert_eq!(extra.get_component(0, 0).unwrap(), 9.0);
    assert!(data.read().unwrap().storage.version() > 0);
    assert!(InterleavedBufferAttribute::new(data, 2, 3, false).is_err());
}

#[test]
fn geometry_normals_tangents_groups_and_bounds() {
    let mut plane = PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap();
    plane.compute_vertex_normals().unwrap();
    plane.compute_tangents().unwrap();
    assert_eq!(plane.attributes["normal"].vector3(0).unwrap(), Vector3::Z);
    assert_eq!(plane.attributes["tangent"].vector3(0).unwrap(), Vector3::X);
    let expanded = plane.to_non_indexed().unwrap();
    assert_eq!(expanded.vertex_count(), 6);
    assert!(expanded.index.is_none());
    assert_eq!(
        plane.compute_bounding_box().unwrap(),
        Box3 {
            min: Vector3::new(-1.0, -1.0, 0.0),
            max: Vector3::new(1.0, 1.0, 0.0)
        }
    );
}

#[test]
fn raycasting_mesh_line_points_and_layers() {
    let mut scene = Scene::new();
    let geometry = Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap());
    let material = Arc::new(Material::default());
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(geometry, material.clone())));
    scene.update().unwrap();
    let raycaster = Raycaster {
        ray: Ray {
            origin: Vector3::new(0.25, 0.1, 5.0),
            direction: Vector3::NEG_Z,
        },
        ..Default::default()
    };
    let hits = raycaster.intersect_object(&scene, mesh, false).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].distance, 5.0);
    assert!((hits[0].uv.unwrap() - Vector2::new(0.625, 0.55)).length() < 1e-12);
    scene.get_mut(mesh).unwrap().layers.set(2);
    assert!(
        raycaster
            .intersect_object(&scene, mesh, false)
            .unwrap()
            .is_empty()
    );
    let mut g = BufferGeometry::default();
    g.set_from_points(&[Vector3::new(-1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)])
        .unwrap();
    let g = Arc::new(g);
    let line = scene.insert(NodeKind::Line(Line {
        geometry: g.clone(),
        material: material.clone(),
        segments: false,
    }));
    let points = scene.insert(NodeKind::Points(Points {
        geometry: g,
        material,
    }));
    scene.update().unwrap();
    assert_eq!(
        raycaster
            .intersect_object(&scene, line, false)
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        raycaster
            .intersect_object(&scene, points, false)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn camera_webgpu_depth_and_ray() {
    let c = PerspectiveCamera::default();
    let m = c.projection_matrix().unwrap();
    assert!(m.project_point3(Vector3::new(0.0, 0.0, -c.near)).z.abs() < 1e-12);
    assert!((m.project_point3(Vector3::new(0.0, 0.0, -c.far)).z - 1.0).abs() < 1e-12);
    let ray = Camera::Perspective(c)
        .ray(Vector2::ZERO, Matrix4::from_translation(Vector3::Z * 5.0))
        .unwrap();
    assert_eq!(ray.origin, Vector3::Z * 5.0);
    assert_eq!(ray.direction, Vector3::NEG_Z);
}
