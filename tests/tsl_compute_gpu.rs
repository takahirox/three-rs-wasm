use three_rs_wasm::{
    compute::{BufferAccess, GpuBuffer},
    renderer::*,
    tsl::{compute::*, *},
};
#[test]
fn typed_storage_snapshot_guard_and_gpu_render_binding() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let data: Vec<[f32; 2]> = (0..67).map(|i| [i as f32, 2.0]).collect();
    let a = GpuBuffer::new(&r, bytemuck::cast_slice(&data), BufferAccess::ReadWrite).unwrap();
    let b = GpuBuffer::zeroed(&r, 67 * 8, BufferAccess::ReadWrite).unwrap();
    let index = instance_index();
    let value = storage_element(0, index.clone());
    let stores = [
        BufferStore {
            binding: 0,
            index: index.clone(),
            value: value.clone() + float(1.0),
        },
        BufferStore {
            binding: 1,
            index: index.clone(),
            value: value * float(2.0),
        },
    ];
    let compute = pollster::block_on(BufferCompute::new(
        &r,
        65,
        &[(&a, Type::Vec2), (&b, Type::Vec2)],
        &stores,
    ))
    .unwrap();
    for _ in 0..3 {
        compute.dispatch(&r).unwrap();
    }
    let aa = a.read(&r).unwrap();
    let bb = b.read(&r).unwrap();
    let av: &[[f32; 2]] = bytemuck::cast_slice(&aa);
    let bv: &[[f32; 2]] = bytemuck::cast_slice(&bb);
    for i in 0..65 {
        assert_eq!(av[i], [i as f32 + 3.0, 5.0]);
        assert_eq!(bv[i], [(i as f32 + 2.0) * 2.0, 8.0]);
    }
    assert_eq!(av[65], data[65]);
    assert_eq!(bv[65], [0.0; 2]);
    let mut graph = NodeMaterial::new(vec3(
        storage_element(0, index.clone()).x(),
        float(0.0),
        float(0.0),
    ));
    graph.position = Some(vec3(storage_element(0, index).x(), float(0.0), float(0.0)));
    pollster::block_on(graph.build_with_storage(&r, &[(&a, Type::Vec2)], &[])).unwrap();
    assert!(buffer_store_wgsl(0, &[Type::Vec2], &[true], &stores).is_err());
    assert!(buffer_store_wgsl(65, &[Type::Vec2, Type::Vec3], &[true, true], &stores).is_err());
    assert!(buffer_store_wgsl(65, &[Type::Vec2, Type::Vec2], &[false, true], &stores).is_err());
    assert!(NodeMaterial::new(position_geometry()).wgsl(0).is_err());
    assert!(
        texture_store_wgsl(
            1,
            &uvec2(uint(0), uint(0)),
            &vec4(shape_circle(true), float(1.0))
        )
        .is_err()
    );
}
#[test]
fn hdr_storage_read_write_preserves_negative_values_and_ping_pong() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let fmt = wgpu::TextureFormat::Rgba16Float;
    let make = || {
        r.device
            .create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: 4,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: fmt,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default())
    };
    let a = make();
    let b = make();
    let coord = uvec2(instance_index(), uint(0));
    let initial = vec4(
        vec3(float(-0.5), float(2.0), instance_index().to_float()),
        float(1.0),
    );
    let init = pollster::block_on(TextureKernel::new(
        &r,
        4,
        coord.clone(),
        initial,
        (&a, fmt),
        &[],
    ))
    .unwrap();
    let value = texture_load(0, coord.clone())
        + vec4(vec3(float(0.25), float(-0.5), float(0.0)), float(0.0));
    let ab = pollster::block_on(TextureKernel::new(
        &r,
        4,
        coord.clone(),
        value.clone(),
        (&b, fmt),
        &[(&a, fmt)],
    ))
    .unwrap();
    let ba = pollster::block_on(TextureKernel::new(
        &r,
        4,
        coord,
        value,
        (&a, fmt),
        &[(&b, fmt)],
    ))
    .unwrap();
    init.dispatch(&r).unwrap();
    ab.dispatch(&r).unwrap();
    ba.dispatch(&r).unwrap();
    ab.dispatch(&r).unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let effect = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &Texture::External(0).sample(uv()),
        &[(&b, &sampler)],
    ))
    .unwrap();
    let output = RenderTarget::with_options(
        &r.device,
        4,
        1,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let dummy = RenderTarget::new(&r.device, 1, 1).unwrap();
    effect.apply(&r, &dummy, None, &output).unwrap();
    let bytes = r.read_rgba(&output).unwrap();
    for pixel in bytes.as_chunks::<4>().0 {
        assert!(pixel[0].abs_diff(64) <= 1, "{pixel:?}");
        assert!(pixel[1].abs_diff(128) <= 1, "{pixel:?}");
    }
}

#[test]
fn native_shader_points_draw_resident_instances_without_billboard_expansion() {
    use std::sync::Arc;
    use three_rs_wasm::{
        attribute::BufferAttribute, camera::*, geometry::*, material::Material, math::*, scene::*,
    };
    let r = pollster::block_on(Renderer::new()).unwrap();
    let positions = [
        [0.03125f32, 0.03125],
        [-0.46875, 0.03125],
        [0.53125, 0.03125],
    ];
    let buffer = GpuBuffer::new(&r, bytemuck::cast_slice(&positions), BufferAccess::Read).unwrap();
    let p = storage_element(0, instance_index());
    let mut graph = NodeMaterial::new(vec3(float(1.0), float(0.0), float(0.0)));
    graph.position = Some(vec3(p.x(), p.y(), float(0.0)));
    let m =
        pollster::block_on(graph.build_with_storage(&r, &[(&buffer, Type::Vec2)], &[])).unwrap();
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(vec![0.0f32; 3], 3, false).unwrap()),
    );
    g.instance_count = Some(3);
    let mut scene = Scene::new();
    let h = scene.insert(NodeKind::Points(Points {
        geometry: Arc::new(g),
        material: Arc::new(Material::Shader(m)),
    }));
    scene.get_mut(h).unwrap().frustum_culled = false;
    let cam = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.0,
        far: 1.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position = Vector3::Z;
    let target = RenderTarget::new(&r.device, 32, 32).unwrap();
    r.render(&mut scene, cam, &target).unwrap();
    let transfers = r.transfer_counts();
    assert_eq!(
        transfers.1, 80,
        "one resident vertex, no billboard triangles"
    );
    let pixels = r.read_rgba(&target).unwrap();
    assert_eq!(
        pixels
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| p[0] > 250 && p[1] == 0 && p[2] == 0)
            .count(),
        3
    );
    r.render(&mut scene, cam, &target).unwrap();
    assert_eq!(transfers, r.transfer_counts());
}

#[test]
fn mip_generator_filters_hdr_storage_on_gpu_and_reuses_bindings() {
    use three_rs_wasm::mipmap::MipGenerator;
    let r = pollster::block_on(Renderer::new()).unwrap();
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 1,
        },
        mip_level_count: 3,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let base = texture.create_view(&wgpu::TextureViewDescriptor {
        mip_level_count: Some(1),
        ..Default::default()
    });
    let all = texture.create_view(&Default::default());
    let x = instance_index().modulo(uint(4));
    let y = instance_index() / uint(4);
    let graph = vec4(
        vec3(x.to_float() - float(1.25), float(0.5), float(0.0)),
        float(1.0),
    );
    let kernel = pollster::block_on(TextureKernel::new(
        &r,
        16,
        uvec2(x, y),
        graph,
        (&base, wgpu::TextureFormat::Rgba16Float),
        &[],
    ))
    .unwrap();
    let mip = MipGenerator::new(&r.device, &texture).unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let effect = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &Texture::External(0).sample_level(uv(), float(2.0)),
        &[(&all, &sampler)],
    ))
    .unwrap();
    let out = RenderTarget::with_options(
        &r.device,
        1,
        1,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let dummy = RenderTarget::new(&r.device, 1, 1).unwrap();
    for _ in 0..3 {
        kernel.dispatch(&r).unwrap();
        mip.update(&r.device, &r.queue);
        effect.apply(&r, &dummy, None, &out).unwrap();
        let px = r.read_rgba(&out).unwrap();
        assert!(px[0].abs_diff(64) <= 1);
        assert!(px[1].abs_diff(128) <= 1);
    }
}
