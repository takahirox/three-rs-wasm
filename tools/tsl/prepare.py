#!/usr/bin/env python3
"""Extract pinned, redistributable assets and reference helpers without network."""
from pathlib import Path
import tarfile
import zipfile
import io
import subprocess
ROOT=Path(__file__).resolve().parents[2]
COMMIT='148ef33ecb6d2502ff796d4554abd1549c95d519'
with tarfile.open(ROOT/f'.cache/three-{COMMIT}.tar.gz') as tar:
    # Keep independent reference imports and static geometry preparation usable
    # on a clean checkout, including transitive addon/decoder dependencies.
    for entry in tar:
        path=entry.name.split('/',1)[-1]
        if entry.isfile() and (path.startswith('src/') or path.startswith('examples/jsm/') or path=='package.json'):
            destination=ROOT/'.cache/three-r186'/path
            destination.parent.mkdir(parents=True,exist_ok=True)
            destination.write_bytes(tar.extractfile(entry).read())
    for path in [*[f'examples/textures/cube/angus/cube_m0{level}_c0{face}.jpg' for level in range(9) for face in range(6)],*[f'examples/textures/cube/SwedishRoyalCastle/{name}.jpg' for name in ['px','nx','py','ny','pz','nz']],*[f'examples/textures/planets/{name}' for name in ['earth_day_4096.jpg','earth_night_4096.jpg','earth_bump_roughness_clouds_4096.jpg']],'examples/textures/spiritedaway.ktx2','examples/jsm/math/ImprovedNoise.js','examples/textures/3d/head256x256x109.zip','examples/textures/sprites/blossom.png','examples/models/gltf/LeePerrySmith/LeePerrySmith_License.txt','examples/models/gltf/LeePerrySmith/LeePerrySmith.glb','examples/models/gltf/PrimaryIonDrive.glb','examples/textures/equirectangular/moonless_golf_1k.hdr','examples/textures/water/Water_1_M_Normal.jpg','examples/textures/roughness_map.jpg','examples/textures/noises/voronoi/grayscale-256x256.png','examples/textures/noises/perlin/rgb-256x256.png','examples/textures/hardwood2_diffuse.jpg','examples/models/gltf/Michelle.glb','examples/textures/opengameart/smoke1.png','examples/models/json/suzanne_buffergeometry.json','examples/textures/sprite1.png','examples/textures/sprites/snowflake1.png','examples/textures/sprites/circle.png','examples/jsm/tsl/display/AfterImageNode.js',*[f'examples/textures/transition/transition{i}.png' for i in range(1,7)], *[f'examples/jsm/tsl/display/{name}' for name in ['FXAANode.js','SSAAPassNode.js','TransitionNode.js','radialBlur.js']], 'examples/textures/planets/earth_lights_2048.png', 'examples/jsm/tsl/display/GaussianBlurNode.js', 'examples/jsm/capabilities/WebGPU.js', 'examples/jsm/tsl/display/DotScreenNode.js', 'examples/jsm/tsl/display/RGBShiftNode.js', 'examples/textures/758px-Canestra_di_frutta_(Caravaggio).jpg', 'examples/textures/crate.gif', 'examples/textures/uv_grid_opengl.jpg', 'examples/textures/2294472375_24a3b8ef46_o.jpg']:
        entry=next(m for m in tar.getmembers() if m.name.endswith('/'+path))
        data=tar.extractfile(entry).read()
        destination=ROOT/('.cache/three-r186/'+path)
        destination.parent.mkdir(parents=True,exist_ok=True)
        destination.write_bytes(data)
        if path.endswith('/head256x256x109.zip'):
            (ROOT/'web/gallery/assets/head256x256x109.raw').write_bytes(zipfile.ZipFile(io.BytesIO(data)).read('head256x256x109'))
        if path.endswith('/spiritedaway.ktx2'):
            (ROOT/'web/gallery/assets/spiritedaway.ktx2').write_bytes(data)
        if '/cube/angus/' in path:
            (ROOT/'web/gallery/assets'/Path(path).name).write_bytes(data)
        if '/SwedishRoyalCastle/' in path:
            (ROOT/'web/gallery/assets'/('castle-'+Path(path).name)).write_bytes(data)
        if '/planets/earth_' in path and path.endswith('4096.jpg'):
            (ROOT/'web/gallery/assets'/Path(path).name).write_bytes(data)
        if path.endswith('/blossom.png'):
            (ROOT/'web/gallery/assets/blossom.png').write_bytes(data)
        if path.endswith('/Water_1_M_Normal.jpg') or path.endswith('/roughness_map.jpg'):
            (ROOT/'web/gallery/assets'/Path(path).name).write_bytes(data)
        if '/noises/' in path:
            (ROOT/'web/gallery/assets'/('flames-'+Path(path).name)).write_bytes(data)
        if path.endswith('/PrimaryIonDrive.glb'):
            (ROOT/'web/models/PrimaryIonDrive.glb').write_bytes(data)
        if path.endswith('/moonless_golf_1k.hdr'):
            (ROOT/'web/environments/moonless_golf_1k.hdr').write_bytes(data)
        if path.endswith('/LeePerrySmith_License.txt'):
            (ROOT/'web/models/LeePerrySmith_License.txt').write_bytes(data)
        if path.endswith('/LeePerrySmith.glb'):
            (ROOT/'web/models/LeePerrySmith.glb').write_bytes(data)
        if path.endswith('/hardwood2_diffuse.jpg'):
            (ROOT/'web/gallery/assets/hardwood2_diffuse.jpg').write_bytes(data)
        if path.endswith('/Michelle.glb'):
            (ROOT/'web/models/Michelle.glb').write_bytes(data)
        if 'Canestra_di_frutta' in path:
            (ROOT/'web/gallery/assets/caravaggio.jpg').write_bytes(data)
        if '/transition/' in path or path in ['examples/textures/opengameart/smoke1.png','examples/models/json/suzanne_buffergeometry.json','examples/textures/sprite1.png','examples/textures/sprites/snowflake1.png','examples/textures/sprites/circle.png']:
            (ROOT/'web/gallery/assets'/Path(path).name).write_bytes(data)
        if path.endswith('earth_lights_2048.png'):
            destination=ROOT/'web/gallery/assets/earth-lights.png'
            destination.write_bytes(data)

subprocess.run(["node",str(ROOT/"tools/tsl/prepare-volume.mjs")],check=True)

subprocess.run(["node",str(ROOT/"tools/tsl/prepare-geometry.mjs")],check=True)

subprocess.run(['python3', str(ROOT/'tools/tsl/prepare-next.py')], check=True)
