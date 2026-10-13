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

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var map_t: texture_2d<f32>;
@group(0) @binding(2) 
var map_s: sampler;
var<private> vMapUv_1: vec2<f32>;
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
    let _e26 = value_1;
    return _e26;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e26 = value_3;
    let _e35 = pt_pow_2(((_e26.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e36 = value_3;
    let _e40 = value_3;
    let _e50 = mix(_e35, (_e36.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e40.xyz <= vec3(0.04045f))));
    let _e51 = value_3;
    return vec4<f32>(_e50.x, _e50.y, _e50.z, _e51.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e26 = value_5;
    let _e30 = pt_pow_2(_e26.xyz, vec3(0.41666f));
    let _e36 = value_5;
    let _e40 = value_5;
    let _e50 = mix(((_e30 * 1.055f) - vec3(0.055f)), (_e36.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e40.xyz <= vec3(0.0031308f))));
    let _e51 = value_5;
    return vec4<f32>(_e50.x, _e50.y, _e50.z, _e51.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e26 = value_7;
    let _e44 = (_e26.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e45 = value_7;
    let _e51 = LinearTransferOETF(vec4<f32>(_e44.x, _e44.y, _e44.z, _e45.w));
    return _e51;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e31 = weights;
    let _e32 = rgb_1;
    return dot(_e31, _e32);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e26 = x_17;
    let _e27 = x_17;
    return (_e26 * _e27);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e26 = x_19;
    let _e27 = x_19;
    return (_e26 * _e27);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e26 = x_21;
    let _e27 = x_21;
    let _e29 = x_21;
    return ((_e26 * _e27) * _e29);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e26 = x_23;
    let _e27 = x_23;
    x2_ = (_e26 * _e27);
    let _e30 = x2_;
    let _e31 = x2_;
    return (_e30 * _e31);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e26 = v_17;
    let _e28 = v_17;
    let _e31 = v_17;
    return max(max(_e26.x, _e28.y), _e31.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e26 = v_19;
    return dot(_e26, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e32 = uv_1;
    let _e34 = a;
    let _e35 = b_8;
    dt = dot(_e32.xy, vec2<f32>(_e34, _e35));
    let _e39 = dt;
    sn = (_e39 - (floor((_e39 / 3.1415927f)) * 3.1415927f));
    let _e46 = sn;
    let _e48 = c;
    return fract((sin(_e46) * _e48));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e26 = v_21;
    return length(_e26);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e28 = matrix_1;
    let _e29 = dir_1;
    return normalize((_e28 * vec4<f32>(_e29.x, _e29.y, _e29.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e28 = normal_1;
    let _e34 = viewMatrix_1;
    return normalize((vec4<f32>(_e28.x, _e28.y, _e28.z, 0f) * _e34).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e28 = dir_3;
    let _e34 = viewMatrix_3;
    return normalize((vec4<f32>(_e28.x, _e28.y, _e28.z, 0f) * _e34).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e30 = m_1[2][3];
    return (_e30 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e26 = dir_5;
    let _e28 = dir_5;
    u = ((atan2(_e26.z, _e28.x) * 0.15915494f) + 0.5f);
    let _e36 = dir_5;
    v_22 = ((asin(clamp(_e36.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e48 = u;
    let _e49 = v_22;
    return vec2<f32>(_e48, _e49);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e27 = diffuseColor_1;
    return (0.31830987f * _e27);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e32 = dotVH_1;
    let _e36 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e32) - 6.98316f) * _e36));
    let _e40 = f0_1;
    let _e42 = fresnel;
    let _e45 = f90_1;
    let _e46 = fresnel;
    return ((_e40 * (1f - _e42)) + vec3((_e45 * _e46)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e32 = dotVH_3;
    let _e36 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e32) - 6.98316f) * _e36));
    let _e40 = f0_3;
    let _e42 = fresnel_1;
    let _e45 = f90_3;
    let _e46 = fresnel_1;
    return ((_e40 * (1f - _e42)) + (_e45 * _e46));
}

fn main_1() {
    var diffuseColor_2: vec4<f32>;
    var sampledDiffuseColor: vec4<f32>;
    var specularStrength: f32;
    var reflectedLight: ReflectedLight = ReflectedLight(vec3(0f), vec3(0f), vec3(0f), vec3(0f));
    var outgoingLight: vec3<f32>;

    let _e24 = global.diffuse;
    let _e25 = global.opacity;
    diffuseColor_2 = vec4<f32>(_e24.x, _e24.y, _e24.z, _e25);
    let _e31 = vMapUv_1;
    let _e32 = textureSample(map_t, map_s, _e31);
    sampledDiffuseColor = _e32;
    let _e34 = diffuseColor_2;
    let _e35 = sampledDiffuseColor;
    diffuseColor_2 = (_e34 * _e35);
    specularStrength = 1f;
    let _e50 = reflectedLight;
    reflectedLight.indirectDiffuse = (_e50.indirectDiffuse + vec3(1f));
    let _e56 = reflectedLight;
    let _e58 = diffuseColor_2;
    reflectedLight.indirectDiffuse = (_e56.indirectDiffuse * _e58.xyz);
    let _e61 = reflectedLight;
    outgoingLight = _e61.indirectDiffuse;
    diffuseColor_2.w = 1f;
    let _e66 = outgoingLight;
    let _e67 = diffuseColor_2;
    pc_fragColor = vec4<f32>(_e66.x, _e66.y, _e66.z, _e67.w);
    let _e73 = pc_fragColor;
    let _e74 = linearToOutputTexel(_e73);
    pc_fragColor = _e74;
    return;
}

@fragment 
fn main(@location(0) vMapUv: vec2<f32>) -> FragmentOutput {
    vMapUv_1 = vMapUv;
    main_1();
    let _e31 = pc_fragColor;
    return FragmentOutput(_e31);
}
