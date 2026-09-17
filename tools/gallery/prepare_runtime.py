#!/usr/bin/env python3
"""Prepare pinned example code/assets locally for the runtime port probes."""
import tarfile
from pathlib import PurePosixPath
from build import ARCHIVE, ROOT
count=total=0
with tarfile.open(ARCHIVE) as archive:
 for entry in archive:
  path=entry.name.split('/',1)[-1]
  if not entry.isfile() or not path.startswith('examples/') or path.startswith('examples/screenshots/'):continue
  relative=PurePosixPath(path)
  if relative.is_absolute() or '..' in relative.parts:raise ValueError('unsafe archive path')
  target=ROOT/'.cache/three-r186'/path;target.parent.mkdir(parents=True,exist_ok=True)
  if not target.exists() or target.stat().st_size!=entry.size:target.write_bytes(archive.extractfile(entry).read())
  count+=1;total+=entry.size
print(f'Prepared {count} pinned reference files ({total//1024//1024} MiB)')
