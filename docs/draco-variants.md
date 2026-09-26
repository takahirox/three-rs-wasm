# Uniform-buffer arrays, Draco, keyframes, material variants and facecap

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/draco_variants.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_ubo_arrays` | 293 | 300 point lights in the LightingData block, updated every frame, the plane and 100 spheres, the orbit without pan, the count control |
| `webgl_loader_draco` | 294 | The Draco bunny with computed normals, the hemisphere light, the PCF-shadowed spotlight, fog, the camera on `Date.now()` |
| `webgl_animation_keyframes` | 295 | The Sky and its PMREM environment, the Draco Littlest Tokyo glTF and its clip, ACES, the damped orbit |
| `webgl_loader_gltf_variants` | 296 | The HDR environment and background, the KHR_materials_variants shoe, on-demand rendering, the orbit, the variant control |
| `webgpu_morphtargets_face` | 297 | The RoomEnvironment PMREM, the meshopt/KTX2 facecap glTF and its 52-target morph clip, ACES, the damped orbit with azimuth and distance limits |

`webgl_morphtargets_face` is excluded as an equivalent of the WebGPU port. The
other four have no WebGPU equivalent and are compared against the WebGL
renderer.

## Port notes

### Uniform-buffer arrays

- **LightingData.** The block holds 300 positions and 300 colors. The port
  keeps it in one resident storage buffer. `animate()` writes the 300 moved
  positions each frame (4,800 bytes), as the original's block update does.
- **Colors and count.** Each light color is `Color( 0xffffff * random )`: the
  floored hex in linear space. The block's count starts at `POINTLIGHTS_MAX`
  (300). The GUI shows 200 but applies it only on change.
- **Shader.** The fragment loop is ported as written:
  - `vPositionEye` is the world position;
  - the light adds `color × getDistanceAttenuation( d, 4, 0.7 )`;
  - the result is written without an output conversion.
- **Pixel ratio.** The original sets no pixel ratio, so the page renders at 1.

### Draco bunny

- **Decoding.** DRACOLoader's standalone path is ported: `decode_draco` reads
  position, normal, color and uv from a `.drc` file with its face indices.
  `computeVertexNormals` then runs, as in the example.
- **Lights.** The HemisphereLight sits at `Object3D.DEFAULT_UP`. The spotlight
  casts a PCF shadow with radius 8.
- **Camera.** The camera circles on `Date.now() * 0.0003`.

### Littlest Tokyo

- **Sky.** The Sky shares the SkyMesh program. Its uniforms are the example's:
  - turbidity 0, rayleigh 3;
  - mieDirectionalG 0.7, cloudElevation 1;
  - the fixed sun position.
- **Environment.** `PMREMGenerator.fromScene( sky )` captures and prefilters
  the sky once.
- **Model.** The Draco-compressed glTF plays its first clip.
- **Controls.** The damped OrbitControls update once per animation frame
  before rendering, as `animate()` does.

### Material variants

- **Import.** The glTF importer now returns each primitive's
  `KHR_materials_variants` mappings. Their materials are built with the same
  textures as the default material.
- **Selection.** `selectVariant()` assigns the mapped material, or restores
  the original when no mapping lists the variant.
- **Rendering.** The HDR is the environment and the background. The page
  renders on change.

### Facecap

- **Model.** The glTF (meshopt geometry, KTX2 textures) plays its 52-target
  morph clip.
- **Controls.** OrbitControls now supports `minAzimuthAngle` and
  `maxAzimuthAngle`. The damped controls update after each render, as the
  original's loop does. The first frame therefore shows the camera's initial
  orientation.
- **Sliders.** The morph sliders only display the influences, which the mixer
  overwrites every frame; they are not reproduced.

Inspector, Stats and lil-gui appearance are not reproduced. OrbitControls touch
input is not separately verified.

## Comparison

`reference/three-js/draco-variants.html` executes the pinned sources on the
example clock.

- **Coverage.** Every control, orbit input and resize, at DPR 1 and 2.
- **Controls.** Damped controls update once per rendered frame on both sides.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Uniform-buffer arrays, MSAA off / on | 0% / 0.00, 0.557% / 0.09 | 0% / 0.00, 0.576% / 0.08 |
| Draco bunny, MSAA off / on | 2.712% / 0.60, 2.865% / 0.63 | 2.681% / 0.60, 2.754% / 0.61 |
| Littlest Tokyo, MSAA off / on | 0.327% / 0.27, 8.857% / 1.94 | 0.205% / 0.23, 4.506% / 1.05 |
| Material variants, MSAA off / on | 0.112% / 0.47, 0.752% / 0.57 | 0.066% / 0.46, 0.358% / 0.51 |
| Facecap, MSAA off / on | 0% / 0.76, 0% / 0.76 | 0% / 0.79, 0% / 0.79 |

Scoped bounds:

- **MSAA edges.** With MSAA off, these scenes match within the ordinary
  threshold; with MSAA on, only edge pixels differ. The bounds are:
  - uniform-buffer arrays: 1% / 0.15;
  - Littlest Tokyo (many thin wires and edges): 10% / 2.0;
  - material variants: 1% / 0.6.
- **Draco bunny.** PCF rotates its five Vogel taps by interleaved gradient
  noise of the fragment coordinate. WebGL's fragment rows count from the
  bottom and its shadow-map v points up; the port uses WebGPU's top-down
  convention. The radius-8 penumbra's noise therefore differs pixel by pixel.
  This case is bounded at 3% / 0.7.
- **Facecap.** The WebGPU reference's solid ACES background comes out one
  8-bit step brighter (106 against the computed 105.4); no pixel exceeds the
  threshold. This case is bounded at 0.5% / 0.8.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the 101 uniform-buffer meshes;
  - the bunny scene;
  - the 79 Littlest Tokyo draws;
  - the shoe;
  - the facecap meshes.

  The only exception is the variant example's HDR background. The port draws
  it with a fullscreen triangle; WebGL uses a 36-index box.
- **Uploads.**
  - The light positions stream 4,800 bytes a frame on both sides.
  - Skinned and morphed nodes write only their resident draw data.
  - No scene uploads geometry or texture data by writes after a warm pass.
- **Environments.** They are prefiltered once.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create no
  GPU resources.

No GPU timing parity is claimed. Full measurements are in
[draco-variants-comparison.json](draco-variants-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js draco-variants.spec.js
DRACO_DPR=2 npx playwright test -c playwright.gallery.config.js draco-variants.spec.js
```
