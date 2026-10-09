# SVGRenderer

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

`src/svg.rs` is the Rust SVGRenderer and SVGObject of
`examples/jsm/renderers/SVGRenderer.js`, together with the Projector it draws
through (`examples/jsm/renderers/Projector.js`). The scene's faces, lines,
sprites and points are projected and clipped on the CPU, painter-sorted, and
written as SVG `<path>` elements. The original works the same way: SVG
output is CPU projection in three.js too, so no GPU work is replaced.

| Three.js | Rust |
| --- | --- |
| `new SVGObject( node )` | `NodeKind::SVGObject( SVGObject::new( node ) )` |
| `new THREE.Sprite( new SpriteMaterial( … ) )` | `NodeKind::Sprite( Sprite { material } )` with `SpriteMaterial` |
| `new SVGRenderer()` | `SVGRenderer::new()` |
| `domElement`, `getSize()`, `setSize( w, h )`, `setQuality()`, `setClearColor()`, `setPrecision()`, `clear()` | `dom_element()`, `get_size()`, `set_size( w, h )`, `set_quality()`, `set_clear_color()`, `set_precision()`, `clear()` |
| `autoClear`, `sortObjects`, `sortElements`, `overdraw`, `info.render` | `auto_clear`, `sort_objects`, `sort_elements`, `overdraw`, `info` |
| `render( scene, camera )` | `render( &mut scene, camera )`; `render_from( &mut scene, &mut camera_scene, camera )` takes the camera from another scene |
| `THREE.ColorManagement.enabled` | `SVGRenderer::color_management` |

- **The same output.** Projection, near/far clipping, frustum culling,
  backface tests, painter sorting, overdraw and path batching follow the
  original's operations. That includes:
  - three.js's matrix inverse and normal matrix;
  - the camera's decompose/recompose when its scale is not exactly one;
  - the reuse of intersection vertices during triangle clipping;
  - WebGL clip space ( z from −1 to 1 ).

  Path numbers are printed as JavaScript prints them.
- **SVGObjects.** As in the original, they are placed with the camera's
  world matrix as it stood before the render updated it. Faces, lines and
  sprites use the updated matrix.
- **Materials.** These are written as the original writes them:
  - MeshBasicMaterial, and Lambert, Phong and Standard with the original's
    ambient, directional and point light sums;
  - MeshNormalMaterial;
  - LineBasicMaterial, and its dashed form;
  - SpriteMaterial and PointsMaterial.

  Faces of other materials keep the previous face's color, as in the
  original. Line caps and wireframe joins use the materials' defaults
  ( `round` ).
- **Sprites.** `NodeKind::Sprite` exists for SVGRenderer. The GPU renderer
  does not draw sprites yet.
- **Precision.** `set_precision` uses Rust's decimal formatting. It can round
  an exact half differently from JavaScript's `toFixed`.
- **Platforms.** The renderer exists in browser ( wasm ) builds. Native
  builds carry SVGObject nodes without elements.

## Examples

| Example | Id | Notes |
| --- | --- | --- |
| `svg_lines` | 479 | Three width-10 lines and a dashed one around a circle, turned with the scene by the clock |
| `svg_sandbox` | 480 | The QR code ( Lambert, vertex colors ), cubes, a plane, a cylinder, a turning triangle field, sprites, SVG circles and an SVG file, with damped OrbitControls |

Both pages turn color management off, and so do their ports. The SVG
element lies over the gallery's canvas and carries the pointer.

svg_sandbox's per-frame turn of the triangle field and its controls run as
60 fps steps, as the fixture runs them. OrbitControls' pointer and wheel
handlers call `update()`, as in r186. Each update keeps the camera's previous
rotation in its world matrix until the next render, because three.js's
`lookAt` updates the world matrix before turning the camera. The SVGObjects
show that lag for one frame.

`tests/browser/texture-volumes.spec.js` compares, against the original pages:
- every child of the SVG element: its tag, path data, style, transform and
  shape rendering. Numbers must match to 1e-6 over the clock, a drag and a
  wheel. After an input, the comparison reads the second frame: the gallery
  renders on input, while the fixture renders only when asked.
- the captures, at the default thresholds.
