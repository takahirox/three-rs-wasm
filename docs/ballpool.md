# webgpu_postprocessing_ssgi_ballpool

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source |
| --- | ---: | --- |
| `webgpu_postprocessing_ssgi_ballpool` | 484 | `src/browser/ballpool.rs`, `src/bounce/` |

The page fills a box with balls and steps them with the
[@perplexdotgg/bounce](https://www.npmjs.com/package/@perplexdotgg/bounce)
1.8.1 rigid-body engine (and monomorph 2.3.1, its object pools). A
shadow-casting point light follows the pointer. The frame runs the
webgpu_postprocessing_ssgi pipeline: the MRT scene pass, SSGI with 2 slices
and 8 steps, the composite and TRAA. Then ACES Filmic tone mapping with
exposure 0.5.

## Physics: `crate::bounce`

`src/bounce/` ports the parts of bounce the page reaches:
- the world and its options;
- box and sphere shapes, static and dynamic bodies, respawns and linear
  impulses;
- `advanceTime` and the step;
- the BVH broadphase, GJK and EPA, the contact manifolds, the warm-started
  sequential impulse solver, the position solver and sleeping islands.

The port is written operation by operation from the package's build. It
keeps monomorph's pool behavior:
- free slots are reused last-in first-out;
- live objects iterate in index order;
- reference lists persist on reuse;
- constructor defaults mutate the options object.

It also keeps the package's quirks, such as the old-manifold index lookup in
`addContactConstraint`. JavaScript's `Math.min` / `Math.max` NaN and −0
semantics and stable sorts are written out.

Two runtime dependencies are not part of the package's code:

- **`Math.sin` / `Math.cos`.** Quaternion integration and the solver's
  rotation deltas call them. Browsers differ in the last bit. Chrome's V8
  uses LLVM libc's routines, which are not correctly rounded for every input
  and depend on the CPU's FMA support. The Wasm build therefore calls the
  host's own `Math.sin` / `Math.cos`. The page's other `Math.tan` and
  `Math.exp` calls that feed the bodies (the camera fit and the pointer
  easing) do the same. The native build, used only by unit tests, uses
  correctly rounded double-double sin and cos.
- **`performance.now()`.** `advanceTime( step, time )` with a falsy `time`
  simulates the time since its first call. While that first call's time is
  falsy too (0 at the fixture's time 0), it simulates one step. The page's
  timer gives 0 whenever two frames share a timestamp. The port takes the
  clock as an argument: the example clock in the gallery, and the fixture
  sets the reference page's `performance.now` to that clock.

`tests/browser/bounce-physics.spec.js` runs the original package and the
Wasm port side by side in one page and compares every body's position,
orientation, sleep flag and velocities bit for bit after every frame:
- the page's wall and ball layout for aspects 1 (257 balls) and 16∶9
  (458 balls), over 600 and 900 frames;
- the timer's uneven deltas, the page's five-ball respawns and
  pointer-style impulses;
- a 1,500-frame undisturbed session in which bodies fall asleep.

## Rendering

As in [compute-examples.md](compute-examples.md), every stage runs the WGSL
three.js r186 generates for the page (`src/browser/ballpool/`). The SSGI,
composite and TRAA modules are byte-identical to webgpu_postprocessing_ssgi's
and volume_traa's, and are shared with those ports.

- **Instancing.** The balls are one InstancedMesh with the page's ten
  instance colors. Every frame uploads the instance matrices, as the page's
  `setMatrixAt` loop does. InstanceNode reads them, and the previous frame's
  for the velocity, from two buffers:
  - up to 1,024 balls (the 64 KiB uniform buffer limit): uniform arrays
    sized to the count;
  - more balls (aspects above 3.97): instanced vertex attributes.

  Each path is its own captured module. The two number their uniforms from
  different starts (the offset is applied by name). The previous matrices
  are the array the last scene pass drew, or this frame's for a new mesh,
  as InstanceNode's after-object update copies them.
- **Point shadow.** The 1024² cube (near 0.5, far 500: the light has no
  distance) draws the balls when the InstancedMesh's bounding sphere meets a
  face's frustum. The sphere is computed once, on the first frustum test,
  as three.js does. The walls receive shadows but cast none.
- **Camera and pointer.** `fitCameraToBox`, `Raycaster.setFromCamera`, the
  pushes and the light's front-plane hit follow the three.js f64
  operations: `lookAt` through `Quaternion.setFromRotationMatrix`,
  `Matrix4.compose`, `invert` and the WebGPU `makePerspective`. The pushes
  therefore produce the original's impulses bit for bit. The pointer stops
  moving 50 ms after its last move on the example clock. A held pointer
  (two touches) respawns five balls per frame.
- **Resize.** The page rebuilds the world, the box and the balls from the
  new aspect on every resize. The port does the same when the output size
  changes, continuing the seeded random sequence.
- **TRAA start-up.** r186's TRAANode builds its history at the composite
  render target's initial 1 × 1 size on the first frame, because the target
  only renders when the resolve samples it. The first frame then restarts
  from the composite target, renders the passes after `setViewOffset` (so
  even the first frame is jittered), and keeps no previous depth. It reads
  the zero-initialized `DepthTexture( 1, 1 )`, the near plane, for two
  resolves, so the second frame's history counts as disoccluded except on
  edges. The second frame restarts the history again. The port follows
  all of this.

## Comparison

`tests/browser/compute-examples.spec.js` (`webgpu_postprocessing_ssgi_ballpool`)
captures:
- the second frame;
- one second of falling;
- two pointer moves, then 20 frames of pushes and light motion;
- three frames with the button held, then 30 more;
- the 640 × 400 resize, which rebuilds a 412-ball pool.

`tests/browser/ballpool-wide.spec.js` renders 40 frames at 2100 × 500.
That is 1,083 balls, beyond the uniform-array limit, so they are drawn from
instanced vertex attributes. One pixel exceeds the threshold.

From the third frame on, the outputs match. The worst capture has a mean
error of 0.00002 levels and no pixel beyond the default threshold.

The second frame has a scoped bound of 4.5% of pixels and a mean error of
1.4. r186 restarts that frame's history from the composite target its
loading frame rendered mid-resize, and that target holds the image offset.
At 512 × 512 the top 85 rows are black; at 640 × 400 the image is
stretched. The loading frame itself presents a flat 1 × 1 color. The port
restarts from the loading frame's composite and presents that frame
normally. So the second frame differs on the balls' silhouettes, where TRAA
keeps history: 3.96% of pixels, mean error 1.21.

## Performance evidence

The page's per-frame work is CPU physics, matrix uploads and the GPU
passes. The port keeps the same execution architecture:
- the physics steps on the CPU, in Wasm instead of JavaScript;
- the instance matrices upload once per frame (the current and previous
  arrays, 64 bytes per ball each);
- the sphere geometry uploads once, and the walls' boxes once per rebuild.

The GPU residency test checks that steady-state frames create no buffers,
textures, bind groups or pipelines. No timing parity is claimed.
