# TSL graph, compute water, SSGI, SSS, SSR, fog scattering, backdrop water, volumetric fog, skinning instances and retargeting

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_tsl_graph` | 394 | `tsl_graph.rs` | Four ShaderBall.glb models with the page's TSL graph materials (physical, standard, Phong and transparent basic), the monochrome_studio PMREM, the grid ground, 4× MSAA and NeutralToneMapping |
| `webgpu_compute_water` | 395 | `compute_water.rs` | The 128 × 128 height field in ping-pong storage buffers, the pointer raycast ripple, 100 compute-floated Draco ducks, the double-sided water and the blouberg_sunrise sky |
| `webgpu_postprocessing_ssgi` | 396 | `ssgi.rs` | The Cornell box with a point shadow, the MRT scene pass, SSGINode's AO and one-bounce GI, the composite and TRAA, and the AO, GI and direct views |
| `webgpu_postprocessing_sss` | 397 | `sss.rs` | The Draco nemetona statue, the velocity and depth pre-pass, SSSNode's screen-space shadows in the directional light's shadow context, TRAA, and the shadow-map-only and SSS views |
| `webgpu_postprocessing_ssr` | 398 | `ssr.rs` | steampunk_camera.glb over the metallic floor, the MRT pass, SSRNode, the five-mip roughness blur, the composite, SMAA's three passes and ACES, with the blur quality and binary refinement variants |
| `webgpu_custom_fog_scattering` | 399 | `fog_scattering.rs` | Ten TreeGenerator variants grown on the CPU at load and drawn as 156 seeded instances and two hero trunks in FogExp2, the half-resolution Gaussian blur mixed by the fog factor, FirstPersonControls and 4× MSAA |
| `webgpu_backdrop_water` | 400 | `backdrop_water.rs` | The skinned, dancing Michelle with the sun's shadow, 100 floating ice spheres, the caustic pillar, the water's backdrop refraction over copies of the frame's color and depth, and the depth-driven blur |
| `webgpu_postprocessing_fog` | 401 | `fog_volume.rs` | The Lucy statue (PLY, computed normals) under the SunLight addon's two fitted cascades, the 96³ tri-noise volume ray-marched at reduced resolution, and the JBU, Gaussian and raw denoisers |
| `webgpu_skinning_instancing_individual` | 402 | `skinning_instances.rs` | Thirty Michelles with their own animation times, proportions and belly morph, skinned in one compute pass and drawn instanced, the SunLight cascades on a shadow-catcher ground and the RoomEnvironment PMREM |
| `webgpu_animation_retargeting` | 403 | `retargeting.rs` | Michelle's SambaDance retargeted onto the Soldier with SkeletonUtils.retargetClip, both skinned, over a reflector floor under the light-speed background |

All ten are WebGPU examples compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`).

## Port notes

### Generated WGSL

As in [compute-examples.md](compute-examples.md), each port runs the WGSL
three.js r186 generates for its page, captured from `createShaderModule` on
the reference page with the pipelines, bind groups, passes and uniform
writes. Uniform structs are packed by name from each module's own struct.
The modules run unchanged with automatic layouts, except that the water and
skinning compute kernels drop the subgroup built-ins they declare but never
use. Build-time constants that the page changes are each a captured variant:
SSR's binary refinement, the blur quality (the loop bound), the fog
denoisers, ground and MSAA depth reads, and the instance counts of the fog
scattering trees.

### TSL graph (394)

The fixture applies the page's graph file directly, as the page does when no
edited graphs are stored, since the replaced Inspector never starts the TSL
Graph extension. The ShaderBall's KHR_mesh_quantization vertices are uploaded
as stored (snorm16 and unorm16 in 20-byte strides), as three.js does. The
graph editor itself is not reproduced.

### Compute water (395)

The fixture seeds the page's SimplexNoise with the seeded Math.random and
skips appending the replaced Inspector's element. Each requested frame counts
toward the next height step, every 7 − speed frames. The pointer raycast
ripple and the orbit follow the page's pointer handling.

### SSGI and SSS (396, 397)

- **Frame ID.** SSGINode and SSSNode read frameId. The fixture stops the
  renderer's own loop once ready and restarts frameId at 12000, a multiple of
  12 past every frame the loading loop rendered. Both sides then advance it
  per requested frame. The first frame after loading does not advance it.
- **SSS seed.** SSSNode assigns frame.frameId over its uniform, so each build
  of the node inlines that frame's ID into the shader. The port rebuilds the
  SSS pipeline with that seed when the output or temporal filtering changes.
- **SSS velocity.** Read back from the original: the pre-pass's previous
  camera view and its current projection keep the first frame's unjittered
  values, while the previous projection follows the camera. A camera move or
  resize therefore leaves a velocity, and the port writes the same matrices.
- **Pixel ratio.** The SSGI page never calls setPixelRatio, so it renders at
  a pixel ratio of 1, as does the SSR page (398), whose call is commented out.

### SSR (398)

The RoomEnvironment PMREM is the shared GPU capture of the room
(`room_environment.rs`) and matches the page's. The SSR and blur pipelines
for every binary-refinement and blur-quality value are built at load, so
switching them creates no resources.

### Fog scattering (399)

TreeGenerator is ported in f64 with three.js's Vector3 and Quaternion
operation order and the mulberry32 generator. The placements follow
MathUtils.seededRandom(25). FirstPersonControls move only as time advances:
a drag at a fixed time leaves the view unchanged, in both runtimes.

### Backdrop water (400)

- **Clock.** The fixture drives THREE.Timer by the example clock, so the
  mixer, the ice spheres' motion and the TSL time follow it.
- **Ice.** Each sphere's height follows sin(elapsed + Object3D.id). The ids
  start at 19, as the page creates its objects.
- **Auto-rotation.** OrbitControls.update() turns by one auto-rotation step on
  every call while no pointer is down: in init(), each frame, and in the
  wheel handler. The port takes the same steps.
- **Backdrop.** The framebuffer's color and depth are copied after the opaque
  objects, as viewportSharedTexture and viewportDepthTexture do.

### Volumetric fog (401)

- **Lucy.** PLYLoader's positions are scaled by 0.0024, and computeVertexNormals
  accumulates the face normals into the Float32 attribute as three.js does.
- **SunLight.** The two cascades are refitted to the view frustum each frame
  as SunLightShadow.updateMatrices does: practical splits, bounding spheres
  snapped to the texel grid in the inset 4096 × 2048 atlas, and the caster
  ceiling.
- **Noise.** The 96³ tri-noise texture is generated once on the CPU, as the
  page does.
- **Fixture.** The GUI stand-in gains show() and hide() for the denoiser
  controls.

### Skinning instances (402)

Each frame, as the page does, the mixer is set to each instance's time on the
CPU (the mixer gained `setTime` for this), the proportion bones are rescaled,
and the bone matrices and the foot-grounded instance matrix are written for
that instance. One compute pass then skins all instances' vertices with the
belly morph into a storage buffer, which the shadow and scene passes read.

### Retargeting (403)

- **Clip.** SkeletonUtils.retargetClip runs once at load on simulations of
  both glTF node trees with three.js's matrix semantics. The source pose is
  sampled by a mixer per frame. The loader's load-time world matrices stay
  stale where the page never updates them, and the port keeps them stale too.
  The target keeps its last retargeted state, and the source bones are
  restored, as uncacheAction does.
- **Culling.** Each SkinnedMesh's bounding sphere is computed from the
  skinned vertices on its first render, as computeBoundingSphere does. With
  it, the reflection culls the visor and the main view keeps it.
- **Reflector.** The virtual camera and its oblique clip plane follow
  ReflectorNode. The floor is hidden in the reflection.
- **Helpers.** The skeleton helpers are not reproduced.

## Comparison

Captures use the page's fixture clock, seeded Math.random, the same controls
and a 640 × 400 resize. The thresholds are the defaults (0.5% of pixels over
6/255, mean 0.6/255) except:

- **MSAA.** Without MSAA, 394 and 395 match exactly in every state. With 4×
  MSAA their edges differ for a reason not identified. Those comparisons use
  MSAA-only bounds: 394 at 2% / 0.5 and 395 at 31% / 6.2 (the wireframe
  lines).
- **Exact matches.** 396–401 and 403 match exactly in every state, with and
  without MSAA. 402 is within 0.2% of pixels.

## Performance evidence

- **Residency.** Warmed cycles of time, controls, parameters and resize
  create no GPU resources. SSS leaves its parameters out of the cycle,
  because each SSS rebuild bakes a new seed into a new pipeline, as SSSNode
  does. The volumetric fog keeps the targets of each resolution scale it
  visits, so revisiting a scale creates none.
- **Simulations and skinning.** The water height field and ducks, and the
  skinned instance vertices, stay on the GPU. Only uniforms and, for the
  skinned characters, the bone and instance matrices are written per frame,
  as in the originals.
- **CPU work kept as in the originals:**
  - Animation sampling and the SunLight cascade fitting each frame.
  - The tree, tri-noise and retargeted-clip generation once at load.
- **Uploads.** Models, images and environment maps are decoded and uploaded
  once. No vertex or index data is written in steady frames.

No GPU timing parity is claimed.

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
COMPUTE_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
```
