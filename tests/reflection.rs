use three_rs_wasm::{camera::*, math::*, reflection::*};
#[test]
fn reflection_mirrors_camera_and_clips_the_opposite_halfspace() {
    let camera = PerspectiveCamera {
        aspect: 1.5,
        ..Default::default()
    };
    let world = Matrix4::look_at_rh(Vector3::new(2., 3., 4.), Vector3::ZERO, Vector3::Y).inverse();
    let (reflected, matrix) = planar_camera(&camera, world, Vector4::new(0., 1., 0., 0.))
        .unwrap()
        .unwrap();
    assert!(matrix.w_axis.truncate().distance(Vector3::new(2., -3., 4.)) < 1e-10);
    let projection = reflected.projection_matrix().unwrap();
    let clip = |y| projection * matrix.inverse() * Vector4::new(0., y, 0., 1.);
    assert!(clip(1.).z > 0.);
    assert!(clip(-1.).z < 0.);
    assert!(clip(0.).z.abs() < 1e-10);
    assert!(
        planar_camera(&camera, matrix, Vector4::new(0., 1., 0., 0.))
            .unwrap()
            .is_none()
    );
    assert!(oblique_projection(camera.projection_matrix().unwrap(), Vector4::ZERO).is_err());
}
#[test]
fn clipping_does_not_change_projected_xy_or_serialize_an_unused_plane() {
    let camera = PerspectiveCamera::default();
    let p = camera.projection_matrix().unwrap();
    let q = oblique_projection(p, Vector4::new(0., 0., -1., -2.)).unwrap();
    for v in [
        Vector4::new(1., 2., -3., 1.),
        Vector4::new(-2., 1., -4., 1.),
    ] {
        let a = p * v;
        let b = q * v;
        assert_eq!((a.x, a.y, a.w), (b.x, b.y, b.w));
    }
    assert!(
        !serde_json::to_string(&camera)
            .unwrap()
            .contains("oblique_clip_plane")
    );
}

#[test]
fn reflected_background_ray_keeps_forward_orientation_beyond_infinity() {
    let camera = PerspectiveCamera {
        fov: 50.,
        near: 0.25,
        far: 30.,
        ..Default::default()
    };
    let world = Matrix4::look_at_rh(
        Vector3::new(-4., 1., 4.),
        Vector3::new(0., 0.75, 0.),
        Vector3::Y,
    )
    .inverse();
    let (reflected, _) = planar_camera(&camera, world, Vector4::new(0., 1., 0., 0.))
        .unwrap()
        .unwrap();
    let base = camera.projection_matrix().unwrap().inverse();
    let clipped = reflected.projection_matrix().unwrap().inverse();
    let mut crosses_infinity = false;
    for y in [-1., -0.5, 0., 0.5, 1.] {
        let a = base * Vector4::new(0.2, y, 1., 1.);
        let b = clipped * Vector4::new(0.2, y, 1., 1.);
        crosses_infinity |= b.w < 0.;
        assert!(a.truncate().normalize().distance(b.truncate().normalize()) < 1e-10);
    }
    assert!(crosses_infinity);
}

#[test]
fn recursive_reflection_preserves_the_official_projection_at_each_bounce() {
    let cases: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/reflector-cameras-r186.json")).unwrap();
    let initial = PerspectiveCamera {
        fov: 45.,
        aspect: 1.5,
        near: 1.,
        far: 500.,
        ..Default::default()
    };
    let world = Matrix4::look_at_rh(
        Vector3::new(0., 75., 160.),
        Vector3::new(0., 40., 0.),
        Vector3::Y,
    )
    .inverse();
    let floor = Vector4::new(0., 1., 0., 0.);
    let wall = Vector4::new(0., 0., 1., 50.);
    for (i, planes) in [[floor, wall], [wall, floor]].into_iter().enumerate() {
        let mut camera = initial.clone();
        let mut matrix = world;
        for (j, plane) in planes.into_iter().enumerate() {
            (camera, matrix) = planar_camera(&camera, matrix, plane).unwrap().unwrap();
            for (key, actual) in [
                ("matrix", matrix),
                ("projection", camera.projection_matrix().unwrap()),
            ] {
                for (a, b) in actual
                    .to_cols_array()
                    .into_iter()
                    .zip(cases[i][j][key].as_array().unwrap())
                {
                    assert!(
                        (a - b.as_f64().unwrap()).abs() < 1e-9,
                        "case {i} bounce {j} {key}: {a} != {b}"
                    );
                }
            }
        }
    }
}
