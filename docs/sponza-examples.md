# Sponza examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

Both pages load Sponza from glTF-Sample-Assets
(`https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/`)
through its model index. The model's files are under the CRYENGINE Limited
License Agreement, which does not grant redistribution. The ports therefore
fetch the same URLs at run time, as the pages do, and vendor nothing. Like
the originals, they follow the repository's `main` branch.

glTF images are requested together, as the browser's loaders issue them, so
Sponza's 69 textures download in parallel (about 6 s on a local network,
against the original's 5.6 s).

| Example | Id | Port |
| --- | --- | --- |
| `webgpu_lightprobes_sponza` | 481 | `lightprobes_sponza.rs` |
| `webgpu_vxgi_sponza` | 482 | `vxgi_sponza.rs` |

## webgpu_lightprobes_sponza

The port follows the three.js r187 page ( `7e89746a` ). Sponza is lit by a
shadowed SunLight ( 0xfff2dc, intensity 100, shadow far 50, two cascades of
2048² side by side ) under SkyMesh with ACES, with FirstPersonControls and
diffuse GI from a 10 × 7 × 7 LightProbeGrid ( 23.3 × 12.8 × 10.5 at
( -0.5, 6, -0.3 ) ). The page bakes 16 probes per frame, in two passes
( one bounce ). It draws through MeshletDrawingPipeline, the default of its
Pipeline control. Every stage runs the WGSL r187 generates for the page:
`gallery/src/browser/lightprobes_sponza/` ( the lit builds with GI and
shadows on or off, direct and with pulled vertices, the captures, the sky,
the shadow maps; `shaders.rs` lists each build's uniforms by meaning and its
bindings ) and `gallery/src/browser/meshlet/` ( the culling ); the SH,
repack and helper passes are lightprobes_complex's and the output pass
cubemap_dynamic's, which are the same.

- **Bake.** As r187's LightProbeGrid bakes ( `lightprobes.rs` ): per batch
  of 8 probes, one render of the 48 90° sub-cameras ( ArrayCamera ) into the
  192 × 256 face atlas, culled per sub-camera; the solid-angle SH projection
  into the batch rows; the repack of the probe range ( rows per Z slice )
  into the padded RGBA16F atlas. The shadow-casting SunLight is replaced by a
  directional bake light whose 2048² shadow covers the casters' sphere,
  rendered once per bake call; the direct pass adds the unlit bounce grid,
  the bounce pass the previous pass's snapshot.
- **MeshletDrawingPipeline** ( `gallery/src/browser/meshlet.rs` ):
  - MeshletGeometryUtils.createMeshlets runs at load, as the page runs it:
    the vertices are welded, six levels are simplified and each is split into
    clusters of 64 triangles ( padded with degenerate triangles ) by three's
    bundled meshoptimizer 1.1 modules ( `web/examples/assets/meshopt/`,
    called through JavaScript as the page calls them, so the clusters are the
    page's ). Nothing is baked.
  - One geometry is processed per projection; until every geometry a render
    list needs is processed, its objects draw through the regular pipeline,
    then its culling is created, sized for all of them.
  - Each render list ( the camera, each shadow cascade, the bake light ) has
    a culling: the objects' transforms, the clusters' spheres and cones, the
    batch and geometry tables, the work queues and the indirect arguments, in
    storage buffers that stay on the GPU. A projection runs one compute pass
    ( reset, object culling with the level of detail, cluster arguments,
    cluster culling — with normal cones in the camera view — and draw
    arguments ); each batch ( a material and vertex layout ) then draws its
    visible clusters with one drawIndirect of 192 vertices per cluster,
    pulled from the geometry pool.
  - The camera view occlusion-culls: the clusters visible in the last frame
    are drawn into a depth buffer, its hierarchical depth is reduced on the
    GPU ( kernels generated per view size, as three generates them ), and the
    other clusters are tested against it. It is skipped while it culls fewer
    than 30 % of the clusters, measured in the three projections before each
    read of the counters ( every 30 projections, without waiting for the
    GPU ). The reads grow the budgets that overflow and shrink the buffers to
    the first measure.
  - The double-sided alpha-tested materials, the sky, the probe helper and
    the probes' ArrayCamera captures draw through the regular pipeline, as
    MeshletDrawingPipeline leaves them.
  - Not ported, as the page doesn't use them: LOD objects, instanced meshes,
    position nodes, far materials, static bundle groups, the depth prepass
    and the debug view. The batch table is uploaded when it changes, where
    the original rewrites it every projection.
- **Default.** The Pipeline control's Default draws every mesh directly,
  culled on the CPU.

`tests/browser/compute-examples.spec.js` compares the captures after 250
frames ( the bake and the meshlets done ) at each state: the initial view,
Default and Meshlet, GI off and on, the probe helper shown and resized, a new
light azimuth, no bounce, a 4 × 7 × 7 grid and a second's walk forward
( FirstPersonControls' W key, whose window key events the gallery page
forwards, as for webgpu_vxgi_sponza ). With and without 4× MSAA, every
state matches at the default thresholds: at most 2 pixels differ
( 0.0008 % ), with a mean error of at most 0.0036. Shadows off, a new light
elevation and intensity, a 10 × 4 × 7 grid and two bounces match as well
( no pixel over the threshold ).

The frames' GPU work matches the page's frame by frame. Over the first 160
frames, each frame's compute passes, dispatches, render passes, indirect and
direct draws are the original's: the regular draws while the meshlets are
processed, the bake frames with the bake light's culling, the occlusion
culling until its first measure ( frame 53 ), then three projections of it
every 30 frames. The workload check compares a steady frame: 37 direct
draws ( the alpha-tested materials and the sky ), 66 drawIndirect ( 22
batches in the camera view and in each cascade ) and three compute passes of
five dispatches, on both sides ( `gpuWork` ). The original uploads its batch
tables each frame ( 1,584 bytes ); the port uploads nothing. The residency
test starts after 250 frames ( `warmup` ) and checks that frames create
nothing.

The gallery renders at `min( devicePixelRatio, 1.5 )`, as the page sets.

Not ported: the Inspector panel, the progress bar and the "Log Camera"
button. These have no effect on rendering.

## webgpu_vxgi_sponza

Sponza is lit by a shadowed DirectionalLight ( 0xfff2dc, intensity 100,
2048² map ) and a 0.5 AmbientLight under SkyMesh, with FirstPersonControls.
The page leaves the pixel ratio at 1, and so does the gallery.
VXGINode traces voxel cones for AO and indirect light, and TRAA resolves
the cones' noise. Every stage runs the WGSL that r186 generates for the page
( `gallery/src/browser/vxgi_sponza/` ):

- **Collection.** VXGIVolume's scene collector runs once on the CPU, as the
  addon does:
  - the triangles are split to the sub-voxel edge;
  - each triangle's albedo is read at its centroid from the texture's 64²
    downscale. The downscale is a high-quality canvas draw of an
    ImageBitmap ( premultiplyAlpha none ), which is how the addon draws the
    loader's bitmap.

  The 274,627 triangles and their albedo match the original's buffer.
- **Volume.** The triangles are voxelized on the GPU into a
  144 × 64 × 96 opacity volume and its mips. The light is injected through
  the shadow map, and one bounce goes into the radiance volume. This runs
  again only when the light or the injection settings change.
- **Frame.** Each frame runs the shadow pass, the pre-pass ( packed normals
  and velocity ), the cone tracer, the scene pass and TRAA. The cone
  tracer's AO and GI rotate with `frameId`, and the scene pass reads them in
  its lighting. TRAA then resolves the result. The shadow and camera passes
  cull each primitive by the bounding sphere GLTFLoader gives its geometry
  ( the POSITION accessor box's center and half diagonal ), as three does,
  so both sides draw the same primitives.
- **GUI.** The GUI follows the page's order. Its output views follow
  `updatePostprocessing()`:
  - Direct renders the scene pass without the GI context, so the GI pass
    and its volume update do not run;
  - AO and GI show the effect alone;
  - turning temporal filtering off shows the scene pass without TRAA.

  The cone, AO, GI, bounce and light settings apply.

Not reproduced: the voxel views ( radiance and opacity, and their level )
and directional radiance. They return an error. The Inspector panel and the
progress bar are not ported either; they have no effect on rendering.

### Determinism and the comparison

The voxelize kernel ORs each triangle's coverage into the voxel's
occupancy. It then stores the triangle's id into the voxel without
ordering, so one of the voxel's triangles wins. That winner sets the voxel's
albedo and normal for the injection, and it changes between runs of the
original itself.

After 64 frames, the original's combined view differs from another run of
itself by up to 10.5 % of the pixels ( mean error up to 2.1 ). The port keeps the
same kernel and the same race.

`tests/browser/compute-examples.spec.js` ( `serialVoxels` ) rewrites that
store to an `atomicMax` of the id in both runtimes' kernels, when the shader
module is created. With the winner fixed, the comparison holds the default
thresholds after 64 frames at each state:
- the initial view;
- the AO, GI, Direct and Combined outputs;
- temporal filtering off and on;
- GI intensity 0 and back;
- 6 cones and back;
- 0, 2 and 1 bounces;
- a new light azimuth;
- a resize.

Every state differs by at most 0.11 % of the pixels. The residency test
checks that frames and parameter changes create nothing.
