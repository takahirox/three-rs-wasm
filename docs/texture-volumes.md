# Texture arrays, volumes, environments, stereo, compressed textures, channels, clipping, post passes and depth buffers

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/texture_volumes.rs`; the NRRD slices are in
`src/browser/texture_volumes/slices.rs` and the array examples in
`src/browser/texture_volumes/arrays.rs`. The EXR environment is in
`src/browser/envmap_exr.rs` and the shadow-map viewer in
`src/browser/shadowmap_viewer.rs`. The batch that follows adds
`src/browser/stereo_loaders.rs` (display stereo), `dds.rs`, `kinect.rs`,
`channels.rs`, `wide_gamut.rs`, `uv_tests.rs`, `clipping_stencil.rs`,
`ascii.rs`, `glitch.rs` and `ssao.rs`; the next adds `sao.rs`, `taa.rs`,
`outline.rs`, `ktx2.rs`, `elements_text.rs`, `reversed_depth.rs`,
`log_depth.rs`, `lines_raycast.rs`, `pvr.rs` (with the PVRTC decoder in
`pvrtc.rs`) and `ktx.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_texture2darray_layerupdate` | 314 | The three-layer KTX2 (Basis) array kept compressed, one instanced plane per layer, and srcLayer/destLayer/transfer copying a source layer on the GPU, rendering only on change |
| `webgl_texture3d` | 315 | The gzip NRRD stent as a float 3D texture, VolumeRenderShader1's MIP and ISO ray marches, both colormaps, all five controls and the z-up orthographic orbit (zoom 0.5–4), rendering only on change |
| `webgl_texture2darray` | 324 | The unzipped 256×256×109 head as a red DataArrayTexture with nearest filtering, the layer bouncing by 0.4 per frame, in a raw ShaderMaterial |
| `webgl_texture2darray_compressed` | 325 | The Basis KTX2 array kept compressed, the timer cycling its five layers, in a raw ShaderMaterial |
| `webgl_rendertarget_texture2darray` | 326 | Each frame's head layer rendered on the GPU into a 256×256×109 red WebGLArrayRenderTarget, then shown from that layer, with the intensity control |
| `webgl_materials_envmaps_exr` | 332 | The PIZ EXR and PNG panoramas prefiltered once into PMREMs for the turning torus knot and shown as backgrounds, with ACES exposure, roughness, metalness and the map switch |
| `webgl_shadowmap_viewer` | 333 | Spot and directional BasicShadowMap shadows on the spinning knot and cube, both CameraHelpers, and the two ShadowMapViewer HUDs with their canvas labels |
| `webgl_loader_nrrd` | 321 | NRRDLoader's short-typed LPS volume, the three VolumeSlices repainted on the CPU through 2D canvases, the BoxHelper, TrackballControls and all seven controls |
| `webgpu_display_stereo` | 334 | StereoPassNode, AnaglyphPassNode (seven algorithms, three color modes, frameCorners at the plane distance) and ParallaxBarrierPassNode over half-float eye targets; 500 instanced spheres reflecting the cube map per fragment, the cube background and OrbitControls |
| `webgl_loader_texture_dds` | 335 | DDSLoader's DXT1/DXT3/DXT5 and BC6H block textures uploaded once in their GPU formats, the ARGB/RGB mip chains, three single-file cube maps, alpha test, additive and transparent materials on thirteen turning meshes |
| `webgl_video_kinect` | 336 | 640 × 480 points whose depth is read from the Kinect video in the vertex stage, additive 0.2-alpha 2-pixel squares, the four GUI values and the pointer-following camera |
| `webgl_materials_channels` | 337 | The displaced ninja head through MeshStandardMaterial, MeshNormalMaterial with its normal map, the VelocityShader and MeshDepthMaterial's four packings, three sides, both cameras and their damped OrbitControls |
| `webgl_test_wide_gamut` | 338 | LinearDisplayP3 working space with sRGB output, the sRGB logo converted to Display P3 on upload, the P3 logo uploaded unconverted, contained backgrounds and the scissor slider |
| `misc_uv_tests` | 339 | UVsDebug's Canvas 2D drawings of nine geometries' uvs and indices |
| `webgpu_clipping_stencil` | 340 | Three ClippingGroup planes, per-plane stencil caps, PlaneHelpers, the clipped SunLight shadow on a ShadowNodeMaterial ground and every control |
| `webgl_effects_ascii` | 341 | The bouncing flat-shaded sphere at pixel ratio 1, AsciiEffect's inverted character table and TrackballControls |
| `webgl_postprocessing_glitch` | 342 | 100 instanced flat-shaded spheres, GlitchPass (displacement map, RGB shift, snow, trigger cycle, wild mode) and OutputPass, after the photosensitivity warning |
| `webgl_postprocessing_ssao` | 343 | 100 instanced Lambert boxes, SSAOPass (normals and depth, kernel, simplex rotation noise, blur, multiplied composite, all five outputs, the parameters) and OutputPass |
| `webgl_postprocessing_sao` | 344 | The knot of turning spheres, SAOPass (normal render, 24-bit packed depth, spiral occlusion, depth-limited separable blur, the four outputs and every parameter) and OutputPass |
| `webgl_postprocessing_taa` | 345 | TAARenderPass's jittered accumulation (sample levels, the index/200 toggle and the Disabled/Enabled switch) over the turning boxes, frame by frame |
| `webgpu_postprocessing_outline` | 346 | The OBJ tree, 20 spheres, torus and floor with SunLight shadows, OutlineNode's depth, mask, edge and blur passes, the pulse, both colors, pointer selection and OrbitControls |
| `webgpu_loader_texture_ktx2` | 347 | The page's sections and labels, and each KTX2 file (uncompressed, BC, ETC, ASTC, ETC1S and UASTC) uploaded as stored or transcoded once, drawn into its element's scissored viewport |
| `webgl_multiple_elements_text` | 348 | The article (text and MathML) over the fixed canvas, six views of lattice or random molecules displaced by plane, cylindrical and spherical waves, per-view viewports and OrbitControls |
| `webgpu_reversed_depth_buffer` | 349 | Five pairs of nearly coplanar planes drawn by three renderers side by side: Depth24Plus, logarithmic fragment depth and reversed Depth32Float |
| `webgpu_camera_logarithmicdepthbuffer` | 350 | Fifteen text labels from 1 µm to 1000 light years with near 1e-6 and far 1e27, the normal and logarithmic views on either side of the draggable border, the per-frame zoom, wheel and mouse |
| `webgpu_lines_fat_raycasting` | 351 | The CatmullRom spiral as LineSegments2 or Line2 in world units or pixels with alpha to coverage, the pointer raycast and its two spheres, the visualized threshold and the translation |
| `webgl_loader_texture_pvrtc` | 352 | PVRLoader's v2 and v3 PVRTC 2/4 bpp maps with and without mips, the alpha flares and two cube maps reflected by the tori |
| `webgl_loader_texture_ktx` | 353 | KTXLoader's KTX 1 files chosen by the WebGL extensions as the original does: PVRTC, BC1/BC3/BC5, ETC1, EAC RG and ASTC color maps, flares and packed two-channel normal maps under a point light |

`webgpu_display_stereo`, `webgpu_clipping_stencil`,
`webgpu_postprocessing_outline`, `webgpu_loader_texture_ktx2`,
`webgpu_reversed_depth_buffer`, `webgpu_camera_logarithmicdepthbuffer` and
`webgpu_lines_fat_raycasting` are WebGPU examples and are compared against the
WebGPU renderer (the WebGL outline, KTX2, reversed-depth, logarithmic-depth and
fat-line twins are listed as equivalents only). None of the others has an
official WebGPU equivalent in the pinned inventory, so each is compared against
the WebGL renderer. `misc_uv_tests` draws no WebGL at all; its canvases are
compared pixel for pixel.

## Port notes

### Layer updates

- **Array.** The source KTX2 stays block-compressed. Its layers are copied on
  the GPU into a three-layer array, as `copyTextureToTexture` with a layer
  does. A new CompressedArrayTexture has no color space, so the array uses
  the linear block format.
- **Drawing.** One instanced plane draws all three layers; each instance
  samples its own layer.

### Float volume

- **Loader.** `parse_nrrd` reads the text header, skips the gzip member
  header, inflates the data and decodes little-endian floats or shorts.
- **Ray march.** The MIP and ISO loops take VolumeRenderShader1's 887 steps
  from the back face. The colormap lookup moves after the loop, into uniform
  control flow, since WGSL forbids implicit-LOD sampling in the loop.
- **Camera.** The orthographic frustum keeps its height on resize, and the
  orbit zoom is clamped to 0.5–4.

### Texture arrays

- **Head volume.** The head is uploaded once into an R8 array with nearest
  filtering. The `int depth` uniform truncates, as the WebGL uniform does.
- **Stepping.** The per-frame 0.4 step runs once per 60 fps step, with the
  reflection test once per frame, as the fixture's stepped frame does.
- **Render target.** The array render target is a DataArrayTexture, so it
  keeps nearest filtering. Only the current layer is rendered each frame, by
  the GPU pass. WebGPU attachments start at the top row, so the pass flips v
  to keep texel rows where WebGL writes them.

### EXR environment

- **Maps.** `decode_exr` (with PIZ) and the PNG are each prefiltered once
  into a resident PMREM atlas; switching maps swaps the resident
  environments.
- **Background.** An sRGB background texture is not tone mapped, as
  WebGLBackground turns tone mapping off for it: `Scene::background_tone_mapped`
  now carries that.
- **Debug plane.** It shows the port's own cube-UV atlas, whose layout is not
  three's, so it is not compared.

### Shadow-map viewer

- **Atlas.** `Renderer::shadow_atlas` exposes the resident shadow depth atlas.
  Each HUD reads its light's layer with `textureLoad` and reproduces the depth
  material's RGBA8 `1 − depth` (cleared white), sampled bilinearly and shown
  as `1 − r`. Its raw output is decoded before the output encoding, so it
  reaches the canvas unchanged.
- **Overlay.** The scene and the HUD scene render into one target; the HUD
  clears only depth, as `autoClear = false` with `clearDepth()` does. A pass
  without shadow casters now keeps the resident atlas and caster slots, so
  the overlay no longer reallocates them.
- **Shadow cameras.** Their view now follows Matrix4.lookAt, including the
  0.0001 nudge when the forward axis is parallel to `up` (the directional
  light straight above).
- **Helpers.** CameraHelper keeps the projection it read when constructed:
  the example's near, far and frustum edits, with the spot camera's default
  fov of 50, since the shadow pass sets 2 × angle only later.
- **Labels.** Each light's name is drawn in `Bold 20px Arial` on a canvas
  sized by `measureText`, as a CanvasTexture.

### NRRD slices

- **Volume.** The IJK-to-RAS matrix is built from `space directions` and the
  LPS transition, as NRRDLoader does. The spacing, axis order, RAS
  dimensions and the min/max window follow the loader.
- **Slices.** `extractPerpendicularPlane` is ported: the inverse-matrix
  directions, the slice lengths and plane size, the IJK index, and the
  access directions. Indices use JavaScript's arithmetic, so an index past
  an edge reads the neighbouring row and one outside the data fails both
  threshold tests.
- **Repaint.** Each repaint thresholds and windows the slice on the CPU,
  puts it into a buffer canvas, scales it into the plane-sized canvas with
  the browser's `drawImage`, and copies that canvas into the texture. This
  is the original's path, including:
  - the threshold setters marking every slice's geometry, so their repaint
    clears the canvas;
  - the window settings drawing over the previous canvas.
- **Planes.** The plane size depends only on the axis, so each slice keeps
  its resident PlaneGeometry and changes only its matrix. The original
  builds a new one when the index changes.
- **Controls.** TrackballControls runs with rotateSpeed 5, zoomSpeed 5,
  panSpeed 2 and distances 100–500, stepped at 60 fps. `Trackball` now takes
  its rotate and zoom speeds as fields.

### Display stereo

- **Passes.** The stereo, anaglyph and parallax-barrier composites are one
  WGSL pass over two half-float eye targets. The anaglyph matrices are the
  node's full table (column-major). The anaglyph eyes use frameCorners at the
  plane distance; the stereo eyes use the half-width aspect.
- **Spheres.** One InstancedMesh draw; the 500 matrices are streamed each
  frame, as the original's `instanceMatrix.needsUpdate` does. The node
  material reflects per fragment (reflectVector) and samples the CubeTexture
  mirrored in x; the background samples level 0.
- **Controls.** OrbitControls runs on the camera's matrices lagged by one
  event, as the pipeline example's camera is never rendered directly.

### DDS textures

- **Loader.** `parse_dds` follows DDSLoader: the FourCC and DX10 formats, the
  uncompressed ARGB and RGB mips reordered to RGBA, and the six faces of a
  cube map. Block data is uploaded as stored; each level copies its physical,
  block-rounded size.
- **Filtering.** The example's `LinearFilter` maps (and the loader's
  single-level ones) sample level 0; mip chains filter trilinearly with
  anisotropy 4, as WebGL applies it only to mipmapped filters.
- **Cube maps.** A single-file DDS cube is a CompressedTexture, not a
  CubeTexture, so WebGL's `flipEnvMap` is +1: the reflection is not mirrored.
- **Materials.** Opaque materials write alpha 1; the signed BC6H texels below
  zero encode to 0. The additive double-sided material draws back faces,
  then front faces.

### Kinect points

- **Frames.** Each new video frame is decoded into an ImageBitmap without
  color space conversion and copied to the GPU once. WebGL uploads the
  NoColorSpace VideoTexture with `UNPACK_COLORSPACE_CONVERSION_WEBGL = NONE`;
  a direct video copy to WebGPU would convert the untagged frame to sRGB.
  The bitmap decodes asynchronously and uploads on the next frame.
- **Points.** The vertex stage reads the depth at level 0 (magnified,
  linear) and places each point; the position attribute's (x, y) is derived
  from the instance index. Frustum culling uses the attribute's bounding
  sphere. modelViewMatrix is composed in f64, as three does.

### Material channels

- **Displacement.** Every material displaces on the GPU at level 0 of the
  displacement map.
- **Normal map.** getTangentFrame is ported with WebGL's `dFdy` as `−dpdy`.
  WebGLMaterials negates the normalScale uniform of a BackSide material; the
  port does the same for the normal and standard materials.
- **Depth.** vHighPrecisionZW uses WebGL's −1..1 projection composed in f64;
  the four packings are packing.glsl's.
- **Velocity.** The previous and current projection-view matrices and
  `modelMatrixPrev` (identity until the first render) follow the example.
- **Raw output.** The normal, depth and velocity shaders write their values
  unconverted: the target's sRGB encoding is undone first.
- **Cameras.** Both damped OrbitControls update every frame; the wheel zooms
  the orthographic camera within 0.5–1.5. A resize changes only the active
  camera, and the orthographic frustum becomes ± innerHeight × aspect.

### Wide gamut

- **Upload.** WebGL's `unpackColorSpace` is Display P3: the sRGB logo is
  converted by the browser (`copyExternalImageToTexture` into Display P3),
  the P3 logo matches the working primaries and is copied unconverted.
- **Output.** `value.rgb * mat3( … )` with three's 4-decimal
  working-to-sRGB matrix, then the sRGB transfer.
- **Slider.** The page's slider and labels are DOM overlays; the scissor
  split is `round( sliderPos × pixelRatio )`.

### UV tests

- **Drawing.** UVsDebug's outlines, face numbers and vertex labels are drawn
  with the page's Canvas 2D API from the Rust geometries, with the same
  double-precision arithmetic (divideScalar multiplies by the reciprocal).
  No WebGPU drawing is involved, as the original uses no WebGL.

### Clipping stencil

- **Stencil.** Each ClippingGroup mesh inverts its plane's bit on both faces
  without color or depth; each cap draws where its bit is set, clears it and
  is clipped by the other planes. renderOrder 1, 1.1, 2, … keeps its order as
  tenths.
- **Ground.** ShadowNodeMaterial's ShadowMaskModel: black with alpha
  opacity × (1 − the product of the light shadow factors), transparent and
  double sided (back faces, then front faces).
- **Planes.** The caps follow their planes and the helpers are
  PlaneHelper's outline strip and 0.2-opacity plane; `negated` negates the
  plane on every change, as the GUI callback does.

### ASCII effect

- **Readback.** After the frame, in the same task, the canvas is drawn into a
  canvas at 0.15 of its size with `drawImage` and read with `getImageData`,
  as AsciiEffect does; every second row's brightness picks a character of
  the inverted set, and the table is written with the effect's markup.
- **Controls.** TrackballControls reads its element's rectangle once, before
  the first render has filled the table, so its height is 0, as in the
  original. The renderer keeps pixel ratio 1.

### Glitch

- **Random numbers.** GlitchPass draws from Math.random each frame, and
  three's UUIDs do too. The fixture rewrites the addon alone to draw from the
  seeded sequence; the port consumes the same sequence in the same order:
  the instances, the 64 × 64 heightmap, the trigger, then each frame's draws.
- **Shader.** DigitalGlitch reads the half-float scene target flipped in v
  and uses WebGL's bottom-up `gl_FragCoord.y` for the snow. The pass is one
  full-screen triangle, as FullScreenQuad.
- **Warning.** The page shows the photosensitivity warning and starts after
  “Okay”.

### SSAO

- **Random numbers.** SSAOPass's kernel and SimplexNoise's permutation and
  rotation noise draw from Math.random; they use the seeded sequence as the
  glitch does. The kernel is written into the shader as constants.
- **Passes.** The override render draws the instances with the normal
  material; its clear is replaced by the scene's Color background, which
  WebGLBackground always clears with. Depth, normals and noise are read with
  `textureLoad` at WebGL's texel (rows from the bottom). The blur averages 5 × 5
  texel centers; Default multiplies the blurred occlusion into the read
  buffer (DstColor, Zero).
- **Pixel ratio.** The example never sets the pixel ratio, so the port renders
  at 1.

### SAO

- **Passes.** The normal render, SAO, the two blur directions and the
  multiplied or replaced copy are full-screen triangles with WebGL's texel
  rows. Depth is quantized to 24 bits as the depth texture stores it; the
  noise uses the exact pixel-centre uv WebGL interpolates.

### TAA

- **Accumulation.** The jitter tables, the sample and hold targets, the
  additive One/One copies and the index/200 toggle follow TAARenderPass; the
  test aligns frames with the original, including the level and enable
  changes.

### Outline

- **Passes.** Depth and mask mirror scenes, downsampling, two edge and blur
  pairs and the composite follow OutlineNode in RGBA8 targets; the mirror
  scenes are warmed with the first render so selection changes create no GPU
  resources. The pointer raycast selects in `prepare`.

### KTX2 views

- **Page.** The sections, descriptions and labels are built as the page
  builds them; the gallery's own heading, language and font-smoothing rules are
  reverted for the page. Each view is drawn into its element's viewport and
  scissor of one canvas-sized target.

### Multiple elements with text

- **Waves.** The original displaces every point on the CPU each frame; the
  port keeps the lattice and random positions resident and evaluates the same
  plane, cylindrical and spherical waves in the vertex stage.
- **Views.** The article is the original's markup; each view is drawn at
  WebGL's rounded viewport, clipped to the canvas with a matching view
  offset. The random positions use the fixture's seeded sequence.

### Reversed and logarithmic depth

- **Buffers.** The three renderers are three viewports of one canvas: a
  Depth24Plus buffer with LessEqual, the same with the fragment depth of
  `viewZToLogarithmicDepth`, and a Depth32Float buffer cleared to 0 with
  GreaterEqual and the reversed projection. The planes keep three's vertex
  order and matrix products, so the z-fighting matches.
- **Renderer.** `logarithmicDepthBuffer` is a renderer setting: built-in
  materials seen through a perspective camera write the logarithmic depth
  from the camera's near and far. Frustum planes that degenerate at far =
  1e27 reject nothing, as in three.

### Fat line raycasting

- **Raycast.** LineSegments2.raycast runs as the explicit CPU query in world
  units or CSS-pixel screen space, on the matrixWorld the previous render
  left; the threshold line copies the line's transform before the frame's
  turn, as the original's order does.
- **Output.** The renderer has `alpha: true`: alpha-to-coverage edges keep
  their alpha, and the output unpremultiplies, encodes and premultiplies as
  RenderOutputNode does, onto the page's black body.

### PVRTC

- **Decoding.** WebGPU has no PVRTC formats. Each level is decoded once at
  load with the steps of Imagination's reference decompressor and uploaded as
  RGBA8; the decoded texels match the Metal adapter's hardware decode. WebGL
  has no sRGB PVRTC formats, so the texels are sampled as stored in both.
- **Filters.** Mip chains are sampled trilinearly without anisotropy.

### KTX

- **Formats.** The original picks its textures from the WebGL extensions;
  the port queries the same list from a WebGL context and uploads BC, ETC and
  ASTC data as stored (sRGB variants where the textures are sRGB), and PVRTC
  decoded as above.
- **Normal maps.** BC5 and EAC RG normal maps reconstruct z from x and y, as
  WebGL's `USE_PACKED_NORMALMAP` does; the renderer applies this to any
  two-channel CompressedTexture normal map.

Stats and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/texture-volumes.html` executes the pinned sources on the
example clock. The suite covers every control, the transfers, drags,
trackball pans, wheels and resize, at DPR 1 and 2. Examples with
antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Layer updates, MSAA off / on | 0% / 0.01, 0.002% / 0.01 | 0% / 0.01, 0.068% / 0.03 |
| Float volume | 1.542% / 0.38 | 1.506% / 0.28 |
| NRRD slices, MSAA off / on | 0.001% / 0.00, 0.801% / 0.33 | 0% / 0.00, 0.413% / 0.17 |
| Head array | 0% / 0.00 | 0% / 0.00 |
| Compressed array | 0% / 0.00 | 0% / 0.00 |
| Array render target | 0% / 0.00 | 0% / 0.00 |
| EXR environment | 0.076% / 0.16 | 0.042% / 0.16 |
| Shadow-map viewer, MSAA off / on | 0.001% / 0.00, 1.657% / 0.51 | 0% / 0.00, 0.864% / 0.27 |
| Display stereo | 0.006% / 0.05 | 0.049% / 0.05 |
| DDS textures, MSAA off / on | 0% / 0.00, 0.455% / 0.10 | 0% / 0.00, 0.206% / 0.05 |
| Kinect points | 0.567% / 0.16 | 0.247% / 0.07 |
| Material channels | 0.376% / 0.08 | 0.365% / 0.08 |
| Wide gamut, MSAA off / on | 0% / 0.00, 0% / 0.00 | 0% / 0.00, 0% / 0.00 |
| UV tests (canvas pixels) | 0 pixels | 0 pixels |
| Clipping stencil, MSAA off / on | 0.001% / 0.00, 0.289% / 0.07 | 0% / 0.00, 0.143% / 0.04 |
| ASCII effect (table cells) | 0.66% of cells | 0.66% of cells |
| Glitch | 1.690% / 0.17 | 0.880% / 0.09 |
| SSAO | 0% / 0.00 | 0.001% / 0.00 |
| SAO | 0% / 0.01 | 0% / 0.00 |
| TAA (frame by frame) | 0.026% / 0.01 | 0.001% / 0.00 |
| Outline | 0% / 0.01 | 0.001% / 0.01 |
| KTX2 views (page) | 0% / 0.00 | 0% / 0.00 |
| Multiple elements with text | 1.262% / 0.27 | 0.327% / 0.10 |
| Reversed depth buffer | 0% / 0.00 | 0% / 0.00 |
| Logarithmic depth buffer | 0.099% / 0.13 | 0.092% / 0.13 |
| Fat line raycasting | 1.194% / 0.31 | 1.062% / 0.31 |
| PVRTC, MSAA off / on | 0.032% / 0.01, 0.528% / 0.17 | 0.026% / 0.00, 0.237% / 0.08 |
| KTX, MSAA off / on | 0.421% / 0.53, 0.402% / 0.30 | 0.030% / 0.23, 0.187% / 0.24 |

### Volume colormap lookup

- **Cause.** The original samples the colormap with implicit LOD in
  non-uniform control flow, so its derivatives come from neighbouring
  fragments whose rays stop at different steps. The port samples in uniform
  control flow after the loop, where the derivatives differ.
- **Effect.** Pixels along the stent's edges and fine structure differ in
  both styles: up to 0.7% in MIP and 1.5% in ISO.
- **Bound.** The volume is bounded at 2% / 0.5.

### Kinect point squares

- **Cause.** WebGL rasterizes `gl_PointSize` points; the port draws two
  triangles per point. Where a point's edge falls within the rasterizer's
  sub-pixel precision of a pixel center, the two rules cover different
  rows, and a 0.2-alpha contribution moves by one pixel.
- **Effect.** Isolated ±51 differences on the dense grid: up to 0.57%.
- **Bound.** 0.8% / 0.6.

### Glitch edges

- **Cause.** The RGB shift and the displacement sample the scene target at
  shifted coordinates with linear filtering; along polygon edges the
  filtered values differ between the backends' texture coordinate
  precision. Bypassed frames match to 0.1%.
- **Effect.** Dashed differences along edges in glitched frames: up to 1.7%,
  mean error at most 0.17.
- **Bound.** 2% / 0.6.

### Wave displacement precision

- **Cause.** The original displaces the points on the CPU in double
  precision; the port evaluates the same waves in f32 in the vertex stage.
- **Effect.** A few sprite edges land on the neighbouring pixel: up to 1.3%,
  all at sprite edges.
- **Bound.** 2% / 0.4.

### Fat line threshold

- **Cause.** The visualized threshold is a translucent alpha-to-coverage
  overlay; its dithered samples resolve slightly brighter in the port.
- **Effect.** Up to 1.2% of pixels, all on the 4 px threshold ribbons; every
  other state of the test is exact.
- **Bound.** 1.5% / 0.5 for that state only.

### ASCII cells

The table is compared cell by cell. Brightness near a character threshold
changes a few edge cells: at most 0.66% of cells, bounded at 1%.

### MSAA

With 4× MSAA, the slice example differs only along the box helper's and the
planes' edges while rotating, and the shadow-map viewer along its helper
lines and silhouettes; with MSAA off both match. With MSAA they are bounded
at 1.2% / 0.45 and 2.5% / 0.8. The PVRTC boxes differ along their edges with
MSAA only, bounded at 0.8% / 0.3.

## Performance evidence

- **Draw workload.** The measured draws equal the original's.
- **Uploads.** No geometry or texture data is written in steady frames:
  - the layer example copies layers on the GPU only on transfer;
  - the volume is uploaded once;
  - the slices copy their canvases only on a repaint, which a control
    change requests, as the original's `needsUpdate` does.
  - the array render target is drawn one layer per frame on the GPU;
  - the EXR and PNG PMREMs are prefiltered once.
- **Background draw.** WebGL draws the equirectangular background as a
  36-index box and the port as a fullscreen triangle; the workload test pairs
  the two.
  WebGPU draws the stereo example's cube background as a 5,952-index sphere
  and the port as a 36-index box, once per eye.
- **Streaming.** The stereo spheres stream their 500 instance matrices each
  frame, as the original's uniform-buffer upload does, within 1.25 × its
  bytes. Nothing else is streamed:
  - each new Kinect frame is copied once from an ImageBitmap;
  - the DDS textures, the glitch heightmap and the SSAO noise and kernel
    are uploaded once;
  - the channels, Kinect and clipping examples keep their geometry resident
    and displace or place it on the GPU;
  - the multiple-elements views keep their molecules resident and displace
    them in the vertex stage, where the original rewrites every position on
    the CPU each frame;
  - the KTX and KTX2 textures are uploaded once as stored, and the PVRTC
    levels are decoded once at load (RGBA8 keeps 4–8 × the compressed
    memory).
- **Readback.** The ASCII effect reads the frame back each frame, as the
  original does, at 0.15 of its size.
- **Queries.** The fat-line raycast and the outline selection are CPU
  queries per frame, as in the originals; they do not replace GPU work.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize
  create no GPU resources; the ASCII and glitch tests repeat their cycles
  too.

No GPU timing parity is claimed. Full measurements are in
[texture-volumes-comparison.json](texture-volumes-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
VOLUMES_DPR=2 npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js
```
