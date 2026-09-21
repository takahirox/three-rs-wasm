use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
#[test]
fn instanced_line_width_uses_pixels_or_distance_attenuated_world_units() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.,
            aspect: 1.,
            near: 0.1,
            far: 20.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position.z = 2.;
        let out = RenderTarget::new(&r.device, 64, 64).unwrap();
        let segments = [
            [
                [-1., 0.5, 0., 0.],
                [1., 0.5, 0., 2.],
                [0., 1., 0., 1.],
                [0., 1., 0., 1.],
            ],
            [
                [-2., -1., -2., 0.],
                [2., -1., -2., 4.],
                [0., 1., 0., 1.],
                [0., 1., 0., 1.],
            ],
        ];
        let data = tsl::lines::LineSegments::new(&r, &segments).unwrap();
        let mut m = data
            .material(&r, tsl::lines::LineOptions::default())
            .await
            .unwrap();
        let world = data
            .material(
                &r,
                tsl::lines::LineOptions {
                    world_units: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        m.uniforms[0] = [8., 0., 0., 1.];
        m.uniforms[1] = [1., 0., 1., 1.];
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(tsl::lines::geometry(2).unwrap()),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(h).unwrap().frustum_culled = false;
        let widths = |bytes: &[u8]| {
            let mut n = [0; 2];
            for y in 0..64 {
                if bytes[(y * 64 + 32) * 4 + 1] > 200 {
                    n[y / 32] += 1;
                }
            }
            n
        };
        r.render(&mut s, c, &out).unwrap();
        assert_eq!(widths(&r.read_rgba(&out).unwrap()), [8, 8]);
        if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0] = [0.25, 1., 0., 1.];
            m.program = world.program.clone();
        }
        r.render(&mut s, c, &out).unwrap();
        assert_eq!(widths(&r.read_rgba(&out).unwrap()), [4, 2]);
    });
}
#[test]
fn local_normal_is_not_scaled_and_alpha_test_is_dynamic() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.1,
            far: 10.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position.z = 2.;
        let out = RenderTarget::new(&r.device, 16, 16).unwrap();
        let graph = NodeMaterial::new(alpha_test(
            vec4(normal_local(), uniform(0, Type::Float)),
            float(0.5),
        ));
        let mut m = graph.build(&r, &[]).await.unwrap();
        m.uniforms[0][0] = 0.75;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(h).unwrap().scale = Vector3::new(2., 1., 4.);
        r.render(&mut s, c, &out).unwrap();
        let p = r.read_rgba(&out).unwrap();
        assert_eq!(&p[(8 * 16 + 8) * 4..][..3], &[0, 0, 255]);
        if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0][0] = 0.25;
        }
        r.render(&mut s, c, &out).unwrap();
        let p = r.read_rgba(&out).unwrap();
        assert_eq!(&p[(8 * 16 + 8) * 4..][..3], &[0, 0, 0]);
    });
}
