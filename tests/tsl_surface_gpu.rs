use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::SurfaceNodes, *},
};
#[test]
fn lit_nodes_preserve_defaults_and_update_without_geometry_uploads() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position.z = 3.0;
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 3.0,
        target: Vector3::ZERO,
    }));
    scene.get_mut(light).unwrap().position = Vector3::new(1.0, 2.0, 3.0);
    let material = MeshStandardMaterial::default();
    let object = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(1.0, 32, 16).unwrap()),
        Arc::new(Material::Standard(material.clone())),
    )));
    let target = RenderTarget::new(&r.device, 64, 64).unwrap();
    r.render(&mut scene, cam, &target).unwrap();
    let baseline = r.read_rgba(&target).unwrap();
    let program = pollster::block_on(SurfaceNodes::default().build(&r, &[], &[])).unwrap();
    let mut node_material = material.clone();
    node_material.properties.vertex_program = Some(Arc::new(program));
    let set = |scene: &mut Scene, m: MeshStandardMaterial| {
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(object).unwrap().kind {
            mesh.materials[0] = Arc::new(Material::Standard(m));
        }
    };
    set(&mut scene, node_material);
    r.render(&mut scene, cam, &target).unwrap();
    assert_eq!(r.read_rgba(&target).unwrap(), baseline);
    let graph = SurfaceNodes {
        color: Some(vec3(float(0.2), float(0.1), float(0.05))),
        roughness: Some(uniform(0, Type::Float)),
        metalness: Some(float(0.5)),
        emissive: Some(vec3(float(0.1), float(0.0), float(0.0))),
        ..Default::default()
    };
    let mut material = material;
    material.properties.vertex_program = Some(Arc::new(
        pollster::block_on(graph.build(&r, &[], &[])).unwrap(),
    ));
    material.properties.vertex_uniforms[0][0] = 0.1;
    set(&mut scene, material.clone());
    r.render(&mut scene, cam, &target).unwrap();
    let first = r.read_rgba(&target).unwrap();
    assert_ne!(first, baseline);
    let resources = r.resource_counts();
    let transfers = r.transfer_counts();
    material.properties.vertex_uniforms[0][0] = 0.9;
    set(&mut scene, material);
    r.render(&mut scene, cam, &target).unwrap();
    assert_ne!(first, r.read_rgba(&target).unwrap());
    assert_eq!(r.resource_counts(), resources);
    assert_eq!(r.transfer_counts(), transfers);
    assert!(
        SurfaceNodes {
            roughness: Some(tsl::uv()),
            ..Default::default()
        }
        .wgsl(0, &[])
        .is_err()
    );
}

#[test]
fn depth_nodes_sample_the_attachment_and_rebind_after_resize() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.0,
        far: 4.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 2.0;
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap()),
        Arc::new(Material::Basic(MeshBasicMaterial::default())),
    )));
    for samples in [0, 4] {
        let mut input = RenderTarget::with_options(
            &r.device,
            32,
            32,
            RenderTargetOptions {
                samples,
                ..Default::default()
            },
        )
        .unwrap();
        let mut output = RenderTarget::with_options(
            &r.device,
            32,
            32,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )
        .unwrap();
        let depth = input
            .depth_texture()
            .unwrap()
            .create_view(&Default::default());
        let graph = vec4(splat(depth_texture(uv()), Type::Vec3), float(1.0));
        let effect = if samples > 1 {
            pollster::block_on(multisampled_depth_effect(
                &r,
                wgpu::TextureFormat::Rgba8Unorm,
                &graph,
                &depth,
            ))
        } else {
            pollster::block_on(depth_effect(
                &r,
                wgpu::TextureFormat::Rgba8Unorm,
                &graph,
                &depth,
            ))
        }
        .unwrap();
        let mut effect = effect;
        for size in [32, 48] {
            input.set_size(&r.device, size, size).unwrap();
            output.set_size(&r.device, size, size).unwrap();
            effect
                .set_depth(
                    &r,
                    &input
                        .depth_texture()
                        .unwrap()
                        .create_view(&Default::default()),
                )
                .unwrap();
            r.render(&mut scene, camera, &input).unwrap();
            effect.apply(&r, &input, None, &output).unwrap();
            let image = r.read_rgba(&output).unwrap();
            let center = ((size / 2 * size + size / 2) * 4) as usize;
            assert_eq!(&image[center..center + 4], &[128, 128, 128, 255]);
            assert_eq!(&image[..4], &[255, 255, 255, 255]);
        }
    }
    assert!(effect_wgsl(&depth_texture(uv())).is_err());
}

#[test]
fn mrt_nodes_write_distinct_attachments_in_one_draw() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.0,
        far: 2.0,
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position.z = 1.0;
    let graph = SurfaceNodes {
        color: Some(vec3(float(0.5), float(0.1), float(0.0))),
        ..Default::default()
    };
    let program = pollster::block_on(graph.build_mrt(
        &r,
        &[tsl::output(), vec3(float(0.0), float(0.75), float(0.0))],
        &[],
        &[],
    ))
    .unwrap();
    let mut material = MeshBasicMaterial::default();
    material.properties.vertex_program = Some(Arc::new(program));
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Basic(material)),
    )));
    let target = RenderTarget::with_options(
        &r.device,
        8,
        8,
        RenderTargetOptions {
            count: 2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    r.render(&mut scene, cam, &target).unwrap();
    assert_eq!(&r.read_rgba(&target).unwrap()[..4], &[128, 26, 0, 255]);
    let view = target.textures()[1].create_view(&Default::default());
    let sampler = r.device.create_sampler(&Default::default());
    let copy = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::Texture::External(0).sample(uv()),
        &[(&view, &sampler)],
    ))
    .unwrap();
    let output = RenderTarget::with_options(
        &r.device,
        8,
        8,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    copy.apply(&r, &target, None, &output).unwrap();
    assert_eq!(&r.read_rgba(&output).unwrap()[..4], &[0, 191, 0, 255]);
    assert!(r.render(&mut scene, cam, &output).is_err());
}

#[test]
fn workgroup_snapshot_reverses_all_typed_buffers_without_races() {
    use three_rs_wasm::{
        compute::{BufferAccess, GpuBuffer},
        tsl::compute::{BufferCompute, BufferStore},
    };
    let r = pollster::block_on(Renderer::new()).unwrap();
    let types = [Type::Float, Type::Vec2, Type::Vec3, Type::Vec4];
    let strides = [1, 2, 4, 4];
    let buffers: Vec<_> = strides
        .iter()
        .map(|stride| {
            let values: Vec<f32> = (0..32)
                .flat_map(|i| std::iter::repeat_n(i as f32, *stride))
                .collect();
            GpuBuffer::new(&r, bytemuck::cast_slice(&values), BufferAccess::ReadWrite).unwrap()
        })
        .collect();
    let bindings: Vec<_> = buffers.iter().zip(types).collect();
    let stores: Vec<_> = (0..4)
        .map(|binding| BufferStore {
            binding,
            index: instance_index(),
            value: storage_element(binding, uint(31) - instance_index()),
        })
        .collect();
    for count in [0, 65] {
        assert!(
            pollster::block_on(BufferCompute::new_workgroup_snapshot(
                &r, count, &bindings, &stores
            ))
            .is_err()
        );
    }
    let kernel = pollster::block_on(BufferCompute::new_workgroup_snapshot(
        &r, 32, &bindings, &stores,
    ))
    .unwrap();
    for iteration in 0..20 {
        kernel.dispatch(&r).unwrap();
        for (binding, buffer) in buffers.iter().enumerate() {
            let bytes = buffer.read(&r).unwrap();
            let values: &[f32] = bytemuck::cast_slice(&bytes);
            for i in 0..32 {
                for c in 0..[1, 2, 3, 4][binding] {
                    assert_eq!(
                        values[i * strides[binding] + c],
                        if iteration % 2 == 0 {
                            (31 - i) as f32
                        } else {
                            i as f32
                        }
                    );
                }
            }
        }
    }
}

#[test]
fn bloom_threshold_and_strength_apply_to_hdr_input() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    scene.background = Color::linear(0.1, 0.1, 0.1);
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    let input = RenderTarget::with_options(
        &r.device,
        64,
        64,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            depth_buffer: false,
            ..Default::default()
        },
    )
    .unwrap();
    r.render(&mut scene, cam, &input).unwrap();
    let mut bloom = pollster::block_on(tsl::bloom::Bloom::new(&r)).unwrap();
    let target = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let copy = pollster::block_on(effect(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::Texture::Input.sample(uv()),
    ))
    .unwrap();
    let read = |bloom: &mut tsl::bloom::Bloom| {
        copy.apply(&r, bloom.render(&r, &input).unwrap(), None, &target)
            .unwrap();
        r.read_rgba(&target).unwrap()
    };
    let bright = read(&mut bloom);
    assert!(
        bright
            .as_chunks::<4>()
            .0
            .iter()
            .all(|p| (70..=80).contains(&p[0]))
    );
    bloom.strength = 0.0;
    assert!(
        read(&mut bloom)
            .as_chunks::<4>()
            .0
            .iter()
            .all(|p| p[..3] == [0, 0, 0])
    );
    bloom.strength = 1.0;
    bloom.threshold = 0.2;
    assert!(
        read(&mut bloom)
            .as_chunks::<4>()
            .0
            .iter()
            .all(|p| p[..3] == [0, 0, 0])
    );
}

#[test]
fn mixed_mrt_formats_clear_only_the_color_attachment() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let options = RenderTargetOptions {
        count: 2,
        format: wgpu::TextureFormat::Rgba16Float,
        color_formats: vec![
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Rgba8Unorm,
        ],
        ..Default::default()
    };
    let input = RenderTarget::with_options(&r.device, 16, 16, options.clone()).unwrap();
    assert!(
        RenderTarget::with_options(
            &r.device,
            16,
            16,
            RenderTargetOptions {
                count: 1,
                ..options.clone()
            }
        )
        .is_err()
    );
    assert!(
        RenderTarget::with_options(
            &r.device,
            16,
            16,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..options
            }
        )
        .is_err()
    );
    let mut scene = Scene::new();
    scene.background = Color::linear(1.0, 0.5, 0.25);
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    r.render(&mut scene, cam, &input).unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let view = input.textures()[1].create_view(&Default::default());
    let copy = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::Texture::External(0).sample(uv()),
        &[(&view, &sampler)],
    ))
    .unwrap();
    let target = RenderTarget::with_options(
        &r.device,
        16,
        16,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    copy.apply(&r, &input, None, &target).unwrap();
    assert!(r.read_rgba(&target).unwrap().iter().all(|v| *v == 0));
}

#[test]
fn material_light_selection_preserves_only_selected_lighting() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position.z = 3.0;
    let lights: Vec<_> = [0xff0000, 0x0000ff]
        .into_iter()
        .map(|hex| {
            scene.insert(NodeKind::Light(Light::Ambient {
                color: Color::from_hex(hex),
                intensity: 3.0,
            }))
        })
        .collect();
    let h = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Standard(MeshStandardMaterial::default())),
    )));
    let target = RenderTarget::new(&r.device, 32, 32).unwrap();
    for selected in [vec![lights[0]], vec![lights[1]], vec![]] {
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h).unwrap().kind {
            Arc::make_mut(&mut mesh.materials[0])
                .properties_mut()
                .lights = Some(selected.clone());
        }
        r.render(&mut scene, cam, &target).unwrap();
        let rgba = r.read_rgba(&target).unwrap();
        let p = &rgba[(16 * 32 + 16) * 4..][..3];
        if selected.is_empty() {
            assert_eq!(p, [0, 0, 0]);
        } else if selected[0] == lights[0] {
            assert!(p[0] > 200 && p[1] == 0 && p[2] == 0);
        } else {
            assert!(p[2] > 200 && p[0] == 0 && p[1] == 0);
        }
    }
}

#[test]
fn normal_map_nodes_orient_double_sided_surfaces_toward_the_viewer() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.0,
        ..Default::default()
    })));
    let graph = SurfaceNodes {
        normal: Some(surface::normal_map(
            vec3(float(0.5), float(0.5), float(1.0)),
            uv(),
        )),
        output: Some(vec4(normal_view() * float(0.5) + float(0.5), float(1.0))),
        ..Default::default()
    };
    let mut material = MeshStandardMaterial::default();
    material.properties.side = Side::Double;
    material.properties.vertex_program = Some(Arc::new(
        pollster::block_on(graph.build(&r, &[], &[])).unwrap(),
    ));
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Standard(material)),
    )));
    let target = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    for z in [3.0, -3.0] {
        scene.get_mut(cam).unwrap().position.z = z;
        scene.look_at(cam, Vector3::ZERO).unwrap();
        r.render(&mut scene, cam, &target).unwrap();
        let rgba = r.read_rgba(&target).unwrap();
        assert_eq!(&rgba[(16 * 32 + 16) * 4..][..4], &[128, 128, 255, 255]);
    }
}
