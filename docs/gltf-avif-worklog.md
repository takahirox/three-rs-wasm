# Forest House / glTF AVIF port

The gallery adds `webgl_loader_gltf_avif` from the pinned Three.js r186 revision.
The original Forest House GLB is unchanged: Draco geometry, 12 AVIF textures,
unlit materials, alpha blending and alpha masking. Asset provenance and the
CC BY-NC 4.0 license are in [THIRD_PARTY.md](../web/THIRD_PARTY.md).

The scene uses the original camera, background, OrbitControls gestures and
render-on-change behavior. Geometry stays on the GPU. AVIF images are decoded by
the browser and uploaded directly from ImageBitmap to WebGPU, preserving RGB under
transparent pixels; a Canvas roundtrip loses this information and changes mip
filtering. Shared bitmap ownership closes each bitmap when its last texture is
released. There is no CPU pixel readback or per-frame geometry upload.

Renderer changes needed by the port:

- Optional shader-side sRGB encoding before blending into an unorm target, matching
  the original WebGL canvas; existing linear/HDR targets keep their defaults.
- Back-face then front-face rendering for transparent double-sided materials,
  sharing uniforms and geometry between passes. `force_single_pass` retains the
  Three.js ShaderMaterial default of a single pass.
- Browser on-demand frame scheduling and explicit device release when leaving the
  example. Existing animated examples continue requesting frames.

## Validation

`tests/browser/gltf-avif.spec.js` compares independent original GLTFLoader/Draco
output against Rust Draco import and WebGPU rendering. It covers initial view,
rotation, pan, zoom and a 640×400 resize, with matched 1× and 4× sampling.
The 1× run compares every RGB channel directly (6/255 threshold, at most 0.5%
different pixels). WebGL and WebGPU have different subpixel MSAA coverage; the 4×
run applies the same channel/fraction thresholds to 3×3 area averages and also
records the unfiltered fraction. Production rendering remains 4× MSAA. This does
not claim identical antialiasing between the two APIs.

The resource test enforces no GPU submissions or writes while idle; no geometry
reuploads, texture uploads or resource allocations while orbiting; exactly 12
native-image uploads; and image/device release on exit.

Run after preparing the pinned reference and building release Wasm:

```sh
python3 tools/gallery/prepare_avif.py
npx playwright test --config playwright.gallery.config.js
```

Performance is measured separately at 512×512, DPR 1, 4× MSAA, with forced redraws
of the same static camera for both renderers (3-second warmup, 3-second sample).
Normal idle behavior is tested separately. Forced redraw is a diagnostic workload,
not the example's normal continuous animation. The profiler records CPU work,
GPU timestamps when available, draw calls, buffer writes and logical allocation
sizes. WebGL texture/driver memory is not instrumented, so total-memory parity
cannot be inferred from these data.

```sh
EXAMPLE=webgl_loader_gltf_avif OUT=docs/gltf-avif-performance.json node tools/core/profile-expanded.mjs
GPU=1 EXAMPLE=webgl_loader_gltf_avif OUT=docs/gltf-avif-gpu-performance.json node tools/core/profile-expanded.mjs
```

The gallery still labels the port partial: information overlays differ from the
original, MSAA coverage is backend-dependent, and broad performance parity is not
claimed. Browsers need native AVIF decoding as well as WebGPU support.

## Recorded results (2026-09-19)

Chrome 152 on Apple Metal 3, 512×512, DPR 1, 4× MSAA:

| Metric | Original Three.js WebGL | Rust/Wasm/WebGPU |
| --- | ---: | ---: |
| CPU frame p50 / p95, without GPU probes | 0.20 / 0.30 ms | 0.20 / 0.30 ms |
| GPU frame p50 / p95, separate probe run | 0.049 / 0.255 ms | 0.044 / 0.231 ms |
| Draw calls per frame | 14 | 15 (14 scene + presentation) |
| Scene triangles | 11,914 | 11,914 |
| Logical buffer allocation | 513,040 bytes | 1,162,400 bytes |
| Steady-state geometry/texture uploads | 0 | 0 |
| Steady-state WebGPU resource creation, uninstrumented run | — | 0 |

These short, fixed-camera runs show no large frame-time regression in this scene;
they do not establish performance parity across hardware or interactive workloads.
Rust's wider vertex format and uniform buffers still use about 0.62 MiB more
logical buffer memory. Both use the same twelve 1024×1024 uncompressed textures
with mipmaps (about 64 MiB of material texture storage); neither AVIF nor Draco is
a GPU-resident compression format. GPU memory numbers exclude driver overhead.
The extra WebGPU presentation pass is included in its GPU frame times.

Raw reports: [CPU/resources](gltf-avif-performance.json),
[GPU timing](gltf-avif-gpu-performance.json), [visual checks](gltf-avif-visual.json).
Direct 1× image differences range from 0.0088% to 0.0145%; 4× area-average
differences range from 0.277% to 0.409%, with raw MSAA differences of 0.91–1.26%.

Final verification: gallery image/behavior cases and the corrected raw-shader
regressions passed (68 cases in the full run, then all 7 affected/new cases in the
focused rerun); all 23 Core browser tests passed; the AVIF example passed from the
packaged `/three-rs-wasm/` site path. Native compression/material/render-state tests
passed (4 tests), native and Wasm clippy passed, and gallery inventory consistency,
formatting and whitespace checks passed. The release Wasm bundle was rebuilt.
