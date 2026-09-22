# Raw shader and raycast selection examples

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/interactive_shaders.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_shader` | 183 | Fullscreen plane, the original time-driven fragment translated to WGSL |
| `webgl_postprocessing_procedural` | 184 | Fullscreen `rand` noise, three materials selected by the `procedure` control |
| `webgl_interactive_cubes` | 185 | 2,000 Lambert cubes with individual materials, orbiting perspective camera, hover highlight |
| `webgl_interactive_cubes_ortho` | 186 | The same scene with the orthographic camera and resize-driven frustum |
| `webgl_interactive_points` | 187 | 1,538 merged box vertices as sized sprites, hovered point enlarged |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

The shader sources are translated statement for statement. GLSL `mod` is
written with `floor`, since WGSL `%` truncates. ShaderMaterial output is not
color-managed, so the fullscreen and point scenes write raw values; the Lambert
cubes encode sRGB before blending and the MSAA resolve, as WebGL does.
`Color.setHex` decodes the random cube colors from sRGB, while `Color.setHSL`
writes the point colors in the working color space without conversion.

The cubes keep one mesh and one material per object, as in the original. The
original's per-frame raycast runs on the CPU against the scene each frame.
Hovering changes only the emissive color of the old and new selection. The
points use the resident six-vertex sprite path, not per-vertex CPU work. The
original's size attribute becomes a resident storage buffer, uploaded only when
the selection changes. Position records stay resident. The CPU raycast reads a
retained copy of the same positions, standing in for the Points object the
original raycasts. The box vertices are merged with the `mergeVertices` rule:
the first occurrence in index order wins, keyed by a 1e-4 tolerance. The
original's initial `undefined` selection and the enlarged size it leaves on
deselected points are retained.

Three examples advance by a fixed amount per frame: the cube camera turns 0.1°
and the point cloud rotates 0.0005 / 0.001 rad. The ports use elapsed time at a
60 fps equivalent. The reference fixture applies the same substitution, so its
comparisons stay deterministic. Stats and lil-gui styling are not reproduced.

The WebGL `flipY` upload and the top-down `gl_PointCoord` together sample the
point texture at the quad's upward `uv`. Raw TSL sampling does not apply the
texture's `flip_y` matrix. An earlier port flipped the coordinate, which moved
the asymmetric disc edge by a texel and failed the comparison.

## Comparison

`reference/three-js/interactive-shaders.html` executes the pinned sources. It
seeds randomness, fixes time, captures GUI controllers and replaces the three
per-frame increments as described above. The suite compares time steps, every
procedure selection, pointer hover and leave states, and resize, at DPR 1 and 2.
It also checks warmed GPU residency and the original draw workload.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Shader | 0% / 0 | 0% / 0 |
| Procedural, 512² / 1024² states | 0% / 0 | 0% / 0 |
| Cubes, perspective, MSAA off | 0.002% / 0.001 | 0.001% / 0.001 |
| Cubes, orthographic, MSAA off | 0.001% / 0.002 | 0.001% / 0.001 |
| Points | 0.255% / 0.26 | 0.072% / 0.08 |
| Cubes, perspective, 4× MSAA | 3.115% / 0.98 | 1.558% / 0.49 |
| Cubes, orthographic, 4× MSAA | 9.074% / 2.45 | 4.592% / 1.22 |

### MSAA

With MSAA disabled on both sides, the cube scenes match per pixel. That result
includes all 2,000 transforms, colors, lighting, culling and hover highlights.
The 4× MSAA differences lie only on cube silhouettes; the WebGL and WebGPU
sample and resolve paths weight edge coverage differently. The orthographic view
shows all 2,000 small cubes, so it has far more edge pixels than the
perspective view. The suite requires the ordinary threshold with MSAA off.
With MSAA on, it bounds the perspective scene to 3.5% / 1.1 and the orthographic
scene to 9.5% / 2.6. These bounds are local to those two cases. They do not
relax geometry, controls, residency or workload checks.

### Hashed noise at non-power-of-two heights

The procedural shaders hash `vUv` through `fract(sin(x) * 43758.5453)`. At
512² and 1024², every image state matches the original per pixel. After resizing
to 640×400 (1280×800 at DPR 2), rows whose interpolated `vUv.y` rounds to a
neighboring Float32 differ completely, 44% of pixels in total.

The suite also renders the unmodified original with `vUv.y` changed by one ULP
inside the fragment shader (`uvUlp=1`). That change alone alters 62–88% of the
original's pixels at every size. Per-pixel equality is therefore not a
meaningful criterion for such states. For resized states whose height is not a
multiple of 256, the suite requires three checks:

- The one-ULP experiment changes more than 20% of pixels.
- The per-channel histogram L1 distance is at most 1.5× the one-ULP distance.
- Channel means match within 0.5/255.

Measured: histogram distance 0.0226 against 0.0302 for the one-ULP change, and a
mean shift of 0.21/255 at DPR 1. At DPR 2 they are 0.0108 against 0.0141, and
0.03/255. The noise shaders, their inputs and the plane are unchanged.

## Performance evidence

- **Draw workload.** The measured draws equal the original: one fullscreen draw
  for each shader scene, 569 / 1,999 frustum-culled cube draws of 36 indices, and
  one instanced draw of 1,538 sprites.
- **Uploads.** After a warm pass, a measured frame uploads no geometry, no
  attributes and no texture data.
- **Point buffers.** The point scene allocates exactly 49,216 bytes of position
  and color records and 6,152 bytes of sizes.
- **Warmed resources.** Warmed cycles of time steps, controls, pointer moves and
  resize leave GPU resource creation, geometry transfer and texture counts
  unchanged.
- **First-visibility records.** A cube's per-draw record is created the first
  time the cube enters the frustum. This is not repeated.

No GPU timing parity is claimed. Full measurements are in
[interactive-shaders-comparison.json](interactive-shaders-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js interactive-shaders.spec.js
INTERACTIVE_DPR=2 npx playwright test -c playwright.gallery.config.js interactive-shaders.spec.js
```
