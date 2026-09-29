# Texture arrays, the float volume, NRRD slices, the EXR environment and the shadow-map viewer

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/texture_volumes.rs`; the NRRD slices are in
`src/browser/texture_volumes/slices.rs` and the array examples in
`src/browser/texture_volumes/arrays.rs`. The EXR environment is in
`src/browser/envmap_exr.rs` and the shadow-map viewer in
`src/browser/shadowmap_viewer.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_texture2darray_layerupdate` | 314 | The three-layer KTX2 (Basis) array kept compressed, one instanced plane per layer, and srcLayer/destLayer/transfer copying a source layer on the GPU, rendering only on change |
| `webgl_texture3d` | 315 | The gzip NRRD stent as a float 3D texture, VolumeRenderShader1's MIP and ISO ray marches, both colormaps, all five controls and the z-up orthographic orbit (zoom 0.5–4), rendering only on change |
| `webgl_texture2darray` | 324 | The unzipped 256×256×109 head as a red DataArrayTexture with nearest filtering, the layer bouncing by 0.4 per frame, in a raw ShaderMaterial |
| `webgl_texture2darray_compressed` | 325 | The Basis KTX2 array kept compressed, the timer cycling its five layers, in a raw ShaderMaterial |
| `webgl_rendertarget_texture2darray` | 326 | Each frame's head layer rendered on the GPU into a 256×256×109 red WebGLArrayRenderTarget, then shown from that layer, with the intensity control |
| `webgl_materials_envmaps_exr` | 332 | The PIZ EXR and PNG panoramas prefiltered once into PMREMs for the turning torus knot and shown as backgrounds, with ACES exposure, roughness, metalness and the map switch |
| `webgl_shadowmap_viewer` | 333 | Spot and directional BasicShadowMap shadows on the spinning knot and cube, both CameraHelpers, and the two ShadowMapViewer HUDs with their canvas labels |
| `webgl_loader_nrrd` | 321 | NRRDLoader's short-typed LPS volume, the three VolumeSlices repainted on the CPU through 2D canvases, the BoxHelper, TrackballControls and all seven controls |

None of these has an official WebGPU equivalent in the pinned inventory, so
each is compared against the WebGL renderer.

## Port notes

### Layer updates

- **Array.** The source KTX2 stays block-compressed. Its layers are copied on
  the GPU into a three-layer array, as `copyTextureToTexture` with a layer
  does. A new CompressedArrayTexture has no color space, so the array uses
  the linear block format.
- **Drawing.** One instanced plane draws all three layers; each instance
  samples its own layer.

### Float volume

- **Loader.** `parse_nrrd` reads the text header, skips the gzip member
  header, inflates the data and decodes little-endian floats or shorts.
- **Ray march.** The MIP and ISO loops take VolumeRenderShader1's 887 steps
  from the back face. The colormap lookup moves after the loop, into uniform
  control flow, since WGSL forbids implicit-LOD sampling in the loop.
- **Camera.** The orthographic frustum keeps its height on resize, and the
  orbit zoom is clamped to 0.5–4.

### Texture arrays

- **Head volume.** The head is uploaded once into an R8 array with nearest
  filtering. The `int depth` uniform truncates, as the WebGL uniform does.
- **Stepping.** The per-frame 0.4 step runs once per 60 fps step, with the
  reflection test once per frame, as the fixture's stepped frame does.
- **Render target.** The array render target is a DataArrayTexture, so it
  keeps nearest filtering. Only the current layer is rendered each frame, by
  the GPU pass. WebGPU attachments start at the top row, so the pass flips v
  to keep texel rows where WebGL writes them.

### EXR environment

- **Maps.** `decode_exr` (with PIZ) and the PNG are each prefiltered once
  into a resident PMREM atlas; switching maps swaps the resident
  environments.
- **Background.** An sRGB background texture is not tone mapped, as
  WebGLBackground turns tone mapping off for it: `Scene::background_tone_mapped`
  now carries that.
- **Debug plane.** It shows the port's own cube-UV atlas, whose layout is not
  three's, so it is not compared.

### Shadow-map viewer

- **Atlas.** `Renderer::shadow_atlas` exposes the resident shadow depth atlas.
  Each HUD reads its light's layer with `textureLoad` and reproduces the depth
  material's RGBA8 `1 − depth` (cleared white), sampled bilinearly and shown
  as `1 − r`. Its raw output is decoded before the output encoding, so it
  reaches the canvas unchanged.
- **Overlay.** The scene and the HUD scene render into one target; the HUD
  clears only depth, as `autoClear = false` with `clearDepth()` does. A pass
  without shadow casters now keeps the resident atlas and caster slots, so
  the overlay no longer reallocates them.
- **Shadow cameras.** Their view now follows Matrix4.lookAt, including the
  0.0001 nudge when the forward axis is parallel to `up` (the directional
  light straight above).
- **Helpers.** CameraHelper keeps the projection it read when constructed:
  the example's near, far and frustum edits, with the spot camera's default
  fov of 50, since the shadow pass sets 2 × angle only later.
- **Labels.** Each light's name is drawn in `Bold 20px Arial` on a canvas
  sized by `measureText`, as a CanvasTexture.

### NRRD slices

- **Volume.** The IJK-to-RAS matrix is built from `space directions` and the
  LPS transition, as NRRDLoader does. The spacing, axis order, RAS
  dimensions and the min/max window follow the loader.
- **Slices.** `extractPerpendicularPlane` is ported: the inverse-matrix
  directions, the slice lengths and plane size, the IJK index, and the
  access directions. Indices use JavaScript's arithmetic, so an index past
  an edge reads the neighbouring row and one outside the data fails both
  threshold tests.
- **Repaint.** Each repaint thresholds and windows the slice on the CPU,
  puts it into a buffer canvas, scales it into the plane-sized canvas with
  the browser's `drawImage`, and copies that canvas into the texture. This
  is the original's path, including:
  - the threshold setters marking every slice's geometry, so their repaint
    clears the canvas;
  - the window settings drawing over the previous canvas.
- **Planes.** The plane size depends only on the axis, so each slice keeps
  its resident PlaneGeometry and changes only its matrix. The original
  builds a new one when the index changes.
- **Controls.** TrackballControls runs with rotateSpeed 5, zoomSpeed 5,
  panSpeed 2 and distances 100–500, stepped at 60 fps. `Trackball` now takes
  its rotate and zoom speeds as fields.

Stats and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/texture-volumes.html` executes the pinned sources on the
example clock. The suite covers every control, the transfers, drags,
trackball pans, wheels and resize, at DPR 1 and 2. Examples with
antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Layer updates, MSAA off / on | 0% / 0.01, 0.002% / 0.01 | 0% / 0.01, 0.068% / 0.03 |
| Float volume | 1.542% / 0.38 | 1.506% / 0.28 |
| NRRD slices, MSAA off / on | 0.001% / 0.00, 0.801% / 0.33 | 0% / 0.00, 0.413% / 0.17 |
| Head array | 0% / 0.00 | 0% / 0.00 |
| Compressed array | 0% / 0.00 | 0% / 0.00 |
| Array render target | 0% / 0.00 | 0% / 0.00 |
| EXR environment | 0.076% / 0.16 | 0.042% / 0.16 |
| Shadow-map viewer, MSAA off / on | 0.001% / 0.00, 1.657% / 0.51 | 0% / 0.00, 0.864% / 0.27 |

### Volume colormap lookup

- **Cause.** The original samples the colormap with implicit LOD in
  non-uniform control flow, so its derivatives come from neighbouring
  fragments whose rays stop at different steps. The port samples in uniform
  control flow after the loop, where the derivatives differ.
- **Effect.** Pixels along the stent's edges and fine structure differ in
  both styles: up to 0.7% in MIP and 1.5% in ISO.
- **Bound.** The volume is bounded at 2% / 0.5.

### MSAA

With 4× MSAA, the slice example differs only along the box helper's and the
planes' edges while rotating, and the shadow-map viewer along its helper
lines and silhouettes; with MSAA off both match. With MSAA they are bounded
at 1.2% / 0.45 and 2.5% / 0.8.

## Performance evidence

- **Draw workload.** The measured draws equal the original's.
- **Uploads.** No geometry or texture data is written in steady frames:
  - the layer example copies layers on the GPU only on transfer;
  - the volume is uploaded once;
  - the slices copy their canvases only on a repaint, which a control
    change requests, as the original's `needsUpdate` does.
  - the array render target is drawn one layer per frame on the GPU;
  - the EXR and PNG PMREMs are prefiltered once.
- **Background draw.** WebGL draws the equirectangular background as a
  36-index box and the port as a fullscreen triangle; the workload test pairs
  the two.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize
  create no GPU resources.

No GPU timing parity is claimed. Full measurements are in
[texture-volumes-comparison.json](texture-volumes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
VOLUMES_DPR=2 npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
```
