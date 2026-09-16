# MVP scope and review

The source of truth remains Issues #1 and #2 and the unchanged r186 compatibility
and exclusion manifests. `implementation.json` records Rust mappings and evidence
for 350 unique required APIs; the inventory contains 394 entries when distinct
accessor forms, exclusions and deferred items are counted.

| Acceptance requirement | Executable evidence |
|---|---|
| Core accounting | Pinned source extraction, manifest consistency, implementation mappings |
| Behavioral compatibility | Live r186 differential probe: transforms, attach, bounds, typed attributes, all six Euler orders, primitives, cameras, color and ray intersections |
| Rust browser applications | Frozen 3-application, 27-capability manifest; animation, pointer selection, visibility timing and scene/resource rebuild assertions |
| Migration | Basic cube and lit textured hierarchy, rendered by both implementations and compared at fixed image tolerances |
| Resource lifecycle | 1,000 scene create/drop cycles; generational handle rejection; shared-resource release; 50 GPU render/target cycles with active allocation counters |
| Type safety | Compile-fail checks for forged handles and mutation while a node is borrowed |
| Wasm/browser | Wasm build and clippy; Chrome/Playwright running actual WebGPU and Rust event callbacks |
| Quality gates | Formatting, native/Wasm clippy, Rust tests, browser tests and CI workflow |
| One-command evaluation | `./scripts/check-mvp`, nonzero on any failed or unavailable required check |

Implementation review found and corrected:

- The pre-existing inventory parser mistook an object default argument for a
  method body, omitting RenderTarget fields. Private implementation methods are
  filtered according to the existing inventory rule; no exclusions were added.
- Integer attributes needed JavaScript wrapping/rounding behavior; half floats
  needed r186 truncation, overflow clamping and normalized conversion.
- Euler orders other than XYZ needed axis reordering when calling glam.
- r186 world updates respect dirty and force flags. Graph update/disposal now use
  iteration so deep hierarchies do not exhaust the call stack.
- Interleaved geometry clones must preserve internal aliasing while separating
  their storage from the original geometry.
- Transparent objects require depth ordering. Mirrored mesh transforms require
  front-face reversal; point material sizes require generated quads on WebGPU.
- Invalid camera/viewport/texture inputs should return errors before reaching GPU
  validation. Private attribute layouts and scene handles have no unchecked
  deserialization entry point.
- Rust event dispatch must allow listener removal without retaining the
  dispatcher in its own callbacks. Browser callbacks and animation requests are
  released with the app; visibility listeners are released with their timers.
- Build tools must share one Rust toolchain; Wasm-only code also needs clippy.

The implementation uses one library crate with ordinary modules, glam for math,
wgpu for graphics and serde for serialization. It does not add an ECS, a custom
shader language, a plugin system, JavaScript scene synchronization, WebGL support,
advanced loaders, shadows or other deferred subsystems.

The renderer currently supports up to eight nonambient lights per scene and
returns an error above that limit. Line width follows WebGPU's one-pixel line
primitives. Depth/stencil resolve flags are retained as configuration, matching
the r186 WebGPU backend's use of attachment store semantics rather than WebGL
depth resolves. General Three.js JavaScript source/drop-in compatibility is not
claimed. The scene's serializable model has an output API, not an advanced loader.

Implementation mappings are reviewable accounting, not a substitute for tests.
Neither mapping records nor README files can make behavioral or browser checks
pass. A clean checkout and CI must execute the same acceptance command before a
final merge decision.
