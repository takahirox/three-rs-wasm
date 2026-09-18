# three-rs-wasm

A Rust-first WebGPU 3D library using Three.js r186 as its behavioral reference.
The frozen compatibility manifest defines the MVP target. Separate acceptance
scripts cover the MVP, glTF PBR milestone, expanded Core features and examples
gallery; passing them does not establish full Three.js or performance parity.

Engine state, transforms, geometry, materials, raycasting and rendering live in
Rust. Browser bindings provide DOM access and input. Rendering uses wgpu's WebGPU
backend in the browser; there is no JavaScript scene mirror or WebGL fallback.

## Demo: Three.js examples in Rust/Wasm

The main demo is the [examples gallery](https://takahirox.github.io/three-rs-wasm/):
select an example from the list to launch its Rust/Wasm/WebGPU port. The gallery
retains the pinned Three.js r186 browser's CSS, fonts, thumbnails, categories,
search and mobile drawer. Only runnable ports appear in the list.

With the [development prerequisites](#development) installed, build an optimized
browser package and start the local server from the repository root:

```sh
PATH="$(dirname "$(rustup which rustc)"):$PATH" wasm-pack build --target web --out-dir web/pkg --release --no-typescript
python3 tools/serve.py
```

Open <http://127.0.0.1:8173/web/gallery/> in a WebGPU-capable browser.

The example ports are a work in progress. See the
[coverage report](docs/examples-coverage.md) for current support and limitations.
Example names retain the upstream
`webgl_` or `webgpu_` prefix; all ports render with WebGPU. See
[asset credits](web/THIRD_PARTY.md).

## Development

Install Rust 1.94 or newer with rustup, the `wasm32-unknown-unknown` target,
Node.js, Python 3, wasm-pack and a browser as described below. Ensure `cargo` and
`rustc` resolve to the same toolchain.
On machines that also have Homebrew Rust, the acceptance script selects the
active rustup toolchain for all tools. For individual browser commands, prepend
that toolchain's `bin` directory to `PATH` as well.

```sh
python3 tools/compat/prepare_reference.py
cargo test --test core
python3 tests/compat/compare.py
cargo test --test gpu --test pbr
wasm-pack build --target web --out-dir web/pkg --dev --no-typescript
npm ci
npm run test:browser
```

GPU tests require an available hardware or software graphics adapter; lack of an adapter fails the test.
The browser tests use installed Chrome on macOS and Playwright Chromium on other
platforms (`npx playwright install chromium`). Linux CI uses software Vulkan for
native tests and Chromium's SwiftShader Vulkan driver for browser tests. The local server binds loopback.

`./scripts/check-mvp` runs the MVP acceptance gate. Missing suites or missing
implementation evidence fail; README files alone cannot satisfy a check.

## Rust API mapping

| Three.js | Rust |
|---|---|
| `Object3D` | `scene::Object3D`, an opaque scene-scoped handle |
| Object properties | `Scene::get` / `Scene::get_mut` and `Node` fields |
| `add`, `attach`, traversal | `Scene` graph operations |
| Math values | Double-precision glam types under Three.js names |
| Geometry and materials | Owned values shared explicitly with `Arc` |
| Typed array attributes | `BufferAttribute<T>` and typed aliases |
| Raycasting | `Raycaster`, `Raycast`, typed `Intersection` |
| Events | Typed `EventDispatcher<E>` with listener tokens |

Call `Scene::update` before world-space queries; rendering does this automatically.
Dropping the scene releases its shared resource references. `Scene::dispose`
removes a subtree and invalidates its handles. `remove_from_parent` only detaches
the node so it can be reused. Cloning a subtree shares geometry and materials,
matching the useful Three.js scene-copy behavior.

Behavioral comparisons run the pinned JavaScript implementation, not generated
Rust golden files. Geometry storage is float32 (absolute/relative tolerance
`2e-6`); double-precision math uses `1e-10`. Integer/index results should be exact.

## Example coverage and validation

The coverage catalog tracks implementation progress and known limitations.
Examples requiring WebGL-specific APIs are excluded. Only runnable entries appear
in the gallery; investigation pages label upstream thumbnails as reference previews.

Run `./scripts/check-gallery` for source/asset accounting, desktop/mobile UI image
comparisons, runnable examples, PMREM image parity, and existing glTF/point-light
regressions. It starts a separate test server on port 8174. A passing gallery gate
does **not** mean all examples are reproduced. See [coverage and missing capabilities](docs/examples-coverage.md).
Run `./scripts/check-gltf-pbr` for the MVP + glTF PBR acceptance gate; see
[glTF PBR API, validation and limitations](docs/gltf-pbr.md).
`python3 tools/gallery/build.py` reproducibly regenerates the gallery inventory,
upstream shell/assets and investigation report from the pinned archive.

GitHub Actions builds the release Wasm package, checks the gallery under the
project URL prefix, and deploys the demo to GitHub Pages on pushes to `main`.
For a local preview of the published layout, run `python3 tools/build_site.py`
after the release build, serve `.cache/pages` with a static HTTP server, and open
`/three-rs-wasm/`. The deployment contains demo assets and licenses only.

[Per-example prerequisites and execution evidence](docs/gallery-port-attempts.md)
distinguish implemented browser ports from source-level blockers. To repeat the
actual 49-model importer experiment, run `python3 tools/gallery/prepare_gltf_attempts.py`,
then `cargo run --locked --example gltf_gallery_probe -- .cache/gallery-gltf/manifest.json`.
Successful static import is not animation or material-extension fidelity.
`python3 tools/gallery/audit.py` regenerates the per-example review using the
recorded importer results. The line/point picking ports compare all geometry and
transforms against upstream, but native WebGL rasterization differences remain.

[All-example runtime attempts](docs/gallery-runtime-results.md) additionally
record execution of all 605 retained upstream examples in instrumented Chrome,
followed by native Rust rendering of eligible captured scenes. These single-frame
probes use upstream geometry construction and asset decoding; they are not full
Rust ports and are not added to the gallery. The report separates observed
prerequisites, probe limits, empty frames and actual scene renders, and includes
commands for repeating the experiment.

Core rendering foundations now include shadows, additional lights/materials, physical
extensions, WGSL/compute/postprocessing, animation/skin/morph, instancing/batching
and compressed glTF. See [Core implementation and limits](docs/core-expansion.md).
Run `./scripts/check-core` for semantic, GPU and original Three.js comparisons.

Performance is part of reproduction: see [acceptance criteria and audit](docs/performance-parity.md).
GPU skin/morph and vertex displacement retain source geometry on the GPU; CPU
static merging and RGBA transcoding are not claimed as equivalent GPU features.
