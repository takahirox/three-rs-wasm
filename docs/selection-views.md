# Animation keyframes, drag controls, box selection and the camera array

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/selection_views.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `misc_animation_groups` | 253 | 25 boxes sharing one material and one AnimationObjectGroup clip: quaternion, discrete color and opacity keyframes |
| `misc_animation_keys` | 254 | One box with position, scale, quaternion, discrete color and opacity tracks, and the AxesHelper |
| `misc_controls_drag` | 255 | 200 shadowed boxes, DragControls (left drag moves, right drag rotates), Shift-click grouping with emissive highlights, on-demand rendering |
| `misc_boxselection` | 256 | 200 shadowed boxes, SelectionBox frustum selection with emissive highlights, the SelectionHelper rectangle |
| `webgpu_camera_array` | 257 | A 6 × 6 ArrayCamera over a rotating, shadow-casting cylinder |

`webgpu_camera_array` supersedes `webgl_camera_array`, which is excluded as its
WebGL equivalent. The misc examples have no WebGPU equivalents and are compared
against the original WebGL renderer.

## Port notes

### Keyframe animations

- **Evaluation.** The clips are evaluated as the AnimationMixer does, on the
  three-second loop:
  - linear interpolation for position, scale and opacity;
  - a half turn about x for the quaternion track;
  - the discrete color track holding each key until the next.
- **Shared state.** The group example drives all 25 meshes from one clip
  state.
- **Randomness.** None.

### Drag controls

- **Picking.** Pointer events use the renderer's raycaster (an explicit CPU
  query, as in DragControls):
  - the drag plane faces the camera through the selected object;
  - a pan maps the intersection back through the parent's inverse world
    matrix;
  - a right drag rotates about the camera's up and right axes at the
    example's rotate speed of 2.
- **Grouping.** Shift-click follows the example:
  - `group.attach` and `scene.attach` keep world transforms;
  - the emissive highlight marks grouped boxes;
  - the draggable list switches to the group, or back to all boxes when the
    group empties.
- **Rendering.** Frames render on load, input and resize, as the original
  renders from its drag and click handlers.
- **Not ported.** Hover cursors and the touch mode toggle ('M').

### Box selection

- **Selection.** `SelectionBox._updateFrustum` is ported for the perspective
  camera:
  - the unprojected corner points, including the example's `z = 0.5` start and
    end points;
  - the `Number.MAX_VALUE` far plane, whose non-finite normal passes every
    point as it does in JavaScript;
  - the containment test on each mesh's bounding-sphere center.
- **Overlay.** The SelectionHelper rectangle is a DOM element with the
  example's CSS. It is hidden in the pixel comparison, as the reference
  fixture does not load the example's style sheet.

### Camera array

- **Views.** The 36 sub-cameras copy the root camera's 50° lens and the window
  aspect, and look at the origin from their grid positions.
- **Viewports.** Each renders into its viewport with WebGPU's top-left origin
  (`floor( x × pixelRatio )`, `floor( ceil( width ) × pixelRatio )`).
- **Shadows.** As in WebGPURenderer, one shadow pass serves all views. The
  renderer now has `Scene::shadow_auto_update`, Three.js's
  `shadowMap.autoUpdate`: when false, a render reuses the last shadow maps.
- **Draw order.** WebGPU records each object's 36 views together, while the
  port renders view by view. The one-pixel overlaps of adjacent `ceil`-sized
  viewports can therefore resolve differently.

## Shadow renderer changes

Two changes to shadow-map rendering apply to every scene:

- **Culling.** Shadow casters are culled against each shadow camera's frustum,
  including each point-light cube face, as WebGLShadowMap and the WebGPU
  shadow passes do. The box-selection scene's draw count (190 shadow draws of
  200 casters) now equals the original's.
- **Resident draw data.** Shadow draw data is keyed by layer, caster and
  material group rather than by draw position. Reordered casters, as in
  Shift-click grouping, and casters culled from a moving point light's faces
  keep their bindings.

## Comparison

`reference/three-js/selection-views.html` executes the pinned sources on the
example clock, with seeded randomness. The camera array's per-frame rotation
runs as 60 fps steps. The suite covers:

- time steps across the animation loops;
- drags, rotations, Shift-click grouping and group drags;
- box selections;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Animation groups, MSAA off / on | 0% / 0.00, 0.431% / 0.09 | 0% / 0.00, 0.198% / 0.04 |
| Animation keys, MSAA off / on | 0% / 0.00, 0.106% / 0.02 | 0% / 0.00, 0.052% / 0.01 |
| Drag controls, MSAA off / on | 0.415% / 0.06, 2.425% / 0.53 | 0.434% / 0.06, 1.426% / 0.29 |
| Box selection, MSAA off / on | 0.076% / 0.01, 1.919% / 0.64 | 0.072% / 0.01, 0.996% / 0.33 |
| Camera array (no MSAA) | 1.115% / 0.32 | 1.116% / 0.32 |

### Bounds

- **Camera array.** Bounded at 1.5% / 0.4 with MSAA off. The differing pixels
  are the one-pixel viewport overlaps described above.
- **MSAA.** Silhouettes and shadow edges, with these bounds:

| Case | Bound |
| --- | --- |
| Animation groups | 1% / 0.2 |
| Animation keys | 1% / 0.2 |
| Drag controls | 3% / 0.7 |
| Box selection | 3% / 0.8 |

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the camera array's 36 views of each object and its single shadow pass;
  - the frustum-culled shadow casters.
- **Uploads.** No scene uploads geometry or texture data after a warm pass.
- **Warmed cycles.** Warmed cycles of time and input create no GPU resources.
  This includes Shift-click grouping that ends where it began.

No GPU timing parity is claimed. Full measurements are in
[selection-views-comparison.json](selection-views-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js selection-views.spec.js
SELECTION_DPR=2 npx playwright test -c playwright.gallery.config.js selection-views.spec.js
```
