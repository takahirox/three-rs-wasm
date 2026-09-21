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
fn secondary_uv_lightmap_is_diffuse_irradiance_not_emission() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let out = target(&r);
        let (mut s, c, h) = scene();
        let mut g = PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap();
        g.set_attribute(
            "uv1",
            Attribute::F32(
                three_rs_wasm::attribute::BufferAttribute::new(
                    vec![0.25, 0.75, 0.25, 0.75, 0.25, 0.75, 0.25, 0.75],
                    2,
                    false,
                )
                .unwrap(),
            ),
        );
        if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
            m.geometry = Arc::new(g);
        }
        let mut m = MeshPhongMaterial::default();
        m.properties.color = Color::linear(0.5, 0.25, 1.0);
        m.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                light_map: Some(
                    vec3(uv1().x(), uv1().y(), float(1.0)) * float(std::f32::consts::PI),
                ),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap(),
        ));
        material(&mut s, h, Material::Phong(m));
        r.render(&mut s, c, &out).unwrap();
        let pixels = r.read_rgba(&out).unwrap();
        let p = &pixels[(32 * 64 + 32) * 4..][..3];
        for (actual, expected) in p.iter().zip([32i32, 48, 255]) {
            assert!((*actual as i32 - expected).abs() <= 1, "{p:?}");
        }
        assert!(
            tsl::effect(&r, wgpu::TextureFormat::Rgba8Unorm, &uv1())
                .await
                .is_err()
        );
    });
}
#[test]
fn gpu_box_blur_preserves_kernel_extent_and_alpha() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = target(&r);
        let out = target(&r);
        // Draw a single bright texel; a radius-one box kernel must cover exactly 3x3 pixels.
        let draw = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &vec4(
                splat(
                    (uv().x() * float(64.0))
                        .floor()
                        .equal(float(32.0))
                        .and((uv().y() * float(64.0)).floor().equal(float(32.0)))
                        .select(float(1.0), float(0.0)),
                    Type::Vec3,
                ),
                float(1.0),
            ),
        )
        .await
        .unwrap();
        let blank = target(&r);
        draw.apply(&r, &blank, None, &input).unwrap();
        let blur = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::display::box_blur(
                tsl::Texture::Input,
                uv(),
                float(1.0),
                float(1.0),
                float(0.0).greater_than(float(1.0)),
            ),
        )
        .await
        .unwrap();
        blur.apply(&r, &input, None, &out).unwrap();
        let pixels = r.read_rgba(&out).unwrap();
        let mut count = 0;
        for p in pixels.as_chunks::<4>().0 {
            assert_eq!(p[3], 255);
            if p[0] > 0 {
                count += 1;
                assert!((p[0] as i32 - 28).abs() <= 1);
            }
        }
        assert_eq!(count, 9);
    });
}
#[test]
fn lensflare_threshold_and_center_attenuation() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = target(&r);
        let out = target(&r);
        let blank = target(&r);
        let draw = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &vec4(splat(float(0.8), Type::Vec3), float(1.0)),
        )
        .await
        .unwrap();
        draw.apply(&r, &blank, None, &input).unwrap();
        let mut flare = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::display::lensflare(
                tsl::Texture::Input,
                uv(),
                splat(float(1.0), Type::Vec3),
                uniform(0, Type::Vec4),
            ),
        )
        .await
        .unwrap();
        flare.parameters[0] = [0.5, 1.0, 0.25, 25.0];
        flare.apply(&r, &input, None, &out).unwrap();
        let pixels = r.read_rgba(&out).unwrap();
        let expected = (0.3_f64 * (1.0 - (2.0_f64).sqrt() / 128.0).powi(25) * 255.0).round() as i32;
        assert!((pixels[(32 * 64 + 32) * 4] as i32 - expected).abs() <= 1);
        assert_eq!(pixels[0], 0);
        flare.parameters[0][0] = 1.0;
        flare.apply(&r, &input, None, &out).unwrap();
        assert!(
            r.read_rgba(&out)
                .unwrap()
                .as_chunks::<4>()
                .0
                .iter()
                .all(|p| p[..3] == [0, 0, 0])
        );
    });
}

#[test]
fn transmission_snapshot_preserves_opaque_color_and_depth() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let (mut s, c, backdrop) = scene();
        material(
            &mut s,
            backdrop,
            Material::Basic(MeshBasicMaterial {
                properties: MaterialProperties {
                    color: Color::linear(0.2, 0.5, 0.8),
                    ..Default::default()
                },
            }),
        );
        let glass = MeshPhysicalMaterial {
            transmission: 1.0,
            thickness: 0.2,
            base: MeshStandardMaterial {
                roughness: 0.1,
                ..Default::default()
            },
            ..Default::default()
        };
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.55, 24, 16).unwrap()),
            Arc::new(Material::Physical(glass)),
        )));
        s.get_mut(h).unwrap().position.z = 0.6;
        let foreground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(0.4, 0.8, 1, 1).unwrap()),
            Arc::new(Material::Basic(MeshBasicMaterial {
                properties: MaterialProperties {
                    color: Color::linear(0.8, 0.1, 0.2),
                    ..Default::default()
                },
            })),
        )));
        s.get_mut(foreground).unwrap().position = Vector3::new(0.2, 0.0, 1.3);
        let out = target(&r);
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
            let mut images = Vec::new();
            for fallback in [false, true, false] {
                // Hooks retain the old two-scene-render path, providing an independent oracle.
                s.get_mut(backdrop).unwrap().render_hooks.before =
                    fallback.then(|| Arc::new(|_: &mut Scene, _: Object3D| {}) as _);
                r.render(&mut s, c, &hdr).unwrap();
                r.blit_tone_mapped(
                    &hdr,
                    &out.texture.create_view(&Default::default()),
                    wgpu::TextureFormat::Rgba8Unorm,
                    1.0,
                    false,
                );
                images.push(r.read_rgba(&out).unwrap());
            }
            for actual in [&images[0], &images[2]] {
                assert!(
                    actual
                        .iter()
                        .zip(&images[1])
                        .all(|(a, b)| a.abs_diff(*b) <= 1),
                    "samples={samples}"
                );
            }
            let center = &images[0][(32 * 64 + 36) * 4..][..3];
            assert!(
                center[0] > center[1] * 2,
                "opaque foreground lost: {center:?}"
            );
        }
    });
}
