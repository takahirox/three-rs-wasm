"""Run both implementations; do not maintain Rust-generated golden outputs."""
import json
import math
import subprocess


def compare(actual, expected, path=""):
    if isinstance(expected, dict):
        assert actual.keys() == expected.keys(), path
        for key in expected:
            compare(actual[key], expected[key], f"{path}.{key}")
    elif isinstance(expected, list):
        assert len(actual) == len(expected), path
        for i, (a, e) in enumerate(zip(actual, expected)):
            compare(a, e, f"{path}[{i}]")
    elif isinstance(expected,int) and any(k in path for k in ('_index','normalized_encoded')):
        assert actual==expected,(path,actual,expected)
    elif isinstance(expected,(int,float)) and not isinstance(expected,bool):
        # Float32 geometry storage rounds at each transform; math uses Float64.
        tolerance = 2e-6 if path.startswith(('.bounds[','.sphere[','.plane_positions[','.sphere_positions[')) else 1e-10
        assert math.isfinite(actual) and math.isclose(actual, expected, rel_tol=tolerance, abs_tol=tolerance), (path, actual, expected)
    else:
        assert actual == expected, (path, actual, expected)


reference = json.loads(subprocess.check_output(["node", "tests/compat/reference.mjs"]))
rust = json.loads(subprocess.check_output(["cargo", "run", "--locked", "--quiet", "--example", "compat_probe"]))
compare(rust, reference)
print(f"Behavior differential tests: PASS ({len(reference)} cases, Three.js r186)")
