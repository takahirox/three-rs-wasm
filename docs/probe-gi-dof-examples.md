# Compute rasterizer IBL, progressive shadow map, Ready Player Me retargeting, SSR denoise, simple GI, DoF 2, subsurface scattering, GPGPU birds, VRML and USDZ

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_compute_rasterizer_ibl` | 414 | `compute_rasterizer_ibl.rs` | 15,625 turning DamagedHelmets in Meshopt LODs and 64-triangle meshlets, the compute passes (clear, frustum, LOD and HZB occlusion culling, indirect dispatch, the atomic rasterizer, indirect draw arguments), the visibility resolve under the royal_esplanade PMREM, the hardware queue and the next frame's depth pyramid |
| `webgpu_shadowmap_progressive` | 415 | `shadowmap_progressive.rs` | ProgressiveLightMapGPU accumulating four random lights' soft shadows into a 1024² float lightmap over the ground and the ShadowmappableMesh model, the potpack uv layout, the blurred seams, fog and the two TransformControls |
| `webgpu_animation_retargeting_readyplayer` | 416 | `retargeting_readyplayer.rs` | The Mixamo dance retargeted onto ten Ready Player Me avatars and two source characters, GPU skinning, the hemisphere and two directional lights, and the reflector floor |
| `webgpu_postprocessing_ssr_denoise` | 417 | `ssr_denoise.rs` | The dungeon under the SunLight cascades and the quarry HDR, stochastic SSR, the temporal reprojection, the recurrent denoise fed back into SSR, the grade, TRAA and sharpening |
| `webgl_simple_gi` | 418 | `simple_gi.rs` | SimpleGI's three bounces of per-vertex 32 × 32 views of the scene, 32 vertices per frame, into the torus knot's vertex colors |
| `webgl_postprocessing_dof2` | 419 | `dof2.rs` | Falling leaves, flat-shaded heads and balls in Phong with the Bridge2 reflection, the raycast autofocus, the depth pass and BokehShader2 |
| `webgl_materials_subsurface_scattering` | 420 | `subsurface.rs` | The turning Stanford bunny with SubsurfaceScatteringShader under an ambient, a directional and two range-limited point lights |
| `webgl_gpgpu_birds_gltf` | 421 | `birds_gltf.rs` | 4,096 flocking Parrots: the GPUComputationRenderer velocity and position passes, the baked morph animation and the instanced birds in MeshStandardMaterial with fog |
| `webgl_loader_vrml` | 422 | `vrml.rs` | The sixteen VRML samples (meshes, background spheres, lines, points and textures) with the damped orbit that resets on each load |
| `webgl_loader_usdz` | 423 | `usdz.rs` | The saeukkang model lit by the venice_sunset PMREM, the blurred background and ACES at exposure 2 |

The four WebGPU examples are compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`). The six WebGL examples have no
WebGPU counterpart in r186 and are compared against WebGLRenderer
(`tests/browser/texture-volumes.spec.js`).

## Port notes

### Generated WGSL and WebGL programs

The WebGPU ports run the WGSL three.js r186 generates for each page, as in
[compute-examples.md](compute-examples.md). A new helper (`wgsl_bind.rs`)
binds those modules by the names three.js gives their resources. It skips
declarations no entry point uses, such as the samplers three.js declares beside
textures it only loads.

The WebGL ports translate the GLSL that WebGLRenderer assembles for each
material to WGSL, keeping the same expressions and order: Phong, physical,
basic, line and point shading, BokehShader2, the GPGPU shaders and the
background programs. Two differences come from the APIs. WebGL's `dFdy` runs
up the screen, so the flat normals negate WGSL's `dpdy`. Textures uploaded
with `flipY` are sampled at `1 - v`. The canvas is tone mapped (where the page
asks) and sRGB-encoded in the shader, and transparent objects blend on the
encoded canvas, as in WebGL.

### Compute rasterizer IBL (414)

The page builds the helmet's LODs and meshlets with meshoptimizer 1.1 at load.
`tools/tsl/prepare-rasterizer-ibl.mjs` runs the same library code on the same
geometry and stores the result, which the port packs into the page's storage
buffers. The HZB kernels are generated per level from the page's template, and
each frame culls against the previous frame's pyramid. The page's first frame
is computed with the matrices from before the renderer switches the camera to
WebGPU's clip space, and the port does the same.

### Progressive shadow map (415)

The lightmap is two rgba32float targets that alternate each frame, with four
shadow maps (rgba8 color and 24-bit depth). potpack lays out the second uv
set, as addObjectsToLightMap does, and the fixture's seeded Math.random moves
the lights. The shadow camera's target trails one update behind, as the
page's does. The TransformControls gizmos are drawn by the engine over the
scene target. The lightmap debug view is not reproduced.

### Ready Player Me retargeting (416)

FBXLoader's parse of `mixamo.fbx` (nodes with the world matrices the loader
leaves, the skinned meshes and clip 0) is baked by
`tools/tsl/prepare-mixamo.mjs`. The retargeted clip is baked at load, as the
page does with SkeletonUtils.retargetClip. The FBX has bones with duplicate
names, and the port binds each name to its first node in traversal order, as
the page's lookup does. Culling uses each skinned mesh's bounding sphere from
its first culling.

### SSR denoise (417)

- **Order.** Each frame runs the cascades' shadow atlas, the scene MRT (color,
  packed normal with roughness, velocity, diffuse color with metalness), SSR,
  its copy, the temporal reprojection (seeding its history on the first frame),
  the denoise, the graded sum, TRAA, the copy the sharpening reads, the
  sharpening and the output. The depth, normal and resolve histories are
  copied after their passes, as the nodes do.
- **Culling.** GLTFLoader's bounding spheres come from the POSITION
  accessors' min and max (half the box diagonal). The port uses the same
  spheres for the camera's and the cascades' frustum culling and for the sort
  depth.
- **Pixel ratio.** The page never sets a pixel ratio, so the gallery renders
  it at one canvas pixel per CSS pixel, as the original does.
- **Restarts.** A resize reseeds the reprojection history, clears the
  denoise target and restarts TRAA from the beauty pass, with an empty
  previous depth. The noise index and frameId keep counting.
- **Fixture.** The page counts its noise and jitter from the first loading
  frame and keeps the loading frames' histories. The fixture stops the loop,
  restarts the counts, resizes the histories so that the next frame reseeds
  them, and renders that first frame, as the port does when it loads. The
  page's four Inspector panels share one control list in the fixture.
- The output modes, step exponent and binary refinement rebuild shaders and
  are not reproduced. The gallery leaves those controls out.

### Simple GI (418)

The original renders each vertex's view into a 32 × 32 target and reads it
back synchronously, summing the bytes on the CPU. The port draws the frame's
32 views into the tiles of one 1024 × 32 target and sums each tile's 8-bit
texels in a compute pass that writes the vertex colors in place. The colors
never leave the GPU, and the clone's colors are a GPU buffer copy. The first
bounce's clone has no color attribute, which WebGL reads as black. The fixture
runs the page's `requestAnimationFrame( compute )` once per frame, after the
render.

### DoF 2 (419)

- **Autofocus.** The page raycasts the scene through the pointer on the CPU
  each frame, before the leaves move. The port runs the same raycast over the
  same triangles, with the identity matrices the objects have before their
  first render.
- **Passes.** The color and depth targets are half float and sized in CSS
  pixels, as the page sizes them. The depth pass uses BokehDepthShader as the
  override material, so the double-sided leaves become front-sided while the
  background keeps its own material.
- **Draws.** Each leaf, head and ball in view is drawn individually after
  frustum culling, as WebGLRenderer draws them, from one per-object storage
  buffer.
- **Rebuilds.** BokehShader2's RINGS and SAMPLES are pipeline constants, and
  each setting's program is built once and reused.
- The leaves draw from Math.random through an alias, which the fixture seeds.

### Subsurface scattering (420)

FBXLoader's parse of the bunny (its non-indexed attributes) is baked by
`tools/tsl/prepare-bunny.mjs`. The ShaderMaterial defines no `USE_MAP`, so
the page's white map is never sampled, and `diffuse` is a Vector3 that color
management leaves alone.

### GPGPU birds (421)

- **Simulation.** The velocity and position variables render into float
  render-target pairs from the previous frame's textures, in the page's loop
  order. The page's seeded Math.random selects the Parrot and fills the seeds
  and initial textures. The port draws the same sequence, including the seeds
  the shader never reads.
- **Drawing.** The page bakes the morph animation into a float texture and
  copies the bird 4,096 times into one geometry. The port bakes the same
  texture and draws one bird instanced, with the per-vertex and per-bird
  values the page stores. Its vertex stage samples the textures with nearest
  filtering at the same coordinates.
- **Shading.** MeshStandardMaterial's physical shading uses the r186 DFG LUT.
  WebGL applies the fog after the sRGB conversion, with the fog color in the
  output color space.

### VRML (422)

VRMLLoader's parse of the sixteen samples is baked by
`tools/tsl/prepare-vrml.mjs`. The bake stores the renderables as the loader
leaves them: world matrices, geometries, materials, and textures (map.gif and
the PixelTexture data). The VRML parsing itself is not ported. All sixteen
scenes are uploaded at load, so switching assets creates nothing. Each frame
culls and sorts as WebGLRenderer does:

- opaque objects by group order, render order, material id, clip-space z of
  the bounding-sphere center, then id;
- transparent objects back to front, with double-sided ones drawn as a back
  pass and then a front pass.

Points are squares of the attenuated `gl_PointSize`.

### USDZ (423)

USDLoader's parse (one mesh and its MeshPhysicalMaterial) and the archive's
base color image are baked by `tools/tsl/prepare-usdz.mjs`. The USD parsing
itself is not ported. WebGLRenderer prefilters the HDR into a PMREM cubeUV
texture for both the environment and the blurred background. The port runs
r186's PMREM generator as the WebGPU renderer generates it, which uses the same
algorithm and layout. WebGPU stores the texture with the opposite vertical
orientation, and the port's lookups account for it. The HDR is sampled without
mipmaps, as HDRLoader's texture has none.

## Comparison

Captures use the pages' fixture clocks, seeded Math.random, the same controls
and a 640 × 400 resize. The thresholds are the defaults (0.5% of pixels over
6/255, mean 0.6/255) except:

- **DoF 2.** The flat-shaded heads' facet edges fall to the neighboring facet
  in a few pixels between WebGL's and WebGPU's rasterization. The bounds are
  3.5% / 0.6.
- **GPGPU birds.** The thousands of small flat facets show the same edge
  differences: 1% / 0.6 without MSAA. With 4× MSAA almost every edge sample
  resolves differently. The reference canvas also has 4 samples, so the
  difference is in edge coverage, consistent with WebGL (ANGLE on Metal)
  rasterizing the canvas upside down with a mirrored sample pattern. The MSAA
  bounds are 16% / 3.5.
- **USDZ.** With MSAA the silhouette's edges differ in 0.5–0.6% of pixels.
  The MSAA bounds are 0.8% / 0.6.
- **Compute rasterizer IBL.** At DPR 2 the first frame after switching to the
  XZ grid (capture 12) differs in 3.2% of pixels. Its occlusion test reads
  the previous grid's pyramid with each instance's previous position, and
  the original culls more distant chunks than the port does. The inputs
  checked against the original at DPR 2 match: the screen size, the pyramid
  levels, the previous view-projection and the pass order. The
  port's occlusion is active, since a pyramid filled with the near plane culls
  everything. The cause is not identified. That capture alone uses 3.5% / 2.8.
  Every other capture uses the default bounds. At DPR 1 it is within 0.03%.

The largest fractions of differing pixels over all states, with and without
MSAA and at DPR 1 and 2 (excluding the scoped capture above):

| Example | Largest fraction |
| --- | ---: |
| Compute rasterizer IBL (414) | 0.04% |
| Progressive shadow map (415) | 0.09% |
| Ready Player Me retargeting (416) | 0.02% |
| SSR denoise (417) | 0.08% |
| Simple GI (418) | 0.0005% |
| DoF 2 (419) | 3.0% |
| Subsurface scattering (420) | 0.19% |
| GPGPU birds (421) | 0.9% without MSAA, 14.6% with |
| VRML (422) | 0.007% |
| USDZ (423) | 0.001% without MSAA, 0.6% with |

## Performance evidence

- **Residency.** Warmed cycles of time, controls, parameters and resize
  create no GPU resources. The DoF bokeh programs are built once per setting,
  and the VRML scenes are all resident.
- **GPU state.** The compute rasterizer's queues, visibility buffers and depth
  pyramid, the progressive lightmap, the SSR, reprojection, denoise and TRAA
  histories, the simple GI vertex colors, and the birds' simulation textures
  stay on the GPU.
- **CPU work kept as in the originals:**
  - the retargeted clip, the helmet LODs and meshlets, and the potpack layout,
    once;
  - the SunLight cascade fitting and the culling and ordering of the scenes,
    each frame;
  - the DoF raycast autofocus and the leaves' motion, each frame.
- **CPU work moved to the GPU.** SimpleGI's per-vertex readback and sum run in
  a compute pass that writes the vertex colors in place. Nothing is read back
  or uploaded.
- **Uploads.** No vertex or index data is written in steady frames. The DoF
  leaves' matrices, like the original's per-object uniforms, are the only
  per-object data written each frame.

No GPU timing parity is claimed.

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js texture-volumes.spec.js
COMPUTE_DPR=2 VOLUMES_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js texture-volumes.spec.js
```
