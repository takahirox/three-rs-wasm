# Instancing image-difference investigation

Following this investigation, the user requested gallery access to the current
implementation. It is now listed as an explicitly labelled prototype with these
known differences. This does not change the failed WebGL comparison or establish
appearance/performance parity; statements about being hidden below describe the
state at the time of the investigation.

Investigation of commit `7324792`, on 2026-09-19 (JST), using pinned Three.js r186,
Chrome on Apple Metal, 512×512, DPR 1. This is a diagnosis, not a reproduction claim.
The original WebGL comparison still fails, and Instancing remains outside the gallery.
No Core shader, sample count, acceptance threshold or production asset was changed.

## Main finding: multisample rasterization orientation

The initial original 4× comparison differs in **5.910%** of pixels (RGB channel error >6/255).
The actual WebGL framebuffer reports four samples, so this is not a sample-count
mismatch. The following controlled experiment isolates the orientation of the
multisample rasterization from material shading:

1. Negate the reference camera's projection Y scale.
2. Reverse front-face winding to preserve culling and `gl_FrontFacing`.
3. Negate both normal-map scale components, compensating for the reversed
   derivative-built tangent frame in this pinned model (which has no tangents).
4. Flip the captured reference rows back before comparing.

This aligns the effective MSAA rasterization orientation on this browser/backend.
With the original physical materials, the difference falls to **1.649%**. For an
unlit base-color texture it falls from **4.211% to 0.335%**. At 1×, reflecting and
unreflecting changes the unlit result only from 0.391% to 0.390%.
This is strong evidence for the multisample orientation/coverage difference as the
largest source of the original 4× mismatch, rather than the instancing transforms.
It is specific to this tested WebGL/ANGLE versus WebGPU/Metal configuration, not a
claim about every implementation of either API.

The projection reflection is a diagnostic, not a proposed production fix. Reversing
an entire rendering convention would also affect culling, derivative tangent
frames, render targets and postprocessing. Simply switching to 1× would reduce
quality and would not establish reproduction.

## Remaining difference: reflective shading, not instance placement

A 1× constant-color diagnostic has **zero differing model pixels**. Its remaining
860 differing pixels are in the environment background. The foreground mask used
here is the 64,933 pixels with the constant model color (197,197,197); it is a
region diagnostic, not a replacement acceptance metric.

After aligning the 4× rasterization orientation:

| Diagnostic output | Differing foreground pixels (>6/255) |
| --- | ---: |
| Original physical material | 3,459 |
| Mapped normal (displayed as color) | 7 |
| Final material roughness (displayed as color) | 59 |
| Environment radiance before BRDF composition (displayed as color) | 5,018 |
| Disable metalness/roughness texture, retain scalar factors | 8 |

Intermediate outputs retain the same ACES/sRGB conversion on both sides. They
are not float readbacks, so small input errors may still be amplified by sharp
reflections. Disabling the metalness/roughness map changes the material and does
not demonstrate that the map loader is incorrect. It localizes the sensitive path
to roughness-dependent environment reflection. The normal map, instance positions
and geometry are not the dominant source at the tested tolerance.

### Mipmap rounding is present but does not explain the remaining mismatch

Actual GPU readback of the packed metalness/roughness texture confirms that mip 0
and mip 1 are identical. Later levels differ by up to 1–2/255. Copying the **entire
WebGL mip chain** into the Rust renderer's texture, after initialization, changes
the aligned 4× comparison only from **1.649% to approximately 1.64%**.
Thus mip generation precision is not the main remaining cause. This CPU readback /
copy is confined to the investigation tool; no production CPU fallback was added.

Other rejected explanations: removing the normal map did not materially improve
the original 4× result (5.910% → 5.875%); switching WGSL interpolation to centroid
or sample made it worse; forcing explicit texture LOD did not resolve it.

### PMREM coordinate convention contributes, but is not a complete explanation

A separate shader experiment made PMREM construction and environment lookup both
use the WebGL Y convention. Combined with aligned rasterization, this reduced the
physical-material difference to **1.294%**. Replacing the atlas with actual
WebGL-generated PMREM data gave **1.288%**, essentially the same result. This
supports a contribution from the prefilter coordinate convention, but disproves
that PMREM atlas generation alone explains the residual.

The remaining roughly 1–2% has **not** been reduced to a single shader expression or driver
operation. Further work should compare float roughness / reflected directions and
sampled environment values with identical PMREM and material mip data. Possible
small input/sampling errors should be measured before changing Core math. The
saved experiments do not justify blaming all residual pixels on backend precision.

The sharp-background reconstruction also accounts for about 0.328% of the full
image in the constant-color test. It is independent of the instance transforms.

## Additional finding: Rust/WebGPU initialization-to-initialization variation

Two runs of the saved diagnostic suite produced identical reference WebGL images,
but different Rust/WebGPU images in several cases:

- Original 4× comparison: **5.910% / 6.226%**.
- Aligned 4× comparison: **1.096% / 1.649%**.
- Between runs, the Rust images for these two cases each differed in 1,503 pixels
  (>6/255, maximum channel difference 88); their GL images were byte-identical.
- The unlit textured Rust output also varied (264 pixels after alignment), so this
  variation is not exclusively a PMREM-generation problem.
- Forcing both `dpdxCoarse` / `dpdyCoarse` and explicit coarse gradients for material
  texture sampling did **not** remove variation in a separate six-initialization
  experiment (aligned differences 1.243% or 1.775%).

The MSAA-orientation finding remains supported by all these runs, but small
improvements from individual ablations must not be treated as stable causal
estimates. The next investigation should first isolate this Rust/WebGPU-side
variation (captured pipeline inputs, GPU textures and repeated scene initialization)
before tuning PMREM or loosening image thresholds. The current evidence does not
identify whether its ultimate cause is Core state, shader compilation, or driver
behavior. Neither backend rounding nor a Core bug can yet be asserted as its cause.

## Reproduce and inspect

With the usual release Wasm build, pinned reference checkout and assets prepared,
serve the repository locally and run:

```sh
node tools/gltf_examples/diagnose_instancing.mjs
```

`BASE_URL`, `CHROME` and `OUTPUT` override the server, browser executable and output
directory. The script locally exposes example 16 through a routed catalog; it does
not change the gallery catalog. It records the browser, actual GL sample count,
11 ablation cases, original PNGs, and mip readback differences. Running it twice
with different `OUTPUT` directories also exposes the initialization variation. Default output is
`.cache/instancing-investigation/repro/`. Browser/page errors fail the run.

[Saved repeat-run results](gltf-instancing-investigation.json) contain the core
reproducible experiments. Supplemental sampling, PMREM and region experiments are
saved in [additional measurements](gltf-instancing-investigation-extra.json).
The tool is an investigative instrument, not a test that waives WebGL acceptance
or a performance benchmark. No production tests were needed because Core and
example implementations remain unchanged.
