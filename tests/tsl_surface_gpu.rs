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

#[test]
fn vertex_normal_matches_geometry_normal_and_keeps_geometry_resident() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let camera = s.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    s.get_mut(camera).unwrap().position.z = 3.;
    let light = s.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 2.,
        target: Vector3::ZERO,
    }));
    s.get_mut(light).unwrap().position = Vector3::new(1., 1., 2.);
    let graph = SurfaceNodes {
        vertex_normal: Some(uniform(0, Type::Vec3)),
        ..Default::default()
    };
    let mut material = MeshStandardMaterial::default();
    material.properties.vertex_program = Some(Arc::new(
        pollster::block_on(graph.build(&r, &[], &[])).unwrap(),
    ));
    material.properties.vertex_uniforms[0] = [0., 0., 1., 0.];
    let object = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
        Arc::new(Material::Standard(material.clone())),
    )));
    let target = RenderTarget::new(&r.device, 64, 64).unwrap();
    r.render(&mut s, camera, &target).unwrap();
    let baseline = r.read_rgba(&target).unwrap();
    let counts = r.resource_counts();
    let transfers = r.transfer_counts();
    let normal = Vector3::new(1., 0., 1.).normalize();
    material.properties.vertex_uniforms[0] = [normal.x as f32, 0., normal.z as f32, 0.];
    if let NodeKind::Mesh(m) = &mut s.get_mut(object).unwrap().kind {
        m.materials[0] = Arc::new(Material::Standard(material));
    }
    r.render(&mut s, camera, &target).unwrap();
    let actual = r.read_rgba(&target).unwrap();
    assert_ne!(actual, baseline);
    assert_eq!(counts, r.resource_counts());
    assert_eq!(transfers, r.transfer_counts());
    let mut geometry = PlaneGeometry::build(2., 2., 1, 1).unwrap();
    geometry.set_attribute(
        "normal",
        Attribute::F32(
            three_rs_wasm::attribute::BufferAttribute::new(
                [normal.x as f32, 0., normal.z as f32].repeat(4),
                3,
                false,
            )
            .unwrap(),
        ),
    );
    if let NodeKind::Mesh(m) = &mut s.get_mut(object).unwrap().kind {
        m.geometry = Arc::new(geometry);
        m.materials[0] = Arc::new(Material::Standard(MeshStandardMaterial::default()));
    }
    r.render(&mut s, camera, &target).unwrap();
    assert_eq!(actual, r.read_rgba(&target).unwrap());
}

#[test]
fn negative_material_radiance_is_clamped_before_fog() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        s.fog = Some(Fog::Linear {
            color: Color(Vector3::new(0.25, 0.5, 0.75)),
            near: 0.,
            far: 4.,
        });
        let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 4.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position.z = 2.;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::Basic(MeshBasicMaterial {
                properties: MaterialProperties {
                    color: Color(Vector3::splat(-1.)),
                    ..Default::default()
                },
            })),
        )));
        let out = RenderTarget::new(&r.device, 16, 16).unwrap();
        for lit in [false, true] {
            if lit && let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
                m.materials[0] = Arc::new(Material::Standard(MeshStandardMaterial {
                    emissive: Color(Vector3::splat(-1.)),
                    ..Default::default()
                }));
            }
            r.render(&mut s, c, &out).unwrap();
            let pixels = r.read_rgba(&out).unwrap();
            // Fog at depth 2 has factor 0.5: linear (0.125,0.25,0.375), encoded as sRGB.
            for p in pixels.as_chunks::<4>().0 {
                for (a, b) in p.iter().zip([99u8, 137, 165, 255]) {
                    assert!(a.abs_diff(b) <= 1, "{p:?}");
                }
            }
        }
    });
}

#[test]
fn texture_storage_flip_does_not_flip_derivative_normal_frames() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let c = s.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        s.get_mut(c).unwrap().position.z = 3.;
        let l = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(l).unwrap().position = Vector3::new(2., 3., 4.);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::default()),
        )));
        let out = RenderTarget::new(&r.device, 64, 64).unwrap();
        let mut texture = three_rs_wasm::material::Texture::from_rgba(
            2,
            2,
            vec![
                90, 210, 235, 255, 160, 220, 245, 255, 210, 90, 230, 255, 180, 70, 250, 255,
            ],
            false,
        )
        .unwrap();
        texture.rotation = 0.3;
        texture.repeat = Vector2::new(1.2, 0.8);
        let mut flipped = texture.clone();
        flipped.flip_y = false;
        flipped.rgba = texture.rgba[8..]
            .iter()
            .chain(&texture.rgba[..8])
            .copied()
            .collect();
        for clearcoat in [false, true] {
            let mut images = Vec::new();
            for map in [&texture, &flipped] {
                let material = if clearcoat {
                    Material::Physical(MeshPhysicalMaterial {
                        clearcoat: 1.,
                        clearcoat_roughness: 0.25,
                        clearcoat_normal_map: Some(Arc::new(map.clone())),
                        ..Default::default()
                    })
                } else {
                    Material::Standard(MeshStandardMaterial {
                        normal_map: Some(Arc::new(map.clone())),
                        roughness: 0.3,
                        energy_conservation: true,
                        ..Default::default()
                    })
                };
                if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
                    m.materials[0] = Arc::new(material);
                }
                r.render(&mut s, c, &out).unwrap();
                images.push(r.read_rgba(&out).unwrap());
            }
            let max = images[0]
                .iter()
                .zip(&images[1])
                .map(|(a, b)| a.abs_diff(*b))
                .max()
                .unwrap();
            assert!(max <= 1, "clearcoat {clearcoat}: max error {max}");
        }
    });
}

#[test]
fn fragment_light_color_matches_lambert_albedo_modulation() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let c = s.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        s.get_mut(c).unwrap().position.z = 3.;
        let l = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(l).unwrap().position = Vector3::new(1., 2., 3.);
        let tint = vec3(
            (position_world().x() + float(1.)) * float(0.5),
            float(0.2),
            float(0.8),
        );
        let light = Arc::new(
            SurfaceNodes {
                light_color: Some(
                    light_index()
                        .less_than(uint(1))
                        .select(light_color() * tint.clone(), light_color()),
                ),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap(),
        );
        let albedo = Arc::new(
            SurfaceNodes {
                color: Some(tint),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap(),
        );
        assert!(
            SurfaceNodes {
                color: Some(light_color()),
                ..Default::default()
            }
            .wgsl(0, &[])
            .is_err()
        );
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        let out = RenderTarget::new(&r.device, 64, 64).unwrap();
        let mut images = Vec::new();
        for program in [light, albedo] {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
                Arc::make_mut(&mut m.materials[0])
                    .properties_mut()
                    .vertex_program = Some(program);
            }
            r.render(&mut s, c, &out).unwrap();
            images.push(r.read_rgba(&out).unwrap());
        }
        let max = images[0]
            .iter()
            .zip(&images[1])
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap();
        assert!(max <= 1, "max error {max}");
    });
}

#[test]
fn depth_and_normal_attachments_can_be_sampled_together_and_rebound() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.,
        far: 4.,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 2.;
    let program = pollster::block_on(SurfaceNodes::default().build_mrt(
        &r,
        &[output(), vec3(float(0.25), float(0.5), float(0.75))],
        &[],
        &[],
    ))
    .unwrap();
    let mut material = MeshBasicMaterial::default();
    material.properties.vertex_program = Some(Arc::new(program));
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
        Arc::new(Material::Basic(material)),
    )));
    let make = || {
        RenderTarget::with_options(
            &r.device,
            8,
            8,
            RenderTargetOptions {
                count: 2,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )
        .unwrap()
    };
    let a = make();
    let b = make();
    let out = RenderTarget::with_options(
        &r.device,
        8,
        8,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let graph = vec4(
        vec3(
            depth_texture(uv()),
            tsl::Texture::External(0).sample(uv()).y(),
            tsl::Texture::External(0).sample(uv()).swizzle("z"),
        ),
        float(1.),
    );
    let mut effect = pollster::block_on(depth_effect_with_textures(
        &r,
        out.options().format,
        &graph,
        &a.depth_texture().unwrap().create_view(&Default::default()),
        &[(&a.textures()[1].create_view(&Default::default()), &sampler)],
    ))
    .unwrap();
    for (target, z) in [(&a, 2.), (&b, 1.), (&a, 3.)] {
        scene.get_mut(camera).unwrap().position.z = z;
        r.render(&mut scene, camera, target).unwrap();
        effect
            .set_depth_and_textures(
                &r,
                &target
                    .depth_texture()
                    .unwrap()
                    .create_view(&Default::default()),
                &[(
                    &target.textures()[1].create_view(&Default::default()),
                    &sampler,
                )],
            )
            .unwrap();
        effect.apply(&r, target, None, &out).unwrap();
        let image = r.read_rgba(&out).unwrap();
        for pixel in image.as_chunks::<4>().0 {
            for (actual, expected) in
                pixel
                    .iter()
                    .zip([(z / 4. * 255.).round() as u8, 128, 191, 255])
            {
                assert!(actual.abs_diff(expected) <= 1, "{pixel:?}");
            }
        }
    }
    assert!(
        effect
            .set_depth(
                &r,
                &a.depth_texture().unwrap().create_view(&Default::default())
            )
            .is_err()
    );
    assert!(
        effect
            .set_depth_and_textures(
                &r,
                &a.depth_texture().unwrap().create_view(&Default::default()),
                &[]
            )
            .is_err()
    );
}

#[test]
fn orthographic_specular_and_tsl_view_direction_do_not_depend_on_camera_distance() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        far: 20.,
        ..Default::default()
    })));
    let light = s.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 2.,
        target: Vector3::ZERO,
    }));
    s.get_mut(light).unwrap().position = Vector3::new(1., 1., 3.);
    let mut m = MeshPhongMaterial {
        shininess: 10.,
        specular: Color::WHITE,
        ..Default::default()
    };
    m.properties.color = Color::from_hex(0x357b98);
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(3., 3., 1, 1).unwrap()),
        Arc::new(Material::Phong(m)),
    )));
    let out = RenderTarget::new(&r.device, 32, 32).unwrap();
    let mut reference = None;
    for z in [2., 4., 10.] {
        s.get_mut(c).unwrap().position.z = z;
        r.render(&mut s, c, &out).unwrap();
        let image = r.read_rgba(&out).unwrap();
        if let Some(expected) = &reference {
            assert_eq!(&image, expected);
        } else {
            reference = Some(image);
        }
    }
    let mut m = MeshBasicMaterial::default();
    m.properties.vertex_program = Some(Arc::new(
        pollster::block_on(
            SurfaceNodes {
                color: Some(position_view_direction() * float(0.5) + float(0.5)),
                ..Default::default()
            }
            .build(&r, &[], &[]),
        )
        .unwrap(),
    ));
    if let NodeKind::Mesh(mesh) = &mut s.get_mut(h).unwrap().kind {
        mesh.materials[0] = Arc::new(Material::Basic(m));
    }
    r.render(&mut s, c, &out).unwrap();
    let image = r.read_rgba(&out).unwrap();
    for pixel in image.as_chunks::<4>().0 {
        assert_eq!(*pixel, [188, 188, 255, 255]);
    }
}

#[test]
fn retroreflection_matches_a_mirrored_view_and_preserves_resident_geometry() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 45.,
        aspect: 1.,
        ..Default::default()
    })));
    let light = s.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 0.05,
        target: Vector3::ZERO,
    }));
    s.get_mut(light).unwrap().position = Vector3::new(1., 0., 2.);
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(8., 8., 1, 1).unwrap()),
        Arc::new(Material::Physical(MeshPhysicalMaterial::default())),
    )));
    let target = RenderTarget::new(&r.device, 1, 1).unwrap();
    for anisotropy in [0., 0.6] {
        let mut samples = Vec::new();
        for (retro, x) in [(1., 1.), (0., -1.), (0., 1.)] {
            s.get_mut(c).unwrap().position = Vector3::new(x, 0., 2.);
            s.look_at(c, Vector3::ZERO).unwrap();
            let mut m = MeshPhysicalMaterial {
                retroreflectivity: retro,
                anisotropy,
                ..Default::default()
            };
            m.base.energy_conservation = true;
            m.base.roughness = 0.22;
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(h).unwrap().kind {
                mesh.materials[0] = Arc::new(Material::Physical(m));
            }
            r.render(&mut s, c, &target).unwrap();
            samples.push(r.read_rgba(&target).unwrap());
        }
        for i in 0..3 {
            assert!(samples[0][i].abs_diff(samples[1][i]) <= 1, "{samples:?}");
        }
        assert!(
            samples[0][0] > samples[2][0] + 5,
            "retroreflection must return light to its source: {samples:?}"
        );
    }
    let before = r.transfer_counts();
    r.render(&mut s, c, &target).unwrap();
    assert_eq!(before, r.transfer_counts());
    if let NodeKind::Mesh(mesh) = &mut s.get_mut(h).unwrap().kind
        && let Material::Physical(m) = Arc::make_mut(&mut mesh.materials[0])
    {
        m.retroreflectivity = f64::NAN;
    }
    assert!(r.render(&mut s, c, &target).is_err());
}

#[test]
fn local_shading_normals_include_the_model_normal_transform() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.,
        ..Default::default()
    })));
    s.get_mut(c).unwrap().position = Vector3::new(1., 1., 4.);
    s.look_at(c, Vector3::ZERO).unwrap();
    let l = s.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity: 2.,
        target: Vector3::ZERO,
    }));
    s.get_mut(l).unwrap().position = Vector3::new(1., 2., 3.);
    let mut material = MeshStandardMaterial::default();
    material.properties.color = Color::from_hex(0x647982);
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
        Arc::new(Material::Standard(material.clone())),
    )));
    s.get_mut(h).unwrap().quaternion =
        Quaternion::from_rotation_x(-0.7) * Quaternion::from_rotation_y(0.3);
    s.get_mut(h).unwrap().scale = Vector3::new(1.5, 0.7, 2.);
    let out = RenderTarget::new(&r.device, 32, 32).unwrap();
    r.render(&mut s, c, &out).unwrap();
    let reference = r.read_rgba(&out).unwrap();
    material.properties.vertex_program = Some(Arc::new(
        pollster::block_on(
            SurfaceNodes {
                normal: Some(tsl::surface::transform_normal_to_view(vec3(
                    float(0.),
                    float(0.),
                    float(1.),
                ))),
                ..Default::default()
            }
            .build(&r, &[], &[]),
        )
        .unwrap(),
    ));
    if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
        m.materials[0] = Arc::new(Material::Standard(material));
    }
    r.render(&mut s, c, &out).unwrap();
    let actual = r.read_rgba(&out).unwrap();
    for (a, b) in actual.iter().zip(&reference) {
        assert!(a.abs_diff(*b) <= 1);
    }
}
