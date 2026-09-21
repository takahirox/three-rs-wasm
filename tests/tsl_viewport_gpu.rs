use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, *},
};
fn target(r: &Renderer) -> RenderTarget {
    RenderTarget::with_options(
        &r.device,
        64,
        64,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap()
}
fn scene() -> (Scene, Object3D, Object3D) {
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.1,
        far: 10.0,
        ..Default::default()
    })));
    s.get_mut(c).unwrap().position.z = 2.0;
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::default()),
    )));
    (s, c, h)
}
fn material(s: &mut Scene, h: Object3D, m: Material) {
    if let NodeKind::Mesh(mesh) = &mut s.get_mut(h).unwrap().kind {
        mesh.materials = vec![Arc::new(m)];
    }
}
#[test]
fn snapshots_follow_transparent_order_and_keep_depth_with_msaa() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let out = target(&r);
        let (mut s, c, backdrop) = scene();
        let mut base = MeshBasicMaterial::default();
        base.properties.color = Color::linear(0.8, 0.4, 0.2);
        material(&mut s, backdrop, Material::Basic(base));
        for z in [0.2, 0.4] {
            let program = SurfaceNodes {
                backdrop: Some(vec4(
                    tsl::viewport::color(tsl::viewport::screen_uv()).rgb() * float(0.5),
                    float(1.0),
                )),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap();
            let mut m = MeshBasicMaterial::default();
            m.properties.transparent = true;
            m.properties.vertex_program = Some(Arc::new(program));
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(1.5, 1.5, 1, 1).unwrap()),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(h).unwrap().position.z = z;
        }
        for samples in [1, 4] {
            let hdr = RenderTarget::with_options(
                &r.device,
                64,
                64,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    samples,
                    ..Default::default()
                },
            )
            .unwrap();
            r.render(&mut s, c, &hdr).unwrap();
            r.blit_tone_mapped(
                &hdr,
                &out.texture.create_view(&Default::default()),
                wgpu::TextureFormat::Rgba8Unorm,
                1.0,
                false,
            );
            let pixels = r.read_rgba(&out).unwrap();
            let p = &pixels[(32 * 64 + 32) * 4..][..3];
            for (a, e) in p.iter().zip([51i32, 26, 13]) {
                assert!((*a as i32 - e).abs() <= 1, "{samples}: {p:?}");
            }
        }
    });
}
#[test]
fn fsr1_preserves_constant_color_and_alpha() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = target(&r);
        let output = target(&r);
        let color = vec4(vec3(float(0.25), float(0.5), float(0.75)), float(0.5));
        let draw = tsl::effect(&r, wgpu::TextureFormat::Rgba8Unorm, &color)
            .await
            .unwrap();
        draw.apply(&r, &input, None, &output).unwrap();
        let up = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::fsr1::easu(tsl::Texture::Input, uv()),
        )
        .await
        .unwrap();
        up.apply(&r, &output, None, &input).unwrap();
        let sharpen = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::fsr1::rcas(
                tsl::Texture::Input,
                uv(),
                float(0.2),
                float(0.0).greater_than(float(1.0)),
            ),
        )
        .await
        .unwrap();
        sharpen.apply(&r, &input, None, &output).unwrap();
        let p = r.read_rgba(&output).unwrap();
        for (a, e) in p[(32 * 64 + 32) * 4..][..4]
            .iter()
            .zip([64i32, 128, 191, 128])
        {
            assert!((*a as i32 - e).abs() <= 1);
        }
    });
}

#[test]
fn viewport_depth_resolves_msaa_and_double_sided_color_recaptures() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let (mut s, c, wall) = scene();
        let mut base = MeshBasicMaterial::default();
        base.properties.color = Color::linear(0.8, 0.4, 0.2);
        material(&mut s, wall, Material::Basic(base));
        let program = SurfaceNodes {
            backdrop: Some(vec4(
                tsl::viewport::color(tsl::viewport::screen_uv()).rgb() * float(0.5),
                float(1.0),
            )),
            ..Default::default()
        }
        .build(&r, &[], &[])
        .await
        .unwrap();
        let mut m = MeshBasicMaterial::default();
        m.properties.transparent = true;
        m.properties.side = Side::Double;
        m.properties.vertex_program = Some(Arc::new(program));
        let box_ = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1.5, 1.5, 0.5).unwrap()),
            Arc::new(Material::Basic(m)),
        )));
        s.get_mut(box_).unwrap().position.z = 0.5;
        for samples in [1, 4] {
            let out = RenderTarget::with_options(
                &r.device,
                64,
                64,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    samples,
                    ..Default::default()
                },
            )
            .unwrap();
            s.get_mut(box_).unwrap().visible = true;
            r.render(&mut s, c, &out).unwrap();
            let pixels = r.read_rgba(&out).unwrap();
            for (a, e) in pixels[(32 * 64 + 32) * 4..][..3]
                .iter()
                .zip([51i32, 26, 13])
            {
                assert!(
                    (*a as i32 - e).abs() <= 1,
                    "both sides must capture: {samples}"
                );
            }
            s.get_mut(box_).unwrap().visible = false;
            let program = SurfaceNodes {
                backdrop: Some(vec4(
                    splat(tsl::viewport::depth(tsl::viewport::screen_uv()), Type::Vec3),
                    float(1.0),
                )),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap();
            let mut m = MeshBasicMaterial::default();
            m.properties.transparent = true;
            m.properties.vertex_program = Some(Arc::new(program));
            let probe = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1).unwrap()),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(probe).unwrap().position.z = 0.2;
            r.render(&mut s, c, &out).unwrap();
            let pixels = r.read_rgba(&out).unwrap();
            assert!((pixels[(32 * 64 + 32) * 4] as f32 / 255.0 - 1.9 / 9.9).abs() < 0.005);
            s.get_mut(probe).unwrap().visible = false;
        }
    });
}

#[test]
fn ordinary_tsl_materials_keep_four_texture_slots() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = r
            .upload_texture(&Arc::new(
                three_rs_wasm::material::Texture::from_rgba(1, 1, vec![64, 128, 192, 255], false)
                    .unwrap(),
            ))
            .unwrap();
        let color = (tsl::Texture::External(0).sample(uv())
            + tsl::Texture::External(1).sample(uv())
            + tsl::Texture::External(2).sample(uv())
            + tsl::Texture::External(3).sample(uv()))
            * float(0.25);
        let shader = NodeMaterial {
            position: None,
            color,
        }
        .build(&r, &[(&input.view, &input.sampler); 4])
        .await
        .unwrap();
        let (mut s, c, h) = scene();
        material(&mut s, h, Material::Shader(shader));
        let out = target(&r);
        r.render(&mut s, c, &out).unwrap();
        let pixels = r.read_rgba(&out).unwrap();
        assert_eq!(&pixels[(32 * 64 + 32) * 4..][..4], &[64, 128, 192, 255]);
    });
}
