# Shadow meshes, dynamic instancing, depth texture, render to texture and the normal-map composer

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/shadow_rtt.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_shadowmesh` | 278 | Five animated objects with ShadowMesh planar shadows, the stencil, ArrowHelpers, the directional/point light switch |
| `webgl_instancing_dynamic` | 279 | 10,000 instances with per-frame matrices, the tweened colors, RoomEnvironment, Neutral tone mapping, the camera path |
| `webgl_depth_texture` | 280 | 50 instanced torus knots rendered to a target with a DepthTexture, the linearized depth pass, the damped orbit, the format, type and samples controls |
| `webgl_rtt` | 281 | The shader quad and Phong tori rendered to a texture, the screen quad and 25 textured spheres without autoClear, the mouse-following camera |
| `webgl_materials_normalmap` | 282 | The LeePerrySmith head with color, specular and normal maps, the BleachBypass, ColorCorrection, Output and FXAA composer, the damped orbit, the normal-map controls |

These five WebGL examples have no WebGPU equivalent and are compared against
the original WebGL renderer.

## Port notes

### Shadow meshes

- **Projection.** `ShadowMesh.update()` is ported: the planar projection onto
  y = 0.01 from the light's homogeneous position (w 0.001 or 0.9), times the
  mesh's world matrix from its last render. The shadow program projects with
  the full matrix, keeping its w.
- **Stencil.** The black shadows are drawn at 0.6 opacity without depth
  writes. The stencil test (equal 0, increment on pass) darkens each pixel
  once. The canvas target has a stencil attachment, as `{ stencil: true }`
  requests.
- **Helpers.** ArrowHelper is ported: a line and a five-sided cone, turned
  from +Y to the light direction.
- **Motion.** Rotations and angles advance by the timer's delta, wrapping at
  2π. The button switches the light, the background, the ground color and the
  helpers.

### Dynamic instancing

- **Setup.** The HSL base colors are kept as `getHex()` values, since the
  tweens restart from those.
- **Per frame.** Each frame streams the 10,000 matrices with the new heights,
  in Float32 as the original decomposes them. The camera's `up.x` changes
  after `lookAt`, so it takes effect on the next frame.
- **Tweens.** `setInterval( startTween, 3000 )` runs a 2 s Sinusoidal.In tween.
  Instance colors change only while it runs. Its completion resets `t` and
  advances the color pair.

### Depth texture

- **Post pass.** `post-frag` is ported: 1 − the depth linearized between the
  near and far planes. WebGPU's window depth has the same hyperbolic form as
  `gl_FragCoord.z`. The raw value is written, as the ShaderMaterial has no
  color-space conversion.
- **Controls.** The damped controls update after the render, as in
  `animate()`. Every control change creates a new target, as
  `setupRenderTarget()` does.
- **Format and type.** The depth format and type do not change the image.
- **Samples.** WebGPU multisamples at four samples, so any nonzero sample
  count uses four. WebGL resolves depth to one implementation-chosen sample;
  the port reads sample 3, the closest on the reference GPU.

### Render to texture

- **Scenes.** The render target, the orthographic camera and the planes use
  the page size at load, as the original does.
- **Shaders.** `fragment_shader_pass_1` writes (r, g, time) raw into the linear
  target. The screen quad and the spheres encode the texture on output.
- **Orientation.** Render targets are stored top row first, so the texture is
  sampled at (u, 1 − v), matching WebGL's bottom-up render target.
- **Drawing.** Without autoClear, the screen quad and the spheres share one
  clear.
- **Steps.** The time uniform's bounce and the camera easing step at 60 fps.
  The tori follow `Date.now()` on the example clock.
- **Resize.** The original has no resize handler, so its canvas keeps the
  load-time size. The port follows the gallery's resize, and this case is not
  compared after a resize.

### Normal-map composer

- **Material.** Phong with the sRGB color and specular maps and the
  tangent-space normal map (derivative-based TBN), on the glTF geometry.
- **Composer.** The composer runs the scene into a half-float target, then:
  - BleachBypass (opacity 0.2, three's luminance weights);
  - ColorCorrection (pow 1.4, 1.45, 1.45; mul 1.1);
  - OutputPass (no tone mapping, the sRGB transfer);
  - FXAA to the canvas, using the port of FXAANode, the same algorithm as
    FXAAShader.
- **Setup.** The page renders at a pixel ratio of 1, as the original sets
  none. The material is replaced only when a control changes it.

Stats and lil-gui appearance are not reproduced. OrbitControls touch input is
not separately verified.

## Comparison

`reference/three-js/shadow-rtt.html` executes the pinned sources on the example
clock.

- **Coverage.** Every control, the light button, orbit input, mouse moves and
  resize (except `webgl_rtt`), at DPR 1 and 2.
- **Clock.** TWEEN and `setInterval` run on the example clock. Per-frame easing
  and the time uniform's bounce step at 60 fps.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Shadow meshes (no MSAA) | 0% / 0.00 | 0% / 0.00 |
| Dynamic instancing, MSAA off / on | 0.052% / 0.12, 5.053% / 0.62 | 0.036% / 0.08, 2.629% / 0.34 |
| Depth texture (no MSAA; with the samples control) | 0.298% / 0.63 | 0.149% / 0.31 |
| Render to texture (no MSAA) | 0% / 0.00 | 0% / 0.00 |
| Normal-map composer (no MSAA) | 0.010% / 0.03 | 0.005% / 0.03 |

Two scoped bounds apply:

- **Dynamic instancing with 4× MSAA.** Only the resolved edges of the 10,000
  boxes differ. This case is bounded at 6% / 0.8.
- **Depth texture with the samples control.** The resolved-depth sample
  differs along the knots' edges. This case is bounded at 0.5% / 0.8.

Without the samples control the depth texture matches at 0% / 0.05.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, except the depth
  post pass. The port draws it with a fullscreen triangle; WebGL uses a
  6-index quad.
- **Instance uploads.** Each frame streams the instances, as the original
  does. The port uploads each instance's matrix and color together (80 bytes)
  as resident draw data, 800,000 bytes a frame. WebGL streams the 640,000
  bytes of matrices, and the colors only during a tween.
- **Other uploads.** No other scene uploads geometry or texture data after a
  warm pass.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create
  no GPU resources, except the depth texture's controls. Those create a new
  target, as the original does.
- **Bind groups.** A draw slot now keeps its four most recent bind groups, so
  toggling the normal map reuses them.

No GPU timing parity is claimed. Full measurements are in
[shadow-rtt-comparison.json](shadow-rtt-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js shadow-rtt.spec.js
SHADOW_DPR=2 npx playwright test -c playwright.gallery.config.js shadow-rtt.spec.js
```
