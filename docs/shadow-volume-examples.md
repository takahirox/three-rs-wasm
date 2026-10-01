# Cascaded shadows, instanced morphs, volumetric lighting, god rays, test memory, PCSS, TSL instancing and the Draco exporter

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_shadowmap_csm` | 374 | `shadowmap_csm.rs` | 80 Phong boxes over the floor, CSMShadowNode with four 2048² cascades (uniform, logarithmic and practical splits, texel snapping, margin), the CSMHelper, the orthographic camera and every control |
| `webgpu_instancing_morph` | 375 | `instancing_morph.rs` | 1,024 instanced horses with per-instance morph influences written each frame from the glTF track, 15 resident morph targets, SunLight's two cascades in one atlas, hemisphere light, fog and the circling camera |
| `webgpu_volume_lighting` | 376 | `volume_lighting.rs` | The turning teapot and floor, the point light's cube shadow and the spot light's projected colors.png shadow, the quarter-resolution VolumeNodeMaterial ray march with the 128³ ImprovedNoise smoke and Bayer dither, the Gaussian denoise and NeutralToneMapping |
| `webgpu_postprocessing_godrays` | 377 | `godrays.rs` | godrays_demo.glb with its backdrops and the pink point light's 2048² cube shadow, GodraysNode at half resolution, the bilateral blur and depthAwareBlend |
| `webgpu_test_memory` | 378 | `memory_outline.rs` | A new random sphere and canvas texture every frame (mipmapped on the GPU), disposed with the previous ones; the shadowed Lambert plane; OutlineNode's depth, mask, edge, blur and composite passes |
| `webgpu_volume_lighting_rectarea` | 379 | `volume_rectarea.rs` | The torus knot over the checkered floor, three turning RectAreaLights with their panels evaluated with the LTC tables, and the volumetric pass of 376 over the three area lights |
| `webgpu_volume_lighting_traa` | 380 | `volume_traa.rs` | 376's scene with the full-resolution additive volume in the scene pass, the depth pre-pass, color and velocity MRT, interleaved-gradient dithering on the Halton sequence, TRAA and NeutralToneMapping |
| `webgl_shadowmap_pcss` | 381 | `shadowmap_pcss.rs` | Twenty bouncing Phong spheres, the column and ground in fog under the BasicShadowMap directional light, and its CameraHelper |
| `webgl_tsl_instancing` | 382 | `picking_buffers.rs` | The INSTANCED, MERGED and NAIVE suzanne builds with their count, MeshPhongNodeMaterial's position- and time-driven colorNode, ambient and directional lights and auto-rotation |
| `misc_exporter_draco` | 383 | `exporters_matcap.rs` | The torus knot on the shadowed, fogged ground, and Export DRC with DRACOExporter's defaults |

374–380 are WebGPU examples compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`). 381 and 382 are WebGL-only
examples (`texture-volumes.spec.js`, `picking-buffers.spec.js`); 383 is a
misc example (`exporters-matcap.spec.js`).

## Port notes

### Generated WGSL (374–380)

As in [compute-examples.md](compute-examples.md), the WGSL three.js r186
generates for each page was captured from `createShaderModule` on the
reference page, with the pipelines, bind groups, passes and uniform writes.
The modules run unchanged with automatic layouts. Uniform structs are packed
by name from each module's own struct. 376's shadows, noise texture, Bayer
texture and spot map are shared by 379 and 380, and 376's blur and output
modules by 379.

### Cascaded shadows and instanced morphs

- **CSM.** The cascade splits, the CSMFrustum corners, the cascade cameras
  and their texel snapping are computed on the CPU each frame, as
  CSMShadowNode does. Each cascade's pass is culled against its own frustum.
- **Pixel ratio.** The CSM and TRAA pages never call `setPixelRatio`; both
  render at a pixel ratio of 1 on any display, as the ports do.
- **Morphs.** The 1,024 influence rows are evaluated from the glTF track and
  written to the influence texture each frame, as `setMorphAt` and
  `morphTexture.needsUpdate` do. The vertex stage blends the 15 resident
  targets.

### Volumetric lighting, god rays and test memory

- **Volumes.** Each resolution scale keeps its own volumetric targets, so
  changing the resolution and back creates nothing new.
- **God rays.** Front-side casters render back faces into the cube shadow,
  double-sided ones both.
- **Test memory.** Each frame builds the sphere from five draws of the seeded
  Math.random, as animate() does. Recreating the light draws two more. A
  resize keeps the current sphere and rebuilds only its bind groups, so the
  random sequence advances once per animate() as in the original. OutlineNode's
  blurs take their texel size from the downsampled mask, as the generated
  shader does.

### TRAA (380)

- **Jitter.** TRAANode offsets the camera by the 32-sample Halton sequence
  for the pre-pass and scene pass. The frame that builds the output node
  renders unjittered, since its before-pipeline hook registers during that
  frame. Switching TRAA rebuilds the output node and repeats this.
- **Velocity.** VelocityNode's previous projection, view and model matrices
  are the last frame's (the current ones on the first frame); the current
  projection is the unjittered one.
- **History.** A new history starts from the beauty buffer. The resolve
  reads TRAANode's 1 × 1 placeholder depth for its first two resolves and the
  previous frame's pre-pass depth afterwards.
- **Start-up frames.** In r186 the first resolve renders into TRAANode's
  targets before they take the drawing-buffer size: the original shows one
  flat 1 × 1 frame and starts from a different history. The port starts from
  the beauty buffer, as TRAANode intends. The histories converge (under 0.5%
  of pixels by frame 41), so each capture renders 60 frames.
- **Frames.** Only requested frames (a seek or animate()) advance TRAA,
  the jitter and the dither offsets; other redraws present the last frame
  again.

### Rect-area volumes (379)

The LTC tables are the crate's r186 RectAreaLightTexturesLib data, as two
layers of one 64 × 64 texture (32-bit float where filterable, else half
float).

### PCSS (381)

The page splices its PCSS functions into `shadowmap_pars_fragment`, but its
second replacement (the call inside getShadow) looks for
`if ( frustumTest ) {` directly followed by the depth read, while r186's chunk
has a blank line between them. The replacement does not apply and the
original draws the plain BasicShadowMap comparison, so the port does too. The
CameraHelper keeps the projection it was built with (far 500), since the page
changes `far` without updating it.

### TSL instancing (382)

- **colorNode.** The colorNode runs as the crate's Phong diffuse hook, with
  TSL time (the example clock) as a material uniform.
- **Output.** WebGLNodesHandler's output node converts to sRGB and the
  WebGLRenderer program converts again. The original's lit colors are
  therefore sRGB-encoded twice, and the port's output hook does the same.
- **Instanced rebuilds.** In r186, an InstancedMesh rebuilt after `clean()`
  draws nothing under WebGLNodesHandler. The port draws it. The test changes
  the method and count only through the merged and naive builds.
- **Fixture.** The fixture times WebGLNodesHandler's NodeFrame on the example
  clock.

### Draco exporter (383)

The export follows draco_encoder.js 1.5.7 as DRACOExporter drives it:
MeshBuilder's position, faces, normal and uv in local space, the wrapper's
attribute-value and point deduplication, EdgeBreaker at speeds 5 / 5, and
16-bit positions with 8-bit normals and texture coordinates. The `draco-core`
crate encodes it and the file matches the original byte for byte. The
reference fixture loads the page's draco_encoder.js from jsDelivr.

## Comparison

Captures use the page's fixture clock, seeded Math.random, the same controls
and a 640 × 400 resize. The thresholds are the defaults (0.5% of pixels over
6/255, mean 0.6/255) except:

- **WebGL MSAA.** 381 and 382 compare against WebGLRenderer's MSAA resolve
  at 1.5% / 0.4 and 10% / 2.1. Without MSAA both pass the default thresholds
  (under 0.04% of pixels).
- **TRAA.** 380 compares converged frames (60 per capture), as above.
- **Exporter files.** The exported DRC must match byte for byte.

## Performance evidence

- **Residency.** Warmed cycles of time, controls, parameters and resize create
  no GPU resources. The exception is test memory, whose page creates and
  disposes a sphere and texture every frame: its resident set stays flat and
  its only texture upload is the frame's 256 × 256 canvas.
- **Draw workload.** The draws match the original's in counts and instances.
- **Uploads.** No vertex, index or storage data is written in steady frames.
  - The instanced horses' influence texture is rewritten each frame, as the
    original's is.
  - Test memory uploads its new sphere and canvas each frame, as the original
    does.
  - Uniforms are the only other per-frame writes.
- **Assets.** Models, images and the noise texture are decoded and uploaded
  once.

No GPU timing parity is claimed.

```sh
npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
COMPUTE_DPR=2 npx playwright test -c playwright.gallery.config.js compute-examples.spec.js
npx playwright test -c playwright.gallery.config.js texture-volumes.spec.js -g pcss
npx playwright test -c playwright.gallery.config.js picking-buffers.spec.js -g tsl_instancing
npx playwright test -c playwright.gallery.config.js exporters-matcap.spec.js -g draco
```
