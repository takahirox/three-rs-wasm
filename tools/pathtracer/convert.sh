#!/usr/bin/env bash
# Regenerates src/browser/pathtracer/*.wgsl from the GLSL three.js r186 compiles for
# webgl_renderer_pathtracer ( captured from the reference page's shaderSource calls,
# src/browser/pathtracer/glsl/ ): glsl_to_vulkan.py, then naga's GLSL frontend and WGSL
# backend ( tools/pathtracer/naga-tool, naga 26 ), then wgsl_fixup.py.
set -euo pipefail
cd "$(dirname "$0")/../.."
for name in path_tracing equirect_to_cube_uv pmrem_copy clamped_interpolation; do
  { echo 'diagnostic(off, derivative_uniformity);'
    python3 tools/pathtracer/glsl_to_vulkan.py "src/browser/pathtracer/glsl/$name.glsl" | CARGO_TARGET_DIR=.cache/naga-tool/target cargo run -q --release --manifest-path tools/pathtracer/naga-tool/Cargo.toml -- frag | python3 tools/pathtracer/wgsl_fixup.py; } > "src/browser/pathtracer/$name.wgsl"
done
# The NoToneMapping variant of the quad: three compiles it without TONE_MAPPING.
sed '/^#define TONE_MAPPING$/d' src/browser/pathtracer/glsl/clamped_interpolation.glsl > .cache/clamped_interpolation_linear.glsl
{ echo 'diagnostic(off, derivative_uniformity);'
  python3 tools/pathtracer/glsl_to_vulkan.py .cache/clamped_interpolation_linear.glsl | CARGO_TARGET_DIR=.cache/naga-tool/target cargo run -q --release --manifest-path tools/pathtracer/naga-tool/Cargo.toml -- frag | python3 tools/pathtracer/wgsl_fixup.py; } > src/browser/pathtracer/clamped_interpolation_linear.wgsl
# The transparentBackground variant: no backgroundMap ( FEATURE_BACKGROUND_MAP 0 ).
sed 's/^#define FEATURE_BACKGROUND_MAP 1$/#define FEATURE_BACKGROUND_MAP 0/' src/browser/pathtracer/glsl/path_tracing.glsl > .cache/path_tracing_no_background.glsl
{ echo 'diagnostic(off, derivative_uniformity);'
  python3 tools/pathtracer/glsl_to_vulkan.py .cache/path_tracing_no_background.glsl | CARGO_TARGET_DIR=.cache/naga-tool/target cargo run -q --release --manifest-path tools/pathtracer/naga-tool/Cargo.toml -- frag | python3 tools/pathtracer/wgsl_fixup.py; } > src/browser/pathtracer/path_tracing_no_background.wgsl
