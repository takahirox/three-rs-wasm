# Gaussian splats, water, custom fog, VXGI, GTAO, shadow-map array, compute rasterizer, building generator, loft geometry and sculpting

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_gaussian_splat` | 404 | `gaussian_splat.rs` | The lion (SPZ v3), millipede (SPLAT) and tomatoes (SPZ v4, zstd) splats decoded once, the GPU counting sort when the view turns, the spherical-harmonic colour pass when the camera moves, and the instanced splat quads |
| `webgpu_water` | 405 | `water.rs` | The Draco pool.glb with its transmissive and emissive props, the moonless_golf UltraHDR sky, four floors, Water2Mesh's flowing normal maps with the refraction copy and the ReflectorNode view, the emissive MRT through BloomNode, ACES and FXAA |
| `webgpu_custom_fog` | 406 | `custom_fog.rs` | TerrainGenerator's eroded 512² terrain and ForestGenerator's 500,000 instanced trees generated once, the page's valley fog node, the SkyMesh PMREM rebaked when the sun moves, the on-demand 4096² shadow and FirstPersonControls |
| `webgpu_vxgi` | 407 | `vxgi.rs` | The Cornell box voxelized once into a 72 × 56 × 72 opacity volume and its mips, the radiance volume relit when the injection settings change, the cone-traced AO and GI, the scene pass, TRAA and the output views |
| `webgpu_postprocessing_ao` | 408 | `gtao.rs` | The gallery room with the Draco Tennyson bust, three spotlights, the RoomEnvironment PMREM, GTAONode at half resolution with its magic-square noise, TRAA and NeutralToneMapping |
| `webgpu_shadowmap_array` | 409 | `shadow_array.rs` | Instanced columns, boxes, spheres and tori, the batched forest, the torus knot, TileShadowNode's 2 × 2 tiles in a 4096² depth array and the tile helpers |
| `webgpu_compute_rasterizer` | 410 | `compute_rasterizer.rs` | 160,000 teapots in seven LODs through the five compute passes (clear, frustum and LOD culling into the work queue, indirect dispatch, the atomic software rasterizer, the hardware queue's indirect draw), the visibility resolve and the hardware fallback |
| `webgpu_generator_building` | 411 | `generator_building.rs` | SkyscraperGenerator's tower generated when a building parameter changes, its partId-shaded material, the SkyMesh sky baked into the PMREM for the time of day, the SunLight cascades, the shadow-catching ground and the auto-rotating orbit |
| `webgpu_geometry_loft` | 412 | `geometry_loft.rs` | Twenty LoftGeometry pieces and two circles generated once, their seventeen node materials, the SunLight's 4096² cascades, the RoomEnvironment PMREM, the vignette background, the turning group and the sections view |
| `webgpu_sculpt` | 413 | `sculpt.rs` | The Sculptor addon's dynamic-topology strokes on the CPU with its nine tools, the attribute update ranges, the brush cursor and the damped orbit |

All ten are WebGPU examples compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`).

## Port notes

### Generated WGSL

As in [compute-examples.md](compute-examples.md), each port runs the WGSL
three.js r186 generates for its page, captured from `createShaderModule` on
the reference page with the pipelines, bind groups, passes and uniform
writes. Uniform structs are packed by name from each module's own struct. The
modules run unchanged with automatic layouts, except that the compute kernels
of 407 and 410 drop the subgroup built-ins they declare but never use. Shaders
shared with earlier ports are included from those ports' directories, byte for
byte.

### Gaussian splats (404)

The loaders decode once at load, as SPLATLoader and SPZLoader do (zstd for the
SPZ v4 tomatoes). The 4096-bin counting sort (reset, histogram, prefix and
scatter) runs only when the view direction turns past the page's threshold,
and the spherical-harmonic colours only when the camera moves. Switching the
source reuses the decoded splats it has already loaded. The marker shown at a
clicked splat is not reproduced.

### Water (405)

- **Clock.** WaterNode advances its flow by NodeFrame.deltaTime. The fixture
  drives NodeFrame's update by the example clock and stops the renderer's own
  loop, so no display frame takes a step.
- **Floors.** Each floor has its own object uniforms, as the page's meshes do.
- **Resize.** After a resize r186 copies the transmission source at the new
  size from a target still at the old size and fails validation. The
  comparison leaves the resize out.

### Custom fog (406)

TerrainGenerator and ForestGenerator are ported in f64 with the Float32Array
stores, mulberry32 and V8's Math.hypot, and three.js's matrix composition
order. The sky PMREM and the on-demand shadow are rebuilt only when the sun's
parameters change or the terrain regenerates, as the page's
`shadow.needsUpdate` does.

### VXGI (407)

The scene's triangles are collected once on the CPU and split along their
longest edge until no edge exceeds the voxel limit, as VXGIVolume does. The
volume is voxelized once. The radiance volume is re-injected only when the
light or the injection settings change. VXGINode turns its cones by
`frameId % 64`, so the fixture stops the loading loop and restarts frameId at
12800. Both sides then advance it per requested frame. The page renders at a
pixel ratio of 1. The voxel view and the directional radiance are not
reproduced.

### GTAO (408)

GTAONode rotates its slices by frameId. The fixture restarts it at 12000, as
for SSGI and SSS. The spot lights' uniform numbering skips one value in the
first light, as the generated shader does. The SSAO mode, the sample count (a
shader constant) and the transparent mesh are not reproduced.

### Shadow-map array (409)

Each layer of the depth array draws only the casters with its own tile
camera, as r186's per-layer render bundles do. The batched trees are sorted
front to back on the CPU into the indirect texture per camera, as BatchedMesh
does. The tile helpers and the camera helper follow one frame behind, since
the page updates them after rendering.

### Compute rasterizer (410)

The seven TeapotGeometry LODs, the meshlet IDs, the 64-triangle chunk bounds
and the instance placements are packed once at load into storage buffers.
Each frame runs the five compute passes, the indirect dispatch and the
indirect hardware draw. Resizing reallocates only the visibility buffers. The
fixture's GUI stand-in gains addEventListener for the Mode control. The page's
output pass after the quad is overwritten by the final output pass and is not
repeated.

### Building generator (411)

SkyscraperGenerator is ported in f64: the seeded style, the footprint and its
faces, the authored modules (ShapeGeometry with Earcut, ExtrudeGeometry with
WorldUVGenerator, LatheGeometry), the shells and the bake. Three.js's geometry
transforms are followed where the generator applies them: normal matrices,
renormalized normals and computeVertexNormals through the Float32Array. Its
output matches the original bit for bit for the page's settings and for the
arcade and storefront variants checked. OrbitControls.update() turns by one
auto-rotation step at init and per frame. A resize redraws the frame without
taking a step.

### Loft geometry (412)

LoftGeometry, its caps and every section builder (the SplineCurve profiles and
tangents included) match the original bit for bit. The materials' uniforms
are filled from each generated shader's structs by how the shader uses each
field. The group turns by 0.001 rad per requested frame, and a resize redraws
without turning it. The wireframe view is not reproduced.

### Sculpting (413)

- **Sculptor.** SculptorMesh (welding, vertex and face rings, the octree),
  SculptorTools (subdivision, decimation, smoothing and the nine tools) and the
  stroke code are ported with the addon's typed-array stores and order of
  operations. For the same pointer path the mesh matches the original bit for
  bit: every tool was checked against the addon outside the browser.
- **Uploads.** The changed vertices reach the GPU as the addon's attribute
  update ranges (merged across gaps of 96 components and capped at eight). The
  index is rewritten when the topology changes. Buffers are recreated only when
  the addon reallocates its arrays.
- **Events.** Pointer events are handled before the next frame renders, in the
  page's listener order: the Sculptor, the cursor, then OrbitControls. The
  cursor follows OrbitControls' change events, which fire only past the
  controls' EPS.
- The wireframe view and the GLB export are not reproduced.

## Comparison

Captures use the page's fixture clock, seeded Math.random, the same controls
and a 640 × 400 resize. The thresholds are the defaults (0.5% of pixels over
6/255, mean 0.6/255) except:

- **Custom fog.** The first frame after loading differs in sparse forest
  pixels (0.35% without and 0.54% with MSAA). Every later state matches
  exactly. The comparison uses 0.6% / 0.2.
- **Water.** The resize is left out, as above.

The largest fractions of differing pixels over all states (DPR 1, with and
without MSAA) are 0.005% for 404, 0.16% for 405, 0.01% for 407, 0.02% for 408,
0.32% for 409, 0.006% for 410, 0.12% for 411 and 0.003% for 412. Sculpting
(413) matches exactly in every state.

## Performance evidence

- **Residency.** Warmed cycles of time, controls, parameters and resize
  create no GPU resources. The building generator regenerates its tower on a
  building parameter, as the page does, so its cycle leaves those parameters
  out. The sculpting cycle covers hovering and orbiting, since strokes change
  the mesh.
- **GPU state.** The splat sort, the voxel volumes, the compute rasterizer's
  queues and visibility buffers and the shadow arrays stay on the GPU.
- **CPU work kept as in the originals:**
  - The splat decoding, the terrain and forest, the voxelization triangles,
    the teapot LODs, the loft geometry and the tower generation, once (or per
    regeneration).
  - The SunLight cascade fitting and BatchedMesh's tree sorting each frame.
  - The sculpting strokes on the CPU, as the Sculptor addon does them.
- **Uploads.** No vertex or index data is written in steady frames. Sculpting
  uploads only the addon's update ranges. The shadow-map array rewrites
  BatchedMesh's indirect texture each frame, as the original does.
- **Draws.** The workload test counts the original's render-bundle draws (the
  per-layer shadow passes) where its passes execute them, and the port's draws
  match.

No GPU timing parity is claimed.

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
COMPUTE_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
```
