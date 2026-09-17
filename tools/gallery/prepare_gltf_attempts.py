#!/usr/bin/env python3
"""Prepare real upstream assets for the native Rust importer, without network access."""
import base64
import hashlib
import json
import struct
import tarfile
from pathlib import Path
from urllib.parse import unquote
from build import ARCHIVE, ROOT

cache = ROOT/'.cache/gallery-gltf'
cache.mkdir(parents=True, exist_ok=True)
with tarfile.open(ARCHIVE) as archive:
 for entry in archive:
  path=entry.name.split('/',1)[-1]
  if not entry.isfile() or not path.startswith('examples/models/gltf/'): continue
  relative=Path(path.removeprefix('examples/models/gltf/'))
  if '..' in relative.parts: raise ValueError('unsafe archive path')
  target=cache/relative;target.parent.mkdir(parents=True,exist_ok=True)
  target.write_bytes(archive.extractfile(entry).read())

attempts=[]
for path in sorted(cache.rglob('*')):
 if not path.is_file() or path.suffix not in ('.glb','.gltf'):continue
 name=path.relative_to(cache).as_posix()
 entry={'asset':name,'path':str(path),'buffers':[],'images':[]}
 try:
  raw=path.read_bytes();entry["sha256"]=hashlib.sha256(raw).hexdigest();blob=None
  if path.suffix=='.glb':
   length,kind=struct.unpack_from('<II',raw,12)
   doc=json.loads(raw[20:20+length])
   offset=20+length
   if offset+8<=len(raw):
    size,kind=struct.unpack_from('<II',raw,offset);blob=raw[offset+8:offset+8+size]
  else:doc=json.loads(raw)
  def uri_data(uri):
   if uri.startswith('data:'):
    header,data=uri.split(',',1)
    return base64.b64decode(data) if ';base64' in header else unquote(data).encode()
   target=(path.parent/unquote(uri)).resolve()
   if not target.is_relative_to(cache):raise ValueError('asset URI escapes cache')
   return target.read_bytes()
  buffers=[uri_data(b['uri']) if 'uri' in b else blob for b in doc.get('buffers',[])]
  images=[]
  for image in doc.get('images',[]):
   if 'uri' in image:images.append(uri_data(image['uri']))
   else:
    view=doc['bufferViews'][image['bufferView']];start=view.get('byteOffset',0)
    images.append(buffers[view['buffer']][start:start+view['byteLength']])
  resolved=cache/'resolved'/name;resolved.mkdir(parents=True,exist_ok=True)
  for key,values in [('buffers',buffers),('images',images)]:
   for i,value in enumerate(values):
    target=resolved/f'{key}-{i}.bin';target.write_bytes(value);entry[key].append(str(target))
  entry['extensions_used']=doc.get('extensionsUsed',[])
  entry['animations']=len(doc.get('animations',[]))
  entry['skins']=len(doc.get('skins',[]))
 except Exception as error:entry['preparation_error']=str(error).replace(str(cache),'models/gltf')
 attempts.append(entry)
manifest=cache/'manifest.json';manifest.write_text(json.dumps(attempts,indent=2)+'\n')
print(f'Prepared {len(attempts)} original glTF/GLB assets')
