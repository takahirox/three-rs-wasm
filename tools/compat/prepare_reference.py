#!/usr/bin/env python3
"""Fetch only the pinned upstream revision, safely extracting source files."""
import io
import json
import tarfile
import urllib.request
from pathlib import Path, PurePosixPath

ROOT=Path(__file__).resolve().parents[2]
baseline=json.loads((ROOT/'compat/three-r187/api.json').read_text())['baseline']
commit=baseline['upstream_commit']
destination=ROOT/'.cache/three-r187'
stamp=destination/'.revision'
# The build the examples load, whose module order tools/tsl/module_order.py follows.
extras=('examples/jsm/loaders/UltraHDRLoader.js','examples/jsm/loaders/GLTFLoader.js','examples/jsm/utils/BufferGeometryUtils.js','examples/jsm/utils/SkeletonUtils.js','build/three.core.js','build/three.tsl.js','build/three.webgpu.js')
# The addons and the example assets the loader and geometry oracles
# ( tools/core/*-oracle.mjs ) run on; the MaterialX page's local documents and
# the textures they read, for its reference ( tests/browser/compute-examples.spec.js ).
prefixes=('examples/jsm/','examples/fonts/','examples/models/fbx/','examples/models/vrml/','examples/models/usdz/','examples/materialx/')
materialx_textures=tuple(f'examples/textures/{name}' for name in ('memorial.exr','memorial.hdr','tiff/crate_uncompressed.tif','brick_diffuse.jpg','brick_roughness.jpg','brick_bump.jpg','equirectangular/san_giuseppe_bridge_2k.hdr'))+('examples/models/gltf/ShaderBall.glb',)
extras+=materialx_textures
markers=('examples/jsm/loaders/VRMLLoader.js','examples/models/vrml/house.wrl','examples/models/usdz/saeukkang.usdz','examples/materialx/displacement.mtlx')
if stamp.exists() and stamp.read_text().strip()==commit and all((destination/p).exists() for p in (*extras,*markers)):
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
            if relative.parts[0]!='src' and str(relative) not in ('LICENSE','package.json',*extras) and not str(relative).startswith(prefixes):
                continue
            target=destination/str(relative)
            target.parent.mkdir(parents=True,exist_ok=True)
            target.write_bytes(tar.extractfile(entry).read())
    stamp.write_text(commit+'\n')
    print('Prepared Three.js reference:',commit)
