"""Current evidence for the five user-selected glTF reproduction attempts."""
import json
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
read=lambda name:json.loads((ROOT/name).read_text())
catalog={r['id']:r for r in read('web/gallery/catalog.json')['examples']}
imports={r['asset']:r for r in read('docs/gltf-examples-import-attempts.json')}
renders={r['id']:r for r in read('docs/gltf-examples-render-attempts.json')}
targets={
 'webgpu_loader_gltf_iridescence':'IridescenceLamp.glb',
 'webgpu_loader_gltf_anisotropy':'AnisotropyBarnLamp.glb',
 'webgpu_loader_gltf_sheen':'SheenChair.glb',
 'webgpu_loader_gltf_transmission':'IridescentDishWithOlives.glb',
 'webgl_loader_gltf_instancing':'DamagedHelmet/glTF-instancing/DamagedHelmetGpuInstancing.gltf',
}
notes={
 'webgpu_loader_gltf_anisotropy':'Environment reflection still uses an isotropic roughness direction; direct-light anisotropy alone does not reproduce anisotropic IBL. Clearcoat/transmission also require matched-image validation.',
 'webgpu_loader_gltf_sheen':'SheenChair default material/HDR render differs from the original. Image fidelity and the fabric sheen control remain incomplete; successful import is insufficient.',
 'webgpu_loader_gltf_transmission':'Rough transmission/refraction sampling differs from the original. The real clip imports, but full animated/interactive image acceptance remains pending.',
 'webgl_loader_gltf_instancing':'EXT_mesh_gpu_instancing imports and executes on the GPU, but PBR/environment rendering differs from the original. This is a fidelity blocker, not a missing GPU-instancing API.',
}
rows=[]
for name,asset in targets.items():
 e=catalog[name];r=renders[name]
 row={'id':name,'stage':'validated-partial-gallery' if e.get('port') else 'render-mismatch','source':e['source'],'source_sha256':e['source_sha256'],'asset':asset,'import':imports[asset]['result'],'comparison':r.get('comparison'),'render_errors':r['errors'],'steady_resource_creations':r['steady']['creates']-r['rust']['costs']['creates'],'runtime_test':e.get('port',{}).get('test') if e.get('port') else None,'images':f'.cache/gltf-examples/results/{name}/','remaining':e['port']['limitations'] if e.get('port') else [notes[name],'Image comparison exceeds 0.5% differing pixels at 6/255. No tolerance was relaxed and this candidate is not listed in the gallery.']}
 rows.append(row)
report={'scope':'Five glTF-specific examples selected after the user narrowed the task from all 22: WebGPU iridescence/anisotropy/sheen/transmission and WebGL instancing.','revision':read('compat/three-r186/api.json')['baseline']['upstream_commit'],'counts':dict(Counter(r['stage'] for r in rows)),'interpretation':'Only validated gallery entries are published. Native import/query and initial browser comparison do not establish behavioral or performance parity. No CPU per-frame or reduced-quality fallback is substituted.','examples':rows}
(ROOT/'docs/gltf-examples-attempts.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
lines=['# Five glTF example addition attempts','','The user narrowed the scope to **five glTF-specific examples**. Each one was imported and rendered against its pinned original scene. Only validated candidates enter the gallery.','','| Example | Stage | Initial raw image difference | Remaining issue |','| --- | --- | ---: | --- |']
for r in rows:lines.append(f"| {r['id']} | {r['stage']} | {r['comparison']['rawFraction']*100:.3f}% | {'; '.join(r['remaining'])} |")
lines += ['', '[Per-example evidence](gltf-examples-attempts.json), [native model import/query results](gltf-examples-import-attempts.json), [original/Rust render and allocation measurements](gltf-examples-render-attempts.json).','','The four new scene candidates are implemented in `src/browser/gltf_examples.rs`; instancing uses the existing scene. Unaccepted candidates use unpublished `.cache/gltf-examples/` assets and test-only catalog overrides. The pinned Three.js code runs only in reference pages.','','## Core fixes required by these attempts','','- Transmission preserves independent draw slots across the opaque and final passes, eliminating repeated bind-group/buffer creation.','- Physical extension textures allocate occupied layers only and retain GPU-generated mip chains with minification/trilinear sampling. Iridescence uses one 2048×2048 layer (about 21.3 MiB with mips), instead of twelve layers (192 MiB without mips). Temporary mip sources are explicitly released after the GPU copy.','- Mixed-size maps can still require padded layers, and anisotropic extension-map sampling remains unsupported. No general physical-material/performance parity claim is made.','','## Accepted Iridescence validation', '', 'Seven fixed-time/camera/resize views pass the unchanged raw RGB threshold (6/255; at most 0.5% differing pixels). The browser regression also verifies automatic rotation, pointer pause, no steady GPU allocations or geometry/texture uploads, compact mipmapped storage, and device release.', '', 'On the recorded Apple Metal / Chrome workload (512×512, 3 s warmup + 3 s sample), CPU p50/p95 is 0.20/0.30 ms for Rust and 0.30/0.40 ms for Three.js. Separate timestamp instrumentation gives GPU frame p50/p95 of 0.211/0.467 ms and 0.449/2.566 ms respectively. This is a short, single-machine measurement, not general performance parity. CPU timing counts draw callbacks and excludes non-render polling callbacks; the reference renders once per animation frame.', '', '[CPU/resource report](gltf-iridescence-performance.json), [GPU timing report](gltf-iridescence-gpu-performance.json).', '', 'Final regression checks: 73 gallery browser tests, 23 Core browser tests, two native material/render-state tests, and the packaged Pages Iridescence smoke test passed. Release Wasm build, native/Wasm clippy, formatting and gallery inventory checks passed.', '', '## Reproduction','','Run `python3 tools/gltf_examples/prepare.py`, build release Wasm, and run `node tools/gltf_examples/probe.mjs` with the local server on port 8173. Run `python3 tools/gltf_examples/report.py` to regenerate this report.','The native records come from `examples/gltf_core_probe.rs`, using the corresponding entries prepared by `tools/gallery/prepare_gltf_attempts.py`; CPU deformation there is an explicit query oracle, not the render path.']
(ROOT/'docs/gltf-examples-attempts.md').write_text('\n'.join(lines)+'\n')
print(report['counts'])
