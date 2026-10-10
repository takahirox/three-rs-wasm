diagnostic(off, derivative_uniformity);
struct Uniforms {
    viewMatrix: mat4x4<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
}

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

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

var<private> pc_fragColor: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var envMap_t: texture_2d<f32>;
@group(0) @binding(2) 
var envMap_s: sampler;
var<private> vOutputDirection_1: vec3<f32>;

fn pt_isnan(v: vec3<f32>) -> vec3<bool> {
    var v_1: vec3<f32>;
    var b: vec3<u32>;

    v_1 = v;
    let _e11 = v_1;
    b = (bitcast<vec3<u32>>(_e11) & vec3(2147483647u));
    let _e17 = b;
    return (_e17 > vec3(2139095040u));
}

fn pt_isinf(v_2: vec3<f32>) -> vec3<bool> {
    var v_3: vec3<f32>;
    var b_1: vec3<u32>;

    v_3 = v_2;
    let _e11 = v_3;
    b_1 = (bitcast<vec3<u32>>(_e11) & vec3(2147483647u));
    let _e17 = b_1;
    return (_e17 == vec3(2139095040u));
}

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e13 = y_1;
    let _e14 = x_1;
    return exp2((_e13 * log2(_e14)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e13 = y_3;
    let _e14 = x_3;
    return exp2((_e13 * log2(_e14)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e13 = y_5;
    let _e14 = x_5;
    return exp2((_e13 * log2(_e14)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e13 = y_7;
    let _e14 = x_7;
    return exp2((_e13 * log2(_e14)));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e11 = value_1;
    return _e11;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e11 = value_3;
    let _e20 = pt_pow_2(((_e11.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e21 = value_3;
    let _e25 = value_3;
    let _e35 = mix(_e20, (_e21.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e25.xyz <= vec3(0.04045f))));
    let _e36 = value_3;
    return vec4<f32>(_e35.x, _e35.y, _e35.z, _e36.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e11 = value_5;
    let _e15 = pt_pow_2(_e11.xyz, vec3(0.41666f));
    let _e21 = value_5;
    let _e25 = value_5;
    let _e35 = mix(((_e15 * 1.055f) - vec3(0.055f)), (_e21.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e25.xyz <= vec3(0.0031308f))));
    let _e36 = value_5;
    return vec4<f32>(_e35.x, _e35.y, _e35.z, _e36.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e11 = value_7;
    let _e29 = (_e11.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e30 = value_7;
    let _e36 = LinearTransferOETF(vec4<f32>(_e29.x, _e29.y, _e29.z, _e30.w));
    return _e36;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e16 = weights;
    let _e17 = rgb_1;
    return dot(_e16, _e17);
}

fn pow2_(x_8: f32) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e12 = x_9;
    let _e13 = x_9;
    return (_e12 * _e13);
}

fn pow2_1(x_10: vec3<f32>) -> vec3<f32> {
    var x_11: vec3<f32>;

    x_11 = x_10;
    let _e12 = x_11;
    let _e13 = x_11;
    return (_e12 * _e13);
}

fn pow3_(x_12: f32) -> f32 {
    var x_13: f32;

    x_13 = x_12;
    let _e12 = x_13;
    let _e13 = x_13;
    let _e15 = x_13;
    return ((_e12 * _e13) * _e15);
}

fn pow4_(x_14: f32) -> f32 {
    var x_15: f32;
    var x2_: f32;

    x_15 = x_14;
    let _e12 = x_15;
    let _e13 = x_15;
    x2_ = (_e12 * _e13);
    let _e16 = x2_;
    let _e17 = x2_;
    return (_e16 * _e17);
}

fn max3_(v_4: vec3<f32>) -> f32 {
    var v_5: vec3<f32>;

    v_5 = v_4;
    let _e12 = v_5;
    let _e14 = v_5;
    let _e17 = v_5;
    return max(max(_e12.x, _e14.y), _e17.z);
}

fn average(v_6: vec3<f32>) -> f32 {
    var v_7: vec3<f32>;

    v_7 = v_6;
    let _e12 = v_7;
    return dot(_e12, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_2: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e18 = uv_1;
    let _e20 = a;
    let _e21 = b_2;
    dt = dot(_e18.xy, vec2<f32>(_e20, _e21));
    let _e25 = dt;
    sn = (_e25 - (floor((_e25 / 3.1415927f)) * 3.1415927f));
    let _e32 = sn;
    let _e34 = c;
    return fract((sin(_e32) * _e34));
}

fn precisionSafeLength(v_8: vec3<f32>) -> f32 {
    var v_9: vec3<f32>;

    v_9 = v_8;
    let _e12 = v_9;
    return length(_e12);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e14 = matrix_1;
    let _e15 = dir_1;
    return normalize((_e14 * vec4<f32>(_e15.x, _e15.y, _e15.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e14 = normal_1;
    let _e20 = viewMatrix_1;
    return normalize((vec4<f32>(_e14.x, _e14.y, _e14.z, 0f) * _e20).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e14 = dir_3;
    let _e20 = viewMatrix_3;
    return normalize((vec4<f32>(_e14.x, _e14.y, _e14.z, 0f) * _e20).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e16 = m_1[2][3];
    return (_e16 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_10: f32;

    dir_5 = dir_4;
    let _e12 = dir_5;
    let _e14 = dir_5;
    u = ((atan2(_e12.z, _e14.x) * 0.15915494f) + 0.5f);
    let _e22 = dir_5;
    v_10 = ((asin(clamp(_e22.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e34 = u;
    let _e35 = v_10;
    return vec2<f32>(_e34, _e35);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e13 = diffuseColor_1;
    return (0.31830987f * _e13);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e18 = dotVH_1;
    let _e22 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e18) - 6.98316f) * _e22));
    let _e26 = f0_1;
    let _e28 = fresnel;
    let _e31 = f90_1;
    let _e32 = fresnel;
    return ((_e26 * (1f - _e28)) + vec3((_e31 * _e32)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e18 = dotVH_3;
    let _e22 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e18) - 6.98316f) * _e22));
    let _e26 = f0_3;
    let _e28 = fresnel_1;
    let _e31 = f90_3;
    let _e32 = fresnel_1;
    return ((_e26 * (1f - _e28)) + (_e31 * _e32));
}

fn main_1() {
    var outputDirection: vec3<f32>;
    var uv_2: vec2<f32>;

    let _e10 = vOutputDirection_1;
    outputDirection = normalize(_e10);
    let _e13 = outputDirection;
    let _e14 = equirectUv(_e13);
    uv_2 = _e14;
    let _e16 = uv_2;
    let _e17 = textureSample(envMap_t, envMap_s, _e16);
    let _e18 = _e17.xyz;
    pc_fragColor = vec4<f32>(_e18.x, _e18.y, _e18.z, 1f);
    return;
}

@fragment 
fn main(@location(0) vOutputDirection: vec3<f32>) -> FragmentOutput {
    vOutputDirection_1 = vOutputDirection;
    main_1();
    let _e17 = pc_fragColor;
    return FragmentOutput(_e17);
}
