# Cube refraction, PLY, KMZ, Collada and EXR

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/refraction_loaders.rs`, with the file formats in
`src/browser/refraction_loaders/formats.rs`. The zip and XML readers are shared
with [helpers-formats.md](helpers-formats.md).

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_materials_cubemap_refraction` | 238 | Three Lucy statues refracting the Park3Med cube, the cube background, the mouse-following camera |
| `webgl_loader_ply` | 239 | The dolphins and Lucy PLY models, flat-shaded, with fog, a hemisphere light and two shadow-casting lights |
| `webgl_loader_kmz` | 240 | The zipped Collada box on its grid, OrbitControls, on-demand rendering |
| `webgl_loader_collada` | 241 | The textured Collada elf, turning |
| `webgl_loader_texture_exr` | 242 | The PIZ-compressed memorial EXR with Reinhard tone mapping, the exposure control, on-demand rendering |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### PLY

- **Parsing.** The reader follows `PLYLoader`: the header lines, then ASCII
  tokens or binary values for vertex positions and triangle or quad faces.
  These are the properties the two example files contain.
- **Normals.** Both examples call `computeVertexNormals`.

### Refraction

- **Materials.** The three Phong materials take `envmap_fragment`'s
  CubeRefractionMapping: the camera-to-fragment ray is refracted about the
  world normal, sampled from the cube with x flipped, and applied by the
  MultiplyOperation, scaled by the reflectivity. The ratios and reflectivities
  are the example's (0.98, 0.985, 0.98 and 1, 1, 0.9).
- **Background.** The cube is drawn as `backgroundCube` does: a
  camera-centered box at the far plane.
- **Camera.** The camera eases toward the mouse (`( clientX - windowHalfX ) × 4`)
  by 0.05 per 60 fps step of example time, in the port and in the reference
  fixture alike.

### Shadows

- **Setup.** The PLY scene's two directional lights keep the example's
  settings: 2 × 2 orthographic shadow cameras from 1 to 4, 1024-texel maps,
  bias −0.001. They filter with r186's PCF: five Vogel-disk taps rotated by
  interleaved gradient noise.
- **Noise.** WebGL seeds that noise with `gl_FragCoord`, whose rows count from
  the bottom. WebGPU's fragment coordinates count from the top, so the rotation
  differs along shadow edges.

### Collada and KMZ

- **Loading.** `KMZLoader` is ported: the zip, `doc.kml`'s
  `Placemark Model Link href`, then `ColladaLoader` on the named document.
- **Collada.** The static-mesh path of `ColladaParser` and `ColladaComposer`
  is ported:
  - sources and accessors;
  - triangles, polylists and quads, as one geometry per primitive type, with a
    group and material per primitive;
  - phong, blinn, lambert and constant effects, with diffuse textures through
    their samplers and surfaces;
  - diffuse, specular and emissive colors converted from sRGB;
  - node transforms, the asset unit and the Z-up rotation.
- **Not ported.** Skins, animation, kinematics and polygons above four
  vertices. Neither asset uses them.
- **Textures.** The elf's textures are repeating sRGB maps with mipmaps.

### EXR

- **Decoding.** `EXRLoader` is ported for single-part scanline images of HALF
  channels, uncompressed or PIZ-compressed:
  - the bitmap lookup table;
  - the Huffman decoder, with its 32-bit accumulator wraparound;
  - the 14- and 16-bit inverse wavelets;
  - the per-scanline channel interleave.
  The decoded half-float data equals EXRLoader's output word for word.
- **Texture.** The texture keeps the loader's bottom-up rows, linear filters
  and single level.
- **Display.** The scene reuses the HDR example's Reinhard display.

Stats and the lil-gui appearance are not reproduced. OrbitControls touch input
is not separately verified.

## Comparison

`reference/three-js/refraction-loaders.html` executes the pinned sources on
the example clock. It waits for the elf's own LoadingManager. The suite covers:

- time steps;
- mouse movements;
- the exposure control;
- orbit drags, pans and wheels;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Refraction, MSAA off / on | 0.535% / 0.46, 2.137% / 0.73 | 0.461% / 0.45, 1.16% / 0.57 |
| PLY, MSAA off / on | 0.795% / 0.13, 1.844% / 0.26 | 0.655% / 0.11, 1.237% / 0.20 |
| KMZ, MSAA off / on | 0.33% / 0.34, 2.13% / 0.22 | 0.166% / 0.17, 1.173% / 0.12 |
| Collada | <0.001% / 0.02 | <0.001% / 0.01 |
| EXR | 0% / 0.00 | 0% / 0.00 |

### Bounds with MSAA off

- **Refraction.** The refracted statues sample fine environment detail, and a
  few of those pixels differ. The suite bounds that scene at 0.7% / 0.6.
- **PLY.** The flat-shaded facet edges take derivative normals, and the shadow
  edges take the flipped noise. The suite bounds that scene at 1% / 0.6.

### MSAA

With 4× MSAA, the differences lie on the statue, dolphin and box silhouettes,
the facets and the grid lines. The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| Refraction | 3% / 1 |
| PLY | 3% / 0.5 |
| KMZ | 3% / 0.4 |

The Collada and EXR examples render without antialiasing.

## Performance evidence

- **Draw workload.** The measured draws equal the original's:
  - three 300,000-index statues and the background box;
  - each PLY model once per shadow map and once on screen;
  - the box and grid;
  - the elf's four material groups;
  - the EXR quad.
- **Uploads.** After a warm pass, no scene uploads geometry or texture data.
- **Warmed cycles.** Warmed cycles of time, input and exposure changes create
  no GPU resources.
- **Idle.** The KMZ and EXR scenes stay idle without input.

No GPU timing parity is claimed. Full measurements are in
[refraction-loaders-comparison.json](refraction-loaders-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js refraction-loaders.spec.js
REFRACTION_DPR=2 npx playwright test -c playwright.gallery.config.js refraction-loaders.spec.js
```
