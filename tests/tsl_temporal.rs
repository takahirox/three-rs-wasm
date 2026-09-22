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
fn temporal_resolve_preserves_constant_hdr_and_resizes_without_reuploading_geometry() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    for upscaling in [false, true] {
        let mut aa = pollster::block_on(async {
            if upscaling {
                tsl::temporal::TemporalAA::upscaling(&r).await
            } else {
                tsl::temporal::TemporalAA::new(&r).await
            }
        })
        .unwrap();
        let mut s = Scene::new();
        s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
        let camera = PerspectiveCamera {
            near: 0.1,
            far: 10.,
            ..Default::default()
        };
        let c = s.insert(NodeKind::Camera(Camera::Perspective(camera.clone())));
        s.get_mut(c).unwrap().position.z = 2.;
        let world = Matrix4::from_translation(Vector3::new(0., 0., 2.));
        let program = pollster::block_on(SurfaceNodes::default().build_mrt(
            &r,
            &[
                vec4(vec3(float(2.), float(1.), float(0.5)), float(1.)),
                vec4(vec3(float(0.), float(0.), float(0.)), float(0.)),
            ],
            &[],
            &[],
        ))
        .unwrap();
        let mut m = MeshBasicMaterial::default();
        m.properties.vertex_program = Some(Arc::new(program));
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1).unwrap()),
            Arc::new(Material::Basic(m)),
        )));
        let mut input = RenderTarget::with_options(
            &r.device,
            32,
            32,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                count: 2,
                ..Default::default()
            },
        )
        .unwrap();
        for size in [32, 48] {
            input.set_size(&r.device, size, size).unwrap();
            if upscaling {
                aa.set_output_size(size * 2, size * 2).unwrap();
            }
            for _ in 0..3 {
                r.render(&mut s, c, &input).unwrap();
                aa.apply(
                    &r,
                    &input,
                    world,
                    camera.projection_matrix().unwrap(),
                    [0.1, 10.],
                    false,
                )
                .unwrap();
            }
            let out = RenderTarget::with_options(
                &r.device,
                aa.output().width,
                aa.output().height,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    ..Default::default()
                },
            )
            .unwrap();
            let effect = pollster::block_on(tsl::effect(
                &r,
                wgpu::TextureFormat::Rgba8Unorm,
                &(tsl::Texture::Input.sample(uv()) / float(2.)),
            ))
            .unwrap();
            effect.apply(&r, aa.output(), None, &out).unwrap();
            let actual = r.read_rgba(&out).unwrap();
            let width = out.width;
            let p = ((width / 2 * width + width / 2) * 4) as usize;
            assert_eq!(&actual[p..p + 4], &[255, 128, 64, 128]);
            let before = (r.resource_counts(), r.transfer_counts());
            r.render(&mut s, c, &input).unwrap();
            aa.apply(
                &r,
                &input,
                world,
                camera.projection_matrix().unwrap(),
                [0.1, 10.],
                false,
            )
            .unwrap();
            assert_eq!((r.resource_counts(), r.transfer_counts()), before);
        }
        assert!(
            aa.apply(&r, &input, world, Matrix4::ZERO, [0.1, 10.], false)
                .is_err()
        );
        let invalid = RenderTarget::new(&r.device, 32, 32).unwrap();
        assert!(
            aa.apply(
                &r,
                &invalid,
                world,
                camera.projection_matrix().unwrap(),
                [0.1, 10.],
                false
            )
            .is_err()
        );
    }
}

#[test]
fn motion_vectors_interpolate_vertex_clip_positions_and_follow_previous_camera() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let matrix = |m: Matrix4| {
        m.to_cols_array_2d().map(|c| {
            vec4(
                vec3(float(c[0] as f32), float(c[1] as f32), float(c[2] as f32)),
                float(c[3] as f32),
            )
        })
    };
    let cam = OrthographicCamera {
        left: -1.,
        right: 1.,
        top: 1.,
        bottom: -1.,
        near: 0.,
        far: 4.,
        ..Default::default()
    };
    let view = Matrix4::from_translation(Vector3::new(0., 0., -2.));
    let projection = cam.projection_matrix().unwrap();
    let current = projection * view;
    let previous = projection * Matrix4::from_translation(Vector3::new(-0.2, -0.3, -2.));
    let vectors = tsl::motion::MotionVectors {
        current_clip: tsl::motion::clip_position(position_geometry(), matrix(current)),
        previous_clip: tsl::motion::clip_position(position_geometry(), matrix(previous)),
    };
    let program =
        pollster::block_on(vectors.build(&r, &SurfaceNodes::default(), &[], &[])).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(cam)));
    s.get_mut(c).unwrap().position.z = 2.;
    let mut m = MeshBasicMaterial::default();
    m.properties.vertex_program = Some(Arc::new(program));
    s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(1., 1., 1, 1).unwrap()),
        Arc::new(Material::Basic(m)),
    )));
    let target = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            count: 2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    r.render(&mut s, c, &target).unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let effect = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::Texture::External(0).sample(uv()),
        &[(
            &target.textures()[1].create_view(&Default::default()),
            &sampler,
        )],
    ))
    .unwrap();
    let out = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    effect.apply(&r, &target, None, &out).unwrap();
    let pixels = r.read_rgba(&out).unwrap();
    let p = (16 * 32 + 16) * 4;
    assert_eq!(pixels[p], 51);
    assert!((pixels[p + 1] as i32 - 77).abs() <= 1);
    assert_eq!(pixels[p + 2], 0);
}

#[test]
fn motion_vectors_include_previous_gpu_skinning_and_morph_pose() {
    use three_rs_wasm::{attribute::BufferAttribute, deformation::Skin};
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let camera = OrthographicCamera {
        left: -1.,
        right: 1.,
        top: 1.,
        bottom: -1.,
        near: 0.,
        far: 4.,
        ..Default::default()
    };
    let clip =
        camera.projection_matrix().unwrap() * Matrix4::from_translation(Vector3::new(0., 0., -2.));
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(camera)));
    s.get_mut(c).unwrap().position.z = 2.;
    let joint = s.insert(NodeKind::Group);
    let mut geometry = PlaneGeometry::build(1., 1., 1, 1).unwrap();
    let count = geometry.vertex_count();
    geometry.set_attribute(
        "skinIndex",
        Attribute::F32(BufferAttribute::new(vec![0.; count * 4], 4, false).unwrap()),
    );
    geometry.set_attribute(
        "skinWeight",
        Attribute::F32(BufferAttribute::new([1., 0., 0., 0.].repeat(count), 4, false).unwrap()),
    );
    geometry.morph_targets_relative = true;
    geometry.morph_attributes.insert(
        "position".into(),
        vec![Attribute::F32(
            BufferAttribute::new([0., 0.2, 0.].repeat(count), 3, false).unwrap(),
        )],
    );
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(Material::Basic(MeshBasicMaterial::default())),
    )));
    s.get_mut(h).unwrap().skin = Some(Skin {
        joints: vec![joint],
        inverse_bind_matrices: vec![Matrix4::IDENTITY],
    });
    s.get_mut(h).unwrap().morph_weights = vec![0.];
    s.update().unwrap();
    let mut pose = tsl::motion::PreviousPose::new(&r, &s, h).unwrap();
    assert_eq!(pose.buffer.buffer.size(), 68);
    let matrix = || {
        clip.to_cols_array_2d().map(|c| {
            vec4(
                vec3(float(c[0] as f32), float(c[1] as f32), float(c[2] as f32)),
                float(c[3] as f32),
            )
        })
    };
    let program = pollster::block_on(
        tsl::motion::MotionVectors {
            current_clip: tsl::motion::clip_position(position_geometry(), matrix()),
            previous_clip: tsl::motion::clip_position(tsl::motion::previous_position(0), matrix()),
        }
        .build(
            &r,
            &SurfaceNodes::default(),
            &[(&pose.buffer, Type::Float)],
            &[],
        ),
    )
    .unwrap();
    if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
        Arc::make_mut(&mut m.materials[0])
            .properties_mut()
            .vertex_program = Some(Arc::new(program));
    }
    let target = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            count: 2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let out = RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let sampler = r.device.create_sampler(&Default::default());
    let effect = pollster::block_on(effect_with_textures(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::Texture::External(0).sample(uv()),
        &[(
            &target.textures()[1].create_view(&Default::default()),
            &sampler,
        )],
    ))
    .unwrap();
    r.render(&mut s, c, &target).unwrap();
    let resident = r.transfer_counts();
    s.get_mut(joint).unwrap().position.x = 0.2;
    s.get_mut(h).unwrap().morph_weights[0] = 0.5;
    s.update().unwrap();
    pose.update(&r, &s).unwrap();
    for expected in [[51, 26], [0, 0]] {
        r.render(&mut s, c, &target).unwrap();
        effect.apply(&r, &target, None, &out).unwrap();
        let data = r.read_rgba(&out).unwrap();
        let p = (16 * 32 + 16) * 4;
        for i in 0..2 {
            assert!(
                (data[p + i] as i32 - expected[i]).abs() <= 1,
                "{expected:?}: {:?}",
                &data[p..p + 4]
            );
        }
        let after = r.transfer_counts();
        assert_eq!(
            (after.0, after.1, after.2),
            (resident.0, resident.1, resident.2)
        );
        pose.update(&r, &s).unwrap();
    }
}
