# OffscreenCanvas worker, volume fire, batched LOD BVH, LDraw and CSG

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgl_worker_offscreencanvas` | 444 | `offscreen.rs`, `web/gallery/offscreen-worker.js` | The page's scene.js on the main thread and on an OffscreenCanvas in a module Worker, with the main-thread jank button |
| `webgpu_volume_fire` | 445 | `volume_fire.rs` | The 100 × 100 × 200 fluid fire's compute kernels per simulation step, the volume's colored spot shadow, the ray-marched volume, its denoise and BloomNode |
| `webgl_batch_lod_bvh` | 446 | `batch_lod_bvh.rs` | 500,000 torus knot instances in one BatchedMesh with four meshoptimizer LODs, BVH frustum culling, LOD selection and hover ray casts each frame |
| `webgl_loader_ldraw` | 447 | `ldraw.rs`, `ldraw_loader.rs` | The seventeen packed LDraw models parsed at load, with conditional lines, building steps and the page's GUI |
| `webgl_geometry_csg` | 448 | `geometry_csg.rs`, `csg_eval.rs` | three-bvh-csg's subtraction, intersection or addition of two turning brushes on the CPU each frame |

`webgpu_volume_fire` is compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`). The other four have no r186 WebGPU
counterpart and are compared against WebGLRenderer
(`tests/browser/texture-volumes.spec.js`).

## Engine additions

- **Per-node draw ranges** ( `Node::draw_range` ): a node draws its own
  index range of a geometry it shares with other nodes, intersected with the
  geometry's draw range. Nodes sharing one geometry share its GPU buffers.
  The batched LODs and the LDraw line groups use it.

## Port notes

### OffscreenCanvas worker ( 444 )

The left canvas is the gallery application on the main thread. The right one
is transferred to a module Worker that runs a second renderer from the same
package on its own animation frames, as the page's worker does. START JANK
blocks the main thread ( ten million Math.random calls every 1/60 s ), which
stops only the left canvas. Under the gallery clock both read the gallery time
as Date.now.

### Volume fire ( 445 )

Each simulation step runs the page's seven compute kernels in their order with
the step's uniforms written first, as renderer.compute submits them:

1. velocity advection ( buoyancy, curl-noise turbulence, the teapot's wind );
2. divergence;
3. two Jacobi pressure iterations;
4. the projection;
5. the dye advection;
6. the emission from the teapot's vertices.

The dye grids ping-pong after each step. The curl noise is computed once at
load. The steps follow the page's accumulator ( 1/120 s × the simulation
speed, at most a 1/30 s frame ) on the example clock, and only frames that
advance the clock run animate(): a re-render for pointer input runs no steps.

Each frame then draws:

- the volume caster's colored spot shadow map;
- the scene ( the lava teapot, the floor and the caster's colorless pass );
- the half-resolution ray-marched volume and its Gaussian denoise;
- BloomNode and the ACES output.

Every stage runs the WGSL r186 generates for the page. The floor's material
is static, so three writes its object uniforms ( the fire light's inputs ) at
its first render only. The port keeps them, as the page does.

DragControls move the teapot with a CPU ray cast against its triangles.

The page never sets a pixel ratio, so it renders at CSS pixels on any
display. The gallery renders this example at CSS pixels as well.

### Batched LOD BVH ( 446 )

The LODs are baked by `tools/tsl/prepare-batch-lod.mjs`. It runs
@three.ez/simplify-geometry 0.0.1's simplifyGeometriesByErrorLOD with
meshoptimizer 1.1.1, the versions the page loads. Only the index changes:
the port builds the knots at load and checks each knot's position hash
against the bake.

The instance data follows the fixture's seeded Math.random, including the
eight draws the colors texture's two UUIDs take at the first setColorAt.

Each frame, as @three.ez/batched-mesh-extensions' onBeforeRender does:

- bvh.js frustum culling, with the single-precision planes and instance
  boxes ( or per-instance bounding spheres with useBVH off );
- the screen-size LOD metric;
- the front-to-back order.

The matrices and colors stay in GPU instance buffers. The port draws each
knot and LOD's visible instances with one instanced draw, where WebGL issues
one multi-draw entry per instance. The draw order is grouped by LOD as a
result.

MapControls grab the ground under the pointer, as r186's do. Hover ray casts
test the instance boxes and the knots' LOD 0 triangles ( three-mesh-bvh's
first hit, front faces ) on the CPU. They use the camera before the same
move's pan, as the page's document listener runs before MapControls'.

### LDraw ( 447 )

LDrawLoader is ported ( `ldraw_loader.rs` ):

- the line parser and !COLOUR materials with their edge and conditional-edge
  materials;
- BFC winding and the subobject hierarchy: parts become groups, primitives
  merge into their part;
- the smoothed normals across soft edges;
- createObject's per-color groups and the building steps.

The results depend on JavaScript semantics, which the port keeps: the
half-edge map's insertion order, the shared normal wrappers, and the stable
color sort. The packed models are served from `web/gallery/assets/ldraw/`.

LDrawConditionalLineMaterial's vertex test runs as an engine shader program.
The control points and direction travel in the vertex's normal, tangent and
UV slots. The page renders into a half-float output buffer, so the gallery
tone maps and encodes this example on presentation, as for 442.

Merging hides the model, as on the page: the merged group has no
numBuildingSteps, so the building step becomes NaN. The port builds no merged
geometry.

### CSG ( 448 )

three-bvh-csg 0.0.18's Evaluator with its default LegacyTriangleSplitter is
ported ( `csg_eval.rs` ), together with the three-mesh-bvh 0.9.10 parts it
uses:

- the MeshBVH build: CENTER split, three triangles per leaf, an indirect
  buffer and Float32 bounds;
- bvhcast's alternating traversal and raycastFirst;
- ExtendedTriangle's intersection test;
- the half-edge map, the triangle splitter, the hit-side classification and
  GeometryBuilder's Float32 attributes.

One library behaviour carries over: the split triangles' blended normals are
not renormalized. The library's Vector4 gets an undefined w, so its length is
NaN and `normalize()` divides by 1. The result's geometry keeps its identity,
so each frame's output is written into the same resident buffers.

## Comparison tolerances

- **Volume fire**: the emission kernel stores from every teapot vertex, and
  vertices sharing a voxel race. The original differs from itself by 1.2–5.8 %
  of the pixels ( mean 0.15–1.4 ) over the spec's states. The port is bounded
  at 10 % and a mean of 3. Its early states match the original's within 0.13 %
  before the race grows, and the uniforms written per step are identical.
- **4× MSAA edges** of sub-pixel geometry and one-pixel lines:
  - the far batched knots ( 12 % );
  - the LDraw lines ( 3.5 % );
  - the CSG wireframe ( 2 % ).

  Without MSAA these match at the default thresholds.

## Workload

- The batched knots' draws match WebGL's multi-draw entries one for one:
  29,703 at 1280 × 720. The workload test counts WEBGL_multi_draw entries and
  expands the port's instanced draws per instance.
- The offscreen page's main-thread scene renders on its own animation
  frames. The workload holds the clock at 0 so that a frame drawn before the
  last request turns the group the same way.
- The CSG result streams each frame on both sides. The port's interleaved
  vertex is 80 bytes against WebGL's 32 ( position, normal and UV ), so its
  stream is bounded at 2.5 times the original's.
- Showing the CSG wireframe builds its line index again, as WebGL's
  wireframe attribute is. The residency cycle leaves that parameter out.
- LDraw model, flat-color, merge and smoothing changes reload and parse the
  model, as the page does. The residency cycle leaves those parameters out.

## Not ported

- LDraw's merged geometry ( the merged model is hidden on the page ).
- Stats panels and the Inspector's GUI chrome.
