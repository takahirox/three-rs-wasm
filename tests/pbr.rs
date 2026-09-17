use std::sync::Arc;
use three_rs_wasm::{
    camera::*, environment::EnvironmentMap, geometry::*, material::*, renderer::*, scene::*,
};
#[test]
fn hdr_is_linear_and_lights_a_metal() {
    let environment =
        EnvironmentMap::from_hdr(include_bytes!("../web/environments/royal_esplanade_2k.hdr"))
            .unwrap();
    std::fs::create_dir_all(".cache").unwrap();
    std::fs::write(
        ".cache/gltf-pbr-decoded-hdr.bin",
        bytemuck::cast_slice(&environment.rgba),
    )
    .unwrap();
    assert_eq!((environment.width, environment.height), (2048, 1024));
    assert!(environment.rgba.iter().any(|v| v.to_f32() > 1.0));
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    scene.environment = Some(Arc::new(environment));
    scene.background_environment = true;
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    scene.get_mut(camera).unwrap().position.z = 4.0;
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(1.0, 1.0, 1.0).unwrap()),
        Arc::new(Material::Standard(MeshStandardMaterial::default())),
    )));
    let target = RenderTarget::with_options(
        &renderer.device,
        32,
        32,
        RenderTargetOptions {
            samples: 4,
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
    .unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let output = RenderTarget::new(&renderer.device, 32, 32).unwrap();
    let view = output.texture.create_view(&Default::default());
    renderer.blit_tone_mapped(
        &target,
        &view,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        1.0,
        true,
    );
    let pixels = renderer.read_rgba(&output).unwrap();
    assert!(pixels.as_chunks::<4>().0.iter().any(|p| p[0] > 30));
}

#[test]
fn texture_cache_releases_replacements_and_does_not_upload_unchanged_images() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    scene.get_mut(camera).unwrap().position.z = 4.0;
    let target = RenderTarget::new(&renderer.device, 16, 16).unwrap();
    let geometry = Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap());
    for i in 0..20 {
        let texture =
            Arc::new(Texture::from_rgba(1, 1, vec![i * 10, 100, 200, 255], true).unwrap());
        let weak = Arc::downgrade(&texture);
        let mut material = Material::default();
        material.properties_mut().map = Some(texture);
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(material),
        )));
        renderer.render(&mut scene, camera, &target).unwrap();
        let before = renderer.resource_counts();
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(before, renderer.resource_counts());
        assert_eq!(before.0, 2);
        scene.dispose(mesh).unwrap();
        renderer.collect_resources();
        assert!(weak.upgrade().is_none());
        assert_eq!(renderer.resource_counts().0, 1);
    }
    assert_eq!(renderer.resource_counts().1, 21);
}

#[test]
fn importer_loads_original_pbr_maps_and_rejects_missing_buffers() {
    let bytes = include_bytes!("../web/models/BoomBox.glb");
    let asset = gltf::Gltf::from_slice(bytes).unwrap();
    assert!(three_rs_wasm::gltf::import(&asset, &[], &[]).is_err());
    let buffers = vec![asset.blob.clone().unwrap()];
    let images = asset
        .images()
        .map(|image| match image.source() {
            gltf::image::Source::View { view, .. } => buffers[view.buffer().index()]
                [view.offset()..view.offset() + view.length()]
                .to_vec(),
            _ => panic!("fixture should embed images"),
        })
        .collect::<Vec<_>>();
    let imported = three_rs_wasm::gltf::import(&asset, &buffers, &images).unwrap();
    assert!(imported.triangles > 1000);
    let mut scene = Scene::new();
    let handles = imported.instantiate(&mut scene).unwrap();
    for handle in handles {
        let NodeKind::Mesh(mesh) = &scene.get(handle).unwrap().kind else {
            panic!()
        };
        let Material::Standard(material) = mesh.materials[0].as_ref() else {
            panic!("must retain PBR")
        };
        assert!(material.properties.map.as_ref().unwrap().srgb);
        assert!(!material.metallic_roughness_map.as_ref().unwrap().srgb);
        assert!(!material.normal_map.as_ref().unwrap().srgb);
        assert!(material.emissive_map.as_ref().unwrap().srgb);
    }
}

#[test]
fn pbr_features_have_independent_visible_effects() {
    use three_rs_wasm::math::*;
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    scene.get_mut(camera).unwrap().position.z = 3.0;
    let rgba = (0..32)
        .flat_map(|y| {
            (0..64).flat_map(move |x| {
                [
                    if x > 30 && x < 40 { 12.0 } else { 0.1 },
                    y as f32 / 8.0,
                    x as f32 / 16.0,
                    1.0,
                ]
                .map(half::f16::from_f32)
            })
        })
        .collect();
    scene.environment = Some(Arc::new(EnvironmentMap {
        width: 64,
        height: 32,
        rgba,
    }));
    let target = RenderTarget::with_options(
        &renderer.device,
        32,
        32,
        RenderTargetOptions {
            samples: 4,
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
    .unwrap();
    let output = RenderTarget::new(&renderer.device, 32, 32).unwrap();
    let view = output.texture.create_view(&Default::default());
    let geometry = Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap());
    let sample = |scene: &mut Scene, material: MeshStandardMaterial, exposure: f64| {
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Standard(material)),
        )));
        renderer.render(scene, camera, &target).unwrap();
        renderer.blit_tone_mapped(
            &target,
            &view,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            exposure,
            true,
        );
        let pixels = renderer.read_rgba(&output).unwrap();
        scene.dispose(mesh).unwrap();
        pixels[(16 * 32 + 16) * 4..(16 * 32 + 16) * 4 + 3].to_vec()
    };
    let texture = |rgba| Some(Arc::new(Texture::from_rgba(1, 1, rgba, false).unwrap()));
    let baseline = MeshStandardMaterial {
        roughness: 0.8,
        metalness: 0.8,
        ..Default::default()
    };
    let base = sample(&mut scene, baseline.clone(), 1.0);
    let mut unused_red = baseline.clone();
    unused_red.metallic_roughness_map = texture(vec![0, 255, 255, 255]);
    assert_eq!(
        sample(&mut scene, unused_red, 1.0),
        base,
        "packed map red channel is unused"
    );
    let mut linear = baseline.clone();
    linear.emissive = Color(Vector3::ONE);
    linear.emissive_map = texture(vec![128, 128, 128, 255]);
    let mut srgb = linear.clone();
    Arc::make_mut(srgb.emissive_map.as_mut().unwrap()).srgb = true;
    assert_ne!(
        sample(&mut scene, linear, 1.0),
        sample(&mut scene, srgb, 1.0),
        "sRGB color maps differ from linear data maps"
    );
    let mut changes = Vec::new();
    let mut m = baseline.clone();
    m.normal_map = texture(vec![230, 128, 190, 255]);
    changes.push(("normal", m.clone()));
    m.normal_scale = Vector2::ZERO;
    assert_eq!(sample(&mut scene, m, 1.0), base);
    let mut m = baseline.clone();
    m.metallic_roughness_map = texture(vec![255, 255, 0, 255]);
    changes.push(("packed B metalness", m));
    let mut m = baseline.clone();
    m.metallic_roughness_map = texture(vec![255, 0, 255, 255]);
    changes.push(("packed G roughness", m));
    let mut m = baseline.clone();
    m.occlusion_map = texture(vec![0, 255, 255, 255]);
    changes.push(("AO", m.clone()));
    m.occlusion_strength = 0.0;
    assert_eq!(sample(&mut scene, m, 1.0), base);
    let mut m = baseline.clone();
    m.emissive = Color(Vector3::splat(4.0));
    m.emissive_map = texture(vec![255, 0, 0, 255]);
    changes.push(("emissive", m));
    let mut m = baseline.clone();
    m.properties.color = Color(Vector3::new(0.2, 0.4, 0.8));
    changes.push(("base factor", m));
    for (name, m) in changes {
        assert_ne!(
            sample(&mut scene, m, 1.0),
            base,
            "{name} must affect output"
        );
    }
    assert_ne!(sample(&mut scene, baseline.clone(), 0.25), base, "exposure");
    scene.environment = None;
    assert_eq!(
        sample(&mut scene, baseline, 1.0),
        vec![0, 0, 0],
        "metal requires IBL"
    );
}

#[test]
fn environment_replacements_release_owners_without_refiltering_unchanged_frames() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    let target = RenderTarget::new(&renderer.device, 8, 8).unwrap();
    scene.background_environment = true;
    for i in 0..20 {
        let image = Arc::new(EnvironmentMap {
            width: 64,
            height: 32,
            rgba: vec![half::f16::from_f32(1.0 + i as f32 / 20.0); 64 * 32 * 4],
        });
        let weak = Arc::downgrade(&image);
        scene.environment = Some(image);
        renderer.render(&mut scene, camera, &target).unwrap();
        let before = renderer.resource_counts();
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(before, renderer.resource_counts());
        assert_eq!(before.2, i + 1);
        scene.environment = None;
        renderer.collect_resources();
        assert!(weak.upgrade().is_none());
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(renderer.resource_counts().2, i + 1);
    }
}

#[test]
fn supplied_tangent_handedness_controls_normal_map_orientation() {
    use three_rs_wasm::{attribute::BufferAttribute, math::*};
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    scene.get_mut(camera).unwrap().position.z = 3.0;
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 3.0,
        target: Vector3::ZERO,
    }));
    scene.get_mut(light).unwrap().position = Vector3::new(0.0, 3.0, 3.0);
    let target = RenderTarget::new(&renderer.device, 32, 32).unwrap();
    let mut red = Vec::new();
    for sign in [1.0, -1.0] {
        let mut geometry = PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap();
        geometry.set_attribute(
            "tangent",
            Attribute::F32(
                BufferAttribute::new([1.0, 0.0, 0.0, sign].repeat(4), 4, false).unwrap(),
            ),
        );
        let material = MeshStandardMaterial {
            metalness: 0.0,
            normal_map: Some(Arc::new(
                Texture::from_rgba(1, 1, vec![128, 230, 200, 255], false).unwrap(),
            )),
            ..Default::default()
        };
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        )));
        scene.get_mut(mesh).unwrap().scale = Vector3::new(1.2, 0.8, 1.0);
        renderer.render(&mut scene, camera, &target).unwrap();
        red.push(renderer.read_rgba(&target).unwrap()[(16 * 32 + 16) * 4]);
        scene.dispose(mesh).unwrap();
    }
    assert!(
        red[0] > red[1] + 40,
        "mirrored UV handedness must reverse the mapped Y normal: {red:?}"
    );
}

#[test]
fn alpha_mask_and_double_sided_materials_preserve_coverage() {
    use three_rs_wasm::math::*;
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -2.0,
        right: 2.0,
        top: 2.0,
        bottom: -2.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 3.0;
    let mut image =
        Texture::from_rgba(2, 1, vec![255, 255, 255, 0, 255, 255, 255, 255], true).unwrap();
    image.filter = Filter::Nearest;
    let material = MeshStandardMaterial {
        emissive: Color::from_hex(0xff0000),
        properties: MaterialProperties {
            map: Some(Arc::new(image)),
            alpha_test: 0.5,
            side: Side::Double,
            ..Default::default()
        },
        ..Default::default()
    };
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Standard(material)),
    )));
    let target = RenderTarget::new(&renderer.device, 64, 64).unwrap();
    for rotation in [0.0, std::f64::consts::PI] {
        scene.get_mut(mesh).unwrap().quaternion = Quaternion::from_rotation_y(rotation);
        renderer.render(&mut scene, camera, &target).unwrap();
        let pixels = renderer.read_rgba(&target).unwrap();
        let red = pixels
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| p[0] > 200)
            .count();
        assert_eq!(red, 512);
        assert!(pixels.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
    }
}

#[test]
fn importer_rejects_required_extensions_with_actionable_names() {
    let asset = gltf::Gltf::from_slice_without_validation(
        br#"{"asset":{"version":"2.0"},"extensionsRequired":["KHR_draco_mesh_compression"]}"#,
    )
    .unwrap();
    let error = match three_rs_wasm::gltf::import(&asset, &[], &[]) {
        Ok(_) => panic!("required extension accepted"),
        Err(error) => error.to_string(),
    };
    assert!(error.contains("KHR_draco_mesh_compression"));
    assert!(error.contains("compression::prepare_gltf"));
}

#[test]
fn ldr_environment_linearizes_rgb_but_preserves_alpha() {
    let mut texture = Texture::from_rgba(64, 1, [128, 64, 255, 128].repeat(64), true).unwrap();
    let environment = EnvironmentMap::from_texture(&texture).unwrap();
    assert!(
        (environment.rgba[0].to_f64() - three_rs_wasm::math::srgb_to_linear(128.0 / 255.0)).abs()
            < 0.0001
    );
    assert!((environment.rgba[3].to_f64() - 128.0 / 255.0).abs() < 0.0003);
    texture.srgb = false;
    let linear = EnvironmentMap::from_texture(&texture).unwrap();
    assert!((linear.rgba[0].to_f64() - 128.0 / 255.0).abs() < 0.0003);
    texture.rgba.pop();
    assert!(EnvironmentMap::from_texture(&texture).is_err());
}
