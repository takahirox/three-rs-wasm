# Moving the baseline from r186 to r187

three.js r187 ( `7e89746a` ) replaces r186 ( `148ef33e` ) as the pinned
release. Between them: 526 commits, 323 files under `src/`, 174 under
`examples/jsm/`; 17 examples added and 3 removed. This file tracks the move
until every ported example matches r187 again; it is removed when done.

## Done

- The pin: `compat/three-r187/`, `.cache/three-r187`, the tools that read the
  revision ( `tools/compat/prepare_reference.py`, `tools/tsl/prepare*.py` ).
- The node system transpiled from r187 ( `tools/tsl/transpile.mjs` ), the
  MaterialX addons in the order their imports give ( `tools/tsl/module_order.py`
  writes `.cache/tslsrc/addon_order.txt` ). The runtime gained `WeakRef`,
  `FinalizationRegistry`, `trimStart` and `trimEnd`.
- The TSL corpus: r187's goldens, its `three/tsl` exports; 4217 / 4217
  programs match ( `scripts/check-tsl` but the MaterialX test, step 2 ).
- Removed examples: webgl_renderer_pathtracer, webgpu_compute_rasterizer and
  webgpu_compute_rasterizer_ibl left the gallery with their code, tests and
  docs; the generated gallery files list r187's examples.

## To do

1. **MaterialX.** The page changed ( four more documents: EXR, TIFF, HDR
   emission, displacement; a UV plane and a geometry selector;
   `compileAsync` before `scene.add` ). `three_tsl::materialx_page` follows the
   page, then `tsl/tests/materialx/` is replaced by the r187 page's shaders
   ( `tools/compat/capture_wgsl.mjs` reproduces the r186 fixtures exactly ).
   With the r186 page order, 58 of 74 shaders already match r187; the rest
   differ only in function ids, which follow node creation order.
2. **PMREM.** r187 replaces the CubeUV atlas with cube render targets
   ( `PMREMGenerator`, `PMREMNode`, `PMREMUtils`; WebGL's `PMREMGenerator`
   too ). The gallery's PMREM and every port that samples it move to the
   r187 layout: 19 WebGPU shader sets with `getUV`/`getFace`/`roughnessToMip`,
   and the WebGL ports' hand-written CubeUV sampling.
3. **Generated WGSL.** Every WebGPU port runs the WGSL its page generates;
   r187 changes it for every page captured so far. For each port, capture the
   r187 page ( `node tools/compat/capture_wgsl.mjs .cache/upstream r187 … ` over
   a checkout of r186 and r187 side by side ) and replace the port's shaders,
   adapting bindings where they moved.
4. **Changed pages.** 65 ported pages changed in r187; each port follows its
   page. Largest first: webgpu_postprocessing_ssgi_ballpool,
   webgpu_lightprobes_complex, webgl_materials_envmaps_fasthdr,
   webgpu_postprocessing_lensflare, webgpu_sandbox, webgpu_loader_materialx,
   webgpu_volume_fire, webgpu_generator_building, webgpu_lightprobes,
   misc_exporter_exr, webgpu_postprocessing_pixel,
   webgpu_compute_particles_fluid, webgpu_compute_reduce, misc_exporter_ktx2,
   webgpu_postprocessing_ssr_denoise and 47 more
   ( `git diff --numstat 148ef33e r187 -- 'examples/*.html'` ).
5. **Changed addons.** Ported loaders and addons that changed: GLTFLoader,
   EXRLoader, SVGLoader, USD ( USDAParser, USDComposer, USDLoader ), PLYLoader,
   KTX2Loader, IESLoader, LDrawLoader, OBJLoader, Collada, OrbitControls,
   CCDIKSolver, Octree, SkyMesh, SkyscraperGenerator, LightProbeGrid,
   BufferGeometryUtils, the exporters, the TSL display nodes. Regenerate the
   oracle fixtures ( `tools/core/*-oracle.mjs`, `*-fixtures.mjs` ) on r187 and
   fix what no longer matches.
6. **Core inventory.** `make compat-check` on r187; classify new members.
7. **Text.** Limitations, docs and `web/THIRD_PARTY.md` name r187.
