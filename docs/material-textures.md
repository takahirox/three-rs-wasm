# Material and texture examples

Five pinned r186 WebGPU examples run through the Rust/Wasm renderer:

| Official example | Implementation |
| --- | --- |
| `webgpu_materials_arrays` | Six materials per grouped, shared paper-solid geometry; hemispheric/directional lighting and shadows |
| `webgpu_clipping` | Global union and local union/intersection clipping, MSAA coverage, independent local shadow clipping; spot and cascaded sun shadows |
| `webgpu_materials_texture_manualmipmap` | Eight explicit colored mip levels, linear/nearest filtering, scissored scenes and mouse-controlled perspective |
| `webgpu_textures_anisotropy` | Resident mipmapped crate textures, anisotropy 16 versus 1, scissored scenes and mouse-controlled perspective |
| `webgpu_textures_partialupdate` | A resident 512×512 carbon texture updated by uploading 32×32 pixels and copying that region on the GPU |

The reviewed WebGL equivalents are excluded from the gallery. All five retain
partial-port status: Inspector presentation and comparable GPU timing remain
unverified. The manual-mipmap port responds to window resize; the pinned
original has no resize handler, so its pixel comparison retains its initial
viewport. The other four are also compared after resize.

`GpuTexture::from_rgba_mipmaps` validates and uploads an explicit RGBA8 mip
chain. `GpuTexture::copy_region_from` validates and copies a level-zero rectangle
between resident compatible 2D textures, without regenerating other mip levels
or performing color/orientation conversion. The partial-update example converts
Three.js's bottom-origin destination into the Core texture's top-origin storage
coordinates. `Scene::clipping_shadows` controls global planes in shadow passes;
material `clip_shadows` independently controls local planes. This scene adapter
does not claim a general hierarchical `ClippingGroup` API.

The manual painting follows r186 WebGPU behavior: `LinearFilter` samples the
generated mip chain, while `NearestFilter` compiles to a level-zero texture load.
This differs from interpreting the HTML's labels as a promise to disable all
mips. The final 1×1 manually supplied mip preserves Canvas 2D fractional coverage.

Validation uses the original pinned scripts, deterministic time/randomness,
GUI capture and local asset URLs. The fixture advances Three.js's node frame so
shadow maps update on every measured frame. Pixel tolerance remains 6/255 with
at most 0.5% differing pixels. Tests cover clipping toggles, orbit/pan/zoom,
mouse movement, resize, stable GPU resource counts, zero static-geometry uploads,
and the 4,096-byte update plus one 32×32 GPU copy.

```sh
npx playwright test --config playwright.gallery.config.js material-textures.spec.js
MATERIAL_TEXTURES_DPR=2 npx playwright test --config playwright.gallery.config.js material-textures.spec.js
cargo test --locked --test texture_operations
```

Asset regeneration requires the pinned archive, Node dependencies and Chrome:
`python3 tools/gallery/prepare-materials.py`. Ordinary tests use committed
assets; `tools/tsl/prepare.py` extracts the independent official reference assets.

Recorded validation: 132 native tests, 23 existing Core browser tests, 11 new
browser tests at each of DPR 1 and 2, and six published-layout checks passed.
[Stored DPR 2 measurements and GPU workloads](material-textures-comparison.json)
retain the per-state errors; this report makes no timing-parity claim.
