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
and control-flow nodes, matrices, general atomic compute graphs, arbitrary
attributes/varyings or automatic render-graph scheduling. Lit surface hooks and typed
storage updates are described below. `select` evaluates both expressions;
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

## Five storage and instancing examples

The next five ports use the pinned r186 WebGPU originals:

| Gallery example | Workload and behavior |
| --- | --- |
| `webgpu_particles` | 2,000 smoke billboards plus 1,000 fire billboards, resident random ranges, UV rotation, lifetime/color/opacity, indirect fire draw, speed and Orbit controls |
| `webgpu_instance_mesh` | 1,000 Suzanne instances, upstream CPU matrix animation, GPU world-normal/random-color mixing and instance-count control |
| `webgpu_compute_points` | 300,000 native one-pixel point instances, GPU position/velocity updates, pointer reset and boundary controls |
| `webgpu_compute_particles` | 200,000 GPU simulated billboard particles, gravity/friction/bounce, pointer impulses, PCG colors and alpha-to-coverage circle edges |
| `webgpu_compute_texture_pingpong` | Two 512×512 RGBA16Float storage textures, signed initialization, five-tap GPU updates, periodic reseeding and ten-level GPU mip chains |

The new APIs are reusable independently of the browser scenes:

- `storage_element(binding, index)` reads typed storage in vertex, fragment and
  compute stages. `NodeMaterial::build_with_storage` and
  `SpriteNodeMaterial::build_with_storage` accept `(GpuBuffer, Type)` bindings.
  `instanced_attribute` remains the vec4-layout convenience used by existing ports.
  Buffer allocation/layout belongs to the caller: vec2 strides are 8 bytes;
  vec3/vec4 strides are 16 bytes, following WGSL array alignment.
- `compute::BufferCompute` compiles a list of typed `BufferStore`s with a guarded
  64-thread workgroup. It evaluates all right-hand sides before stores, allowing
  position/velocity updates from one snapshot. This is a per-invocation expression
  subset, without atomics, workgroup memory or a general statement/control-flow API.
  Callers must avoid cross-invocation read/write races and conflicting aliased
  buffer bindings. Uniform slots remain 16 vec4s.
- `compute::TextureKernel` binds existing write-only output and read-only input
  storage views. `texture_load` uses texel coordinates; vec2 coordinates permit
  negative offsets after conversion to signed integers. RGBA8Unorm, RGBA16Float
  and RGBA32Float kernels are supported. Bind separate kernels for each ping-pong
  direction; no texture copies or readbacks occur in the scene.
- `mipmap::MipGenerator` retains views, bindings and a filter pipeline for a
  filterable, renderable 2D texture. The caller schedules GPU mip generation after
  writes. Screen-space transmission reuses this same implementation. The HDR
  example retains both chains, including after canvas resize.
- `position_local`, `normal_world`, `normalize`, `fwidth`, `hash` and `shape_circle`
  provide the additional graph inputs/operators. World normal and local position
  are fragment inputs. `MaterialProperties::alpha_to_coverage` selects MSAA
  coverage; `shape_circle(true)` provides derivative-smoothed opacity for it.
- Shader materials on `Points` now use native WebGPU one-pixel primitives.
  `PointsMaterial` retains its existing sized-billboard path. The 300,000-point
  scene uses one resident vertex and one instanced draw, not triangle expansion.
  `GpuBuffer::zeroed` allocates simulation storage without a CPU zero array.

The official source fixture retains shader expressions and uses seeded RangeNode
initialization, explicit time and captured GUI controls. Integer `instanceIndex`
modulo/division intentionally follows the generated official WGSL, including
truncation of sqrt(200000) to 447 for the particle grid. Pointer impulses dispatch
on input, before the next simulation step. In `still=1`, `gallery_time` advances
one compute step; camera/input/resize-only redraws do not advance simulation.
These discrete simulations are not random-access time-seeking APIs. Single-touch
particle interaction and two-touch Orbit dolly/pan follow the original controls.

Validation keeps the existing 6/255 channel / 0.5% pixel threshold. Cases include
several times, parameter changes, pointer/settled Orbit input, resize, periodic
texture resets and sixty additional ping-pong steps. All 39 captured states pass;
the maximum fraction exceeding 6/255 is 0.007421875%. Smoke/fire, compute points
and HDR ping-pong have no pixels exceeding the threshold in these states.
Native checks cover typed update snapshots, partial workgroups, GPU render
bindings, negative HDR values, ping-pong, mip filtering and native point drawing.
Browser checks enforce retained resources before/after resize and compare actual
scene draw counts, compute dispatch sizes and render-pass counts with upstream.
Geometry stays resident; only changed instance matrices (as in the original),
small uniforms and GPU-generated simulation/texture data change each frame.
CPU/GPU frame-time parity and Inspector styling remain unverified; these are
partial ports. Raw image measurements, upload/API counts and PNG locations:
[`tsl-compute-comparison.json`](tsl-compute-comparison.json).

On 2026-09-20, the 11 new browser checks, 157 browser regressions and 28 related
native GPU/semantic tests passed, as did formatting, native/Wasm Clippy and pinned
asset/catalog checks. The old unported-gallery diagnostic test now uses Compute
Birds because Compute Particles is implemented.

## Lit surfaces, MRT and GPU geometry

`surface::SurfaceNodes` replaces position, base color, view-space normal,
roughness, metalness, emissive or Phong specular inputs while retaining the core
lighting path. `output` runs after lighting. `mx_noise_float` implements the
pinned MaterialX noise; `surface::normal_map` constructs a derivative tangent
basis from explicitly supplied UV coordinates. Geometry remains GPU resident.

`build_mrt` writes multiple outputs in one scene draw. `output()`, `normal_view()`,
`diffuse_color()` and `emissive()` address the resolved lit fragment. Attachment
formats can differ via `RenderTargetOptions::color_formats`. Environment
background outputs are configured with `Scene::background_outputs`.
`depth_effect` and `multisampled_depth_effect` sample a depth attachment on the
GPU; the multisampled version reads sample zero explicitly.

`bloom::Bloom` retains five HDR levels, separable Gaussian passes and a composite
pass. Threshold, strength and radius update uniforms. It accepts a custom input
node for emissive-only or per-object masked bloom. `MaterialProperties::lights`
selects ambient/punctual lights without duplicating scene geometry.

`vertex_index()` reads resident storage positions in the vertex stage.
`BufferCompute::new_workgroup_snapshot` evaluates reads across one workgroup
before writes using a storage barrier (1–64 elements); it is not a global barrier
or an atomic API. Larger independent vertex simulations use `BufferCompute::new`.

The additional official scenes cover rect-area lighting, raging sea, halftone,
skinning, depth texture, multiple render targets, custom point lights,
ShaderToy, flames, custom fog, selective lights, Phong lights, MRT, three Bloom
variants, storage buffers, compute geometry, tornado and MRT masks. ShaderToy
ports the two embedded shaders through `WgslFn`; arbitrary GLSL translation is
not implemented. Storage Buffer displays only its WebGPU pane. Inspector styling
and hardware timing equivalence remain outside the validated scope.

Validation: `tests/tsl_surface_gpu.rs`, `tests/browser/tsl-surface.spec.js` and the
pinned-script fixture `reference/three-js/tsl-surface.html`.

The 20-scene capture records 169 states in [comparison data](tsl-surface-comparison.json),
including the same 6/255 and 0.5% pixel threshold used by the previous batches.

## Texture dimensions, volumes and additional GPU operations

The next twenty gallery ports (runtime IDs 78–97) add multisampled renderbuffers,
layers, 2D texture arrays, Perlin/cloud volumes, computed 3D textures, compressed
arrays, centroid/sample/flat interpolation, texture gather, anamorphic bloom,
Earth, occlusion queries, instance uniforms, bokeh depth of field, multiple
elements/canvases, structured indirect drawing, instance points, custom cubemap
mipmaps and array/3D render targets. The cubemap mip example's original uses a
built-in material; its Rust port uses the common TSL reflection/sampling graph.

`NodeMaterial::build_with_texture_types` supports typed array, 3D and cube views.
`Texture::sample_array`, `sampling::{cube,volume,gather,gather_compare}`, and
`surface::bump_map` operate on GPU texture data. `GpuTexture::from_cube_rgba`
generates all six mip chains on the GPU; `from_cube_mipmaps` validates and uploads
caller-supplied levels. Compressed arrays retain a supported GPU block format.

`BufferGeometry::gpu_indirect` accepts a resident INDIRECT buffer, including one
written by Compute. `OcclusionQueries` resolves actual visibility queries
asynchronously. `SpriteNodeMaterial::build_points` uses viewport pixel dimensions.
`RenderTarget::set_load_color` allows successive viewport passes to preserve color.
Bokeh DOF uses separate near/far blur kernels and GPU compositing; anamorphic bloom
keeps the original full-resolution bright extraction and reduced-resolution blur.

The 3D render-target example needs the narrowly documented local wgpu patch in
`vendor/wgpu/PATCH.md`. It forwards the existing depthSlice field to WebGPU; it does
not insert an intermediate copy. `tests/browser/tsl-extended.spec.js` compares the
original scripts, GUI controls, camera movement and resize, and checks residency
and draw/dispatch workloads. GPU timing parity is not inferred from these checks.
Inspector presentation remains adapted, and entries remain marked partial ports.

## Path instances, image filters, 3D LUT and parallax

Runtime IDs 98–102 add the r186 WebGPU examples `instance_path`,
`postprocessing_sobel`, `postprocessing_smaa`, `postprocessing_3dlut` and
`parallax_uv`. The original controls, assets, camera interaction and animation
are ported through Rust/Wasm; Three.js is used only by the independent reference
fixtures and offline path preparation.

- `display::sobel` evaluates luminance gradients in the fragment shader.
- `smaa::SmaaPass` implements the original three-stage SMAA 1x Medium filter,
  using the original area/search lookup tables and resident intermediate targets.
- `lut::Lut3D` reads normalized `.CUBE`, `.3dl` and horizontal/vertical PNG
  strips; its effect uses a native GPU 3D texture and trilinear sampling.
  Linear-to-sRGB conversion can be fused into the same grading pass.
- `surface::parallax_uv` uses geometry tangents; `parallax_uv_frame` matches
  r186's shared derivative/normal-map context for geometry without tangents.
  `blend_overlay` supplies the ice surface's linear-color overlay operation.
- `EnvironmentMap::from_scene` captures six views and prefilters their HDR
  CubeUV atlas entirely on the GPU. Capture runs when the environment is created
  or explicitly refreshed, never as a CPU readback in the frame loop.

The path example retains 1,000 GPU instances; the smoke uses GPU vertex
animation and two-sided transparent rendering. LUT selection reuses nine
resident tables. `tools/tsl/prepare.py` also runs `prepare-next.py`, which copies
pinned assets, extracts SMAA tables and generates the static heart path.

`tests/browser/tsl-next.spec.js` compares the unchanged original shaders at
fixed times, parameter settings, camera orbit/pan/zoom and resized viewports.
It checks GPU geometry workload, attribute residency and stable resource counts.
`tests/tsl_next_gpu.rs` covers LUT parsing/interpolation, Sobel edges and GPU
scene capture. Results are recorded in [the comparison report](tsl-next-comparison.json).
Inspector styling and frame-time parity remain outside these checks; the gallery
entries retain their partial-port designation.

## Environment graphs, alpha hashing and chromatic aberration

Runtime IDs 103–107 add `webgpu_cubemap_mix`, `webgpu_cubemap_adjustments`,
`webgpu_materials_envmaps_bpcem`, `webgpu_materials_alphahash` and
`webgpu_postprocessing_ca`.

`SurfaceNodes::environment` supplies linear HDR radiance for PBR lighting.
`environment_direction()` and `environment_roughness()` expose the current
radiance/irradiance sampling context. Explicit `environment::reflect_vector()`
uses the resolved material normal without the default roughness bending;
`material_normal_world()` includes normal maps. `environment::pmrem` samples
resident cube-UV atlases, and `parallax_correct` intersects reflected rays with
the environment box. `EnvironmentMap::prefilter` prepares an atlas once;
`from_cube_hdr` accepts six image faces and `from_cube_scene` captures a scene
through a GPU CubeCamera. Both use GPU padding and convolution.

`surface::alpha_hash` implements the derivative-scaled Wyman hash/discard test
on the GPU. The gallery combines it with 27 native instances and the existing
SSAA pass. `display::chromatic_aberration` independently offsets RGB samples
from a resident postprocessing texture. HDR values survive the intermediate
sRGB conversion; clamping happens only at final display output.

The original-script fixture preserves the official `time` node's render-group
update frequency. For Cubemap Adjustments it also places scene-node controls
in that group: r186's `NodeMaterialObserver.containsNode` inspects material
properties but misses `scene.environmentNode`, leaving those controls stale
on stationary meshes. This fixture correction changes uniform refresh only,
not shader expressions, textures or PMREM generation. Other adaptations seed
random initial data, supply the test clock and capture Inspector controls.

Image, interaction and GPU residency/workload evidence is in
[the environment comparison report](tsl-environment-comparison.json).
Inspector appearance and GPU frame-time equivalence remain unclaimed.

Ordinary JPEG textures use browser color management, including the Adobe RGB
profile in the BPCEM floor's roughness image. glTF retains its separate,
profile-independent decoding path. Ignoring that profile changed roughness
values and therefore both direct lighting and blurred reflections.

## PMREM, lightmaps and depth/bloom effects

The additional ports are `webgpu_pmrem_cubemap`, `webgpu_pmrem_scene`,
`webgpu_materials_lightmap`, `webgpu_postprocessing_dof_basic`, and
`webgpu_postprocessing_lensflare`.

- `uv1()` reads the secondary geometry UV channel in material fragment graphs.
  `SurfaceNodes::light_map` supplies linear baked irradiance before diffuse
  BRDF evaluation. It remains distinct from emission and direct lighting.
- `display::box_blur` runs the original square sampling kernel on the GPU,
  including optional premultiplied-alpha filtering. DoF Basic combines this
  with the scene's resident depth attachment, world-space click-to-focus,
  Neutral tone mapping and FXAA after sRGB conversion.
- `display::lensflare` samples the original thresholded, weighted ghosts.
  The example uses emissive MRT, five-level HDR Bloom, quarter-resolution
  RGBA8 flare target, a full-resolution HDR RTT copy, and two full-resolution
  HDR Gaussian passes. Their
  direction multiplier is 8 and their default sigma is 4.
- PMREM Cubemap uses six original Pisa HDR faces. PMREM Scene captures the
  six colored spheres and Park3Med background on the GPU, then samples the
  resulting atlas with adjustable roughness. Background cube sampling uses
  explicit level zero, matching the official background-node context.

The camera near/far values remain those of each original example during
Orbit updates. This matters for depth reconstruction and close-up viewing.
The two UltraHDR assets use lossless half-float PNG containers; their pixels
are decoded offline by the pinned official loader, never tone-mapped to LDR.
To regenerate, run `python3 tools/tsl/prepare-lighting.py`, start
`python3 tools/serve.py`, then run `node tools/tsl/prepare-lighting-hdr.mjs`.

Validation: `tests/tsl_lighting_gpu.rs` and
`tests/browser/tsl-lighting.spec.js`; recorded comparisons are in
[tsl-lighting-comparison.json](tsl-lighting-comparison.json).
Inspector appearance and hardware timing equivalence remain unclaimed.

## Framebuffer effects, soft particles and FSR1

Runtime IDs 113–117 add `webgpu_backdrop`, `webgpu_backdrop_area`,
`webgpu_refraction`, `webgpu_particles_soft`, and `webgpu_upscaling_fsr1`.

`viewport::color`, `depth`, and `screen_uv` read resident GPU framebuffer
snapshots. `safe_uv` rejects refracted coordinates occluded by foreground depth;
`hash_blur` runs the original 45-sample stochastic kernel. Color is captured in
transparent draw order, including between the back and front sides of a
transparent double-sided mesh. Depth is captured before that mesh. MSAA depth
uses sample zero, matching the official depth resolve. Targets must retain
resolved color and their multisampled attachments; only 2D targets are supported.
The renderer creates snapshots only for materials that need them and reuses
textures/pipelines across frames. It does not redraw the scene or read back pixels. Extra framebuffer bindings
are reserved only for materials using viewport nodes, preserving the four
external texture slots available to ordinary TSL materials.

`SurfaceNodes::backdrop` accepts RGBA: RGB replaces diffuse lighting and A mixes
it with the material's diffuse contribution. Specular highlights remain lit.
`viewport::soft_particles` applies the original depth-intersection contrast curve.
The smoke scene uses 50 GPU instances and four resident randomized attributes;
Wasm updates controls/time, not per-particle positions. Michelle uses GPU skinning.

`fsr1::easu` and `fsr1::rcas` are separate GPU passes with 12- and 5-tap kernels.
The example renders Littlest Tokyo into an adjustable-resolution MSAA HDR target,
then upsamples and sharpens at display resolution. The Bilinear control uses the
same source scene. EASU/RCAS also have a same-input comparison against the official
HDR textures, separating kernel accuracy from scene rasterization differences.

The original-script fixture fixes time (including implicit `oscSine` time), seeds
RangeNode initialization independently, and captures GUI controls. Soft-particle
uniform controls use render-group refresh so parameter edits at a fixed test time
reach the GPU. Pipeline/target warm-up uses fixed poses; the FSR1 comparison advances its
animated glTF once and rewinds to zero before measuring, so the reference
initializes its animated bindings before the zero-time image. Original shader
math and assets are preserved. Reference scripts use their actual light world
transforms: the camera's default spot offset is (0,1,0), and the unattached smoke
spot target has an identity world transform.

See [the comparison report](tsl-viewport-comparison.json) and
`tests/browser/tsl-viewport.spec.js`. Inspector styling and frame-time equivalence
remain unclaimed.
