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
    ghostAmount: f32,
    haloAmount: f32,
    chromaticAberration: f32,
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
var<private> vAspectRatio_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

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

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e29 = value_1;
    return _e29;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e29 = value_3;
    let _e38 = pt_pow_2(((_e29.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e39 = value_3;
    let _e43 = value_3;
    let _e53 = mix(_e38, (_e39.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e43.xyz <= vec3(0.04045f))));
    let _e54 = value_3;
    return vec4<f32>(_e53.x, _e53.y, _e53.z, _e54.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e29 = value_5;
    let _e33 = pt_pow_2(_e29.xyz, vec3(0.41666f));
    let _e39 = value_5;
    let _e43 = value_5;
    let _e53 = mix(((_e33 * 1.055f) - vec3(0.055f)), (_e39.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e43.xyz <= vec3(0.0031308f))));
    let _e54 = value_5;
    return vec4<f32>(_e53.x, _e53.y, _e53.z, _e54.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e29 = value_7;
    let _e47 = (_e29.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e48 = value_7;
    let _e54 = LinearTransferOETF(vec4<f32>(_e47.x, _e47.y, _e47.z, _e48.w));
    return _e54;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e34 = weights;
    let _e35 = rgb_1;
    return dot(_e34, _e35);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e29 = x_17;
    let _e30 = x_17;
    return (_e29 * _e30);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e29 = x_19;
    let _e30 = x_19;
    return (_e29 * _e30);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e29 = x_21;
    let _e30 = x_21;
    let _e32 = x_21;
    return ((_e29 * _e30) * _e32);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e29 = x_23;
    let _e30 = x_23;
    x2_ = (_e29 * _e30);
    let _e33 = x2_;
    let _e34 = x2_;
    return (_e33 * _e34);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e29 = v_17;
    let _e31 = v_17;
    let _e34 = v_17;
    return max(max(_e29.x, _e31.y), _e34.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e29 = v_19;
    return dot(_e29, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e35 = uv_1;
    let _e37 = a;
    let _e38 = b_8;
    dt = dot(_e35.xy, vec2<f32>(_e37, _e38));
    let _e42 = dt;
    sn = (_e42 - (floor((_e42 / 3.1415927f)) * 3.1415927f));
    let _e49 = sn;
    let _e51 = c;
    return fract((sin(_e49) * _e51));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e29 = v_21;
    return length(_e29);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e31 = matrix_1;
    let _e32 = dir_1;
    return normalize((_e31 * vec4<f32>(_e32.x, _e32.y, _e32.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e31 = normal_1;
    let _e37 = viewMatrix_1;
    return normalize((vec4<f32>(_e31.x, _e31.y, _e31.z, 0f) * _e37).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e31 = dir_3;
    let _e37 = viewMatrix_3;
    return normalize((vec4<f32>(_e31.x, _e31.y, _e31.z, 0f) * _e37).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e33 = m_1[2][3];
    return (_e33 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e29 = dir_5;
    let _e31 = dir_5;
    u = ((atan2(_e29.z, _e31.x) * 0.15915494f) + 0.5f);
    let _e39 = dir_5;
    v_22 = ((asin(clamp(_e39.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e51 = u;
    let _e52 = v_22;
    return vec2<f32>(_e51, _e52);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e30 = diffuseColor_1;
    return (0.31830987f * _e30);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e35 = dotVH_1;
    let _e39 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e35) - 6.98316f) * _e39));
    let _e43 = f0_1;
    let _e45 = fresnel;
    let _e48 = f90_1;
    let _e49 = fresnel;
    return ((_e43 * (1f - _e45)) + vec3((_e48 * _e49)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e35 = dotVH_3;
    let _e39 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e35) - 6.98316f) * _e39));
    let _e43 = f0_3;
    let _e45 = fresnel_1;
    let _e48 = f90_3;
    let _e49 = fresnel_1;
    return ((_e43 * (1f - _e45)) + (_e48 * _e49));
}

fn sampleGhost(direction: vec2<f32>, color: vec3<f32>, offset: f32) -> vec3<f32> {
    var suv: vec2<f32>;
    var result: vec3<f32>;
    var d: f32;

    let _e31 = vUv_1;
    suv = clamp(((vec2(1f) - _e31) + (direction * offset)), vec2(0f), vec2(1f));
    let _e42 = suv;
    let _e43 = textureSample(inputBuffer_t, inputBuffer_s, _e42);
    result = (_e43.xyz * color);
    let _e48 = suv;
    d = clamp((length((vec2(0.5f) - _e48)) / 0.35355338f), 0f, 1f);
    let _e60 = result;
    let _e62 = d;
    let _e65 = pt_pow((1f - _e62), 3f);
    result = (_e60 * _e65);
    let _e67 = result;
    return _e67;
}

fn sampleGhosts(amount: f32) -> vec4<f32> {
    var amount_1: f32;
    var color_1: vec3<f32> = vec3(0f);
    var direction_1: vec2<f32>;

    amount_1 = amount;
    let _e32 = vUv_1;
    direction_1 = (_e32 - vec2(0.5f));
    let _e37 = color_1;
    let _e38 = direction_1;
    let _e45 = sampleGhost(_e38, vec3<f32>(0.8f, 0.8f, 1f), -5f);
    color_1 = (_e37 + _e45);
    let _e47 = color_1;
    let _e48 = direction_1;
    let _e55 = sampleGhost(_e48, vec3<f32>(1f, 0.8f, 0.4f), -1.5f);
    color_1 = (_e47 + _e55);
    let _e57 = color_1;
    let _e58 = direction_1;
    let _e65 = sampleGhost(_e58, vec3<f32>(0.9f, 1f, 0.8f), -0.4f);
    color_1 = (_e57 + _e65);
    let _e67 = color_1;
    let _e68 = direction_1;
    let _e75 = sampleGhost(_e68, vec3<f32>(1f, 0.8f, 0.4f), -0.2f);
    color_1 = (_e67 + _e75);
    let _e77 = color_1;
    let _e78 = direction_1;
    let _e85 = sampleGhost(_e78, vec3<f32>(0.9f, 0.7f, 0.7f), -0.1f);
    color_1 = (_e77 + _e85);
    let _e87 = color_1;
    let _e88 = direction_1;
    let _e94 = sampleGhost(_e88, vec3<f32>(0.5f, 1f, 0.4f), 0.7f);
    color_1 = (_e87 + _e94);
    let _e96 = color_1;
    let _e97 = direction_1;
    let _e103 = sampleGhost(_e97, vec3<f32>(0.5f, 0.5f, 0.5f), 1f);
    color_1 = (_e96 + _e103);
    let _e105 = color_1;
    let _e106 = direction_1;
    let _e112 = sampleGhost(_e106, vec3<f32>(1f, 1f, 0.6f), 2.5f);
    color_1 = (_e105 + _e112);
    let _e114 = color_1;
    let _e115 = direction_1;
    let _e121 = sampleGhost(_e115, vec3<f32>(0.5f, 0.8f, 1f), 10f);
    color_1 = (_e114 + _e121);
    let _e123 = color_1;
    let _e124 = amount_1;
    let _e125 = (_e123 * _e124);
    return vec4<f32>(_e125.x, _e125.y, _e125.z, 1f);
}

fn cubicRingMask(x_24: f32, radius: f32, thickness: f32) -> f32 {
    var v_23: f32;

    v_23 = min((abs((x_24 - radius)) / thickness), 1f);
    let _e37 = v_23;
    let _e38 = v_23;
    let _e42 = v_23;
    return (1f - ((_e37 * _e38) * (3f - (2f * _e42))));
}

fn sampleHalo(radius_1: f32) -> vec3<f32> {
    var direction_2: vec2<f32>;
    var offset_1: vec3<f32>;
    var suv_1: vec2<f32>;
    var result_1: vec3<f32>;
    var wuv: vec2<f32>;
    var d_1: f32;

    let _e28 = vUv_1;
    let _e32 = vAspectRatio_1;
    let _e35 = vAspectRatio_1;
    direction_2 = (normalize(((_e28 - vec2(0.5f)) / _e32)) * _e35);
    let _e38 = global.texelSize;
    let _e40 = global.chromaticAberration;
    offset_1 = (vec3((_e38.x * _e40)) * vec3<f32>(-1f, 0f, 1f));
    let _e51 = vUv_1;
    let _e54 = direction_2;
    suv_1 = fract(((vec2(1f) - _e51) + (_e54 * radius_1)));
    let _e59 = suv_1;
    let _e60 = direction_2;
    let _e61 = offset_1;
    let _e65 = textureSample(inputBuffer_t, inputBuffer_s, (_e59 + (_e60 * _e61.x)));
    let _e67 = suv_1;
    let _e68 = direction_2;
    let _e69 = offset_1;
    let _e73 = textureSample(inputBuffer_t, inputBuffer_s, (_e67 + (_e68 * _e69.y)));
    let _e75 = suv_1;
    let _e76 = direction_2;
    let _e77 = offset_1;
    let _e81 = textureSample(inputBuffer_t, inputBuffer_s, (_e75 + (_e76 * _e77.z)));
    result_1 = vec3<f32>(_e65.x, _e73.y, _e81.z);
    let _e85 = vUv_1;
    let _e90 = vAspectRatio_1;
    wuv = (((_e85 - vec2<f32>(0.5f, 0f)) / _e90) + vec2<f32>(0.5f, 0f));
    let _e97 = wuv;
    d_1 = clamp(distance(_e97, vec2(0.5f)), 0f, 1f);
    let _e105 = result_1;
    let _e106 = d_1;
    let _e109 = cubicRingMask(_e106, 0.45f, 0.25f);
    result_1 = (_e105 * _e109);
    let _e111 = result_1;
    return _e111;
}

fn sampleHalos(amount_2: f32) -> vec4<f32> {
    var color_2: vec3<f32> = vec3(0f);

    let _e31 = color_2;
    let _e33 = sampleHalo(0.3f);
    color_2 = (_e31 + _e33);
    let _e35 = color_2;
    return (vec4<f32>(_e35.x, _e35.y, _e35.z, 1f) * amount_2);
}

fn main_1() {
    let _e27 = pc_fragColor;
    let _e28 = global.ghostAmount;
    let _e29 = sampleGhosts(_e28);
    pc_fragColor = (_e27 + _e29);
    let _e31 = pc_fragColor;
    let _e32 = global.haloAmount;
    let _e33 = sampleHalos(_e32);
    pc_fragColor = (_e31 + _e33);
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vAspectRatio: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vAspectRatio_1 = vAspectRatio;
    main_1();
    let _e37 = pc_fragColor;
    return FragmentOutput(_e37);
}
