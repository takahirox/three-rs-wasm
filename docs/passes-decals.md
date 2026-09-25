# Composer backgrounds, lava, uniform buffers, RGB halftone and decals

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/passes_decals.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_postprocessing_backgrounds` | 288 | ClearPass, TexturePass, CubeTexturePass, RenderPass and OutputPass, the orbit without zoom, all eight controls |
| `webgl_shader_lava` | 289 | The lava ShaderMaterial torus, BloomPass, OutputPass, the time and rotation at delta × 5 |
| `webgl_ubo` | 290 | 200 tetrahedra and crates with the ViewData and LightingData uniforms, the per-frame rotations |
| `webgl_postprocessing_rgb_halftone` | 291 | 50 normal/UV cubes, the Phong floor, the rotating group and point light, HalftonePass, the orbit, all ten controls |
| `webgl_decals` | 292 | LeePerrySmith, the pointer raycast and normal line, DecalGeometry on click, the three controls and Clear, the orbit |

None of these scenes has a WebGPU equivalent. All five are compared against
the WebGL renderer.

## Port notes

### Composer backgrounds

- **Passes.** Each pass draws into the composer's read buffer, a half-float
  target with depth:
  - ClearPass fills it with the clear color and alpha;
  - TexturePass draws the wood texture times the opacity with premultiplied
    blending;
  - CubeTexturePass draws a 10-unit back-sided box with the sRGB pisa cube,
    flipped in x. Its camera copies the view camera's projection and rotation.
    Below opacity 1 it blends normally;
  - RenderPass draws the sphere and the three point lights without clearing.

  OutputPass then encodes sRGB to the canvas.
- **Ping-pong.** OutputPass swaps the read and write buffers every frame. With
  ClearPass off, a frame shows the buffer drawn two frames earlier. The port
  keeps both buffers and alternates them.
- **Controls.** OrbitControls has zoom disabled.

### Lava

- **Shader.** The fragment shader samples `cloud.png` (NoColorSpace) and
  `lavatile.jpg` (sRGB) with TextureLoader's flipped rows and `uvScale`
  (3, 1).
- **Fog.** The fog depth is `gl_FragCoord.z / gl_FragCoord.w`. That is GL's
  window depth times w, not the view depth, and is computed from the camera's
  near and far planes.
- **Color profile.** `lavatile.jpg` carries an Adobe RGB profile, which the
  reference's WebGL upload does not apply. The port decodes it without color
  management.
- **Bloom.** BloomPass(1.25) is ported:
  - a 25-tap Gaussian (sigma 4) in steps of 1/512 of the texture, first
    horizontal, then vertical, into full-resolution half-float targets;
  - an additive combine with SRC_ALPHA, ONE blending, so the torus's
    unclamped alpha also scales the bloom.

  Negative and over-range values are kept in the half-float targets.
- **Time.** The timer delta is scaled by 5 for the time uniform and the
  rotation.

### Uniform buffers

- **Shaders.** The two RawShaderMaterials become two programs:
  - ViewData maps to the renderer's shared per-frame camera uniforms;
  - LightingData never changes, so its values are compiled into both
    programs as constants.
- **Lighting.** Lighting is Phong in eye space:
  - the light is at (0, 0, 10) in eye space;
  - the ambient, diffuse and specular colors are converted to linear as
    `THREE.Color` does;
  - the shininess is 64.
- **Color.** Output is sRGB. Each tetrahedron's color is
  `Color( 0xffffff * random )`, which floors the hex.
- **Rotation.** The rotations accumulate as Euler XYZ angles.

### RGB halftone

- **Scene.** The cubes' ShaderMaterial writes `abs( normal ) + ( uv, 0 )` with
  alpha 0.
- **Pass.** HalftoneShader is ported with GL's bottom-up pixel coordinates:
  - all five shapes;
  - the per-channel grid angles, scatter and the eight-sample average;
  - the five blending modes and greyscale.

  Its width and height are the composer's target size.
- **Output.** The pass writes to the canvas without an output conversion, as
  the original's last pass does.

### Decals

- **Raycast.** It runs on the head's object-space triangles with front-face
  culling, as Raycaster does for a FrontSide material, from the pointer
  position in CSS pixels.
- **Line.** The helper line from the hit to 10 units along the face normal is
  a resident unit segment placed by its transform. The original rewrites two
  vertices instead. It is hidden until the first hit, as the original's
  zero-length line draws nothing.
- **Shooting.** A click without an orbit change shoots a decal. The mouse
  helper's `lookAt` gives the orientation, as Euler XYZ; then:
  - the random z rotation, if enabled;
  - the random scale;
  - the random color.
- **DecalGeometry.** DecalGeometry is ported: the six box-plane clips and the
  box UVs. The decal is built once on the CPU at the click, as the original
  does.
- **Material.** The decal material is Phong with transparency, no depth write,
  polygon offset factor −4 (a new material property) and render order by
  index.

lil-gui and Stats appearance are not reproduced. OrbitControls touch input is
not separately verified.

## Comparison

`reference/three-js/passes-decals.html` executes the pinned sources on the
example clock.

- **Coverage.** Every compared control, clicks that shoot decals, orbit input
  and resize, at DPR 1 and 2.
- **Loading.** The fixture waits for the decal example's glTF head.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Composer backgrounds (no MSAA) | 0% / 0.17 | 0% / 0.08 |
| Lava (no MSAA) | 0.113% / 0.08 | 0.064% / 0.04 |
| Uniform buffers, MSAA off / on | 0% / 0.01, 1.287% / 0.24 | 0.001% / 0.01, 0.632% / 0.12 |
| RGB halftone (no MSAA) | 0.002% / 0.00 | 0.003% / 0.00 |
| Decals, MSAA off / on | 0.036% / 0.09, 0.306% / 0.15 | 0.019% / 0.04, 0.156% / 0.07 |

The uniform-buffer scene's many small crates and tetrahedra differ at MSAA
edges. With MSAA on, that case is bounded at 1.5% / 0.3. With MSAA off it
matches within the ordinary threshold.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the background box and sphere;
  - the torus;
  - the 96 visible uniform-buffer meshes;
  - the 47 visible halftone meshes;
  - the head.

  The helper line and the fullscreen passes draw three or fewer vertices on
  both sides.
- **Uploads.** No scene uploads geometry or texture data by writes after a
  warm pass.
- **Warmed cycles.** Warmed cycles create no GPU resources. They cover time,
  controls, pointer moves over the head, orbit input and resize.
- **Decals.** A decal's geometry is created once per click, as the original
  creates it.

No GPU timing parity is claimed. Full measurements are in
[passes-decals-comparison.json](passes-decals-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js passes-decals.spec.js
PASSES_DPR=2 npx playwright test -c playwright.gallery.config.js passes-decals.spec.js
```
