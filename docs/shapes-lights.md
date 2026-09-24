# Cube-mapped heads, STL, extruded shapes, spot lights and the hemisphere light

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/shapes_lights.rs`, with STLLoader, Earcut and
ExtrudeGeometry in `src/browser/shapes_lights/formats.rs`. The curve's
arc-length sampling and Frenet frames are in `src/curve.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_materials_cubemap` | 243 | Three Walt heads reflecting and refracting the castle cube, the cube background, the polar-limited orbit |
| `webgl_loader_stl` | 244 | The ASCII, binary and colored STL models with fog, a hemisphere light and two shadow-casting lights |
| `webgl_geometry_extrude_shapes` | 245 | The triangle and star extruded along closed and random splines, and the bevelled star, TrackballControls |
| `webgl_lights_spotlights` | 246 | Three tweened shadow-casting spot lights, their helpers, the orbit |
| `webgl_lights_hemisphere` | 247 | The morphing flamingo, the hemisphere and shadow-casting directional light with their helpers, the sky shader, the light toggles |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Cube map

- **Materials.** Lambert's `envmap_fragment` is ported:
  - the camera-to-fragment ray is reflected (or, for the middle head,
    refracted at 0.95) about the world normal;
  - it samples the cube with x flipped;
  - it is combined by MultiplyOperation, or by MixOperation at 0.3 for the
    third head.
- **Background.** The castle cube is drawn as a camera-centered box at the
  far plane.
- **Controls.** OrbitControls keep the example's polar limits (π/4 to π/1.5),
  with zoom and pan disabled.

### STL

- **Parsing.** The reader follows `STLLoader`:
  - binary files by their size test, with face normals;
  - the Materialise `COLOR=` header and 15-bit face colors, converted from
    sRGB;
  - ASCII `normal` and `vertex` records.
- **Shadows.** The two lights keep `addShadowedLight`'s camera, bias −0.002
  and default 512-texel maps.

### Extruded shapes

- **Geometry.** `ExtrudeGeometry` is ported:
  - the shape's line points;
  - the clockwise reversal and `mergeOverlappingPoints`;
  - bevel vectors and layers;
  - lid faces (group 0) and side walls (group 1), non-indexed with computed
    normals.
- **Triangulation.** Earcut's unhashed path is ported for these simple
  contours.
- **Paths.** Path extrusion uses the curve's `getSpacedPoints` over 200
  arc-length divisions and `computeFrenetFrames`, including the closed-curve
  twist correction.
- **Check.** All three geometries match Three.js's positions to within Float32
  rounding (at most 6e-6) and have the same group ranges.
- **Not generated.** The UV attribute; the Lambert materials do not use it.
- **Randomness.** `MathUtils.randFloat` draws the random spline from the
  seeded stream, in the port and in the reference fixture alike.

### Spot lights

- **Tweens.** TWEEN's semantics are ported:
  - each `updateTweens` starts, per light, an angle and penumbra tween and a
    position tween from the light's current values, with
    `Easing.Quadratic.Out`;
  - later tweens overwrite earlier ones in update order;
  - finished tweens are removed.
- **Timing.** The five-second `setTimeout` runs on the example clock.
  Tweens start at the timer's time and update at the frame's time, in the
  port and in the reference fixture alike.
- **Penumbra.** The tweened penumbra ranges from 1 to 2. The renderer now
  accepts such values as Three.js does: `cos( angle × ( 1 − penumbra ) )` is
  unclamped.
- **Shadows.** The spot shadows take their field of view from the angle, near
  0.5 and far 50 (the light distance).
- **Helpers.** Each `SpotLightHelper` cone is updated every frame.

### Hemisphere light

- **Flamingo.** The glTF morph animation plays with `setDuration( 1 )`. Its
  Standard materials use r186's energy-conserving lighting, as WebGL applies
  it.
- **Helpers.** The helpers follow their constructors:
  - `HemisphereLightHelper`: the rotated octahedron, sky- and ground-colored
    halves, turned away from the light;
  - `DirectionalLightHelper`: the light plane, and the target line scaled to
    the light distance.
- **Sky.** The sky ShaderMaterial writes its gradient without output
  encoding. The port decodes it so the encoded target stores the original's
  values.
- **Toggles.** The two light toggles are checkboxes in the gallery.
- **Not ported.** The shadow-intensity control.

Stats and the lil-gui appearance are not reproduced. OrbitControls and
TrackballControls touch input is not separately verified.

## Comparison

`reference/three-js/shapes-lights.html` executes the pinned sources on the
example clock. It uses seeded randomness and example-clock TWEEN timers. The
suite covers:

- time steps across several tween periods;
- the light toggles;
- orbit and trackball drags, pans, wheels and key holds;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Cube map, MSAA off / on | 0.129% / 0.07, 1.421% / 0.26 | 0.056% / 0.04, 0.622% / 0.14 |
| STL, MSAA off / on | 0.318% / 0.06, 0.672% / 0.12 | 0.321% / 0.06, 0.491% / 0.09 |
| Extruded shapes, MSAA off / on | 0% / 0.00, 0.692% / 0.11 | <0.001% / 0.00, 0.349% / 0.06 |
| Spot lights, MSAA off / on | 0.012% / 0.00, 1.788% / 0.55 | 0.009% / 0.00, 0.909% / 0.28 |
| Hemisphere light, MSAA off / on | 0.174% / 0.05, 0.661% / 0.18 | 0.131% / 0.04, 0.374% / 0.10 |

### MSAA

All five scenes match within the ordinary threshold with MSAA off. With 4×
MSAA, the differences lie on silhouettes and helper lines. The suite requires,
with MSAA:

| Case | Bound |
| --- | --- |
| Cube map | 2% / 0.35 |
| STL | 1% / 0.2 |
| Extruded shapes | 1% / 0.2 |
| Spot lights | 2.5% / 0.7 |
| Hemisphere light | 1% / 0.25 |

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including each
  shadow pass. Draws of three or fewer vertices are left out on both sides:
  the port's fullscreen triangles, and the directional helper's two-vertex
  target line.
- **Uploads.** After a warm pass, no scene uploads geometry or texture data.
  The flamingo's morph weights travel as the renderer's pose data.
- **Warmed cycles.** Warmed cycles of time and input create no GPU resources.
  The hemisphere toggles are left out of that cycle: switching a light on or
  off rebuilds the renderer's light and shadow bindings.

No GPU timing parity is claimed. Full measurements are in
[shapes-lights-comparison.json](shapes-lights-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js shapes-lights.spec.js
SHAPES_DPR=2 npx playwright test -c playwright.gallery.config.js shapes-lights.spec.js
```
