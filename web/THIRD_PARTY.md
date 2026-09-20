# Official Three.js cube demo

`crate.gif` is copied unchanged from Three.js r186, commit
`148ef33ecb6d2502ff796d4554abd1549c95d519`, `examples/textures/crate.gif`.
The corresponding source example is:
https://github.com/mrdoob/three.js/blob/148ef33ecb6d2502ff796d4554abd1549c95d519/examples/webgl_geometry_cube.html

Three.js attribution and MIT terms are in ../LICENSE-THREE.

`/web/?example=2` ports the unit BoxGeometry, MeshBasicMaterial with sRGB texture,
70-degree PerspectiveCamera (near 0.1, far 100, z=2), and per-frame Euler rotations
(x += 0.005, y += 0.01) to Rust/Wasm. Click-to-pause is an additional demo control.
The original fullscreen WebGL renderer becomes a 512x512 WebGPU canvas, with
antialiasing and mipmaps disabled to match the current renderer. No claim of
pixel identity with the fullscreen, antialiased original is made.

`?example=2&static=1` uses rotation (0.4, 0.7, 0) for deterministic comparison.
The browser migration test compares this view against pinned Three.js with the
same viewport, sampling and rotation, using the existing fixed image tolerance.

# Point Lights sculpture

`models/WaltHead.obj` is copied unchanged from the same pinned Three.js r186
archive (`examples/models/obj/walt/WaltHead.obj`). Model credit: David OReilly;
displacement effect: oosmoxiecode, as credited by the official example:
https://threejs.org/examples/webgpu_lights_pointlights.html

The port is `/web/?example=3`, implemented in `src/browser/point_lights.rs`.
It retains all 16,160 input triangles, expands them into 64,640 tetrahedron faces,
and reproduces both point-light trajectories, intensities, camera and per-vertex
distance/sine displacement. The original TSL displacement is translated to a GPU WGSL vertex program; MeshPhongNodeMaterial becomes MeshStandardMaterial (roughness 0.4), so the
specular response differs from the official example. Random per-face phases use
a fixed seed for reproducible comparison. UI and orbit controls are adapted.
It renders at 720x720 without MSAA. The official example enables antialiasing;
this port uses the same non-MSAA sampling as its Three.js comparison, since the
two renderers currently produce different MSAA edge resolves. No shadows or post-processing were added.

`reference/three-js/point-lights.html` runs pinned Three.js with the same adapted
material and seed at t=6 seconds. Its image is compared against the Rust static
view using the existing tolerance (6/255, <=0.5% differing pixels), separately
from the unchanged frozen MVP migration manifest. Browser tests also exercise
animation, pause, deformation and orbit controls.

# glTF / GLB viewer

`models/RobotExpressive/RobotExpressive.glb` is copied unchanged from the same
pinned Three.js archive for animation/skinning/morph tests and demos. Model by
Tomás Laulhé (Quaternius), CC0 1.0, modifications by Don McCurdy. The upstream
notice is preserved in `models/RobotExpressive/README.md`.

DamagedHelmet is copied from the pinned Three.js r186 archive. Its original
credit and CC Attribution-NonCommercial notice are preserved in
`models/DamagedHelmet/README.md` (theblueturtle_).

BoomBox is from KhronosGroup/glTF-Sample-Assets, commit
`90d7ede14c7e280af263824604b427a1ca02cb66`,
`Models/BoomBox/glTF-Binary/BoomBox.glb`. Credits: Microsoft; CC0.
See `models/BoomBox-LICENSE.md` for the upstream notice.

`?example=4` reads glTF JSON, its external BIN and JPEGs; `?example=5` reads
GLB binary data and embedded PNGs. gltf-rs parses the files in Rust/Wasm.
The library importer preserves static transforms, triangle primitives, indices,
normals, tangents, UV0 and vertex colors. It imports original metallic-roughness
PBR materials, including normal/AO/emissive maps and texture sampling state.
HDR lighting, ACES exposure and four-sample HDR rendering are enabled.
The comparison pages retain the original Three.js PBR materials and UltraHDR.
Runtime parsing, image decoding, scene state, material setup and WebGPU rendering
remain in Rust/Wasm. See `../docs/gltf-pbr.md` for the supported subset.

# M2 HDR environment and shading

Royal Esplanade is by Greg Zaal, [Poly Haven, CC0](https://polyhaven.com/a/royal_esplanade).
The original UltraHDR file is copied from the pinned Three.js archive:
`examples/textures/equirectangular/royal_esplanade_2k.hdr.jpg`.
`royal_esplanade_2k.hdr` is a Radiance RGBE conversion of the **official r186
UltraHDRLoader's HDR reconstruction**, not the SDR base JPEG or a separately
exposed download. Run `node tools/convert-environment.mjs` with `tools/serve.py`
running to reproduce it. Source and output hashes are recorded in
`environments.json`; tests verify HDR equivalence against the original decoder.

Cube-UV sampling, GGX filtering, DFG lookup data and ACES matrices are ported from
Three.js r186 under the MIT terms in `../LICENSE-THREE`.

## Examples gallery

`gallery/index.html` is adapted from the pinned Three.js `examples/index.html`;
`files/main.css`, icons, and gallery thumbnails are copied from that same revision
(`148ef33ecb6d2502ff796d4554abd1549c95d519`). Three.js code is MIT licensed;
see `../LICENSE-THREE`. The original Three.js header is retained to reproduce the
requested layout; the browser title and running pages identify three-rs-wasm.
The code button links to the exact original example. Thumbnails depict upstream
examples and are not claims that their scenes have been ported to Rust. Source
links/hashes are recorded for every entry in `gallery/catalog.json`.

Roboto Mono (Regular/Medium, unmodified from the Three.js archive) is by the
Roboto Mono Project Authors and licensed under SIL OFL 1.1; see
`files/OFL-RobotoMono.txt` and <https://github.com/googlefonts/robotomono>.

New gallery scene ports retain the algorithms from their pinned upstream HTML
sources (Koch curves, line morph targets, line/point raycasting, UV transforms).
`gallery/assets/uv-grid.jpg` is the unmodified upstream `textures/uv_grid_opengl.jpg`.
`gallery/assets/panorama.jpg` is the unmodified upstream
`textures/2294472375_24a3b8ef46_o.jpg`, credited by the official panorama example to
[Jón Ragnarsson](https://www.flickr.com/photos/jonragnarsson/2294472375/).
The importer probes read original models from the local reference cache only;
those probe models are not copied into the public demo assets.

`gallery/assets/spot1Lux.hdr` is copied unchanged from the pinned Three.js
`examples/textures/equirectangular/spot1Lux.hdr` for the PMREM light calibration scene.

The LTC area-light tables in `src/shaders/ltc-r186.bin` and shading formulas
are derived from the same MIT-licensed Three.js revision.
`tools/core/prepare_ltc.py --check` verifies the tables against the pinned source.

## Additional geometry and instancing ports

`src/geometry/primitives.rs` and the segmented BoxGeometry construction follow the
MIT-licensed r186 geometry algorithms. Attribution and terms: `../LICENSE-THREE`.
The independent geometry oracle in `tests/fixtures/procedural-geometries.json` is
generated by `tools/core/geometry-fixtures.mjs` from the pinned upstream sources.

`models/DamagedHelmet/glTF-instancing/` contains the unmodified official
`DamagedHelmetGpuInstancing.gltf` and its three instance-attribute buffers from the
same pinned archive. It references the already vendored DamagedHelmet geometry
and images; the model attribution above also applies.

Additional example assets (Three.js r186, commit
`148ef33ecb6d2502ff796d4554abd1549c95d519`):

- `web/models/Horse/Horse.glb`: unmodified
  `examples/models/gltf/Horse.glb` from the Three.js repository, used by
  `webgl_morphtargets_horse`. The embedded asset metadata identifies THREE.GLTFExporter.
- `web/models/AnimatedMorphSphere/AnimatedMorphSphere.{gltf,bin}`: unmodified
  `examples/models/gltf/AnimatedMorphSphere/glTF/` files. Howard Wolosky,
  CC0/public domain, as stated in the upstream model README.
- `web/gallery/assets/disc.png`: unmodified `examples/textures/sprites/disc.png`.
- Hilbert/Catmull-Rom and procedural scene formulas are ports of the pinned
  Three.js examples/utilities; the Three.js MIT notice is retained in
  `../LICENSE-THREE`.

## Forest House (glTF AVIF example)

`models/AVIFTest/forest_house.glb` is copied unchanged from the pinned Three.js
r186 archive (`examples/models/gltf/AVIFTest/forest_house.glb`).
[Forest House](https://sketchfab.com/3d-models/forest-house-52429e4ef7bf4deda1309364a2cda86f)
by peachyroyalty is licensed under
[CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/); the asset is not
covered by this repository's MIT license. The pinned file uses Draco geometry
and AVIF textures. `python3 tools/gallery/prepare_avif.py` verifies its SHA-256
and prepares the upstream Draco decoder for independent reference tests only.
The product uses the Rust Draco decoder and the browser AVIF codec. Scene,
materials and camera controls run in Rust; no JavaScript Three.js renderer is
loaded by the product. The original information overlay is not reproduced.

## Iridescence Lamp

`models/IridescenceLamp.glb` is copied unchanged from the pinned Three.js r186
archive. © 2022 Wayfair, LLC; asset by Eric Chadwick, licensed under
[Creative Commons Attribution 4.0](https://creativecommons.org/licenses/by/4.0/).
[Original model and attribution](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/IridescenceLamp).

`environments/venice_sunset_1k.hdr` is copied unchanged from the same archive.
Venice Sunset is by Greg Zaal, [Poly Haven, CC0](https://polyhaven.com/a/venice_sunset).
Source paths and content hashes are recorded in `tools/gltf_examples/assets.json`.

## Additional physical glTF scenes (Three.js r186)

The following unchanged GLBs are copied from the pinned Three.js examples revision.
The original embedded PNG/JPEG textures are retained.

- `models/AnisotropyBarnLamp.glb`: © 2023 Wayfair, LLC; Eric Chadwick.
  [CC BY 4.0 and source](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/AnisotropyBarnLamp).
- `models/SheenChair.glb`: © 2020 Wayfair, LLC; Eric Chadwick.
  [CC0 and source](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/SheenChair).
- `models/IridescentDishWithOlives.glb`: © 2020 Wayfair, LLC; Eric Chadwick.
  [CC BY 4.0 and source](https://github.com/KhronosGroup/glTF-Sample-Assets/tree/main/Models/IridescentDishWithOlives).

These scenes use the existing Royal Esplanade environment credited above.

## Dispersion and compressed glTF scenes (Three.js r186)

These files are copied unchanged from the pinned archive; source paths and
SHA-256 hashes are recorded in `tools/gltf_examples/assets.json`.

- `models/DispersionTest.glb`: © 2023 Analytical Graphics, Inc.; model by
  Ed Mackey, CC BY 4.0. Cloth backdrop CC0 by Adobe Inc. Attribution is embedded
  in the original GLB's `asset.copyright`.
- `models/coffeemat.glb`: [Coffeemat](https://sketchfab.com/3d-models/coffeemat-7fb196a40a6e4697aad9ca2f75c8b33d)
  by [Roman Red](https://sketchfab.com/OFFcours1),
  [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), as recorded in
  the original GLB's `asset.extras`. Meshopt and KTX2/Basis payloads are retained.
- `environments/pedestrian_overpass_1k.hdr`: Pedestrian Overpass by Greg Zaal,
  [Poly Haven, CC0](https://polyhaven.com/a/pedestrian_overpass).

# TSL examples

The Rust graph translations and `src/browser/tsl_crt.wgsl` follow the pinned
Three.js r186 examples `webgpu_tsl_interoperability`, `webgpu_texturegrad`, and
`webgpu_procedural_texture`, and the `Checker` / `GaussianBlurNode` implementations.
Three.js MIT terms are in `../LICENSE-THREE`. The CRT example credits
[Xor's Mini CRT](https://mini.gmshaders.com/p/gm-shaders-mini-crt).

`gallery/assets/earth-lights.png` is the unchanged
`examples/textures/planets/earth_lights_2048.png` from commit
`148ef33ecb6d2502ff796d4554abd1549c95d519`.
SHA-256: `0564fa57f5fade2f65e56d3ad59b3c6bf5e32c44f60370a3e46fd367b0cabaee`.
`gallery/assets/uv-grid.jpg` is the existing pinned `uv_grid_opengl.jpg`.
`tools/tsl/prepare.py` restores the image and reference-only helper modules from
that same archive; `tools/gallery/check.py` verifies published asset bytes.

The additional GPU-pass translations follow `webgpu_compute_texture`,
`webgpu_rtt`, `webgpu_postprocessing`, `webgpu_postprocessing_difference`, and
`webgpu_postprocessing_masking`, including `DotScreenNode`, `RGBShiftNode`,
color-adjustment nodes and Neutral tone mapping from the same MIT-licensed
Three.js revision.

`gallery/assets/caravaggio.jpg` is the unchanged
`examples/textures/758px-Canestra_di_frutta_(Caravaggio).jpg` from that archive
(Caravaggio's *Basket of Fruit*).
SHA-256: `3f9a556c4e12583981427e2d6b7fa268c1a46c9036a23514acd9e179c22ae37f`.
The masking and difference examples reuse the already credited panorama and
crate assets. `tools/tsl/prepare.py` restores reference assets and
`tools/gallery/check.py` verifies the published Caravaggio image byte-for-byte.

The direct-output, radial blur, FXAA, SSAA and transition ports follow the
pinned r186 examples and `radialBlur`, `FXAANode`, `SSAAPassNode`,
`TransitionNode` and `PhysicalLightingModel` energy compensation (MIT; see `../LICENSE-THREE`). GPU helper translations are in
`src/tsl/` and `src/postprocessing/ssaa.rs`.

The following unchanged `examples/textures/transition/transitionN.png` masks
come from the same pinned archive. `tools/tsl/prepare.py` restores them and
`tools/gallery/check.py` checks their bytes. SHA-256:

- `gallery/assets/transition1.png`: `a5da995d423c701997784a0e3aa662f57569413dd0951666adb71a55539f514f`
- `gallery/assets/transition2.png`: `217881c3b114d1df84278dd4fc5ed641abc467e0e588db89bcdb1532fb1e4d51`
- `gallery/assets/transition3.png`: `b5d09c828cc1a98610664b77695122cf89444891b9a8d25ddab622fd5d20eacf`
- `gallery/assets/transition4.png`: `ea6fda270aa178d68bf134c0d429b266381183f80b89f9f2f8460c8c9b97b03c`
- `gallery/assets/transition5.png`: `93027d664ab8e57d4f07db582b522277091a44973fa2aab2abae346806c2c198`
- `gallery/assets/transition6.png`: `43848b237c3d6a66aa31f9f8375c286c6542be85ac8c2a82e4d0eb8a2a66ee68`

The height-fog, sprite, instanced-sprite, galaxy and afterimage translations
follow the same pinned Three.js r186 examples, `SpriteNodeMaterial`, `Fog.js`,
`RangeNode` and `AfterImageNode` (MIT; see `../LICENSE-THREE`). The galaxy
example credits [Three.js Journey](https://threejs-journey.com/lessons/animated-galaxy).

The following PNG files are copied unchanged from that archive; preparation and
byte verification use `tools/tsl/prepare.py` and `tools/gallery/check.py`.

- `gallery/assets/sprite1.png`: `examples/textures/sprite1.png`; SHA-256 `0c69d3d1eaed72c3f13ddf93db0e233475b0cf850d3e3d9e45fd7a3803cac9e4`.
- `gallery/assets/snowflake1.png`: `examples/textures/sprites/snowflake1.png`; SHA-256 `7b0c12c16b37b0d03e73009da109e97cdfdd1f40f047fd025c80954a8422d220`.
- `gallery/assets/circle.png`: `examples/textures/sprites/circle.png`; SHA-256 `05fff2c8a01602cd4d8099cf456386ab28498093d084d5ca3805d16ebc71d3f8`.

### TSL compute and instancing scenes

`gallery/assets/smoke1.png` and `gallery/assets/suzanne_buffergeometry.json`
are copied unchanged from Three.js r186 commit
`148ef33ecb6d2502ff796d4554abd1549c95d519`, at
`examples/textures/opengameart/smoke1.png` and
`examples/models/json/suzanne_buffergeometry.json` respectively.
The smoke texture is credited upstream to OpenGameArt. Scene expressions and
PCG hash follow the five corresponding official WebGPU examples (MIT;
see `LICENSE-THREE`). `tools/tsl/prepare.py` extracts the pinned assets and
`tools/gallery/check.py` checks their bytes against the archive.

### TSL surfaces, lighting and GPU geometry

The twenty additional scenes and their WGSL translations follow the pinned r186
examples (MIT; `LICENSE-THREE`). MaterialX noise follows Three.js's vendored
MaterialX implementation. ShaderToy retains the source credits in the official
`webgpu_shadertoy.html`; these are fixed embedded shader translations, not a
GLSL transpiler. Raging Sea, Halftone, Flames and Tornado retain the Three.js
Journey attribution from their official pages.

Additional unchanged assets from that archive:

- `gallery/assets/hardwood2_diffuse.jpg`: `examples/textures/hardwood2_diffuse.jpg`; SHA-256 `3986fe3bd7d3c24b18e7da17ae72bbef9385dbc355c931c52a04dbfe1412c802`.
- `gallery/assets/Water_1_M_Normal.jpg`: `examples/textures/water/Water_1_M_Normal.jpg`; SHA-256 `6d7825469a374ff84700b4fb1a2890bd1fce0ca1f777bdd4d5ecdaf15833a804`.
- `gallery/assets/roughness_map.jpg`: `examples/textures/roughness_map.jpg`; SHA-256 `261c6f32ab65a6ab1efa76a85702a1f46168081a3eeabd9d0b49de2f4ba4d7a1`.
- `gallery/assets/flames-grayscale-256x256.png`: `examples/textures/noises/voronoi/grayscale-256x256.png`; SHA-256 `0bd7c9a6e440cb7ddd97a231fc5c3f93582a91609ead627075f6f2aa284aaccc`.
- `gallery/assets/flames-rgb-256x256.png`: `examples/textures/noises/perlin/rgb-256x256.png`; SHA-256 `a6e852dd6115654f6c436454faba04fc7dc5c355a1a3769a7c9c905dbcb8f41a`.
- `models/Michelle.glb`: `examples/models/gltf/Michelle.glb`; SHA-256 `7a87e15a99ccbc5e5877be66e1e4ecae0a581adcafa0cce1a5569f49909e968e`.
- `models/PrimaryIonDrive.glb`: `examples/models/gltf/PrimaryIonDrive.glb`; SHA-256 `4da2bc76db5a1f639866f05219426c9fde0ece3239c3cb054b5675a8dc1cde21`.
- `models/LeePerrySmith.glb`: `examples/models/gltf/LeePerrySmith/LeePerrySmith.glb`; SHA-256 `402b8a8ac9f03232e6d64b5962929703a069daf99d3c49ac8eb0e48bedc9c576`.
- `environments/moonless_golf_1k.hdr`: `examples/textures/equirectangular/moonless_golf_1k.hdr`; SHA-256 `4f597078024bd81429431e872d466d8808653ad62a8bc8c61d8052af7466c3aa`.

`gallery/assets/teapot-18.json` is generated once from the pinned MIT-licensed
`TeapotGeometry` by `tools/tsl/prepare-geometry.mjs`. The float32 LTC table
is generated from the same original tables as the half-float table by
`tools/core/prepare_ltc.py`; see the existing LTC attribution.

The Lee Perry-Smith head scan is by Lee Perry-Smith / Infinite,
[CC BY 3.0](https://creativecommons.org/licenses/by/3.0/), based on
www.triplegangers.com; the [original license](models/LeePerrySmith_License.txt)
is retained. The displayed mesh is scaled and deformed on the GPU.
Primary Ion Drive is by Mike Murdock / indierocktopus,
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), from
[Sketchfab](https://sketchfab.com/models/d3f50a66fee74c6588dd9bc92f7fe7b3).
Michelle is credited by the official example to Mixamo.
The embedded ShaderToy sources credit
[jackdavenport](https://www.shadertoy.com/view/Mt2SzR) and
[trinketMage](https://www.shadertoy.com/view/3tcBzH); WGSL translations retain
those scene contents.

### Additional TSL texture and volume examples

- `gallery/assets/blossom.png`: Three.js r186 example asset, Three.js MIT distribution.
- `gallery/assets/head256x256x109.raw`: scanned head data by Divine Augustine,
  distributed with the official `webgpu_textures_2d-array` example under
  [CPOL](https://www.codeproject.com/info/cpol10.aspx). Extracted without modification
  from `textures/3d/head256x256x109.zip`.
- `gallery/assets/volume-{perlin,cloud}.raw`: generated from the official MIT
  ImprovedNoise implementation and r186 example initialization; see
  `tools/tsl/prepare-volume.mjs`. Volume raymarchers follow the MIT Three.js
  `Raymarching.js`, `Texture3DNode.js` and volume examples.

### Extended TSL, cubemap and postprocessing assets

The following assets are copied unchanged from the pinned Three.js r186 MIT
repository distribution by `tools/tsl/prepare.py`:

- `gallery/assets/spiritedaway.ktx2` (the official compressed array example).
- `gallery/assets/earth_{day,night,bump_roughness_clouds}_4096.jpg`.
- `gallery/assets/castle-{px,nx,py,ny,pz,nz}.jpg`, from SwedishRoyalCastle.
- `gallery/assets/cube_m0*_c0*.jpg`, from the angus custom cubemap mip chain.

`teapot-50-18.json`, `dodecahedron.json`, and `hilbert-points.json` are static
geometry/curve data generated by `tools/tsl/prepare-geometry.mjs` using the pinned
MIT TeapotGeometry, DodecahedronGeometry and GeometryUtils implementations.
The anamorphic, bokeh DOF, bump, sampling and volume shaders follow the corresponding
MIT Three.js TSL nodes. See their upstream files and this repository's LICENSE.

`vendor/wgpu` is the crates.io wgpu 26.0.1 package under its included MIT and
Apache-2.0 licenses. `vendor/wgpu/PATCH.md` documents the WebGPU depthSlice fix.
