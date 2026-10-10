# webgl_renderer_pathtracer

The page path traces the LDraw X-wing ( 7140 ) with three-gpu-pathtracer
0.0.24 and three-mesh-bvh 0.9.10. The model sits on a metal floor with a
radial fade, lit by the blurred royal_esplanade UltraHDR environment, under
a gradient background. `src/browser/pathtracer.rs` and `src/browser/pathtracer/`
port it. `tests/browser/pathtracer.spec.js` compares it with the original
page, which the `texture-volumes.html` fixture runs on the example clock.

## The scene and its data textures

Loading follows the page:

1. LDrawLoader parses the model ( `ldraw_loader.rs` ). r186's `cloneResult`
   drops `doubleSided`, so the cloned faces are one-sided while `totalFaces`
   still counts both sides; the port keeps the zero tail this leaves.
2. LDrawUtils.mergeObject merges the meshes ( `scene.rs` ).
3. The page changes the materials. Every roughness is multiplied by 0.25.
   Translucent parts become MeshPhysicalMaterial with transmission 1,
   thickness 1, IOR 1.4 and an HSL lightness of at least 0.35.
4. The floor is sized to the model's bounds.
5. PathTracingSceneGenerator bakes and merges the geometry. Meshes are
   ordered by uuid, which the fixture renumbers in traversal order.
   setCommonAttributes adds zero uvs, computeTangents tangents and white
   colours.
6. A SAH MeshBVH is built over it ( `bvh.rs` ): 32 shared bins, one
   triangle per leaf, Float32 bounds and an indirect buffer.

The packed textures match the original's uploads byte for byte:

- index;
- position;
- the BVH bounds and contents;
- the four-layer attribute array;
- the material index;
- MaterialsTexture's 47 texels per material;
- the stratified samples and blue noise, which the fixture seeds with one LCG
  for both runtimes;
- the floor map;
- the gradient background;
- the UltraHDR source.

The test checks each hash.

## The environment

BlurredEnvMapGenerator renders the source's PMREM cube-UV top level and
copies it back to an equirect at blur 0. The port runs the same passes on
the GPU ( `render.rs` ) and reads the floats back as DataUtils.toHalfFloat
stores them. EquirectHdrInfoUniform's marginal and conditional tables are
then built on the CPU.

ANGLE and Dawn filter the cube-UV atlas with different rounding. Of the
read-back halfs, 73.5% are identical. The rest differ by at most 0.5%
relative ( a few units in the last place ). The test bounds this
difference.

## The shaders

`tools/pathtracer/convert.sh` regenerates the WGSL from the GLSL that r186
compiles for the page ( `glsl/` ), in three steps:

1. `glsl_to_vulkan.py` rewrites the GLSL into Vulkan-style GLSL 450. It
   moves uniforms into one block, flattens uniform structs, splits samplers
   from textures, and shares samplers so that the 17 samplers fit WebGPU's
   16 per stage. It also renames locals that shadow WGSL builtins.
2. naga 26 translates that into WGSL.
3. `wgsl_fixup.py` replaces aliased pointer arguments ( `inout result.a,
   inout result.b` ) with temporaries copied in and out. This is GLSL's
   inout semantics, which WGSL's alias analysis rejects.

Two choices keep WebGL's arithmetic:

- `isnan` and `isinf` test the bits, because WGSL may assume finite math.
- `pow` is `exp2( y * log2( x ) )`. That is how the reference GPU evaluates
  GLSL's pow: a negative base gives NaN. Dawn's exact pow instead returns a
  finite value, which kept refracted paths alive that the original
  discards. This was found by tracing the paths of the transmissive engine
  tips with the material's DEBUG_MODE instrumentation in both runtimes.

The variant without a backgroundMap ( FEATURE_BACKGROUND_MAP 0, for
transparentBackground ) is generated the same way.

## Rendering

WebGLPathTracer.renderSample follows the original:

- the render-scale update;
- the queued reset;
- three's Clock ( 100 ms render delay );
- one tile per frame;
- the recompile that skips the first update and each variant change;
- the fade over 500 ms once three samples exist;
- the ClampedInterpolationMaterial quad, tone mapped or linear.

PathTracingRenderer blends each tile into a float target with
NormalBlending. WebGPU cannot blend float targets, so a pass over the tile
computes the same products into a second target, and the tile is copied
back.

With a transparent background, the alpha mode runs as in the original:

- the sample renders without blending;
- BlendMaterial mixes it full-screen into one of the two blend targets;
- the renderTask swaps its local targets after each sample;
- the quad always reads blend target 1, as the original's `target` getter
  does.

Until the samples exist, and while the quad fades in, the page rasterizes
the scene. The port draws the same scene with the engine renderer: the
merged meshes with the page materials, the blurred environment, ACES
Filmic, and the gradient as `Scene::background_map`. The background map is
an equirect background of its own beside the environment, as
`scene.background = texture` is. It therefore also backs the transmission
pass, as WebGLRenderer's background does there.

## Interaction and GUI

- OrbitControls turns the camera. A change beyond OrbitControls' EPS resets
  the path tracer. Until the first move the camera is the exact page camera,
  in three's f64 operations.
- The GUI has enable, pause, toneMapping, transparentBackground ( with the
  page's checkerboard under the premultiplied canvas ), resolutionScale,
  tiles, and the floor's roughness and metalness ( updateMaterials ).
- The download button saves the canvas as drawn, un-premultiplied, as
  toDataURL does.

## Tolerance

The data are identical, but ANGLE and Dawn round the path tracer's float
math differently, so individual paths diverge into scattered noise. Across
the raster frames, the fade, the path-traced image and every GUI scenario,
the measured mean error is 0.18 to 0.34 per channel ( of 255 ). At most
0.46% of pixels differ by more than 32. The test allows a mean below 1.0
and under 1% of pixels above 32. Sample counts must match exactly.

The raster fallback's transmissive engine tips render a lighter orange than
the original's. That is the engine's transmission material.

No timing-parity claim is made. The steady state creates no GPU resources
and keeps geometry resident in the raster frames and the path-traced
frames ( the residency test ). The stratified samples are uploaded once
per sample, as the original uploads them.
