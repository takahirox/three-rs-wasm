# Official Three.js cube demo

`crate.gif` is copied unchanged from Three.js r186, commit
`148ef33ecb6d2502ff796d4554abd1549c95d519`, `examples/textures/crate.gif`.
The corresponding source example is:
https://github.com/mrdoob/three.js/blob/148ef33ecb6d2502ff796d4554abd1549c95d519/examples/webgl_geometry_cube.html

Three.js attribution and MIT terms are in ../LICENSE-THREE.

`/web/?example=2` ports the unit BoxGeometry, MeshBasicMaterial with sRGB texture,
70-degree PerspectiveCamera (near 0.1, far 100, z=2), and per-frame Euler rotations
(x += 0.005, y += 0.01) to Rust/Wasm. Click-to-pause is an additional demo control.
The original fullscreen WebGL renderer becomes a 512x512 WebGPU canvas, with
antialiasing and mipmaps disabled to match the current renderer. No claim of
pixel identity with the fullscreen, antialiased original is made.

`?example=2&static=1` uses rotation (0.4, 0.7, 0) for deterministic comparison.
The browser migration test compares this view against pinned Three.js with the
same viewport, sampling and rotation, using the existing fixed image tolerance.

# Point Lights sculpture

`models/WaltHead.obj` is copied unchanged from the same pinned Three.js r186
archive (`examples/models/obj/walt/WaltHead.obj`). Model credit: David OReilly;
displacement effect: oosmoxiecode, as credited by the official example:
https://threejs.org/examples/webgpu_lights_pointlights.html

The port is `/web/?example=3`, implemented in `src/browser/point_lights.rs`.
It retains all 16,160 input triangles, expands them into 64,640 tetrahedron faces,
and reproduces both point-light trajectories, intensities, camera and per-vertex
distance/sine displacement. The original TSL displacement runs on the CPU in
Rust; MeshPhongNodeMaterial becomes MeshStandardMaterial (roughness 0.4), so the
specular response differs from the official example. Random per-face phases use
a fixed seed for reproducible comparison. UI and orbit controls are adapted.
It renders at 720x720 without MSAA. The official example enables antialiasing;
this port uses the same non-MSAA sampling as its Three.js comparison, since the
two renderers currently produce different MSAA edge resolves. No shadows or post-processing were added.

`reference/three-js/point-lights.html` runs pinned Three.js with the same adapted
material and seed at t=6 seconds. Its image is compared against the Rust static
view using the existing tolerance (6/255, <=0.5% differing pixels), separately
from the unchanged frozen MVP migration manifest. Browser tests also exercise
animation, pause, deformation and orbit controls.

# glTF / GLB viewer

DamagedHelmet is copied from the pinned Three.js r186 archive. Its original
credit and CC Attribution-NonCommercial notice are preserved in
`models/DamagedHelmet/README.md` (theblueturtle_).

BoomBox is from KhronosGroup/glTF-Sample-Assets, commit
`90d7ede14c7e280af263824604b427a1ca02cb66`,
`Models/BoomBox/glTF-Binary/BoomBox.glb`. Credits: Microsoft; CC0.
See `models/BoomBox-LICENSE.md` for the upstream notice.

`?example=4` reads glTF JSON, its external BIN and JPEGs; `?example=5` reads
GLB binary data and embedded PNGs. gltf-rs parses the files in Rust/Wasm.
The viewer imports triangle primitives, indices, normals, UV0, vertex colors,
base-color factors/textures, sampler wrapping and static node transforms.
It uses a base-color material: HDR environment lighting, metallic/roughness,
normal, emissive and occlusion maps are not applied. It is not a reproduction
of the official loader example's PBR lighting. Animation, skins, morphs, data
URIs and required extensions are outside this static demo loader's scope.

The comparison page uses the actual pinned Three.js GLTFLoader and converts
materials to the same base-color mode. Both models must meet the existing image
tolerance and pass orbit interaction checks. The demo itself never invokes the
JavaScript GLTFLoader.
