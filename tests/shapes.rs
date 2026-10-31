//! TextGeometry ( FontLoader shapes extruded with ExtrudeGeometry's
//! WorldUVGenerator uvs and computeVertexNormals ) and RoundedBoxGeometry
//! against the pinned r187 geometries ( tools/core/shapes-oracle.mjs ).
use serde_json::Value;
use std::f64::consts::PI;
use three_rs_wasm::geometry::{
    Attribute, BufferGeometry, DodecahedronGeometry, ParametricGeometry, RoundedBoxGeometry,
};
use three_rs_wasm::math::Vector3;
use three_rs_wasm::nurbs::{NurbsCurve, NurbsSurface, NurbsVolume};
use three_rs_wasm::shapes::{
    Extrude, Font, Path, Shape, extrude_uv, shape_geometry, text_geometry,
};

fn oracle() -> Vec<Value> {
    let path = ".cache/shapes-oracle.json";
    if !std::path::Path::new(path).exists() {
        let status = std::process::Command::new("node")
            .arg("tools/core/shapes-oracle.mjs")
            .status()
            .expect("node runs the shapes oracle");
        assert!(status.success(), "the shapes oracle failed");
    }
    serde_json::from_slice::<Value>(&std::fs::read(path).unwrap())
        .unwrap()
        .as_array()
        .unwrap()
        .clone()
}

fn base64(s: &str) -> Vec<u8> {
    let value = |c: u8| match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        _ => 63,
    };
    let bytes: Vec<u8> = s.bytes().filter(|&c| c != b'=').collect();
    let mut out = vec![];
    for chunk in bytes.chunks(4) {
        let mut n = 0u32;
        for (i, &c) in chunk.iter().enumerate() {
            n |= u32::from(value(c)) << (18 - 6 * i);
        }
        for i in 0..chunk.len() - 1 {
            out.push((n >> (16 - 8 * i)) as u8);
        }
    }
    out
}

/// Float32 values agree exactly, or within one unit in the last place where
/// a native `Math.sin` / `cos` ( bevels, rounded corners ) rounds otherwise.
fn compare(name: &str, expected: &Value, g: &BufferGeometry, errors: &mut Vec<String>, ulps: i64) {
    for (key, e) in expected["attributes"].as_object().unwrap() {
        let data: Vec<f32> = base64(e["data"].as_str().unwrap())
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        let Some(Attribute::F32(a)) = g.attributes.get(key) else {
            errors.push(format!("{name}: {key} missing"));
            continue;
        };
        let a = a.array();
        if a.len() != data.len() {
            errors.push(format!(
                "{name}: {key} length {} != {}",
                a.len(),
                data.len()
            ));
            continue;
        }
        let bad: Vec<usize> = (0..a.len())
            .filter(|&i| {
                let (x, y) = (a[i], data[i]);
                !(x == y || (x.to_bits() as i64 - y.to_bits() as i64).abs() <= ulps)
            })
            .collect();
        if let Some(&i) = bad.first() {
            errors.push(format!(
                "{name}: {key} {} values differ, first [{i}] {} != {}",
                bad.len(),
                a[i],
                data[i]
            ));
        }
    }
    let groups: Vec<[u64; 3]> = expected["groups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| [0, 1, 2].map(|k| v[k].as_u64().unwrap()))
        .collect();
    let actual: Vec<[u64; 3]> = g
        .groups
        .iter()
        .map(|v| [v.start as u64, v.count as u64, v.material_index as u64])
        .collect();
    if groups != actual {
        errors.push(format!("{name}: groups {actual:?} != {groups:?}"));
    }
    match (expected["index"].as_str(), &g.index) {
        (None, None) => {}
        (Some(e), Some(a)) => {
            let e: Vec<u32> = base64(e)
                .chunks(4)
                .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
                .collect();
            if &e != a {
                errors.push(format!("{name}: index differs"));
            }
        }
        _ => errors.push(format!("{name}: index presence")),
    }
}

fn f64s(v: &Value) -> Vec<f64> {
    base64(v.as_str().unwrap())
        .chunks(8)
        .map(|c| f64::from_le_bytes(c.try_into().unwrap()))
        .collect()
}
/// Doubles agree to a relative 1e-12 ( `Math.cos` / `sin` natively ).
fn compare_f64(name: &str, expected: &[f64], actual: &[f64], errors: &mut Vec<String>) {
    if expected.len() != actual.len() {
        return errors.push(format!(
            "{name}: length {} != {}",
            actual.len(),
            expected.len()
        ));
    }
    if let Some(i) = (0..expected.len()).find(|&i| {
        let (e, a) = (expected[i], actual[i]);
        !(e == a || (e - a).abs() <= 1e-12 * e.abs().max(1.0))
    }) {
        errors.push(format!("{name}: [{i}] {} != {}", actual[i], expected[i]));
    }
}
fn flat(points: &[[f64; 2]]) -> Vec<f64> {
    points.iter().flatten().copied().collect()
}

/// webgl_geometry_shapes's shapes.
fn page_shapes() -> Vec<(&'static str, Shape)> {
    let shape = |p: Path| Shape {
        outline: p,
        holes: vec![],
    };
    let california: Vec<[f64; 2]> = [
        [610., 320.],
        [450., 300.],
        [392., 392.],
        [266., 438.],
        [190., 570.],
        [190., 600.],
        [160., 620.],
        [160., 650.],
        [180., 640.],
        [165., 680.],
        [150., 670.],
        [90., 737.],
        [80., 795.],
        [50., 835.],
        [64., 870.],
        [60., 945.],
        [300., 945.],
        [300., 743.],
        [600., 473.],
        [626., 425.],
        [600., 370.],
        [610., 320.],
    ]
    .map(|[x, y]| [x * 0.25, y * 0.25])
    .to_vec();
    let mut triangle = Path::default();
    triangle.move_to([80., 20.]);
    for p in [[40., 80.], [120., 80.], [80., 20.]] {
        triangle.line_to(p);
    }
    let mut heart = Path::default();
    heart.move_to([25., 25.]);
    heart.cubic_to([25., 25.], [20., 0.], [0., 0.]);
    heart.cubic_to([-30., 0.], [-30., 35.], [-30., 35.]);
    heart.cubic_to([-30., 55.], [-10., 77.], [25., 95.]);
    heart.cubic_to([60., 77.], [80., 55.], [80., 35.]);
    heart.cubic_to([80., 35.], [80., 0.], [50., 0.]);
    heart.cubic_to([35., 0.], [25., 25.], [25., 25.]);
    let mut square = Path::default();
    square.move_to([0., 0.]);
    for p in [[0., 80.], [80., 80.], [80., 0.], [0., 0.]] {
        square.line_to(p);
    }
    let mut track = Path::default();
    track.move_to([40., 40.]);
    track.line_to([40., 160.]);
    track.absarc([60., 160.], 20., PI, 0., true);
    track.line_to([80., 40.]);
    track.absarc([60., 40.], 20., 2. * PI, PI, true);
    let mut fish = Path::default();
    fish.move_to([0., 0.]);
    fish.quadratic_to([50., -80.], [90., -10.]);
    fish.quadratic_to([100., -10.], [115., -40.]);
    fish.quadratic_to([115., 0.], [115., 40.]);
    fish.quadratic_to([100., 10.], [90., 10.]);
    fish.quadratic_to([50., 80.], [0., 0.]);
    let mut spline = Path::default();
    spline.move_to([0., 0.]);
    spline.spline_thru(&[[70., 20.], [80., 90.], [-30., 70.], [0., 0.]]);
    let mut smiley = Path::default();
    smiley.move_to([80., 40.]);
    smiley.absarc([40., 40.], 40., 0., PI * 2., false);
    let mut eye1 = Path::default();
    eye1.move_to([35., 20.]);
    eye1.absellipse([25., 20.], [10., 10.], 0., PI * 2., true, 0.);
    let mut eye2 = Path::default();
    eye2.move_to([65., 20.]);
    eye2.absarc([55., 20.], 10., 0., PI * 2., true);
    let mut mouth = Path::default();
    mouth.move_to([20., 40.]);
    mouth.quadratic_to([40., 60.], [60., 40.]);
    mouth.cubic_to([70., 45.], [70., 50.], [60., 60.]);
    mouth.quadratic_to([40., 80.], [20., 60.]);
    mouth.quadratic_to([5., 50.], [20., 40.]);
    vec![
        ("california", shape(Path::from_points(&california))),
        ("triangle", shape(triangle)),
        ("heart", shape(heart)),
        ("square", shape(square)),
        ("track", shape(track)),
        ("fish", shape(fish)),
        ("spline", shape(spline)),
        (
            "smiley",
            Shape {
                outline: smiley,
                holes: vec![eye1, eye2, mouth],
            },
        ),
    ]
}

fn geometry_of(
    positions: Vec<f32>,
    uvs: Vec<f32>,
    groups: Vec<(usize, usize, usize)>,
) -> BufferGeometry {
    use three_rs_wasm::attribute::BufferAttribute;
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(positions, 3, false).unwrap()),
    );
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(uvs, 2, false).unwrap()),
    );
    for (s, c, m) in groups {
        g.add_group(s, c, m);
    }
    g.compute_vertex_normals().unwrap();
    g
}

#[test]
fn shapes_match_pinned_three() {
    let mut errors = vec![];
    for case in oracle() {
        match case["kind"].as_str().unwrap() {
            "text" => {
                let font = Font::parse(
                    &std::fs::read(format!(
                        ".cache/three-r187/examples/fonts/{}.typeface.json",
                        case["font"].as_str().unwrap()
                    ))
                    .unwrap(),
                )
                .unwrap();
                let bevel = case["bevel"].as_array().map(|b| {
                    (
                        b[0].as_f64().unwrap(),
                        b[1].as_f64().unwrap(),
                        b[2].as_u64().unwrap() as usize,
                    )
                });
                let text = case["text"].as_str().unwrap();
                let g = text_geometry(
                    &font,
                    text,
                    case["size"].as_f64().unwrap(),
                    &Extrude {
                        curve_segments: case["curveSegments"].as_u64().unwrap() as usize,
                        steps: 1,
                        depth: case["depth"].as_f64().unwrap(),
                        bevel,
                    },
                )
                .unwrap();
                let mut g = g;
                if case["center"].as_bool() == Some(true) {
                    // computeBoundingBox(), then translate by half the negated size.
                    let b = g.compute_bounding_box().unwrap();
                    let size = b.max - b.min;
                    g.translate(-0.5 * size).unwrap();
                }
                let ulps = if bevel.is_some() { 1 } else { 0 };
                compare(
                    &format!("text {text:?}"),
                    &case["geometry"],
                    &g,
                    &mut errors,
                    ulps,
                );
            }
            "shape" => {
                let name = case["name"].as_str().unwrap();
                let (_, shape) = page_shapes().into_iter().find(|(n, _)| *n == name).unwrap();
                let g = shape_geometry(std::slice::from_ref(&shape), 12, false).unwrap();
                compare(&format!("shape {name}"), &case["flat"], &g, &mut errors, 1);
                let (p, uv, groups) = extrude_uv(
                    std::slice::from_ref(&shape),
                    &Extrude {
                        curve_segments: 12,
                        steps: 2,
                        depth: 8.,
                        bevel: Some((1., 1., 2)),
                    },
                );
                let g = geometry_of(p, uv, groups);
                compare(
                    &format!("extruded {name}"),
                    &case["extruded"],
                    &g,
                    &mut errors,
                    1,
                );
                let mut outline = shape.outline.clone();
                outline.auto_close = true;
                compare_f64(
                    &format!("{name} points"),
                    &f64s(&case["points"]),
                    &flat(&outline.points(12)),
                    &mut errors,
                );
                compare_f64(
                    &format!("{name} spaced"),
                    &f64s(&case["spaced"]),
                    &flat(&outline.spaced_points(50)),
                    &mut errors,
                );
                for (k, (h, e)) in shape
                    .holes
                    .iter()
                    .zip(case["holes"].as_array().unwrap())
                    .enumerate()
                {
                    let mut h = h.clone();
                    h.auto_close = true;
                    compare_f64(
                        &format!("{name} hole {k} points"),
                        &f64s(&e["points"]),
                        &flat(&h.points(12)),
                        &mut errors,
                    );
                    compare_f64(
                        &format!("{name} hole {k} spaced"),
                        &f64s(&e["spaced"]),
                        &flat(&h.spaced_points(50)),
                        &mut errors,
                    );
                }
            }
            "textShapes" => {
                let font = Font::parse(
                    &std::fs::read(format!(
                        ".cache/three-r187/examples/fonts/{}.typeface.json",
                        case["font"].as_str().unwrap()
                    ))
                    .unwrap(),
                )
                .unwrap();
                let text = case["text"].as_str().unwrap();
                let shapes = font.shapes_in(
                    text,
                    case["size"].as_f64().unwrap(),
                    case["direction"].as_str().unwrap(),
                );
                let g = shape_geometry(&shapes, 12, true).unwrap();
                compare(
                    &format!("text shapes {text:?}"),
                    &case["geometry"],
                    &g,
                    &mut errors,
                    0,
                );
                let mut outlines = vec![];
                for s in &shapes {
                    outlines.push(s.outline.points(12));
                    for h in &s.holes {
                        outlines.push(h.points(12));
                    }
                }
                let expected = case["outlines"].as_array().unwrap();
                if expected.len() != outlines.len() {
                    errors.push(format!(
                        "text shapes {text:?}: {} outlines != {}",
                        outlines.len(),
                        expected.len()
                    ));
                }
                for (k, (e, a)) in expected.iter().zip(&outlines).enumerate() {
                    compare_f64(
                        &format!("text {text:?} outline {k}"),
                        &f64s(e),
                        &flat(a),
                        &mut errors,
                    );
                }
            }
            "nurbsCurve" => {
                let control = f64s(&case["control"]);
                let points: Vec<[f64; 4]> = control
                    .chunks(4)
                    .map(|c| [c[0], c[1], c[2], c[3]])
                    .collect();
                let knots: Vec<f64> = case["knots"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect();
                let curve = NurbsCurve::new(3, knots, points, None, None);
                let actual: Vec<f64> = curve
                    .points(200)
                    .iter()
                    .flat_map(|p| p.to_array())
                    .collect();
                compare_f64("nurbs curve", &f64s(&case["points"]), &actual, &mut errors);
            }
            "nurbsSurface" => {
                let v = |x: f64, y: f64, z: f64, w: f64| [x, y, z, w];
                let surface = NurbsSurface {
                    degrees: [2, 3],
                    knots: [
                        vec![0., 0., 0., 1., 1., 1.],
                        vec![0., 0., 0., 0., 1., 1., 1., 1.],
                    ],
                    control_points: vec![
                        vec![
                            v(-200., -200., 100., 1.),
                            v(-200., -100., -200., 1.),
                            v(-200., 100., 250., 1.),
                            v(-200., 200., -100., 1.),
                        ],
                        vec![
                            v(0., -200., 0., 1.),
                            v(0., -100., -100., 5.),
                            v(0., 100., 150., 5.),
                            v(0., 200., 0., 1.),
                        ],
                        vec![
                            v(200., -200., -100., 1.),
                            v(200., -100., 200., 1.),
                            v(200., 100., -250., 1.),
                            v(200., 200., 100., 1.),
                        ],
                    ],
                };
                let g = ParametricGeometry::build(|a, b| surface.point(a, b), 20, 20).unwrap();
                compare("nurbs surface", &case["geometry"], &g, &mut errors, 1);
            }
            "nurbsVolume" => {
                let v = |x: f64, y: f64, z: f64, w: f64| [x, y, z, w];
                let volume = NurbsVolume {
                    degrees: [2, 3, 1],
                    knots: [
                        vec![0., 0., 0., 1., 1., 1.],
                        vec![0., 0., 0., 0., 1., 1., 1., 1.],
                        vec![0., 0., 1., 1.],
                    ],
                    control_points: vec![
                        vec![
                            vec![v(-200., -200., -200., 1.), v(-200., -200., 200., 1.)],
                            vec![v(-200., -100., -200., 1.), v(-200., -100., 200., 1.)],
                            vec![v(-200., 100., -200., 1.), v(-200., 100., 200., 1.)],
                            vec![v(-200., 200., -200., 1.), v(-200., 200., 200., 1.)],
                        ],
                        vec![
                            vec![v(0., -200., -200., 1.), v(0., -200., 200., 1.)],
                            vec![v(0., -100., -200., 1.), v(0., -100., 200., 1.)],
                            vec![v(0., 100., -200., 1.), v(0., 100., 200., 1.)],
                            vec![v(0., 200., -200., 1.), v(0., 200., 200., 1.)],
                        ],
                        vec![
                            vec![v(200., -200., -200., 1.), v(200., -200., 200., 1.)],
                            vec![v(200., -100., 0., 1.), v(200., -100., 100., 1.)],
                            vec![v(200., 100., 0., 1.), v(200., 100., 100., 1.)],
                            vec![v(200., 200., 0., 1.), v(200., 200., 100., 1.)],
                        ],
                    ],
                };
                let face = case["face"].as_str().unwrap();
                let f = |a: f64, b: f64| -> Vector3 {
                    match face {
                        "front" => volume.point(a, b, 0.),
                        "middle" => volume.point(a, b, 0.5),
                        "back" => volume.point(a, b, 1.),
                        "top" => volume.point(a, 1., b),
                        _ => volume.point(0., a, b),
                    }
                };
                let g = ParametricGeometry::build(f, 20, 20).unwrap();
                compare(
                    &format!("nurbs volume {face}"),
                    &case["geometry"],
                    &g,
                    &mut errors,
                    1,
                );
            }
            "pathAt" => {
                let mut path = Path::default();
                path.move_to([-2.5, -2.5]);
                path.cubic_to([-2.5, -2.5], [-2., 0.], [0., 0.]);
                path.cubic_to([3., 0.], [3., -3.5], [3., -3.5]);
                path.cubic_to([3., -5.5], [1., -7.7], [-2.5, -9.5]);
                path.cubic_to([-6., -7.7], [-8., -5.5], [-8., -3.5]);
                path.cubic_to([-8., -3.5], [-8., 0.], [-5., 0.]);
                path.cubic_to([-3.5, 0.], [-2.5, -2.5], [-2.5, -2.5]);
                let actual: Vec<[f64; 2]> = (0..1000)
                    .map(|i| path.point_at(i as f64 / 1000.).unwrap())
                    .collect();
                compare_f64(
                    "path getPointAt",
                    &f64s(&case["points"]),
                    &flat(&actual),
                    &mut errors,
                );
            }
            "wireframe" => {
                let g = three_rs_wasm::geometry::WireframeGeometry::build(
                    &three_rs_wasm::geometry::IcosahedronGeometry::build(20., 1).unwrap(),
                )
                .unwrap();
                compare("wireframe", &case["geometry"], &g, &mut errors, 0);
            }
            "wireframeIndexed" => {
                let g = three_rs_wasm::geometry::WireframeGeometry::build(
                    &three_rs_wasm::geometry::BoxGeometry::segmented(1., 2., 3., 2, 2, 2).unwrap(),
                )
                .unwrap();
                compare("wireframe indexed", &case["geometry"], &g, &mut errors, 0);
            }
            "mergeVertices" => {
                let mut d = DodecahedronGeometry::build(10., 0).unwrap();
                d.delete_attribute("normal");
                d.delete_attribute("uv");
                let g = three_rs_wasm::geometry_utils::merge_vertices(&d, 1e-4).unwrap();
                compare("mergeVertices", &case["geometry"], &g, &mut errors, 0);
            }
            _ => {
                let a: Vec<f64> = case["args"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect();
                let g = RoundedBoxGeometry::build(a[0], a[1], a[2], a[3] as u32, a[4]).unwrap();
                compare(
                    &format!("roundedBox {a:?}"),
                    &case["geometry"],
                    &g,
                    &mut errors,
                    1,
                );
            }
        }
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}
