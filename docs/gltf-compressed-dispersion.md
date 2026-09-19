# Compressed glTF and dispersion examples

Added the remaining two dedicated WebGPU glTF examples from pinned Three.js
r186: `webgpu_loader_gltf_compressed` (example 34) and
`webgpu_loader_gltf_dispersion` (example 33). All seven r186 WebGPU glTF entries
are now available as partial ports. Equivalent WebGL entries are excluded.

The original model files and environment are unchanged and checksum-verified by
`tools/gltf_examples/prepare.py`. Attribution is in `web/THIRD_PARTY.md`.
The reference fixture uses upstream GLTFLoader, KTX2Loader, MeshoptDecoder and
WebGPURenderer; the product uses Rust/Wasm and wgpu, with no Three.js renderer.

## Implementation

- Meshopt fallback buffers are left empty until the existing decoder prepares
  their contents. The browser importer retains KTX2/Basis image payloads.
- GPU upload selects ETC2 for ETC1S where supported, BC7 otherwise, and prefers
  ASTC 4×4 for supported UASTC images. All supplied mip levels are transcoded
  directly to blocks and uploaded once. Unsupported compressed GPU formats
  return an error instead of silently expanding textures to RGBA.
- Reinhard tone mapping is applied after the HDR MSAA resolve. The existing
  ACES API and behavior remain supported.
- Refraction now uses the upstream viewport texture's nearest magnification
  and linear minification/mipmap filtering. The former linear magnification
  produced a 0.97% initial dispersion difference, especially at IOR 1.0. The
  correction reduced it below the existing 0.5% threshold without changing
  resolution, samples, assets or image tolerance.
- Both examples include the original camera, Orbit/pan/zoom constraints and
  resize behavior. Coffeemat preserves its camera-attached point light, scale,
  offset and gray background; DispersionTest preserves HDR lighting and blur.

## Validation

Release Wasm and the upstream reference were compared at 512×512, 4x MSAA,
including orbit, pan, zoom and resize to 640×400. All seven states per example
have fewer than 0.05% pixels with any RGB channel differing by more than 6/255.
Raw results: [gltf-compressed-dispersion.json](gltf-compressed-dispersion.json).

On the tested browser/GPU, both implementations used ETC2 RGB blocks and the
same 13-level 4096² and 10-level 512² mip layouts. Rust uploaded 45,088,848
compressed bytes versus 56,273,672 for the reference; the Rust material texture
cache avoids one duplicate 4096² upload. These numbers cover the observed
compressed textures, not total GPU memory.

Steady-state resource creations and geometry reuploads were zero before and
after resize. Browser regression tests for the existing anisotropy, sheen,
transmission, iridescence and glTF scenes passed (16 tests), followed by nine
release-build tests for the additions and gallery. Six native tests covering
compression, block-texture rendering, Reinhard output, materials and render
state passed. Native/Wasm clippy, formatting and gallery generation checks pass.

No frame-time or complete performance parity claim is made. Information
overlays still differ. Compressed physical-extension texture arrays, layered
or cube Basis images and non-block-aligned base dimensions remain unsupported.
The explicit `compression::decode_basis` RGBA utility is separate from this
compressed rendering path.

Reproduce after preparing the pinned reference and assets, with the repository
server available:

```sh
python3 tools/gltf_examples/prepare.py
cargo test --test compressed_gpu --test compression
wasm-pack build --target web --out-dir web/pkg --release --no-typescript
npx playwright test gltf-physical.spec.js --grep 'dispersion|compressed'
```

Local demo routes:
`/web/gallery/#webgpu_loader_gltf_compressed` and
`/web/gallery/#webgpu_loader_gltf_dispersion`.
