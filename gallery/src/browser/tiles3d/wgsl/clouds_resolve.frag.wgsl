diagnostic(off, derivative_uniformity);
struct Uniforms {
    texelSize: vec2<f32>,
    frame: i32,
    varianceGamma: f32,
    temporalAlpha: f32,
    jitterOffset: vec2<f32>,
}

struct FragmentOutput {
    @location(0) outputColor: vec4<f32>,
    @location(1) outputShadowLength: f32,
}

const varianceOffsets: array<vec2<i32>, 4> = array<vec2<i32>, 4>(vec2<i32>(1i, 0i), vec2<i32>(0i, -1i), vec2<i32>(0i, 1i), vec2<i32>(-1i, 0i));
const neighborOffsets: array<vec2<i32>, 9> = array<vec2<i32>, 9>(vec2<i32>(-1i, -1i), vec2<i32>(-1i, 0i), vec2<i32>(-1i, 1i), vec2<i32>(0i, -1i), vec2<i32>(0i, 0i), vec2<i32>(0i, 1i), vec2<i32>(1i, -1i), vec2<i32>(1i, 0i), vec2<i32>(1i, 1i));
const bayerIndices: array<vec4<i32>, 4> = array<vec4<i32>, 4>(vec4<i32>(0i, 12i, 3i, 15i), vec4<i32>(8i, 4i, 11i, 7i), vec4<i32>(2i, 14i, 1i, 13i), vec4<i32>(10i, 6i, 9i, 5i));

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var colorBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var colorBuffer_s: sampler;
@group(0) @binding(3) 
var depthVelocityBuffer_t: texture_2d<f32>;
@group(0) @binding(4) 
var depthVelocityBuffer_s: sampler;
@group(0) @binding(5) 
var colorHistoryBuffer_t: texture_2d<f32>;
@group(0) @binding(6) 
var colorHistoryBuffer_s: sampler;
@group(0) @binding(7) 
var shadowLengthBuffer_t: texture_2d<f32>;
@group(0) @binding(8) 
var shadowLengthBuffer_s: sampler;
@group(0) @binding(9) 
var shadowLengthHistoryBuffer_t: texture_2d<f32>;
@group(0) @binding(10) 
var shadowLengthHistoryBuffer_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> outputColor_2: vec4<f32>;
var<private> outputShadowLength_2: f32;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e24 = y_1;
    let _e25 = x_1;
    return exp2((_e24 * log2(_e25)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e24 = y_3;
    let _e25 = x_3;
    return exp2((_e24 * log2(_e25)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e24 = y_5;
    let _e25 = x_5;
    return exp2((_e24 * log2(_e25)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e24 = y_7;
    let _e25 = x_7;
    return exp2((_e24 * log2(_e25)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e23 = x_9;
    (*i) = trunc(_e23);
    let _e25 = x_9;
    let _e26 = (*i);
    return (_e25 - _e26);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e23 = x_11;
    (*i_1) = trunc(_e23);
    let _e25 = x_11;
    let _e26 = (*i_1);
    return (_e25 - _e26);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e23 = x_13;
    (*i_2) = trunc(_e23);
    let _e25 = x_13;
    let _e26 = (*i_2);
    return (_e25 - _e26);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e23 = x_15;
    (*i_3) = trunc(_e23);
    let _e25 = x_15;
    let _e26 = (*i_3);
    return (_e25 - _e26);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e22 = v_1;
    b = (bitcast<u32>(_e22) & 2147483647u);
    let _e27 = b;
    return (_e27 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e22 = v_3;
    b_1 = (bitcast<u32>(_e22) & 2147483647u);
    let _e27 = b_1;
    return (_e27 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e22 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e22) & vec2(2147483647u));
    let _e28 = b_2;
    return (_e28 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e22 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e22) & vec3(2147483647u));
    let _e28 = b_3;
    return (_e28 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e22 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e22) & vec4(2147483647u));
    let _e28 = b_4;
    return (_e28 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e22 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e22) & vec2(2147483647u));
    let _e28 = b_5;
    return (_e28 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e22 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e22) & vec3(2147483647u));
    let _e28 = b_6;
    return (_e28 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e22 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e22) & vec4(2147483647u));
    let _e28 = b_7;
    return (_e28 == vec4(2139095040u));
}

fn turbo(x_16: f32) -> vec3<f32> {
    var r: f32;
    var g: f32;
    var b_8: f32;

    r = (0.1357f + (x_16 * (4.5974f - (x_16 * (42.3277f - (x_16 * (130.5887f - (x_16 * (150.5666f - (x_16 * 58.1375f))))))))));
    g = (0.0914f + (x_16 * (2.1856f + (x_16 * (4.8052f - (x_16 * (14.0195f - (x_16 * (4.2109f + (x_16 * 2.7747f))))))))));
    b_8 = (0.1067f + (x_16 * (12.5925f - (x_16 * (60.1097f - (x_16 * (109.0745f - (x_16 * (88.5066f - (x_16 * 26.8183f))))))))));
    let _e73 = r;
    let _e74 = g;
    let _e75 = b_8;
    return vec3<f32>(_e73, _e74, _e75);
}

fn textureCatmullRom(tex_t: texture_2d<f32>, tex_s: sampler, uv: vec2<f32>) -> vec4<f32> {
    var uv_1: vec2<f32>;
    var texSize: vec2<f32>;
    var samplePos: vec2<f32>;
    var texPos1_: vec2<f32>;
    var f: vec2<f32>;
    var w0_: vec2<f32>;
    var w1_: vec2<f32>;
    var w2_: vec2<f32>;
    var w3_: vec2<f32>;
    var w12_: vec2<f32>;
    var offset12_: vec2<f32>;
    var texPos0_: vec2<f32>;
    var texPos3_: vec2<f32>;
    var texPos12_: vec2<f32>;
    var result: vec4<f32> = vec4(0f);

    uv_1 = uv;
    let _e26 = textureDimensions(tex_t, 0i);
    texSize = vec2<f32>(vec2<i32>(_e26));
    let _e30 = uv_1;
    let _e31 = texSize;
    samplePos = (_e30 * _e31);
    let _e34 = samplePos;
    texPos1_ = (floor((_e34 - vec2(0.5f))) + vec2(0.5f));
    let _e43 = samplePos;
    let _e44 = texPos1_;
    f = (_e43 - _e44);
    let _e47 = f;
    let _e50 = f;
    let _e53 = f;
    w0_ = (_e47 * (vec2(-0.5f) + (_e50 * (vec2(1f) - (0.5f * _e53)))));
    let _e63 = f;
    let _e64 = f;
    let _e69 = f;
    w1_ = (vec2(1f) + ((_e63 * _e64) * (vec2(-2.5f) + (1.5f * _e69))));
    let _e77 = f;
    let _e79 = f;
    let _e82 = f;
    w2_ = (_e77 * (vec2(0.5f) + (_e79 * (vec2(2f) - (1.5f * _e82)))));
    let _e91 = f;
    let _e92 = f;
    let _e97 = f;
    w3_ = ((_e91 * _e92) * (vec2(-0.5f) + (0.5f * _e97)));
    let _e103 = w1_;
    let _e104 = w2_;
    w12_ = (_e103 + _e104);
    let _e107 = w2_;
    let _e108 = w1_;
    let _e109 = w2_;
    offset12_ = (_e107 / (_e108 + _e109));
    let _e113 = texPos1_;
    texPos0_ = (_e113 - vec2(1f));
    let _e118 = texPos1_;
    texPos3_ = (_e118 + vec2(2f));
    let _e123 = texPos1_;
    let _e124 = offset12_;
    texPos12_ = (_e123 + _e124);
    let _e127 = texPos0_;
    let _e128 = texSize;
    texPos0_ = (_e127 / _e128);
    let _e130 = texPos3_;
    let _e131 = texSize;
    texPos3_ = (_e130 / _e131);
    let _e133 = texPos12_;
    let _e134 = texSize;
    texPos12_ = (_e133 / _e134);
    let _e139 = result;
    let _e140 = texPos0_;
    let _e142 = texPos0_;
    let _e145 = textureSample(tex_t, tex_s, vec2<f32>(_e140.x, _e142.y));
    let _e146 = w0_;
    let _e149 = w0_;
    result = (_e139 + ((_e145 * _e146.x) * _e149.y));
    let _e153 = result;
    let _e154 = texPos12_;
    let _e156 = texPos0_;
    let _e159 = textureSample(tex_t, tex_s, vec2<f32>(_e154.x, _e156.y));
    let _e160 = w12_;
    let _e163 = w0_;
    result = (_e153 + ((_e159 * _e160.x) * _e163.y));
    let _e167 = result;
    let _e168 = texPos3_;
    let _e170 = texPos0_;
    let _e173 = textureSample(tex_t, tex_s, vec2<f32>(_e168.x, _e170.y));
    let _e174 = w3_;
    let _e177 = w0_;
    result = (_e167 + ((_e173 * _e174.x) * _e177.y));
    let _e181 = result;
    let _e182 = texPos0_;
    let _e184 = texPos12_;
    let _e187 = textureSample(tex_t, tex_s, vec2<f32>(_e182.x, _e184.y));
    let _e188 = w0_;
    let _e191 = w12_;
    result = (_e181 + ((_e187 * _e188.x) * _e191.y));
    let _e195 = result;
    let _e196 = texPos12_;
    let _e198 = texPos12_;
    let _e201 = textureSample(tex_t, tex_s, vec2<f32>(_e196.x, _e198.y));
    let _e202 = w12_;
    let _e205 = w12_;
    result = (_e195 + ((_e201 * _e202.x) * _e205.y));
    let _e209 = result;
    let _e210 = texPos3_;
    let _e212 = texPos12_;
    let _e215 = textureSample(tex_t, tex_s, vec2<f32>(_e210.x, _e212.y));
    let _e216 = w3_;
    let _e219 = w12_;
    result = (_e209 + ((_e215 * _e216.x) * _e219.y));
    let _e223 = result;
    let _e224 = texPos0_;
    let _e226 = texPos3_;
    let _e229 = textureSample(tex_t, tex_s, vec2<f32>(_e224.x, _e226.y));
    let _e230 = w0_;
    let _e233 = w3_;
    result = (_e223 + ((_e229 * _e230.x) * _e233.y));
    let _e237 = result;
    let _e238 = texPos12_;
    let _e240 = texPos3_;
    let _e243 = textureSample(tex_t, tex_s, vec2<f32>(_e238.x, _e240.y));
    let _e244 = w12_;
    let _e247 = w3_;
    result = (_e237 + ((_e243 * _e244.x) * _e247.y));
    let _e251 = result;
    let _e252 = texPos3_;
    let _e254 = texPos3_;
    let _e257 = textureSample(tex_t, tex_s, vec2<f32>(_e252.x, _e254.y));
    let _e258 = w3_;
    let _e261 = w3_;
    result = (_e251 + ((_e257 * _e258.x) * _e261.y));
    let _e265 = result;
    return _e265;
}

fn textureCatmullRom_1(tex_t_1: texture_2d_array<f32>, tex_s_1: sampler, uv_2: vec3<f32>) -> vec4<f32> {
    var uv_3: vec3<f32>;
    var texSize_1: vec2<f32>;
    var samplePos_1: vec2<f32>;
    var texPos1_1: vec2<f32>;
    var f_1: vec2<f32>;
    var w0_1: vec2<f32>;
    var w1_1: vec2<f32>;
    var w2_1: vec2<f32>;
    var w3_1: vec2<f32>;
    var w12_1: vec2<f32>;
    var offset12_1: vec2<f32>;
    var texPos0_1: vec2<f32>;
    var texPos3_1: vec2<f32>;
    var texPos12_1: vec2<f32>;
    var result_1: vec4<f32> = vec4(0f);

    uv_3 = uv_2;
    let _e26 = textureDimensions(tex_t_1, 0i);
    let _e29 = textureNumLayers(tex_t_1);
    texSize_1 = vec2<f32>(vec3<i32>(vec3<u32>(_e26.x, _e26.y, _e29)).xy);
    let _e35 = uv_3;
    let _e37 = texSize_1;
    samplePos_1 = (_e35.xy * _e37);
    let _e40 = samplePos_1;
    texPos1_1 = (floor((_e40 - vec2(0.5f))) + vec2(0.5f));
    let _e49 = samplePos_1;
    let _e50 = texPos1_1;
    f_1 = (_e49 - _e50);
    let _e53 = f_1;
    let _e56 = f_1;
    let _e59 = f_1;
    w0_1 = (_e53 * (vec2(-0.5f) + (_e56 * (vec2(1f) - (0.5f * _e59)))));
    let _e69 = f_1;
    let _e70 = f_1;
    let _e75 = f_1;
    w1_1 = (vec2(1f) + ((_e69 * _e70) * (vec2(-2.5f) + (1.5f * _e75))));
    let _e83 = f_1;
    let _e85 = f_1;
    let _e88 = f_1;
    w2_1 = (_e83 * (vec2(0.5f) + (_e85 * (vec2(2f) - (1.5f * _e88)))));
    let _e97 = f_1;
    let _e98 = f_1;
    let _e103 = f_1;
    w3_1 = ((_e97 * _e98) * (vec2(-0.5f) + (0.5f * _e103)));
    let _e109 = w1_1;
    let _e110 = w2_1;
    w12_1 = (_e109 + _e110);
    let _e113 = w2_1;
    let _e114 = w1_1;
    let _e115 = w2_1;
    offset12_1 = (_e113 / (_e114 + _e115));
    let _e119 = texPos1_1;
    texPos0_1 = (_e119 - vec2(1f));
    let _e124 = texPos1_1;
    texPos3_1 = (_e124 + vec2(2f));
    let _e129 = texPos1_1;
    let _e130 = offset12_1;
    texPos12_1 = (_e129 + _e130);
    let _e133 = texPos0_1;
    let _e134 = texSize_1;
    texPos0_1 = (_e133 / _e134);
    let _e136 = texPos3_1;
    let _e137 = texSize_1;
    texPos3_1 = (_e136 / _e137);
    let _e139 = texPos12_1;
    let _e140 = texSize_1;
    texPos12_1 = (_e139 / _e140);
    let _e145 = result_1;
    let _e146 = texPos0_1;
    let _e148 = texPos0_1;
    let _e150 = uv_3;
    let _e152 = vec3<f32>(_e146.x, _e148.y, _e150.z);
    let _e156 = textureSample(tex_t_1, tex_s_1, _e152.xy, i32(_e152.z));
    let _e157 = w0_1;
    let _e160 = w0_1;
    result_1 = (_e145 + ((_e156 * _e157.x) * _e160.y));
    let _e164 = result_1;
    let _e165 = texPos12_1;
    let _e167 = texPos0_1;
    let _e169 = uv_3;
    let _e171 = vec3<f32>(_e165.x, _e167.y, _e169.z);
    let _e175 = textureSample(tex_t_1, tex_s_1, _e171.xy, i32(_e171.z));
    let _e176 = w12_1;
    let _e179 = w0_1;
    result_1 = (_e164 + ((_e175 * _e176.x) * _e179.y));
    let _e183 = result_1;
    let _e184 = texPos3_1;
    let _e186 = texPos0_1;
    let _e188 = uv_3;
    let _e190 = vec3<f32>(_e184.x, _e186.y, _e188.z);
    let _e194 = textureSample(tex_t_1, tex_s_1, _e190.xy, i32(_e190.z));
    let _e195 = w3_1;
    let _e198 = w0_1;
    result_1 = (_e183 + ((_e194 * _e195.x) * _e198.y));
    let _e202 = result_1;
    let _e203 = texPos0_1;
    let _e205 = texPos12_1;
    let _e207 = uv_3;
    let _e209 = vec3<f32>(_e203.x, _e205.y, _e207.z);
    let _e213 = textureSample(tex_t_1, tex_s_1, _e209.xy, i32(_e209.z));
    let _e214 = w0_1;
    let _e217 = w12_1;
    result_1 = (_e202 + ((_e213 * _e214.x) * _e217.y));
    let _e221 = result_1;
    let _e222 = texPos12_1;
    let _e224 = texPos12_1;
    let _e226 = uv_3;
    let _e228 = vec3<f32>(_e222.x, _e224.y, _e226.z);
    let _e232 = textureSample(tex_t_1, tex_s_1, _e228.xy, i32(_e228.z));
    let _e233 = w12_1;
    let _e236 = w12_1;
    result_1 = (_e221 + ((_e232 * _e233.x) * _e236.y));
    let _e240 = result_1;
    let _e241 = texPos3_1;
    let _e243 = texPos12_1;
    let _e245 = uv_3;
    let _e247 = vec3<f32>(_e241.x, _e243.y, _e245.z);
    let _e251 = textureSample(tex_t_1, tex_s_1, _e247.xy, i32(_e247.z));
    let _e252 = w3_1;
    let _e255 = w12_1;
    result_1 = (_e240 + ((_e251 * _e252.x) * _e255.y));
    let _e259 = result_1;
    let _e260 = texPos0_1;
    let _e262 = texPos3_1;
    let _e264 = uv_3;
    let _e266 = vec3<f32>(_e260.x, _e262.y, _e264.z);
    let _e270 = textureSample(tex_t_1, tex_s_1, _e266.xy, i32(_e266.z));
    let _e271 = w0_1;
    let _e274 = w3_1;
    result_1 = (_e259 + ((_e270 * _e271.x) * _e274.y));
    let _e278 = result_1;
    let _e279 = texPos12_1;
    let _e281 = texPos3_1;
    let _e283 = uv_3;
    let _e285 = vec3<f32>(_e279.x, _e281.y, _e283.z);
    let _e289 = textureSample(tex_t_1, tex_s_1, _e285.xy, i32(_e285.z));
    let _e290 = w12_1;
    let _e293 = w3_1;
    result_1 = (_e278 + ((_e289 * _e290.x) * _e293.y));
    let _e297 = result_1;
    let _e298 = texPos3_1;
    let _e300 = texPos3_1;
    let _e302 = uv_3;
    let _e304 = vec3<f32>(_e298.x, _e300.y, _e302.z);
    let _e308 = textureSample(tex_t_1, tex_s_1, _e304.xy, i32(_e304.z));
    let _e309 = w3_1;
    let _e312 = w3_1;
    result_1 = (_e297 + ((_e308 * _e309.x) * _e312.y));
    let _e316 = result_1;
    return _e316;
}

fn clipAABB(current: vec4<f32>, history: vec4<f32>, minColor: vec4<f32>, maxColor: vec4<f32>) -> vec4<f32> {
    var pClip: vec3<f32>;
    var eClip: vec3<f32>;
    var vClip: vec4<f32>;
    var vUnit: vec3<f32>;
    var aUnit: vec3<f32>;
    var maUnit: f32;

    pClip = (0.5f * (maxColor.xyz + minColor.xyz));
    eClip = ((0.5f * (maxColor.xyz - minColor.xyz)) + vec3(0.0000001f));
    let _e41 = pClip;
    vClip = (history - vec4<f32>(_e41.x, _e41.y, _e41.z, current.w));
    let _e49 = vClip;
    let _e51 = eClip;
    vUnit = (_e49.xyz / _e51);
    let _e54 = vUnit;
    aUnit = abs(_e54);
    let _e57 = aUnit;
    let _e59 = aUnit;
    let _e61 = aUnit;
    maUnit = max(_e57.x, max(_e59.y, _e61.z));
    let _e66 = maUnit;
    if (_e66 > 1f) {
        {
            let _e69 = pClip;
            let _e75 = vClip;
            let _e76 = maUnit;
            return (vec4<f32>(_e69.x, _e69.y, _e69.z, current.w) + (_e75 / vec4(_e76)));
        }
    }
    return history;
}

fn varianceClipping(inputBuffer_t: texture_2d<f32>, inputBuffer_s: sampler, coord: vec2<i32>, current_1: vec4<f32>, history_1: vec4<f32>, gamma: f32) -> vec4<f32> {
    var moment1_: vec4<f32>;
    var moment2_: vec4<f32>;
    var neighbor: vec4<f32>;
    var N: f32 = 5f;
    var mean: vec4<f32>;
    var varianceGamma: vec4<f32>;
    var minColor_1: vec4<f32>;
    var maxColor_1: vec4<f32>;

    moment1_ = current_1;
    moment2_ = (current_1 * current_1);
    let _e47 = textureLoad(inputBuffer_t, coord, 0i);
    neighbor = _e47;
    let _e48 = moment1_;
    let _e49 = neighbor;
    moment1_ = (_e48 + _e49);
    let _e51 = moment2_;
    let _e52 = neighbor;
    let _e53 = neighbor;
    moment2_ = (_e51 + (_e52 * _e53));
    let _e71 = textureLoad(inputBuffer_t, coord, 0i);
    neighbor = _e71;
    let _e72 = moment1_;
    let _e73 = neighbor;
    moment1_ = (_e72 + _e73);
    let _e75 = moment2_;
    let _e76 = neighbor;
    let _e77 = neighbor;
    moment2_ = (_e75 + (_e76 * _e77));
    let _e95 = textureLoad(inputBuffer_t, coord, 0i);
    neighbor = _e95;
    let _e96 = moment1_;
    let _e97 = neighbor;
    moment1_ = (_e96 + _e97);
    let _e99 = moment2_;
    let _e100 = neighbor;
    let _e101 = neighbor;
    moment2_ = (_e99 + (_e100 * _e101));
    let _e119 = textureLoad(inputBuffer_t, coord, 0i);
    neighbor = _e119;
    let _e120 = moment1_;
    let _e121 = neighbor;
    moment1_ = (_e120 + _e121);
    let _e123 = moment2_;
    let _e124 = neighbor;
    let _e125 = neighbor;
    moment2_ = (_e123 + (_e124 * _e125));
    let _e133 = moment1_;
    let _e134 = N;
    mean = (_e133 / vec4(_e134));
    let _e138 = moment2_;
    let _e139 = N;
    let _e142 = mean;
    let _e143 = mean;
    varianceGamma = (sqrt(max(((_e138 / vec4(_e139)) - (_e142 * _e143)), vec4(0f))) * gamma);
    let _e152 = mean;
    let _e153 = varianceGamma;
    minColor_1 = (_e152 - _e153);
    let _e156 = mean;
    let _e157 = varianceGamma;
    maxColor_1 = (_e156 + _e157);
    let _e160 = mean;
    let _e161 = minColor_1;
    let _e162 = maxColor_1;
    let _e164 = minColor_1;
    let _e165 = maxColor_1;
    let _e166 = clipAABB(clamp(_e160, _e161, _e162), history_1, _e164, _e165);
    return _e166;
}

fn varianceClipping_1(inputBuffer_t_1: texture_2d<f32>, inputBuffer_s_1: sampler, coord_1: vec2<i32>, current_2: vec4<f32>, history_2: vec4<f32>) -> vec4<f32> {
    let _e28 = varianceClipping(inputBuffer_t_1, inputBuffer_s_1, coord_1, current_2, history_2, 1f);
    return _e28;
}

fn varianceClipping_2(inputBuffer_t_2: texture_2d<f32>, inputBuffer_s_2: sampler, coord_2: vec2<f32>, current_3: vec4<f32>, history_3: vec4<f32>, gamma_1: f32) -> vec4<f32> {
    var moment1_1: vec4<f32>;
    var moment2_1: vec4<f32>;
    var neighbor_1: vec4<f32>;
    var N_1: f32 = 5f;
    var mean_1: vec4<f32>;
    var varianceGamma_1: vec4<f32>;
    var minColor_2: vec4<f32>;
    var maxColor_2: vec4<f32>;

    moment1_1 = current_3;
    moment2_1 = (current_3 * current_3);
    let _e35 = vec2<i32>(1i, 0i);
    let _e46 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(1i, 0i));
    neighbor_1 = _e46;
    let _e47 = moment1_1;
    let _e48 = neighbor_1;
    moment1_1 = (_e47 + _e48);
    let _e50 = moment2_1;
    let _e51 = neighbor_1;
    let _e52 = neighbor_1;
    moment2_1 = (_e50 + (_e51 * _e52));
    let _e61 = vec2<i32>(0i, -1i);
    let _e69 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(0i, -1i));
    neighbor_1 = _e69;
    let _e70 = moment1_1;
    let _e71 = neighbor_1;
    moment1_1 = (_e70 + _e71);
    let _e73 = moment2_1;
    let _e74 = neighbor_1;
    let _e75 = neighbor_1;
    moment2_1 = (_e73 + (_e74 * _e75));
    let _e87 = vec2<i32>(0i, 1i);
    let _e92 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(0i, 1i));
    neighbor_1 = _e92;
    let _e93 = moment1_1;
    let _e94 = neighbor_1;
    moment1_1 = (_e93 + _e94);
    let _e96 = moment2_1;
    let _e97 = neighbor_1;
    let _e98 = neighbor_1;
    moment2_1 = (_e96 + (_e97 * _e98));
    let _e113 = vec2<i32>(-1i, 0i);
    let _e115 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(-1i, 0i));
    neighbor_1 = _e115;
    let _e116 = moment1_1;
    let _e117 = neighbor_1;
    moment1_1 = (_e116 + _e117);
    let _e119 = moment2_1;
    let _e120 = neighbor_1;
    let _e121 = neighbor_1;
    moment2_1 = (_e119 + (_e120 * _e121));
    let _e129 = moment1_1;
    let _e130 = N_1;
    mean_1 = (_e129 / vec4(_e130));
    let _e134 = moment2_1;
    let _e135 = N_1;
    let _e138 = mean_1;
    let _e139 = mean_1;
    varianceGamma_1 = (sqrt(max(((_e134 / vec4(_e135)) - (_e138 * _e139)), vec4(0f))) * gamma_1);
    let _e148 = mean_1;
    let _e149 = varianceGamma_1;
    minColor_2 = (_e148 - _e149);
    let _e152 = mean_1;
    let _e153 = varianceGamma_1;
    maxColor_2 = (_e152 + _e153);
    let _e156 = mean_1;
    let _e157 = minColor_2;
    let _e158 = maxColor_2;
    let _e160 = minColor_2;
    let _e161 = maxColor_2;
    let _e162 = clipAABB(clamp(_e156, _e157, _e158), history_3, _e160, _e161);
    return _e162;
}

fn varianceClipping_3(inputBuffer_t_3: texture_2d<f32>, inputBuffer_s_3: sampler, coord_3: vec2<f32>, current_4: vec4<f32>, history_4: vec4<f32>) -> vec4<f32> {
    let _e28 = varianceClipping_2(inputBuffer_t_3, inputBuffer_s_3, coord_3, current_4, history_4, 1f);
    return _e28;
}

fn getClosestFragment(coord_4: vec2<i32>) -> vec4<f32> {
    var result_2: vec4<f32> = vec4<f32>(10000000f, 0f, 0f, 0f);
    var neighbor_2: vec4<f32>;

    let _e64 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e64;
    let _e65 = neighbor_2;
    let _e67 = result_2;
    if (_e65.x < _e67.x) {
        {
            let _e70 = neighbor_2;
            result_2 = _e70;
        }
    }
    let _e101 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e101;
    let _e102 = neighbor_2;
    let _e104 = result_2;
    if (_e102.x < _e104.x) {
        {
            let _e107 = neighbor_2;
            result_2 = _e107;
        }
    }
    let _e138 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e138;
    let _e139 = neighbor_2;
    let _e141 = result_2;
    if (_e139.x < _e141.x) {
        {
            let _e144 = neighbor_2;
            result_2 = _e144;
        }
    }
    let _e175 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e175;
    let _e176 = neighbor_2;
    let _e178 = result_2;
    if (_e176.x < _e178.x) {
        {
            let _e181 = neighbor_2;
            result_2 = _e181;
        }
    }
    let _e212 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e212;
    let _e213 = neighbor_2;
    let _e215 = result_2;
    if (_e213.x < _e215.x) {
        {
            let _e218 = neighbor_2;
            result_2 = _e218;
        }
    }
    let _e249 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e249;
    let _e250 = neighbor_2;
    let _e252 = result_2;
    if (_e250.x < _e252.x) {
        {
            let _e255 = neighbor_2;
            result_2 = _e255;
        }
    }
    let _e286 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e286;
    let _e287 = neighbor_2;
    let _e289 = result_2;
    if (_e287.x < _e289.x) {
        {
            let _e292 = neighbor_2;
            result_2 = _e292;
        }
    }
    let _e323 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e323;
    let _e324 = neighbor_2;
    let _e326 = result_2;
    if (_e324.x < _e326.x) {
        {
            let _e329 = neighbor_2;
            result_2 = _e329;
        }
    }
    let _e360 = textureLoad(depthVelocityBuffer_t, coord_4, 0i);
    neighbor_2 = _e360;
    let _e361 = neighbor_2;
    let _e363 = result_2;
    if (_e361.x < _e363.x) {
        {
            let _e366 = neighbor_2;
            result_2 = _e366;
        }
    }
    let _e367 = result_2;
    return _e367;
}

fn temporalUpscale(coord_5: vec2<i32>, lowResCoord: vec2<i32>, currentFrame: bool, outputColor: ptr<function, vec4<f32>>, outputShadowLength: ptr<function, f32>) {
    var currentColor: vec4<f32>;
    var currentShadowLength: vec4<f32>;
    var depthVelocity: vec4<f32>;
    var velocity: vec2<f32>;
    var prevUv: vec2<f32>;
    var historyColor: vec4<f32>;
    var clippedColor: vec4<f32>;
    var historyShadowLength: vec4<f32>;
    var clippedShadowLength: vec4<f32>;

    let _e32 = textureLoad(colorBuffer_t, lowResCoord, 0i);
    currentColor = _e32;
    let _e35 = textureLoad(shadowLengthBuffer_t, lowResCoord, 0i);
    let _e36 = _e35.xyz;
    currentShadowLength = vec4<f32>(_e36.x, _e36.y, _e36.z, 1f);
    if currentFrame {
        {
            let _e43 = currentColor;
            (*outputColor) = _e43;
            let _e44 = currentShadowLength;
            (*outputShadowLength) = _e44.x;
            return;
        }
    }
    let _e46 = getClosestFragment(lowResCoord);
    depthVelocity = _e46;
    let _e48 = depthVelocity;
    velocity = _e48.yz;
    let _e51 = vUv_1;
    let _e52 = velocity;
    prevUv = (_e51 - _e52);
    let _e55 = prevUv;
    let _e59 = prevUv;
    let _e64 = prevUv;
    let _e69 = prevUv;
    if ((((_e55.x < 0f) || (_e59.x > 1f)) || (_e64.y < 0f)) || (_e69.y > 1f)) {
        {
            let _e74 = currentColor;
            (*outputColor) = _e74;
            let _e75 = currentShadowLength;
            (*outputShadowLength) = _e75.x;
            return;
        }
    }
    let _e77 = prevUv;
    let _e78 = textureSample(colorHistoryBuffer_t, colorHistoryBuffer_s, _e77);
    historyColor = _e78;
    let _e80 = vUv_1;
    let _e81 = currentColor;
    let _e82 = historyColor;
    let _e83 = global.varianceGamma;
    let _e84 = varianceClipping_2(colorBuffer_t, colorBuffer_s, _e80, _e81, _e82, _e83);
    clippedColor = _e84;
    let _e86 = clippedColor;
    (*outputColor) = _e86;
    let _e87 = prevUv;
    let _e88 = textureSample(shadowLengthHistoryBuffer_t, shadowLengthHistoryBuffer_s, _e87);
    let _e89 = _e88.xyz;
    historyShadowLength = vec4<f32>(_e89.x, _e89.y, _e89.z, 1f);
    let _e96 = vUv_1;
    let _e97 = currentShadowLength;
    let _e98 = historyShadowLength;
    let _e99 = global.varianceGamma;
    let _e100 = varianceClipping_2(shadowLengthBuffer_t, shadowLengthBuffer_s, _e96, _e97, _e98, _e99);
    clippedShadowLength = _e100;
    let _e102 = clippedShadowLength;
    (*outputShadowLength) = _e102.x;
    return;
}

fn temporalAntialiasing(coord_6: vec2<i32>, outputColor_1: ptr<function, vec4<f32>>, outputShadowLength_1: ptr<function, f32>) {
    var currentColor_1: vec4<f32>;
    var currentShadowLength_1: vec4<f32>;
    var depthVelocity_1: vec4<f32>;
    var velocity_1: vec2<f32>;
    var prevUv_1: vec2<f32>;
    var historyColor_1: vec4<f32>;
    var clippedColor_1: vec4<f32>;
    var historyShadowLength_1: vec4<f32>;
    var clippedShadowLength_1: vec4<f32>;

    let _e30 = textureLoad(colorBuffer_t, coord_6, 0i);
    currentColor_1 = _e30;
    let _e33 = textureLoad(shadowLengthBuffer_t, coord_6, 0i);
    let _e34 = _e33.xyz;
    currentShadowLength_1 = vec4<f32>(_e34.x, _e34.y, _e34.z, 1f);
    let _e41 = getClosestFragment(coord_6);
    depthVelocity_1 = _e41;
    let _e43 = depthVelocity_1;
    velocity_1 = _e43.yz;
    let _e46 = vUv_1;
    let _e47 = velocity_1;
    prevUv_1 = (_e46 - _e47);
    let _e50 = prevUv_1;
    let _e54 = prevUv_1;
    let _e59 = prevUv_1;
    let _e64 = prevUv_1;
    if ((((_e50.x < 0f) || (_e54.x > 1f)) || (_e59.y < 0f)) || (_e64.y > 1f)) {
        {
            let _e69 = currentColor_1;
            (*outputColor_1) = _e69;
            let _e70 = currentShadowLength_1;
            (*outputShadowLength_1) = _e70.x;
            return;
        }
    }
    let _e72 = prevUv_1;
    let _e73 = textureSample(colorHistoryBuffer_t, colorHistoryBuffer_s, _e72);
    historyColor_1 = _e73;
    let _e75 = currentColor_1;
    let _e76 = historyColor_1;
    let _e77 = varianceClipping_1(colorBuffer_t, colorBuffer_s, coord_6, _e75, _e76);
    clippedColor_1 = _e77;
    let _e79 = clippedColor_1;
    let _e80 = currentColor_1;
    let _e81 = global.temporalAlpha;
    (*outputColor_1) = mix(_e79, _e80, vec4(_e81));
    let _e84 = prevUv_1;
    let _e85 = textureSample(shadowLengthHistoryBuffer_t, shadowLengthHistoryBuffer_s, _e84);
    let _e86 = _e85.xyz;
    historyShadowLength_1 = vec4<f32>(_e86.x, _e86.y, _e86.z, 1f);
    let _e93 = currentShadowLength_1;
    let _e94 = historyShadowLength_1;
    let _e95 = varianceClipping_1(shadowLengthBuffer_t, shadowLengthBuffer_s, coord_6, _e93, _e94);
    clippedShadowLength_1 = _e95;
    let _e97 = clippedShadowLength_1;
    let _e99 = currentShadowLength_1;
    let _e101 = global.temporalAlpha;
    (*outputShadowLength_1) = mix(_e97.x, _e99.x, _e101);
    return;
}

fn main_1() {
    var coord_7: vec2<i32>;
    var lowResCoord_1: vec2<i32>;
    var local: array<vec4<i32>, 4> = bayerIndices;
    var bayerValue: i32;
    var currentFrame_1: bool;
    var local_1: vec4<f32>;
    var local_2: f32;

    let _e27 = gl_FragCoord_1;
    coord_7 = vec2<i32>(_e27.xy);
    let _e31 = coord_7;
    lowResCoord_1 = (_e31 / vec2(4i));
    let _e36 = coord_7;
    let _e40 = coord_7;
    let _e48 = local[(_e40.x % 4i)][(_e36.y % 4i)];
    bayerValue = _e48;
    let _e50 = bayerValue;
    let _e51 = global.frame;
    currentFrame_1 = (_e50 == (_e51 % 16i));
    let _e56 = coord_7;
    let _e57 = lowResCoord_1;
    let _e58 = currentFrame_1;
    temporalUpscale(_e56, _e57, _e58, (&local_1), (&local_2));
    let _e65 = local_1;
    outputColor_2 = _e65;
    let _e66 = local_2;
    outputShadowLength_2 = _e66;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e47 = outputColor_2;
    let _e49 = outputShadowLength_2;
    return FragmentOutput(_e47, _e49);
}
