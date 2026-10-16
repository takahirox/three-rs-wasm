//! The VRMLLoader port against the pinned r186 VRMLLoader: every VRML asset
//! webgl_loader_vrml loads, compared in the canonical form with full buffers
//! ( tools/core/vrml-oracle.mjs ).
use serde_json::Value;
use three_rs_wasm::vrml::{self, Options};

const ASSETS: &[&str] = &[
    "creaseAngle",
    "crystal",
    "house",
    "elevationGrid1",
    "elevationGrid2",
    "extrusion1",
    "extrusion2",
    "extrusion3",
    "lines",
    "linesTransparent",
    "meshWithLines",
    "meshWithTexture",
    "pixelTexture",
    "points",
    "camera",
    "multilineString",
];

fn oracle() {
    if std::path::Path::new(".cache/vrml-oracle/multilineString.json").exists() {
        return;
    }
    let status = std::process::Command::new("node")
        .arg("tools/core/vrml-oracle.mjs")
        .status()
        .expect("node runs the VRMLLoader oracle");
    assert!(status.success(), "the VRMLLoader oracle failed");
}

/// A canonical number: its double's bits in hexadecimal.
fn number(v: &str) -> Option<f64> {
    if v == "NaN" {
        return Some(f64::NAN);
    }
    (v.len() == 16)
        .then(|| u64::from_str_radix(v, 16).ok().map(f64::from_bits))
        .flatten()
}

/// Natively, `Math.sin` / `Math.cos` / `Math.acos` differ from the host's in
/// the last bit for some inputs, so numbers match to a relative 1e-12, or to
/// a Float32 ulp for Float32 values ( the browser test,
/// tests/browser/vrml-loader.spec.js, compares bit for bit ).
fn compare(path: &str, e: &Value, a: &Value, errors: &mut Vec<String>) {
    if errors.len() > 40 {
        return;
    }
    match (e, a) {
        (Value::Array(e), Value::Array(a)) => {
            if e.len() != a.len() {
                errors.push(format!("{path}: length {} != {}", a.len(), e.len()));
                return;
            }
            for (i, (e, a)) in e.iter().zip(a).enumerate() {
                compare(&format!("{path}[{i}]"), e, a, errors);
            }
        }
        (Value::Object(e), Value::Object(a)) => {
            for (k, e) in e {
                compare(&format!("{path}.{k}"), e, &a[k], errors);
            }
        }
        (Value::String(es), Value::String(as_)) if es != as_ => {
            let ok = match (number(es), number(as_)) {
                (Some(x), Some(y)) => {
                    (x.is_nan() && y.is_nan())
                        || (x - y).abs() <= 1e-12 * x.abs().max(1.0)
                        || (f64::from(x as f32) == x
                            && f64::from(y as f32) == y
                            && ((x as f32).to_bits() as i64 - (y as f32).to_bits() as i64).abs()
                                <= 1)
                }
                _ => false,
            };
            if !ok {
                errors.push(format!("{path}: {as_} != {es}"));
            }
        }
        _ if e != a => errors.push(format!("{path}: {a} != {e}")),
        _ => {}
    }
}

#[test]
fn vrml_loader_matches_the_original() {
    oracle();
    let mut failures = vec![];
    for asset in ASSETS {
        let text = std::fs::read_to_string(format!(
            ".cache/three-r186/examples/models/vrml/{asset}.wrl"
        ))
        .unwrap();
        let scene = vrml::parse(
            &text,
            &Options {
                path: "models/vrml/".into(),
                window_size: (1.0, 1.0),
            },
        )
        .unwrap_or_else(|e| panic!("{asset}: {e}"));
        let actual: Value = serde_json::from_str(&vrml::canonical(&scene, true)).unwrap();
        let expected: Value = serde_json::from_str(
            &std::fs::read_to_string(format!(".cache/vrml-oracle/{asset}.json")).unwrap(),
        )
        .unwrap();
        let mut errors = vec![];
        compare(asset, &expected, &actual, &mut errors);
        failures.extend(errors);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
