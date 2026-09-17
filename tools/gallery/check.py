#!/usr/bin/env python3
"""Check every catalog entry against the fixed archive and actual runtime mapping."""
import hashlib
import json
import tarfile
from pathlib import Path
from build import ARCHIVE, ROOT, UI_FILES, PORTS

catalog = json.loads((ROOT/'web/gallery/catalog.json').read_text())
files = json.loads((ROOT/'web/gallery/files.json').read_text())
rows = catalog['examples']
ids = [row['id'] for row in rows]
assets={'panorama.jpg':'examples/textures/2294472375_24a3b8ef46_o.jpg','uv-grid.jpg':'examples/textures/uv_grid_opengl.jpg','spot1Lux.hdr':'examples/textures/equirectangular/spot1Lux.hdr'}
assert len(ids) == len(set(ids)), 'duplicate catalog ID'
with tarfile.open(ARCHIVE) as tar:
 sources = {}
 for entry in tar:
  path = entry.name.split('/',1)[-1]
  if entry.isfile() and (path in assets.values() or path=='examples/files.json' or path.startswith('files/') or path.startswith('examples/screenshots/') or (path.startswith('examples/') and path.endswith('.html'))):
   sources[path] = tar.extractfile(entry).read()
 original = json.loads(sources['examples/files.json'])
 assert ids == [name for group in original.values() for name in group], 'missing/reordered source examples'
 assert set(name for group in files.values() for name in group) == {r['id'] for r in rows if r['port'] and r['status']!='excluded'}
 for row in rows:
  name = row['id']
  assert hashlib.sha256(sources[f'examples/{name}.html']).hexdigest()==row['source_sha256'], name
  if row['status']=='excluded':
   assert name in {'webgl_buffergeometry_glbufferattribute','webgl_clipculldistance'}, name
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
 for name in UI_FILES: assert (ROOT/f'web/files/{name}').read_bytes()==sources[f'files/{name}'], name
 for name in ['webgl_loader_gltf_instancing','webgl_loader_gltf_compressed','webgl_loader_gltf_avif','webgl_depth_texture','webgl_loader_texture_ktx']:
  assert next(r for r in rows if r['id']==name)['status']!='excluded', 'portable examples must not be excluded'
print(f'Gallery inventory: {len(rows)} sources / {sum(r["status"]!="excluded" for r in rows)} included / {len(PORTS)} partial ports verified')
