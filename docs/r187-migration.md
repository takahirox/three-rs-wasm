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

- WebGL's r187 PMREM ( cube targets ) in the engine ( `src/environment_gpu.rs`,
  `src/shaders/pmrem.wgsl` ) for every WebGL port and the TSL environments;
  envmaps_fasthdr loads r187's prefiltered KTX2 cubes; a sharp background
  uploads only its texture.
- Core inventory: `make compat-check` passes on r187's members ( 394, all
  accounted for ).
- MaterialX: `three_tsl::materialx_page` follows the r187 page and builds its
  shaders with three's modules in the build's order ( `order::init_bundled`,
  `tools/tsl/module_order.py` ), as Fn layouts are named by a counter;
  `tsl/tests/materialx/` holds the page's 182 shaders ( UV plane and back
  sides included ). The gallery port draws the new documents, the UV plane and
  its selector, r187's two transmission copies ( back and front side ) and
  its cube PMREM ( `loader_materialx/pmrem.rs` ); it matches the page in
  `tests/browser/compute-examples.spec.js` ( no pixel over the bound ).

## To do

1. **PMREM and generated WGSL.** r187's PMREM is shared
   ( `gallery/src/browser/pmrem.rs`: fromEquirectangular, fromCubemap,
   fromScene, RoomEnvironment ). `tools/compat/rebase_wgsl.py` moved the
   shaders whose interface stayed; ports moved by hand since: loader_materialx,
   cubemap_dynamic, custom_fog, tsl_graph, generator_building, geometry_loft,
   water, compute_water, caustics, godrays, compute_reduce, shadow_array,
   volume_lighting, volume_caustics, volume_fire, vxgi, vxgi_sponza, fog,
   test_memory, sss, lightprobes_complex, shadowmap_progressive, retargeting,
   retargeting_readyplayer, gaussian_splat, ssgi, volume_traa,
   volume_rectarea, lights_projector, instancing_morph, shadowmap_opacity,
   lights_dynamic, shadowmap_vsm, shadowmap_csm, backdrop_water,
   fog_scattering, deferred, compute_cloth, fluid, generator_city,
   lightprobes ( its bake shared with lightprobes_complex ), retro, gtao, ssr,
   ssr_denoise, skinning_instances: every WebGPU port runs r187's shaders.
   lightprobes_sponza draws Sponza through the ported MeshletDrawingPipeline
   ( `gallery/src/browser/meshlet.rs` ), its draws, indirect draws and compute
   passes matching the page's frame by frame. Left:
   - gtao's reference starts its loading frame at a timing-dependent frameId
     ( 8 in one probe ), the port at 0.
   - A port that switches back mid-render to an output target it replaced in
     an earlier frame renders into an expired canvas texture ( browser.rs
     restores targets at a frame's start and end only; fog keeps one target ).

2. **Rapier.** The engine is rapier 0.36.0 ( rapier.js 0.21.0's, which r187's
   RapierPhysics loads ); the physics_rapier_* pages and ballpool step on it,
   ballpool matching its page at every state. Open: physics_rapier_basic's
   MSAA capture of the dragged pile ( state 5 ) at 1.30 % / 0.33 against
   1.2 % / 0.35, collider outlines and shadow edges.
3. **Changed pages.** 65 ported pages changed in r187; each port follows its
   page. Largest first: webgpu_postprocessing_ssgi_ballpool,
   webgpu_lightprobes_complex, webgl_materials_envmaps_fasthdr,
   webgpu_postprocessing_lensflare, webgpu_sandbox, webgpu_loader_materialx,
   webgpu_volume_fire, webgpu_generator_building, webgpu_lightprobes,
   misc_exporter_exr, webgpu_postprocessing_pixel,
   webgpu_compute_particles_fluid, webgpu_compute_reduce, misc_exporter_ktx2,
   webgpu_postprocessing_ssr_denoise and 47 more
   ( `git diff --numstat 148ef33e r187 -- 'examples/*.html'` ). All follow
   their pages now; the WebGL pages moved to r187's DirectionalLight shadows
   ( `Shadow::bounds`, PCF radius in texels, per-face point-shadow depth ).
   Left:
   - r187's `range()` hashes the instance index with a seed from the object's
     id instead of Math.random ( webgpu_skinning_instancing follows it );
     webgpu_instance_mesh, webgpu_particles, webgpu_particles_soft and
     webgpu_tsl_galaxy may use it too.
   - texture-volumes' combined workload test: webgpu_postprocessing_outline
     draws 22 more than its page ( 2280 ), then the run hangs loading
     css3d_periodictable.
4. **Changed addons.** Ported and on r187's oracles: USD, SVGLoader,
   IESLoader ( and IESSpotLightNode ), PLYLoader, CCDIKSolver, Octree,
   BufferGeometryUtils, the PLY / OBJ / EXR exporters, GCode, PCD, PDB, STL and
   TGA loaders, LatheGeometry's last normal. Left: GLTFLoader's
   KHR_materials_diffuse_roughness ( MeshPhysicalMaterial.diffuseRoughness
   and its BRDF; r187's new webgpu_loader_gltf_diffuse_roughness ). KTX2Loader
   cube maps and the background-only PMREM came with the engine's PMREM.
5. **Runtime results.** `docs/gallery-runtime-results.*` record the r186
   pages' runtime outcomes ( 607 examples ); r187 lists 621. Rerun
   `tools/gallery/runtime/run.mjs` on r187 and update `report.py`'s count;
   until then `report.py --check` fails.
6. **Text.** Limitations, docs and `web/THIRD_PARTY.md` name r187. The
   r186 mentions left are the jsDelivr asset pin ( r187's commit is refused ),
   byte-identical fixtures named after r186, contrasts between the releases
   and historical records.
7. **Found while moving the text.**
   - Still on r186 captures: linked_particles' 18 shaders,
     `src/shaders/light_probe_grid/`, the 3D tiles GLSL; leftover r186
     files no port loads ( pmrem_cube_uv, volume_caustics' and
     volume_lighting's `blur*_fs` and `teapot_*` ).
   - TAAU: r187's VelocityNode fills `currentProjectionMatrix` each frame
     and TAAU overrides before the pipeline, so its current clip is
     unjittered; `tsl_procedural/taau.rs` and
     `tools/tsl/check-taau-velocity.mjs` keep r186's jittered velocity.
   - webgl_raycaster_bvh: r187's `intersectsFrustum` returns true, so the
     page draws BVHHelper; the port does not.
   - WebXR: r187 removed `setProjectionFromUnion` ( cameraXR copies the left
     view's projection ); `src/xr.rs` builds r186's union camera.
   - `web/examples/ui/ui.js` loads `three@0.186.0` ( the Inspector ) and
     `ui/three/constants.js` is r186's.
   - Not re-checked on r187: webgpu_water's resize validation error,
     WebGLNodesHandler's InstancedMesh rebuild.
