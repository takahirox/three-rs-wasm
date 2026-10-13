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

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @location(1) vCameraPosition: vec3<f32>,
    @location(2) vCameraDirection: vec3<f32>,
    @location(3) vRayDirection: vec3<f32>,
    @location(4) vViewPosition: vec3<f32>,
    @location(5) sun: vec3<f32>,
    @location(6) sky: vec3<f32>,
    @location(7) minSun: vec3<f32>,
    @location(8) minSky: vec3<f32>,
    @location(9) maxSun: vec3<f32>,
    @location(10) maxSky: vec3<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

const m: f32 = 1f;
const nm: f32 = 1f;
const rad: f32 = 1f;
const sr: f32 = 1f;
const watt: f32 = 1f;
const lm: f32 = 1f;
const km: f32 = 1000f;
const m2_: f32 = 1f;
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
var depthBuffer_t: texture_2d<f32>;
@group(0) @binding(22) 
var depthBuffer_s: sampler;
@group(0) @binding(23) 
var shadowBuffer_t: texture_2d_array<f32>;
@group(0) @binding(24) 
var shadowBuffer_s: sampler;
var<private> shadowIntervals: array<vec2<f32>, 2>;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> densityProfile: CloudDensityProfile;
var<private> vUv: vec2<f32>;
var<private> vCameraPosition: vec3<f32>;
var<private> vCameraDirection: vec3<f32>;
var<private> vRayDirection: vec3<f32>;
var<private> vViewPosition: vec3<f32>;
var<private> vGroundIrradiance: GroundIrradiance;
var<private> vCloudsIrradiance: CloudsIrradiance;
var<private> position_1: vec3<f32>;
var<private> gl_Position: vec4<f32>;

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

fn ClampCosine(mu: f32) -> f32 {
    return clamp(mu, -1f, 1f);
}

fn ClampDistance(d: f32) -> f32 {
    return max(d, 0f);
}

fn ClampRadius(atmosphere: AtmosphereParameters, r: f32) -> f32 {
    return clamp(r, atmosphere.bottom_radius, atmosphere.top_radius);
}

fn SafeSqrt(a: f32) -> f32 {
    return sqrt(max(a, 0f));
}

fn DistanceToTopAtmosphereBoundary(atmosphere_1: AtmosphereParameters, r_1: f32, mu_1: f32) -> f32 {
    var discriminant: f32;

    discriminant = (((r_1 * r_1) * ((mu_1 * mu_1) - 1f)) + (atmosphere_1.top_radius * atmosphere_1.top_radius));
    let _e309 = discriminant;
    let _e310 = SafeSqrt(_e309);
    let _e312 = ClampDistance(((-(r_1) * mu_1) + _e310));
    return _e312;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_2: f32, mu_2: f32) -> f32 {
    var discriminant_1: f32;

    discriminant_1 = (((r_2 * r_2) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e309 = discriminant_1;
    let _e310 = SafeSqrt(_e309);
    let _e312 = ClampDistance(((-(r_2) * mu_2) - _e310));
    return _e312;
}

fn RayIntersectsGround(atmosphere_3: AtmosphereParameters, r_3: f32, mu_3: f32) -> bool {
    return ((mu_3 < 0f) && ((((r_3 * r_3) * ((mu_3 * mu_3) - 1f)) + (atmosphere_3.bottom_radius * atmosphere_3.bottom_radius)) >= 0f));
}

fn GetTextureCoordFromUnitRange(x_16: f32, texture_size: i32) -> f32 {
    return ((0.5f / f32(texture_size)) + (x_16 * (1f - (1f / f32(texture_size)))));
}

fn GetTransmittanceTextureUvFromRMu(atmosphere_4: AtmosphereParameters, r_4: f32, mu_4: f32) -> vec2<f32> {
    var H: f32;
    var rho: f32;
    var d_1: f32;
    var d_min: f32;
    var d_max: f32;
    var x_mu: f32;
    var x_r: f32;

    H = sqrt(((atmosphere_4.top_radius * atmosphere_4.top_radius) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    let _e311 = SafeSqrt(((r_4 * r_4) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e311;
    let _e313 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_4, mu_4);
    d_1 = _e313;
    d_min = (atmosphere_4.top_radius - r_4);
    let _e318 = rho;
    let _e319 = H;
    d_max = (_e318 + _e319);
    let _e322 = d_1;
    let _e323 = d_min;
    let _e325 = d_max;
    let _e326 = d_min;
    x_mu = ((_e322 - _e323) / (_e325 - _e326));
    let _e330 = rho;
    let _e331 = H;
    x_r = (_e330 / _e331);
    let _e334 = x_mu;
    let _e336 = GetTextureCoordFromUnitRange(_e334, 256i);
    let _e337 = x_r;
    let _e339 = GetTextureCoordFromUnitRange(_e337, 64i);
    return vec2<f32>(_e336, _e339);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_5: f32, mu_5: f32) -> vec3<f32> {
    var uv: vec2<f32>;

    let _e299 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_5, mu_5);
    uv = _e299;
    let _e301 = uv;
    let _e303 = textureSampleLevel(transmittance_texture_t, transmittance_texture_s, _e301, 0f);
    return vec3<f32>(_e303.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_6: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e310 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_6) * mu_6) * d_2)) + (r_6 * r_6))));
    r_d = _e310;
    let _e314 = r_d;
    let _e316 = ClampCosine((((r_6 * mu_6) + d_2) / _e314));
    mu_d = _e316;
    if ray_r_mu_intersects_ground {
        {
            let _e318 = r_d;
            let _e319 = mu_d;
            let _e321 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e318, -(_e319));
            let _e323 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, -(mu_6));
            return min((_e321 / _e323), vec3(1f));
        }
    } else {
        {
            let _e328 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, mu_6);
            let _e329 = r_d;
            let _e330 = mu_d;
            let _e331 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e329, _e330);
            return min((_e328 / _e331), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_7: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_7);
    let _e303 = sin_theta_h;
    let _e304 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e303 * _e304)), 0f)));
    let _e312 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_7, mu_s);
    let _e313 = sin_theta_h;
    let _e318 = sin_theta_h;
    let _e322 = cos_theta_h;
    return (_e312 * smoothstep(((-(_e313) * atmosphere_7.sun_angular_radius) / rad), ((_e318 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e322)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e303 = k;
    return (_e303 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e312 = k_1;
    let _e325 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e312 * (1f + (nu_1 * nu_1))) / _e325);
}

fn GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_8: AtmosphereParameters, r_8: f32, mu_7: f32, mu_s_1: f32, nu_2: f32, ray_r_mu_intersects_ground_1: bool) -> vec4<f32> {
    var H_1: f32;
    var rho_1: f32;
    var u_r: f32;
    var r_mu: f32;
    var discriminant_2: f32;
    var u_mu: f32;
    var d_3: f32;
    var d_min_1: f32;
    var d_max_1: f32;
    var local: f32;
    var d_4: f32;
    var d_min_2: f32;
    var d_max_2: f32;
    var d_5: f32;
    var d_min_3: f32;
    var d_max_3: f32;
    var a_1: f32;
    var D: f32;
    var A: f32;
    var u_mu_s: f32;
    var u_nu: f32;

    H_1 = sqrt(((atmosphere_8.top_radius * atmosphere_8.top_radius) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    let _e314 = SafeSqrt(((r_8 * r_8) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e314;
    let _e316 = rho_1;
    let _e317 = H_1;
    let _e320 = GetTextureCoordFromUnitRange((_e316 / _e317), 32i);
    u_r = _e320;
    r_mu = (r_8 * mu_7);
    let _e324 = r_mu;
    let _e325 = r_mu;
    discriminant_2 = (((_e324 * _e325) - (r_8 * r_8)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e335 = r_mu;
            let _e337 = discriminant_2;
            let _e338 = SafeSqrt(_e337);
            d_3 = (-(_e335) - _e338);
            d_min_1 = (r_8 - atmosphere_8.bottom_radius);
            let _e344 = rho_1;
            d_max_1 = _e344;
            let _e348 = d_max_1;
            let _e349 = d_min_1;
            if (_e348 == _e349) {
                local = 0f;
            } else {
                let _e352 = d_3;
                let _e353 = d_min_1;
                let _e355 = d_max_1;
                let _e356 = d_min_1;
                local = ((_e352 - _e353) / (_e355 - _e356));
            }
            let _e360 = local;
            let _e364 = GetTextureCoordFromUnitRange(_e360, 64i);
            u_mu = (0.5f - (0.5f * _e364));
        }
    } else {
        {
            let _e367 = r_mu;
            let _e369 = discriminant_2;
            let _e370 = H_1;
            let _e371 = H_1;
            let _e374 = SafeSqrt((_e369 + (_e370 * _e371)));
            d_4 = (-(_e367) + _e374);
            d_min_2 = (atmosphere_8.top_radius - r_8);
            let _e380 = rho_1;
            let _e381 = H_1;
            d_max_2 = (_e380 + _e381);
            let _e386 = d_4;
            let _e387 = d_min_2;
            let _e389 = d_max_2;
            let _e390 = d_min_2;
            let _e396 = GetTextureCoordFromUnitRange(((_e386 - _e387) / (_e389 - _e390)), 64i);
            u_mu = (0.5f + (0.5f * _e396));
        }
    }
    let _e400 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e400;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e406 = H_1;
    d_max_3 = _e406;
    let _e408 = d_5;
    let _e409 = d_min_3;
    let _e411 = d_max_3;
    let _e412 = d_min_3;
    a_1 = ((_e408 - _e409) / (_e411 - _e412));
    let _e418 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e418;
    let _e420 = D;
    let _e421 = d_min_3;
    let _e423 = d_max_3;
    let _e424 = d_min_3;
    A = ((_e420 - _e421) / (_e423 - _e424));
    let _e429 = a_1;
    let _e430 = A;
    let _e436 = a_1;
    let _e440 = GetTextureCoordFromUnitRange((max((1f - (_e429 / _e430)), 0f) / (1f + _e436)), 32i);
    u_mu_s = _e440;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e447 = u_nu;
    let _e448 = u_mu_s;
    let _e449 = u_mu;
    let _e450 = u_r;
    return vec4<f32>(_e447, _e448, _e449, _e450);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_9: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_9 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e309 = x_mu_s;
    let _e311 = GetTextureCoordFromUnitRange(_e309, 64i);
    let _e312 = x_r_1;
    let _e314 = GetTextureCoordFromUnitRange(_e312, 16i);
    return vec2<f32>(_e311, _e314);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_10: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_1: vec2<f32>;

    let _e299 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_10, mu_s_3);
    uv_1 = _e299;
    let _e301 = uv_1;
    let _e303 = textureSampleLevel(irradiance_texture_t, irradiance_texture_s, _e301, 0f);
    return vec3<f32>(_e303.xyz);
}

fn GetExtrapolatedSingleMieScattering(atmosphere_11: AtmosphereParameters, scattering: vec4<f32>) -> vec3<f32> {
    if (scattering.x < 0.00001f) {
        {
            return vec3(0f);
        }
    }
    return ((((scattering.xyz * scattering.w) / vec3(scattering.x)) * (atmosphere_11.rayleigh_scattering.x / atmosphere_11.mie_scattering.x)) * (atmosphere_11.mie_scattering / atmosphere_11.rayleigh_scattering));
}

fn GetCombinedScattering(atmosphere_12: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, r_11: f32, mu_8: f32, mu_s_4: f32, nu_3: f32, ray_r_mu_intersects_ground_2: bool, single_mie_scattering: ptr<function, vec3<f32>>) -> vec3<f32> {
    var uvwz: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;
    var combined_scattering: vec4<f32>;
    var scattering_1: vec3<f32>;

    let _e305 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_12, r_11, mu_8, mu_s_4, nu_3, ray_r_mu_intersects_ground_2);
    uvwz = _e305;
    let _e307 = uvwz;
    tex_coord_x = (_e307.x * 7f);
    let _e315 = tex_coord_x;
    tex_x = floor(_e315);
    let _e318 = tex_coord_x;
    let _e319 = tex_x;
    lerp = (_e318 - _e319);
    let _e322 = tex_x;
    let _e323 = uvwz;
    let _e329 = uvwz;
    let _e331 = uvwz;
    uvw0_ = vec3<f32>(((_e322 + _e323.y) / 8f), _e329.z, _e331.w);
    let _e335 = tex_x;
    let _e338 = uvwz;
    let _e344 = uvwz;
    let _e346 = uvwz;
    uvw1_ = vec3<f32>((((_e335 + 1f) + _e338.y) / 8f), _e344.z, _e346.w);
    let _e350 = uvw0_;
    let _e352 = textureSampleLevel(scattering_texture_t, scattering_texture_s, _e350, 0f);
    let _e354 = lerp;
    let _e357 = uvw1_;
    let _e359 = textureSampleLevel(scattering_texture_t, scattering_texture_s, _e357, 0f);
    let _e360 = lerp;
    combined_scattering = ((_e352 * (1f - _e354)) + (_e359 * _e360));
    let _e364 = combined_scattering;
    scattering_1 = vec3<f32>(_e364.xyz);
    let _e368 = combined_scattering;
    let _e369 = GetExtrapolatedSingleMieScattering(atmosphere_12, _e368);
    (*single_mie_scattering) = _e369;
    let _e370 = scattering_1;
    return _e370;
}

fn GetScattering(atmosphere_13: AtmosphereParameters, scattering_texture_t_1: texture_3d<f32>, scattering_texture_s_1: sampler, r_12: f32, mu_9: f32, mu_s_5: f32, nu_4: f32, ray_r_mu_intersects_ground_3: bool) -> vec3<f32> {
    var uvwz_1: vec4<f32>;
    var tex_coord_x_1: f32;
    var tex_x_1: f32;
    var lerp_1: f32;
    var uvw0_1: vec3<f32>;
    var uvw1_1: vec3<f32>;
    var scattering_2: vec3<f32>;

    let _e302 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_13, r_12, mu_9, mu_s_5, nu_4, ray_r_mu_intersects_ground_3);
    uvwz_1 = _e302;
    let _e304 = uvwz_1;
    tex_coord_x_1 = (_e304.x * 7f);
    let _e312 = tex_coord_x_1;
    tex_x_1 = floor(_e312);
    let _e315 = tex_coord_x_1;
    let _e316 = tex_x_1;
    lerp_1 = (_e315 - _e316);
    let _e319 = tex_x_1;
    let _e320 = uvwz_1;
    let _e326 = uvwz_1;
    let _e328 = uvwz_1;
    uvw0_1 = vec3<f32>(((_e319 + _e320.y) / 8f), _e326.z, _e328.w);
    let _e332 = tex_x_1;
    let _e335 = uvwz_1;
    let _e341 = uvwz_1;
    let _e343 = uvwz_1;
    uvw1_1 = vec3<f32>((((_e332 + 1f) + _e335.y) / 8f), _e341.z, _e343.w);
    let _e347 = uvw0_1;
    let _e349 = textureSampleLevel(scattering_texture_t_1, scattering_texture_s_1, _e347, 0f);
    let _e351 = lerp_1;
    let _e354 = uvw1_1;
    let _e356 = textureSampleLevel(scattering_texture_t_1, scattering_texture_s_1, _e354, 0f);
    let _e357 = lerp_1;
    scattering_2 = vec3<f32>(((_e349 * (1f - _e351)) + (_e356 * _e357)).xyz);
    let _e363 = scattering_2;
    return _e363;
}

fn GetSkyRadiance(atmosphere_14: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, scattering_texture_t_2: texture_3d<f32>, scattering_texture_s_2: sampler, single_mie_scattering_texture_t_1: texture_3d<f32>, single_mie_scattering_texture_s_1: sampler, camera: vec3<f32>, view_ray: vec3<f32>, shadow_length: f32, sun_direction: vec3<f32>, transmittance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_1: vec3<f32>;
    var r_13: f32;
    var rmu: f32;
    var distance_to_top_atmosphere_boundary: f32;
    var mu_10: f32;
    var mu_s_6: f32;
    var nu_5: f32;
    var ray_r_mu_intersects_ground_4: bool = false;
    var local_1: vec3<f32>;
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
    let _e307 = camera_1;
    r_13 = length(_e307);
    let _e310 = camera_1;
    rmu = dot(_e310, view_ray);
    let _e313 = rmu;
    let _e315 = rmu;
    let _e316 = rmu;
    let _e318 = r_13;
    let _e319 = r_13;
    let _e326 = SafeSqrt((((_e315 * _e316) - (_e318 * _e319)) + (atmosphere_14.top_radius * atmosphere_14.top_radius)));
    distance_to_top_atmosphere_boundary = (-(_e313) - _e326);
    let _e329 = distance_to_top_atmosphere_boundary;
    if (_e329 > 0f) {
        {
            let _e334 = camera_1;
            let _e335 = distance_to_top_atmosphere_boundary;
            camera_1 = (_e334 + (view_ray * _e335));
            r_13 = atmosphere_14.top_radius;
            let _e339 = rmu;
            let _e340 = distance_to_top_atmosphere_boundary;
            rmu = (_e339 + _e340);
        }
    } else {
        let _e342 = r_13;
        if (_e342 > atmosphere_14.top_radius) {
            {
                (*transmittance) = vec3(1f);
                return vec3(0f);
            }
        }
    }
    let _e351 = rmu;
    let _e352 = r_13;
    mu_10 = (_e351 / _e352);
    let _e355 = camera_1;
    let _e357 = r_13;
    mu_s_6 = (dot(_e355, sun_direction) / _e357);
    nu_5 = dot(view_ray, sun_direction);
    let _e364 = ray_r_mu_intersects_ground_4;
    if _e364 {
        local_1 = vec3(0f);
    } else {
        let _e367 = r_13;
        let _e368 = mu_10;
        let _e369 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_14, transmittance_texture_t_3, transmittance_texture_s_3, _e367, _e368);
        local_1 = _e369;
    }
    let _e371 = local_1;
    (*transmittance) = _e371;
    if (shadow_length == 0f) {
        {
            let _e378 = r_13;
            let _e379 = mu_10;
            let _e380 = mu_s_6;
            let _e381 = nu_5;
            let _e382 = ray_r_mu_intersects_ground_4;
            let _e385 = GetCombinedScattering(atmosphere_14, scattering_texture_t_2, scattering_texture_s_2, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, _e378, _e379, _e380, _e381, _e382, (&single_mie_scattering_1));
            scattering_3 = _e385;
        }
    } else {
        {
            d_6 = shadow_length;
            let _e387 = d_6;
            let _e388 = d_6;
            let _e391 = r_13;
            let _e393 = mu_10;
            let _e395 = d_6;
            let _e398 = r_13;
            let _e399 = r_13;
            let _e403 = ClampRadius(atmosphere_14, sqrt((((_e387 * _e388) + (((2f * _e391) * _e393) * _e395)) + (_e398 * _e399))));
            r_p = _e403;
            let _e405 = r_13;
            let _e406 = mu_10;
            let _e408 = d_6;
            let _e410 = r_p;
            mu_p = (((_e405 * _e406) + _e408) / _e410);
            let _e413 = r_13;
            let _e414 = mu_s_6;
            let _e416 = d_6;
            let _e417 = nu_5;
            let _e420 = r_p;
            mu_s_p = (((_e413 * _e414) + (_e416 * _e417)) / _e420);
            let _e423 = r_p;
            let _e424 = mu_p;
            let _e425 = mu_s_p;
            let _e426 = nu_5;
            let _e427 = ray_r_mu_intersects_ground_4;
            let _e430 = GetCombinedScattering(atmosphere_14, scattering_texture_t_2, scattering_texture_s_2, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, _e423, _e424, _e425, _e426, _e427, (&single_mie_scattering_1));
            scattering_3 = _e430;
            let _e431 = r_13;
            let _e432 = mu_10;
            let _e433 = ray_r_mu_intersects_ground_4;
            let _e434 = GetTransmittance(atmosphere_14, transmittance_texture_t_3, transmittance_texture_s_3, _e431, _e432, shadow_length, _e433);
            shadow_transmittance = _e434;
            let _e436 = r_p;
            let _e437 = mu_p;
            let _e438 = mu_s_p;
            let _e439 = nu_5;
            let _e440 = ray_r_mu_intersects_ground_4;
            let _e441 = GetScattering(atmosphere_14, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e436, _e437, _e438, _e439, _e440);
            higher_order_scattering = _e441;
            let _e443 = scattering_3;
            let _e444 = higher_order_scattering;
            single_scattering = (_e443 - _e444);
            let _e447 = single_scattering;
            let _e448 = shadow_transmittance;
            let _e450 = higher_order_scattering;
            scattering_3 = ((_e447 * _e448) + _e450);
            let _e452 = single_mie_scattering_1;
            let _e453 = shadow_transmittance;
            single_mie_scattering_1 = (_e452 * _e453);
        }
    }
    let _e455 = scattering_3;
    let _e456 = nu_5;
    let _e457 = RayleighPhaseFunction(_e456);
    let _e459 = single_mie_scattering_1;
    let _e461 = nu_5;
    let _e462 = MiePhaseFunction(atmosphere_14.mie_phase_function_g, _e461);
    return ((_e455 * _e457) + (_e459 * _e462));
}

fn ClosestPointOnRay(camera_2: vec3<f32>, point: vec3<f32>) -> vec3<f32> {
    var ray: vec3<f32>;
    var t: f32;

    ray = (point - camera_2);
    let _e298 = ray;
    let _e301 = ray;
    let _e302 = ray;
    t = clamp((-(dot(camera_2, _e298)) / dot(_e301, _e302)), 0f, 1f);
    let _e309 = t;
    let _e310 = ray;
    return (camera_2 + (_e309 * _e310));
}

fn RaySphereIntersections(camera_3: vec3<f32>, direction: vec3<f32>, radius: f32) -> vec2<f32> {
    var b_8: f32;
    var c: f32;
    var discriminant_3: f32;
    var Q: f32;

    b_8 = (2f * dot(direction, camera_3));
    c = (dot(camera_3, camera_3) - (radius * radius));
    let _e305 = b_8;
    let _e306 = b_8;
    let _e309 = c;
    discriminant_3 = ((_e305 * _e306) - (4f * _e309));
    let _e313 = discriminant_3;
    Q = sqrt(_e313);
    let _e316 = b_8;
    let _e318 = Q;
    let _e320 = b_8;
    let _e322 = Q;
    return (vec2<f32>((-(_e316) - _e318), (-(_e320) + _e322)) * 0.5f);
}

fn ClipAtBottomAtmosphere(atmosphere_15: AtmosphereParameters, view_ray_1: vec3<f32>, camera_4: ptr<function, vec3<f32>>, point_1: ptr<function, vec3<f32>>) -> bool {
    var eps: f32 = 0f;
    var bottom_radius: f32;
    var r_camera: f32;
    var r_point: f32;
    var camera_below: bool;
    var point_below: bool;
    var t_1: vec2<f32>;
    var local_2: f32;
    var intersection: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;

    let _e301 = eps;
    bottom_radius = (atmosphere_15.bottom_radius + _e301);
    let _e304 = (*camera_4);
    r_camera = length(_e304);
    let _e307 = (*point_1);
    r_point = length(_e307);
    let _e310 = r_camera;
    let _e311 = bottom_radius;
    camera_below = (_e310 < _e311);
    let _e314 = r_point;
    let _e315 = bottom_radius;
    point_below = (_e314 < _e315);
    let _e318 = (*camera_4);
    let _e319 = bottom_radius;
    let _e320 = RaySphereIntersections(_e318, view_ray_1, _e319);
    t_1 = _e320;
    let _e322 = (*camera_4);
    let _e323 = camera_below;
    if _e323 {
        let _e324 = t_1;
        local_2 = _e324.y;
    } else {
        let _e326 = t_1;
        local_2 = _e326.x;
    }
    let _e329 = local_2;
    intersection = (_e322 + (view_ray_1 * _e329));
    let _e333 = camera_below;
    if _e333 {
        let _e334 = intersection;
        local_3 = _e334;
    } else {
        let _e335 = (*camera_4);
        local_3 = _e335;
    }
    let _e337 = local_3;
    (*camera_4) = _e337;
    let _e338 = point_below;
    if _e338 {
        let _e339 = intersection;
        local_4 = _e339;
    } else {
        let _e340 = (*point_1);
        local_4 = _e340;
    }
    let _e342 = local_4;
    (*point_1) = _e342;
    let _e343 = camera_below;
    let _e344 = point_below;
    return (_e343 && _e344);
}

fn GetSkyRadianceToPoint(atmosphere_16: AtmosphereParameters, transmittance_texture_t_4: texture_2d<f32>, transmittance_texture_s_4: sampler, scattering_texture_t_3: texture_3d<f32>, scattering_texture_s_3: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, camera_5: vec3<f32>, point_2: vec3<f32>, shadow_length_1: f32, sun_direction_1: vec3<f32>, transmittance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_6: vec3<f32>;
    var point_3: vec3<f32>;
    var view_ray_2: vec3<f32>;
    var r_14: f32;
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
    let _e308 = camera_6;
    let _e309 = point_3;
    let _e310 = ClosestPointOnRay(_e308, _e309);
    if (length(_e310) > atmosphere_16.top_radius) {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e318 = point_3;
    let _e319 = camera_6;
    view_ray_2 = normalize((_e318 - _e319));
    let _e323 = view_ray_2;
    let _e328 = ClipAtBottomAtmosphere(atmosphere_16, _e323, (&camera_6), (&point_3));
    if _e328 {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e333 = camera_6;
    r_14 = length(_e333);
    let _e336 = camera_6;
    let _e337 = view_ray_2;
    rmu_1 = dot(_e336, _e337);
    let _e340 = rmu_1;
    let _e342 = rmu_1;
    let _e343 = rmu_1;
    let _e345 = r_14;
    let _e346 = r_14;
    let _e353 = SafeSqrt((((_e342 * _e343) - (_e345 * _e346)) + (atmosphere_16.top_radius * atmosphere_16.top_radius)));
    distance_to_top_atmosphere_boundary_1 = (-(_e340) - _e353);
    let _e356 = distance_to_top_atmosphere_boundary_1;
    if (_e356 > 0f) {
        {
            let _e361 = camera_6;
            let _e362 = view_ray_2;
            let _e363 = distance_to_top_atmosphere_boundary_1;
            camera_6 = (_e361 + (_e362 * _e363));
            r_14 = atmosphere_16.top_radius;
            let _e367 = rmu_1;
            let _e368 = distance_to_top_atmosphere_boundary_1;
            rmu_1 = (_e367 + _e368);
        }
    }
    let _e370 = rmu_1;
    let _e371 = r_14;
    mu_11 = (_e370 / _e371);
    let _e374 = camera_6;
    let _e376 = r_14;
    mu_s_7 = (dot(_e374, sun_direction_1) / _e376);
    let _e379 = view_ray_2;
    nu_6 = dot(_e379, sun_direction_1);
    let _e382 = point_3;
    let _e383 = camera_6;
    d_7 = length((_e382 - _e383));
    let _e387 = r_14;
    let _e388 = mu_11;
    let _e389 = RayIntersectsGround(atmosphere_16, _e387, _e388);
    ray_r_mu_intersects_ground_5 = _e389;
    let _e391 = ray_r_mu_intersects_ground_5;
    if !(_e391) {
        {
            let _e397 = r_14;
            let _e398 = r_14;
            let _e402 = SafeSqrt((1f - ((atmosphere_16.bottom_radius * atmosphere_16.bottom_radius) / (_e397 * _e398))));
            mu_horizon = -(_e402);
            let _e407 = mu_11;
            let _e408 = mu_horizon;
            let _e409 = eps_1;
            mu_11 = max(_e407, (_e408 + _e409));
        }
    }
    let _e412 = r_14;
    let _e413 = mu_11;
    let _e414 = d_7;
    let _e415 = ray_r_mu_intersects_ground_5;
    let _e416 = GetTransmittance(atmosphere_16, transmittance_texture_t_4, transmittance_texture_s_4, _e412, _e413, _e414, _e415);
    (*transmittance_1) = _e416;
    let _e418 = r_14;
    let _e419 = mu_11;
    let _e420 = mu_s_7;
    let _e421 = nu_6;
    let _e422 = ray_r_mu_intersects_ground_5;
    let _e425 = GetCombinedScattering(atmosphere_16, scattering_texture_t_3, scattering_texture_s_3, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, _e418, _e419, _e420, _e421, _e422, (&single_mie_scattering_2));
    scattering_4 = _e425;
    let _e427 = d_7;
    d_7 = max((_e427 - shadow_length_1), 0f);
    let _e433 = d_7;
    let _e434 = d_7;
    let _e437 = r_14;
    let _e439 = mu_11;
    let _e441 = d_7;
    let _e444 = r_14;
    let _e445 = r_14;
    let _e449 = ClampRadius(atmosphere_16, sqrt((((_e433 * _e434) + (((2f * _e437) * _e439) * _e441)) + (_e444 * _e445))));
    r_p_1 = _e449;
    let _e451 = r_14;
    let _e452 = mu_11;
    let _e454 = d_7;
    let _e456 = r_p_1;
    mu_p_1 = (((_e451 * _e452) + _e454) / _e456);
    let _e459 = r_14;
    let _e460 = mu_s_7;
    let _e462 = d_7;
    let _e463 = nu_6;
    let _e466 = r_p_1;
    mu_s_p_1 = (((_e459 * _e460) + (_e462 * _e463)) / _e466);
    let _e470 = r_p_1;
    let _e471 = mu_p_1;
    let _e472 = mu_s_p_1;
    let _e473 = nu_6;
    let _e474 = ray_r_mu_intersects_ground_5;
    let _e477 = GetCombinedScattering(atmosphere_16, scattering_texture_t_3, scattering_texture_s_3, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, _e470, _e471, _e472, _e473, _e474, (&single_mie_scattering_p));
    scattering_p = _e477;
    let _e479 = (*transmittance_1);
    shadow_transmittance_1 = _e479;
    if (shadow_length_1 > 0f) {
        {
            let _e485 = r_14;
            let _e486 = mu_11;
            let _e487 = d_7;
            let _e488 = ray_r_mu_intersects_ground_5;
            let _e489 = GetTransmittance(atmosphere_16, transmittance_texture_t_4, transmittance_texture_s_4, _e485, _e486, _e487, _e488);
            shadow_transmittance_1 = _e489;
        }
    }
    let _e490 = r_14;
    let _e491 = mu_11;
    let _e492 = mu_s_7;
    let _e493 = nu_6;
    let _e494 = ray_r_mu_intersects_ground_5;
    let _e495 = GetScattering(atmosphere_16, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e490, _e491, _e492, _e493, _e494);
    higher_order_scattering_1 = _e495;
    let _e497 = scattering_4;
    let _e498 = higher_order_scattering_1;
    single_scattering_1 = (_e497 - _e498);
    let _e501 = r_p_1;
    let _e502 = mu_p_1;
    let _e503 = mu_s_p_1;
    let _e504 = nu_6;
    let _e505 = ray_r_mu_intersects_ground_5;
    let _e506 = GetScattering(atmosphere_16, higher_order_scattering_texture_t, higher_order_scattering_texture_s, _e501, _e502, _e503, _e504, _e505);
    higher_order_scattering_p = _e506;
    let _e508 = scattering_p;
    let _e509 = higher_order_scattering_p;
    single_scattering_p = (_e508 - _e509);
    let _e512 = single_scattering_1;
    let _e513 = shadow_transmittance_1;
    let _e514 = single_scattering_p;
    let _e517 = higher_order_scattering_1;
    let _e519 = (*transmittance_1);
    let _e520 = higher_order_scattering_p;
    scattering_4 = (((_e512 - (_e513 * _e514)) + _e517) - (_e519 * _e520));
    let _e523 = single_mie_scattering_2;
    let _e524 = shadow_transmittance_1;
    let _e525 = single_mie_scattering_p;
    single_mie_scattering_2 = (_e523 - (_e524 * _e525));
    let _e528 = scattering_4;
    let _e529 = single_mie_scattering_2;
    let _e535 = GetExtrapolatedSingleMieScattering(atmosphere_16, vec4<f32>(_e528.x, _e528.y, _e528.z, _e529.x));
    single_mie_scattering_2 = _e535;
    let _e536 = single_mie_scattering_2;
    let _e541 = mu_s_7;
    single_mie_scattering_2 = (_e536 * smoothstep(0f, 0.01f, _e541));
    let _e544 = scattering_4;
    let _e545 = nu_6;
    let _e546 = RayleighPhaseFunction(_e545);
    let _e548 = single_mie_scattering_2;
    let _e550 = nu_6;
    let _e551 = MiePhaseFunction(atmosphere_16.mie_phase_function_g, _e550);
    return ((_e544 * _e546) + (_e548 * _e551));
}

fn GetSunAndSkyIrradiance(atmosphere_17: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, irradiance_texture_t_1: texture_2d<f32>, irradiance_texture_s_1: sampler, point_4: vec3<f32>, normal: vec3<f32>, sun_direction_2: vec3<f32>, sky_irradiance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_15: f32;
    var mu_s_8: f32;

    r_15 = length(point_4);
    let _e306 = r_15;
    mu_s_8 = (dot(point_4, sun_direction_2) / _e306);
    let _e309 = r_15;
    let _e310 = mu_s_8;
    let _e311 = GetIrradiance(atmosphere_17, irradiance_texture_t_1, irradiance_texture_s_1, _e309, _e310);
    let _e314 = r_15;
    (*sky_irradiance) = ((_e311 * (1f + (dot(normal, point_4) / _e314))) * 0.5f);
    let _e321 = r_15;
    let _e322 = mu_s_8;
    let _e323 = GetTransmittanceToSun(atmosphere_17, transmittance_texture_t_5, transmittance_texture_s_5, _e321, _e322);
    return ((atmosphere_17.solar_irradiance * _e323) * max(dot(normal, sun_direction_2), 0f));
}

fn GetSunAndSkyScalarIrradiance(atmosphere_18: AtmosphereParameters, transmittance_texture_t_6: texture_2d<f32>, transmittance_texture_s_6: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, point_5: vec3<f32>, sun_direction_3: vec3<f32>, sky_irradiance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_16: f32;
    var mu_s_9: f32;

    r_16 = length(point_5);
    let _e305 = r_16;
    mu_s_9 = (dot(point_5, sun_direction_3) / _e305);
    let _e308 = r_16;
    let _e309 = mu_s_9;
    let _e310 = GetIrradiance(atmosphere_18, irradiance_texture_t_2, irradiance_texture_s_2, _e308, _e309);
    (*sky_irradiance_1) = ((_e310 * 2f) * 3.1415927f);
    let _e316 = r_16;
    let _e317 = mu_s_9;
    let _e318 = GetTransmittanceToSun(atmosphere_18, transmittance_texture_t_6, transmittance_texture_s_6, _e316, _e317);
    return (atmosphere_18.solar_irradiance * _e318);
}

fn GetSolarLuminance() -> vec3<f32> {
    let _e294 = ATMOSPHERE;
    let _e297 = ATMOSPHERE;
    let _e300 = ATMOSPHERE;
    let _e305 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return ((_e294.solar_irradiance / vec3(((3.1415927f * _e297.sun_angular_radius) * _e300.sun_angular_radius))) * _e305);
}

fn GetSkyLuminance(camera_7: vec3<f32>, view_ray_3: vec3<f32>, shadow_length_2: f32, sun_direction_4: vec3<f32>, transmittance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var view_ray_4: vec3<f32>;

    view_ray_4 = view_ray_3;
    let _e300 = ATMOSPHERE;
    let _e301 = view_ray_4;
    let _e304 = GetSkyRadiance(_e300, transmittance_texture_t_7, transmittance_texture_s_7, scattering_texture_t_4, scattering_texture_s_4, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, camera_7, _e301, shadow_length_2, sun_direction_4, transmittance_2);
    let _e305 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e304 * _e305);
}

fn GetSkyLuminanceToPoint(camera_8: vec3<f32>, point_6: vec3<f32>, shadow_length_3: f32, sun_direction_5: vec3<f32>, transmittance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    let _e299 = ATMOSPHERE;
    let _e302 = GetSkyRadianceToPoint(_e299, transmittance_texture_t_7, transmittance_texture_s_7, scattering_texture_t_4, scattering_texture_s_4, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, camera_8, point_6, shadow_length_3, sun_direction_5, transmittance_3);
    let _e303 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e302 * _e303);
}

fn GetSunAndSkyIlluminance(p: vec3<f32>, normal_1: vec3<f32>, sun_direction_6: vec3<f32>, sky_irradiance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance: vec3<f32>;

    let _e298 = ATMOSPHERE;
    let _e301 = GetSunAndSkyIrradiance(_e298, transmittance_texture_t_7, transmittance_texture_s_7, irradiance_texture_t_3, irradiance_texture_s_3, p, normal_1, sun_direction_6, sky_irradiance_2);
    sun_irradiance = _e301;
    let _e303 = (*sky_irradiance_2);
    let _e304 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_2) = (_e303 * _e304);
    let _e306 = sun_irradiance;
    let _e307 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e306 * _e307);
}

fn GetSunAndSkyScalarIlluminance(p_1: vec3<f32>, sun_direction_7: vec3<f32>, sky_irradiance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance_1: vec3<f32>;

    let _e297 = ATMOSPHERE;
    let _e300 = GetSunAndSkyScalarIrradiance(_e297, transmittance_texture_t_7, transmittance_texture_s_7, irradiance_texture_t_3, irradiance_texture_s_3, p_1, sun_direction_7, sky_irradiance_3);
    sun_irradiance_1 = _e300;
    let _e302 = (*sky_irradiance_3);
    let _e303 = global.SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_3) = (_e302 * _e303);
    let _e305 = sun_irradiance_1;
    let _e306 = global.SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e305 * _e306);
}

fn sampleSunSkyIrradiance(positionECEF: vec3<f32>) {
    var local_5: vec3<f32>;
    var surfaceNormal: vec3<f32>;
    var radii: vec2<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;

    let _e299 = global.sunDirection;
    let _e300 = vGroundIrradiance;
    let _e305 = GetSunAndSkyScalarIlluminance((positionECEF * 0.001f), _e299, (&local_5));
    let _e306 = local_5;
    vGroundIrradiance.sky = _e306;
    vGroundIrradiance.sun = _e305;
    surfaceNormal = normalize(positionECEF);
    let _e309 = global.bottomRadius;
    let _e310 = global.minHeight;
    let _e311 = global.maxHeight;
    radii = ((vec2(_e309) + vec2<f32>(_e310, _e311)) * 0.001f);
    let _e319 = surfaceNormal;
    let _e320 = radii;
    let _e323 = global.sunDirection;
    let _e324 = vCloudsIrradiance;
    let _e329 = GetSunAndSkyScalarIlluminance((_e319 * _e320.x), _e323, (&local_6));
    let _e330 = local_6;
    vCloudsIrradiance.minSky = _e330;
    vCloudsIrradiance.minSun = _e329;
    let _e332 = surfaceNormal;
    let _e333 = radii;
    let _e336 = global.sunDirection;
    let _e337 = vCloudsIrradiance;
    let _e342 = GetSunAndSkyScalarIlluminance((_e332 * _e333.y), _e336, (&local_7));
    let _e343 = local_7;
    vCloudsIrradiance.maxSky = _e343;
    vCloudsIrradiance.maxSun = _e342;
    return;
}

fn pt_main() {
    var viewPosition: vec3<f32>;
    var worldDirection: vec3<f32>;
    var cameraDirection: vec3<f32>;

    let _e295 = position_1;
    vUv = ((_e295.xy * 0.5f) + vec2(0.5f));
    let _e302 = global.inverseProjectionMatrix;
    let _e303 = position_1;
    viewPosition = (_e302 * vec4<f32>(_e303.x, _e303.y, _e303.z, 1f)).xyz;
    let _e312 = global.inverseViewMatrix;
    let _e313 = viewPosition;
    let _e314 = _e313.xyz;
    worldDirection = (_e312 * vec4<f32>(_e314.x, _e314.y, _e314.z, 0f)).xyz;
    let _e323 = global.inverseViewMatrix;
    cameraDirection = normalize((_e323 * vec4<f32>(0f, 0f, -1f, 0f)).xyz);
    let _e334 = global.worldToECEFMatrix;
    let _e335 = global.cameraPosition;
    vCameraPosition = (_e334 * vec4<f32>(_e335.x, _e335.y, _e335.z, 1f)).xyz;
    let _e343 = global.worldToECEFMatrix;
    let _e344 = cameraDirection;
    vCameraDirection = (_e343 * vec4<f32>(_e344.x, _e344.y, _e344.z, 0f)).xyz;
    let _e352 = global.worldToECEFMatrix;
    let _e353 = worldDirection;
    vRayDirection = (_e352 * vec4<f32>(_e353.x, _e353.y, _e353.z, 0f)).xyz;
    let _e361 = viewPosition;
    vViewPosition = _e361;
    let _e362 = vCameraPosition;
    let _e363 = global.altitudeCorrection;
    sampleSunSkyIrradiance((_e362 + _e363));
    let _e366 = position_1;
    let _e367 = _e366.xy;
    gl_Position = vec4<f32>(_e367.x, _e367.y, 1f, 1f);
    return;
}

fn main_1() {
    let _e300 = global.pt_u_shadowIntervals[0];
    shadowIntervals[0i] = _e300.xy;
    let _e306 = global.pt_u_shadowIntervals[1];
    shadowIntervals[1i] = _e306.xy;
    let _e309 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e309;
    let _e311 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e311;
    let _e313 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e313;
    let _e315 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e315;
    let _e321 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e321;
    let _e327 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e327;
    let _e333 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e333;
    let _e339 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e339;
    let _e345 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e345;
    let _e351 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e351;
    let _e357 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e357;
    let _e363 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e363;
    let _e369 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e369;
    let _e375 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e375;
    let _e377 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e377;
    let _e383 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e383;
    let _e389 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e389;
    let _e395 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e395;
    let _e401 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e401;
    let _e407 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e407;
    let _e413 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e413;
    let _e419 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e419;
    let _e425 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e425;
    let _e431 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e431;
    let _e437 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e437;
    let _e439 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e439;
    let _e441 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e441;
    let _e443 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e443;
    let _e449 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e449;
    let _e455 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e455;
    let _e461 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e461;
    let _e467 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e467;
    let _e473 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e473;
    let _e479 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e479;
    let _e485 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e485;
    let _e491 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e491;
    let _e497 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e497;
    let _e503 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e503;
    let _e505 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e505;
    let _e507 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e507;
    let _e509 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e509;
    let _e511 = global.pt_s_densityProfile_expTerms;
    densityProfile.expTerms = _e511;
    let _e513 = global.pt_s_densityProfile_exponents;
    densityProfile.exponents = _e513;
    let _e515 = global.pt_s_densityProfile_linearTerms;
    densityProfile.linearTerms = _e515;
    let _e517 = global.pt_s_densityProfile_constantTerms;
    densityProfile.constantTerms = _e517;
    pt_main();
    let _e519 = gl_Position;
    gl_Position.y = -(_e519.y);
    let _e523 = gl_Position;
    let _e525 = gl_Position;
    gl_Position.z = ((_e523.z + _e525.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>) -> VertexOutput {
    position_1 = position;
    main_1();
    let _e353 = vUv;
    let _e355 = vCameraPosition;
    let _e357 = vCameraDirection;
    let _e359 = vRayDirection;
    let _e361 = vViewPosition;
    let _e364 = vGroundIrradiance.sun;
    let _e366 = vGroundIrradiance.sky;
    let _e369 = vCloudsIrradiance.minSun;
    let _e371 = vCloudsIrradiance.minSky;
    let _e373 = vCloudsIrradiance.maxSun;
    let _e375 = vCloudsIrradiance.maxSky;
    let _e377 = gl_Position;
    return VertexOutput(_e353, _e355, _e357, _e359, _e361, _e364, _e366, _e369, _e371, _e373, _e375, _e377);
}
