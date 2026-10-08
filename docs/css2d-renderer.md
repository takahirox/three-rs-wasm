# CSS2DRenderer and CSS3DRenderer

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

# CSS3DRenderer

`src/css3d.rs` is the Rust CSS3DRenderer, CSS3DObject and CSS3DSprite of
`examples/jsm/renderers/CSS3DRenderer.js`: HTML elements placed in 3D by CSS
transforms under a perspective or orthographic camera.

| Three.js | Rust |
| --- | --- |
| `new CSS3DObject( element )`, `new CSS3DSprite( element )` | `NodeKind::CSS3DObject( CSS3DObject::new( element ) )`, `NodeKind::CSS3DSprite( CSS3DSprite::new( element ) )` |
| `object.element`, `sprite.rotation2D` | `element`, `rotation_2d` |
| `new CSS3DRenderer( { element } )` | `CSS3DRenderer::new( CSS3DRendererParameters { element } )` |
| `domElement`, `getSize()`, `setSize( w, h )` | `dom_element()`, `get_size()`, `set_size( w, h )` |
| `render( scene, camera )` | `render( &mut scene, camera )`; `render_from( &mut scene, &camera_scene, camera )` takes the camera from another scene |

- **The same output.** The renderer writes the camera element's
  `perspective`, `scale`, `translate` and `matrix3d`, the view element's view
  offset, and every object's `translate(-50%,-50%)matrix3d(...)` as the
  original does. Numbers are printed as JavaScript prints them, and
  magnitudes below 1e-10 are written as 0. Styles are written only when they
  change.
- **Sprites.** A CSS3DSprite faces the camera: its matrix is the camera's
  inverse rotation, optionally turned by `rotation_2d`, at the sprite's
  position and scale.
- **Removal and platforms** are as for CSS2DRenderer. `Scene::dispose`
  detaches the elements of a subtree that is not added again; a node only
  removed from its parent becomes a root and still renders.

## Examples

| Example | Id | Notes |
| --- | --- | --- |
| `css3d_sandbox` | 473 | Random coloured divs and matching wireframe planes, TrackballControls, view offset GUI |
| `css3d_orthographic` | 474 | Four divs and wireframe planes under an orthographic camera, OrbitControls zoom 0.5–2 |
| `css3d_sprites` | 475 | 512 sprites tweened between four layouts every 6 s |
| `css3d_periodictable` | 476 | 118 element cards tweened into table, sphere, helix and grid by the page's buttons |
| `css3d_molecules` | 477 | PDB atoms as tinted ball sprites and bonds as crossed divs; visualisation and 17 molecules |
| `css3d_mixed` | 478 | An iframe behind the canvas, seen through a NoBlending premultiplied cutout |

The pages' tweens follow tween.js 23.1.1, and their Euler and quaternion
conversions use three.js's operations, so the transforms match the
original's to 1e-6. The pages without a canvas ( sprites, periodictable,
molecules ) hide or clear the gallery's canvas beneath the CSS layer. The
gallery's own stylesheet rules ( font smoothing, border-box sizing, the
examples index's iframe placement ) are undone for the CSS layer.

`css3d_mixed` iframes `./#webgl_animation_keyframes` as the original does.
Here that URL opens this gallery rather than three.js's examples index. The
canvas uses premultiplied alpha. The cutout's material uses NoBlending
( `blending: Some( BlendState::REPLACE )` ), which keeps its zero opacity
where an opaque material would write 1, and `premultiplied_alpha`, which
writes ( 0, 0, 0, 0 ). Damped OrbitControls run on a full-window element under
the CSS layer, and the iframe takes the pointer where it shows.

`tests/browser/texture-volumes.spec.js` compares, against the original
pages:
- every CSS element's transform, display, width and height over the clocks,
  buttons, GUI values and controls;
- the captures. css3d_sprites allows 2% of pixels for the sprites' edge
  resampling. css3d_sandbox's MSAA capture allows 1.2%. css3d_mixed allows
  1.5% for line and oblique-edge rasterization after its drag. In
  css3d_mixed both iframes are served the same plain page.

css3d_sandbox's drag is not compared: the original's TrackballControls
inertia runs on real time. `css3d_youtube` is not ported; it embeds YouTube
videos.
