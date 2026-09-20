# Example selection and comparison

- When equivalent official WebGL and WebGPU scenes exist, port and list only
  the WebGPU example. Compare it against the official WebGPU renderer.
- When only a WebGL version exists, port it to Rust/WebGPU. Document and allow
  backend-related image differences, such as MSAA sample placement and output
  conversion order. Do not simply increase every image threshold or forgive
  missing geometry, materials, lighting or behavior.
- Keep raw image measurements. Any additional tolerance needs a per-example
  explanation and evidence. Existing passing thresholds need not be loosened.
- Performance and execution architecture remain acceptance requirements; see
  [performance parity](performance-parity.md). An adapted Three.js WebGPU scene
  can help diagnose a WebGL-only port, but is not an official WebGPU example.
- Judge equivalence by scene content and purpose, not just names. Reviewed
  pairs live in `WEBGPU_EQUIVALENTS` in `tools/gallery/build.py`.

## Current gallery cleanup (pinned r186)

| Removed WebGL listing | Preferred official example | State |
| --- | --- | --- |
| `webgl_loader_gltf` | `webgpu_loader_gltf` | Existing partial port retained |
| `webgl_morphtargets` | `webgpu_morphtargets` | Existing partial port retained |
| `webgl_pmrem_test` | `webgpu_pmrem_test` | Existing partial port retained |
| `webgl_pmrem_equirectangular` | `webgpu_pmrem_equirectangular` | Existing partial port retained |
| `webgl_panorama_equirectangular` | `webgpu_equirectangular` | Same panoramic photograph and viewing purpose; controls and projection setup differ |
| `webgl_lights_rectarealight` | `webgpu_lights_rectarealight` | WebGPU TSL checker/LTC port listed |

The area-light WebGPU port uses the official procedural checker roughness node.
Its transparent MSAA output is unpremultiplied before sRGB conversion and then
premultiplied again, matching the official WebGPU canvas.

Instancing and AVIF glTF examples remain listed with their limitations because
no equivalent official WebGPU scenes were found in this baseline. Historical
investigation reports retain their original measurements and scope.

The newly added `webgpu_loader_gltf_compressed` and
`webgpu_loader_gltf_dispersion` also supersede their corresponding official
WebGL examples; those WebGL entries are excluded from the port inventory.
