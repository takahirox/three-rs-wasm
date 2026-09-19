#!/usr/bin/env python3
"""Extract pinned, redistributable assets and reference helpers without network."""
from pathlib import Path
import tarfile
ROOT=Path(__file__).resolve().parents[2]
COMMIT='148ef33ecb6d2502ff796d4554abd1549c95d519'
with tarfile.open(ROOT/f'.cache/three-{COMMIT}.tar.gz') as tar:
    for path in ['examples/textures/planets/earth_lights_2048.png', 'examples/jsm/tsl/display/GaussianBlurNode.js', 'examples/jsm/capabilities/WebGPU.js']:
        entry=next(m for m in tar.getmembers() if m.name.endswith('/'+path))
        data=tar.extractfile(entry).read()
        destination=ROOT/('.cache/three-r186/'+path)
        destination.parent.mkdir(parents=True,exist_ok=True)
        destination.write_bytes(data)
        if path.endswith('.png'):
            destination=ROOT/'web/gallery/assets/earth-lights.png'
            destination.write_bytes(data)
