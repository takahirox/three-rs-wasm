#version 300 es
#define SHADER_TYPE RawShaderMaterial
#define SHADER_NAME CloudsResolveMaterial
#define TEMPORAL_UPSCALE 1
#define SHADOW_LENGTH 1
#define FRAMEBUFFER_PRECISION_HIGH 1
precision highp float;
precision highp sampler2DArray;

// A fifth-order polynomial approximation of Turbo color map.
// See: https://observablehq.com/@mbostock/turbo
// prettier-ignore
vec3 turbo(const float x) {
  float r = 0.1357 + x * (4.5974 - x * (42.3277 - x * (130.5887 - x * (150.5666 - x * 58.1375))));
  float g = 0.0914 + x * (2.1856 + x * (4.8052 - x * (14.0195 - x * (4.2109 + x * 2.7747))));
  float b = 0.1067 + x * (12.5925 - x * (60.1097 - x * (109.0745 - x * (88.5066 - x * 26.8183))));
  return vec3(r, g, b);
}

// Taken from https://gist.github.com/TheRealMJP/c83b8c0f46b63f3a88a5986f4fa982b1
// TODO: Use 5-taps version: https://www.shadertoy.com/view/MtVGWz
// Or even 4 taps (requires preprocessing in the input buffer):
// https://www.shadertoy.com/view/4tyGDD

/**
 * MIT License
 *
 * Copyright (c) 2019 MJP
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

vec4 textureCatmullRom(sampler2D tex, vec2 uv) {
  vec2 texSize = vec2(textureSize(tex, 0));

  // We're going to sample a a 4x4 grid of texels surrounding the target UV
  // coordinate. We'll do this by rounding down the sample location to get the
  // exact center of our "starting" texel. The starting texel will be at
  // location [1, 1] in the grid, where [0, 0] is the top left corner.
  vec2 samplePos = uv * texSize;
  vec2 texPos1 = floor(samplePos - 0.5) + 0.5;

  // Compute the fractional offset from our starting texel to our original
  // sample location, which we'll feed into the Catmull-Rom spline function to
  // get our filter weights.
  vec2 f = samplePos - texPos1;

  // Compute the Catmull-Rom weights using the fractional offset that we
  // calculated earlier. These equations are pre-expanded based on our knowledge
  // of where the texels will be located, which lets us avoid having to evaluate
  // a piece-wise function.
  vec2 w0 = f * (-0.5 + f * (1.0 - 0.5 * f));
  vec2 w1 = 1.0 + f * f * (-2.5 + 1.5 * f);
  vec2 w2 = f * (0.5 + f * (2.0 - 1.5 * f));
  vec2 w3 = f * f * (-0.5 + 0.5 * f);

  // Work out weighting factors and sampling offsets that will let us use
  // bilinear filtering to simultaneously evaluate the middle 2 samples from the
  // 4x4 grid.
  vec2 w12 = w1 + w2;
  vec2 offset12 = w2 / (w1 + w2);

  // Compute the final UV coordinates we'll use for sampling the texture
  vec2 texPos0 = texPos1 - 1.0;
  vec2 texPos3 = texPos1 + 2.0;
  vec2 texPos12 = texPos1 + offset12;

  texPos0 /= texSize;
  texPos3 /= texSize;
  texPos12 /= texSize;

  vec4 result = vec4(0.0);
  result += texture(tex, vec2(texPos0.x, texPos0.y)) * w0.x * w0.y;
  result += texture(tex, vec2(texPos12.x, texPos0.y)) * w12.x * w0.y;
  result += texture(tex, vec2(texPos3.x, texPos0.y)) * w3.x * w0.y;

  result += texture(tex, vec2(texPos0.x, texPos12.y)) * w0.x * w12.y;
  result += texture(tex, vec2(texPos12.x, texPos12.y)) * w12.x * w12.y;
  result += texture(tex, vec2(texPos3.x, texPos12.y)) * w3.x * w12.y;

  result += texture(tex, vec2(texPos0.x, texPos3.y)) * w0.x * w3.y;
  result += texture(tex, vec2(texPos12.x, texPos3.y)) * w12.x * w3.y;
  result += texture(tex, vec2(texPos3.x, texPos3.y)) * w3.x * w3.y;

  return result;
}

vec4 textureCatmullRom(sampler2DArray tex, vec3 uv) {
  vec2 texSize = vec2(textureSize(tex, 0));
  vec2 samplePos = uv.xy * texSize;
  vec2 texPos1 = floor(samplePos - 0.5) + 0.5;
  vec2 f = samplePos - texPos1;
  vec2 w0 = f * (-0.5 + f * (1.0 - 0.5 * f));
  vec2 w1 = 1.0 + f * f * (-2.5 + 1.5 * f);
  vec2 w2 = f * (0.5 + f * (2.0 - 1.5 * f));
  vec2 w3 = f * f * (-0.5 + 0.5 * f);
  vec2 w12 = w1 + w2;
  vec2 offset12 = w2 / (w1 + w2);
  vec2 texPos0 = texPos1 - 1.0;
  vec2 texPos3 = texPos1 + 2.0;
  vec2 texPos12 = texPos1 + offset12;
  texPos0 /= texSize;
  texPos3 /= texSize;
  texPos12 /= texSize;
  vec4 result = vec4(0.0);
  result += texture(tex, vec3(texPos0.x, texPos0.y, uv.z)) * w0.x * w0.y;
  result += texture(tex, vec3(texPos12.x, texPos0.y, uv.z)) * w12.x * w0.y;
  result += texture(tex, vec3(texPos3.x, texPos0.y, uv.z)) * w3.x * w0.y;
  result += texture(tex, vec3(texPos0.x, texPos12.y, uv.z)) * w0.x * w12.y;
  result += texture(tex, vec3(texPos12.x, texPos12.y, uv.z)) * w12.x * w12.y;
  result += texture(tex, vec3(texPos3.x, texPos12.y, uv.z)) * w3.x * w12.y;
  result += texture(tex, vec3(texPos0.x, texPos3.y, uv.z)) * w0.x * w3.y;
  result += texture(tex, vec3(texPos12.x, texPos3.y, uv.z)) * w12.x * w3.y;
  result += texture(tex, vec3(texPos3.x, texPos3.y, uv.z)) * w3.x * w3.y;
  return result;
}

#ifdef VARIANCE_9_SAMPLES
#define VARIANCE_OFFSET_COUNT 8
const ivec2 varianceOffsets[8] = ivec2[8](
  ivec2(-1, -1),
  ivec2(-1, 1),
  ivec2(1, -1),
  ivec2(1, 1),
  ivec2(1, 0),
  ivec2(0, -1),
  ivec2(0, 1),
  ivec2(-1, 0)
);
#else // VARIANCE_9_SAMPLES
#define VARIANCE_OFFSET_COUNT 4
const ivec2 varianceOffsets[4] = ivec2[4](ivec2(1, 0), ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0));
#endif // VARIANCE_9_SAMPLES

// Reference: https://github.com/playdeadgames/temporal
vec4 clipAABB(const vec4 current, const vec4 history, const vec4 minColor, const vec4 maxColor) {
  vec3 pClip = 0.5 * (maxColor.rgb + minColor.rgb);
  vec3 eClip = 0.5 * (maxColor.rgb - minColor.rgb) + 1e-7;
  vec4 vClip = history - vec4(pClip, current.a);
  vec3 vUnit = vClip.xyz / eClip;
  vec3 aUnit = abs(vUnit);
  float maUnit = max(aUnit.x, max(aUnit.y, aUnit.z));
  if (maUnit > 1.0) {
    return vec4(pClip, current.a) + vClip / maUnit;
  }
  return history;
}

#ifdef VARIANCE_SAMPLER_ARRAY
#define VARIANCE_SAMPLER sampler2DArray
#define VARIANCE_SAMPLER_COORD ivec3
#else // VARIANCE_SAMPLER_ARRAY
#define VARIANCE_SAMPLER sampler2D
#define VARIANCE_SAMPLER_COORD ivec2
#endif // VARIANCE_SAMPLER_ARRAY

// Variance clipping
// Reference: https://developer.download.nvidia.com/gameworks/events/GDC2016/msalvi_temporal_supersampling.pdf
vec4 varianceClipping(
  const VARIANCE_SAMPLER inputBuffer,
  const VARIANCE_SAMPLER_COORD coord,
  const vec4 current,
  const vec4 history,
  const float gamma
) {
  vec4 moment1 = current;
  vec4 moment2 = current * current;
  vec4 neighbor;
  
    #if 0 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[0]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 0 < VARIANCE_OFFSET_COUNT
  
    #if 1 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[1]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 1 < VARIANCE_OFFSET_COUNT
  
    #if 2 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[2]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 2 < VARIANCE_OFFSET_COUNT
  
    #if 3 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[3]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 3 < VARIANCE_OFFSET_COUNT
  
    #if 4 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[4]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 4 < VARIANCE_OFFSET_COUNT
  
    #if 5 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[5]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 5 < VARIANCE_OFFSET_COUNT
  
    #if 6 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[6]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 6 < VARIANCE_OFFSET_COUNT
  
    #if 7 < VARIANCE_OFFSET_COUNT
    neighbor = texelFetchOffset(inputBuffer, coord, 0, varianceOffsets[7]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 7 < VARIANCE_OFFSET_COUNT
  

  const float N = float(VARIANCE_OFFSET_COUNT + 1);
  vec4 mean = moment1 / N;
  vec4 varianceGamma = sqrt(max(moment2 / N - mean * mean, 0.0)) * gamma;
  vec4 minColor = mean - varianceGamma;
  vec4 maxColor = mean + varianceGamma;
  return clipAABB(clamp(mean, minColor, maxColor), history, minColor, maxColor);
}

vec4 varianceClipping(
  const VARIANCE_SAMPLER inputBuffer,
  const VARIANCE_SAMPLER_COORD coord,
  const vec4 current,
  const vec4 history
) {
  return varianceClipping(inputBuffer, coord, current, history, 1.0);
}

vec4 varianceClipping(
  const sampler2D inputBuffer,
  const vec2 coord,
  const vec4 current,
  const vec4 history,
  const float gamma
) {
  vec4 moment1 = current;
  vec4 moment2 = current * current;
  vec4 neighbor;
  
    #if 0 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[0]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 0 < VARIANCE_OFFSET_COUNT
  
    #if 1 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[1]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 1 < VARIANCE_OFFSET_COUNT
  
    #if 2 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[2]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 2 < VARIANCE_OFFSET_COUNT
  
    #if 3 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[3]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 3 < VARIANCE_OFFSET_COUNT
  
    #if 4 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[4]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 4 < VARIANCE_OFFSET_COUNT
  
    #if 5 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[5]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 5 < VARIANCE_OFFSET_COUNT
  
    #if 6 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[6]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 6 < VARIANCE_OFFSET_COUNT
  
    #if 7 < VARIANCE_OFFSET_COUNT
    neighbor = textureOffset(inputBuffer, coord, varianceOffsets[7]);
    moment1 += neighbor;
    moment2 += neighbor * neighbor;
    #endif // 7 < VARIANCE_OFFSET_COUNT
  

  const float N = float(VARIANCE_OFFSET_COUNT + 1);
  vec4 mean = moment1 / N;
  vec4 varianceGamma = sqrt(max(moment2 / N - mean * mean, 0.0)) * gamma;
  vec4 minColor = mean - varianceGamma;
  vec4 maxColor = mean + varianceGamma;
  return clipAABB(clamp(mean, minColor, maxColor), history, minColor, maxColor);
}

vec4 varianceClipping(
  const sampler2D inputBuffer,
  const vec2 coord,
  const vec4 current,
  const vec4 history
) {
  return varianceClipping(inputBuffer, coord, current, history, 1.0);
}


uniform sampler2D colorBuffer;
uniform sampler2D depthVelocityBuffer;
uniform sampler2D colorHistoryBuffer;

#ifdef SHADOW_LENGTH
uniform sampler2D shadowLengthBuffer;
uniform sampler2D shadowLengthHistoryBuffer;
#endif // SHADOW_LENGTH

uniform vec2 texelSize;
uniform int frame;
uniform float varianceGamma;
uniform float temporalAlpha;
uniform vec2 jitterOffset;

in vec2 vUv;

layout(location = 0) out vec4 outputColor;
#ifdef SHADOW_LENGTH
layout(location = 1) out float outputShadowLength;
#endif // SHADOW_LENGTH

const ivec2 neighborOffsets[9] = ivec2[9](
  ivec2(-1, -1),
  ivec2(-1, 0),
  ivec2(-1, 1),
  ivec2(0, -1),
  ivec2(0, 0),
  ivec2(0, 1),
  ivec2(1, -1),
  ivec2(1, 0),
  ivec2(1, 1)
);

const ivec4[4] bayerIndices = ivec4[4](
  ivec4(0, 12, 3, 15),
  ivec4(8, 4, 11, 7),
  ivec4(2, 14, 1, 13),
  ivec4(10, 6, 9, 5)
);

vec4 getClosestFragment(const ivec2 coord) {
  vec4 result = vec4(1e7, 0.0, 0.0, 0.0);
  vec4 neighbor;
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[0]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[1]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[2]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[3]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[4]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[5]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[6]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[7]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
    neighbor = texelFetchOffset(depthVelocityBuffer, coord, 0, neighborOffsets[8]);
    if (neighbor.r < result.r) {
      result = neighbor;
    }
  
  return result;
}

void temporalUpscale(
  const ivec2 coord,
  const ivec2 lowResCoord,
  const bool currentFrame,
  out vec4 outputColor,
  out float outputShadowLength
) {
  vec4 currentColor = texelFetch(colorBuffer, lowResCoord, 0);
  #ifdef SHADOW_LENGTH
  vec4 currentShadowLength = vec4(texelFetch(shadowLengthBuffer, lowResCoord, 0).rgb, 1.0);
  #endif // SHADOW_LENGTH

  if (currentFrame) {
    // Use the texel just rendered without any accumulation.
    outputColor = currentColor;
    #ifdef SHADOW_LENGTH
    outputShadowLength = currentShadowLength.r;
    #endif // SHADOW_LENGTH
    return;
  }

  vec4 depthVelocity = getClosestFragment(lowResCoord);
  vec2 velocity = depthVelocity.gb;
  vec2 prevUv = vUv - velocity;
  if (prevUv.x < 0.0 || prevUv.x > 1.0 || prevUv.y < 0.0 || prevUv.y > 1.0) {
    outputColor = currentColor;
    #ifdef SHADOW_LENGTH
    outputShadowLength = currentShadowLength.r;
    #endif // SHADOW_LENGTH
    return; // Rejection
  }

  // Variance clipping with a large variance gamma seems to work fine for
  // upsampling. This increases ghosting, of course, but it's hard to notice on
  // clouds.
  // vec4 historyColor = textureCatmullRom(colorHistoryBuffer, prevUv);
  vec4 historyColor = texture(colorHistoryBuffer, prevUv);
  vec4 clippedColor = varianceClipping(colorBuffer, vUv, currentColor, historyColor, varianceGamma);
  outputColor = clippedColor;

  #ifdef SHADOW_LENGTH
  // Sampling the shadow length history using scene depth doesn't make much
  // sense, but it's too hard to derive it properly. At least this approach
  // resolves the edges of scene objects.
  // vec4 historyShadowLength = vec4(textureCatmullRom(shadowLengthHistoryBuffer, prevUv).rgb, 1.0);
  vec4 historyShadowLength = vec4(texture(shadowLengthHistoryBuffer, prevUv).rgb, 1.0);
  vec4 clippedShadowLength = varianceClipping(
    shadowLengthBuffer,
    vUv,
    currentShadowLength,
    historyShadowLength,
    varianceGamma
  );
  outputShadowLength = clippedShadowLength.r;
  #endif // SHADOW_LENGTH
}

void temporalAntialiasing(const ivec2 coord, out vec4 outputColor, out float outputShadowLength) {
  vec4 currentColor = texelFetch(colorBuffer, coord, 0);
  #ifdef SHADOW_LENGTH
  vec4 currentShadowLength = vec4(texelFetch(shadowLengthBuffer, coord, 0).rgb, 1.0);
  #endif // SHADOW_LENGTH

  vec4 depthVelocity = getClosestFragment(coord);
  vec2 velocity = depthVelocity.gb;

  vec2 prevUv = vUv - velocity;
  if (prevUv.x < 0.0 || prevUv.x > 1.0 || prevUv.y < 0.0 || prevUv.y > 1.0) {
    outputColor = currentColor;
    #ifdef SHADOW_LENGTH
    outputShadowLength = currentShadowLength.r;
    #endif // SHADOW_LENGTH
    return; // Rejection
  }

  vec4 historyColor = texture(colorHistoryBuffer, prevUv);
  vec4 clippedColor = varianceClipping(colorBuffer, coord, currentColor, historyColor);
  outputColor = mix(clippedColor, currentColor, temporalAlpha);

  #ifdef SHADOW_LENGTH
  vec4 historyShadowLength = vec4(texture(shadowLengthHistoryBuffer, prevUv).rgb, 1.0);
  vec4 clippedShadowLength = varianceClipping(
    shadowLengthBuffer,
    coord,
    currentShadowLength,
    historyShadowLength
  );
  outputShadowLength = mix(clippedShadowLength.r, currentShadowLength.r, temporalAlpha);
  #endif // SHADOW_LENGTH
}

void main() {
  ivec2 coord = ivec2(gl_FragCoord.xy);

  #if !defined(SHADOW_LENGTH)
  float outputShadowLength;
  #endif // !defined(SHADOW_LENGTH)

  #ifdef TEMPORAL_UPSCALE
  ivec2 lowResCoord = coord / 4;
  int bayerValue = bayerIndices[coord.x % 4][coord.y % 4];
  bool currentFrame = bayerValue == frame % 16;
  temporalUpscale(coord, lowResCoord, currentFrame, outputColor, outputShadowLength);
  #else // TEMPORAL_UPSCALE
  temporalAntialiasing(coord, outputColor, outputShadowLength);
  #endif // TEMPORAL_UPSCALE

  #if defined(SHADOW_LENGTH) && defined(DEBUG_SHOW_SHADOW_LENGTH)
  outputColor = vec4(turbo(outputShadowLength * 0.05), 1.0);
  #endif // defined(SHADOW_LENGTH) && defined(DEBUG_SHOW_SHADOW_LENGTH)

  #ifdef DEBUG_SHOW_VELOCITY
  outputColor.rgb = outputColor.rgb + vec3(abs(texture(depthVelocityBuffer, vUv).gb) * 10.0, 0.0);
  #endif // DEBUG_SHOW_VELOCITY
}
