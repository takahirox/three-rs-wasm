# Sponza examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

Both pages load Sponza from glTF-Sample-Assets
(`https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/`)
through its model index. The model's files are under the CRYENGINE Limited
License Agreement, which does not grant redistribution. The ports therefore
fetch the same URLs at run time, as the pages do, and vendor nothing. Like
the originals, they follow the repository's `main` branch.

glTF images are requested together, as the browser's loaders issue them, so
Sponza's 69 textures download in parallel (about 6 s on a local network,
against the original's 5.6 s).

| Example | Id | Port |
| --- | --- | --- |
| `webgpu_lightprobes_sponza` | 481 | `lightprobes_sponza.rs` |
| `webgpu_vxgi_sponza` | 482 | `vxgi_sponza.rs` |

## webgpu_lightprobes_sponza

Sponza is lit by a shadowed SunLight ( 0xfff2dc, intensity 100, shadow far
50, 2048² maps ) under SkyMesh with ACES, with FirstPersonControls and
diffuse GI from a 10 × 7 × 7 LightProbeGrid. The grid bakes four probes per
frame, in two passes ( one bounce ), as the page's `updateProbes()` does.

The engine's `LightProbeGrid` ( `src/light_probe_grid.rs`,
`NodeKind::LightProbeGrid` ) ports `LightProbeGrid.js`, `LightProbeGridNode`
and `LightProbeGridHelper`:

- **Bake.** `light_probe_grid::bake( baker, renderer, scene, grid, options )`
  follows `LightProbeGrid.bake()`:
  - a CubeCamera capture per probe, with the grid hidden and each
    shadow-casting SunLight replaced by a directional bake light whose
    shadow covers the casters' bounding sphere;
  - each shadow map rendered once per bake;
  - the 512-direction Fibonacci SH projection into a 9 × N batch row;
  - the seven repack passes into the padded RGBA16F 3D atlas;
  - indirect passes ( `pass`, `bounces` ) lit by a snapshot of the previous
    pass's atlas.

  The SH projection and repack shaders are the WGSL r186 generates. The
  capture cube, batch target, bind groups, repack uniforms and bake camera
  are pooled, so a frame of baking creates no GPU objects.
- **Lighting.** Lit materials add the grid's irradiance to their indirect
  diffuse, as LightProbeGridNode does:
  - the atlas is sampled at the position offset half a probe spacing along
    the normal, clamped to texel centres;
  - L2 SH is evaluated for the shading normal;
  - intensity and falloff apply.

  The first visible baked grid applies. Only one grid is supported per
  render.
  The atlas binds only into the built-in materials' pipelines and into custom
  shader programs that sample it ( `shaders/probe_grid.wgsl` ). Other custom
  programs leave it out, so their pipelines stay within the 16 sampled
  textures per stage that WebGPU guarantees ( SwiftShader's limit ).
- **Helper.** `light_probe_grid::helper()` draws one instanced sphere per
  probe, shaded by its SH irradiance, as LightProbeGridHelper does.

Sponza's materials use energy conservation, as WebGPURenderer's
MeshStandardMaterial does. Without it, indirect diffuse misses the
`1 − ( single + multiple scattering )` factor and GI reads about 6 % brighter.

The engine's shadow atlas now only grows. A pass with fewer shadow cameras
reuses the atlas's first layers and keeps the other layers' caster slots;
here, that is a bake's directional light between frames of the SunLight's
two cascades.

`tests/browser/compute-examples.spec.js` compares the captures after 250
frames ( a complete bake ) at each state:
- the initial view;
- GI off and on;
- the probe helper shown, then resized;
- a new light azimuth;
- no bounce;
- a 4 × 7 × 7 grid.

Without MSAA, every state matches at the default thresholds ( at most 0.01 %
of pixels differ ). With 4× MSAA, edge resolve allows 3.5 % / 1.2. The
residency test checks that baking frames create nothing.

The gallery renders at `min( devicePixelRatio, 1.5 )`, as the page sets.

Not ported: the Inspector panel, the progress bar and the "Log Camera"
button. These have no effect on rendering.

## webgpu_vxgi_sponza

Sponza is lit by a shadowed DirectionalLight ( 0xfff2dc, intensity 100,
2048² map ) and a 0.5 AmbientLight under SkyMesh, with FirstPersonControls.
The page leaves the pixel ratio at 1, and so does the gallery.
VXGINode traces voxel cones for AO and indirect light, and TRAA resolves
the cones' noise. Every stage runs the WGSL that r186 generates for the page
( `src/browser/vxgi_sponza/` ):

- **Collection.** VXGIVolume's scene collector runs once on the CPU, as the
  addon does:
  - the triangles are split to the sub-voxel edge;
  - each triangle's albedo is read at its centroid from the texture's 64²
    downscale. The downscale is a high-quality canvas draw of an
    ImageBitmap ( premultiplyAlpha none ), which is how the addon draws the
    loader's bitmap.

  The 274,627 triangles and their albedo match the original's buffer.
- **Volume.** The triangles are voxelized on the GPU into a
  144 × 64 × 96 opacity volume and its mips. The light is injected through
  the shadow map, and one bounce goes into the radiance volume. This runs
  again only when the light or the injection settings change.
- **Frame.** Each frame runs the shadow pass, the pre-pass ( packed normals
  and velocity ), the cone tracer, the scene pass and TRAA. The cone
  tracer's AO and GI rotate with `frameId`, and the scene pass reads them in
  its lighting. TRAA then resolves the result.
- **GUI.** The GUI follows the page's order. Its output views follow
  `updatePostprocessing()`:
  - Direct renders the scene pass without the GI context, so the GI pass
    and its volume update do not run;
  - AO and GI show the effect alone;
  - turning temporal filtering off shows the scene pass without TRAA.

  The cone, AO, GI, bounce and light settings apply.

Not reproduced: the voxel views ( radiance and opacity, and their level )
and directional radiance. They return an error. The Inspector panel and the
progress bar are not ported either; they have no effect on rendering.

### Determinism and the comparison

The voxelize kernel ORs each triangle's coverage into the voxel's
occupancy. It then stores the triangle's id into the voxel without
ordering, so one of the voxel's triangles wins. That winner sets the voxel's
albedo and normal for the injection, and it changes between runs of the
original itself.

After 64 frames, the original's combined view differs from another run of
itself by up to 10.5 % of the pixels ( mean error up to 2.1 ). The port keeps the
same kernel and the same race.

`tests/browser/compute-examples.spec.js` ( `serialVoxels` ) rewrites that
store to an `atomicMax` of the id in both runtimes' kernels, when the shader
module is created. With the winner fixed, the comparison holds the default
thresholds after 64 frames at each state:
- the initial view;
- the AO, GI, Direct and Combined outputs;
- temporal filtering off and on;
- GI intensity 0 and back;
- 6 cones and back;
- 0, 2 and 1 bounces;
- a new light azimuth;
- a resize.

Every state differs by at most 0.11 % of the pixels. The residency test
checks that frames and parameter changes create nothing.
