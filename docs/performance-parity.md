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
| Shadow/transmission targets | Reused between frames, reallocated when dimensions/layer/sample requirements change. HDR targets with retained color/depth snapshot opaque rendering into the GPU mip chain and resume with transparent/transmissive meshes, avoiding repeated opaque geometry. Encoded outputs, hooks and occlusion queries retain the existing separate opaque pass. Refraction mip views, bindings and pipeline remain resident. |
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

## Five additional TSL examples (runtime IDs 98–102)

Instance Path, Sobel, SMAA, 3D LUT and Parallax UV use GPU vertex/fragment work,
resident textures/instance attributes and the original scene geometry counts.
RoomEnvironment is captured and prefiltered once on the GPU with its six boxes
in one instanced draw. Smoke draws back and front faces separately, as upstream.
SMAA retains all three filter stages and original lookup textures. The nine LUTs
are native 3D textures; color conversion and grading share one fullscreen pass.

[Recorded comparisons](tsl-next-comparison.json) cover 62 image states, GUI
parameters, camera orbit/pan/wheel and resize. All pass the unchanged 6/255 and
0.5% pixel thresholds (largest differing-pixel fraction: 0.088%). The five steady
workloads upload zero vertex/index/storage-attribute bytes and allocate no new
GPU resources after warm-up. Twenty-three browser checks (including existing
PBR/gallery regressions), eighteen native GPU checks, and six DPR 2 image/residency
checks pass. Sobel and LUT still have an additional fullscreen presentation pass;
pass counts and uniform bytes are reported explicitly. GPU frame-time parity and
Inspector styling parity are not claimed, and entries remain partial ports.

## Environment graphs and transparency (runtime IDs 103–107)

Cubemap Mix, Cubemap Adjustments, Box Projected Environment Mapping, Alpha Hash
and Chromatic Aberration use resident GPU textures and the original scene draw
sizes. Environment blending, color adjustment, box-ray intersection, alpha
hashing and channel sampling execute in shaders. CubeCamera renders directly
into six texture-array layers before GPU PMREM filtering. RoomEnvironment is
captured once; SSAA retains the official eight-sample default and 27 instances.
The chromatic scene retains native one-pixel WebGPU points and native grid lines.

[Recorded comparisons](tsl-environment-comparison.json) include 71 image states,
controls, camera orbit/pan/wheel and resizing at the unchanged 6/255 and 0.5%
thresholds. Warmed geometry uploads are zero, and resource counts remain stable
before and after resize. Geometry draw counts and GPU dispatches match the
reference; pass counts and uniform bytes are recorded separately. Chromatic
Aberration retains an additional presentation pass. No GPU timing parity is
claimed; gallery entries remain partial ports and Inspector styling is adapted.

The BPCEM roughness JPEG embeds Adobe RGB. Ordinary TextureLoader-style decoding
now honors its profile; glTF decoding continues to ignore image profiles.
The reference fixture correction for static scene-node uniforms is documented
in [the TSL API notes](tsl.md). It does not change the official shader math.

All 123 browser regression checks, 15 native GPU checks, six DPR 2 checks and
five packaged-site startup checks pass. Two additional checks verify the final
layered capture and workload implementation. Maximum differing-pixel fractions
are 0.322% at DPR 1 and 0.145% at DPR 2; timing parity remains unmeasured.

## PMREM, lightmap, DoF and lensflare ports

The five additions retain static geometry, shader pipelines, bindings and render
targets across warmed animation and resize cycles. Browser checks compare original
mesh draw counts and enforce zero geometry/joint-weight/morph-source uploads.
Bath Day uploads only five 19-joint bone palettes (6,080 bytes/frame); skinning stays
in the vertex shader. Its opaque snapshot removes 11 redundant scene draws, leaving
39 scene draws, matching Three.js. PMREM Cubemap and Lensflares use a fullscreen
background triangle instead of the original 5,952-index sky sphere.

Lensflares preserves the original emissive MRT, five-level HDR Bloom, quarter-size
RGBA8 flare target, full-size HDR RTT copy and two full-size Gaussian passes (19
render passes including presentation, matching the reference). DoF keeps the
original GPU box kernel, depth-based mixing and FXAA. These are workload/residency
checks, not CPU/GPU timing parity measurements. See
[recorded comparisons](tsl-lighting-comparison.json).

## Viewport effects, soft particles and FSR1 (runtime IDs 113–117)

Backdrop, Backdrop Area and Refraction sample resident framebuffer snapshots
between transparent draws. A double-sided material recaptures color before its
front side, while retaining the pre-object depth. The renderer avoids a second
scene traversal or geometry re-render for these captures. Soft Particles retains
50 native GPU instances, four resident random attribute buffers, the 100,000-face
Lucy mesh and the original GPU depth-fade/UV/position/scale expressions.

FSR1 uses the original low-resolution MSAA HDR scene, full-resolution 12-tap EASU
and 5-tap RCAS passes. Its Bilinear mode bypasses those two kernels. Same-input
HDR comparisons check the filters separately from glTF rendering. Camera damping
and the original distance bounds are retained for the smoke and upscaling scenes;
Backdrop pauses portal rotation during dragging.

Warmed frame tests require unchanged geometry transfers and stable GPU resource
counts, including after resizing. Michelle updates one 65-joint palette (4,160
bytes); Littlest Tokyo updates eight 32-joint palettes (16,384 bytes). These sizes
match the original's uniform bone buffers; the port uses storage bone buffers.
No vertices, joint weights/indices or particle attributes stream every frame.
Mesh draw sizes match the reference, apart from replacing backdrop sky spheres
with fullscreen triangles. The MSAA smoke depth resolve uses one additional
fullscreen pass; Backdrop Area eliminates a redundant identical framebuffer copy.
Pass counts and uniform upload bytes are recorded in
[the comparison report](tsl-viewport-comparison.json). These checks do not establish
hardware frame-time equivalence; the entries remain partial ports.

All 54 image states pass at DPR 1 and DPR 2 without changing the 6/255 and 0.5%
thresholds (maximum differing-pixel fractions: 0.493% and 0.208%). Twenty-three
browser regression checks, ten DPR 2 checks, twenty native GPU checks and six
packaged-site startup checks pass. The ordinary-material four-texture regression
also verifies that adding viewport nodes does not reduce the existing texture
budget for materials that do not use them.

## TSL materials and transparency batch

The five r186 ports added at runtime IDs 118–122 pass fixed-time image comparisons
at DPR 1 and 2, including controls, camera input and resize. Native tests verify
back-lit SSS and weighted OIT draw-order reversal with shared opaque depth.
All five retain warmed GPU resources and static geometry during animation.
Michelle's 30 instances share one 4,160-byte bone palette; Three.js transfers it
as uniforms and Rust as storage. OIT keeps separate accumulation/revealage
attachments; its final composite and canvas presentation are currently separate
(one more fullscreen pass than Three.js). Animated Toon lighting uses 5,660 bytes
of per-draw uniform updates versus the original's 76 shared bytes in the measured
frame. These costs are recorded; no CPU/GPU timing equivalence is claimed.
See [raw image metrics and grouped GPU workloads](tsl-materials-comparison.json).

### Material, contact shadow and instanced line ports

The material and sandbox scenes retain all geometry and textures on the GPU;
vertex displacement samples the transition texture in the vertex shader.
Sandbox points use native one-pixel primitives, matching the original workload.
Contact shadows use one 512-square depth-color pass and two separable Gaussian
passes. Two floor draws with no front-facing fragments are omitted from the
shadow pass. Fat lines use the original 18-index geometry per instance in both
viewports (767 Hilbert segments or 120 icosahedron edges). The solid inset
background uses a six-index quad instead of the official 5,952-index sphere.
Eight line shader programs share one segment buffer and are prepared at startup.
Render pipelines are cached on first use of each variant/target configuration;
subsequent control changes reuse them. Unused shader variants are prepared eagerly.

Appearance, control and resize comparisons, GPU pass/draw workloads and
steady-state residency are recorded in `tsl-primitives-comparison.json`.
These are workload measurements, not claims of CPU/GPU timing parity.
In the measured animated frame, Core's per-draw uniform blocks still upload
more data than Three.js's split/shared uniforms: materials 74,672 vs 1,428 bytes,
sandbox 4,376 vs 68, contact shadows 26,232 vs 528, and each line scene 10,496 vs
288. Geometry/instance/texture uploads remain zero. The port uses 2/2/5/3/3 render
passes respectively versus 2/2/6/6/6; contact shadows fold the explicit clear into
the depth pass, and line viewports share a final presentation. These differences
are retained in the report rather than interpreted as timing equivalence.

### Procedural materials, GPU particles and temporal rendering (IDs 128–147)

Twenty additional r186 examples use Rust TSL graphs and resident WebGPU resources.
The batch covers Portal, MaterialX noise, rough/blurred/recursive reflections,
compute skinning points, procedural wood/terrain/angular slicing, MRT readback,
shadow maps, tree reflection, compute audio, pixelation, snow, rain,
retroreflection, TRAA, TAAU and motion blur. Static geometry and textures are
prepared once. Snow/rain simulation, instanced tree deformation, material noise,
skinning and motion-vector generation execute on the GPU.

The comparison report `tsl-procedural-comparison.json` records images at DPR 1/2,
controls, orbit/pan/zoom, resize, resident resources and original/port GPU workloads.
Temporal scenes also compare 60 consecutive animated frames, sampling frames
1, 16 and 60. TRAA/TAAU fixed-pose comparisons warm 128 frames to align history;
the fixture advances one reference frame for each Rust input/resize redraw so
Halton phases agree. This does not assert identical initial history before the
original renderer initializes its camera projection. Native tests separately
check HDR history initialization, resize, and previous camera/skin/morph motion.
The independently measured Retro DPR 2 noise tolerance is in `example-policy.md`.

Animated storage uploads contain only bone palettes (and previous poses where
required): Portal 17,152 bytes, compute points and blurred reflection 4,160 each,
motion blur 17,152, TAAU 32,768. All other measured frames upload zero geometry or
instance bytes. TAAU resolution changes allocate new history/input targets;
subsequent animation and parameter use at each resolution retain resources.
Compute snow dispatches 1,563 groups, rain 782, and compute skinning points 256.
Audio dispatches 5,614 groups and reads back 1,436,968 bytes per play, matching the
original. Its playback, delay samples and analyser image are tested separately.
MRT's selected 512-square attachment intentionally makes a 1,048,576-byte GPU to
CPU to texture round trip, as the official readback example does.

Pass/draw counts match closely, with separate presentation passes in TRAA,
retroreflection, tree reflection, snow and terrain. Rain uses an additional draw;
several other ports remove redundant background geometry or copies. Core's shared
per-draw uniform blocks remain larger than Three.js's split uniforms: TAAU uploads
334,800 vs 42,984 bytes, MaterialX noise 98,132 vs 2,432, and snow 107,612 vs 260
in the measured animated frame. The shadow atlas allocates layers at the largest
map size even when individual lights use smaller viewports. These overheads are
recorded rather than treated as timing or memory equivalence. These entries remain
partial ports; no measured CPU/GPU frame-time parity is claimed.

Core temporal APIs accept explicit current/previous transforms. The gallery
adapter preserves r186 TAAU's jittered current velocity, unused zero lock history,
and constant sharpness behavior; see `tsl.md` and `taau-r186-velocity.json`.

### Material groups, clipping and texture operations

`tests/browser/material-textures.spec.js` checks five additional pinned WebGPU
scenes. Shared paper geometry keeps its material groups as resident draws.
Clipping executes in fragment shaders (including MSAA coverage); the clipping
scene retains spot shadows and the two sun cascades. Scissored comparison scenes
reuse full-size attachments and resident samplers. Partial updates upload only
4,096 bytes and issue one 32×32 GPU texture copy per update, without replacing
the destination texture or uploading geometry. Warmed resource counts and
geometry transfer counts stay unchanged, including after resize. Measured
geometry draws match the original; total render-pass counts do not exceed it.
The reference fixture advances Three.js's node frame on each callback to avoid
accidentally measuring cached shadows. This is workload evidence, not GPU timing
parity. See [implementation and validation](material-textures.md).

### Furnace and fixed geometry

The furnace/convex/NURBS/text batch compares 121, 4, 8, 28 and 58 geometry draws,
respectively, with the actual official renderer. The four WebGL-only examples
retain their individual objects and original static geometry indices. Only
WebGL point primitives are normalized to six-vertex WebGPU billboards in the
workload comparison; the shader computes their screen-space size. All five
perform zero warmed static-geometry/texture uploads and preserve GPU resource
counts through animation, controls and resize. Furnace and both text examples
are explicitly tested to remain idle until interaction or resize. Fixed addon
generated assets are disclosed in [shapes.md](shapes.md); this is not complete
font/NURBS/convex addon coverage or a GPU timing-parity claim.

The [buffer-particle examples](buffer-particles.md) retain 500,000 / 100,000 /
75,000 instances or 20,000 native lines in one scene draw. Typed resident storage
preserves packed byte colors without per-particle quad attribute duplication.
Tests enforce exact particle storage sizes, zero steady-state geometry/texture
transfers and no warmed GPU resource creation, including after resize. Explicit
line visibility edits upload only their mask and retain the full draw workload.

### Raw shader and raycast selection

The [shader, procedural noise and interactive cube/point ports](interactive-shaders.md)
match the original draw counts: one fullscreen draw per shader scene,
569 / 1,999 frustum-culled cube draws and one instanced draw of 1,538 sprites.
The 2,000 cubes keep individual meshes and materials, as the original does;
hover highlights change only material uniforms. Point sizes live in a resident
storage buffer, written only when the selection changes. The per-frame raycasts
are the original's own CPU queries, not per-vertex rendering work. After a warm
pass, the scenes make zero geometry and texture uploads. Warmed time, control,
pointer and resize cycles create no GPU resources. Per-cube draw records are
created once, when a cube first enters the frustum. No GPU timing parity is claimed.

### Instanced and sprite picking, orientation, atlas and canvas texture

The [picking, orientation, panorama and canvas-texture ports](interactive-objects.md)
match the original draws: one instanced draw of 1,000 icosahedra, three
orientation meshes, six atlas faces, one canvas cube and three sprites. Hover
recoloring changes resident instance colors or sprite uniforms only. Picking
uses the original's CPU queries. Canvas strokes are copied into the resident
texture on the GPU, and its mipmaps are regenerated in place. After a warm
pass, the scenes make zero geometry, attribute and texture uploads. Warmed
control, hover, drawing, drag and resize cycles create no GPU resources. No GPU
timing parity is claimed.

### Voxel painting, triangle picking, clipping, comparison and custom blending

The [voxel, picking, clipping, comparison and blending ports](interactive-scenes.md)
match the original draws: the grid and rollover, the 5,000-triangle mesh and its
outline, fifteen clipped spheres, the two comparison meshes, and 132 blend and
label quads. Voxels share one geometry and one material. Picking uses the
original's CPU queries. Only the hovered triangle's 4-vertex outline is
rewritten, as in the original. The two comparison scenes render into one target
with scissor rectangles. The voxel and clipping scenes stay idle until input or
resize. Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Multiple views, OBB, sorted points and the XYZ/TGA loaders

The [viewport, OBB, sorted-points and loader ports](views-loaders.md) match the
original draws: 27 viewport draws, 100 boxes, 3,120 sorted point billboards, 201
XYZ billboards and two crates. OBB collisions and the point depth sort are the
original's CPU work. The sorted points keep positions and colors resident and
write only 8 bytes of order and size per point each frame. After a warm pass,
the other scenes make no geometry or texture uploads. Warmed cycles create no
GPU resources. No GPU timing parity is claimed.

### Stereo effects and the PCD/ImageBitmap loaders

The [stereo-effect and loader ports](stereo-loaders.md) match the original
draws:

- 126, 230 and 231 culled sky-box and sphere draws for the stereo, anaglyph and
  parallax scenes, whose 500 spheres share one geometry and one material;
- one draw of 59,750 PCD point billboards;
- seven ImageBitmap draws.

The anaglyph and parallax eye targets are rebuilt only on resize. PCD clouds are
parsed and uploaded once and stay resident across file switches; the original
re-uploads on every switch. After a warm pass, no scene makes geometry or
texture uploads. Warmed cycles create no GPU resources. No GPU timing parity is
claimed.

### Orbit/map controls, camera helpers, custom attributes and draw ranges

The [controls, camera-helper, custom-attribute and draw-range ports](controls-attributes.md)
match the original draws: one instanced draw of 500 meshes per controls scene,
eight camera-view draws, the displaced sphere, and the BoxHelper, 500 point
billboards and connected segments.

- **Camera helpers.** Unprojected on the GPU from resident NDC points, instead
  of the original's per-frame CPU rewrite.
- **Custom attributes.** Writes only its displacement attribute each frame,
  33,540 bytes, as the original.
- **Draw range.** Keeps the original's CPU particle and connection work. It
  writes 104,800 bytes in the measured frame, against the original's
  24,012,000.

The other scenes make no geometry uploads after a warm pass. Warmed cycles
create no GPU resources. No GPU timing parity is claimed.

### HDR texture, voxel terrain, trackball controls, sprites and LOD

The [HDR, terrain, trackball, sprite and LOD ports](trackball-sprites.md)
match the original draws:

- one HDR quad;
- one merged terrain draw of 138,084 indices;
- one instanced draw of 500 cones;
- 205 sprite draws;
- 137 culled LOD level draws.

The HDR texture and the terrain are uploaded once. The terrain is merged into
one resident geometry, as the original merges it. The LOD field prepares every
level's draw data in one startup pass, so flying through it creates no GPU
resources. After a warm pass, no scene uploads geometry or texture data.
Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Noise terrain and the GCode, VOX and OBJ/MTL loaders

The [terrain and loader ports](terrain-loaders.md) match the original draws:

- one 390,150-index terrain draw, plus the raycast cone;
- two GCode line draws;
- one greedy-meshed VOX draw;
- 13 OBJ material-group draws.

The raycast uses the original's per-move CPU query. GCode files are parsed once
per asset and stay resident; the original re-parses on every switch. After a
warm pass, no scene uploads geometry or texture data. Warmed cycles create no
GPU resources. No GPU timing parity is claimed.

### MDD, edge split, 3DS, teapot and instance scattering

The [MDD, edge-split, 3DS, teapot and scattering ports](models-modifiers.md)
match the original draws: one morphing box, the edge-split, 3DS and teapot
meshes, and the torus knot with two 2,000-instance flower draws.

- **MDD.** Morphs on the GPU.
- **Edge split and teapot.** Keep every visited geometry and material
  combination resident instead of rebuilding it.
- **Scattering.** Keeps the original's per-frame CPU instance work. It writes
  319,960 bytes of instance transforms and colors each frame, against the
  original's 256,000 bytes of instance matrices.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### BVH, framebuffer texture, float readback, GPU picking and instancing performance

The [BVH, framebuffer-texture, float-readback, picking and instancing
ports](picking-buffers.md) match the original draws and per-frame uploads:

- **BVH.** Evaluates the clip on the CPU, as AnimationMixer does, and writes
  the 1,344 bytes of skeleton-helper positions that the original uploads.
- **Framebuffer texture.** Writes the original's 28,812 bytes of curve colors
  and copies the framebuffer region on the GPU.
- **Float readback and picking.** Read one texel each frame through a resident
  buffer, asynchronously.
- **Instancing performance.** Keeps the original's three methods, including
  the NAIVE per-mesh draws, and rebuilds on every change as the original does.

Warmed cycles create no GPU resources, apart from the instancing rebuilds. No
GPU timing parity is claimed.

### PDB molecules, helpers, simplifier, AMF and TIFF

The [PDB, helpers, simplifier, AMF and TIFF ports](helpers-formats.md) match
the original draws: 49 caffeine meshes, 13 helper draws, the two heads, the
rook and grid, and the three TIFF planes.

- **Helpers.** Computes the normal and tangent helper segments of the static
  head once and keeps them resident. The original re-uploads the same
  445,392 bytes every frame.
- **Simplifier.** Runs the original's meshoptimizer 1.1 routines on the CPU,
  once per ratio.
- **PDB and simplifier.** Keep each molecule and ratio resident instead of
  rebuilding it.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Cube refraction, PLY, KMZ, Collada and EXR

The [refraction, PLY, KMZ, Collada and EXR ports](refraction-loaders.md) match
the original draws, including the PLY models' two shadow-map passes. No scene
uploads geometry or texture data after a warm pass, and warmed cycles create no
GPU resources. The EXR and TIFF decoders run once at load, on the CPU, as the
original loaders do. No GPU timing parity is claimed.

### Cube-mapped heads, STL, extruded shapes, spot lights and hemisphere light

The [cube-map, STL, extrusion, spot-light and hemisphere-light
ports](shapes-lights.md) match the original draws, including each shadow pass.

- **Setup work.** Extrusion, triangulation and STL parsing run once at setup,
  on the CPU, as in the original.
- **Per-frame work.** The spot-light tweens update three lights per frame; the
  flamingo morphs on the GPU.
- **Uploads.** No scene uploads geometry or texture data after a warm pass.

Warmed cycles create no GPU resources, apart from the light toggles, which
rebuild the light bindings. No GPU timing parity is claimed.

### Advanced clipping, spline tubes, text, tessellated text and text lines

The [advanced-clipping, spline-tube, text, tessellation and text-line
ports](text-clipping.md) match the original draws, including the instanced
boxes' two shadow passes.

- **Setup work.** Font outlines, Earcut, extrusion, tessellation and tube
  generation run on the CPU at setup, and on each text or tube change, as in
  the original.
- **Per-frame work.** The clipping planes are recomputed per frame, as in the
  original.
- **Uploads.** The text lines stream their displacement attribute, the same
  1,278,864 bytes per frame the original uploads. No other scene uploads
  geometry or texture data after a warm pass.

Warmed cycles create no GPU resources. The geometry-building controls are left
out of those cycles. No GPU timing parity is claimed.

### Keyframes, drag controls, box selection and the camera array

The [keyframe, drag-control, box-selection and camera-array
ports](selection-views.md) match the original draws.

- **Camera array.** The 36 views share one shadow pass, through the new
  `Scene::shadow_auto_update`.
- **Culling.** Shadow casters are frustum-culled per shadow camera, as in
  Three.js, and shadow draw data stays resident per caster.
- **CPU work.** Picking and selection run as explicit CPU queries on input.

No scene uploads geometry or texture data after a warm pass, and warmed cycles
create no GPU resources. No GPU timing parity is claimed.

### STL, PLY and OBJ exporters, matcap and physical lights

The [exporter, matcap and physical-lights ports](exporters-matcap.md) match the
original draws.

- **CPU work.** The exporters run on the CPU only when their buttons are
  pressed. The EXR decodes once at load.
- **Uploads.** No scene uploads geometry or texture data after a warm pass.

Warmed cycles create no GPU resources. The OBJ scene selection and the shadow
toggle are left out, as they build geometry and light bindings. No GPU timing
parity is claimed.

### Sky, sun light, pointer-lock walk, video panorama and ocean

The [sky, sun-light, pointer-lock, video-panorama and ocean
ports](sky-water.md) match the original draws.

- **Per-frame work.** The sky's six-face cube capture and the ocean's
  half-resolution reflector run each frame, as in the original.
- **Environment.** Moving the sun builds a new PMREM environment, as
  `updateSun()` does.
- **Video.** Frames are copied from the video element only when it presents
  a new one.

Warmed cycles of time and input create no GPU resources. No GPU timing parity is
claimed.

### Clearcoat, fly controls, point-light shadows, light probe and tone mapping

The [clearcoat, fly-controls, point-light-shadow, light-probe and tone-mapping
ports](lights-probes.md) match the original draws, except the tone-mapping
background's fullscreen triangle.

- **Backgrounds.** Raw cube backgrounds are drawn as r186's 32 × 32 background
  sphere.
- **Points.** The fly stars are native one-pixel points, as r186's WebGPU
  Points draw them.
- **Shadows.** Point-light casters are culled per cube face.
- **Light probe.** The SH coefficients are computed once on the CPU, as
  `LightProbeGenerator.fromCubeTexture` does.

Warmed cycles of time, input and resize create no GPU resources. No GPU timing
parity is claimed.

### EXR and KTX2 exporters, video cubes, async compilation and object-space normals

The [exporter, video-cube, async-compilation and object-space normal
ports](exporters-video.md) match the original draws, except the PMREM
background's fullscreen triangle.

- **Exports.** PMREM exports read the atlas back asynchronously from the GPU.
  Data textures are encoded from their resident Float32 source.
- **Video.** The 200 cubes share one program. Frames are copied from the video
  element only when it presents a new one.
- **Compilation.** The 256 distinct programs are created 16 a frame while the
  scene animates, without blocking the first frames, and are shown at 1 s.

Warmed cycles of time, input and resize create no GPU resources. No GPU timing
parity is claimed.

### Shadow meshes, dynamic instancing, depth texture, render to texture and the normal-map composer

The [shadow-mesh, instancing, depth-texture, render-to-texture and composer
ports](shadow-rtt.md) match the original draws, except the depth post pass's
fullscreen triangle.

- **Instances.** The 10,000 instances stream each frame, as the original's
  DynamicDrawUsage matrices do. The port packs matrix and color at 80 bytes an
  instance.
- **Shadows.** ShadowMesh projections are GPU per-vertex, with the matrix
  computed once per object and frame.
- **Composer.** The composer passes are fullscreen effects on resident
  half-float targets.
- **Bind groups.** Draw slots keep their four most recent bind groups.

Warmed cycles create no GPU resources, except the depth texture's
target-recreating controls. No GPU timing parity is claimed.

### Cube-camera probe, HDR environment maps, UltraHDR, transmission and the dungeon

The [probe, environment-map, UltraHDR, transmission and dungeon
ports](probes-hdr.md) match the original draws, except equirectangular
backgrounds drawn with a fullscreen triangle.

- **Probe.** The probe's SH is computed once on the CPU, from the same 8-bit
  values the cube capture would hold.
- **UltraHDR.** The gain map is applied once on the CPU after browser JPEG
  decoding, as UltraHDRLoader does.
- **Environments.** All environments are prefiltered once, and switching
  between them does not refilter.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Composer backgrounds, lava, uniform buffers, RGB halftone and decals

The [composer, lava, uniform-buffer, halftone and decal
ports](passes-decals.md) match the original draws.

- **Passes.** The composer passes run on the GPU in half-float targets, as
  EffectComposer does.
- **Decals.** A decal's geometry is clipped once on the CPU at the click, as
  DecalGeometry does.
- **Helper line.** The pointer's helper line is placed by its transform, without
  rewriting geometry.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Uniform-buffer arrays, Draco, keyframes, material variants and facecap

The [uniform-buffer, Draco, keyframe, variant and facecap
ports](draco-variants.md) match the original draws, except the variant example's
HDR background drawn with a fullscreen triangle.

- **Light arrays.** The 300 light positions stream each frame into a resident
  storage buffer, as the original updates its uniform block.
- **Decoding.** Draco, meshopt and KTX2 decoding happen once at load.
- **Environments.** They are prefiltered once.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Canvas raycast textures, 3D partial updates, cube mips, lens flares and the car

The [canvas texture, 3D update, cube mip, lens flare and car
ports](texture-flares.md) match the original draws.

- **Canvas and cloud updates.** Canvas redraws are copied on the GPU when the
  pointer moves. Cloud blocks are generated on the CPU and written once, as the
  original does.
- **Lens flares.** The occlusion test copies 16 × 16 patches and draws its
  quads on the GPU, as LensflareMesh does.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Mapped spotlight, multiple clones, skinning blends and the walking soldier

The [spotlight and skinning ports](spot-skinning.md) match the original draws,
including every shadow pass.

- **Spotlight map.** The map's projection matrix is computed per frame on the
  CPU, as three updates the shadow matrix.
- **Mixing.** Animation blending runs on the CPU per bone, as three's mixer
  does.
- **Shared skeleton.** The shared-skeleton mode evaluates three copies of the
  skeleton, where the original evaluates one.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.


### Transform controls, curve modifiers, the IK arm and the glTF export

The [transform-controls, curve, IK and glTF export ports](transform-curves.md) match the
original draws, including the IK example's six cube-camera faces.

- **Gizmo.** Its handles are resident. Updates change only transforms and swap
  between resident materials.
- **Curve flows.** The text bends on the GPU from the half-float spline
  texture. The texture and the curve lines are rewritten only on drag end, as
  the original rewrites them.
- **IK.** The solver and `computeBoundingSphere()` run on the CPU, as the
  original's do. Skinning stays on the GPU.
- **Export.** The GLB is assembled only when Export is pressed, with the
  images encoded by the browser canvas, as GLTFExporter encodes them.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Texture volumes, loaders, raycaster helper and audio timing

- **Layers and volume.** Layer transfers copy on the GPU; the float volume is
  uploaded once.
- **NRRD slices.** Repaints threshold and window on the CPU and copy a canvas
  into the texture only on a control change, as the original repaints.
- **Loaders.** Collada, 3MF and TrueType geometry is built once on the CPU at
  load (or on a 3MF switch or text change), as the loaders do. Skinning stays
  on the GPU.
- **Raycaster helper.** The raycast runs on the CPU as the original's
  explicit query. Only the 20 instance matrices are written per frame; the
  unchanged helper lines stay resident.
- **Audio.** Bounce detection and placement run in Rust; the Web Audio graph
  is the browser's, as in the original.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Texture arrays, MD2 characters, audio scenes and HUD overlays

- **Arrays.** Array textures are uploaded once. The render-target example
  renders one layer per frame on the GPU, as the original does.
- **MD2.** All frames stay resident as morph targets; each frame writes only
  the influences, and the GPU morphs the vertices.
- **Audio.** The visualizer uploads its 64 analyser bins each frame, as
  `needsUpdate` does. Placement and settings run in Rust; the browser runs
  the Web Audio graph.
- **Overlays.** The shadow-map HUDs read the resident shadow atlas; the
  caster-free overlay pass keeps it resident.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Stereo, compressed textures, video points, channels and post passes

- **Stereo.** The 500 sphere matrices stream each frame in one instanced
  draw, as the original's instance buffer does; the eye passes and the
  composite stay on the GPU.
- **Textures.** The DDS block textures and mip chains are uploaded once in
  their own formats, without RGBA decoding. The wide-gamut logos are
  converted by the browser once on upload.
- **Video.** Each new Kinect frame is copied once; the 307,200 points are
  placed in the vertex stage, from the instance index, with no position
  stream.
- **Channels and clipping.** Displacement, normal-map frames, depth
  packing, velocity, stencil caps and the shadow mask run on the GPU over
  resident geometry.
- **Post passes.** The glitch and SSAO passes are full-screen triangles over
  resident targets, rebound only on resize; their random textures and the
  SSAO kernel are created once. The ASCII effect reads the frame back each
  frame at 0.15 of its size, as the original does.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### SAO, TAA, outline, texture loaders, depth buffers and fat-line raycasting

- **Post passes.** SAO, TAA and OutlineNode run as full-screen passes over
  resident targets; the outline mirror scenes are warmed at the first render.
- **Points.** The multiple-elements molecules stay resident and are displaced
  in the vertex stage; the original rewrites every position on the CPU each
  frame.
- **Textures.** KTX and KTX2 data is uploaded once as stored. PVRTC levels are
  decoded once at load to RGBA8 (4–8 × the compressed memory), since WebGPU
  has no PVRTC formats; nothing is decoded per frame.
- **Depth.** The reversed-depth planes use one resident vertex buffer and
  three pipelines; the logarithmic depth is written by the built-in fragment
  stage.
- **Queries.** The fat-line raycast is the original's per-frame CPU query; the
  ribbons stay resident on the GPU.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Memory tests, instancing, batching, bundles, marching cubes and dynamic points

- **Instancing.** The 50,000 volume boxes and the Menger volume are uploaded
  once; the ray march runs per fragment.
- **Batching.** The mesh batch keeps its geometry, matrices and colors in
  GPU buffers and issues one indexed draw per visible instance after the
  original's per-frame CPU cull and sort; only changed matrices and the draw
  IDs are uploaded. The render bundle is recorded once and replayed.
- **CPU-rebuilt data.** The marching cubes surface, the memory tests' sphere,
  texture and 100 programs, and the dynamic points' random walk are rebuilt
  on the CPU each frame, as the originals do; the surfaces and points are
  written into resident buffers, and the memory tests dispose what they make.
- **Points.** The dynamic points upload 12 bytes per moved point; the vertex
  stage expands each resident position into its point square.
- **Post passes.** The advanced and dynamic-points composers are full-screen
  draws over resident targets with one uniform buffer; bind groups are
  created once per target set.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Compute, clustered and dynamic lighting, VSM, projectors and transmission

- **Compute.** The birds, both bitonic sorts, the attractors and the linked
  particles run their compute passes over resident storage buffers that the
  render passes read directly; nothing is read back or re-uploaded per frame.
- **Lighting.** The clustered lights are sorted on the CPU into the light
  texture each frame, as ClusteredLighting does (the same bytes as the
  original), and assigned to clusters on the GPU. The dynamic point lights
  fill uniform arrays; adding or removing lights recompiles nothing.
- **Shadows.** The VSM, projector and caustics shadow maps are re-rendered
  each frame, as their originals do, culled against each shadow camera's
  frustum. The transmitted-shadow dragons render their shadow once
  (`autoUpdate` off).
- **Transmission.** The frame copy and its mip chain are GPU copies and
  draws; the transmissive meshes read them on the GPU.
- **Assets.** The PLY, glTF (including Draco) and images are decoded and
  uploaded once, with mipmaps generated once on the GPU. The projector's
  video frames are copied once each.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Cascaded shadows, instanced morphs, volumes, god rays, test memory and TRAA

- **Shadows.** CSM cascades, the cube and spot shadows of the volumetric
  scenes and the god rays' cube shadow are re-rendered each frame, as their
  originals do, each pass culled against its camera.
- **Morphs.** The 1,024 horses' influences are rewritten to their texture each
  frame, as `setMorphAt` and `morphTexture.needsUpdate` do; the 15 morph
  targets stay resident and are blended on the GPU.
- **Volumes and TRAA.** The ray marches, blurs, pre-pass, velocity MRT and TRAA
  resolve run on resident targets, created once per size; history copies are
  GPU copies.
- **Test memory.** The page builds and disposes a sphere and canvas texture
  each frame; so does the port, and its resident set stays flat.
- **WebGL and exporter scenes.** The PCSS and TSL instancing scenes use the
  crate renderer with resident geometry. TSL time is a material uniform. The
  Draco export encodes only on request.

Warmed cycles create no GPU resources beyond test memory's per-frame sphere.
No GPU timing parity is claimed.

### Protoplanet, retro, dynamic cubemap, deferred, cloth, caustic volumes, HDR, fluid and light probes

- **Simulations.** The protoplanet's GPUComputationRenderer pair, the cloth's
  Verlet buffers and the fluid's particles and grid stay on the GPU. Each
  requested frame runs their passes, as the originals do per animate(); the
  fluid writes its own indirect-dispatch counts.
- **Per-frame passes.** The dynamic cubemap's six faces and PMREM, the
  light-probe scenes' point shadows and the caustic volume's mipmapped frame
  copies run every frame, as in the originals, on targets created once per
  size.
- **Bakes.** The light-probe grids bake on the GPU at load and on a
  resolution change, with no readback. A rebake allocates new grids, as
  LightProbeGrid does; the residency cycle leaves that parameter out.
- **Assets.** Models, HDR and UltraHDR environments and textures are decoded
  and uploaded once. Retro's helmet loads on first selection and stays
  resident.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### TSL graph, compute water, SSGI, SSS, SSR, fog scattering, backdrop water, volumetric fog, skinning instances and retargeting

- **GPU state.** The water height field and ducks, and the thirty skinned
  instances' vertices, stay on the GPU. The instances are skinned in one
  compute pass, as in the original.
- **CPU work as in the originals.** Animation sampling (thirty mixer poses per
  frame for the instances), the SunLight cascade fitting and the bone and
  instance matrix uploads run each frame. The trees, the tri-noise volume and
  the retargeted clip are generated once at load.
- **Per-frame passes.** The cascades' shadow atlas, the reflector's mirrored
  view and the backdrop's color and depth copies run every frame on targets
  created once per size.
- **Variants.** SSR's refinement and blur variants are built at load, and the
  volumetric fog keeps the targets of each resolution scale it visits. SSS
  rebuilds its pipeline with the frame ID baked in, as SSSNode does, so its
  residency cycle leaves the parameters out.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Gaussian splats, water, custom fog, VXGI, GTAO, shadow-map array, compute rasterizer, building generator, loft geometry and sculpting

- **GPU state.** The splat sort, the VXGI opacity and radiance volumes, the
  compute rasterizer's work queues, indirect arguments and visibility buffers,
  and the shadow-map array stay on the GPU. The splat sort runs only when the
  view turns, and the radiance volume is re-injected only when its inputs
  change, as in the originals.
- **CPU work as in the originals.** The splat decoding, the terrain and
  forest, the voxelization triangles, the teapot LODs, the loft geometry and
  the skyscraper are generated once, or when a parameter regenerates them. The
  SunLight cascade fitting and BatchedMesh's tree sorting run each frame.
  Sculpting runs the Sculptor addon's strokes on the CPU, as the addon does.
- **Uploads.** No vertex or index data is written in steady frames. A
  sculpting stroke uploads only the addon's attribute update ranges and, when
  the topology changes, the index. Buffers are recreated only when the addon
  reallocates its arrays.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Compute rasterizer IBL, progressive shadow map, Ready Player Me retargeting, SSR denoise, simple GI, DoF 2, subsurface scattering, GPGPU birds, VRML and USDZ

- **GPU state.** The compute rasterizer's queues, visibility buffers and
  depth pyramid, the progressive lightmap, the SSR, reprojection, denoise and
  TRAA histories, the simple GI vertex colors and the birds' simulation
  textures stay on the GPU.
- **CPU work as in the originals.** The retargeted clip, the helmet LODs and
  the potpack layout are built once. The SunLight cascade fitting, the scenes'
  culling and ordering, and the DoF raycast autofocus run each frame.
- **CPU work moved to the GPU.** SimpleGI's per-vertex readback and byte sum
  run in a compute pass that writes the vertex colors in place.
- **Uploads.** No vertex or index data is written in steady frames. The birds
  draw one instanced bird where the page builds a merged geometry. The page's
  per-bird values are the same in both.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### USDZ exporter, shadow-map performance, GTAO, subdivision, BVH raycasting, spline editor, glTF exporter, arcball, video frame, G-code exporter, SVG, FBX, FPS game, random UV, transmission alpha, webcam, 3DM, IFC, watch and Lottie

- **GPU state.** The shadow maps ( PCF cascades, VSM moments and their
  blurs ), GTAO's normal, AO and denoise targets, the transmission mip chain,
  the TSL surface programs and every model's geometry stay on the GPU.
- **CPU work as in the originals.**
  - Each frame: the horses' morph weights, BVH ray casts, the FPS physics
    substeps, the ArcballControls and tween cameras, and the Lottie canvas
    drawing.
  - On changes or at load: Loop subdivision on parameter changes; the spline
    outlines when a point moves; the SVG shapes and strokes; the IFC merge.
- **Uploads.**
  - Unchanged geometry is not written in steady frames.
  - The BVH rays' instance matrices and line positions are written each
    frame, as the page updates them.
  - Video, webcam and Lottie frames are copied into their textures when a
    new frame is presented, as VideoTexture, VideoFrameTexture and
    CanvasTexture upload them.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### OffscreenCanvas worker, volume fire, batched LOD BVH, LDraw and CSG

- **GPU state.**
  - The fire's eight voxel grids and the compute kernels stay on the GPU.
    Each simulation step dispatches the page's seven kernels.
  - The batched knots' matrices and colors are resident instance buffers.
  - The LDraw parts' geometries are shared by every instance of a part.
  - The CSG result's buffers are reused at their grown capacity.
- **CPU work as in the originals.**
  - Each frame: the batch's BVH culling, LOD selection and sort; the CSG
    evaluation.
  - On pointer moves: the batch's hover ray casts and the fire's DragControls
    ray cast.
  - At load and on reloads: the LDraw parse.
- **Uploads.**
  - Steady frames write no geometry for the fire, the batch or LDraw.
  - The batch writes the visible instances only when the culled set changes,
    where the original rewrites its indirect texture every frame.
  - The CSG result streams each frame, as the original's does. Its 80-byte
    interleaved vertex makes the stream up to 2.5 times WebGL's.
- **Draws.** The batch draws one instanced draw per knot LOD where WebGL issues
  one multi-draw entry per instance; the drawn instances match one for one.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### City generator

- **GPU state.**
  - The towers, sidewalks and furniture are resident after a build. Each
    tower is one draw; each furniture generator is one instanced draw with
    its matrices in a uniform array, as r186's InstanceNode keeps them.
  - The probe bake runs on the GPU: per probe the six 16² captures and the
    SH projection, then the rows' repack into the atlas. The bounce pass
    snapshots the atlas with a texture copy.
- **CPU work as in the original.** The generators run when the seed changes;
  the light-space shadow fit and the PMREM bake when the time of day does.
- **Uploads.** Steady frames write uniforms only.
- **Draws.** The scene, shadow and probe-capture draws match the original's
  one for one.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### Compute reduce and MaterialX loader

- **GPU state.**
  - The reduction buffers stay resident. Each step dispatches the selected
    algorithm's kernels with the page's workgroup counts, and the
    validation and reset run on the GPU. Only the log buttons read a buffer
    back.
  - The ShaderBall's de-indexed attributes, the documents' mipmapped images
    and the PMREM are uploaded once. The transmission copy and its mipmaps
    run on the GPU when a transmissive ball is visible.
- **CPU work as in the original.** The MikkTSpace tangents are computed once
  at load; each frame culls and sorts the 72 meshes.
- **Uploads.** Steady frames write uniforms only.
- **Draws.** The MaterialX frame's draws and passes match the original's
  one for one.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.

### WebGL pages without a WebGPU counterpart

- **GPU state.** The scenes are resident. The split screens render each
  composer's targets on the GPU; the afterimage history ping-pongs between
  two resident targets; the mirrors' reflections render into resident
  targets, three per mirror.
- **CPU work as in the original.** The reflection cameras and their oblique
  projections, per render.
- **Uploads.** Steady frames write uniforms only.
- **Draws.** One scene render per composer, and one per Reflector
  onBeforeRender, as on the pages.

Warmed cycles create no GPU resources. No GPU timing parity is claimed.
