#!/usr/bin/env python3
"""Per-example port prerequisites. This is source review, never a runtime-pass claim."""
import json
import re
import sys
from collections import Counter
from pathlib import Path
from build import ROOT, REV

# Explicit translations present in this library. A name match alone is insufficient.
CORE = set('Scene PerspectiveCamera OrthographicCamera Mesh Color PlaneGeometry BoxGeometry SphereGeometry Vector3 Vector2 Vector4 WebGLRenderer WebGPURenderer DirectionalLight AmbientLight PointLight MeshBasicMaterial MeshStandardMaterial TextureLoader Timer BufferGeometry Group Raycaster Matrix4 Object3D Float32BufferAttribute BufferAttribute Line LineBasicMaterial Points PointsMaterial Box3 Quaternion Euler Plane Matrix3 LineSegments Sphere Layers InstancedBufferGeometry InstancedBufferAttribute InterleavedBuffer InterleavedBufferAttribute Frustum Ray Int16BufferAttribute Uint8BufferAttribute'.split())
CORE.update('CircleGeometry RingGeometry TorusGeometry TorusKnotGeometry LatheGeometry CylinderGeometry ParametricGeometry CapsuleGeometry PolyhedronGeometry IcosahedronGeometry OctahedronGeometry TetrahedronGeometry CatmullRomCurve3'.split())
CORE.update(f'{kind}BufferAttribute' for kind in ('Int8','Uint8','Uint8Clamped','Int16','Uint16','Int32','Uint32','Float16','Float32','Float64'))
# These have partial ports or have no bearing on the rendered scene. Controls/UI
# limitations are still retained in catalog.json; this is not an API parity list.
ADDONS = {'OrbitControls','Stats','GUI','Inspector','GLTFLoader','UltraHDRLoader'}
LOCAL = {
 'animation-skinning':'src/gltf.rs', 'morph-targets':'src/gltf.rs',
 'gpu-compute':'src/renderer.rs','programmable-shading':'src/shader.wgsl',
 'shadows':'src/renderer.rs','postprocessing':'src/renderer.rs',
 'physical-materials':'src/material.rs','advanced-lights':'src/scene.rs',
 'asset-loaders':'src/gltf.rs','procedural-geometry':'src/geometry.rs',
 'dynamic-textures':'src/material.rs','custom-blending':'src/renderer.rs',
}
SPECIAL = {
 'webgl_materials_normalmap_object_space': ('ObjectSpaceNormalMap','Only tangent-space normal maps are implemented in src/shader.wgsl.'),
 'webgl_test_wide_gamut': ('DisplayP3ColorSpace','Working/output color spaces are limited to linear/sRGB; Display P3 is missing.'),
 'webgl_worker_offscreencanvas': ('new Worker','BrowserApp requires Window, Document and an HtmlCanvasElement; worker/offscreen rendering is missing.'),
 'webgl_loader_texture_hdr': ('ReinhardToneMapping','Only no-tone-mapping and ACES presentation are implemented; float HDR material textures and Reinhard are missing.'),
}

def local_for(symbol):
 if 'Material' in symbol or 'Texture' in symbol:return 'src/material.rs'
 if 'Curve' in symbol:return 'src/curve.rs'
 if 'Geometry' in symbol:return 'src/geometry.rs'
 if 'Light' in symbol or 'Helper' in symbol:return 'src/scene.rs'
 if 'Loader' in symbol or 'Exporter' in symbol:return 'src/gltf.rs'
 return 'src/lib.rs'

catalog=json.loads((ROOT/'web/gallery/catalog.json').read_text())
assets=json.loads((ROOT/'docs/gallery-gltf-attempts.json').read_text())
rows=[]
for entry in catalog['examples']:
 name=entry['id'];source=(ROOT/'.cache/three-r186/examples'/f'{name}.html').read_text()
 lines=source.splitlines();blockers=[]
 def add(kind,symbol,line,reason,local):
  if any(b['kind']==kind and b['symbol']==symbol for b in blockers):return
  blockers.append({'kind':kind,'symbol':symbol,'upstream_line':line,'reason':reason,'local':local})
 imported={}
 for match in re.finditer(r"import\s+\{([^}]+)\}\s+from\s+['\"](three/addons/[^'\"]+)['\"]",source):
  for symbol in match[1].split(','):imported[symbol.strip().split(' as ')[-1]]=match[2]
 for match in re.finditer(r"import\s+([A-Z]\w*)\s+from\s+['\"](three/addons/[^'\"]+)['\"]",source): imported[match[1]]=match[2]
 for number,line in enumerate(lines,1):
  if line.lstrip().startswith('//'):continue
  for symbol in re.findall(r'new\s+THREE\.([A-Z]\w*)\s*\(',line):
   if symbol not in CORE:
    add('core-api',symbol,number,f'No Rust implementation for {symbol}; the scene needs this constructor.',local_for(symbol))
  for symbol in re.findall(r'new\s+([A-Z]\w*)\s*\(',line):
   if symbol in imported and symbol not in ADDONS:
    add('addon-api',symbol,number,f'No Rust port of {imported[symbol]}.',local_for(symbol))
 if name in SPECIAL:
  token,reason=SPECIAL[name];line=next(i for i,s in enumerate(lines,1) if token in s)
  add('behavior',token,line,reason, 'src/browser.rs' if 'worker' in name else 'src/material.rs')
 used_assets=[]
 # Match quoted filenames and full relative paths, not arbitrary name fragments.
 literals=set(re.findall(r"['\"]([^'\"\n]+\.(?:glb|gltf))['\"]",source))
 for asset in assets:
  if not any(asset['asset']==p or asset['asset'].endswith('/'+p) or p.endswith('/'+asset['asset']) for p in literals):continue
  used_assets.append(asset['asset'])
  if asset['result']['status']=='rejected':
   add('import-rejected',asset['asset'],None,asset['result']['error'][:350], 'src/gltf.rs')
  for extension in asset.get('extensions_used') or []:
   add('asset-extension',extension,None,f'{asset["asset"]} uses {extension}; successful fallback import does not implement this extension.', 'src/gltf.rs')
  if asset.get('animations'):add('asset-animation',asset['asset'],None,'Static importer does not play the animation clips in this asset.','src/gltf.rs')
 # Shader functions, shadows and other property-driven features need more than
 # constructor inventory. Keep these as review candidates, not proven blockers.
 review=[{'feature':k,'local':LOCAL.get(k,'src/lib.rs'),'evidence':v} for k,v in entry['requirements'].items() if k not in ('camera-controls','inspector')]
 if not blockers and not entry['port'] and review:
  for feature in review:
   if feature['feature'] in ('programmable-shading','postprocessing','shadows'):
    item=feature['evidence'][0]
    add('rendering-prerequisite',feature['feature'],item['line'],f'The current fixed render pipeline has no {feature["feature"]} API; this effect needs a dedicated Rust/WGSL implementation.',feature['local'])
 state='excluded' if entry['status']=='excluded' else 'runnable-partial' if entry['port'] else 'blocked-at-port-review' if blockers else 'requires-feature-review' if review else 'needs-implementation'
 rows.append({'id':name,'state':state,'source':entry['source'],'source_sha256':entry['source_sha256'],'runtime_test':entry['port']['test'] if entry['port'] else None,'blockers':blockers,'feature_review':review,'import_attempts':used_assets})
report={'revision':REV,'method':'Historical baseline before Core expansion; missing-feature labels below are not current capability claims. See core-expansion.md. All upstream example sources checked against explicit Rust constructor/addon translations; 49 real upstream glTF assets exercised by the native importer. Only runtime_test entries have browser ports. Review is not execution.','counts':dict(Counter(r['state'] for r in rows)),'examples':rows}
def output(path,payload):
 if '--check' in sys.argv:
  if not path.exists() or path.read_text()!=payload:raise SystemExit(f'Audit drift: {path.name}; run tools/gallery/audit.py')
 else:path.write_text(payload)
output(ROOT/'docs/gallery-port-attempts.json',json.dumps(report,ensure_ascii=False,indent=2)+'\n')
lines=['# Per-example port review and execution evidence','','**Historical baseline before Core expansion.** See [current Core status](core-expansion.md).','','Every upstream ID is listed, including the two excluded WebGL-only examples. **Port review is not a browser execution attempt.** This source-level review identifies missing API/addon/asset prerequisites; the separate [all-example runtime run](gallery-runtime-results.md) records actual browser execution and native captured-frame attempts. Only runnable rows have implemented browser scenes.','','The separate [glTF importer run](gallery-gltf-attempts.json) actually loads all 49 bundled glTF/GLB assets: 15 static imports succeed and 34 reject. Optional unsupported material extensions and animation still prevent fidelity even when static import succeeds.','','[Full source-line evidence and prerequisites](gallery-port-attempts.json). Image differences for new ports are recorded by `gallery-scenes.spec.js`; interactive line/point images do not yet match native WebGL rasterization. Their full vertex/color/index arrays and world transforms are compared independently.','','| Example | Stage reached | Missing prerequisite / validation |','| --- | --- | --- |']
for row in rows:
 evidence=row['runtime_test'] or '; '.join(b['symbol'] for b in row['blockers'][:4]) or '; '.join(r['feature'] for r in row['feature_review']) or 'Explicit WebGL API'
 lines.append(f'| [{row["id"]}]({row["source"]}) | {row["state"]} | {evidence} |')
output(ROOT/'docs/gallery-port-attempts.md','\n'.join(lines)+'\n')
print(json.dumps(report['counts']))
for row in rows:
 if row['state']=='needs-implementation':print('IMPLEMENT:',row['id'])
