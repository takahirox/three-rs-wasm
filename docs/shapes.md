# Furnace and fixed geometry examples

| Example | Runtime behavior |
| --- | --- |
| `webgpu_furnace_test` | 121 physical spheres with varying roughness/metalness, constant PMREM environment, tint toggle |
| `webgl_geometry_convex` | Rotating convex hull, alpha-tested point markers, camera-mounted point light, axes and orbit/pan/zoom |
| `webgl_geometry_nurbs` | Rational curve and control polygon, rational surface and five volume slices, damped horizontal drag |
| `webgl_geometry_text_shapes` | Filled and outlined Helvetiker text, orbit/pan/zoom |
| `webgl_geometry_text_stroke` | Filled and stroked English/Hebrew/Chinese text with M PLUS Rounded 1c, orbit/pan/zoom |

Only the Furnace example has an official WebGPU version in pinned r186. The
other four are compared against the original WebGL renderer. Their output uses
sRGB-before-blending and sRGB-before-MSAA-resolve to match the original path.
Furnace retains linear HDR rendering. Static furnace/text scenes redraw only
on load, interaction and resize; rotating convex and damped NURBS scenes animate.

Fixed geometry is generated offline from the pinned source algorithms and fonts.
The assets preserve individual objects, original indices and draw boundaries.
Rust controls scene transforms, materials, camera, input and GPU rendering. This
does not implement general ConvexGeometry, NURBS, FontLoader or SVGLoader APIs.
These are explicit partial example ports, not complete addon coverage. Static
geometry is never regenerated or streamed per frame. WebGL points use resident
six-vertex GPU billboards on WebGPU; they are not updated per vertex on the CPU.

`tools/gallery/prepare-shapes.mjs` regenerates geometry/assets and provenance.
`tools/tsl/prepare.py` extracts the independent original assets used by the
reference fixture. Source scripts only change asset URLs, deterministic time
and randomness, GUI capture and test-controlled rendering. Geometry is not
replaced in the reference.

```sh
npx playwright test --config playwright.gallery.config.js shapes.spec.js
SHAPES_DPR=2 npx playwright test --config playwright.gallery.config.js shapes.spec.js
```

Tests include original rendering, controls, resize, resource residency and
steady-state draw workload. Point counts are normalized from one WebGL point
to six WebGPU billboard vertices; fullscreen presentation triangles are excluded
from geometry counts. GPU timing parity is not claimed.

## WebGL MSAA comparison

Disabling MSAA on both sides reduces the maximum fraction of pixels differing
by more than 6/255 to 0.1825% for NURBS, 0.0409% for outlined text and 0.0008%
for stroked text (DPR 1, including controls and resize). With 4× MSAA those
maxima are 1.661%, 1.661% and 1.736%, respectively; mean RGB error remains below
0.55/255. This isolates the larger discrepancy to the WebGL/WebGPU sampling and
resolve paths, rather than missing surfaces or glyphs. The suite therefore
requires the ordinary 0.5% threshold without MSAA, and bounds the same three
MSAA-enabled examples to 2.0%, 2.0% and 2.2%, with mean error at most 0.6/255.
Furnace and convex retain the ordinary 0.5% threshold in both modes. Both modes
are tested at DPR 1 and 2; existing example thresholds are unchanged.

Final validation passed: 19 browser tests at each DPR (1 and 2), six packaged
Pages checks, Rust formatting and Wasm Clippy. [Recorded comparisons and draw
workloads](shapes-comparison.json) include all 148 image states across both
sample counts and pixel ratios.
