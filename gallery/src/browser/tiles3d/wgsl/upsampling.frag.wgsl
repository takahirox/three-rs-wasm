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
    radius: f32,
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
@group(0) @binding(3) 
var supportBuffer_t: texture_2d<f32>;
@group(0) @binding(4) 
var supportBuffer_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> vUv0_1: vec2<f32>;
var<private> vUv1_1: vec2<f32>;
var<private> vUv2_1: vec2<f32>;
var<private> vUv3_1: vec2<f32>;
var<private> vUv4_1: vec2<f32>;
var<private> vUv5_1: vec2<f32>;
var<private> vUv6_1: vec2<f32>;
var<private> vUv7_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e26 = y_1;
    let _e27 = x_1;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e26 = y_3;
    let _e27 = x_3;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e26 = y_5;
    let _e27 = x_5;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e26 = y_7;
    let _e27 = x_7;
    return exp2((_e26 * log2(_e27)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e25 = x_9;
    (*i) = trunc(_e25);
    let _e27 = x_9;
    let _e28 = (*i);
    return (_e27 - _e28);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e25 = x_11;
    (*i_1) = trunc(_e25);
    let _e27 = x_11;
    let _e28 = (*i_1);
    return (_e27 - _e28);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e25 = x_13;
    (*i_2) = trunc(_e25);
    let _e27 = x_13;
    let _e28 = (*i_2);
    return (_e27 - _e28);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e25 = x_15;
    (*i_3) = trunc(_e25);
    let _e27 = x_15;
    let _e28 = (*i_3);
    return (_e27 - _e28);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e24 = v_1;
    b = (bitcast<u32>(_e24) & 2147483647u);
    let _e29 = b;
    return (_e29 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e24 = v_3;
    b_1 = (bitcast<u32>(_e24) & 2147483647u);
    let _e29 = b_1;
    return (_e29 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e24 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e24) & vec2(2147483647u));
    let _e30 = b_2;
    return (_e30 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e24 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e24) & vec3(2147483647u));
    let _e30 = b_3;
    return (_e30 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e24 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e24) & vec4(2147483647u));
    let _e30 = b_4;
    return (_e30 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e24 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e24) & vec2(2147483647u));
    let _e30 = b_5;
    return (_e30 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e24 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e24) & vec3(2147483647u));
    let _e30 = b_6;
    return (_e30 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e24 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e24) & vec4(2147483647u));
    let _e30 = b_7;
    return (_e30 == vec4(2139095040u));
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
    let _e51 = value_7;
    let _e53 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e51.xyz);
    let _e54 = value_7;
    let _e60 = LinearTransferOETF(vec4<f32>(_e53.x, _e53.y, _e53.z, _e54.w));
    return _e60;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e39 = weights;
    let _e40 = rgb_1;
    return dot(_e39, _e40);
}

fn main_1() {
    var c: vec4<f32> = vec4(0f);
    var baseColor: vec4<f32>;

    let _e35 = c;
    let _e36 = vUv0_1;
    let _e37 = textureSample(inputBuffer_t, inputBuffer_s, _e36);
    c = (_e35 + (_e37 * 0.0625f));
    let _e41 = c;
    let _e42 = vUv1_1;
    let _e43 = textureSample(inputBuffer_t, inputBuffer_s, _e42);
    c = (_e41 + (_e43 * 0.125f));
    let _e47 = c;
    let _e48 = vUv2_1;
    let _e49 = textureSample(inputBuffer_t, inputBuffer_s, _e48);
    c = (_e47 + (_e49 * 0.0625f));
    let _e53 = c;
    let _e54 = vUv3_1;
    let _e55 = textureSample(inputBuffer_t, inputBuffer_s, _e54);
    c = (_e53 + (_e55 * 0.125f));
    let _e59 = c;
    let _e60 = vUv_1;
    let _e61 = textureSample(inputBuffer_t, inputBuffer_s, _e60);
    c = (_e59 + (_e61 * 0.25f));
    let _e65 = c;
    let _e66 = vUv4_1;
    let _e67 = textureSample(inputBuffer_t, inputBuffer_s, _e66);
    c = (_e65 + (_e67 * 0.125f));
    let _e71 = c;
    let _e72 = vUv5_1;
    let _e73 = textureSample(inputBuffer_t, inputBuffer_s, _e72);
    c = (_e71 + (_e73 * 0.0625f));
    let _e77 = c;
    let _e78 = vUv6_1;
    let _e79 = textureSample(inputBuffer_t, inputBuffer_s, _e78);
    c = (_e77 + (_e79 * 0.125f));
    let _e83 = c;
    let _e84 = vUv7_1;
    let _e85 = textureSample(inputBuffer_t, inputBuffer_s, _e84);
    c = (_e83 + (_e85 * 0.0625f));
    let _e89 = vUv_1;
    let _e90 = textureSample(supportBuffer_t, supportBuffer_s, _e89);
    baseColor = _e90;
    let _e92 = baseColor;
    let _e93 = c;
    let _e94 = global.radius;
    pc_fragColor = mix(_e92, _e93, vec4(_e94));
    let _e97 = pc_fragColor;
    let _e98 = linearToOutputTexel(_e97);
    pc_fragColor = _e98;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vUv0_: vec2<f32>, @location(2) vUv1_: vec2<f32>, @location(3) vUv2_: vec2<f32>, @location(4) vUv3_: vec2<f32>, @location(5) vUv4_: vec2<f32>, @location(6) vUv5_: vec2<f32>, @location(7) vUv6_: vec2<f32>, @location(8) vUv7_: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vUv0_1 = vUv0_;
    vUv1_1 = vUv1_;
    vUv2_1 = vUv2_;
    vUv3_1 = vUv3_;
    vUv4_1 = vUv4_;
    vUv5_1 = vUv5_;
    vUv6_1 = vUv6_;
    vUv7_1 = vUv7_;
    main_1();
    let _e65 = pc_fragColor;
    return FragmentOutput(_e65);
}
