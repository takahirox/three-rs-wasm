"""Prepare pinned assets for selected glTF example reproduction attempts."""
import json, tarfile, hashlib, shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
rev=json.loads((ROOT/'compat/three-r186/api.json').read_text())['baseline']['upstream_commit']
cache=ROOT/'.cache/gltf-examples'
assets=json.loads((ROOT/'tools/gltf_examples/assets.json').read_text())
asset_sources={e['source'] for e in assets} | {'examples/textures/equirectangular/royal_esplanade_2k.hdr.jpg'}
# Upstream decoder dependencies are for the independent reference only.
reference_sources={'examples/jsm/'+name for name in (
 'loaders/KTX2Loader.js','utils/WorkerPool.js','libs/ktx-parse.module.js',
 'libs/zstddec.module.js','math/ColorSpaces.js','libs/meshopt_decoder.module.js',
 'libs/basis/basis_transcoder.js','libs/basis/basis_transcoder.wasm')}

with tarfile.open(ROOT/'.cache'/f'three-{rev}.tar.gz') as archive:
 for entry in archive:
  rel=entry.name.split('/',1)[-1]
  if not entry.isfile():continue
  if rel in reference_sources:
   target=ROOT/'.cache/three-r186'/rel;target.parent.mkdir(parents=True,exist_ok=True)
   target.write_bytes(archive.extractfile(entry).read())
  if rel in asset_sources:
   target=cache/rel.removeprefix('examples/');target.parent.mkdir(parents=True,exist_ok=True)
   target.write_bytes(archive.extractfile(entry).read())
print('Pinned model/environment assets ready under .cache/gltf-examples (not published)')

for entry in assets:
 source=cache/entry['source'].removeprefix('examples/')
 if hashlib.sha256(source.read_bytes()).hexdigest()!=entry['sha256']:raise ValueError('Pinned glTF asset checksum mismatch')
 target=ROOT/entry['destination'];target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,target)
print('Verified published physical glTF assets')
