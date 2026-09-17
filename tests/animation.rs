use std::sync::Arc;
use three_rs_wasm::{
    animation::*, attribute::BufferAttribute, deformation::*, geometry::*, material::*, math::*,
    raycast::Raycaster, scene::*,
};
#[test]
fn keyframes_blend_loop_and_restore() {
    let mut scene = Scene::new();
    let target = scene.insert(NodeKind::Group);
    let track = Track {
        target,
        property: Property::Position,
        times: vec![0.0, 2.0],
        values: vec![vec![0.0; 3], vec![4.0, 0.0, 0.0]],
        interpolation: Interpolation::Linear,
    };
    let mut mixer = AnimationMixer::default();
    let action = mixer
        .play(Arc::new(Clip {
            name: "move".into(),
            tracks: vec![track.clone()],
        }))
        .unwrap();
    mixer.actions[action].weight = 0.5;
    mixer.update(&mut scene, 1.0).unwrap();
    assert_eq!(scene.get(target).unwrap().position.x, 1.0);
    mixer.update(&mut scene, 2.0).unwrap();
    assert_eq!(scene.get(target).unwrap().position.x, 1.0);
    mixer.actions[action].looping = LoopMode::Once;
    mixer.update(&mut scene, 0.0).unwrap();
    assert_eq!(scene.get(target).unwrap().position.x, 2.0);
    mixer.restore(&mut scene).unwrap();
    assert_eq!(scene.get(target).unwrap().position, Vector3::ZERO);
    let cubic = Track {
        values: vec![
            vec![0.0; 3],
            vec![0.0; 3],
            vec![2.0, 0.0, 0.0],
            vec![2.0, 0.0, 0.0],
            vec![4.0, 0.0, 0.0],
            vec![0.0; 3],
        ],
        interpolation: Interpolation::CubicSpline,
        ..track
    };
    assert_eq!(cubic.sample(1.0).unwrap(), vec![2.0, 0.0, 0.0]);
    let rotation = Track {
        target,
        property: Property::Rotation,
        times: vec![0.0, 1.0],
        values: vec![
            vec![0.0, 0.0, 0.0, 1.0],
            Quaternion::from_rotation_y(std::f64::consts::PI)
                .to_array()
                .to_vec(),
        ],
        interpolation: Interpolation::Linear,
    };
    let middle = rotation.sample(0.5).unwrap();
    assert!((middle[1] - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);
}
#[test]
fn morph_then_skin_matches_ray_queries_without_mutating_shared_geometry() {
    let mut scene = Scene::new();
    let bone = scene.insert(NodeKind::Group);
    scene.get_mut(bone).unwrap().position.x = 2.0;
    let mut g = PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap();
    let count = g.vertex_count();
    g.morph_targets_relative = true;
    g.morph_attributes.insert(
        "position".into(),
        vec![Attribute::F32(
            BufferAttribute::new((0..count).flat_map(|_| [0.0, 1.0, 0.0]).collect(), 3, false)
                .unwrap(),
        )],
    );
    g.set_attribute(
        "skinIndex",
        Attribute::U16(BufferAttribute::new(vec![0; count * 4], 4, false).unwrap()),
    );
    g.set_attribute(
        "skinWeight",
        Attribute::F32(
            BufferAttribute::new(
                (0..count).flat_map(|_| [1.0, 0.0, 0.0, 0.0]).collect(),
                4,
                false,
            )
            .unwrap(),
        ),
    );
    let geometry = Arc::new(g);
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        geometry.clone(),
        Arc::new(Material::default()),
    )));
    scene.get_mut(mesh).unwrap().morph_weights = vec![0.5];
    scene.get_mut(mesh).unwrap().skin = Some(Skin {
        joints: vec![bone],
        inverse_bind_matrices: vec![Matrix4::IDENTITY],
    });
    scene.update().unwrap();
    let result = evaluate(&scene, mesh).unwrap().unwrap();
    assert!(
        (result.attributes["position"].vector3(0).unwrap()
            - geometry.attributes["position"].vector3(0).unwrap()
            - Vector3::new(2.0, 0.5, 0.0))
        .length()
            < 1e-6
    );
    let mut ray = Raycaster::default();
    ray.set(Vector3::new(2.0, 0.5, 2.0), -Vector3::Z);
    assert!(
        !ray.intersect_object(&scene, mesh, false)
            .unwrap()
            .is_empty()
    );
    ray.set(Vector3::new(0.0, 0.0, 2.0), -Vector3::Z);
    assert!(
        ray.intersect_object(&scene, mesh, false)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn duplicate_times_choose_last_key_without_division_by_zero() {
    let mut scene = Scene::new();
    let target = scene.insert(NodeKind::Group);
    let track = Track {
        target,
        property: Property::Position,
        times: vec![0.0, 1.0, 1.0, 2.0],
        values: vec![vec![0.0; 3], vec![1.0; 3], vec![2.0; 3], vec![4.0; 3]],
        interpolation: Interpolation::Linear,
    };
    assert_eq!(track.sample(1.0).unwrap(), vec![2.0; 3]);
    assert_eq!(track.sample(1.5).unwrap(), vec![3.0; 3]);
}

#[test]
fn cross_fade_starts_target_at_zero_weight() {
    let mut scene = Scene::new();
    let target = scene.insert(NodeKind::Group);
    let clip = |value| {
        Arc::new(Clip {
            name: String::new(),
            tracks: vec![Track {
                target,
                property: Property::Position,
                times: vec![0.0],
                values: vec![vec![value; 3]],
                interpolation: Interpolation::Linear,
            }],
        })
    };
    let mut mixer = AnimationMixer::default();
    let from = mixer.play(clip(2.0)).unwrap();
    let to = mixer.play(clip(6.0)).unwrap();
    mixer.cross_fade(from, to, 1.0).unwrap();
    mixer.update(&mut scene, 0.5).unwrap();
    assert_eq!(scene.get(target).unwrap().position, Vector3::splat(4.0));
    mixer.update(&mut scene, 0.5).unwrap();
    assert_eq!(scene.get(target).unwrap().position, Vector3::splat(6.0));
}
