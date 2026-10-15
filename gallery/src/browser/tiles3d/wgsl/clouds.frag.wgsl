diagnostic(off, derivative_uniformity);
struct DensityProfileLayer {
    width: f32,
    exp_term: f32,
    exp_scale: f32,
    linear_term: f32,
    constant_term: f32,
}

struct DensityProfile {
    layers: array<DensityProfileLayer, 2>,
}

struct AtmosphereParameters {
    solar_irradiance: vec3<f32>,
    sun_angular_radius: f32,
    bottom_radius: f32,
    top_radius: f32,
    rayleigh_density: DensityProfile,
    rayleigh_scattering: vec3<f32>,
    mie_density: DensityProfile,
    mie_scattering: vec3<f32>,
    mie_extinction: vec3<f32>,
    mie_phase_function_g: f32,
    absorption_density: DensityProfile,
    absorption_extinction: vec3<f32>,
    ground_albedo: vec3<f32>,
    mu_s_min: f32,
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

struct IntersectionResult {
    ground: bool,
    first: vec4<f32>,
    second: vec4<f32>,
}

struct Uniforms {
    pt_s_ATMOSPHERE_solar_irradiance: vec3<f32>,
    pt_s_ATMOSPHERE_sun_angular_radius: f32,
    pt_s_ATMOSPHERE_bottom_radius: f32,
    pt_s_ATMOSPHERE_top_radius: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_0_width: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_1_width: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term: f32,
    pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term: f32,
    pt_s_ATMOSPHERE_rayleigh_scattering: vec3<f32>,
    pt_s_ATMOSPHERE_mie_density_layers_0_width: f32,
    pt_s_ATMOSPHERE_mie_density_layers_0_exp_term: f32,
    pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale: f32,
    pt_s_ATMOSPHERE_mie_density_layers_0_linear_term: f32,
    pt_s_ATMOSPHERE_mie_density_layers_0_constant_term: f32,
    pt_s_ATMOSPHERE_mie_density_layers_1_width: f32,
    pt_s_ATMOSPHERE_mie_density_layers_1_exp_term: f32,
    pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale: f32,
    pt_s_ATMOSPHERE_mie_density_layers_1_linear_term: f32,
    pt_s_ATMOSPHERE_mie_density_layers_1_constant_term: f32,
    pt_s_ATMOSPHERE_mie_scattering: vec3<f32>,
    pt_s_ATMOSPHERE_mie_extinction: vec3<f32>,
    pt_s_ATMOSPHERE_mie_phase_function_g: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_0_width: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_1_width: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term: f32,
    pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term: f32,
    pt_s_ATMOSPHERE_absorption_extinction: vec3<f32>,
    pt_s_ATMOSPHERE_ground_albedo: vec3<f32>,
    pt_s_ATMOSPHERE_mu_s_min: f32,
    SUN_SPECTRAL_RADIANCE_TO_LUMINANCE: vec3<f32>,
    SKY_SPECTRAL_RADIANCE_TO_LUMINANCE: vec3<f32>,
    inverseProjectionMatrix: mat4x4<f32>,
    inverseViewMatrix: mat4x4<f32>,
    cameraPosition: vec3<f32>,
    worldToECEFMatrix: mat4x4<f32>,
    altitudeCorrection: vec3<f32>,
    bottomRadius: f32,
    sunDirection: vec3<f32>,
    minHeight: f32,
    maxHeight: f32,
    resolution: vec2<f32>,
    frame: i32,
    ecefToWorldMatrix: mat4x4<f32>,
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
    hazeDensityScale: f32,
    hazeExponent: f32,
    hazeScatteringCoefficient: f32,
    hazeAbsorptionCoefficient: f32,
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
    shadowTopHeight: f32,
    shadowBottomHeight: f32,
    shadowLayerMask: vec4<f32>,
    pt_s_densityProfile_expTerms: vec4<f32>,
    pt_s_densityProfile_exponents: vec4<f32>,
    pt_s_densityProfile_linearTerms: vec4<f32>,
    pt_s_densityProfile_constantTerms: vec4<f32>,
    viewMatrix: mat4x4<f32>,
    reprojectionMatrix: mat4x4<f32>,
    viewReprojectionMatrix: mat4x4<f32>,
    cameraNear: f32,
    cameraFar: f32,
    cameraHeight: f32,
    temporalJitter: vec2<f32>,
    targetUvScale: vec2<f32>,
    mipLevelScale: f32,
    skyLightScale: f32,
    groundBounceScale: f32,
    powderScale: f32,
    powderExponent: f32,
    maxIterationCount: i32,
    minStepSize: f32,
    maxStepSize: f32,
    maxRayDistance: f32,
    perspectiveStepScale: f32,
    maxIterationCountToSun: i32,
    maxIterationCountToGround: i32,
    minSecondaryStepSize: f32,
    secondaryStepScale: f32,
    shadowTexelSize: vec2<f32>,
    pt_u_shadowIntervals: array<vec4<f32>, 2>,
    shadowMatrices: array<mat4x4<f32>, 2>,
    shadowFar: f32,
    maxShadowFilterRadius: f32,
    maxShadowLengthIterationCount: i32,
    minShadowLengthStepSize: f32,
    maxShadowLengthRayDistance: f32,
}

struct FragmentOutput {
    @location(0) outputColor: vec4<f32>,
    @location(1) pt_out_outputDepthVelocity: vec4<f32>,
    @location(2) outputShadowLength: f32,
}

const PackUpscale: f32 = 1.0039216f;
const UnpackDownscale: f32 = 0.99609375f;
const ShiftRight8_: f32 = 0.00390625f;
const Inv255_: f32 = 0.003921569f;
const PackFactors: vec4<f32> = vec4<f32>(1f, 256f, 65536f, 16777216f);
const UnpackFactors2_: vec2<f32> = vec2<f32>(0.99609375f, 0.00390625f);
const UnpackFactors3_: vec3<f32> = vec3<f32>(0.99609375f, 0.0038909912f, 0.000015258789f);
const UnpackFactors4_: vec4<f32> = vec4<f32>(0.99609375f, 0.0038909912f, 0.000015199184f, 0.000000059604645f);
const m_3: f32 = 1f;
const nm: f32 = 1f;
const rad: f32 = 1f;
const sr: f32 = 1f;
const watt: f32 = 1f;
const lm: f32 = 1f;
const km: f32 = 1000f;
const m2_1: f32 = 1f;
const m3_: f32 = 1f;
const pi: f32 = 3.1415927f;
const deg: f32 = 0.017453292f;
const watt_per_square_meter: f32 = 1f;
const watt_per_square_meter_per_sr: f32 = 1f;
const watt_per_square_meter_per_nm: f32 = 1f;
const watt_per_square_meter_per_sr_per_nm: f32 = 1f;
const watt_per_cubic_meter_per_sr_per_nm: f32 = 1f;
const cd: f32 = 1f;
const kcd: f32 = 1000f;
const cd_per_square_meter: f32 = 1f;
const kcd_per_square_meter: f32 = 1000f;
const scatterAnisotropy: vec2<f32> = vec2<f32>(0.7f, -0.2f);
const scatterAnisotropyMix: f32 = 0.5f;

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var transmittance_texture_t_7: texture_2d<f32>;
@group(0) @binding(2) 
var transmittance_texture_s_7: sampler;
@group(0) @binding(3) 
var scattering_texture_t_4: texture_3d<f32>;
@group(0) @binding(4) 
var scattering_texture_s_4: sampler;
@group(0) @binding(5) 
var irradiance_texture_t_3: texture_2d<f32>;
@group(0) @binding(6) 
var irradiance_texture_s_3: sampler;
@group(0) @binding(7) 
var single_mie_scattering_texture_t_3: texture_3d<f32>;
@group(0) @binding(8) 
var single_mie_scattering_texture_s_3: sampler;
@group(0) @binding(9) 
var higher_order_scattering_texture_t: texture_3d<f32>;
@group(0) @binding(10) 
var higher_order_scattering_texture_s: sampler;
@group(0) @binding(11) 
var stbnTexture_t: texture_3d<f32>;
@group(0) @binding(12) 
var stbnTexture_s: sampler;
@group(0) @binding(13) 
var localWeatherTexture_t: texture_2d<f32>;
@group(0) @binding(14) 
var localWeatherTexture_s: sampler;
@group(0) @binding(15) 
var shapeTexture_t: texture_3d<f32>;
@group(0) @binding(16) 
var shapeTexture_s: sampler;
@group(0) @binding(17) 
var shapeDetailTexture_t: texture_3d<f32>;
@group(0) @binding(18) 
var shapeDetailTexture_s: sampler;
@group(0) @binding(19) 
var turbulenceTexture_t: texture_2d<f32>;
@group(0) @binding(20) 
var turbulenceTexture_s: sampler;
@group(0) @binding(21) 
var depthBuffer_t_1: texture_2d<f32>;
@group(0) @binding(22) 
var depthBuffer_s_1: sampler;
@group(0) @binding(23) 
var shadowBuffer_t: texture_2d_array<f32>;
@group(0) @binding(24) 
var shadowBuffer_s: sampler;
var<private> shadowIntervals: array<vec2<f32>, 2>;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> densityProfile: CloudDensityProfile;
var<private> vUv_1: vec2<f32>;
var<private> vCameraPosition_1: vec3<f32>;
var<private> vCameraDirection_1: vec3<f32>;
var<private> vRayDirection_1: vec3<f32>;
var<private> vViewPosition_1: vec3<f32>;
var<private> vGroundIrradiance: GroundIrradiance;
var<private> vCloudsIrradiance: CloudsIrradiance;
var<private> gl_FragCoord_1: vec4<f32>;
var<private> outputColor: vec4<f32>;
var<private> pt_out_outputDepthVelocity: vec4<f32>;
var<private> outputDepthVelocity: vec3<f32>;
var<private> outputShadowLength: f32;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e271 = y_1;
    let _e272 = x_1;
    return exp2((_e271 * log2(_e272)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e271 = y_3;
    let _e272 = x_3;
    return exp2((_e271 * log2(_e272)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e271 = y_5;
    let _e272 = x_5;
    return exp2((_e271 * log2(_e272)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e271 = y_7;
    let _e272 = x_7;
    return exp2((_e271 * log2(_e272)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e270 = x_9;
    (*i) = trunc(_e270);
    let _e272 = x_9;
    let _e273 = (*i);
    return (_e272 - _e273);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e270 = x_11;
    (*i_1) = trunc(_e270);
    let _e272 = x_11;
    let _e273 = (*i_1);
    return (_e272 - _e273);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e270 = x_13;
    (*i_2) = trunc(_e270);
    let _e272 = x_13;
    let _e273 = (*i_2);
    return (_e272 - _e273);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e270 = x_15;
    (*i_3) = trunc(_e270);
    let _e272 = x_15;
    let _e273 = (*i_3);
    return (_e272 - _e273);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e269 = v_1;
    b = (bitcast<u32>(_e269) & 2147483647u);
    let _e274 = b;
    return (_e274 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e269 = v_3;
    b_1 = (bitcast<u32>(_e269) & 2147483647u);
    let _e274 = b_1;
    return (_e274 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e269 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e269) & vec2(2147483647u));
    let _e275 = b_2;
    return (_e275 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e269 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e269) & vec3(2147483647u));
    let _e275 = b_3;
    return (_e275 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e269 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e269) & vec4(2147483647u));
    let _e275 = b_4;
    return (_e275 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e269 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e269) & vec2(2147483647u));
    let _e275 = b_5;
    return (_e275 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e269 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e269) & vec3(2147483647u));
    let _e275 = b_6;
    return (_e275 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e269 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e269) & vec4(2147483647u));
    let _e275 = b_7;
    return (_e275 == vec4(2139095040u));
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e276 = x_17;
    let _e277 = x_17;
    return (_e276 * _e277);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e276 = x_19;
    let _e277 = x_19;
    return (_e276 * _e277);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e276 = x_21;
    let _e277 = x_21;
    let _e279 = x_21;
    return ((_e276 * _e277) * _e279);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e276 = x_23;
    let _e277 = x_23;
    x2_ = (_e276 * _e277);
    let _e280 = x2_;
    let _e281 = x2_;
    return (_e280 * _e281);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e276 = v_17;
    let _e278 = v_17;
    let _e281 = v_17;
    return max(max(_e276.x, _e278.y), _e281.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e276 = v_19;
    return dot(_e276, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e282 = uv_1;
    let _e284 = a;
    let _e285 = b_8;
    dt = dot(_e282.xy, vec2<f32>(_e284, _e285));
    let _e289 = dt;
    sn = (_e289 - (floor((_e289 / 3.1415927f)) * 3.1415927f));
    let _e296 = sn;
    let _e298 = c;
    return fract((sin(_e296) * _e298));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;
    var maxComponent: f32;

    v_21 = v_20;
    let _e276 = v_21;
    let _e278 = max3_(abs(_e276));
    maxComponent = _e278;
    let _e280 = v_21;
    let _e281 = maxComponent;
    let _e285 = maxComponent;
    return (length((_e280 / vec3(_e281))) * _e285);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e278 = matrix_1;
    let _e279 = dir_1;
    return normalize((_e278 * vec4<f32>(_e279.x, _e279.y, _e279.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e278 = normal_1;
    let _e284 = viewMatrix_1;
    return normalize((vec4<f32>(_e278.x, _e278.y, _e278.z, 0f) * _e284).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e278 = dir_3;
    let _e284 = viewMatrix_3;
    return normalize((vec4<f32>(_e278.x, _e278.y, _e278.z, 0f) * _e284).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e280 = m_1[2][3];
    return (_e280 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e276 = dir_5;
    let _e278 = dir_5;
    u = ((atan2(_e276.z, _e278.x) * 0.15915494f) + 0.5f);
    let _e286 = dir_5;
    v_22 = ((asin(clamp(_e286.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e298 = u;
    let _e299 = v_22;
    return vec2<f32>(_e298, _e299);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e277 = diffuseColor_1;
    return (0.31830987f * _e277);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e282 = dotVH_1;
    let _e286 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e282) - 6.98316f) * _e286));
    let _e290 = f0_1;
    let _e292 = fresnel;
    let _e295 = f90_1;
    let _e296 = fresnel;
    return ((_e290 * (1f - _e292)) + vec3((_e295 * _e296)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e282 = dotVH_3;
    let _e286 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e282) - 6.98316f) * _e286));
    let _e290 = f0_3;
    let _e292 = fresnel_1;
    let _e295 = f90_3;
    let _e296 = fresnel_1;
    return ((_e290 * (1f - _e292)) + (_e295 * _e296));
}

fn packNormalToRGB(normal_2: vec3<f32>) -> vec3<f32> {
    var normal_3: vec3<f32>;

    normal_3 = normal_2;
    let _e276 = normal_3;
    return ((normalize(_e276) * 0.5f) + vec3(0.5f));
}

fn unpackRGBToNormal(rgb: vec3<f32>) -> vec3<f32> {
    var rgb_1: vec3<f32>;

    rgb_1 = rgb;
    let _e277 = rgb_1;
    return ((2f * _e277.xyz) - vec3(1f));
}

fn packDepthToRGBA(v_23: f32) -> vec4<f32> {
    var v_24: f32;
    var vuf: f32;
    var af: f32;
    var bf: f32;
    var gf: f32;

    v_24 = v_23;
    let _e284 = v_24;
    if (_e284 <= 0f) {
        return vec4<f32>(0f, 0f, 0f, 0f);
    }
    let _e292 = v_24;
    if (_e292 >= 1f) {
        return vec4<f32>(1f, 1f, 1f, 1f);
    }
    let _e301 = v_24;
    let _e310 = pt_modf((_e301 * 16777216f), (&vuf));
    af = _e310;
    let _e312 = vuf;
    let _e316 = pt_modf((_e312 * ShiftRight8_), (&vuf));
    bf = _e316;
    let _e318 = vuf;
    let _e322 = pt_modf((_e318 * ShiftRight8_), (&vuf));
    gf = _e322;
    let _e324 = vuf;
    let _e326 = gf;
    let _e328 = bf;
    let _e330 = af;
    return vec4<f32>((_e324 * Inv255_), (_e326 * PackUpscale), (_e328 * PackUpscale), _e330);
}

fn packDepthToRGB(v_25: f32) -> vec3<f32> {
    var v_26: f32;
    var vuf_1: f32;
    var bf_1: f32;
    var gf_1: f32;

    v_26 = v_25;
    let _e284 = v_26;
    if (_e284 <= 0f) {
        return vec3<f32>(0f, 0f, 0f);
    }
    let _e291 = v_26;
    if (_e291 >= 1f) {
        return vec3<f32>(1f, 1f, 1f);
    }
    let _e299 = v_26;
    let _e308 = pt_modf((_e299 * 65536f), (&vuf_1));
    bf_1 = _e308;
    let _e310 = vuf_1;
    let _e314 = pt_modf((_e310 * ShiftRight8_), (&vuf_1));
    gf_1 = _e314;
    let _e316 = vuf_1;
    let _e318 = gf_1;
    let _e320 = bf_1;
    return vec3<f32>((_e316 * Inv255_), (_e318 * PackUpscale), _e320);
}

fn packDepthToRG(v_27: f32) -> vec2<f32> {
    var v_28: f32;
    var vuf_2: f32;
    var gf_2: f32;

    v_28 = v_27;
    let _e284 = v_28;
    if (_e284 <= 0f) {
        return vec2<f32>(0f, 0f);
    }
    let _e290 = v_28;
    if (_e290 >= 1f) {
        return vec2<f32>(1f, 1f);
    }
    let _e297 = v_28;
    let _e302 = pt_modf((_e297 * 256f), (&vuf_2));
    gf_2 = _e302;
    let _e304 = vuf_2;
    let _e306 = gf_2;
    return vec2<f32>((_e304 * Inv255_), _e306);
}

fn unpackRGBAToDepth(v_29: vec4<f32>) -> f32 {
    var v_30: vec4<f32>;

    v_30 = v_29;
    let _e284 = v_30;
    return dot(_e284, UnpackFactors4_);
}

fn unpackRGBToDepth(v_31: vec3<f32>) -> f32 {
    var v_32: vec3<f32>;

    v_32 = v_31;
    let _e284 = v_32;
    return dot(_e284, UnpackFactors3_);
}

fn unpackRGToDepth(v_33: vec2<f32>) -> f32 {
    var v_34: vec2<f32>;

    v_34 = v_33;
    let _e284 = v_34;
    let _e290 = v_34;
    return ((_e284.x * 0.99609375f) + (_e290.y * 0.00390625f));
}

fn pack2HalfToRGBA(v_35: vec2<f32>) -> vec4<f32> {
    var v_36: vec2<f32>;
    var r: vec4<f32>;

    v_36 = v_35;
    let _e284 = v_36;
    let _e286 = v_36;
    let _e291 = v_36;
    let _e293 = v_36;
    r = vec4<f32>(_e284.x, fract((_e286.x * 255f)), _e291.y, fract((_e293.y * 255f)));
    let _e300 = r;
    let _e302 = r;
    let _e307 = r;
    let _e309 = r;
    let _e311 = r;
    let _e316 = r;
    return vec4<f32>((_e300.x - (_e302.y / 255f)), _e307.y, (_e309.z - (_e311.w / 255f)), _e316.w);
}

fn unpackRGBATo2Half(v_37: vec4<f32>) -> vec2<f32> {
    var v_38: vec4<f32>;

    v_38 = v_37;
    let _e284 = v_38;
    let _e286 = v_38;
    let _e291 = v_38;
    let _e293 = v_38;
    return vec2<f32>((_e284.x + (_e286.y / 255f)), (_e291.z + (_e293.w / 255f)));
}

fn viewZToOrthographicDepth(viewZ: f32, near: f32, far: f32) -> f32 {
    var viewZ_1: f32;
    var near_1: f32;
    var far_1: f32;

    viewZ_1 = viewZ;
    near_1 = near;
    far_1 = far;
    let _e288 = viewZ_1;
    let _e289 = near_1;
    let _e291 = near_1;
    let _e292 = far_1;
    return ((_e288 + _e289) / (_e291 - _e292));
}

fn orthographicDepthToViewZ(depth: f32, near_2: f32, far_2: f32) -> f32 {
    var depth_1: f32;
    var near_3: f32;
    var far_3: f32;

    depth_1 = depth;
    near_3 = near_2;
    far_3 = far_2;
    let _e288 = depth_1;
    let _e289 = near_3;
    let _e290 = far_3;
    let _e293 = near_3;
    return ((_e288 * (_e289 - _e290)) - _e293);
}

fn viewZToPerspectiveDepth(viewZ_2: f32, near_4: f32, far_4: f32) -> f32 {
    var viewZ_3: f32;
    var near_5: f32;
    var far_5: f32;

    viewZ_3 = viewZ_2;
    near_5 = near_4;
    far_5 = far_4;
    let _e288 = near_5;
    let _e289 = viewZ_3;
    let _e291 = far_5;
    let _e293 = far_5;
    let _e294 = near_5;
    let _e296 = viewZ_3;
    return (((_e288 + _e289) * _e291) / ((_e293 - _e294) * _e296));
}

fn perspectiveDepthToViewZ(depth_2: f32, near_6: f32, far_6: f32) -> f32 {
    var depth_3: f32;
    var near_7: f32;
    var far_7: f32;

    depth_3 = depth_2;
    near_7 = near_6;
    far_7 = far_6;
    let _e288 = near_7;
    let _e289 = far_7;
    let _e291 = far_7;
    let _e292 = near_7;
    let _e294 = depth_3;
    let _e296 = far_7;
    return ((_e288 * _e289) / (((_e291 - _e292) * _e294) - _e296));
}

fn readDepthValue(depthBuffer_t: texture_2d<f32>, depthBuffer_s: sampler, uv_2: vec2<f32>) -> f32 {
    let _e285 = textureSample(depthBuffer_t, depthBuffer_s, uv_2);
    return _e285.x;
}

fn reverseLogDepth(depth_4: f32, near_8: f32, far_8: f32) -> f32 {
    return depth_4;
}

fn linearizeDepth(depth_5: f32, near_9: f32, far_9: f32) -> f32 {
    var ndc: f32;

    ndc = ((depth_5 * 2f) - 1f);
    let _e294 = ndc;
    return (((2f * near_9) * far_9) / ((far_9 + near_9) - (_e294 * (far_9 - near_9))));
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

fn turbo(x_40: f32) -> vec3<f32> {
    var r_1: f32;
    var g: f32;
    var b_9: f32;

    r_1 = (0.1357f + (x_40 * (4.5974f - (x_40 * (42.3277f - (x_40 * (130.5887f - (x_40 * (150.5666f - (x_40 * 58.1375f))))))))));
    g = (0.0914f + (x_40 * (2.1856f + (x_40 * (4.8052f - (x_40 * (14.0195f - (x_40 * (4.2109f + (x_40 * 2.7747f))))))))));
    b_9 = (0.1067f + (x_40 * (12.5925f - (x_40 * (60.1097f - (x_40 * (109.0745f - (x_40 * (88.5066f - (x_40 * 26.8183f))))))))));
    let _e334 = r_1;
    let _e335 = g;
    let _e336 = b_9;
    return vec3<f32>(_e334, _e335, _e336);
}

fn checker(uv_3: vec2<f32>, repeats: vec2<f32>) -> f32 {
    var c_1: vec2<f32>;
    var result: f32;

    c_1 = floor((repeats * uv_3));
    let _e287 = c_1;
    let _e289 = c_1;
    let _e291 = (_e287.x + _e289.y);
    result = (_e291 - (floor((_e291 / 2f)) * 2f));
    let _e298 = result;
    return sign(_e298);
}

fn checker_1(uv_4: vec2<f32>, repeats_1: f32) -> f32 {
    let _e285 = checker(uv_4, vec2(repeats_1));
    return _e285;
}

fn raySphereFirstIntersection(origin: vec3<f32>, direction: vec3<f32>, center: vec3<f32>, radius: f32) -> f32 {
    var a_1: vec3<f32>;
    var b_10: f32;
    var c_2: f32;
    var discriminant: f32;
    var local: f32;

    a_1 = (origin - center);
    let _e289 = a_1;
    b_10 = (2f * dot(direction, _e289));
    let _e293 = a_1;
    let _e294 = a_1;
    c_2 = (dot(_e293, _e294) - (radius * radius));
    let _e299 = b_10;
    let _e300 = b_10;
    let _e303 = c_2;
    discriminant = ((_e299 * _e300) - (4f * _e303));
    let _e307 = discriminant;
    if (_e307 < 0f) {
        local = -1f;
    } else {
        let _e312 = b_10;
        let _e314 = discriminant;
        local = ((-(_e312) - sqrt(_e314)) * 0.5f);
    }
    let _e320 = local;
    return _e320;
}

fn raySphereFirstIntersection_1(origin_1: vec3<f32>, direction_1: vec3<f32>, radius_1: f32) -> f32 {
    let _e287 = raySphereFirstIntersection(origin_1, direction_1, vec3(0f), radius_1);
    return _e287;
}

fn raySphereFirstIntersection_2(origin_2: vec3<f32>, direction_2: vec3<f32>, center_1: vec3<f32>, radius_2: vec4<f32>) -> vec4<f32> {
    var a_2: vec3<f32>;
    var b_11: f32;
    var c_3: vec4<f32>;
    var discriminant_1: vec4<f32>;
    var mask: vec4<f32>;

    a_2 = (origin_2 - center_1);
    let _e289 = a_2;
    b_11 = (2f * dot(direction_2, _e289));
    let _e293 = a_2;
    let _e294 = a_2;
    c_3 = (vec4(dot(_e293, _e294)) - (radius_2 * radius_2));
    let _e300 = b_11;
    let _e301 = b_11;
    let _e304 = c_3;
    discriminant_1 = (vec4((_e300 * _e301)) - (4f * _e304));
    let _e309 = discriminant_1;
    mask = step(_e309, vec4(0f));
    let _e314 = b_11;
    let _e318 = discriminant_1;
    let _e328 = mask;
    return mix(((vec4(-(_e314)) - sqrt(max(vec4(0f), _e318))) * 0.5f), vec4(-1f), _e328);
}

fn raySphereFirstIntersection_3(origin_3: vec3<f32>, direction_3: vec3<f32>, radius_3: vec4<f32>) -> vec4<f32> {
    let _e287 = raySphereFirstIntersection_2(origin_3, direction_3, vec3(0f), radius_3);
    return _e287;
}

fn raySphereSecondIntersection(origin_4: vec3<f32>, direction_4: vec3<f32>, center_2: vec3<f32>, radius_4: f32) -> f32 {
    var a_3: vec3<f32>;
    var b_12: f32;
    var c_4: f32;
    var discriminant_2: f32;
    var local_1: f32;

    a_3 = (origin_4 - center_2);
    let _e289 = a_3;
    b_12 = (2f * dot(direction_4, _e289));
    let _e293 = a_3;
    let _e294 = a_3;
    c_4 = (dot(_e293, _e294) - (radius_4 * radius_4));
    let _e299 = b_12;
    let _e300 = b_12;
    let _e303 = c_4;
    discriminant_2 = ((_e299 * _e300) - (4f * _e303));
    let _e307 = discriminant_2;
    if (_e307 < 0f) {
        local_1 = -1f;
    } else {
        let _e312 = b_12;
        let _e314 = discriminant_2;
        local_1 = ((-(_e312) + sqrt(_e314)) * 0.5f);
    }
    let _e320 = local_1;
    return _e320;
}

fn raySphereSecondIntersection_1(origin_5: vec3<f32>, direction_5: vec3<f32>, radius_5: f32) -> f32 {
    let _e287 = raySphereSecondIntersection(origin_5, direction_5, vec3(0f), radius_5);
    return _e287;
}

fn raySphereSecondIntersection_2(origin_6: vec3<f32>, direction_6: vec3<f32>, center_3: vec3<f32>, radius_6: vec4<f32>) -> vec4<f32> {
    var a_4: vec3<f32>;
    var b_13: f32;
    var c_5: vec4<f32>;
    var discriminant_3: vec4<f32>;
    var mask_1: vec4<f32>;

    a_4 = (origin_6 - center_3);
    let _e289 = a_4;
    b_13 = (2f * dot(direction_6, _e289));
    let _e293 = a_4;
    let _e294 = a_4;
    c_5 = (vec4(dot(_e293, _e294)) - (radius_6 * radius_6));
    let _e300 = b_13;
    let _e301 = b_13;
    let _e304 = c_5;
    discriminant_3 = (vec4((_e300 * _e301)) - (4f * _e304));
    let _e309 = discriminant_3;
    mask_1 = step(_e309, vec4(0f));
    let _e314 = b_13;
    let _e318 = discriminant_3;
    let _e328 = mask_1;
    return mix(((vec4(-(_e314)) + sqrt(max(vec4(0f), _e318))) * 0.5f), vec4(-1f), _e328);
}

fn raySphereSecondIntersection_3(origin_7: vec3<f32>, direction_7: vec3<f32>, radius_7: vec4<f32>) -> vec4<f32> {
    let _e287 = raySphereSecondIntersection_2(origin_7, direction_7, vec3(0f), radius_7);
    return _e287;
}

fn raySphereIntersections(origin_8: vec3<f32>, direction_8: vec3<f32>, center_4: vec3<f32>, radius_8: f32, intersection1_: ptr<function, f32>, intersection2_: ptr<function, f32>) {
    var a_5: vec3<f32>;
    var b_14: f32;
    var c_6: f32;
    var discriminant_4: f32;
    var Q: f32;

    a_5 = (origin_8 - center_4);
    let _e291 = a_5;
    b_14 = (2f * dot(direction_8, _e291));
    let _e295 = a_5;
    let _e296 = a_5;
    c_6 = (dot(_e295, _e296) - (radius_8 * radius_8));
    let _e301 = b_14;
    let _e302 = b_14;
    let _e305 = c_6;
    discriminant_4 = ((_e301 * _e302) - (4f * _e305));
    let _e309 = discriminant_4;
    if (_e309 < 0f) {
        {
            (*intersection1_) = -1f;
            (*intersection2_) = -1f;
            return;
        }
    } else {
        {
            let _e316 = discriminant_4;
            Q = sqrt(_e316);
            let _e319 = b_14;
            let _e321 = Q;
            (*intersection1_) = ((-(_e319) - _e321) * 0.5f);
            let _e325 = b_14;
            let _e327 = Q;
            (*intersection2_) = ((-(_e325) + _e327) * 0.5f);
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
    var b_15: f32;
    var c_7: vec4<f32>;
    var discriminant_5: vec4<f32>;
    var mask_2: vec4<f32>;
    var Q_1: vec4<f32>;

    a_6 = (origin_10 - center_5);
    let _e291 = a_6;
    b_15 = (2f * dot(direction_10, _e291));
    let _e295 = a_6;
    let _e296 = a_6;
    c_7 = (vec4(dot(_e295, _e296)) - (radius_10 * radius_10));
    let _e302 = b_15;
    let _e303 = b_15;
    let _e306 = c_7;
    discriminant_5 = (vec4((_e302 * _e303)) - (4f * _e306));
    let _e311 = discriminant_5;
    mask_2 = step(_e311, vec4(0f));
    let _e318 = discriminant_5;
    Q_1 = sqrt(max(vec4(0f), _e318));
    let _e322 = b_15;
    let _e324 = Q_1;
    let _e332 = mask_2;
    (*intersection1_2) = mix(((vec4(-(_e322)) - _e324) * 0.5f), vec4(-1f), _e332);
    let _e334 = b_15;
    let _e336 = Q_1;
    let _e344 = mask_2;
    (*intersection2_2) = mix(((vec4(-(_e334)) + _e336) * 0.5f), vec4(-1f), _e344);
    return;
}

fn raySphereIntersections_3(origin_11: vec3<f32>, direction_11: vec3<f32>, radius_11: vec4<f32>, intersection1_3: ptr<function, vec4<f32>>, intersection2_3: ptr<function, vec4<f32>>) {
    raySphereIntersections_2(origin_11, direction_11, vec3(0f), radius_11, intersection1_3, intersection2_3);
    return;
}

fn getCascadeIndex(viewMatrix_4: mat4x4<f32>, worldPosition: vec3<f32>, intervals: array<vec2<f32>, 2>, near_10: f32, far_10: f32) -> i32 {
    var viewPosition: vec4<f32>;
    var depth_6: f32;
    var interval: vec2<f32>;

    viewPosition = (viewMatrix_4 * vec4<f32>(worldPosition.x, worldPosition.y, worldPosition.z, 1f));
    let _e294 = viewPosition;
    let _e296 = viewZToOrthographicDepth(_e294.z, near_10, far_10);
    depth_6 = _e296;
    interval = intervals[0];
    let _e301 = depth_6;
    let _e302 = interval;
    let _e305 = depth_6;
    let _e306 = interval;
    if ((_e301 >= _e302.x) && (_e305 < _e306.y)) {
        {
            return 0i;
        }
    }
    interval = intervals[1];
    let _e313 = depth_6;
    let _e314 = interval;
    let _e317 = depth_6;
    let _e318 = interval;
    if ((_e313 >= _e314.x) && (_e317 < _e318.y)) {
        {
            return 1i;
        }
    }
    return 1i;
}

fn getFadedCascadeIndex(viewMatrix_5: mat4x4<f32>, worldPosition_1: vec3<f32>, intervals_1: array<vec2<f32>, 2>, near_11: f32, far_11: f32, jitter: f32) -> i32 {
    var viewPosition_1: vec4<f32>;
    var depth_7: f32;
    var interval_1: vec2<f32>;
    var intervalCenter: f32;
    var closestEdge: f32;
    var margin: f32;
    var nextIndex: i32 = -1i;
    var prevIndex: i32 = -1i;
    var alpha: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: i32;

    viewPosition_1 = (viewMatrix_5 * vec4<f32>(worldPosition_1.x, worldPosition_1.y, worldPosition_1.z, 1f));
    let _e295 = viewPosition_1;
    let _e297 = viewZToOrthographicDepth(_e295.z, near_11, far_11);
    depth_7 = _e297;
    interval_1 = intervals_1[0];
    let _e312 = interval_1;
    let _e314 = interval_1;
    intervalCenter = ((_e312.x + _e314.y) * 0.5f);
    let _e319 = depth_7;
    let _e320 = intervalCenter;
    if (_e319 < _e320) {
        let _e322 = interval_1;
        local_2 = _e322.x;
    } else {
        let _e324 = interval_1;
        local_2 = _e324.y;
    }
    let _e327 = local_2;
    closestEdge = _e327;
    let _e328 = closestEdge;
    let _e329 = closestEdge;
    margin = ((_e328 * _e329) * 0.5f);
    let _e333 = interval_1;
    let _e334 = margin;
    interval_1 = (_e333 + (_e334 * vec2<f32>(-0.5f, 0.5f)));
    let _e341 = depth_7;
    let _e342 = interval_1;
    let _e345 = depth_7;
    let _e346 = interval_1;
    if ((_e341 >= _e342.x) && (_e345 < _e346.y)) {
        {
            let _e350 = nextIndex;
            prevIndex = _e350;
            nextIndex = 0i;
            let _e352 = depth_7;
            let _e353 = interval_1;
            let _e356 = interval_1;
            let _e358 = depth_7;
            let _e361 = margin;
            alpha = clamp((min((_e352 - _e353.x), (_e356.y - _e358)) / _e361), 0f, 1f);
        }
    }
    interval_1 = intervals_1[1];
    let _e368 = interval_1;
    let _e370 = interval_1;
    intervalCenter = ((_e368.x + _e370.y) * 0.5f);
    let _e375 = depth_7;
    let _e376 = intervalCenter;
    if (_e375 < _e376) {
        let _e378 = interval_1;
        local_3 = _e378.x;
    } else {
        let _e380 = interval_1;
        local_3 = _e380.y;
    }
    let _e383 = local_3;
    closestEdge = _e383;
    let _e384 = closestEdge;
    let _e385 = closestEdge;
    margin = ((_e384 * _e385) * 0.5f);
    let _e389 = interval_1;
    let _e390 = margin;
    interval_1 = (_e389 + (_e390 * vec2<f32>(-0.5f, 0.5f)));
    let _e397 = depth_7;
    let _e398 = interval_1;
    if (_e397 >= _e398.x) {
        {
            let _e401 = nextIndex;
            prevIndex = _e401;
            nextIndex = 1i;
            let _e403 = depth_7;
            let _e404 = interval_1;
            let _e407 = margin;
            alpha = clamp(((_e403 - _e404.x) / _e407), 0f, 1f);
        }
    }
    let _e412 = alpha;
    if (jitter <= _e412) {
        let _e414 = nextIndex;
        local_4 = _e414;
    } else {
        let _e415 = prevIndex;
        local_4 = _e415;
    }
    let _e417 = local_4;
    return _e417;
}

fn interleavedGradientNoise(coord: vec2<f32>) -> f32 {
    var magic: vec3<f32> = vec3<f32>(0.06711056f, 0.00583715f, 52.982918f);

    let _e288 = magic;
    let _e290 = magic;
    return fract((_e288.z * fract(dot(coord, _e290.xy))));
}

fn vogelDisk(index: i32, sampleCount: i32, phi: f32) -> vec2<f32> {
    var goldenAngle: f32 = 2.3999631f;
    var r_2: f32;
    var theta: f32;

    r_2 = (sqrt((f32(index) + 0.5f)) / sqrt(f32(sampleCount)));
    let _e296 = goldenAngle;
    theta = ((f32(index) * _e296) + phi);
    let _e300 = r_2;
    let _e301 = theta;
    let _e303 = theta;
    return (_e300 * vec2<f32>(cos(_e301), sin(_e303)));
}

fn ClampCosine(mu: f32) -> f32 {
    return clamp(mu, -1f, 1f);
}

fn ClampDistance(d: f32) -> f32 {
    return max(d, 0f);
}

fn ClampRadius(atmosphere: AtmosphereParameters, r_3: f32) -> f32 {
    return clamp(r_3, atmosphere.bottom_radius, atmosphere.top_radius);
}

fn SafeSqrt(a_7: f32) -> f32 {
    return sqrt(max(a_7, 0f));
}

fn DistanceToTopAtmosphereBoundary(atmosphere_1: AtmosphereParameters, r_4: f32, mu_1: f32) -> f32 {
    var discriminant_6: f32;

    discriminant_6 = (((r_4 * r_4) * ((mu_1 * mu_1) - 1f)) + (atmosphere_1.top_radius * atmosphere_1.top_radius));
    let _e317 = discriminant_6;
    let _e318 = SafeSqrt(_e317);
    let _e320 = ClampDistance(((-(r_4) * mu_1) + _e318));
    return _e320;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_5: f32, mu_2: f32) -> f32 {
    var discriminant_7: f32;

    discriminant_7 = (((r_5 * r_5) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e317 = discriminant_7;
    let _e318 = SafeSqrt(_e317);
    let _e320 = ClampDistance(((-(r_5) * mu_2) - _e318));
    return _e320;
}

fn RayIntersectsGround(atmosphere_3: AtmosphereParameters, r_6: f32, mu_3: f32) -> bool {
    return ((mu_3 < 0f) && ((((r_6 * r_6) * ((mu_3 * mu_3) - 1f)) + (atmosphere_3.bottom_radius * atmosphere_3.bottom_radius)) >= 0f));
}

fn GetTextureCoordFromUnitRange(x_41: f32, texture_size: i32) -> f32 {
    return ((0.5f / f32(texture_size)) + (x_41 * (1f - (1f / f32(texture_size)))));
}

fn GetTransmittanceTextureUvFromRMu(atmosphere_4: AtmosphereParameters, r_7: f32, mu_4: f32) -> vec2<f32> {
    var H: f32;
    var rho: f32;
    var d_1: f32;
    var d_min: f32;
    var d_max: f32;
    var x_mu: f32;
    var x_r: f32;

    H = sqrt(((atmosphere_4.top_radius * atmosphere_4.top_radius) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    let _e319 = SafeSqrt(((r_7 * r_7) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e319;
    let _e321 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_7, mu_4);
    d_1 = _e321;
    d_min = (atmosphere_4.top_radius - r_7);
    let _e326 = rho;
    let _e327 = H;
    d_max = (_e326 + _e327);
    let _e330 = d_1;
    let _e331 = d_min;
    let _e333 = d_max;
    let _e334 = d_min;
    x_mu = ((_e330 - _e331) / (_e333 - _e334));
    let _e338 = rho;
    let _e339 = H;
    x_r = (_e338 / _e339);
    let _e342 = x_mu;
    let _e344 = GetTextureCoordFromUnitRange(_e342, 256i);
    let _e345 = x_r;
    let _e347 = GetTextureCoordFromUnitRange(_e345, 64i);
    return vec2<f32>(_e344, _e347);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_8: f32, mu_5: f32) -> vec3<f32> {
    var uv_5: vec2<f32>;

    let _e307 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_8, mu_5);
    uv_5 = _e307;
    let _e309 = uv_5;
    let _e310 = textureSample(transmittance_texture_t, transmittance_texture_s, _e309);
    return vec3<f32>(_e310.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_9: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e318 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_9) * mu_6) * d_2)) + (r_9 * r_9))));
    r_d = _e318;
    let _e322 = r_d;
    let _e324 = ClampCosine((((r_9 * mu_6) + d_2) / _e322));
    mu_d = _e324;
    if ray_r_mu_intersects_ground {
        {
            let _e326 = r_d;
            let _e327 = mu_d;
            let _e329 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e326, -(_e327));
            let _e331 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_9, -(mu_6));
            return min((_e329 / _e331), vec3(1f));
        }
    } else {
        {
            let _e336 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_9, mu_6);
            let _e337 = r_d;
            let _e338 = mu_d;
            let _e339 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e337, _e338);
            return min((_e336 / _e339), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_10: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_10);
    let _e311 = sin_theta_h;
    let _e312 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e311 * _e312)), 0f)));
    let _e320 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_10, mu_s);
    let _e321 = sin_theta_h;
    let _e326 = sin_theta_h;
    let _e330 = cos_theta_h;
    return (_e320 * smoothstep(((-(_e321) * atmosphere_7.sun_angular_radius) / rad), ((_e326 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e330)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e311 = k;
    return (_e311 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g_1: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g_1 * g_1))) / (2f + (g_1 * g_1)));
    let _e320 = k_1;
    let _e333 = pt_pow(((1f + (g_1 * g_1)) - ((2f * g_1) * nu_1)), 1.5f);
    return ((_e320 * (1f + (nu_1 * nu_1))) / _e333);
}

fn GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_8: AtmosphereParameters, r_11: f32, mu_7: f32, mu_s_1: f32, nu_2: f32, ray_r_mu_intersects_ground_1: bool) -> vec4<f32> {
    var H_1: f32;
    var rho_1: f32;
    var u_r: f32;
    var r_mu: f32;
    var discriminant_8: f32;
    var u_mu: f32;
    var d_3: f32;
    var d_min_1: f32;
    var d_max_1: f32;
    var local_5: f32;
    var d_4: f32;
    var d_min_2: f32;
    var d_max_2: f32;
    var d_5: f32;
    var d_min_3: f32;
    var d_max_3: f32;
    var a_8: f32;
    var D: f32;
    var A: f32;
    var u_mu_s: f32;
    var u_nu: f32;

    H_1 = sqrt(((atmosphere_8.top_radius * atmosphere_8.top_radius) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    let _e322 = SafeSqrt(((r_11 * r_11) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e322;
    let _e324 = rho_1;
    let _e325 = H_1;
    let _e328 = GetTextureCoordFromUnitRange((_e324 / _e325), 32i);
    u_r = _e328;
    r_mu = (r_11 * mu_7);
    let _e332 = r_mu;
    let _e333 = r_mu;
    discriminant_8 = (((_e332 * _e333) - (r_11 * r_11)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e343 = r_mu;
            let _e345 = discriminant_8;
            let _e346 = SafeSqrt(_e345);
            d_3 = (-(_e343) - _e346);
            d_min_1 = (r_11 - atmosphere_8.bottom_radius);
            let _e352 = rho_1;
            d_max_1 = _e352;
            let _e356 = d_max_1;
            let _e357 = d_min_1;
            if (_e356 == _e357) {
                local_5 = 0f;
            } else {
                let _e360 = d_3;
                let _e361 = d_min_1;
                let _e363 = d_max_1;
                let _e364 = d_min_1;
                local_5 = ((_e360 - _e361) / (_e363 - _e364));
            }
            let _e368 = local_5;
            let _e372 = GetTextureCoordFromUnitRange(_e368, 64i);
            u_mu = (0.5f - (0.5f * _e372));
        }
    } else {
        {
            let _e375 = r_mu;
            let _e377 = discriminant_8;
            let _e378 = H_1;
            let _e379 = H_1;
            let _e382 = SafeSqrt((_e377 + (_e378 * _e379)));
            d_4 = (-(_e375) + _e382);
            d_min_2 = (atmosphere_8.top_radius - r_11);
            let _e388 = rho_1;
            let _e389 = H_1;
            d_max_2 = (_e388 + _e389);
            let _e394 = d_4;
            let _e395 = d_min_2;
            let _e397 = d_max_2;
            let _e398 = d_min_2;
            let _e404 = GetTextureCoordFromUnitRange(((_e394 - _e395) / (_e397 - _e398)), 64i);
            u_mu = (0.5f + (0.5f * _e404));
        }
    }
    let _e408 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e408;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e414 = H_1;
    d_max_3 = _e414;
    let _e416 = d_5;
    let _e417 = d_min_3;
    let _e419 = d_max_3;
    let _e420 = d_min_3;
    a_8 = ((_e416 - _e417) / (_e419 - _e420));
    let _e426 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e426;
    let _e428 = D;
    let _e429 = d_min_3;
    let _e431 = d_max_3;
    let _e432 = d_min_3;
    A = ((_e428 - _e429) / (_e431 - _e432));
    let _e437 = a_8;
    let _e438 = A;
    let _e444 = a_8;
    let _e448 = GetTextureCoordFromUnitRange((max((1f - (_e437 / _e438)), 0f) / (1f + _e444)), 32i);
    u_mu_s = _e448;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e455 = u_nu;
    let _e456 = u_mu_s;
    let _e457 = u_mu;
    let _e458 = u_r;
    return vec4<f32>(_e455, _e456, _e457, _e458);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_12: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_12 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e317 = x_mu_s;
    let _e319 = GetTextureCoordFromUnitRange(_e317, 64i);
    let _e320 = x_r_1;
    let _e322 = GetTextureCoordFromUnitRange(_e320, 16i);
    return vec2<f32>(_e319, _e322);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_13: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_6: vec2<f32>;

    let _e307 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_13, mu_s_3);
    uv_6 = _e307;
    let _e309 = uv_6;
    let _e310 = textureSample(irradiance_texture_t, irradiance_texture_s, _e309);
    return vec3<f32>(_e310.xyz);
}

fn GetExtrapolatedSingleMieScattering(atmosphere_11: AtmosphereParameters, scattering: vec4<f32>) -> vec3<f32> {
    if (scattering.x < 0.00001f) {
        {
            return vec3(0f);
        }
    }
    return ((((scattering.xyz * scattering.w) / vec3(scattering.x)) * (atmosphere_11.rayleigh_scattering.x / atmosphere_11.mie_scattering.x)) * (atmosphere_11.mie_scattering / atmosphere_11.rayleigh_scattering));
}

fn GetCombinedScattering(atmosphere_12: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, r_14: f32, mu_8: f32, mu_s_4: f32, nu_3: f32, ray_r_mu_intersects_ground_2: bool, single_mie_scattering: ptr<function, vec3<f32>>) -> vec3<f32> {
    var uvwz: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;
    var combined_scattering: vec4<f32>;
    var scattering_1: vec3<f32>;

    let _e313 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_12, r_14, mu_8, mu_s_4, nu_3, ray_r_mu_intersects_ground_2);
    uvwz = _e313;
    let _e315 = uvwz;
    tex_coord_x = (_e315.x * 7f);
    let _e323 = tex_coord_x;
    tex_x = floor(_e323);
    let _e326 = tex_coord_x;
    let _e327 = tex_x;
    lerp = (_e326 - _e327);
    let _e330 = tex_x;
    let _e331 = uvwz;
    let _e337 = uvwz;
    let _e339 = uvwz;
    uvw0_ = vec3<f32>(((_e330 + _e331.y) / 8f), _e337.z, _e339.w);
    let _e343 = tex_x;
    let _e346 = uvwz;
    let _e352 = uvwz;
    let _e354 = uvwz;
    uvw1_ = vec3<f32>((((_e343 + 1f) + _e346.y) / 8f), _e352.z, _e354.w);
    let _e358 = uvw0_;
    let _e359 = textureSample(scattering_texture_t, scattering_texture_s, _e358);
    let _e361 = lerp;
    let _e364 = uvw1_;
    let _e365 = textureSample(scattering_texture_t, scattering_texture_s, _e364);
    let _e366 = lerp;
    combined_scattering = ((_e359 * (1f - _e361)) + (_e365 * _e366));
    let _e370 = combined_scattering;
    scattering_1 = vec3<f32>(_e370.xyz);
    let _e374 = combined_scattering;
    let _e375 = GetExtrapolatedSingleMieScattering(atmosphere_12, _e374);
    (*single_mie_scattering) = _e375;
    let _e376 = scattering_1;
    return _e376;
}

fn GetScattering(atmosphere_13: AtmosphereParameters, scattering_texture_t_1: texture_3d<f32>, scattering_texture_s_1: sampler, r_15: f32, mu_9: f32, mu_s_5: f32, nu_4: f32, ray_r_mu_intersects_ground_3: bool) -> vec3<f32> {
    var uvwz_1: vec4<f32>;
    var tex_coord_x_1: f32;
    var tex_x_1: f32;
    var lerp_1: f32;
    var uvw0_1: vec3<f32>;
    var uvw1_1: vec3<f32>;
    var scattering_2: vec3<f32>;

    let _e310 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_13, r_15, mu_9, mu_s_5, nu_4, ray_r_mu_intersects_ground_3);
    uvwz_1 = _e310;
    let _e312 = uvwz_1;
    tex_coord_x_1 = (_e312.x * 7f);
    let _e320 = tex_coord_x_1;
    tex_x_1 = floor(_e320);
    let _e323 = tex_coord_x_1;
    let _e324 = tex_x_1;
    lerp_1 = (_e323 - _e324);
    let _e327 = tex_x_1;
    let _e328 = uvwz_1;
    let _e334 = uvwz_1;
    let _e336 = uvwz_1;
    uvw0_1 = vec3<f32>(((_e327 + _e328.y) / 8f), _e334.z, _e336.w);
    let _e340 = tex_x_1;
    let _e343 = uvwz_1;
    let _e349 = uvwz_1;
    let _e351 = uvwz_1;
    uvw1_1 = vec3<f32>((((_e340 + 1f) + _e343.y) / 8f), _e349.z, _e351.w);
    let _e355 = uvw0_1;
    let _e356 = textureSample(scattering_texture_t_1, scattering_texture_s_1, _e355);
    let _e358 = lerp_1;
    let _e361 = uvw1_1;
    let _e362 = textureSample(scattering_texture_t_1, scattering_texture_s_1, _e361);
    let _e363 = lerp_1;
    scattering_2 = vec3<f32>(((_e356 * (1f - _e358)) + (_e362 * _e363)).xyz);
    let _e369 = scattering_2;
    return _e369;
}

fn GetSkyRadiance(atmosphere_14: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, scattering_texture_t_2: texture_3d<f32>, scattering_texture_s_2: sampler, single_mie_scattering_texture_t_1: texture_3d<f32>, single_mie_scattering_texture_s_1: sampler, camera: vec3<f32>, view_ray: vec3<f32>, shadow_length: f32, sun_direction: vec3<f32>, transmittance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_1: vec3<f32>;
    var r_16: f32;
    var rmu: f32;
    var distance_to_top_atmosphere_boundary: f32;
    var mu_10: f32;
    var mu_s_6: f32;
    var nu_5: f32;
    var ray_r_mu_intersects_ground_4: bool = false;
    var local_6: vec3<f32>;
    var single_mie_scattering_1: vec3<f32>;
    var scattering_3: vec3<f32>;
    var d_6: f32;
    var r_p: f32;
    var mu_p: f32;
    var mu_s_p: f32;
    var shadow_transmittance: vec3<f32>;
    var higher_order_scattering: vec3<f32>;
    var single_scattering: vec3<f32>;

    camera_1 = camera;
    let _e315 = camera_1;
    r_16 = length(_e315);
    let _e318 = camera_1;
    rmu = dot(_e318, view_ray);
    let _e321 = rmu;
    let _e323 = rmu;
    let _e324 = rmu;
    let _e326 = r_16;
    let _e327 = r_16;
    let _e334 = SafeSqrt((((_e323 * _e324) - (_e326 * _e327)) + (atmosphere_14.top_radius * atmosphere_14.top_radius)));
    distance_to_top_atmosphere_boundary = (-(_e321) - _e334);
    let _e337 = distance_to_top_atmosphere_boundary;
    if (_e337 > 0f) {
        {
            let _e342 = camera_1;
            let _e343 = distance_to_top_atmosphere_boundary;
            camera_1 = (_e342 + (view_ray * _e343));
            r_16 = atmosphere_14.top_radius;
            let _e347 = rmu;
            let _e348 = distance_to_top_atmosphere_boundary;
            rmu = (_e347 + _e348);
        }
    } else {
        let _e350 = r_16;
        if (_e350 > atmosphere_14.top_radius) {
            {
                (*transmittance) = vec3(1f);
                return vec3(0f);
            }
        }
    }
    let _e359 = rmu;
    let _e360 = r_16;
    mu_10 = (_e359 / _e360);
    let _e363 = camera_1;
    let _e365 = r_16;
    mu_s_6 = (dot(_e363, sun_direction) / _e365);
    nu_5 = dot(view_ray, sun_direction);
    let _e372 = ray_r_mu_intersects_ground_4;
    if _e372 {
        local_6 = vec3(0f);
    } else {
        let _e375 = r_16;
        let _e376 = mu_10;
        let _e377 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_14, transmittance_texture_t_3, transmittance_texture_s_3, _e375, _e376);
        local_6 = _e377;
    }
    let _e379 = local_6;
    (*transmittance) = _e379;
    if (shadow_length == 0f) {
        {
            let _e386 = r_16;
            let _e387 = mu_10;
            let _e388 = mu_s_6;
            let _e389 = nu_5;
            let _e390 = ray_r_mu_intersects_ground_4;
            let _e393 = GetCombinedScattering(atmosphere_14, scattering_texture_t_2, scattering_texture_s_2, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, _e386, _e387, _e388, _e389, _e390, (&single_mie_scattering_1));
            scattering_3 = _e393;
        }
    } else {
        {
            d_6 = shadow_length;
            let _e395 = d_6;
            let _e396 = d_6;
            let _e399 = r_16;
            let _e401 = mu_10;
            let _e403 = d_6;
            let _e406 = r_16;
            let _e407 = r_16;
            let _e411 = ClampRadius(atmosphere_14, sqrt((((_e395 * _e396) + (((2f * _e399) * _e401) * _e403)) + (_e406 * _e407))));
            r_p = _e411;
            let _e413 = r_16;
            let _e414 = mu_10;
            let _e416 = d_6;
            let _e418 = r_p;
            mu_p = (((_e413 * _e414) + _e416) / _e418);
            let _e421 = r_16;
            let _e422 = mu_s_6;
            let _e424 = d_6;
            let _e425 = nu_5;
            let _e428 = r_p;
            mu_s_p = (((_e421 * _e422) + (_e424 * _e425)) / _e428);
            let _e431 = r_p;
            let _e432 = mu_p;
            let _e433 = mu_s_p;
            let _e434 = nu_5;
            let _e435 = ray_r_mu_intersects_ground_4;
            let _e438 = GetCombinedScattering(atmosphere_14, scattering_texture_t_2, scattering_texture_s_2, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, _e431, _e432, _e433, _e434, _e435, (&single_mie_scattering_1));
            scattering_3 = _e438;
            let _e439 = r_16;
            let _e440 = mu_10;
            let _e441 = ray_r_mu_intersects_ground_4;
            let _e442 = GetTransmittance(atmosphere_14, transmittance_texture_t_3, transmittance_texture_s_3, _e439, _e440, shadow_length, _e441);
            shadow_transmittance = _e442;
            let _e444 = r_p;
            let _e445 = mu_p;
            let _e446 = mu_s_p;
            let _e447 = nu_5;
            let _e448 = ray_r_mu_intersects_ground_4;
            let _e449 = GetScattering(atmosphere_14, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e444, _e445, _e446, _e447, _e448);
            higher_order_scattering = _e449;
            let _e451 = scattering_3;
            let _e452 = higher_order_scattering;
            single_scattering = (_e451 - _e452);
            let _e455 = single_scattering;
            let _e456 = shadow_transmittance;
            let _e458 = higher_order_scattering;
            scattering_3 = ((_e455 * _e456) + _e458);
            let _e460 = single_mie_scattering_1;
            let _e461 = shadow_transmittance;
            single_mie_scattering_1 = (_e460 * _e461);
        }
    }
    let _e463 = scattering_3;
    let _e464 = nu_5;
    let _e465 = RayleighPhaseFunction(_e464);
    let _e467 = single_mie_scattering_1;
    let _e469 = nu_5;
    let _e470 = MiePhaseFunction(atmosphere_14.mie_phase_function_g, _e469);
    return ((_e463 * _e465) + (_e467 * _e470));
}

fn ClosestPointOnRay(camera_2: vec3<f32>, point: vec3<f32>) -> vec3<f32> {
    var ray: vec3<f32>;
    var t: f32;

    ray = (point - camera_2);
    let _e306 = ray;
    let _e309 = ray;
    let _e310 = ray;
    t = clamp((-(dot(camera_2, _e306)) / dot(_e309, _e310)), 0f, 1f);
    let _e317 = t;
    let _e318 = ray;
    return (camera_2 + (_e317 * _e318));
}

fn RaySphereIntersections(camera_3: vec3<f32>, direction_12: vec3<f32>, radius_12: f32) -> vec2<f32> {
    var b_16: f32;
    var c_8: f32;
    var discriminant_9: f32;
    var Q_2: f32;

    b_16 = (2f * dot(direction_12, camera_3));
    c_8 = (dot(camera_3, camera_3) - (radius_12 * radius_12));
    let _e313 = b_16;
    let _e314 = b_16;
    let _e317 = c_8;
    discriminant_9 = ((_e313 * _e314) - (4f * _e317));
    let _e321 = discriminant_9;
    Q_2 = sqrt(_e321);
    let _e324 = b_16;
    let _e326 = Q_2;
    let _e328 = b_16;
    let _e330 = Q_2;
    return (vec2<f32>((-(_e324) - _e326), (-(_e328) + _e330)) * 0.5f);
}

fn ClipAtBottomAtmosphere(atmosphere_15: AtmosphereParameters, view_ray_1: vec3<f32>, camera_4: ptr<function, vec3<f32>>, point_1: ptr<function, vec3<f32>>) -> bool {
    var eps: f32 = 0f;
    var bottom_radius: f32;
    var r_camera: f32;
    var r_point: f32;
    var camera_below: bool;
    var point_below: bool;
    var t_1: vec2<f32>;
    var local_7: f32;
    var intersection: vec3<f32>;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;

    let _e309 = eps;
    bottom_radius = (atmosphere_15.bottom_radius + _e309);
    let _e312 = (*camera_4);
    r_camera = length(_e312);
    let _e315 = (*point_1);
    r_point = length(_e315);
    let _e318 = r_camera;
    let _e319 = bottom_radius;
    camera_below = (_e318 < _e319);
    let _e322 = r_point;
    let _e323 = bottom_radius;
    point_below = (_e322 < _e323);
    let _e326 = (*camera_4);
    let _e327 = bottom_radius;
    let _e328 = RaySphereIntersections(_e326, view_ray_1, _e327);
    t_1 = _e328;
    let _e330 = (*camera_4);
    let _e331 = camera_below;
    if _e331 {
        let _e332 = t_1;
        local_7 = _e332.y;
    } else {
        let _e334 = t_1;
        local_7 = _e334.x;
    }
    let _e337 = local_7;
    intersection = (_e330 + (view_ray_1 * _e337));
    let _e341 = camera_below;
    if _e341 {
        let _e342 = intersection;
        local_8 = _e342;
    } else {
        let _e343 = (*camera_4);
        local_8 = _e343;
    }
    let _e345 = local_8;
    (*camera_4) = _e345;
    let _e346 = point_below;
    if _e346 {
        let _e347 = intersection;
        local_9 = _e347;
    } else {
        let _e348 = (*point_1);
        local_9 = _e348;
    }
    let _e350 = local_9;
    (*point_1) = _e350;
    let _e351 = camera_below;
    let _e352 = point_below;
    return (_e351 && _e352);
}

fn GetSkyRadianceToPoint(atmosphere_16: AtmosphereParameters, transmittance_texture_t_4: texture_2d<f32>, transmittance_texture_s_4: sampler, scattering_texture_t_3: texture_3d<f32>, scattering_texture_s_3: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, camera_5: vec3<f32>, point_2: vec3<f32>, shadow_length_1: f32, sun_direction_1: vec3<f32>, transmittance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_6: vec3<f32>;
    var point_3: vec3<f32>;
    var view_ray_2: vec3<f32>;
    var r_17: f32;
    var rmu_1: f32;
    var distance_to_top_atmosphere_boundary_1: f32;
    var mu_11: f32;
    var mu_s_7: f32;
    var nu_6: f32;
    var d_7: f32;
    var ray_r_mu_intersects_ground_5: bool;
    var mu_horizon: f32;
    var eps_1: f32 = 0.004f;
    var single_mie_scattering_2: vec3<f32>;
    var scattering_4: vec3<f32>;
    var r_p_1: f32;
    var mu_p_1: f32;
    var mu_s_p_1: f32;
    var single_mie_scattering_p: vec3<f32>;
    var scattering_p: vec3<f32>;
    var shadow_transmittance_1: vec3<f32>;
    var higher_order_scattering_1: vec3<f32>;
    var single_scattering_1: vec3<f32>;
    var higher_order_scattering_p: vec3<f32>;
    var single_scattering_p: vec3<f32>;

    camera_6 = camera_5;
    point_3 = point_2;
    let _e316 = camera_6;
    let _e317 = point_3;
    let _e318 = ClosestPointOnRay(_e316, _e317);
    if (length(_e318) > atmosphere_16.top_radius) {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e326 = point_3;
    let _e327 = camera_6;
    view_ray_2 = normalize((_e326 - _e327));
    let _e331 = view_ray_2;
    let _e336 = ClipAtBottomAtmosphere(atmosphere_16, _e331, (&camera_6), (&point_3));
    if _e336 {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e341 = camera_6;
    r_17 = length(_e341);
    let _e344 = camera_6;
    let _e345 = view_ray_2;
    rmu_1 = dot(_e344, _e345);
    let _e348 = rmu_1;
    let _e350 = rmu_1;
    let _e351 = rmu_1;
    let _e353 = r_17;
    let _e354 = r_17;
    let _e361 = SafeSqrt((((_e350 * _e351) - (_e353 * _e354)) + (atmosphere_16.top_radius * atmosphere_16.top_radius)));
    distance_to_top_atmosphere_boundary_1 = (-(_e348) - _e361);
    let _e364 = distance_to_top_atmosphere_boundary_1;
    if (_e364 > 0f) {
        {
            let _e369 = camera_6;
            let _e370 = view_ray_2;
            let _e371 = distance_to_top_atmosphere_boundary_1;
            camera_6 = (_e369 + (_e370 * _e371));
            r_17 = atmosphere_16.top_radius;
            let _e375 = rmu_1;
            let _e376 = distance_to_top_atmosphere_boundary_1;
            rmu_1 = (_e375 + _e376);
        }
    }
    let _e378 = rmu_1;
    let _e379 = r_17;
    mu_11 = (_e378 / _e379);
    let _e382 = camera_6;
    let _e384 = r_17;
    mu_s_7 = (dot(_e382, sun_direction_1) / _e384);
    let _e387 = view_ray_2;
    nu_6 = dot(_e387, sun_direction_1);
    let _e390 = point_3;
    let _e391 = camera_6;
    d_7 = length((_e390 - _e391));
    let _e395 = r_17;
    let _e396 = mu_11;
    let _e397 = RayIntersectsGround(atmosphere_16, _e395, _e396);
    ray_r_mu_intersects_ground_5 = _e397;
    let _e399 = ray_r_mu_intersects_ground_5;
    if !(_e399) {
        {
            let _e405 = r_17;
            let _e406 = r_17;
            let _e410 = SafeSqrt((1f - ((atmosphere_16.bottom_radius * atmosphere_16.bottom_radius) / (_e405 * _e406))));
            mu_horizon = -(_e410);
            let _e415 = mu_11;
            let _e416 = mu_horizon;
            let _e417 = eps_1;
            mu_11 = max(_e415, (_e416 + _e417));
        }
    }
    let _e420 = r_17;
    let _e421 = mu_11;
    let _e422 = d_7;
    let _e423 = ray_r_mu_intersects_ground_5;
    let _e424 = GetTransmittance(atmosphere_16, transmittance_texture_t_4, transmittance_texture_s_4, _e420, _e421, _e422, _e423);
    (*transmittance_1) = _e424;
    let _e426 = r_17;
    let _e427 = mu_11;
    let _e428 = mu_s_7;
    let _e429 = nu_6;
    let _e430 = ray_r_mu_intersects_ground_5;
    let _e433 = GetCombinedScattering(atmosphere_16, scattering_texture_t_3, scattering_texture_s_3, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, _e426, _e427, _e428, _e429, _e430, (&single_mie_scattering_2));
    scattering_4 = _e433;
    let _e435 = d_7;
    d_7 = max((_e435 - shadow_length_1), 0f);
    let _e441 = d_7;
    let _e442 = d_7;
    let _e445 = r_17;
    let _e447 = mu_11;
    let _e449 = d_7;
    let _e452 = r_17;
    let _e453 = r_17;
    let _e457 = ClampRadius(atmosphere_16, sqrt((((_e441 * _e442) + (((2f * _e445) * _e447) * _e449)) + (_e452 * _e453))));
    r_p_1 = _e457;
    let _e459 = r_17;
    let _e460 = mu_11;
    let _e462 = d_7;
    let _e464 = r_p_1;
    mu_p_1 = (((_e459 * _e460) + _e462) / _e464);
    let _e467 = r_17;
    let _e468 = mu_s_7;
    let _e470 = d_7;
    let _e471 = nu_6;
    let _e474 = r_p_1;
    mu_s_p_1 = (((_e467 * _e468) + (_e470 * _e471)) / _e474);
    let _e478 = r_p_1;
    let _e479 = mu_p_1;
    let _e480 = mu_s_p_1;
    let _e481 = nu_6;
    let _e482 = ray_r_mu_intersects_ground_5;
    let _e485 = GetCombinedScattering(atmosphere_16, scattering_texture_t_3, scattering_texture_s_3, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, _e478, _e479, _e480, _e481, _e482, (&single_mie_scattering_p));
    scattering_p = _e485;
    let _e487 = (*transmittance_1);
    shadow_transmittance_1 = _e487;
    if (shadow_length_1 > 0f) {
        {
            let _e493 = r_17;
            let _e494 = mu_11;
            let _e495 = d_7;
            let _e496 = ray_r_mu_intersects_ground_5;
            let _e497 = GetTransmittance(atmosphere_16, transmittance_texture_t_4, transmittance_texture_s_4, _e493, _e494, _e495, _e496);
            shadow_transmittance_1 = _e497;
        }
    }
    let _e498 = r_17;
    let _e499 = mu_11;
    let _e500 = mu_s_7;
    let _e501 = nu_6;
    let _e502 = ray_r_mu_intersects_ground_5;
    let _e503 = GetScattering(atmosphere_16, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e498, _e499, _e500, _e501, _e502);
    higher_order_scattering_1 = _e503;
    let _e505 = scattering_4;
    let _e506 = higher_order_scattering_1;
    single_scattering_1 = (_e505 - _e506);
    let _e509 = r_p_1;
    let _e510 = mu_p_1;
    let _e511 = mu_s_p_1;
    let _e512 = nu_6;
    let _e513 = ray_r_mu_intersects_ground_5;
    let _e514 = GetScattering(atmosphere_16, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e509, _e510, _e511, _e512, _e513);
    higher_order_scattering_p = _e514;
    let _e516 = scattering_p;
    let _e517 = higher_order_scattering_p;
    single_scattering_p = (_e516 - _e517);
    let _e520 = single_scattering_1;
    let _e521 = shadow_transmittance_1;
    let _e522 = single_scattering_p;
    let _e525 = higher_order_scattering_1;
    let _e527 = (*transmittance_1);
    let _e528 = higher_order_scattering_p;
    scattering_4 = (((_e520 - (_e521 * _e522)) + _e525) - (_e527 * _e528));
    let _e531 = single_mie_scattering_2;
    let _e532 = shadow_transmittance_1;
    let _e533 = single_mie_scattering_p;
    single_mie_scattering_2 = (_e531 - (_e532 * _e533));
    let _e536 = scattering_4;
    let _e537 = single_mie_scattering_2;
    let _e543 = GetExtrapolatedSingleMieScattering(atmosphere_16, vec4<f32>(_e536.x, _e536.y, _e536.z, _e537.x));
    single_mie_scattering_2 = _e543;
    let _e544 = single_mie_scattering_2;
    let _e549 = mu_s_7;
    single_mie_scattering_2 = (_e544 * smoothstep(0f, 0.01f, _e549));
    let _e552 = scattering_4;
    let _e553 = nu_6;
    let _e554 = RayleighPhaseFunction(_e553);
    let _e556 = single_mie_scattering_2;
    let _e558 = nu_6;
    let _e559 = MiePhaseFunction(atmosphere_16.mie_phase_function_g, _e558);
    return ((_e552 * _e554) + (_e556 * _e559));
}

fn GetSunAndSkyIrradiance(atmosphere_17: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, irradiance_texture_t_1: texture_2d<f32>, irradiance_texture_s_1: sampler, point_4: vec3<f32>, normal_4: vec3<f32>, sun_direction_2: vec3<f32>, sky_irradiance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_18: f32;
    var mu_s_8: f32;

    r_18 = length(point_4);
    let _e314 = r_18;
    mu_s_8 = (dot(point_4, sun_direction_2) / _e314);
    let _e317 = r_18;
    let _e318 = mu_s_8;
    let _e319 = GetIrradiance(atmosphere_17, irradiance_texture_t_1, irradiance_texture_s_1, _e317, _e318);
    let _e322 = r_18;
    (*sky_irradiance) = ((_e319 * (1f + (dot(normal_4, point_4) / _e322))) * 0.5f);
    let _e329 = r_18;
    let _e330 = mu_s_8;
    let _e331 = GetTransmittanceToSun(atmosphere_17, transmittance_texture_t_5, transmittance_texture_s_5, _e329, _e330);
    return ((atmosphere_17.solar_irradiance * _e331) * max(dot(normal_4, sun_direction_2), 0f));
}

fn GetSunAndSkyScalarIrradiance(atmosphere_18: AtmosphereParameters, transmittance_texture_t_6: texture_2d<f32>, transmittance_texture_s_6: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, point_5: vec3<f32>, sun_direction_3: vec3<f32>, sky_irradiance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_19: f32;
    var mu_s_9: f32;

    r_19 = length(point_5);
    let _e313 = r_19;
    mu_s_9 = (dot(point_5, sun_direction_3) / _e313);
    let _e316 = r_19;
    let _e317 = mu_s_9;
    let _e318 = GetIrradiance(atmosphere_18, irradiance_texture_t_2, irradiance_texture_s_2, _e316, _e317);
    (*sky_irradiance_1) = ((_e318 * 2f) * 3.1415927f);
    let _e324 = r_19;
    let _e325 = mu_s_9;
    let _e326 = GetTransmittanceToSun(atmosphere_18, transmittance_texture_t_6, transmittance_texture_s_6, _e324, _e325);
    return (atmosphere_18.solar_irradiance * _e326);
}

fn GetSolarLuminance() -> vec3<f32> {
    let _e302 = ATMOSPHERE;
    let _e305 = ATMOSPHERE;
    let _e308 = ATMOSPHERE;
    let _e313 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return ((_e302.solar_irradiance / vec3(((3.1415927f * _e305.sun_angular_radius) * _e308.sun_angular_radius))) * _e313);
}

fn GetSkyLuminance(camera_7: vec3<f32>, view_ray_3: vec3<f32>, shadow_length_2: f32, sun_direction_4: vec3<f32>, transmittance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var view_ray_4: vec3<f32>;

    view_ray_4 = view_ray_3;
    let _e308 = ATMOSPHERE;
    let _e309 = view_ray_4;
    let _e312 = GetSkyRadiance(_e308, transmittance_texture_t_7, transmittance_texture_s_7, scattering_texture_t_4, scattering_texture_s_4, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, camera_7, _e309, shadow_length_2, sun_direction_4, transmittance_2);
    let _e313 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e312 * _e313);
}

fn GetSkyLuminanceToPoint(camera_8: vec3<f32>, point_6: vec3<f32>, shadow_length_3: f32, sun_direction_5: vec3<f32>, transmittance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    let _e307 = ATMOSPHERE;
    let _e310 = GetSkyRadianceToPoint(_e307, transmittance_texture_t_7, transmittance_texture_s_7, scattering_texture_t_4, scattering_texture_s_4, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, camera_8, point_6, shadow_length_3, sun_direction_5, transmittance_3);
    let _e311 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e310 * _e311);
}

fn GetSunAndSkyIlluminance(p: vec3<f32>, normal_5: vec3<f32>, sun_direction_6: vec3<f32>, sky_irradiance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance: vec3<f32>;

    let _e306 = ATMOSPHERE;
    let _e309 = GetSunAndSkyIrradiance(_e306, transmittance_texture_t_7, transmittance_texture_s_7, irradiance_texture_t_3, irradiance_texture_s_3, p, normal_5, sun_direction_6, sky_irradiance_2);
    sun_irradiance = _e309;
    let _e311 = (*sky_irradiance_2);
    let _e312 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_2) = (_e311 * _e312);
    let _e314 = sun_irradiance;
    let _e315 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e314 * _e315);
}

fn GetSunAndSkyScalarIlluminance(p_1: vec3<f32>, sun_direction_7: vec3<f32>, sky_irradiance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance_1: vec3<f32>;

    let _e305 = ATMOSPHERE;
    let _e308 = GetSunAndSkyScalarIrradiance(_e305, transmittance_texture_t_7, transmittance_texture_s_7, irradiance_texture_t_3, irradiance_texture_s_3, p_1, sun_direction_7, sky_irradiance_3);
    sun_irradiance_1 = _e308;
    let _e310 = (*sky_irradiance_3);
    let _e311 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_3) = (_e310 * _e311);
    let _e313 = sun_irradiance_1;
    let _e314 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e313 * _e314);
}

fn getSTBN() -> f32 {
    var size: vec3<i32>;
    var scale: vec3<f32>;

    let _e303 = textureDimensions(stbnTexture_t, 0i);
    size = vec3<i32>(_e303);
    let _e307 = size;
    scale = (vec3(1f) / vec3<f32>(_e307));
    let _e313 = gl_FragCoord_1;
    let _e314 = _e313.xy;
    let _e315 = global.frame;
    let _e316 = size;
    let _e323 = scale;
    let _e325 = textureSample(stbnTexture_t, stbnTexture_s, (vec3<f32>(_e314.x, _e314.y, f32((_e315 % _e316.z))) * _e323));
    return _e325.x;
}

fn getSphericalUv(position: vec3<f32>) -> vec2<f32> {
    var st: vec2<f32>;
    var phi_1: f32;
    var theta_1: f32;

    st = normalize(position.yx);
    let _e307 = st;
    let _e309 = st;
    phi_1 = atan2(_e307.x, _e309.y);
    theta_1 = asin(normalize(position).z);
    let _e317 = phi_1;
    let _e322 = theta_1;
    return vec2<f32>(((_e317 * 0.15915494f) + 0.5f), ((_e322 * 0.31830987f) + 0.5f));
}

fn getCubeSphereUv(position_1: vec3<f32>) -> vec2<f32> {
    var n: vec3<f32>;
    var f: vec3<f32>;
    var c_9: vec3<f32>;
    var m_2: vec2<f32>;
    var local_10: vec2<f32>;
    var local_11: vec2<f32>;
    var local_12: vec2<f32>;
    var m2_: vec2<f32>;
    var q: f32;
    var q2_: f32;
    var uv_7: vec2<f32>;
    var local_13: f32;

    n = normalize(position_1);
    let _e306 = n;
    f = abs(_e306);
    let _e309 = n;
    let _e310 = f;
    let _e312 = f;
    let _e314 = f;
    c_9 = (_e309 / vec3(max(_e310.x, max(_e312.y, _e314.z))));
    let _e322 = f;
    let _e324 = f;
    if all((_e322.yy > _e324.xz)) {
        {
            let _e328 = c_9;
            if (_e328.y > 0f) {
                let _e332 = n;
                let _e335 = n;
                local_10 = vec2<f32>(-(_e332.x), _e335.z);
            } else {
                let _e338 = n;
                local_10 = _e338.xz;
            }
            let _e341 = local_10;
            m_2 = _e341;
        }
    } else {
        let _e342 = f;
        let _e344 = f;
        if all((_e342.xx > _e344.yz)) {
            {
                let _e348 = c_9;
                if (_e348.x > 0f) {
                    let _e352 = n;
                    local_11 = _e352.yz;
                } else {
                    let _e354 = n;
                    let _e357 = n;
                    local_11 = vec2<f32>(-(_e354.y), _e357.z);
                }
                let _e361 = local_11;
                m_2 = _e361;
            }
        } else {
            {
                let _e362 = c_9;
                if (_e362.z > 0f) {
                    let _e366 = n;
                    local_12 = _e366.xy;
                } else {
                    let _e368 = n;
                    let _e370 = n;
                    local_12 = vec2<f32>(_e368.x, -(_e370.y));
                }
                let _e375 = local_12;
                m_2 = _e375;
            }
        }
    }
    let _e376 = m_2;
    let _e377 = m_2;
    m2_ = (_e376 * _e377);
    let _e380 = m2_;
    q = (dot(_e380.xy, vec2<f32>(-2f, 2f)) - 3f);
    let _e390 = q;
    let _e391 = q;
    q2_ = (_e390 * _e391);
    let _e397 = m2_;
    let _e400 = m2_;
    let _e406 = m2_;
    let _e409 = q2_;
    let _e415 = m_2;
    if (_e415.x > 0f) {
        local_13 = 1f;
    } else {
        local_13 = -1f;
    }
    let _e423 = local_13;
    uv_7.x = (sqrt((((1.5f + _e397.x) - _e400.y) - (0.5f * sqrt(((-24f * _e406.x) + _e409))))) * _e423);
    let _e428 = uv_7;
    let _e430 = uv_7;
    let _e436 = m_2;
    uv_7.y = (sqrt((6f / (3f - (_e428.x * _e430.x)))) * _e436.y);
    let _e439 = uv_7;
    return ((_e439 * 0.5f) + vec2(0.5f));
}

fn getGlobeUv(position_2: vec3<f32>) -> vec2<f32> {
    let _e304 = getCubeSphereUv(position_2);
    return _e304;
}

fn getMipLevel(uv_8: vec2<f32>) -> f32 {
    var mipLevelScale: f32 = 0.1f;
    var coord_1: vec2<f32>;
    var ddx: vec2<f32>;
    var ddy: vec2<f32>;
    var deltaMaxSqr: f32;

    let _e306 = global.resolution;
    coord_1 = (uv_8 * _e306);
    let _e309 = coord_1;
    let _e310 = dpdx(_e309);
    ddx = _e310;
    let _e312 = coord_1;
    let _e313 = dpdy(_e312);
    ddy = _e313;
    let _e315 = ddx;
    let _e316 = ddx;
    let _e318 = ddy;
    let _e319 = ddy;
    let _e322 = mipLevelScale;
    deltaMaxSqr = (max(dot(_e315, _e316), dot(_e318, _e319)) * _e322);
    let _e328 = deltaMaxSqr;
    return max(0f, (0.5f * log2(max(1f, _e328))));
}

fn insideLayerIntervals(height: f32) -> bool {
    var gt: vec3<bool>;
    var lt: vec3<bool>;

    let _e305 = global.minIntervalHeights;
    gt = (vec3(height) > _e305);
    let _e309 = global.maxIntervalHeights;
    lt = (vec3(height) < _e309);
    let _e312 = gt;
    let _e314 = lt;
    let _e317 = gt;
    let _e319 = lt;
    let _e322 = gt;
    let _e324 = lt;
    return any(vec3<bool>((_e312.x && _e314.x), (_e317.y && _e319.y), (_e322.z && _e324.z)));
}

fn shapeAlteringFunction(heightFraction: vec4<f32>, bias: vec4<f32>) -> vec4<f32> {
    var biased: vec4<f32>;
    var x_42: vec4<f32>;

    let _e305 = pt_pow_3(heightFraction, bias);
    biased = _e305;
    let _e307 = biased;
    x_42 = clamp(((_e307 * 2f) - vec4(1f)), vec4(-1f), vec4(1f));
    let _e321 = x_42;
    let _e322 = x_42;
    return (vec4(1f) - (_e321 * _e322));
}

fn sampleWeather(uv_9: vec2<f32>, height_1: f32, mipLevel: f32) -> WeatherSample {
    var weather: WeatherSample;
    var localWeather: vec4<f32>;
    var heightScale: vec4<f32>;
    var factor: vec4<f32>;

    let _e309 = global.minLayerHeights;
    let _e310 = global.maxLayerHeights;
    let _e311 = remapClamped_7(vec4(height_1), _e309, _e310);
    weather.heightFraction = _e311;
    let _e312 = global.localWeatherRepeat;
    let _e314 = global.localWeatherOffset;
    let _e316 = textureSampleLevel(localWeatherTexture_t, localWeatherTexture_s, ((uv_9 * _e312) + _e314), mipLevel);
    let _e318 = global.weatherExponents;
    let _e319 = pt_pow_3(_e316.xyzw, _e318);
    localWeather = _e319;
    let _e321 = weather;
    let _e323 = global.shapeAlteringBiases;
    let _e324 = shapeAlteringFunction(_e321.heightFraction, _e323);
    heightScale = _e324;
    let _e327 = global.coverage;
    let _e328 = heightScale;
    factor = (vec4(1f) - (_e327 * _e328));
    let _e334 = localWeather;
    let _e337 = global.coverageFilterWidths;
    let _e339 = factor;
    let _e340 = factor;
    let _e341 = global.coverageFilterWidths;
    let _e343 = remapClamped_7(mix(_e334, vec4(1f), _e337), _e339, (_e340 + _e341));
    weather.density = _e343;
    let _e344 = weather;
    return _e344;
}

fn getLayerDensity(heightFraction_1: vec4<f32>) -> vec4<f32> {
    let _e304 = densityProfile;
    let _e306 = densityProfile;
    let _e311 = densityProfile;
    let _e315 = densityProfile;
    return (((_e304.expTerms * exp((_e306.exponents * heightFraction_1))) + (_e311.linearTerms * heightFraction_1)) + _e315.constantTerms);
}

fn sampleMedia(weather_1: WeatherSample, position_3: vec3<f32>, uv_10: vec2<f32>, mipLevel_1: f32, jitter_1: f32, sampleCount_1: ptr<function, vec3<i32>>) -> MediaSample {
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
    let _e313 = global.localWeatherOffset;
    localWeatherSpeed = length(_e313);
    let _e316 = surfaceNormal;
    let _e318 = localWeatherSpeed;
    evolution = ((-(_e316) * _e318) * 20000f);
    let _e326 = global.localWeatherRepeat;
    let _e328 = global.turbulenceRepeat;
    turbulenceUv = ((uv_10 * _e326) * _e328);
    let _e331 = global.turbulenceDisplacement;
    let _e332 = turbulenceUv;
    let _e333 = textureSample(turbulenceTexture_t, turbulenceTexture_s, _e332);
    let _e341 = density;
    let _e347 = remapClamped_7(weather_1.heightFraction, vec4(0.3f), vec4(0f));
    turbulence = ((_e331 * ((_e333.xyz * 2f) - vec3(1f))) * dot(_e341, _e347));
    let _e350 = evolution;
    let _e352 = turbulence;
    let _e354 = global.shapeRepeat;
    let _e356 = global.shapeOffset;
    shapePosition = ((((position_3 + _e350) + _e352) * _e354) + _e356);
    let _e359 = shapePosition;
    let _e360 = textureSample(shapeTexture_t, shapeTexture_s, _e359);
    shape = _e360.x;
    let _e363 = density;
    let _e365 = shape;
    let _e368 = global.shapeAmounts;
    let _e372 = remapClamped_7(_e363, (vec4((1f - _e365)) * _e368), vec4(1f));
    density = _e372;
    if (((mipLevel_1 * 0.5f) + ((jitter_1 - 0.5f) * 0.5f)) < 0.5f) {
        {
            let _e382 = turbulence;
            let _e384 = global.shapeDetailRepeat;
            let _e386 = global.shapeDetailOffset;
            detailPosition = (((position_3 + _e382) * _e384) + _e386);
            let _e389 = detailPosition;
            let _e390 = textureSample(shapeDetailTexture_t, shapeDetailTexture_s, _e389);
            detail = _e390.x;
            let _e393 = detail;
            let _e395 = pt_pow(_e393, 6f);
            let _e398 = detail;
            let _e406 = remapClamped_7(weather_1.heightFraction, vec4(0.2f), vec4(0.4f));
            modifier = mix(vec4(_e395), vec4((1f - _e398)), _e406);
            let _e411 = modifier;
            let _e412 = global.shapeDetailAmounts;
            modifier = mix(vec4(0f), _e411, _e412);
            let _e414 = density;
            let _e417 = modifier;
            let _e423 = remapClamped_7((_e414 * 2f), vec4<f32>((_e417 * 0.5f)), vec4(1f));
            density = _e423;
        }
    }
    let _e424 = density;
    let _e425 = global.densityScales;
    let _e428 = getLayerDensity(weather_1.heightFraction);
    density = clamp(((_e424 * _e425) * _e428), vec4(0f), vec4(1f));
    let _e436 = density;
    let _e438 = density;
    let _e441 = density;
    let _e444 = density;
    densitySum = (((_e436.x + _e438.y) + _e441.z) + _e444.w);
    let _e449 = density;
    let _e450 = densitySum;
    media.weight = (_e449 / vec4(_e450));
    let _e454 = densitySum;
    let _e455 = global.scatteringCoefficient;
    media.scattering = (_e454 * _e455);
    let _e458 = densitySum;
    let _e459 = global.absorptionCoefficient;
    let _e461 = media;
    media.extinction = ((_e458 * _e459) + _e461.scattering);
    let _e464 = media;
    return _e464;
}

fn sampleMedia_1(weather_2: WeatherSample, position_4: vec3<f32>, uv_11: vec2<f32>, mipLevel_2: f32, jitter_2: f32) -> MediaSample {
    var sampleCount_2: vec3<i32>;

    let _e311 = sampleMedia(weather_2, position_4, uv_11, mipLevel_2, jitter_2, (&sampleCount_2));
    return _e311;
}

fn getViewZ(depth_8: f32) -> f32 {
    let _e310 = global.cameraNear;
    let _e311 = global.cameraFar;
    let _e312 = perspectiveDepthToViewZ(depth_8, _e310, _e311);
    return _e312;
}

fn ecefToWorld(positionECEF: vec3<f32>) -> vec3<f32> {
    let _e310 = global.ecefToWorldMatrix;
    let _e311 = global.altitudeCorrection;
    let _e312 = (positionECEF - _e311);
    return (_e310 * vec4<f32>(_e312.x, _e312.y, _e312.z, 1f)).xyz;
}

fn getShadowUv(worldPosition_2: vec3<f32>, cascadeIndex: i32) -> vec2<f32> {
    var clip: vec4<f32>;

    let _e312 = global.shadowMatrices[cascadeIndex];
    clip = (_e312 * vec4<f32>(worldPosition_2.x, worldPosition_2.y, worldPosition_2.z, 1f));
    let _e320 = clip;
    let _e321 = clip;
    clip = (_e320 / vec4(_e321.w));
    let _e325 = clip;
    return ((_e325.xy * 0.5f) + vec2(0.5f));
}

fn getDistanceToShadowTop(rayPosition: vec3<f32>) -> f32 {
    let _e310 = global.sunDirection;
    let _e313 = global.bottomRadius;
    let _e314 = global.shadowTopHeight;
    let _e316 = raySphereSecondIntersection(rayPosition, _e310, vec3(0f), (_e313 + _e314));
    return _e316;
}

fn readShadowOpticalDepth(uv_12: vec2<f32>, distanceToTop: f32, distanceOffset: f32, cascadeIndex_1: i32) -> f32 {
    var shadow: vec4<f32>;
    var distanceToFront: f32;

    let _e316 = vec3<f32>(uv_12.x, uv_12.y, f32(cascadeIndex_1));
    let _e320 = textureSample(shadowBuffer_t, shadowBuffer_s, _e316.xy, i32(_e316.z));
    shadow = _e320;
    let _e324 = shadow;
    distanceToFront = max(0f, ((distanceToTop - distanceOffset) - _e324.x));
    let _e329 = shadow;
    let _e331 = shadow;
    let _e334 = shadow;
    let _e336 = distanceToFront;
    return min((_e329.z + _e331.w), (_e334.y * _e336));
}

fn sampleShadowOpticalDepthPCF(worldPosition_3: vec3<f32>, distanceToTop_1: f32, distanceOffset_1: f32, radius_13: f32, cascadeIndex_2: i32) -> f32 {
    var uv_13: vec2<f32>;
    var sum: f32 = 0f;
    var offset: vec2<f32>;

    let _e314 = getShadowUv(worldPosition_3, cascadeIndex_2);
    uv_13 = _e314;
    let _e316 = uv_13;
    let _e320 = uv_13;
    let _e325 = uv_13;
    let _e330 = uv_13;
    if ((((_e316.x < 0f) || (_e320.x > 1f)) || (_e325.y < 0f)) || (_e330.y > 1f)) {
        {
            return 0f;
        }
    }
    if (radius_13 < 0.1f) {
        {
            let _e338 = uv_13;
            let _e339 = readShadowOpticalDepth(_e338, distanceToTop_1, distanceOffset_1, cascadeIndex_2);
            return _e339;
        }
    }
    let _e345 = gl_FragCoord_1;
    let _e347 = global.temporalJitter;
    let _e348 = global.resolution;
    let _e351 = interleavedGradientNoise((_e345.xy + (_e347 * _e348)));
    let _e354 = vogelDisk(0i, 8i, (_e351 * 6.2831855f));
    offset = _e354;
    let _e355 = sum;
    let _e356 = uv_13;
    let _e357 = offset;
    let _e359 = global.shadowTexelSize;
    let _e362 = readShadowOpticalDepth((_e356 + ((_e357 * radius_13) * _e359)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e355 + _e362);
    let _e366 = gl_FragCoord_1;
    let _e368 = global.temporalJitter;
    let _e369 = global.resolution;
    let _e372 = interleavedGradientNoise((_e366.xy + (_e368 * _e369)));
    let _e375 = vogelDisk(1i, 8i, (_e372 * 6.2831855f));
    offset = _e375;
    let _e376 = sum;
    let _e377 = uv_13;
    let _e378 = offset;
    let _e380 = global.shadowTexelSize;
    let _e383 = readShadowOpticalDepth((_e377 + ((_e378 * radius_13) * _e380)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e376 + _e383);
    let _e387 = gl_FragCoord_1;
    let _e389 = global.temporalJitter;
    let _e390 = global.resolution;
    let _e393 = interleavedGradientNoise((_e387.xy + (_e389 * _e390)));
    let _e396 = vogelDisk(2i, 8i, (_e393 * 6.2831855f));
    offset = _e396;
    let _e397 = sum;
    let _e398 = uv_13;
    let _e399 = offset;
    let _e401 = global.shadowTexelSize;
    let _e404 = readShadowOpticalDepth((_e398 + ((_e399 * radius_13) * _e401)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e397 + _e404);
    let _e408 = gl_FragCoord_1;
    let _e410 = global.temporalJitter;
    let _e411 = global.resolution;
    let _e414 = interleavedGradientNoise((_e408.xy + (_e410 * _e411)));
    let _e417 = vogelDisk(3i, 8i, (_e414 * 6.2831855f));
    offset = _e417;
    let _e418 = sum;
    let _e419 = uv_13;
    let _e420 = offset;
    let _e422 = global.shadowTexelSize;
    let _e425 = readShadowOpticalDepth((_e419 + ((_e420 * radius_13) * _e422)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e418 + _e425);
    let _e429 = gl_FragCoord_1;
    let _e431 = global.temporalJitter;
    let _e432 = global.resolution;
    let _e435 = interleavedGradientNoise((_e429.xy + (_e431 * _e432)));
    let _e438 = vogelDisk(4i, 8i, (_e435 * 6.2831855f));
    offset = _e438;
    let _e439 = sum;
    let _e440 = uv_13;
    let _e441 = offset;
    let _e443 = global.shadowTexelSize;
    let _e446 = readShadowOpticalDepth((_e440 + ((_e441 * radius_13) * _e443)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e439 + _e446);
    let _e450 = gl_FragCoord_1;
    let _e452 = global.temporalJitter;
    let _e453 = global.resolution;
    let _e456 = interleavedGradientNoise((_e450.xy + (_e452 * _e453)));
    let _e459 = vogelDisk(5i, 8i, (_e456 * 6.2831855f));
    offset = _e459;
    let _e460 = sum;
    let _e461 = uv_13;
    let _e462 = offset;
    let _e464 = global.shadowTexelSize;
    let _e467 = readShadowOpticalDepth((_e461 + ((_e462 * radius_13) * _e464)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e460 + _e467);
    let _e471 = gl_FragCoord_1;
    let _e473 = global.temporalJitter;
    let _e474 = global.resolution;
    let _e477 = interleavedGradientNoise((_e471.xy + (_e473 * _e474)));
    let _e480 = vogelDisk(6i, 8i, (_e477 * 6.2831855f));
    offset = _e480;
    let _e481 = sum;
    let _e482 = uv_13;
    let _e483 = offset;
    let _e485 = global.shadowTexelSize;
    let _e488 = readShadowOpticalDepth((_e482 + ((_e483 * radius_13) * _e485)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e481 + _e488);
    let _e492 = gl_FragCoord_1;
    let _e494 = global.temporalJitter;
    let _e495 = global.resolution;
    let _e498 = interleavedGradientNoise((_e492.xy + (_e494 * _e495)));
    let _e501 = vogelDisk(7i, 8i, (_e498 * 6.2831855f));
    offset = _e501;
    let _e502 = sum;
    let _e503 = uv_13;
    let _e504 = offset;
    let _e506 = global.shadowTexelSize;
    let _e509 = readShadowOpticalDepth((_e503 + ((_e504 * radius_13) * _e506)), distanceToTop_1, distanceOffset_1, cascadeIndex_2);
    sum = (_e502 + _e509);
    let _e511 = sum;
    return (_e511 / 8f);
}

fn sampleShadowOpticalDepth(rayPosition_1: vec3<f32>, distanceOffset_2: f32, radius_14: f32, jitter_3: f32) -> f32 {
    var distanceToTop_2: f32;
    var worldPosition_4: vec3<f32>;
    var cascadeIndex_3: i32;
    var local_14: f32;

    let _e313 = getDistanceToShadowTop(rayPosition_1);
    distanceToTop_2 = _e313;
    let _e315 = distanceToTop_2;
    if (_e315 <= 0f) {
        {
            return 0f;
        }
    }
    let _e319 = ecefToWorld(rayPosition_1);
    worldPosition_4 = _e319;
    let _e321 = global.viewMatrix;
    let _e322 = worldPosition_4;
    let _e323 = shadowIntervals;
    let _e324 = global.cameraNear;
    let _e325 = global.shadowFar;
    let _e326 = getFadedCascadeIndex(_e321, _e322, _e323, _e324, _e325, jitter_3);
    cascadeIndex_3 = _e326;
    let _e328 = cascadeIndex_3;
    if (_e328 >= 0i) {
        let _e331 = worldPosition_4;
        let _e332 = distanceToTop_2;
        let _e333 = cascadeIndex_3;
        let _e334 = sampleShadowOpticalDepthPCF(_e331, _e332, distanceOffset_2, radius_14, _e333);
        local_14 = _e334;
    } else {
        local_14 = 0f;
    }
    let _e337 = local_14;
    return _e337;
}

fn henyeyGreenstein(g_2: vec2<f32>, cosTheta: f32) -> vec2<f32> {
    var g2_: vec2<f32>;

    g2_ = (g_2 * g_2);
    let _e315 = g2_;
    let _e321 = g2_;
    let _e330 = pt_pow_1(((vec2(1f) + _e321) - ((2f * g_2) * cosTheta)), vec2(1.5f));
    return (0.07957747f * ((vec2(1f) - _e315) / max(vec2(0.0000001f), _e330)));
}

fn phaseFunction(cosTheta_1: f32, attenuation: f32) -> f32 {
    var g_3: vec2<f32> = scatterAnisotropy;
    var weights: vec2<f32> = vec2<f32>(0.5f, 0.5f);

    let _e318 = g_3;
    let _e320 = henyeyGreenstein((_e318 * attenuation), cosTheta_1);
    let _e321 = weights;
    return dot(_e320, _e321);
}

fn phaseFunction_1(cosTheta_2: f32) -> f32 {
    let _e311 = phaseFunction(cosTheta_2, 1f);
    return _e311;
}

fn marchOpticalDepth(rayOrigin: vec3<f32>, rayDirection: vec3<f32>, maxIterationCount: i32, mipLevel_3: f32, jitter_4: f32, rayDistance: ptr<function, f32>) -> f32 {
    var iterationCount: i32;
    var stepSize: f32;
    var nextDistance: f32;
    var opticalDepth: f32 = 0f;
    var i_4: i32 = 0i;
    var position_5: vec3<f32>;
    var uv_14: vec2<f32>;
    var height_2: f32;
    var weather_3: WeatherSample;
    var media_1: MediaSample;

    let _e322 = remap(mipLevel_3, 0f, 1f, f32((maxIterationCount + 1i)), 1f);
    iterationCount = i32(max(0f, (_e322 - jitter_4)));
    let _e327 = iterationCount;
    if (_e327 == 0i) {
        {
            return 0.5f;
        }
    }
    let _e331 = global.minSecondaryStepSize;
    let _e332 = iterationCount;
    stepSize = (_e331 / f32(_e332));
    let _e336 = stepSize;
    nextDistance = (_e336 * jitter_4);
    loop {
        let _e343 = i_4;
        let _e344 = iterationCount;
        if !((_e343 < _e344)) {
            break;
        }
        {
            let _e350 = nextDistance;
            (*rayDistance) = _e350;
            let _e351 = (*rayDistance);
            position_5 = ((_e351 * rayDirection) + rayOrigin);
            let _e355 = position_5;
            let _e356 = getGlobeUv(_e355);
            uv_14 = _e356;
            let _e358 = position_5;
            let _e360 = global.bottomRadius;
            height_2 = (length(_e358) - _e360);
            let _e363 = uv_14;
            let _e364 = height_2;
            let _e365 = sampleWeather(_e363, _e364, mipLevel_3);
            weather_3 = _e365;
            let _e367 = weather_3;
            let _e368 = position_5;
            let _e369 = uv_14;
            let _e370 = sampleMedia_1(_e367, _e368, _e369, mipLevel_3, jitter_4);
            media_1 = _e370;
            let _e372 = opticalDepth;
            let _e373 = media_1;
            let _e375 = stepSize;
            opticalDepth = (_e372 + (_e373.extinction * _e375));
            let _e378 = nextDistance;
            let _e379 = stepSize;
            nextDistance = (_e378 + _e379);
            let _e381 = stepSize;
            let _e382 = global.secondaryStepScale;
            stepSize = (_e381 * _e382);
        }
        continuing {
            let _e347 = i_4;
            i_4 = (_e347 + 1i);
        }
    }
    let _e384 = opticalDepth;
    return _e384;
}

fn marchOpticalDepth_1(rayOrigin_1: vec3<f32>, rayDirection_1: vec3<f32>, maxIterationCount_1: i32, mipLevel_4: f32, jitter_5: f32) -> f32 {
    var rayDistance_1: f32;

    let _e317 = marchOpticalDepth(rayOrigin_1, rayDirection_1, maxIterationCount_1, mipLevel_4, jitter_5, (&rayDistance_1));
    return _e317;
}

fn approximateMultipleScattering(opticalDepth_1: f32, cosTheta_3: f32) -> f32 {
    var coeffs: vec3<f32> = vec3(1f);
    var attenuation_1: vec3<f32> = vec3<f32>(0.5f, 0.5f, 0.5f);
    var scattering_5: f32 = 0f;
    var beerLambert: f32;

    let _e323 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e323.y));
    let _e327 = scattering_5;
    let _e328 = coeffs;
    let _e330 = beerLambert;
    let _e332 = coeffs;
    let _e334 = phaseFunction(cosTheta_3, _e332.z);
    scattering_5 = (_e327 + ((_e328.x * _e330) * _e334));
    let _e337 = coeffs;
    let _e338 = attenuation_1;
    coeffs = (_e337 * _e338);
    let _e341 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e341.y));
    let _e345 = scattering_5;
    let _e346 = coeffs;
    let _e348 = beerLambert;
    let _e350 = coeffs;
    let _e352 = phaseFunction(cosTheta_3, _e350.z);
    scattering_5 = (_e345 + ((_e346.x * _e348) * _e352));
    let _e355 = coeffs;
    let _e356 = attenuation_1;
    coeffs = (_e355 * _e356);
    let _e359 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e359.y));
    let _e363 = scattering_5;
    let _e364 = coeffs;
    let _e366 = beerLambert;
    let _e368 = coeffs;
    let _e370 = phaseFunction(cosTheta_3, _e368.z);
    scattering_5 = (_e363 + ((_e364.x * _e366) * _e370));
    let _e373 = coeffs;
    let _e374 = attenuation_1;
    coeffs = (_e373 * _e374);
    let _e377 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e377.y));
    let _e381 = scattering_5;
    let _e382 = coeffs;
    let _e384 = beerLambert;
    let _e386 = coeffs;
    let _e388 = phaseFunction(cosTheta_3, _e386.z);
    scattering_5 = (_e381 + ((_e382.x * _e384) * _e388));
    let _e391 = coeffs;
    let _e392 = attenuation_1;
    coeffs = (_e391 * _e392);
    let _e395 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e395.y));
    let _e399 = scattering_5;
    let _e400 = coeffs;
    let _e402 = beerLambert;
    let _e404 = coeffs;
    let _e406 = phaseFunction(cosTheta_3, _e404.z);
    scattering_5 = (_e399 + ((_e400.x * _e402) * _e406));
    let _e409 = coeffs;
    let _e410 = attenuation_1;
    coeffs = (_e409 * _e410);
    let _e413 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e413.y));
    let _e417 = scattering_5;
    let _e418 = coeffs;
    let _e420 = beerLambert;
    let _e422 = coeffs;
    let _e424 = phaseFunction(cosTheta_3, _e422.z);
    scattering_5 = (_e417 + ((_e418.x * _e420) * _e424));
    let _e427 = coeffs;
    let _e428 = attenuation_1;
    coeffs = (_e427 * _e428);
    let _e431 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e431.y));
    let _e435 = scattering_5;
    let _e436 = coeffs;
    let _e438 = beerLambert;
    let _e440 = coeffs;
    let _e442 = phaseFunction(cosTheta_3, _e440.z);
    scattering_5 = (_e435 + ((_e436.x * _e438) * _e442));
    let _e445 = coeffs;
    let _e446 = attenuation_1;
    coeffs = (_e445 * _e446);
    let _e449 = coeffs;
    beerLambert = exp((-(opticalDepth_1) * _e449.y));
    let _e453 = scattering_5;
    let _e454 = coeffs;
    let _e456 = beerLambert;
    let _e458 = coeffs;
    let _e460 = phaseFunction(cosTheta_3, _e458.z);
    scattering_5 = (_e453 + ((_e454.x * _e456) * _e460));
    let _e463 = coeffs;
    let _e464 = attenuation_1;
    coeffs = (_e463 * _e464);
    let _e466 = scattering_5;
    return _e466;
}

fn getGroundSunSkyIrradiance(position_6: vec3<f32>, surfaceNormal_1: vec3<f32>, height_3: f32, skyIrradiance: ptr<function, vec3<f32>>) -> vec3<f32> {
    let _e317 = global.sunDirection;
    let _e320 = GetSunAndSkyIlluminance(((position_6 - (surfaceNormal_1 * height_3)) * 0.001f), surfaceNormal_1, _e317, skyIrradiance);
    return _e320;
}

fn getCloudsSunSkyIrradiance(position_7: vec3<f32>, height_4: f32, skyIrradiance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    let _e314 = global.sunDirection;
    let _e317 = GetSunAndSkyScalarIlluminance((position_7 * 0.001f), _e314, skyIrradiance_1);
    return _e317;
}

fn approximateRadianceFromGround(position_8: vec3<f32>, surfaceNormal_2: vec3<f32>, height_5: f32, mipLevel_5: f32, jitter_6: f32) -> vec3<f32> {
    var opticalDepthToGround: f32;
    var skyIrradiance_2: vec3<f32>;
    var sunIrradiance: vec3<f32>;
    var groundAlbedo: f32 = 0.3f;
    var groundIrradiance: vec3<f32>;
    var bouncedRadiance: vec3<f32>;

    let _e315 = global.maxIterationCountToGround;
    let _e316 = marchOpticalDepth_1(position_8, -(surfaceNormal_2), _e315, mipLevel_5, jitter_6);
    opticalDepthToGround = _e316;
    let _e321 = getGroundSunSkyIrradiance(position_8, surfaceNormal_2, height_5, (&skyIrradiance_2));
    sunIrradiance = _e321;
    let _e325 = skyIrradiance_2;
    let _e327 = global.coverage;
    let _e329 = sunIrradiance;
    groundIrradiance = (_e325 + ((1f - _e327) * _e329));
    let _e333 = groundAlbedo;
    let _e336 = groundIrradiance;
    bouncedRadiance = ((_e333 * 0.31830987f) * _e336);
    let _e339 = bouncedRadiance;
    let _e340 = opticalDepthToGround;
    return (_e339 * exp(-(_e340)));
}

fn marchClouds(rayOrigin_2: vec3<f32>, rayDirection_2: vec3<f32>, rayNearFar: vec2<f32>, cosTheta_4: f32, jitter_7: f32, rayStartTexelsPerPixel: f32, frontDepth: ptr<function, f32>, sampleCount_3: ptr<function, vec3<i32>>) -> vec4<f32> {
    var radianceIntegral: vec3<f32> = vec3(0f);
    var transmittanceIntegral: f32 = 1f;
    var weightedDistanceSum: f32 = 0f;
    var transmittanceSum: f32 = 0f;
    var maxRayDistance: f32;
    var stepSize_1: f32;
    var rayDistance_2: f32;
    var i_5: i32 = 0i;
    var position_9: vec3<f32>;
    var height_6: f32;
    var mipLevel_6: f32;
    var uv_15: vec2<f32>;
    var weather_4: WeatherSample;
    var media_2: MediaSample;
    var skyIrradiance_3: vec3<f32>;
    var sunIrradiance_1: vec3<f32>;
    var surfaceNormal_3: vec3<f32>;
    var sunRayDistance: f32;
    var opticalDepth_2: f32;
    var radiance: vec3<f32>;
    var groundRadiance: vec3<f32>;
    var skyGradient: f32;
    var transmittance_4: f32;
    var clampedExtinction: f32;
    var scatteringIntegral: vec3<f32>;
    var local_15: f32;

    maxRayDistance = (rayNearFar.y - rayNearFar.x);
    let _e330 = global.minStepSize;
    let _e331 = global.perspectiveStepScale;
    stepSize_1 = (_e330 + ((_e331 - 1f) * rayNearFar.x));
    let _e338 = stepSize_1;
    rayDistance_2 = ((_e338 * jitter_7) * 2f);
    loop {
        let _e345 = i_5;
        let _e346 = global.maxIterationCount;
        if !((_e345 < _e346)) {
            break;
        }
        {
            let _e352 = rayDistance_2;
            let _e353 = maxRayDistance;
            if (_e352 > _e353) {
                {
                    break;
                }
            }
            let _e355 = rayDistance_2;
            position_9 = ((_e355 * rayDirection_2) + rayOrigin_2);
            let _e359 = position_9;
            let _e361 = global.bottomRadius;
            height_6 = (length(_e359) - _e361);
            let _e365 = rayDistance_2;
            mipLevel_6 = log2(max(1f, (rayStartTexelsPerPixel + (_e365 * 0.00001f))));
            let _e372 = height_6;
            let _e373 = insideLayerIntervals(_e372);
            if _e373 {
                {
                    let _e374 = stepSize_1;
                    let _e375 = global.perspectiveStepScale;
                    stepSize_1 = (_e374 * _e375);
                    let _e377 = rayDistance_2;
                    let _e378 = stepSize_1;
                    let _e379 = global.maxStepSize;
                    let _e381 = mipLevel_6;
                    rayDistance_2 = (_e377 + mix(_e378, _e379, min(1f, _e381)));
                    continue;
                }
            }
            let _e385 = position_9;
            let _e386 = getGlobeUv(_e385);
            uv_15 = _e386;
            let _e388 = uv_15;
            let _e389 = height_6;
            let _e390 = mipLevel_6;
            let _e391 = sampleWeather(_e388, _e389, _e390);
            weather_4 = _e391;
            let _e393 = weather_4;
            let _e395 = global.minDensity;
            if !(any((_e393.density > vec4(_e395)))) {
                {
                    let _e400 = stepSize_1;
                    let _e401 = global.perspectiveStepScale;
                    stepSize_1 = (_e400 * _e401);
                    let _e403 = rayDistance_2;
                    let _e404 = stepSize_1;
                    let _e405 = global.maxStepSize;
                    let _e407 = mipLevel_6;
                    rayDistance_2 = (_e403 + mix(_e404, _e405, min(1f, _e407)));
                    continue;
                }
            }
            let _e411 = weather_4;
            let _e412 = position_9;
            let _e413 = uv_15;
            let _e414 = mipLevel_6;
            let _e417 = sampleMedia(_e411, _e412, _e413, _e414, jitter_7, sampleCount_3);
            media_2 = _e417;
            let _e419 = media_2;
            let _e421 = global.minExtinction;
            if (_e419.extinction > _e421) {
                {
                    let _e424 = position_9;
                    let _e425 = height_6;
                    let _e428 = getCloudsSunSkyIrradiance(_e424, _e425, (&skyIrradiance_3));
                    sunIrradiance_1 = _e428;
                    let _e430 = position_9;
                    surfaceNormal_3 = normalize(_e430);
                    sunRayDistance = 0f;
                    let _e435 = position_9;
                    let _e436 = global.sunDirection;
                    let _e437 = global.maxIterationCountToSun;
                    let _e438 = mipLevel_6;
                    let _e441 = marchOpticalDepth(_e435, _e436, _e437, _e438, jitter_7, (&sunRayDistance));
                    opticalDepth_2 = _e441;
                    let _e443 = height_6;
                    let _e444 = global.shadowTopHeight;
                    if (_e443 < _e444) {
                        {
                            let _e446 = opticalDepth_2;
                            let _e447 = position_9;
                            let _e448 = sunRayDistance;
                            let _e449 = global.maxShadowFilterRadius;
                            let _e450 = global.sunDirection;
                            let _e451 = surfaceNormal_3;
                            let _e455 = remapClamped_4(dot(_e450, _e451), 0.1f, 0f);
                            let _e457 = sampleShadowOpticalDepth(_e447, _e448, (_e449 * _e455), jitter_7);
                            opticalDepth_2 = (_e446 + _e457);
                        }
                    }
                    let _e459 = sunIrradiance_1;
                    let _e460 = opticalDepth_2;
                    let _e461 = approximateMultipleScattering(_e460, cosTheta_4);
                    radiance = (_e459 * _e461);
                    let _e464 = height_6;
                    let _e465 = global.shadowTopHeight;
                    let _e467 = mipLevel_6;
                    if ((_e464 < _e465) && (_e467 < 0.5f)) {
                        {
                            let _e471 = position_9;
                            let _e472 = surfaceNormal_3;
                            let _e473 = height_6;
                            let _e474 = mipLevel_6;
                            let _e475 = approximateRadianceFromGround(_e471, _e472, _e473, _e474, jitter_7);
                            groundRadiance = _e475;
                            let _e477 = radiance;
                            let _e478 = groundRadiance;
                            let _e481 = global.groundBounceScale;
                            radiance = (_e477 + ((_e478 * 0.07957747f) * _e481));
                        }
                    }
                    let _e484 = weather_4;
                    let _e491 = media_2;
                    skyGradient = dot(((_e484.heightFraction * 0.5f) + vec4(0.5f)), _e491.weight);
                    let _e495 = radiance;
                    let _e496 = skyIrradiance_3;
                    let _e499 = skyGradient;
                    let _e501 = global.skyLightScale;
                    radiance = (_e495 + (((_e496 * 0.07957747f) * _e499) * _e501));
                    let _e504 = radiance;
                    let _e505 = media_2;
                    radiance = (_e504 * _e505.scattering);
                    let _e508 = radiance;
                    let _e510 = global.powderScale;
                    let _e511 = media_2;
                    let _e514 = global.powderExponent;
                    radiance = (_e508 * (1f - (_e510 * exp((-(_e511.extinction) * _e514)))));
                    let _e520 = media_2;
                    let _e523 = stepSize_1;
                    transmittance_4 = exp((-(_e520.extinction) * _e523));
                    let _e527 = media_2;
                    clampedExtinction = max(_e527.extinction, 0.0000001f);
                    let _e532 = radiance;
                    let _e533 = radiance;
                    let _e534 = transmittance_4;
                    let _e537 = clampedExtinction;
                    scatteringIntegral = ((_e532 - (_e533 * _e534)) / vec3(_e537));
                    let _e541 = radianceIntegral;
                    let _e542 = transmittanceIntegral;
                    let _e543 = scatteringIntegral;
                    radianceIntegral = (_e541 + (_e542 * _e543));
                    let _e546 = transmittanceIntegral;
                    let _e547 = transmittance_4;
                    transmittanceIntegral = (_e546 * _e547);
                    let _e549 = weightedDistanceSum;
                    let _e550 = rayDistance_2;
                    let _e551 = transmittanceIntegral;
                    weightedDistanceSum = (_e549 + (_e550 * _e551));
                    let _e554 = transmittanceSum;
                    let _e555 = transmittanceIntegral;
                    transmittanceSum = (_e554 + _e555);
                }
            }
            let _e557 = transmittanceIntegral;
            let _e558 = global.minTransmittance;
            if (_e557 <= _e558) {
                {
                    break;
                }
            }
            let _e560 = stepSize_1;
            let _e561 = global.perspectiveStepScale;
            stepSize_1 = (_e560 * _e561);
            let _e563 = rayDistance_2;
            let _e564 = stepSize_1;
            rayDistance_2 = (_e563 + _e564);
        }
        continuing {
            let _e349 = i_5;
            i_5 = (_e349 + 1i);
        }
    }
    let _e566 = transmittanceSum;
    if (_e566 > 0f) {
        let _e569 = weightedDistanceSum;
        let _e570 = transmittanceSum;
        local_15 = (_e569 / _e570);
    } else {
        local_15 = -1f;
    }
    let _e575 = local_15;
    (*frontDepth) = _e575;
    let _e576 = radianceIntegral;
    let _e577 = transmittanceIntegral;
    let _e579 = global.minTransmittance;
    let _e580 = remapClamped_4(_e577, 1f, _e579);
    return vec4<f32>(_e576.x, _e576.y, _e576.z, _e580);
}

fn marchShadowLength(rayOrigin_3: vec3<f32>, rayDirection_3: vec3<f32>, rayNearFar_1: vec2<f32>, jitter_8: f32) -> f32 {
    var shadowLength: f32 = 0f;
    var maxRayDistance_1: f32;
    var stepSize_2: f32;
    var rayDistance_3: f32;
    var attenuationFactor: f32 = 0.9995f;
    var attenuation_2: f32 = 1f;
    var i_6: i32 = 0i;
    var position_10: vec3<f32>;
    var opticalDepth_3: f32;

    maxRayDistance_1 = (rayNearFar_1.y - rayNearFar_1.x);
    let _e319 = global.minShadowLengthStepSize;
    stepSize_2 = _e319;
    let _e321 = stepSize_2;
    rayDistance_3 = (_e321 * jitter_8);
    loop {
        let _e332 = i_6;
        let _e333 = global.maxShadowLengthIterationCount;
        if !((_e332 < _e333)) {
            break;
        }
        {
            let _e339 = rayDistance_3;
            let _e340 = maxRayDistance_1;
            if (_e339 > _e340) {
                {
                    break;
                }
            }
            let _e342 = rayDistance_3;
            position_10 = ((_e342 * rayDirection_3) + rayOrigin_3);
            let _e346 = position_10;
            let _e349 = sampleShadowOpticalDepth(_e346, 0f, 0f, jitter_8);
            opticalDepth_3 = _e349;
            let _e351 = shadowLength;
            let _e353 = opticalDepth_3;
            let _e357 = stepSize_2;
            let _e359 = attenuation_2;
            shadowLength = (_e351 + (((1f - exp(-(_e353))) * _e357) * _e359));
            let _e362 = stepSize_2;
            let _e363 = global.perspectiveStepScale;
            stepSize_2 = (_e362 * _e363);
            let _e365 = rayDistance_3;
            let _e366 = stepSize_2;
            rayDistance_3 = (_e365 + _e366);
        }
        continuing {
            let _e336 = i_6;
            i_6 = (_e336 + 1i);
        }
    }
    let _e368 = shadowLength;
    return _e368;
}

fn approximateHaze(rayOrigin_4: vec3<f32>, rayDirection_4: vec3<f32>, maxRayDistance_2: f32, cosTheta_5: f32, shadowLength_1: f32) -> vec4<f32> {
    var modulation: f32;
    var density_1: f32;
    var normalAtOrigin: vec3<f32>;
    var normalAtHorizon: vec3<f32>;
    var alpha_1: f32;
    var normal_6: vec3<f32>;
    var angle: f32;
    var exponent: f32;
    var linearTerm: f32;
    var expTerm: f32;
    var shadowExpTerm: f32;
    var opticalDepth_4: f32;
    var shadowOpticalDepth: f32;
    var transmittance_5: f32;
    var shadowTransmittance: f32;
    var skyIrradiance_4: vec3<f32>;
    var sunIrradiance_2: vec3<f32>;
    var inscatter: vec3<f32>;

    let _e314 = global.coverage;
    let _e317 = remapClamped_4(_e314, 0.2f, 0.4f);
    modulation = _e317;
    let _e319 = global.cameraHeight;
    let _e320 = modulation;
    if ((_e319 * _e320) < 0f) {
        {
            return vec4(0f);
        }
    }
    let _e326 = modulation;
    let _e327 = global.hazeDensityScale;
    let _e329 = global.cameraHeight;
    let _e331 = global.hazeExponent;
    density_1 = ((_e326 * _e327) * exp((-(_e329) * _e331)));
    let _e336 = density_1;
    if (_e336 < 0.0000001f) {
        {
            return vec4(0f);
        }
    }
    normalAtOrigin = normalize(rayOrigin_4);
    let _e346 = global.bottomRadius;
    normalAtHorizon = ((rayOrigin_4 - (dot(rayOrigin_4, rayDirection_4) * rayDirection_4)) / vec3(_e346));
    let _e350 = normalAtOrigin;
    let _e351 = normalAtHorizon;
    let _e355 = remapClamped_4(dot(_e350, _e351), 0.9f, 1f);
    alpha_1 = _e355;
    let _e357 = normalAtOrigin;
    let _e358 = normalAtHorizon;
    let _e359 = alpha_1;
    normal_6 = mix(_e357, _e358, vec3(_e359));
    let _e363 = normal_6;
    angle = max(dot(_e363, rayDirection_4), 0.00001f);
    let _e368 = angle;
    let _e369 = global.hazeExponent;
    exponent = (_e368 * _e369);
    let _e372 = density_1;
    let _e373 = global.hazeExponent;
    let _e375 = angle;
    linearTerm = ((_e372 / _e373) / _e375);
    let _e380 = exponent;
    expTerm = (1f - exp((-(maxRayDistance_2) * _e380)));
    let _e388 = exponent;
    shadowExpTerm = (1f - exp((-(min(maxRayDistance_2, shadowLength_1)) * _e388)));
    let _e393 = expTerm;
    let _e394 = linearTerm;
    opticalDepth_4 = (_e393 * _e394);
    let _e397 = expTerm;
    let _e398 = shadowExpTerm;
    let _e400 = linearTerm;
    shadowOpticalDepth = max(((_e397 - _e398) * _e400), 0f);
    let _e406 = opticalDepth_4;
    transmittance_5 = clamp((1f - exp(-(_e406))), 0f, 1f);
    let _e415 = shadowOpticalDepth;
    shadowTransmittance = clamp((1f - exp(-(_e415))), 0f, 1f);
    let _e423 = vGroundIrradiance;
    skyIrradiance_4 = _e423.sky;
    let _e426 = vGroundIrradiance;
    sunIrradiance_2 = _e426.sun;
    let _e429 = sunIrradiance_2;
    let _e430 = phaseFunction_1(cosTheta_5);
    let _e432 = shadowTransmittance;
    inscatter = ((_e429 * _e430) * _e432);
    let _e435 = inscatter;
    let _e436 = skyIrradiance_4;
    let _e439 = global.skyLightScale;
    let _e441 = transmittance_5;
    inscatter = (_e435 + (((_e436 * 0.07957747f) * _e439) * _e441));
    let _e444 = inscatter;
    let _e445 = global.hazeScatteringCoefficient;
    let _e446 = global.hazeAbsorptionCoefficient;
    let _e447 = global.hazeScatteringCoefficient;
    inscatter = (_e444 * (_e445 / (_e446 + _e447)));
    let _e451 = inscatter;
    let _e452 = transmittance_5;
    return vec4<f32>(_e451.x, _e451.y, _e451.z, _e452);
}

fn applyAerialPerspective(cameraPosition: vec3<f32>, frontPosition: vec3<f32>, shadowLength_2: f32, color: ptr<function, vec4<f32>>) {
    var transmittance_6: vec3<f32>;
    var inscatter_1: vec3<f32>;

    let _e320 = global.sunDirection;
    let _e323 = GetSkyLuminanceToPoint((cameraPosition * 0.001f), (frontPosition * 0.001f), (shadowLength_2 * 0.001f), _e320, (&transmittance_6));
    inscatter_1 = _e323;
    let _e325 = (*color);
    let _e327 = (*color);
    let _e329 = transmittance_6;
    let _e331 = inscatter_1;
    let _e332 = (*color);
    let _e335 = ((_e327.xyz * _e329) + (_e331 * _e332.w));
    (*color).x = _e335.x;
    (*color).y = _e335.y;
    (*color).z = _e335.z;
    return;
}

fn rayIntersectsGround(cameraPosition_1: vec3<f32>, rayDirection_5: vec3<f32>) -> bool {
    var r_20: f32;
    var mu_12: f32;

    r_20 = length(cameraPosition_1);
    let _e314 = r_20;
    mu_12 = (dot(cameraPosition_1, rayDirection_5) / _e314);
    let _e317 = mu_12;
    let _e320 = r_20;
    let _e321 = r_20;
    let _e323 = mu_12;
    let _e324 = mu_12;
    let _e329 = global.bottomRadius;
    let _e330 = global.bottomRadius;
    return ((_e317 < 0f) && ((((_e320 * _e321) * ((_e323 * _e324) - 1f)) + (_e329 * _e330)) >= 0f));
}

fn getIntersections(cameraPosition_2: vec3<f32>, rayDirection_6: vec3<f32>) -> IntersectionResult {
    var intersections: IntersectionResult;

    let _e313 = rayIntersectsGround(cameraPosition_2, rayDirection_6);
    intersections.ground = _e313;
    let _e314 = global.bottomRadius;
    let _e316 = global.minHeight;
    let _e317 = global.maxHeight;
    let _e318 = global.shadowTopHeight;
    let _e322 = intersections;
    let _e324 = intersections;
    var pt_inout_0 = intersections.first;
    var pt_inout_1 = intersections.second;
    raySphereIntersections_3(cameraPosition_2, rayDirection_6, (vec4(_e314) + vec4<f32>(0f, _e316, _e317, _e318)), (&pt_inout_0), (&pt_inout_1));
    intersections.first = pt_inout_0;
    intersections.second = pt_inout_1;
    let _e330 = intersections;
    return _e330;
}

fn getRayNearFar(intersections_1: IntersectionResult) -> vec2<f32> {
    var nearFar: vec2<f32>;

    let _e311 = global.cameraHeight;
    let _e312 = global.minHeight;
    if (_e311 < _e312) {
        {
            if intersections_1.ground {
                {
                    nearFar = vec2(-1f);
                }
            } else {
                {
                    nearFar = vec2<f32>(intersections_1.second.y, intersections_1.second.z);
                    let _e324 = nearFar;
                    let _e326 = global.maxRayDistance;
                    nearFar.y = min(_e324.y, _e326);
                }
            }
        }
    } else {
        let _e328 = global.cameraHeight;
        let _e329 = global.maxHeight;
        if (_e328 < _e329) {
            {
                if intersections_1.ground {
                    {
                        let _e332 = global.cameraNear;
                        nearFar = vec2<f32>(_e332, intersections_1.first.y);
                    }
                } else {
                    {
                        let _e336 = global.cameraNear;
                        nearFar = vec2<f32>(_e336, intersections_1.second.z);
                    }
                }
            }
        } else {
            {
                nearFar = vec2<f32>(intersections_1.first.z, intersections_1.second.z);
                if intersections_1.ground {
                    {
                        nearFar.y = intersections_1.first.y;
                    }
                }
            }
        }
    }
    let _e349 = nearFar;
    return _e349;
}

fn getShadowRayNearFar(intersections_2: IntersectionResult) -> vec2<f32> {
    var nearFar_1: vec2<f32>;

    let _e311 = global.cameraHeight;
    let _e312 = global.shadowTopHeight;
    if (_e311 < _e312) {
        {
            if intersections_2.ground {
                {
                    let _e315 = global.cameraNear;
                    nearFar_1 = vec2<f32>(_e315, intersections_2.first.x);
                }
            } else {
                {
                    let _e319 = global.cameraNear;
                    nearFar_1 = vec2<f32>(_e319, intersections_2.second.w);
                }
            }
        }
    } else {
        {
            nearFar_1 = vec2<f32>(intersections_2.first.w, intersections_2.second.w);
            if intersections_2.ground {
                {
                    nearFar_1.y = intersections_2.first.x;
                }
            }
        }
    }
    let _e333 = nearFar_1;
    let _e335 = global.maxShadowLengthRayDistance;
    nearFar_1.y = min(_e333.y, _e335);
    let _e337 = nearFar_1;
    return _e337;
}

fn getHazeRayNearFar(intersections_3: IntersectionResult) -> vec2<f32> {
    var nearFar_2: vec2<f32>;

    let _e311 = global.cameraHeight;
    let _e312 = global.maxHeight;
    if (_e311 < _e312) {
        {
            if intersections_3.ground {
                {
                    let _e315 = global.cameraNear;
                    nearFar_2 = vec2<f32>(_e315, intersections_3.first.x);
                }
            } else {
                {
                    let _e319 = global.cameraNear;
                    nearFar_2 = vec2<f32>(_e319, intersections_3.second.z);
                }
            }
        }
    } else {
        {
            let _e323 = global.cameraNear;
            nearFar_2 = vec2<f32>(_e323, intersections_3.second.z);
            if intersections_3.ground {
                {
                    nearFar_2.y = intersections_3.first.x;
                }
            }
        }
    }
    let _e331 = nearFar_2;
    return _e331;
}

fn getRayDistanceToScene(rayDirection_7: vec3<f32>, viewZ_4: ptr<function, f32>) -> f32 {
    var depth_9: f32;

    let _e311 = vUv_1;
    let _e312 = global.targetUvScale;
    let _e314 = global.temporalJitter;
    let _e316 = readDepthValue(depthBuffer_t_1, depthBuffer_s_1, ((_e311 * _e312) + _e314));
    depth_9 = _e316;
    let _e318 = depth_9;
    if (_e318 < 0.9999999f) {
        {
            let _e323 = depth_9;
            let _e324 = global.cameraNear;
            let _e325 = global.cameraFar;
            let _e326 = reverseLogDepth(_e323, _e324, _e325);
            depth_9 = _e326;
            let _e327 = depth_9;
            let _e328 = getViewZ(_e327);
            (*viewZ_4) = _e328;
            let _e329 = (*viewZ_4);
            let _e331 = vCameraDirection_1;
            return (-(_e329) / dot(rayDirection_7, _e331));
        }
    }
    (*viewZ_4) = 0f;
    return 0f;
}

fn pt_main() {
    var cameraPosition_3: vec3<f32>;
    var rayDirection_8: vec3<f32>;
    var cosTheta_6: f32;
    var intersections_4: IntersectionResult;
    var rayNearFar_2: vec2<f32>;
    var shadowRayNearFar: vec2<f32>;
    var hazeRayNearFar: vec2<f32>;
    var sceneViewZ: f32;
    var rayDistanceToScene: f32;
    var intersectsGround: bool;
    var intersectsScene: bool;
    var stbn: f32;
    var color_1: vec4<f32> = vec4(0f);
    var frontDepth_1: f32;
    var depthVelocity: vec3<f32> = vec3(0f);
    var shadowLength_3: f32 = 0f;
    var hitClouds: bool = false;
    var rayOrigin_5: vec3<f32>;
    var globeUv: vec2<f32>;
    var mipLevel_7: f32;
    var marchedFrontDepth: f32;
    var sampleCount_4: vec3<i32> = vec3(0i);
    var frontPosition_1: vec3<f32>;
    var frontPositionWorld: vec3<f32>;
    var prevClip: vec4<f32>;
    var prevUv: vec2<f32>;
    var velocity: vec2<f32>;
    var local_16: f32;
    var frontView: vec3<f32>;
    var prevClip_1: vec4<f32>;
    var prevUv_1: vec2<f32>;
    var velocity_1: vec2<f32>;
    var haze: vec4<f32>;

    let _e309 = vCameraPosition_1;
    let _e310 = global.altitudeCorrection;
    cameraPosition_3 = (_e309 + _e310);
    let _e313 = vRayDirection_1;
    rayDirection_8 = normalize(_e313);
    let _e316 = global.sunDirection;
    let _e317 = rayDirection_8;
    cosTheta_6 = dot(_e316, _e317);
    let _e320 = cameraPosition_3;
    let _e321 = rayDirection_8;
    let _e322 = getIntersections(_e320, _e321);
    intersections_4 = _e322;
    let _e324 = intersections_4;
    let _e325 = getRayNearFar(_e324);
    rayNearFar_2 = _e325;
    let _e327 = intersections_4;
    let _e328 = getShadowRayNearFar(_e327);
    shadowRayNearFar = _e328;
    let _e330 = intersections_4;
    let _e331 = getHazeRayNearFar(_e330);
    hazeRayNearFar = _e331;
    let _e334 = rayDirection_8;
    let _e337 = getRayDistanceToScene(_e334, (&sceneViewZ));
    rayDistanceToScene = _e337;
    let _e339 = rayDistanceToScene;
    if (_e339 > 0f) {
        {
            let _e343 = rayNearFar_2;
            let _e345 = rayDistanceToScene;
            rayNearFar_2.y = min(_e343.y, _e345);
            let _e348 = shadowRayNearFar;
            let _e350 = rayDistanceToScene;
            shadowRayNearFar.y = min(_e348.y, _e350);
            let _e353 = hazeRayNearFar;
            let _e355 = rayDistanceToScene;
            hazeRayNearFar.y = min(_e353.y, _e355);
        }
    }
    let _e357 = rayNearFar_2;
    intersectsGround = any((_e357 < vec2(0f)));
    let _e363 = rayNearFar_2;
    let _e365 = rayNearFar_2;
    intersectsScene = (_e363.y < _e365.x);
    let _e369 = getSTBN();
    stbn = _e369;
    let _e374 = rayNearFar_2;
    frontDepth_1 = _e374.y;
    let _e384 = intersectsGround;
    let _e386 = intersectsScene;
    if (!(_e384) && !(_e386)) {
        {
            let _e389 = rayNearFar_2;
            let _e391 = rayDirection_8;
            let _e393 = cameraPosition_3;
            rayOrigin_5 = ((_e389.x * _e391) + _e393);
            let _e396 = rayOrigin_5;
            let _e397 = getGlobeUv(_e396);
            globeUv = _e397;
            let _e399 = globeUv;
            let _e400 = global.localWeatherRepeat;
            let _e402 = getMipLevel((_e399 * _e400));
            let _e403 = global.mipLevelScale;
            mipLevel_7 = (_e402 * _e403);
            let _e407 = mipLevel_7;
            let _e410 = global.cameraHeight;
            let _e412 = global.maxHeight;
            mipLevel_7 = mix(0f, _e407, min(1f, ((0.2f * _e410) / _e412)));
            let _e420 = rayOrigin_5;
            let _e421 = rayDirection_8;
            let _e422 = rayNearFar_2;
            let _e423 = cosTheta_6;
            let _e424 = stbn;
            let _e426 = mipLevel_7;
            let _e427 = pt_pow(2f, _e426);
            let _e432 = marchClouds(_e420, _e421, _e422, _e423, _e424, _e427, (&marchedFrontDepth), (&sampleCount_4));
            color_1 = _e432;
            let _e433 = marchedFrontDepth;
            hitClouds = (_e433 >= 0f);
            let _e436 = hitClouds;
            if _e436 {
                {
                    let _e437 = rayNearFar_2;
                    let _e439 = marchedFrontDepth;
                    frontDepth_1 = (_e437.x + _e439);
                    let _e442 = shadowRayNearFar;
                    let _e444 = frontDepth_1;
                    let _e445 = shadowRayNearFar;
                    let _e448 = color_1;
                    shadowRayNearFar.y = mix(_e442.y, min(_e444, _e445.y), _e448.w);
                    let _e451 = shadowRayNearFar;
                    if all((_e451 >= vec2(0f))) {
                        {
                            let _e456 = shadowRayNearFar;
                            let _e458 = rayDirection_8;
                            let _e460 = cameraPosition_3;
                            let _e462 = rayDirection_8;
                            let _e463 = shadowRayNearFar;
                            let _e464 = stbn;
                            let _e465 = marchShadowLength(((_e456.x * _e458) + _e460), _e462, _e463, _e464);
                            shadowLength_3 = _e465;
                        }
                    }
                    let _e467 = hazeRayNearFar;
                    let _e469 = frontDepth_1;
                    let _e470 = hazeRayNearFar;
                    let _e473 = color_1;
                    hazeRayNearFar.y = mix(_e467.y, min(_e469, _e470.y), _e473.w);
                    let _e476 = cameraPosition_3;
                    let _e477 = frontDepth_1;
                    let _e478 = rayDirection_8;
                    frontPosition_1 = (_e476 + (_e477 * _e478));
                    let _e482 = cameraPosition_3;
                    let _e483 = frontPosition_1;
                    let _e484 = shadowLength_3;
                    applyAerialPerspective(_e482, _e483, _e484, (&color_1));
                    let _e487 = frontPosition_1;
                    let _e488 = ecefToWorld(_e487);
                    frontPositionWorld = _e488;
                    let _e490 = global.reprojectionMatrix;
                    let _e491 = frontPositionWorld;
                    prevClip = (_e490 * vec4<f32>(_e491.x, _e491.y, _e491.z, 1f));
                    let _e499 = prevClip;
                    let _e500 = prevClip;
                    prevClip = (_e499 / vec4(_e500.w));
                    let _e504 = prevClip;
                    prevUv = ((_e504.xy * 0.5f) + vec2(0.5f));
                    let _e512 = vUv_1;
                    let _e513 = prevUv;
                    velocity = (_e512 - _e513);
                    let _e516 = frontDepth_1;
                    let _e517 = velocity;
                    depthVelocity = vec3<f32>(_e516, _e517.x, _e517.y);
                }
            }
        }
    }
    let _e521 = hitClouds;
    if !(_e521) {
        {
            let _e523 = shadowRayNearFar;
            if all((_e523 >= vec2(0f))) {
                {
                    let _e528 = shadowRayNearFar;
                    let _e530 = rayDirection_8;
                    let _e532 = cameraPosition_3;
                    let _e534 = rayDirection_8;
                    let _e535 = shadowRayNearFar;
                    let _e536 = stbn;
                    let _e537 = marchShadowLength(((_e528.x * _e530) + _e532), _e534, _e535, _e536);
                    shadowLength_3 = _e537;
                }
            }
            let _e538 = sceneViewZ;
            if (_e538 < 0f) {
                let _e541 = sceneViewZ;
                local_16 = -(_e541);
            } else {
                let _e543 = global.cameraFar;
                local_16 = _e543;
            }
            let _e545 = local_16;
            frontDepth_1 = _e545;
            let _e546 = vViewPosition_1;
            let _e547 = frontDepth_1;
            frontView = (_e546 * _e547);
            let _e550 = global.viewReprojectionMatrix;
            let _e551 = frontView;
            prevClip_1 = (_e550 * vec4<f32>(_e551.x, _e551.y, _e551.z, 1f));
            let _e559 = prevClip_1;
            let _e560 = prevClip_1;
            prevClip_1 = (_e559 / vec4(_e560.w));
            let _e564 = prevClip_1;
            prevUv_1 = ((_e564.xy * 0.5f) + vec2(0.5f));
            let _e572 = vUv_1;
            let _e573 = prevUv_1;
            velocity_1 = (_e572 - _e573);
            let _e576 = frontDepth_1;
            let _e577 = velocity_1;
            depthVelocity = vec3<f32>(_e576, _e577.x, _e577.y);
        }
    }
    let _e581 = global.cameraNear;
    let _e582 = rayDirection_8;
    let _e584 = cameraPosition_3;
    let _e586 = rayDirection_8;
    let _e587 = hazeRayNearFar;
    let _e589 = hazeRayNearFar;
    let _e592 = cosTheta_6;
    let _e593 = shadowLength_3;
    let _e594 = approximateHaze(((_e581 * _e582) + _e584), _e586, (_e587.y - _e589.x), _e592, _e593);
    haze = _e594;
    let _e596 = color_1;
    let _e598 = color_1;
    let _e600 = haze;
    let _e602 = haze;
    let _e605 = mix(_e598.xyz, _e600.xyz, vec3(_e602.w));
    color_1.x = _e605.x;
    color_1.y = _e605.y;
    color_1.z = _e605.z;
    let _e613 = color_1;
    let _e616 = haze;
    let _e620 = haze;
    color_1.w = ((_e613.w * (1f - _e616.w)) + _e620.w);
    let _e623 = color_1;
    outputColor = _e623;
    let _e624 = depthVelocity;
    outputDepthVelocity = _e624;
    let _e625 = shadowLength_3;
    outputShadowLength = (_e625 * 0.001f);
    return;
}

fn main_1() {
    let _e313 = global.pt_u_shadowIntervals[0];
    shadowIntervals[0i] = _e313.xy;
    let _e319 = global.pt_u_shadowIntervals[1];
    shadowIntervals[1i] = _e319.xy;
    let _e322 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e322;
    let _e324 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e324;
    let _e326 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e326;
    let _e328 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e328;
    let _e334 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e334;
    let _e340 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e340;
    let _e346 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e346;
    let _e352 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e352;
    let _e358 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e358;
    let _e364 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e364;
    let _e370 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e370;
    let _e376 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e376;
    let _e382 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e382;
    let _e388 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e388;
    let _e390 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e390;
    let _e396 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e396;
    let _e402 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e402;
    let _e408 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e408;
    let _e414 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e414;
    let _e420 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e420;
    let _e426 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e426;
    let _e432 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e432;
    let _e438 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e438;
    let _e444 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e444;
    let _e450 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e450;
    let _e452 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e452;
    let _e454 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e454;
    let _e456 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e456;
    let _e462 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e462;
    let _e468 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e468;
    let _e474 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e474;
    let _e480 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e480;
    let _e486 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e486;
    let _e492 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e492;
    let _e498 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e498;
    let _e504 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e504;
    let _e510 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e510;
    let _e516 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e516;
    let _e518 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e518;
    let _e520 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e520;
    let _e522 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e522;
    let _e524 = global.pt_s_densityProfile_expTerms;
    densityProfile.expTerms = _e524;
    let _e526 = global.pt_s_densityProfile_exponents;
    densityProfile.exponents = _e526;
    let _e528 = global.pt_s_densityProfile_linearTerms;
    densityProfile.linearTerms = _e528;
    let _e530 = global.pt_s_densityProfile_constantTerms;
    densityProfile.constantTerms = _e530;
    pt_main();
    let _e531 = outputDepthVelocity;
    pt_out_outputDepthVelocity = vec4<f32>(_e531.x, _e531.y, _e531.z, 1f);
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vCameraPosition: vec3<f32>, @location(2) vCameraDirection: vec3<f32>, @location(3) vRayDirection: vec3<f32>, @location(4) vViewPosition: vec3<f32>, @location(5) sun: vec3<f32>, @location(6) sky: vec3<f32>, @location(7) minSun: vec3<f32>, @location(8) minSky: vec3<f32>, @location(9) maxSun: vec3<f32>, @location(10) maxSky: vec3<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vCameraPosition_1 = vCameraPosition;
    vCameraDirection_1 = vCameraDirection;
    vRayDirection_1 = vRayDirection;
    vViewPosition_1 = vViewPosition;
    vGroundIrradiance.sun = sun;
    vGroundIrradiance.sky = sky;
    vCloudsIrradiance.minSun = minSun;
    vCloudsIrradiance.minSky = minSky;
    vCloudsIrradiance.maxSun = maxSun;
    vCloudsIrradiance.maxSky = maxSky;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e403 = outputColor;
    let _e405 = pt_out_outputDepthVelocity;
    let _e407 = outputShadowLength;
    return FragmentOutput(_e403, _e405, _e407);
}
