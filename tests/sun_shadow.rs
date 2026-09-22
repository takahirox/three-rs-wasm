use three_rs_wasm::{camera::*, math::*, shadow::Shadow, sun_shadow::cascades};
#[test]
fn stabilized_cascades_match_pinned_three_matrices() {
    let rows: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/sun-cascades-r186.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let camera = if row["ortho"].as_bool().unwrap() {
            Camera::Orthographic(OrthographicCamera {
                left: -4.,
                right: 4.,
                top: 3.,
                bottom: -3.,
                near: 0.1,
                far: 100.,
                ..Default::default()
            })
        } else {
            Camera::Perspective(PerspectiveCamera {
                fov: 35.,
                aspect: 1.5,
                near: 0.1,
                far: 100.,
                ..Default::default()
            })
        };
        let matrix: [f64; 16] = serde_json::from_value(row["camera"].clone()).unwrap();
        let p: [f64; 3] = serde_json::from_value(row["position"].clone()).unwrap();
        let actual = cascades(
            &camera,
            Matrix4::from_cols_array(&matrix),
            Vector3::from_array(p),
            Shadow {
                far: 30.,
                ..Default::default()
            },
            1024,
        )
        .unwrap();
        for (i, c) in actual.iter().enumerate() {
            let expected: [f64; 16] =
                serde_json::from_value(row["cascades"][i]["matrix"].clone()).unwrap();
            for (a, b) in c.projection_view.to_cols_array().iter().zip(expected) {
                assert!((a - b).abs() < 1e-10, "{a} {b}");
            }
            let range: [f64; 4] =
                serde_json::from_value(row["cascades"][i]["range"].clone()).unwrap();
            for (a, b) in c.range.to_array().iter().zip(range) {
                assert!((a - b).abs() < 1e-10);
            }
        }
    }
}
