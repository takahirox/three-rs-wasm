# Instancing: MSAA processing investigation

2026-09-19. Chrome 153.0.8010.48 on the recorded Apple GPU adapter.
The result is specific to this browser/backend combination, not a universal
WebGL-versus-WebGPU sample-pattern guarantee.

## Confirmed difference: coverage sample positions

A new minimal probe renders an 8×8 rectangle without Three.js, Wasm, glTF,
textures, HDR, lighting, blending or depth. It sweeps the rectangle boundary
across one pixel and reads back resolved colors. This measures coverage locations
independently of the complicated material shaders.

Coordinates below are screen coordinates: top-left (0,0), X right, Y down.
The sweep localizes each coordinate to a 1/16-pixel interval, consistent with
these eighth-pixel positions. Raw intervals and all 289 coverage measurements
per backend are preserved in [the measurement JSON](gltf-instancing-msaa-probe.json).

| Sample set, sorted by screen Y | WebGL | WebGPU |
| --- | --- | --- |
| Top row | (0.625, 0.125) | (0.375, 0.125) |
| Second row | (0.125, 0.375) | (0.875, 0.375) |
| Third row | (0.875, 0.625) | (0.125, 0.625) |
| Bottom row | (0.375, 0.875) | (0.625, 0.875) |

These **sets** are vertically reflected; rows are not API sample-index identities.
WebGL's default canvas and an explicit offscreen RGBA8 four-sample renderbuffer
produce the same positions, so the difference is not limited to the browser's
canvas presentation step.

A white rectangle covering X < 0.5 and Y < 0.25 inside the measured pixel covers
zero GL samples but one WebGPU sample. The resolved values are therefore **0 versus
64** out of 255. The converse rectangle X < 0.25, Y < 0.5 gives **64 versus 0**.
This difference exists even without material textures or HDR.

A triangle edge can consequently contribute a different amount of foreground,
background or a neighboring triangle to the same pixel. A finely triangulated,
high-contrast reflective helmet amplifies the visible consequence. This establishes
a mechanism for the earlier large MSAA-dependent difference; it does not prove
that every differing helmet pixel has this cause or that one pattern is generally
blurrier/higher quality than the other.

## What matches in the tested paths

| Operation | Evidence |
| --- | --- |
| Sample count | Actual WebGL count is 4; WebGPU target and pipeline use count 4. |
| Default varying interpolation | Both probes evaluate at pixel center (0.5,0.5), including partially covered pixels. No per-sample shading or centroid qualifier is used in the production mesh shader. |
| Resolve of stored color values | Full/half/quarter coverage gives 255/128/64 for white, and 128/64/32 for input gray 0.5, identically when coverage counts match. |
| Tone mapping / encoding order for example 16 | Rust already runs ACES then sRGB in the fragment shader before the unorm MSAA resolve, matching the reference's default unsigned-byte canvas path. |
| Final Rust presentation | `textureLoad` copies the resolved texel; it does not blur or linearly resample the image. Additional tone mapping / sRGB encoding is disabled for this example. |

These probes establish equal behavior for the tested colors and coverage cases,
not bit-exact equivalence of every possible resolve operation.

The relevant pinned code is `src/browser.rs` (example-16 target and presentation),
`src/render_target.rs` (multisample allocation), `src/renderer.rs` (resolve target
and pipeline count), `src/shader.wgsl` (ACES/sRGB output), and `src/present.wgsl`
(texel copy). The reference uses `WebGLRenderer({antialias:true})` with default
`UnsignedByteType`; r186's optional floating-point `WebGLOutput` path is not used.

WGSL specifies perspective/center interpolation by default:
[WGSL interpolation](https://www.w3.org/TR/WGSL/#interpolation).
WebGPU's exposed MSAA controls are count, sample mask and alpha-to-coverage; there
is no sample-position array in this descriptor:
[WebGPU multisample state](https://gpuweb.github.io/gpuweb/#dictdef-gpumultisamplestate).
Changing centroid interpolation would change where color is evaluated, not move
the coverage samples. It is not a substitute for aligning sample positions.

## Consequences and next step

The large discrepancy has a concrete rasterization mechanism. It is not evidence
that our renderer uses fewer samples, averages them incorrectly, or adds a blur
in the final copy. The earlier projection-reflection experiment is consistent
with these independently measured positions.

A potential compatibility approach is a reflected offscreen render followed by a
reflected presentation. That would need consistent handling of winding, derivative
normals, background rays, screen-space effects and other render targets. It must
be opt-in or validated across other scenes/backends before modifying Core. The
current investigation does not implement that change, reduce sample count, relax
acceptance, or resolve the remaining HDR/initialization differences.

## Reproduce

```sh
node tools/gltf_examples/probe_msaa.mjs
```

The tool uses a routed blank localhost page (no scene assets), saves the complete
measurements under `.cache/instancing-investigation/msaa/`, and checks GL, shader
compile/link and uncaptured GPU errors. `CHROME`, `BASE_URL` and `OUTPUT` can be
changed. The final run measures WebGL canvas, WebGL offscreen, and WebGPU coverage
at 289 boundaries each, plus eight color/interpolation cases. It completed with
no graphics validation errors. No production rendering code changed.

The local `msaa.html` illustration reads the measured positions and lets you vary
a rectangle's coverage or reflect the WebGPU pattern. It visualizes the saved
measurements; it is not a live rerun of the GPU probe.

The [further MSAA investigation](gltf-instancing-msaa-detail.md) finds occasional
1/255 resolve rounding differences over 990 cases and checks depth/derivatives
and input stability. The earlier small probe set did not expose this rounding.
