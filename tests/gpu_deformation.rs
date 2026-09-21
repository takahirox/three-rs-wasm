use std::sync::Arc;
use three_rs_wasm::{
    animation::*, camera::*, deformation, geometry::*, material::*, math::*, renderer::*, scene::*,
};
#[test]
fn gpu_skin_and_morph_match_cpu_oracle_with_shadows_without_vertex_reuploads() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let oracle = pollster::block_on(Renderer::new()).unwrap();
    let asset = gltf::Gltf::from_slice(include_bytes!(
        "../web/models/RobotExpressive/RobotExpressive.glb"
    ))
    .unwrap();
    let model =
        three_rs_wasm::gltf::import_animated(&asset, &[asset.blob.clone().unwrap()], &[]).unwrap();
    let mut scene = Scene::new();
    let instance = model.instantiate(&mut scene).unwrap();
    for &h in &instance.meshes {
        scene.get_mut(h).unwrap().cast_shadow = true;
    }
    scene.shadow_map_size = 128;
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 2.0,
        target: Vector3::new(0.0, 2.0, 0.0),
    }));
    {
        let n = scene.get_mut(light).unwrap();
        n.position = Vector3::new(4.0, 7.0, 5.0);
        n.cast_shadow = true;
        n.shadow.extent = 6.0;
        n.shadow.far = 30.0;
    }
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::WHITE,
        intensity: 0.5,
    }));
    let floor = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(12.0, 12.0, 1, 1).unwrap()),
        Arc::new(Material::Lambert(MeshLambertMaterial::default())),
    )));
    {
        let n = scene.get_mut(floor).unwrap();
        n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        n.receive_shadow = true;
        n.position.y = -0.02;
    }
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 40.0,
        aspect: 1.0,
        near: 0.1,
        far: 40.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position = Vector3::new(6.0, 5.0, 10.0);
    scene.look_at(camera, Vector3::new(0.0, 2.0, 0.0)).unwrap();
    let target = RenderTarget::new(&renderer.device, 128, 128).unwrap();
    let expected = RenderTarget::new(&oracle.device, 128, 128).unwrap();
    let per_pose_budget: usize = instance
        .meshes
        .iter()
        .map(|&h| {
            let n = scene.get(h).unwrap();
            let bones = n.skin.as_ref().map_or(0, |s| s.joints.len());
            let morphs = n
                .geometry()
                .unwrap()
                .morph_attributes
                .values()
                .map(Vec::len)
                .max()
                .unwrap_or(0);
            (bones * 16 + morphs).max(4) * 4
        })
        .sum();
    let mut baseline = None;
    let mut compared = 0;
    for clip in &instance.clips {
        let mut mixer = AnimationMixer::default();
        let action = mixer.play(clip.clone()).unwrap();
        mixer.actions[action].looping = LoopMode::Once;
        for time in [0.0, 0.25] {
            mixer.actions[action].time = time;
            mixer.update(&mut scene, 0.0).unwrap();
            scene.update().unwrap();
            renderer.render(&mut scene, camera, &target).unwrap();
            let actual = renderer.read_rgba(&target).unwrap();
            let counts = renderer.transfer_counts();
            if let Some((uploads, bytes, static_bytes)) = baseline {
                assert_eq!(
                    (counts.0, counts.1, counts.2),
                    (uploads, bytes, static_bytes),
                    "pose-only frames must not upload vertex/morph source data"
                );
            } else {
                baseline = Some((counts.0, counts.1, counts.2));
            }
            let mut saved = Vec::new();
            for &h in &instance.meshes {
                if let Some(geometry) = deformation::evaluate(&scene, h).unwrap() {
                    let node = scene.get_mut(h).unwrap();
                    let NodeKind::Mesh(mesh) = &mut node.kind else {
                        panic!()
                    };
                    saved.push((
                        h,
                        mesh.geometry.clone(),
                        node.skin.take(),
                        std::mem::take(&mut node.morph_weights),
                    ));
                    mesh.geometry = geometry;
                }
            }
            oracle.render(&mut scene, camera, &expected).unwrap();
            let reference = oracle.read_rgba(&expected).unwrap();
            for (h, geometry, skin, weights) in saved {
                let node = scene.get_mut(h).unwrap();
                let NodeKind::Mesh(mesh) = &mut node.kind else {
                    panic!()
                };
                mesh.geometry = geometry;
                node.skin = skin;
                node.morph_weights = weights;
            }
            let bad = actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(reference.as_chunks::<4>().0.iter())
                .filter(|(a, b)| (0..3).any(|c| a[c].abs_diff(b[c]) > 3))
                .count();
            assert!(
                bad < 128 * 128 / 100,
                "{} at {}: {} pixels differ",
                clip.name,
                time,
                bad
            );
            compared += 1;
        }
        mixer.restore(&mut scene).unwrap();
    }
    assert_eq!(compared, 28);
    let counts = renderer.transfer_counts();
    assert!(
        counts.3 <= (per_pose_budget * compared + 64) as u64,
        "pose transfer is bone/weight-sized, not vertex-sized: {counts:?}"
    );
    println!(
        "28 real-model frames: geometry uploads {}, resident bytes {}, morph/skin bytes {}, pose bytes {}",
        counts.0, counts.1, counts.2, counts.3
    );
}

#[test]
fn sparse_morph_streams_match_cpu_without_allocating_absent_attributes() {
    use three_rs_wasm::attribute::BufferAttribute;
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let oracle = pollster::block_on(Renderer::new()).unwrap();
    for mask in [1u32, 2, 4, 8, 5, 10, 15] {
        let mut geometry = PlaneGeometry::build(1.2, 1.2, 2, 2).unwrap();
        let count = geometry.vertex_count();
        geometry.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(vec![0.4; count * 4], 4, false).unwrap()),
        );
        geometry.compute_tangents().unwrap();
        geometry.morph_targets_relative = true;
        for (i, name) in ["position", "normal", "color", "tangent"]
            .into_iter()
            .enumerate()
        {
            if mask & (1 << i) == 0 {
                continue;
            }
            let width = if name == "color" { 4 } else { 3 };
            let values = (0..count)
                .flat_map(|_| (0..width).map(|c| if c == 0 { 0.3 } else { 0.0 }))
                .collect();
            geometry.morph_attributes.insert(
                name.into(),
                vec![Attribute::F32(
                    BufferAttribute::new(values, width, false).unwrap(),
                )],
            );
        }
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
        scene.get_mut(camera).unwrap().position.z = 3.0;
        let mut material = MeshStandardMaterial::default();
        material.properties.vertex_colors = true;
        material.normal_map = Some(Arc::new(
            Texture::from_rgba(1, 1, vec![210, 150, 230, 255], false).unwrap(),
        ));
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        )));
        scene.get_mut(mesh).unwrap().morph_weights = vec![0.7];
        let light = scene.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.0,
            target: Vector3::ZERO,
        }));
        scene.get_mut(light).unwrap().position = Vector3::new(2.0, 3.0, 4.0);
        let target = RenderTarget::new(&renderer.device, 64, 64).unwrap();
        let before = renderer.transfer_counts();
        renderer.render(&mut scene, camera, &target).unwrap();
        let after = renderer.transfer_counts();
        assert_eq!(
            after.2 - before.2,
            count as u64 * 16 * mask.count_ones() as u64,
            "only present vec4 morph streams may be uploaded"
        );
        let actual = renderer.read_rgba(&target).unwrap();
        let evaluated = deformation::evaluate(&scene, mesh).unwrap().unwrap();
        let node = scene.get_mut(mesh).unwrap();
        if let NodeKind::Mesh(m) = &mut node.kind {
            m.geometry = evaluated;
        }
        node.morph_weights.clear();
        let target = RenderTarget::new(&oracle.device, 64, 64).unwrap();
        oracle.render(&mut scene, camera, &target).unwrap();
        let expected = oracle.read_rgba(&target).unwrap();
        let different = actual
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<4>().0.iter())
            .filter(|(a, b)| (0..3).any(|c| a[c].abs_diff(b[c]) > 3))
            .count();
        assert!(
            different < 64 * 64 / 100,
            "mask {mask}: {different} pixels differ"
        );
    }
}
