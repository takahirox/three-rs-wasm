diagnostic(off, derivative_uniformity);
struct IncidentLight {
    color: vec3<f32>,
    direction: vec3<f32>,
    visible: bool,
}

struct ReflectedLight {
    directDiffuse: vec3<f32>,
    directSpecular: vec3<f32>,
    indirectDiffuse: vec3<f32>,
    indirectSpecular: vec3<f32>,
}

struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    texelSize: vec2<f32>,
    thresholdLevel: f32,
    thresholdRange: f32,
}

struct VertexOutput {
    @location(0) vCenterUv1_: vec2<f32>,
    @location(1) vCenterUv2_: vec2<f32>,
    @location(2) vCenterUv3_: vec2<f32>,
    @location(3) vCenterUv4_: vec2<f32>,
    @location(4) vRowUv1_: vec2<f32>,
    @location(5) vRowUv2_: vec2<f32>,
    @location(6) vRowUv3_: vec2<f32>,
    @location(7) vRowUv4_: vec2<f32>,
    @location(8) vRowUv5_: vec2<f32>,
    @location(9) vRowUv6_: vec2<f32>,
    @location(10) vRowUv7_: vec2<f32>,
    @location(11) vRowUv8_: vec2<f32>,
    @location(12) vRowUv9_: vec2<f32>,
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
var<private> vCenterUv1_: vec2<f32>;
var<private> vCenterUv2_: vec2<f32>;
var<private> vCenterUv3_: vec2<f32>;
var<private> vCenterUv4_: vec2<f32>;
var<private> vRowUv1_: vec2<f32>;
var<private> vRowUv2_: vec2<f32>;
var<private> vRowUv3_: vec2<f32>;
var<private> vRowUv4_: vec2<f32>;
var<private> vRowUv5_: vec2<f32>;
var<private> vRowUv6_: vec2<f32>;
var<private> vRowUv7_: vec2<f32>;
var<private> vRowUv8_: vec2<f32>;
var<private> vRowUv9_: vec2<f32>;
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

    let _e38 = position_1;
    uv_1 = ((_e38.xy * 0.5f) + vec2(0.5f));
    let _e46 = uv_1;
    let _e47 = global.texelSize;
    vCenterUv1_ = (_e46 + (_e47 * vec2<f32>(-1f, 1f)));
    let _e54 = uv_1;
    let _e55 = global.texelSize;
    vCenterUv2_ = (_e54 + (_e55 * vec2<f32>(1f, 1f)));
    let _e61 = uv_1;
    let _e62 = global.texelSize;
    vCenterUv3_ = (_e61 + (_e62 * vec2<f32>(-1f, -1f)));
    let _e70 = uv_1;
    let _e71 = global.texelSize;
    vCenterUv4_ = (_e70 + (_e71 * vec2<f32>(1f, -1f)));
    let _e78 = uv_1;
    let _e79 = global.texelSize;
    vRowUv1_ = (_e78 + (_e79 * vec2<f32>(-2f, 2f)));
    let _e86 = uv_1;
    let _e87 = global.texelSize;
    vRowUv2_ = (_e86 + (_e87 * vec2<f32>(0f, 2f)));
    let _e93 = uv_1;
    let _e94 = global.texelSize;
    vRowUv3_ = (_e93 + (_e94 * vec2<f32>(2f, 2f)));
    let _e100 = uv_1;
    let _e101 = global.texelSize;
    vRowUv4_ = (_e100 + (_e101 * vec2<f32>(-2f, 0f)));
    let _e108 = uv_1;
    let _e109 = global.texelSize;
    vRowUv5_ = (_e108 + _e109);
    let _e111 = uv_1;
    let _e112 = global.texelSize;
    vRowUv6_ = (_e111 + (_e112 * vec2<f32>(2f, 0f)));
    let _e118 = uv_1;
    let _e119 = global.texelSize;
    vRowUv7_ = (_e118 + (_e119 * vec2<f32>(-2f, -2f)));
    let _e127 = uv_1;
    let _e128 = global.texelSize;
    vRowUv8_ = (_e127 + (_e128 * vec2<f32>(0f, -2f)));
    let _e135 = uv_1;
    let _e136 = global.texelSize;
    vRowUv9_ = (_e135 + (_e136 * vec2<f32>(2f, -2f)));
    let _e144 = position_1;
    let _e145 = _e144.xy;
    gl_Position = vec4<f32>(_e145.x, _e145.y, 1f, 1f);
    return;
}

fn main_1() {
    pt_main();
    let _e40 = gl_Position;
    gl_Position.y = -(_e40.y);
    let _e44 = gl_Position;
    let _e46 = gl_Position;
    gl_Position.z = ((_e44.z + _e46.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_1 = normal;
    uv_2 = uv;
    main_1();
    let _e63 = vCenterUv1_;
    let _e65 = vCenterUv2_;
    let _e67 = vCenterUv3_;
    let _e69 = vCenterUv4_;
    let _e71 = vRowUv1_;
    let _e73 = vRowUv2_;
    let _e75 = vRowUv3_;
    let _e77 = vRowUv4_;
    let _e79 = vRowUv5_;
    let _e81 = vRowUv6_;
    let _e83 = vRowUv7_;
    let _e85 = vRowUv8_;
    let _e87 = vRowUv9_;
    let _e89 = gl_Position;
    return VertexOutput(_e63, _e65, _e67, _e69, _e71, _e73, _e75, _e77, _e79, _e81, _e83, _e85, _e87, _e89);
}
