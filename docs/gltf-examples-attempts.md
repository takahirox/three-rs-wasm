# Five glTF example addition attempts

The user narrowed the scope to **five glTF-specific examples**. Each one was imported and rendered against its pinned original scene. Only validated candidates enter the gallery.

| Example | Stage | Initial raw image difference | Remaining issue |
| --- | --- | ---: | --- |
| webgpu_loader_gltf_iridescence | validated-partial-gallery | 0.333% | 公式IridescenceLamp・HDR環境・自動回転・Orbit操作を移植。情報表示は未一致。一般の屈折・拡張材質と性能の完全互換は未保証。 |
| webgpu_loader_gltf_anisotropy | render-mismatch | 18.156% | Environment reflection still uses an isotropic roughness direction; direct-light anisotropy alone does not reproduce anisotropic IBL. Clearcoat/transmission also require matched-image validation.; Image comparison exceeds 0.5% differing pixels at 6/255. No tolerance was relaxed and this candidate is not listed in the gallery. |
| webgpu_loader_gltf_sheen | render-mismatch | 2.159% | SheenChair default material/HDR render differs from the original. Image fidelity and the fabric sheen control remain incomplete; successful import is insufficient.; Image comparison exceeds 0.5% differing pixels at 6/255. No tolerance was relaxed and this candidate is not listed in the gallery. |
| webgpu_loader_gltf_transmission | render-mismatch | 6.112% | Rough transmission/refraction sampling differs from the original. The real clip imports, but full animated/interactive image acceptance remains pending.; Image comparison exceeds 0.5% differing pixels at 6/255. No tolerance was relaxed and this candidate is not listed in the gallery. |
| webgl_loader_gltf_instancing | render-mismatch | 4.715% | EXT_mesh_gpu_instancing imports and executes on the GPU, but PBR/environment rendering differs from the original. This is a fidelity blocker, not a missing GPU-instancing API.; Image comparison exceeds 0.5% differing pixels at 6/255. No tolerance was relaxed and this candidate is not listed in the gallery. |

[Per-example evidence](gltf-examples-attempts.json), [native model import/query results](gltf-examples-import-attempts.json), [original/Rust render and allocation measurements](gltf-examples-render-attempts.json).

The four new scene candidates are implemented in `src/browser/gltf_examples.rs`; instancing uses the existing scene. Unaccepted candidates use unpublished `.cache/gltf-examples/` assets and test-only catalog overrides. The pinned Three.js code runs only in reference pages.

## Core fixes required by these attempts

- Transmission preserves independent draw slots across the opaque and final passes, eliminating repeated bind-group/buffer creation.
- Physical extension textures allocate occupied layers only and retain GPU-generated mip chains with minification/trilinear sampling. Iridescence uses one 2048×2048 layer (about 21.3 MiB with mips), instead of twelve layers (192 MiB without mips). Temporary mip sources are explicitly released after the GPU copy.
- Mixed-size maps can still require padded layers, and anisotropic extension-map sampling remains unsupported. No general physical-material/performance parity claim is made.

## Accepted Iridescence validation

Seven fixed-time/camera/resize views pass the unchanged raw RGB threshold (6/255; at most 0.5% differing pixels). The browser regression also verifies automatic rotation, pointer pause, no steady GPU allocations or geometry/texture uploads, compact mipmapped storage, and device release.

On the recorded Apple Metal / Chrome workload (512×512, 3 s warmup + 3 s sample), CPU p50/p95 is 0.20/0.30 ms for Rust and 0.30/0.40 ms for Three.js. Separate timestamp instrumentation gives GPU frame p50/p95 of 0.211/0.467 ms and 0.449/2.566 ms respectively. This is a short, single-machine measurement, not general performance parity. CPU timing counts draw callbacks and excludes non-render polling callbacks; the reference renders once per animation frame.

[CPU/resource report](gltf-iridescence-performance.json), [GPU timing report](gltf-iridescence-gpu-performance.json).

Final regression checks: 73 gallery browser tests, 23 Core browser tests, two native material/render-state tests, and the packaged Pages Iridescence smoke test passed. Release Wasm build, native/Wasm clippy, formatting and gallery inventory checks passed.

## Reproduction

Run `python3 tools/gltf_examples/prepare.py`, build release Wasm, and run `node tools/gltf_examples/probe.mjs` with the local server on port 8173. Run `python3 tools/gltf_examples/report.py` to regenerate this report.
The native records come from `examples/gltf_core_probe.rs`, using the corresponding entries prepared by `tools/gallery/prepare_gltf_attempts.py`; CPU deformation there is an explicit query oracle, not the render path.
