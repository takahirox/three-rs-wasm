# Example selection and comparison

- When equivalent official WebGL and WebGPU scenes exist, port and list only
  the WebGPU example. Compare it against the official WebGPU renderer.
- When only a WebGL version exists, port it to Rust/WebGPU. Document and allow
  backend-related image differences, such as MSAA sample placement and output
  conversion order. Do not simply increase every image threshold or forgive
  missing geometry, materials, lighting or behavior.
- Keep raw image measurements. Any additional tolerance needs a per-example
  explanation and evidence. Existing passing thresholds need not be loosened.
- Performance and execution architecture remain acceptance requirements; see
  [performance parity](performance-parity.md). An adapted Three.js WebGPU scene
  can help diagnose a WebGL-only port, but is not an official WebGPU example.
- Judge equivalence by scene content and purpose, not just names. Reviewed
  pairs live in `WEBGPU_EQUIVALENTS` in `tools/gallery/build.py`.

## Current gallery cleanup (pinned r186)

| Removed WebGL listing | Preferred official example | State |
| --- | --- | --- |
| `webgl_loader_gltf` | `webgpu_loader_gltf` | Existing partial port retained |
| `webgl_morphtargets` | `webgpu_morphtargets` | Existing partial port retained |
| `webgl_pmrem_test` | `webgpu_pmrem_test` | Existing partial port retained |
| `webgl_pmrem_equirectangular` | `webgpu_pmrem_equirectangular` | Existing partial port retained |
| `webgl_panorama_equirectangular` | `webgpu_equirectangular` | Same panoramic photograph and viewing purpose; controls and projection setup differ |
| `webgl_lights_rectarealight` | `webgpu_lights_rectarealight` | WebGPU TSL checker/LTC port listed |

The area-light WebGPU port uses the official procedural checker roughness node.
Its transparent MSAA output is unpremultiplied before sRGB conversion and then
premultiplied again, matching the official WebGPU canvas.

Instancing and AVIF glTF examples remain listed with their limitations because
no equivalent official WebGPU scenes were found in this baseline. Historical
investigation reports retain their original measurements and scope.

The newly added `webgpu_loader_gltf_compressed` and
`webgpu_loader_gltf_dispersion` also supersede their corresponding official
WebGL examples; those WebGL entries are excluded from the port inventory.

## Retroreflective Materials: stochastic reflection

The r186 WebGPU scene uses 32 pseudorandom `hashBlur` samples. At DPR 2, changing
only the original shader's two reflection UV components by one Float32 ULP
produces up to 0.524% pixels differing by more than 6/255. The port's measured
maximum is 0.824%; its 3×3 averaged comparison remains below 0.087%. This is a
precision-sensitive sampling comparison, not permission to omit reflective
materials, reduce the 32 samples, lower resolution or remove surface detail.

For this example at DPR > 1 only, require all three bounds: raw differing pixels
at most 1%, raw mean absolute channel error at most 0.5/255, and at most 0.1%
differing pixels after 3×3 averaging (still a 6/255 channel threshold). Preserve
raw images and both metrics. DPR 1 and every other example retain their existing
thresholds. Controls, camera motion, resize and GPU residency are still tested.

The one-ULP experiment is recorded in
[retroreflection-precision.json](retroreflection-precision.json). With the local
server running, reproduce it with `node tools/tsl/check-retro-precision.mjs`.
This diagnostic patches only shaders inside its own browser session; acceptance
comparisons use the unmodified pinned reference shaders.

The material/texture batch also prefers `webgpu_clipping`,
`webgpu_materials_texture_manualmipmap`, `webgpu_textures_anisotropy` and
`webgpu_textures_partialupdate` over their reviewed WebGL counterparts.
The dedicated comparison fixture executes the pinned WebGPU scripts.

The WebGL-only NURBS/text batch records matched MSAA-on/off comparisons in
[shapes.md](shapes.md). Its three scoped MSAA allowances require the ordinary
threshold with MSAA off as a separate regression check; they do not relax
geometry, controls, residency or workload checks.

The WebGL-only buffer-particle batch records point-primitive and dense-line MSAA
exceptions in [buffer-particles.md](buffer-particles.md). Original-backend images
remain required, along with ordinary-threshold quad/WebGPU diagnostic comparisons.
The adapted fixtures are not official WebGPU examples and are not gallery entries.

The WebGL-only raw shader and raycast batch ([interactive-shaders.md](interactive-shaders.md))
bounds only the two 4× MSAA cube comparisons, and requires per-pixel matches
with MSAA off. Hashed procedural noise is compared by distribution only at
resized non-power-of-two heights. That check also requires a recorded one-ULP
`vUv` experiment showing the original itself changes most pixels. Its
power-of-two states must match per pixel.

The WebGL-only picking and canvas batch ([interactive-objects.md](interactive-objects.md))
bounds only the instancing and orientation-wireframe 4× MSAA comparisons. The
same scenes must match within the ordinary threshold with MSAA off.

The WebGL-only voxel, picking, clipping, comparison and blending batch
([interactive-scenes.md](interactive-scenes.md)) bounds only 4× MSAA comparisons
of line, dense-edge and alpha-to-coverage edges. All five scenes match within
the ordinary threshold with MSAA off.

The WebGL-only viewport, OBB, sorted-points and loader batch
([views-loaders.md](views-loaders.md)) bounds only two 4× MSAA wireframe or
edge comparisons. It also bounds one precision-sensitive sorted-points state
by the original's own measured one-ULP camera sensitivity. All other states
keep the ordinary threshold.

The WebGL-only stereo-effect and loader batch
([stereo-loaders.md](stereo-loaders.md)) bounds four cases:

- the anaglyph and parallax composites of 8-bit linear eye targets;
- the PCD points, where the original departs from the exact square coverage the
  suite computes and the port matches;
- one 4× MSAA edge comparison.

The stereo and ImageBitmap scenes match within the ordinary threshold with MSAA
off.

The WebGL-only controls, camera-helper, custom-attribute and draw-range batch
([controls-attributes.md](controls-attributes.md)) bounds only 4× MSAA
comparisons: silhouettes, wireframes and one-pixel points, and the dense
additive draw-range lines. The draw-range scene must keep the original's mean
level in every state. All five scenes match within the ordinary threshold with
MSAA off.

The WebGL-only HDR-texture, voxel-terrain, trackball, sprite and LOD batch
([trackball-sprites.md](trackball-sprites.md)) bounds only 4× MSAA comparisons
of silhouettes and of the dense LOD wireframes. The LOD field must keep the
original's mean level in every state. All five scenes match within the ordinary
threshold with MSAA off.

`webgl_morphtargets` and `webgl_panorama_equirectangular` remain excluded in
favor of their ported WebGPU equivalents.
`webgl_buffergeometry_glbufferattribute` remains excluded as a WebGL-specific
API.

The WebGL-only terrain, GCode, VOX and OBJ batch
([terrain-loaders.md](terrain-loaders.md)) bounds only 4× MSAA comparisons of
silhouettes and the dense GCode lines. The GCode scene must keep the original's
mean level in every state. All five scenes match within the ordinary threshold
with MSAA off.

The WebGL-only MDD, edge-split, 3DS, teapot and scattering batch
([models-modifiers.md](models-modifiers.md)) bounds only 4× MSAA comparisons of
silhouettes, the teapot wireframe and the scattered flowers. All five scenes
match within the ordinary threshold with MSAA off.

The WebGL-only BVH, framebuffer-texture, float-readback, GPU-picking and
instancing-performance batch ([picking-buffers.md](picking-buffers.md)) bounds
4× MSAA comparisons of lines, the dense Gosper curve and silhouettes. With MSAA
off, it bounds only the BVH scene, whose grid centre line lies exactly on a
pixel-row boundary. The float-readback and GPU-picking examples have no resize
handler, so their resized frames are not compared.

The WebGL-only PDB, helpers, simplifier, AMF and TIFF batch
([helpers-formats.md](helpers-formats.md)) bounds 4× MSAA comparisons of
silhouettes, flat facets and grid lines. With MSAA off, it bounds only the
flat-shaded simplified head, whose facet edges take derivative normals. The PDB
labels are part of the compared pixels.

The WebGL-only refraction, PLY, KMZ, Collada and EXR batch
([refraction-loaders.md](refraction-loaders.md)) bounds 4× MSAA comparisons of
silhouettes, facets and grid lines. With MSAA off, it bounds only the refracted
statues' fine environment detail, and the PLY scene's flat facets and shadow
edges. The shadow edges differ because WebGL's `gl_FragCoord` rows are flipped
relative to WebGPU's.
