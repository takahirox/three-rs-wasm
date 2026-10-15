#version 300 es
#define SHADER_TYPE RawShaderMaterial
#define SHADER_NAME ShadowMaterial
#define SHADOW 1
#define TEMPORAL_PASS 1
#define TEMPORAL_JITTER 1
#define LOCAL_WEATHER_CHANNELS rgba
#define CASCADE_COUNT 2
#define SHAPE_DETAIL 1
#define TURBULENCE 1
#define FRAMEBUFFER_PRECISION_HIGH 1
precision highp float;

layout(location = 0) in vec3 position;

out vec2 vUv;

void main() {
  vUv = position.xy * 0.5 + 0.5;
  gl_Position = vec4(position.xy, 1.0, 1.0);
}
