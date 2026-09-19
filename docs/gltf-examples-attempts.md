# Five glTF example addition attempts

The user narrowed the scope to **five glTF-specific examples**. Each one was imported and rendered against its pinned original scene. Instancing is also available as an explicitly labelled prototype at user request; its image comparison still fails.

| Example | Stage | Initial raw image difference | Remaining issue |
| --- | --- | ---: | --- |
| webgpu_loader_gltf_iridescence | validated-partial-gallery | 0.000% | 公式IridescenceLamp・HDR環境・自動回転・Orbit操作を移植。情報表示は未一致。一般の屈折・拡張材質と性能の完全互換は未保証。 |
| webgpu_loader_gltf_anisotropy | validated-partial-gallery | 0.212% | 公式の異方性反射・クリアコート・透過材質とOrbit操作を移植。情報表示とInspector UIは未一致。 |
| webgpu_loader_gltf_sheen | validated-partial-gallery | 0.003% | 公式SheenChair・Sheen調整・減衰付きOrbit操作を移植。調整UIの外観と情報表示は未一致。 |
| webgpu_loader_gltf_transmission | validated-partial-gallery | 0.007% | 公式の透過・玉虫色材質・蓋のアニメーション・自動回転とOrbit操作を移植。情報表示とInspector UIは未一致。 |
| webgl_loader_gltf_instancing | prototype-gallery | 5.910% | 試作版：公式モデルのGPUインスタンシングとOrbit操作を実装。公式WebGL版とのMSAA・金属反射の描画差と、初期化ごとの描画変動は調査中。外観・性能の同等性は未確認。 |

[Per-example evidence](gltf-examples-attempts.json), [native model import/query results](gltf-examples-import-attempts.json), [original/Rust render and allocation measurements](gltf-examples-render-attempts.json).

The four new scene candidates are implemented in `src/browser/gltf_examples.rs`; instancing uses the existing scene. The three newly accepted physical scenes use pinned published assets. Instancing now uses the real gallery catalog; the original WebGL reference is kept distinct from the common-WebGPU diagnostic. The pinned Three.js code runs only in reference pages.

## Core fixes required by these attempts

- Transmission preserves independent draw slots across the opaque and final passes, eliminating repeated bind-group/buffer creation.
- Physical extension textures allocate occupied layers only and retain GPU-generated mip chains with minification/trilinear sampling. Iridescence uses one 2048×2048 layer (about 21.3 MiB with mips), instead of twelve layers (192 MiB without mips). Temporary mip sources are explicitly released after the GPU copy.
- Mixed-size maps can still require padded layers, and anisotropic extension-map sampling remains unsupported. No general physical-material/performance parity claim is made.

The next Core changes and updated acceptance evidence are recorded in [physical glTF worklog](gltf-physical-worklog.md).

## Earlier Iridescence validation

Seven fixed-time/camera/resize views pass the unchanged raw RGB threshold (6/255; at most 0.5% differing pixels). The browser regression also verifies automatic rotation, pointer pause, no steady GPU allocations or geometry/texture uploads, compact mipmapped storage, and device release.

On the recorded Apple Metal / Chrome workload (512×512, 3 s warmup + 3 s sample), CPU p50/p95 is 0.20/0.30 ms for Rust and 0.30/0.40 ms for Three.js. Separate timestamp instrumentation gives GPU frame p50/p95 of 0.211/0.467 ms and 0.449/2.566 ms respectively. This is a short, single-machine measurement, not general performance parity. CPU timing counts draw callbacks and excludes non-render polling callbacks; the reference renders once per animation frame.

[CPU/resource report](gltf-iridescence-performance.json), [GPU timing report](gltf-iridescence-gpu-performance.json).

Final regression checks: 73 gallery browser tests, 23 Core browser tests, two native material/render-state tests, and the packaged Pages Iridescence smoke test passed. Release Wasm build, native/Wasm clippy, formatting and gallery inventory checks passed.

## Reproduction

Run `python3 tools/gltf_examples/prepare.py`, build release Wasm, and run `node tools/gltf_examples/probe.mjs` with the local server on port 8173. Run `python3 tools/gltf_examples/report.py` to regenerate this report.
The native records come from `examples/gltf_core_probe.rs`, using the corresponding entries prepared by `tools/gallery/prepare_gltf_attempts.py`; CPU deformation there is an explicit query oracle, not the render path.
