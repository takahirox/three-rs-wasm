# Implemented demo performance audit

Chrome 152, Apple Metal-3 adapter, 1280x800 CSS pixels at DPR 2 (2560x1600), unchanged Rust demo sample counts and workloads, sequential cases; 3 seconds warmup, 3 seconds measurement.

CPU timings are animation-frame callback durations. GPU timings below are scene-pass
durations from a separate timestamp-query run; presentation is recorded separately in
the raw report. No resolution, MSAA, shader quality, animation or asset was reduced.

| Example | CPU p50 before → after (ms) | CPU p95 after (ms) | GPU scene p50 / p95 (ms) | Buffer / bind-group creations after |
| --- | ---: | ---: | ---: | ---: |
| `webgl_animation_skinning_morph` | 0.5 → 0.3 | 0.5 | 0.410 / 0.527 | 0 / 0 |
| `webgl_geometry_cube` | 0.2 → 0.2 | 0.3 | 0.036 / 0.254 | 0 / 0 |
| `webgl_interactive_lines` | 0.7 → 0.6 | 0.7 | 0.099 / 0.454 | 4 / 47 |
| `webgl_interactive_raycasting_points` | 2.0 → 0.9 | 1.0 | 0.268 / 0.575 | 0 / 0 |
| `webgl_loader_gltf` | 0.4 → 0.2 | 0.3 | 0.868 / 1.457 | 0 / 0 |
| `webgl_materials_texture_rotation` | 0.2 → 0.2 | 0.3 | 0.038 / 0.404 | 0 / 0 |
| `webgl_pmrem_equirectangular` | 1.8 → 0.3 | 0.4 | 0.735 / 1.173 | 0 / 0 |
| `webgl_pmrem_test` | 0.9 → 0.3 | 0.4 | 1.038 / 1.593 | 0 / 0 |
| `webgl_panorama_equirectangular` | 0.2 → 0.2 | 0.3 | 0.063 / 0.215 | 0 / 0 |
| `webgl_buffergeometry_lines` | 0.2 → 0.2 | 0.3 | 0.162 / 0.289 | 0 / 0 |
| `webgl_buffergeometry_lines_indexed` | 0.2 → 0.2 | 0.3 | 0.033 / 0.086 | 0 / 0 |
| `webgpu_equirectangular` | 0.2 → 0.2 | 0.2 | 0.123 / 0.795 | 0 / 0 |
| `webgpu_lights_pointlights` | 0.2 → 0.2 | 0.4 | 0.333 / 0.715 | 0 / 0 |
| `webgpu_loader_gltf` | 0.4 → 0.2 | 0.3 | 0.864 / 1.571 | 0 / 0 |
| `webgpu_pmrem_equirectangular` | 1.8 → 0.3 | 0.4 | 0.728 / 1.548 | 0 / 0 |
| `webgpu_pmrem_test` | 0.9 → 0.3 | 0.4 | 1.027 / 1.546 | 0 / 0 |
| `BoomBox` | 0.3 → 0.2 | 0.3 | 0.148 / 0.464 | 0 / 0 |
| `mvp_perspective` | — → 0.2 | 0.3 | 0.018 / 0.075 | 0 / 0 |
| `mvp_orthographic` | — → 0.2 | 0.3 | 0.043 / 0.289 | 0 / 0 |

Creation counts cover the complete 3-second CPU/API sample (approximately 180 frames).
The interactive-lines scene changes visibility; draw-slot reassignment still creates
a small number of resources. It is not a zero-allocation claim for all dynamic scenes.

## Findings and fixes

- Static culling recomputed bounds on a deep clone of the geometry every frame. Bounds now remain cached until geometry ownership or position/morph versions change. World transforms still apply each frame.
- HDR backgrounds allocated a uniform buffer and bind group each frame. They now reuse both, uploading changed camera/options only.
- Indirect command buffers are also retained and only updated when commands change.
- Shadow draws and fullscreen effects had the same resource churn. They now reuse draw resources; shadow atlas and attachment views are retained until dimensions/layers change.
- All 19 cases retained geometry/index and skin/morph source data without reupload during the GPU sample. Material texture upload/filter counters and resident texture counts remained unchanged.
- No per-frame texture creation, pipeline creation or texture uploads were observed after warmup. Compute dispatch already reuses its pipeline/bindings; explicit readback is separate from rendering.

## Limits

- Steady-state audit of 16 runnable gallery entries, BoomBox and two initial MVP demos (19 cases); several gallery entries share a Rust implementation. Before/after paired measurements cover the first 17 cases.
- Short local runs, not a large-scene stress test or an original Three.js parity benchmark.
- GPU timestamps use a separate instrumented run; its 240 readback/resolve buffer creations per case are measurement overhead, not renderer allocations.
- CPU/API runs do not enable GPU queries; timings are observations, not portable CI thresholds.
- Shadow/effect/compute/indirect paths were additionally reviewed and covered by native regression tests, not by these gallery timing measurements.

Known incomplete compressed-texture residency, physical-extension array memory costs,
dynamic batching and approximate effects retain their existing limitations in
[performance-parity.md](performance-parity.md). This audit does not promote them to parity.

[Raw measurements](gallery-performance.json). Reproduce with a local server and
`OUT=/tmp/gallery-cpu.json node tools/core/profile-gallery.mjs`; run again with
`GPU=1 OUT=/tmp/gallery-gpu.json` for GPU timestamps. Never run GPU workloads in parallel
when collecting comparison samples.

Validation after the fixes: `scripts/check-core` passed (native semantic/GPU tests,
28 skin/morph oracle frames with shadows, clippy on native/Wasm, 23 browser tests);
`scripts/check-gallery` passed (32 browser tests). Native GPU/PBR tests passed,
including culling after geometry/world-transform changes. The 5 HDR/PBR/lifecycle
browser tests passed, including 20 overlapping model replacement cycles.
