# Orbit and map controls, camera helpers, custom attributes and draw ranges

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/controls_attributes.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `misc_controls_orbit` | 208 | 500 instanced flat-shaded cones in exponential fog, damped OrbitControls with distance and polar limits |
| `misc_controls_map` | 209 | 500 instanced boxes, damped MapControls, the `zoomToCursor` and `screenSpacePanning` controls |
| `webgl_camera` | 210 | Two viewports, a perspective/orthographic camera rig with CameraHelpers, wireframe spheres, 10,000 points, O/P keys |
| `webgl_custom_attributes` | 211 | A 128×64 displaced sphere with a per-frame CPU noise walk and an HSL-drifting color |
| `webgl_buffergeometry_drawrange` | 212 | 1,000 bouncing particles, per-frame CPU connection search, additive points and lines, all six controls |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Controls

- **OrbitControls.** The update step follows `OrbitControls.update()`:
  - the spherical coordinates are re-derived from the camera each step;
  - damping factor 0.05;
  - the distance and polar-angle clamps;
  - the safe polar range.

  As in the original, the pointer and wheel handlers call `update()` on every
  event, and the damped examples also call it once per frame.
- **Panning.** Panning uses the camera's current matrix. With
  `screenSpacePanning` off, it moves along the ground plane (`up × x`).
- **MapControls.** The mouse buttons are swapped: left pans, and right or
  modified left rotates.
- **zoomToCursor.** Follows the original: the dolly direction through the
  unprojected pointer, then the target re-derived from the view ray, with the
  20° tilt limit.
- **Not ported.** OrbitControls keyboard input and the cursor style. Map touch
  gestures are not separately verified.

### Camera helpers

- **Helpers.** `CameraHelper.update()` rewrites 50 positions on the CPU every
  frame. The port keeps the helper's fixed NDC points resident instead. The
  vertex stage applies the active camera's `matrixWorld ×
  projectionMatrixInverse`, computed in f64 with WebGL's depth range. The
  helper's unset `p` point stays at the camera position, as in the original.
- **Views.** Two viewports use WebGL's rounded `setViewport` rectangles. Each
  view's clear color is drawn by a scissored fullscreen triangle. The rig
  follows `lookAt`, the animated field of view and the far plane.
- **Points.** They are one-pixel minimum `gl_PointSize` squares from a
  resident buffer.

### Custom attributes

- **Per-frame work.** The original's per-frame work runs on the CPU in both:
  the noise random walk (`Math.random`, clamped, in Float32 storage) and
  `Color.offsetHSL` in the linear working space. It runs as 60 fps steps of
  example time. The reference fixture runs the same step count and then
  recomputes the displacement once.
- **Uploads.** Only the 8,385-value displacement attribute is written each
  frame, as the original uploads it.
- **Output.** The shader is the original's: displaced positions, amplitude UVs
  and the half-Lambert grey texture. Its output is raw.

### Draw range

- **Per-frame work.** The particle motion and the O(n²) connection search are
  the original's CPU work. They use Float32 positions and f64 velocities and
  distances, and honor the connection limits. They run as 60 fps steps of
  example time; only the last step's connections are drawn, as the original
  draws each frame's.
- **Uploads.** The port writes only the particle positions and the connected
  segments' endpoints and weights. The original re-uploads its full 12 MB
  position and color arrays each frame.
- **Draw calls.** The connection lines are one resident 2-vertex line per
  instance. Their instance count is the original's draw range. Points, lines
  and the BoxHelper blend additively in the original's order.

Stats styling and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/controls-attributes.html` executes the pinned sources. It
uses seeded randomness (including `MathUtils.randFloatSpread`) and fixed time.
The suite covers:

- time steps;
- settled damped drags, pans and wheel zooms, including a cursor zoom;
- every draw-range control, the O and P keys;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Orbit controls, MSAA off / on | 0.001% / 0.00, 0.803% / 0.12 | 0% / 0.00, 0.402% / 0.06 |
| Map controls, MSAA off / on | 0.017% / 0.01, 0.609% / 0.12 | 0.011% / 0.01, 0.310% / 0.06 |
| Camera, MSAA off / on | 0.298% / 0.57, 4.424% / 1.61 | 0.157% / 0.30, 1.630% / 0.79 |
| Custom attributes | 0.003% / 0.00 | 0% / 0.00 |
| Draw range, MSAA off / on | 0.062% / 0.08, 28.955% / 8.02 | 0.037% / 0.04, 22.820% / 6.41 |

### MSAA

All five scenes match within the ordinary threshold with MSAA off. That
includes every draw-range state, so the particle simulation, connections and
weights agree.

With 4× MSAA, the differences lie on:

- the controls scenes' box and cone silhouettes;
- the camera scene's wireframes and one-pixel points;
- the draw-range scene's dense additive lines.

In the draw-range scene, per-sample line coverage differs between WebGL and
WebGPU, and thousands of overlapping additive lines accumulate the per-pixel
differences. The total light is the same: in every state, the mean RGB level of
the port is within 0.1% of the original's (for example 48.102 against 48.097).

The suite requires, with MSAA:

- the draw-range mean levels within 0.5%, in every state;
- orbit within 1% / 0.15;
- map within 0.75% / 0.15;
- camera within 5% / 1.8;
- draw range within 33% / 9.2.

MSAA-off comparisons keep the ordinary threshold.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - one instanced draw of 500 cones or boxes;
  - eight camera draws: the helper, five wireframe sphere draws across the two
    views after the same frustum culling, and the points in both views;
  - one displaced sphere;
  - the BoxHelper, 500 point billboards and the connected segments (5,550
    line vertices in the measured frame).

  WebGL points are counted as six-vertex billboards.
- **Attribute uploads per measured frame:**

  | Example | Port | Original |
  | --- | ---: | ---: |
  | Controls scenes | 0 bytes | 0 bytes |
  | Camera | 0 bytes | 600 bytes (helper positions) |
  | Custom attributes | 33,540 bytes (the displacement) | 33,540 bytes |
  | Draw range | 104,800 bytes | 24,012,000 bytes |

  The suite requires zero for the static scenes, and no more than the original
  for the two streaming scenes.
- **Warmed cycles.** Warmed cycles of time, controls, keys, parameters and
  resize create no GPU resources.

No GPU timing parity is claimed. Full measurements are in
[controls-attributes-comparison.json](controls-attributes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js controls-attributes.spec.js
CONTROLS_DPR=2 npx playwright test -c playwright.gallery.config.js controls-attributes.spec.js
```
