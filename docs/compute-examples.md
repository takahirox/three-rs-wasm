# Compute, clustered and dynamic lighting, projectors, VSM, transmitted shadows and caustics

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/compute_birds.rs`, `sort_bitonic.rs`,
`attractors.rs`, `linked_particles.rs`, `lights_clustered.rs`,
`lights_dynamic.rs`, `shadowmap_vsm.rs`, `lights_projector.rs`,
`shadowmap_opacity.rs` and `caustics.rs`. The WGSL modules of the last seven
are in the directory of the same name.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgpu_compute_birds` | 364 | 8,192 birds: the velocity pass (every bird against every other) and the position and flap pass over resident storage buffers, read by the instanced birds' vertex stage; the icosahedron sky, fog, the pointer ray and the three rules |
| `webgpu_compute_sort_bitonic` | 365 | Two bitonic sorts of 16,384 shuffled values (workgroup-local swaps with global flip and disperse, and global-only swap, align and set passes) on the page's 100 ms and 1 s timers, with the 128 × 128 grid and swap-zone display |
| `webgpu_tsl_compute_attractors_particles` | 366 | 262,144 particles drawn by three turning attractors at a fixed 1/60 s step, speed-colored additive sprites, the attractor helpers and three rotating TransformControls |
| `webgpu_tsl_vfx_linkedparticles` | 367 | 8,192 short-lived particles spawned toward the pointer, fractal-noise turbulence, ribbons to the two nearest live particles, the orbiting light on the flat metal icosahedron, BloomNode, ACES and the auto-rotating controls |
| `webgpu_lights_clustered` | 368 | 876 point lights on bouncing spheres between four Phong spheres, the per-frame depth-sorted light texture, the cluster compute pass over 32-pixel tiles and 24 depth slices, renderOutput with the heat map, and FXAA |
| `webgpu_lights_dynamic` | 369 | 100 PBR shapes and a metal sphere lit by orbiting point lights batched by DynamicLighting into 16-light uniform arrays, the light markers, add, remove, remove all, auto-add and the dynamic-mode switch |
| `webgpu_shadowmap_vsm` | 370 | The turning torus knot, four pillars and ground, a spot and a turning directional light with VSM shadows (depth, vertical and horizontal passes, radius and samples per light), fog and the animate switch |
| `webgpu_lights_projector` | 371 | Lucy under the circling ProjectorLight with PCF shadows, the procedural Worley caustic, the Sintel video or colors.png projected through its shadow matrix, the SpotLightHelper, ACES and every control |
| `webgpu_shadowmap_opacity` | 372 | Two DragonAttenuation dragons refracting a mipmapped copy of the frame through their volumes, with colored transmitted shadows rendered once, over the cloth backdrop, and AgX at exposure 1.5 |
| `webgpu_caustics` | 373 | The turning Draco duck (double-sided transmission, back then front faces over fresh mipmapped copies) casting its refracted Caustic_Free shadow onto the hardwood floor, the glass-plane alternative and the occlusion control |

All ten are WebGPU examples and are compared against the WebGPU renderer. The
WebGL twins of the birds and the VSM shadows are listed as equivalents only.

## Port notes

### Generated WGSL

The WGSL three.js r186 generates for each page was captured from
`createShaderModule` on the reference page, with the pipeline states, draw
order and pass structure.

- **Birds, bitonic sort and attractors.** These translate the generated
  modules into compact WGSL with the same expressions, in their `.rs` files.
- **The other seven.** These run the captured modules, committed unchanged
  except for three edits:

  - `enable subgroups;` and the unused subgroup-size builtin are removed.
  - Grid sizes and light-array lengths are placeholders where the page derives
    them at run time (the clustered tile grid, the dynamic light-array length).
  - Identical modules are shared between examples.

Pipelines use automatic layouts. Each pipeline gets its own bind groups from
`get_bind_group_layout`, since automatic layouts are not interchangeable.
Uniform structs are filled by their field order in the WGSL: the projector,
opacity and caustics ports parse the struct from the source and apply the
WGSL alignment rules, so each variant's layout comes from its own module.

### Birds, bitonic sort, attractors and linked particles

- **Birds.** The velocity pass reads other birds' velocities while they are
  written, as the original's does, so the original differs from itself
  between runs. The pointer ray is reset to y = 10 after each compute, as the
  page does.
- **Bitonic sort.** Storage bindings need 256-byte offsets, so each half binds
  its whole buffer and a uniform selects the span. The halves are drawn into
  the two halves of one canvas.
- **Attractors.** The sprites are drawn between the helpers and the gizmos,
  keeping the original's render-list order. Queued pointer events reach the
  gizmos before the orbit, as in the original.
- **Linked particles.** The fixture drives TSL's `deltaTime` and `time` from
  the example clock, since NodeFrame reads `performance.now()`.

### Clustered and dynamic lighting

- **Clustered.** The lights are sorted by view depth on the CPU into the
  float texture with each slice's range, as the addon does, and the cluster
  pass assigns them on the GPU.
- **Dynamic.** In dynamic mode the point lights fill 16-entry uniform arrays
  in light order; the 17th light onward is ignored, as the addon ignores it.
  With dynamic mode off, three.js compiles per-light code for each light
  count. The port reuses the same loop with an array long enough for every
  light (the next power of two), rebuilding the pipelines when the count
  outgrows it. Marker uniforms are kept per light slot, so re-adding lights
  creates nothing. The point-light count display is not drawn.

### VSM, projector, transmitted shadows and caustics

- **VSM.** Each light has its depth map and its VSM vertical and horizontal
  targets. Each shadow camera culls against its own frustum, as three.js does.
- **Projector.** The box attenuation, the Worley caustic and the map lookup
  use the projector's shadow matrix; the shadow camera's aspect is the map's.
  The video mode plays the Sintel video and copies each new frame once. Turning
  shadows off makes three.js recompile without the shadow code. The port
  instead sets the shadow intensity to 0 and skips the depth pass, which
  gives the same image.
- **Transmission.** The resolved frame is copied into a mipmapped half-float
  texture after the opaque pass. The mips are generated by three's
  `WebGPUTexturePassUtils` shader. The dragons and the duck sample it with the
  generated bicubic lookups. The duck and the glass draw back faces and then
  front faces, each over a fresh copy, as three renders double-sided
  transmissive meshes.
- **Colored shadows.** With `shadowMap.transmitted`, the shadow passes also
  write the casters' `castShadowNode` color, which receivers multiply in.
  The shadow pipelines keep the page's winding and culling for each caster
  and face. The dragons' shadow is drawn once (`autoUpdate` off).
- **Duck rotation.** The page turns the duck 0.01 rad per animation frame. The
  fixture and the port both advance it in 60 fps steps of the example clock.
- **Material color.** The page's Inspector applies `setHex` (sRGB) to the
  duck's color, and so does the port. The fixture's color setter writes the
  components directly, so this control is not image-compared.

## Comparison

`reference/three-js/compute-examples.html` runs the pinned pages on the
example clock with the seeded `Math.random`. The suite covers every control
the fixture can drive, drags, wheels and resize, at DPR 1 and 2. Examples with
antialiasing are compared with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima (fraction / mean):

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Birds, MSAA off / on | 1.737% / 2.76, 2.907% / 2.32 | 1.505% / 2.46, 1.541% / 1.50 |
| Bitonic sort (both sorts, step by step) | 0% / 0.00 | 0% / 0.00 |
| Attractors, MSAA off / on | 0.015% / 0.01, 0.04% / 0.01 | 0.014% / 0.02, 0.044% / 0.01 |
| Linked particles, MSAA off / on | 0% / 0.00, 0% / 0.00 | 0% / 0.00, 0% / 0.00 |
| Linked particles (pointer trails) | 0% / 0.00 | 0% / 0.00 |
| Clustered lights | 0.003% / 0.00 | 0.001% / 0.00 |
| Dynamic lights, MSAA off / on | 0% / 0.00, 0% / 0.00 | 0% / 0.00, 0% / 0.00 |
| VSM shadows, MSAA off / on | 0% / 0.00, 0% / 0.00 | 0.001% / 0.00, 0.001% / 0.00 |
| Projector, MSAA off / on | 0% / 0.00, 0.023% / 0.00 | 0% / 0.00, 0% / 0.00 |
| Transmitted shadows, MSAA off / on | 0.008% / 0.01, 0.005% / 0.01 | 0.002% / 0.01, 0.003% / 0.01 |
| Caustics, MSAA off / on | 0% / 0.00, 0% / 0.00 | 0% / 0.00, 0% / 0.00 |

### Bird velocity race

- **Cause.** The velocity pass reads the other birds' velocities while they
  are written, as the original's does. The GPU's schedule decides which
  values a bird sees.
- **Effect.** The original differs from itself by 0.4–0.9% between runs, and
  the port differs from it by up to 2.9% of pixels (mean 2.8/255), all on the
  thin, moving birds.
- **Bound.** The birds are bounded at 4% / 4, with and without MSAA.

### Transmission start-up frame

- **Observation.** About one page load in four, the first frames of the
  dragons differ from later frames by under one level (mean 0.3/255) on the
  left dragon only. Later frames are stable and match the original exactly.
  The cause has not been identified.
- **Capture.** The shadowmap-opacity captures render two frames before each
  screenshot.

## Performance evidence

- **Residency.** Warmed cycles of time, controls, input and resize create no
  GPU resources for any of the ten examples.
- **Compute.** Birds, both sorts, attractors and linked particles keep their
  state in resident storage buffers that the render passes read. Nothing is
  read back or re-uploaded per frame.
- **Draw workload.** The measured draws equal the original's in counts and
  instances, including the culled draws of the dynamic-lighting shapes and the
  VSM shadow passes.
- **Uploads.** No vertex, index or storage data is written in steady frames.
  - The clustered example rewrites its sorted light texture each frame, as
    the original does (the same bytes).
  - Uniforms are the only other per-frame writes.
  - The projector's video frames are copied once each.
  - The glTF, PLY and image assets are uploaded once, with mipmaps generated
    once on the GPU.
- **Shadows.** The opacity example's shadow is rendered once, as
  `autoUpdate = false` asks. The others re-render their shadow maps each
  frame, as their originals do.

No GPU timing parity is claimed. Full measurements are in
[compute-examples-comparison.json](compute-examples-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
COMPUTE_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
```
