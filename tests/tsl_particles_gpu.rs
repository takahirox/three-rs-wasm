use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::Material,
    math::*,
    postprocessing::afterimage::AfterImagePass,
    renderer::*,
    scene::{Mesh, NodeKind, Scene, ToneMapping},
    tsl::{self, sprites::SpriteNodeMaterial, *},
};

#[test]
fn sprite_attributes_billboard_from_two_camera_axes_and_discard_node_alpha() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let data = [[0.0f32, -0.5, 0.0, 0.0], [0.0, 0.5, 0.0, 1.0]];
    let buffer = GpuBuffer::new(&r, bytemuck::cast_slice(&data), BufferAccess::Read).unwrap();
    let attr = instanced_attribute(0);
    let mut graph = SpriteNodeMaterial::new(vec4(
        vec3(
            attr.swizzle("w"),
            float(1.0) - attr.swizzle("w"),
            float(0.0),
        ),
        uniform(0, Type::Float),
    ));
    graph.position = attr.rgb();
    graph.scale = float(0.5);
    let mut material = pollster::block_on(graph.build(&r, &[&buffer], &[])).unwrap();
    material.uniforms[0][0] = 1.0;
    material.properties.alpha_test = 0.4;
    let mut geometry = PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap();
    geometry.instance_count = Some(2);
    let mut scene = Scene::new();
    scene.background = Color::BLACK;
    let cam = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.0,
        far: 10.0,
        ..Default::default()
    })));
    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(Material::Shader(material)),
    )));
    scene.get_mut(mesh).unwrap().frustum_culled = false;
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
    let mut images = vec![];
    for position in [Vector3::new(0.0, 0.0, 3.0), Vector3::new(3.0, 0.0, 0.0)] {
        scene.get_mut(cam).unwrap().position = position;
        scene.look_at(cam, Vector3::ZERO).unwrap();
        r.render(&mut scene, cam, &target).unwrap();
        let pixels = r.read_rgba(&target).unwrap();
        assert_eq!(&pixels[(8 * 32 + 16) * 4..][..3], &[255, 0, 0]);
        assert_eq!(&pixels[(24 * 32 + 16) * 4..][..3], &[0, 255, 0]);
        images.push(pixels);
    }
    assert_eq!(images[0], images[1]);
    let uploads = r.transfer_counts();
    if let NodeKind::Mesh(mesh) = &mut scene.get_mut(mesh).unwrap().kind
        && let Material::Shader(material) = Arc::make_mut(&mut mesh.materials[0])
    {
        material.uniforms[0][0] = 0.0;
    }
    r.render(&mut scene, cam, &target).unwrap();
    assert!(
        r.read_rgba(&target)
            .unwrap()
            .chunks_exact(4)
            .all(|p| p[..3] == [0, 0, 0])
    );
    assert_eq!(uploads, r.transfer_counts());
    assert!(pollster::block_on(graph.build(&r, &[], &[])).is_err());
    graph.scale = uv().rgb();
    assert!(pollster::block_on(graph.build(&r, &[&buffer], &[])).is_err());
}

#[test]
fn afterimage_retains_decays_thresholds_and_clears_resized_history() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut input = RenderTarget::with_options(
        &r.device,
        4,
        4,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
    .unwrap();
    let dummy = RenderTarget::new(&r.device, 1, 1).unwrap();
    let mut output = RenderTarget::with_options(
        &r.device,
        4,
        4,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let mut source = pollster::block_on(tsl::effect(
        &r,
        wgpu::TextureFormat::Rgba16Float,
        &uniform(0, Type::Vec4),
    ))
    .unwrap();
    source.parameters[0] = [1.0, 0.05, 0.4, 1.0];
    source.apply(&r, &dummy, None, &input).unwrap();
    let mut history = pollster::block_on(AfterImagePass::new(&r)).unwrap();
    history.damp = 0.5;
    for expected in [[255u8, 13, 102], [128, 0, 51], [64, 0, 25], [32, 0, 0]] {
        history.render(&r, &input).unwrap();
        r.blit(
            history.output(),
            &output.texture.create_view(&Default::default()),
            wgpu::TextureFormat::Rgba8Unorm,
        );
        for pixel in r.read_rgba(&output).unwrap().chunks_exact(4) {
            for (actual, expected) in pixel[..3].iter().zip(expected) {
                assert!(actual.abs_diff(expected) <= 1, "{pixel:?} vs {expected}");
            }
        }
        source.parameters[0] = [0.0; 4];
        source.apply(&r, &dummy, None, &input).unwrap();
    }
    input.set_size(&r.device, 8, 4).unwrap();
    output.set_size(&r.device, 8, 4).unwrap();
    source.apply(&r, &dummy, None, &input).unwrap();
    history.render(&r, &input).unwrap();
    r.blit(
        history.output(),
        &output.texture.create_view(&Default::default()),
        wgpu::TextureFormat::Rgba8Unorm,
    );
    assert!(r.read_rgba(&output).unwrap().iter().all(|v| *v == 0));
}

#[test]
fn transparent_presentation_encodes_unpremultiplied_color_then_restores_alpha() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let input = RenderTarget::with_options(
        &r.device,
        1,
        1,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
    .unwrap();
    let output = RenderTarget::with_options(
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
    let mut effect = pollster::block_on(tsl::effect(
        &r,
        wgpu::TextureFormat::Rgba16Float,
        &uniform(0, Type::Vec4),
    ))
    .unwrap();
    for (value, expected) in [
        ([0.125, 0.125, 0.125, 0.5], [68u8, 68, 68, 128]),
        ([0.0; 4], [0; 4]),
    ] {
        effect.parameters[0] = value;
        effect.apply(&r, &dummy, None, &input).unwrap();
        r.blit_premultiplied_srgb(
            &input,
            &output.texture.create_view(&Default::default()),
            wgpu::TextureFormat::Rgba8Unorm,
            1.0,
            ToneMapping::None,
        );
        for (a, b) in r.read_rgba(&output).unwrap().iter().zip(expected) {
            assert!(a.abs_diff(b) <= 1);
        }
    }
}
