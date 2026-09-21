use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    postprocessing::ssaa::SsaaPass,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
#[test]
fn inline_output_preserves_lighting_and_updates_without_geometry_uploads() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let program = Arc::new(
        pollster::block_on(output_program(
            &r,
            &vec4(
                saturation(output().rgb(), uniform(0, Type::Float)),
                output().swizzle("w"),
            ),
        ))
        .unwrap(),
    );
    assert!(pollster::block_on(output_program(&r, &uv())).is_err());
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    s.get_mut(c).unwrap().position.z = 3.0;
    s.insert(NodeKind::Light(Light::Ambient {
        color: Color::WHITE,
        intensity: 3.0,
    }));
    let mut m = MeshPhongMaterial::default();
    m.properties.color = Color::from_hex(0xff2200);
    m.properties.vertex_program = Some(program);
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(1.0, 1.0, 1.0).unwrap()),
        Arc::new(Material::Phong(m)),
    )));
    let target = RenderTarget::new(&r.device, 32, 32).unwrap();
    r.render(&mut s, c, &target).unwrap();
    let before = r.read_rgba(&target).unwrap();
    let p = &before[(16 * 32 + 16) * 4..][..3];
    assert_eq!(p[0], p[1]);
    assert_eq!(p[1], p[2]);
    assert!(p[0] > 20);
    let transfers = r.transfer_counts();
    if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
        Arc::make_mut(&mut m.materials[0])
            .properties_mut()
            .vertex_uniforms[0][0] = 1.0;
    }
    r.render(&mut s, c, &target).unwrap();
    let after = r.read_rgba(&target).unwrap();
    let p = &after[(16 * 32 + 16) * 4..][..3];
    assert!(p[0] > p[1] + 40);
    assert_eq!(r.transfer_counts(), transfers);
}
#[test]
fn ssaa_accumulates_gpu_samples_and_restores_camera_even_on_error() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    s.background = Color::linear(0.0, 0.0, 1.0);
    s.background_alpha = 0.25;
    let view = ViewOffset {
        full_width: 16.0,
        full_height: 16.0,
        offset_x: 0.25,
        offset_y: -0.25,
        width: 16.0,
        height: 16.0,
    };
    let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        view: Some(view),
        ..Default::default()
    })));
    s.get_mut(c).unwrap().position.z = 3.0;
    let hdr = RenderTarget::with_options(
        &r.device,
        16,
        16,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
    .unwrap();
    let output = RenderTarget::with_options(
        &r.device,
        16,
        16,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    let mut ssaa = pollster::block_on(SsaaPass::new(&r)).unwrap();
    for level in [0, 3, 5] {
        ssaa.sample_level = level;
        ssaa.render(&r, &mut s, c, &hdr).unwrap();
        r.blit(
            &hdr,
            &output.texture.create_view(&Default::default()),
            wgpu::TextureFormat::Rgba8Unorm,
        );
        let pixels = r.read_rgba(&output).unwrap();
        assert!(
            pixels.as_chunks::<4>().0.iter().all(|p| p[0] == 0
                && p[1] == 0
                && p[2].abs_diff(64) <= 1
                && p[3].abs_diff(64) <= 1)
        );
    }
    assert!(ssaa.render(&r, &mut s, c, &output).is_err());
    if let NodeKind::Camera(Camera::Perspective(camera)) = &s.get(c).unwrap().kind {
        let restored = camera.view.unwrap();
        assert_eq!(
            (restored.offset_x, restored.offset_y),
            (view.offset_x, view.offset_y)
        );
    }
    let color = vec4(
        display::srgb(vec3(float(0.25), float(0.5), float(1.0))),
        float(1.0),
    );
    assert!(tsl::effect_wgsl(&color).is_ok());
}
