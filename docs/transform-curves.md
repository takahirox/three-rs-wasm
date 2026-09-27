# Transform controls, curve modifiers, the CCD IK arm and the glTF normal-map export

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/transform_controls.rs` (TransformControls) and
`src/browser/transform_curves.rs`; the normal-map export in
`src/browser/gltf_normals.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `misc_controls_transform` | 308 | The crate under TransformControls: translate, rotate and scale gizmos, snapping, local/world space, axis toggles, size, enable, reset, the perspective/orthographic switch and random fov/zoom, OrbitControls, rendering only on change |
| `webgpu_modifier_curve` | 309 | CurveModifierGPU's Flow on 3D text, the closed centripetal curve and its line, click-to-select handles dragged by TransformControls, the curve update on drag end |
| `webgl_modifier_curve_instanced` | 310 | InstancedFlow: eight colored text instances on two curves, the same handle selection and dragging |
| `misc_exporter_gltf_normals` | 313 | OpenGL and DirectX normal maps on two double-sided planes (normalScale.y signs), two directional lights, OrbitControls rendering on change, the Export button's GLTFExporter GLB |
| `webgl_animation_skinning_ik` | 312 | Kira (Draco, WebP), CCDIKSolver with rotation limits and CCDIKHelper, the mirror sphere's 1024² CubeCamera, the head's lookAt, TransformControls on the IK target, damped OrbitControls, all four controls |

`webgl_modifier_curve` is excluded as the WebGL equivalent of
`webgpu_modifier_curve`. The other four examples have no WebGPU equivalent
and are compared against the WebGL renderer. `webgpu_camera` (311), added in
the same batch, is documented in [controls-attributes.md](controls-attributes.md).

## Port notes

### TransformControls

- **Gizmo.** The translate, rotate and scale gizmos, pickers and helpers are
  built as `setupGizmo()` builds them. Each definition's transform is baked
  into its geometry, and the clone order and infinite `renderOrder` are kept.
  - **Materials.** The handles swap between resident base and highlighted
    materials, so highlighting creates no GPU resources.
  - **Update.** Each render runs the root's `updateMatrixWorld`: the attached
    object's and camera's decompositions, the eye vector, handle placement and
    scaling, camera-facing hiding, show flags, highlighting, and the drag plane.
- **Pointer handling.** Hover, down, move and up follow the addon's handlers:
  - picker raycasts against the visible handles;
  - the 100000² drag plane, with its orientation and `_dirVector` persisting
    between updates;
  - translation, rotation and scale with their snaps;
  - `mouseDown`, `mouseUp`, `dragging-changed`, `change` and `objectChange`.
- **Event order.** Pointer, key and orbit input is queued and replayed in DOM
  listener order before each frame. The misc example's `change` listener
  renders, so its gizmo updates at each change event, as the original's does.
- **misc_controls_transform keys.** They follow `event.key`: with Shift held,
  letters arrive as capitals and match no case.
  - **Random fov and zoom.** `v` draws them from the fixture's seeded random.
  - **Orthographic camera.** The orbit zooms and pans the orthographic camera
    as OrbitControls does, through two new `Controls` helpers:
    `wheel_scale`/`take_scale` and `pan_axes`.

### Curve modifiers

- **Spline texture.** `updateSplineTexture` builds the RGBA half-float spline
  texture: `getSpacedPoints` and `computeFrenetFrames` over 512 arc-length
  divisions, and `DataUtils.toHalfFloat`'s truncating table conversion. The
  texture samples linearly, repeating in S and clamping in T.
- **Stale length.** `getLength()` returns the curve's cached lengths after the
  first update, because moving the handles does not mark the curve for update.
  The port keeps this: `spineLength` lags one update.
- **Vertex stage.** Sampling and bending happen in a vertex stage over the
  resident text:
  - **WebGPU.** The normalNode, basis × normalLocal, is used as the view
    normal.
  - **InstancedFlow.** It reads each instance's length, curve and offset from
    its translation. Its instance matrices keep the initial curve lengths, as
    the original's do.
- **Line updates.** Curve lines are rewritten in place on drag end, as
  `setFromPoints` rewrites them.
- **Animation.** `moveAlongCurve( 0.001 )` runs as 60 fps steps of example time.

### Skinning IK

- **Solver.** CCDIKSolver runs one iteration per frame, with three's XYZ Euler
  decomposition for the rotation limits.
- **Frame order.** Each frame follows `animate()`:
  1. the mirror sphere's CubeCamera renders six 1024² faces into an 8-bit
     linear cube;
  2. the followSphere lerp and the head's `lookAt` (with `rotation.y += π`);
  3. the IK update;
  4. the damped orbit;
  5. the view.
- **glTF nodes.** Their matrices recompose from position, quaternion and scale,
  as three's do.
- **Skeleton root.** Bone 0 is re-parented under `Kira_Shirt_left`, keeping its
  local transform.
- **Bounding spheres.** `updateIK()`'s `computeBoundingSphere()` is kept: it
  recomputes each skinned mesh's culling sphere on the CPU, as the original
  does.
- **Lighting.** The glTF standard materials use r186 physical lighting, whose
  ambient diffuse is attenuated by specular multiple scattering.

### glTF normal-map export

- **Scene.** The two planes share one PlaneGeometry. The DirectX map is
  rendered with `normalScale.y` negated, and the standard materials use r186
  physical lighting.
- **GLTFExporter.** The Export button writes the GLB that
  `GLTFExporter.parse( [ mesh1, mesh2 ], { binary: true } )` writes:
  - the JSON keys in the exporter's creation order, serialized by
    `JSON.stringify`;
  - node matrices and shared accessors (the second mesh reuses the first's);
  - the OpenGL map's green channel inverted on a canvas, because `normalScale.y`
    is positive without tangents;
  - each image flipped into a canvas and encoded by `canvas.toBlob( 'image/png' )`,
    appended as buffer views after the geometry;
  - 4-byte chunk padding with spaces (JSON) and zeros (binary).

  The export matches the original byte for byte.

### Core changes

- **SkinnedMesh culling.** Camera passes now cull skinned meshes by their
  cached bounding sphere. The sphere is computed from the pose at first use and
  recomputed by `Renderer::compute_bounding_sphere`, as three caches
  `boundingSphere`. Shadow passes share the same cache.
- **glTF bounds.** Imported primitives take GLTFLoader's `computeBounds`: the
  POSITION accessor's min/max box, expanded by morph displacements, and a
  sphere spanning its diagonal.
- **Lossy WebP.** Lossy WebP images decode through the browser, as the
  loaders' `ImageBitmap`s do. `EXT_texture_webp` sources resolve before import.
- **Curves.** The `Curve` trait takes explicit arc lengths for spaced points
  and Frenet frames.

lil-gui, Stats and the Inspector are not reproduced.

## Comparison

`reference/three-js/transform-curves.html` executes the pinned sources on the
example clock.

- **Coverage.** Each example's pointer, key, control, orbit and resize input is
  compared at DPR 1 and 2.
- **IK convergence.** The IK example's captures render 300 frames, so that the
  per-frame solver converges.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Transform controls, MSAA off / on | 0.053% / 0.02, 2.876% / 0.55 | 0.040% / 0.01, 1.438% / 0.28 |
| Curve modifier (WebGPU), MSAA off / on | 0.269% / 0.09, 1.097% / 0.30 | 0.241% / 0.07, 0.669% / 0.18 |
| Instanced curve modifier, MSAA off / on | 0.019% / 0.10, 1.704% / 0.36 | 0.020% / 0.10, 0.871% / 0.22 |
| Skinning IK, MSAA off / on | 0.072% / 0.04, 1.334% / 0.23 | 0.043% / 0.02, 0.679% / 0.11 |
| glTF normals, MSAA off / on | 0% / 0.01, 0.164% / 0.05 | 0% / 0.00, 0.080% / 0.02 |

The glTF normals scene matches within the ordinary threshold with and without
MSAA, and its export is byte-identical. With MSAA off, the other four match
within the ordinary threshold too. With MSAA on,
only edges (the thin gizmo tori and lines, text and helper outlines) differ.
These cases are bounded at:

- transform controls: 3% / 0.6;
- WebGPU curve modifier: 1.2% / 0.35;
- instanced curve modifier: 2% / 0.4;
- skinning IK: 1.5% / 0.3.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including the
  six cube-camera faces' culling in the IK example.
- **Uploads.**
  - The curve examples write no geometry or texture data in steady frames.
    The spline texture and curve lines are rewritten only on drag end.
  - The IK example streams only the CCDIKHelper line (96 bytes against the
    original's 240) and the skin palettes.
- **Warmed cycles.** Warmed cycles of time, input and resize create no GPU
  resources.

No GPU timing parity is claimed. Full measurements are in
[transform-curves-comparison.json](transform-curves-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js transform-curves.spec.js
TRANSFORM_DPR=2 npx playwright test -c playwright.gallery.config.js transform-curves.spec.js
```
