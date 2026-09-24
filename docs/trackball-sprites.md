# HDR texture, voxel terrain, trackball controls, sprites and LOD

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/trackball_sprites.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_loader_texture_hdr` | 213 | The memorial HDR as a half-float texture on an orthographic quad, Reinhard tone mapping, the exposure control, on-demand rendering |
| `webgl_geometry_minecraft` | 214 | The 128×128 ImprovedNoise voxel terrain merged into one geometry, the atlas texture, FirstPersonControls |
| `misc_controls_trackball` | 215 | 500 instanced cones in fog, damped TrackballControls with the A/S/D keys and the orthographic-camera toggle |
| `webgl_sprites` | 216 | 200 rotating, scaling sprites with fog, and five HUD sprites drawn after a depth clear |
| `webgl_lod` | 217 | 1,000 five-level wireframe LODs chosen by camera distance, FlyControls |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory. `webgpu_sprites` is a different scene: it has no HUD pass and a
different fog. Each is compared against the original WebGL renderer.
`webgl_morphtargets`, `webgl_panorama_equirectangular` and
`webgl_buffergeometry_glbufferattribute` stay excluded: the first two in favor
of their WebGPU ports, and the last as a WebGL-specific API.

## Port notes

### HDR texture

- **Decoding.** The RGBE reader follows `HDRLoader`, for flat or new-style RLE
  scanlines. It converts with `RGBEByteToRGBHalf`: channel × 2^(e − 128) / 255,
  clamped to 65504. It then rounds to Float32 and truncates the mantissa into
  half floats, with `DataUtils`' tables.
- **Texture.** The image crate scales by 1/256 and rounds its halves, so it is
  not used here. The texture is uploaded once, with linear filtering, no
  mipmaps, and sampled flipped as `flipY` uploads it.
- **Shading.** The fragment applies `ReinhardToneMapping` with the exposure,
  then the sRGB output encoding.
- **Rendering.** The scene renders only on control changes and resize.

### Voxel terrain

- **Terrain.** `ImprovedNoise` and `generateHeight` are ported, with the
  original's seeded `Math.random()` first draw. `getY` truncates with `| 0`,
  and out-of-range reads give 0, as undefined does.
- **Geometry.** The five face templates are rebuilt with the edited uvs and
  rotations, rounded to Float32. They are merged in the original's face order
  into one resident indexed geometry.
- **FirstPersonControls.** The original's damped velocity and look velocities
  are kept. Zero-length frames do not step the controls, in the port and in
  the reference fixture alike. Otherwise the renders that input triggers in the
  still test mode would add damping steps.

### Trackball controls

- **Behavior.** `TrackballControls` is ported with its absolute pointer
  positions: the on-circle rotation, screen-space zoom and pan, damping 0.2,
  and the A/S/D key states. The orthographic zoom and pan scale is kept,
  including the original's use of the width for both axes.
- **Stepping.** `update()` runs as 60 fps steps of example time. The reference
  fixture runs the same step count.
- **Camera toggle.** Switching creates fresh controls for the other camera, as
  the original does.
- **Not verified.** Touch gestures and `multiTouchRoll`.

### Sprites

- **Vertex stage.** The billboard follows `sprite.glsl.js`: the model-view
  translation, the model scale lengths, center and rotation.
- **Fragment stage.** The fragment follows the sprite shader's order: the map
  and diffuse color, the sRGB encoding, then fog toward black.
- **Materials.** The C sprites' shared map carries the original's offset
  (−0.5, −0.5) and repeat (2, 2). Colors use `setHSL` in the working space.
- **Animation.** Per-frame rotation increments run as 60 fps steps.
- **Draw order.** The transparent sprites keep the original's back-to-front
  order. The HUD pass renders over them after a depth clear.

### LOD

- **Level selection.** `LOD.update` chooses among the 50/300/1000/2000/8000
  levels after `FlyControls.update( delta )`. The fly controls follow the
  pointer position, the buttons and the keys.
- **Draw data.** Per-object draw data is created on first draw. So one
  unculled pass into a 1×1 target prepares every level at startup, and flying
  then reveals no new GPU resources.

Stats styling and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/trackball-sprites.html` executes the pinned sources. It
uses seeded randomness, and a Timer and timers on the example clock. The suite
covers:

- time steps;
- scripted drags, pans, wheels, key holds and control changes, each captured at
  its own time;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| HDR texture | 0% / 0.00 | 0% / 0.00 |
| Voxel terrain, MSAA off / on | 0.034% / 0.01, 0.550% / 0.09 | 0.015% / 0.00, 0.303% / 0.05 |
| Trackball, MSAA off / on | 0% / 0.00, 1.154% / 0.19 | 0% / 0.00, 0.480% / 0.08 |
| Sprites | 0.041% / 0.01 | 0.042% / 0.00 |
| LOD, MSAA off / on | 0.028% / 0.01, 16.279% / 6.03 | 0.012% / 0.01, 11.009% / 4.00 |

### MSAA

All five scenes match within the ordinary threshold with MSAA off. With 4×
MSAA, the differences lie on the terrain and cone silhouettes and on the LOD
field's dense wireframes, where per-sample line coverage differs between WebGL
and WebGPU. The LOD field carries the same light: in every state, the mean RGB
level of the port is within 0.14% of the original's.

The suite requires, with MSAA:

- the LOD mean levels within 0.5%, in every state;
- the terrain within 0.75% / 0.15;
- the trackball within 1.5% / 0.25;
- the LOD field within 20% / 7.5.

MSAA-off comparisons keep the ordinary threshold.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - one HDR quad;
  - one merged terrain draw of 138,084 indices;
  - one instanced draw of 500 cones;
  - 205 sprite draws;
  - 137 culled LOD level draws, 166,200 wireframe indices in the measured
    frame.
- **Uploads.** After a warm pass, no example uploads geometry or texture data.
  The original streams nothing either.
- **Warmed cycles.** Warmed cycles of time, input scripts, keys, control changes
  and resize create no GPU resources. That holds for the LOD field after its
  startup preparation.
- **Idle.** The HDR scene stays idle without input.

No GPU timing parity is claimed. Full measurements are in
[trackball-sprites-comparison.json](trackball-sprites-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js trackball-sprites.spec.js
TRACKBALL_DPR=2 npx playwright test -c playwright.gallery.config.js trackball-sprites.spec.js
```
