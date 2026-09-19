# Rust TSL subset

`three_rs_wasm::tsl` constructs expression graphs in Rust and compiles them to
WGSL. Build the material/pass once, then change uniform values. Vertex and pixel
work executes on the GPU; the browser examples do not load Three.js or evaluate
TSL expressions on the CPU. This is not a JavaScript runtime or a full TSL port.

```rust,no_run
use three_rs_wasm::tsl::*;
# async fn example(renderer: &three_rs_wasm::renderer::Renderer) -> three_rs_wasm::Result<()> {
let clock = uniform(0, Type::Float);
let wave = (uv().x() * float(20.0) + clock).sin() * float(0.5) + float(0.5);
let graph = NodeMaterial::new(vec3(wave.clone(), wave, float(1.0)));
let mut material = graph.build(renderer, &[]).await?;
material.uniforms[0][0] = 1.0; // time in seconds, supplied by the application
// Insert Material::Shader(material) into a mesh.
# Ok(()) }
```

Supported building blocks:

- Float, bool and float vectors; scalar broadcasting, vector constructors and
  swizzles; arithmetic, floor-based modulo, powers, trigonometry, clamp, comparison
  and selection. `checker` matches Three.js's 2×2 pattern per UV unit.
- Explicit uniform slots (16 vec4s), vertex UV/position/normal inputs and
  interpolated fragment UV. `NodeMaterial.position` is a GPU vertex expression;
  `color` supplies unlit float/vec3/vec4 output. Existing material fog applies.
- Implicit texture sampling, explicit LOD and explicit gradients. Vertex texture
  sampling requires explicit LOD. Material maps, retained external 2D
  texture/sampler pairs, and fullscreen-pass inputs are available.
- `function(|| ...)` provides Fn-style Rust closure composition. Cloned nodes
  share identity, so a reused expression is emitted once per stage. `WgslFn`
  integrates a native WGSL function with a checked argument/result signature.
  The GPU validates its body, including derivative/stage restrictions.
- `effect` compiles the same nodes into an existing fullscreen `Effect`.
  `gaussian_blur` builds the pinned GaussianBlurNode kernel; the application
  schedules its horizontal and vertical passes with resident render targets.

Graph compilation rejects incompatible types, invalid swizzles, out-of-range
uniform/texture slots, wrong stage inputs and conflicting native function names.
Shader creation is fallible; invalid native WGSL is not treated as successful.

Texture nodes address the bound storage directly. They do not implicitly apply
`Texture.uv_matrix()` or color-space conversion. Use an sRGB GPU texture for sRGB
input, and construct any desired coordinate transform explicitly. `Texture::Map`
uses the material's map binding; `Texture::External(i)` refers to the corresponding
`build` argument. Effect UV coordinates start at the top left. The CRT/gradient
examples prepare image rows once to match WebGPU TextureLoader's upload flip;
render-to-texture coordinates are explicitly converted to Three.js QuadMesh UVs.

Not implemented: the JavaScript DSL/parser/transpiler, general mutable variables
and control-flow nodes, matrices/integer/storage/compute node graphs, arbitrary
attributes/varyings, automatic render-graph scheduling, or PBR node hooks such as
roughness/normal/lighting/output overrides. `select` evaluates both expressions;
it does not promise lazy branches. Native WGSL functions are an explicit escape
hatch, not a claim that their behavior has been implemented as Rust nodes.

## Examples and checks

The gallery adds the official WGSL/TSL Interoperability, Texture Gradient and
Procedural Texture scenes. Texture Gradient shows the official WebGPU scene in
one canvas; its duplicate forced-WebGL comparison panel is omitted by the
project's example policy. Inspector styling is still different. Procedural
Texture keeps the upstream 512×512 HDR target, two 85-tap separable blur passes,
UV scale, blur amount, and auto-update behavior; no CPU image generation/readback.

`reference/three-js/tsl.html` executes the pinned upstream example scripts with
only deterministic time, captured GUI parameters, the omitted WebGL panel and
fixture layout. Rust scenes use independently built graphs. Rendering tests cover
three times, every exposed control, render-to-texture freeze/re-enable, and resize.
The existing 6/255 channel / 0.5% pixel threshold is unchanged.

```sh
python3 tools/compat/prepare_reference.py
python3 tools/gallery/build.py
python3 tools/tsl/prepare.py
cargo test --test tsl --test tsl_gpu --test programming
wasm-pack build --target web --out-dir web/pkg --release --no-typescript
npx playwright test tests/browser/tsl.spec.js
```

Native tests check graph errors, GPU vertex displacement, signed modulo, uniform
updates, native WGSL integration, generated textures and blur. Browser tests check
that buffer/texture/bind-group/shader/pipeline creation and geometry uploads remain
unchanged after warm-up and after resize. These are architecture/residency checks,
not a claim of equivalent CPU/GPU frame times. No hardware timing parity is claimed.

Validation on 2026-09-20: all 25 image states had zero pixels exceeding the
6/255 per-channel tolerance (small sub-threshold quantization differences remain).
The 14 targeted browser checks and 5 native checks passed, including existing
RawShaderMaterial and gallery-layout regression cases. Structured results:
[`tsl-comparison.json`](tsl-comparison.json).
