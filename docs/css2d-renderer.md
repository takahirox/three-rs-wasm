# CSS2DRenderer

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

`src/css2d.rs` is the Rust CSS2DRenderer and CSS2DObject of
`examples/jsm/renderers/CSS2DRenderer.js`: HTML elements over the canvas at
their objects' projected positions.

| Three.js | Rust |
| --- | --- |
| `new CSS2DObject( element )` | `NodeKind::CSS2DObject( CSS2DObject::new( element ) )`, inserted and added like any node |
| `object.element`, `center`, `rotation2D` | `CSS2DObject::element`, `center`, `rotation_2d` |
| `new CSS2DRenderer( { element } )` | `CSS2DRenderer::new( CSS2DRendererParameters { element } )` |
| `domElement`, `getSize()`, `setSize( w, h )`, `sortObjects` | `dom_element()`, `get_size()`, `set_size( w, h )`, `sort_objects` |
| `render( scene, camera )` | `render( &mut scene, camera )` |

- **The same output.** For each object the renderer writes `display`,
  `transform-origin`, `transform` and `z-index` as the original does, numbers
  printed as JavaScript prints them. Objects behind the camera, outside its
  layers or under an invisible node are hidden. The z-index follows render
  order, then distance to the camera.
- **Removal.** As CSS2DObject's `removed` listener does, removing a node from
  its parent takes its subtree's elements out of the DOM; the next render
  appends visible ones again.
- **Copies.** Cloning a CSS2DObject clones its element ( `cloneNode( true )` ).
- **Platforms.** The renderer exists in browser ( wasm ) builds; native builds
  carry CSS2DObject nodes without elements.
- **Clip depth.** The original's visibility test, −1 ≤ z ≤ 1 in WebGL clip
  space, is 0 ≤ z ≤ 1 in the engine's WebGPU clip space.

## css2d_label ( 472 )

The Earth and the Moon with name and mass labels on camera layers 0 and 1,
toggled by the GUI. The label renderer's element lies over the canvas and
carries OrbitControls, as on the page. The gallery page is `lang="ja"` and
smooths fonts; the label renderer's element restores the page's English
generic fonts and main.css's inherited body styles.

`tests/browser/texture-volumes.spec.js` compares the captures ( labels
included ) at the default thresholds, and every label's inline style string
with the original's over the clock, the layer buttons and a wheel.
