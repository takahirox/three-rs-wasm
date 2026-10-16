//! The USDLoader port against the pinned r186 USDLoader: webgl_loader_usdz's
//! saeukkang.usdz, the fixtures of tools/core/usd-fixtures.mjs and, on
//! macOS, the system's own USDC files, compared in the canonical form with
//! full buffers ( tools/core/usd-oracle.mjs ).
use serde_json::Value;
use three_rs_wasm::usd;

const INPUTS: &[&str] = &[
    "transforms.usda",
    "skel.usda",
    "mesh.usdz",
    "scene.usdz",
    "saeukkang.usdz",
    "bishop.usdc",
    "king.usdc",
    "mesh.usdc",
];

fn oracle() {
    if std::path::Path::new(".cache/usd-oracle/saeukkang.usdz.json").exists() {
        return;
    }
    let status = std::process::Command::new("node")
        .arg("tools/core/usd-oracle.mjs")
        .status()
        .expect("node runs the USDLoader oracle");
    assert!(status.success(), "the USDLoader oracle failed");
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
fn usd_loader_matches_the_original() {
    oracle();
    let mut failures = vec![];
    for input in INPUTS {
        let Ok(bytes) = std::fs::read(format!(".cache/usd-fixtures/{input}")) else {
            // The macOS files are only on macOS.
            continue;
        };
        let scene = usd::parse(&bytes, "").unwrap_or_else(|e| panic!("{input}: {e}"));
        let actual: Value = serde_json::from_str(&usd::canonical(&scene, true)).unwrap();
        let expected: Value = serde_json::from_str(
            &std::fs::read_to_string(format!(".cache/usd-oracle/{input}.json")).unwrap(),
        )
        .unwrap();
        let mut errors = vec![];
        compare(input, &expected, &actual, &mut errors);
        failures.extend(errors);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
