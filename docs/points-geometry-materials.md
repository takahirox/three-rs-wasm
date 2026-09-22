# Twenty point, geometry and material examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
These additions use the existing Rust/Wasm renderer and GPU TSL graphs.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_points_billboards` | 163 | 10,000 camera-facing points, pointer camera |
| `webgl_points_sprites` | 164 | Five groups of 10,000 snowflakes sharing positions, texture toggle |
| `webgl_points_waves` | 165 | 2,500 procedurally animated points, GPU size and position |
| `webgl_custom_attributes_points` | 166 | 100,000 points, GPU size/color animation |
| `webgl_custom_attributes_points3` | 167 | Original sphere and box-frame points, GPU animation |
| `webgl_buffergeometry_attributes_none` | 168 | 10,000 procedurally generated triangles |
| `webgl_buffergeometry_attributes_integer` | 169 | 10,000 triangles, integer-selected texture |
| `webgl_buffergeometry_instancing` | 170 | 50,000 rotating triangles, draw-count control |
| `webgl_buffergeometry_instancing_interleaved` | 171 | 5,000 cubes, shared rotation and resident instance matrices |
| `webgl_materials_modified` | 172 | Two heads, GPU vertex and normal deformation |
| `webgl_materials_wireframe` | 173 | Native and derivative-based wireframes, thickness control |
| `webgl_materials_texture_filters` | 174 | Original paired linear/nearest texture filtering |
| `webgl_geometry_shapes` | 175 | Original filled/extruded shapes, lines, points and drag rotation |
| `webgl_geometry_colors_lookuptable` | 176 | Pressure model, four LUTs and legend |
| `webgl_buffergeometry_uint` | 177 | 500,000 triangles with packed normals/colors |
| `webgpu_materials_basic` | 178 | 500 environment-mapped spheres, reflection/refraction/opacity controls |
| `webgpu_materials_envmaps` | 179 | Cube and panorama environments, independent background/material rotation |
| `webgpu_materials_displacementmap` | 180 | Ninja head, GPU displacement, normal/AO maps, moving lights and environment |
| `webgl_materials_bumpmap` | 181 | Height-map derivatives, Phong lighting, spot shadow, enable/scale controls |
| `webgl_materials_blending` | 182 | Five textures × five blend modes, labels and animated background |

The environment and displacement scenes have official WebGPU equivalents;
only those variants are listed. The other WebGL scenes have no equivalent
WebGPU scene in the pinned inventory. Diagnostic backend conversions below
are not presented as additional official examples.

## Core changes and scope

A node can limit the active prefix of its resident instances without mutating
the geometry. Main rendering, shadows and ray queries honor the count, and
counts beyond capacity are rejected. This fixes geometry re-uploads when the
instancing GUI changes its draw count. GPU regression tests cover hiding,
restoring, ray hits and resource residency.

AO now attenuates ambient, hemisphere and light-map illumination as well as IBL.
Direct lighting and emission remain unoccluded. A GPU regression test checks
these distinctions. Panorama conversion can select an sRGB byte cube target,
while the existing HDR entry point retains its float target. The environment
example uses the original one-mip converted panorama and explicit background
LOD zero; allowing implicit background LOD had introduced visible blur.

Static attributes remain on the GPU. Points use shared instanced quads rather
than copies of every particle attribute. No CPU per-vertex animation is added.
The integer triangle example samples only its selected texture; the interleaved
cube example combines resident matrices with one animated quaternion on the GPU.
The 500,000-triangle scene uses 24 MB of packed attributes. Controls change
uniforms/material state or draw count, not regenerated geometry. Disabling bump
mapping removes the bump program and its texture samples; re-enabling reuses it.
Sphere object transforms in example 178 remain CPU-updated as in the original.

Fixed frame/shape/pressure/OBJ geometry and LUT data are prepared from the pinned
source by `tools/gallery/prepare-*.mjs`. This is not a general OBJLoader,
Shape/Extrude or InterleavedBuffer API implementation. Asset manifests retain
source hashes, and licenses are listed in `web/THIRD_PARTY.md`. The original
filter scene does not resize; the port's responsive resize is explicitly added
to its comparison fixture. Stats/Inspector styling is not reproduced.

## Comparison and performance evidence

The four new browser suites execute the pinned source with deterministic time
and random seed. They compare animation, every exposed control, pointer/orbit
interaction and resize at DPR 1 and 2. GPU instrumentation checks original draw
workload, packed buffer sizes, stable resources and no steady-state geometry or
texture uploads. Shadow draws are counted for the bump example. Backgrounds
(box versus sphere) and presentation triangles are excluded from the environment
scene's mesh workload comparison. Dynamic object transform uploads are recorded
separately from geometry. No GPU frame-time parity claim is made.

Ordinary limits are 0.5% pixels with an RGB difference above 6/255, and mean RGB
error 0.6/255. The following WebGL-only allowances are local to each native
comparison, and require additional diagnostics passing the ordinary limits:

| Case | Native differing-pixel limit | Mean error limit | Required diagnostic |
| --- | ---: | ---: | --- |
| Snow sprites | 2% | 1/255 | Same point shader using instanced quads |
| Attribute-free/integer triangles, MSAA | 16% | 5/255 | MSAA off and same geometry on official WebGPU |
| Modified heads, MSAA | 0.6% | 0.6/255 | MSAA off |
| Wireframes, MSAA | 12% | 6/255 | MSAA off and official WebGPU |
| Shapes, MSAA | 3.5% | 1/255 | MSAA off and official WebGPU |
| Packed 500,000 triangles, MSAA | 36% | 6.5/255 | MSAA off and official WebGPU |
| Bump map | 8% | 1.5/255 | Same scene with official WebGPU, MSAA off/on |

Dense overlapping triangles amplify WebGL/WebGPU MSAA coverage and resolve
variation. The large native differing-pixel percentages are retained in the
report, not called pixel parity. For bump mapping, the strongest tested height
scale amplifies backend derivative/filtering differences even without MSAA;
the WebGPU diagnostic preserves geometry, lighting, shadow and material inputs.
It establishes a backend-dependent difference, not a proof of the individual
contribution of derivative precision versus mip filtering. These allowances do
not lower object counts, resolution, behavior or performance requirements.

`points-geometry-materials-comparison.json` contains final per-state measurements
and resource/workload evidence. Run after preparing upstream assets and building
release Wasm:

```sh
npx playwright test -c playwright.gallery.config.js point-clouds.spec.js shader-geometry.spec.js geometry-materials.spec.js environment-materials.spec.js
CLOUDS_DPR=2 GEOMETRY_DPR=2 MATERIALS_DPR=2 ENVIRONMENT_DPR=2 npx playwright test -c playwright.gallery.config.js point-clouds.spec.js shader-geometry.spec.js geometry-materials.spec.js environment-materials.spec.js
```

Select any entry in `/web/gallery/` for interactive inspection. Reference
fixtures use `/reference/three-js/{point-clouds,shader-geometry,geometry-materials,environment-materials}.html?id=EXAMPLE_ID`.

Final image coverage: 37 comparison cases at each DPR, totaling 596 saved states.
All 20 scenes also pass warmed GUI/time/resize residency checks, with four
full-workload comparison suites. The active-count case was rerun after its Core
fix. Existing shapes, material-texture and glTF image/resource regressions pass.
Native PBR, lighting, instancing and TSL surface tests, release Wasm, native/Wasm
Clippy (`-D warnings`), formatting and inventory/asset hash checks pass.
The packaged Pages layout passes 21 checks: gallery entry plus all 20 new scenes.
