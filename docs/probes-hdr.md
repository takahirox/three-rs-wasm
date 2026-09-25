# Cube-camera probe, HDR environment maps, UltraHDR, transmission and the dungeon

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/probes_hdr.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgpu_lightprobe_cubecamera` | 283 | The pisa background, the probe from a CubeCamera capture, LightProbeHelper, the orbit, on-demand rendering |
| `webgl_materials_envmaps_hdr` | 284 | The generated, LDR and HDR environments and backgrounds, the debug PMREM plane, roughness, metalness and exposure, the orbit |
| `webgl_loader_texture_ultrahdr` | 285 | The UltraHDR environment and background, the metallic torus knot, auto-rotation, the resolution switch, the orbit |
| `webgpu_materials_transmission` | 286 | The UltraHDR background and environment, the double-sided transmissive sphere with its alpha stripes, all twelve controls |
| `webgpu_performance` | 287 | The UltraHDR environment, the 798-mesh dungeon glTF, the `static` switch, the orbit |

`webgl_lightprobe_cubecamera` and `webgl_performance` are excluded as
equivalents of these WebGPU ports. The HDR environment-map and UltraHDR
examples have no WebGPU equivalent and are compared against the WebGL
renderer.

## Port notes

### Cube-camera probe

- **Capture.** The CubeCamera renders the pisa background into a 256² 8-bit
  NoColorSpace cube. `fromCubeRenderTarget` then reads it back.
- **SH.** The capture's texels are the faces' texels (same layout, 256²),
  stored as 8-bit linear values. The port projects those values onto the SH
  basis once on the CPU, an explicit one-time query, with LightProbeGenerator's
  weights.
- **Helper.** LightProbeHelper (size 5) shows the irradiance. The page renders
  on change.

### HDR environment maps

- **Environments.** Three PMREMs are prepared on the GPU:
  - DebugEnvironment (room, point light, emissive panels) through `fromScene`;
  - the LDR pisa cube;
  - the HDR pisa cube.
- **Backgrounds.** The LDR and HDR cubes are drawn on WebGL's camera-centered
  unit box. The generated one uses the PMREM itself.
- **Tone mapping.** WebGLBackground tone-maps a cube background unless it is
  sRGB, so the LDR box sets `tone_mapped = false`, a material property added
  for this.
- **Debug plane.** It maps the chosen atlas in three's texel layout. Each of
  the port's face regions is mirrored and ±Y exchanged.
- **Motion.** The torus turns 0.005 a frame, stepped at 60 fps.

### UltraHDR

- **Parsing.** UltraHDRLoader's parse is ported: the MPF image offsets and the
  gain map's `hdrgm:` XMP.
- **Decoding.** Both JPEGs are decoded by the browser and drawn to a 2D canvas.
  The gain map is scaled to the primary's size there, as `drawImage` does in
  the original.
- **Recovery.** The recovery formula follows the loader: the 1.8 display boost,
  the truncating sRGB table and `DataUtils.toHalfFloat`.
- **Environment.** The result is the equirectangular environment and
  background.
- **Controls.**
  - A resolution change loads the other file asynchronously and replaces the
    environment once decoded, as `loadEnvironment()` does.
  - FloatType is kept at half precision.

### Transmission

- **Material.** MeshPhysicalMaterial with transmission, IOR, thickness,
  specular and double-sided transparency.
- **Alpha map.** The 2 × 2 alpha map is a white map with the same alpha,
  repeated (1, 3.5) with nearest magnification.
- **Environment.** `envMap` is the scene environment of this single mesh;
  `envMapIntensity` scales it.

### Dungeon

- **Load.** The glTF (WebP textures) loads with the UltraHDR environment.
- **Static.** The `static` switch marks the meshes static, which does not
  change the image.

Inspector, Stats and lil-gui appearance are not reproduced. OrbitControls touch
input is not separately verified.

## Comparison

`reference/three-js/probes-hdr.html` executes the pinned sources on the example
clock.

- **Coverage.** Every compared control, orbit input and resize, at DPR 1 and 2.
- **Loading.** The fixture waits for the loads that finish after their file:
  gain-map decoding, the probe capture and compileAsync.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Cube-camera probe, MSAA off / on | 0% / 0.01, 0% / 0.01 | 0% / 0.01, 0% / 0.01 |
| HDR environment maps (no MSAA) | 0.238% / 1.74 | 0.074% / 1.74 |
| UltraHDR, MSAA off / on | 4.069% / 1.24, 4.256% / 1.30 | 0.424% / 0.15, 0.535% / 0.16 |
| Transmission, MSAA off / on | 4.983% / 1.28, 4.989% / 1.28 | 4.961% / 1.28, 4.959% / 1.28 |
| Dungeon, MSAA off / on | 0.010% / 0.07, 0.272% / 0.10 | 0.011% / 0.07, 0.142% / 0.08 |

Three scoped bounds apply:

- **Generated environment background.** The DebugEnvironment capture is about
  1% brighter, a mean error of 1.7 with no pixel over the threshold. The HDR
  and LDR states match within the ordinary threshold. This case is bounded at
  0.5% / 1.8.
- **UltraHDR background.** WebGL converts the equirectangular background to a
  mipmapped cube and samples it trilinearly; the port samples its base level.
  Minified ground texture differs after the camera turns. This case is bounded
  at 5% / 1.4.
- **Transmission.** The double-sided sphere's inner faces, seen through the
  front's transparent stripes, are shaded slightly differently. This case is
  bounded at 5% / 1.3.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including the
  798 dungeon meshes and the background box or sphere. The only exception is
  equirectangular backgrounds. The port draws them with a fullscreen triangle,
  where WebGL uses a 36-index box and WebGPU a 5,952-index sphere.
- **Environments.** All environments are prefiltered once. Switching between
  prefiltered maps rebinds them without filtering, and the environment filter
  counter now counts only real filtering.
- **Uploads.** No scene uploads geometry or texture data by writes after a
  warm pass.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create no
  GPU resources.

No GPU timing parity is claimed. Full measurements are in
[probes-hdr-comparison.json](probes-hdr-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js probes-hdr.spec.js
PROBES_DPR=2 npx playwright test -c playwright.gallery.config.js probes-hdr.spec.js
```
