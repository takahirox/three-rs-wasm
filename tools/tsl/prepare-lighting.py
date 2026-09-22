"""Pinned source assets for the PMREM/lightmap and postprocessing ports."""
from pathlib import Path
import tarfile
ROOT = Path(__file__).resolve().parents[2]
paths = ['models/gltf/bath_day.glb', 'models/gltf/space_ship_hallway.glb',
         'textures/equirectangular/royal_esplanade_2k.hdr.jpg', 'textures/equirectangular/spruit_sunrise_2k.hdr.jpg', 'textures/equirectangular/ice_planet_close.jpg']
paths += [f'textures/cube/Park3Med/{f}.jpg' for f in ['px','nx','py','ny','pz','nz']]
paths += [f'models/json/lightmap/{f}' for f in ['lightmap.json','lightmap-ao-shadow.png','rocks.jpg','stone.jpg']]
with tarfile.open(ROOT/'.cache/three-148ef33ecb6d2502ff796d4554abd1549c95d519.tar.gz') as tar:
 for member in tar:
  path=member.name.split('/',1)[-1]
  if not member.isfile() or path not in ['examples/'+p for p in paths]:continue
  target=ROOT/'web/gallery/assets/tsl-lighting'/path.removeprefix('examples/')
  target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(tar.extractfile(member).read())
