#!/usr/bin/env python3
"""Fail if an M2 feature has no existing, executed test mapping."""
import json
from pathlib import Path
root=Path(__file__).resolve().parents[2]
features=set(json.loads((root/'tests/gltf-pbr/manifest.json').read_text())['features'])
coverage=json.loads((root/'tests/gltf-pbr/coverage.json').read_text())
if set(coverage)!=features: raise SystemExit('M2 feature coverage keys do not match frozen manifest')
for feature,(path,name) in coverage.items():
    if name not in (root/path).read_text(): raise SystemExit(f'Missing test for {feature}: {path} / {name}')
print(f'M2 coverage: {len(features)} features mapped')
