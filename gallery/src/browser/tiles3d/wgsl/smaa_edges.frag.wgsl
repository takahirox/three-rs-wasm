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
var<private> vUv0_1: vec2<f32>;
var<private> vUv1_1: vec2<f32>;
var<private> vUv2_1: vec2<f32>;
var<private> vUv3_1: vec2<f32>;
var<private> vUv4_1: vec2<f32>;
var<private> vUv5_1: vec2<f32>;
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
    let _e28 = value_1;
    return _e28;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e28 = value_3;
    let _e37 = pt_pow_2(((_e28.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e38 = value_3;
    let _e42 = value_3;
    let _e52 = mix(_e37, (_e38.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e42.xyz <= vec3(0.04045f))));
    let _e53 = value_3;
    return vec4<f32>(_e52.x, _e52.y, _e52.z, _e53.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e28 = value_5;
    let _e32 = pt_pow_2(_e28.xyz, vec3(0.41666f));
    let _e38 = value_5;
    let _e42 = value_5;
    let _e52 = mix(((_e32 * 1.055f) - vec3(0.055f)), (_e38.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e42.xyz <= vec3(0.0031308f))));
    let _e53 = value_5;
    return vec4<f32>(_e52.x, _e52.y, _e52.z, _e53.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e28 = value_7;
    let _e46 = (_e28.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e47 = value_7;
    let _e53 = LinearTransferOETF(vec4<f32>(_e46.x, _e46.y, _e46.z, _e47.w));
    return _e53;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e33 = weights;
    let _e34 = rgb_1;
    return dot(_e33, _e34);
}

fn main_1() {
    var threshold: vec2<f32> = vec2(0.1f);
    var delta: vec4<f32>;
    var c: vec3<f32>;
    var cLeft: vec3<f32>;
    var t: vec3<f32>;
    var cTop: vec3<f32>;
    var edges: vec2<f32>;
    var cRight: vec3<f32>;
    var cBottom: vec3<f32>;
    var maxDelta: vec2<f32>;
    var cLeftLeft: vec3<f32>;
    var cTopTop: vec3<f32>;
    var finalDelta: f32;

    let _e30 = vUv_1;
    let _e31 = textureSample(inputBuffer_t, inputBuffer_s, _e30);
    c = _e31.xyz;
    let _e34 = vUv0_1;
    let _e35 = textureSample(inputBuffer_t, inputBuffer_s, _e34);
    cLeft = _e35.xyz;
    let _e38 = c;
    let _e39 = cLeft;
    t = abs((_e38 - _e39));
    let _e44 = t;
    let _e46 = t;
    let _e49 = t;
    delta.x = max(max(_e44.x, _e46.y), _e49.z);
    let _e52 = vUv1_1;
    let _e53 = textureSample(inputBuffer_t, inputBuffer_s, _e52);
    cTop = _e53.xyz;
    let _e56 = c;
    let _e57 = cTop;
    t = abs((_e56 - _e57));
    let _e61 = t;
    let _e63 = t;
    let _e66 = t;
    delta.y = max(max(_e61.x, _e63.y), _e66.z);
    let _e69 = threshold;
    let _e70 = delta;
    edges = step(_e69, _e70.xy);
    let _e74 = edges;
    if (dot(_e74, vec2(1f)) == 0f) {
        {
            discard;
        }
    }
    let _e80 = vUv2_1;
    let _e81 = textureSample(inputBuffer_t, inputBuffer_s, _e80);
    cRight = _e81.xyz;
    let _e84 = c;
    let _e85 = cRight;
    t = abs((_e84 - _e85));
    let _e89 = t;
    let _e91 = t;
    let _e94 = t;
    delta.z = max(max(_e89.x, _e91.y), _e94.z);
    let _e97 = vUv3_1;
    let _e98 = textureSample(inputBuffer_t, inputBuffer_s, _e97);
    cBottom = _e98.xyz;
    let _e101 = c;
    let _e102 = cBottom;
    t = abs((_e101 - _e102));
    let _e106 = t;
    let _e108 = t;
    let _e111 = t;
    delta.w = max(max(_e106.x, _e108.y), _e111.z);
    let _e114 = delta;
    let _e116 = delta;
    maxDelta = max(_e114.xy, _e116.zw);
    let _e120 = vUv4_1;
    let _e121 = textureSample(inputBuffer_t, inputBuffer_s, _e120);
    cLeftLeft = _e121.xyz;
    let _e124 = c;
    let _e125 = cLeftLeft;
    t = abs((_e124 - _e125));
    let _e129 = t;
    let _e131 = t;
    let _e134 = t;
    delta.z = max(max(_e129.x, _e131.y), _e134.z);
    let _e137 = vUv5_1;
    let _e138 = textureSample(inputBuffer_t, inputBuffer_s, _e137);
    cTopTop = _e138.xyz;
    let _e141 = c;
    let _e142 = cTopTop;
    t = abs((_e141 - _e142));
    let _e146 = t;
    let _e148 = t;
    let _e151 = t;
    delta.w = max(max(_e146.x, _e148.y), _e151.z);
    let _e154 = maxDelta;
    let _e156 = delta;
    maxDelta = max(_e154.xy, _e156.zw);
    let _e159 = maxDelta;
    let _e161 = maxDelta;
    finalDelta = max(_e159.x, _e161.y);
    let _e165 = edges;
    let _e166 = finalDelta;
    let _e168 = delta;
    edges = (_e165 * step(vec2(_e166), (2f * _e168.xy)));
    let _e174 = edges;
    pc_fragColor = vec4<f32>(_e174.x, _e174.y, 0f, 1f);
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vUv0_: vec2<f32>, @location(2) vUv1_: vec2<f32>, @location(3) vUv2_: vec2<f32>, @location(4) vUv3_: vec2<f32>, @location(5) vUv4_: vec2<f32>, @location(6) vUv5_: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vUv0_1 = vUv0_;
    vUv1_1 = vUv1_;
    vUv2_1 = vUv2_;
    vUv3_1 = vUv3_;
    vUv4_1 = vUv4_;
    vUv5_1 = vUv5_;
    main_1();
    let _e51 = pc_fragColor;
    return FragmentOutput(_e51);
}
