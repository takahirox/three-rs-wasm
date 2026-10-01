# Protoplanet, retro, dynamic cubemap, deferred, cloth, volume caustics, HDR, fluid and light probes

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgl_gpgpu_protoplanet` | 384 | `gpgpu_protoplanet.rs` | 4,096 debris particles in the 64 × 64 GPUComputationRenderer: the all-pairs gravity and merge velocity pass, the position pass and the mass-sized points, with every control |
| `webgpu_postprocessing_retro` | 385 | `retro.rs` | The baked coffee mug and its smoke under the PS1 sky through RetroPassNode at a quarter of the drawing buffer, barrelUV, color bleeding, Bayer dither, posterize, vignette and scanlines; Damaged Helmet with venice_sunset as a CubeMapNode cube (retro) or a PMREM (plain) |
| `webgpu_cubemap_dynamic` | 386 | `cubemap_dynamic.rs` | The mirror sphere over a 256² CubeCamera render of the pisaHDR sky, the box and the torus knot, PMREMNode regenerating the sphere's environment from the cube every frame, 4× MSAA and ACES |
| `webgpu_deferred` | 387 | `deferred.rs` | The teapot, eight orbiting point lights and their spheres and six transparent double-sided panels under the royal_esplanade UltraHDR sky, in the deferred (MRT and full-screen resolve) and forward modes |
| `webgpu_compute_cloth` | 388 | `compute_cloth.rs` | The 31 × 31 Verlet cloth hung from six points, wind and the swinging sphere in 1/360 s compute steps, the MeshPhysicalNodeMaterial cloth and the instanced wireframe springs and vertices |
| `webgpu_volume_caustics` | 389 | `volume_caustics.rs` | The glass duck's caustic spot shadow, the transmissive duck, the half-resolution volume ray march through the 128³ noise smoke, BloomNode and the output mix |
| `webgpu_hdr` | 390 | `hdr.rs` | The pointer-driven HDR brush over the white pixel-space pass, AfterImageNode's ping-pong and the extended sRGB output |
| `webgpu_compute_particles_fluid` | 391 | `fluid.rs` | 32,768 MLS-MPM particles on a 64³ grid: the indirect-dispatch counts, grid clear, fixed-point atomic particle-to-grid passes, grid update and grid-to-particle move with the pointer ray's force, drawn as instanced icosahedra under the UltraHDR sky |
| `webgpu_lightprobes` | 392 | `lightprobes.rs` | The Cornell box with its shadow-casting point light, the 6 × 6 × 6 LightProbeGrid baked on the GPU, the GI-lit materials, the probe helper, and the GI, resolution (rebake) and helper controls |
| `webgpu_lightprobes_complex` | 393 | `lightprobes.rs` | Two rooms joined by a doorway with the warm and cool shadowed lights, the back-side room shell, and two grids with falloff 1, baked in turn |

385–393 are WebGPU examples compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`). 384 is a WebGL-only example
compared against WebGLRenderer (`texture-volumes.spec.js`).

## Port notes

### Generated WGSL (385–393)

As in [compute-examples.md](compute-examples.md), the WGSL three.js r186
generates for each page was captured from `createShaderModule` on the
reference page, with the pipelines, bind groups, passes and uniform writes.
The modules run unchanged with automatic layouts, except that the cloth and
fluid modules drop the subgroup built-ins they declare but never use. Uniform
structs are packed by name from each module's own struct. The PMREM blur and
cubeUV passes are shared (`pmrem_cube_uv.rs`) by 385, 386 and 387; 392 and
393 reuse the godrays point-shadow modules and 386's output modules, which
the pages generate byte for byte.

### Protoplanet (384)

The page's two GLSL compute shaders and the particle shaders are translated
to WGSL with the same expressions. Both variables read the previous frame's
textures and render into the other target of their float pairs, as
GPUComputationRenderer does. WebGL points become six-vertex billboards of the
same clamped size, depth-tested at their centers. The simulation steps once
per animate(); a separate test follows it frame by frame over 120 frames.

### Retro (385)

- **Retro pass.** The scene renders at a quarter of the drawing buffer with
  4× MSAA, each material swapped for its retro version (vertex snapping to
  the pass resolution, affine texturing, level-0 reads), as RetroPassNode
  does.
- **Helmet.** Selecting Damaged Helmet loads it and venice_sunset_1k.hdr: a
  512² CubeMapNode cube in retro mode, a PMREM (cubeUV 768 × 1024, ten GGX
  levels) otherwise.
- **Known differences.** The port keeps both models' buffers after a switch
  (the original disposes and reloads the geometry), and does not draw the
  helmet until its environment has loaded (the original draws it first
  without one).

### Dynamic cubemap (386)

Each frame the CubeCamera renders its six faces with the sphere hidden and
PMREMNode regenerates the sphere's environment from the cube. The sky's PMREM
is generated once. The rotations and the auto-rotation advance on the clock
as 60 fps steps; the fixture drives the page's increments the same way.

### Deferred (387)

The deferred mode renders the opaque teapot and spheres unlit into the MRT
(diffuse color, view position and metalness, view normal and roughness),
resolves the lights in a full-screen pass over the sky background, then
draws the transparent panels over the opaque depth. The forward mode lights
everything in one pass. The sky is a 1024² CubeMapNode cube for the
background and a lodMax 9 PMREM for the lighting.

### Cloth (388)

Each requested frame runs fixed 1/360 s steps (at most 1/60 s per frame),
each submitting the spring-force and vertex-force compute passes over
resident storage buffers. The cloth mesh reads its positions in the vertex
stage. The wireframe draws the springs as instanced lines and the vertices
as instanced sprites.

### Volume caustics (389)

- **Passes.** The caustic shadow (front and back faces), the floor, the
  duck's back and front faces over mipmapped copies of the frame, the
  half-resolution volume with the bayer16 dither offset by the frame ID and
  stopped at the scene depth, BloomNode's high pass and five blur levels, and
  the output adding 0.7 × bloom.
- **Frame ID.** The page's frameId advances on the renderer's own animation
  loop, so the fixture stops that loop once ready and resets frameId to 0;
  both sides then advance it per requested frame.
- **First frame.** The original lights the volume differently on its first
  frame after loading, so each capture renders two frames.

### HDR (390)

The brush and AfterImageNode run on half-float targets, and the output is
extended sRGB. The original presents on an extended-range canvas. On a
standard display both clamp to the displayable range, and the
white-background screenshots compare equal. The port presents 8-bit output,
so values above 1 are not reproduced on an HDR display. During development
the half-float brush and after-image targets were read back and matched the
original's bit for bit. The fixture tolerates the page's missing `#no-hdr`
element.

### Fluid (391)

- **Simulation.** The page's compute kernels run on resident storage buffers
  each requested frame: the indirect-dispatch counts, grid clear, the two
  fixed-point atomic particle-to-grid passes, grid update and grid-to-particle
  move with the pointer ray's force. The particles' vertex stage reads their
  positions.
- **Pointer.** The pointer ray uses the camera and the canvas-relative pointer
  position, as the page's uniforms do.
- **Start-up.** The original compiles its pipelines asynchronously and
  presents stale frames until they are ready; captures start 1.5 s after
  loading.

### Light probes (392, 393)

- **Bake.** For each probe, LightProbeGrid renders a 32² HalfFloat CubeCamera
  capture (near 0.05, far 20). The 512-direction equal-area Fibonacci SH
  projection then writes one row of the 9 × N float batch target, and the
  seven repack passes write each Z slice and its padding into the RGBA16F 3D
  atlas. Each probe's faces and SH row are one submit, as the face cameras'
  uniforms change between probes. 393 bakes its two grids in turn, both out
  of the scene, as the page adds them only after both bakes.
- **Shadows.** The point shadows are re-rendered every frame, as
  `shadow.autoUpdate` does. The bake reads the current ones.
- **Rebake.** A resolution change rebakes into new grids, as the page does.
  With the probes shown, the page disposes the old grids first, so their
  still-attached helpers sample atlases recreated empty. Each cube capture
  then contains black helper spheres at the old probe positions, and the
  port draws them too.
- **Helper instances.** InstanceNode stores the helper's matrices in a
  uniform array of the probe count, as the generated WGSL declares. For more
  than 1,000 probes (11³ and 12³) the port binds the same array as a storage
  buffer.

## Comparison

Captures use the page's fixture clock, seeded Math.random, the same controls
and a 640 × 400 resize. The thresholds are the defaults (0.5% of pixels over
6/255, mean 0.6/255) except:

- **MSAA.** With `samples=1`, 385, 386, 388, 392 and 393 match exactly in
  every state. With 4× MSAA they differ along geometric edges only, for a
  reason not identified. These comparisons use MSAA-only bounds: 385 at
  8% / 2.5 (the helmet), 386 at 1% / 0.2, 388 at 13% / 7 (wireframe lines),
  391 at 15% / 3.5, 392 at 3% / 0.7 and 393 at 4% / 0.9.
- **Fluid pointer.** During an orbit drag, the original's pointer ray reads a
  camera matrix that OrbitControls has only partly updated. 391's drag and
  resize captures use 0.8% / 0.5.

## Performance evidence

- **Residency.** Warmed cycles of time, controls, parameters and resize create
  no GPU resources. Retro's cycle includes the model switch: the helmet and
  its environment load once and stay resident. 392 and 393 leave out the
  resolution parameter, since each rebake allocates new grids, as
  LightProbeGrid does.
- **Simulations.** The cloth, fluid and protoplanet state stays on the GPU.
  Only uniforms (and the fluid's indirect-dispatch counts, written by its own
  kernel) change per frame.
- **Uploads.** No vertex, index or storage data is written in steady frames.
  Models, images and environment maps are decoded and uploaded once.
- **Per-frame GPU work kept as in the originals:**
  - The dynamic cubemap's six faces and PMREM.
  - The light-probe scenes' point shadows.
  - The volume caustics' mipmapped frame copies.

No GPU timing parity is claimed.

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
COMPUTE_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js -g protoplanet
```
