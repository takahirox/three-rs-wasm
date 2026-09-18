"""Copy the pinned Forest House asset and prepare its independent Draco reference."""
import hashlib
import json
import tarfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REV = json.loads((ROOT / 'compat/three-r186/api.json').read_text())['baseline']['upstream_commit']
ASSET = 'examples/models/gltf/AVIFTest/forest_house.glb'
SHA256 = 'd7e567c978c1b266744e20f8418f29e90591e38a2bd6fdb5220c3b9eed91a137'

with tarfile.open(ROOT / '.cache' / f'three-{REV}.tar.gz') as archive:
    payload = archive.extractfile(f'three.js-{REV}/{ASSET}').read()
    if hashlib.sha256(payload).hexdigest() != SHA256:
        raise SystemExit('Forest House checksum mismatch')
    target = ROOT / 'web/models/AVIFTest/forest_house.glb'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(payload)
    for path in ('examples/jsm/loaders/DRACOLoader.js',
                 'examples/jsm/libs/draco/gltf/draco_decoder.wasm',
                 'examples/jsm/libs/draco/gltf/draco_wasm_wrapper.js'):
        target = ROOT / '.cache/three-r186' / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(archive.extractfile(f'three.js-{REV}/{path}').read())
print('Pinned AVIF model and independent Draco reference ready')
