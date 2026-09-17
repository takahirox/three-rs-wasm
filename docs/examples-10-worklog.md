# Ten additional example scenes

The requested scope is **10 additions** beyond the previous 16 runnable partial
ports, for 26 gallery entries. The selected IDs and evidence are tracked in
`examples-10-progress.json`. Runtime implementations remain Rust/Wasm/WebGPU;
upstream JavaScript is used only by independent reference pages and test tools.
The gallery retains its `partial` classification: Stats/Inspector and some
per-example GUI styling are not complete.

| Official example | Implemented scene and behavior |
| --- | --- |
| `webgl_geometries` | All 16 original shapes, UV texture, Phong lighting, animated camera and transforms |
| `webgl_morphtargets` | Segmented cube, GPU spherical/twisted morphs, sliders, orbit/pan and disabled zoom |
| `webgpu_morphtargets` | Same original morph scene and controls; Inspector UI remains incomplete |
| `webgl_lines_dashed` | Hilbert/Catmull-Rom curve, box edges, cumulative dash distance, fog and rotation |
| `webgl_lights_rectarealight` | Rotating RGB area lights, LTC shading, helpers, checker roughness floor, torus knot and Orbit controls |
| `webgl_geometry_colors` | Three colored icosahedra, wireframes, shadow textures and pointer camera |
| `webgl_buffergeometry_indexed` | Indexed color grid, hemisphere light, rotation and wireframe toggle |
| `webgl_lines_colors` | Six independently colored Hilbert/Catmull-Rom lines, rotation and pointer camera |
| `webgl_morphtargets_horse` | Original Horse asset, GPU morph animation, one-second clip loop, flat normals and orbiting camera |
| `webgl_morphtargets_sphere` | Original AnimatedMorphSphere asset, GPU mesh/point morphs, disc sprites and Orbit controls |

## Correctness and resource checks

The image threshold remains unchanged: at most 0.5% of pixels may differ by more
than 6/255 in any RGB channel. Multiple times/morph weights are compared, plus
independent pointer, morph slider, orbit/pan and wireframe behavior checks.
Geometry and morph source buffers must remain resident during animation; only
transforms and morph weights are uploaded. Gallery tests also cover real catalog
routing, resize, navigation away and absence of upstream renderer imports.

References use the pinned Three.js revision
`148ef33ecb6d2502ff796d4554abd1549c95d519`. Render size is 512×512 at DPR 1. Horse
and MorphSphere use one sample, matching their originals; other additions use
four. MorphSphere is compared with the original **WebGL2** renderer, since Three's
WebGPU Points implementation only supports one-pixel native points. The Rust
implementation expands sprites in the GPU vertex shader. All other references
use Three's WebGPU renderer with the original scene/material workload.

The separate instanced Helmet attempt remains **unlisted and uncounted**: full
PBR comparison still exceeds the threshold. Run its investigation explicitly with
`EXPANDED_PENDING=1 npx playwright test --config playwright.expanded.config.js --grep gltf_instancing`.
The unresolved comparison is not presented as a successful reproduction.

## Core changes exercised by these scenes

- Procedural geometry builders and segmented boxes, compared with 14 upstream
  geometry cases (positions, normals, UVs, index order and groups).
- Catmull-Rom curves: centripetal/chordal/uniform, open/closed, repeated points and endpoints.
- Changed-range draw-uniform writes: the geometry scene fell from 100,352 to
  7,616 bytes/frame without adding queue calls.
- Native one-pixel dashed GPU lines when lineDistance is provided.
- Packed GPU morph storage for present attribute streams only. The morph cube's
  static deformation data fell from 1,045,440 to 209,088 bytes. Sparse stream and
  real Robot skin/morph/shadow cases are covered by GPU tests.
- glTF primitives without normals use GPU flat shading, preserving indexed
  geometry and correctly shading position-only morphs after deformation.
- Point sprite UV orientation matches Three; authored point UVs are preserved.
  An asymmetric texture regression covers both cases and resource residency.
- glTF node TRS retains double precision until upload; required
  EXT_mesh_gpu_instancing is accepted by the animated importer. This importer
  support alone does not qualify the deferred Helmet scene for the gallery.

## Performance evidence and limits

Each `docs/<example>-performance.json` contains a separate three-second warm-up
and three-second CPU/API sample. `-gpu-performance.json` contains GPU timestamp
measurements. The two cube morph entries share one runtime and measurement.
Reports record browser/adapter, draw calls, transferred bytes, logical buffers,
texture descriptors, and resources before/after animation. GPU query/readback
allocations belong to instrumentation and are excluded from allocation claims.
WebGL buffer and uniform APIs differ from WebGPU queue writes; MorphSphere's
backend is explicitly recorded rather than treating those API counts as equivalent.

All measured Rust scenes retain geometry/morph sources and textures in steady
state. CPU p95 was 0.3–0.4 ms on this machine. GPU time is **not equal to Three.js**:
Rust median complete-frame GPU measurements ranged from about 0.034 to 0.110 ms,
versus roughly 0.026 to 0.075 ms for the references. These small workloads do not
establish scaling parity or a renderer-wide performance claim. Full distributions
are retained in the JSON reports.

The universal vertex layout and cached wireframe/point expansion consume more
memory than specialized Three.js buffers. For MorphSphere, Rust logical buffers
are about 1.32 MB; Three WebGL buffers are about 0.081 MB **plus morph textures**.
GPU sprite expansion is necessary for point sizes above one pixel in WebGPU, but
this resident CPU-side quad duplication is not an acceptable basis for claiming
large point-cloud scaling parity. The 500,000-point examples were not added via
this path. No example uses CPU per-frame skinning or morph vertex evaluation.

## Validation commands

- `npx playwright test --config playwright.gallery.config.js` includes the new
  expanded comparisons and existing gallery/glTF/Point Lights regressions.
- `npx playwright test --config playwright.core.config.js` covers existing browser
  animation, materials and GPU residency checks.
- Native GPU/core tests include `gpu`, `gpu_deformation`, `pbr`, `gltf_animation`,
  `procedural_geometry`, `curves`, `instancing`, `batching_lines`, `materials`, `core`;
  library tests cover changed-range upload alignment.
- Native/wasm Clippy, formatting, gallery build/check, prerequisite audit and
  runtime-report consistency are checked separately.

Final checks on 2026-09-17: 58 gallery browser tests, 23 Core browser tests,
2 additional area-light/MorphSphere orbit tests, 39 native integration tests and
1 native library test passed. Native/wasm Clippy, formatting, catalog regeneration,
source/asset accounting, prerequisite audit and runtime-report consistency passed.
The final catalog was checked to contain exactly the original 16 plus the selected
10 additions.
