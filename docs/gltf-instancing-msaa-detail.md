# Further MSAA checks

Follow-up to [the sample-position probe](gltf-instancing-msaa.md) and the
[HDR-off reflected-render experiment](gltf-instancing-msaa-flip.md).
Chrome 153.0.8010.48, same local Apple GPU setup, 2026-09-19.

## A second, small difference: resolve rounding

The minimal geometry probe now tests 990 combinations: three matching coverage
counts, five background intensities, 33 foreground intensities and GL dithering
on/off. Color values are already display-encoded and stored in unorm attachments,
as in Instancing's production output path.

78 combinations produced different resolved values, but the maximum difference
was **1/255**. For example, a half-covered white shape on gray 0.5 gave 191 in
WebGL and 192 in WebGPU. These results qualify the earlier small set of probes:
the averaging behavior agrees, but its quantized result is not always identical.

This difference is below the current >6/255 per-channel threshold. In the tested
path, tone mapping and sRGB conversion have already happened before resolve and
the final pass copies the texel, so there is no later nonlinear tone mapper to
amplify this particular 1/255 difference. It cannot by itself explain the observed
0.38–0.85% above-threshold fraction. This is a measured bound for these 990 cases,
not a proof for every possible color or attachment format.

Reproduce: `node tools/gltf_examples/probe_msaa.mjs`.

## Depth format and shader derivatives

Using HDR off, the same fixed direct light, 4× MSAA and the Rust-side orientation
reflection, three independent initializations per variant gave:

| Variant | Differing pixels, per run |
| --- | --- |
| Existing depth32float and default derivatives | 0.376%, 0.846%, 0.376% |
| Request depth24plus | 0.376%, 0.376%, 0.846% |
| Force dpdxCoarse / dpdyCoarse | 0.375%, 0.376%, 0.846% |
| Force dpdxFine / dpdyFine | 0.376%, 0.376%, 0.846% |

No variant consistently eliminates the remaining discrepancy or the two observed
output states. Requesting depth24plus does not prove that the implementation uses
a physical 24-bit depth buffer. These results do not identify a depth-precision
bug. The derivative variant changes explicit shader derivatives, not every
implicit hardware texture LOD calculation. The earlier experiment also forcing
coarse gradients on material sampling did not eliminate initialization variation.

The maintained comparison tool accepts `VARIANT=depth24`, `VARIANT=coarse`, or
`VARIANT=fine` (default: `default`). For example:

```sh
MATRIX=1 RUST_FLIP=1 CASE=hdr-off-msaa-on-reflected REPEATS=3 VARIANT=fine \
  OUTPUT=.cache/instancing-investigation/fine \
  node tools/gltf_examples/diagnose_instancing.mjs
```

The diagnostic hook now installs instrumentation and reflection in one init script
with an explicit order. Fixing that order alone does **not** eliminate variation.
The generic Core shader and render-target configuration have not been changed.

## Repeated frames and captured inputs

Two sets of **unmodified gallery** measurements, six fresh pages and eight fresh
pages with `set_samples(4)` target recreation, each requested three frames per
page. All three frames per page were byte-identical. The initial screenshots were
also identical across each set. Recorded buffers, all material mips, samplers,
shader sources and pipeline descriptors matched across these initializations.
Thus continuous-frame noise was not reproduced, and the previously seen variation
is not reproduced by every initialization procedure.

In contrast, the actual HDR-off reflected **diagnostic** still produced about
0.376% versus 0.846% under six fresh initializations, even with the hook order fixed.
The WebGL images of those two states were identical. Rust images differed in
1,276 pixels above threshold (maximum channel difference 148).

For those differing Rust states, the following captured inputs were identical:

- Uploaded/mapped buffer contents and descriptors, including scene/instance data.
- Material texture data for **all 61 mip levels across six textures**, read back
  from the GPU, including the fallback texture.
- Sampler descriptors, final shader sources and render-pipeline descriptors.

SHA-256 signatures and one canonical input record are preserved in
[the measurement report](gltf-instancing-msaa-detail.json). This rules out changes
in the measured asset data or those pipeline parameters as the explanation of
these two captures. It does **not** prove a driver bug: bind-group associations,
all render-pass state and shader compiler behavior have not been fully captured.
Nor does it establish a general Core initialization bug, given the unmodified
14-page results above. The modified diagnostic itself remains within scope of
possible causes.

The investigative capture scripts and full per-run records are retained locally
under `.cache/instancing-investigation/`: `capture-state.mjs`,
`capture-recreate.mjs`, `probe-all-mips.mjs`, and `msaa-all-mips/results.json`.
Their large intermediate captures are not production assets.

## Conclusion

The newly confirmed additional MSAA difference is small unorm resolve rounding.
The large known mismatch still has the independently measured reflected coverage
pattern as its dominant identified cause. No second large MSAA averaging,
interpolation or depth-setting mismatch has been established by these checks.

The remaining threshold failures and diagnostic initialization variation are still
unresolved. The next useful capture would track resource bindings and complete
render-pass/draw state for the two output states, rather than changing sample count
or relaxing the visual threshold. No reproduction acceptance or performance-parity
claim is made.
