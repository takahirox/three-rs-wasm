# Instanced and sprite picking, orientation, atlas and canvas-texture examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/interactive_objects.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_instancing_raycast` | 188 | 1,000 Phong icosahedron instances, hemisphere light, hover recoloring, `count` control, damped orbit |
| `webgl_math_orientation_transform` | 189 | Normal-shaded cone turning toward a new random target every 2 s with `rotateTowards`, `useLookAt` control, wireframe sphere |
| `webgl_panorama_cube` | 190 | Inverted box with six materials cut from one atlas, damped orbit with negative rotate speed |
| `webgl_materials_texture_canvas` | 191 | Rotating box textured by the 128×128 DOM drawing canvas |
| `webgl_raycaster_sprite` | 192 | Three sprites with center, rotation and fixed-size variants, hover highlight, orbit/pan/zoom |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

- **Instancing.** The icosahedra are one instanced draw with resident instance
  matrices and colors. The per-frame raycast is the original's CPU query. It
  honors the active `count`. A hovered white instance takes a random sRGB color,
  as with `setColorAt`. The mouse starts at (1, 1), off the grid.
- **OrbitControls.** Rust keeps the controls' spherical state, damping factor
  (0.05), rotate speed, distance limits, wheel scale and screen-space panning.
  It calls the same update step from every pointer and wheel handler, as
  OrbitControls does. Damped scenes also update once per frame. Keyboard
  controls are not ported.
- **Orientation.** `Matrix4.lookAt`, `setFromRotationMatrix`, `angleTo` and the
  r186 `slerp` are reproduced in f64. Targets follow `setTimeout( 2000 )` on the
  example clock, and the rotation step uses the Timer delta between frames.
- **Output encoding.** WebGL writes `MeshNormalMaterial` output without sRGB
  encoding, so this scene renders raw display values. Its translucent wireframe
  uses the encoded 0xcccccc value, so blending happens in display space, as in
  WebGL. The Phong, atlas and canvas scenes encode sRGB before blending and the
  MSAA resolve. The sprite scene writes the sprites' encoded colors directly.
- **Panorama atlas.** The atlas is decoded by the browser and split into six
  square tiles, like the original's `drawImage` crops. Each tile has
  generated mipmaps.
- **Canvas texture.** The drawing canvas is the original DOM element, and its
  pointer handlers call Rust. Rust issues the same Canvas2D calls, including the
  never-reset path that redraws the whole stroke each time. Each change copies
  the canvas into the resident texture on the GPU (`copyExternalImageToTexture`)
  and regenerates its mipmaps there.
  - *Earlier attempt.* A `getImageData` readback made Chrome rasterize the canvas
    in software. The strokes came out lighter than the original's.
  - *Page styling.* The gallery page stretches every canvas, so the drawing
    canvas keeps an explicit 128 CSS-pixel size.
- **Sprites.** Sprites reuse the resident sprite material. The shader's
  `center` offset is baked into each quad. Fixed-size sprites scale by view
  depth, and rotation is applied after alignment. Hover picking ports
  `Sprite.raycast` for these sprites. It runs in the pointer handler, after the
  controls have moved the camera, in the original's listener order.
- **Frame-count animation.** The canvas cube's rotation of 0.01 rad per frame
  uses 60 fps time. Stats and lil-gui styling are not reproduced.

## Comparison

`reference/three-js/interactive-objects.html` executes the pinned sources. It
seeds randomness and fixes time. It drives the orientation example's Timer and
`setTimeout` from the fixed clock, captures GUI controllers, and adds the
drawing canvas with its original CSS. The suite covers:

- time steps and every control;
- hover sequences, drawing strokes, and drags settled over 240 frames;
- wheel zoom, right-button pan, and resize.

It runs at DPR 1 and 2, with and without MSAA wherever the original enables
antialiasing.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Instancing, MSAA off | 0% / 0 | 0% / 0 |
| Orientation, MSAA off | 0.236% / 0.22 | 0.121% / 0.12 |
| Panorama cube | 0% / 0.05 | 0% / 0 |
| Canvas texture, MSAA off / on | 0% / 0, 0.226% / 0.14 | 0% / 0, 0.113% / 0.07 |
| Sprites, MSAA off / on | 0% / 0, 0.200% / 0.06 | 0% / 0, 0.100% / 0.03 |
| Instancing, 4× MSAA | 1.111% / 0.30 | 0.557% / 0.15 |
| Orientation, 4× MSAA | 8.593% / 2.10 | 5.088% / 1.28 |

With MSAA off, the remaining orientation pixels lie on the sphere's equator
line. It falls exactly on a pixel-row boundary, where the two backends break
line-rasterization ties differently.

With 4× MSAA, the instancing differences lie only on sphere silhouettes. The
orientation differences lie on the 1-pixel wireframe lines, whose coverage
WebGL and WebGPU resolve differently. This is the dense-line case recorded in
[buffer-particles.md](buffer-particles.md).

The suite requires the ordinary threshold with MSAA off. With MSAA on, it bounds
instancing to 1.25% / 0.35 and orientation to 9.5% / 2.3. These bounds apply
only to those two cases. They do not relax geometry, controls, residency or
workload checks.

## Performance evidence

- **Draw workload.** The measured draws equal the original:
  - one instanced draw of 960 vertices × 1,000;
  - the cone, target and wireframe sphere (48 / 2,880 / 11,904 indices);
  - six atlas-face draws;
  - one canvas-cube draw;
  - three sprite draws.
- **Uploads.** After a warm pass, a measured frame uploads no geometry,
  attributes, transform records or texture data.
- **Warmed cycles.** Cycles of time steps, controls, hover, drawing, drags and
  resize leave GPU resource creation, geometry transfer and texture counts
  unchanged.
- **Canvas edits.** Drawing rewrites the existing texture and its mip chain in
  place. Nothing is reallocated.

No GPU timing parity is claimed. Full measurements are in
[interactive-objects-comparison.json](interactive-objects-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js interactive-objects.spec.js
OBJECTS_DPR=2 npx playwright test -c playwright.gallery.config.js interactive-objects.spec.js
```
