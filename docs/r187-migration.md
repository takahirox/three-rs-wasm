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
   - gtao, ssgi, ssgi_ballpool and sss follow r187's frame numbering ( the
     loop `render()` starts counts frame 1; `fixtureStopLoop` ).
   Output targets an example may present ( `Example::on_output_targets_mut` )
   each get their texture back around every frame, so switching back
   mid-render no longer renders into an expired canvas texture.

2. **Rapier.** The engine is rapier 0.36.0 ( rapier.js 0.21.0's, which r187's
   RapierPhysics loads ); the physics_rapier_* pages and ballpool step on it,
   ballpool matching its page at every state. physics_rapier_basic passes
   its bounds ( MSAA states 5 / 6 at 1.06 % / 1.18 % against 1.2 % ) by
   drawing in WebGLRenderer's order ( material id before depth ) with
   `gl_FragCoord`'s shadow noise, behind the opt-in `Renderer::webgl`; the
   other WebGL ports still sort by depth and take top-left noise.
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
   r187's `range()` ( an instance hash seeded by the object's id ) runs in
   the shaders of skinning_instancing, instance_mesh, particles,
   particles_soft and tsl_galaxy ( `tsl::range` ). texture-volumes' combined
   workload test passes: a geometry's rewritten attributes upload alone
   ( second vertex buffer ), so the ammo soft bodies stream exactly their
   pages' bytes; the ammo pages step with Timer's signed delta; the
   offscreen-canvas and CSS fixtures render once per fixture frame. Left:
   - The reversed and logarithmic depth buffer references render without
     the pages' styles ( white background, 8 px offset ).
   - misc_controls_arcball's draw count differed once in a full run.
4. **Changed addons.** Ported and on r187's oracles: USD, SVGLoader,
   IESLoader ( and IESSpotLightNode ), PLYLoader, CCDIKSolver, Octree,
   BufferGeometryUtils, the PLY / OBJ / EXR exporters, GCode, PCD, PDB, STL and
   TGA loaders, LatheGeometry's last normal. GLTFLoader's
   KHR_materials_diffuse_roughness ( diffuseRoughness and BRDF_EON in the
   engine ) with r187's new webgpu_loader_gltf_diffuse_roughness. KTX2Loader
   cube maps and the background-only PMREM came with the engine's PMREM.
5. **Runtime results.** `docs/gallery-runtime-results.*` record r187's 621
   examples ( 578 run: 36 captured scenes, 32 rendered by the Rust probe ).
6. **Text.** Limitations, docs and `web/THIRD_PARTY.md` name r187. The
   r186 mentions left are the jsDelivr asset pin ( r187's commit is refused ),
   byte-identical fixtures named after r186, contrasts between the releases
   and historical records.
7. **Found while moving the text.**
   - Done: linked_particles, the engine LightProbeGrid ( r187's rewrite )
     and the 3D tiles GLSL run r187's shaders; unused r186 shaders are gone;
     TAAU's velocity is unjittered and rebuild frames skip the jitter as
     r187's do; the sun cascades' PCF radius is in texels on both axes.
   - The hand-written WGSL that cited r186 sources ( 18 files ) was checked
     against r187: no shader math changed, so only their citations moved.
   - Done: webgl_raycaster_bvh draws r187's BVHHelper; the XR camera takes
     the left view's matrix and projection; the gallery serves r187's
     Inspector, lil-gui and stats.js ( `three@0.187.0` ); webgpu_water and
     webgl_tsl_instancing's InstancedMesh rebuild are compared on r187.
   - webgpu_water draws r187's first two frames ( the output quad alone,
     then the reflector's identity plane ); every state within 0.15 %.
   - The UI records of webgl_gpgpu_birds_gltf, webgl_loader_svg ( r187's
     three new files ), envmaps_fasthdr and dof2 come from r187's pages.
