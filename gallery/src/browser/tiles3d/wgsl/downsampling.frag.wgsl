diagnostic(off, derivative_uniformity);
struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    texelSize: vec2<f32>,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> vUv00_1: vec2<f32>;
var<private> vUv01_1: vec2<f32>;
var<private> vUv02_1: vec2<f32>;
var<private> vUv03_1: vec2<f32>;
var<private> vUv04_1: vec2<f32>;
var<private> vUv05_1: vec2<f32>;
var<private> vUv06_1: vec2<f32>;
var<private> vUv07_1: vec2<f32>;
var<private> vUv08_1: vec2<f32>;
var<private> vUv09_1: vec2<f32>;
var<private> vUv10_1: vec2<f32>;
var<private> vUv11_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e22 = y_1;
    let _e23 = x_1;
    return exp2((_e22 * log2(_e23)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e22 = y_3;
    let _e23 = x_3;
    return exp2((_e22 * log2(_e23)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e22 = y_5;
    let _e23 = x_5;
    return exp2((_e22 * log2(_e23)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e22 = y_7;
    let _e23 = x_7;
    return exp2((_e22 * log2(_e23)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e21 = x_9;
    (*i) = trunc(_e21);
    let _e23 = x_9;
    let _e24 = (*i);
    return (_e23 - _e24);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e21 = x_11;
    (*i_1) = trunc(_e21);
    let _e23 = x_11;
    let _e24 = (*i_1);
    return (_e23 - _e24);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e21 = x_13;
    (*i_2) = trunc(_e21);
    let _e23 = x_13;
    let _e24 = (*i_2);
    return (_e23 - _e24);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e21 = x_15;
    (*i_3) = trunc(_e21);
    let _e23 = x_15;
    let _e24 = (*i_3);
    return (_e23 - _e24);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e20 = v_1;
    b = (bitcast<u32>(_e20) & 2147483647u);
    let _e25 = b;
    return (_e25 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e20 = v_3;
    b_1 = (bitcast<u32>(_e20) & 2147483647u);
    let _e25 = b_1;
    return (_e25 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e20 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e20) & vec2(2147483647u));
    let _e26 = b_2;
    return (_e26 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e20 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e20) & vec3(2147483647u));
    let _e26 = b_3;
    return (_e26 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e20 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e20) & vec4(2147483647u));
    let _e26 = b_4;
    return (_e26 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e20 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e20) & vec2(2147483647u));
    let _e26 = b_5;
    return (_e26 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e20 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e20) & vec3(2147483647u));
    let _e26 = b_6;
    return (_e26 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e20 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e20) & vec4(2147483647u));
    let _e26 = b_7;
    return (_e26 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e34 = value_1;
    return _e34;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e34 = value_3;
    let _e43 = pt_pow_2(((_e34.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e44 = value_3;
    let _e48 = value_3;
    let _e58 = mix(_e43, (_e44.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e48.xyz <= vec3(0.04045f))));
    let _e59 = value_3;
    return vec4<f32>(_e58.x, _e58.y, _e58.z, _e59.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e34 = value_5;
    let _e38 = pt_pow_2(_e34.xyz, vec3(0.41666f));
    let _e44 = value_5;
    let _e48 = value_5;
    let _e58 = mix(((_e38 * 1.055f) - vec3(0.055f)), (_e44.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e48.xyz <= vec3(0.0031308f))));
    let _e59 = value_5;
    return vec4<f32>(_e58.x, _e58.y, _e58.z, _e59.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e34 = value_7;
    let _e52 = (_e34.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e53 = value_7;
    let _e59 = LinearTransferOETF(vec4<f32>(_e52.x, _e52.y, _e52.z, _e53.w));
    return _e59;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e39 = weights;
    let _e40 = rgb_1;
    return dot(_e39, _e40);
}

fn clampToBorder(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;

    uv_1 = uv;
    let _e34 = uv_1;
    let _e38 = uv_1;
    let _e43 = uv_1;
    let _e48 = uv_1;
    return select(0f, 1f, ((((_e34.x >= 0f) && (_e38.x <= 1f)) && (_e43.y >= 0f)) && (_e48.y <= 1f)));
}

fn main_1() {
    var c: vec4<f32> = vec4(0f);
    var w: vec4<f32>;

    let _e36 = vUv00_1;
    let _e37 = clampToBorder(_e36);
    let _e38 = vUv01_1;
    let _e39 = clampToBorder(_e38);
    let _e40 = vUv02_1;
    let _e41 = clampToBorder(_e40);
    let _e42 = vUv03_1;
    let _e43 = clampToBorder(_e42);
    w = (0.125f * vec4<f32>(_e37, _e39, _e41, _e43));
    let _e47 = c;
    let _e48 = w;
    let _e50 = vUv00_1;
    let _e51 = textureSample(inputBuffer_t, inputBuffer_s, _e50);
    c = (_e47 + (_e48.x * _e51));
    let _e54 = c;
    let _e55 = w;
    let _e57 = vUv01_1;
    let _e58 = textureSample(inputBuffer_t, inputBuffer_s, _e57);
    c = (_e54 + (_e55.y * _e58));
    let _e61 = c;
    let _e62 = w;
    let _e64 = vUv02_1;
    let _e65 = textureSample(inputBuffer_t, inputBuffer_s, _e64);
    c = (_e61 + (_e62.z * _e65));
    let _e68 = c;
    let _e69 = w;
    let _e71 = vUv03_1;
    let _e72 = textureSample(inputBuffer_t, inputBuffer_s, _e71);
    c = (_e68 + (_e69.w * _e72));
    let _e76 = vUv04_1;
    let _e77 = clampToBorder(_e76);
    let _e78 = vUv05_1;
    let _e79 = clampToBorder(_e78);
    let _e80 = vUv06_1;
    let _e81 = clampToBorder(_e80);
    let _e82 = vUv07_1;
    let _e83 = clampToBorder(_e82);
    w = (0.05556f * vec4<f32>(_e77, _e79, _e81, _e83));
    let _e86 = c;
    let _e87 = w;
    let _e89 = vUv04_1;
    let _e90 = textureSample(inputBuffer_t, inputBuffer_s, _e89);
    c = (_e86 + (_e87.x * _e90));
    let _e93 = c;
    let _e94 = w;
    let _e96 = vUv05_1;
    let _e97 = textureSample(inputBuffer_t, inputBuffer_s, _e96);
    c = (_e93 + (_e94.y * _e97));
    let _e100 = c;
    let _e101 = w;
    let _e103 = vUv06_1;
    let _e104 = textureSample(inputBuffer_t, inputBuffer_s, _e103);
    c = (_e100 + (_e101.z * _e104));
    let _e107 = c;
    let _e108 = w;
    let _e110 = vUv07_1;
    let _e111 = textureSample(inputBuffer_t, inputBuffer_s, _e110);
    c = (_e107 + (_e108.w * _e111));
    let _e115 = vUv08_1;
    let _e116 = clampToBorder(_e115);
    let _e117 = vUv09_1;
    let _e118 = clampToBorder(_e117);
    let _e119 = vUv10_1;
    let _e120 = clampToBorder(_e119);
    let _e121 = vUv11_1;
    let _e122 = clampToBorder(_e121);
    w = (0.05556f * vec4<f32>(_e116, _e118, _e120, _e122));
    let _e125 = c;
    let _e126 = w;
    let _e128 = vUv08_1;
    let _e129 = textureSample(inputBuffer_t, inputBuffer_s, _e128);
    c = (_e125 + (_e126.x * _e129));
    let _e132 = c;
    let _e133 = w;
    let _e135 = vUv09_1;
    let _e136 = textureSample(inputBuffer_t, inputBuffer_s, _e135);
    c = (_e132 + (_e133.y * _e136));
    let _e139 = c;
    let _e140 = w;
    let _e142 = vUv10_1;
    let _e143 = textureSample(inputBuffer_t, inputBuffer_s, _e142);
    c = (_e139 + (_e140.z * _e143));
    let _e146 = c;
    let _e147 = w;
    let _e149 = vUv11_1;
    let _e150 = textureSample(inputBuffer_t, inputBuffer_s, _e149);
    c = (_e146 + (_e147.w * _e150));
    let _e153 = c;
    let _e155 = vUv_1;
    let _e156 = textureSample(inputBuffer_t, inputBuffer_s, _e155);
    c = (_e153 + (0.05556f * _e156));
    let _e159 = c;
    pc_fragColor = _e159;
    let _e160 = pc_fragColor;
    let _e161 = linearToOutputTexel(_e160);
    pc_fragColor = _e161;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vUv00_: vec2<f32>, @location(2) vUv01_: vec2<f32>, @location(3) vUv02_: vec2<f32>, @location(4) vUv03_: vec2<f32>, @location(5) vUv04_: vec2<f32>, @location(6) vUv05_: vec2<f32>, @location(7) vUv06_: vec2<f32>, @location(8) vUv07_: vec2<f32>, @location(9) vUv08_: vec2<f32>, @location(10) vUv09_: vec2<f32>, @location(11) vUv10_: vec2<f32>, @location(12) vUv11_: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vUv00_1 = vUv00_;
    vUv01_1 = vUv01_;
    vUv02_1 = vUv02_;
    vUv03_1 = vUv03_;
    vUv04_1 = vUv04_;
    vUv05_1 = vUv05_;
    vUv06_1 = vUv06_;
    vUv07_1 = vUv07_;
    vUv08_1 = vUv08_;
    vUv09_1 = vUv09_;
    vUv10_1 = vUv10_;
    vUv11_1 = vUv11_;
    main_1();
    let _e75 = pc_fragColor;
    return FragmentOutput(_e75);
}
