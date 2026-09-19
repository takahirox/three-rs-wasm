#!/usr/bin/env python3
"""Extract pinned, redistributable assets and reference helpers without network."""
from pathlib import Path
import tarfile
ROOT=Path(__file__).resolve().parents[2]
COMMIT='148ef33ecb6d2502ff796d4554abd1549c95d519'
with tarfile.open(ROOT/f'.cache/three-{COMMIT}.tar.gz') as tar:
    for path in [*[f'examples/textures/transition/transition{i}.png' for i in range(1,7)], *[f'examples/jsm/tsl/display/{name}' for name in ['FXAANode.js','SSAAPassNode.js','TransitionNode.js','radialBlur.js']], 'examples/textures/planets/earth_lights_2048.png', 'examples/jsm/tsl/display/GaussianBlurNode.js', 'examples/jsm/capabilities/WebGPU.js', 'examples/jsm/tsl/display/DotScreenNode.js', 'examples/jsm/tsl/display/RGBShiftNode.js', 'examples/textures/758px-Canestra_di_frutta_(Caravaggio).jpg', 'examples/textures/crate.gif', 'examples/textures/uv_grid_opengl.jpg', 'examples/textures/2294472375_24a3b8ef46_o.jpg']:
        entry=next(m for m in tar.getmembers() if m.name.endswith('/'+path))
        data=tar.extractfile(entry).read()
        destination=ROOT/('.cache/three-r186/'+path)
        destination.parent.mkdir(parents=True,exist_ok=True)
        destination.write_bytes(data)
        if 'Canestra_di_frutta' in path:
            (ROOT/'web/gallery/assets/caravaggio.jpg').write_bytes(data)
        if '/transition/' in path:
            (ROOT/'web/gallery/assets'/Path(path).name).write_bytes(data)
        if path.endswith('earth_lights_2048.png'):
            destination=ROOT/'web/gallery/assets/earth-lights.png'
            destination.write_bytes(data)
