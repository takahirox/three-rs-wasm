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

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

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
var<private> vUv: vec2<f32>;
var<private> position_1: vec3<f32>;
var<private> gl_Position: vec4<f32>;

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

fn pt_main() {
    let _e105 = position_1;
    vUv = ((_e105.xy * 0.5f) + vec2(0.5f));
    let _e113 = position_1;
    let _e114 = _e113.xy;
    gl_Position = vec4<f32>(_e114.x, _e114.y, 1f, 1f);
    return;
}

fn main_1() {
    let _e107 = global.pt_s_densityProfile_expTerms;
    densityProfile.expTerms = _e107;
    let _e109 = global.pt_s_densityProfile_exponents;
    densityProfile.exponents = _e109;
    let _e111 = global.pt_s_densityProfile_linearTerms;
    densityProfile.linearTerms = _e111;
    let _e113 = global.pt_s_densityProfile_constantTerms;
    densityProfile.constantTerms = _e113;
    pt_main();
    let _e115 = gl_Position;
    gl_Position.y = -(_e115.y);
    let _e119 = gl_Position;
    let _e121 = gl_Position;
    gl_Position.z = ((_e119.z + _e121.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>) -> VertexOutput {
    position_1 = position;
    main_1();
    let _e121 = vUv;
    let _e123 = gl_Position;
    return VertexOutput(_e121, _e123);
}
