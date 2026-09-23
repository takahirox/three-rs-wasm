# Voxel painting, triangle picking, clipping, scene comparison and custom blending

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/interactive_scenes.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_interactive_voxelpainter` | 193 | Grid, invisible pick plane, translucent rollover, click to add and Shift-click to remove textured Lambert voxels, on-demand rendering |
| `webgl_interactive_buffergeometry` | 194 | 5,000 vertex-colored Phong triangles in fog, per-frame triangle picking with a highlighted outline |
| `webgl_clipping_intersection` | 195 | 15 nested double-sided Phong spheres, three shared clipping planes, all four controls, plane helpers, orbit/zoom, on-demand rendering |
| `webgl_multiple_scenes_comparison` | 196 | Two scenes split by a draggable slider through scissor rectangles, orbit/pan/zoom |
| `webgl_materials_blending_custom` | 197 | 110 custom blend-factor combinations, 21 canvas labels, the equation control and a scrolling checker background |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

- **Voxel painter.**
  - Voxels share one geometry and one material. The pick uses Three.js's
    `face.normal`: the unflipped, local triangle normal, computed from the hit
    triangle.
  - Snapping follows the original's floor-and-offset arithmetic.
  - Shift is tracked through keydown and keyup (keyCode 16), as in the original.
  - Adding a voxel creates its per-draw record once.
- **Triangle picking.**
  - Normals are computed from the f64 vertices before Float32 storage, as the
    original does.
  - The per-frame raycast is the original's CPU query.
  - While a triangle is hit, the 4-vertex outline is rewritten through the mesh
    matrix. The original does the same with `applyMatrix4`. Each hovered frame
    uploads 368 bytes: four 80-byte renderer vertices plus the 48-byte position
    input.
- **Clipping.** The clipping planes and intersection mode use the existing
  resident clipping path. `PlaneHelper` is reproduced: an outline line strip and
  a one-sided translucent quad, placed with `lookAt( normal )` and
  `translateZ( -constant )`.
- **OrbitControls.** Controls use the shared Rust OrbitControls update step, as
  in [interactive-objects.md](interactive-objects.md). Keyboard controls are not
  ported.
- **Scene comparison.**
  - Each scene renders into a shared target under the original's rounded,
    DPR-scaled scissor rectangles. The target keeps multisampled color between
    the two passes.
  - WebGL clears only inside the scissor rectangle. Here the right scene draws
    its background as one fullscreen triangle inside its scissor rectangle. It
    is excluded from workload counts like the presentation triangle.
  - The slider is the original DOM element. Dragging it disables the controls,
    and its position is not changed by resize, as in the original.
- **Custom blending.** The alpha channel reuses the color factors and equation,
  as with CustomBlending. WebGPU requires One factors for Min/Max, which GL
  ignores for those equations. The labels, the checker and its scroll follow the
  `webgl_materials_blending` port.
  - The planes and labels are built-in `MeshBasicMaterial` maps on an encoded
    target. All 110 blend states share one shader, and all labels share one
    pipeline.
  - Blend state is part of a WebGPU pipeline, so the scene needs one pipeline per
    combination, as Three.js's WebGPU renderer would. That comes to 121
    pipelines and 5 shader modules.
  - A first version built one custom program per texture: 163 pipelines and 27
    modules. On CI's software GPU that was still compiling when the next test
    began.

### WebGL output path

These fixes apply to every `encode_srgb` (WebGL-style) target:

- **Fog order.** WebGL runs `fog_fragment` after `colorspace_fragment`, so fog now
  mixes the encoded color toward the encoded fog color. `refreshFogUniforms`
  converts the fog color to the output color space.
- **Clip edges without MSAA.** WebGL keeps the `ALPHA_TO_COVERAGE` clipping shader
  on single-sample canvases, so edge fragments with nonzero clip opacity survive.
  WebGPU-reference targets keep their hard clip.
- **Canvas views.** The five examples render display values to a raw canvas view.
  An earlier listing error presented them through an sRGB view and encoded them
  twice.
- **Geometry uploads.** Resident geometry buffers are now filled with queue writes
  instead of mapped-at-creation buffers. A mapped range holds a Wasm memory view
  until unmap, and allocator growth could detach it. This build's memory layout
  triggered that in `webgpu_centroid_sampling`, the same hazard `draw_gpu.rs`
  already avoided.

## Comparison

`reference/three-js/interactive-scenes.html` executes the pinned sources with
fixed time and seeded randomness. It captures GUI controllers and provides the
original container, slider and CSS. The suite covers:

- time steps and every control;
- hover, clicks and Shift-clicks;
- drags, wheel zoom, right-button pan and slider drags;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA. Both
on-demand scenes are also tested to stay idle until input or resize.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Voxel painter, MSAA off | 0.002% / 0.00 | 0.002% / 0.00 |
| Triangle picking, MSAA off | 0.105% / 0.08 | 0.079% / 0.05 |
| Clipping, MSAA off | 0.001% / 0.00 | 0% / 0.00 |
| Scene comparison, MSAA off | 0.212% / 0.57 | 0.148% / 0.49 |
| Custom blending | 0% / 0.06 | 0% / 0.04 |
| Voxel painter, 4× MSAA | 3.557% / 0.94 | 1.797% / 0.48 |
| Triangle picking, 4× MSAA | 5.378% / 0.97 | 2.843% / 0.51 |
| Clipping, 4× MSAA | 2.310% / 0.57 | 1.062% / 0.27 |
| Scene comparison, 4× MSAA | 1.982% / 0.63 | 1.124% / 0.52 |

With MSAA off, all five scenes match within the ordinary threshold. With 4×
MSAA, the differences lie on:

- the voxel grid lines;
- the dense triangle edges;
- the comparison wireframe;
- the clipping edges, whose alpha-to-coverage sample masks are
  backend-dependent.

The suite requires the ordinary threshold with MSAA off, and bounds only these
MSAA cases:

| Case | Differing pixels | Mean error |
| --- | ---: | ---: |
| Voxel painter | 4% | 1.1 |
| Triangle picking | 6% | 1.1 |
| Clipping | 2.6% | 0.7 |
| Scene comparison | 2.2% | 0.75 |

These bounds do not relax geometry, controls, residency or workload checks.

## Performance evidence

- **Draw workload.** The measured draws equal the original:
  - the grid and rollover;
  - the 15,000-vertex triangle mesh and its 4-point outline;
  - fifteen 6,624-index spheres;
  - the solid and wireframe comparison meshes;
  - 132 label and blend quads plus the background.
- **Uploads.** After a warm pass, a measured frame uploads no geometry, attribute
  or texture data. The one exception is the hovered triangle outline (368 bytes),
  as in the original.
- **Warmed cycles.** Warmed cycles of time, controls, hover, slider and resize
  create no GPU resources.
- **Idle.** The voxel painter and clipping scenes render only on input and resize.

No GPU timing parity is claimed. Full measurements are in
[interactive-scenes-comparison.json](interactive-scenes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js interactive-scenes.spec.js
SCENES_DPR=2 npx playwright test -c playwright.gallery.config.js interactive-scenes.spec.js
```
