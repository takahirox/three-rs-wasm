# Buffer geometry particles and selective drawing

Five pinned r186 scenes are now in the Rust/WebGPU gallery:

| Official example | Runtime | Retained workload |
| --- | --- | --- |
| `webgl_buffergeometry_points` | 158 | 500,000 colored points, fog and rotation |
| `webgl_buffergeometry_points_interleaved` | 159 | 500,000 points with packed normalized byte colors |
| `webgl_buffergeometry_custom_attributes_particles` | 160 | 100,000 additive textured points with animated sizes |
| `webgl_buffergeometry_instancing_billboards` | 161 | 75,000 instanced six-triangle circle billboards |
| `webgl_buffergeometry_selective_draw` | 162 | 20,000 native lines, repeated cull/show controls and culled count |

These scenes have no equivalent official WebGPU scene in the pinned inventory.
The WebGPU smoke/fire particle scenes, soft particles and wide-line examples
have different content and behavior. Existing equivalents remain preferred.

## Runtime scope

The existing typed TSL storage and sprite projection APIs are sufficient; no
additional renderer abstraction or CPU fallback was introduced. Rust generates
static attributes once. Animated position, size and color work runs on the GPU.
Every scene retains a single scene draw and its original particle/line count.

Point attributes occupy 12,000,000 bytes for the Float32 color scene and
8,000,000 bytes for the packed scene. The latter retains the original 16-byte
position/color record and unpacks normalized byte colors in WGSL. This is not
an implementation of the general Three.js InterleavedBuffer API.

The custom particle scene uses 1,200,000 bytes of positions. Its original CPU
size loop is evaluated in the GPU vertex shader; hue is derived from the instance
index. Billboards retain 900,000 bytes of translations and one shared circle.
Point expansion uses one shared quad, not six copies of each particle attribute.

Selective drawing retains all 40,000 line vertices and discards hidden fragments
on the GPU. Clicking a control updates only the 80,000-byte visibility buffer;
subsequent animation uploads no attributes. It never filters geometry or changes
the draw count on the CPU. The shared renderer's general line vertex layout is
still larger than the original example's dedicated attribute layout.

Shader outputs use an RGBA8 target and the original WebGL output convention.
In particular, the built-in PointsMaterial applies fog **after** sRGB encoding;
custom/raw shaders output display values directly. Point coordinates and regular
mesh UVs have different vertical conventions for the spark texture.

Stats and the exact control-panel styling are not ported. No GPU frame-time or
complete performance parity claim is made.

## Visual checks and backend differences

`tests/browser/buffer-particles.spec.js` captures four animation times, a resized
viewport, repeated culling and restoration, at DPR 1 and 2. The fixture executes
the pinned original scripts, fixing random seed and time only and suppressing
Stats. Native original rendering is always measured and retained.

Raw comparisons count pixels with any RGB channel differing by more than 6/255.
Most cases retain the ordinary 0.5% differing-pixel and 0.6/255 mean-error limits.
Two narrowly scoped exceptions require additional diagnostic comparisons:

- **500,000-point scenes:** native WebGL point rasterization differs from WebGPU
  triangle expansion. The original renders with approximately 2.8% maximum
  differing pixels at DPR 1 and 1.44% at DPR 2. A diagnostic replacing only the
  original point primitive with instanced GPU quads passes the ordinary limits.
  Native original comparisons must stay below 3.2% and mean error 1.2/255; the
  quad diagnostic is separately required to pass 0.5% and 0.6/255. The original
  geometry, colors, fog, matrices and point-size expressions are retained.
- **20,000-line MSAA:** dense overlapping 1-pixel lines expose WebGL/WebGPU MSAA
  rasterization differences. The original MSAA-on result reaches about 14.6%
  differing pixels at DPR 2, with mean error below 2.8/255. MSAA-off passes the
  ordinary limits. A diagnostic using the official WebGPU renderer with the same
  lines, raw color output and visibility mask produces zero differing pixels in
  both DPR 1 and DPR 2 comparisons. Native MSAA comparisons must stay below 16% and
  mean error 3/255. Both MSAA-off and the WebGPU diagnostic retain the ordinary
  limits. The diagnostic is not listed as an official WebGPU example.

These allowances do not change other examples' limits, particle counts, controls,
resolution or performance requirements. See `buffer-particles-comparison.json`
for the final per-state measurements, residency and draw-workload evidence.

Run after preparing the pinned assets and building release Wasm:

```sh
npx playwright test -c playwright.gallery.config.js buffer-particles.spec.js
PARTICLES_DPR=2 npx playwright test -c playwright.gallery.config.js buffer-particles.spec.js
```

For interactive inspection, open `/web/gallery/#webgl_buffergeometry_points`
or select any of the five entries in the gallery. Reference and diagnostic
fixtures live at `/reference/three-js/buffer-particles.html?id=EXAMPLE_ID`;
`&pointQuads=1` and `&webgpuLines=1` enable the explicitly adapted diagnostics.

Final local validation: 16 browser tests passed at each DPR (108 saved image
comparisons in total); six packaged-site checks passed for the gallery entry and
five examples. Release Wasm compilation, formatting, Wasm type checking and
catalog/asset hash checks also passed.
