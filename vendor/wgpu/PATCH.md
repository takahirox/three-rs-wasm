# Local wgpu 26.0.1 fix

Source: crates.io wgpu 26.0.1, under the included MIT/Apache-2.0 licenses.
Only `src/backend/webgpu.rs` and its generated `GpuFeatureName` change: pass `RenderPassColorAttachment.depth_slice`
to `GPURenderPassColorAttachment.depthSlice`, as the native backend already does.
Without this, rendering a nonzero slice of a 3D texture silently targets slice zero.
No GPU copy or CPU rendering fallback is introduced. Remove this patch when
upgrading to a version that forwards the field. The layered gallery comparison
exercises multiple distinct slices in Chromium WebGPU.

The web backend also maps `Features::SUBGROUP` to the `subgroups` GPU feature
( a `GpuFeatureName::Subgroups` variant ), which wgpu 26 leaves out, so that a
device can be requested with it and run WGSL that enables subgroups.

For WebXR on WebGPU ( docs/webxr-gpu.md ), the web backend also exposes what
`XRGPUBinding` needs, without changing other behavior:
`wgpu::set_xr_compatible( true )` requests later adapters with
`xrCompatible`; `Device::as_web_device` returns the device's `GPUDevice`; and
`Device::create_texture_from_web` wraps a `GPUTexture` made outside wgpu ( a
projection layer's color texture ) as a `Texture` for rendering.
