# EXR and KTX2 exporters, video cubes, async compilation and object-space normals

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/exporters_video.rs`, with the file writers in
`src/browser/exporters_video/formats.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `misc_exporter_exr` | 273 | The PMREM background, the Float data texture quad, the damped orbit, the target, type and compression controls, EXR export |
| `misc_exporter_ktx2` | 274 | The PMREM background with AgX, the data texture quad, the damped orbit, the target control, KTX2 export |
| `webgpu_materials_video` | 275 | The VideoTexture on 200 Phong cubes with per-cube UVs, the cycling hues, the 1000-frame drift, the mouse-following camera |
| `webgpu_compile_async` | 276 | 256 unique MaterialX noise materials added after 1 s, the swinging normal-material sphere, the orthographic camera |
| `webgl_materials_normalmap_object_space` | 277 | Nefertiti with an object-space normal map, normals removed, the camera-mounted point light, on-demand rendering |

`webgl_materials_video` is excluded as an equivalent of the WebGPU port. The
two exporters and `webgl_materials_normalmap_object_space` have no WebGPU
equivalent and are compared against the original WebGL renderer.

## Port notes

### Exporters

- **Background.** `scene.background` is the PMREM render target itself, so the
  background samples the cube-UV atlas at blurriness 0. `Scene::background_pmrem`
  selects that path.
- **Data texture.** `createDataTexture()` is computed in f64 and stored as
  Float32, as in the original. The quad shows it nearest-filtered from a
  half-float copy.
- **Target switch.** `swapScene()` moves the camera, disables the controls for
  the data texture and, for KTX2, switches AgX tone mapping off. The damped
  controls update once per frame, enabled or not, as `animate()` does.
- **Readback.** A PMREM export copies the atlas from the GPU asynchronously, as
  `readRenderTargetPixelsAsync` does. The port samples its atlas with the
  direction's y negated, so the readback reorders each face region: rows
  reversed, ±Y faces exchanged. The result has three's texel layout.
- **EXR.** EXRExporter is ported: the A, B, G, R scanline order, the bottom-up
  rows, `DataUtils.toHalfFloat`, and ZIP (16 lines) and ZIPS (1 line) blocks.
  Blocks are compressed with miniz_oxide; fflate's zlib bytes differ, so
  compressed files are compared after inflating.
- **KTX2.** ktx-parse's `write` is ported for uncompressed RGBA data:
  - the data format descriptor with signed float samples;
  - the `KTXwriter` key;
  - the level alignment.

### Video cubes

- **Video.** The page's looping `#video` restarts at 3 s on `play`. The source
  elements carry the original's types, so browsers without Theora choose the
  MP4.
- **Uploads.** VideoTexture uploads a frame only when the video presents a new
  one.
- **Materials.** The 200 Phong materials share one program that multiplies the
  material color by the video. Only their colors change per frame.
- **Motion.** Colors follow `Date.now()` on the example clock. The camera easing
  and the frame counter step at 60 fps. `mouseX` and `mouseY` stay 0 until the
  first mouse move.

### Async compilation

- **Materials.** Each material is a distinct program with its own constants:
  - `mx_noise_vec3`, `mx_worley_noise_vec3` (squared distance) and
    `mx_cell_noise_float` on the 2D UV;
  - `mx_fractal_noise_vec3`, whose vec3 layout widens the UV to `vec3( uv, 0 )`;
  - the `hash` tints.
- **Build.** The programs are created between frames, 16 a frame, while the
  sphere animates, as `compileAsync` builds in the background. They skip the
  creation-time validation pipeline, and their pipelines compile on first
  draw. The group appears at 1 s, when the original's timer adds it; any
  programs still pending are created then.
- **Sphere.** MeshNormalNodeMaterial takes `directionToColor( normalView )` as
  sRGB.
- **Resize.** After a resize the camera uses the 12-unit frustum of the
  original's `onWindowResize`, not the 20-unit one of `init`.
- **Not ported.** The two page-reload mode buttons and the frame-time readouts.

### Object-space normal map

- **Normals.** `normal_fragment_maps` with `USE_NORMALMAP_OBJECTSPACE` is ported:
  the map's normal is flipped for back faces and taken through the normal
  matrix.
- **Roughness.** The normal attribute is deleted, so WebGL reads it as zero.
  `nonPerturbedNormal` and `geometryRoughness` are then NaN, and on the
  reference GPU the roughness clamp `min( NaN, 1.0 )` yields 1. The port uses
  roughness 1.
- **Setup.** The model is halved and recentered by its bounding box. The point
  light is a child of the camera. The page renders on change at a pixel ratio
  of 1, as the original sets none.

lil-gui appearance is not reproduced. OrbitControls touch input is not
separately verified.

## Comparison

`reference/three-js/exporters-video.html` executes the pinned sources on the
example clock.

- **Coverage.** Every control, both export targets, video seeks, mouse moves,
  orbit input and resize, at DPR 1 and 2.
- **Fixture changes.** The fixture awaits `addMeshes()` and runs its timer on
  the example clock. Because the example clock starts at 0, the sphere's
  `sphereStartTime > 0` check becomes `>= 0`.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| EXR exporter, MSAA off / on | 0% / 0.19, 0% / 0.19 | 0% / 0.19, 0% / 0.19 |
| KTX2 exporter, MSAA off / on | 0% / 0.15, 0% / 0.15 | 0% / 0.15, 0% / 0.15 |
| Video cubes, MSAA off / on | 0% / 0.11, 0.100% / 0.11 | 0% / 0.11, 0.057% / 0.11 |
| Async compilation, MSAA off / on | 0% / 0.02, 0% / 0.02 | 0% / 0.02, 0% / 0.02 |
| Object-space normals (no MSAA) | 0.011% / 0.23 | 0.007% / 0.20 |

All five scenes match within the ordinary threshold.

Exported files:

| File | Result |
| --- | --- |
| EXR data texture: Half ZIP, Float ZIPS, Half NONE | Identical header, block table lines and pixel bytes |
| EXR PMREM, Half ZIP | Identical header and lines; 0.025% of half floats more than 2% apart |
| KTX2 data texture | Identical container and data |
| KTX2 PMREM | Identical container; 0.004% of half floats more than 2% apart |

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the 200 video cubes;
  - the 256 planes and the sphere;
  - the Nefertiti mesh.

  The only exception is the PMREM background. The port draws it with a
  fullscreen triangle; WebGL uses a 36-index box.
- **Uploads.** The video's frames are copied from the element, as VideoTexture
  does. No scene uploads geometry or texture data by writes after a warm pass.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create no
  GPU resources.
- **Exports.** A PMREM export creates one readback buffer, as the original
  allocates its read array.

No GPU timing parity is claimed. Full measurements are in
[exporters-video-comparison.json](exporters-video-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js exporters-video.spec.js
EXPORTERS_DPR=2 npx playwright test -c playwright.gallery.config.js exporters-video.spec.js
```
