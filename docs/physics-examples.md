# Rapier physics examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

The six `physics_rapier_*` pages use RapierPhysics.js, which loads rapier.js
0.17.3 ( `@dimforge/rapier3d-compat` ). That package is the Rust crate
rapier3d 0.26.1 built to wasm. The ports link the same crate
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

- **Crate and dependencies.** rapier3d is pinned to 0.26.1. Cargo.lock pins
  nalgebra, simba, libm, spade, robust, matrixmultiply, wide, hashbrown and
  rustc-hash to the versions in rapier.js 0.17.3's lock file. The crate is
  built without features beyond `debug-render`, like the compat package.
- **The bindings' defaults.** Bodies and colliders go through the builder
  calls of rapier.js's `createRigidBody` and `createCollider`, with
  RigidBodyDesc's and ColliderDesc's defaults. These include the zero
  additional mass properties and the 16-bit halves of the 0xffffffff groups.
  World.step's query pipeline update and the controllers' filters are kept
  too. Shapes follow `getShape()`: half extents, a rounded box's inner
  cuboid, a capsule along y.
- **Steps on the example clock.** RapierPhysics steps on
  `setInterval( step, 1000 / 60 )`. The delay is a WebIDL long, so it runs
  every 16 ms, and each step's timestep is its Timer delta. The pages' own
  intervals ( addBody, the instancing reset ) run on the same clock. Timers
  due at the same time run in the order they were last scheduled. The
  fixture drives the page's timers the same way, with its Timer on that clock.
- **Page order.** Per-frame logic runs per rendered frame, as animate() does:
  the removal walk with for…of over a shrinking array, the terrain's spawn
  test, the character's movement after the render, and the vehicle's control
  and `updateVehicle( 1 / 60 )`.

A probe built from rapier.js's own lock file stepped the basic scene in
bit-identical states for the first 1,150 steps, compared with
`@dimforge/rapier3d-compat@0.17.3`.

## Where trajectories part

Rapier's broad phase reports new contact pairs from a hashbrown map with
foldhash's default hasher. Its seed mixes stack, static and heap addresses,
and advances with each map. The order of a pair's constraints therefore
depends on the binary and its allocation history. rapier.js itself steps the
same world differently when it runs twice in one page:

- Basic scene: 147 of 210 coordinates differ after 1,800 steps.
- Joint chain: the swing differs from the first step, by 3.4 cm after 600
  steps ( a third run matches the first ).
- Instancing: 1,393 of 2,400 coordinates differ after one step, by up to
  3.3 mm.

A port cannot reproduce one particular order without the original's memory
layout. The comparisons stay within the time each scene follows the same
steps:

- **Basic**: 0–8 s, before the pile grows.
- **Instancing**: 0, 1 and 3 steps. The 800 bodies start overlapping, so
  contacts order every step.
- **Joints**: the first second, before the chains' differences show.
- **Terrain, character, vehicle**: the bodies meet only the terrain, floor or
  ground, so the full captures compare.

## Comparison tolerances

- **Collider outlines and grid lines**: WebGL and WebGPU rasterize one-pixel
  lines differently. Basic 0.8 %, character and vehicle 1 % with MSAA.
- **Instancing**: 800 small instances' edges and the 512² shadow map: 0.4 %
  at rest ( 2.3 % with MSAA ), 1.5 % ( 3.9 % ) after three steps, mean
  under 1.
- The terrain and joints captures pass the default thresholds.

## Workload

- **CPU, as on the pages.** Each step writes the bodies' poses to the meshes,
  or to the instance matrices.
- **Streamed lines.** RapierHelper replaces its line attributes every frame
  on the page. The port writes the frame's lines into the geometry's resident
  vertex buffer, which grows only when they do not fit. The engine's vertex
  is 80 bytes against the page's 28 ( position and color ), so the
  streamed-bytes bound is 3×.
- **Residency.** Steady cycles create no GPU resources. The character's
  residency cycle walks forward and back, and leaves out the drag: a camera
  turning further each cycle culls the scattered bodies in and out, and the
  engine drops the draw slots of meshes it no longer draws.
- No timing parity is claimed.
