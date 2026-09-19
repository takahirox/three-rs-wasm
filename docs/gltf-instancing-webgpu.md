# Instancing: WebGPU-to-WebGPU comparison

The reference adapts the official r186 helmet Instancing scene to Three.js
`WebGPURenderer`; it is not a separate published upstream WebGPU example.
The Rust side uses the current example 16. Neither side uses the diagnostic
Y reflection. Model, camera, original HDR environment and 512×512 viewport are
held constant. Each condition was captured in three fresh pages.

| Samples | Run 1 | Run 2 | Run 3 |
| --- | ---: | ---: | ---: |
| 1 (MSAA off) | 0.406% | 0.111% | 0.111% |
| 4 | 4.414% | 4.066% | 4.428% |

The metric is the fraction of all pixels with any RGB channel differing by
more than 6/255. All three 1x captures meet the existing 0.5% threshold; none of
the 4x captures do. Initialization variation remains, so a single best capture
must not be used as the result. This is an image comparison, not a performance
measurement.

## Confirmed output-path difference

Three.js creates a 4x `rgba16float` scene pipeline. Its
`src/renderers/common/Renderer.js::_getFrameBufferTarget()` uses the linear
working color space and applies tone mapping and output conversion in a
separate pass after resolution.

Our example 16 selects `Rgba8Unorm` and `encode_srgb: true` in `src/browser.rs`.
`src/shader.wgsl::fs_main` applies ACES and sRGB encoding before MSAA resolution;
the presentation pass then copies the result. Thus this example retains the
output path used for matching WebGL even though it runs on WebGPU.

This is a concrete candidate for the remaining 4x discrepancy. Its contribution
has not yet been isolated by changing only the output path; these measurements
do not prove it explains the entire difference. The previous WebGL/WebGPU
sample-pattern comparison alone cannot explain this WebGPU-to-WebGPU result.

## Reproduction and artifacts

With the local repository server running on port 8173:

```sh
REFERENCE_BACKEND=webgpu REPEATS=3 OUTPUT=.cache/instancing-investigation/webgpu \
  node tools/gltf_examples/diagnose_instancing.mjs
```

Saved metrics: [gltf-instancing-webgpu.json](gltf-instancing-webgpu.json).
The reference check records renderer samples and actual pipeline sample counts
and target formats. Renderer `currentSamples` is not used: it describes the
current/final pass rather than necessarily the scene pass.

Local images and the six-case viewer are at
`http://127.0.0.1:8173/.cache/instancing-investigation/webgpu.html`.
All six displayed image metrics were checked against the saved JSON, without
page errors. The viewer and PNGs are ignored local artifacts; the metrics and
this report are durable repository files. Production rendering is unchanged.
