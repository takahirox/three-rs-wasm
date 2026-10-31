# webgpu_postprocessing_ssgi_ballpool

Pinned Three.js r187: `7e89746a687bb88271253698fa108b1dd59825d0`.

| Official example | Runtime ID | Source |
| --- | ---: | --- |
| `webgpu_postprocessing_ssgi_ballpool` | 484 | `gallery/src/browser/ballpool.rs`, `src/physics.rs` |

The page fills a box with balls and steps them with RapierPhysics.js, which
loads rapier.js 0.21.0 ( `@dimforge/rapier3d-compat` ). A shadow-casting
point light follows the pointer. The frame runs the
webgpu_postprocessing_ssgi pipeline: the MRT scene pass, SSGI with 2 slices
and 8 steps, the composite and TRAA. Then ACES Filmic tone mapping with
exposure 0.5.

## Physics: `crate::physics`

rapier.js 0.21.0 is the Rust crate rapier3d 0.36.0 built to wasm. The port
links the same crate through `crate::physics::RapierPhysics`, as the
physics_rapier ports do ( [physics-examples.md](physics-examples.md) ):
- the walls are fixed bodies ( `addMesh( mesh, 0, 0.4 )`, the front wall
  without a mesh ), the balls one dynamic body per instance
  ( `addMesh( ballsMesh, 1, 0.5 )` );
- RapierPhysics's 16 ms interval steps the world on the example clock with
  its Timer's delta, at most 1 / 30 s, and writes each body's pose into the
  instance matrices;
- a rebuild removes the walls' bodies and the balls' mesh; respawns go
  through `setMeshPosition` and `wakeUp`, pushes through `applyImpulse`.

rapier3d 0.36 orders contact pairs with a fixed-seed hash, so the port takes
the same steps as the page: the captures match it after falling, pushes,
respawns and the resize.

`src/bounce/` keeps the port of @perplexdotgg/bounce 1.8.1, which r186's page
used; `tests/browser/bounce-physics.spec.js` still compares it with the
package bit for bit.

## Rendering

As in [compute-examples.md](compute-examples.md), every stage runs the WGSL
three.js r187 generates for the page (`gallery/src/browser/ballpool/`). The SSGI,
composite and TRAA modules are byte-identical to webgpu_postprocessing_ssgi's
and volume_traa's, and are shared with those ports.

- **Instancing.** The balls are one InstancedMesh with the page's ten
  instance colors. The instance matrices upload in frames after a physics
  step ( RapierPhysics sets `needsUpdate` ), the previous ones when the
  matrices they copy changed, as r187's attribute versions do.
  InstanceNode reads them, and the previous frame's for the velocity, as
  instanced vertex attributes: r187's InstancedMesh binds its instance data
  per object for every count. The previous matrices are the array the last
  scene pass drew, or a clone of the matrices for a new mesh, as
  InstanceNode's after-object update copies them.
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
- **TRAA start-up.** r187's TRAANode builds its beauty ( the composite's
  `convertToTexture` ) before itself, so the composite renders first. On the
  loading frame the history's size differs, and TRAA restarts the history
  from that composite; that resolve reads the freshly sized ( zero ) history
  depth. The loading frame renders unjittered with frameId 1 ( its render()
  starts r187's animation loop, whose start counts frame 1; the fixture stops
  the loop there ), so the second frame is frameId 2, jittered, and blends
  the loading frame's history. The port follows all of this.

## Comparison

`tests/browser/compute-examples.spec.js` (`webgpu_postprocessing_ssgi_ballpool`)
captures:
- the second frame;
- one second of falling;
- two pointer moves, then 20 frames of pushes and light motion;
- three frames with the button held, then 30 more;
- the 640 × 400 resize, which rebuilds a 412-ball pool.

`tests/browser/ballpool-wide.spec.js` renders 40 frames at 2100 × 500,
1,083 balls.

All captures pass the default bounds: no pixel beyond the threshold, and a
worst mean error of 0.00003 levels.

## Performance evidence

The page's per-frame work is CPU physics, matrix uploads and the GPU
passes. The port keeps the same execution architecture:
- the physics steps on the CPU, in Wasm instead of JavaScript;
- the instance matrices upload after physics steps, the previous ones when
  they change (64 bytes per ball each), as the page's attributes do;
- the sphere geometry uploads once, and the walls' boxes once per rebuild.

The GPU residency test checks that steady-state frames create no buffers,
textures, bind groups or pipelines. No timing parity is claimed.
