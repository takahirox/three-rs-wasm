# BVH, framebuffer texture, float readback, GPU picking and instancing performance

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/picking_buffers.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_loader_bvh` | 228 | The pirouette BVH as a bone hierarchy, played by AnimationMixer on a loop, with its SkeletonHelper and grid |
| `webgl_framebuffer_texture` | 229 | The Gosper curve with per-frame colors, its centre copied to a texture each frame and shown by a HUD sprite |
| `webgl_read_float_buffer` | 230 | A Float32 render target shown on screen, and the value under the mouse read back each frame |
| `webgl_interactive_cubes_gpu` | 231 | 5,000 merged boxes, a one-pixel GPU picking pass with async readback, the highlight box, TrackballControls |
| `webgl_instancing_performance` | 232 | Suzanne as INSTANCED, MERGED or NAIVE meshes, the method and count controls, auto-rotating OrbitControls |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### BVH

- **Parsing.** The reader follows `BVHLoader`: the node hierarchy with end
  sites, and per-frame channels. Rotations are multiplied in channel order;
  positions have the joint offset added, as the clip's tracks do.
- **Animation.** Track times and values are Float32, as `KeyframeTrack`
  stores them. They are evaluated as `AnimationMixer` does: LoopRepeat over the
  clip duration, linear positions, and `Quaternion.slerpFlat` rotations.
- **Skeleton helper.** One segment per bone with a bone parent, blue at the
  child and green at the parent, drawn without depth test. Its positions are
  written from the bone world matrices every frame, the same bytes the original
  uploads with its `needsUpdate` attribute.

### Framebuffer texture

- **Curve.** `GeometryUtils.gosper( 8 )` is ported, centred and scaled.
  `updateColors` rewrites the 2,401 `setHSL` colors each frame into a storage
  buffer of three floats per vertex, the size of the original's
  `DynamicDrawUsage` attribute. The offset steps by 25 per 60 fps step of
  example time, in the port and in the reference fixture alike.
- **Copy.** After the scene pass, the centred `128 × dpr` region of the
  resolved canvas is copied into a resident texture, as
  `copyFramebufferToTexture` does. GL counts that origin from the bottom row.
- **Sprite.** The HUD sprite samples the copy without decoding it, then
  encodes the result again, as SpriteMaterial does with a `NoColorSpace`
  texture. It is drawn after a depth clear, with the orthographic HUD camera.
- **Selection frame.** The 128-pixel frame is a DOM overlay, as in the
  original. Panning is disabled.

### Float readback

- **Passes.** The RTT scene (the `pass_1` quad, two Phong tori and two
  directional lights) renders into a Float32 target of CSS-pixel size. The
  screen quad samples it with nearest filtering, then applies
  `colorspace_fragment`'s encoding.
- **Time.** The uniform bounces by 0.01 per 60 fps step of example time.
- **Readback.** The texel under the mouse is copied to a resident buffer and
  mapped asynchronously; its values fill the `#values` text once the map
  finishes. The original reads synchronously, so its text is one or more
  frames ahead.
- **Size.** The original has no resize handler: its target, cameras and plane
  keep their initial size. The port keeps them too, but the gallery canvas
  follows the window, so resized frames are not compared.

### GPU picking

- **Geometry.** The 5,000 boxes are transformed and merged once, with sRGB
  random colors in linear space, as `mergeGeometries` produces them.
- **Picking pass.** A copy of the camera with a one-pixel view offset at
  `floor( pointer × dpr )` draws the shared geometry into a 1×1 target cleared
  to −1, as `setViewOffset` and the picking material do. The original writes
  integer ids to an `RGBA32I` target. The port writes each box's id
  (`vertex_index / 24`) as a float to an `Rgba32Float` target, which is exact
  for these 5,000 ids.
- **Readback.** The texel is read asynchronously, as
  `readRenderTargetPixelsAsync` does, and moves the highlight box when it
  arrives.
- **Controls.** TrackballControls use `staticMoving` and pan speed 0.8. They
  step as 60 fps steps of example time, in the port and in the fixture alike.
- **Size.** The original has no resize handler, so resized frames are not
  compared.

### Instancing performance

- **Methods.** INSTANCED makes one instanced draw, MERGED transforms and
  merges the copies, and NAIVE adds one mesh per copy. Each uses the
  original's `randomizeMatrix`, including `Quaternion.random`.
- **Rebuilds.** Like the original, every method or count change disposes the
  meshes and builds new ones. These rebuilds create GPU resources, so the
  residency cycle leaves the controls out.
- **Controls.** `autoRotate` advances once per 60 fps step of example time.
- **Output.** WebGL's `MeshNormalMaterial` writes its packed normals without
  the output encoding, and the port's output is raw as well.

Stats, the lil-gui appearance and the instancing example's GPU memory text are
not reproduced.

## Comparison

`reference/three-js/picking-buffers.html` executes the pinned sources. It uses
seeded randomness (including `Quaternion.random`), and a Timer on the example
clock. The suite covers:

- time steps;
- every control;
- orbit and trackball drags, pans and wheels;
- pointer picks and the float values under the pointer;
- resize, where the original handles it, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| BVH, MSAA off / on | 0.371% / 0.63, 2.409% / 0.67 | 0.187% / 0.32, 1.146% / 0.32 |
| Framebuffer texture, MSAA off / on | 0.14% / 0.22, 7.431% / 2.67 | 0.109% / 0.17, 5.395% / 1.88 |
| Float readback | 0% / 0.00 | 0% / 0.00 |
| GPU picking, MSAA off / on | 0.003% / 0.00, 2.002% / 0.40 | 0.005% / 0.00, 1.062% / 0.21 |
| Instancing performance, MSAA off / on | 0.002% / 0.00, 14.557% / 2.78 | 0.001% / 0.00, 6.74% / 1.33 |

The float values under the pointer match the original's within 2e-3 (in
practice within 1e-7).

### Pixel-row tie

In the BVH scene, the camera looks at the origin, so the grid's centre line
lies exactly on the boundary between two pixel rows. WebGL draws it on the row
below and WebGPU on the row above. That line alone gives a mean error of 0.63,
so the suite bounds this scene at 0.5% / 0.7 with MSAA off. Every other state
is exact once the camera moves.

### MSAA

With 4× MSAA, the differences lie on:

- the grid and skeleton lines;
- the dense one-pixel Gosper curve, which the sprite then shows a second time;
- the box and Suzanne silhouettes.

The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| BVH | 3% / 0.8 |
| Framebuffer texture | 9% / 3.2 |
| GPU picking | 2.5% / 0.5 |
| Instancing performance | 16% / 3.1 |

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the 44-vertex grid and 112-vertex skeleton segments;
  - the 2,401-vertex curve strip and the sprite;
  - the RTT quad, two 2,700-index tori and the screen quad;
  - the 180,000-index picking and scene draws, and the highlight box;
  - one INSTANCED draw of 1,000 × 2,901 indices.
- **Uploads.** The per-frame uploads equal the original's: 1,344 bytes of
  skeleton positions and 28,812 bytes of curve colors. The other scenes upload
  no geometry or texture data after a warm pass.
- **Warmed cycles.** Warmed cycles of time, input and (except the instancing
  rebuilds) control changes create no GPU resources.

No GPU timing parity is claimed. Full measurements are in
[picking-buffers-comparison.json](picking-buffers-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js picking-buffers.spec.js
PICKING_DPR=2 npx playwright test -c playwright.gallery.config.js picking-buffers.spec.js
```
