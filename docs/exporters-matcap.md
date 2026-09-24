# STL, PLY and OBJ exporters, the matcap head and physically based lights

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/exporters_matcap.rs`, with the exporters in
`src/browser/exporters_matcap/formats.rs`. The EXR decoder is in
`src/browser/refraction_loaders/formats.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `misc_exporter_stl` | 258 | The box on a fogged, shadowed ground with a grid; ASCII and binary STL downloads |
| `misc_exporter_ply` | 259 | The vertex-colored box; ASCII, big-endian and little-endian PLY downloads |
| `misc_exporter_obj` | 260 | Six selectable scenes (triangle, cube, cylinder, multiple and rotated objects, point cloud); OBJ downloads |
| `webgpu_materials_matcap` | 261 | The normal-mapped head with the EXR matcap, ACES tone mapping, the color and exposure controls |
| `webgpu_lights_physical` | 262 | A photometric bulb and hemisphere light, bump-, roughness- and metalness-mapped Standard materials, point-light shadows, Reinhard exposure |

`webgpu_materials_matcap` and `webgpu_lights_physical` supersede their WebGL
equivalents, which are excluded. The exporters have no WebGPU equivalents and
are compared against the original WebGL renderer.

## Port notes

### Exporters

- **Ports.** STLExporter, PLYExporter and OBJExporter are ported for
  meshes and points:
  - world-space vertices by `applyMatrix4`;
  - normals by Three.js's `Matrix3.getNormalMatrix` arithmetic and
    `normalize` (a multiply by `1 / length`), including STLExporter's double
    normalization;
  - sRGB-converted vertex colors;
  - OBJ's `v/vt/vn` face indices and point-cloud colors.
- **Text output.** Numbers in the text formats use JavaScript's
  `Number.prototype.toString`:
  - the fewest round-tripping digits;
  - ties between 17-digit candidates go to the even digit, as for the
    cylinder's `10.395584106445312`;
  - decimal form from 1e-6 to 1e21, exponent form outside it.
- **Downloads.** An export button stores the file in the demo;
  `app.gallery_take_export()` hands the page the name and bytes, which it
  downloads as the example's anchor does.
- **Equality.** Every exported file equals the original's byte for byte. The
  suite exports each STL and PLY variant, and an OBJ of every selectable
  scene.

### Matcap

- **EXRLoader.** `040full.exr` uses ZIP compression and FLOAT channels. The
  decoder now handles:
  - zlib inflate, the byte predictor and `interleaveScalar`;
  - FLOAT samples converted by `DataUtils.toHalfFloat`'s base and shift
    tables, which truncate rather than round;
  - ZIPS blocks.
- **Material.** MeshMatcapNodeMaterial is a Lambert surface whose output graph
  samples the resident half-float matcap:
  - at `matcapUV`, from the resolved view normal, which includes the normal
    map;
  - multiplied by the color uniform.
- **Normal map.** The TextureLoader normal map is flipped on load, as in the
  original.
- **Not ported.** Drag-and-drop matcap replacement.

### Physical lights

- **Lights.** Photometric units follow the example:
  - `PointLight.power` becomes intensity / 4π;
  - the bulb's emissive intensity is that intensity / 0.02²;
  - the hemisphere intensity is the selected irradiance;
  - exposure is `exposure⁵`.
- **Materials.** Standard materials use surface graphs:
  - maps with their repeats (the floor's 10 × 24) and anisotropy 4;
  - bump maps through the shared `bump_map` derivative;
  - `roughness × roughnessMap.g` and `metalness × metalnessMap.b`.
- **Energy.** WebGPU's PhysicalLightingModel conserves specular energy in both
  direct and indirect diffuse light, so the materials use the renderer's
  energy-conserving path. Without it, the grazing floor under the
  hemisphere light was 10/255 too bright.
- **Shadows.** The shadows toggle switches the bulb's shadow casting.

Inspector appearance is not reproduced. OrbitControls touch input is not
separately verified.

## Comparison

`reference/three-js/exporters-matcap.html` executes the pinned sources on the
example clock.

- **Stubs.** The Inspector and GUI are stubbed.
- **Downloads.** The fixture captures each download's Blob for the byte
  comparison.
- **Coverage.** Every control, orbit input and resize, at DPR 1 and 2.
  Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| STL scene, MSAA off / on | 0% / 0.00, 1.074% / 0.15 | 0.001% / 0.00, 0.533% / 0.08 |
| PLY scene, MSAA off / on | 0.001% / 0.00, 1.113% / 0.16 | 0.002% / 0.00, 0.554% / 0.08 |
| OBJ scenes, MSAA off / on | 0% / 0.00, 0.044% / 0.01 | 0% / 0.00, 0.027% / 0.00 |
| Matcap, MSAA off / on | 0% / 0.11, 0.301% / 0.25 | 0% / 0.11, 0.152% / 0.18 |
| Physical lights (no MSAA) | 0.079% / 0.11 | 0.070% / 0.12 |

All scenes match within the ordinary threshold with MSAA off. With 4× MSAA,
the differences lie on silhouettes and grid lines. The suite bounds MSAA at 2%
/ 0.4 for the STL, PLY, OBJ and matcap scenes.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including the
  shadow passes.
- **Uploads.** No scene uploads geometry or texture data after a warm pass.
  The EXR decodes once, on the CPU, as EXRLoader does. The exporters run only
  when their buttons are pressed.
- **Warmed cycles.** Warmed cycles of time and input create no GPU resources.
  Controls are left out of those cycles for two scenes:
  - the OBJ scene selection, which builds new geometry as the original does;
  - the physical-lights controls, whose shadow toggle rebuilds the light
    bindings.

No GPU timing parity is claimed. Full measurements are in
[exporters-matcap-comparison.json](exporters-matcap-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js exporters-matcap.spec.js
EXPORTERS_DPR=2 npx playwright test -c playwright.gallery.config.js exporters-matcap.spec.js
```
