# Clearcoat, fly controls, point-light shadows, light probe and tone mapping

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/lights_probes.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgpu_clearcoat` | 268 | Four clearcoat spheres (carbon, golf-ball, flakes and scratched-gold normal maps), the HDR cube environment and background, the moving point light, the orbit |
| `misc_controls_fly` | 269 | The planet with its clouds and moon, 20 star Points, FogExp2, FlyControls by pointer and keys, the film grain pass |
| `webgpu_shadowmap_pointlight` | 270 | Two moving point lights with PCF cube shadows, their alpha-tested spheres, the back-sided room, the orbit |
| `webgpu_lightprobe` | 271 | LightProbeGenerator.fromCubeTexture, the probe-lit sphere with its envMap, LightProbeHelper, the three intensity controls, the orbit |
| `webgpu_tonemapping` | 272 | The seven tone mappings, exposure, background blurriness and intensity, the Draco venice mask, the damped orbit |

Their WebGL counterparts are excluded as equivalents of these WebGPU ports:

- `webgl_materials_physical_clearcoat`;
- `webgl_shadowmap_pointlight`;
- `webgl_lightprobe`;
- `webgl_tonemapping`.

In r186, `misc_controls_fly` itself renders with WebGPURenderer.

## Port notes

### Clearcoat

- **Environment.** HDRCubeTextureLoader's RGBE faces become one resident
  half-float cube. Its PMREM lights the spheres. The background samples the
  cube itself, as `scene.background` does.
- **Flakes.** FlakesTexture is drawn on a 512² canvas from a seeded stream.
- **Materials.** The spheres use WebGPU's PhysicalLightingModel, with specular
  energy conservation.

### Fly controls

- **Controls.** FlyControls' move state follows pointer offset and keys. Each
  step moves the camera by a third of the distance to the nearer surface.
- **Stars.** r186's WebGPU Points draw PointsMaterial as native one-pixel
  points (`point-list`), whatever its size. The port does the same.
- **Film.** FilmNode's grain uses TSL's `rand` on the example clock.

### Point-light shadows

- **Filtering.** r186's `getPointShadow` PCF is ported:
  - five Vogel-disk taps rotated by interleaved gradient noise;
  - offsets in the tangent frame of the light-to-fragment direction,
    `radius / mapSize` apart;
  - each tap looked up in its own cube face against the fragment's face depth.

  The shared point-shadow shader now uses this filter in place of offsets in
  one face's UV.
- **Alpha map.** The 2 × 2 canvas alpha map is a white map with that alpha.
  With `alphaTest` 0.5 it discards the same fragments in the view and in the
  shadow maps.

### Light probe

- **Generation.** CubeTextureLoader marks the pisa faces sRGB.
  `fromCubeTexture` is ported on the CPU (an explicit one-time query): it
  decodes each texel to linear and projects it on the nine SH bases, with
  solid-angle weights.
- **Lighting.** The sphere's irradiance adds `getShIrradianceAt( normal )` ×
  intensity. LightProbeHelper draws the unscaled coefficients.
- **Background.** The raw cube, as a camera-centered 32 × 32 sphere at the far
  plane, as r186 draws `scene.background`.
- **Intensity refresh.** r186's NodeMaterialObserver does not track light
  intensities. The original therefore shows a light-probe or directional
  intensity change only at the mesh's next refresh. The port applies such
  changes at once. The reference fixture forces the refresh by disposing the
  materials after each parameter change.

### Tone mapping

- **Tone mappings.** None, Linear, Reinhard, Cineon, ACESFilmic, AgX and
  Neutral. Cineon and AgX were added to the output shader.
- **Background.** The equirectangular HDR is blurred and scaled by the
  controls.
- **Controls.** The damped orbit steps at 60 fps of example time.

Inspector and lil-gui appearance are not reproduced. OrbitControls and
FlyControls touch input is not separately verified.

## Comparison

`reference/three-js/lights-probes.html` executes the pinned sources on the
example clock.

- **Coverage.** Every control, fly pointer and keys, orbit drag and wheel, and
  resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Clearcoat, MSAA off / on | 0.064% / 0.31, 0.064% / 0.31 | 0.194% / 0.32, 0.194% / 0.32 |
| Fly controls, MSAA off / on | 0.518% / 0.23, 0% / 0.04 | 0.236% / 0.11, 0% / 0.04 |
| Point-light shadows, MSAA off / on | 0.005% / 0.05, 0.005% / 0.05 | 0.006% / 0.05, 0.006% / 0.05 |
| Light probe, MSAA off / on | 0% / 0.01, 0% / 0.01 | 0% / 0.01, 0% / 0.01 |
| Tone mapping, MSAA off / on | 0.390% / 0.49, 0.587% / 0.52 | 0.263% / 0.47, 0.322% / 0.48 |

Two scoped bounds apply:

- **Fly controls without MSAA.** The rim of the 6,371-unit planet's cloud
  layer is rasterized one pixel apart along parts of its silhouette. This case
  is bounded at 0.8% / 0.6. With MSAA it matches exactly.
- **4× MSAA.** Resolve differences are bounded at 2% / 0.4, and at 2% / 0.6
  for tone mapping.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - the background spheres;
  - the four clearcoat spheres;
  - the 20 native star Points;
  - the point lights' per-face culled shadow casters.

  The only exception is the tone-mapping background. The port draws it with a
  fullscreen triangle, while r186 uses a 5,952-index sphere.
- **Uploads.** No scene uploads geometry or texture data by writes after a
  warm pass.
- **Warmed cycles.** Warmed cycles of time, input and resize create no GPU
  resources.

No GPU timing parity is claimed. Full measurements are in
[lights-probes-comparison.json](lights-probes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js lights-probes.spec.js
LIGHTS_DPR=2 npx playwright test -c playwright.gallery.config.js lights-probes.spec.js
```
