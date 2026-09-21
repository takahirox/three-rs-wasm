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
fn scene() -> (Scene, Object3D) {
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
    (s, c)
}
fn target(r: &Renderer) -> RenderTarget {
    RenderTarget::with_options(
        &r.device,
        32,
        32,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap()
}
fn plane(s: &mut Scene, w: f64, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(w, 2.0, 1, 1).unwrap()),
        Arc::new(m),
    )))
}
#[test]
fn subsurface_scattering_transmits_back_light_without_a_diffuse_cosine() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let (mut s, c) = scene();
        let out = target(&r);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.0,
            target: Vector3::ZERO,
        }));
        s.get_mut(light).unwrap().position.z = -3.0;
        let program = SurfaceNodes {
            thickness_color: Some(vec3(float(0.2), float(0.4), float(0.6))),
            thickness: Some(vec4(vec3(float(0.1), float(0.0), float(1.0)), float(2.0))),
            thickness_scale: Some(float(1.0)),
            ..Default::default()
        }
        .build(&r, &[], &[])
        .await
        .unwrap();
        let mut m = MeshPhysicalMaterial::default();
        m.base.properties.vertex_program = Some(Arc::new(program));
        let h = plane(&mut s, 2.0, Material::Physical(m));
        r.render(&mut s, c, &out).unwrap();
        let pixels = r.read_rgba(&out).unwrap();
        let p = &pixels[(16 * 32 + 16) * 4..][..3];
        for (a, b) in p.iter().zip([51u8, 102, 153]) {
            assert!(a.abs_diff(b) <= 2, "{p:?}");
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_program = None;
        }
        r.render(&mut s, c, &out).unwrap();
        let p = r.read_rgba(&out).unwrap();
        assert_eq!(&p[(16 * 32 + 16) * 4..][..3], &[0, 0, 0]);
    });
}
#[test]
fn weighted_oit_is_order_independent_and_keeps_opaque_depth() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let (mut s, c) = scene();
        let out = target(&r);
        let mut opaque = MeshBasicMaterial::default();
        opaque.properties.color = Color::linear(1.0, 0.0, 0.0);
        let o = plane(&mut s, 1.0, Material::Basic(opaque));
        s.get_mut(o).unwrap().position = Vector3::new(-0.5, 0.0, 0.5);
        let hdr = wgpu::TextureFormat::Rgba16Float;
        let beauty = RenderTarget::with_options(
            &r.device,
            32,
            32,
            RenderTargetOptions {
                format: hdr,
                ..Default::default()
            },
        )
        .unwrap();
        let mut accum = RenderTarget::with_options(
            &r.device,
            32,
            32,
            RenderTargetOptions {
                format: hdr,
                count: 2,
                color_formats: vec![hdr, wgpu::TextureFormat::R8Unorm],
                load_depth: true,
                clear_colors: vec![wgpu::Color::TRANSPARENT, wgpu::Color::WHITE],
                ..Default::default()
            },
        )
        .unwrap();
        accum
            .set_depth_texture(beauty.depth_texture().unwrap().clone())
            .unwrap();
        let program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(&r, &tsl::oit::outputs(), &[], &[])
                .await
                .unwrap(),
        );
        let mut handles = vec![];
        for (i, color) in [Color::linear(0.0, 1.0, 0.0), Color::linear(0.0, 0.0, 1.0)]
            .into_iter()
            .enumerate()
        {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = color;
            m.properties.opacity = 0.5;
            m.properties.transparent = true;
            m.properties.depth_write = false;
            m.properties.vertex_program = Some(program.clone());
            m.properties.attachment_blending = tsl::oit::blending();
            let h = plane(&mut s, 2.0, Material::Basic(m));
            s.get_mut(h).unwrap().position.z = i as f64 * 0.1;
            handles.push(h);
        }
        let sampler = r.device.create_sampler(&Default::default());
        let composite = effect_with_textures(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::oit::composite(
                tsl::Texture::Input.sample(uv()),
                tsl::Texture::History.sample(uv()),
                tsl::Texture::External(0).sample(uv()).x(),
            ),
            &[(
                &accum.textures()[1].create_view(&Default::default()),
                &sampler,
            )],
        )
        .await
        .unwrap();
        let mut frames = vec![];
        for reverse in [false, true] {
            for &h in &handles {
                s.get_mut(h).unwrap().visible = false;
            }
            s.get_mut(o).unwrap().visible = true;
            r.render(&mut s, c, &beauty).unwrap();
            s.get_mut(o).unwrap().visible = false;
            for (i, &h) in handles.iter().enumerate() {
                s.get_mut(h).unwrap().visible = true;
                s.get_mut(h).unwrap().render_order = if reverse { 1 - i as i32 } else { i as i32 };
            }
            r.render(&mut s, c, &accum).unwrap();
            composite.apply(&r, &beauty, Some(&accum), &out).unwrap();
            frames.push(r.read_rgba(&out).unwrap());
        }
        for (a, b) in frames[0].iter().zip(&frames[1]) {
            assert!(a.abs_diff(*b) <= 1);
        }
        assert_eq!(&frames[0][(16 * 32 + 8) * 4..][..3], &[255, 0, 0]);
        let p = &frames[0][(16 * 32 + 24) * 4..][..3];
        assert_eq!(p[0], 0);
        assert!(p[1] > 80 && p[2] > 80, "{p:?}");
    });
}
