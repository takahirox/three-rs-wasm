# Static glTF PBR / HDR (M2)

The Rust library imports static metallic-roughness glTF scenes and renders them
through wgpu, including in WebAssembly/WebGPU. Three.js is used only by the
reference and asset conversion tools, never by the Rust demo.

## Run and validate

Install the Rust toolchain and wasm32-unknown-unknown target, wasm-pack 0.14.0,
Node.js and the Playwright browser dependencies described in the main README.
Run `./scripts/check-gltf-pbr` for the complete prerequisite MVP and M2 gates.
Missing assets, adapters, checks or excessive image differences are failures.
The final success line is `GLTF PBR: PASS` only if all gates pass.

For the demo, build with `wasm-pack build --target web --out-dir web/pkg
--release --no-typescript`, run `python3 tools/serve.py`, and open
http://127.0.0.1:8173/web/?example=4 (DamagedHelmet) or `?example=5` (BoomBox).
Model selection, exposure, environment intensity/rotation/background/blur and
orbit/zoom controls act on Rust-owned scene state. Resize preserves aspect.
Failed replacement loads leave the previous scene usable; newer requests win.

## Library mapping

| Capability | Rust API |
| --- | --- |
| Static glTF import | `gltf::import(&gltf::Gltf, buffers, encoded_images)` |
| Install a validated asset | `ImportedGltf::instantiate(&mut Scene)` |
| Material factors and maps | `MeshStandardMaterial`, `MaterialProperties` |
| HDR decode | `EnvironmentMap::from_hdr` (Radiance RGBE) |
| Image based lighting | `Scene::environment`, intensity and Y rotation |
| Environment background | `Scene::background_environment`, `background_blur` |
| HDR MSAA intermediate | `RenderTargetOptions { format: Rgba16Float, samples: 4, .. }` |
| ACES and exposure after resolve | `Renderer::blit_tone_mapped` to an sRGB target |
| Explicit cache collection / diagnostics | `Renderer::collect_resources`, `resource_counts` |

Import resolves accessors and constructs an asset before mutating a destination
scene. The caller supplies resource bytes; the browser adapter resolves external,
embedded GLB and data-URI resources. Lossless images are decoded in Rust. The browser adapter uses its JPEG codec
for parity with GLTFLoader’s JPEG IDCT rounding, then passes owned RGBA pixels
to `gltf::import_decoded`; native callers can use `gltf::import` with encoded images. Each imported
mesh retains its static world transform and material assignment. Supported
fixtures use triangle primitives, indices, normals, tangents (including their
sign), TEXCOORD_0 and COLOR_0. Missing tangents use screen-space derivatives with
the pinned GLTFLoader normal-scale convention. Required extensions and skins
are rejected; morph primitives and unsupported UV channels fail explicitly.
Animation playback, compressed textures/meshes and advanced physical extensions
are outside this milestone. This is not an arbitrary glTF conformance claim.

Base color and emissive images use sRGB; metal/roughness (B/G), normal and AO (R)
images use linear sampling. GPU mip generation preserves the format's conversion
and rounding. GPU images are cached by their immutable `Arc<Texture>` owner;
use a new Arc (or `Arc::make_mut`) when changing data or sampler settings.
Weak cache ownership permits disposal when scenes release their materials.
Environment filtering runs once per changed environment, with only one active
cached environment. Pipeline caches are bounded by render configurations.

## Reference and HDR provenance

`tests/gltf-pbr/manifest.json` was committed before broad rendering work. It fixes
r186, original source asset hashes, eight camera/exposure/orientation settings,
256x256 output, four samples, ACES/sRGB and the image comparison tolerance.
The reference keeps GLTFLoader's original materials and decodes the original
UltraHDR with the pinned UltraHDRLoader. Earlier base-color comparison pages
have been upgraded to use these actual PBR materials; M1 frozen manifests and
image tolerances remain unchanged.

The demo uses a verified Radiance HDR conversion of that exact reconstructed
UltraHDR. This avoids platform JPEG/interpolation differences in gain-map
recovery. `web/environments.json` records both source and derivative hashes.
`tools/convert-environment.mjs` reproduces the conversion with the pinned loader
while the local server is running. The HDR equivalence test compares every
channel of Rust-decoded HDR against the official reconstruction, allowing only
Radiance RGBE's quantization bound (1/128 of each pixel's peak channel).
Values above one remain unclamped through lighting and four-sample RGBA16F
resolve; ACES exposure and the sRGB attachment conversion occur afterward.

The GGX prefilter, cube-UV layout/roughness mapping and 16x16 DFG LUT follow r186.
Alpha-mask discard is specialized per pipeline; opaque shaders retain helper-lane
derivatives for normal mapping and geometric roughness on hardware WebGPU.
The sharp background reproduces the reference's panorama-to-cube interpolation
by evaluating the four cube texels directly; it does not allocate a second full
cube texture. Blurred background sampling uses the prefiltered environment.

Asset licenses and credits, including DamagedHelmet's noncommercial condition,
are in `web/THIRD_PARTY.md`. All runtime assets are tracked in git. The pinned
reference is acquired by `tools/compat/prepare_reference.py` on a clean checkout.

## Evidence

`tests/gltf-pbr/coverage.json` maps required features to executable tests.
Native tests cover material effects and 20 texture/environment replacement/drop
cycles. Browser tests cover the real PBR comparison, HDR equivalence, controls,
resize, failed requests, overlapping loads and 20 model replacement cycles.
Reference, actual and amplified difference images and numerical reports are
written to `test-results/` and retained as CI artifacts. macOS uses Chrome/Metal;
Linux uses the configured Chromium/SwiftShader Vulkan path, with no adapter skip.
