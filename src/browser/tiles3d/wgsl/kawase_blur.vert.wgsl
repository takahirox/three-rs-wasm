diagnostic(off, derivative_uniformity);
struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    texelSize: vec4<f32>,
    kernel: f32,
    scale: f32,
}

struct VertexOutput {
    @location(0) vUv0_: vec2<f32>,
    @location(1) vUv1_: vec2<f32>,
    @location(2) vUv2_: vec2<f32>,
    @location(3) vUv3_: vec2<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
var<private> position_1: vec3<f32>;
var<private> normal_1: vec3<f32>;
var<private> uv_2: vec2<f32>;
var<private> vUv0_: vec2<f32>;
var<private> vUv1_: vec2<f32>;
var<private> vUv2_: vec2<f32>;
var<private> vUv3_: vec2<f32>;
var<private> gl_Position: vec4<f32>;

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

fn pt_main() {
    var uv_1: vec2<f32>;
    var dUv: vec2<f32>;

    let _e29 = position_1;
    uv_1 = ((_e29.xy * 0.5f) + vec2(0.5f));
    let _e37 = global.texelSize;
    let _e39 = global.kernel;
    let _e42 = global.texelSize;
    let _e45 = global.scale;
    dUv = (((_e37.xy * vec2(_e39)) + _e42.zw) * _e45);
    let _e48 = uv_1;
    let _e50 = dUv;
    let _e53 = uv_1;
    let _e55 = dUv;
    vUv0_ = vec2<f32>((_e48.x - _e50.x), (_e53.y + _e55.y));
    let _e59 = uv_1;
    let _e61 = dUv;
    let _e64 = uv_1;
    let _e66 = dUv;
    vUv1_ = vec2<f32>((_e59.x + _e61.x), (_e64.y + _e66.y));
    let _e70 = uv_1;
    let _e72 = dUv;
    let _e75 = uv_1;
    let _e77 = dUv;
    vUv2_ = vec2<f32>((_e70.x + _e72.x), (_e75.y - _e77.y));
    let _e81 = uv_1;
    let _e83 = dUv;
    let _e86 = uv_1;
    let _e88 = dUv;
    vUv3_ = vec2<f32>((_e81.x - _e83.x), (_e86.y - _e88.y));
    let _e93 = position_1;
    let _e94 = _e93.xy;
    gl_Position = vec4<f32>(_e94.x, _e94.y, 1f, 1f);
    return;
}

fn main_1() {
    pt_main();
    let _e31 = gl_Position;
    gl_Position.y = -(_e31.y);
    let _e35 = gl_Position;
    let _e37 = gl_Position;
    gl_Position.z = ((_e35.z + _e37.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_1 = normal;
    uv_2 = uv;
    main_1();
    let _e45 = vUv0_;
    let _e47 = vUv1_;
    let _e49 = vUv2_;
    let _e51 = vUv3_;
    let _e53 = gl_Position;
    return VertexOutput(_e45, _e47, _e49, _e51, _e53);
}
