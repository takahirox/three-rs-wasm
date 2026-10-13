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

struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    resolution: vec2<f32>,
    texelSize: vec2<f32>,
    cameraNear: f32,
    cameraFar: f32,
    aspect: f32,
    time: f32,
    e1InverseViewMatrix: mat4x4<f32>,
    e1InverseProjectionMatrix: mat4x4<f32>,
    e1CameraPosition: vec3<f32>,
    e1WorldToECEFMatrix: mat4x4<f32>,
    e1AltitudeCorrection: vec3<f32>,
    e1GeometricErrorCorrectionAmount: f32,
    e1EllipsoidRadii: vec3<f32>,
    e0BlendOpacity: f32,
    e1BlendOpacity: f32,
    pt_s_e1ATMOSPHERE_solar_irradiance: vec3<f32>,
    pt_s_e1ATMOSPHERE_sun_angular_radius: f32,
    pt_s_e1ATMOSPHERE_bottom_radius: f32,
    pt_s_e1ATMOSPHERE_top_radius: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_width: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_scale: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_linear_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_constant_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_width: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_scale: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_linear_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_constant_term: f32,
    pt_s_e1ATMOSPHERE_rayleigh_scattering: vec3<f32>,
    pt_s_e1ATMOSPHERE_mie_density_layers_0_width: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_term: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_scale: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_0_linear_term: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_0_constant_term: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_1_width: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_term: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_scale: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_1_linear_term: f32,
    pt_s_e1ATMOSPHERE_mie_density_layers_1_constant_term: f32,
    pt_s_e1ATMOSPHERE_mie_scattering: vec3<f32>,
    pt_s_e1ATMOSPHERE_mie_extinction: vec3<f32>,
    pt_s_e1ATMOSPHERE_mie_phase_function_g: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_0_width: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_term: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_scale: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_0_linear_term: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_0_constant_term: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_1_width: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_term: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_scale: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_1_linear_term: f32,
    pt_s_e1ATMOSPHERE_absorption_density_layers_1_constant_term: f32,
    pt_s_e1ATMOSPHERE_absorption_extinction: vec3<f32>,
    pt_s_e1ATMOSPHERE_ground_albedo: vec3<f32>,
    pt_s_e1ATMOSPHERE_mu_s_min: f32,
    e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE: vec3<f32>,
    e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE: vec3<f32>,
    e1ProjectionMatrix: mat4x4<f32>,
    e1ViewMatrix: mat4x4<f32>,
    e1BottomRadius: f32,
    e1SunDirection: vec3<f32>,
    e1CosSunAngularRadius: f32,
    e1MoonDirection: vec3<f32>,
    e1MoonAngularRadius: f32,
    e1LunarRadianceScale: f32,
    e1AlbedoScale: f32,
    pt_u_e1ShadowIntervals: array<vec4<f32>, 2>,
    e1ShadowMatrices: array<mat4x4<f32>, 2>,
    e1InverseShadowMatrices: array<mat4x4<f32>, 2>,
    e1ShadowFar: f32,
    e1ShadowTopHeight: f32,
    e1ShadowRadius: f32,
    e1Frame: i32,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

const PackUpscale: f32 = 1.0039216f;
const UnpackDownscale: f32 = 0.99609375f;
const ShiftRight8_: f32 = 0.00390625f;
const Inv255_: f32 = 0.003921569f;
const PackFactors: vec4<f32> = vec4<f32>(1f, 256f, 65536f, 16777216f);
const UnpackFactors2_: vec2<f32> = vec2<f32>(0.99609375f, 0.00390625f);
const UnpackFactors3_: vec3<f32> = vec3<f32>(0.99609375f, 0.0038909912f, 0.000015258789f);
const UnpackFactors4_: vec4<f32> = vec4<f32>(0.99609375f, 0.0038909912f, 0.000015199184f, 0.000000059604645f);
const m_2: f32 = 1f;
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
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
@group(0) @binding(3) 
var depthBuffer_t_1: texture_2d<f32>;
@group(0) @binding(4) 
var depthBuffer_s_1: sampler;
@group(0) @binding(5) 
var e0CloudsBuffer_t: texture_2d<f32>;
@group(0) @binding(6) 
var e0CloudsBuffer_s: sampler;
@group(0) @binding(7) 
var e1Transmittance_texture_t_7: texture_2d<f32>;
@group(0) @binding(8) 
var e1Transmittance_texture_s_7: sampler;
@group(0) @binding(9) 
var e1Scattering_texture_t_4: texture_3d<f32>;
@group(0) @binding(10) 
var e1Scattering_texture_s_4: sampler;
@group(0) @binding(11) 
var e1Irradiance_texture_t_3: texture_2d<f32>;
@group(0) @binding(12) 
var e1Irradiance_texture_s_3: sampler;
@group(0) @binding(13) 
var e1Single_mie_scattering_texture_t_3: texture_3d<f32>;
@group(0) @binding(14) 
var e1Single_mie_scattering_texture_s_3: sampler;
@group(0) @binding(15) 
var e1Higher_order_scattering_texture_t: texture_3d<f32>;
@group(0) @binding(16) 
var e1Higher_order_scattering_texture_s: sampler;
@group(0) @binding(17) 
var e1NormalBuffer_t: texture_2d<f32>;
@group(0) @binding(18) 
var e1NormalBuffer_s: sampler;
@group(0) @binding(19) 
var e1OverlayBuffer_t: texture_2d<f32>;
@group(0) @binding(20) 
var e1OverlayBuffer_s: sampler;
@group(0) @binding(21) 
var e1ShadowBuffer_t: texture_2d_array<f32>;
@group(0) @binding(22) 
var e1ShadowBuffer_s: sampler;
@group(0) @binding(23) 
var e1StbnTexture_t: texture_3d<f32>;
@group(0) @binding(24) 
var e1StbnTexture_s: sampler;
@group(0) @binding(25) 
var e1ShadowLengthBuffer_t: texture_2d<f32>;
@group(0) @binding(26) 
var e1ShadowLengthBuffer_s: sampler;
var<private> e1ShadowIntervals: array<vec2<f32>, 2>;
var<private> e1ATMOSPHERE: AtmosphereParameters;
var<private> vUv_1: vec2<f32>;
var<private> e1VCameraPosition_1: vec3<f32>;
var<private> e1VRayDirection_1: vec3<f32>;
var<private> e1VGeometryAltitudeCorrection_1: vec3<f32>;
var<private> e1VEllipsoidRadiiSquared_1: vec3<f32>;
var<private> pc_fragColor: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e194 = y_1;
    let _e195 = x_1;
    return exp2((_e194 * log2(_e195)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e194 = y_3;
    let _e195 = x_3;
    return exp2((_e194 * log2(_e195)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e194 = y_5;
    let _e195 = x_5;
    return exp2((_e194 * log2(_e195)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e194 = y_7;
    let _e195 = x_7;
    return exp2((_e194 * log2(_e195)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e193 = x_9;
    (*i) = trunc(_e193);
    let _e195 = x_9;
    let _e196 = (*i);
    return (_e195 - _e196);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e193 = x_11;
    (*i_1) = trunc(_e193);
    let _e195 = x_11;
    let _e196 = (*i_1);
    return (_e195 - _e196);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e193 = x_13;
    (*i_2) = trunc(_e193);
    let _e195 = x_13;
    let _e196 = (*i_2);
    return (_e195 - _e196);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e193 = x_15;
    (*i_3) = trunc(_e193);
    let _e195 = x_15;
    let _e196 = (*i_3);
    return (_e195 - _e196);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e192 = v_1;
    b = (bitcast<u32>(_e192) & 2147483647u);
    let _e197 = b;
    return (_e197 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e192 = v_3;
    b_1 = (bitcast<u32>(_e192) & 2147483647u);
    let _e197 = b_1;
    return (_e197 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e192 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e192) & vec2(2147483647u));
    let _e198 = b_2;
    return (_e198 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e192 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e192) & vec3(2147483647u));
    let _e198 = b_3;
    return (_e198 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e192 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e192) & vec4(2147483647u));
    let _e198 = b_4;
    return (_e198 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e192 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e192) & vec2(2147483647u));
    let _e198 = b_5;
    return (_e198 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e192 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e192) & vec3(2147483647u));
    let _e198 = b_6;
    return (_e198 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e192 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e192) & vec4(2147483647u));
    let _e198 = b_7;
    return (_e198 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e198 = value_1;
    return _e198;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e198 = value_3;
    let _e207 = pt_pow_2(((_e198.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e208 = value_3;
    let _e212 = value_3;
    let _e222 = mix(_e207, (_e208.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e212.xyz <= vec3(0.04045f))));
    let _e223 = value_3;
    return vec4<f32>(_e222.x, _e222.y, _e222.z, _e223.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e198 = value_5;
    let _e202 = pt_pow_2(_e198.xyz, vec3(0.41666f));
    let _e208 = value_5;
    let _e212 = value_5;
    let _e222 = mix(((_e202 * 1.055f) - vec3(0.055f)), (_e208.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e212.xyz <= vec3(0.0031308f))));
    let _e223 = value_5;
    return vec4<f32>(_e222.x, _e222.y, _e222.z, _e223.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e198 = value_7;
    let _e216 = (_e198.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e217 = value_7;
    let _e223 = LinearTransferOETF(vec4<f32>(_e216.x, _e216.y, _e216.z, _e217.w));
    return _e223;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e203 = weights;
    let _e204 = rgb_1;
    return dot(_e203, _e204);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e198 = x_17;
    let _e199 = x_17;
    return (_e198 * _e199);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e198 = x_19;
    let _e199 = x_19;
    return (_e198 * _e199);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e198 = x_21;
    let _e199 = x_21;
    let _e201 = x_21;
    return ((_e198 * _e199) * _e201);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e198 = x_23;
    let _e199 = x_23;
    x2_ = (_e198 * _e199);
    let _e202 = x2_;
    let _e203 = x2_;
    return (_e202 * _e203);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e198 = v_17;
    let _e200 = v_17;
    let _e203 = v_17;
    return max(max(_e198.x, _e200.y), _e203.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e198 = v_19;
    return dot(_e198, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e204 = uv_1;
    let _e206 = a;
    let _e207 = b_8;
    dt = dot(_e204.xy, vec2<f32>(_e206, _e207));
    let _e211 = dt;
    sn = (_e211 - (floor((_e211 / 3.1415927f)) * 3.1415927f));
    let _e218 = sn;
    let _e220 = c;
    return fract((sin(_e218) * _e220));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e198 = v_21;
    return length(_e198);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e200 = matrix_1;
    let _e201 = dir_1;
    return normalize((_e200 * vec4<f32>(_e201.x, _e201.y, _e201.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e200 = normal_1;
    let _e206 = viewMatrix_1;
    return normalize((vec4<f32>(_e200.x, _e200.y, _e200.z, 0f) * _e206).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e200 = dir_3;
    let _e206 = viewMatrix_3;
    return normalize((vec4<f32>(_e200.x, _e200.y, _e200.z, 0f) * _e206).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e202 = m_1[2][3];
    return (_e202 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e198 = dir_5;
    let _e200 = dir_5;
    u = ((atan2(_e198.z, _e200.x) * 0.15915494f) + 0.5f);
    let _e208 = dir_5;
    v_22 = ((asin(clamp(_e208.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e220 = u;
    let _e221 = v_22;
    return vec2<f32>(_e220, _e221);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e199 = diffuseColor_1;
    return (0.31830987f * _e199);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e204 = dotVH_1;
    let _e208 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e204) - 6.98316f) * _e208));
    let _e212 = f0_1;
    let _e214 = fresnel;
    let _e217 = f90_1;
    let _e218 = fresnel;
    return ((_e212 * (1f - _e214)) + vec3((_e217 * _e218)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e204 = dotVH_3;
    let _e208 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e204) - 6.98316f) * _e208));
    let _e212 = f0_3;
    let _e214 = fresnel_1;
    let _e217 = f90_3;
    let _e218 = fresnel_1;
    return ((_e212 * (1f - _e214)) + (_e217 * _e218));
}

fn packNormalToRGB(normal_2: vec3<f32>) -> vec3<f32> {
    var normal_3: vec3<f32>;

    normal_3 = normal_2;
    let _e198 = normal_3;
    return ((normalize(_e198) * 0.5f) + vec3(0.5f));
}

fn unpackRGBToNormal(rgb_2: vec3<f32>) -> vec3<f32> {
    var rgb_3: vec3<f32>;

    rgb_3 = rgb_2;
    let _e199 = rgb_3;
    return ((2f * _e199.xyz) - vec3(1f));
}

fn packDepthToRGBA(v_23: f32) -> vec4<f32> {
    var v_24: f32;
    var vuf: f32;
    var af: f32;
    var bf: f32;
    var gf: f32;

    v_24 = v_23;
    let _e206 = v_24;
    if (_e206 <= 0f) {
        return vec4<f32>(0f, 0f, 0f, 0f);
    }
    let _e214 = v_24;
    if (_e214 >= 1f) {
        return vec4<f32>(1f, 1f, 1f, 1f);
    }
    let _e223 = v_24;
    let _e232 = pt_modf((_e223 * 16777216f), (&vuf));
    af = _e232;
    let _e234 = vuf;
    let _e238 = pt_modf((_e234 * ShiftRight8_), (&vuf));
    bf = _e238;
    let _e240 = vuf;
    let _e244 = pt_modf((_e240 * ShiftRight8_), (&vuf));
    gf = _e244;
    let _e246 = vuf;
    let _e248 = gf;
    let _e250 = bf;
    let _e252 = af;
    return vec4<f32>((_e246 * Inv255_), (_e248 * PackUpscale), (_e250 * PackUpscale), _e252);
}

fn packDepthToRGB(v_25: f32) -> vec3<f32> {
    var v_26: f32;
    var vuf_1: f32;
    var bf_1: f32;
    var gf_1: f32;

    v_26 = v_25;
    let _e206 = v_26;
    if (_e206 <= 0f) {
        return vec3<f32>(0f, 0f, 0f);
    }
    let _e213 = v_26;
    if (_e213 >= 1f) {
        return vec3<f32>(1f, 1f, 1f);
    }
    let _e221 = v_26;
    let _e230 = pt_modf((_e221 * 65536f), (&vuf_1));
    bf_1 = _e230;
    let _e232 = vuf_1;
    let _e236 = pt_modf((_e232 * ShiftRight8_), (&vuf_1));
    gf_1 = _e236;
    let _e238 = vuf_1;
    let _e240 = gf_1;
    let _e242 = bf_1;
    return vec3<f32>((_e238 * Inv255_), (_e240 * PackUpscale), _e242);
}

fn packDepthToRG(v_27: f32) -> vec2<f32> {
    var v_28: f32;
    var vuf_2: f32;
    var gf_2: f32;

    v_28 = v_27;
    let _e206 = v_28;
    if (_e206 <= 0f) {
        return vec2<f32>(0f, 0f);
    }
    let _e212 = v_28;
    if (_e212 >= 1f) {
        return vec2<f32>(1f, 1f);
    }
    let _e219 = v_28;
    let _e224 = pt_modf((_e219 * 256f), (&vuf_2));
    gf_2 = _e224;
    let _e226 = vuf_2;
    let _e228 = gf_2;
    return vec2<f32>((_e226 * Inv255_), _e228);
}

fn unpackRGBAToDepth(v_29: vec4<f32>) -> f32 {
    var v_30: vec4<f32>;

    v_30 = v_29;
    let _e206 = v_30;
    return dot(_e206, UnpackFactors4_);
}

fn unpackRGBToDepth(v_31: vec3<f32>) -> f32 {
    var v_32: vec3<f32>;

    v_32 = v_31;
    let _e206 = v_32;
    return dot(_e206, UnpackFactors3_);
}

fn unpackRGToDepth(v_33: vec2<f32>) -> f32 {
    var v_34: vec2<f32>;

    v_34 = v_33;
    let _e206 = v_34;
    let _e212 = v_34;
    return ((_e206.x * 0.99609375f) + (_e212.y * 0.00390625f));
}

fn pack2HalfToRGBA(v_35: vec2<f32>) -> vec4<f32> {
    var v_36: vec2<f32>;
    var r: vec4<f32>;

    v_36 = v_35;
    let _e206 = v_36;
    let _e208 = v_36;
    let _e213 = v_36;
    let _e215 = v_36;
    r = vec4<f32>(_e206.x, fract((_e208.x * 255f)), _e213.y, fract((_e215.y * 255f)));
    let _e222 = r;
    let _e224 = r;
    let _e229 = r;
    let _e231 = r;
    let _e233 = r;
    let _e238 = r;
    return vec4<f32>((_e222.x - (_e224.y / 255f)), _e229.y, (_e231.z - (_e233.w / 255f)), _e238.w);
}

fn unpackRGBATo2Half(v_37: vec4<f32>) -> vec2<f32> {
    var v_38: vec4<f32>;

    v_38 = v_37;
    let _e206 = v_38;
    let _e208 = v_38;
    let _e213 = v_38;
    let _e215 = v_38;
    return vec2<f32>((_e206.x + (_e208.y / 255f)), (_e213.z + (_e215.w / 255f)));
}

fn viewZToOrthographicDepth(viewZ: f32, near: f32, far: f32) -> f32 {
    var viewZ_1: f32;
    var near_1: f32;
    var far_1: f32;

    viewZ_1 = viewZ;
    near_1 = near;
    far_1 = far;
    let _e210 = viewZ_1;
    let _e211 = near_1;
    let _e213 = near_1;
    let _e214 = far_1;
    return ((_e210 + _e211) / (_e213 - _e214));
}

fn orthographicDepthToViewZ(depth: f32, near_2: f32, far_2: f32) -> f32 {
    var depth_1: f32;
    var near_3: f32;
    var far_3: f32;

    depth_1 = depth;
    near_3 = near_2;
    far_3 = far_2;
    let _e210 = depth_1;
    let _e211 = near_3;
    let _e212 = far_3;
    let _e215 = near_3;
    return ((_e210 * (_e211 - _e212)) - _e215);
}

fn viewZToPerspectiveDepth(viewZ_2: f32, near_4: f32, far_4: f32) -> f32 {
    var viewZ_3: f32;
    var near_5: f32;
    var far_5: f32;

    viewZ_3 = viewZ_2;
    near_5 = near_4;
    far_5 = far_4;
    let _e210 = near_5;
    let _e211 = viewZ_3;
    let _e213 = far_5;
    let _e215 = far_5;
    let _e216 = near_5;
    let _e218 = viewZ_3;
    return (((_e210 + _e211) * _e213) / ((_e215 - _e216) * _e218));
}

fn perspectiveDepthToViewZ(depth_2: f32, near_6: f32, far_6: f32) -> f32 {
    var depth_3: f32;
    var near_7: f32;
    var far_7: f32;

    depth_3 = depth_2;
    near_7 = near_6;
    far_7 = far_6;
    let _e210 = near_7;
    let _e211 = far_7;
    let _e213 = far_7;
    let _e214 = near_7;
    let _e216 = depth_3;
    let _e218 = far_7;
    return ((_e210 * _e211) / (((_e213 - _e214) * _e216) - _e218));
}

fn sRGBToLinear(value_8: vec4<f32>) -> vec4<f32> {
    var value_9: vec4<f32>;

    value_9 = value_8;
    let _e206 = value_9;
    let _e215 = pt_pow_2(((_e206.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e216 = value_9;
    let _e220 = value_9;
    let _e230 = mix(_e215, (_e216.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e220.xyz <= vec3(0.04045f))));
    let _e231 = value_9;
    return vec4<f32>(_e230.x, _e230.y, _e230.z, _e231.w);
}

fn readDepth(uv_2: vec2<f32>) -> f32 {
    var uv_3: vec2<f32>;
    var depth_4: f32;

    uv_3 = uv_2;
    let _e206 = uv_3;
    let _e207 = textureSample(depthBuffer_t_1, depthBuffer_s_1, _e206);
    depth_4 = _e207.x;
    let _e210 = depth_4;
    return _e210;
}

fn getViewZ(depth_5: f32) -> f32 {
    var depth_6: f32;

    depth_6 = depth_5;
    let _e206 = depth_6;
    let _e207 = global.cameraNear;
    let _e208 = global.cameraFar;
    let _e209 = perspectiveDepthToViewZ(_e206, _e207, _e208);
    return _e209;
}

fn RGBToHCV(RGB: vec3<f32>) -> vec3<f32> {
    var RGB_1: vec3<f32>;
    var P: vec4<f32>;
    var Q: vec4<f32>;
    var C: f32;
    var H: f32;

    RGB_1 = RGB;
    let _e206 = RGB_1;
    let _e207 = _e206.zy;
    let _e216 = RGB_1;
    let _e217 = _e216.yz;
    let _e226 = RGB_1;
    let _e228 = RGB_1;
    P = mix(vec4<f32>(_e207.x, _e207.y, -1f, 0.6666667f), vec4<f32>(_e217.x, _e217.y, 0f, -0.33333334f), vec4(step(_e226.z, _e228.y)));
    let _e234 = P;
    let _e235 = _e234.xyw;
    let _e236 = RGB_1;
    let _e242 = RGB_1;
    let _e244 = P;
    let _e245 = _e244.yzx;
    let _e250 = P;
    let _e252 = RGB_1;
    Q = mix(vec4<f32>(_e235.x, _e235.y, _e235.z, _e236.x), vec4<f32>(_e242.x, _e245.x, _e245.y, _e245.z), vec4(step(_e250.x, _e252.x)));
    let _e258 = Q;
    let _e260 = Q;
    let _e262 = Q;
    C = (_e258.x - min(_e260.w, _e262.y));
    let _e267 = Q;
    let _e269 = Q;
    let _e273 = C;
    let _e278 = Q;
    H = abs((((_e267.w - _e269.y) / ((6f * _e273) + 0.000001f)) + _e278.z));
    let _e283 = H;
    let _e284 = C;
    let _e285 = Q;
    return vec3<f32>(_e283, _e284, _e285.x);
}

fn RGBToHSL(RGB_2: vec3<f32>) -> vec3<f32> {
    var RGB_3: vec3<f32>;
    var HCV: vec3<f32>;
    var L: f32;
    var S: f32;

    RGB_3 = RGB_2;
    let _e206 = RGB_3;
    let _e207 = RGBToHCV(_e206);
    HCV = _e207;
    let _e209 = HCV;
    let _e211 = HCV;
    L = (_e209.z - (_e211.y * 0.5f));
    let _e217 = HCV;
    let _e220 = L;
    S = (_e217.y / ((1f - abs(((_e220 * 2f) - 1f))) + 0.000001f));
    let _e231 = HCV;
    let _e233 = S;
    let _e234 = L;
    return vec3<f32>(_e231.x, _e233, _e234);
}

fn HueToRGB(H_1: f32) -> vec3<f32> {
    var H_2: f32;
    var R: f32;
    var G: f32;
    var B: f32;

    H_2 = H_1;
    let _e206 = H_2;
    R = (abs(((_e206 * 6f) - 3f)) - 1f);
    let _e216 = H_2;
    G = (2f - abs(((_e216 * 6f) - 2f)));
    let _e225 = H_2;
    B = (2f - abs(((_e225 * 6f) - 4f)));
    let _e233 = R;
    let _e234 = G;
    let _e235 = B;
    return clamp(vec3<f32>(_e233, _e234, _e235), vec3(0f), vec3(1f));
}

fn HSLToRGB(HSL: vec3<f32>) -> vec3<f32> {
    var HSL_1: vec3<f32>;
    var RGB_4: vec3<f32>;
    var C_1: f32;

    HSL_1 = HSL;
    let _e206 = HSL_1;
    let _e208 = HueToRGB(_e206.x);
    RGB_4 = _e208;
    let _e212 = HSL_1;
    let _e219 = HSL_1;
    C_1 = ((1f - abs(((2f * _e212.z) - 1f))) * _e219.y);
    let _e223 = RGB_4;
    let _e227 = C_1;
    let _e229 = HSL_1;
    return (((_e223 - vec3(0.5f)) * _e227) + vec3(_e229.z));
}

fn e0MainImage(inputColor: vec4<f32>, uv_4: vec2<f32>, outputColor: ptr<function, vec4<f32>>) {
    (*outputColor) = inputColor;
    return;
}

fn e1ReadDepthValue(depthBuffer_t: texture_2d<f32>, depthBuffer_s: sampler, uv_5: vec2<f32>) -> f32 {
    let _e207 = textureSample(depthBuffer_t, depthBuffer_s, uv_5);
    return _e207.x;
}

fn e1ReverseLogDepth(depth_7: f32, near_8: f32, far_8: f32) -> f32 {
    return depth_7;
}

fn e1LinearizeDepth(depth_8: f32, near_9: f32, far_9: f32) -> f32 {
    var ndc: f32;

    ndc = ((depth_8 * 2f) - 1f);
    let _e216 = ndc;
    return (((2f * near_9) * far_9) / ((far_9 + near_9) - (_e216 * (far_9 - near_9))));
}

fn e1Remap(x_24: f32, min1_: f32, max1_: f32, min2_: f32, max2_: f32) -> f32 {
    return (min2_ + (((x_24 - min1_) / (max1_ - min1_)) * (max2_ - min2_)));
}

fn e1Remap_1(x_25: vec2<f32>, min1_1: vec2<f32>, max1_1: vec2<f32>, min2_1: vec2<f32>, max2_1: vec2<f32>) -> vec2<f32> {
    return (min2_1 + (((x_25 - min1_1) / (max1_1 - min1_1)) * (max2_1 - min2_1)));
}

fn e1Remap_2(x_26: vec3<f32>, min1_2: vec3<f32>, max1_2: vec3<f32>, min2_2: vec3<f32>, max2_2: vec3<f32>) -> vec3<f32> {
    return (min2_2 + (((x_26 - min1_2) / (max1_2 - min1_2)) * (max2_2 - min2_2)));
}

fn e1Remap_3(x_27: vec4<f32>, min1_3: vec4<f32>, max1_3: vec4<f32>, min2_3: vec4<f32>, max2_3: vec4<f32>) -> vec4<f32> {
    return (min2_3 + (((x_27 - min1_3) / (max1_3 - min1_3)) * (max2_3 - min2_3)));
}

fn e1RemapClamped(x_28: f32, min1_4: f32, max1_4: f32, min2_4: f32, max2_4: f32) -> f32 {
    return clamp((min2_4 + (((x_28 - min1_4) / (max1_4 - min1_4)) * (max2_4 - min2_4))), min2_4, max2_4);
}

fn e1RemapClamped_1(x_29: vec2<f32>, min1_5: vec2<f32>, max1_5: vec2<f32>, min2_5: vec2<f32>, max2_5: vec2<f32>) -> vec2<f32> {
    return clamp((min2_5 + (((x_29 - min1_5) / (max1_5 - min1_5)) * (max2_5 - min2_5))), min2_5, max2_5);
}

fn e1RemapClamped_2(x_30: vec3<f32>, min1_6: vec3<f32>, max1_6: vec3<f32>, min2_6: vec3<f32>, max2_6: vec3<f32>) -> vec3<f32> {
    return clamp((min2_6 + (((x_30 - min1_6) / (max1_6 - min1_6)) * (max2_6 - min2_6))), min2_6, max2_6);
}

fn e1RemapClamped_3(x_31: vec4<f32>, min1_7: vec4<f32>, max1_7: vec4<f32>, min2_7: vec4<f32>, max2_7: vec4<f32>) -> vec4<f32> {
    return clamp((min2_7 + (((x_31 - min1_7) / (max1_7 - min1_7)) * (max2_7 - min2_7))), min2_7, max2_7);
}

fn e1Remap_4(x_32: f32, min1_8: f32, max1_8: f32) -> f32 {
    return ((x_32 - min1_8) / (max1_8 - min1_8));
}

fn e1Remap_5(x_33: vec2<f32>, min1_9: vec2<f32>, max1_9: vec2<f32>) -> vec2<f32> {
    return ((x_33 - min1_9) / (max1_9 - min1_9));
}

fn e1Remap_6(x_34: vec3<f32>, min1_10: vec3<f32>, max1_10: vec3<f32>) -> vec3<f32> {
    return ((x_34 - min1_10) / (max1_10 - min1_10));
}

fn e1Remap_7(x_35: vec4<f32>, min1_11: vec4<f32>, max1_11: vec4<f32>) -> vec4<f32> {
    return ((x_35 - min1_11) / (max1_11 - min1_11));
}

fn e1RemapClamped_4(x_36: f32, min1_12: f32, max1_12: f32) -> f32 {
    return clamp(((x_36 - min1_12) / (max1_12 - min1_12)), 0f, 1f);
}

fn e1RemapClamped_5(x_37: vec2<f32>, min1_13: vec2<f32>, max1_13: vec2<f32>) -> vec2<f32> {
    return clamp(((x_37 - min1_13) / (max1_13 - min1_13)), vec2(0f), vec2(1f));
}

fn e1RemapClamped_6(x_38: vec3<f32>, min1_14: vec3<f32>, max1_14: vec3<f32>) -> vec3<f32> {
    return clamp(((x_38 - min1_14) / (max1_14 - min1_14)), vec3(0f), vec3(1f));
}

fn e1RemapClamped_7(x_39: vec4<f32>, min1_15: vec4<f32>, max1_15: vec4<f32>) -> vec4<f32> {
    return clamp(((x_39 - min1_15) / (max1_15 - min1_15)), vec4(0f), vec4(1f));
}

fn e1SignNotZero(v_39: vec2<f32>) -> vec2<f32> {
    var v_40: vec2<f32>;
    var local: f32;
    var local_1: f32;

    v_40 = v_39;
    let _e206 = v_40;
    if (_e206.x >= 0f) {
        local = 1f;
    } else {
        local = -1f;
    }
    let _e214 = local;
    let _e215 = v_40;
    if (_e215.y >= 0f) {
        local_1 = 1f;
    } else {
        local_1 = -1f;
    }
    let _e223 = local_1;
    return vec2<f32>(_e214, _e223);
}

fn e1PackNormalToVec2_(v_41: vec3<f32>) -> vec2<f32> {
    var v_42: vec3<f32>;
    var p: vec2<f32>;
    var local_2: vec2<f32>;

    v_42 = v_41;
    let _e206 = v_42;
    let _e209 = v_42;
    let _e212 = v_42;
    let _e216 = v_42;
    p = (_e206.xy * (1f / ((abs(_e209.x) + abs(_e212.y)) + abs(_e216.z))));
    let _e223 = v_42;
    if (_e223.z <= 0f) {
        let _e228 = p;
        let _e233 = p;
        let _e234 = e1SignNotZero(_e233);
        local_2 = ((vec2(1f) - abs(_e228.yx)) * _e234);
    } else {
        let _e236 = p;
        local_2 = _e236;
    }
    let _e238 = local_2;
    return _e238;
}

fn e1UnpackVec2ToNormal(e: vec2<f32>) -> vec3<f32> {
    var e_1: vec2<f32>;
    var v_43: vec3<f32>;

    e_1 = e;
    let _e206 = e_1;
    let _e207 = _e206.xy;
    let _e209 = e_1;
    let _e213 = e_1;
    v_43 = vec3<f32>(_e207.x, _e207.y, ((1f - abs(_e209.x)) - abs(_e213.y)));
    let _e221 = v_43;
    if (_e221.z < 0f) {
        {
            let _e225 = v_43;
            let _e228 = v_43;
            let _e233 = v_43;
            let _e235 = e1SignNotZero(_e233.xy);
            let _e236 = ((vec2(1f) - abs(_e228.yx)) * _e235);
            v_43.x = _e236.x;
            v_43.y = _e236.y;
        }
    }
    let _e241 = v_43;
    return normalize(_e241);
}

fn e1ScreenToView(uv_6: vec2<f32>, depth_9: f32, viewZ_4: f32, e1ProjectionMatrix: mat4x4<f32>, e1InverseProjectionMatrix: mat4x4<f32>) -> vec3<f32> {
    var clip: vec4<f32>;
    var clipW: f32;

    let _e216 = ((vec3<f32>(uv_6.x, uv_6.y, depth_9) * 2f) - vec3(1f));
    clip = vec4<f32>(_e216.x, _e216.y, _e216.z, 1f);
    clipW = ((e1ProjectionMatrix[2].w * viewZ_4) + e1ProjectionMatrix[3].w);
    let _e234 = clip;
    let _e235 = clipW;
    clip = (_e234 * _e235);
    let _e237 = clip;
    return (e1InverseProjectionMatrix * _e237).xyz;
}

fn e1RaySphereFirstIntersection(origin: vec3<f32>, direction: vec3<f32>, center: vec3<f32>, radius: f32) -> f32 {
    var a_1: vec3<f32>;
    var b_9: f32;
    var c_1: f32;
    var discriminant: f32;
    var local_3: f32;

    a_1 = (origin - center);
    let _e211 = a_1;
    b_9 = (2f * dot(direction, _e211));
    let _e215 = a_1;
    let _e216 = a_1;
    c_1 = (dot(_e215, _e216) - (radius * radius));
    let _e221 = b_9;
    let _e222 = b_9;
    let _e225 = c_1;
    discriminant = ((_e221 * _e222) - (4f * _e225));
    let _e229 = discriminant;
    if (_e229 < 0f) {
        local_3 = -1f;
    } else {
        let _e234 = b_9;
        let _e236 = discriminant;
        local_3 = ((-(_e234) - sqrt(_e236)) * 0.5f);
    }
    let _e242 = local_3;
    return _e242;
}

fn e1RaySphereFirstIntersection_1(origin_1: vec3<f32>, direction_1: vec3<f32>, radius_1: f32) -> f32 {
    let _e209 = e1RaySphereFirstIntersection(origin_1, direction_1, vec3(0f), radius_1);
    return _e209;
}

fn e1RaySphereFirstIntersection_2(origin_2: vec3<f32>, direction_2: vec3<f32>, center_1: vec3<f32>, radius_2: vec4<f32>) -> vec4<f32> {
    var a_2: vec3<f32>;
    var b_10: f32;
    var c_2: vec4<f32>;
    var discriminant_1: vec4<f32>;
    var mask: vec4<f32>;

    a_2 = (origin_2 - center_1);
    let _e211 = a_2;
    b_10 = (2f * dot(direction_2, _e211));
    let _e215 = a_2;
    let _e216 = a_2;
    c_2 = (vec4(dot(_e215, _e216)) - (radius_2 * radius_2));
    let _e222 = b_10;
    let _e223 = b_10;
    let _e226 = c_2;
    discriminant_1 = (vec4((_e222 * _e223)) - (4f * _e226));
    let _e231 = discriminant_1;
    mask = step(_e231, vec4(0f));
    let _e236 = b_10;
    let _e240 = discriminant_1;
    let _e250 = mask;
    return mix(((vec4(-(_e236)) - sqrt(max(vec4(0f), _e240))) * 0.5f), vec4(-1f), _e250);
}

fn e1RaySphereFirstIntersection_3(origin_3: vec3<f32>, direction_3: vec3<f32>, radius_3: vec4<f32>) -> vec4<f32> {
    let _e209 = e1RaySphereFirstIntersection_2(origin_3, direction_3, vec3(0f), radius_3);
    return _e209;
}

fn e1RaySphereSecondIntersection(origin_4: vec3<f32>, direction_4: vec3<f32>, center_2: vec3<f32>, radius_4: f32) -> f32 {
    var a_3: vec3<f32>;
    var b_11: f32;
    var c_3: f32;
    var discriminant_2: f32;
    var local_4: f32;

    a_3 = (origin_4 - center_2);
    let _e211 = a_3;
    b_11 = (2f * dot(direction_4, _e211));
    let _e215 = a_3;
    let _e216 = a_3;
    c_3 = (dot(_e215, _e216) - (radius_4 * radius_4));
    let _e221 = b_11;
    let _e222 = b_11;
    let _e225 = c_3;
    discriminant_2 = ((_e221 * _e222) - (4f * _e225));
    let _e229 = discriminant_2;
    if (_e229 < 0f) {
        local_4 = -1f;
    } else {
        let _e234 = b_11;
        let _e236 = discriminant_2;
        local_4 = ((-(_e234) + sqrt(_e236)) * 0.5f);
    }
    let _e242 = local_4;
    return _e242;
}

fn e1RaySphereSecondIntersection_1(origin_5: vec3<f32>, direction_5: vec3<f32>, radius_5: f32) -> f32 {
    let _e209 = e1RaySphereSecondIntersection(origin_5, direction_5, vec3(0f), radius_5);
    return _e209;
}

fn e1RaySphereSecondIntersection_2(origin_6: vec3<f32>, direction_6: vec3<f32>, center_3: vec3<f32>, radius_6: vec4<f32>) -> vec4<f32> {
    var a_4: vec3<f32>;
    var b_12: f32;
    var c_4: vec4<f32>;
    var discriminant_3: vec4<f32>;
    var mask_1: vec4<f32>;

    a_4 = (origin_6 - center_3);
    let _e211 = a_4;
    b_12 = (2f * dot(direction_6, _e211));
    let _e215 = a_4;
    let _e216 = a_4;
    c_4 = (vec4(dot(_e215, _e216)) - (radius_6 * radius_6));
    let _e222 = b_12;
    let _e223 = b_12;
    let _e226 = c_4;
    discriminant_3 = (vec4((_e222 * _e223)) - (4f * _e226));
    let _e231 = discriminant_3;
    mask_1 = step(_e231, vec4(0f));
    let _e236 = b_12;
    let _e240 = discriminant_3;
    let _e250 = mask_1;
    return mix(((vec4(-(_e236)) + sqrt(max(vec4(0f), _e240))) * 0.5f), vec4(-1f), _e250);
}

fn e1RaySphereSecondIntersection_3(origin_7: vec3<f32>, direction_7: vec3<f32>, radius_7: vec4<f32>) -> vec4<f32> {
    let _e209 = e1RaySphereSecondIntersection_2(origin_7, direction_7, vec3(0f), radius_7);
    return _e209;
}

fn e1RaySphereIntersections(origin_8: vec3<f32>, direction_8: vec3<f32>, center_4: vec3<f32>, radius_8: f32, intersection1_: ptr<function, f32>, intersection2_: ptr<function, f32>) {
    var a_5: vec3<f32>;
    var b_13: f32;
    var c_5: f32;
    var discriminant_4: f32;
    var Q_1: f32;

    a_5 = (origin_8 - center_4);
    let _e213 = a_5;
    b_13 = (2f * dot(direction_8, _e213));
    let _e217 = a_5;
    let _e218 = a_5;
    c_5 = (dot(_e217, _e218) - (radius_8 * radius_8));
    let _e223 = b_13;
    let _e224 = b_13;
    let _e227 = c_5;
    discriminant_4 = ((_e223 * _e224) - (4f * _e227));
    let _e231 = discriminant_4;
    if (_e231 < 0f) {
        {
            (*intersection1_) = -1f;
            (*intersection2_) = -1f;
            return;
        }
    } else {
        {
            let _e238 = discriminant_4;
            Q_1 = sqrt(_e238);
            let _e241 = b_13;
            let _e243 = Q_1;
            (*intersection1_) = ((-(_e241) - _e243) * 0.5f);
            let _e247 = b_13;
            let _e249 = Q_1;
            (*intersection2_) = ((-(_e247) + _e249) * 0.5f);
            return;
        }
    }
}

fn e1RaySphereIntersections_1(origin_9: vec3<f32>, direction_9: vec3<f32>, radius_9: f32, intersection1_1: ptr<function, f32>, intersection2_1: ptr<function, f32>) {
    e1RaySphereIntersections(origin_9, direction_9, vec3(0f), radius_9, intersection1_1, intersection2_1);
    return;
}

fn e1RaySphereIntersections_2(origin_10: vec3<f32>, direction_10: vec3<f32>, center_5: vec3<f32>, radius_10: vec4<f32>, intersection1_2: ptr<function, vec4<f32>>, intersection2_2: ptr<function, vec4<f32>>) {
    var a_6: vec3<f32>;
    var b_14: f32;
    var c_6: vec4<f32>;
    var discriminant_5: vec4<f32>;
    var mask_2: vec4<f32>;
    var Q_2: vec4<f32>;

    a_6 = (origin_10 - center_5);
    let _e213 = a_6;
    b_14 = (2f * dot(direction_10, _e213));
    let _e217 = a_6;
    let _e218 = a_6;
    c_6 = (vec4(dot(_e217, _e218)) - (radius_10 * radius_10));
    let _e224 = b_14;
    let _e225 = b_14;
    let _e228 = c_6;
    discriminant_5 = (vec4((_e224 * _e225)) - (4f * _e228));
    let _e233 = discriminant_5;
    mask_2 = step(_e233, vec4(0f));
    let _e240 = discriminant_5;
    Q_2 = sqrt(max(vec4(0f), _e240));
    let _e244 = b_14;
    let _e246 = Q_2;
    let _e254 = mask_2;
    (*intersection1_2) = mix(((vec4(-(_e244)) - _e246) * 0.5f), vec4(-1f), _e254);
    let _e256 = b_14;
    let _e258 = Q_2;
    let _e266 = mask_2;
    (*intersection2_2) = mix(((vec4(-(_e256)) + _e258) * 0.5f), vec4(-1f), _e266);
    return;
}

fn e1RaySphereIntersections_3(origin_11: vec3<f32>, direction_11: vec3<f32>, radius_11: vec4<f32>, intersection1_3: ptr<function, vec4<f32>>, intersection2_3: ptr<function, vec4<f32>>) {
    e1RaySphereIntersections_2(origin_11, direction_11, vec3(0f), radius_11, intersection1_3, intersection2_3);
    return;
}

fn getCascadeIndex(e1ViewMatrix: mat4x4<f32>, worldPosition: vec3<f32>, intervals: array<vec2<f32>, 2>, near_10: f32, far_10: f32) -> i32 {
    var viewPosition: vec4<f32>;
    var depth_10: f32;
    var interval: vec2<f32>;

    viewPosition = (e1ViewMatrix * vec4<f32>(worldPosition.x, worldPosition.y, worldPosition.z, 1f));
    let _e216 = viewPosition;
    let _e218 = viewZToOrthographicDepth(_e216.z, near_10, far_10);
    depth_10 = _e218;
    interval = intervals[0];
    let _e223 = depth_10;
    let _e224 = interval;
    let _e227 = depth_10;
    let _e228 = interval;
    if ((_e223 >= _e224.x) && (_e227 < _e228.y)) {
        {
            return 0i;
        }
    }
    interval = intervals[1];
    let _e235 = depth_10;
    let _e236 = interval;
    let _e239 = depth_10;
    let _e240 = interval;
    if ((_e235 >= _e236.x) && (_e239 < _e240.y)) {
        {
            return 1i;
        }
    }
    return 1i;
}

fn getFadedCascadeIndex(e1ViewMatrix_1: mat4x4<f32>, worldPosition_1: vec3<f32>, intervals_1: array<vec2<f32>, 2>, near_11: f32, far_11: f32, jitter: f32) -> i32 {
    var viewPosition_1: vec4<f32>;
    var depth_11: f32;
    var interval_1: vec2<f32>;
    var intervalCenter: f32;
    var closestEdge: f32;
    var margin: f32;
    var nextIndex: i32 = -1i;
    var prevIndex: i32 = -1i;
    var alpha: f32;
    var local_5: f32;
    var local_6: f32;
    var local_7: i32;

    viewPosition_1 = (e1ViewMatrix_1 * vec4<f32>(worldPosition_1.x, worldPosition_1.y, worldPosition_1.z, 1f));
    let _e217 = viewPosition_1;
    let _e219 = viewZToOrthographicDepth(_e217.z, near_11, far_11);
    depth_11 = _e219;
    interval_1 = intervals_1[0];
    let _e234 = interval_1;
    let _e236 = interval_1;
    intervalCenter = ((_e234.x + _e236.y) * 0.5f);
    let _e241 = depth_11;
    let _e242 = intervalCenter;
    if (_e241 < _e242) {
        let _e244 = interval_1;
        local_5 = _e244.x;
    } else {
        let _e246 = interval_1;
        local_5 = _e246.y;
    }
    let _e249 = local_5;
    closestEdge = _e249;
    let _e250 = closestEdge;
    let _e251 = closestEdge;
    margin = ((_e250 * _e251) * 0.5f);
    let _e255 = interval_1;
    let _e256 = margin;
    interval_1 = (_e255 + (_e256 * vec2<f32>(-0.5f, 0.5f)));
    let _e263 = depth_11;
    let _e264 = interval_1;
    let _e267 = depth_11;
    let _e268 = interval_1;
    if ((_e263 >= _e264.x) && (_e267 < _e268.y)) {
        {
            let _e272 = nextIndex;
            prevIndex = _e272;
            nextIndex = 0i;
            let _e274 = depth_11;
            let _e275 = interval_1;
            let _e278 = interval_1;
            let _e280 = depth_11;
            let _e283 = margin;
            alpha = clamp((min((_e274 - _e275.x), (_e278.y - _e280)) / _e283), 0f, 1f);
        }
    }
    interval_1 = intervals_1[1];
    let _e290 = interval_1;
    let _e292 = interval_1;
    intervalCenter = ((_e290.x + _e292.y) * 0.5f);
    let _e297 = depth_11;
    let _e298 = intervalCenter;
    if (_e297 < _e298) {
        let _e300 = interval_1;
        local_6 = _e300.x;
    } else {
        let _e302 = interval_1;
        local_6 = _e302.y;
    }
    let _e305 = local_6;
    closestEdge = _e305;
    let _e306 = closestEdge;
    let _e307 = closestEdge;
    margin = ((_e306 * _e307) * 0.5f);
    let _e311 = interval_1;
    let _e312 = margin;
    interval_1 = (_e311 + (_e312 * vec2<f32>(-0.5f, 0.5f)));
    let _e319 = depth_11;
    let _e320 = interval_1;
    if (_e319 >= _e320.x) {
        {
            let _e323 = nextIndex;
            prevIndex = _e323;
            nextIndex = 1i;
            let _e325 = depth_11;
            let _e326 = interval_1;
            let _e329 = margin;
            alpha = clamp(((_e325 - _e326.x) / _e329), 0f, 1f);
        }
    }
    let _e334 = alpha;
    if (jitter <= _e334) {
        let _e336 = nextIndex;
        local_7 = _e336;
    } else {
        let _e337 = prevIndex;
        local_7 = _e337;
    }
    let _e339 = local_7;
    return _e339;
}

fn e1InterleavedGradientNoise(coord: vec2<f32>) -> f32 {
    var magic: vec3<f32> = vec3<f32>(0.06711056f, 0.00583715f, 52.982918f);

    let _e210 = magic;
    let _e212 = magic;
    return fract((_e210.z * fract(dot(coord, _e212.xy))));
}

fn e1VogelDisk(index: i32, sampleCount: i32, phi: f32) -> vec2<f32> {
    var goldenAngle: f32 = 2.3999631f;
    var r_1: f32;
    var theta: f32;

    r_1 = (sqrt((f32(index) + 0.5f)) / sqrt(f32(sampleCount)));
    let _e218 = goldenAngle;
    theta = ((f32(index) * _e218) + phi);
    let _e222 = r_1;
    let _e223 = theta;
    let _e225 = theta;
    return (_e222 * vec2<f32>(cos(_e223), sin(_e225)));
}

fn e1ClampCosine(mu: f32) -> f32 {
    return clamp(mu, -1f, 1f);
}

fn e1ClampDistance(d: f32) -> f32 {
    return max(d, 0f);
}

fn e1ClampRadius(atmosphere: AtmosphereParameters, r_2: f32) -> f32 {
    return clamp(r_2, atmosphere.bottom_radius, atmosphere.top_radius);
}

fn e1SafeSqrt(a_7: f32) -> f32 {
    return sqrt(max(a_7, 0f));
}

fn e1DistanceToTopAtmosphereBoundary(atmosphere_1: AtmosphereParameters, r_3: f32, mu_1: f32) -> f32 {
    var discriminant_6: f32;

    discriminant_6 = (((r_3 * r_3) * ((mu_1 * mu_1) - 1f)) + (atmosphere_1.top_radius * atmosphere_1.top_radius));
    let _e239 = discriminant_6;
    let _e240 = e1SafeSqrt(_e239);
    let _e242 = e1ClampDistance(((-(r_3) * mu_1) + _e240));
    return _e242;
}

fn e1DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_4: f32, mu_2: f32) -> f32 {
    var discriminant_7: f32;

    discriminant_7 = (((r_4 * r_4) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e239 = discriminant_7;
    let _e240 = e1SafeSqrt(_e239);
    let _e242 = e1ClampDistance(((-(r_4) * mu_2) - _e240));
    return _e242;
}

fn e1RayIntersectsGround(atmosphere_3: AtmosphereParameters, r_5: f32, mu_3: f32) -> bool {
    return ((mu_3 < 0f) && ((((r_5 * r_5) * ((mu_3 * mu_3) - 1f)) + (atmosphere_3.bottom_radius * atmosphere_3.bottom_radius)) >= 0f));
}

fn e1GetTextureCoordFromUnitRange(x_40: f32, texture_size: i32) -> f32 {
    return ((0.5f / f32(texture_size)) + (x_40 * (1f - (1f / f32(texture_size)))));
}

fn e1GetTransmittanceTextureUvFromRMu(atmosphere_4: AtmosphereParameters, r_6: f32, mu_4: f32) -> vec2<f32> {
    var H_3: f32;
    var rho: f32;
    var d_1: f32;
    var d_min: f32;
    var d_max: f32;
    var x_mu: f32;
    var x_r: f32;

    H_3 = sqrt(((atmosphere_4.top_radius * atmosphere_4.top_radius) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    let _e241 = e1SafeSqrt(((r_6 * r_6) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e241;
    let _e243 = e1DistanceToTopAtmosphereBoundary(atmosphere_4, r_6, mu_4);
    d_1 = _e243;
    d_min = (atmosphere_4.top_radius - r_6);
    let _e248 = rho;
    let _e249 = H_3;
    d_max = (_e248 + _e249);
    let _e252 = d_1;
    let _e253 = d_min;
    let _e255 = d_max;
    let _e256 = d_min;
    x_mu = ((_e252 - _e253) / (_e255 - _e256));
    let _e260 = rho;
    let _e261 = H_3;
    x_r = (_e260 / _e261);
    let _e264 = x_mu;
    let _e266 = e1GetTextureCoordFromUnitRange(_e264, 256i);
    let _e267 = x_r;
    let _e269 = e1GetTextureCoordFromUnitRange(_e267, 64i);
    return vec2<f32>(_e266, _e269);
}

fn e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, e1Transmittance_texture_t: texture_2d<f32>, e1Transmittance_texture_s: sampler, r_7: f32, mu_5: f32) -> vec3<f32> {
    var uv_7: vec2<f32>;

    let _e229 = e1GetTransmittanceTextureUvFromRMu(atmosphere_5, r_7, mu_5);
    uv_7 = _e229;
    let _e231 = uv_7;
    let _e232 = textureSample(e1Transmittance_texture_t, e1Transmittance_texture_s, _e231);
    return vec3<f32>(_e232.xyz);
}

fn e1GetTransmittance(atmosphere_6: AtmosphereParameters, e1Transmittance_texture_t_1: texture_2d<f32>, e1Transmittance_texture_s_1: sampler, r_8: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e240 = e1ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_8) * mu_6) * d_2)) + (r_8 * r_8))));
    r_d = _e240;
    let _e244 = r_d;
    let _e246 = e1ClampCosine((((r_8 * mu_6) + d_2) / _e244));
    mu_d = _e246;
    if ray_r_mu_intersects_ground {
        {
            let _e248 = r_d;
            let _e249 = mu_d;
            let _e251 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, e1Transmittance_texture_t_1, e1Transmittance_texture_s_1, _e248, -(_e249));
            let _e253 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, e1Transmittance_texture_t_1, e1Transmittance_texture_s_1, r_8, -(mu_6));
            return min((_e251 / _e253), vec3(1f));
        }
    } else {
        {
            let _e258 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, e1Transmittance_texture_t_1, e1Transmittance_texture_s_1, r_8, mu_6);
            let _e259 = r_d;
            let _e260 = mu_d;
            let _e261 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, e1Transmittance_texture_t_1, e1Transmittance_texture_s_1, _e259, _e260);
            return min((_e258 / _e261), vec3(1f));
        }
    }
}

fn e1GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, e1Transmittance_texture_t_2: texture_2d<f32>, e1Transmittance_texture_s_2: sampler, r_9: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_9);
    let _e233 = sin_theta_h;
    let _e234 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e233 * _e234)), 0f)));
    let _e242 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, e1Transmittance_texture_t_2, e1Transmittance_texture_s_2, r_9, mu_s);
    let _e243 = sin_theta_h;
    let _e248 = sin_theta_h;
    let _e252 = cos_theta_h;
    return (_e242 * smoothstep(((-(_e243) * atmosphere_7.sun_angular_radius) / rad), ((_e248 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e252)));
}

fn e1RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e233 = k;
    return (_e233 * (1f + (nu * nu)));
}

fn e1MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e242 = k_1;
    let _e255 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e242 * (1f + (nu_1 * nu_1))) / _e255);
}

fn e1GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_8: AtmosphereParameters, r_10: f32, mu_7: f32, mu_s_1: f32, nu_2: f32, ray_r_mu_intersects_ground_1: bool) -> vec4<f32> {
    var H_4: f32;
    var rho_1: f32;
    var u_r: f32;
    var r_mu: f32;
    var discriminant_8: f32;
    var u_mu: f32;
    var d_3: f32;
    var d_min_1: f32;
    var d_max_1: f32;
    var local_8: f32;
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

    H_4 = sqrt(((atmosphere_8.top_radius * atmosphere_8.top_radius) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    let _e244 = e1SafeSqrt(((r_10 * r_10) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e244;
    let _e246 = rho_1;
    let _e247 = H_4;
    let _e250 = e1GetTextureCoordFromUnitRange((_e246 / _e247), 32i);
    u_r = _e250;
    r_mu = (r_10 * mu_7);
    let _e254 = r_mu;
    let _e255 = r_mu;
    discriminant_8 = (((_e254 * _e255) - (r_10 * r_10)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e265 = r_mu;
            let _e267 = discriminant_8;
            let _e268 = e1SafeSqrt(_e267);
            d_3 = (-(_e265) - _e268);
            d_min_1 = (r_10 - atmosphere_8.bottom_radius);
            let _e274 = rho_1;
            d_max_1 = _e274;
            let _e278 = d_max_1;
            let _e279 = d_min_1;
            if (_e278 == _e279) {
                local_8 = 0f;
            } else {
                let _e282 = d_3;
                let _e283 = d_min_1;
                let _e285 = d_max_1;
                let _e286 = d_min_1;
                local_8 = ((_e282 - _e283) / (_e285 - _e286));
            }
            let _e290 = local_8;
            let _e294 = e1GetTextureCoordFromUnitRange(_e290, 64i);
            u_mu = (0.5f - (0.5f * _e294));
        }
    } else {
        {
            let _e297 = r_mu;
            let _e299 = discriminant_8;
            let _e300 = H_4;
            let _e301 = H_4;
            let _e304 = e1SafeSqrt((_e299 + (_e300 * _e301)));
            d_4 = (-(_e297) + _e304);
            d_min_2 = (atmosphere_8.top_radius - r_10);
            let _e310 = rho_1;
            let _e311 = H_4;
            d_max_2 = (_e310 + _e311);
            let _e316 = d_4;
            let _e317 = d_min_2;
            let _e319 = d_max_2;
            let _e320 = d_min_2;
            let _e326 = e1GetTextureCoordFromUnitRange(((_e316 - _e317) / (_e319 - _e320)), 64i);
            u_mu = (0.5f + (0.5f * _e326));
        }
    }
    let _e330 = e1DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e330;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e336 = H_4;
    d_max_3 = _e336;
    let _e338 = d_5;
    let _e339 = d_min_3;
    let _e341 = d_max_3;
    let _e342 = d_min_3;
    a_8 = ((_e338 - _e339) / (_e341 - _e342));
    let _e348 = e1DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e348;
    let _e350 = D;
    let _e351 = d_min_3;
    let _e353 = d_max_3;
    let _e354 = d_min_3;
    A = ((_e350 - _e351) / (_e353 - _e354));
    let _e359 = a_8;
    let _e360 = A;
    let _e366 = a_8;
    let _e370 = e1GetTextureCoordFromUnitRange((max((1f - (_e359 / _e360)), 0f) / (1f + _e366)), 32i);
    u_mu_s = _e370;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e377 = u_nu;
    let _e378 = u_mu_s;
    let _e379 = u_mu;
    let _e380 = u_r;
    return vec4<f32>(_e377, _e378, _e379, _e380);
}

fn e1GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_11: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_11 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e239 = x_mu_s;
    let _e241 = e1GetTextureCoordFromUnitRange(_e239, 64i);
    let _e242 = x_r_1;
    let _e244 = e1GetTextureCoordFromUnitRange(_e242, 16i);
    return vec2<f32>(_e241, _e244);
}

fn e1GetIrradiance(atmosphere_10: AtmosphereParameters, e1Irradiance_texture_t: texture_2d<f32>, e1Irradiance_texture_s: sampler, r_12: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_8: vec2<f32>;

    let _e229 = e1GetIrradianceTextureUvFromRMuS(atmosphere_10, r_12, mu_s_3);
    uv_8 = _e229;
    let _e231 = uv_8;
    let _e232 = textureSample(e1Irradiance_texture_t, e1Irradiance_texture_s, _e231);
    return vec3<f32>(_e232.xyz);
}

fn e1GetExtrapolatedSingleMieScattering(atmosphere_11: AtmosphereParameters, scattering: vec4<f32>) -> vec3<f32> {
    if (scattering.x < 0.00001f) {
        {
            return vec3(0f);
        }
    }
    return ((((scattering.xyz * scattering.w) / vec3(scattering.x)) * (atmosphere_11.rayleigh_scattering.x / atmosphere_11.mie_scattering.x)) * (atmosphere_11.mie_scattering / atmosphere_11.rayleigh_scattering));
}

fn e1GetCombinedScattering(atmosphere_12: AtmosphereParameters, e1Scattering_texture_t: texture_3d<f32>, e1Scattering_texture_s: sampler, e1Single_mie_scattering_texture_t: texture_3d<f32>, e1Single_mie_scattering_texture_s: sampler, r_13: f32, mu_8: f32, mu_s_4: f32, nu_3: f32, ray_r_mu_intersects_ground_2: bool, single_mie_scattering: ptr<function, vec3<f32>>) -> vec3<f32> {
    var uvwz: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;
    var combined_scattering: vec4<f32>;
    var scattering_1: vec3<f32>;

    let _e235 = e1GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_12, r_13, mu_8, mu_s_4, nu_3, ray_r_mu_intersects_ground_2);
    uvwz = _e235;
    let _e237 = uvwz;
    tex_coord_x = (_e237.x * 7f);
    let _e245 = tex_coord_x;
    tex_x = floor(_e245);
    let _e248 = tex_coord_x;
    let _e249 = tex_x;
    lerp = (_e248 - _e249);
    let _e252 = tex_x;
    let _e253 = uvwz;
    let _e259 = uvwz;
    let _e261 = uvwz;
    uvw0_ = vec3<f32>(((_e252 + _e253.y) / 8f), _e259.z, _e261.w);
    let _e265 = tex_x;
    let _e268 = uvwz;
    let _e274 = uvwz;
    let _e276 = uvwz;
    uvw1_ = vec3<f32>((((_e265 + 1f) + _e268.y) / 8f), _e274.z, _e276.w);
    let _e280 = uvw0_;
    let _e281 = textureSample(e1Scattering_texture_t, e1Scattering_texture_s, _e280);
    let _e283 = lerp;
    let _e286 = uvw1_;
    let _e287 = textureSample(e1Scattering_texture_t, e1Scattering_texture_s, _e286);
    let _e288 = lerp;
    combined_scattering = ((_e281 * (1f - _e283)) + (_e287 * _e288));
    let _e292 = combined_scattering;
    scattering_1 = vec3<f32>(_e292.xyz);
    let _e296 = combined_scattering;
    let _e297 = e1GetExtrapolatedSingleMieScattering(atmosphere_12, _e296);
    (*single_mie_scattering) = _e297;
    let _e298 = scattering_1;
    return _e298;
}

fn e1GetScattering(atmosphere_13: AtmosphereParameters, e1Scattering_texture_t_1: texture_3d<f32>, e1Scattering_texture_s_1: sampler, r_14: f32, mu_9: f32, mu_s_5: f32, nu_4: f32, ray_r_mu_intersects_ground_3: bool) -> vec3<f32> {
    var uvwz_1: vec4<f32>;
    var tex_coord_x_1: f32;
    var tex_x_1: f32;
    var lerp_1: f32;
    var uvw0_1: vec3<f32>;
    var uvw1_1: vec3<f32>;
    var scattering_2: vec3<f32>;

    let _e232 = e1GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_13, r_14, mu_9, mu_s_5, nu_4, ray_r_mu_intersects_ground_3);
    uvwz_1 = _e232;
    let _e234 = uvwz_1;
    tex_coord_x_1 = (_e234.x * 7f);
    let _e242 = tex_coord_x_1;
    tex_x_1 = floor(_e242);
    let _e245 = tex_coord_x_1;
    let _e246 = tex_x_1;
    lerp_1 = (_e245 - _e246);
    let _e249 = tex_x_1;
    let _e250 = uvwz_1;
    let _e256 = uvwz_1;
    let _e258 = uvwz_1;
    uvw0_1 = vec3<f32>(((_e249 + _e250.y) / 8f), _e256.z, _e258.w);
    let _e262 = tex_x_1;
    let _e265 = uvwz_1;
    let _e271 = uvwz_1;
    let _e273 = uvwz_1;
    uvw1_1 = vec3<f32>((((_e262 + 1f) + _e265.y) / 8f), _e271.z, _e273.w);
    let _e277 = uvw0_1;
    let _e278 = textureSample(e1Scattering_texture_t_1, e1Scattering_texture_s_1, _e277);
    let _e280 = lerp_1;
    let _e283 = uvw1_1;
    let _e284 = textureSample(e1Scattering_texture_t_1, e1Scattering_texture_s_1, _e283);
    let _e285 = lerp_1;
    scattering_2 = vec3<f32>(((_e278 * (1f - _e280)) + (_e284 * _e285)).xyz);
    let _e291 = scattering_2;
    return _e291;
}

fn e1GetSkyRadiance(atmosphere_14: AtmosphereParameters, e1Transmittance_texture_t_3: texture_2d<f32>, e1Transmittance_texture_s_3: sampler, e1Scattering_texture_t_2: texture_3d<f32>, e1Scattering_texture_s_2: sampler, e1Single_mie_scattering_texture_t_1: texture_3d<f32>, e1Single_mie_scattering_texture_s_1: sampler, camera: vec3<f32>, view_ray: vec3<f32>, shadow_length: f32, sun_direction: vec3<f32>, transmittance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_1: vec3<f32>;
    var r_15: f32;
    var rmu: f32;
    var distance_to_top_atmosphere_boundary: f32;
    var mu_10: f32;
    var mu_s_6: f32;
    var nu_5: f32;
    var ray_r_mu_intersects_ground_4: bool;
    var local_9: vec3<f32>;
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
    let _e237 = camera_1;
    r_15 = length(_e237);
    let _e240 = camera_1;
    rmu = dot(_e240, view_ray);
    let _e243 = rmu;
    let _e245 = rmu;
    let _e246 = rmu;
    let _e248 = r_15;
    let _e249 = r_15;
    let _e256 = e1SafeSqrt((((_e245 * _e246) - (_e248 * _e249)) + (atmosphere_14.top_radius * atmosphere_14.top_radius)));
    distance_to_top_atmosphere_boundary = (-(_e243) - _e256);
    let _e259 = distance_to_top_atmosphere_boundary;
    if (_e259 > 0f) {
        {
            let _e264 = camera_1;
            let _e265 = distance_to_top_atmosphere_boundary;
            camera_1 = (_e264 + (view_ray * _e265));
            r_15 = atmosphere_14.top_radius;
            let _e269 = rmu;
            let _e270 = distance_to_top_atmosphere_boundary;
            rmu = (_e269 + _e270);
        }
    } else {
        let _e272 = r_15;
        if (_e272 > atmosphere_14.top_radius) {
            {
                (*transmittance) = vec3(1f);
                return vec3(0f);
            }
        }
    }
    let _e281 = rmu;
    let _e282 = r_15;
    mu_10 = (_e281 / _e282);
    let _e285 = camera_1;
    let _e287 = r_15;
    mu_s_6 = (dot(_e285, sun_direction) / _e287);
    nu_5 = dot(view_ray, sun_direction);
    let _e292 = r_15;
    let _e293 = mu_10;
    let _e294 = e1RayIntersectsGround(atmosphere_14, _e292, _e293);
    ray_r_mu_intersects_ground_4 = _e294;
    let _e296 = ray_r_mu_intersects_ground_4;
    if _e296 {
        local_9 = vec3(0f);
    } else {
        let _e299 = r_15;
        let _e300 = mu_10;
        let _e301 = e1GetTransmittanceToTopAtmosphereBoundary(atmosphere_14, e1Transmittance_texture_t_3, e1Transmittance_texture_s_3, _e299, _e300);
        local_9 = _e301;
    }
    let _e303 = local_9;
    (*transmittance) = _e303;
    if (shadow_length == 0f) {
        {
            let _e310 = r_15;
            let _e311 = mu_10;
            let _e312 = mu_s_6;
            let _e313 = nu_5;
            let _e314 = ray_r_mu_intersects_ground_4;
            let _e317 = e1GetCombinedScattering(atmosphere_14, e1Scattering_texture_t_2, e1Scattering_texture_s_2, e1Single_mie_scattering_texture_t_1, e1Single_mie_scattering_texture_s_1, _e310, _e311, _e312, _e313, _e314, (&single_mie_scattering_1));
            scattering_3 = _e317;
        }
    } else {
        {
            d_6 = shadow_length;
            let _e319 = d_6;
            let _e320 = d_6;
            let _e323 = r_15;
            let _e325 = mu_10;
            let _e327 = d_6;
            let _e330 = r_15;
            let _e331 = r_15;
            let _e335 = e1ClampRadius(atmosphere_14, sqrt((((_e319 * _e320) + (((2f * _e323) * _e325) * _e327)) + (_e330 * _e331))));
            r_p = _e335;
            let _e337 = r_15;
            let _e338 = mu_10;
            let _e340 = d_6;
            let _e342 = r_p;
            mu_p = (((_e337 * _e338) + _e340) / _e342);
            let _e345 = r_15;
            let _e346 = mu_s_6;
            let _e348 = d_6;
            let _e349 = nu_5;
            let _e352 = r_p;
            mu_s_p = (((_e345 * _e346) + (_e348 * _e349)) / _e352);
            let _e355 = r_p;
            let _e356 = mu_p;
            let _e357 = mu_s_p;
            let _e358 = nu_5;
            let _e359 = ray_r_mu_intersects_ground_4;
            let _e362 = e1GetCombinedScattering(atmosphere_14, e1Scattering_texture_t_2, e1Scattering_texture_s_2, e1Single_mie_scattering_texture_t_1, e1Single_mie_scattering_texture_s_1, _e355, _e356, _e357, _e358, _e359, (&single_mie_scattering_1));
            scattering_3 = _e362;
            let _e363 = r_15;
            let _e364 = mu_10;
            let _e365 = ray_r_mu_intersects_ground_4;
            let _e366 = e1GetTransmittance(atmosphere_14, e1Transmittance_texture_t_3, e1Transmittance_texture_s_3, _e363, _e364, shadow_length, _e365);
            shadow_transmittance = _e366;
            let _e368 = r_p;
            let _e369 = mu_p;
            let _e370 = mu_s_p;
            let _e371 = nu_5;
            let _e372 = ray_r_mu_intersects_ground_4;
            let _e373 = e1GetScattering(atmosphere_14, e1Higher_order_scattering_texture_t, e1Higher_order_scattering_texture_s, _e368, _e369, _e370, _e371, _e372);
            higher_order_scattering = _e373;
            let _e375 = scattering_3;
            let _e376 = higher_order_scattering;
            single_scattering = (_e375 - _e376);
            let _e379 = single_scattering;
            let _e380 = shadow_transmittance;
            let _e382 = higher_order_scattering;
            scattering_3 = ((_e379 * _e380) + _e382);
            let _e384 = single_mie_scattering_1;
            let _e385 = shadow_transmittance;
            single_mie_scattering_1 = (_e384 * _e385);
        }
    }
    let _e387 = scattering_3;
    let _e388 = nu_5;
    let _e389 = e1RayleighPhaseFunction(_e388);
    let _e391 = single_mie_scattering_1;
    let _e393 = nu_5;
    let _e394 = e1MiePhaseFunction(atmosphere_14.mie_phase_function_g, _e393);
    return ((_e387 * _e389) + (_e391 * _e394));
}

fn e1ClosestPointOnRay(camera_2: vec3<f32>, point: vec3<f32>) -> vec3<f32> {
    var ray: vec3<f32>;
    var t: f32;

    ray = (point - camera_2);
    let _e228 = ray;
    let _e231 = ray;
    let _e232 = ray;
    t = clamp((-(dot(camera_2, _e228)) / dot(_e231, _e232)), 0f, 1f);
    let _e239 = t;
    let _e240 = ray;
    return (camera_2 + (_e239 * _e240));
}

fn e1RaySphereIntersections_4(camera_3: vec3<f32>, direction_12: vec3<f32>, radius_12: f32) -> vec2<f32> {
    var b_15: f32;
    var c_7: f32;
    var discriminant_9: f32;
    var Q_3: f32;

    b_15 = (2f * dot(direction_12, camera_3));
    c_7 = (dot(camera_3, camera_3) - (radius_12 * radius_12));
    let _e235 = b_15;
    let _e236 = b_15;
    let _e239 = c_7;
    discriminant_9 = ((_e235 * _e236) - (4f * _e239));
    let _e243 = discriminant_9;
    Q_3 = sqrt(_e243);
    let _e246 = b_15;
    let _e248 = Q_3;
    let _e250 = b_15;
    let _e252 = Q_3;
    return (vec2<f32>((-(_e246) - _e248), (-(_e250) + _e252)) * 0.5f);
}

fn e1ClipAtBottomAtmosphere(atmosphere_15: AtmosphereParameters, view_ray_1: vec3<f32>, camera_4: ptr<function, vec3<f32>>, point_1: ptr<function, vec3<f32>>) -> bool {
    var eps: f32 = 0f;
    var bottom_radius: f32;
    var r_camera: f32;
    var r_point: f32;
    var camera_below: bool;
    var point_below: bool;
    var t_1: vec2<f32>;
    var local_10: f32;
    var intersection: vec3<f32>;
    var local_11: vec3<f32>;
    var local_12: vec3<f32>;

    let _e231 = eps;
    bottom_radius = (atmosphere_15.bottom_radius + _e231);
    let _e234 = (*camera_4);
    r_camera = length(_e234);
    let _e237 = (*point_1);
    r_point = length(_e237);
    let _e240 = r_camera;
    let _e241 = bottom_radius;
    camera_below = (_e240 < _e241);
    let _e244 = r_point;
    let _e245 = bottom_radius;
    point_below = (_e244 < _e245);
    let _e248 = (*camera_4);
    let _e249 = bottom_radius;
    let _e250 = e1RaySphereIntersections_4(_e248, view_ray_1, _e249);
    t_1 = _e250;
    let _e252 = (*camera_4);
    let _e253 = camera_below;
    if _e253 {
        let _e254 = t_1;
        local_10 = _e254.y;
    } else {
        let _e256 = t_1;
        local_10 = _e256.x;
    }
    let _e259 = local_10;
    intersection = (_e252 + (view_ray_1 * _e259));
    let _e263 = camera_below;
    if _e263 {
        let _e264 = intersection;
        local_11 = _e264;
    } else {
        let _e265 = (*camera_4);
        local_11 = _e265;
    }
    let _e267 = local_11;
    (*camera_4) = _e267;
    let _e268 = point_below;
    if _e268 {
        let _e269 = intersection;
        local_12 = _e269;
    } else {
        let _e270 = (*point_1);
        local_12 = _e270;
    }
    let _e272 = local_12;
    (*point_1) = _e272;
    let _e273 = camera_below;
    let _e274 = point_below;
    return (_e273 && _e274);
}

fn e1GetSkyRadianceToPoint(atmosphere_16: AtmosphereParameters, e1Transmittance_texture_t_4: texture_2d<f32>, e1Transmittance_texture_s_4: sampler, e1Scattering_texture_t_3: texture_3d<f32>, e1Scattering_texture_s_3: sampler, e1Single_mie_scattering_texture_t_2: texture_3d<f32>, e1Single_mie_scattering_texture_s_2: sampler, camera_5: vec3<f32>, point_2: vec3<f32>, shadow_length_1: f32, sun_direction_1: vec3<f32>, transmittance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var camera_6: vec3<f32>;
    var point_3: vec3<f32>;
    var view_ray_2: vec3<f32>;
    var r_16: f32;
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
    let _e238 = camera_6;
    let _e239 = point_3;
    let _e240 = e1ClosestPointOnRay(_e238, _e239);
    if (length(_e240) > atmosphere_16.top_radius) {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e248 = point_3;
    let _e249 = camera_6;
    view_ray_2 = normalize((_e248 - _e249));
    let _e253 = view_ray_2;
    let _e258 = e1ClipAtBottomAtmosphere(atmosphere_16, _e253, (&camera_6), (&point_3));
    if _e258 {
        {
            (*transmittance_1) = vec3(1f);
            return vec3(0f);
        }
    }
    let _e263 = camera_6;
    r_16 = length(_e263);
    let _e266 = camera_6;
    let _e267 = view_ray_2;
    rmu_1 = dot(_e266, _e267);
    let _e270 = rmu_1;
    let _e272 = rmu_1;
    let _e273 = rmu_1;
    let _e275 = r_16;
    let _e276 = r_16;
    let _e283 = e1SafeSqrt((((_e272 * _e273) - (_e275 * _e276)) + (atmosphere_16.top_radius * atmosphere_16.top_radius)));
    distance_to_top_atmosphere_boundary_1 = (-(_e270) - _e283);
    let _e286 = distance_to_top_atmosphere_boundary_1;
    if (_e286 > 0f) {
        {
            let _e291 = camera_6;
            let _e292 = view_ray_2;
            let _e293 = distance_to_top_atmosphere_boundary_1;
            camera_6 = (_e291 + (_e292 * _e293));
            r_16 = atmosphere_16.top_radius;
            let _e297 = rmu_1;
            let _e298 = distance_to_top_atmosphere_boundary_1;
            rmu_1 = (_e297 + _e298);
        }
    }
    let _e300 = rmu_1;
    let _e301 = r_16;
    mu_11 = (_e300 / _e301);
    let _e304 = camera_6;
    let _e306 = r_16;
    mu_s_7 = (dot(_e304, sun_direction_1) / _e306);
    let _e309 = view_ray_2;
    nu_6 = dot(_e309, sun_direction_1);
    let _e312 = point_3;
    let _e313 = camera_6;
    d_7 = length((_e312 - _e313));
    let _e317 = r_16;
    let _e318 = mu_11;
    let _e319 = e1RayIntersectsGround(atmosphere_16, _e317, _e318);
    ray_r_mu_intersects_ground_5 = _e319;
    let _e321 = ray_r_mu_intersects_ground_5;
    if !(_e321) {
        {
            let _e327 = r_16;
            let _e328 = r_16;
            let _e332 = e1SafeSqrt((1f - ((atmosphere_16.bottom_radius * atmosphere_16.bottom_radius) / (_e327 * _e328))));
            mu_horizon = -(_e332);
            let _e337 = mu_11;
            let _e338 = mu_horizon;
            let _e339 = eps_1;
            mu_11 = max(_e337, (_e338 + _e339));
        }
    }
    let _e342 = r_16;
    let _e343 = mu_11;
    let _e344 = d_7;
    let _e345 = ray_r_mu_intersects_ground_5;
    let _e346 = e1GetTransmittance(atmosphere_16, e1Transmittance_texture_t_4, e1Transmittance_texture_s_4, _e342, _e343, _e344, _e345);
    (*transmittance_1) = _e346;
    let _e348 = r_16;
    let _e349 = mu_11;
    let _e350 = mu_s_7;
    let _e351 = nu_6;
    let _e352 = ray_r_mu_intersects_ground_5;
    let _e355 = e1GetCombinedScattering(atmosphere_16, e1Scattering_texture_t_3, e1Scattering_texture_s_3, e1Single_mie_scattering_texture_t_2, e1Single_mie_scattering_texture_s_2, _e348, _e349, _e350, _e351, _e352, (&single_mie_scattering_2));
    scattering_4 = _e355;
    let _e357 = d_7;
    d_7 = max((_e357 - shadow_length_1), 0f);
    let _e363 = d_7;
    let _e364 = d_7;
    let _e367 = r_16;
    let _e369 = mu_11;
    let _e371 = d_7;
    let _e374 = r_16;
    let _e375 = r_16;
    let _e379 = e1ClampRadius(atmosphere_16, sqrt((((_e363 * _e364) + (((2f * _e367) * _e369) * _e371)) + (_e374 * _e375))));
    r_p_1 = _e379;
    let _e381 = r_16;
    let _e382 = mu_11;
    let _e384 = d_7;
    let _e386 = r_p_1;
    mu_p_1 = (((_e381 * _e382) + _e384) / _e386);
    let _e389 = r_16;
    let _e390 = mu_s_7;
    let _e392 = d_7;
    let _e393 = nu_6;
    let _e396 = r_p_1;
    mu_s_p_1 = (((_e389 * _e390) + (_e392 * _e393)) / _e396);
    let _e400 = r_p_1;
    let _e401 = mu_p_1;
    let _e402 = mu_s_p_1;
    let _e403 = nu_6;
    let _e404 = ray_r_mu_intersects_ground_5;
    let _e407 = e1GetCombinedScattering(atmosphere_16, e1Scattering_texture_t_3, e1Scattering_texture_s_3, e1Single_mie_scattering_texture_t_2, e1Single_mie_scattering_texture_s_2, _e400, _e401, _e402, _e403, _e404, (&single_mie_scattering_p));
    scattering_p = _e407;
    let _e409 = (*transmittance_1);
    shadow_transmittance_1 = _e409;
    if (shadow_length_1 > 0f) {
        {
            let _e415 = r_16;
            let _e416 = mu_11;
            let _e417 = d_7;
            let _e418 = ray_r_mu_intersects_ground_5;
            let _e419 = e1GetTransmittance(atmosphere_16, e1Transmittance_texture_t_4, e1Transmittance_texture_s_4, _e415, _e416, _e417, _e418);
            shadow_transmittance_1 = _e419;
        }
    }
    let _e420 = r_16;
    let _e421 = mu_11;
    let _e422 = mu_s_7;
    let _e423 = nu_6;
    let _e424 = ray_r_mu_intersects_ground_5;
    let _e425 = e1GetScattering(atmosphere_16, e1Higher_order_scattering_texture_t, e1Higher_order_scattering_texture_s, _e420, _e421, _e422, _e423, _e424);
    higher_order_scattering_1 = _e425;
    let _e427 = scattering_4;
    let _e428 = higher_order_scattering_1;
    single_scattering_1 = (_e427 - _e428);
    let _e431 = r_p_1;
    let _e432 = mu_p_1;
    let _e433 = mu_s_p_1;
    let _e434 = nu_6;
    let _e435 = ray_r_mu_intersects_ground_5;
    let _e436 = e1GetScattering(atmosphere_16, e1Higher_order_scattering_texture_t, e1Higher_order_scattering_texture_s, _e431, _e432, _e433, _e434, _e435);
    higher_order_scattering_p = _e436;
    let _e438 = scattering_p;
    let _e439 = higher_order_scattering_p;
    single_scattering_p = (_e438 - _e439);
    let _e442 = single_scattering_1;
    let _e443 = shadow_transmittance_1;
    let _e444 = single_scattering_p;
    let _e447 = higher_order_scattering_1;
    let _e449 = (*transmittance_1);
    let _e450 = higher_order_scattering_p;
    scattering_4 = (((_e442 - (_e443 * _e444)) + _e447) - (_e449 * _e450));
    let _e453 = single_mie_scattering_2;
    let _e454 = shadow_transmittance_1;
    let _e455 = single_mie_scattering_p;
    single_mie_scattering_2 = (_e453 - (_e454 * _e455));
    let _e458 = scattering_4;
    let _e459 = single_mie_scattering_2;
    let _e465 = e1GetExtrapolatedSingleMieScattering(atmosphere_16, vec4<f32>(_e458.x, _e458.y, _e458.z, _e459.x));
    single_mie_scattering_2 = _e465;
    let _e466 = single_mie_scattering_2;
    let _e471 = mu_s_7;
    single_mie_scattering_2 = (_e466 * smoothstep(0f, 0.01f, _e471));
    let _e474 = scattering_4;
    let _e475 = nu_6;
    let _e476 = e1RayleighPhaseFunction(_e475);
    let _e478 = single_mie_scattering_2;
    let _e480 = nu_6;
    let _e481 = e1MiePhaseFunction(atmosphere_16.mie_phase_function_g, _e480);
    return ((_e474 * _e476) + (_e478 * _e481));
}

fn e1GetSunAndSkyIrradiance(atmosphere_17: AtmosphereParameters, e1Transmittance_texture_t_5: texture_2d<f32>, e1Transmittance_texture_s_5: sampler, e1Irradiance_texture_t_1: texture_2d<f32>, e1Irradiance_texture_s_1: sampler, point_4: vec3<f32>, normal_4: vec3<f32>, sun_direction_2: vec3<f32>, sky_irradiance: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_17: f32;
    var mu_s_8: f32;

    r_17 = length(point_4);
    let _e236 = r_17;
    mu_s_8 = (dot(point_4, sun_direction_2) / _e236);
    let _e239 = r_17;
    let _e240 = mu_s_8;
    let _e241 = e1GetIrradiance(atmosphere_17, e1Irradiance_texture_t_1, e1Irradiance_texture_s_1, _e239, _e240);
    let _e244 = r_17;
    (*sky_irradiance) = ((_e241 * (1f + (dot(normal_4, point_4) / _e244))) * 0.5f);
    let _e251 = r_17;
    let _e252 = mu_s_8;
    let _e253 = e1GetTransmittanceToSun(atmosphere_17, e1Transmittance_texture_t_5, e1Transmittance_texture_s_5, _e251, _e252);
    return ((atmosphere_17.solar_irradiance * _e253) * max(dot(normal_4, sun_direction_2), 0f));
}

fn e1GetSunAndSkyScalarIrradiance(atmosphere_18: AtmosphereParameters, e1Transmittance_texture_t_6: texture_2d<f32>, e1Transmittance_texture_s_6: sampler, e1Irradiance_texture_t_2: texture_2d<f32>, e1Irradiance_texture_s_2: sampler, point_5: vec3<f32>, sun_direction_3: vec3<f32>, sky_irradiance_1: ptr<function, vec3<f32>>) -> vec3<f32> {
    var r_18: f32;
    var mu_s_9: f32;

    r_18 = length(point_5);
    let _e235 = r_18;
    mu_s_9 = (dot(point_5, sun_direction_3) / _e235);
    let _e238 = r_18;
    let _e239 = mu_s_9;
    let _e240 = e1GetIrradiance(atmosphere_18, e1Irradiance_texture_t_2, e1Irradiance_texture_s_2, _e238, _e239);
    (*sky_irradiance_1) = ((_e240 * 2f) * 3.1415927f);
    let _e246 = r_18;
    let _e247 = mu_s_9;
    let _e248 = e1GetTransmittanceToSun(atmosphere_18, e1Transmittance_texture_t_6, e1Transmittance_texture_s_6, _e246, _e247);
    return (atmosphere_18.solar_irradiance * _e248);
}

fn e1GetSolarLuminance() -> vec3<f32> {
    let _e224 = e1ATMOSPHERE;
    let _e227 = e1ATMOSPHERE;
    let _e230 = e1ATMOSPHERE;
    let _e235 = global.e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return ((_e224.solar_irradiance / vec3(((3.1415927f * _e227.sun_angular_radius) * _e230.sun_angular_radius))) * _e235);
}

fn e1GetSkyLuminance(camera_7: vec3<f32>, view_ray_3: vec3<f32>, shadow_length_2: f32, sun_direction_4: vec3<f32>, transmittance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var view_ray_4: vec3<f32>;

    view_ray_4 = view_ray_3;
    let _e230 = e1ATMOSPHERE;
    let _e231 = view_ray_4;
    let _e234 = e1GetSkyRadiance(_e230, e1Transmittance_texture_t_7, e1Transmittance_texture_s_7, e1Scattering_texture_t_4, e1Scattering_texture_s_4, e1Single_mie_scattering_texture_t_3, e1Single_mie_scattering_texture_s_3, camera_7, _e231, shadow_length_2, sun_direction_4, transmittance_2);
    let _e235 = global.e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e234 * _e235);
}

fn e1GetSkyLuminanceToPoint(camera_8: vec3<f32>, point_6: vec3<f32>, shadow_length_3: f32, sun_direction_5: vec3<f32>, transmittance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    let _e229 = e1ATMOSPHERE;
    let _e232 = e1GetSkyRadianceToPoint(_e229, e1Transmittance_texture_t_7, e1Transmittance_texture_s_7, e1Scattering_texture_t_4, e1Scattering_texture_s_4, e1Single_mie_scattering_texture_t_3, e1Single_mie_scattering_texture_s_3, camera_8, point_6, shadow_length_3, sun_direction_5, transmittance_3);
    let _e233 = global.e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e232 * _e233);
}

fn e1GetSunAndSkyIlluminance(p_1: vec3<f32>, normal_5: vec3<f32>, sun_direction_6: vec3<f32>, sky_irradiance_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance: vec3<f32>;

    let _e228 = e1ATMOSPHERE;
    let _e231 = e1GetSunAndSkyIrradiance(_e228, e1Transmittance_texture_t_7, e1Transmittance_texture_s_7, e1Irradiance_texture_t_3, e1Irradiance_texture_s_3, p_1, normal_5, sun_direction_6, sky_irradiance_2);
    sun_irradiance = _e231;
    let _e233 = (*sky_irradiance_2);
    let _e234 = global.e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_2) = (_e233 * _e234);
    let _e236 = sun_irradiance;
    let _e237 = global.e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e236 * _e237);
}

fn e1GetSunAndSkyScalarIlluminance(p_2: vec3<f32>, sun_direction_7: vec3<f32>, sky_irradiance_3: ptr<function, vec3<f32>>) -> vec3<f32> {
    var sun_irradiance_1: vec3<f32>;

    let _e227 = e1ATMOSPHERE;
    let _e230 = e1GetSunAndSkyScalarIrradiance(_e227, e1Transmittance_texture_t_7, e1Transmittance_texture_s_7, e1Irradiance_texture_t_3, e1Irradiance_texture_s_3, p_2, sun_direction_7, sky_irradiance_3);
    sun_irradiance_1 = _e230;
    let _e232 = (*sky_irradiance_3);
    let _e233 = global.e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
    (*sky_irradiance_3) = (_e232 * _e233);
    let _e235 = sun_irradiance_1;
    let _e236 = global.e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    return (_e235 * _e236);
}

fn e1GetLunarRadiance(e1MoonAngularRadius: f32) -> vec3<f32> {
    var radiance: vec3<f32>;

    let _e225 = e1ATMOSPHERE;
    let _e234 = global.e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
    radiance = (((_e225.solar_irradiance * 0.000002f) / vec3(((3.1415927f * e1MoonAngularRadius) * e1MoonAngularRadius))) * _e234);
    let _e237 = radiance;
    return _e237;
}

fn e1IntersectSphere(ray_1: vec3<f32>, point_7: vec3<f32>, radius_13: f32) -> f32 {
    var P_1: vec3<f32>;
    var PoR: f32;
    var D_1: f32;

    P_1 = -(point_7);
    let _e229 = P_1;
    PoR = dot(_e229, ray_1);
    let _e232 = P_1;
    let _e233 = P_1;
    D_1 = (dot(_e232, _e233) - (radius_13 * radius_13));
    let _e238 = PoR;
    let _e240 = PoR;
    let _e241 = PoR;
    let _e243 = D_1;
    return (-(_e238) - sqrt(((_e240 * _e241) - _e243)));
}

fn e1OrenNayarDiffuse(L_1: vec3<f32>, V: vec3<f32>, N: vec3<f32>) -> f32 {
    var NoL: f32;
    var NoV: f32;
    var s: f32;
    var t_2: f32;

    NoL = dot(N, L_1);
    NoV = dot(N, V);
    let _e232 = NoL;
    let _e233 = NoV;
    s = (dot(L_1, V) - (_e232 * _e233));
    let _e238 = NoL;
    let _e239 = NoV;
    let _e242 = s;
    t_2 = mix(1f, max(_e238, _e239), step(0f, _e242));
    let _e247 = NoL;
    let _e251 = s;
    let _e253 = t_2;
    return (max(0f, _e247) * (0.62406015f + ((0.41284403f * _e251) / _e253)));
}

fn e1GetSkyLuminance_1(e1CameraPosition: vec3<f32>, rayDirection: vec3<f32>, shadowLength: f32, e1SunDirection: vec3<f32>, e1MoonDirection: vec3<f32>, e1MoonAngularRadius_1: f32, e1LunarRadianceScale: f32, fragmentAngle: f32) -> vec3<f32> {
    var transmittance_4: vec3<f32>;
    var radiance_1: vec3<f32>;
    var viewDotSun: f32;
    var angle: f32;
    var antialias: f32;
    var intersection_1: f32;
    var normal_6: vec3<f32>;
    var diffuse: f32;
    var viewDotMoon: f32;
    var angle_1: f32;
    var antialias_1: f32;

    let _e235 = e1GetSkyLuminance(e1CameraPosition, rayDirection, shadowLength, e1SunDirection, (&transmittance_4));
    radiance_1 = _e235;
    viewDotSun = dot(rayDirection, e1SunDirection);
    let _e239 = viewDotSun;
    let _e240 = global.e1CosSunAngularRadius;
    if (_e239 > _e240) {
        {
            let _e242 = viewDotSun;
            angle = acos(clamp(_e242, -1f, 1f));
            let _e249 = e1ATMOSPHERE;
            let _e251 = e1ATMOSPHERE;
            let _e254 = angle;
            antialias = smoothstep(_e249.sun_angular_radius, (_e251.sun_angular_radius - fragmentAngle), _e254);
            let _e257 = radiance_1;
            let _e258 = transmittance_4;
            let _e259 = e1GetSolarLuminance();
            let _e261 = antialias;
            radiance_1 = (_e257 + ((_e258 * _e259) * _e261));
        }
    }
    let _e264 = e1IntersectSphere(rayDirection, e1MoonDirection, e1MoonAngularRadius_1);
    intersection_1 = _e264;
    let _e266 = intersection_1;
    if (_e266 > 0f) {
        {
            let _e269 = intersection_1;
            normal_6 = normalize((e1MoonDirection - (rayDirection * _e269)));
            let _e275 = normal_6;
            let _e276 = e1OrenNayarDiffuse(-(e1SunDirection), rayDirection, _e275);
            diffuse = _e276;
            viewDotMoon = dot(rayDirection, e1MoonDirection);
            let _e280 = viewDotMoon;
            angle_1 = acos(clamp(_e280, -1f, 1f));
            let _e288 = angle_1;
            antialias_1 = smoothstep(e1MoonAngularRadius_1, (e1MoonAngularRadius_1 - fragmentAngle), _e288);
            let _e291 = radiance_1;
            let _e292 = transmittance_4;
            let _e293 = e1GetLunarRadiance(e1MoonAngularRadius_1);
            let _e296 = diffuse;
            let _e298 = antialias_1;
            radiance_1 = (_e291 + ((((_e292 * _e293) * e1LunarRadianceScale) * _e296) * _e298));
        }
    }
    let _e301 = radiance_1;
    return _e301;
}

fn e1ReadNormal(uv_9: vec2<f32>, degenerate: ptr<function, bool>) -> vec3<f32> {
    var normal_7: vec3<f32>;

    let _e226 = textureSample(e1NormalBuffer_t, e1NormalBuffer_s, uv_9);
    normal_7 = _e226.xyz;
    let _e229 = normal_7;
    (*degenerate) = all((_e229 == vec3(0f)));
    let _e235 = normal_7;
    return ((2f * _e235) - vec3(1f));
}

fn e1CorrectGeometricError(positionECEF: ptr<function, vec3<f32>>, normalECEF: ptr<function, vec3<f32>>) {
    var sphereNormal: vec3<f32>;
    var spherePosition: vec3<f32>;

    let _e226 = (*positionECEF);
    let _e227 = e1VEllipsoidRadiiSquared_1;
    sphereNormal = normalize((_e226 / _e227));
    let _e231 = e1ATMOSPHERE;
    let _e233 = sphereNormal;
    spherePosition = (_e231.bottom_radius * _e233);
    let _e236 = (*normalECEF);
    let _e237 = sphereNormal;
    let _e238 = global.e1GeometricErrorCorrectionAmount;
    (*normalECEF) = mix(_e236, _e237, vec3(_e238));
    let _e241 = (*positionECEF);
    let _e242 = spherePosition;
    let _e243 = global.e1GeometricErrorCorrectionAmount;
    (*positionECEF) = mix(_e241, _e242, vec3(_e243));
    return;
}

fn e1GetSunSkyIrradiance(positionECEF_1: vec3<f32>, normal_8: vec3<f32>, inputColor_1: vec3<f32>, sunTransmittance: f32) -> vec3<f32> {
    var diffuse_1: vec3<f32>;
    var skyIrradiance: vec3<f32>;
    var sunIrradiance: vec3<f32>;

    let _e228 = global.e1AlbedoScale;
    diffuse_1 = ((inputColor_1 * _e228) * 0.31830987f);
    let _e234 = global.e1SunDirection;
    let _e237 = e1GetSunAndSkyIlluminance(positionECEF_1, normal_8, _e234, (&skyIrradiance));
    sunIrradiance = _e237;
    let _e239 = sunIrradiance;
    sunIrradiance = (_e239 * sunTransmittance);
    let _e241 = diffuse_1;
    let _e242 = sunIrradiance;
    let _e243 = skyIrradiance;
    return (_e241 * (_e242 + _e243));
}

fn e1ApplyTransmittanceInscatter(positionECEF_2: vec3<f32>, shadowLength_1: f32, radiance_2: ptr<function, vec3<f32>>) {
    var shadowLength_2: f32;
    var transmittance_5: vec3<f32>;
    var inscatter: vec3<f32>;

    shadowLength_2 = shadowLength_1;
    let _e229 = e1VCameraPosition_1;
    let _e230 = shadowLength_2;
    let _e231 = global.e1SunDirection;
    let _e234 = e1GetSkyLuminanceToPoint(_e229, positionECEF_2, _e230, _e231, (&transmittance_5));
    inscatter = _e234;
    let _e236 = (*radiance_2);
    let _e237 = transmittance_5;
    (*radiance_2) = (_e236 * _e237);
    let _e239 = (*radiance_2);
    let _e240 = inscatter;
    (*radiance_2) = (_e239 + _e240);
    return;
}

fn e1GetSTBN() -> f32 {
    var size: vec3<i32>;
    var scale: vec3<f32>;

    let _e225 = textureDimensions(e1StbnTexture_t, 0i);
    size = vec3<i32>(_e225);
    let _e229 = size;
    scale = (vec3(1f) / vec3<f32>(_e229));
    let _e235 = gl_FragCoord_1;
    let _e236 = _e235.xy;
    let _e237 = global.e1Frame;
    let _e238 = size;
    let _e245 = scale;
    let _e247 = textureSample(e1StbnTexture_t, e1StbnTexture_s, (vec3<f32>(_e236.x, _e236.y, f32((_e237 % _e238.z))) * _e245));
    return _e247.x;
}

fn e1GetShadowUv(worldPosition_2: vec3<f32>, cascadeIndex: i32) -> vec2<f32> {
    var clip_1: vec4<f32>;

    let _e228 = global.e1ShadowMatrices[cascadeIndex];
    clip_1 = (_e228 * vec4<f32>(worldPosition_2.x, worldPosition_2.y, worldPosition_2.z, 1f));
    let _e236 = clip_1;
    let _e237 = clip_1;
    clip_1 = (_e236 / vec4(_e237.w));
    let _e241 = clip_1;
    return ((_e241.xy * 0.5f) + vec2(0.5f));
}

fn e1GetDistanceToShadowTop(positionECEF_3: vec3<f32>) -> f32 {
    let _e229 = global.e1SunDirection;
    let _e232 = global.e1BottomRadius;
    let _e233 = global.e1ShadowTopHeight;
    let _e235 = e1RaySphereSecondIntersection((positionECEF_3 / vec3(0.001f)), _e229, vec3(0f), (_e232 + _e233));
    return _e235;
}

fn e1ReadShadowOpticalDepth(uv_10: vec2<f32>, distanceToTop: f32, cascadeIndex_1: i32) -> f32 {
    var shadow: vec4<f32>;

    let _e231 = vec3<f32>(uv_10.x, uv_10.y, f32(cascadeIndex_1));
    let _e235 = textureSample(e1ShadowBuffer_t, e1ShadowBuffer_s, _e231.xy, i32(_e231.z));
    shadow = _e235;
    let _e237 = shadow;
    let _e239 = shadow;
    let _e242 = shadow;
    return min(_e237.z, (_e239.y * max(0f, (distanceToTop - _e242.x))));
}

fn e1SampleShadowOpticalDepthPCF(worldPosition_3: vec3<f32>, distanceToTop_1: f32, radius_14: f32, cascadeIndex_2: i32) -> f32 {
    var uv_11: vec2<f32>;
    var texelSize: vec2<f32>;
    var sum: f32 = 0f;
    var offset: vec2<f32>;

    let _e229 = e1GetShadowUv(worldPosition_3, cascadeIndex_2);
    uv_11 = _e229;
    let _e231 = uv_11;
    let _e235 = uv_11;
    let _e240 = uv_11;
    let _e245 = uv_11;
    if ((((_e231.x < 0f) || (_e235.x > 1f)) || (_e240.y < 0f)) || (_e245.y > 1f)) {
        {
            return 0f;
        }
    }
    let _e254 = textureDimensions(e1ShadowBuffer_t, 0i);
    let _e257 = textureNumLayers(e1ShadowBuffer_t);
    texelSize = (vec2(1f) / vec2<f32>(vec3<i32>(vec3<u32>(_e254.x, _e254.y, _e257)).xy));
    let _e269 = gl_FragCoord_1;
    let _e271 = e1InterleavedGradientNoise(_e269.xy);
    let _e274 = e1VogelDisk(0i, 8i, (_e271 * 6.2831855f));
    offset = _e274;
    let _e275 = sum;
    let _e276 = uv_11;
    let _e277 = offset;
    let _e279 = texelSize;
    let _e282 = e1ReadShadowOpticalDepth((_e276 + ((_e277 * radius_14) * _e279)), distanceToTop_1, cascadeIndex_2);
    sum = (_e275 + _e282);
    let _e286 = gl_FragCoord_1;
    let _e288 = e1InterleavedGradientNoise(_e286.xy);
    let _e291 = e1VogelDisk(1i, 8i, (_e288 * 6.2831855f));
    offset = _e291;
    let _e292 = sum;
    let _e293 = uv_11;
    let _e294 = offset;
    let _e296 = texelSize;
    let _e299 = e1ReadShadowOpticalDepth((_e293 + ((_e294 * radius_14) * _e296)), distanceToTop_1, cascadeIndex_2);
    sum = (_e292 + _e299);
    let _e303 = gl_FragCoord_1;
    let _e305 = e1InterleavedGradientNoise(_e303.xy);
    let _e308 = e1VogelDisk(2i, 8i, (_e305 * 6.2831855f));
    offset = _e308;
    let _e309 = sum;
    let _e310 = uv_11;
    let _e311 = offset;
    let _e313 = texelSize;
    let _e316 = e1ReadShadowOpticalDepth((_e310 + ((_e311 * radius_14) * _e313)), distanceToTop_1, cascadeIndex_2);
    sum = (_e309 + _e316);
    let _e320 = gl_FragCoord_1;
    let _e322 = e1InterleavedGradientNoise(_e320.xy);
    let _e325 = e1VogelDisk(3i, 8i, (_e322 * 6.2831855f));
    offset = _e325;
    let _e326 = sum;
    let _e327 = uv_11;
    let _e328 = offset;
    let _e330 = texelSize;
    let _e333 = e1ReadShadowOpticalDepth((_e327 + ((_e328 * radius_14) * _e330)), distanceToTop_1, cascadeIndex_2);
    sum = (_e326 + _e333);
    let _e337 = gl_FragCoord_1;
    let _e339 = e1InterleavedGradientNoise(_e337.xy);
    let _e342 = e1VogelDisk(4i, 8i, (_e339 * 6.2831855f));
    offset = _e342;
    let _e343 = sum;
    let _e344 = uv_11;
    let _e345 = offset;
    let _e347 = texelSize;
    let _e350 = e1ReadShadowOpticalDepth((_e344 + ((_e345 * radius_14) * _e347)), distanceToTop_1, cascadeIndex_2);
    sum = (_e343 + _e350);
    let _e354 = gl_FragCoord_1;
    let _e356 = e1InterleavedGradientNoise(_e354.xy);
    let _e359 = e1VogelDisk(5i, 8i, (_e356 * 6.2831855f));
    offset = _e359;
    let _e360 = sum;
    let _e361 = uv_11;
    let _e362 = offset;
    let _e364 = texelSize;
    let _e367 = e1ReadShadowOpticalDepth((_e361 + ((_e362 * radius_14) * _e364)), distanceToTop_1, cascadeIndex_2);
    sum = (_e360 + _e367);
    let _e371 = gl_FragCoord_1;
    let _e373 = e1InterleavedGradientNoise(_e371.xy);
    let _e376 = e1VogelDisk(6i, 8i, (_e373 * 6.2831855f));
    offset = _e376;
    let _e377 = sum;
    let _e378 = uv_11;
    let _e379 = offset;
    let _e381 = texelSize;
    let _e384 = e1ReadShadowOpticalDepth((_e378 + ((_e379 * radius_14) * _e381)), distanceToTop_1, cascadeIndex_2);
    sum = (_e377 + _e384);
    let _e388 = gl_FragCoord_1;
    let _e390 = e1InterleavedGradientNoise(_e388.xy);
    let _e393 = e1VogelDisk(7i, 8i, (_e390 * 6.2831855f));
    offset = _e393;
    let _e394 = sum;
    let _e395 = uv_11;
    let _e396 = offset;
    let _e398 = texelSize;
    let _e401 = e1ReadShadowOpticalDepth((_e395 + ((_e396 * radius_14) * _e398)), distanceToTop_1, cascadeIndex_2);
    sum = (_e394 + _e401);
    let _e403 = sum;
    return (_e403 / 8f);
}

fn e1SampleShadowOpticalDepth(worldPosition_4: vec3<f32>, positionECEF_4: vec3<f32>, radius_15: f32, jitter_1: f32) -> f32 {
    var distanceToTop_2: f32;
    var cascadeIndex_3: i32;
    var local_13: f32;

    let _e229 = e1GetDistanceToShadowTop(positionECEF_4);
    distanceToTop_2 = _e229;
    let _e231 = distanceToTop_2;
    if (_e231 <= 0f) {
        {
            return 0f;
        }
    }
    let _e235 = global.e1ViewMatrix;
    let _e236 = e1ShadowIntervals;
    let _e237 = global.cameraNear;
    let _e238 = global.e1ShadowFar;
    let _e239 = getFadedCascadeIndex(_e235, worldPosition_4, _e236, _e237, _e238, jitter_1);
    cascadeIndex_3 = _e239;
    let _e241 = cascadeIndex_3;
    if (_e241 >= 0i) {
        let _e244 = distanceToTop_2;
        let _e245 = cascadeIndex_3;
        let _e246 = e1SampleShadowOpticalDepthPCF(worldPosition_4, _e244, radius_15, _e245);
        local_13 = _e246;
    } else {
        local_13 = 0f;
    }
    let _e249 = local_13;
    return _e249;
}

fn e1GetShadowRadius(worldPosition_5: vec3<f32>) -> f32 {
    var clip_2: vec4<f32>;
    var shadowSize: vec2<f32>;
    var offset_1: vec3<f32>;
    var clipX: vec4<f32>;
    var clipY: vec4<f32>;
    var worldX: vec4<f32>;
    var worldY: vec4<f32>;
    var viewProjectionMatrix: mat4x4<f32>;
    var projected: vec4<f32>;
    var projectedX: vec4<f32>;
    var projectedY: vec4<f32>;
    var center_6: vec2<f32>;
    var offsetX: vec2<f32>;
    var offsetY: vec2<f32>;
    var size_1: f32;

    let _e228 = global.e1ShadowMatrices[0];
    clip_2 = (_e228 * vec4<f32>(worldPosition_5.x, worldPosition_5.y, worldPosition_5.z, 1f));
    let _e236 = clip_2;
    let _e237 = clip_2;
    clip_2 = (_e236 / vec4(_e237.w));
    let _e242 = textureDimensions(e1ShadowBuffer_t, 0i);
    let _e245 = textureNumLayers(e1ShadowBuffer_t);
    shadowSize = vec2<f32>(vec3<i32>(vec3<u32>(_e242.x, _e242.y, _e245)).xy);
    let _e252 = shadowSize;
    let _e254 = (vec2(2f) / _e252);
    offset_1 = vec3<f32>(_e254.x, _e254.y, 0f);
    let _e260 = clip_2;
    let _e261 = offset_1;
    clipX = (_e260 + _e261.xzzz);
    let _e265 = clip_2;
    let _e266 = offset_1;
    clipY = (_e265 + _e266.zyzz);
    let _e272 = global.e1InverseShadowMatrices[0];
    let _e273 = clipX;
    worldX = (_e272 * _e273);
    let _e278 = global.e1InverseShadowMatrices[0];
    let _e279 = clipY;
    worldY = (_e278 * _e279);
    let _e282 = global.e1ProjectionMatrix;
    let _e283 = global.e1ViewMatrix;
    viewProjectionMatrix = (_e282 * _e283);
    let _e286 = viewProjectionMatrix;
    projected = (_e286 * vec4<f32>(worldPosition_5.x, worldPosition_5.y, worldPosition_5.z, 1f));
    let _e294 = viewProjectionMatrix;
    let _e295 = worldX;
    projectedX = (_e294 * _e295);
    let _e298 = viewProjectionMatrix;
    let _e299 = worldY;
    projectedY = (_e298 * _e299);
    let _e302 = projected;
    let _e303 = projected;
    projected = (_e302 / vec4(_e303.w));
    let _e307 = projectedX;
    let _e308 = projectedX;
    projectedX = (_e307 / vec4(_e308.w));
    let _e312 = projectedY;
    let _e313 = projectedY;
    projectedY = (_e312 / vec4(_e313.w));
    let _e317 = projected;
    let _e324 = global.resolution;
    center_6 = (((_e317.xy * 0.5f) + vec2(0.5f)) * _e324);
    let _e327 = projectedX;
    let _e334 = global.resolution;
    offsetX = (((_e327.xy * 0.5f) + vec2(0.5f)) * _e334);
    let _e337 = projectedY;
    let _e344 = global.resolution;
    offsetY = (((_e337.xy * 0.5f) + vec2(0.5f)) * _e344);
    let _e347 = offsetX;
    let _e348 = center_6;
    let _e351 = offsetY;
    let _e352 = center_6;
    size_1 = max(length((_e347 - _e348)), length((_e351 - _e352)));
    let _e357 = size_1;
    let _e361 = global.e1ShadowRadius;
    let _e362 = e1RemapClamped(_e357, 10f, 50f, 0f, _e361);
    return _e362;
}

fn e1MainImage(inputColor_2: vec4<f32>, uv_12: vec2<f32>, outputColor_1: ptr<function, vec4<f32>>) {
    var shadowLength_3: f32 = 0f;
    var overlay: vec4<f32>;
    var rayDirection_1: vec3<f32>;
    var dRDdx: vec3<f32>;
    var dRDdy: vec3<f32>;
    var fragmentAngle_1: f32;
    var depth_12: f32;
    var viewPosition_2: vec3<f32>;
    var worldPosition_6: vec3<f32>;
    var positionECEF_5: vec3<f32>;
    var viewNormal: vec3<f32>;
    var degenerateNormal: bool = false;
    var worldNormal: vec3<f32>;
    var normalECEF_1: vec3<f32>;
    var stbn: f32;
    var radius_16: f32;
    var opticalDepth: f32;
    var sunTransmittance_1: f32;
    var radiance_3: vec3<f32>;

    let _e230 = textureSample(e1ShadowLengthBuffer_t, e1ShadowLengthBuffer_s, uv_12);
    shadowLength_3 = _e230.x;
    let _e232 = textureSample(e1OverlayBuffer_t, e1OverlayBuffer_s, uv_12);
    overlay = _e232;
    let _e234 = overlay;
    if (_e234.w == 1f) {
        {
            let _e238 = overlay;
            (*outputColor_1) = _e238;
            return;
        }
    }
    let _e239 = e1VRayDirection_1;
    rayDirection_1 = normalize(_e239);
    let _e242 = rayDirection_1;
    let _e243 = dpdx(_e242);
    dRDdx = _e243;
    let _e245 = rayDirection_1;
    let _e246 = dpdy(_e245);
    dRDdy = _e246;
    let _e248 = dRDdx;
    let _e249 = dRDdy;
    let _e252 = rayDirection_1;
    fragmentAngle_1 = (length((_e248 + _e249)) / length(_e252));
    let _e256 = e1ReadDepthValue(depthBuffer_t_1, depthBuffer_s_1, uv_12);
    depth_12 = _e256;
    let _e258 = depth_12;
    if (_e258 >= 1f) {
        {
            let _e263 = (*outputColor_1);
            let _e265 = e1VCameraPosition_1;
            let _e266 = rayDirection_1;
            let _e267 = shadowLength_3;
            let _e268 = global.e1SunDirection;
            let _e269 = global.e1MoonDirection;
            let _e270 = global.e1MoonAngularRadius;
            let _e271 = global.e1LunarRadianceScale;
            let _e272 = fragmentAngle_1;
            let _e273 = e1GetSkyLuminance_1(_e265, _e266, _e267, _e268, _e269, _e270, _e271, _e272);
            (*outputColor_1).x = _e273.x;
            (*outputColor_1).y = _e273.y;
            (*outputColor_1).z = _e273.z;
            (*outputColor_1).w = 1f;
            let _e282 = (*outputColor_1);
            let _e284 = (*outputColor_1);
            let _e287 = overlay;
            let _e291 = overlay;
            let _e293 = ((_e284.xyz * (1f - _e287.w)) + _e291.xyz);
            (*outputColor_1).x = _e293.x;
            (*outputColor_1).y = _e293.y;
            (*outputColor_1).z = _e293.z;
            return;
        }
    }
    let _e300 = depth_12;
    let _e301 = global.cameraNear;
    let _e302 = global.cameraFar;
    let _e303 = e1ReverseLogDepth(_e300, _e301, _e302);
    depth_12 = _e303;
    let _e304 = depth_12;
    let _e305 = depth_12;
    let _e306 = getViewZ(_e305);
    let _e307 = global.e1ProjectionMatrix;
    let _e308 = global.e1InverseProjectionMatrix;
    let _e309 = e1ScreenToView(uv_12, _e304, _e306, _e307, _e308);
    viewPosition_2 = _e309;
    let _e311 = global.e1InverseViewMatrix;
    let _e312 = viewPosition_2;
    worldPosition_6 = (_e311 * vec4<f32>(_e312.x, _e312.y, _e312.z, 1f)).xyz;
    let _e321 = global.e1WorldToECEFMatrix;
    let _e322 = worldPosition_6;
    positionECEF_5 = (_e321 * vec4<f32>(_e322.x, _e322.y, _e322.z, 1f)).xyz;
    let _e331 = positionECEF_5;
    let _e334 = e1VGeometryAltitudeCorrection_1;
    positionECEF_5 = ((_e331 * 0.001f) + _e334);
    let _e341 = e1ReadNormal(uv_12, (&degenerateNormal));
    viewNormal = _e341;
    let _e342 = global.e1InverseViewMatrix;
    let _e343 = viewNormal;
    worldNormal = (_e342 * vec4<f32>(_e343.x, _e343.y, _e343.z, 0f)).xyz;
    let _e352 = global.e1WorldToECEFMatrix;
    let _e353 = worldNormal;
    normalECEF_1 = (_e352 * vec4<f32>(_e353.x, _e353.y, _e353.z, 0f)).xyz;
    e1CorrectGeometricError((&positionECEF_5), (&normalECEF_1));
    let _e366 = e1GetSTBN();
    stbn = _e366;
    let _e368 = worldPosition_6;
    let _e369 = e1GetShadowRadius(_e368);
    radius_16 = _e369;
    let _e371 = worldPosition_6;
    let _e372 = positionECEF_5;
    let _e373 = radius_16;
    let _e374 = stbn;
    let _e375 = e1SampleShadowOpticalDepth(_e371, _e372, _e373, _e374);
    opticalDepth = _e375;
    let _e377 = opticalDepth;
    sunTransmittance_1 = exp(-(_e377));
    let _e382 = degenerateNormal;
    if !(_e382) {
        {
            let _e384 = positionECEF_5;
            let _e385 = normalECEF_1;
            let _e387 = sunTransmittance_1;
            let _e388 = e1GetSunSkyIrradiance(_e384, _e385, inputColor_2.xyz, _e387);
            radiance_3 = _e388;
        }
    } else {
        {
            radiance_3 = inputColor_2.xyz;
        }
    }
    let _e390 = positionECEF_5;
    let _e391 = shadowLength_3;
    e1ApplyTransmittanceInscatter(_e390, _e391, (&radiance_3));
    let _e394 = radiance_3;
    (*outputColor_1) = vec4<f32>(_e394.x, _e394.y, _e394.z, inputColor_2.w);
    let _e400 = (*outputColor_1);
    let _e402 = (*outputColor_1);
    let _e405 = overlay;
    let _e409 = overlay;
    let _e411 = ((_e402.xyz * (1f - _e405.w)) + _e409.xyz);
    (*outputColor_1).x = _e411.x;
    (*outputColor_1).y = _e411.y;
    (*outputColor_1).z = _e411.z;
    return;
}

fn blend23_(dst: vec4<f32>, src: vec4<f32>, opacity: f32) -> vec4<f32> {
    var dst_1: vec4<f32>;
    var src_1: vec4<f32>;
    var opacity_1: f32;

    dst_1 = dst;
    src_1 = src;
    opacity_1 = opacity;
    let _e231 = dst_1;
    let _e232 = src_1;
    let _e233 = opacity_1;
    return mix(_e231, _e232, vec4(_e233));
}

fn pt_main() {
    var color0_: vec4<f32>;
    var color1_: vec4<f32> = vec4(0f);

    let _e225 = vUv_1;
    let _e226 = textureSample(inputBuffer_t, inputBuffer_s, _e225);
    color0_ = _e226;
    let _e231 = color0_;
    let _e232 = vUv_1;
    e0MainImage(_e231, _e232, (&color1_));
    let _e235 = color0_;
    let _e236 = color1_;
    let _e237 = global.e0BlendOpacity;
    let _e238 = blend23_(_e235, _e236, _e237);
    color0_ = _e238;
    let _e239 = color0_;
    let _e240 = vUv_1;
    e1MainImage(_e239, _e240, (&color1_));
    let _e243 = color0_;
    let _e244 = color1_;
    let _e245 = global.e1BlendOpacity;
    let _e246 = blend23_(_e243, _e244, _e245);
    color0_ = _e246;
    let _e248 = color0_;
    color0_.w = clamp(_e248.w, 0f, 1f);
    let _e253 = color0_;
    pc_fragColor = _e253;
    let _e254 = pc_fragColor;
    let _e255 = linearToOutputTexel(_e254);
    pc_fragColor = _e255;
    return;
}

fn main_1() {
    let _e229 = global.pt_u_e1ShadowIntervals[0];
    e1ShadowIntervals[0i] = _e229.xy;
    let _e235 = global.pt_u_e1ShadowIntervals[1];
    e1ShadowIntervals[1i] = _e235.xy;
    let _e238 = global.pt_s_e1ATMOSPHERE_solar_irradiance;
    e1ATMOSPHERE.solar_irradiance = _e238;
    let _e240 = global.pt_s_e1ATMOSPHERE_sun_angular_radius;
    e1ATMOSPHERE.sun_angular_radius = _e240;
    let _e242 = global.pt_s_e1ATMOSPHERE_bottom_radius;
    e1ATMOSPHERE.bottom_radius = _e242;
    let _e244 = global.pt_s_e1ATMOSPHERE_top_radius;
    e1ATMOSPHERE.top_radius = _e244;
    let _e250 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_width;
    e1ATMOSPHERE.rayleigh_density.layers[0i].width = _e250;
    let _e256 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e256;
    let _e262 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    e1ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e262;
    let _e268 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e268;
    let _e274 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e274;
    let _e280 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_width;
    e1ATMOSPHERE.rayleigh_density.layers[1i].width = _e280;
    let _e286 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e286;
    let _e292 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    e1ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e292;
    let _e298 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e298;
    let _e304 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e304;
    let _e306 = global.pt_s_e1ATMOSPHERE_rayleigh_scattering;
    e1ATMOSPHERE.rayleigh_scattering = _e306;
    let _e312 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_width;
    e1ATMOSPHERE.mie_density.layers[0i].width = _e312;
    let _e318 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_term;
    e1ATMOSPHERE.mie_density.layers[0i].exp_term = _e318;
    let _e324 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_scale;
    e1ATMOSPHERE.mie_density.layers[0i].exp_scale = _e324;
    let _e330 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_linear_term;
    e1ATMOSPHERE.mie_density.layers[0i].linear_term = _e330;
    let _e336 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_constant_term;
    e1ATMOSPHERE.mie_density.layers[0i].constant_term = _e336;
    let _e342 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_width;
    e1ATMOSPHERE.mie_density.layers[1i].width = _e342;
    let _e348 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_term;
    e1ATMOSPHERE.mie_density.layers[1i].exp_term = _e348;
    let _e354 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_scale;
    e1ATMOSPHERE.mie_density.layers[1i].exp_scale = _e354;
    let _e360 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_linear_term;
    e1ATMOSPHERE.mie_density.layers[1i].linear_term = _e360;
    let _e366 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_constant_term;
    e1ATMOSPHERE.mie_density.layers[1i].constant_term = _e366;
    let _e368 = global.pt_s_e1ATMOSPHERE_mie_scattering;
    e1ATMOSPHERE.mie_scattering = _e368;
    let _e370 = global.pt_s_e1ATMOSPHERE_mie_extinction;
    e1ATMOSPHERE.mie_extinction = _e370;
    let _e372 = global.pt_s_e1ATMOSPHERE_mie_phase_function_g;
    e1ATMOSPHERE.mie_phase_function_g = _e372;
    let _e378 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_width;
    e1ATMOSPHERE.absorption_density.layers[0i].width = _e378;
    let _e384 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_term;
    e1ATMOSPHERE.absorption_density.layers[0i].exp_term = _e384;
    let _e390 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_scale;
    e1ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e390;
    let _e396 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_linear_term;
    e1ATMOSPHERE.absorption_density.layers[0i].linear_term = _e396;
    let _e402 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_constant_term;
    e1ATMOSPHERE.absorption_density.layers[0i].constant_term = _e402;
    let _e408 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_width;
    e1ATMOSPHERE.absorption_density.layers[1i].width = _e408;
    let _e414 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_term;
    e1ATMOSPHERE.absorption_density.layers[1i].exp_term = _e414;
    let _e420 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_scale;
    e1ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e420;
    let _e426 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_linear_term;
    e1ATMOSPHERE.absorption_density.layers[1i].linear_term = _e426;
    let _e432 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_constant_term;
    e1ATMOSPHERE.absorption_density.layers[1i].constant_term = _e432;
    let _e434 = global.pt_s_e1ATMOSPHERE_absorption_extinction;
    e1ATMOSPHERE.absorption_extinction = _e434;
    let _e436 = global.pt_s_e1ATMOSPHERE_ground_albedo;
    e1ATMOSPHERE.ground_albedo = _e436;
    let _e438 = global.pt_s_e1ATMOSPHERE_mu_s_min;
    e1ATMOSPHERE.mu_s_min = _e438;
    pt_main();
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) e1VCameraPosition: vec3<f32>, @location(2) e1VRayDirection: vec3<f32>, @location(3) e1VGeometryAltitudeCorrection: vec3<f32>, @location(4) e1VEllipsoidRadiiSquared: vec3<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    e1VCameraPosition_1 = e1VCameraPosition;
    e1VRayDirection_1 = e1VRayDirection;
    e1VGeometryAltitudeCorrection_1 = e1VGeometryAltitudeCorrection;
    e1VEllipsoidRadiiSquared_1 = e1VEllipsoidRadiiSquared;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e299 = pc_fragColor;
    return FragmentOutput(_e299);
}
