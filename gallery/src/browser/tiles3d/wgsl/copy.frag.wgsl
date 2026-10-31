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
    opacity: f32,
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
    let _e22 = value_1;
    return _e22;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e22 = value_3;
    let _e31 = pt_pow_2(((_e22.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e32 = value_3;
    let _e36 = value_3;
    let _e46 = mix(_e31, (_e32.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e36.xyz <= vec3(0.04045f))));
    let _e47 = value_3;
    return vec4<f32>(_e46.x, _e46.y, _e46.z, _e47.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e22 = value_5;
    let _e26 = pt_pow_2(_e22.xyz, vec3(0.41666f));
    let _e32 = value_5;
    let _e36 = value_5;
    let _e46 = mix(((_e26 * 1.055f) - vec3(0.055f)), (_e32.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e36.xyz <= vec3(0.0031308f))));
    let _e47 = value_5;
    return vec4<f32>(_e46.x, _e46.y, _e46.z, _e47.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e39 = value_7;
    let _e41 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e39.xyz);
    let _e42 = value_7;
    let _e48 = LinearTransferOETF(vec4<f32>(_e41.x, _e41.y, _e41.z, _e42.w));
    return _e48;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e27 = weights;
    let _e28 = rgb_1;
    return dot(_e27, _e28);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e22 = x_17;
    let _e23 = x_17;
    return (_e22 * _e23);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e22 = x_19;
    let _e23 = x_19;
    return (_e22 * _e23);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e22 = x_21;
    let _e23 = x_21;
    let _e25 = x_21;
    return ((_e22 * _e23) * _e25);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e22 = x_23;
    let _e23 = x_23;
    x2_ = (_e22 * _e23);
    let _e26 = x2_;
    let _e27 = x2_;
    return (_e26 * _e27);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e22 = v_17;
    let _e24 = v_17;
    let _e27 = v_17;
    return max(max(_e22.x, _e24.y), _e27.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e22 = v_19;
    return dot(_e22, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e28 = uv_1;
    let _e30 = a;
    let _e31 = b_8;
    dt = dot(_e28.xy, vec2<f32>(_e30, _e31));
    let _e35 = dt;
    sn = (_e35 - (floor((_e35 / 3.1415927f)) * 3.1415927f));
    let _e42 = sn;
    let _e44 = c;
    return fract((sin(_e42) * _e44));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e22 = v_21;
    return length(_e22);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e24 = matrix_1;
    let _e25 = dir_1;
    return normalize((_e24 * vec4<f32>(_e25.x, _e25.y, _e25.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e24 = normal_1;
    let _e30 = viewMatrix_1;
    return normalize((vec4<f32>(_e24.x, _e24.y, _e24.z, 0f) * _e30).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e24 = dir_3;
    let _e30 = viewMatrix_3;
    return normalize((vec4<f32>(_e24.x, _e24.y, _e24.z, 0f) * _e30).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e26 = m_1[2][3];
    return (_e26 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e22 = dir_5;
    let _e24 = dir_5;
    u = ((atan2(_e22.z, _e24.x) * 0.15915494f) + 0.5f);
    let _e32 = dir_5;
    v_22 = ((asin(clamp(_e32.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e44 = u;
    let _e45 = v_22;
    return vec2<f32>(_e44, _e45);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e23 = diffuseColor_1;
    return (0.31830987f * _e23);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e28 = dotVH_1;
    let _e32 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e28) - 6.98316f) * _e32));
    let _e36 = f0_1;
    let _e38 = fresnel;
    let _e41 = f90_1;
    let _e42 = fresnel;
    return ((_e36 * (1f - _e38)) + vec3((_e41 * _e42)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e28 = dotVH_3;
    let _e32 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e28) - 6.98316f) * _e32));
    let _e36 = f0_3;
    let _e38 = fresnel_1;
    let _e41 = f90_3;
    let _e42 = fresnel_1;
    return ((_e36 * (1f - _e38)) + (_e41 * _e42));
}

fn main_1() {
    var texel: vec4<f32>;

    let _e20 = vUv_1;
    let _e21 = textureSample(inputBuffer_t, inputBuffer_s, _e20);
    texel = _e21;
    let _e23 = global.opacity;
    let _e24 = texel;
    pc_fragColor = (_e23 * _e24);
    let _e26 = pc_fragColor;
    let _e27 = linearToOutputTexel(_e26);
    pc_fragColor = _e27;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    main_1();
    let _e27 = pc_fragColor;
    return FragmentOutput(_e27);
}
