use three_rs_wasm::{curve::*, math::Vector3};
#[test]
fn catmull_rom_matches_original_including_endpoints_and_repeated_points() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/curves.json")).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let vector = |p: &serde_json::Value| {
            Vector3::new(
                p[0].as_f64().unwrap(),
                p[1].as_f64().unwrap(),
                p[2].as_f64().unwrap(),
            )
        };
        let curve = CatmullRomCurve3 {
            points: fixture["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(vector)
                .collect(),
            closed: fixture["closed"].as_bool().unwrap(),
            curve_type: match fixture["type"].as_str().unwrap() {
                "centripetal" => CatmullRomType::Centripetal,
                "chordal" => CatmullRomType::Chordal,
                _ => CatmullRomType::Uniform {
                    tension: fixture["tension"].as_f64().unwrap(),
                },
            },
        };
        for (actual, expected) in curve
            .points(31)
            .unwrap()
            .iter()
            .zip(fixture["samples"].as_array().unwrap())
        {
            assert!(
                actual.distance(vector(expected)) < 1e-12,
                "{actual:?} != {expected}"
            );
        }
    }
}
