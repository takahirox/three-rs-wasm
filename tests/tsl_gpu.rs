use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::{Material, ShaderMaterial},
    renderer::*,
    scene::{Mesh, NodeKind, Scene},
    tsl::*,
};

#[test]
fn compute_texture_writes_partial_workgroups_and_reuses_external_bindings() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let index = instance_index();
    let x = index.modulo(uint(9));
    let y = index / uint(9);
    let graph = vec4(
        vec3(
            x.to_float() / float(8.0),
            y.to_float() / float(7.0),
            uniform(0, Type::Float),
        ),
        float(1.0),
    );
    let compute = pollster::block_on(compute::TextureCompute::new(
        &renderer,
        9,
        8,
        uvec2(x, y),
        graph,
    ))
    .unwrap();
    let target = || {
        RenderTarget::with_options(
            &renderer.device,
            9,
            8,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                depth_buffer: false,
                ..Default::default()
            },
        )
        .unwrap()
    };
    let dummy = target();
    let output = target();
    let pass = pollster::block_on(effect_with_textures(
        &renderer,
        wgpu::TextureFormat::Rgba8Unorm,
        &Texture::External(0).sample(uv()),
        &[(&compute.view, &compute.sampler)],
    ))
    .unwrap();
    for blue in [0.25, 0.75] {
        let mut uniforms = [[0.0; 4]; 16];
        uniforms[0][0] = blue;
        compute.set_uniforms(&renderer, &uniforms).unwrap();
        compute.dispatch(&renderer).unwrap();
        pass.apply(&renderer, &dummy, None, &output).unwrap();
        let pixels = renderer.read_rgba(&output).unwrap();
        for y in 0..8 {
            for x in 0..9 {
                let expected = [
                    (x as f32 / 8.0 * 255.0).round() as u8,
                    (y as f32 / 7.0 * 255.0).round() as u8,
                    (blue * 255.0).round() as u8,
                    255,
                ];
                for (a, b) in pixels[(y * 9 + x) * 4..][..4].iter().zip(expected) {
                    assert!(a.abs_diff(b) <= 1);
                }
            }
        }
    }
}

#[test]
fn nodes_drive_gpu_vertex_fragment_native_functions_and_uniform_updates() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let function = WgslFn::new(
        "half_value",
        "fn half_value(x:f32)->f32{return x*0.5;}",
        &[Type::Float],
        Type::Float,
    )
    .unwrap();
    let color = vec3(
        function.call(&[uniform(0, Type::Float)]),
        float(-0.25).modulo(float(1.0)),
        float(0.0),
    );
    let mut nodes = NodeMaterial::new(color);
    nodes.position =
        Some(position_geometry() + vec3(uniform(1, Type::Float), float(0.0), float(0.0)));
    let mut material = pollster::block_on(nodes.build(&renderer, &[])).unwrap();
    material.uniforms[0][0] = 1.0;
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.0,
        far: 2.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 1.0;
    let object = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap()),
        Arc::new(Material::Shader(material)),
    )));
    let target = RenderTarget::with_options(
        &renderer.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let first = renderer.read_rgba(&target).unwrap();
    assert_eq!(
        &first[(16 * 32 + 16) * 4..(16 * 32 + 16) * 4 + 4],
        &[128, 191, 0, 255]
    );
    let resources = renderer.resource_counts();
    let transfers = renderer.transfer_counts();
    if let NodeKind::Mesh(mesh) = &mut scene.get_mut(object).unwrap().kind
        && let Material::Shader(ShaderMaterial { uniforms, .. }) =
            Arc::make_mut(&mut mesh.materials[0])
    {
        uniforms[0][0] = 0.5;
        uniforms[1][0] = 0.75;
    }
    renderer.render(&mut scene, camera, &target).unwrap();
    let next = renderer.read_rgba(&target).unwrap();
    assert_eq!(
        &next[(16 * 32 + 28) * 4..(16 * 32 + 28) * 4 + 4],
        &[64, 191, 0, 255]
    );
    assert_eq!(next[(16 * 32 + 16) * 4], 0);
    assert_eq!(renderer.resource_counts(), resources);
    assert_eq!(renderer.transfer_counts(), transfers);
    let invalid = WgslFn::new(
        "invalid",
        "fn invalid(x:f32)->f32{return missing;}",
        &[Type::Float],
        Type::Float,
    )
    .unwrap();
    assert!(
        pollster::block_on(NodeMaterial::new(invalid.call(&[float(1.0)])).build(&renderer, &[]))
            .is_err()
    );
}

#[test]
fn graph_passes_generate_and_blur_a_texture_on_gpu() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let target = || {
        RenderTarget::with_options(
            &renderer.device,
            32,
            32,
            RenderTargetOptions {
                format,
                depth_buffer: false,
                ..Default::default()
            },
        )
        .unwrap()
    };
    let dummy = target();
    let generated = target();
    let output = target();
    let mut pass = pollster::block_on(effect(
        &renderer,
        format,
        &checker(uv() * uniform(0, Type::Float)),
    ))
    .unwrap();
    pass.parameters[0][0] = 2.0;
    pass.apply(&renderer, &dummy, None, &generated).unwrap();
    let before = renderer.read_rgba(&generated).unwrap();
    assert_eq!(before[(3 * 32 + 3) * 4], 0);
    assert_eq!(before[(3 * 32 + 11) * 4], 255);
    let graph =
        gaussian_blur(Texture::Input, uv(), vec2(float(1.0 / 32.0), float(0.0)), 2).unwrap();
    let blur = pollster::block_on(effect(&renderer, format, &graph)).unwrap();
    blur.apply(&renderer, &generated, None, &output).unwrap();
    let after = renderer.read_rgba(&output).unwrap();
    assert!(after[(3 * 32 + 7) * 4] > 50 && after[(3 * 32 + 7) * 4] < 128);
    assert!(blur.apply(&renderer, &output, None, &output).is_err());
}
