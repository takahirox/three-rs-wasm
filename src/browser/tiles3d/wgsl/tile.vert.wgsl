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
    mapTransform: mat3x3<f32>,
    diffuse: vec3<f32>,
    opacity: f32,
}

struct VertexOutput {
    @location(0) vMapUv: vec2<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var map_t: texture_2d<f32>;
@group(0) @binding(2) 
var map_s: sampler;
var<private> position_1: vec3<f32>;
var<private> normal_3: vec3<f32>;
var<private> uv_3: vec2<f32>;
var<private> vMapUv: vec2<f32>;
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

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e28 = x_17;
    let _e29 = x_17;
    return (_e28 * _e29);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e28 = x_19;
    let _e29 = x_19;
    return (_e28 * _e29);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e28 = x_21;
    let _e29 = x_21;
    let _e31 = x_21;
    return ((_e28 * _e29) * _e31);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e28 = x_23;
    let _e29 = x_23;
    x2_ = (_e28 * _e29);
    let _e32 = x2_;
    let _e33 = x2_;
    return (_e32 * _e33);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e28 = v_17;
    let _e30 = v_17;
    let _e33 = v_17;
    return max(max(_e28.x, _e30.y), _e33.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e28 = v_19;
    return dot(_e28, vec3(0.3333333f));
}

fn rand(uv_1: vec2<f32>) -> f32 {
    var uv_2: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_2 = uv_1;
    let _e34 = uv_2;
    let _e36 = a;
    let _e37 = b_8;
    dt = dot(_e34.xy, vec2<f32>(_e36, _e37));
    let _e41 = dt;
    sn = (_e41 - (floor((_e41 / 3.1415927f)) * 3.1415927f));
    let _e48 = sn;
    let _e50 = c;
    return fract((sin(_e48) * _e50));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e28 = v_21;
    return length(_e28);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e30 = matrix_1;
    let _e31 = dir_1;
    return normalize((_e30 * vec4<f32>(_e31.x, _e31.y, _e31.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal_1: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_2: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_2 = normal_1;
    viewMatrix_1 = viewMatrix;
    let _e30 = normal_2;
    let _e36 = viewMatrix_1;
    return normalize((vec4<f32>(_e30.x, _e30.y, _e30.z, 0f) * _e36).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e30 = dir_3;
    let _e36 = viewMatrix_3;
    return normalize((vec4<f32>(_e30.x, _e30.y, _e30.z, 0f) * _e36).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e32 = m_1[2][3];
    return (_e32 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e28 = dir_5;
    let _e30 = dir_5;
    u = ((atan2(_e28.z, _e30.x) * 0.15915494f) + 0.5f);
    let _e38 = dir_5;
    v_22 = ((asin(clamp(_e38.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e50 = u;
    let _e51 = v_22;
    return vec2<f32>(_e50, _e51);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e29 = diffuseColor_1;
    return (0.31830987f * _e29);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e34 = dotVH_1;
    let _e38 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e34) - 6.98316f) * _e38));
    let _e42 = f0_1;
    let _e44 = fresnel;
    let _e47 = f90_1;
    let _e48 = fresnel;
    return ((_e42 * (1f - _e44)) + vec3((_e47 * _e48)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e34 = dotVH_3;
    let _e38 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e34) - 6.98316f) * _e38));
    let _e42 = f0_3;
    let _e44 = fresnel_1;
    let _e47 = f90_3;
    let _e48 = fresnel_1;
    return ((_e42 * (1f - _e44)) + (_e47 * _e48));
}

fn pt_main() {
    var transformed: vec3<f32>;
    var mvPosition: vec4<f32>;

    let _e26 = global.mapTransform;
    let _e27 = uv_3;
    vMapUv = (_e26 * vec3<f32>(_e27.x, _e27.y, 1f)).xy;
    let _e35 = position_1;
    transformed = vec3<f32>(_e35);
    let _e38 = transformed;
    mvPosition = vec4<f32>(_e38.x, _e38.y, _e38.z, 1f);
    let _e45 = global.modelViewMatrix;
    let _e46 = mvPosition;
    mvPosition = (_e45 * _e46);
    let _e49 = global.projectionMatrix;
    let _e50 = mvPosition;
    gl_Position = (_e49 * _e50);
    return;
}

fn main_1() {
    pt_main();
    let _e28 = gl_Position;
    gl_Position.y = -(_e28.y);
    let _e32 = gl_Position;
    let _e34 = gl_Position;
    gl_Position.z = ((_e32.z + _e34.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_3 = normal;
    uv_3 = uv;
    main_1();
    let _e39 = vMapUv;
    let _e41 = gl_Position;
    return VertexOutput(_e39, _e41);
}
