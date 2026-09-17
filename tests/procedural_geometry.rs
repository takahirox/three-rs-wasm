use three_rs_wasm::{geometry::*, math::*};
#[test]
fn procedural_meshes_match_pinned_three() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/procedural-geometries.json")).unwrap();
    let points = (0..8)
        .map(|i| {
            let i = i as f64;
            Vector2::new(
                (i * 0.2).sin() * (i * 0.1).sin() * 15.0 + 50.0,
                (i - 5.0) * 2.0,
            )
        })
        .collect::<Vec<_>>();
    let cases = [
        ("box", BoxGeometry::segmented(2.0, 3.0, 4.0, 2, 3, 4)),
        ("capsule", CapsuleGeometry::build(2.0, 3.0, 4, 7, 2)),
        ("icosahedron", IcosahedronGeometry::build(3.0, 0)),
        ("icosphere", IcosahedronGeometry::build(3.0, 2)),
        ("octahedron", OctahedronGeometry::build(3.0, 0)),
        ("tetrahedron", TetrahedronGeometry::build(3.0, 0)),
        ("circle", CircleGeometry::build(3.0, 7, 0.2, 4.0)),
        ("ring", RingGeometry::build(1.0, 3.0, 7, 3, 0.2, 4.0)),
        ("torus", TorusGeometry::build(3.0, 0.7, 7, 9, 4.0, 0.3, 5.0)),
        ("knot", TorusKnotGeometry::build(3.0, 0.7, 12, 7, 2, 3)),
        ("lathe", LatheGeometry::build(&points, 7, 0.2, 4.0)),
        (
            "cylinder",
            CylinderGeometry::build(1.0, 3.0, 4.0, 7, 3, false, 0.2, 4.0),
        ),
        (
            "cone",
            CylinderGeometry::build(0.0, 3.0, 4.0, 7, 3, false, 0.2, 4.0),
        ),
        (
            "parametric",
            ParametricGeometry::build(
                |u, v| Vector3::new(u, (u * 4.0).sin() * (v * 3.0).cos(), v),
                7,
                5,
            ),
        ),
    ];
    for (name, result) in cases {
        let geometry = result.unwrap();
        let expected = &fixture[name];
        assert_eq!(
            serde_json::to_value(&geometry.index).unwrap(),
            expected["index"],
            "{name}: index"
        );
        for (key, values) in expected["attributes"].as_object().unwrap() {
            let attribute = &geometry.attributes[key];
            let values = values.as_array().unwrap();
            assert_eq!(
                attribute.count() * attribute.item_size(),
                values.len(),
                "{name}: {key} length"
            );
            for (i, value) in values.iter().enumerate() {
                let actual = attribute
                    .get_component(i / attribute.item_size(), i % attribute.item_size())
                    .unwrap();
                let expected = value.as_f64().unwrap();
                assert!(
                    (actual - expected).abs() < 1e-6,
                    "{name}: {key}[{i}]: {actual} != {expected}"
                );
            }
        }
        assert_eq!(
            geometry.groups.len(),
            expected["groups"].as_array().unwrap().len()
        );
        for (actual, expected) in geometry
            .groups
            .iter()
            .zip(expected["groups"].as_array().unwrap())
        {
            assert_eq!(actual.start as u64, expected["start"].as_u64().unwrap());
            assert_eq!(actual.count as u64, expected["count"].as_u64().unwrap());
            assert_eq!(
                actual.material_index as u64,
                expected["materialIndex"].as_u64().unwrap()
            );
        }
    }
}
