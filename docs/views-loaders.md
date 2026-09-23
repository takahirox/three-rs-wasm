# Multiple views, OBB collisions, sorted custom points, XYZ and TGA loaders

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/views_loaders.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_multiple_views` | 198 | One scene in three viewports with their own cameras and clear colors, mouse-driven camera motion |
| `webgl_math_obb` | 199 | 100 rotating Lambert boxes, pairwise OBB collision coloring, click-to-select OBB ray hits, damped orbit |
| `webgl_custom_attributes_points2` | 200 | 3,120 merged sphere and cube points, per-frame CPU depth sort, animated sizes, transparent disc sprites |
| `webgl_loader_xyz` | 201 | The helix XYZ point cloud, centered and rotating |
| `webgl_loader_texture_tga` | 202 | Two Phong crates with 8-bit grey and color-mapped TGA textures, orbit/pan |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Multiple views

- **Viewports.** Each view renders the shared scene into its original viewport:
  floored CSS rectangles, then WebGL's `round(× pixelRatio)`. Each scissored
  clear color is drawn as one fullscreen triangle inside that viewport.
- **Shadows.** The radial-gradient shadow canvas is rasterized by the same
  Canvas2D.
- **Camera motion.** The per-frame camera increments (`mouseX * 0.05`) use 60 fps
  steps of example time. The reference fixture scales the original the same way.

### OBB

- **Collision and ray tests.** `OBB.applyMatrix4`, `intersectsOBB` (SAT) and
  `intersectRay` are ported. Collision coloring runs on the CPU every frame and
  changes only material colors, as in the original.
- **Hitbox.** A click moves the wireframe hitbox under the nearest hit, or
  detaches it when nothing is hit.

### Sorted points

- **Merging.** The sphere and cube vertices are merged with the `mergeVertices`
  rule.
- **Buffers.** Positions and colors stay resident. Each frame writes only the
  sorted draw order and the animated sizes (8 bytes per point). The original
  re-uploads its index and size attributes.
- **Sort timing.** `sortPoints()` runs before `render()` refreshes
  `matrixWorld`, so the original sorts with the previous frame's rotation. The
  port does the same.
- **Float arithmetic.** The depth sort uses Three.js's own float arithmetic
  (`setFromEuler`, `compose`, `makePerspective`, `multiplyMatrices`,
  `applyMatrix4`). Its draw order matched the original index-for-index in a
  recorded comparison.
- **Vertex stage.** It takes CPU-computed model-view and projection matrices, as
  WebGL's uniforms do.

### Loaders

- **XYZ.** The parser follows `XYZLoader`: three or six values per line, with
  optional sRGB byte colors.
- **TGA.** The decoder follows `TGALoader`: color-mapped, true-color and grey
  images, raw or RLE, with origin-dependent row order. The `image` crate
  rejected the color-mapped crate.

Stats styling is not reproduced. OrbitControls keyboard input is not ported.

## Comparison

`reference/three-js/views-loaders.html` executes the pinned sources. It uses
fixed time, seeded randomness and a Timer on the example clock. The suite
covers:

- time steps and mouse motion;
- clicks, settled damped drags, pan and wheel;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Multiple views, MSAA off | 0.157% / 0.14 | 0.101% / 0.09 |
| OBB, MSAA off | 0.087% / 0.12 | 0.044% / 0.06 |
| Sorted points | 1.175% / 0.43 (t=0.4), otherwise ≤0.09% | 0.109% / 0.05 |
| XYZ, MSAA off / on | 0.010% / 0.02, 0.097% / 0.06 | 0.005% / 0.01, 0.038% / 0.02 |
| TGA, MSAA off / on | 0% / 0.01, 0.089% / 0.02 | 0% / 0.00, 0.042% / 0.01 |
| Multiple views, 4× MSAA | 1.662% / 0.42 | 0.922% / 0.23 |
| OBB, 4× MSAA | 1.892% / 0.64 | 0.938% / 0.32 |

### Sorted points at t = 0.4

The capture after t = 0 sorts with the unrotated matrix, then draws with a 0.04
rad rotation. Neighboring cube points in a column then differ in depth by
0.04 × 0.04 × 8 units, about 4e-7 in NDC. Their transparent square corners write
depth. So which point survives each overlap depends on float32 rounding of
nearly tied depths.

Moving the original's camera by one Float32 ULP (300 → 300 + 2⁻¹⁵) changes the
original itself:

| DPR | t = 0.4 | Other states (at most) |
| --- | --- | --- |
| 1 | 1.70% of pixels | 0.16% |
| 2 | 0.15% of pixels | none |

The suite renders that one-ULP variant alongside the original, and bounds each
state by 1.25× its measured sensitivity, but never below the ordinary limits.
States that are insensitive keep the ordinary threshold. The sort order, sizes,
geometry and draw workload are not relaxed.

### MSAA

With MSAA off, the multiple-view and OBB scenes match within the ordinary
threshold. With 4× MSAA, the differences lie on the black wireframe edges and
the box silhouettes, as in the earlier batches.

The suite requires the ordinary threshold with MSAA off. With MSAA on, it bounds
multiple views to 1.9% / 0.5 and OBB to 2.1% / 0.75.

## Performance evidence

- **Draw workload.** The measured draws equal the original:
  - 27 viewport draws for the three views;
  - 100 box draws;
  - one draw of 3,120 point billboards;
  - one draw of 201 XYZ billboards;
  - two crate draws.

  WebGL points are counted as six-vertex billboards.
- **Uploads.** After a warm pass, a measured frame uploads no geometry or texture
  data. The one exception is the sorted-points order and size buffers: 24,960
  bytes per frame, against the original's index and size re-upload.
- **Warmed cycles.** Warmed time, click, drag and resize cycles create no GPU
  resources.

No GPU timing parity is claimed. Full measurements are in
[views-loaders-comparison.json](views-loaders-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js views-loaders.spec.js
VIEWS_DPR=2 npx playwright test -c playwright.gallery.config.js views-loaders.spec.js
```
