#!/usr/bin/env bash
# Regenerates src/browser/tiles3d/wgsl/*.wgsl from the GLSL three.js compiles for
# webgl_loader_3dtiles ( captured on r186 from the reference page's shaderSource calls,
# src/browser/tiles3d/glsl/, listed in programs.txt ): glsl_to_vulkan.py, then naga's
# GLSL frontend and WGSL backend ( tools/tiles3d/naga-tool ), then
# tools/tiles3d/wgsl_fixup.py. Every program but the canvas output renders
# into a render target and flips clip y.
set -euo pipefail
cd "$(dirname "$0")/../.."
G=src/browser/tiles3d/glsl
W=src/browser/tiles3d/wgsl
T=.cache/tiles3d-vk
mkdir -p "$W" "$T"
naga() {
  CARGO_TARGET_DIR=.cache/naga-tool/target cargo run -q --release --manifest-path tools/tiles3d/naga-tool/Cargo.toml -- "$1"
}
while read -r name _ _; do
  flip=--flip
  [ "$name" = output ] && flip=
  python3 tools/tiles3d/glsl_to_vulkan.py $flip "$G/$name.vert" "$G/$name.frag" "$T/$name.vert" "$T/$name.frag"
  { echo 'diagnostic(off, derivative_uniformity);'; naga vert < "$T/$name.vert" | python3 tools/tiles3d/wgsl_fixup.py; } > "$W/$name.vert.wgsl"
  { echo 'diagnostic(off, derivative_uniformity);'; naga frag < "$T/$name.frag" | python3 tools/tiles3d/wgsl_fixup.py; } > "$W/$name.frag.wgsl"
done < "$G/programs.txt"
