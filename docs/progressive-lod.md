# webgl_loader_gltf_progressive_lod

The page loads three Needle Cloud models through GLTFLoader with
@needle-tools/gltf-progressive 3.2.0: the floating world, the airship and the
knight. They stand under linear fog and the quarry_01 environment, rotated
−π/2 about Y. `src/browser/progressive_lod.rs` and
`src/browser/progressive_lod/progressive.rs` port it. The models and their
LOD files are fetched from cloud.needle.tools at run time, as the page fetches
them, with the package's `Accept: */*;progressive=allowed;usecase=default`
header.

## What the package does, and the port

- **Registration ( afterRoot ).** Every mesh and texture with a
  NEEDLE_progressive extension gets LOD information: the model directory, the
  guid key, the level ( the LOD count ) and the primitive index. The
  low-resolution geometries and textures stay cached under their keys. Meshes
  with LODs register their low-resolution geometry as the raycast mesh, which
  the skinned bounds later use. Materials are shared as GLTFLoader shares them,
  per glTF material and its derivative-tangent, vertex-colour and
  flat-shading variant. A material's texture LOD state and its min/max LOD
  table therefore belong to every primitive that uses it.
- **The render list.** After every render, LODsManager walks the render list.
  The port builds that list as WebGLRenderer.projectObject does: visible
  meshes whose bounding sphere meets the frustum ( WebGL clip space ), with a
  SkinnedMesh's own sphere computed once from its skinned vertices. The list
  is sorted opaque, transparent, then transmissive, by material, depth and
  object.
- **The update interval.** The update interval follows the package's FPS
  buffer on the example clock.
- **calculateLodLevel.** This is ported in the package's f64 operations:
  - the box through the world and projection-view matrices, and the near-face
    test;
  - the coverage scaled by canvas height over screen.availHeight and by the
    aspect;
  - the depth term from the view-space box;
  - the mesh level, the first LOD whose primitive density over the coverage
    is below 200,000;
  - the texture level, the lowest-resolution LOD taller than the projected
    pixel size, starting from the lowest level on the first update.
- **Skinned bounds.** Skinned meshes recompute their box every 30 frames from
  the raycast geometry, CPU-skinned as SkinnedMesh.computeBoundingBox does.
  Each mesh's frame offset is assigned in encounter order, keeping the
  package's reassignment of offset 0.
- **getOrLoadLOD.** Levels past the last return the low-resolution cache at
  once, applied after the update loop as microtasks are. Other levels wait for
  a slot of the 50-wide PromiseQueue, which is granted on the next tick before
  the next render. Loads are cached by URL and guid.
- **Applying a LOD.** The LOD file ( `?v=` hash ) is parsed with the engine's
  Draco, KTX2 and WebP decoding, and its mesh or texture is found by guid.
  - A geometry replaces the mesh's geometry while that level is still the
    requested one.
  - A texture is copied with the current texture's sampler, colour space and
    transform ( copySettings ). It replaces the slot unless the assigned
    texture is already of a higher level.

The importer gained two things for these assets:

- TEXCOORD_2 as a third UV channel. The knight's material samples its normal
  map with texCoord 2.
- Zero-filled accessors without a bufferView. The world contains one.

## Comparison

`tests/browser/progressive-lod.spec.js` runs the original page and the port
on the example clock, waiting for the network to settle between frame
ranges. Two scenarios are compared: the page camera, and an orbit drag after
the first LODs arrived. In both, the 39 meshes show identical mesh LOD
levels, vertex counts and per-slot texture LOD levels. The frames match with
a mean error of about 0.19 per channel ( of 255 ) and 0.1% of pixels over 32.
The steady state after the LODs settle creates no GPU resources and uploads
no geometry.

Which frame a LOD appears in depends on network latency in both runtimes, so
only the settled state is compared. Mesh names keep their dots; GLTFLoader's
sanitized names drop them.

No timing-parity claim is made.
