# three-rs-wasm

A Rust-first WebGPU 3D library using Three.js r186 as its behavioral reference.
The frozen compatibility manifest defines the MVP target; the acceptance scripts
verify its implementation and the separate glTF PBR milestone.

Engine state, transforms, geometry, materials, raycasting and rendering live in
Rust. Browser bindings provide DOM access and input. Rendering uses wgpu's WebGPU
backend in the browser; there is no JavaScript scene mirror or WebGL fallback.

## Development

Install Rust with rustup, the `wasm32-unknown-unknown` target, Node.js, Python 3,
wasm-pack and Chrome. Ensure `cargo` and `rustc` resolve to the same toolchain.
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

The browser example is at `/web/` when serving the repository. Click the cube to
select and pause it; click outside to resume. All animation, input and raycasting
code is Rust. The JavaScript bootstrap only loads Wasm and owns the app lifetime.

`./scripts/check-mvp` runs the complete acceptance gate. Missing suites or missing
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

## Interactive demos

Build an optimized browser package for the deforming model, then start the local
server from the repository root:

```sh
PATH="$(dirname "$(rustup which rustc)"):$PATH" wasm-pack build --target web --out-dir web/pkg --release --no-typescript
python3 tools/serve.py
```

Open <http://127.0.0.1:8173/web/?example=3> for the official Point Lights port:
16,160 model triangles expanded into 64,640 animated tetrahedron faces, two moving
colored lights, orbit/zoom, pause, speed and deformation controls. Other tabs show
the introductory scenes and official textured cube. Rendering and deformation
remain in Rust; the DOM controls forward user input to the app. See
[asset credits and differences from the official examples](web/THIRD_PARTY.md).

The glTF tabs show DamagedHelmet (`?example=4`, external .gltf/.bin/JPEG) and
BoomBox (`?example=5`, embedded GLB). Both load in Rust/Wasm and support drag
and zoom, with original PBR maps, HDR reflections, exposure and background controls.
Run `./scripts/check-gltf-pbr` for the complete MVP + M2 acceptance gate.
See [M2 API, validation and limitations](docs/gltf-pbr.md) and
[asset credits](web/THIRD_PARTY.md).
