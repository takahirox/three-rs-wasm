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

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @location(1) vUv00_: vec2<f32>,
    @location(2) vUv01_: vec2<f32>,
    @location(3) vUv02_: vec2<f32>,
    @location(4) vUv03_: vec2<f32>,
    @location(5) vUv04_: vec2<f32>,
    @location(6) vUv05_: vec2<f32>,
    @location(7) vUv06_: vec2<f32>,
    @location(8) vUv07_: vec2<f32>,
    @location(9) vUv08_: vec2<f32>,
    @location(10) vUv09_: vec2<f32>,
    @location(11) vUv10_: vec2<f32>,
    @location(12) vUv11_: vec2<f32>,
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
var<private> uv_1: vec2<f32>;
var<private> vUv: vec2<f32>;
var<private> vUv00_: vec2<f32>;
var<private> vUv01_: vec2<f32>;
var<private> vUv02_: vec2<f32>;
var<private> vUv03_: vec2<f32>;
var<private> vUv04_: vec2<f32>;
var<private> vUv05_: vec2<f32>;
var<private> vUv06_: vec2<f32>;
var<private> vUv07_: vec2<f32>;
var<private> vUv08_: vec2<f32>;
var<private> vUv09_: vec2<f32>;
var<private> vUv10_: vec2<f32>;
var<private> vUv11_: vec2<f32>;
var<private> gl_Position: vec4<f32>;

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

fn pt_main() {
    let _e34 = position_1;
    vUv = ((_e34.xy * 0.5f) + vec2(0.5f));
    let _e41 = vUv;
    let _e42 = global.texelSize;
    vUv00_ = (_e41 + (_e42 * vec2<f32>(-1f, 1f)));
    let _e49 = vUv;
    let _e50 = global.texelSize;
    vUv01_ = (_e49 + (_e50 * vec2<f32>(1f, 1f)));
    let _e56 = vUv;
    let _e57 = global.texelSize;
    vUv02_ = (_e56 + (_e57 * vec2<f32>(-1f, -1f)));
    let _e65 = vUv;
    let _e66 = global.texelSize;
    vUv03_ = (_e65 + (_e66 * vec2<f32>(1f, -1f)));
    let _e73 = vUv;
    let _e74 = global.texelSize;
    vUv04_ = (_e73 + (_e74 * vec2<f32>(-2f, 2f)));
    let _e81 = vUv;
    let _e82 = global.texelSize;
    vUv05_ = (_e81 + (_e82 * vec2<f32>(0f, 2f)));
    let _e88 = vUv;
    let _e89 = global.texelSize;
    vUv06_ = (_e88 + (_e89 * vec2<f32>(2f, 2f)));
    let _e95 = vUv;
    let _e96 = global.texelSize;
    vUv07_ = (_e95 + (_e96 * vec2<f32>(-2f, 0f)));
    let _e103 = vUv;
    let _e104 = global.texelSize;
    vUv08_ = (_e103 + (_e104 * vec2<f32>(2f, 0f)));
    let _e110 = vUv;
    let _e111 = global.texelSize;
    vUv09_ = (_e110 + (_e111 * vec2<f32>(-2f, -2f)));
    let _e119 = vUv;
    let _e120 = global.texelSize;
    vUv10_ = (_e119 + (_e120 * vec2<f32>(0f, -2f)));
    let _e127 = vUv;
    let _e128 = global.texelSize;
    vUv11_ = (_e127 + (_e128 * vec2<f32>(2f, -2f)));
    let _e136 = position_1;
    let _e137 = _e136.xy;
    gl_Position = vec4<f32>(_e137.x, _e137.y, 1f, 1f);
    return;
}

fn main_1() {
    pt_main();
    let _e36 = gl_Position;
    gl_Position.y = -(_e36.y);
    let _e40 = gl_Position;
    let _e42 = gl_Position;
    gl_Position.z = ((_e40.z + _e42.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_1 = normal;
    uv_1 = uv;
    main_1();
    let _e59 = vUv;
    let _e61 = vUv00_;
    let _e63 = vUv01_;
    let _e65 = vUv02_;
    let _e67 = vUv03_;
    let _e69 = vUv04_;
    let _e71 = vUv05_;
    let _e73 = vUv06_;
    let _e75 = vUv07_;
    let _e77 = vUv08_;
    let _e79 = vUv09_;
    let _e81 = vUv10_;
    let _e83 = vUv11_;
    let _e85 = gl_Position;
    return VertexOutput(_e59, _e61, _e63, _e65, _e67, _e69, _e71, _e73, _e75, _e77, _e79, _e81, _e83, _e85);
}
