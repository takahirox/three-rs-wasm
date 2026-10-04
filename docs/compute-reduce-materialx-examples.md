# Compute reduce and MaterialX loader

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgpu_compute_reduce` | 450 | `compute_reduce.rs`, `compute_reduce/` | The six reductions of 262,144 ones, run, validated and reset each second in both halves, the four display modes and the buffer log buttons |
| `webgpu_loader_materialx` | 451 | `loader_materialx.rs`, `loader_materialx/` | 36 ShaderBall models with the page's twelve MaterialX samples and 24 local test documents, the PMREM environment, the transmission pass and the mesh visibility GUI |

Both are compared against the WebGPU renderer
(`tests/browser/compute-examples.spec.js`).

## Compute reduce ( 450 )

Each half of the page is one of its two renderers, drawn here into its half
of the canvas. Each half runs the page's stepAnimation every second:

1. **Run**: the selected algorithm's kernels in the page's order with their
   dispatch counts:
   - Reduce 0 halves the buffer per pass, from 131,072 down to 1;
   - Reduce 1 accumulates naively in three dispatches;
   - Reduce 2 reduces in workgroup memory in two;
   - Reduce 3 and 4 reduce with subgroup operations, Reduce 4 over the
     vectorized buffer;
   - the incorrect baseline races on one element.
2. **Validate**: element 0 against the element count, which colors the grid
   green or red.
3. **Reset**: the buffers return to ones.

The display shaders show the 512 × 512 grid, the power-of-two elements,
element 0 or the workgroup sums. Buffer to Log and the log buttons read the
selected buffer back and print it to the console.

The subgroup kernels need the device's `subgroups` feature. wgpu 26's WebGPU
backend does not map it, so `vendor/wgpu` adds the mapping
( `vendor/wgpu/PATCH.md` ). Without the feature the example reports an error.

## MaterialX loader ( 451 )

Every material runs the WGSL r186 compiles for the page's MaterialXLoader
material. The port does not parse the MaterialX documents or compile their
node graphs. The object uniforms are recognized from how each captured
shader uses its fields:

- the model and normal matrices, and the PMREM rotation;
- the physical material's defaults ( opacity, ior, metalness, specular
  intensity, emissive, attenuation );
- the thickness of 1 the loader sets on transmissive materials;
- the cube-UV texel sizes and lodMax 9.

The render uniforms carry the camera, the drawing buffer size and TSL time.
TSL time animates the rotate3d test.

At load, as the page does:

- **ShaderBall**: computePrefabTangents de-indexes both meshes and adds
  MikkTSpace tangents ( the `mikktspace` crate, signs negated ). The
  tangents match the page's bit for bit. Each attribute keeps its own vertex
  buffer and type: snorm16 positions and normals, unorm16 uvs, float
  tangents.
- **Images**: decoded without color conversion or flip ( ImageBitmapLoader
  with imageOrientation none ) and mipmapped on the GPU.
- **Environment**: the HDR equirect, one level as HDRLoader leaves it,
  through PMREMGenerator at lodMax 9.

Each frame draws:

1. the opaque balls front to back, culled by their bounding spheres;
2. when a transmissive ball is visible, the transmission's viewport mip
   texture: the resolved frame, copied and mipmapped;
3. the transmissive balls' back faces ( transparentDoublePass );
4. the transparent list back to front: the grid ground ( renderOrder -1 ),
   then the transparent materials' calibration ( 1 ) and preview ( 2 )
   meshes;
5. the LinearToneMapping output ( exposure 0.5 ).

With and without MSAA, the rendering matches the original within the
default thresholds, over the visibility toggles, TSL time and OrbitControls
drags and zoom.

## Differences

- **Compute reduce**: the GPU timestamp text in `#info` and the subgroup
  explanation panel's DOM animation are not ported.
- **MaterialX loader**: the page loads grid.png once per document, as nine
  textures. The port shares one texture.
- The Inspector's panels are not ported.

## Not added

`webgpu_lightprobes_sponza` and `webgpu_vxgi_sponza` load Sponza from
glTF-Sample-Assets. Its files are under the CRYENGINE Limited License
Agreement, which does not grant redistribution of the asset files. They are
not vendored, and the two examples remain unported.
