use std::sync::Arc;
use three_rs_wasm::{
    attribute::BufferAttribute,
    compute_skinning::ComputeSkinning,
    deformation::{self, Skin},
    geometry::*,
    material::*,
    math::*,
    renderer::Renderer,
    scene::*,
};
#[test]
fn compute_skinning_matches_query_oracle_and_tracks_world_displacement() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let joint0 = s.insert(NodeKind::Group);
        let joint1 = s.insert(NodeKind::Group);
        let mut g = BufferGeometry::default();
        for (name, size, values) in [
            ("position", 3, vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]),
            (
                "skinIndex",
                4,
                vec![0., 1., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0.],
            ),
            (
                "skinWeight",
                4,
                vec![0.25, 0.75, 0., 0., 1., 0., 0., 0., 0.5, 0.5, 0., 0.],
            ),
        ] {
            g.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(values, size, false).unwrap()),
            );
        }
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Basic(MeshBasicMaterial::default())),
        )));
        s.get_mut(h).unwrap().position = Vector3::new(3., 1., -2.);
        s.get_mut(h).unwrap().skin = Some(Skin {
            joints: vec![joint0, joint1],
            inverse_bind_matrices: vec![
                Matrix4::IDENTITY,
                Matrix4::from_translation(Vector3::new(0., -0.3, 0.)),
            ],
        });
        let skin = ComputeSkinning::new(&r, &s, h).await.unwrap();
        assert_eq!(skin.count(), 3);
        let mut previous = [Vector3::ZERO; 3];
        for t in [0., 0.25, 1., 1.] {
            s.get_mut(joint0).unwrap().quaternion = Quaternion::from_rotation_y(t);
            s.get_mut(joint1).unwrap().position = Vector3::new(t, 2. * t, -t);
            s.get_mut(joint1).unwrap().scale = Vector3::new(1. + t, 1., 1.);
            s.update().unwrap();
            skin.update(&r, &s).unwrap();
            let p = skin.positions.read(&r).unwrap();
            let v = skin.displacement.read(&r).unwrap();
            let p: &[f32] = bytemuck::cast_slice(&p);
            let v: &[f32] = bytemuck::cast_slice(&v);
            let oracle = deformation::evaluate(&s, h).unwrap().unwrap();
            for i in 0..3 {
                let expected = s
                    .get(h)
                    .unwrap()
                    .matrix_world
                    .transform_point3(oracle.attributes["position"].vector3(i).unwrap());
                let actual =
                    Vector3::new(p[i * 4] as f64, p[i * 4 + 1] as f64, p[i * 4 + 2] as f64);
                let displacement =
                    Vector3::new(v[i * 4] as f64, v[i * 4 + 1] as f64, v[i * 4 + 2] as f64);
                assert!(actual.distance(expected) < 1e-5);
                assert!(displacement.distance(expected - previous[i]) < 1e-5);
                previous[i] = expected;
            }
        }
    });
}
