# Stereo effects, PCD and ImageBitmap loaders

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/stereo_loaders.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_effects_stereo` | 203 | 500 refracting spheres around a Park3Med sky box, side-by-side StereoCamera views, mouse-driven camera easing |
| `webgl_effects_anaglyph` | 204 | 500 reflecting spheres around the Pisa sky box, two eye targets composited by the anaglyph color matrices |
| `webgl_effects_parallaxbarrier` | 205 | The same scene, two eye targets interleaved by alternate framebuffer rows |
| `webgl_loader_pcd` | 206 | The four PCD files (ASCII, binary, 8-bit binary, LZF-compressed), size, color and file controls, orbit |
| `webgl_loader_imagebitmap` | 207 | A rotating grid with three Image cubes and three ImageBitmap cubes added on the example clock |

None of these scenes has an equivalent official WebGPU example in the pinned
inventory, so each is compared against the original WebGL renderer.

## Port notes

### Stereo effects

- **Sky box and environment.** Both cube maps are sRGB, as `CubeTextureLoader`
  sets. The sky box is the background cube. Each sphere's `MeshBasicMaterial`
  env direction is computed per vertex from the world normal and camera
  position, as `vReflect` is, and interpolated.
- **StereoCamera.** Eye separation 0.064, focus 10. Each eye's off-axis frustum
  is shifted as in `StereoCamera.update`. The side-by-side example renders both
  halves into one target with WebGL's rounded viewports and scissors.
- **Anaglyph.** The eye cameras follow `AnaglyphEffect`'s `frameCorners`
  projection at plane distance 3.
- **Composites.** Anaglyph and parallax render each eye into its own 8-bit
  linear target, like the original's `WebGLRenderTarget`s, with linear
  minification and nearest magnification. One fullscreen triangle then
  composites them:
  - anaglyph: the original color matrices, clamped, then sRGB-encoded;
  - parallax: WebGL's bottom-up `gl_FragCoord.y` row parity.

  The eye targets and the composite binding are rebuilt only on resize.
- **Camera easing.** The per-frame easing `(mouse - position) * 0.05` uses 60 fps
  steps of example time, `0.95^step`. The reference fixture scales the original
  the same way.

### PCD

- **Parsing.** The parser follows `PCDLoader`: ASCII, binary and LZF
  `binary_compressed` data, x/y/z positions and packed rgb colors in sRGB.
- **Loading.** Each file is centered and rotated by π about x, as in the
  original. The type control resets the size and color to their defaults, since
  the original recreates its GUI and material.
- **Residency.** The original re-parses and re-uploads a file each time it is
  selected. The port uploads each parsed cloud once and keeps it resident. It
  shows the selected one again with fresh material defaults.
- **Rendering.** The scene renders on demand: only after input, a control change
  or a resize.

### ImageBitmap

- **Timers.** The six `setTimeout` loads run on the example clock at 0.3 s to
  1.9 s. The reference fixture runs its timers on the same clock.
- **Textures.** The original loads the same JPEG six times, with cache-busting
  query strings. The port decodes it once and shares one texture among the six
  cubes. The Image cubes keep their `0xff8888` tint.

Stats styling and the lil-gui appearance are not reproduced. OrbitControls
keyboard input is not ported.

## Comparison

`reference/three-js/stereo-loaders.html` executes the pinned sources. It uses
fixed time, seeded randomness and example-clock timers. The suite covers:

- time steps and mouse motion;
- every PCD control, drag and wheel;
- the timed cube loads;
- resize, at DPR 1 and 2.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Stereo | 0.328% / 0.10 | 0.235% / 0.09 |
| ImageBitmap, MSAA off | 0.357% / 0.48 | 0.180% / 0.24 |
| Anaglyph | 1.100% / 0.16 (after resize), otherwise ≤0.60% | 0.677% / 0.11 |
| Parallax barrier | 0.547% / 0.12 (after resize), otherwise ≤0.26% | 0.300% / 0.09 |
| PCD, MSAA off | 0.746% / 1.90 | 0.368% / 0.94 |
| PCD, 4× MSAA | 10.112% / 6.73 | 3.737% / 2.41 |
| ImageBitmap, 4× MSAA | 2.854% / 0.55 | 1.480% / 0.29 |

### Anaglyph and parallax eye targets

The stereo example has the same spheres, sky box and per-vertex env direction.
It renders them straight to the sRGB-encoded output and meets the ordinary
threshold. The other two first store each eye in an 8-bit linear target.

- **Why dark tones differ.** Encoding a dark linear value to sRGB has a slope up
  to 12.92, so a one-level difference in the target becomes several output
  levels.
- **Why anaglyph differs most.** Its right-eye blue row adds a 1.2264 gain.
- **Where the differences are.** Nine tenths of the differing anaglyph pixels
  are within 20/255. They lie inside the small reflecting spheres. The 640×400
  resize, with smaller spheres, has the most.
- **Not a precision effect.** Moving the original's camera by one Float32 ULP
  changes at most 0.003% of its pixels.

The suite bounds anaglyph by 1.25% / 0.25 and parallax by 0.65% / 0.2, with MSAA
off on both sides. The eye cameras, composite math, draws and residency are not
relaxed.

### PCD point coverage

The initial Zaghetto cloud draws 59,750 points about 1.3 pixels wide. The suite
computes the exact square coverage of every point from the pinned file. It uses
the pixel centers, or the four standard 4× sample positions, and compares both
renderers against that coverage:

| Pixels more than 40/255 from exact coverage | DPR 1 | DPR 2 |
| --- | --- | --- |
| Port, MSAA off | 0.071% | 0.038% |
| Original, MSAA off | 0.622% | 0.304% |
| Port, 4× MSAA | 0.270% | 0.154% |
| Original, 4× MSAA | 7.541% | 3.099% |

So the remaining PCD difference is the original's WebGL point rasterization
departing from square coverage. Neither side is systematically shifted, and
lit-pixel counts and run-length histograms agree. Two changes to the port were
tried against the original:

- rounding the point size made the match much worse;
- snapping point centers to an 8-bit subpixel grid did not improve it.

The suite requires, for this state:

- the port within 0.15% of exact coverage (0.4% with MSAA);
- the original at least four times further from it.

It bounds the PCD comparison by 0.85% / 2.2 with MSAA off and by 12% / 7.5 with
MSAA. Point sizes, geometry, colors and draws are not relaxed.

### MSAA

With MSAA off, the ImageBitmap scene matches within the ordinary threshold. With
4× MSAA, the differences lie on the cube and grid-line edges, as in the earlier
batches. The suite bounds it by 3.5% / 0.7 with MSAA.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, after the same
  frustum culling:
  - stereo: 126 draws, two sky boxes and the visible spheres of both eyes;
  - anaglyph: 230 draws;
  - parallax: 231 draws;
  - PCD: one draw of 59,750 point billboards;
  - ImageBitmap: seven draws, the grid and six cubes.

  WebGL points are counted as six-vertex billboards. Fullscreen triangles (the
  composites and the port's present pass) are left out on both sides.
- **Shared resources.** The 500 spheres share one geometry and one material.
- **Uploads.** After a warm pass, a measured frame uploads no geometry or texture
  data.
- **Warmed cycles.** Warmed cycles of time, motion, control changes, file
  switches and resize create no GPU resources.
- **Idle.** The PCD scene stays idle without input.

No GPU timing parity is claimed. Full measurements are in
[stereo-loaders-comparison.json](stereo-loaders-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js stereo-loaders.spec.js
STEREO_DPR=2 npx playwright test -c playwright.gallery.config.js stereo-loaders.spec.js
```
