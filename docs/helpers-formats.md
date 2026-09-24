# PDB molecules, helpers, simplifier, AMF and TIFF

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/helpers_formats.rs`, with the file formats in
`src/browser/helpers_formats/formats.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_loader_pdb` | 233 | The 17 PDB molecules as per-atom and per-bond meshes, their CSS2D labels, the molecule control, TrackballControls |
| `webgl_helpers` | 234 | The head with vertex-normal, tangent, box, wireframe and edge helpers, both grids and the orbiting point light's helper |
| `webgl_modifier_simplifier` | 235 | The head beside its SimplifyModifier result, the ratio control, OrbitControls, on-demand rendering |
| `webgl_loader_amf` | 236 | The zipped AMF rook with its default flat Phong material, a Z-up orbit, on-demand rendering |
| `webgl_loader_texture_tiff` | 237 | The uncompressed, LZW and JPEG crate TIFFs as DataTextures, on-demand rendering |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### PDB

- **Parsing.** The reader follows `PDBLoader`:
  - `ATOM`/`HETATM` fixed columns through `parseFloat` and `parseInt`;
  - the element from columns 76–78 or 12–14 and its CPK color;
  - `CONECT` bonds, each pair once.
- **Meshes.** The Float32 atom and bond geometries are centered on their
  bounding box. Each atom is an icosahedron with its own Phong material; each
  bond is a scaled box turned by `lookAt` before it joins the root, as in the
  original.
- **Labels.** `CSS2DRenderer` is reproduced in the DOM:
  - each label is projected to CSS pixels with `translate(-50%,-50%)`;
  - labels outside the depth range are hidden;
  - the z-index follows the camera distance, ties in scene order.
  The layer takes the original page's `main.css` type (Monospace 13px/24px,
  English, default font smoothing) and the example's `.label` rule. The labels
  match the original's pixels.
- **Switching.** Each molecule is built once, on first selection, and kept.
  The original rebuilds on every switch.
- **Controls.** TrackballControls with the example's distance limits step as
  60 fps steps of example time, in the port and in the fixture alike.

### Helpers

- **Tangents.** `computeTangents` runs on the glTF head.
- **Helpers.** The helper geometries are ported:
  - `VertexNormalsHelper` and `VertexTangentsHelper` world-space segments;
  - `WireframeGeometry`, with each edge once in either direction;
  - `EdgesGeometry` at 1°, with the original's rounded position hashes and
    edge order;
  - `PolarGridHelper`, `GridHelper` and the `PointLightHelper` sphere.
- **Box helpers.** Each `BoxHelper` box is computed as `Box3.setFromObject`
  does at construction time, including the scene box over every helper
  already added.
- **Uploads.** The head never moves, so the helper segments are computed once
  and stay resident. The original recomputes and re-uploads the same
  445,392 bytes of normal and tangent segments every frame.

### Simplifier

- **Simplification.** `SimplifyModifier` calls meshoptimizer 1.1's
  `simplifyWithAttributes` (normal and uv weights 0.25 and 0.5, error 1) and
  `compactMesh`. The port calls the same routines in `optimesh`, a bit-exact
  Rust port of meshoptimizer 1.1. For the ratios 0.37, 0.01 and 1, its index
  buffers equal the original's wasm build exactly.
- **Residency.** Each ratio's result is built once and kept. The original
  rebuilds on every change.
- **Shading.** The simplified copy uses the glTF material with flat shading.

### AMF

- **Parsing.** The zip entry is inflated, then parsed as `DOMParser` and
  `AMFLoader` read it:
  - the unit scale;
  - materials and their last color;
  - object and mesh colors (raw values, as `new Color( r, g, b )` stores the
    text);
  - vertices, normals and volume triangles.
- **Material.** The rook uses the loader's default flat Phong material.
- **Controls.** OrbitControls rotate about the camera's Z up, around the
  example's target, with zoom disabled.

### TIFF

- **Decoding.** `UTIF.decode`, `decodeImage` and `toRGBA8` are ported for the
  three files:
  - an uncompressed palette image;
  - an LZW RGB image with the horizontal predictor;
  - a JPEG-compressed RGB image through UTIF's bundled pdf.js baseline
    decoder, with its JPEG tables, integer inverse DCT and R/G/B component
    rule.
  All three decoded images equal UTIF's output byte for byte.
- **Textures.** The DataTextures keep the loader's settings: flipped rows,
  sRGB, linear filters and a single level, as WebGL's one-level storage
  samples them.

Stats and the lil-gui appearance are not reproduced. TrackballControls and
OrbitControls touch input is not separately verified.

## Comparison

`reference/three-js/helpers-formats.html` executes the pinned sources on the
example clock. The suite covers:

- time steps;
- every molecule and ratio;
- trackball drags, wheels and key holds;
- orbit drags and pans;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| PDB, MSAA off / on | 0.001% / 0.00, 1.059% / 0.26 | 0.001% / 0.00, 0.521% / 0.13 |
| Helpers | 0.034% / 0.02 | 0.019% / 0.02 |
| Simplifier, MSAA off / on | 0.627% / 0.12, 1.174% / 0.28 | 0.386% / 0.08, 0.643% / 0.16 |
| AMF, MSAA off / on | 0.291% / 0.05, 3.094% / 0.56 | 0.234% / 0.04, 1.59% / 0.29 |
| TIFF, MSAA off / on | 0% / 0.00, 0.001% / 0.00 | 0% / 0.00, 0.000% / 0.00 |

### Flat shading

The simplified head's facets take their normals from screen-space derivatives
of the view position. At facet edges these differ between the two backends, so
the suite bounds that scene at 0.8% / 0.6 with MSAA off.

### MSAA

With 4× MSAA, the differences lie on the atom, head and rook silhouettes, the
simplified facets and the grid lines. The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| PDB | 1.5% / 0.35 |
| Simplifier | 1.5% / 0.35 |
| AMF | 4% / 0.7 |

TIFF keeps the ordinary threshold with MSAA as well. The helpers example
renders without antialiasing.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - 49 caffeine draws: 24 atoms and 25 bonds;
  - 13 helper draws;
  - the head and its simplified copy;
  - the grid and the rook;
  - the three textured planes.
- **Uploads.** After a warm pass, no scene uploads geometry or texture data.
  The helpers example is the one difference: the original re-uploads its
  helper segments each frame (see above).
- **Warmed cycles.** Warmed cycles of time, input and control changes create
  no GPU resources.
- **Idle.** The simplifier, AMF and TIFF scenes stay idle without input.

No GPU timing parity is claimed. Full measurements are in
[helpers-formats-comparison.json](helpers-formats-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js helpers-formats.spec.js
HELPERS_DPR=2 npx playwright test -c playwright.gallery.config.js helpers-formats.spec.js
```
