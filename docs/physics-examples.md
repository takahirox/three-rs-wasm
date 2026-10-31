# Rapier physics examples

Pinned Three.js r187: `7e89746a687bb88271253698fa108b1dd59825d0`.

The six `physics_rapier_*` pages, and webgpu_postprocessing_ssgi_ballpool
( [ballpool.md](ballpool.md) ), use RapierPhysics.js, which loads rapier.js
0.21.0 ( `@dimforge/rapier3d-compat` ). That package is the Rust crate
rapier3d 0.36.0 built to wasm ( rapier.js now lives in the rapier
repository's `bindings/typescript`; 0.21.0 is commit `b716d375` ). The ports link the same crate
( `src/physics.rs` ) and run it on the CPU in wasm, as the pages do. Each is
compared against WebGLRenderer ( `tests/browser/texture-volumes.spec.js` ).

The ammo.js ( Bullet ), Jolt and bounce pages are not ported: their engines
are C++ or JavaScript libraries without a Rust build.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `physics_rapier_basic` | 464 | `physics_rapier_basic.rs` | A body a second ( box, ball or rounded box ) dropped on the floor, the removal below −10 in animate(), RapierHelper, damped OrbitControls |
| `physics_rapier_instancing` | 465 | `physics_rapier_instancing.rs` | 400 boxes and 400 balls as two InstancedMeshes, a body per instance, the 16 ms reset interval, SHAKE |
| `physics_rapier_joints` | 466 | `physics_rapier_joints.rs` | The fixed pivot and three capsule links on spherical impulse joints with angular damping 10, RapierHelper |
| `physics_rapier_terrain` | 467 | `physics_rapier_terrain.rs` | The 128 × 128 heightfield collider and plane, up to 30 random bodies spawned by the page's Timer, the per-frame wake-up and push |
| `physics_rapier_character_controller` | 468 | `physics_rapier_character_controller.rs` | KinematicCharacterController with dynamic-body impulses, the ten random bodies, WASD and the arrows |
| `physics_rapier_vehicle_controller` | 469 | `physics_rapier_vehicle_controller.rs` | DynamicRayCastVehicleController with four wheels, the engine, steering, brake and reset keys, the following OrbitControls |

## The same engine

- **Crate and features.** rapier3d is pinned to 0.36.0 with `dim3`, `f32`,
  `std` and `debug-render`. The compat package is the bindings'
  non-deterministic, non-SIMD build: rapier3d's default features
  ( `dim3`, `f32`, `std` ) plus `serde-serialize`, `debug-render`,
  `profiler` and `fem`. The last three change no rigid-body arithmetic
  ( serialization, timers, and the FEM soft-body solver, which only runs
  for soft bodies ), so they are left out. Neither build enables
  `enhanced-determinism` or the `+simd128` target feature.
- **Dependencies.** The repository keeps no lock file for the bindings;
  Cargo.lock pins the versions the package's wasm names in its panic
  locations: parry3d 0.31.1, nalgebra 0.35.0, glam 0.33.10, glamx 0.3.1,
  hashbrown 0.17.1, smallvec 1.16.2 and ena 0.14.4.
- **The bindings' defaults.** Bodies and colliders go through the builder
  calls of rapier.js's `createRigidBody` and `createCollider`, with
  RigidBodyDesc's and ColliderDesc's defaults. These include the zero
  additional mass properties and PGS iterations, and the 16-bit halves of
  the 0xffffffff groups ( `InteractionTestMode::And` ). World.step passes the
  world's ( empty ) soft-body set; scene queries read the broad phase's
  tree, as the controllers' bindings do ( `as_query_pipeline_mut` with their
  filters ). Shapes follow `getShape()`: half extents, a rounded box's inner
  cuboid, a capsule along y.
- **Steps on the example clock.** RapierPhysics steps on
  `setInterval( step, 1000 / 60 )`. The delay is a WebIDL long, so it runs
  every 16 ms, and each step's timestep is its Timer delta, at most 1 / 30 s
  ( r187 ). The pages' own intervals ( addBody, the instancing reset ) run on
  the same clock. Timers due at the same time run in the order they were
  last scheduled. The fixture drives the page's timers the same way, with
  its Timer on that clock.
- **Page order.** Per-frame logic runs per rendered frame, as animate() does:
  the removal walk with for…of over a shrinking array, the terrain's spawn
  test, the character's movement after the render, and the vehicle's control
  and `updateVehicle( 1 / 60 )`.

## Trajectories

rapier3d 0.36 keys its hash maps with foldhash's fixed seed, so a world's
contact order no longer depends on memory addresses. rapier.js 0.21.0 steps
the same world identically when it runs several times in one process ( 400
falling boxes and balls, 1,200 steps: every coordinate equal ), and the
ports take the same steps: the ballpool's captures after falling, pushes and
respawns have no pixel beyond the default threshold. The captures keep the
windows they had:

- **Basic**: 0–8 s.
- **Instancing**: 0, 1 and 3 steps.
- **Joints**: the first second.
- **Terrain, character, vehicle**: the full captures.

## Comparison tolerances

- **Collider outlines and grid lines**: WebGL and WebGPU rasterize one-pixel
  lines differently. Basic 0.8 %, character and vehicle 1 % with MSAA.
  With MSAA, the basic page's dragged view of the 8 s pile ( state 5 )
  exceeds its 1.2 % bound: 1.30 % of pixels, mean 0.33, all on collider
  outlines and shadow edges; the bodies match. This remains open.
- **Instancing**: 800 small instances' edges and the 512² shadow map: 0.4 %
  at rest ( 2.3 % with MSAA ), 1.5 % ( 3.9 % ) after three steps, mean
  under 1.
- The terrain and joints captures pass the default thresholds.

## Workload

- **CPU, as on the pages.** Each step writes the bodies' poses to the meshes,
  or to the instance matrices.
- **Streamed lines.** RapierHelper replaces its line attributes every frame
  on the page. The port writes the frame's lines into the geometry's resident
  vertex buffer, which grows only when they do not fit. Its vertex holds the
  page's position and color, 28 bytes.
- **Residency.** Steady cycles create no GPU resources. The character's
  residency cycle walks forward and back, and leaves out the drag: a camera
  turning further each cycle culls the scattered bodies in and out, and the
  engine drops the draw slots of meshes it no longer draws.
- No timing parity is claimed.
