#!/usr/bin/env python3
"""Fetch only the pinned upstream revision, safely extracting source files."""
import io
import json
import tarfile
import urllib.request
from pathlib import Path, PurePosixPath

ROOT=Path(__file__).resolve().parents[2]
baseline=json.loads((ROOT/'compat/three-r186/api.json').read_text())['baseline']
commit=baseline['upstream_commit']
destination=ROOT/'.cache/three-r186'
stamp=destination/'.revision'
extras=('examples/jsm/loaders/UltraHDRLoader.js','examples/jsm/loaders/GLTFLoader.js','examples/jsm/utils/BufferGeometryUtils.js','examples/jsm/utils/SkeletonUtils.js')
if stamp.exists() and stamp.read_text().strip()==commit and all((destination/p).exists() for p in extras):
    print('Three.js reference ready:',commit)
else:
    archive=ROOT/'.cache'/f'three-{commit}.tar.gz'
    archive.parent.mkdir(parents=True,exist_ok=True)
    if not archive.exists():
        request=urllib.request.Request(f'https://codeload.github.com/mrdoob/three.js/tar.gz/{commit}',headers={'User-Agent':'three-rs-wasm-tests'})
        with urllib.request.urlopen(request,timeout=120) as response:
            data=response.read()
        archive.write_bytes(data)
    with tarfile.open(archive) as tar:
        for entry in tar:
            parts=PurePosixPath(entry.name).parts
            if len(parts)<2 or parts[0]!=f'three.js-{commit}' or not entry.isfile():
                continue
            relative=PurePosixPath(*parts[1:])
            if '..' in relative.parts or relative.is_absolute():
                raise RuntimeError('unsafe archive path')
            if relative.parts[0]!='src' and str(relative) not in ('LICENSE','package.json',*extras):
                continue
            target=destination/str(relative)
            target.parent.mkdir(parents=True,exist_ok=True)
            target.write_bytes(tar.extractfile(entry).read())
    stamp.write_text(commit+'\n')
    print('Prepared Three.js reference:',commit)
