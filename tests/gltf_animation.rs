use three_rs_wasm::{animation::*, deformation::evaluate, scene::*};

#[test]
fn original_robot_clips_deform_real_gltf_and_export_comparison_samples() {
    let asset = gltf::Gltf::from_slice(include_bytes!(
        "../web/models/RobotExpressive/RobotExpressive.glb"
    ))
    .unwrap();
    let buffers = vec![asset.blob.clone().unwrap()];
    let imported = three_rs_wasm::gltf::import_animated(&asset, &buffers, &[]).unwrap();
    let mut scene = Scene::new();
    let instance = imported.instantiate(&mut scene).unwrap();
    assert_eq!(instance.clips.len(), 14);
    let mut records = Vec::new();
    for clip in &instance.clips {
        let mut mixer = AnimationMixer::default();
        let action = mixer.play(clip.clone()).unwrap();
        mixer.actions[action].looping = LoopMode::Once;
        for time in [0.0, 0.25] {
            mixer.actions[action].time = time;
            mixer.update(&mut scene, 0.0).unwrap();
            scene.update().unwrap();
            let mut meshes = Vec::new();
            for &handle in &instance.meshes {
                let node = scene.get(handle).unwrap();
                let deformed = evaluate(&scene, handle).unwrap();
                let geometry = deformed.as_ref().or_else(|| node.geometry()).unwrap();
                let positions = geometry.positions().unwrap();
                let points = positions
                    .iter()
                    .step_by((positions.len() / 64).max(1))
                    .map(|p| node.matrix_world.transform_point3(*p).to_array())
                    .collect::<Vec<_>>();
                assert!(points.iter().flatten().all(|v| v.is_finite()));
                meshes.push(serde_json::json!({"name":node.name,"vertices":positions.len(),"points":points}));
            }
            records.push(serde_json::json!({"clip":clip.name,"time":time,"meshes":meshes}));
        }
        mixer.restore(&mut scene).unwrap();
    }
    std::fs::create_dir_all(".cache").unwrap();
    std::fs::write(
        ".cache/core-animation-samples.json",
        serde_json::to_vec(&records).unwrap(),
    )
    .unwrap();
}

#[test]
fn required_gpu_instancing_preserves_all_original_instances() {
    use three_rs_wasm::material::Texture;
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("web/models/DamagedHelmet/glTF-instancing");
    let bytes = std::fs::read(base.join("DamagedHelmetGpuInstancing.gltf")).unwrap();
    // gltf-json has no schema entry for EXT_mesh_gpu_instancing; the renderer
    // validates and handles its payload itself, rather than silently dropping it.
    let asset = gltf::Gltf::from_slice_without_validation(&bytes).unwrap();
    let buffers = asset
        .buffers()
        .map(|buffer| match buffer.source() {
            gltf::buffer::Source::Uri(uri) => std::fs::read(base.join(uri)).unwrap(),
            _ => panic!("external fixture buffer"),
        })
        .collect::<Vec<_>>();
    let images = asset
        .images()
        .map(|_| Texture::from_rgba(1, 1, vec![255; 4], true).unwrap())
        .collect::<Vec<_>>();
    assert!(
        three_rs_wasm::gltf::import_decoded(&asset, &buffers, &images).is_err(),
        "static import must not drop required instances"
    );
    let mut scene = Scene::new();
    let instance = three_rs_wasm::gltf::import_animated_decoded(&asset, &buffers, &images)
        .unwrap()
        .instantiate(&mut scene)
        .unwrap();
    assert_eq!(instance.meshes.len(), 1);
    let node = scene.get(instance.meshes[0]).unwrap();
    assert_eq!(node.instances.len(), 64);
    let translation = std::fs::read(base.join("GpuInstancingTranslation.bin")).unwrap();
    for (i, instance) in node.instances.iter().enumerate() {
        for axis in 0..3 {
            let offset = i * 12 + axis * 4;
            let expected =
                f32::from_le_bytes(translation[offset..offset + 4].try_into().unwrap()) as f64;
            assert_eq!(instance.matrix.w_axis[axis], expected);
        }
        assert!(instance.matrix.is_finite());
        assert!(instance.matrix.determinant() > 0.0);
    }
}

#[test]
fn position_only_horse_morphs_use_gpu_flat_normals_and_keep_indexed_geometry() {
    let asset = gltf::Gltf::from_slice(include_bytes!("../web/models/Horse/Horse.glb")).unwrap();
    let primitive = asset.meshes().next().unwrap().primitives().next().unwrap();
    assert!(primitive.get(&gltf::Semantic::Normals).is_none());
    let vertex_count = primitive.get(&gltf::Semantic::Positions).unwrap().count();
    let imported =
        three_rs_wasm::gltf::import_animated(&asset, &[asset.blob.clone().unwrap()], &[]).unwrap();
    let mut scene = Scene::new();
    let instance = imported.instantiate(&mut scene).unwrap();
    let NodeKind::Mesh(mesh) = &scene.get(instance.meshes[0]).unwrap().kind else {
        panic!("horse mesh")
    };
    assert!(mesh.materials[0].properties().flat_shading);
    assert_eq!(mesh.geometry.vertex_count(), vertex_count);
    assert!(!mesh.geometry.has_attribute("normal"));
    assert!(mesh.geometry.morph_attributes.contains_key("position"));
}
