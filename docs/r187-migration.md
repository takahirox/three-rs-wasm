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
  programs match.
- Removed examples: webgl_renderer_pathtracer, webgpu_compute_rasterizer and
  webgpu_compute_rasterizer_ibl left the gallery with their code, tests and
  docs; the generated gallery files list r187's examples.

- MaterialX: `three_tsl::materialx_page` follows the r187 page and builds its
  shaders with three's modules in the build's order ( `order::init_bundled`,
  `tools/tsl/module_order.py` ), as Fn layouts are named by a counter;
  `tsl/tests/materialx/` holds the page's 182 shaders ( UV plane and back
  sides included ). The gallery port draws the new documents, the UV plane and
  its selector, r187's two transmission copies ( back and front side ) and
  its cube PMREM ( `loader_materialx/pmrem.rs` ); it matches the page in
  `tests/browser/compute-examples.spec.js` ( no pixel over the bound ).

## To do

1. **PMREM.** r187 replaces the CubeUV atlas with cube render targets
   ( `PMREMGenerator`, `PMREMNode`, `PMREMUtils`; WebGL's `PMREMGenerator`
   too ). `gallery/src/browser/loader_materialx/pmrem.rs` already ports
   `fromEquirectangular` with r187's WGSL. The gallery's PMREM and every
   port that samples it move to the r187 layout: 19 WebGPU shader sets with `getUV`/`getFace`/`roughnessToMip`,
   and the WebGL ports' hand-written CubeUV sampling.
2. **Generated WGSL.** Every WebGPU port runs the WGSL its page generates;
   r187 changes it for every page captured so far. For each port, capture the
   r187 page ( `node tools/compat/capture_wgsl.mjs .cache/upstream r187 … ` over
   a checkout of r186 and r187 side by side ) and replace the port's shaders,
   adapting bindings where they moved.
3. **Changed pages.** 65 ported pages changed in r187; each port follows its
   page. Largest first: webgpu_postprocessing_ssgi_ballpool,
   webgpu_lightprobes_complex, webgl_materials_envmaps_fasthdr,
   webgpu_postprocessing_lensflare, webgpu_sandbox, webgpu_loader_materialx,
   webgpu_volume_fire, webgpu_generator_building, webgpu_lightprobes,
   misc_exporter_exr, webgpu_postprocessing_pixel,
   webgpu_compute_particles_fluid, webgpu_compute_reduce, misc_exporter_ktx2,
   webgpu_postprocessing_ssr_denoise and 47 more
   ( `git diff --numstat 148ef33e r187 -- 'examples/*.html'` ).
4. **Changed addons.** Ported loaders and addons that changed: GLTFLoader,
   EXRLoader, SVGLoader, USD ( USDAParser, USDComposer, USDLoader ), PLYLoader,
   KTX2Loader, IESLoader, LDrawLoader, OBJLoader, Collada, OrbitControls,
   CCDIKSolver, Octree, SkyMesh, SkyscraperGenerator, LightProbeGrid,
   BufferGeometryUtils, the exporters, the TSL display nodes. Regenerate the
   oracle fixtures ( `tools/core/*-oracle.mjs`, `*-fixtures.mjs` ) on r187 and
   fix what no longer matches.
5. **Core inventory.** `make compat-check` on r187; classify new members.
6. **Runtime results.** `docs/gallery-runtime-results.*` record the r186
   pages' runtime outcomes ( 607 examples ); r187 lists 621. Rerun
   `tools/gallery/runtime/run.mjs` on r187 and update `report.py`'s count;
   until then `report.py --check` fails.
7. **Text.** Limitations, docs and `web/THIRD_PARTY.md` name r187.
