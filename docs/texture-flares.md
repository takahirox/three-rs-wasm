# Canvas raycast textures, 3D partial updates, cube mips, lens flares and the car

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/texture_flares.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_raycaster_texture` | 298 | The canvas-drawn grid and cross shared by three textures, the pointer raycast and `transformUv`, all seven circle-texture controls |
| `webgl_texture3d_partialupdate` | 299 | The 128³ cloud texture filled block by block every 1.5 s, the jittered raymarch, the gradient sky, the orbit, all four controls |
| `webgl_materials_cubemap_render_to_mipmaps` | 300 | The cube render target's levels 0 to 8 drawn per face and tinted per level, the two environment-mapped spheres, the polar-limited orbit |
| `webgpu_lensflares` | 301 | 3,000 Phong boxes, three point lights with LensflareMesh occlusion tests and elements, fog, FlyControls |
| `webgl_materials_car` | 302 | The Draco Ferrari with clearcoat, metal and transmissive glass, the HDR environment, the moving grid and wheels, the multiplied AO plane, the orbit, the three color inputs |

`webgl_lensflares` is excluded as an equivalent of the WebGPU port. The other
four have no WebGPU equivalent and are compared against the WebGL renderer.

## Port notes

### Canvas raycast textures

- **Canvas.** The page draws the grid image and the yellow cross with the
  browser's Canvas 2D, as the original does, using the same cross sizes.
- **Textures.** Each redraw copies the canvas on the GPU into the three
  textures in use and regenerates their mipmaps. This matches texImage2D with
  `flipY`.
- **Raycast.** A pointer hit applies the hit texture's `transformUv`:
  - its UV matrix;
  - its wrapping;
  - its `flipY`.
- **Wrapping.** The circle's wrap modes select one of nine programs that share
  its texture. Each program binds the matching sampler. The original
  re-uploads the unchanged image on these changes; the port does not.

### 3D partial updates

- **Cloud blocks.** Every 1.5 s the next 30³ block is generated on the CPU
  with ImprovedNoise, as `generateCloudTexture` does, then written into its
  cell. The bytes store ToUint8 of the value, as the Uint8Array does.
- **Raymarch.** The fragment shader is ported: the box bounds, the steps, the
  shading and the early exit.
  - **Jitter.** The start jitter hashes `gl_FragCoord` with the frame count.
    Rows count from the bottom, as in GL.
  - **Output.** Its sRGB encoding is the target's.
- **Sky.** The browser draws the 1 × 32 gradient.

### Cube mips

- **Capture.** `renderToCubeTexture` renders each face of levels 0 to 8 with
  the source cube sampled at its implicit level and tinted per level. The port
  draws each face texel's direction with a fullscreen triangle, which gives the
  same per-pixel directions and derivatives as the CubeCamera's box.
- **Spheres.** MeshBasicMaterial's `envMap` reflection vector is computed per
  vertex, as WebGL's basic material does. The CubeTexture is sampled with x
  flipped; the render target is not.

### Lens flares

- **Occlusion test.** Each visible LensflareMesh follows the WebGPU renderer's
  order:
  1. copy the 16 × 16 framebuffer patch under the light;
  2. draw the magenta probe with the depth test;
  3. copy the occlusion map;
  4. restore the patch.
- **Elements.** The additive elements read nine occlusion texels in their
  vertex shader, sampling their upright textures.
- **Light color.** `element.color.convertSRGBToLinear()` converts the light's
  own color in place when its flare first draws. The light is dimmer from the
  next frame.
- **Invisible quad.** The mesh's own transparent quad is still drawn, as in the
  original. It does not write depth here, because the original draws it after
  the probe.
- **FlyControls.** The pointer, buttons and keys are ported with movement
  speed 2,500 and roll speed π/6.

### Car

- **Materials.** The named meshes get the example's materials; the named wheel
  nodes turn with `performance.now()`.
- **Grid.** The grid moves by JavaScript's remainder. GridHelper's material is
  not tone mapped.
- **AO plane.** The plane multiplies with premultiplied alpha.
- **Color inputs.** The three color inputs are the controls.

Inspector, Stats and lil-gui appearance are not reproduced. OrbitControls touch
input is not separately verified.

## Comparison

`reference/three-js/texture-flares.html` executes the pinned sources on the
example clock.

- **Coverage.** Every control, pointer moves over the three meshes, fly input,
  orbit input and resize, at DPR 1 and 2.
- **Car inputs.** The fixture drives the car's color inputs with `input`
  events.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Canvas raycast textures (no MSAA) | 0% / 0.08 | 0% / 0.06 |
| 3D partial updates (no MSAA) | 0.033% / 0.01 | 0.033% / 0.01 |
| Cube mips, MSAA off / on | 0.280% / 0.09, 0.468% / 0.12 | 0.161% / 0.06, 0.288% / 0.08 |
| Lens flares (no MSAA) | 0% / 0.26 | 0% / 0.26 |
| Car, MSAA off / on | 0.414% / 0.20, 4.799% / 0.79 | 0.246% / 0.17, 2.556% / 0.47 |

The car's MSAA edges (grid lines, trim and glass outlines) are bounded at
5% / 0.8. With MSAA off it matches within the ordinary threshold.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the three canvas meshes;
  - the sky and the cloud box;
  - the two spheres;
  - 174 lens-flare frame draws, with every probe, restore and element quad;
  - the car's 103 draws.
- **Uploads.**
  - Canvas redraws are copied on the GPU when the pointer moves, as the
    original re-uploads.
  - Cloud blocks are written once each.
  - No scene uploads geometry or texture data by writes after a warm pass.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create no
  GPU resources.

No GPU timing parity is claimed. Full measurements are in
[texture-flares-comparison.json](texture-flares-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js texture-flares.spec.js
FLARES_DPR=2 npx playwright test -c playwright.gallery.config.js texture-flares.spec.js
```
