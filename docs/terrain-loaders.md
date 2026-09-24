# Noise terrain, GCode, VOX and OBJ/MTL loaders

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/terrain_loaders.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_geometry_terrain` | 218 | A 256×256 ImprovedNoise heightfield, its baked Canvas2D texture, exponential fog, FirstPersonControls |
| `webgl_geometry_terrain_raycast` | 219 | The same terrain under OrbitControls, with a cone following the pointer's raycast hit and face normal |
| `webgl_loader_gcode` | 220 | The three GCode files as extruded and travel line segments, the asset control, on-demand rendering |
| `webgl_loader_vox` | 221 | The monu10 VOX model, greedy-meshed with its palette colors, under hemisphere and directional light |
| `webgl_loader_obj` | 222 | The male02 OBJ with its MTL Phong materials and textures, a camera-attached point light, damped orbit |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Terrain

- **Height field.** `generateHeight` is ported with `ImprovedNoise`, including
  the Uint8Array accumulation (truncation modulo 256). The first-person example
  replaces `Math.random` with its own `sin( seed ++ )` sequence; the raycast
  example draws from the page's seeded `Math.random`. Both are reproduced.
- **Texture.** `generateTexture` computes the slope shading with the original's
  out-of-range reads (NaN, stored as 0) and `Uint8ClampedArray` rounding. The 4×
  upscale is drawn by the same Canvas2D. The per-pixel noise is added afterwards.
  The result is a mipmapped sRGB texture, as `CanvasTexture` uploads it.
- **Geometry.** The plane is rotated, and its heights are written into one
  resident geometry.
- **Controls.** FirstPersonControls keep the original's damped velocities.
  Zero-length frames do not step them, in the port and in the reference fixture
  alike. Otherwise the renders that input triggers in the still test mode would
  add damping steps.
- **Raycast.** The raycast example casts on the CPU for each pointer move, as
  the original does. The cone looks along the hit triangle's
  `Triangle.getNormal`, then moves to the hit point.
- **Normal material.** WebGL's `MeshNormalMaterial` writes its packed normals
  without the output encoding. The cone decodes them first, so the encoded
  target stores the original's values.

### GCode

- **Parsing.** The parser follows `GCodeLoader`:
  - the comment rule (`/;.+/`), which keeps a trailing carriage return;
  - `parseFloat` prefixes;
  - absolute and relative moves, and the M82/M83 extrusion overrides;
  - `G92` resets.
- **Extrusion flag.** A segment is extruded when it adds filament. The flag is
  set on the old state after the new one was cloned, so it is not carried.
- **Residency.** Each file's segments are parsed once, on first selection, and
  kept resident. The original re-parses on every switch. The asset control
  resets the controls as `controls.reset()` does.
- **Rendering.** The scene renders on demand.

### VOX

- **Parsing.** The chunks, scene-graph nodes and palette follow `VOXLoader`.
  `buildMesh` is ported: per-axis slice masks merged greedily into quads, sRGB
  palette colors in linear space, and `computeVertexNormals`. The example's
  `scene.children[ 0 ]` is the transformed shape mesh.
- **Lighting.** The hemisphere light sits at `Object3D.DEFAULT_UP`.
  `MeshStandardMaterial` uses r186's energy-conserving physical shading, which
  the WebGL renderer also applies: indirect diffuse is scaled by the remaining
  single- and multi-scattering energy.

### OBJ/MTL

- **OBJ.** The parser follows `OBJLoader`'s state machine: `o`/`g` objects,
  `usemtl` groups with inherited and pruned materials, fan triangulation, and
  negative indices.
- **MTL.** Kd and Ks are converted from sRGB, Ns is read, and each map_Kd is a
  repeating sRGB map.
- **Textures.** The port decodes each image once and shares it between
  materials. The original loads it once per material.

Stats styling and the lil-gui appearance are not reproduced. OrbitControls and
FirstPersonControls touch input is not separately verified.

## Comparison

`reference/three-js/terrain-loaders.html` executes the pinned sources. It uses
seeded randomness (except where the terrain example seeds its own), and a Timer
on the example clock. The suite covers:

- scripted first-person drags and key holds;
- pointer raycasts, orbit drags, pans and wheels;
- every GCode asset;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Terrain | 0.002% / 0.42 | 0.001% / 0.42 |
| Raycast terrain, MSAA off / on | 0.198% / 0.06, 0.719% / 0.18 | 0.066% / 0.02, 0.290% / 0.07 |
| GCode, MSAA off / on | 0.058% / 0.10, 10.977% / 4.35 | 0.046% / 0.08, 10.641% / 3.63 |
| VOX, MSAA off / on | 0.001% / 0.00, 1.863% / 0.27 | 0% / 0.00, 0.931% / 0.13 |
| OBJ, MSAA off / on | 0% / 0.00, 0.301% / 0.06 | 0% / 0.00, 0.143% / 0.03 |

The terrain's small uniform mean error comes from the texture: few pixels exceed
the threshold.

### MSAA

All five scenes match within the ordinary threshold with MSAA off. With 4×
MSAA, the differences lie on:

- the terrain and voxel silhouettes;
- the dense one-pixel GCode lines, where per-sample line coverage differs
  between WebGL and WebGPU.

The GCode scene carries the same light: in every state, the mean RGB level of
the port is within 0.15% of the original's.

The suite requires, with MSAA:

- the GCode mean levels within 0.5%, in every state;
- the raycast terrain within 1% / 0.25;
- the GCode scene within 13% / 5;
- the VOX scene within 2.5% / 0.35.

OBJ keeps the ordinary threshold with MSAA as well.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - one terrain draw of 390,150 indices;
  - the raycast terrain and its cone;
  - two GCode line draws (129,288 vertices for benchy);
  - one VOX draw of 33,144 indices;
  - 13 OBJ material-group draws.
- **Uploads.** After a warm pass, no example uploads geometry or texture data.
  The original streams nothing either.
- **Warmed cycles.** Warmed cycles of time, input scripts, asset switches and
  resize create no GPU resources.
- **Idle.** The GCode scene stays idle without input.

No GPU timing parity is claimed. Full measurements are in
[terrain-loaders-comparison.json](terrain-loaders-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js terrain-loaders.spec.js
TERRAIN_DPR=2 npx playwright test -c playwright.gallery.config.js terrain-loaders.spec.js
```
