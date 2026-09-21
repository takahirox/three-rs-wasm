use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    environment::EnvironmentMap,
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
fn cube_pmrem_faces_and_custom_pbr_environment() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let out = target(&r);
        let (mut s, c, h) = scene();
        let colors = [
            [1., 0., 0., 1.],
            [0., 1., 0., 1.],
            [0., 0., 1., 1.],
            [1., 1., 0., 1.],
            [1., 0., 1., 1.],
            [0., 1., 1., 1.],
        ];
        let pixels: Vec<half::f16> = colors
            .iter()
            .flat_map(|c| (0..32 * 32).flat_map(move |_| c.iter().map(|v| half::f16::from_f32(*v))))
            .collect();
        assert!(EnvironmentMap::from_cube_hdr(&r, 31, &pixels).is_err());
        let mut env = EnvironmentMap::from_cube_hdr(&r, 32, &pixels).unwrap();
        let gpu = env.prefilter(&r).unwrap();
        // Three.js image cubemaps reverse X, in contrast with render-target cubemaps.
        for (dir, index) in [
            ([1., 0., 0.], 1),
            ([-1., 0., 0.], 0),
            ([0., 1., 0.], 2),
            ([0., -1., 0.], 3),
            ([0., 0., 1.], 4),
            ([0., 0., -1.], 5),
        ] {
            let node = tsl::environment::pmrem(
                tsl::Texture::External(0),
                vec3(float(dir[0]), float(dir[1]), float(dir[2])),
                float(0.),
                float(gpu.max_mip()),
            );
            let m = NodeMaterial::new(node)
                .build(&r, &[(gpu.view(), gpu.sampler())])
                .await
                .unwrap();
            material(&mut s, h, Material::Shader(m));
            r.render(&mut s, c, &out).unwrap();
            let px = r.read_rgba(&out).unwrap();
            for channel in 0..3 {
                assert!(
                    (px[(32 * 64 + 32) * 4 + channel] as f32 - colors[index][channel] * 255.).abs()
                        < 3.
                );
            }
        }
        let graph = SurfaceNodes {
            environment: Some(uniform(0, Type::Vec3)),
            ..Default::default()
        };
        let program = Arc::new(graph.build(&r, &[], &[]).await.unwrap());
        s.environment = None;
        for value in [0., 0.5, 1.] {
            let mut m = MeshStandardMaterial {
                roughness: 0.5,
                metalness: 1.0,
                energy_conservation: true,
                ..Default::default()
            };
            m.properties.vertex_program = Some(program.clone());
            m.properties.vertex_uniforms[0] = [value, value, value, 0.];
            material(&mut s, h, Material::Standard(m));
            r.render(&mut s, c, &out).unwrap();
            let px = r.read_rgba(&out).unwrap();
            let center = px[(32 * 64 + 32) * 4];
            assert!(
                (center as f32 - value * 255.).abs() < 3.,
                "{value}: {center}"
            );
        }
        assert!(
            SurfaceNodes {
                output: Some(environment_roughness()),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .is_err()
        );
        assert!(EnvironmentMap::from_cube_scene(&r, &mut s, Vector3::ZERO, 1., 0.5, 32).is_err());
        let mut capture = Scene::new();
        let origin = Vector3::new(5.0, 4.0, 3.0);
        let mut red = MeshBasicMaterial::default();
        red.properties.color = Color::from_hex(0xff0000);
        let wall = capture.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1.0, 2.0, 2.0).unwrap()),
            Arc::new(Material::Basic(red)),
        )));
        capture.get_mut(wall).unwrap().position = origin + Vector3::X * 3.0;
        let before = capture.handles().count();
        let mut captured =
            EnvironmentMap::from_cube_scene(&r, &mut capture, origin, 0.1, 10.0, 32).unwrap();
        assert_eq!(before, capture.handles().count());
        let gpu = captured.prefilter(&r).unwrap();
        let color = tsl::environment::pmrem(
            tsl::Texture::External(0),
            vec3(float(1.0), float(0.0), float(0.0)),
            float(0.0),
            float(gpu.max_mip()),
        );
        material(
            &mut s,
            h,
            Material::Shader(
                NodeMaterial::new(color)
                    .build(&r, &[(gpu.view(), gpu.sampler())])
                    .await
                    .unwrap(),
            ),
        );
        r.render(&mut s, c, &out).unwrap();
        let px = r.read_rgba(&out).unwrap();
        assert!(px[(32 * 64 + 32) * 4] > 250);
        assert_eq!(px[(32 * 64 + 32) * 4 + 1], 0);
    });
}
#[test]
fn alpha_hash_endpoints_and_coverage() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let out = target(&r);
        let (mut s, c, h) = scene();
        s.background = Color::BLACK;
        let alpha = alpha_hash(
            uniform(0, Type::Float),
            position_local(),
            float(1.).greater_than(float(0.)),
        );
        let program = Arc::new(
            SurfaceNodes {
                color: Some(vec4(splat(float(1.), Type::Vec3), alpha)),
                ..Default::default()
            }
            .build(&r, &[], &[])
            .await
            .unwrap(),
        );
        for (value, min, max) in [(0., 0, 0), (0.5, 1400, 2700), (1., 4096, 4096)] {
            let mut m = MeshBasicMaterial::default();
            m.properties.vertex_program = Some(program.clone());
            m.properties.vertex_uniforms[0][0] = value;
            material(&mut s, h, Material::Basic(m));
            r.render(&mut s, c, &out).unwrap();
            let pixels = r.read_rgba(&out).unwrap();
            let coverage = pixels
                .as_chunks::<4>()
                .0
                .iter()
                .filter(|p| p[0] > 0)
                .count();
            assert!((min..=max).contains(&coverage), "{value}: {coverage}");
        }
    });
}
#[test]
fn chromatic_channel_offsets_on_gpu() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = target(&r);
        let out = target(&r);
        let blank = target(&r);
        let gradient = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &vec4(vec3(uv().x(), uv().y(), uv().x()), float(1.)),
        )
        .await
        .unwrap();
        gradient.apply(&r, &blank, None, &input).unwrap();
        let mut pass = tsl::effect(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::display::chromatic_aberration(
                tsl::Texture::Input,
                uv(),
                uniform(0, Type::Float),
                vec2(float(0.5), float(0.5)),
                float(1.2),
            ),
        )
        .await
        .unwrap();
        for strength in [0., 3.] {
            pass.parameters[0][0] = strength;
            pass.apply(&r, &input, None, &out).unwrap();
            let data = r.read_rgba(&out).unwrap();
            let x = (48.5 / 64.) as f32;
            let y = (32.5 / 64.) as f32;
            let distance = ((x - 0.5).powi(2) + (y - 0.5).powi(2)).sqrt();
            let shift = 0.02 * 1.2 * strength + 0.01 * strength * distance;
            let expected = [
                (0.5 + (x - 0.5) * (1. + shift)) * 255.,
                y * 255.,
                (0.5 + (x - 0.5) * (1. - shift)) * 255.,
            ];
            for channel in 0..3 {
                assert!((data[(32 * 64 + 48) * 4 + channel] as f32 - expected[channel]).abs() < 2.);
            }
        }
    });
}
