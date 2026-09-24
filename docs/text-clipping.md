# Advanced clipping, spline tubes, 3D text, tessellated text and text lines

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

The scenes are implemented in `src/browser/text_clipping.rs`, and the
CurveExtras curves and TubeGeometry are in `src/browser/text_clipping/tube.rs`.
The following are in `src/browser/text_shapes.rs` and are shared by the three
text scenes:

- FontLoader's typeface glyph paths;
- `ShapePath.toShapes`;
- Earcut with holes;
- ExtrudeGeometry with holes and bevels.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_clipping_advanced` | 248 | The instanced boxes clipped by a moving tetrahedron, with clipped shadows from a spot light and a directional light. The rotating cylindrical global planes, the plane visualization, the four toggles and the orbit |
| `webgl_geometry_extrude_splines` | 249 | The 16 selectable curves as tubes with a wireframe overlay, the spline camera with look-ahead, the CameraHelper, the eight controls and the orbit |
| `webgl_geometry_text` | 250 | Mirrored bevelled text in fog, the four buttons (color, font, weight, bevel), typing and backspace, and the eased drag rotation |
| `webgl_modifier_tessellation` | 251 | The centered bevelled text, tessellated, with random face colors and per-face displacement in a raw ShaderMaterial, and TrackballControls |
| `webgl_custom_attributes_lines` | 252 | The text's vertices as one additive line strip, with a per-vertex random-walk displacement and HSL colors |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Text geometry

- **Fonts.** Glyph outlines follow `Font.generateShapes`:
  - the `m`, `l`, `q` and `b` commands, scaled by `size / resolution`;
  - each glyph advancing by its `ha`, and missing glyphs replaced by `?`;
  - newlines moving down by the line height.
- **Curves.** `CurvePath.getPoints` samples lines once and curves
  `curveSegments` times, skipping repeated points.
- **Shapes.** `ShapePath.toShapes` is r186's version:
  - subpaths sorted by area;
  - nesting found by containment tests on interior points;
  - outer shapes and holes assigned by non-zero winding.
- **Triangulation.** Earcut is ported in full:
  - hole elimination and bridge finding;
  - z-order hashing for contours above 80 points;
  - local intersection curing and polygon splitting.
- **Extrusion.** ExtrudeGeometry follows the original:
  - holes reversed against the contour;
  - bevel vectors for the contour and holes;
  - contracted and expanded bevel layers;
  - side walls for every ring;
  - lid and side groups per shape.
- **Check.** Five text geometries match Three.js's positions and groups to
  within Float32 rounding (at most 3e-5). They cover four faces, bevels on and
  off, 3 to 10 curve segments and glyphs with holes.
- **Buttons.** The four GUI buttons are gallery buttons.
- **Font loading.** All ten faces the buttons can select are fetched when the
  scene loads, and each is parsed on first use. The original fetches a face
  when it is selected.
- **Input.** The gallery forwards `keydown` and `keypress` as the original's
  document listeners receive them. The first keydown clears the text, and
  backspace removes a character.

### Advanced clipping

- **Planes.** Each frame, as in the original:
  - the tetrahedron's `setFromCoplanarPoints` planes are transformed by the
    object's matrix and the bouncing scale;
  - the plane meshes take the same transform times `planeToMatrix`;
  - the five cylindrical global planes turn with `makeRotationY( time × 0.1 )`.
- **Shadows.** Clipped shadows use the material's local planes only, since
  `WebGLClipping` never applies `renderer.clippingPlanes` to shadow maps.
- **Controls.** Turning local clipping off also hides the visualization, as
  the property setter does. The gallery checkbox is not updated by `listen()`.

### Spline tubes

- **Curves.** The 14 CurveExtras curves and the two CatmullRom splines use the
  shared `Curve` sampling in `src/curve.rs`:
  - arc-length mapping over 200 divisions;
  - tangents;
  - Frenet frames with the closed-curve twist correction.
- **Tubes.** TubeGeometry is ported, including the closed tube's repeated
  first row. A geometry control rebuilds the tube, as `addTube` does.
- **Camera.** The spline camera moves on a 20-second loop along the scaled
  path, offset by the interpolated binormal, with or without look-ahead.
  - It keeps the aspect it was created with, as the original never updates it.
  - Animation view renders through it.
  - The CameraHelper unprojects its frustum on the GPU from the camera's
    world and inverse projection matrices.

### Tessellated text

- **Geometry.** `center()` and `TessellateModifier( 8, 6 )` are ported.
- **Attributes.** Each face's random HSL color is the color attribute. Its
  random displacement is stored in `uv.x`, resident and uploaded once.
- **Output.** The raw ShaderMaterial writes its lit color without output
  encoding, as the original does.

### Text lines

- **Drawing.** The non-indexed text geometry is drawn as a single line strip
  with additive blending and no depth test.
- **Displacement.** The random walk takes three draws per vertex per 60 fps
  step. The whole Float32 array is uploaded each frame, as the original's
  `needsUpdate` does.

Stats and the lil-gui appearance are not reproduced. OrbitControls and
TrackballControls touch input is not separately verified.

## Comparison

`reference/three-js/text-clipping.html` executes the pinned sources on the
example clock, with seeded randomness. Per-frame easing, controls and random
walks run as 60 fps steps. The suite covers:

- time steps;
- all toggles and buttons, and the geometry controls;
- typing and backspace;
- drags, pans, wheels and trackball input;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Advanced clipping | 0.015% / 0.02 | 0.019% / 0.02 |
| Spline tubes, MSAA off / on | 1.381% / 1.44, 5.165% / 1.26 | 0.812% / 0.97, 2.864% / 0.73 |
| Text, MSAA off / on | 0% / 0.00, 0.878% / 0.13 | 0% / 0.00, 0.472% / 0.06 |
| Tessellated text, MSAA off / on | 0.002% / 0.00, 7.192% / 2.32 | 0.001% / 0.00, 4.206% / 1.25 |
| Text lines, MSAA off / on | 0.056% / 0.01, 10.822% / 1.19 | 0.038% / 0.01, 12.065% / 1.38 |

### Wireframe depth ties

- **Cause.** The spline tube's wireframe lies exactly on its faces and passes
  `LessEqual` only where line and face depths tie.
- **Evidence.** Measured in the reference:
  - `LessEqual` draws 3881 wireframe pixels, `Less` draws 871;
  - so most wire pixels depend on exact depth equality;
  - the port draws 3902 wire pixels, but which pixels tie differs between
    the WebGL and WebGPU depth paths.
- **Bound.** With MSAA off, the spline scene alone is bounded at 1.5% / 1.6.

### MSAA

With 4× MSAA, the differences lie on silhouettes, the dense random-colored
facets, the thousands of one-pixel line segments and the wireframe ties. With
MSAA off, the text, tessellation and line scenes match within the ordinary
threshold. The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| Spline tubes | 7% / 1.8 |
| Text | 1% / 0.2 |
| Tessellated text | 8% / 2.6 |
| Text lines | 14% / 1.6 |

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including the
  instanced boxes' two shadow passes.
- **Uploads.** After a warm pass, no scene uploads texture data, and only the
  text lines upload attribute bytes. That upload is the displacement array,
  1,278,864 bytes per frame on both sides.
- **Setup work.** Text and tube geometry is built on the CPU at setup and on
  each text or tube change, as in the original.
- **Warmed cycles.** Warmed cycles of time and input create no GPU resources.
  The spline and text controls are left out of that cycle: like the original,
  they build new geometry.

No GPU timing parity is claimed. Full measurements are in
[text-clipping-comparison.json](text-clipping-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js text-clipping.spec.js
TEXT_DPR=2 npx playwright test -c playwright.gallery.config.js text-clipping.spec.js
```
