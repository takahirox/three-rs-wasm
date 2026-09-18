"""Prepare pinned assets for five selected glTF example reproduction attempts."""
import json, tarfile, hashlib, shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
rev=json.loads((ROOT/'compat/three-r186/api.json').read_text())['baseline']['upstream_commit']
cache=ROOT/'.cache/gltf-examples'
with tarfile.open(ROOT/'.cache'/f'three-{rev}.tar.gz') as archive:
 for entry in archive:
  rel=entry.name.split('/',1)[-1]
  if not entry.isfile():continue
  if rel in ('examples/models/gltf/IridescenceLamp.glb','examples/models/gltf/AnisotropyBarnLamp.glb','examples/models/gltf/SheenChair.glb','examples/models/gltf/IridescentDishWithOlives.glb','examples/textures/equirectangular/venice_sunset_1k.hdr','examples/textures/equirectangular/royal_esplanade_2k.hdr.jpg'):
   target=cache/rel.removeprefix('examples/');target.parent.mkdir(parents=True,exist_ok=True)
   target.write_bytes(archive.extractfile(entry).read())
print('Pinned model/environment assets ready under .cache/gltf-examples (not published)')

for entry in json.loads((ROOT/'tools/gltf_examples/assets.json').read_text()):
 source=cache/entry['source'].removeprefix('examples/')
 if hashlib.sha256(source.read_bytes()).hexdigest()!=entry['sha256']:raise ValueError('Pinned glTF asset checksum mismatch')
 target=ROOT/entry['destination'];target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,target)
print('Verified published physical glTF assets')
