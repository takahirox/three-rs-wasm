#!/usr/bin/env python3
"""Check every catalog entry against the fixed archive and actual runtime mapping."""
import hashlib
import json
import tarfile
from pathlib import Path
from build import ARCHIVE, ROOT, UI_FILES, PORTS, WEBGPU_EQUIVALENTS

catalog = json.loads((ROOT/'web/gallery/catalog.json').read_text())
files = json.loads((ROOT/'web/gallery/files.json').read_text())
rows = catalog['examples']
ids = [row['id'] for row in rows]
assets={'caravaggio.jpg':'examples/textures/758px-Canestra_di_frutta_(Caravaggio).jpg','panorama.jpg':'examples/textures/2294472375_24a3b8ef46_o.jpg','uv-grid.jpg':'examples/textures/uv_grid_opengl.jpg','spot1Lux.hdr':'examples/textures/equirectangular/spot1Lux.hdr','earth-lights.png':'examples/textures/planets/earth_lights_2048.png'}
assets.update({f'transition{i}.png':f'examples/textures/transition/transition{i}.png' for i in range(1,7)})
assets.update({'smoke1.png':'examples/textures/opengameart/smoke1.png','suzanne_buffergeometry.json':'examples/models/json/suzanne_buffergeometry.json','sprite1.png':'examples/textures/sprite1.png','snowflake1.png':'examples/textures/sprites/snowflake1.png','circle.png':'examples/textures/sprites/circle.png'})
assets.update({'hardwood2_diffuse.jpg':'examples/textures/hardwood2_diffuse.jpg','Water_1_M_Normal.jpg':'examples/textures/water/Water_1_M_Normal.jpg','roughness_map.jpg':'examples/textures/roughness_map.jpg','flames-grayscale-256x256.png':'examples/textures/noises/voronoi/grayscale-256x256.png','flames-rgb-256x256.png':'examples/textures/noises/perlin/rgb-256x256.png'})
assets.update({f'cube_m0{level}_c0{face}.jpg':f'examples/textures/cube/angus/cube_m0{level}_c0{face}.jpg' for level in range(9) for face in range(6)})
assets.update({f'castle-{face}.jpg':f'examples/textures/cube/SwedishRoyalCastle/{face}.jpg' for face in ['px','nx','py','ny','pz','nz']})
assets.update({name:f'examples/textures/planets/{name}' for name in ['earth_day_4096.jpg','earth_night_4096.jpg','earth_bump_roughness_clouds_4096.jpg']})
assets.update({'spiritedaway.ktx2':'examples/textures/spiritedaway.ktx2','blossom.png':'examples/textures/sprites/blossom.png'})
assets.update({str(path.relative_to(ROOT/'web/gallery/assets')):'examples/'+str(path.relative_to(ROOT/'web/gallery/assets/tsl-next')) for path in (ROOT/'web/gallery/assets/tsl-next').rglob('*') if path.is_file() and path.name not in ['path.json','smaa-area.png','smaa-search.png']})
assets.update({str(path.relative_to(ROOT/'web/gallery/assets')):'examples/'+str(path.relative_to(ROOT/'web/gallery/assets/tsl-environment')) for path in (ROOT/'web/gallery/assets/tsl-environment').rglob('*') if path.is_file()})
assets.update({str(path.relative_to(ROOT/'web/gallery/assets')):'examples/'+str(path.relative_to(ROOT/'web/gallery/assets/tsl-lighting')) for path in (ROOT/'web/gallery/assets/tsl-lighting').rglob('*') if path.is_file() and not path.name.endswith('.rgba16f.png') and path.name != 'hdr.json'})
assets.update({str(path.relative_to(ROOT/'web/gallery/assets')):'examples/'+str(path.relative_to(ROOT/'web/gallery/assets/tsl-viewport')) for path in (ROOT/'web/gallery/assets/tsl-viewport').rglob('*') if path.is_file()})
assets.update({str(path.relative_to(ROOT/'web/gallery/assets')):'examples/'+str(path.relative_to(ROOT/'web/gallery/assets/tsl-materials')) for path in (ROOT/'web/gallery/assets/tsl-materials').rglob('*') if path.is_file() and path.suffix not in ['.bin','.json']})
assets.update({'tsl-procedural/checker.png':'examples/textures/checker.png','tsl-procedural/webgpu-audio-processing.mp3':'examples/sounds/webgpu-audio-processing.mp3','tsl-procedural/FloorsCheckerboard_S_Normal.jpg':'examples/textures/floors/FloorsCheckerboard_S_Normal.jpg','tsl-procedural/FloorsCheckerboard_S_Diffuse.jpg':'examples/textures/floors/FloorsCheckerboard_S_Diffuse.jpg','tsl-procedural/decal-diffuse.png':'examples/textures/decal/decal-diffuse.png','tsl-procedural/decal-normal.jpg':'examples/textures/decal/decal-normal.jpg','tsl-procedural/pedestrian_overpass_1k.hdr':'examples/textures/equirectangular/pedestrian_overpass_1k.hdr','tsl-procedural/san_giuseppe_bridge_2k.hdr':'examples/textures/equirectangular/san_giuseppe_bridge_2k.hdr','tsl-procedural/gears.glb':'examples/models/gltf/gears.glb','tsl-procedural/Xbot.glb':'examples/models/gltf/Xbot.glb','tsl-procedural/uv_grid_directx.jpg':'examples/textures/uv_grid_directx.jpg'})
assets.update({'tsl-primitives/'+record['file']:'examples/'+record['source'] for record in json.loads((ROOT/'web/gallery/assets/tsl-primitives/manifest.json').read_text()) if 'source' in record})
extra_assets={'web/models/Michelle.glb':'examples/models/gltf/Michelle.glb','web/models/PrimaryIonDrive.glb':'examples/models/gltf/PrimaryIonDrive.glb','web/models/LeePerrySmith.glb':'examples/models/gltf/LeePerrySmith/LeePerrySmith.glb','web/models/LeePerrySmith_License.txt':'examples/models/gltf/LeePerrySmith/LeePerrySmith_License.txt','web/environments/moonless_golf_1k.hdr':'examples/textures/equirectangular/moonless_golf_1k.hdr'}
assert len(ids) == len(set(ids)), 'duplicate catalog ID'
with tarfile.open(ARCHIVE) as tar:
 sources = {}
 for entry in tar:
  path = entry.name.split('/',1)[-1]
  if entry.isfile() and (path in assets.values() or path in extra_assets.values() or path=='examples/files.json' or path.startswith('files/') or path.startswith('examples/screenshots/') or (path.startswith('examples/') and path.endswith('.html'))):
   sources[path] = tar.extractfile(entry).read()
 original = json.loads(sources['examples/files.json'])
 assert ids == [name for group in original.values() for name in group], 'missing/reordered source examples'
 assert set(name for group in files.values() for name in group) == {r['id'] for r in rows if r['port'] and r['status']!='excluded'}
 for row in rows:
  name = row['id']
  assert hashlib.sha256(sources[f'examples/{name}.html']).hexdigest()==row['source_sha256'], name
  if row['status']=='excluded':
   assert name in {'webgl_buffergeometry_glbufferattribute','webgl_clipculldistance'} | WEBGPU_EQUIVALENTS.keys(), name
   if name in WEBGPU_EQUIVALENTS: assert row['preferred_example']==WEBGPU_EQUIVALENTS[name]
   assert row['excluded_reason'] and not row['port']
  else:
   assert (ROOT/f'web/gallery/screenshots/{name}.jpg').read_bytes()==sources[f'examples/screenshots/{name}.jpg'], name
  if row['port']:
   assert row['port']==PORTS[name]
   assert (ROOT/row['port']['source']).is_file() and (ROOT/row['port']['test']).is_file()
   assert row['status']=='partial' and row['port']['limitations']
  else: assert row['status'] in ('excluded','not-ported')
  for evidence in row['requirements'].values():
   for item in evidence: assert sources[f'examples/{name}.html'].decode().splitlines()[item['line']-1].strip().startswith(item['text'])
 for name,upstream in assets.items():assert (ROOT/f'web/gallery/assets/{name}').read_bytes()==sources[upstream],name
 for name,upstream in extra_assets.items():assert (ROOT/name).read_bytes()==sources[upstream],name
 for name in UI_FILES: assert (ROOT/f'web/files/{name}').read_bytes()==sources[f'files/{name}'], name
 for name in ['webgl_loader_gltf_instancing','webgl_loader_gltf_compressed','webgl_loader_gltf_avif','webgl_depth_texture','webgl_loader_texture_ktx']:
  assert name in WEBGPU_EQUIVALENTS or next(r for r in rows if r['id']==name)['status']!='excluded', 'portable examples need a WebGPU equivalent to be excluded'
print(f'Gallery inventory: {len(rows)} sources / {sum(r["status"]!="excluded" for r in rows)} included / {len(PORTS)} partial ports verified')

for record in json.loads((ROOT/'web/gallery/assets/tsl-lighting/hdr.json').read_text()):
 root=ROOT/'web/gallery/assets/tsl-lighting'
 assert hashlib.sha256((root/record['source']).read_bytes()).hexdigest()==record['source_sha256']
 assert hashlib.sha256((root/record['file']).read_bytes()).hexdigest()==record['sha256']

for record in json.loads((ROOT/'web/gallery/assets/tsl-materials/geometry.json').read_text()):
 assert hashlib.sha256((ROOT/'web/gallery/assets/tsl-materials'/record['file']).read_bytes()).hexdigest()==record['sha256']
 with tarfile.open(ARCHIVE) as tar:
  entry=next(e for e in tar if e.name.split('/',1)[-1]=='examples/'+record['source'])
  assert hashlib.sha256(tar.extractfile(entry).read()).hexdigest()==record['source_sha256']

for record in json.loads((ROOT/'web/gallery/assets/tsl-primitives/manifest.json').read_text()):
 assert hashlib.sha256((ROOT/'web/gallery/assets/tsl-primitives'/record['file']).read_bytes()).hexdigest()==record['sha256']

for record in json.loads((ROOT/"web/gallery/assets/tsl-procedural/geometry.json").read_text()):
 assert hashlib.sha256((ROOT/"web/gallery/assets/tsl-procedural"/record["file"]).read_bytes()).hexdigest()==record["sha256"]
 with tarfile.open(ARCHIVE) as tar:
  entry=next(e for e in tar if e.name.split("/",1)[-1]=="examples/"+record["source"])
  assert hashlib.sha256(tar.extractfile(entry).read()).hexdigest()==record["source_sha256"]

record=json.loads((ROOT/'web/gallery/assets/tsl-procedural/reflection-tree.json').read_text())
assert hashlib.sha256((ROOT/'web/gallery/assets/tsl-procedural'/record['file']).read_bytes()).hexdigest()==record['sha256']
assert hashlib.sha256((ROOT/'.cache/three-r186'/record['source']).read_bytes()).hexdigest()==record['source_sha256']

record=json.loads((ROOT/'web/gallery/assets/tsl-procedural/snow-teapot-manifest.json').read_text())
assert hashlib.sha256((ROOT/'web/gallery/assets/tsl-procedural'/record['file']).read_bytes()).hexdigest()==record['sha256']
assert hashlib.sha256((ROOT/'.cache/three-r186'/record['source']).read_bytes()).hexdigest()==record['source_sha256']

record=json.loads((ROOT/'web/gallery/assets/tsl-procedural/retro-base-manifest.json').read_text())
assert hashlib.sha256((ROOT/'web/gallery/assets/tsl-procedural'/record['file']).read_bytes()).hexdigest()==record['sha256']
assert hashlib.sha256((ROOT/'.cache/three-r186'/record['source']).read_bytes()).hexdigest()==record['source_sha256']

for record in json.loads((ROOT/"web/gallery/assets/material-textures/manifest.json").read_text())["files"]:
 assert hashlib.sha256((ROOT/"web/gallery/assets/material-textures"/record["file"]).read_bytes()).hexdigest()==record["sha256"]

for record in json.loads((ROOT/"web/gallery/assets/shapes/manifest.json").read_text()):
 assert hashlib.sha256((ROOT/"web/gallery/assets/shapes"/record["file"]).read_bytes()).hexdigest()==record["sha256"]
 if "source_sha256" in record:
  assert hashlib.sha256((ROOT/".cache/three-r186"/record["source"]).read_bytes()).hexdigest()==record["source_sha256"]

for record in json.loads((ROOT/"web/gallery/assets/buffer-particles-manifest.json").read_text()):
 assert hashlib.sha256((ROOT/"web/gallery/assets"/record["file"]).read_bytes()).hexdigest()==record["sha256"]
 assert hashlib.sha256((ROOT/".cache/three-r186"/record["source"]).read_bytes()).hexdigest()==record["sha256"]

for folder in ['point-clouds','shader-geometry','geometry-materials','environment-materials']:
 for record in json.loads((ROOT/'web/gallery/assets'/folder/'manifest.json').read_text()):
  assert hashlib.sha256((ROOT/'web/gallery/assets'/folder/record['file']).read_bytes()).hexdigest()==record['sha256']
  expected=record.get('source_sha256',record['sha256'])
  assert hashlib.sha256((ROOT/'.cache/three-r186'/record['source']).read_bytes()).hexdigest()==expected

for record in json.loads((ROOT/"web/gallery/assets/interactive-objects-manifest.json").read_text())+json.loads((ROOT/"web/gallery/assets/interactive-scenes-manifest.json").read_text())+json.loads((ROOT/"web/gallery/assets/views-loaders-manifest.json").read_text())+json.loads((ROOT/"web/gallery/assets/stereo-loaders-manifest.json").read_text()):
 assert hashlib.sha256((ROOT/"web/gallery/assets"/record["file"]).read_bytes()).hexdigest()==record["sha256"]
 assert hashlib.sha256((ROOT/".cache/three-r186"/record["source"]).read_bytes()).hexdigest()==record["sha256"]
