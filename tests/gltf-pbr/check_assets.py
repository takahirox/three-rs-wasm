#!/usr/bin/env python3
"""Validate the frozen source assets and the documented HDR derivative."""
import hashlib
import json
from pathlib import Path
root=Path(__file__).resolve().parents[2]
manifest=json.loads((root/'tests/gltf-pbr/manifest.json').read_text())
provenance=json.loads((root/'web/environments.json').read_text())
assets=dict(manifest['sha256'])
assets[provenance['source']]=provenance['source_sha256']
assets[provenance['derived']]=provenance['derived_sha256']
for name,digest in assets.items():
    actual=hashlib.sha256((root/name).read_bytes()).hexdigest()
    if actual!=digest: raise SystemExit(f'Asset digest mismatch: {name}')
print(f'M2 assets: {len(assets)} hashes verified')
