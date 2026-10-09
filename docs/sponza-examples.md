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

Not ported: the Inspector panel, the progress bar and the "Log Camera"
button. These have no effect on rendering.
