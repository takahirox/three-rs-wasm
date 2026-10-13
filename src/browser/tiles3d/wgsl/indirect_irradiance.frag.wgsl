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
    luminanceFromRadiance: mat3x3<f32>,
    scatteringOrder: i32,
}

struct FragmentOutput {
    @location(0) outputColor: vec4<f32>,
}

const m_1: f32 = 1f;
const nm: f32 = 1f;
const rad: f32 = 1f;
const sr: f32 = 1f;
const watt: f32 = 1f;
const lm: f32 = 1f;
const PI: f32 = 3.1415927f;
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
const IRRADIANCE_TEXTURE_SIZE: vec2<f32> = vec2<f32>(64f, 16f);

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var singleRayleighScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(2) 
var singleRayleighScatteringTexture_s: sampler;
@group(0) @binding(3) 
var singleMieScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(4) 
var singleMieScatteringTexture_s: sampler;
@group(0) @binding(5) 
var multipleScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(6) 
var multipleScatteringTexture_s: sampler;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> outputColor: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e97 = y_1;
    let _e98 = x_1;
    return exp2((_e97 * log2(_e98)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e97 = y_3;
    let _e98 = x_3;
    return exp2((_e97 * log2(_e98)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e97 = y_5;
    let _e98 = x_5;
    return exp2((_e97 * log2(_e98)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e97 = y_7;
    let _e98 = x_7;
    return exp2((_e97 * log2(_e98)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e96 = x_9;
    (*i) = trunc(_e96);
    let _e98 = x_9;
    let _e99 = (*i);
    return (_e98 - _e99);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e96 = x_11;
    (*i_1) = trunc(_e96);
    let _e98 = x_11;
    let _e99 = (*i_1);
    return (_e98 - _e99);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e96 = x_13;
    (*i_2) = trunc(_e96);
    let _e98 = x_13;
    let _e99 = (*i_2);
    return (_e98 - _e99);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e96 = x_15;
    (*i_3) = trunc(_e96);
    let _e98 = x_15;
    let _e99 = (*i_3);
    return (_e98 - _e99);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e95 = v_1;
    b = (bitcast<u32>(_e95) & 2147483647u);
    let _e100 = b;
    return (_e100 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e95 = v_3;
    b_1 = (bitcast<u32>(_e95) & 2147483647u);
    let _e100 = b_1;
    return (_e100 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e95 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e95) & vec2(2147483647u));
    let _e101 = b_2;
    return (_e101 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e95 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e95) & vec3(2147483647u));
    let _e101 = b_3;
    return (_e101 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e95 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e95) & vec4(2147483647u));
    let _e101 = b_4;
    return (_e101 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e95 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e95) & vec2(2147483647u));
    let _e101 = b_5;
    return (_e101 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e95 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e95) & vec3(2147483647u));
    let _e101 = b_6;
    return (_e101 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e95 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e95) & vec4(2147483647u));
    let _e101 = b_7;
    return (_e101 == vec4(2139095040u));
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
    let _e129 = discriminant;
    let _e130 = SafeSqrt(_e129);
    let _e132 = ClampDistance(((-(r_1) * mu_1) + _e130));
    return _e132;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_2: f32, mu_2: f32) -> f32 {
    var discriminant_1: f32;

    discriminant_1 = (((r_2 * r_2) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e129 = discriminant_1;
    let _e130 = SafeSqrt(_e129);
    let _e132 = ClampDistance(((-(r_2) * mu_2) - _e130));
    return _e132;
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
    let _e131 = SafeSqrt(((r_4 * r_4) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e131;
    let _e133 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_4, mu_4);
    d_1 = _e133;
    d_min = (atmosphere_4.top_radius - r_4);
    let _e138 = rho;
    let _e139 = H;
    d_max = (_e138 + _e139);
    let _e142 = d_1;
    let _e143 = d_min;
    let _e145 = d_max;
    let _e146 = d_min;
    x_mu = ((_e142 - _e143) / (_e145 - _e146));
    let _e150 = rho;
    let _e151 = H;
    x_r = (_e150 / _e151);
    let _e154 = x_mu;
    let _e156 = GetTextureCoordFromUnitRange(_e154, 256i);
    let _e157 = x_r;
    let _e159 = GetTextureCoordFromUnitRange(_e157, 64i);
    return vec2<f32>(_e156, _e159);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_5: f32, mu_5: f32) -> vec3<f32> {
    var uv: vec2<f32>;

    let _e119 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_5, mu_5);
    uv = _e119;
    let _e121 = uv;
    let _e122 = textureSample(transmittance_texture_t, transmittance_texture_s, _e121);
    return vec3<f32>(_e122.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_6: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e130 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_6) * mu_6) * d_2)) + (r_6 * r_6))));
    r_d = _e130;
    let _e134 = r_d;
    let _e136 = ClampCosine((((r_6 * mu_6) + d_2) / _e134));
    mu_d = _e136;
    if ray_r_mu_intersects_ground {
        {
            let _e138 = r_d;
            let _e139 = mu_d;
            let _e141 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e138, -(_e139));
            let _e143 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, -(mu_6));
            return min((_e141 / _e143), vec3(1f));
        }
    } else {
        {
            let _e148 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, mu_6);
            let _e149 = r_d;
            let _e150 = mu_d;
            let _e151 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e149, _e150);
            return min((_e148 / _e151), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_7: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_7);
    let _e123 = sin_theta_h;
    let _e124 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e123 * _e124)), 0f)));
    let _e132 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_7, mu_s);
    let _e133 = sin_theta_h;
    let _e138 = sin_theta_h;
    let _e142 = cos_theta_h;
    return (_e132 * smoothstep(((-(_e133) * atmosphere_7.sun_angular_radius) / rad), ((_e138 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e142)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e123 = k;
    return (_e123 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e132 = k_1;
    let _e145 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e132 * (1f + (nu_1 * nu_1))) / _e145);
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
    let _e134 = SafeSqrt(((r_8 * r_8) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e134;
    let _e136 = rho_1;
    let _e137 = H_1;
    let _e140 = GetTextureCoordFromUnitRange((_e136 / _e137), 32i);
    u_r = _e140;
    r_mu = (r_8 * mu_7);
    let _e144 = r_mu;
    let _e145 = r_mu;
    discriminant_2 = (((_e144 * _e145) - (r_8 * r_8)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e155 = r_mu;
            let _e157 = discriminant_2;
            let _e158 = SafeSqrt(_e157);
            d_3 = (-(_e155) - _e158);
            d_min_1 = (r_8 - atmosphere_8.bottom_radius);
            let _e164 = rho_1;
            d_max_1 = _e164;
            let _e168 = d_max_1;
            let _e169 = d_min_1;
            if (_e168 == _e169) {
                local = 0f;
            } else {
                let _e172 = d_3;
                let _e173 = d_min_1;
                let _e175 = d_max_1;
                let _e176 = d_min_1;
                local = ((_e172 - _e173) / (_e175 - _e176));
            }
            let _e180 = local;
            let _e184 = GetTextureCoordFromUnitRange(_e180, 64i);
            u_mu = (0.5f - (0.5f * _e184));
        }
    } else {
        {
            let _e187 = r_mu;
            let _e189 = discriminant_2;
            let _e190 = H_1;
            let _e191 = H_1;
            let _e194 = SafeSqrt((_e189 + (_e190 * _e191)));
            d_4 = (-(_e187) + _e194);
            d_min_2 = (atmosphere_8.top_radius - r_8);
            let _e200 = rho_1;
            let _e201 = H_1;
            d_max_2 = (_e200 + _e201);
            let _e206 = d_4;
            let _e207 = d_min_2;
            let _e209 = d_max_2;
            let _e210 = d_min_2;
            let _e216 = GetTextureCoordFromUnitRange(((_e206 - _e207) / (_e209 - _e210)), 64i);
            u_mu = (0.5f + (0.5f * _e216));
        }
    }
    let _e220 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e220;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e226 = H_1;
    d_max_3 = _e226;
    let _e228 = d_5;
    let _e229 = d_min_3;
    let _e231 = d_max_3;
    let _e232 = d_min_3;
    a_1 = ((_e228 - _e229) / (_e231 - _e232));
    let _e238 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e238;
    let _e240 = D;
    let _e241 = d_min_3;
    let _e243 = d_max_3;
    let _e244 = d_min_3;
    A = ((_e240 - _e241) / (_e243 - _e244));
    let _e249 = a_1;
    let _e250 = A;
    let _e256 = a_1;
    let _e260 = GetTextureCoordFromUnitRange((max((1f - (_e249 / _e250)), 0f) / (1f + _e256)), 32i);
    u_mu_s = _e260;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e267 = u_nu;
    let _e268 = u_mu_s;
    let _e269 = u_mu;
    let _e270 = u_r;
    return vec4<f32>(_e267, _e268, _e269, _e270);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_9: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_9 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e129 = x_mu_s;
    let _e131 = GetTextureCoordFromUnitRange(_e129, 64i);
    let _e132 = x_r_1;
    let _e134 = GetTextureCoordFromUnitRange(_e132, 16i);
    return vec2<f32>(_e131, _e134);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_10: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_1: vec2<f32>;

    let _e119 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_10, mu_s_3);
    uv_1 = _e119;
    let _e121 = uv_1;
    let _e122 = textureSample(irradiance_texture_t, irradiance_texture_s, _e121);
    return vec3<f32>(_e122.xyz);
}

fn GetLayerDensity(layer: DensityProfileLayer, altitude: f32) -> f32 {
    var density: f32;

    density = (((layer.exp_term * exp((layer.exp_scale * altitude))) + (layer.linear_term * altitude)) + layer.constant_term);
    let _e127 = density;
    return clamp(_e127, 0f, 1f);
}

fn GetProfileDensity(profile: DensityProfile, altitude_1: f32) -> f32 {
    var layers: array<DensityProfileLayer, 2>;
    var local_1: f32;

    layers[0i] = profile.layers[0];
    layers[1i] = profile.layers[1];
    let _e129 = layers[0];
    if (altitude_1 < _e129.width) {
        let _e134 = layers[0];
        let _e135 = GetLayerDensity(_e134, altitude_1);
        local_1 = _e135;
    } else {
        let _e138 = layers[1];
        let _e139 = GetLayerDensity(_e138, altitude_1);
        local_1 = _e139;
    }
    let _e141 = local_1;
    return _e141;
}

fn ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_11: AtmosphereParameters, profile_1: DensityProfile, r_11: f32, mu_8: f32) -> f32 {
    var SAMPLE_COUNT: i32 = 500i;
    var dx: f32;
    var result: f32 = 0f;
    var i_4: i32 = 0i;
    var d_i: f32;
    var r_i: f32;
    var y_i: f32;
    var local_2: f32;
    var weight_i: f32;

    let _e120 = DistanceToTopAtmosphereBoundary(atmosphere_11, r_11, mu_8);
    let _e121 = SAMPLE_COUNT;
    dx = (_e120 / f32(_e121));
    loop {
        let _e131 = i_4;
        let _e132 = SAMPLE_COUNT;
        if !((_e131 <= _e132)) {
            break;
        }
        {
            let _e138 = i_4;
            let _e140 = dx;
            d_i = (f32(_e138) * _e140);
            let _e143 = d_i;
            let _e144 = d_i;
            let _e149 = d_i;
            r_i = sqrt((((_e143 * _e144) + (((2f * r_11) * mu_8) * _e149)) + (r_11 * r_11)));
            let _e156 = r_i;
            let _e159 = GetProfileDensity(profile_1, (_e156 - atmosphere_11.bottom_radius));
            y_i = _e159;
            let _e161 = i_4;
            let _e164 = i_4;
            let _e165 = SAMPLE_COUNT;
            if ((_e161 == 0i) || (_e164 == _e165)) {
                local_2 = 0.5f;
            } else {
                local_2 = 1f;
            }
            let _e171 = local_2;
            weight_i = _e171;
            let _e173 = result;
            let _e174 = y_i;
            let _e175 = weight_i;
            let _e177 = dx;
            result = (_e173 + ((_e174 * _e175) * _e177));
        }
        continuing {
            let _e135 = i_4;
            i_4 = (_e135 + 1i);
        }
    }
    let _e180 = result;
    return _e180;
}

fn ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_12: AtmosphereParameters, r_12: f32, mu_9: f32) -> vec3<f32> {
    var optical_depth: vec3<f32>;

    let _e119 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.rayleigh_density, r_12, mu_9);
    let _e123 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.mie_density, r_12, mu_9);
    let _e128 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.absorption_density, r_12, mu_9);
    optical_depth = (((atmosphere_12.rayleigh_scattering * _e119) + (atmosphere_12.mie_extinction * _e123)) + (atmosphere_12.absorption_extinction * _e128));
    let _e132 = optical_depth;
    return exp(-(_e132));
}

fn GetUnitRangeFromTextureCoord(u: f32, texture_size_1: i32) -> f32 {
    return ((u - (0.5f / f32(texture_size_1))) / (1f - (1f / f32(texture_size_1))));
}

fn GetRMuFromTransmittanceTextureUv(atmosphere_13: AtmosphereParameters, uv_2: vec2<f32>, r_13: ptr<function, f32>, mu_10: ptr<function, f32>) {
    var x_mu_1: f32;
    var x_r_2: f32;
    var H_2: f32;
    var rho_2: f32;
    var d_min_4: f32;
    var d_max_4: f32;
    var d_6: f32;
    var local_3: f32;

    let _e120 = GetUnitRangeFromTextureCoord(uv_2.x, 256i);
    x_mu_1 = _e120;
    let _e124 = GetUnitRangeFromTextureCoord(uv_2.y, 64i);
    x_r_2 = _e124;
    H_2 = sqrt(((atmosphere_13.top_radius * atmosphere_13.top_radius) - (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e135 = H_2;
    let _e136 = x_r_2;
    rho_2 = (_e135 * _e136);
    let _e139 = rho_2;
    let _e140 = rho_2;
    (*r_13) = sqrt(((_e139 * _e140) + (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e148 = (*r_13);
    d_min_4 = (atmosphere_13.top_radius - _e148);
    let _e151 = rho_2;
    let _e152 = H_2;
    d_max_4 = (_e151 + _e152);
    let _e155 = d_min_4;
    let _e156 = x_mu_1;
    let _e157 = d_max_4;
    let _e158 = d_min_4;
    d_6 = (_e155 + (_e156 * (_e157 - _e158)));
    let _e163 = d_6;
    if (_e163 == 0f) {
        local_3 = 1f;
    } else {
        let _e170 = H_2;
        let _e171 = H_2;
        let _e173 = rho_2;
        let _e174 = rho_2;
        let _e177 = d_6;
        let _e178 = d_6;
        let _e182 = (*r_13);
        let _e184 = d_6;
        local_3 = ((((_e170 * _e171) - (_e173 * _e174)) - (_e177 * _e178)) / ((2f * _e182) * _e184));
    }
    let _e188 = local_3;
    (*mu_10) = _e188;
    let _e189 = (*mu_10);
    let _e190 = ClampCosine(_e189);
    (*mu_10) = _e190;
    return;
}

fn ComputeTransmittanceToTopAtmosphereBoundaryTexture(atmosphere_14: AtmosphereParameters, frag_coord: vec2<f32>) -> vec3<f32> {
    var TRANSMITTANCE_TEXTURE_SIZE: vec2<f32> = vec2<f32>(256f, 64f);
    var r_14: f32;
    var mu_11: f32;

    let _e124 = TRANSMITTANCE_TEXTURE_SIZE;
    GetRMuFromTransmittanceTextureUv(atmosphere_14, (frag_coord / _e124), (&r_14), (&mu_11));
    let _e130 = r_14;
    let _e131 = mu_11;
    let _e132 = ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_14, _e130, _e131);
    return _e132;
}

fn ComputeSingleScatteringIntegrand(atmosphere_15: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, r_15: f32, mu_12: f32, mu_s_4: f32, nu_3: f32, d_7: f32, ray_r_mu_intersects_ground_2: bool, rayleigh: ptr<function, vec3<f32>>, mie: ptr<function, vec3<f32>>) {
    var r_d_1: f32;
    var mu_s_d: f32;
    var transmittance: vec3<f32>;

    let _e134 = ClampRadius(atmosphere_15, sqrt((((d_7 * d_7) + (((2f * r_15) * mu_12) * d_7)) + (r_15 * r_15))));
    r_d_1 = _e134;
    let _e139 = r_d_1;
    let _e141 = ClampCosine((((r_15 * mu_s_4) + (d_7 * nu_3)) / _e139));
    mu_s_d = _e141;
    let _e143 = GetTransmittance(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, r_15, mu_12, d_7, ray_r_mu_intersects_ground_2);
    let _e144 = r_d_1;
    let _e145 = mu_s_d;
    let _e146 = GetTransmittanceToSun(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, _e144, _e145);
    transmittance = (_e143 * _e146);
    let _e149 = transmittance;
    let _e151 = r_d_1;
    let _e154 = GetProfileDensity(atmosphere_15.rayleigh_density, (_e151 - atmosphere_15.bottom_radius));
    (*rayleigh) = (_e149 * _e154);
    let _e156 = transmittance;
    let _e158 = r_d_1;
    let _e161 = GetProfileDensity(atmosphere_15.mie_density, (_e158 - atmosphere_15.bottom_radius));
    (*mie) = (_e156 * _e161);
    return;
}

fn DistanceToNearestAtmosphereBoundary(atmosphere_16: AtmosphereParameters, r_16: f32, mu_13: f32, ray_r_mu_intersects_ground_3: bool) -> f32 {
    var r_17: f32;
    var mu_14: f32;
    var ray_r_mu_intersects_ground_4: bool;

    r_17 = r_16;
    mu_14 = mu_13;
    ray_r_mu_intersects_ground_4 = ray_r_mu_intersects_ground_3;
    let _e121 = ray_r_mu_intersects_ground_4;
    if _e121 {
        {
            let _e122 = r_17;
            let _e123 = mu_14;
            let _e124 = DistanceToBottomAtmosphereBoundary(atmosphere_16, _e122, _e123);
            return _e124;
        }
    } else {
        {
            let _e125 = r_17;
            let _e126 = mu_14;
            let _e127 = DistanceToTopAtmosphereBoundary(atmosphere_16, _e125, _e126);
            return _e127;
        }
    }
}

fn ComputeSingleScattering(atmosphere_17: AtmosphereParameters, transmittance_texture_t_4: texture_2d<f32>, transmittance_texture_s_4: sampler, r_18: f32, mu_15: f32, mu_s_5: f32, nu_4: f32, ray_r_mu_intersects_ground_5: bool, rayleigh_1: ptr<function, vec3<f32>>, mie_1: ptr<function, vec3<f32>>) {
    var SAMPLE_COUNT_1: i32 = 50i;
    var dx_1: f32;
    var rayleigh_sum: vec3<f32> = vec3(0f);
    var mie_sum: vec3<f32> = vec3(0f);
    var i_5: i32 = 0i;
    var d_i_1: f32;
    var rayleigh_i: vec3<f32>;
    var mie_i: vec3<f32>;
    var local_4: f32;
    var weight_i_1: f32;

    let _e126 = DistanceToNearestAtmosphereBoundary(atmosphere_17, r_18, mu_15, ray_r_mu_intersects_ground_5);
    let _e127 = SAMPLE_COUNT_1;
    dx_1 = (_e126 / f32(_e127));
    loop {
        let _e139 = i_5;
        let _e140 = SAMPLE_COUNT_1;
        if !((_e139 <= _e140)) {
            break;
        }
        {
            let _e146 = i_5;
            let _e148 = dx_1;
            d_i_1 = (f32(_e146) * _e148);
            let _e153 = d_i_1;
            ComputeSingleScatteringIntegrand(atmosphere_17, transmittance_texture_t_4, transmittance_texture_s_4, r_18, mu_15, mu_s_5, nu_4, _e153, ray_r_mu_intersects_ground_5, (&rayleigh_i), (&mie_i));
            let _e158 = i_5;
            let _e161 = i_5;
            let _e162 = SAMPLE_COUNT_1;
            if ((_e158 == 0i) || (_e161 == _e162)) {
                local_4 = 0.5f;
            } else {
                local_4 = 1f;
            }
            let _e168 = local_4;
            weight_i_1 = _e168;
            let _e170 = rayleigh_sum;
            let _e171 = rayleigh_i;
            let _e172 = weight_i_1;
            rayleigh_sum = (_e170 + (_e171 * _e172));
            let _e175 = mie_sum;
            let _e176 = mie_i;
            let _e177 = weight_i_1;
            mie_sum = (_e175 + (_e176 * _e177));
        }
        continuing {
            let _e143 = i_5;
            i_5 = (_e143 + 1i);
        }
    }
    let _e180 = rayleigh_sum;
    let _e181 = dx_1;
    (*rayleigh_1) = (((_e180 * _e181) * atmosphere_17.solar_irradiance) * atmosphere_17.rayleigh_scattering);
    let _e187 = mie_sum;
    let _e188 = dx_1;
    (*mie_1) = (((_e187 * _e188) * atmosphere_17.solar_irradiance) * atmosphere_17.mie_scattering);
    return;
}

fn GetRMuMuSNuFromScatteringTextureUvwz(atmosphere_18: AtmosphereParameters, uvwz: vec4<f32>, r_19: ptr<function, f32>, mu_16: ptr<function, f32>, mu_s_6: ptr<function, f32>, nu_5: ptr<function, f32>, ray_r_mu_intersects_ground_6: ptr<function, bool>) {
    var H_3: f32;
    var rho_3: f32;
    var d_min_5: f32;
    var d_max_5: f32;
    var d_8: f32;
    var local_5: f32;
    var d_min_6: f32;
    var d_max_6: f32;
    var d_9: f32;
    var local_6: f32;
    var x_mu_s_1: f32;
    var d_min_7: f32;
    var d_max_7: f32;
    var D_1: f32;
    var A_1: f32;
    var a_2: f32;
    var d_10: f32;
    var local_7: f32;

    H_3 = sqrt(((atmosphere_18.top_radius * atmosphere_18.top_radius) - (atmosphere_18.bottom_radius * atmosphere_18.bottom_radius)));
    let _e130 = H_3;
    let _e133 = GetUnitRangeFromTextureCoord(uvwz.w, 32i);
    rho_3 = (_e130 * _e133);
    let _e136 = rho_3;
    let _e137 = rho_3;
    (*r_19) = sqrt(((_e136 * _e137) + (atmosphere_18.bottom_radius * atmosphere_18.bottom_radius)));
    if (uvwz.z < 0.5f) {
        {
            let _e147 = (*r_19);
            d_min_5 = (_e147 - atmosphere_18.bottom_radius);
            let _e151 = rho_3;
            d_max_5 = _e151;
            let _e153 = d_min_5;
            let _e154 = d_max_5;
            let _e155 = d_min_5;
            let _e165 = GetUnitRangeFromTextureCoord((1f - (2f * uvwz.z)), 64i);
            d_8 = (_e153 + ((_e154 - _e155) * _e165));
            let _e169 = d_8;
            if (_e169 == 0f) {
                local_5 = -1f;
            } else {
                let _e177 = rho_3;
                let _e178 = rho_3;
                let _e180 = d_8;
                let _e181 = d_8;
                let _e186 = (*r_19);
                let _e188 = d_8;
                let _e191 = ClampCosine((-(((_e177 * _e178) + (_e180 * _e181))) / ((2f * _e186) * _e188)));
                local_5 = _e191;
            }
            let _e193 = local_5;
            (*mu_16) = _e193;
            (*ray_r_mu_intersects_ground_6) = true;
        }
    } else {
        {
            let _e196 = (*r_19);
            d_min_6 = (atmosphere_18.top_radius - _e196);
            let _e199 = rho_3;
            let _e200 = H_3;
            d_max_6 = (_e199 + _e200);
            let _e203 = d_min_6;
            let _e204 = d_max_6;
            let _e205 = d_min_6;
            let _e215 = GetUnitRangeFromTextureCoord(((2f * uvwz.z) - 1f), 64i);
            d_9 = (_e203 + ((_e204 - _e205) * _e215));
            let _e219 = d_9;
            if (_e219 == 0f) {
                local_6 = 1f;
            } else {
                let _e226 = H_3;
                let _e227 = H_3;
                let _e229 = rho_3;
                let _e230 = rho_3;
                let _e233 = d_9;
                let _e234 = d_9;
                let _e238 = (*r_19);
                let _e240 = d_9;
                let _e243 = ClampCosine(((((_e226 * _e227) - (_e229 * _e230)) - (_e233 * _e234)) / ((2f * _e238) * _e240)));
                local_6 = _e243;
            }
            let _e245 = local_6;
            (*mu_16) = _e245;
            (*ray_r_mu_intersects_ground_6) = false;
        }
    }
    let _e249 = GetUnitRangeFromTextureCoord(uvwz.y, 32i);
    x_mu_s_1 = _e249;
    d_min_7 = (atmosphere_18.top_radius - atmosphere_18.bottom_radius);
    let _e255 = H_3;
    d_max_7 = _e255;
    let _e259 = DistanceToTopAtmosphereBoundary(atmosphere_18, atmosphere_18.bottom_radius, atmosphere_18.mu_s_min);
    D_1 = _e259;
    let _e261 = D_1;
    let _e262 = d_min_7;
    let _e264 = d_max_7;
    let _e265 = d_min_7;
    A_1 = ((_e261 - _e262) / (_e264 - _e265));
    let _e269 = A_1;
    let _e270 = x_mu_s_1;
    let _e271 = A_1;
    let _e275 = x_mu_s_1;
    let _e276 = A_1;
    a_2 = ((_e269 - (_e270 * _e271)) / (1f + (_e275 * _e276)));
    let _e281 = d_min_7;
    let _e282 = a_2;
    let _e283 = A_1;
    let _e285 = d_max_7;
    let _e286 = d_min_7;
    d_10 = (_e281 + (min(_e282, _e283) * (_e285 - _e286)));
    let _e291 = d_10;
    if (_e291 == 0f) {
        local_7 = 1f;
    } else {
        let _e298 = H_3;
        let _e299 = H_3;
        let _e301 = d_10;
        let _e302 = d_10;
        let _e308 = d_10;
        let _e311 = ClampCosine((((_e298 * _e299) - (_e301 * _e302)) / ((2f * atmosphere_18.bottom_radius) * _e308)));
        local_7 = _e311;
    }
    let _e313 = local_7;
    (*mu_s_6) = _e313;
    let _e319 = ClampCosine(((uvwz.x * 2f) - 1f));
    (*nu_5) = _e319;
    return;
}

fn GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_19: AtmosphereParameters, frag_coord_1: vec3<f32>, r_20: ptr<function, f32>, mu_17: ptr<function, f32>, mu_s_7: ptr<function, f32>, nu_6: ptr<function, f32>, ray_r_mu_intersects_ground_7: ptr<function, bool>) {
    var SCATTERING_TEXTURE_SIZE: vec4<f32> = vec4<f32>(7f, 32f, 128f, 32f);
    var frag_coord_nu: f32;
    var frag_coord_mu_s: f32;
    var uvwz_1: vec4<f32>;

    frag_coord_nu = floor((frag_coord_1.x / 32f));
    frag_coord_mu_s = (frag_coord_1.x - (floor((frag_coord_1.x / 32f)) * 32f));
    let _e147 = frag_coord_nu;
    let _e148 = frag_coord_mu_s;
    let _e152 = SCATTERING_TEXTURE_SIZE;
    uvwz_1 = (vec4<f32>(_e147, _e148, frag_coord_1.y, frag_coord_1.z) / _e152);
    let _e155 = uvwz_1;
    GetRMuMuSNuFromScatteringTextureUvwz(atmosphere_19, _e155, r_20, mu_17, mu_s_7, nu_6, ray_r_mu_intersects_ground_7);
    let _e166 = (*nu_6);
    let _e167 = (*mu_17);
    let _e168 = (*mu_s_7);
    let _e171 = (*mu_17);
    let _e172 = (*mu_17);
    let _e176 = (*mu_s_7);
    let _e177 = (*mu_s_7);
    let _e183 = (*mu_17);
    let _e184 = (*mu_s_7);
    let _e187 = (*mu_17);
    let _e188 = (*mu_17);
    let _e192 = (*mu_s_7);
    let _e193 = (*mu_s_7);
    (*nu_6) = clamp(_e166, ((_e167 * _e168) - sqrt(((1f - (_e171 * _e172)) * (1f - (_e176 * _e177))))), ((_e183 * _e184) + sqrt(((1f - (_e187 * _e188)) * (1f - (_e192 * _e193))))));
    return;
}

fn ComputeSingleScatteringTexture(atmosphere_20: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, frag_coord_2: vec3<f32>, rayleigh_2: ptr<function, vec3<f32>>, mie_2: ptr<function, vec3<f32>>) {
    var r_21: f32;
    var mu_18: f32;
    var mu_s_8: f32;
    var nu_7: f32;
    var ray_r_mu_intersects_ground_8: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_20, frag_coord_2, (&r_21), (&mu_18), (&mu_s_8), (&nu_7), (&ray_r_mu_intersects_ground_8));
    let _e135 = r_21;
    let _e136 = mu_18;
    let _e137 = mu_s_8;
    let _e138 = nu_7;
    let _e139 = ray_r_mu_intersects_ground_8;
    ComputeSingleScattering(atmosphere_20, transmittance_texture_t_5, transmittance_texture_s_5, _e135, _e136, _e137, _e138, _e139, rayleigh_2, mie_2);
    return;
}

fn GetScattering(atmosphere_21: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, r_22: f32, mu_19: f32, mu_s_9: f32, nu_8: f32, ray_r_mu_intersects_ground_9: bool) -> vec3<f32> {
    var uvwz_2: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;

    let _e122 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_21, r_22, mu_19, mu_s_9, nu_8, ray_r_mu_intersects_ground_9);
    uvwz_2 = _e122;
    let _e124 = uvwz_2;
    tex_coord_x = (_e124.x * 7f);
    let _e132 = tex_coord_x;
    tex_x = floor(_e132);
    let _e135 = tex_coord_x;
    let _e136 = tex_x;
    lerp = (_e135 - _e136);
    let _e139 = tex_x;
    let _e140 = uvwz_2;
    let _e146 = uvwz_2;
    let _e148 = uvwz_2;
    uvw0_ = vec3<f32>(((_e139 + _e140.y) / 8f), _e146.z, _e148.w);
    let _e152 = tex_x;
    let _e155 = uvwz_2;
    let _e161 = uvwz_2;
    let _e163 = uvwz_2;
    uvw1_ = vec3<f32>((((_e152 + 1f) + _e155.y) / 8f), _e161.z, _e163.w);
    let _e167 = uvw0_;
    let _e168 = textureSample(scattering_texture_t, scattering_texture_s, _e167);
    let _e170 = lerp;
    let _e173 = uvw1_;
    let _e174 = textureSample(scattering_texture_t, scattering_texture_s, _e173);
    let _e175 = lerp;
    return vec3<f32>(((_e168 * (1f - _e170)) + (_e174 * _e175)).xyz);
}

fn GetScattering_1(atmosphere_22: AtmosphereParameters, single_rayleigh_scattering_texture_t: texture_3d<f32>, single_rayleigh_scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, multiple_scattering_texture_t: texture_3d<f32>, multiple_scattering_texture_s: sampler, r_23: f32, mu_20: f32, mu_s_10: f32, nu_9: f32, ray_r_mu_intersects_ground_10: bool, scattering_order: i32) -> vec3<f32> {
    var rayleigh_3: vec3<f32>;
    var mie_3: vec3<f32>;

    if (scattering_order == 1i) {
        {
            let _e129 = GetScattering(atmosphere_22, single_rayleigh_scattering_texture_t, single_rayleigh_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            rayleigh_3 = _e129;
            let _e131 = GetScattering(atmosphere_22, single_mie_scattering_texture_t, single_mie_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            mie_3 = _e131;
            let _e133 = rayleigh_3;
            let _e134 = RayleighPhaseFunction(nu_9);
            let _e136 = mie_3;
            let _e138 = MiePhaseFunction(atmosphere_22.mie_phase_function_g, nu_9);
            return ((_e133 * _e134) + (_e136 * _e138));
        }
    } else {
        {
            let _e141 = GetScattering(atmosphere_22, multiple_scattering_texture_t, multiple_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            return _e141;
        }
    }
}

fn ComputeScatteringDensity(atmosphere_23: AtmosphereParameters, transmittance_texture_t_6: texture_2d<f32>, transmittance_texture_s_6: sampler, single_rayleigh_scattering_texture_t_1: texture_3d<f32>, single_rayleigh_scattering_texture_s_1: sampler, single_mie_scattering_texture_t_1: texture_3d<f32>, single_mie_scattering_texture_s_1: sampler, multiple_scattering_texture_t_1: texture_3d<f32>, multiple_scattering_texture_s_1: sampler, irradiance_texture_t_1: texture_2d<f32>, irradiance_texture_s_1: sampler, r_24: f32, mu_21: f32, mu_s_11: f32, nu_10: f32, scattering_order_1: i32) -> vec3<f32> {
    var zenith_direction: vec3<f32> = vec3<f32>(0f, 0f, 1f);
    var omega: vec3<f32>;
    var local_8: f32;
    var sun_dir_x: f32;
    var sun_dir_y: f32;
    var omega_s: vec3<f32>;
    var SAMPLE_COUNT_2: i32 = 16i;
    var dphi: f32;
    var dtheta: f32;
    var rayleigh_mie: vec3<f32> = vec3(0f);
    var l: i32 = 0i;
    var theta: f32;
    var cos_theta: f32;
    var sin_theta: f32;
    var ray_r_theta_intersects_ground: bool;
    var distance_to_ground: f32;
    var transmittance_to_ground: vec3<f32>;
    var ground_albedo: vec3<f32>;
    var m: i32;
    var phi: f32;
    var omega_i: vec3<f32>;
    var domega_i: f32;
    var nu1_: f32;
    var incident_radiance: vec3<f32>;
    var ground_normal: vec3<f32>;
    var ground_irradiance: vec3<f32>;
    var nu2_: f32;
    var rayleigh_density: f32;
    var mie_density: f32;

    omega = vec3<f32>(sqrt((1f - (mu_21 * mu_21))), 0f, mu_21);
    let _e142 = omega;
    if (_e142.x == 0f) {
        local_8 = 0f;
    } else {
        let _e149 = omega;
        local_8 = ((nu_10 - (mu_21 * mu_s_11)) / _e149.x);
    }
    let _e153 = local_8;
    sun_dir_x = _e153;
    let _e156 = sun_dir_x;
    let _e157 = sun_dir_x;
    sun_dir_y = sqrt(max(((1f - (_e156 * _e157)) - (mu_s_11 * mu_s_11)), 0f));
    let _e166 = sun_dir_x;
    let _e167 = sun_dir_y;
    omega_s = vec3<f32>(_e166, _e167, mu_s_11);
    let _e172 = SAMPLE_COUNT_2;
    dphi = (pi / f32(_e172));
    let _e176 = SAMPLE_COUNT_2;
    dtheta = (pi / f32(_e176));
    loop {
        let _e187 = l;
        let _e188 = SAMPLE_COUNT_2;
        if !((_e187 < _e188)) {
            break;
        }
        {
            let _e194 = l;
            let _e198 = dtheta;
            theta = ((f32(_e194) + 0.5f) * _e198);
            let _e201 = theta;
            cos_theta = cos(_e201);
            let _e204 = theta;
            sin_theta = sin(_e204);
            let _e207 = cos_theta;
            let _e208 = RayIntersectsGround(atmosphere_23, r_24, _e207);
            ray_r_theta_intersects_ground = _e208;
            distance_to_ground = 0f;
            transmittance_to_ground = vec3(0f);
            ground_albedo = vec3(0f);
            let _e220 = ray_r_theta_intersects_ground;
            if _e220 {
                {
                    let _e221 = cos_theta;
                    let _e222 = DistanceToBottomAtmosphereBoundary(atmosphere_23, r_24, _e221);
                    distance_to_ground = _e222;
                    let _e223 = cos_theta;
                    let _e224 = distance_to_ground;
                    let _e226 = GetTransmittance(atmosphere_23, transmittance_texture_t_6, transmittance_texture_s_6, r_24, _e223, _e224, true);
                    transmittance_to_ground = _e226;
                    ground_albedo = atmosphere_23.ground_albedo;
                }
            }
            m = 0i;
            loop {
                let _e230 = m;
                let _e232 = SAMPLE_COUNT_2;
                if !((_e230 < (2i * _e232))) {
                    break;
                }
                {
                    let _e239 = m;
                    let _e243 = dphi;
                    phi = ((f32(_e239) + 0.5f) * _e243);
                    let _e246 = phi;
                    let _e248 = sin_theta;
                    let _e250 = phi;
                    let _e252 = sin_theta;
                    let _e254 = cos_theta;
                    omega_i = vec3<f32>((cos(_e246) * _e248), (sin(_e250) * _e252), _e254);
                    let _e257 = dtheta;
                    let _e259 = dphi;
                    let _e262 = theta;
                    domega_i = ((((_e257 / rad) * (_e259 / rad)) * sin(_e262)) * sr);
                    let _e267 = omega_s;
                    let _e268 = omega_i;
                    nu1_ = dot(_e267, _e268);
                    let _e271 = omega_i;
                    let _e273 = nu1_;
                    let _e274 = ray_r_theta_intersects_ground;
                    let _e277 = GetScattering_1(atmosphere_23, single_rayleigh_scattering_texture_t_1, single_rayleigh_scattering_texture_s_1, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, multiple_scattering_texture_t_1, multiple_scattering_texture_s_1, r_24, _e271.z, mu_s_11, _e273, _e274, (scattering_order_1 - 1i));
                    incident_radiance = _e277;
                    let _e279 = zenith_direction;
                    let _e281 = omega_i;
                    let _e282 = distance_to_ground;
                    ground_normal = normalize(((_e279 * r_24) + (_e281 * _e282)));
                    let _e288 = ground_normal;
                    let _e289 = omega_s;
                    let _e291 = GetIrradiance(atmosphere_23, irradiance_texture_t_1, irradiance_texture_s_1, atmosphere_23.bottom_radius, dot(_e288, _e289));
                    ground_irradiance = _e291;
                    let _e293 = incident_radiance;
                    let _e294 = transmittance_to_ground;
                    let _e295 = ground_albedo;
                    let _e303 = ground_irradiance;
                    incident_radiance = (_e293 + (((_e294 * _e295) * 0.31830987f) * _e303));
                    let _e306 = omega;
                    let _e307 = omega_i;
                    nu2_ = dot(_e306, _e307);
                    let _e313 = GetProfileDensity(atmosphere_23.rayleigh_density, (r_24 - atmosphere_23.bottom_radius));
                    rayleigh_density = _e313;
                    let _e318 = GetProfileDensity(atmosphere_23.mie_density, (r_24 - atmosphere_23.bottom_radius));
                    mie_density = _e318;
                    let _e320 = rayleigh_mie;
                    let _e321 = incident_radiance;
                    let _e323 = rayleigh_density;
                    let _e325 = nu2_;
                    let _e326 = RayleighPhaseFunction(_e325);
                    let _e329 = mie_density;
                    let _e332 = nu2_;
                    let _e333 = MiePhaseFunction(atmosphere_23.mie_phase_function_g, _e332);
                    let _e337 = domega_i;
                    rayleigh_mie = (_e320 + ((_e321 * (((atmosphere_23.rayleigh_scattering * _e323) * _e326) + ((atmosphere_23.mie_scattering * _e329) * _e333))) * _e337));
                }
                continuing {
                    let _e236 = m;
                    m = (_e236 + 1i);
                }
            }
        }
        continuing {
            let _e191 = l;
            l = (_e191 + 1i);
        }
    }
    let _e340 = rayleigh_mie;
    return _e340;
}

fn ComputeMultipleScattering(atmosphere_24: AtmosphereParameters, transmittance_texture_t_7: texture_2d<f32>, transmittance_texture_s_7: sampler, scattering_density_texture_t: texture_3d<f32>, scattering_density_texture_s: sampler, r_25: f32, mu_22: f32, mu_s_12: f32, nu_11: f32, ray_r_mu_intersects_ground_11: bool) -> vec3<f32> {
    var SAMPLE_COUNT_3: i32 = 50i;
    var dx_2: f32;
    var rayleigh_mie_sum: vec3<f32> = vec3(0f);
    var i_6: i32 = 0i;
    var d_i_2: f32;
    var r_i_1: f32;
    var mu_i: f32;
    var mu_s_i: f32;
    var rayleigh_mie_i: vec3<f32>;
    var local_9: f32;
    var weight_i_2: f32;

    let _e126 = DistanceToNearestAtmosphereBoundary(atmosphere_24, r_25, mu_22, ray_r_mu_intersects_ground_11);
    let _e127 = SAMPLE_COUNT_3;
    dx_2 = (_e126 / f32(_e127));
    loop {
        let _e138 = i_6;
        let _e139 = SAMPLE_COUNT_3;
        if !((_e138 <= _e139)) {
            break;
        }
        {
            let _e145 = i_6;
            let _e147 = dx_2;
            d_i_2 = (f32(_e145) * _e147);
            let _e150 = d_i_2;
            let _e151 = d_i_2;
            let _e156 = d_i_2;
            let _e162 = ClampRadius(atmosphere_24, sqrt((((_e150 * _e151) + (((2f * r_25) * mu_22) * _e156)) + (r_25 * r_25))));
            r_i_1 = _e162;
            let _e165 = d_i_2;
            let _e167 = r_i_1;
            let _e169 = ClampCosine((((r_25 * mu_22) + _e165) / _e167));
            mu_i = _e169;
            let _e172 = d_i_2;
            let _e175 = r_i_1;
            let _e177 = ClampCosine((((r_25 * mu_s_12) + (_e172 * nu_11)) / _e175));
            mu_s_i = _e177;
            let _e179 = r_i_1;
            let _e180 = mu_i;
            let _e181 = mu_s_i;
            let _e182 = GetScattering(atmosphere_24, scattering_density_texture_t, scattering_density_texture_s, _e179, _e180, _e181, nu_11, ray_r_mu_intersects_ground_11);
            let _e183 = d_i_2;
            let _e184 = GetTransmittance(atmosphere_24, transmittance_texture_t_7, transmittance_texture_s_7, r_25, mu_22, _e183, ray_r_mu_intersects_ground_11);
            let _e186 = dx_2;
            rayleigh_mie_i = ((_e182 * _e184) * _e186);
            let _e189 = i_6;
            let _e192 = i_6;
            let _e193 = SAMPLE_COUNT_3;
            if ((_e189 == 0i) || (_e192 == _e193)) {
                local_9 = 0.5f;
            } else {
                local_9 = 1f;
            }
            let _e199 = local_9;
            weight_i_2 = _e199;
            let _e201 = rayleigh_mie_sum;
            let _e202 = rayleigh_mie_i;
            let _e203 = weight_i_2;
            rayleigh_mie_sum = (_e201 + (_e202 * _e203));
        }
        continuing {
            let _e142 = i_6;
            i_6 = (_e142 + 1i);
        }
    }
    let _e206 = rayleigh_mie_sum;
    return _e206;
}

fn ComputeScatteringDensityTexture(atmosphere_25: AtmosphereParameters, transmittance_texture_t_8: texture_2d<f32>, transmittance_texture_s_8: sampler, single_rayleigh_scattering_texture_t_2: texture_3d<f32>, single_rayleigh_scattering_texture_s_2: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, multiple_scattering_texture_t_2: texture_3d<f32>, multiple_scattering_texture_s_2: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, frag_coord_3: vec3<f32>, scattering_order_2: i32) -> vec3<f32> {
    var r_26: f32;
    var mu_23: f32;
    var mu_s_13: f32;
    var nu_12: f32;
    var ray_r_mu_intersects_ground_12: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_25, frag_coord_3, (&r_26), (&mu_23), (&mu_s_13), (&nu_12), (&ray_r_mu_intersects_ground_12));
    let _e142 = r_26;
    let _e143 = mu_23;
    let _e144 = mu_s_13;
    let _e145 = nu_12;
    let _e146 = ComputeScatteringDensity(atmosphere_25, transmittance_texture_t_8, transmittance_texture_s_8, single_rayleigh_scattering_texture_t_2, single_rayleigh_scattering_texture_s_2, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, multiple_scattering_texture_t_2, multiple_scattering_texture_s_2, irradiance_texture_t_2, irradiance_texture_s_2, _e142, _e143, _e144, _e145, scattering_order_2);
    return _e146;
}

fn ComputeMultipleScatteringTexture(atmosphere_26: AtmosphereParameters, transmittance_texture_t_9: texture_2d<f32>, transmittance_texture_s_9: sampler, scattering_density_texture_t_1: texture_3d<f32>, scattering_density_texture_s_1: sampler, frag_coord_4: vec3<f32>, nu_13: ptr<function, f32>) -> vec3<f32> {
    var r_27: f32;
    var mu_24: f32;
    var mu_s_14: f32;
    var ray_r_mu_intersects_ground_13: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_26, frag_coord_4, (&r_27), (&mu_24), (&mu_s_14), nu_13, (&ray_r_mu_intersects_ground_13));
    let _e135 = r_27;
    let _e136 = mu_24;
    let _e137 = mu_s_14;
    let _e138 = (*nu_13);
    let _e139 = ray_r_mu_intersects_ground_13;
    let _e140 = ComputeMultipleScattering(atmosphere_26, transmittance_texture_t_9, transmittance_texture_s_9, scattering_density_texture_t_1, scattering_density_texture_s_1, _e135, _e136, _e137, _e138, _e139);
    return _e140;
}

fn ComputeDirectIrradiance(atmosphere_27: AtmosphereParameters, transmittance_texture_t_10: texture_2d<f32>, transmittance_texture_s_10: sampler, r_28: f32, mu_s_15: f32) -> vec3<f32> {
    var alpha_s: f32;
    var local_10: f32;
    var local_11: f32;
    var average_cosine_factor: f32;

    alpha_s = (atmosphere_27.sun_angular_radius / rad);
    let _e122 = alpha_s;
    if (mu_s_15 < -(_e122)) {
        local_11 = 0f;
    } else {
        let _e126 = alpha_s;
        if (mu_s_15 > _e126) {
            local_10 = mu_s_15;
        } else {
            let _e128 = alpha_s;
            let _e130 = alpha_s;
            let _e134 = alpha_s;
            local_10 = (((mu_s_15 + _e128) * (mu_s_15 + _e130)) / (4f * _e134));
        }
        let _e138 = local_10;
        local_11 = _e138;
    }
    let _e140 = local_11;
    average_cosine_factor = _e140;
    let _e143 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_27, transmittance_texture_t_10, transmittance_texture_s_10, r_28, mu_s_15);
    let _e145 = average_cosine_factor;
    return ((atmosphere_27.solar_irradiance * _e143) * _e145);
}

fn ComputeIndirectIrradiance(atmosphere_28: AtmosphereParameters, single_rayleigh_scattering_texture_t_3: texture_3d<f32>, single_rayleigh_scattering_texture_s_3: sampler, single_mie_scattering_texture_t_3: texture_3d<f32>, single_mie_scattering_texture_s_3: sampler, multiple_scattering_texture_t_3: texture_3d<f32>, multiple_scattering_texture_s_3: sampler, r_29: f32, mu_s_16: f32, scattering_order_3: i32) -> vec3<f32> {
    var SAMPLE_COUNT_4: i32 = 32i;
    var dphi_1: f32;
    var dtheta_1: f32;
    var result_1: vec3<f32> = vec3(0f);
    var omega_s_1: vec3<f32>;
    var j: i32 = 0i;
    var theta_1: f32;
    var i_7: i32;
    var phi_1: f32;
    var omega_1: vec3<f32>;
    var domega: f32;
    var nu_14: f32;

    let _e126 = SAMPLE_COUNT_4;
    dphi_1 = (pi / f32(_e126));
    let _e130 = SAMPLE_COUNT_4;
    dtheta_1 = (pi / f32(_e130));
    omega_s_1 = vec3<f32>(sqrt((1f - (mu_s_16 * mu_s_16))), 0f, mu_s_16);
    loop {
        let _e148 = j;
        let _e149 = SAMPLE_COUNT_4;
        if !((_e148 < (_e149 / 2i))) {
            break;
        }
        {
            let _e157 = j;
            let _e161 = dtheta_1;
            theta_1 = ((f32(_e157) + 0.5f) * _e161);
            i_7 = 0i;
            loop {
                let _e166 = i_7;
                let _e168 = SAMPLE_COUNT_4;
                if !((_e166 < (2i * _e168))) {
                    break;
                }
                {
                    let _e175 = i_7;
                    let _e179 = dphi_1;
                    phi_1 = ((f32(_e175) + 0.5f) * _e179);
                    let _e182 = phi_1;
                    let _e184 = theta_1;
                    let _e187 = phi_1;
                    let _e189 = theta_1;
                    let _e192 = theta_1;
                    omega_1 = vec3<f32>((cos(_e182) * sin(_e184)), (sin(_e187) * sin(_e189)), cos(_e192));
                    let _e196 = dtheta_1;
                    let _e198 = dphi_1;
                    let _e201 = theta_1;
                    domega = ((((_e196 / rad) * (_e198 / rad)) * sin(_e201)) * sr);
                    let _e206 = omega_1;
                    let _e207 = omega_s_1;
                    nu_14 = dot(_e206, _e207);
                    let _e210 = result_1;
                    let _e211 = omega_1;
                    let _e213 = nu_14;
                    let _e215 = GetScattering_1(atmosphere_28, single_rayleigh_scattering_texture_t_3, single_rayleigh_scattering_texture_s_3, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, multiple_scattering_texture_t_3, multiple_scattering_texture_s_3, r_29, _e211.z, mu_s_16, _e213, false, scattering_order_3);
                    let _e216 = omega_1;
                    let _e219 = domega;
                    result_1 = (_e210 + ((_e215 * _e216.z) * _e219));
                }
                continuing {
                    let _e172 = i_7;
                    i_7 = (_e172 + 1i);
                }
            }
        }
        continuing {
            let _e154 = j;
            j = (_e154 + 1i);
        }
    }
    let _e222 = result_1;
    return _e222;
}

fn GetRMuSFromIrradianceTextureUv(atmosphere_29: AtmosphereParameters, uv_3: vec2<f32>, r_30: ptr<function, f32>, mu_s_17: ptr<function, f32>) {
    var x_mu_s_2: f32;
    var x_r_3: f32;

    let _e120 = GetUnitRangeFromTextureCoord(uv_3.x, 64i);
    x_mu_s_2 = _e120;
    let _e124 = GetUnitRangeFromTextureCoord(uv_3.y, 16i);
    x_r_3 = _e124;
    let _e127 = x_r_3;
    (*r_30) = (atmosphere_29.bottom_radius + (_e127 * (atmosphere_29.top_radius - atmosphere_29.bottom_radius)));
    let _e134 = x_mu_s_2;
    let _e138 = ClampCosine(((2f * _e134) - 1f));
    (*mu_s_17) = _e138;
    return;
}

fn ComputeDirectIrradianceTexture(atmosphere_30: AtmosphereParameters, transmittance_texture_t_11: texture_2d<f32>, transmittance_texture_s_11: sampler, frag_coord_5: vec2<f32>) -> vec3<f32> {
    var r_31: f32;
    var mu_s_18: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_30, (frag_coord_5 / IRRADIANCE_TEXTURE_SIZE), (&r_31), (&mu_s_18));
    let _e126 = r_31;
    let _e127 = mu_s_18;
    let _e128 = ComputeDirectIrradiance(atmosphere_30, transmittance_texture_t_11, transmittance_texture_s_11, _e126, _e127);
    return _e128;
}

fn ComputeIndirectIrradianceTexture(atmosphere_31: AtmosphereParameters, single_rayleigh_scattering_texture_t_4: texture_3d<f32>, single_rayleigh_scattering_texture_s_4: sampler, single_mie_scattering_texture_t_4: texture_3d<f32>, single_mie_scattering_texture_s_4: sampler, multiple_scattering_texture_t_4: texture_3d<f32>, multiple_scattering_texture_s_4: sampler, frag_coord_6: vec2<f32>, scattering_order_4: i32) -> vec3<f32> {
    var r_32: f32;
    var mu_s_19: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_31, (frag_coord_6 / IRRADIANCE_TEXTURE_SIZE), (&r_32), (&mu_s_19));
    let _e131 = r_32;
    let _e132 = mu_s_19;
    let _e133 = ComputeIndirectIrradiance(atmosphere_31, single_rayleigh_scattering_texture_t_4, single_rayleigh_scattering_texture_s_4, single_mie_scattering_texture_t_4, single_mie_scattering_texture_s_4, multiple_scattering_texture_t_4, multiple_scattering_texture_s_4, _e131, _e132, scattering_order_4);
    return _e133;
}

fn pt_main() {
    var deltaIrradiance: vec3<f32>;
    var irradiance: vec3<f32>;

    let _e119 = ATMOSPHERE;
    let _e120 = gl_FragCoord_1;
    let _e122 = global.scatteringOrder;
    let _e123 = ComputeIndirectIrradianceTexture(_e119, singleRayleighScatteringTexture_t, singleRayleighScatteringTexture_s, singleMieScatteringTexture_t, singleMieScatteringTexture_s, multipleScatteringTexture_t, multipleScatteringTexture_s, _e120.xy, _e122);
    deltaIrradiance = _e123;
    let _e124 = global.luminanceFromRadiance;
    let _e125 = deltaIrradiance;
    irradiance = (_e124 * _e125);
    let _e127 = irradiance;
    outputColor = vec4<f32>(_e127.x, _e127.y, _e127.z, 1f);
    return;
}

fn main_1() {
    let _e118 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e118;
    let _e120 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e120;
    let _e122 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e122;
    let _e124 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e124;
    let _e130 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e130;
    let _e136 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e136;
    let _e142 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e142;
    let _e148 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e148;
    let _e154 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e154;
    let _e160 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e160;
    let _e166 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e166;
    let _e172 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e172;
    let _e178 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e178;
    let _e184 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e184;
    let _e186 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e186;
    let _e192 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e192;
    let _e198 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e198;
    let _e204 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e204;
    let _e210 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e210;
    let _e216 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e216;
    let _e222 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e222;
    let _e228 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e228;
    let _e234 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e234;
    let _e240 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e240;
    let _e246 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e246;
    let _e248 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e248;
    let _e250 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e250;
    let _e252 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e252;
    let _e258 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e258;
    let _e264 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e264;
    let _e270 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e270;
    let _e276 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e276;
    let _e282 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e282;
    let _e288 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e288;
    let _e294 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e294;
    let _e300 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e300;
    let _e306 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e306;
    let _e312 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e312;
    let _e314 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e314;
    let _e316 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e316;
    let _e318 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e318;
    pt_main();
    return;
}

@fragment 
fn main(@builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e149 = outputColor;
    return FragmentOutput(_e149);
}
