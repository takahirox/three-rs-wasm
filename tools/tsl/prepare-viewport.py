"""Copy original r186 assets used by viewport TSL examples, without conversions."""
from pathlib import Path
import tarfile
ROOT=Path(__file__).resolve().parents[2]
paths=['models/gltf/Michelle.glb','models/gltf/LittlestTokyo.glb','models/ply/binary/Lucy100k.ply','textures/opengameart/smoke1.png','textures/floors/FloorsCheckerboard_S_Normal.jpg']
with tarfile.open(ROOT/'.cache/three-148ef33ecb6d2502ff796d4554abd1549c95d519.tar.gz') as tar:
 for member in tar:
  name=member.name.split('/',1)[-1]
  if member.isfile() and name in ['examples/'+p for p in paths]:
   out=ROOT/'web/gallery/assets/tsl-viewport'/name.removeprefix('examples/')
   out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(tar.extractfile(member).read())
