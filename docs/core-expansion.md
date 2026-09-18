# Core expansion work

Requested scope: implement the major blockers identified by the all-example run.
The existing 15 partial ports and the recorded 605 runtime attempts are the baseline,
not evidence that the following work is complete.

Implementation status (individual APIs, not full Three.js/example parity):

1. Added Phong/Lambert/Normal/Toon/Matcap/Depth materials, Hemisphere/Spot/LTC RectArea lights, flat shading, fog and wireframe.
2. Added directional/spot/point depth shadows with PCF, bias and cast/receive flags.
3. Added fallible WGSL mesh programs, GPU buffers/compute dispatch, and composable fullscreen effects with input/history. Bright-pass, blur and bloom-composite kernels are included.
4. Added step/linear/cubic animation, quaternion interpolation, weighted mixer and fades, morphs and skins; animated glTF importer and RobotExpressive browser demo.
5. Added GPU instance matrices/colors and ray-hit IDs. The CPU static merge utility is not dynamic BatchedMesh support.
6. Added IOR/specular, clearcoat, sheen, anisotropy, iridescence and extension texture maps; opaque-background transmission/refraction with thickness/attenuation/dispersion. Added Draco/meshopt/quantized geometry, KTX2/Basis and WebP decoding, glTF UV1/texture transforms and GPU instancing.
7. Added clipping planes, stencil/custom blend/color-write controls and GPU wide/dashed lines.

Each stage needs semantic tests, GPU checks, and representative comparisons with
the pinned Three.js reference before it is considered complete. Existing MVP,
PBR and gallery checks must continue to pass. TSL need not be reimplemented to
port equivalent effects in Rust/WGSL. WebXR, physics and audio are separate
platform integrations, not part of this Core implementation sequence.

## Evidence and reproduction

`scripts/check-core` runs native semantic/GPU tests, native/Wasm clippy, builds the
Wasm demo and compares materials against pinned original Three.js r186. Animation
comparison covers world-space vertices of all 14 RobotExpressive clips. Browser
tests exercise switching clips and reject requests for Three.js/GLTFLoader scripts.
`scripts/check-gltf-pbr` and `scripts/check-gallery` cover existing behavior.

The current gallery contains 16 partial ports. Demo:
`/web/gallery/#webgl_animation_skinning_morph`.
Import/evaluation success in the real-model probe is not a completed example or
visual parity. Earlier all-example reports are historical baselines; their
missing-feature labels do not describe the expanded Core.

## Limits

- Skinning/morphing now run in GPU vertex shaders for color and shadows. Wide-line
  expansion and demo vertex displacement also run on the GPU. CPU deformation is
  restricted to explicit queries/test oracles. See [performance acceptance and
  audit](performance-parity.md); visual agreement alone does not establish parity.
- WGSL hooks are not TSL/GLSL source compatibility. Custom shader shadow casters
  reject; use a separate evaluated shadow mesh. Eight nonambient lights,
  16 clipping planes and 48 shadow layers are supported. Area lights affect
  Standard/Physical materials and do not cast shadows.
- Extension maps preserve independent UVs, wrap modes and color spaces, with
  nearest/bilinear sampling. Their array binding stays within WebGPU limits but
  now packs occupied layers and retains GPU-generated mipmaps. Anisotropic filtering is still missing; mixed-size maps can require padded layers.
  Base PBR maps retain existing mipmaps.
- Refraction samples opaque screen-space content. Rough refraction uses a small
  convolution rather than Three.js's mip-chain filter. Nested/transparent/offscreen
  content is not traced. Dispersion has no dedicated reference-image test yet.
- Batches require compatible opaque triangle meshes and rebuild when edited;
  GPU-driven BatchedMesh/culling is not implemented. Wide lines have butt caps and
  independent segments rather than rounded Line2 joins.
- Animated glTF imports triangle meshes. Native AVIF is unsupported; browser image
  decoding can use the platform AVIF codec. This is a decode-only fallback, not GPU compressed-texture reproduction. Textures decode mip 0 to
  RGBA, not hardware-compressed residency. Sparse quantized accessors and
  layered/cube Basis images reject.
- The effect API supports porting postprocessing, but complete SSAO/DOF/TAA demos
  and the remaining official examples/addons are not bundled.

The [expanded native glTF probe](core-gltf-results.json) imported and evaluated
47 of 49 official assets. The previous static importer accepted 15. The two
remaining native rejections are AVIF and a non-triangle primitive; this metric
does not assert material fidelity or add gallery entries.
