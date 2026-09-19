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

- Float, bool, uint, uvec2 and float vectors; scalar broadcasting, vector constructors and
  swizzles; arithmetic, floor-based modulo, powers, trigonometry, square root, dot/cross products, max, clamp, comparison
  and selection. `checker` matches Three.js's 2×2 pattern per UV unit.
- Explicit uniform slots (16 vec4s), vertex UV/position/normal inputs and
  interpolated fragment UV. `NodeMaterial.position` is a GPU vertex expression;
  `color` supplies unlit float/vec3/vec4 output. Existing material fog applies.
- Implicit texture sampling, explicit LOD and explicit gradients. Vertex texture
  sampling requires explicit LOD. Material maps, retained external 2D
  texture/sampler pairs, and fullscreen-pass input/history textures are available.
  `effect_with_textures` binds additional sampled textures; `Effect::set_textures`
  rebinds resized targets without recompiling the shader.
- `function(|| ...)` provides Fn-style Rust closure composition. Cloned nodes
  share identity, so a reused expression is emitted once per stage. `WgslFn`
  integrates a native WGSL function with a checked argument/result signature.
  The GPU validates its body, including derivative/stage restrictions.
- `effect` compiles the same nodes into an existing fullscreen `Effect`.
  `gaussian_blur` builds the pinned GaussianBlurNode kernel; the application
  schedules its horizontal and vertical passes with resident render targets.
- `mix`, `luminance`, `saturation`, `hue`, `dot_screen`, and `rgb_shift` compose
  reusable expression graphs. Multi-pass order is explicit in Rust.
- `compute::TextureCompute` compiles `instance_index`, unsigned arithmetic,
  `uvec2` coordinates and vec4 colors into a bounds-checked GPU `textureStore`
  dispatch. Its RGBA8 storage texture, uniforms and compute pipeline are retained;
  repeated dispatches and uniform updates require no CPU image generation.
  Uint-to-float conversion is explicit; uniform slots remain floating point.

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
and control-flow nodes, matrices, general storage-buffer/atomic compute graphs, arbitrary
attributes/varyings, automatic render-graph scheduling, or PBR node hooks such as
roughness/normal/lighting overrides. A limited final lit-color hook is described below. `select` evaluates both expressions;
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
npx playwright test tests/browser/tsl.spec.js tests/browser/tsl-passes.spec.js
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

## Five additional GPU-pass examples

| Gallery example | Implemented work |
| --- | --- |
| `webgpu_compute_texture` | Official 512×512 GPU compute graph, storage texture, on-demand display |
| `webgpu_rtt` | UV-grid cube to RGBA8 render target, mouse-controlled hue/saturation |
| `webgpu_postprocessing` | 100 shared-geometry Phong spheres, HDR scene pass, Dot Screen and RGB Shift |
| `webgpu_postprocessing_difference` | Current/previous HDR textures, motion-dependent saturation, Neutral tone mapping, speed and Orbit controls |
| `webgpu_postprocessing_masking` | Three GPU scene passes, transparent box/torus masks, sampled image composition |

`reference/three-js/tsl-passes.html` executes the unchanged shader expressions in
pinned upstream scripts. The fixture controls time and seeds `Math.random` for
the 100-sphere scene. RGB-shift input remains signed HDR; the Dot Screen quad's
UV follows WebGPU render-target orientation. Scene-only isolation confirmed that
the sphere geometry, Phong lighting and fog match before applying the effects.
Difference comparisons settle history after resize because each backend resizes
its two history textures on a different frame; consecutive equal and changed
poses are also compared before resize.

`tests/browser/tsl-passes.spec.js` compares animation, mouse input, history and
resize. It warms and repeats the same motion cycle before/after resizing and
requires zero new GPU buffers/textures/bind groups/pipelines/shaders and zero
new geometry/deformation-input uploads. Compute Texture must dispatch exactly
once even across resize. Render slots now follow scene/object/material-group
identity, so culling or another scene's render does not discard reusable data.
Scene lifetime and removed handles still release cached entries.

These remain partial ports: Inspector styling and equivalent CPU/GPU frame times
are not claimed. No JavaScript TSL parser or automatic pass scheduler was added.

Validation on 2026-09-20: all 37 new image states passed the unchanged threshold.
The four RTT/postprocessing examples had zero pixels exceeding 6/255; Compute
Texture reached 0.453125% at the resized viewport (allowed: 0.5%). RTT also passed
with device-pixel ratio 2. The 77 browser regressions and all 60 native tests
(including doctests) passed, as did native/Wasm Clippy and formatting checks.
Measurements and paths to the local PNG pairs:
[`tsl-passes-comparison.json`](tsl-passes-comparison.json).

## Five display-filter examples

| Gallery example | Implemented work |
| --- | --- |
| `webgpu_postprocessing_direct` | 100 Phong spheres, inline saturation before Neutral tone mapping |
| `webgpu_postprocessing_radial_blur` | 100 GPU instances, HDR radial blur with runtime sample count |
| `webgpu_postprocessing_fxaa` | 100 GPU instances, sRGB conversion followed by adaptive FXAA |
| `webgpu_postprocessing_ssaa` | 120 GPU instances, 1–32 jittered scene samples and weighted GPU accumulation |
| `webgpu_postprocessing_transition` | Two 500-instance scenes, six texture masks, animated transition and endpoint pass skipping |

`output_program(renderer, node)` compiles `output()` expressions into the lit
material fragment shader, before tone mapping and output encoding. Assign the
program to `MaterialProperties.vertex_program`; `vertex_uniforms` supplies its
uniform slots. This reuses the existing custom-program binding and does not add
a fullscreen effect for saturation. The browser still uses its normal canvas
presentation pass. Arbitrary vertex displacement combined with this hook, and
material normal/roughness/lighting graphs, are outside this API.

`tsl::display` provides `srgb`, `premultiplied_srgb`, `radial_blur`, `fxaa` and
`transition`. Radial blur and adaptive FXAA loops are reusable native WGSL
helpers, not a general Rust control-flow AST. FXAA requires sRGB input; masks
are linear data. External transition mask UVs account for TextureLoader's
upload orientation. `SsaaPass` supports perspective cameras, view offsets and
color-only RGBA16Float accumulation; it restores the original camera view even
on an error. MRT/depth propagation and orthographic SSAA are not implemented.
`MeshStandardMaterial.energy_conservation` enables r186 WebGPU punctual-light
Fresnel attenuation and multiple-scattering compensation, plus ambient/hemisphere
diffuse energy attenuation. The previous lighting remains the default for existing
ports; the three new Standard-material scenes enable the option. Rect-area light
energy compensation is outside this addition. The FXAA investigation isolated the
filter on an identical official input before identifying this lighting difference.
Solid-background alpha is premultiplied, and transparent SSAA canvas output is
encoded after unpremultiplication.

The pinned reference scripts in `reference/three-js/tsl-filters.html` retain
the official shader expressions and use seeded placement and explicit time.
`tests/browser/tsl-filters.spec.js` compares poses, controls and resize, enforces
steady-state GPU resource/geometry residency, and checks instanced draw counts
and SSAA sample counts. `tests/tsl_filters_gpu.rs` checks uniform-only inline
color changes, weighted alpha accumulation and camera restoration. Inspector
UI styling remains a limitation; hardware timing parity is not claimed.

Validation on 2026-09-20: 65 new image states passed the unchanged 6/255
channel / 0.5% pixel threshold, including multiple transition cycles and resize.
112 browser checks and all native tests passed; native/Wasm Clippy and formatting
also passed. FXAA's same-input isolation found only 2 pixels above threshold
at 512×512; enabling r186 lighting in the port reduced its end-to-end discrepancy
to at most 4 pixels across the tested square views. No threshold was relaxed.
Raw measurements, GPU resource counts and local PNG paths:
[`tsl-filters-comparison.json`](tsl-filters-comparison.json).

## Five fog and particle examples

| Gallery example | Implemented work |
| --- | --- |
| `webgpu_fog_height` | 100 instanced Phong columns, fragment world height/view depth fog, Orbit controls |
| `webgpu_sprites` | 200 GPU billboards with individual rotation/scale and linear fog |
| `webgpu_instance_sprites` | 10,000 sprites in one draw, GPU rotation, alpha test, pointer camera and attenuation toggle |
| `webgpu_tsl_galaxy` | 20,000 particles in one draw, resident random attributes, GPU position/scale/color expressions |
| `webgpu_postprocessing_afterimage` | 50,000 GPU-animated particles in one draw and persistent HDR history with damp/enabled controls |

`tsl::sprites::SpriteNodeMaterial` compiles local center, scale, rotation and
size-attenuation expressions into a camera-facing vertex projection. It uses
ordinary plane geometry with `BufferGeometry.instance_count`; no per-instance identity
matrices or CPU-expanded billboards are needed. `instanced_attribute(binding)`
reads an explicitly bound `GpuBuffer` array of vec4s in vertex/fragment stages.
Callers supply at least one vec4 per instance. `instance_index()` now works in
these stages too. External texture bindings follow the attribute buffers.
Negative material output is clamped to zero, matching upstream node materials.
This is an unlit centered billboard subset; arbitrary Sprite centers, sprite
raycasting and sprite shadow/lighting nodes are not implemented.

`position_world()` and positive `view_z()` are fragment/output inputs used by
`exponential_height_fog_factor`. `AfterImagePass` applies the original
component-wise `max(new, old * damp * max(sign(old - 0.1), 0))` in two persistent
RGBA16Float targets. Resizing clears history. The gallery presents that result
directly, retaining both presentation bindings rather than copying into another
target. Disabling the effect presents the scene and pauses history updates.

The instanced-sprite scene has a transparent background upstream. Core's
`blit_premultiplied_srgb` unpremultiplies linear color, applies tone mapping and
sRGB encoding, then restores premultiplied alpha for the unorm canvas view.
Using an opaque black clear instead caused the original edge-brightness mismatch;
readback confirmed identical source texels and all six texture mip levels.
Non-matrix instances use explicit identity transforms/colors in the shader,
including on native Metal. Draw and deformation buffer initialization uses queue
uploads, avoiding detached Wasm views during mapped copies as memory grows.

`reference/three-js/tsl-particles.html` executes the pinned official scripts with
seeded initialization and explicit time. Each upstream RangeNode has its own
seed; only initial random data changes. Per-frame sprite rotation is set to the
corresponding 60 Hz pose for comparison. An opacity=1 node marks the height-fog
material dynamic because the r186 observer does not detect scene-level fog-node
uniform changes at a frozen camera pose. The shader's fog expression is retained.
UI changes and time seeking are applied together so each history sequence has the
same number of rendered frames. Pointer/Orbit comparisons settle damping before
capture; resize starts empty history on both implementations.

`tests/browser/tsl-particles.spec.js` compares poses, controls, pointer/Orbit
input and resize at the existing 6/255 channel / 0.5% pixel threshold. It also
checks draw/pass counts and requires zero warmed GPU resource creations and zero
geometry/deformation-input uploads, before and after resize. Particle movement
must not write vertex/index/storage buffers. Native tests cover camera-facing
projection from two axes, instance attributes, node alpha discard, history decay,
threshold/resize behavior and transparent output encoding. Hardware CPU/GPU
frame-time parity and Inspector styling remain unverified; these are partial ports.

Validation on 2026-09-20: all 42 new image states passed the unchanged threshold.
The largest fraction of pixels exceeding 6/255 was 0.078125% (height fog after
Orbit input and resize); sprites and afterimage had zero exceeding pixels in all
recorded states. The three particle workloads keep one scene draw each, and
warmed frames create no GPU resources or vertex/index/storage uploads. The 67
native tests (including doctests), all 138 browser regressions, native/Wasm Clippy
and formatting checks passed.
Raw measurements, resource counts and local PNG pair locations:
[`tsl-particles-comparison.json`](tsl-particles-comparison.json).
