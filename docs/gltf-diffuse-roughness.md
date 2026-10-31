# glTF diffuse roughness

Pinned Three.js r187: `7e89746a687bb88271253698fa108b1dd59825d0`.

r187 adds KHR_materials_diffuse_roughness to GLTFLoader
( GLTFMaterialsDiffuseRoughnessExtension ), MeshPhysicalMaterial's
`diffuseRoughness` and `diffuseRoughnessMap`, and the rough diffuse lobe they
select: BRDF_EON ( Portsmouth et al. 2025, "EON: A Practical
Energy-Preserving Rough Diffuse BRDF" ) for direct light and its directional
albedo for the indirect diffuse terms, in both the node system
( `BRDF_EON.js`, `PhysicalLightingModel` ) and WebGLRenderer
( `lights_physical_pars_fragment` ). The new page,
`webgpu_loader_gltf_diffuse_roughness`, has no WebGL counterpart; it is listed
and compared against the WebGPU renderer.

| Example | Id | Port |
| --- | --- | --- |
| `webgpu_loader_gltf_diffuse_roughness` | 501 | `gltf_diffuse_roughness.rs` |

## Loader and engine

- `crate::gltf` reads the extension as GLTFLoader does: a material with it is
  a MeshPhysicalMaterial ( `diffuseRoughnessFactor`, 0 when absent;
  `diffuseRoughnessTexture` as non-color data ), and it may be required.
- `MeshPhysicalMaterial::diffuse_roughness` and `diffuse_roughness_map`: above
  0 ( useDiffuseRoughness ), the engine shader lights the diffuse lobe with
  BRDF_EON × ( 1 − metalness ) and takes EON's directional albedo for the
  ambient, light-probe, hemisphere and environment irradiance, as r187 does;
  the map's red channel scales the factor. The map is the thirteenth
  physical extension map, a layer of the same resident array texture.
- `tests/diffuse_roughness.rs` imports the page's sample ( 4 standard and 20
  physical materials with their factors ) and a map, and checks a rendered
  directional light against an f64 BRDF_EON ( with and without the map,
  within one 8-bit level ).

## webgpu_loader_gltf_diffuse_roughness

DiffuseRoughnessParameterSweep.glb's 24 spheres under RoomEnvironment's PMREM
( fromScene, sigma 0.04 ), which is also the background with blurriness 0.5;
4× MSAA ( antialias ), Neutral tone mapping and damped OrbitControls. The
port loads the model through `crate::gltf` and draws each mesh with its
material's program. Its 24 meshes share the geometry GLTFLoader caches for
their identical primitives, uploaded once, whose triangle winding the page
reverses once at load, as the port does on the CPU. Frustum culling and the
front-to-back opaque order follow the renderer's. Every stage runs the WGSL
r187 generates for the page:

- the room, its PMREM and the background: `pmrem/` and `compute_cloth/`
  ( byte-identical );
- MeshStandardMaterial ( the four spheres without the extension ) and the
  output pass: the `compute_cloth/` modules, byte-identical;
- MeshPhysicalMaterial without and with diffuseRoughness:
  `gltf_diffuse_roughness/physical_*` and `rough_*`.

`tests/browser/compute-examples.spec.js` compares the page at the start, after
an orbit drag and a wheel ( both settled ) and after a resize to 640 × 400,
with and without MSAA, at the default bounds ( 6/255, 0.5 % of the pixels,
mean 0.6 ):

| Samples | State 0 | Drag | Wheel | Resize |
| --- | --- | --- | --- | --- |
| 1 | 0 % / 0.0003 | 0 % / 0.0002 | 0.0008 % / 0.0003 | 0.0004 % / 0.0003 |
| 4 | 0 % / 0.0008 | 0 % / 0.0006 | 0 % / 0.0006 | 0 % / 0.0006 |

( pixels over 6/255 / mean channel error ). The workload check counts the
original's draws: the 5,952-index background sphere and 24 × 9,216 indices,
with no attribute or texture uploads in steady frames. Residency cycles of
frames, orbit and wheel create nothing, before and after a resize.

The Inspector is the original's UI; its performance tab shows the Rust
frame as one draw. No timing parity is claimed.
