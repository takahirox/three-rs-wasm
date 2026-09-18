# Physical glTF Core fixes and example updates

Scope: the four previously rejected candidates from the selected five glTF-specific
examples. Anisotropy, Sheen and Transmission now enter the gallery. Instancing is
updated but remains hidden because its original WebGL image comparison still fails.
This does not claim support for all 22 glTF examples or every physical material.

## Root causes and fixes

- **Anisotropy:** the environment radiance used an isotropic normal. It now uses the
  Three.js/Filament bent-normal calculation and the anisotropy map's rotation and
  strength. Clearcoat indirect light was also missing ambient occlusion.
- **Sheen:** indirect sheen light ignored the occlusion texture. Applying AO fixes
  the chair's appearance. The sheen control is ported, including returning from
  zero to nonzero sheen, with damped orbit/pan controls.
- **Transmission:** the fixed nine-tap blur was not the original rough-refraction
  algorithm. A reusable GPU-generated mip pyramid and IOR-scaled bicubic sampling
  replace it. Images, vertices and mip results are not read back to the CPU.
  Bind groups, mip views and pipelines persist between frames. The actual cover
  animation, automatic orbit and pointer/damping behavior are ported.
- **Instancing:** r186 `GLTFLoader` calls `assignFinalMaterial` for both the source
  mesh and the resulting `InstancedMesh`, flipping derivative `normalScale.y`
  twice. The example now matches that pinned behavior; the generic glTF importer
  is unchanged. Its output uses per-fragment ACES/sRGB before MSAA resolve, and
  redraws only for input/resize, matching the original WebGL scene.
- **Shared shader cost:** absent physical extension maps are removed by pipeline
  specialization, including their derivative/LOD work. Sample counts and image
  thresholds were not reduced to obtain performance results.

The previous Instancing comparison incorrectly used a WebGPU reference for a
WebGL example. The default reference now uses the actual WebGL renderer. A
separate, clearly labelled `backend=webgpu&samples=1` diagnostic isolates shared
material/instancing calculations. It is not a substitute for WebGL acceptance.

## Visual and behavioral evidence

The unchanged threshold is an RGB difference above 6/255 in at most 0.5% of pixels.
`gltf-physical.spec.js` compares 24 states across the three added scenes: initial
views, fixed animation/orbit times, orbit, pan, zoom, resize, and sheen values
0/0.4/1. GPU regression checks enforce no steady allocation or static geometry /
texture reupload, including after resize. The prior Iridescence tests remain.

Initial differences after the fixes are 0.212% (Anisotropy), 0.003% (Sheen), and
0.007% (Transmission). Iridescence improves to one differing pixel at 512×512.
See [all five initial render attempts](gltf-examples-render-attempts.json).

Instancing passes its strict common-backend 1x material/control regression, but
its original WebGL 4x comparison still differs in about 5.91% of pixels (3.13%
after a 3×3 area average), above the acceptance threshold. A separate 1x WebGL
probe measured 1.53%. The remaining backend difference includes mip generation
precision: GPU readback of the actual roughness maps found identical mip 1 but
1–2/255 differences at subsequent levels. Output/MSAA coverage also differs.
The evidence does not establish that these explain every remaining pixel.
No tolerance was relaxed and Instancing remains outside the gallery.

The original Inspector/info overlays are not reproduced. The published entries
remain labelled partial, with the UI limitations stated in the catalog. Assets
are unchanged pinned GLBs, with hashes in `tools/gltf_examples/assets.json` and
attribution in `web/THIRD_PARTY.md`.

## Performance evidence

The CPU/resource and separate GPU timestamp reports use the same original scene,
assets, 512×512 viewport and 4x samples, with 3 s warmup and 3 s sampling on Chrome /
Apple Metal. CPU sampling excludes callbacks that did not draw. Resource counters
include the extra refraction mip passes. Timestamp instrumentation itself allocates
readback buffers; allocation assertions use the uninstrumented runs.

- [Anisotropy CPU/resources](gltf-anisotropy-performance.json), [GPU timing](gltf-anisotropy-gpu-performance.json).
- [Sheen CPU/resources](gltf-sheen-performance.json), [GPU timing](gltf-sheen-gpu-performance.json).
- [Transmission CPU/resources](gltf-transmission-performance.json), [GPU timing](gltf-transmission-gpu-performance.json).

These short measurements do not establish general performance parity. The renderer
still has different pass organization, vertex storage and extension-map packing.
The reports retain the memory descriptors and draw/transfer counts rather than
using display-capped FPS as the acceptance argument.

GPU frame timing in this run (milliseconds; median / p95):

| Scene | Three.js | Rust |
| --- | ---: | ---: |
| anisotropy | 0.571 / 3.566 | 0.431 / 2.251 |
| sheen | 0.061 / 0.507 | 0.104 / 0.570 |
| transmission | 0.465 / 2.548 | 0.273 / 0.698 |

Sheen is still slower on the GPU in this run (about 0.104 ms versus 0.061 ms median); no blanket speed parity is claimed. All three uninstrumented runs have zero steady GPU allocations and unchanged static geometry/texture counters.

## Final verification

- Gallery: **83 passed**, including existing AVIF/Iridescence and all new physical
  material/control/resource tests. The 24 accepted physical views have maximum
  differing-pixel fractions of 0.212% / 0.0145% / 0.0225% respectively.
- Core browser tests: **23 passed**; native material/render-state tests: **2 passed**.
- Packaged `/three-rs-wasm/` paths: **3 passed**, including model/HDR requests and
  checks that no asset escaped the project prefix.
- The Instancing diagnostic additionally passed after asserting the pinned
  reference's actual `normalScale.y === 1`. Its common-backend 1x maximum across
  five views is 0.407%; this does not waive the original WebGL mismatch.
- Release Wasm build, native/Wasm clippy, formatting, pinned asset hashes,
  inventory consistency and whitespace checks passed.

[Saved per-view comparison results](gltf-physical-visual.json).
