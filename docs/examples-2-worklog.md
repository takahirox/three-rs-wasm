# Two additional triangle examples

Added `webgl_buffergeometry` (160,000 triangles) and
`webgl_buffergeometry_rawshader` (200 triangles), bringing the gallery to
28 runnable **partial** ports. Scene generation and animation run in Rust/Wasm;
rendering and the raw color animation run on WebGPU. No upstream renderer is
loaded by the product pages and no CPU per-frame vertex update is used.

The reference is Three.js commit `148ef33ecb6d2502ff796d4554abd1549c95d519`:

- [buffergeometry source](https://github.com/mrdoob/three.js/blob/148ef33ecb6d2502ff796d4554abd1549c95d519/examples/webgl_buffergeometry.html)
- [rawshader source](https://github.com/mrdoob/three.js/blob/148ef33ecb6d2502ff796d4554abd1549c95d519/examples/webgl_buffergeometry_rawshader.html)

## Behavior and comparison

Both retain the original vertex counts, RGBA attributes, camera, rotation and
transparency. A repeatable seed replaces Math.random in both implementations.
The Phong scene retains its two directional lights, ambient light and fog. Its
back/front passes share a single resident geometry buffer. RawShaderMaterial uses
one double-sided pass, matching its forceSinglePass default. Its GLSL color formula
is translated to WGSL, including source clamping before UNORM blending and display
output without the usual sRGB conversion.

The independent reference constructs the original scene with Three.js. Phong uses
Three's WebGPU renderer; RawShader uses the original WebGL renderer, since its raw
GLSL is not accepted by Three's WebGPU renderer. Phong uses 4x MSAA and RawShader
uses one sample, matching the originals. No image threshold was relaxed: at most
0.5% of pixels may differ by more than 6/255 in any RGB channel. Tests compare three
animation times at 512x512, plus a fourth time at 800x450. Real animation tests
check frame changes and unchanged geometry, deformation and texture counters.
Gallery integration tests cover selection, resizing and navigation away.

Stats and the original information overlays are not reproduced. The original
Phong example releases CPU attribute arrays after upload; this port retains them
under the current Core ownership model. These omissions and the resource differences
below mean these are not full compatibility/performance-parity claims.

## Renderer work required by the larger scene

Initial GPU scene median was about 1.43 ms versus about 0.29 ms for Three on this
machine. The common shader executed unnecessary material branches. Pipeline
specialization now accounts for material family, physical extensions, map presence,
shadow reception, and light count/types. Material values and animation remain
uniform updates; no pipeline is rebuilt per frame for those values.

The optimized scene median was about 0.43 ms. A static-vertex specialization was
also investigated, did not explain the bottleneck, and is not included.
Existing PBR, skin/morph, shadow, instancing, line and material comparisons pass
with the shared shader change.

## Final measurements and limits

Chrome 153 / Apple Metal, 512x512 at DPR 1; 3-second warmup and 3-second sample per
runtime. CPU/API and GPU timestamp runs are separate. GPU-frame figures sum all
measured passes, including presentation; they are not display-capped frame rates.

| Scene / runtime | CPU callback p50 / p95 (ms) | GPU frame p50 / p95 (ms) | Logical GPU buffer bytes |
| --- | ---: | ---: | ---: |
| Phong / Three WebGPU | 0.20 / 0.30 | 0.336 / 0.775 | 19,200,868 |
| Phong / Rust WebGPU | 0.20 / 0.30 | 0.442 / 1.639 | 38,412,832 |
| RawShader / Three WebGL | 0.10 / 0.20 | 0.029 / 0.160 | 9,600 |
| RawShader / Rust WebGPU | 0.20 / 0.30 | 0.025 / 0.121 | 54,432 |

All runs retain geometry and textures through animation, with no steady-state
GPU buffer, texture, bind-group or pipeline creation in the uninstrumented runs.
Phong has two scene draws plus presentation in both WebGPU implementations.
RawShader has one scene draw; Rust additionally presents its render target.

The common 80-byte vertex layout remains larger than Three's scene-specific
attributes, approximately doubling geometry memory in the Phong scene. The CPU
attribute arrays also remain resident. The larger GPU p95 and memory footprint
are unresolved parity limitations, not a successful full reproduction. No claim
of scaling or hardware-independent timing parity is made. Logical buffer counts
exclude textures/driver overhead; texture descriptors and all timing distributions
are retained in the reports. Timestamp readback allocations belong to profiling.

Raw reports:
[Phong CPU/API](webgl_buffergeometry-performance.json),
[Phong GPU](webgl_buffergeometry-gpu-performance.json),
[RawShader CPU/API](webgl_buffergeometry_rawshader-performance.json),
[RawShader GPU](webgl_buffergeometry_rawshader-gpu-performance.json).

## Validation

- `npx playwright test --config playwright.gallery.config.js`: 66 passed.
- `npx playwright test --config playwright.core.config.js`: 23 passed.
- `cargo test --locked --test gpu --test gpu_deformation --test materials --test pbr --test instancing --test batching_lines -- --test-threads=1`: 19 passed.
- Native all-target and wasm Clippy, formatting, release Wasm build, generated
  catalog/source audit and runtime report consistency passed.

Use the Rust toolchain selected by rustup, as in `scripts/check-gallery`.
With the local demo server running on 8173, measurements can be repeated with
`EXAMPLE=webgl_buffergeometry node tools/core/profile-expanded.mjs` (or
`EXAMPLE=webgl_buffergeometry_rawshader`). Add `GPU=1` and a separate `OUT` file
for timestamp measurements. Run each measurement without other GPU tests.
