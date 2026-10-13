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
    resolution: vec2<f32>,
}

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @location(1) vOffset: vec4<f32>,
    @location(2) vOffset_1: vec4<f32>,
    @location(3) vOffset_2: vec4<f32>,
    @location(4) vPixCoord: vec2<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
@group(0) @binding(3) 
var areaTexture_t: texture_2d<f32>;
@group(0) @binding(4) 
var areaTexture_s: sampler;
@group(0) @binding(5) 
var searchTexture_t: texture_2d<f32>;
@group(0) @binding(6) 
var searchTexture_s: sampler;
var<private> position_1: vec3<f32>;
var<private> normal_1: vec3<f32>;
var<private> uv_1: vec2<f32>;
var<private> vUv: vec2<f32>;
var<private> vOffset: array<vec4<f32>, 3>;
var<private> vPixCoord: vec2<f32>;
var<private> gl_Position: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e28 = y_1;
    let _e29 = x_1;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e28 = y_3;
    let _e29 = x_3;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e28 = y_5;
    let _e29 = x_5;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e28 = y_7;
    let _e29 = x_7;
    return exp2((_e28 * log2(_e29)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e27 = x_9;
    (*i) = trunc(_e27);
    let _e29 = x_9;
    let _e30 = (*i);
    return (_e29 - _e30);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e27 = x_11;
    (*i_1) = trunc(_e27);
    let _e29 = x_11;
    let _e30 = (*i_1);
    return (_e29 - _e30);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e27 = x_13;
    (*i_2) = trunc(_e27);
    let _e29 = x_13;
    let _e30 = (*i_2);
    return (_e29 - _e30);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e27 = x_15;
    (*i_3) = trunc(_e27);
    let _e29 = x_15;
    let _e30 = (*i_3);
    return (_e29 - _e30);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e26 = v_1;
    b = (bitcast<u32>(_e26) & 2147483647u);
    let _e31 = b;
    return (_e31 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e26 = v_3;
    b_1 = (bitcast<u32>(_e26) & 2147483647u);
    let _e31 = b_1;
    return (_e31 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e26 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e26) & vec2(2147483647u));
    let _e32 = b_2;
    return (_e32 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e26 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e26) & vec3(2147483647u));
    let _e32 = b_3;
    return (_e32 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e26 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e26) & vec4(2147483647u));
    let _e32 = b_4;
    return (_e32 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e26 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e26) & vec2(2147483647u));
    let _e32 = b_5;
    return (_e32 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e26 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e26) & vec3(2147483647u));
    let _e32 = b_6;
    return (_e32 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e26 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e26) & vec4(2147483647u));
    let _e32 = b_7;
    return (_e32 == vec4(2139095040u));
}

fn pt_main() {
    let _e30 = position_1;
    vUv = ((_e30.xy * 0.5f) + vec2(0.5f));
    let _e37 = vUv;
    let _e38 = global.resolution;
    vPixCoord = (_e37 * _e38);
    let _e42 = vUv;
    let _e44 = global.texelSize;
    vOffset[0i] = (_e42.xyxy + (_e44.xyxy * vec4<f32>(-0.25f, -0.125f, 1.25f, -0.125f)));
    let _e58 = vUv;
    let _e60 = global.texelSize;
    vOffset[1i] = (_e58.xyxy + (_e60.xyxy * vec4<f32>(-0.125f, -0.25f, -0.125f, 1.25f)));
    let _e76 = vOffset[0];
    let _e77 = _e76.xz;
    let _e80 = vOffset[1];
    let _e81 = _e80.yw;
    let _e94 = global.texelSize;
    vOffset[2i] = (vec4<f32>(_e77.x, _e77.y, _e81.x, _e81.y) + ((vec4<f32>(-2f, 2f, -2f, 2f) * _e94.xxyy) * 8f));
    let _e101 = position_1;
    let _e102 = _e101.xy;
    gl_Position = vec4<f32>(_e102.x, _e102.y, 1f, 1f);
    return;
}

fn main_1() {
    pt_main();
    let _e32 = gl_Position;
    gl_Position.y = -(_e32.y);
    let _e36 = gl_Position;
    let _e38 = gl_Position;
    gl_Position.z = ((_e36.z + _e38.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_1 = normal;
    uv_1 = uv;
    main_1();
    let _e49 = vUv;
    let _e52 = vOffset[0];
    let _e54 = vOffset[1];
    let _e56 = vOffset[2];
    let _e58 = vPixCoord;
    let _e60 = gl_Position;
    return VertexOutput(_e49, _e52, _e54, _e56, _e58, _e60);
}
