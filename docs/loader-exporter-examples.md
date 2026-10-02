# USDZ exporter, shadow-map performance, GTAO, subdivision, BVH raycasting, spline editor, glTF exporter, arcball, video frame, G-code exporter, SVG, FBX, FPS game, random UV, transmission alpha, webcam, 3DM, IFC, watch and Lottie

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `misc_exporter_usdz` | 424 | `usdz_exporter.rs` | The Draco / KTX2 CarbonFrameBike and its looping animation under the RoomEnvironment PMREM, ACES and damped OrbitControls |
| `webgl_shadowmap_performance` | 425 | `shadowmap_performance.rs` | 601 morph-animated horses in 25 AnimationObjectGroups, the extruded text and blocks under the SunLight's PCF cascades, fog and FirstPersonControls |
| `webgl_postprocessing_gtao` | 426 | `gtao_webgl.rs` | Littlest Tokyo through RenderPass, GTAOPass ( normal and depth pass, GTAO, Poisson denoise and the six outputs ) and OutputPass |
| `webgl_modifier_subdivision` | 427 | `subdivision.rs` | three-subdivide's LoopSubdivision on 15 geometries beside the source, on parameter changes |
| `webgl_raycaster_bvh` | 428 | `raycaster_bvh.rs` | three-mesh-bvh's MeshBVH on the bunny and up to 3000 rays cast each frame, with the instanced hit spheres and ray lines |
| `webgl_geometry_spline_editor` | 429 | `spline_editor.rs` | Three CatmullRomCurve3 outlines through draggable boxes, TransformControls, and the spot light's box and line shadows on a ShadowMaterial plane |
| `misc_exporter_gltf` | 430 | `exporter_gltf.rs` | The export test scene ( polyhedra, bump-mapped sphere, hierarchy, lines, points, instancing, quantized and meshopt models, canvas texture ) circled by the camera |
| `misc_controls_arcball` | 431 | `arcball.rs` | ArcballControls around the Cerberus OBJ with its gizmos, animations, limits and clipboard state over the page's gradient |
| `webgpu_video_frame` | 432 | `video_frame.rs` | sintel.mp4 demuxed in Rust into a WebCodecs VideoDecoder, each VideoFrame copied into the plane's texture |
| `misc_exporter_gcode` | 433 | `exporter_gcode.rs` | The Z-up print bed with the selected primitive placed on the XY plane |
| `webgl_loader_svg` | 434 | `svg_loader.rs` | SVGLoader's walk on the browser's DOMParser for the 35 files, with fills, strokes, gradients and render order |
| `webgl_loader_fbx` | 435 | `fbx_loader.rs` | The 14 FBX models as FBXLoader leaves them, their clips and morphs, under the SunLight cascades |
| `games_fps` | 436 | `games_fps.rs` | The Octree world, the capsule player and a hundred spheres in five physics substeps per frame, with VSM shadows |
| `webgl_random_uv` | 437 | `random_uv.rs` | The Draco ShaderBall2 with textureNoTile and the dissolve as TSL surface graphs, VSM shadows on a ShadowMaterial ground |
| `webgl_materials_physical_transmission_alpha` | 438 | `transmission_alpha.rs` | DragonAttenuation's transmissive dragon over its cloth, through an alpha canvas onto the page's table |
| `webgl_materials_video_webcam` | 439 | `webcam.rs` | 128 planes on a Fibonacci sphere sharing the webcam stream as their map |
| `webgl_loader_3dm` | 440 | `loader_3dm.rs` | Rhino_Logo.3dm as Rhino3dmLoader builds it, with the layer toggles and canvas text dots |
| `webgl_loader_ifc` | 441 | `ifc_loader.rs` | The Revit sample project as web-ifc streams it, merged into the opaque and transparent meshes at load |
| `webgl_watch` | 442 | `watch.rs` | The rolex under Neutral tone mapping from a half-float buffer, the camera tween and the clock's hands |
| `webgl_loader_texture_lottie` | 443 | `lottie.rs` | The Lottie logo animation drawn on a canvas each frame and uploaded as a rounded box's map |

`webgpu_video_frame` is compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`). The other nineteen have no r186
WebGPU counterpart: the misc pages, games_fps and the WebGL-only examples are
compared against WebGLRenderer (`tests/browser/texture-volumes.spec.js`).
`webgl_tsl_skinning`, `webgl_tsl_shadowmap` and `webgl_tsl_clearcoat` were
considered and left out: they render the same scenes as `webgpu_skinning`,
`webgpu_shadowmap` and `webgpu_clearcoat`, which are already listed.

## Engine additions

- **ShadowMaterial** ( kind 9 ): the received shadow as alpha over the
  material color, for the spline editor, random UV and their grounds.
- **Bump maps** on MeshStandardMaterial ( `bump_map`, `bump_scale` ): the
  height map's screen-space slope perturbs the normal, as perturbNormalArb
  does, for the glTF exporter's sphere.
- **VSM shadows** ( `ShadowFilter::Vsm` ): the depth moments rendered into
  an RG16F array with the receivers drawn, blurred vertically and
  horizontally with `blur_samples` taps over the shadow radius, and read
  with the Chebyshev bound and three's 0.3 / 0.65 remap. games_fps and
  random UV use it.
- **Line shadow casters**: Line and LineSegments draw into shadow maps, as
  the spline editor's outlines cast.
- **Transmission alpha**: the transmission sample's alpha gives
  transmissionAlpha ( 1 − ( 1 − sample alpha ) × mean transmittance ), and
  the transmission pass clears to white at alpha 0.5 ( premultiplied ) when
  the clear alpha is below one, as WebGLRenderer's transmission pass does.
  Opaque backgrounds keep alpha one, so existing transmission scenes are
  unchanged.
- **envMapIntensity** on MeshStandardMaterial ( `env_map_intensity` ).
- **RoundedBoxGeometry uvs**: the shared rounded box now carries the
  addon's uvs, spread over each side's arcs and plane.
- TransformControls gained `detach()`.

## Port notes

### Baked loader output

Where a page parses its asset with a library that is not three.js, or with a
loader whose output the port renders unchanged, the output is baked by a
tool that runs the pinned code and stored beside the asset's source hash:

- `tools/tsl/prepare-fbx-loader.mjs`: FBXLoader on the 14 FBX models ( node
  tree, meshes, morphs, materials, skins and clips ).
- `tools/tsl/prepare-3dm-loader.mjs`: rhino3dm 8.32.1 ( the version the page
  loads ) decoding Rhino_Logo.3dm at the loader's subdivision level 3, and the
  loader's `_createGeometry`. Text dots keep their canvas parameters; the port
  draws them on a canvas at load as the loader does.
- `tools/tsl/prepare-ifc-loader.mjs`: web-ifc 0.0.77 ( the version the page
  loads ) streaming the model with COORDINATE_TO_ORIGIN: each distinct
  geometry's vertex data and index, and every placement's geometry, colour and
  transformation in stream order. The port runs the page's loadAllGeometry on
  them at load ( the colour conversion, applyMatrix4 and mergeGeometries ).

The libraries themselves ( FBX, OpenNURBS, web-ifc ) are not ported.

### Libraries ported to Rust

three-subdivide's LoopSubdivision ( 427 ), three-mesh-bvh's MeshBVH build and
raycast ( 428 ), the Octree and Capsule addons ( 436 ), SVGLoader's walk and
stroke builder ( 434 ), ArcballControls ( 431 ) and the subset of lottie-web's
canvas renderer the animation uses ( 443 ) run in Rust, with the libraries'
precision ( Float32 storage where they store Float32 ).

### Random UV ( 437 )

The page patches MeshStandardMaterial's GLSL with onBeforeCompile. The port
expresses the same code as TSL surface graphs of WGSL functions: textureNoTile
( the noise texture or three's `rand` value noise picks two offset virtual
patterns, sampled with the base uv's gradients ) and the dissolve that replaces
the alpha with the shaderball_ds ramp. The shadow pass is unchanged, as the
patches leave it.

### Transmission alpha ( 438 )

The canvas is transparent. The gallery page carries the example's grey body
and coloured table behind it, and the fixture adds the same markup. The
dragon's transmission pass clears to WebGL's white at alpha 0.5, so where the
dragon refracts nothing the page shows through by transmissionAlpha. The
fixture points its capture back at the page's scene after the first render's
PMREM pass.

### Webcam ( 439 )

The gallery opens the camera into `#video` with the page's constraints, and
each newly presented frame is copied into the texture, as VideoTexture uploads
it. The texture takes the first frame's size. The tests give both pages the
same fake camera: getUserMedia returns a canvas stream of a fixed pattern.

### Watch ( 442 )

The page renders into a half-float output buffer and tone maps in the output
pass, so the additive glass blends in linear light. The gallery target for
this example is half-float and is tone mapped and encoded on presentation.
The tween and the hands follow the gallery clock in still mode ( a fixed date
plus the gallery time ); live, they follow the local time. The page's
optional TAA and UnrealBloom effects ( off by default ) are not ported.

### Lottie ( 443 )

lottie-web draws each frame on a canvas that the page uploads as the box's
map. The port draws with the browser's Canvas 2D in the same order:

- the layer matrix on the context and the group matrix on the points;
- `floor( c × 255 )` colours;
- the trim's 150-sample float32 length tables and three-decimal segment
  points;
- BezierEasing keyframes;
- the alpha-inverted track matte through two buffer canvases.

The canvas is 500 × 500 at the square of the device pixel ratio, as the page's
container and renderer settings make it. Features the file does not use are
rejected with an error rather than approximated.

## Comparison tolerances

Backend tolerances are scoped in the spec next to each example:

- **Edges** of thin geometry and shadows that rasterize, or resolve under
  4× MSAA, differently: the bike frame, subdivision wireframes, rays, spline
  lines, the glTF scene, Cerberus, SVG, FBX, the FPS world, the webcam planes,
  the 3DM curves and the IFC mullions.
- **Blurred backgrounds**: the blurred lobe background's gradient bands differ
  by a level ( random UV ).
- **Image-based lighting**: WebGL's PMREM and IBL multiple scattering light
  surfaces a few levels apart:
  - the transmission alpha cloth and dragon, over most of the frame once the
    dragon fades;
  - the watch's gold and silver once the GUI roughens them;
  - the Lottie box's mirror face.
- **Coplanar faces**: the IFC model's coplanar faces z-fight a few pixels
  apart.
- **Shadow edges**: the SunLight's PCF edges in the shadow-map performance
  scene.

The GTAO, G-code exporter and video frame ports match at the default
thresholds.

The existing `webgpu_instancing_morph` comparison gets a 4× MSAA bound of
0.6 % of the pixels. At device pixel ratio 2 its 1,024 horses' edges reach
0.51 %, and the previous commit's build measures the same.

The draw workload matches the originals' draws, with two scoped differences:

- At 1280 × 720, one horse at the near cascade's boundary falls inside the
  port's cascade frustum and outside WebGL's. The port draws it once more.
- WebGL draws the glTF exporter's line loop as a five-vertex LINE_LOOP. The
  port closes it as a six-vertex strip.

The USDZ page exports its file once at load, and the decompression draws come
from that export. The fixture waits for the export before measuring, and the
ray caster and the FPS game advance their clocks as their fixtures do.

## Not ported

- The exporters' buttons ( USDZ, glTF and G-code with Polyslice ).
- The watch's TAA and bloom.
- The FBX page's dynamic per-model GUI ( replaced by fixed clip and morph
  controls ).
- ArcballControls' multi-touch gestures.
- Stats panels.
