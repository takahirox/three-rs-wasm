use std::sync::Arc;
use three_rs_wasm::{camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};

#[test]
fn webgpu_renders_basic_mesh_and_recreates_targets() {
    let renderer =
        pollster::block_on(Renderer::new()).expect("WebGPU adapter is required for GPU tests");
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -2.0,
        right: 2.0,
        top: 2.0,
        bottom: -2.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 5.0;
    let mut material = Material::default();
    material.properties_mut().color = Color::from_hex(0xff0000);
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(material),
    )));
    for _ in 0..3 {
        let mut target = RenderTarget::new(&renderer.device, 32, 32).unwrap();
        target.set_size(&renderer.device, 64, 64).unwrap();
        renderer.render(&mut scene, camera, &target).unwrap();
        let pixels = renderer.read_rgba(&target).unwrap();
        assert_eq!(
            &pixels[(32 * 64 + 32) * 4..(32 * 64 + 32) * 4 + 4],
            &[255, 0, 0, 255]
        );
        assert_eq!(&pixels[0..4], &[0, 0, 0, 255]);
        target.dispose();
    }
    for options in [
        RenderTargetOptions {
            samples: 4,
            store_multisampled_depth_buffer: false,
            ..Default::default()
        },
        RenderTargetOptions {
            count: 2,
            ..Default::default()
        },
        RenderTargetOptions {
            depth_buffer: false,
            ..Default::default()
        },
        RenderTargetOptions {
            stencil_buffer: true,
            ..Default::default()
        },
    ] {
        let target = RenderTarget::with_options(&renderer.device, 32, 32, options).unwrap();
        renderer.render(&mut scene, camera, &target).unwrap();
        let pixels = renderer.read_rgba(&target).unwrap();
        assert_eq!(
            &pixels[(16 * 32 + 16) * 4..(16 * 32 + 16) * 4 + 4],
            &[255, 0, 0, 255]
        );
        let copy = target.clone_target(&renderer.device).unwrap();
        assert_eq!(copy.width, target.width);
    }
    let mut volume =
        RenderTarget3D::new(&renderer.device, 32, 32, 3, RenderTargetOptions::default()).unwrap();
    volume.set_layer(2).unwrap();
    renderer.render(&mut scene, camera, &volume).unwrap();
    let pixels = renderer.read_rgba(&volume).unwrap();
    assert_eq!(
        &pixels[(16 * 32 + 16) * 4..(16 * 32 + 16) * 4 + 4],
        &[255, 0, 0, 255]
    );
    let target = RenderTarget::new(&renderer.device, 32, 32).unwrap();
    for count in [0, 6] {
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(mesh).unwrap().kind {
            Arc::make_mut(&mut mesh.geometry).set_indirect(Some(vec![count, 1, 0, 0, 0]));
        }
        renderer.render(&mut scene, camera, &target).unwrap();
        let pixels = renderer.read_rgba(&target).unwrap();
        assert_eq!(pixels[(16 * 32 + 16) * 4], if count == 0 { 0 } else { 255 });
    }
    scene.dispose(mesh).unwrap();
    let mut geometry = BufferGeometry::default();
    geometry.set_from_points(&[Vector3::ZERO]).unwrap();
    let point_material = PointsMaterial {
        size: 8.0,
        size_attenuation: false,
        ..Default::default()
    };
    scene.insert(NodeKind::Points(Points {
        geometry: Arc::new(geometry),
        material: Arc::new(Material::Points(point_material)),
    }));
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(pixels.chunks_exact(4).filter(|p| p[0] > 200).count(), 64);

    // Complete each submission before inspecting native resource counts. The
    // pipeline cache may retain bounded entries; per-frame resources must settle.
    renderer.device.poll(wgpu::PollType::Wait).unwrap();
    let initial = renderer.device.get_internal_counters().hal;
    for _ in 0..50 {
        let temporary = RenderTarget::new(&renderer.device, 32, 32).unwrap();
        renderer.render(&mut scene, camera, &temporary).unwrap();
        renderer.device.poll(wgpu::PollType::Wait).unwrap();
    }
    renderer.device.poll(wgpu::PollType::Wait).unwrap();
    let final_counts = renderer.device.get_internal_counters().hal;
    assert_eq!(final_counts.buffers.read(), initial.buffers.read());
    assert_eq!(final_counts.textures.read(), initial.textures.read());
    assert_eq!(final_counts.bind_groups.read(), initial.bind_groups.read());
}
