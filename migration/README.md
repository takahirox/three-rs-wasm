# Three.js-to-Rust examples

Both pairs render at 512×512 with no antialiasing, no tone mapping and sRGB output.
They run Three.js from the exact commit recorded in the compatibility baseline.
The comparison allows an RGB channel error of 6/255 on each pixel, with no more
than 0.5% of pixels exceeding that limit. These tolerances are fixed in the
manifest before comparisons run; do not adjust them to conceal a rendering bug.

The basic cube maps `Scene`, `PerspectiveCamera`, `BoxGeometry` and
`MeshBasicMaterial` directly to their Rust counterparts. The Rust example stores
the camera and mesh as scene-scoped handles. It also demonstrates Rust-owned
animation, pointer input and object selection.

The second pair uses a transformed parent group, an orthographic camera, multiple
meshes, a PNG texture, `MeshStandardMaterial`, all three required light types,
lines and points. JavaScript `scene.add(mesh)` becomes `Scene::insert`, and a
parent-child relationship becomes `Scene::add(parent, child)`. Shared geometry
and materials use `Arc` rather than JavaScript object references. Browser image
loading uses `Texture::load` through Rust browser bindings.

Open `/reference/three-js/?example=0` or `?example=1` for the JavaScript reference,
and `/web/?example=0&static=1` or `?example=1&static=1` for the Rust counterpart.
The normal Rust page animates unless `static=1` is supplied for deterministic
comparison. No JavaScript scene or property synchronization layer is involved.
