# Performance is part of reproduction

A matching image or successful import is insufficient. An example is not reproduced
when its implementation moves GPU vertex work onto the CPU, streams unchanged
geometry every frame, substitutes repeated geometry rebuilding for batched draws,
or expands GPU-compressed assets without accounting for memory/bandwidth costs.
Such work is an explicitly incomplete prototype, not parity.

For a reproduction claim, compare the pinned original at equal resolution, sample
count, material quality, assets, object/vertex counts and animated workload. Warm
up both implementations. Record CPU frame work, GPU frame time where available,
frame-time distributions, upload bytes, draw counts and GPU memory. A display-capped
60 fps measurement alone does not establish parity. Hardware-dependent timing
must be reported with hardware/browser details; transfer/residency regressions can
be enforced deterministically in CI. Do not relax visual tolerances to gain speed.

## Current audit

| Path | Status |
| --- | --- |
| Rust TSL graphs | Compiled once into GPU vertex/fragment or fullscreen WGSL. The TSL examples retain geometry, render targets and shader/pipeline/binding resources across warmed frame cycles; only uniforms and changed instance poses are uploaded. Procedural Texture uses the upstream 512×512 HDR checker and two 85-tap blur passes. Added radial blur and FXAA use GPU sampling loops; SSAA repeats instanced scene draws and GPU accumulation at the selected 1–32 sample count. Direct saturation executes inside material shaders. Sprite and galaxy animation uses GPU billboard projection and resident per-instance attributes; 10,000, 20,000 and 50,000 particle scenes each issue one scene draw. Afterimage keeps two HDR history targets and both presentation bindings resident. Typed GPU storage drives 300,000 native point instances and 200,000 billboard particles; HDR ping-pong retains two ten-level GPU mip chains. The Suzanne scene retains static geometry and uploads changed instance matrices, matching the upstream CPU animation architecture. See [storage/instancing measurements](tsl-compute-comparison.json). See [TSL scope and checks](tsl.md). No CPU/GPU timing parity claim. |
| Skin and morph rendering | GPU vertex shader for both color and shadow passes. CPU uploads bone matrices and morph weights only. Geometry, indices and morph inputs remain resident. Native tests compare 28 real-model frames with a CPU oracle, including shadows; browser tests enforce zero geometry reuploads during animation. CPU oracle results are separately checked against original Three.js. |
| Line morph demo | GPU morph weights; no per-frame CPU vertex interpolation. |
| Point Lights demo | Original displacement translated to WGSL, static per-face data in GPU storage. CPU updates light positions/time only. Adapted material remains an appearance limitation. |
| Wide/dashed lines | Resident endpoint geometry; GPU near-plane clipping, screen-width expansion and dash masking. Camera/width changes do not rebuild or upload geometry. Rounded joins remain missing. |
| Geometry buffers | Resident cache keyed by source ownership and attribute versions. Renderer culling bounds are cached by geometry ownership and position/morph versions, avoiding per-frame deep copies and vertex scans. Reupload only after actual source changes. Upload callbacks now run on uploads, not on every draw. |
| Shadow/transmission targets | Reused between frames, reallocated when dimensions/layer/sample requirements change. Opaque and final transmission passes retain independent draw slots; refraction mip views, bindings and pipeline are retained across frames. |
| Per-draw resources | Color, shadow and presentation passes reuse uniform and instance buffers and bind groups; changed contents use queue writes. Resource/layout changes rebuild bindings and removed draw slots are released. Robot browser tests enforce zero steady-state allocations, including after resize. HDR backgrounds and fullscreen effects reuse their uniform buffers and bindings too. Shadow atlas views are retained with their texture. Indirect command buffers are reused until their size changes. |
| Static geometry merging utility | NOT BatchedMesh parity. CPU merge is suitable for one-time static preprocessing only; dynamic GPU batching/culling remains unsupported. |
| KTX2/Basis textures | Browser glTF imports retain Basis payloads and transcode all supplied mips to supported ETC2, BC7 or UASTC ASTC 4×4 blocks at first upload; blocks remain GPU resident. The compressed glTF example is compared against the official WebGPU texture formats, mip counts and upload bytes. Unsupported GPU formats produce an error. `compression::decode_basis` remains an explicit RGBA decode utility, not a compressed-rendering parity path. Compressed physical-extension texture arrays are not yet supported. |
| Physical extension texture arrays | Occupied layers and GPU mip chains are retained; minification/trilinear sampling is supported. Pipeline specialization skips absent extension maps. Mixed-size layer padding and missing anisotropic sampling still prevent a general performance/quality parity claim. |
| Rough refraction/postprocessing | Refraction now uses GPU-generated mip chains and IOR-scaled bicubic sampling, validated on Iridescence/Anisotropy/Transmission scenes. Generic postprocessing interfaces still do not establish complete effect parity. See [physical glTF measurements](gltf-physical-worklog.md). |

There is no blanket FPS/performance parity claim for the renderer. The gallery
continues to identify ports as partial; no example is promoted by this audit.
GPU skin/morph storage limits produce an error, never a hidden CPU render fallback.
CPU deformation remains available for explicit ray queries, exports and test oracles,
matching the distinction between rendering and CPU queries in Three.js.

Run `scripts/check-core` for the GPU, image and residency checks, including
`tests/gpu_deformation.rs` and `tests/browser/gpu-performance.spec.js`.

Native Robot test: 28 poses across 14 clips retained the initial 616,308 geometry/index
bytes and 425,872 skin/morph input bytes without reupload. Aggregate pose uploads
were 308,468 bytes, bounded by joint/weight counts. These counters exclude material
uniforms and are transfer evidence, not an FPS or GPU-memory-parity benchmark.

## Robot frame-time investigation (2026-09-17)

The reported severe slowdown was not reproduced in Chrome 152 on the Apple Metal
adapter at 2560×1600, 4× MSAA. Nevertheless, profiling found 43 buffer allocations
and 22 bind-group creations every frame. Color/presentation draw-slot reuse removes
all of these steady-state allocations without changing resolution, shaders or poses.
CPU callback p50/p95 changed from 0.6/0.8 ms to 0.5/0.6 ms in these short runs;
frame intervals remained approximately 16.7 ms. A separate timestamp-query run
measured scene GPU p50/p95 0.410/0.723 ms and presentation 0.037/0.281 ms.
A separate visible-window Chrome run recorded 598 callbacks in 5 seconds (interval
p50/p95 8.3/9.2 ms), CPU p50/p95 0.4/0.6 ms, and zero steady-state resource creations.
This does not establish the cause or resolution of slowdowns in other browsers,
nor performance parity with original Three.js.

Raw summaries and measurement limits: [robot-performance.json](robot-performance.json).
With the local server running, use `node tools/core/profile-robot.mjs` for a 5-second
warmup and 5-second CPU/API/frame sample. `GPU=1` additionally samples WebGPU pass
timestamps (its diagnostic buffers are included in that run's API counts).
`HEADED=1` uses a visible Chrome window; `DPR` and `URL` can select another workload.

The broader implemented-demo audit and before/after measurements are recorded in
[gallery-performance.md](gallery-performance.md).

## Additional triangle scenes (2026-09-18)

The 160,000-triangle Phong port exposed unnecessary work in the common shader.
Specializing material family, maps, shadow reception and light configuration
reduced its scene GPU median from about 1.43 ms to 0.43 ms without reducing the
workload or image threshold. The 80-byte vertex layout still doubles geometry
memory versus Three in this scene; CPU attributes remain resident and GPU p95 is
also higher. Both new examples remain explicitly partial, not full performance
parity. See [scene validation and measurements](examples-2-worklog.md).

## Twenty additional TSL surface examples

The twenty ports added after the compute/instancing batch keep material nodes,
MRT attachments, Bloom levels and GPU simulation buffers resident. Raging Sea
and Tornado deform vertices in GPU shaders; Jelly updates positions and velocity
with compute dispatches. The 500,000 custom-lit points use one native-point draw
with packed resident positions. Michelle uses GPU skinning. CPU work for pointer
raycasts is an explicit query, as in the original, not per-frame vertex animation.

Bloom retains the original five resolutions and Gaussian kernels, pairing
adjacent taps with bilinear sampling. MRT writes the selected outputs in a single
scene draw and preserves their individual formats. Backgrounds use fullscreen
geometry instead of the upstream background sphere; their appearance is checked
against the original scripts. Fixed ShaderToy source is translated to WGSL, not
evaluated per pixel on the CPU. Teapot geometry and the flame gradient are prepared
once, not streamed every frame.

`tests/browser/tsl-surface.spec.js` checks four times, controls, camera movement,
resize, selective-Bloom clicks, MRT rotation pause/resume, and Jelly release and
re-contact. The Jelly fixture renders the camera before setting a fresh ray,
avoiding the upstream Orbit event's temporarily stale world rotation. Twenty
residency tests check unchanged GPU object creation, resource counts and geometry
transfers after warm-up, including after resize. The workload test records draw,
dispatch and render-pass counts and CPU upload bytes; these are not GPU timing
measurements. Full CPU/GPU timing parity remains unclaimed.

The final capture contains 169 image states; the largest fraction exceeding
6/255 is 0.4078%, below the unchanged 0.5% limit. All twenty warmed workloads
perform zero vertex/index/storage-attribute uploads except the two animated
Michelle scenes, which upload a 4,160-byte bone palette per step. Raw pass counts
retain the extra Rust presentation pass in several postprocessed scenes.

See [TSL surface comparison data](tsl-surface-comparison.json) and
[API scope](tsl.md). The image threshold remains 6/255 per channel and 0.5% of
pixels. Inspector presentation differs from the original and the gallery keeps
these entries marked as partial ports.

## Twenty additional texture, volume and GPU-operation ports

Runtime IDs 78–97 keep static geometry and source textures resident. Perlin/cloud
ray marching, Euler petal animation, 200³ texture generation, point sizing and
structured indirect-draw updates execute on the GPU. The indirect argument buffer
is consumed directly by drawIndirect; no count readback or CPU vertex substitute
is used. Occlusion is an asynchronous GPU query with reused resolve/readback
buffers. Forty canvases share one Rust renderer/device; the multiple-elements
example skips views outside the visible page.

The native cube uploader generates all six mip chains on the GPU. The manual
mipmap example uses the supplied levels; compressed arrays remain compressed.
Array/3D render targets write individual layers directly. The local wgpu patch
forwards WebGPU depthSlice, fixing the upstream 26.0.1 backend omission without
adding an intermediate render/copy path. Background color is clamped before
transparent blending, matching Three.js's unsigned material output.

Image comparisons use pinned upstream scripts with seeded random data and fixed
time. The layer-fill interval is driven by that same test clock. Camera damping
is allowed to settle before capturing; DOF gets one settled render after resize
because the upstream pass updates its size uniform on the following render.
DOM labels retain English language/font metrics, and font antialiasing is
normalized for screenshots. Inspector controls and explanatory overlays are
excluded from scene image comparisons. GUI changes, rotation, pan, scrolling and
resize are otherwise exercised by the actual original handlers and Rust controls.

The workload check records scene draws, GPU dispatches, indirect calls, passes
and CPU attribute uploads. It excludes the original's 5,952-index background
sphere when comparing scene geometry with Rust's fullscreen background quad.
Near/far CoC share one RG16F attachment in Rust; their subsequent Gaussian and
Vogel blur stages remain on the GPU. Extra presentation passes are recorded,
not equated with GPU execution time. Frame-time parity remains unclaimed.

Validation data is saved in [the extended TSL report](tsl-extended-comparison.json).
The existing 6/255 channel threshold and 0.5% pixel fraction limit are unchanged.
All 173 image states pass; the largest differing-pixel fraction is 0.473%.
The browser regression suite passes 164 tests, with two additional checks for
continuous layer updates and GPU workloads. Native GPU regression passes 27
tests, and six image/residency checks also pass at DPR 2. All twenty warmed
Rust workloads upload zero vertex/index/storage-attribute bytes.
