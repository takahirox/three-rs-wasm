use std::sync::Arc;
use three_rs_wasm::{camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};

#[test]
fn directional_spot_and_point_shadows_respect_cast_and_receive() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let target = RenderTarget::new(&renderer.device, 64, 64).unwrap();
    for light in [
        Light::Directional {
            color: Color::WHITE,
            intensity: 3.0,
            target: Vector3::ZERO,
        },
        Light::Spot {
            color: Color::WHITE,
            intensity: 3.0,
            target: Vector3::ZERO,
            distance: 0.0,
            decay: 0.0,
            angle: 1.0,
            penumbra: 0.2,
        },
        Light::Point {
            color: Color::WHITE,
            intensity: 3.0,
            distance: 0.0,
            decay: 0.0,
        },
    ] {
        let mut scene = Scene::new();
        scene.shadow_map_size = 128;
        let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -2.0,
            right: 2.0,
            top: 2.0,
            bottom: -2.0,
            near: 0.1,
            far: 20.0,
            ..Default::default()
        })));
        scene.get_mut(camera).unwrap().position.z = 5.0;
        let material = Arc::new(Material::Lambert(MeshLambertMaterial::default()));
        let receiver = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(4.0, 4.0, 1, 1).unwrap()),
            material.clone(),
        )));
        scene.get_mut(receiver).unwrap().receive_shadow = true;
        let caster = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(0.7, 0.7, 0.7).unwrap()),
            material,
        )));
        scene.get_mut(caster).unwrap().position.z = 1.0;
        let lamp = scene.insert(NodeKind::Light(light));
        let node = scene.get_mut(lamp).unwrap();
        node.position = Vector3::new(2.0, 0.0, 3.0);
        node.cast_shadow = true;
        node.shadow.far = 20.0;
        node.shadow.extent = 4.0;
        node.shadow.bias = -0.001;
        renderer.render(&mut scene, camera, &target).unwrap();
        let unshadowed = renderer.read_rgba(&target).unwrap();
        scene.get_mut(caster).unwrap().cast_shadow = true;
        renderer.render(&mut scene, camera, &target).unwrap();
        let shadowed = renderer.read_rgba(&target).unwrap();
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(shadowed, renderer.read_rgba(&target).unwrap());
        let darker = unshadowed
            .chunks_exact(4)
            .zip(shadowed.chunks_exact(4))
            .filter(|(a, b)| a[0] > b[0].saturating_add(40))
            .count();
        assert!(
            darker > 30,
            "only {darker} shadow pixels for {:?}",
            scene.get(lamp).unwrap().kind
        );
        scene.get_mut(receiver).unwrap().receive_shadow = false;
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(unshadowed, renderer.read_rgba(&target).unwrap());
    }
}

#[test]
fn legacy_lights_and_fog_follow_linear_radiometry() {
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
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Lambert(MeshLambertMaterial::default())),
    )));
    let target = RenderTarget::new(&renderer.device, 16, 16).unwrap();
    let sample = |scene: &mut Scene| {
        renderer.render(scene, camera, &target).unwrap();
        let rgba = renderer.read_rgba(&target).unwrap();
        [
            rgba[(8 * 16 + 8) * 4],
            rgba[(8 * 16 + 8) * 4 + 1],
            rgba[(8 * 16 + 8) * 4 + 2],
        ]
    };
    let light = scene.insert(NodeKind::Light(Light::Hemisphere {
        sky: Color::linear(1.0, 0.0, 0.0),
        ground: Color::linear(0.0, 0.0, 1.0),
        intensity: std::f64::consts::PI,
    }));
    scene.get_mut(light).unwrap().position.z = 1.0;
    assert_eq!(sample(&mut scene), [255, 0, 0]);
    scene.get_mut(light).unwrap().position.z = -1.0;
    assert_eq!(sample(&mut scene), [0, 0, 255]);
    scene.get_mut(light).unwrap().kind = NodeKind::Light(Light::Spot {
        color: Color::WHITE,
        intensity: std::f64::consts::PI,
        target: Vector3::ZERO,
        distance: 0.0,
        decay: 0.0,
        angle: 0.2,
        penumbra: 0.5,
    });
    scene.get_mut(light).unwrap().position.z = 2.0;
    assert!(sample(&mut scene)[0] > 250);
    if let NodeKind::Light(Light::Spot { target, .. }) = &mut scene.get_mut(light).unwrap().kind {
        *target = Vector3::X * 10.0;
    }
    assert_eq!(sample(&mut scene), [0, 0, 0]);
    scene.dispose(light).unwrap();
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::WHITE,
        intensity: std::f64::consts::PI,
    }));
    scene.fog = Some(Fog::Linear {
        color: Color::BLACK,
        near: 1.0,
        far: 3.0,
    });
    // The default target stores sRGB: linear 0.5 maps to approximately 188.
    let value = sample(&mut scene)[0];
    assert!((186..=189).contains(&value), "{value}");
    scene.fog = Some(Fog::Exp2 {
        color: Color::BLACK,
        density: 0.5,
    });
    let value = sample(&mut scene)[0];
    assert!((162..=165).contains(&value), "{value}");
    scene.fog = None;
    if let NodeKind::Mesh(m) = &mut scene.get_mut(mesh).unwrap().kind {
        m.materials[0] = Arc::new(Material::Normal(MeshNormalMaterial::default()));
    }
    assert_eq!(sample(&mut scene), [188, 188, 255]);
}
