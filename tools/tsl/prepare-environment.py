"""Copy pinned source assets used by environment-node examples."""
from pathlib import Path
import tarfile
ROOT=Path(__file__).resolve().parents[2]
paths=['textures/equirectangular/pedestrian_overpass_1k.hdr','textures/brick_bump.jpg','textures/lava/lavatile.jpg']
paths += [f'textures/cube/pisaHDR/{f}.hdr' for f in ['px','nx','py','ny','pz','nz']]
paths += [f'textures/cube/MilkyWay/dark-s_{f}.jpg' for f in ['px','nx','py','ny','pz','nz']]
with tarfile.open(ROOT/'.cache/three-148ef33ecb6d2502ff796d4554abd1549c95d519.tar.gz') as tar:
 for member in tar:
  path=member.name.split('/',1)[-1]
  if not member.isfile() or path not in ['examples/'+p for p in paths]:continue
  target=ROOT/'web/gallery/assets/tsl-environment'/path.removeprefix('examples/')
  target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(tar.extractfile(member).read())
