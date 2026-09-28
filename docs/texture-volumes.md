# Texture array layers, the float volume and NRRD slices

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/texture_volumes.rs`; the NRRD slices are in
`src/browser/texture_volumes/slices.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_texture2darray_layerupdate` | 314 | The three-layer KTX2 (Basis) array kept compressed, one instanced plane per layer, and srcLayer/destLayer/transfer copying a source layer on the GPU, rendering only on change |
| `webgl_texture3d` | 315 | The gzip NRRD stent as a float 3D texture, VolumeRenderShader1's MIP and ISO ray marches, both colormaps, all five controls and the z-up orthographic orbit (zoom 0.5–4), rendering only on change |
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
planes' edges while rotating; with MSAA off it matches. It is bounded at
1.2% / 0.45 with MSAA.

## Performance evidence

- **Draw workload.** The measured draws equal the original's.
- **Uploads.** No geometry or texture data is written in steady frames:
  - the layer example copies layers on the GPU only on transfer;
  - the volume is uploaded once;
  - the slices copy their canvases only on a repaint, which a control
    change requests, as the original's `needsUpdate` does.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize
  create no GPU resources.

No GPU timing parity is claimed. Full measurements are in
[texture-volumes-comparison.json](texture-volumes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
VOLUMES_DPR=2 npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
```
