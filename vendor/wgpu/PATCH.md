# Local wgpu 26.0.1 fix

Source: crates.io wgpu 26.0.1, under the included MIT/Apache-2.0 licenses.
Only `src/backend/webgpu.rs` changes: pass `RenderPassColorAttachment.depth_slice`
to `GPURenderPassColorAttachment.depthSlice`, as the native backend already does.
Without this, rendering a nonzero slice of a 3D texture silently targets slice zero.
No GPU copy or CPU rendering fallback is introduced. Remove this patch when
upgrading to a version that forwards the field. The layered gallery comparison
exercises multiple distinct slices in Chromium WebGPU.
