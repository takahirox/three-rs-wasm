diagnostic(off, derivative_uniformity);
struct Uniforms {
    texelSize: vec2<f32>,
    varianceGamma: f32,
    temporalAlpha: f32,
}

struct FragmentOutput {
    @location(0) outputColor: vec4<f32>,
    @location(1) outputColor_1: vec4<f32>,
}

const varianceOffsets: array<vec2<i32>, 8> = array<vec2<i32>, 8>(vec2<i32>(-1i, -1i), vec2<i32>(-1i, 1i), vec2<i32>(1i, -1i), vec2<i32>(1i, 1i), vec2<i32>(1i, 0i), vec2<i32>(0i, -1i), vec2<i32>(0i, 1i), vec2<i32>(-1i, 0i));
const neighborOffsets: array<vec2<i32>, 9> = array<vec2<i32>, 9>(vec2<i32>(-1i, -1i), vec2<i32>(-1i, 0i), vec2<i32>(-1i, 1i), vec2<i32>(0i, -1i), vec2<i32>(0i, 0i), vec2<i32>(0i, 1i), vec2<i32>(1i, -1i), vec2<i32>(1i, 0i), vec2<i32>(1i, 1i));

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t_4: texture_2d_array<f32>;
@group(0) @binding(2) 
var inputBuffer_s_4: sampler;
@group(0) @binding(3) 
var historyBuffer_t: texture_2d_array<f32>;
@group(0) @binding(4) 
var historyBuffer_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> outputColor_1: array<vec4<f32>, 2>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e14 = y_1;
    let _e15 = x_1;
    return exp2((_e14 * log2(_e15)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e14 = y_3;
    let _e15 = x_3;
    return exp2((_e14 * log2(_e15)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e14 = y_5;
    let _e15 = x_5;
    return exp2((_e14 * log2(_e15)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e14 = y_7;
    let _e15 = x_7;
    return exp2((_e14 * log2(_e15)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e13 = x_9;
    (*i) = trunc(_e13);
    let _e15 = x_9;
    let _e16 = (*i);
    return (_e15 - _e16);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e13 = x_11;
    (*i_1) = trunc(_e13);
    let _e15 = x_11;
    let _e16 = (*i_1);
    return (_e15 - _e16);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e13 = x_13;
    (*i_2) = trunc(_e13);
    let _e15 = x_13;
    let _e16 = (*i_2);
    return (_e15 - _e16);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e13 = x_15;
    (*i_3) = trunc(_e13);
    let _e15 = x_15;
    let _e16 = (*i_3);
    return (_e15 - _e16);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e12 = v_1;
    b = (bitcast<u32>(_e12) & 2147483647u);
    let _e17 = b;
    return (_e17 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e12 = v_3;
    b_1 = (bitcast<u32>(_e12) & 2147483647u);
    let _e17 = b_1;
    return (_e17 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e12 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e12) & vec2(2147483647u));
    let _e18 = b_2;
    return (_e18 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e12 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e12) & vec3(2147483647u));
    let _e18 = b_3;
    return (_e18 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e12 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e12) & vec4(2147483647u));
    let _e18 = b_4;
    return (_e18 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e12 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e12) & vec2(2147483647u));
    let _e18 = b_5;
    return (_e18 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e12 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e12) & vec3(2147483647u));
    let _e18 = b_6;
    return (_e18 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e12 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e12) & vec4(2147483647u));
    let _e18 = b_7;
    return (_e18 == vec4(2139095040u));
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
    let _e31 = pClip;
    vClip = (history - vec4<f32>(_e31.x, _e31.y, _e31.z, current.w));
    let _e39 = vClip;
    let _e41 = eClip;
    vUnit = (_e39.xyz / _e41);
    let _e44 = vUnit;
    aUnit = abs(_e44);
    let _e47 = aUnit;
    let _e49 = aUnit;
    let _e51 = aUnit;
    maUnit = max(_e47.x, max(_e49.y, _e51.z));
    let _e56 = maUnit;
    if (_e56 > 1f) {
        {
            let _e59 = pClip;
            let _e65 = vClip;
            let _e66 = maUnit;
            return (vec4<f32>(_e59.x, _e59.y, _e59.z, current.w) + (_e65 / vec4(_e66)));
        }
    }
    return history;
}

fn varianceClipping(inputBuffer_t: texture_2d_array<f32>, inputBuffer_s: sampler, coord: vec3<i32>, current_1: vec4<f32>, history_1: vec4<f32>, gamma: f32) -> vec4<f32> {
    var moment1_: vec4<f32>;
    var moment2_: vec4<f32>;
    var neighbor: vec4<f32>;
    var N: f32 = 9f;
    var mean: vec4<f32>;
    var varianceGamma: vec4<f32>;
    var minColor_1: vec4<f32>;
    var maxColor_1: vec4<f32>;

    moment1_ = current_1;
    moment2_ = (current_1 * current_1);
    let _e51 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e51;
    let _e52 = moment1_;
    let _e53 = neighbor;
    moment1_ = (_e52 + _e53);
    let _e55 = moment2_;
    let _e56 = neighbor;
    let _e57 = neighbor;
    moment2_ = (_e55 + (_e56 * _e57));
    let _e89 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e89;
    let _e90 = moment1_;
    let _e91 = neighbor;
    moment1_ = (_e90 + _e91);
    let _e93 = moment2_;
    let _e94 = neighbor;
    let _e95 = neighbor;
    moment2_ = (_e93 + (_e94 * _e95));
    let _e127 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e127;
    let _e128 = moment1_;
    let _e129 = neighbor;
    moment1_ = (_e128 + _e129);
    let _e131 = moment2_;
    let _e132 = neighbor;
    let _e133 = neighbor;
    moment2_ = (_e131 + (_e132 * _e133));
    let _e165 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e165;
    let _e166 = moment1_;
    let _e167 = neighbor;
    moment1_ = (_e166 + _e167);
    let _e169 = moment2_;
    let _e170 = neighbor;
    let _e171 = neighbor;
    moment2_ = (_e169 + (_e170 * _e171));
    let _e203 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e203;
    let _e204 = moment1_;
    let _e205 = neighbor;
    moment1_ = (_e204 + _e205);
    let _e207 = moment2_;
    let _e208 = neighbor;
    let _e209 = neighbor;
    moment2_ = (_e207 + (_e208 * _e209));
    let _e241 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e241;
    let _e242 = moment1_;
    let _e243 = neighbor;
    moment1_ = (_e242 + _e243);
    let _e245 = moment2_;
    let _e246 = neighbor;
    let _e247 = neighbor;
    moment2_ = (_e245 + (_e246 * _e247));
    let _e279 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e279;
    let _e280 = moment1_;
    let _e281 = neighbor;
    moment1_ = (_e280 + _e281);
    let _e283 = moment2_;
    let _e284 = neighbor;
    let _e285 = neighbor;
    moment2_ = (_e283 + (_e284 * _e285));
    let _e317 = textureLoad(inputBuffer_t, coord.xy, coord.z, 0i);
    neighbor = _e317;
    let _e318 = moment1_;
    let _e319 = neighbor;
    moment1_ = (_e318 + _e319);
    let _e321 = moment2_;
    let _e322 = neighbor;
    let _e323 = neighbor;
    moment2_ = (_e321 + (_e322 * _e323));
    let _e331 = moment1_;
    let _e332 = N;
    mean = (_e331 / vec4(_e332));
    let _e336 = moment2_;
    let _e337 = N;
    let _e340 = mean;
    let _e341 = mean;
    varianceGamma = (sqrt(max(((_e336 / vec4(_e337)) - (_e340 * _e341)), vec4(0f))) * gamma);
    let _e350 = mean;
    let _e351 = varianceGamma;
    minColor_1 = (_e350 - _e351);
    let _e354 = mean;
    let _e355 = varianceGamma;
    maxColor_1 = (_e354 + _e355);
    let _e358 = mean;
    let _e359 = minColor_1;
    let _e360 = maxColor_1;
    let _e362 = minColor_1;
    let _e363 = maxColor_1;
    let _e364 = clipAABB(clamp(_e358, _e359, _e360), history_1, _e362, _e363);
    return _e364;
}

fn varianceClipping_1(inputBuffer_t_1: texture_2d_array<f32>, inputBuffer_s_1: sampler, coord_1: vec3<i32>, current_2: vec4<f32>, history_2: vec4<f32>) -> vec4<f32> {
    let _e18 = varianceClipping(inputBuffer_t_1, inputBuffer_s_1, coord_1, current_2, history_2, 1f);
    return _e18;
}

fn varianceClipping_2(inputBuffer_t_2: texture_2d<f32>, inputBuffer_s_2: sampler, coord_2: vec2<f32>, current_3: vec4<f32>, history_3: vec4<f32>, gamma_1: f32) -> vec4<f32> {
    var moment1_1: vec4<f32>;
    var moment2_1: vec4<f32>;
    var neighbor_1: vec4<f32>;
    var N_1: f32 = 9f;
    var mean_1: vec4<f32>;
    var varianceGamma_1: vec4<f32>;
    var minColor_2: vec4<f32>;
    var maxColor_2: vec4<f32>;

    moment1_1 = current_3;
    moment2_1 = (current_3 * current_3);
    let _e25 = vec2<i32>(-1i, -1i);
    let _e48 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(-1i, -1i));
    neighbor_1 = _e48;
    let _e49 = moment1_1;
    let _e50 = neighbor_1;
    moment1_1 = (_e49 + _e50);
    let _e52 = moment2_1;
    let _e53 = neighbor_1;
    let _e54 = neighbor_1;
    moment2_1 = (_e52 + (_e53 * _e54));
    let _e63 = vec2<i32>(-1i, 1i);
    let _e83 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(-1i, 1i));
    neighbor_1 = _e83;
    let _e84 = moment1_1;
    let _e85 = neighbor_1;
    moment1_1 = (_e84 + _e85);
    let _e87 = moment2_1;
    let _e88 = neighbor_1;
    let _e89 = neighbor_1;
    moment2_1 = (_e87 + (_e88 * _e89));
    let _e101 = vec2<i32>(1i, -1i);
    let _e118 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(1i, -1i));
    neighbor_1 = _e118;
    let _e119 = moment1_1;
    let _e120 = neighbor_1;
    moment1_1 = (_e119 + _e120);
    let _e122 = moment2_1;
    let _e123 = neighbor_1;
    let _e124 = neighbor_1;
    moment2_1 = (_e122 + (_e123 * _e124));
    let _e139 = vec2<i32>(1i, 1i);
    let _e153 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(1i, 1i));
    neighbor_1 = _e153;
    let _e154 = moment1_1;
    let _e155 = neighbor_1;
    moment1_1 = (_e154 + _e155);
    let _e157 = moment2_1;
    let _e158 = neighbor_1;
    let _e159 = neighbor_1;
    moment2_1 = (_e157 + (_e158 * _e159));
    let _e177 = vec2<i32>(1i, 0i);
    let _e188 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(1i, 0i));
    neighbor_1 = _e188;
    let _e189 = moment1_1;
    let _e190 = neighbor_1;
    moment1_1 = (_e189 + _e190);
    let _e192 = moment2_1;
    let _e193 = neighbor_1;
    let _e194 = neighbor_1;
    moment2_1 = (_e192 + (_e193 * _e194));
    let _e215 = vec2<i32>(0i, -1i);
    let _e223 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(0i, -1i));
    neighbor_1 = _e223;
    let _e224 = moment1_1;
    let _e225 = neighbor_1;
    moment1_1 = (_e224 + _e225);
    let _e227 = moment2_1;
    let _e228 = neighbor_1;
    let _e229 = neighbor_1;
    moment2_1 = (_e227 + (_e228 * _e229));
    let _e253 = vec2<i32>(0i, 1i);
    let _e258 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(0i, 1i));
    neighbor_1 = _e258;
    let _e259 = moment1_1;
    let _e260 = neighbor_1;
    moment1_1 = (_e259 + _e260);
    let _e262 = moment2_1;
    let _e263 = neighbor_1;
    let _e264 = neighbor_1;
    moment2_1 = (_e262 + (_e263 * _e264));
    let _e291 = vec2<i32>(-1i, 0i);
    let _e293 = textureSample(inputBuffer_t_2, inputBuffer_s_2, coord_2, vec2<i32>(-1i, 0i));
    neighbor_1 = _e293;
    let _e294 = moment1_1;
    let _e295 = neighbor_1;
    moment1_1 = (_e294 + _e295);
    let _e297 = moment2_1;
    let _e298 = neighbor_1;
    let _e299 = neighbor_1;
    moment2_1 = (_e297 + (_e298 * _e299));
    let _e307 = moment1_1;
    let _e308 = N_1;
    mean_1 = (_e307 / vec4(_e308));
    let _e312 = moment2_1;
    let _e313 = N_1;
    let _e316 = mean_1;
    let _e317 = mean_1;
    varianceGamma_1 = (sqrt(max(((_e312 / vec4(_e313)) - (_e316 * _e317)), vec4(0f))) * gamma_1);
    let _e326 = mean_1;
    let _e327 = varianceGamma_1;
    minColor_2 = (_e326 - _e327);
    let _e330 = mean_1;
    let _e331 = varianceGamma_1;
    maxColor_2 = (_e330 + _e331);
    let _e334 = mean_1;
    let _e335 = minColor_2;
    let _e336 = maxColor_2;
    let _e338 = minColor_2;
    let _e339 = maxColor_2;
    let _e340 = clipAABB(clamp(_e334, _e335, _e336), history_3, _e338, _e339);
    return _e340;
}

fn varianceClipping_3(inputBuffer_t_3: texture_2d<f32>, inputBuffer_s_3: sampler, coord_3: vec2<f32>, current_4: vec4<f32>, history_4: vec4<f32>) -> vec4<f32> {
    let _e18 = varianceClipping_2(inputBuffer_t_3, inputBuffer_s_3, coord_3, current_4, history_4, 1f);
    return _e18;
}

fn getClosestFragment(coord_4: vec3<i32>) -> vec4<f32> {
    var result: vec4<f32> = vec4<f32>(10000000f, 0f, 0f, 0f);
    var neighbor_2: vec4<f32>;

    let _e26 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e59 = textureLoad(inputBuffer_t_4, _e26.xy, _e26.z, 0i);
    neighbor_2 = _e59;
    let _e60 = neighbor_2;
    let _e62 = result;
    if (_e60.x < _e62.x) {
        {
            let _e65 = neighbor_2;
            result = _e65;
        }
    }
    let _e70 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e103 = textureLoad(inputBuffer_t_4, _e70.xy, _e70.z, 0i);
    neighbor_2 = _e103;
    let _e104 = neighbor_2;
    let _e106 = result;
    if (_e104.x < _e106.x) {
        {
            let _e109 = neighbor_2;
            result = _e109;
        }
    }
    let _e114 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e147 = textureLoad(inputBuffer_t_4, _e114.xy, _e114.z, 0i);
    neighbor_2 = _e147;
    let _e148 = neighbor_2;
    let _e150 = result;
    if (_e148.x < _e150.x) {
        {
            let _e153 = neighbor_2;
            result = _e153;
        }
    }
    let _e158 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e191 = textureLoad(inputBuffer_t_4, _e158.xy, _e158.z, 0i);
    neighbor_2 = _e191;
    let _e192 = neighbor_2;
    let _e194 = result;
    if (_e192.x < _e194.x) {
        {
            let _e197 = neighbor_2;
            result = _e197;
        }
    }
    let _e202 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e235 = textureLoad(inputBuffer_t_4, _e202.xy, _e202.z, 0i);
    neighbor_2 = _e235;
    let _e236 = neighbor_2;
    let _e238 = result;
    if (_e236.x < _e238.x) {
        {
            let _e241 = neighbor_2;
            result = _e241;
        }
    }
    let _e246 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e279 = textureLoad(inputBuffer_t_4, _e246.xy, _e246.z, 0i);
    neighbor_2 = _e279;
    let _e280 = neighbor_2;
    let _e282 = result;
    if (_e280.x < _e282.x) {
        {
            let _e285 = neighbor_2;
            result = _e285;
        }
    }
    let _e290 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e323 = textureLoad(inputBuffer_t_4, _e290.xy, _e290.z, 0i);
    neighbor_2 = _e323;
    let _e324 = neighbor_2;
    let _e326 = result;
    if (_e324.x < _e326.x) {
        {
            let _e329 = neighbor_2;
            result = _e329;
        }
    }
    let _e334 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e367 = textureLoad(inputBuffer_t_4, _e334.xy, _e334.z, 0i);
    neighbor_2 = _e367;
    let _e368 = neighbor_2;
    let _e370 = result;
    if (_e368.x < _e370.x) {
        {
            let _e373 = neighbor_2;
            result = _e373;
        }
    }
    let _e378 = (coord_4 + vec3<i32>(0i, 0i, 2i));
    let _e411 = textureLoad(inputBuffer_t_4, _e378.xy, _e378.z, 0i);
    neighbor_2 = _e411;
    let _e412 = neighbor_2;
    let _e414 = result;
    if (_e412.x < _e414.x) {
        {
            let _e417 = neighbor_2;
            result = _e417;
        }
    }
    let _e418 = result;
    return _e418;
}

fn cascade(cascadeIndex: i32, outputColor: ptr<function, vec4<f32>>) {
    var coord_5: vec3<i32>;
    var current_5: vec4<f32>;
    var depthVelocity: vec4<f32>;
    var velocity: vec2<f32>;
    var prevUv: vec2<f32>;
    var history_5: vec4<f32>;
    var clippedHistory: vec4<f32>;

    let _e17 = gl_FragCoord_1;
    let _e19 = vec2<i32>(_e17.xy);
    coord_5 = vec3<i32>(_e19.x, _e19.y, cascadeIndex);
    let _e24 = coord_5;
    let _e28 = textureLoad(inputBuffer_t_4, _e24.xy, _e24.z, 0i);
    current_5 = _e28;
    let _e30 = coord_5;
    let _e31 = getClosestFragment(_e30);
    depthVelocity = _e31;
    let _e33 = depthVelocity;
    let _e35 = global.texelSize;
    velocity = (_e33.yz * _e35);
    let _e38 = vUv_1;
    let _e39 = velocity;
    prevUv = (_e38 - _e39);
    let _e42 = prevUv;
    let _e46 = prevUv;
    let _e51 = prevUv;
    let _e56 = prevUv;
    if ((((_e42.x < 0f) || (_e46.x > 1f)) || (_e51.y < 0f)) || (_e56.y > 1f)) {
        {
            let _e61 = current_5;
            (*outputColor) = _e61;
            return;
        }
    }
    let _e62 = prevUv;
    let _e66 = vec3<f32>(_e62.x, _e62.y, f32(cascadeIndex));
    let _e70 = textureSample(historyBuffer_t, historyBuffer_s, _e66.xy, i32(_e66.z));
    history_5 = _e70;
    let _e72 = coord_5;
    let _e73 = current_5;
    let _e74 = history_5;
    let _e75 = global.varianceGamma;
    let _e76 = varianceClipping(inputBuffer_t_4, inputBuffer_s_4, _e72, _e73, _e74, _e75);
    clippedHistory = _e76;
    let _e78 = clippedHistory;
    let _e79 = current_5;
    let _e80 = global.temporalAlpha;
    (*outputColor) = mix(_e78, _e79, vec4(_e80));
    return;
}

fn main_1() {
    var local: vec4<f32>;
    var local_1: vec4<f32>;

    cascade(0i, (&local));
    let _e23 = local;
    outputColor_1[0i] = _e23;
    cascade(1i, (&local_1));
    let _e32 = local_1;
    outputColor_1[1i] = _e32;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e28 = outputColor_1[0];
    let _e30 = outputColor_1[1];
    return FragmentOutput(_e28, _e30);
}
