# Sky, sun light, pointer-lock walk, video panorama and ocean

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/sky_water.rs`, with SkyMesh in
`src/browser/sky_water/sky.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgpu_sky` | 263 | SkyMesh with clouds, a per-frame six-face CubeCamera capture reflected by a sphere, the eleven sky controls, the orbit |
| `webgpu_lights_sunlight` | 264 | The cascaded SunLight over instanced posts and towers, the sky's PMREM environment, fog, FirstPersonControls, the sun and shadow controls |
| `misc_controls_pointerlock` | 265 | PointerLockControls with look, walk, gravity and jumps, landing on 500 boxes by a downward ray, the jittered floor |
| `webgpu_video_panorama` | 266 | The looping video on an inside-out sphere, uploaded per new frame, with drag-to-look |
| `webgpu_ocean` | 267 | WaterMesh with its half-resolution reflector, SkyMesh, the sky's PMREM environment on a mirror-smooth cube, bloom, the ten controls |

Their WebGL counterparts — `webgl_shaders_sky`, `webgl_lights_sunlight`,
`webgl_video_panorama_equirectangular` and `webgl_shaders_ocean` — are
excluded as equivalents of these WebGPU ports.
`misc_controls_pointerlock` has no WebGPU equivalent and is compared against
the original WebGL renderer.

## Port notes

### SkyMesh

- **Shader.** The Preetham scattering and r186's cloud layer are ported
  to WGSL:
  - the sinless gradient hash;
  - quintic gradient noise and four-octave fbm;
  - coverage, Beer-powder shading and silver lining;
  - the aerial composite.
- **Evaluation.** The vertex node's varyings depend only on uniforms, so
  the fragment evaluates them with the same f32 arithmetic.
- **Depth.** Depth is pinned to the far plane.
- **Time.** TSL `time` drives the cloud drift; the reference fixture runs its
  node clock on the example clock.
- **Culling.** SkyMesh keeps Three.js's frustum culling.

### Sky and cube reflection

- **Capture.** Each frame the sphere is hidden while the sky renders into a
  256² half-float cube.
- **Sampling.** The sphere samples the reflected view ray from that cube, x
  flipped as CubeTextureNode samples it.
- **Output.** ACES tone mapping and the exposure control follow the
  original.

### Sun light

- **Updates.** `updateSun()` is ported:
  - the light's spherical position, and its color and intensity between
    dusk and day;
  - the fog color;
  - a new PMREM environment from a scene holding only the sky, applied at
    `environmentIntensity` 0.5.
- **Towers.** Their heights, offsets and colors come from
  `MathUtils.seededRandom( 6 )` (mulberry32).
- **Controls.** The existing FirstPersonControls port steps with the timer's
  delta.
- **Not ported.** The cascade tint of `show cascades`.

### Pointer-lock walk

- **Setup.** The floor is built as in the original:
  - jittered in Float32;
  - made non-indexed;
  - colored per vertex by `setHSL( …, SRGBColorSpace )`.
- **Movement.** Each box and the walk physics follow `animate()`:
  - damping, gravity and jumps;
  - `moveRight` and `moveForward` from the camera matrix;
  - the downward raycast (an explicit CPU query) for landing.
- **Look.** The YXZ Euler look is clamped to ±π/2.
- **Locking.** The gallery requests pointer lock from its blocker. The suite
  locks both sides directly and sends movement deltas.

### Video panorama

- **Uploads.** VideoTexture's behavior is ported:
  - a new frame is copied with `copyExternalImageToTexture` (flipY, sRGB);
  - only when the video presents one;
  - linear filtering and no mipmaps.
- **Tests.** The suite pauses and seeks both videos to the same frames before
  capturing.

### Ocean

- **Water.** WaterMesh's color node is ported:
  - four normal-map samples drifting with `time`;
  - the reflected sun, and diffuse and scattering terms;
  - the distorted reflector sample and Schlick reflectance.
- **Normal scaling.** TSL's `mul( 1.5, 1.0, 1.5 )` multiplies by each scalar
  in turn, so the normal is `normalize( noise.xzy × 2.25 )`.
- **Normal map.** The normal map is in NoColorSpace.
- **Reflector.** The reflector renders at `round( size × 0.5 )` through
  ReflectorNode's oblique virtual camera, with the water hidden.
- **Bloom.** Bloom (threshold 0) is added to the scene pass.
- **Transient frame.** The original's `updateSun()` moves the sky through the
  PMREM scene, so its first frame after a sun change reflects no sky. The
  suite captures the settled frames.

Inspector and lil-gui appearance are not reproduced. OrbitControls and
FirstPersonControls touch input is not separately verified.

## Comparison

`reference/three-js/sky-water.html` executes the pinned sources on the
example clock, with the node clock set to it.

- **Coverage.** Every control, first-person walking and looking,
  pointer-lock moves and jumps, video seeks, orbit input and resize, at DPR 1
  and 2.
- **Timer.** FirstPersonControls step by the timer's delta, so the sun-light
  captures only move forward in time.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Sky (no MSAA) | 0% / 0.40 | 0% / 0.40 |
| Sun light, MSAA off / on | 0% / 0.30, 0% / 0.30 | 0% / 0.30, 0% / 0.30 |
| Pointer-lock walk, MSAA off / on | 0% / 0.00, 1.054% / 0.14 | 0% / 0.00, 0.439% / 0.05 |
| Video panorama, MSAA off / on | 0% / 0.01, 0% / 0.01 | 0% / 0.01, 0% / 0.01 |
| Ocean (no MSAA) | 0% / 0.31 | 0.001% / 0.29 |

All five scenes match within the ordinary threshold with MSAA off. With 4×
MSAA only the pointer-lock boxes' silhouettes differ, bounded at 2% / 0.4.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the sky's six cube faces per frame;
  - the reflector pass;
  - the sun light's two cascades;
  - the culled boxes of the walk.
- **Uploads.** The video's frames are copied from the element, as
  VideoTexture does. No scene uploads geometry or texture data by writes
  after a warm pass.
- **Warmed cycles.** Warmed cycles of time and input create no GPU resources.
  The sun-light and ocean controls are left out: moving the sun builds a new
  PMREM environment, as in the original.

No GPU timing parity is claimed. Full measurements are in
[sky-water-comparison.json](sky-water-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js sky-water.spec.js
SKY_DPR=2 npx playwright test -c playwright.gallery.config.js sky-water.spec.js
```
