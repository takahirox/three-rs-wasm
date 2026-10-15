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

struct GroundIrradiance {
    sun: vec3<f32>,
    sky: vec3<f32>,
}

struct CloudsIrradiance {
    minSun: vec3<f32>,
    minSky: vec3<f32>,
    maxSun: vec3<f32>,
    maxSky: vec3<f32>,
}

struct CloudDensityProfile {
    expTerms: vec4<f32>,
    exponents: vec4<f32>,
    linearTerms: vec4<f32>,
    constantTerms: vec4<f32>,
}

struct WeatherSample {
    heightFraction: vec4<f32>,
    density: vec4<f32>,
}

struct MediaSample {
    density: f32,
    weight: vec4<f32>,
    scattering: f32,
    extinction: f32,
}

struct Uniforms {
    resolution: vec2<f32>,
    frame: i32,
    bottomRadius: f32,
    worldToECEFMatrix: mat4x4<f32>,
    ecefToWorldMatrix: mat4x4<f32>,
    altitudeCorrection: vec3<f32>,
    sunDirection: vec3<f32>,
    scatteringCoefficient: f32,
    absorptionCoefficient: f32,
    minDensity: f32,
    minExtinction: f32,
    minTransmittance: f32,
    localWeatherRepeat: vec2<f32>,
    localWeatherOffset: vec2<f32>,
    coverage: f32,
    shapeRepeat: vec3<f32>,
    shapeOffset: vec3<f32>,
    shapeDetailRepeat: vec3<f32>,
    shapeDetailOffset: vec3<f32>,
    turbulenceRepeat: vec2<f32>,
    turbulenceDisplacement: f32,
    minLayerHeights: vec4<f32>,
    maxLayerHeights: vec4<f32>,
    minIntervalHeights: vec3<f32>,
    maxIntervalHeights: vec3<f32>,
    densityScales: vec4<f32>,
    shapeAmounts: vec4<f32>,
    shapeDetailAmounts: vec4<f32>,
    weatherExponents: vec4<f32>,
    shapeAlteringBiases: vec4<f32>,
    coverageFilterWidths: vec4<f32>,
    minHeight: f32,
    maxHeight: f32,
    shadowTopHeight: f32,
    shadowBottomHeight: f32,
    shadowLayerMask: vec4<f32>,
    pt_s_densityProfile_expTerms: vec4<f32>,
    pt_s_densityProfile_exponents: vec4<f32>,
    pt_s_densityProfile_linearTerms: vec4<f32>,
    pt_s_densityProfile_constantTerms: vec4<f32>,
    inverseShadowMatrices: array<mat4x4<f32>, 2>,
    reprojectionMatrices: array<mat4x4<f32>, 2>,
    maxIterationCount: i32,
    minStepSize: f32,
    maxStepSize: f32,
    opticalDepthTailScale: f32,
}

struct FragmentOutput {
    @location(0) outputColor: vec4<f32>,
    @location(1) outputColor_1: vec4<f32>,
    @location(2) pt_out_outputDepthVelocity: vec4<f32>,
    @location(3) pt_out_outputDepthVelocity_1: vec4<f32>,
}

const mipLevels: array<f32, 4> = array<f32, 4>(0f, 0.5f, 1f, 2f);

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var stbnTexture_t: texture_3d<f32>;
@group(0) @binding(2) 
var stbnTexture_s: sampler;
@group(0) @binding(3) 
var localWeatherTexture_t: texture_2d<f32>;
@group(0) @binding(4) 
var localWeatherTexture_s: sampler;
@group(0) @binding(5) 
var shapeTexture_t: texture_3d<f32>;
@group(0) @binding(6) 
var shapeTexture_s: sampler;
@group(0) @binding(7) 
var shapeDetailTexture_t: texture_3d<f32>;
@group(0) @binding(8) 
var shapeDetailTexture_s: sampler;
@group(0) @binding(9) 
var turbulenceTexture_t: texture_2d<f32>;
@group(0) @binding(10) 
var turbulenceTexture_s: sampler;
var<private> densityProfile: CloudDensityProfile;
var<private> vUv_1: vec2<f32>;
var<private> gl_FragCoord_1: vec4<f32>;
var<private> outputColor_1: array<vec4<f32>, 2>;
var<private> pt_out_outputDepthVelocity: array<vec4<f32>, 2>;
var<private> outputDepthVelocity_1: array<vec3<f32>, 2>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e107 = y_1;
    let _e108 = x_1;
    return exp2((_e107 * log2(_e108)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e107 = y_3;
    let _e108 = x_3;
    return exp2((_e107 * log2(_e108)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e107 = y_5;
    let _e108 = x_5;
    return exp2((_e107 * log2(_e108)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e107 = y_7;
    let _e108 = x_7;
    return exp2((_e107 * log2(_e108)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e106 = x_9;
    (*i) = trunc(_e106);
    let _e108 = x_9;
    let _e109 = (*i);
    return (_e108 - _e109);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e106 = x_11;
    (*i_1) = trunc(_e106);
    let _e108 = x_11;
    let _e109 = (*i_1);
    return (_e108 - _e109);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e106 = x_13;
    (*i_2) = trunc(_e106);
    let _e108 = x_13;
    let _e109 = (*i_2);
    return (_e108 - _e109);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e106 = x_15;
    (*i_3) = trunc(_e106);
    let _e108 = x_15;
    let _e109 = (*i_3);
    return (_e108 - _e109);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e105 = v_1;
    b = (bitcast<u32>(_e105) & 2147483647u);
    let _e110 = b;
    return (_e110 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e105 = v_3;
    b_1 = (bitcast<u32>(_e105) & 2147483647u);
    let _e110 = b_1;
    return (_e110 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e105 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e105) & vec2(2147483647u));
    let _e111 = b_2;
    return (_e111 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e105 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e105) & vec3(2147483647u));
    let _e111 = b_3;
    return (_e111 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e105 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e105) & vec4(2147483647u));
    let _e111 = b_4;
    return (_e111 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e105 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e105) & vec2(2147483647u));
    let _e111 = b_5;
    return (_e111 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e105 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e105) & vec3(2147483647u));
    let _e111 = b_6;
    return (_e111 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e105 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e105) & vec4(2147483647u));
    let _e111 = b_7;
    return (_e111 == vec4(2139095040u));
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e106 = x_17;
    let _e107 = x_17;
    return (_e106 * _e107);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e106 = x_19;
    let _e107 = x_19;
    return (_e106 * _e107);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e106 = x_21;
    let _e107 = x_21;
    let _e109 = x_21;
    return ((_e106 * _e107) * _e109);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e106 = x_23;
    let _e107 = x_23;
    x2_ = (_e106 * _e107);
    let _e110 = x2_;
    let _e111 = x2_;
    return (_e110 * _e111);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e106 = v_17;
    let _e108 = v_17;
    let _e111 = v_17;
    return max(max(_e106.x, _e108.y), _e111.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e106 = v_19;
    return dot(_e106, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e112 = uv_1;
    let _e114 = a;
    let _e115 = b_8;
    dt = dot(_e112.xy, vec2<f32>(_e114, _e115));
    let _e119 = dt;
    sn = (_e119 - (floor((_e119 / 3.1415927f)) * 3.1415927f));
    let _e126 = sn;
    let _e128 = c;
    return fract((sin(_e126) * _e128));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;
    var maxComponent: f32;

    v_21 = v_20;
    let _e106 = v_21;
    let _e108 = max3_(abs(_e106));
    maxComponent = _e108;
    let _e110 = v_21;
    let _e111 = maxComponent;
    let _e115 = maxComponent;
    return (length((_e110 / vec3(_e111))) * _e115);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e108 = matrix_1;
    let _e109 = dir_1;
    return normalize((_e108 * vec4<f32>(_e109.x, _e109.y, _e109.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e108 = normal_1;
    let _e114 = viewMatrix_1;
    return normalize((vec4<f32>(_e108.x, _e108.y, _e108.z, 0f) * _e114).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e108 = dir_3;
    let _e114 = viewMatrix_3;
    return normalize((vec4<f32>(_e108.x, _e108.y, _e108.z, 0f) * _e114).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e110 = m_1[2][3];
    return (_e110 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e106 = dir_5;
    let _e108 = dir_5;
    u = ((atan2(_e106.z, _e108.x) * 0.15915494f) + 0.5f);
    let _e116 = dir_5;
    v_22 = ((asin(clamp(_e116.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e128 = u;
    let _e129 = v_22;
    return vec2<f32>(_e128, _e129);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e107 = diffuseColor_1;
    return (0.31830987f * _e107);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e112 = dotVH_1;
    let _e116 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e112) - 6.98316f) * _e116));
    let _e120 = f0_1;
    let _e122 = fresnel;
    let _e125 = f90_1;
    let _e126 = fresnel;
    return ((_e120 * (1f - _e122)) + vec3((_e125 * _e126)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e112 = dotVH_3;
    let _e116 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e112) - 6.98316f) * _e116));
    let _e120 = f0_3;
    let _e122 = fresnel_1;
    let _e125 = f90_3;
    let _e126 = fresnel_1;
    return ((_e120 * (1f - _e122)) + (_e125 * _e126));
}

fn remap(x_24: f32, min1_: f32, max1_: f32, min2_: f32, max2_: f32) -> f32 {
    return (min2_ + (((x_24 - min1_) / (max1_ - min1_)) * (max2_ - min2_)));
}

fn remap_1(x_25: vec2<f32>, min1_1: vec2<f32>, max1_1: vec2<f32>, min2_1: vec2<f32>, max2_1: vec2<f32>) -> vec2<f32> {
    return (min2_1 + (((x_25 - min1_1) / (max1_1 - min1_1)) * (max2_1 - min2_1)));
}

fn remap_2(x_26: vec3<f32>, min1_2: vec3<f32>, max1_2: vec3<f32>, min2_2: vec3<f32>, max2_2: vec3<f32>) -> vec3<f32> {
    return (min2_2 + (((x_26 - min1_2) / (max1_2 - min1_2)) * (max2_2 - min2_2)));
}

fn remap_3(x_27: vec4<f32>, min1_3: vec4<f32>, max1_3: vec4<f32>, min2_3: vec4<f32>, max2_3: vec4<f32>) -> vec4<f32> {
    return (min2_3 + (((x_27 - min1_3) / (max1_3 - min1_3)) * (max2_3 - min2_3)));
}

fn remapClamped(x_28: f32, min1_4: f32, max1_4: f32, min2_4: f32, max2_4: f32) -> f32 {
    return clamp((min2_4 + (((x_28 - min1_4) / (max1_4 - min1_4)) * (max2_4 - min2_4))), min2_4, max2_4);
}

fn remapClamped_1(x_29: vec2<f32>, min1_5: vec2<f32>, max1_5: vec2<f32>, min2_5: vec2<f32>, max2_5: vec2<f32>) -> vec2<f32> {
    return clamp((min2_5 + (((x_29 - min1_5) / (max1_5 - min1_5)) * (max2_5 - min2_5))), min2_5, max2_5);
}

fn remapClamped_2(x_30: vec3<f32>, min1_6: vec3<f32>, max1_6: vec3<f32>, min2_6: vec3<f32>, max2_6: vec3<f32>) -> vec3<f32> {
    return clamp((min2_6 + (((x_30 - min1_6) / (max1_6 - min1_6)) * (max2_6 - min2_6))), min2_6, max2_6);
}

fn remapClamped_3(x_31: vec4<f32>, min1_7: vec4<f32>, max1_7: vec4<f32>, min2_7: vec4<f32>, max2_7: vec4<f32>) -> vec4<f32> {
    return clamp((min2_7 + (((x_31 - min1_7) / (max1_7 - min1_7)) * (max2_7 - min2_7))), min2_7, max2_7);
}

fn remap_4(x_32: f32, min1_8: f32, max1_8: f32) -> f32 {
    return ((x_32 - min1_8) / (max1_8 - min1_8));
}

fn remap_5(x_33: vec2<f32>, min1_9: vec2<f32>, max1_9: vec2<f32>) -> vec2<f32> {
    return ((x_33 - min1_9) / (max1_9 - min1_9));
}

fn remap_6(x_34: vec3<f32>, min1_10: vec3<f32>, max1_10: vec3<f32>) -> vec3<f32> {
    return ((x_34 - min1_10) / (max1_10 - min1_10));
}

fn remap_7(x_35: vec4<f32>, min1_11: vec4<f32>, max1_11: vec4<f32>) -> vec4<f32> {
    return ((x_35 - min1_11) / (max1_11 - min1_11));
}

fn remapClamped_4(x_36: f32, min1_12: f32, max1_12: f32) -> f32 {
    return clamp(((x_36 - min1_12) / (max1_12 - min1_12)), 0f, 1f);
}

fn remapClamped_5(x_37: vec2<f32>, min1_13: vec2<f32>, max1_13: vec2<f32>) -> vec2<f32> {
    return clamp(((x_37 - min1_13) / (max1_13 - min1_13)), vec2(0f), vec2(1f));
}

fn remapClamped_6(x_38: vec3<f32>, min1_14: vec3<f32>, max1_14: vec3<f32>) -> vec3<f32> {
    return clamp(((x_38 - min1_14) / (max1_14 - min1_14)), vec3(0f), vec3(1f));
}

fn remapClamped_7(x_39: vec4<f32>, min1_15: vec4<f32>, max1_15: vec4<f32>) -> vec4<f32> {
    return clamp(((x_39 - min1_15) / (max1_15 - min1_15)), vec4(0f), vec4(1f));
}

fn raySphereFirstIntersection(origin: vec3<f32>, direction: vec3<f32>, center: vec3<f32>, radius: f32) -> f32 {
    var a_1: vec3<f32>;
    var b_9: f32;
    var c_1: f32;
    var discriminant: f32;
    var local: f32;

    a_1 = (origin - center);
    let _e111 = a_1;
    b_9 = (2f * dot(direction, _e111));
    let _e115 = a_1;
    let _e116 = a_1;
    c_1 = (dot(_e115, _e116) - (radius * radius));
    let _e121 = b_9;
    let _e122 = b_9;
    let _e125 = c_1;
    discriminant = ((_e121 * _e122) - (4f * _e125));
    let _e129 = discriminant;
    if (_e129 < 0f) {
        local = -1f;
    } else {
        let _e134 = b_9;
        let _e136 = discriminant;
        local = ((-(_e134) - sqrt(_e136)) * 0.5f);
    }
    let _e142 = local;
    return _e142;
}

fn raySphereFirstIntersection_1(origin_1: vec3<f32>, direction_1: vec3<f32>, radius_1: f32) -> f32 {
    let _e109 = raySphereFirstIntersection(origin_1, direction_1, vec3(0f), radius_1);
    return _e109;
}

fn raySphereFirstIntersection_2(origin_2: vec3<f32>, direction_2: vec3<f32>, center_1: vec3<f32>, radius_2: vec4<f32>) -> vec4<f32> {
    var a_2: vec3<f32>;
    var b_10: f32;
    var c_2: vec4<f32>;
    var discriminant_1: vec4<f32>;
    var mask: vec4<f32>;

    a_2 = (origin_2 - center_1);
    let _e111 = a_2;
    b_10 = (2f * dot(direction_2, _e111));
    let _e115 = a_2;
    let _e116 = a_2;
    c_2 = (vec4(dot(_e115, _e116)) - (radius_2 * radius_2));
    let _e122 = b_10;
    let _e123 = b_10;
    let _e126 = c_2;
    discriminant_1 = (vec4((_e122 * _e123)) - (4f * _e126));
    let _e131 = discriminant_1;
    mask = step(_e131, vec4(0f));
    let _e136 = b_10;
    let _e140 = discriminant_1;
    let _e150 = mask;
    return mix(((vec4(-(_e136)) - sqrt(max(vec4(0f), _e140))) * 0.5f), vec4(-1f), _e150);
}

fn raySphereFirstIntersection_3(origin_3: vec3<f32>, direction_3: vec3<f32>, radius_3: vec4<f32>) -> vec4<f32> {
    let _e109 = raySphereFirstIntersection_2(origin_3, direction_3, vec3(0f), radius_3);
    return _e109;
}

fn raySphereSecondIntersection(origin_4: vec3<f32>, direction_4: vec3<f32>, center_2: vec3<f32>, radius_4: f32) -> f32 {
    var a_3: vec3<f32>;
    var b_11: f32;
    var c_3: f32;
    var discriminant_2: f32;
    var local_1: f32;

    a_3 = (origin_4 - center_2);
    let _e111 = a_3;
    b_11 = (2f * dot(direction_4, _e111));
    let _e115 = a_3;
    let _e116 = a_3;
    c_3 = (dot(_e115, _e116) - (radius_4 * radius_4));
    let _e121 = b_11;
    let _e122 = b_11;
    let _e125 = c_3;
    discriminant_2 = ((_e121 * _e122) - (4f * _e125));
    let _e129 = discriminant_2;
    if (_e129 < 0f) {
        local_1 = -1f;
    } else {
        let _e134 = b_11;
        let _e136 = discriminant_2;
        local_1 = ((-(_e134) + sqrt(_e136)) * 0.5f);
    }
    let _e142 = local_1;
    return _e142;
}

fn raySphereSecondIntersection_1(origin_5: vec3<f32>, direction_5: vec3<f32>, radius_5: f32) -> f32 {
    let _e109 = raySphereSecondIntersection(origin_5, direction_5, vec3(0f), radius_5);
    return _e109;
}

fn raySphereSecondIntersection_2(origin_6: vec3<f32>, direction_6: vec3<f32>, center_3: vec3<f32>, radius_6: vec4<f32>) -> vec4<f32> {
    var a_4: vec3<f32>;
    var b_12: f32;
    var c_4: vec4<f32>;
    var discriminant_3: vec4<f32>;
    var mask_1: vec4<f32>;

    a_4 = (origin_6 - center_3);
    let _e111 = a_4;
    b_12 = (2f * dot(direction_6, _e111));
    let _e115 = a_4;
    let _e116 = a_4;
    c_4 = (vec4(dot(_e115, _e116)) - (radius_6 * radius_6));
    let _e122 = b_12;
    let _e123 = b_12;
    let _e126 = c_4;
    discriminant_3 = (vec4((_e122 * _e123)) - (4f * _e126));
    let _e131 = discriminant_3;
    mask_1 = step(_e131, vec4(0f));
    let _e136 = b_12;
    let _e140 = discriminant_3;
    let _e150 = mask_1;
    return mix(((vec4(-(_e136)) + sqrt(max(vec4(0f), _e140))) * 0.5f), vec4(-1f), _e150);
}

fn raySphereSecondIntersection_3(origin_7: vec3<f32>, direction_7: vec3<f32>, radius_7: vec4<f32>) -> vec4<f32> {
    let _e109 = raySphereSecondIntersection_2(origin_7, direction_7, vec3(0f), radius_7);
    return _e109;
}

fn raySphereIntersections(origin_8: vec3<f32>, direction_8: vec3<f32>, center_4: vec3<f32>, radius_8: f32, intersection1_: ptr<function, f32>, intersection2_: ptr<function, f32>) {
    var a_5: vec3<f32>;
    var b_13: f32;
    var c_5: f32;
    var discriminant_4: f32;
    var Q: f32;

    a_5 = (origin_8 - center_4);
    let _e113 = a_5;
    b_13 = (2f * dot(direction_8, _e113));
    let _e117 = a_5;
    let _e118 = a_5;
    c_5 = (dot(_e117, _e118) - (radius_8 * radius_8));
    let _e123 = b_13;
    let _e124 = b_13;
    let _e127 = c_5;
    discriminant_4 = ((_e123 * _e124) - (4f * _e127));
    let _e131 = discriminant_4;
    if (_e131 < 0f) {
        {
            (*intersection1_) = -1f;
            (*intersection2_) = -1f;
            return;
        }
    } else {
        {
            let _e138 = discriminant_4;
            Q = sqrt(_e138);
            let _e141 = b_13;
            let _e143 = Q;
            (*intersection1_) = ((-(_e141) - _e143) * 0.5f);
            let _e147 = b_13;
            let _e149 = Q;
            (*intersection2_) = ((-(_e147) + _e149) * 0.5f);
            return;
        }
    }
}

fn raySphereIntersections_1(origin_9: vec3<f32>, direction_9: vec3<f32>, radius_9: f32, intersection1_1: ptr<function, f32>, intersection2_1: ptr<function, f32>) {
    raySphereIntersections(origin_9, direction_9, vec3(0f), radius_9, intersection1_1, intersection2_1);
    return;
}

fn raySphereIntersections_2(origin_10: vec3<f32>, direction_10: vec3<f32>, center_5: vec3<f32>, radius_10: vec4<f32>, intersection1_2: ptr<function, vec4<f32>>, intersection2_2: ptr<function, vec4<f32>>) {
    var a_6: vec3<f32>;
    var b_14: f32;
    var c_6: vec4<f32>;
    var discriminant_5: vec4<f32>;
    var mask_2: vec4<f32>;
    var Q_1: vec4<f32>;

    a_6 = (origin_10 - center_5);
    let _e113 = a_6;
    b_14 = (2f * dot(direction_10, _e113));
    let _e117 = a_6;
    let _e118 = a_6;
    c_6 = (vec4(dot(_e117, _e118)) - (radius_10 * radius_10));
    let _e124 = b_14;
    let _e125 = b_14;
    let _e128 = c_6;
    discriminant_5 = (vec4((_e124 * _e125)) - (4f * _e128));
    let _e133 = discriminant_5;
    mask_2 = step(_e133, vec4(0f));
    let _e140 = discriminant_5;
    Q_1 = sqrt(max(vec4(0f), _e140));
    let _e144 = b_14;
    let _e146 = Q_1;
    let _e154 = mask_2;
    (*intersection1_2) = mix(((vec4(-(_e144)) - _e146) * 0.5f), vec4(-1f), _e154);
    let _e156 = b_14;
    let _e158 = Q_1;
    let _e166 = mask_2;
    (*intersection2_2) = mix(((vec4(-(_e156)) + _e158) * 0.5f), vec4(-1f), _e166);
    return;
}

fn raySphereIntersections_3(origin_11: vec3<f32>, direction_11: vec3<f32>, radius_11: vec4<f32>, intersection1_3: ptr<function, vec4<f32>>, intersection2_3: ptr<function, vec4<f32>>) {
    raySphereIntersections_2(origin_11, direction_11, vec3(0f), radius_11, intersection1_3, intersection2_3);
    return;
}

fn getIcosahedralVertices(direction_12: vec3<f32>, v1_: ptr<function, vec3<f32>>, v2_: ptr<function, vec3<f32>>, v3_: ptr<function, vec3<f32>>) {
    var a_7: f32 = 0.8506508f;
    var b_15: f32 = 0.5257311f;
    var kT: f32 = 0.618034f;
    var kT2_: f32 = 0.38196602f;
    var absD: vec3<f32>;
    var selector1_: f32;
    var selector2_: f32;
    var selector3_: f32;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var octantSign: vec3<f32>;

    absD = abs(direction_12);
    let _e118 = absD;
    let _e120 = kT2_;
    let _e121 = kT;
    selector1_ = dot(_e118, vec3<f32>(1f, _e120, -(_e121)));
    let _e126 = absD;
    let _e127 = kT;
    let _e130 = kT2_;
    selector2_ = dot(_e126, vec3<f32>(-(_e127), 1f, _e130));
    let _e134 = absD;
    let _e135 = kT2_;
    let _e136 = kT;
    selector3_ = dot(_e134, vec3<f32>(_e135, -(_e136), 1f));
    let _e142 = selector1_;
    if (_e142 > 0f) {
        let _e145 = a_7;
        let _e146 = b_15;
        local_2 = vec3<f32>(_e145, _e146, 0f);
    } else {
        let _e149 = b_15;
        let _e152 = a_7;
        local_2 = vec3<f32>(-(_e149), 0f, _e152);
    }
    let _e155 = local_2;
    (*v1_) = _e155;
    let _e156 = selector2_;
    if (_e156 > 0f) {
        let _e160 = a_7;
        let _e161 = b_15;
        local_3 = vec3<f32>(0f, _e160, _e161);
    } else {
        let _e163 = a_7;
        let _e164 = b_15;
        local_3 = vec3<f32>(_e163, -(_e164), 0f);
    }
    let _e169 = local_3;
    (*v2_) = _e169;
    let _e170 = selector3_;
    if (_e170 > 0f) {
        let _e173 = b_15;
        let _e175 = a_7;
        local_4 = vec3<f32>(_e173, 0f, _e175);
    } else {
        let _e178 = a_7;
        let _e179 = b_15;
        local_4 = vec3<f32>(0f, _e178, -(_e179));
    }
    let _e183 = local_4;
    (*v3_) = _e183;
    octantSign = sign(direction_12);
    let _e186 = (*v1_);
    let _e187 = octantSign;
    (*v1_) = (_e186 * _e187);
    let _e189 = (*v2_);
    let _e190 = octantSign;
    (*v2_) = (_e189 * _e190);
    let _e192 = (*v3_);
    let _e193 = octantSign;
    (*v3_) = (_e192 * _e193);
    return;
}

fn swapIfBigger(a_8: ptr<function, vec4<f32>>, b_16: ptr<function, vec4<f32>>) {
    var t: vec4<f32>;

    let _e106 = (*a_8);
    let _e108 = (*b_16);
    if (_e106.w > _e108.w) {
        {
            let _e111 = (*a_8);
            t = _e111;
            let _e113 = (*b_16);
            (*a_8) = _e113;
            let _e114 = t;
            (*b_16) = _e114;
            return;
        }
    } else {
        return;
    }
}

fn sortVertices(a_9: ptr<function, vec3<f32>>, b_17: ptr<function, vec3<f32>>, c_7: ptr<function, vec3<f32>>) {
    var base: vec3<f32> = vec3<f32>(0.5f, 0.5f, 1f);
    var aw: vec4<f32>;
    var bw: vec4<f32>;
    var cw: vec4<f32>;

    let _e112 = (*a_9);
    let _e113 = (*a_9);
    let _e114 = base;
    aw = vec4<f32>(_e112.x, _e112.y, _e112.z, dot(_e113, _e114));
    let _e121 = (*b_17);
    let _e122 = (*b_17);
    let _e123 = base;
    bw = vec4<f32>(_e121.x, _e121.y, _e121.z, dot(_e122, _e123));
    let _e130 = (*c_7);
    let _e131 = (*c_7);
    let _e132 = base;
    cw = vec4<f32>(_e130.x, _e130.y, _e130.z, dot(_e131, _e132));
    swapIfBigger((&aw), (&bw));
    swapIfBigger((&bw), (&cw));
    swapIfBigger((&aw), (&bw));
    let _e151 = aw;
    (*a_9) = _e151.xyz;
    let _e153 = bw;
    (*b_17) = _e153.xyz;
    let _e155 = cw;
    (*c_7) = _e155.xyz;
    return;
}

fn getPentagonalWeights(direction_13: vec3<f32>, v1_1: vec3<f32>, v2_1: vec3<f32>, v3_1: vec3<f32>) -> vec3<f32> {
    var d1_: f32;
    var d2_: f32;
    var d3_: f32;
    var w: vec3<f32>;

    d1_ = dot(v1_1, direction_13);
    d2_ = dot(v2_1, direction_13);
    d3_ = dot(v3_1, direction_13);
    let _e114 = d1_;
    let _e115 = d2_;
    let _e116 = d3_;
    w = exp((vec3<f32>(_e114, _e115, _e116) * 40f));
    let _e122 = w;
    let _e123 = w;
    let _e125 = w;
    let _e128 = w;
    return (_e122 / vec3(((_e123.x + _e125.y) + _e128.z)));
}

fn getStructureNormal(direction_14: vec3<f32>, jitter: f32, a_10: ptr<function, vec3<f32>>, b_18: ptr<function, vec3<f32>>, c_8: ptr<function, vec3<f32>>, weights: ptr<function, vec3<f32>>) -> vec3<f32> {
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;

    getIcosahedralVertices(direction_14, a_10, b_18, c_8);
    sortVertices(a_10, b_18, c_8);
    let _e122 = (*a_10);
    let _e123 = (*b_18);
    let _e124 = (*c_8);
    let _e125 = getPentagonalWeights(direction_14, _e122, _e123, _e124);
    (*weights) = _e125;
    let _e126 = (*weights);
    if (jitter < _e126.x) {
        let _e129 = (*a_10);
        local_6 = _e129;
    } else {
        let _e130 = (*weights);
        let _e132 = (*weights);
        if (jitter < (_e130.x + _e132.y)) {
            let _e136 = (*b_18);
            local_5 = _e136;
        } else {
            let _e137 = (*c_8);
            local_5 = _e137;
        }
        let _e139 = local_5;
        local_6 = _e139;
    }
    let _e141 = local_6;
    return _e141;
}

fn getStructureNormal_1(direction_15: vec3<f32>, jitter_1: f32) -> vec3<f32> {
    var a_11: vec3<f32>;
    var b_19: vec3<f32>;
    var c_9: vec3<f32>;
    var weights_1: vec3<f32>;

    let _e118 = getStructureNormal(direction_15, jitter_1, (&a_11), (&b_19), (&c_9), (&weights_1));
    return _e118;
}

fn intersectStructuredPlanes(normal_2: vec3<f32>, rayOrigin: vec3<f32>, rayDirection: vec3<f32>, samplePeriod: f32, stepOffset: ptr<function, f32>, stepSize: ptr<function, f32>) {
    var NoD: f32;

    NoD = dot(rayDirection, normal_2);
    let _e112 = NoD;
    (*stepSize) = (samplePeriod / abs(_e112));
    let _e115 = dot(rayOrigin, normal_2);
    let _e121 = NoD;
    (*stepOffset) = (-((_e115 - (floor((_e115 / samplePeriod)) * samplePeriod))) / _e121);
    let _e123 = (*stepOffset);
    if (_e123 < 0f) {
        {
            let _e126 = (*stepOffset);
            let _e127 = (*stepSize);
            (*stepOffset) = (_e126 + _e127);
            return;
        }
    } else {
        return;
    }
}

fn getSTBN() -> f32 {
    var size: vec3<i32>;
    var scale: vec3<f32>;

    let _e105 = textureDimensions(stbnTexture_t, 0i);
    size = vec3<i32>(_e105);
    let _e109 = size;
    scale = (vec3(1f) / vec3<f32>(_e109));
    let _e115 = gl_FragCoord_1;
    let _e116 = _e115.xy;
    let _e117 = global.frame;
    let _e118 = size;
    let _e125 = scale;
    let _e127 = textureSample(stbnTexture_t, stbnTexture_s, (vec3<f32>(_e116.x, _e116.y, f32((_e117 % _e118.z))) * _e125));
    return _e127.x;
}

fn getSphericalUv(position: vec3<f32>) -> vec2<f32> {
    var st: vec2<f32>;
    var phi: f32;
    var theta: f32;

    st = normalize(position.yx);
    let _e109 = st;
    let _e111 = st;
    phi = atan2(_e109.x, _e111.y);
    theta = asin(normalize(position).z);
    let _e119 = phi;
    let _e124 = theta;
    return vec2<f32>(((_e119 * 0.15915494f) + 0.5f), ((_e124 * 0.31830987f) + 0.5f));
}

fn getCubeSphereUv(position_1: vec3<f32>) -> vec2<f32> {
    var n: vec3<f32>;
    var f: vec3<f32>;
    var c_10: vec3<f32>;
    var m_2: vec2<f32>;
    var local_7: vec2<f32>;
    var local_8: vec2<f32>;
    var local_9: vec2<f32>;
    var m2_: vec2<f32>;
    var q: f32;
    var q2_: f32;
    var uv_2: vec2<f32>;
    var local_10: f32;

    n = normalize(position_1);
    let _e108 = n;
    f = abs(_e108);
    let _e111 = n;
    let _e112 = f;
    let _e114 = f;
    let _e116 = f;
    c_10 = (_e111 / vec3(max(_e112.x, max(_e114.y, _e116.z))));
    let _e124 = f;
    let _e126 = f;
    if all((_e124.yy > _e126.xz)) {
        {
            let _e130 = c_10;
            if (_e130.y > 0f) {
                let _e134 = n;
                let _e137 = n;
                local_7 = vec2<f32>(-(_e134.x), _e137.z);
            } else {
                let _e140 = n;
                local_7 = _e140.xz;
            }
            let _e143 = local_7;
            m_2 = _e143;
        }
    } else {
        let _e144 = f;
        let _e146 = f;
        if all((_e144.xx > _e146.yz)) {
            {
                let _e150 = c_10;
                if (_e150.x > 0f) {
                    let _e154 = n;
                    local_8 = _e154.yz;
                } else {
                    let _e156 = n;
                    let _e159 = n;
                    local_8 = vec2<f32>(-(_e156.y), _e159.z);
                }
                let _e163 = local_8;
                m_2 = _e163;
            }
        } else {
            {
                let _e164 = c_10;
                if (_e164.z > 0f) {
                    let _e168 = n;
                    local_9 = _e168.xy;
                } else {
                    let _e170 = n;
                    let _e172 = n;
                    local_9 = vec2<f32>(_e170.x, -(_e172.y));
                }
                let _e177 = local_9;
                m_2 = _e177;
            }
        }
    }
    let _e178 = m_2;
    let _e179 = m_2;
    m2_ = (_e178 * _e179);
    let _e182 = m2_;
    q = (dot(_e182.xy, vec2<f32>(-2f, 2f)) - 3f);
    let _e192 = q;
    let _e193 = q;
    q2_ = (_e192 * _e193);
    let _e199 = m2_;
    let _e202 = m2_;
    let _e208 = m2_;
    let _e211 = q2_;
    let _e217 = m_2;
    if (_e217.x > 0f) {
        local_10 = 1f;
    } else {
        local_10 = -1f;
    }
    let _e225 = local_10;
    uv_2.x = (sqrt((((1.5f + _e199.x) - _e202.y) - (0.5f * sqrt(((-24f * _e208.x) + _e211))))) * _e225);
    let _e230 = uv_2;
    let _e232 = uv_2;
    let _e238 = m_2;
    uv_2.y = (sqrt((6f / (3f - (_e230.x * _e232.x)))) * _e238.y);
    let _e241 = uv_2;
    return ((_e241 * 0.5f) + vec2(0.5f));
}

fn getGlobeUv(position_2: vec3<f32>) -> vec2<f32> {
    let _e106 = getCubeSphereUv(position_2);
    return _e106;
}

fn getMipLevel(uv_3: vec2<f32>) -> f32 {
    var mipLevelScale: f32 = 0.1f;
    var coord: vec2<f32>;
    var ddx: vec2<f32>;
    var ddy: vec2<f32>;
    var deltaMaxSqr: f32;

    let _e108 = global.resolution;
    coord = (uv_3 * _e108);
    let _e111 = coord;
    let _e112 = dpdx(_e111);
    ddx = _e112;
    let _e114 = coord;
    let _e115 = dpdy(_e114);
    ddy = _e115;
    let _e117 = ddx;
    let _e118 = ddx;
    let _e120 = ddy;
    let _e121 = ddy;
    let _e124 = mipLevelScale;
    deltaMaxSqr = (max(dot(_e117, _e118), dot(_e120, _e121)) * _e124);
    let _e130 = deltaMaxSqr;
    return max(0f, (0.5f * log2(max(1f, _e130))));
}

fn insideLayerIntervals(height: f32) -> bool {
    var gt: vec3<bool>;
    var lt: vec3<bool>;

    let _e107 = global.minIntervalHeights;
    gt = (vec3(height) > _e107);
    let _e111 = global.maxIntervalHeights;
    lt = (vec3(height) < _e111);
    let _e114 = gt;
    let _e116 = lt;
    let _e119 = gt;
    let _e121 = lt;
    let _e124 = gt;
    let _e126 = lt;
    return any(vec3<bool>((_e114.x && _e116.x), (_e119.y && _e121.y), (_e124.z && _e126.z)));
}

fn shapeAlteringFunction(heightFraction: vec4<f32>, bias: vec4<f32>) -> vec4<f32> {
    var biased: vec4<f32>;
    var x_40: vec4<f32>;

    let _e107 = pt_pow_3(heightFraction, bias);
    biased = _e107;
    let _e109 = biased;
    x_40 = clamp(((_e109 * 2f) - vec4(1f)), vec4(-1f), vec4(1f));
    let _e123 = x_40;
    let _e124 = x_40;
    return (vec4(1f) - (_e123 * _e124));
}

fn sampleWeather(uv_4: vec2<f32>, height_1: f32, mipLevel: f32) -> WeatherSample {
    var weather: WeatherSample;
    var localWeather: vec4<f32>;
    var heightScale: vec4<f32>;
    var factor: vec4<f32>;

    let _e111 = global.minLayerHeights;
    let _e112 = global.maxLayerHeights;
    let _e113 = remapClamped_7(vec4(height_1), _e111, _e112);
    weather.heightFraction = _e113;
    let _e114 = global.localWeatherRepeat;
    let _e116 = global.localWeatherOffset;
    let _e118 = textureSampleLevel(localWeatherTexture_t, localWeatherTexture_s, ((uv_4 * _e114) + _e116), mipLevel);
    let _e120 = global.weatherExponents;
    let _e121 = pt_pow_3(_e118.xyzw, _e120);
    localWeather = _e121;
    let _e123 = localWeather;
    let _e124 = global.shadowLayerMask;
    localWeather = (_e123 * _e124);
    let _e126 = weather;
    let _e128 = global.shapeAlteringBiases;
    let _e129 = shapeAlteringFunction(_e126.heightFraction, _e128);
    heightScale = _e129;
    let _e132 = global.coverage;
    let _e133 = heightScale;
    factor = (vec4(1f) - (_e132 * _e133));
    let _e139 = localWeather;
    let _e142 = global.coverageFilterWidths;
    let _e144 = factor;
    let _e145 = factor;
    let _e146 = global.coverageFilterWidths;
    let _e148 = remapClamped_7(mix(_e139, vec4(1f), _e142), _e144, (_e145 + _e146));
    weather.density = _e148;
    let _e149 = weather;
    return _e149;
}

fn getLayerDensity(heightFraction_1: vec4<f32>) -> vec4<f32> {
    let _e106 = densityProfile;
    let _e108 = densityProfile;
    let _e113 = densityProfile;
    let _e117 = densityProfile;
    return (((_e106.expTerms * exp((_e108.exponents * heightFraction_1))) + (_e113.linearTerms * heightFraction_1)) + _e117.constantTerms);
}

fn sampleMedia(weather_1: WeatherSample, position_3: vec3<f32>, uv_5: vec2<f32>, mipLevel_1: f32, jitter_2: f32, sampleCount: ptr<function, vec3<i32>>) -> MediaSample {
    var density: vec4<f32>;
    var surfaceNormal: vec3<f32>;
    var localWeatherSpeed: f32;
    var evolution: vec3<f32>;
    var turbulence: vec3<f32> = vec3(0f);
    var turbulenceUv: vec2<f32>;
    var shapePosition: vec3<f32>;
    var shape: f32;
    var detailPosition: vec3<f32>;
    var detail: f32;
    var modifier: vec4<f32>;
    var media: MediaSample;
    var densitySum: f32;

    density = weather_1.density;
    surfaceNormal = normalize(position_3);
    let _e115 = global.localWeatherOffset;
    localWeatherSpeed = length(_e115);
    let _e118 = surfaceNormal;
    let _e120 = localWeatherSpeed;
    evolution = ((-(_e118) * _e120) * 20000f);
    let _e128 = global.localWeatherRepeat;
    let _e130 = global.turbulenceRepeat;
    turbulenceUv = ((uv_5 * _e128) * _e130);
    let _e133 = global.turbulenceDisplacement;
    let _e134 = turbulenceUv;
    let _e135 = textureSample(turbulenceTexture_t, turbulenceTexture_s, _e134);
    let _e143 = density;
    let _e149 = remapClamped_7(weather_1.heightFraction, vec4(0.3f), vec4(0f));
    turbulence = ((_e133 * ((_e135.xyz * 2f) - vec3(1f))) * dot(_e143, _e149));
    let _e152 = evolution;
    let _e154 = turbulence;
    let _e156 = global.shapeRepeat;
    let _e158 = global.shapeOffset;
    shapePosition = ((((position_3 + _e152) + _e154) * _e156) + _e158);
    let _e161 = shapePosition;
    let _e162 = textureSample(shapeTexture_t, shapeTexture_s, _e161);
    shape = _e162.x;
    let _e165 = density;
    let _e167 = shape;
    let _e170 = global.shapeAmounts;
    let _e174 = remapClamped_7(_e165, (vec4((1f - _e167)) * _e170), vec4(1f));
    density = _e174;
    if (((mipLevel_1 * 0.5f) + ((jitter_2 - 0.5f) * 0.5f)) < 0.5f) {
        {
            let _e184 = turbulence;
            let _e186 = global.shapeDetailRepeat;
            let _e188 = global.shapeDetailOffset;
            detailPosition = (((position_3 + _e184) * _e186) + _e188);
            let _e191 = detailPosition;
            let _e192 = textureSample(shapeDetailTexture_t, shapeDetailTexture_s, _e191);
            detail = _e192.x;
            let _e195 = detail;
            let _e197 = pt_pow(_e195, 6f);
            let _e200 = detail;
            let _e208 = remapClamped_7(weather_1.heightFraction, vec4(0.2f), vec4(0.4f));
            modifier = mix(vec4(_e197), vec4((1f - _e200)), _e208);
            let _e213 = modifier;
            let _e214 = global.shapeDetailAmounts;
            modifier = mix(vec4(0f), _e213, _e214);
            let _e216 = density;
            let _e219 = modifier;
            let _e225 = remapClamped_7((_e216 * 2f), vec4<f32>((_e219 * 0.5f)), vec4(1f));
            density = _e225;
        }
    }
    let _e226 = density;
    let _e227 = global.densityScales;
    let _e230 = getLayerDensity(weather_1.heightFraction);
    density = clamp(((_e226 * _e227) * _e230), vec4(0f), vec4(1f));
    let _e238 = density;
    let _e240 = density;
    let _e243 = density;
    let _e246 = density;
    densitySum = (((_e238.x + _e240.y) + _e243.z) + _e246.w);
    let _e251 = density;
    let _e252 = densitySum;
    media.weight = (_e251 / vec4(_e252));
    let _e256 = densitySum;
    let _e257 = global.scatteringCoefficient;
    media.scattering = (_e256 * _e257);
    let _e260 = densitySum;
    let _e261 = global.absorptionCoefficient;
    let _e263 = media;
    media.extinction = ((_e260 * _e261) + _e263.scattering);
    let _e266 = media;
    return _e266;
}

fn sampleMedia_1(weather_2: WeatherSample, position_4: vec3<f32>, uv_6: vec2<f32>, mipLevel_2: f32, jitter_3: f32) -> MediaSample {
    var sampleCount_1: vec3<i32>;

    let _e113 = sampleMedia(weather_2, position_4, uv_6, mipLevel_2, jitter_3, (&sampleCount_1));
    return _e113;
}

fn marchClouds(rayOrigin_1: vec3<f32>, rayDirection_1: vec3<f32>, maxRayDistance: f32, jitter_4: f32, mipLevel_3: f32) -> vec4<f32> {
    var normal_3: vec3<f32>;
    var rayDistance: f32;
    var stepSize_1: f32;
    var extinctionSum: f32 = 0f;
    var maxOpticalDepth: f32 = 0f;
    var maxOpticalDepthTail: f32 = 0f;
    var transmittanceIntegral: f32 = 1f;
    var weightedDistanceSum: f32 = 0f;
    var transmittanceSum: f32 = 0f;
    var sampleCount_2: i32 = 0i;
    var i_4: i32 = 0i;
    var position_5: vec3<f32>;
    var height_2: f32;
    var uv_7: vec2<f32>;
    var weather_3: WeatherSample;
    var media_1: MediaSample;
    var frontDepth: f32;
    var meanExtinction: f32;

    let _e113 = getStructureNormal_1(rayDirection_1, jitter_4);
    normal_3 = _e113;
    let _e117 = normal_3;
    let _e118 = global.maxIterationCount;
    let _e121 = global.minStepSize;
    let _e122 = global.maxStepSize;
    intersectStructuredPlanes(_e117, rayOrigin_1, rayDirection_1, clamp((maxRayDistance / f32(_e118)), _e121, _e122), (&rayDistance), (&stepSize_1));
    let _e128 = rayDistance;
    let _e129 = stepSize_1;
    rayDistance = (_e128 - (_e129 * jitter_4));
    loop {
        let _e148 = i_4;
        let _e149 = global.maxIterationCount;
        if !((_e148 < _e149)) {
            break;
        }
        {
            let _e155 = rayDistance;
            if (_e155 > maxRayDistance) {
                {
                    break;
                }
            }
            let _e157 = rayDistance;
            position_5 = ((_e157 * rayDirection_1) + rayOrigin_1);
            let _e161 = position_5;
            let _e163 = global.bottomRadius;
            height_2 = (length(_e161) - _e163);
            let _e166 = height_2;
            let _e167 = insideLayerIntervals(_e166);
            if _e167 {
                {
                    let _e168 = rayDistance;
                    let _e169 = stepSize_1;
                    rayDistance = (_e168 + _e169);
                    continue;
                }
            }
            let _e171 = position_5;
            let _e172 = getGlobeUv(_e171);
            uv_7 = _e172;
            let _e174 = uv_7;
            let _e175 = height_2;
            let _e176 = sampleWeather(_e174, _e175, mipLevel_3);
            weather_3 = _e176;
            let _e178 = weather_3;
            let _e180 = global.minDensity;
            if any((_e178.density > vec4(_e180))) {
                {
                    let _e184 = weather_3;
                    let _e185 = position_5;
                    let _e186 = uv_7;
                    let _e187 = sampleMedia_1(_e184, _e185, _e186, mipLevel_3, jitter_4);
                    media_1 = _e187;
                    let _e189 = media_1;
                    let _e191 = global.minExtinction;
                    if (_e189.extinction > _e191) {
                        {
                            let _e193 = extinctionSum;
                            let _e194 = media_1;
                            extinctionSum = (_e193 + _e194.extinction);
                            let _e197 = maxOpticalDepth;
                            let _e198 = media_1;
                            let _e200 = stepSize_1;
                            maxOpticalDepth = (_e197 + (_e198.extinction * _e200));
                            let _e203 = transmittanceIntegral;
                            let _e204 = media_1;
                            let _e207 = stepSize_1;
                            transmittanceIntegral = (_e203 * exp((-(_e204.extinction) * _e207)));
                            let _e211 = weightedDistanceSum;
                            let _e212 = rayDistance;
                            let _e213 = transmittanceIntegral;
                            weightedDistanceSum = (_e211 + (_e212 * _e213));
                            let _e216 = transmittanceSum;
                            let _e217 = transmittanceIntegral;
                            transmittanceSum = (_e216 + _e217);
                            let _e219 = sampleCount_2;
                            sampleCount_2 = (_e219 + 1i);
                        }
                    }
                }
            }
            let _e222 = transmittanceIntegral;
            let _e223 = global.minTransmittance;
            if (_e222 <= _e223) {
                {
                    let _e225 = global.opticalDepthTailScale;
                    let _e226 = stepSize_1;
                    let _e229 = sampleCount_2;
                    let _e234 = stepSize_1;
                    maxOpticalDepthTail = min(((_e225 * _e226) * exp(f32((1i - _e229)))), (_e234 * 0.5f));
                    break;
                }
            }
            let _e238 = rayDistance;
            let _e239 = stepSize_1;
            rayDistance = (_e238 + _e239);
        }
        continuing {
            let _e152 = i_4;
            i_4 = (_e152 + 1i);
        }
    }
    let _e241 = sampleCount_2;
    if (_e241 == 0i) {
        {
            return vec4<f32>(maxRayDistance, 0f, 0f, 0f);
        }
    }
    let _e248 = weightedDistanceSum;
    let _e249 = transmittanceSum;
    frontDepth = min((_e248 / _e249), maxRayDistance);
    let _e253 = extinctionSum;
    let _e254 = sampleCount_2;
    meanExtinction = (_e253 / f32(_e254));
    let _e258 = frontDepth;
    let _e259 = meanExtinction;
    let _e260 = maxOpticalDepth;
    let _e261 = maxOpticalDepthTail;
    return vec4<f32>(_e258, _e259, _e260, _e261);
}

fn getRayNearFar(sunPosition: vec3<f32>, rayDirection_2: vec3<f32>, rayNear: ptr<function, f32>, rayFar: ptr<function, f32>) {
    var firstIntersections: vec4<f32>;

    let _e114 = global.bottomRadius;
    let _e115 = global.shadowTopHeight;
    let _e116 = global.shadowBottomHeight;
    let _e122 = raySphereFirstIntersection_2(sunPosition, rayDirection_2, vec3(0f), (vec4(_e114) + vec4<f32>(_e115, _e116, 0f, 0f)));
    firstIntersections = _e122;
    let _e125 = firstIntersections;
    (*rayNear) = max(0f, _e125.x);
    let _e128 = firstIntersections;
    (*rayFar) = _e128.y;
    let _e130 = (*rayFar);
    if (_e130 < 0f) {
        {
            (*rayFar) = 1000000f;
            return;
        }
    } else {
        return;
    }
}

fn cascade(cascadeIndex: i32, mipLevel_4: f32, outputColor: ptr<function, vec4<f32>>, outputDepthVelocity: ptr<function, vec3<f32>>) {
    var clip: vec2<f32>;
    var point: vec4<f32>;
    var sunPosition_1: vec3<f32>;
    var rayDirection_3: vec3<f32>;
    var rayNear_1: f32;
    var rayFar_1: f32;
    var rayOrigin_2: vec3<f32>;
    var stbn: f32;
    var color: vec4<f32>;
    var frontPosition: vec3<f32>;
    var frontPositionWorld: vec3<f32>;
    var prevClip: vec4<f32>;
    var prevUv: vec2<f32>;
    var velocity: vec2<f32>;

    let _e112 = vUv_1;
    clip = ((_e112 * 2f) - vec2(1f));
    let _e120 = global.inverseShadowMatrices[cascadeIndex];
    let _e121 = clip;
    let _e122 = _e121.xy;
    point = (_e120 * vec4<f32>(_e122.x, _e122.y, -1f, 1f));
    let _e131 = point;
    let _e132 = point;
    point = (_e131 / vec4(_e132.w));
    let _e136 = global.worldToECEFMatrix;
    let _e137 = point;
    let _e138 = _e137.xyz;
    let _e146 = global.altitudeCorrection;
    sunPosition_1 = ((_e136 * vec4<f32>(_e138.x, _e138.y, _e138.z, 1f)).xyz + _e146);
    let _e149 = global.sunDirection;
    rayDirection_3 = normalize(-(_e149));
    let _e155 = sunPosition_1;
    let _e156 = rayDirection_3;
    getRayNearFar(_e155, _e156, (&rayNear_1), (&rayFar_1));
    let _e161 = rayNear_1;
    let _e162 = rayDirection_3;
    let _e164 = sunPosition_1;
    rayOrigin_2 = ((_e161 * _e162) + _e164);
    let _e167 = getSTBN();
    stbn = _e167;
    let _e169 = rayOrigin_2;
    let _e170 = rayDirection_3;
    let _e171 = rayFar_1;
    let _e172 = rayNear_1;
    let _e174 = stbn;
    let _e175 = marchClouds(_e169, _e170, (_e171 - _e172), _e174, mipLevel_4);
    color = _e175;
    let _e177 = color;
    (*outputColor) = _e177;
    let _e178 = color;
    let _e180 = rayDirection_3;
    let _e182 = rayOrigin_2;
    frontPosition = ((_e178.x * _e180) + _e182);
    let _e185 = global.ecefToWorldMatrix;
    let _e186 = frontPosition;
    let _e187 = global.altitudeCorrection;
    let _e188 = (_e186 - _e187);
    frontPositionWorld = (_e185 * vec4<f32>(_e188.x, _e188.y, _e188.z, 1f)).xyz;
    let _e198 = global.reprojectionMatrices[cascadeIndex];
    let _e199 = frontPositionWorld;
    prevClip = (_e198 * vec4<f32>(_e199.x, _e199.y, _e199.z, 1f));
    let _e207 = prevClip;
    let _e208 = prevClip;
    prevClip = (_e207 / vec4(_e208.w));
    let _e212 = prevClip;
    prevUv = ((_e212.xy * 0.5f) + vec2(0.5f));
    let _e220 = vUv_1;
    let _e221 = prevUv;
    let _e223 = global.resolution;
    velocity = ((_e220 - _e221) * _e223);
    let _e226 = color;
    let _e228 = velocity;
    (*outputDepthVelocity) = vec3<f32>(_e226.x, _e228.x, _e228.y);
    return;
}

fn pt_main() {
    var local_11: vec4<f32>;
    var local_12: vec3<f32>;
    var local_13: vec4<f32>;
    var local_14: vec3<f32>;

    cascade(0i, 0f, (&local_11), (&local_12));
    let _e130 = local_11;
    outputColor_1[0i] = _e130;
    let _e131 = local_12;
    outputDepthVelocity_1[0i] = _e131;
    cascade(1i, 0.5f, (&local_13), (&local_14));
    let _e153 = local_13;
    outputColor_1[1i] = _e153;
    let _e154 = local_14;
    outputDepthVelocity_1[1i] = _e154;
    return;
}

fn main_1() {
    let _e110 = global.pt_s_densityProfile_expTerms;
    densityProfile.expTerms = _e110;
    let _e112 = global.pt_s_densityProfile_exponents;
    densityProfile.exponents = _e112;
    let _e114 = global.pt_s_densityProfile_linearTerms;
    densityProfile.linearTerms = _e114;
    let _e116 = global.pt_s_densityProfile_constantTerms;
    densityProfile.constantTerms = _e116;
    pt_main();
    let _e121 = outputDepthVelocity_1[0];
    pt_out_outputDepthVelocity[0i] = vec4<f32>(_e121.x, _e121.y, _e121.z, 1f);
    let _e131 = outputDepthVelocity_1[1];
    pt_out_outputDepthVelocity[1i] = vec4<f32>(_e131.x, _e131.y, _e131.z, 1f);
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e130 = outputColor_1[0];
    let _e132 = outputColor_1[1];
    let _e135 = pt_out_outputDepthVelocity[0];
    let _e137 = pt_out_outputDepthVelocity[1];
    return FragmentOutput(_e130, _e132, _e135, _e137);
}
