use three_rs_wasm::{
    renderer::*,
    tsl::{
        materialx::{self, Dimension},
        *,
    },
};
#[test]
fn materialx_noise_runs_on_gpu_and_preserves_scalar_perlin() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let make = || {
            RenderTarget::with_options(
                &r.device,
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
        let input = make();
        let out = make();
        let p = vec3(
            uv().x() * float(7.) - float(3.2),
            uv().y() * float(5.) - float(2.6),
            float(-0.72),
        );
        let mut images = vec![];
        for n in [
            mx_noise_float3(p.clone()),
            materialx::perlin(p.clone(), Dimension::D3),
        ] {
            let pass = effect(&r, format, &(n * float(0.5) + float(0.5)))
                .await
                .unwrap();
            pass.apply(&r, &input, None, &out).unwrap();
            images.push(r.read_rgba(&out).unwrap());
        }
        for (a, b) in images[0].iter().zip(&images[1]) {
            assert!(a.abs_diff(*b) <= 1);
        }
        for dim in [Dimension::D2, Dimension::D3] {
            for n in [
                materialx::perlin_vec3(p.clone(), dim),
                materialx::cell(p.clone(), dim),
                materialx::fractal_vec3(p.clone(), dim, float(3.), float(2.), float(0.5)),
                materialx::worley(p.clone(), dim, float(1.), float(0.)),
                materialx::worley(p.clone(), dim, float(1.), float(1.)),
                materialx::worley_vec3(p.clone(), dim, float(1.), float(1.)),
            ] {
                let pass = effect(&r, format, &n).await.unwrap();
                pass.apply(&r, &input, None, &out).unwrap();
                let image = r.read_rgba(&out).unwrap();
                assert!(image.chunks(4).any(|p| p[0] > 0));
                assert!(image.chunks(4).any(|p| p[0] < 240));
            }
        }
    });
}

#[test]
fn materialx_matches_pinned_three_gpu_oracle() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let make = || {
            RenderTarget::with_options(
                &r.device,
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
        let input = make();
        let out = make();
        let oracle = include_bytes!("fixtures/materialx-r186.rgba");
        for (d, dim) in [Dimension::D2, Dimension::D3].into_iter().enumerate() {
            let p = vec3(
                uv().x() * float(7.) - float(3.2),
                uv().y() * float(5.) - float(2.6),
                float(if d == 0 { 0. } else { -0.72 }),
            );
            let nodes = [
                materialx::perlin(p.clone(), dim) * float(0.5) + float(0.5),
                materialx::perlin_vec3(p.clone(), dim),
                materialx::cell(p.clone(), dim),
                materialx::fractal_vec3(p.clone(), Dimension::D3, float(3.), float(2.), float(0.5)),
                materialx::worley(p.clone(), dim, float(1.), float(0.)),
                materialx::worley(p.clone(), dim, float(1.), float(1.)),
                materialx::worley_vec3(p.clone(), dim, float(1.), float(1.)),
            ];
            for (k, n) in nodes.into_iter().enumerate() {
                let pass = effect(&r, format, &n).await.unwrap();
                pass.apply(&r, &input, None, &out).unwrap();
                let actual = r.read_rgba(&out).unwrap();
                let expected = &oracle[(d * 7 + k) * 4096..][..4096];
                let max = actual
                    .iter()
                    .zip(expected)
                    .map(|(a, b)| a.abs_diff(*b))
                    .max()
                    .unwrap();
                assert!(max <= 1, "dim {d}, kind {k}, max error {max}");
            }
        }
    });
}

#[test]
fn rebound_shader_samples_new_texture_without_changing_geometry() {
    use std::sync::Arc;
    use three_rs_wasm::{
        camera::*,
        geometry::*,
        material::{self, Material},
        scene::*,
    };
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut scene = Scene::new();
        let c = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.1,
            far: 10.,
            ..Default::default()
        })));
        scene.get_mut(c).unwrap().position.z = 2.;
        let image = |rgba| {
            r.upload_texture(&Arc::new(
                material::Texture::from_rgba(1, 1, rgba, false).unwrap(),
            ))
            .unwrap()
        };
        let red = image(vec![255, 0, 0, 255]);
        let green = image(vec![0, 255, 0, 255]);
        let graph = NodeMaterial::new(Texture::External(0).sample(uv()));
        let m = graph.build(&r, &[(&red.view, &red.sampler)]).await.unwrap();
        let h = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::Shader(m)),
        )));
        let out = RenderTarget::new(&r.device, 8, 8).unwrap();
        r.render(&mut scene, c, &out).unwrap();
        assert_eq!(&r.read_rgba(&out).unwrap()[..4], &[255, 0, 0, 255]);
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h).unwrap().kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            assert!(Arc::make_mut(&mut m.program).rebind(&r, &[], &[]).is_err());
            Arc::make_mut(&mut m.program)
                .rebind(&r, &[], &[(&green.view, &green.sampler)])
                .unwrap();
        }
        r.render(&mut scene, c, &out).unwrap();
        assert_eq!(&r.read_rgba(&out).unwrap()[..4], &[0, 255, 0, 255]);
    });
}
