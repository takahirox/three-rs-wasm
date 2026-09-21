"""Pinned assets for path, LUT, parallax and image-filter examples."""
from pathlib import Path
import tarfile
import base64
import re
import subprocess
ROOT=Path(__file__).resolve().parents[2]
paths=['textures/brick_diffuse.jpg','textures/noises/perlin/128x128.png','textures/equirectangular/752-hdri-skies-com_1k.hdr','models/gltf/coffeeMug.glb','models/gltf/DragonAttenuation.glb']+[f'textures/ambientcg/{n}' for n in ['Ice002_1K-JPG_Color.jpg','Ice002_1K-JPG_NormalGL.jpg','Ice002_1K-JPG_Roughness.jpg','Ice002_1K-JPG_Displacement.jpg','Ice003_1K-JPG_Color.jpg']]
with tarfile.open(ROOT/'.cache/three-148ef33ecb6d2502ff796d4554abd1549c95d519.tar.gz') as tar:
 for m in tar:
  p=m.name.split('/',1)[-1]
  if not m.isfile() or not p.startswith('examples/'):continue
  rel=p.removeprefix('examples/')
  if rel not in paths and not rel.startswith('luts/') and rel != 'webgpu_instance_path.html':continue
  data=tar.extractfile(m).read()
  destinations=[ROOT/'.cache/three-r186'/p]
  if not rel.endswith('.html'):destinations.append(ROOT/'web/gallery/assets/tsl-next'/rel)
  for dest in destinations:
   dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)

# Original SMAA lookup tables are embedded PNGs, not regenerated approximations.
source=(ROOT/'.cache/three-r186/examples/jsm/tsl/display/SMAANode.js').read_text()
for name, data in zip(['area','search'], re.findall(r"data:image/png;base64,([^']+)", source), strict=True):
 (ROOT/f'web/gallery/assets/tsl-next/smaa-{name}.png').write_bytes(base64.b64decode(data))
subprocess.run(['node', str(ROOT/'tools/tsl/prepare-path.mjs')], check=True)
