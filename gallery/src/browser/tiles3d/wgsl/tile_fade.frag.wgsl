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
    fadeIn: f32,
    fadeOut: f32,
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
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e30 = y_1;
    let _e31 = x_1;
    return exp2((_e30 * log2(_e31)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e30 = y_3;
    let _e31 = x_3;
    return exp2((_e30 * log2(_e31)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e30 = y_5;
    let _e31 = x_5;
    return exp2((_e30 * log2(_e31)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e30 = y_7;
    let _e31 = x_7;
    return exp2((_e30 * log2(_e31)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e29 = x_9;
    (*i) = trunc(_e29);
    let _e31 = x_9;
    let _e32 = (*i);
    return (_e31 - _e32);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e29 = x_11;
    (*i_1) = trunc(_e29);
    let _e31 = x_11;
    let _e32 = (*i_1);
    return (_e31 - _e32);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e29 = x_13;
    (*i_2) = trunc(_e29);
    let _e31 = x_13;
    let _e32 = (*i_2);
    return (_e31 - _e32);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e29 = x_15;
    (*i_3) = trunc(_e29);
    let _e31 = x_15;
    let _e32 = (*i_3);
    return (_e31 - _e32);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e28 = v_1;
    b = (bitcast<u32>(_e28) & 2147483647u);
    let _e33 = b;
    return (_e33 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e28 = v_3;
    b_1 = (bitcast<u32>(_e28) & 2147483647u);
    let _e33 = b_1;
    return (_e33 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e28 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e28) & vec2(2147483647u));
    let _e34 = b_2;
    return (_e34 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e28 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e28) & vec3(2147483647u));
    let _e34 = b_3;
    return (_e34 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e28 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e28) & vec4(2147483647u));
    let _e34 = b_4;
    return (_e34 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e28 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e28) & vec2(2147483647u));
    let _e34 = b_5;
    return (_e34 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e28 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e28) & vec3(2147483647u));
    let _e34 = b_6;
    return (_e34 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e28 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e28) & vec4(2147483647u));
    let _e34 = b_7;
    return (_e34 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e30 = value_1;
    return _e30;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e30 = value_3;
    let _e39 = pt_pow_2(((_e30.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e40 = value_3;
    let _e44 = value_3;
    let _e54 = mix(_e39, (_e40.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e44.xyz <= vec3(0.04045f))));
    let _e55 = value_3;
    return vec4<f32>(_e54.x, _e54.y, _e54.z, _e55.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e30 = value_5;
    let _e34 = pt_pow_2(_e30.xyz, vec3(0.41666f));
    let _e40 = value_5;
    let _e44 = value_5;
    let _e54 = mix(((_e34 * 1.055f) - vec3(0.055f)), (_e40.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e44.xyz <= vec3(0.0031308f))));
    let _e55 = value_5;
    return vec4<f32>(_e54.x, _e54.y, _e54.z, _e55.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e30 = value_7;
    let _e48 = (_e30.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e49 = value_7;
    let _e55 = LinearTransferOETF(vec4<f32>(_e48.x, _e48.y, _e48.z, _e49.w));
    return _e55;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e35 = weights;
    let _e36 = rgb_1;
    return dot(_e35, _e36);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e30 = x_17;
    let _e31 = x_17;
    return (_e30 * _e31);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e30 = x_19;
    let _e31 = x_19;
    return (_e30 * _e31);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e30 = x_21;
    let _e31 = x_21;
    let _e33 = x_21;
    return ((_e30 * _e31) * _e33);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e30 = x_23;
    let _e31 = x_23;
    x2_ = (_e30 * _e31);
    let _e34 = x2_;
    let _e35 = x2_;
    return (_e34 * _e35);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e30 = v_17;
    let _e32 = v_17;
    let _e35 = v_17;
    return max(max(_e30.x, _e32.y), _e35.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e30 = v_19;
    return dot(_e30, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e36 = uv_1;
    let _e38 = a;
    let _e39 = b_8;
    dt = dot(_e36.xy, vec2<f32>(_e38, _e39));
    let _e43 = dt;
    sn = (_e43 - (floor((_e43 / 3.1415927f)) * 3.1415927f));
    let _e50 = sn;
    let _e52 = c;
    return fract((sin(_e50) * _e52));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e30 = v_21;
    return length(_e30);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e32 = matrix_1;
    let _e33 = dir_1;
    return normalize((_e32 * vec4<f32>(_e33.x, _e33.y, _e33.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e32 = normal_1;
    let _e38 = viewMatrix_1;
    return normalize((vec4<f32>(_e32.x, _e32.y, _e32.z, 0f) * _e38).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e32 = dir_3;
    let _e38 = viewMatrix_3;
    return normalize((vec4<f32>(_e32.x, _e32.y, _e32.z, 0f) * _e38).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e34 = m_1[2][3];
    return (_e34 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e30 = dir_5;
    let _e32 = dir_5;
    u = ((atan2(_e30.z, _e32.x) * 0.15915494f) + 0.5f);
    let _e40 = dir_5;
    v_22 = ((asin(clamp(_e40.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e52 = u;
    let _e53 = v_22;
    return vec2<f32>(_e52, _e53);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e31 = diffuseColor_1;
    return (0.31830987f * _e31);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e36 = dotVH_1;
    let _e40 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e36) - 6.98316f) * _e40));
    let _e44 = f0_1;
    let _e46 = fresnel;
    let _e49 = f90_1;
    let _e50 = fresnel;
    return ((_e44 * (1f - _e46)) + vec3((_e49 * _e50)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e36 = dotVH_3;
    let _e40 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e36) - 6.98316f) * _e40));
    let _e44 = f0_3;
    let _e46 = fresnel_1;
    let _e49 = f90_3;
    let _e50 = fresnel_1;
    return ((_e44 * (1f - _e46)) + (_e49 * _e50));
}

fn bayerDither2x2_(v_23: vec2<f32>) -> f32 {
    var v_24: vec2<f32>;

    v_24 = v_23;
    let _e31 = v_24;
    let _e35 = v_24;
    let _e38 = ((3f * _e31.y) + (2f * _e35.x));
    return (_e38 - (floor((_e38 / 4f)) * 4f));
}

fn bayerDither4x4_(v_25: vec2<f32>) -> f32 {
    var v_26: vec2<f32>;
    var P1_: vec2<f32>;
    var P2_: vec2<f32>;

    v_26 = v_25;
    let _e30 = v_26;
    let _e32 = vec2(2f);
    P1_ = (_e30 - (floor((_e30 / _e32)) * _e32));
    let _e39 = v_26;
    let _e41 = vec2(4f);
    P2_ = floor((0.5f * (_e39 - (floor((_e39 / _e41)) * _e41))));
    let _e50 = P1_;
    let _e51 = bayerDither2x2_(_e50);
    let _e53 = P2_;
    let _e54 = bayerDither2x2_(_e53);
    return ((4f * _e51) + _e54);
}

fn main_1() {
    var diffuseColor_2: vec4<f32>;
    var sampledDiffuseColor: vec4<f32>;
    var specularStrength: f32;
    var reflectedLight: ReflectedLight = ReflectedLight(vec3(0f), vec3(0f), vec3(0f), vec3(0f));
    var outgoingLight: vec3<f32>;
    var bayerValue: f32;
    var bayerBins: f32 = 16f;
    var dither: f32;

    let _e28 = global.diffuse;
    let _e29 = global.opacity;
    diffuseColor_2 = vec4<f32>(_e28.x, _e28.y, _e28.z, _e29);
    let _e35 = vMapUv_1;
    let _e36 = textureSample(map_t, map_s, _e35);
    sampledDiffuseColor = _e36;
    let _e38 = diffuseColor_2;
    let _e39 = sampledDiffuseColor;
    diffuseColor_2 = (_e38 * _e39);
    specularStrength = 1f;
    let _e54 = reflectedLight;
    reflectedLight.indirectDiffuse = (_e54.indirectDiffuse + vec3(1f));
    let _e60 = reflectedLight;
    let _e62 = diffuseColor_2;
    reflectedLight.indirectDiffuse = (_e60.indirectDiffuse * _e62.xyz);
    let _e65 = reflectedLight;
    outgoingLight = _e65.indirectDiffuse;
    diffuseColor_2.w = 1f;
    let _e70 = outgoingLight;
    let _e71 = diffuseColor_2;
    pc_fragColor = vec4<f32>(_e70.x, _e70.y, _e70.z, _e71.w);
    let _e77 = pc_fragColor;
    let _e78 = linearToOutputTexel(_e77);
    pc_fragColor = _e78;
    let _e80 = gl_FragCoord_1;
    let _e81 = _e80.xy;
    let _e83 = vec2(4f);
    let _e89 = bayerDither4x4_(floor((_e81 - (floor((_e81 / _e83)) * _e83))));
    bayerValue = _e89;
    let _e94 = bayerValue;
    let _e96 = bayerBins;
    dither = ((0.5f + _e94) / _e96);
    let _e99 = dither;
    let _e100 = global.fadeIn;
    if (_e99 >= _e100) {
        {
            discard;
        }
    }
    let _e102 = dither;
    let _e103 = global.fadeOut;
    if (_e102 < _e103) {
        {
            discard;
        }
    } else {
        return;
    }
}

@fragment 
fn main(@location(0) vMapUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vMapUv_1 = vMapUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e37 = pc_fragColor;
    return FragmentOutput(_e37);
}
