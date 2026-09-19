# Rust-side MSAA orientation experiment

The user requested trying the alignment approach after the sample-position probe.
This experiment changes **Rust/WebGPU only**, leaving the official WebGL camera,
shaders and captured image unchanged. Production Core and gallery rendering have
not been modified. The opt-in browser hook is `tools/gltf_examples/rust_msaa_flip.js`.

## Changes in the experiment

- Negate clip-space Y at the end of the Instancing vertex shader.
- Reverse mesh front-face winding to preserve culling and front-facing normals.
- Reverse the Y derivatives used to build this model's cotangent frame.
- Reflect the background's inverse-projection ray in screen Y.
- Read row `height - 1 - y` in the existing final `textureLoad` presentation pass.

There is no extra rendering pass, CPU vertex/image processing, interpolation blur,
asset conversion, or sample-count reduction. The hook checks its shader anchors
and records that all three shader changes and the pipeline change were applied.
It is deliberately limited to this fixture, which has no shadows, transmission,
wide lines or custom screen-space effects. It is not a generic Core option.

## Original camera results

Chrome 153.0.8010.48, same 512×512 original scene/materials, three independent
initializations per condition. Raw RGB threshold remains >6/255.

| Samples | Unchanged Rust | Reflected Rust |
| --- | ---: | ---: |
| 1× | 1.177% | 1.511% |
| 4× | 5.910% | 1.637% |

All three captures of each condition gave these same values in this run. At 4×,
the differing-pixel fraction falls by about 72%. At 1× it slightly worsens, so the
experiment is not evidence for universally reflecting the renderer.

[Raw captures' measurements](gltf-instancing-msaa-flip.json).

## Controls and resource checks

A separate check exercised the actual interactive experiment page: initial view,
orbit, pan, zoom, and resize to 640×400. Their original-WebGL image differences
were 1.094%, 1.290%, 1.467%, 1.433%, and 1.393% respectively. None passes the existing
0.5% image acceptance threshold. These comparisons support preserved scene/camera
orientation but are not a claim of pixel-perfect control parity.

The different initial result in that run confirms that the previously recorded
initialization variation has **not** been eliminated.

The page has no idle frames or idle allocations, no new steady-state resource
creations or static asset reuploads after interaction, and three draw commands per
requested frame (background, instanced mesh, presentation). Page errors and
uncaptured GPU validation errors were absent. GPU frame-time parity was not
measured and is not claimed.

[Control/resource measurements](gltf-instancing-msaa-flip-controls.json).

## View / rerun

Local comparison page:
`http://127.0.0.1:8173/.cache/instancing-investigation/rust-flip.html`

Interactive experiment:
`http://127.0.0.1:8173/.cache/instancing-investigation/rust-flip-live.html?id=webgl_loader_gltf_instancing`

The two HTML pages and captures are local ignored artifacts, not deployed gallery
entries. The interactive page loads the normal gallery module through a base URL,
with the diagnostic hook installed before Wasm initialization.

```sh
RUST_FLIP=1 REPEATS=3 OUTPUT=.cache/instancing-investigation/rust-flip \
  node tools/gltf_examples/diagnose_instancing.mjs
```

The remaining reflection/material differences and initialization variation need
separate work. The experiment demonstrates that much of the MSAA-dependent image
difference can be reduced while retaining hardware 4× MSAA, but does not justify a
renderer-wide change or a reproduction acceptance claim yet.

## Follow-up: HDR off, MSAA 4× only

Same fixed white directional light as the HDR/MSAA matrix (intensity 3, position
-3,4,-2), black background, zero HDR environment intensity, ACES retained.
Three independent initializations of each variant produced:

| Run | Original orientation | Reflected orientation |
| --- | ---: | ---: |
| 1 | 3.929% | 0.376% |
| 2 | 3.929% | 0.846% |
| 3 | 4.170% | 0.846% |

The reflection materially reduces the mismatch without HDR, but only one of the
three reflected captures meets the 0.5% threshold. This does not establish stable
acceptance; initialization variation remains. These are not the 1.637% results
above, which retain the original HDR lighting.

[Measurements](gltf-instancing-no-hdr-flip.json). All six captures completed without
page or uncaptured GPU errors and with all expected shader patches applied.

```sh
MATRIX=1 RUST_FLIP=1 CASE=hdr-off-msaa-on REPEATS=3 \
  OUTPUT=.cache/instancing-investigation/no-hdr-flip \
  node tools/gltf_examples/diagnose_instancing.mjs
```

Local comparison: `http://127.0.0.1:8173/.cache/instancing-investigation/no-hdr-flip.html`.
