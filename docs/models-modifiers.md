# MDD, edge split, 3DS, teapot and instance scattering

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/models_modifiers.rs`, with the teapot patch data in
`src/browser/teapot_data.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_loader_mdd` | 223 | A box with the MDD file's four morph targets, played by AnimationMixer on a loop |
| `webgl_modifier_edgesplit` | 224 | The Cerberus OBJ, merged and edge-split, and all five controls, on-demand rendering |
| `webgl_loader_3ds` | 225 | The 3DS portal gun with its Phong material and normal map, TrackballControls |
| `webgl_geometry_teapot` | 226 | The Bezier-patch teapot, all seven controls and the six shadings, on-demand rendering |
| `webgl_instancing_scatter` | 227 | 2,000 instanced flowers sampled on a torus knot, aging every frame, the count and distribution controls |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### MDD

- **Parsing.** The MDD reader follows `MDDLoader`: big-endian frame times and
  absolute morph positions.
- **Animation.** The clip's one-hot keyframes are evaluated as `AnimationMixer`
  does: LoopRepeat over the last key time, with linear interpolation between
  influences. The morph blend runs on the GPU.
- **Output.** WebGL's `MeshNormalMaterial` writes its packed normals without
  the output encoding, and the port's output is raw as well.
- **Uploads.** The only per-frame write is the 16-byte influence pose, the
  counterpart of WebGL's influence uniforms.

### Edge split

- **Parsing and merging.** The OBJ is parsed by the port's `OBJLoader`, then
  merged with `mergeVertices`: a 1e-4 hash over position, normal and uv.
- **Modifier.** `EdgeSplitModifier.modify` is ported with the original's
  quirks:
  - attribute arrays are sized by the index count;
  - `original || currentGroup[ 0 ]` treats index 0 as falsy;
  - the kept normals are marked at index-array positions.
- **Controls.** The damped OrbitControls step only on input, as there is no
  animation loop. They rotate at 0.35 speed.
- **Residency.** Each geometry and each material state is built once. Each
  shown combination is its own resident node, since swapping a node's geometry
  or material would rebuild its draw resources. The original rebuilds the
  geometry on every change.

### 3DS

- **Parsing.** The chunk reader follows `TDSLoader`:
  - the master scale;
  - materials with setRGB colors taken as linear, and shininess;
  - maps as NoColorSpace textures;
  - points, faces, uvs, material groups;
  - the local mesh matrix, whose inverse is applied to the geometry and whose
    decomposition becomes the mesh transform.
- **Material.** The example's `specular.setScalar( 0.1 )` and the shared
  normal map are applied.
- **Controls.** TrackballControls use the default pan speed 0.3. They step as
  60 fps steps of example time, in the port and in the reference fixture alike.

### Teapot

- **Geometry.** `TeapotGeometry` is ported with the original's patch table and
  Bezier basis products, degenerate-triangle culling, and the zero-filled
  index tail.
- **Shadings.** All six shadings are the original's materials. The reflective
  shading keeps Phong lighting, then multiplies it by the per-fragment
  reflection sample of the Pisa cube, as envmap_fragment's MultiplyOperation
  does. It shows the cube as the background, drawn at the far plane as
  `backgroundCube` does.
- **Background.** The other shadings clear to black, since `render()` sets the
  background to null.
- **Residency.** Each tessellation and flag combination is built once. Each
  shown combination is its own resident node.

### Scattering

- **Sampling.** `MeshSurfaceSampler` is ported: Float32 face weights and
  cumulative distribution, binary search, and barycentric position and normal.
  It covers the unweighted and uv-weighted distributions. The seeded random
  stream covers the palette, ages and samples in the original's order.
- **Per-frame work.** The per-frame particle aging, rescaling of the Float32
  instance matrices and resampling are the original's CPU work. They run as
  60 fps steps of example time. `scene.rotation` turns the point light with the
  meshes.
- **Instance uploads.** Each frame writes the instances into the renderer's
  resident draw data: 319,960 bytes of transforms and colors. The original
  uploads 256,000 bytes of instance matrices; its instance colors stay static.
- **Not ported.** The `resample` button.

Stats styling and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/models-modifiers.html` executes the pinned sources. It uses
seeded randomness (including the sampler's generator), and a Timer on the
example clock. The suite covers:

- time steps;
- every control;
- orbit and trackball drags, pans, wheels and key holds;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| MDD, MSAA off / on | 0% / 0.00, 0.207% / 0.07 | 0% / 0.00, 0.104% / 0.04 |
| Edge split, MSAA off / on | 0.088% / 0.04, 0.652% / 0.20 | 0.037% / 0.02, 0.322% / 0.10 |
| 3DS, MSAA off / on | 0.059% / 0.02, 1.002% / 0.23 | 0.019% / 0.01, 0.483% / 0.11 |
| Teapot, MSAA off / on | 0.486% / 0.16, 7.644% / 4.98 | 0.382% / 0.14, 7.544% / 4.89 |
| Scattering, MSAA off / on | 0.002% / 0.00, 3.801% / 0.79 | 0.002% / 0.00, 1.970% / 0.41 |

### MSAA

All five scenes match within the ordinary threshold with MSAA off. With 4×
MSAA, the differences lie on silhouettes, the teapot's wireframe shading, and
the scattered stems and blossoms.

The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| Edge split | 0.75% / 0.25 |
| 3DS | 1.2% / 0.3 |
| Teapot | 8.5% / 5.5 |
| Scattering | 4.5% / 0.95 |

MDD keeps the ordinary threshold with MSAA as well.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - one morphing box;
  - one edge-split draw of 100,623 indices;
  - one 3DS draw of 12,351 indices;
  - one teapot draw of 42,840 indices;
  - the torus knot and the two 2,000-instance flower draws.
- **Uploads.** After a warm pass, no example uploads geometry or texture data.
  The exceptions are the scattering scene's instance data and the MDD pose,
  both listed above.
- **Warmed cycles.** Warmed cycles of time, input, keys and control changes
  (ending where they began) create no GPU resources.
- **Idle.** The teapot scene stays idle without input.

No GPU timing parity is claimed. Full measurements are in
[models-modifiers-comparison.json](models-modifiers-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js models-modifiers.spec.js
MODELS_DPR=2 npx playwright test -c playwright.gallery.config.js models-modifiers.spec.js
```
