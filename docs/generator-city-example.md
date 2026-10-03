# City generator

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_generator_city` | 449 | `generator_city.rs`, `generator_city/` | CityGenerator's 24 towers, sidewalks and street furniture, the 10 × 7 × 8 LightProbeGrid baked ten probes per frame in two passes, the sun's shadow, 4× MSAA, BloomNode and the page's GUI |

The example is compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`).

## Generators

The generators run on the CPU when the seed changes, as the page's do
( `generator_city/city.rs`, `furniture.rs`, `prims.rs`, `geo.rs` ):

- CityGenerator's layout, its mulberry32 draws in the page's order, each
  lot's SkyscraperGenerator parameters and the furniture placements along
  every curb;
- SkyscraperGenerator with the city's pier, chamfer-corner and string-course
  parameters ( the building example's port, extended );
- SidewalkGenerator's rounded slab and curb ( ExtrudeGeometry with a hole,
  curveSegments 6 );
- the eight furniture generators, the three car bodies and the two
  pedestrian poses.

They keep three.js's arithmetic: the primitives' constructors
( CylinderGeometry, BoxGeometry, SphereGeometry, CircleGeometry,
RingGeometry, TorusGeometry, LatheGeometry, IcosahedronGeometry ),
BufferGeometry's transforms ( normals through the normal matrix and
renormalized ), computeVertexNormals, mergeVertices' hashing,
mergeGeometries, LoftGeometry and the Float32Array stores. Every attribute,
index and instance matrix matches the page's bit for bit ( checked for
seeds 94 and 57 ).

Each tower is one non-indexed draw. Each furniture generator is one
instanced draw over its placements, with the instance matrices in a
uniform array sized to createInstances' power-of-two capacity, as r186's
InstanceNode keeps them. The car bodies are bucketed by type in the order
they first appear, each with its paint attribute. Nothing streams after a
build.

## Frame

Each frame runs the page's animate():

1. FirstPersonControls by the timer's delta;
2. updateProbes(): while the bake runs, one row of ten probes. Each probe
   renders a 16² CubeCamera capture of the tower proxy ( one instanced box
   per tower ), the ground and the sky without its disc, then the SH
   projection into its batch row. The row's cells repack into the atlas
   and its padding. The direct pass covers the grid first. The bounce pass
   then starts from a snapshot of it ( copyTextureToTexture ), which its
   captures sample. The bake's shadow map renders the proxy once per call;
3. the sun's shadow of the city ( 4096², normal bias 0.05 );
4. the scene with 4× MSAA: the towers, the road, the sidewalks, the
   furniture and the sky, front to back as r186 sorts them, each frustum
   culled by its bounding sphere ( the instanced generators by their
   instances' union );
5. BloomNode at quarter resolution ( strength 0.05 ) and the ACES output
   ( exposure 0.13 ).

updateSun() places the sun for the time of day, fits the shadow camera
around the city box in light space and bakes the sky into the PMREM, as
the page does. A seed or time-of-day change restarts the bake.

Every stage runs the WGSL r186 generates for the page. The material
uniforms are recognized from how each captured shader uses its fields
( `generator_city/uniforms.rs` ).

## Comparison

With and without MSAA, at device pixel ratio 1 and 2, no pixel differs by
more than 6/255 in any capture: the bake's start and end, the GUI changes, a
second seed, the FirstPersonControls drag and the resize. The bake's start
and end, the exposure and the GI toggle match exactly. After a time-of-day
or seed change, scattered pixels differ by 1/255 ( mean error at most
0.02 ).

## Differences

- Turning global illumination off keeps the materials and sets the grid's
  intensity to 0 instead of recompiling them without the grid. The
  arithmetic adds the same zero; the grid's texture reads remain.
- The probe helper ( show probes, LightProbeGridHelper ) is not ported.
- The Inspector's panels are not ported.
