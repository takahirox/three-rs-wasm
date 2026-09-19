# Instancing: MSAA × HDR ablation

2026-09-19, pinned Three.js r186 versus the current Rust/Wasm WebGPU renderer.
Chrome 153.0.8010.48, 512×512, DPR 1, same camera and unchanged glTF materials.
Each condition was captured after three independent page initializations. Results
are fractions of full-image pixels where any RGB channel differs by more than
6/255; the existing acceptance limit is 0.5%.

| HDR environment | MSAA | Differing pixels (three runs) |
| --- | --- | ---: |
| Off | Off (1×) | 0.467–0.742% |
| Off | On (4×) | 4.170% |
| On | Off (1×) | 1.153–1.523% |
| On | On (4×) | 5.905% |

The actual GL framebuffer reported 0 samples with antialiasing disabled and 4
with it enabled. These are the original rasterization orientations, with no
projection reflection or image alignment correction.

## Controlled lighting and meaning of HDR off

A white directional light of intensity 3, at (-3,4,-2) looking toward the origin,
is present on **both renderers in all six conditions**. This avoids a trivially
black metal surface when environment lighting is removed, while keeping direct
lighting constant between HDR-on and HDR-off. This is a diagnostic modification of
the original scene; do not compare its numbers as if its lighting were unchanged.

HDR off means zero environment illumination/reflection and a black background.
It does **not** disable ACES tone mapping, sRGB output, emissive textures, normal
maps, or metallic/roughness maps. HDR on restores the original environment and HDR
background. The light has no shadow. In Rust, the diagnostic injects only constant
light inputs into the existing shader, retaining the actual PBR equations; the
public `gltf_controls` and `background_intensity` APIs toggle the environment.
There are no production shader or asset changes.

Two additional conditions keep HDR reflection/illumination on but the background
black, isolating the effect of background reconstruction:

| HDR reflection, black background | Differing pixels |
| --- | ---: |
| MSAA off | 0.825–1.195% |
| MSAA 4× | 5.598–5.922% |

## Interpretation and limits

- MSAA increases the mismatch substantially even **without HDR**. Thus the large
  difference cannot be attributed exclusively to HDR prefiltering or reflection.
- HDR also increases mismatch with **MSAA off**. A separate environment-reflection
  path difference remains, consistent with the earlier intermediate-output probes.
- Keeping the HDR background black still leaves a reflection/material mismatch.
- Turning both off does not yield consistently accepted parity. Small direct-lit
  shading/rasterization differences and the initialization variation remain.
- Pixel-threshold fractions are not additive error budgets. These measurements
  cannot determine how much of a subjective blur comes from each operation, or
  identify the exact underlying shader/driver defect.

The three-run range is descriptive, not a confidence interval. The original
WebGL acceptance status is unchanged. No tolerance or image quality was reduced
as an implementation fix.

## Evidence and reproduction

[Raw measurements](gltf-instancing-msaa-hdr.json).

```sh
MATRIX=1 REPEATS=3 OUTPUT=.cache/instancing-investigation/matrix \
  node tools/gltf_examples/diagnose_instancing.mjs
```

A repository HTTP server, existing release Wasm and pinned reference/assets are
required. `BASE_URL` defaults to `http://127.0.0.1:8173`. The tool saves all 36 PNGs
and checks page/console errors and uncaptured GPU validation errors. The completed
18 comparisons passed these execution checks; that is not an image-parity pass.

The local comparison page at
`http://127.0.0.1:8173/.cache/instancing-investigation/matrix.html` provides all
conditions and all three captures, side-by-side, with a wipe slider and red
threshold-difference overlay. The HTML is a local, ignored viewing artifact.

The subsequent [MSAA mechanism investigation](gltf-instancing-msaa.md) directly
measures the different coverage sample positions and checks interpolation/resolve.
