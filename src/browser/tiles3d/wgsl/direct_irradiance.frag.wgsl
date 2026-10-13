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
var transmittanceTexture_t: texture_2d<f32>;
@group(0) @binding(2) 
var transmittanceTexture_s: sampler;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> outputColor: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e89 = y_1;
    let _e90 = x_1;
    return exp2((_e89 * log2(_e90)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e89 = y_3;
    let _e90 = x_3;
    return exp2((_e89 * log2(_e90)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e89 = y_5;
    let _e90 = x_5;
    return exp2((_e89 * log2(_e90)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e89 = y_7;
    let _e90 = x_7;
    return exp2((_e89 * log2(_e90)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e88 = x_9;
    (*i) = trunc(_e88);
    let _e90 = x_9;
    let _e91 = (*i);
    return (_e90 - _e91);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e88 = x_11;
    (*i_1) = trunc(_e88);
    let _e90 = x_11;
    let _e91 = (*i_1);
    return (_e90 - _e91);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e88 = x_13;
    (*i_2) = trunc(_e88);
    let _e90 = x_13;
    let _e91 = (*i_2);
    return (_e90 - _e91);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e88 = x_15;
    (*i_3) = trunc(_e88);
    let _e90 = x_15;
    let _e91 = (*i_3);
    return (_e90 - _e91);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e87 = v_1;
    b = (bitcast<u32>(_e87) & 2147483647u);
    let _e92 = b;
    return (_e92 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e87 = v_3;
    b_1 = (bitcast<u32>(_e87) & 2147483647u);
    let _e92 = b_1;
    return (_e92 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e87 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e87) & vec2(2147483647u));
    let _e93 = b_2;
    return (_e93 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e87 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e87) & vec3(2147483647u));
    let _e93 = b_3;
    return (_e93 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e87 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e87) & vec4(2147483647u));
    let _e93 = b_4;
    return (_e93 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e87 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e87) & vec2(2147483647u));
    let _e93 = b_5;
    return (_e93 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e87 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e87) & vec3(2147483647u));
    let _e93 = b_6;
    return (_e93 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e87 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e87) & vec4(2147483647u));
    let _e93 = b_7;
    return (_e93 == vec4(2139095040u));
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
    let _e121 = discriminant;
    let _e122 = SafeSqrt(_e121);
    let _e124 = ClampDistance(((-(r_1) * mu_1) + _e122));
    return _e124;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_2: f32, mu_2: f32) -> f32 {
    var discriminant_1: f32;

    discriminant_1 = (((r_2 * r_2) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e121 = discriminant_1;
    let _e122 = SafeSqrt(_e121);
    let _e124 = ClampDistance(((-(r_2) * mu_2) - _e122));
    return _e124;
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
    let _e123 = SafeSqrt(((r_4 * r_4) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e123;
    let _e125 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_4, mu_4);
    d_1 = _e125;
    d_min = (atmosphere_4.top_radius - r_4);
    let _e130 = rho;
    let _e131 = H;
    d_max = (_e130 + _e131);
    let _e134 = d_1;
    let _e135 = d_min;
    let _e137 = d_max;
    let _e138 = d_min;
    x_mu = ((_e134 - _e135) / (_e137 - _e138));
    let _e142 = rho;
    let _e143 = H;
    x_r = (_e142 / _e143);
    let _e146 = x_mu;
    let _e148 = GetTextureCoordFromUnitRange(_e146, 256i);
    let _e149 = x_r;
    let _e151 = GetTextureCoordFromUnitRange(_e149, 64i);
    return vec2<f32>(_e148, _e151);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_5: f32, mu_5: f32) -> vec3<f32> {
    var uv: vec2<f32>;

    let _e111 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_5, mu_5);
    uv = _e111;
    let _e113 = uv;
    let _e114 = textureSample(transmittance_texture_t, transmittance_texture_s, _e113);
    return vec3<f32>(_e114.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_6: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e122 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_6) * mu_6) * d_2)) + (r_6 * r_6))));
    r_d = _e122;
    let _e126 = r_d;
    let _e128 = ClampCosine((((r_6 * mu_6) + d_2) / _e126));
    mu_d = _e128;
    if ray_r_mu_intersects_ground {
        {
            let _e130 = r_d;
            let _e131 = mu_d;
            let _e133 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e130, -(_e131));
            let _e135 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, -(mu_6));
            return min((_e133 / _e135), vec3(1f));
        }
    } else {
        {
            let _e140 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, mu_6);
            let _e141 = r_d;
            let _e142 = mu_d;
            let _e143 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e141, _e142);
            return min((_e140 / _e143), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_7: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_7);
    let _e115 = sin_theta_h;
    let _e116 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e115 * _e116)), 0f)));
    let _e124 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_7, mu_s);
    let _e125 = sin_theta_h;
    let _e130 = sin_theta_h;
    let _e134 = cos_theta_h;
    return (_e124 * smoothstep(((-(_e125) * atmosphere_7.sun_angular_radius) / rad), ((_e130 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e134)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e115 = k;
    return (_e115 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e124 = k_1;
    let _e137 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e124 * (1f + (nu_1 * nu_1))) / _e137);
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
    let _e126 = SafeSqrt(((r_8 * r_8) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e126;
    let _e128 = rho_1;
    let _e129 = H_1;
    let _e132 = GetTextureCoordFromUnitRange((_e128 / _e129), 32i);
    u_r = _e132;
    r_mu = (r_8 * mu_7);
    let _e136 = r_mu;
    let _e137 = r_mu;
    discriminant_2 = (((_e136 * _e137) - (r_8 * r_8)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e147 = r_mu;
            let _e149 = discriminant_2;
            let _e150 = SafeSqrt(_e149);
            d_3 = (-(_e147) - _e150);
            d_min_1 = (r_8 - atmosphere_8.bottom_radius);
            let _e156 = rho_1;
            d_max_1 = _e156;
            let _e160 = d_max_1;
            let _e161 = d_min_1;
            if (_e160 == _e161) {
                local = 0f;
            } else {
                let _e164 = d_3;
                let _e165 = d_min_1;
                let _e167 = d_max_1;
                let _e168 = d_min_1;
                local = ((_e164 - _e165) / (_e167 - _e168));
            }
            let _e172 = local;
            let _e176 = GetTextureCoordFromUnitRange(_e172, 64i);
            u_mu = (0.5f - (0.5f * _e176));
        }
    } else {
        {
            let _e179 = r_mu;
            let _e181 = discriminant_2;
            let _e182 = H_1;
            let _e183 = H_1;
            let _e186 = SafeSqrt((_e181 + (_e182 * _e183)));
            d_4 = (-(_e179) + _e186);
            d_min_2 = (atmosphere_8.top_radius - r_8);
            let _e192 = rho_1;
            let _e193 = H_1;
            d_max_2 = (_e192 + _e193);
            let _e198 = d_4;
            let _e199 = d_min_2;
            let _e201 = d_max_2;
            let _e202 = d_min_2;
            let _e208 = GetTextureCoordFromUnitRange(((_e198 - _e199) / (_e201 - _e202)), 64i);
            u_mu = (0.5f + (0.5f * _e208));
        }
    }
    let _e212 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e212;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e218 = H_1;
    d_max_3 = _e218;
    let _e220 = d_5;
    let _e221 = d_min_3;
    let _e223 = d_max_3;
    let _e224 = d_min_3;
    a_1 = ((_e220 - _e221) / (_e223 - _e224));
    let _e230 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e230;
    let _e232 = D;
    let _e233 = d_min_3;
    let _e235 = d_max_3;
    let _e236 = d_min_3;
    A = ((_e232 - _e233) / (_e235 - _e236));
    let _e241 = a_1;
    let _e242 = A;
    let _e248 = a_1;
    let _e252 = GetTextureCoordFromUnitRange((max((1f - (_e241 / _e242)), 0f) / (1f + _e248)), 32i);
    u_mu_s = _e252;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e259 = u_nu;
    let _e260 = u_mu_s;
    let _e261 = u_mu;
    let _e262 = u_r;
    return vec4<f32>(_e259, _e260, _e261, _e262);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_9: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_9 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e121 = x_mu_s;
    let _e123 = GetTextureCoordFromUnitRange(_e121, 64i);
    let _e124 = x_r_1;
    let _e126 = GetTextureCoordFromUnitRange(_e124, 16i);
    return vec2<f32>(_e123, _e126);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_10: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_1: vec2<f32>;

    let _e111 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_10, mu_s_3);
    uv_1 = _e111;
    let _e113 = uv_1;
    let _e114 = textureSample(irradiance_texture_t, irradiance_texture_s, _e113);
    return vec3<f32>(_e114.xyz);
}

fn GetLayerDensity(layer: DensityProfileLayer, altitude: f32) -> f32 {
    var density: f32;

    density = (((layer.exp_term * exp((layer.exp_scale * altitude))) + (layer.linear_term * altitude)) + layer.constant_term);
    let _e119 = density;
    return clamp(_e119, 0f, 1f);
}

fn GetProfileDensity(profile: DensityProfile, altitude_1: f32) -> f32 {
    var layers: array<DensityProfileLayer, 2>;
    var local_1: f32;

    layers[0i] = profile.layers[0];
    layers[1i] = profile.layers[1];
    let _e121 = layers[0];
    if (altitude_1 < _e121.width) {
        let _e126 = layers[0];
        let _e127 = GetLayerDensity(_e126, altitude_1);
        local_1 = _e127;
    } else {
        let _e130 = layers[1];
        let _e131 = GetLayerDensity(_e130, altitude_1);
        local_1 = _e131;
    }
    let _e133 = local_1;
    return _e133;
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

    let _e112 = DistanceToTopAtmosphereBoundary(atmosphere_11, r_11, mu_8);
    let _e113 = SAMPLE_COUNT;
    dx = (_e112 / f32(_e113));
    loop {
        let _e123 = i_4;
        let _e124 = SAMPLE_COUNT;
        if !((_e123 <= _e124)) {
            break;
        }
        {
            let _e130 = i_4;
            let _e132 = dx;
            d_i = (f32(_e130) * _e132);
            let _e135 = d_i;
            let _e136 = d_i;
            let _e141 = d_i;
            r_i = sqrt((((_e135 * _e136) + (((2f * r_11) * mu_8) * _e141)) + (r_11 * r_11)));
            let _e148 = r_i;
            let _e151 = GetProfileDensity(profile_1, (_e148 - atmosphere_11.bottom_radius));
            y_i = _e151;
            let _e153 = i_4;
            let _e156 = i_4;
            let _e157 = SAMPLE_COUNT;
            if ((_e153 == 0i) || (_e156 == _e157)) {
                local_2 = 0.5f;
            } else {
                local_2 = 1f;
            }
            let _e163 = local_2;
            weight_i = _e163;
            let _e165 = result;
            let _e166 = y_i;
            let _e167 = weight_i;
            let _e169 = dx;
            result = (_e165 + ((_e166 * _e167) * _e169));
        }
        continuing {
            let _e127 = i_4;
            i_4 = (_e127 + 1i);
        }
    }
    let _e172 = result;
    return _e172;
}

fn ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_12: AtmosphereParameters, r_12: f32, mu_9: f32) -> vec3<f32> {
    var optical_depth: vec3<f32>;

    let _e111 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.rayleigh_density, r_12, mu_9);
    let _e115 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.mie_density, r_12, mu_9);
    let _e120 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.absorption_density, r_12, mu_9);
    optical_depth = (((atmosphere_12.rayleigh_scattering * _e111) + (atmosphere_12.mie_extinction * _e115)) + (atmosphere_12.absorption_extinction * _e120));
    let _e124 = optical_depth;
    return exp(-(_e124));
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

    let _e112 = GetUnitRangeFromTextureCoord(uv_2.x, 256i);
    x_mu_1 = _e112;
    let _e116 = GetUnitRangeFromTextureCoord(uv_2.y, 64i);
    x_r_2 = _e116;
    H_2 = sqrt(((atmosphere_13.top_radius * atmosphere_13.top_radius) - (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e127 = H_2;
    let _e128 = x_r_2;
    rho_2 = (_e127 * _e128);
    let _e131 = rho_2;
    let _e132 = rho_2;
    (*r_13) = sqrt(((_e131 * _e132) + (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e140 = (*r_13);
    d_min_4 = (atmosphere_13.top_radius - _e140);
    let _e143 = rho_2;
    let _e144 = H_2;
    d_max_4 = (_e143 + _e144);
    let _e147 = d_min_4;
    let _e148 = x_mu_1;
    let _e149 = d_max_4;
    let _e150 = d_min_4;
    d_6 = (_e147 + (_e148 * (_e149 - _e150)));
    let _e155 = d_6;
    if (_e155 == 0f) {
        local_3 = 1f;
    } else {
        let _e162 = H_2;
        let _e163 = H_2;
        let _e165 = rho_2;
        let _e166 = rho_2;
        let _e169 = d_6;
        let _e170 = d_6;
        let _e174 = (*r_13);
        let _e176 = d_6;
        local_3 = ((((_e162 * _e163) - (_e165 * _e166)) - (_e169 * _e170)) / ((2f * _e174) * _e176));
    }
    let _e180 = local_3;
    (*mu_10) = _e180;
    let _e181 = (*mu_10);
    let _e182 = ClampCosine(_e181);
    (*mu_10) = _e182;
    return;
}

fn ComputeTransmittanceToTopAtmosphereBoundaryTexture(atmosphere_14: AtmosphereParameters, frag_coord: vec2<f32>) -> vec3<f32> {
    var TRANSMITTANCE_TEXTURE_SIZE: vec2<f32> = vec2<f32>(256f, 64f);
    var r_14: f32;
    var mu_11: f32;

    let _e116 = TRANSMITTANCE_TEXTURE_SIZE;
    GetRMuFromTransmittanceTextureUv(atmosphere_14, (frag_coord / _e116), (&r_14), (&mu_11));
    let _e122 = r_14;
    let _e123 = mu_11;
    let _e124 = ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_14, _e122, _e123);
    return _e124;
}

fn ComputeSingleScatteringIntegrand(atmosphere_15: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, r_15: f32, mu_12: f32, mu_s_4: f32, nu_3: f32, d_7: f32, ray_r_mu_intersects_ground_2: bool, rayleigh: ptr<function, vec3<f32>>, mie: ptr<function, vec3<f32>>) {
    var r_d_1: f32;
    var mu_s_d: f32;
    var transmittance: vec3<f32>;

    let _e126 = ClampRadius(atmosphere_15, sqrt((((d_7 * d_7) + (((2f * r_15) * mu_12) * d_7)) + (r_15 * r_15))));
    r_d_1 = _e126;
    let _e131 = r_d_1;
    let _e133 = ClampCosine((((r_15 * mu_s_4) + (d_7 * nu_3)) / _e131));
    mu_s_d = _e133;
    let _e135 = GetTransmittance(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, r_15, mu_12, d_7, ray_r_mu_intersects_ground_2);
    let _e136 = r_d_1;
    let _e137 = mu_s_d;
    let _e138 = GetTransmittanceToSun(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, _e136, _e137);
    transmittance = (_e135 * _e138);
    let _e141 = transmittance;
    let _e143 = r_d_1;
    let _e146 = GetProfileDensity(atmosphere_15.rayleigh_density, (_e143 - atmosphere_15.bottom_radius));
    (*rayleigh) = (_e141 * _e146);
    let _e148 = transmittance;
    let _e150 = r_d_1;
    let _e153 = GetProfileDensity(atmosphere_15.mie_density, (_e150 - atmosphere_15.bottom_radius));
    (*mie) = (_e148 * _e153);
    return;
}

fn DistanceToNearestAtmosphereBoundary(atmosphere_16: AtmosphereParameters, r_16: f32, mu_13: f32, ray_r_mu_intersects_ground_3: bool) -> f32 {
    var r_17: f32;
    var mu_14: f32;
    var ray_r_mu_intersects_ground_4: bool;

    r_17 = r_16;
    mu_14 = mu_13;
    ray_r_mu_intersects_ground_4 = ray_r_mu_intersects_ground_3;
    let _e113 = ray_r_mu_intersects_ground_4;
    if _e113 {
        {
            let _e114 = r_17;
            let _e115 = mu_14;
            let _e116 = DistanceToBottomAtmosphereBoundary(atmosphere_16, _e114, _e115);
            return _e116;
        }
    } else {
        {
            let _e117 = r_17;
            let _e118 = mu_14;
            let _e119 = DistanceToTopAtmosphereBoundary(atmosphere_16, _e117, _e118);
            return _e119;
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

    let _e118 = DistanceToNearestAtmosphereBoundary(atmosphere_17, r_18, mu_15, ray_r_mu_intersects_ground_5);
    let _e119 = SAMPLE_COUNT_1;
    dx_1 = (_e118 / f32(_e119));
    loop {
        let _e131 = i_5;
        let _e132 = SAMPLE_COUNT_1;
        if !((_e131 <= _e132)) {
            break;
        }
        {
            let _e138 = i_5;
            let _e140 = dx_1;
            d_i_1 = (f32(_e138) * _e140);
            let _e145 = d_i_1;
            ComputeSingleScatteringIntegrand(atmosphere_17, transmittance_texture_t_4, transmittance_texture_s_4, r_18, mu_15, mu_s_5, nu_4, _e145, ray_r_mu_intersects_ground_5, (&rayleigh_i), (&mie_i));
            let _e150 = i_5;
            let _e153 = i_5;
            let _e154 = SAMPLE_COUNT_1;
            if ((_e150 == 0i) || (_e153 == _e154)) {
                local_4 = 0.5f;
            } else {
                local_4 = 1f;
            }
            let _e160 = local_4;
            weight_i_1 = _e160;
            let _e162 = rayleigh_sum;
            let _e163 = rayleigh_i;
            let _e164 = weight_i_1;
            rayleigh_sum = (_e162 + (_e163 * _e164));
            let _e167 = mie_sum;
            let _e168 = mie_i;
            let _e169 = weight_i_1;
            mie_sum = (_e167 + (_e168 * _e169));
        }
        continuing {
            let _e135 = i_5;
            i_5 = (_e135 + 1i);
        }
    }
    let _e172 = rayleigh_sum;
    let _e173 = dx_1;
    (*rayleigh_1) = (((_e172 * _e173) * atmosphere_17.solar_irradiance) * atmosphere_17.rayleigh_scattering);
    let _e179 = mie_sum;
    let _e180 = dx_1;
    (*mie_1) = (((_e179 * _e180) * atmosphere_17.solar_irradiance) * atmosphere_17.mie_scattering);
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
    let _e122 = H_3;
    let _e125 = GetUnitRangeFromTextureCoord(uvwz.w, 32i);
    rho_3 = (_e122 * _e125);
    let _e128 = rho_3;
    let _e129 = rho_3;
    (*r_19) = sqrt(((_e128 * _e129) + (atmosphere_18.bottom_radius * atmosphere_18.bottom_radius)));
    if (uvwz.z < 0.5f) {
        {
            let _e139 = (*r_19);
            d_min_5 = (_e139 - atmosphere_18.bottom_radius);
            let _e143 = rho_3;
            d_max_5 = _e143;
            let _e145 = d_min_5;
            let _e146 = d_max_5;
            let _e147 = d_min_5;
            let _e157 = GetUnitRangeFromTextureCoord((1f - (2f * uvwz.z)), 64i);
            d_8 = (_e145 + ((_e146 - _e147) * _e157));
            let _e161 = d_8;
            if (_e161 == 0f) {
                local_5 = -1f;
            } else {
                let _e169 = rho_3;
                let _e170 = rho_3;
                let _e172 = d_8;
                let _e173 = d_8;
                let _e178 = (*r_19);
                let _e180 = d_8;
                let _e183 = ClampCosine((-(((_e169 * _e170) + (_e172 * _e173))) / ((2f * _e178) * _e180)));
                local_5 = _e183;
            }
            let _e185 = local_5;
            (*mu_16) = _e185;
            (*ray_r_mu_intersects_ground_6) = true;
        }
    } else {
        {
            let _e188 = (*r_19);
            d_min_6 = (atmosphere_18.top_radius - _e188);
            let _e191 = rho_3;
            let _e192 = H_3;
            d_max_6 = (_e191 + _e192);
            let _e195 = d_min_6;
            let _e196 = d_max_6;
            let _e197 = d_min_6;
            let _e207 = GetUnitRangeFromTextureCoord(((2f * uvwz.z) - 1f), 64i);
            d_9 = (_e195 + ((_e196 - _e197) * _e207));
            let _e211 = d_9;
            if (_e211 == 0f) {
                local_6 = 1f;
            } else {
                let _e218 = H_3;
                let _e219 = H_3;
                let _e221 = rho_3;
                let _e222 = rho_3;
                let _e225 = d_9;
                let _e226 = d_9;
                let _e230 = (*r_19);
                let _e232 = d_9;
                let _e235 = ClampCosine(((((_e218 * _e219) - (_e221 * _e222)) - (_e225 * _e226)) / ((2f * _e230) * _e232)));
                local_6 = _e235;
            }
            let _e237 = local_6;
            (*mu_16) = _e237;
            (*ray_r_mu_intersects_ground_6) = false;
        }
    }
    let _e241 = GetUnitRangeFromTextureCoord(uvwz.y, 32i);
    x_mu_s_1 = _e241;
    d_min_7 = (atmosphere_18.top_radius - atmosphere_18.bottom_radius);
    let _e247 = H_3;
    d_max_7 = _e247;
    let _e251 = DistanceToTopAtmosphereBoundary(atmosphere_18, atmosphere_18.bottom_radius, atmosphere_18.mu_s_min);
    D_1 = _e251;
    let _e253 = D_1;
    let _e254 = d_min_7;
    let _e256 = d_max_7;
    let _e257 = d_min_7;
    A_1 = ((_e253 - _e254) / (_e256 - _e257));
    let _e261 = A_1;
    let _e262 = x_mu_s_1;
    let _e263 = A_1;
    let _e267 = x_mu_s_1;
    let _e268 = A_1;
    a_2 = ((_e261 - (_e262 * _e263)) / (1f + (_e267 * _e268)));
    let _e273 = d_min_7;
    let _e274 = a_2;
    let _e275 = A_1;
    let _e277 = d_max_7;
    let _e278 = d_min_7;
    d_10 = (_e273 + (min(_e274, _e275) * (_e277 - _e278)));
    let _e283 = d_10;
    if (_e283 == 0f) {
        local_7 = 1f;
    } else {
        let _e290 = H_3;
        let _e291 = H_3;
        let _e293 = d_10;
        let _e294 = d_10;
        let _e300 = d_10;
        let _e303 = ClampCosine((((_e290 * _e291) - (_e293 * _e294)) / ((2f * atmosphere_18.bottom_radius) * _e300)));
        local_7 = _e303;
    }
    let _e305 = local_7;
    (*mu_s_6) = _e305;
    let _e311 = ClampCosine(((uvwz.x * 2f) - 1f));
    (*nu_5) = _e311;
    return;
}

fn GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_19: AtmosphereParameters, frag_coord_1: vec3<f32>, r_20: ptr<function, f32>, mu_17: ptr<function, f32>, mu_s_7: ptr<function, f32>, nu_6: ptr<function, f32>, ray_r_mu_intersects_ground_7: ptr<function, bool>) {
    var SCATTERING_TEXTURE_SIZE: vec4<f32> = vec4<f32>(7f, 32f, 128f, 32f);
    var frag_coord_nu: f32;
    var frag_coord_mu_s: f32;
    var uvwz_1: vec4<f32>;

    frag_coord_nu = floor((frag_coord_1.x / 32f));
    frag_coord_mu_s = (frag_coord_1.x - (floor((frag_coord_1.x / 32f)) * 32f));
    let _e139 = frag_coord_nu;
    let _e140 = frag_coord_mu_s;
    let _e144 = SCATTERING_TEXTURE_SIZE;
    uvwz_1 = (vec4<f32>(_e139, _e140, frag_coord_1.y, frag_coord_1.z) / _e144);
    let _e147 = uvwz_1;
    GetRMuMuSNuFromScatteringTextureUvwz(atmosphere_19, _e147, r_20, mu_17, mu_s_7, nu_6, ray_r_mu_intersects_ground_7);
    let _e158 = (*nu_6);
    let _e159 = (*mu_17);
    let _e160 = (*mu_s_7);
    let _e163 = (*mu_17);
    let _e164 = (*mu_17);
    let _e168 = (*mu_s_7);
    let _e169 = (*mu_s_7);
    let _e175 = (*mu_17);
    let _e176 = (*mu_s_7);
    let _e179 = (*mu_17);
    let _e180 = (*mu_17);
    let _e184 = (*mu_s_7);
    let _e185 = (*mu_s_7);
    (*nu_6) = clamp(_e158, ((_e159 * _e160) - sqrt(((1f - (_e163 * _e164)) * (1f - (_e168 * _e169))))), ((_e175 * _e176) + sqrt(((1f - (_e179 * _e180)) * (1f - (_e184 * _e185))))));
    return;
}

fn ComputeSingleScatteringTexture(atmosphere_20: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, frag_coord_2: vec3<f32>, rayleigh_2: ptr<function, vec3<f32>>, mie_2: ptr<function, vec3<f32>>) {
    var r_21: f32;
    var mu_18: f32;
    var mu_s_8: f32;
    var nu_7: f32;
    var ray_r_mu_intersects_ground_8: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_20, frag_coord_2, (&r_21), (&mu_18), (&mu_s_8), (&nu_7), (&ray_r_mu_intersects_ground_8));
    let _e127 = r_21;
    let _e128 = mu_18;
    let _e129 = mu_s_8;
    let _e130 = nu_7;
    let _e131 = ray_r_mu_intersects_ground_8;
    ComputeSingleScattering(atmosphere_20, transmittance_texture_t_5, transmittance_texture_s_5, _e127, _e128, _e129, _e130, _e131, rayleigh_2, mie_2);
    return;
}

fn GetScattering(atmosphere_21: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, r_22: f32, mu_19: f32, mu_s_9: f32, nu_8: f32, ray_r_mu_intersects_ground_9: bool) -> vec3<f32> {
    var uvwz_2: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;

    let _e114 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_21, r_22, mu_19, mu_s_9, nu_8, ray_r_mu_intersects_ground_9);
    uvwz_2 = _e114;
    let _e116 = uvwz_2;
    tex_coord_x = (_e116.x * 7f);
    let _e124 = tex_coord_x;
    tex_x = floor(_e124);
    let _e127 = tex_coord_x;
    let _e128 = tex_x;
    lerp = (_e127 - _e128);
    let _e131 = tex_x;
    let _e132 = uvwz_2;
    let _e138 = uvwz_2;
    let _e140 = uvwz_2;
    uvw0_ = vec3<f32>(((_e131 + _e132.y) / 8f), _e138.z, _e140.w);
    let _e144 = tex_x;
    let _e147 = uvwz_2;
    let _e153 = uvwz_2;
    let _e155 = uvwz_2;
    uvw1_ = vec3<f32>((((_e144 + 1f) + _e147.y) / 8f), _e153.z, _e155.w);
    let _e159 = uvw0_;
    let _e160 = textureSample(scattering_texture_t, scattering_texture_s, _e159);
    let _e162 = lerp;
    let _e165 = uvw1_;
    let _e166 = textureSample(scattering_texture_t, scattering_texture_s, _e165);
    let _e167 = lerp;
    return vec3<f32>(((_e160 * (1f - _e162)) + (_e166 * _e167)).xyz);
}

fn GetScattering_1(atmosphere_22: AtmosphereParameters, single_rayleigh_scattering_texture_t: texture_3d<f32>, single_rayleigh_scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, multiple_scattering_texture_t: texture_3d<f32>, multiple_scattering_texture_s: sampler, r_23: f32, mu_20: f32, mu_s_10: f32, nu_9: f32, ray_r_mu_intersects_ground_10: bool, scattering_order: i32) -> vec3<f32> {
    var rayleigh_3: vec3<f32>;
    var mie_3: vec3<f32>;

    if (scattering_order == 1i) {
        {
            let _e121 = GetScattering(atmosphere_22, single_rayleigh_scattering_texture_t, single_rayleigh_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            rayleigh_3 = _e121;
            let _e123 = GetScattering(atmosphere_22, single_mie_scattering_texture_t, single_mie_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            mie_3 = _e123;
            let _e125 = rayleigh_3;
            let _e126 = RayleighPhaseFunction(nu_9);
            let _e128 = mie_3;
            let _e130 = MiePhaseFunction(atmosphere_22.mie_phase_function_g, nu_9);
            return ((_e125 * _e126) + (_e128 * _e130));
        }
    } else {
        {
            let _e133 = GetScattering(atmosphere_22, multiple_scattering_texture_t, multiple_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            return _e133;
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
    let _e134 = omega;
    if (_e134.x == 0f) {
        local_8 = 0f;
    } else {
        let _e141 = omega;
        local_8 = ((nu_10 - (mu_21 * mu_s_11)) / _e141.x);
    }
    let _e145 = local_8;
    sun_dir_x = _e145;
    let _e148 = sun_dir_x;
    let _e149 = sun_dir_x;
    sun_dir_y = sqrt(max(((1f - (_e148 * _e149)) - (mu_s_11 * mu_s_11)), 0f));
    let _e158 = sun_dir_x;
    let _e159 = sun_dir_y;
    omega_s = vec3<f32>(_e158, _e159, mu_s_11);
    let _e164 = SAMPLE_COUNT_2;
    dphi = (pi / f32(_e164));
    let _e168 = SAMPLE_COUNT_2;
    dtheta = (pi / f32(_e168));
    loop {
        let _e179 = l;
        let _e180 = SAMPLE_COUNT_2;
        if !((_e179 < _e180)) {
            break;
        }
        {
            let _e186 = l;
            let _e190 = dtheta;
            theta = ((f32(_e186) + 0.5f) * _e190);
            let _e193 = theta;
            cos_theta = cos(_e193);
            let _e196 = theta;
            sin_theta = sin(_e196);
            let _e199 = cos_theta;
            let _e200 = RayIntersectsGround(atmosphere_23, r_24, _e199);
            ray_r_theta_intersects_ground = _e200;
            distance_to_ground = 0f;
            transmittance_to_ground = vec3(0f);
            ground_albedo = vec3(0f);
            let _e212 = ray_r_theta_intersects_ground;
            if _e212 {
                {
                    let _e213 = cos_theta;
                    let _e214 = DistanceToBottomAtmosphereBoundary(atmosphere_23, r_24, _e213);
                    distance_to_ground = _e214;
                    let _e215 = cos_theta;
                    let _e216 = distance_to_ground;
                    let _e218 = GetTransmittance(atmosphere_23, transmittance_texture_t_6, transmittance_texture_s_6, r_24, _e215, _e216, true);
                    transmittance_to_ground = _e218;
                    ground_albedo = atmosphere_23.ground_albedo;
                }
            }
            m = 0i;
            loop {
                let _e222 = m;
                let _e224 = SAMPLE_COUNT_2;
                if !((_e222 < (2i * _e224))) {
                    break;
                }
                {
                    let _e231 = m;
                    let _e235 = dphi;
                    phi = ((f32(_e231) + 0.5f) * _e235);
                    let _e238 = phi;
                    let _e240 = sin_theta;
                    let _e242 = phi;
                    let _e244 = sin_theta;
                    let _e246 = cos_theta;
                    omega_i = vec3<f32>((cos(_e238) * _e240), (sin(_e242) * _e244), _e246);
                    let _e249 = dtheta;
                    let _e251 = dphi;
                    let _e254 = theta;
                    domega_i = ((((_e249 / rad) * (_e251 / rad)) * sin(_e254)) * sr);
                    let _e259 = omega_s;
                    let _e260 = omega_i;
                    nu1_ = dot(_e259, _e260);
                    let _e263 = omega_i;
                    let _e265 = nu1_;
                    let _e266 = ray_r_theta_intersects_ground;
                    let _e269 = GetScattering_1(atmosphere_23, single_rayleigh_scattering_texture_t_1, single_rayleigh_scattering_texture_s_1, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, multiple_scattering_texture_t_1, multiple_scattering_texture_s_1, r_24, _e263.z, mu_s_11, _e265, _e266, (scattering_order_1 - 1i));
                    incident_radiance = _e269;
                    let _e271 = zenith_direction;
                    let _e273 = omega_i;
                    let _e274 = distance_to_ground;
                    ground_normal = normalize(((_e271 * r_24) + (_e273 * _e274)));
                    let _e280 = ground_normal;
                    let _e281 = omega_s;
                    let _e283 = GetIrradiance(atmosphere_23, irradiance_texture_t_1, irradiance_texture_s_1, atmosphere_23.bottom_radius, dot(_e280, _e281));
                    ground_irradiance = _e283;
                    let _e285 = incident_radiance;
                    let _e286 = transmittance_to_ground;
                    let _e287 = ground_albedo;
                    let _e295 = ground_irradiance;
                    incident_radiance = (_e285 + (((_e286 * _e287) * 0.31830987f) * _e295));
                    let _e298 = omega;
                    let _e299 = omega_i;
                    nu2_ = dot(_e298, _e299);
                    let _e305 = GetProfileDensity(atmosphere_23.rayleigh_density, (r_24 - atmosphere_23.bottom_radius));
                    rayleigh_density = _e305;
                    let _e310 = GetProfileDensity(atmosphere_23.mie_density, (r_24 - atmosphere_23.bottom_radius));
                    mie_density = _e310;
                    let _e312 = rayleigh_mie;
                    let _e313 = incident_radiance;
                    let _e315 = rayleigh_density;
                    let _e317 = nu2_;
                    let _e318 = RayleighPhaseFunction(_e317);
                    let _e321 = mie_density;
                    let _e324 = nu2_;
                    let _e325 = MiePhaseFunction(atmosphere_23.mie_phase_function_g, _e324);
                    let _e329 = domega_i;
                    rayleigh_mie = (_e312 + ((_e313 * (((atmosphere_23.rayleigh_scattering * _e315) * _e318) + ((atmosphere_23.mie_scattering * _e321) * _e325))) * _e329));
                }
                continuing {
                    let _e228 = m;
                    m = (_e228 + 1i);
                }
            }
        }
        continuing {
            let _e183 = l;
            l = (_e183 + 1i);
        }
    }
    let _e332 = rayleigh_mie;
    return _e332;
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

    let _e118 = DistanceToNearestAtmosphereBoundary(atmosphere_24, r_25, mu_22, ray_r_mu_intersects_ground_11);
    let _e119 = SAMPLE_COUNT_3;
    dx_2 = (_e118 / f32(_e119));
    loop {
        let _e130 = i_6;
        let _e131 = SAMPLE_COUNT_3;
        if !((_e130 <= _e131)) {
            break;
        }
        {
            let _e137 = i_6;
            let _e139 = dx_2;
            d_i_2 = (f32(_e137) * _e139);
            let _e142 = d_i_2;
            let _e143 = d_i_2;
            let _e148 = d_i_2;
            let _e154 = ClampRadius(atmosphere_24, sqrt((((_e142 * _e143) + (((2f * r_25) * mu_22) * _e148)) + (r_25 * r_25))));
            r_i_1 = _e154;
            let _e157 = d_i_2;
            let _e159 = r_i_1;
            let _e161 = ClampCosine((((r_25 * mu_22) + _e157) / _e159));
            mu_i = _e161;
            let _e164 = d_i_2;
            let _e167 = r_i_1;
            let _e169 = ClampCosine((((r_25 * mu_s_12) + (_e164 * nu_11)) / _e167));
            mu_s_i = _e169;
            let _e171 = r_i_1;
            let _e172 = mu_i;
            let _e173 = mu_s_i;
            let _e174 = GetScattering(atmosphere_24, scattering_density_texture_t, scattering_density_texture_s, _e171, _e172, _e173, nu_11, ray_r_mu_intersects_ground_11);
            let _e175 = d_i_2;
            let _e176 = GetTransmittance(atmosphere_24, transmittance_texture_t_7, transmittance_texture_s_7, r_25, mu_22, _e175, ray_r_mu_intersects_ground_11);
            let _e178 = dx_2;
            rayleigh_mie_i = ((_e174 * _e176) * _e178);
            let _e181 = i_6;
            let _e184 = i_6;
            let _e185 = SAMPLE_COUNT_3;
            if ((_e181 == 0i) || (_e184 == _e185)) {
                local_9 = 0.5f;
            } else {
                local_9 = 1f;
            }
            let _e191 = local_9;
            weight_i_2 = _e191;
            let _e193 = rayleigh_mie_sum;
            let _e194 = rayleigh_mie_i;
            let _e195 = weight_i_2;
            rayleigh_mie_sum = (_e193 + (_e194 * _e195));
        }
        continuing {
            let _e134 = i_6;
            i_6 = (_e134 + 1i);
        }
    }
    let _e198 = rayleigh_mie_sum;
    return _e198;
}

fn ComputeScatteringDensityTexture(atmosphere_25: AtmosphereParameters, transmittance_texture_t_8: texture_2d<f32>, transmittance_texture_s_8: sampler, single_rayleigh_scattering_texture_t_2: texture_3d<f32>, single_rayleigh_scattering_texture_s_2: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, multiple_scattering_texture_t_2: texture_3d<f32>, multiple_scattering_texture_s_2: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, frag_coord_3: vec3<f32>, scattering_order_2: i32) -> vec3<f32> {
    var r_26: f32;
    var mu_23: f32;
    var mu_s_13: f32;
    var nu_12: f32;
    var ray_r_mu_intersects_ground_12: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_25, frag_coord_3, (&r_26), (&mu_23), (&mu_s_13), (&nu_12), (&ray_r_mu_intersects_ground_12));
    let _e134 = r_26;
    let _e135 = mu_23;
    let _e136 = mu_s_13;
    let _e137 = nu_12;
    let _e138 = ComputeScatteringDensity(atmosphere_25, transmittance_texture_t_8, transmittance_texture_s_8, single_rayleigh_scattering_texture_t_2, single_rayleigh_scattering_texture_s_2, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, multiple_scattering_texture_t_2, multiple_scattering_texture_s_2, irradiance_texture_t_2, irradiance_texture_s_2, _e134, _e135, _e136, _e137, scattering_order_2);
    return _e138;
}

fn ComputeMultipleScatteringTexture(atmosphere_26: AtmosphereParameters, transmittance_texture_t_9: texture_2d<f32>, transmittance_texture_s_9: sampler, scattering_density_texture_t_1: texture_3d<f32>, scattering_density_texture_s_1: sampler, frag_coord_4: vec3<f32>, nu_13: ptr<function, f32>) -> vec3<f32> {
    var r_27: f32;
    var mu_24: f32;
    var mu_s_14: f32;
    var ray_r_mu_intersects_ground_13: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_26, frag_coord_4, (&r_27), (&mu_24), (&mu_s_14), nu_13, (&ray_r_mu_intersects_ground_13));
    let _e127 = r_27;
    let _e128 = mu_24;
    let _e129 = mu_s_14;
    let _e130 = (*nu_13);
    let _e131 = ray_r_mu_intersects_ground_13;
    let _e132 = ComputeMultipleScattering(atmosphere_26, transmittance_texture_t_9, transmittance_texture_s_9, scattering_density_texture_t_1, scattering_density_texture_s_1, _e127, _e128, _e129, _e130, _e131);
    return _e132;
}

fn ComputeDirectIrradiance(atmosphere_27: AtmosphereParameters, transmittance_texture_t_10: texture_2d<f32>, transmittance_texture_s_10: sampler, r_28: f32, mu_s_15: f32) -> vec3<f32> {
    var alpha_s: f32;
    var local_10: f32;
    var local_11: f32;
    var average_cosine_factor: f32;

    alpha_s = (atmosphere_27.sun_angular_radius / rad);
    let _e114 = alpha_s;
    if (mu_s_15 < -(_e114)) {
        local_11 = 0f;
    } else {
        let _e118 = alpha_s;
        if (mu_s_15 > _e118) {
            local_10 = mu_s_15;
        } else {
            let _e120 = alpha_s;
            let _e122 = alpha_s;
            let _e126 = alpha_s;
            local_10 = (((mu_s_15 + _e120) * (mu_s_15 + _e122)) / (4f * _e126));
        }
        let _e130 = local_10;
        local_11 = _e130;
    }
    let _e132 = local_11;
    average_cosine_factor = _e132;
    let _e135 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_27, transmittance_texture_t_10, transmittance_texture_s_10, r_28, mu_s_15);
    let _e137 = average_cosine_factor;
    return ((atmosphere_27.solar_irradiance * _e135) * _e137);
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

    let _e118 = SAMPLE_COUNT_4;
    dphi_1 = (pi / f32(_e118));
    let _e122 = SAMPLE_COUNT_4;
    dtheta_1 = (pi / f32(_e122));
    omega_s_1 = vec3<f32>(sqrt((1f - (mu_s_16 * mu_s_16))), 0f, mu_s_16);
    loop {
        let _e140 = j;
        let _e141 = SAMPLE_COUNT_4;
        if !((_e140 < (_e141 / 2i))) {
            break;
        }
        {
            let _e149 = j;
            let _e153 = dtheta_1;
            theta_1 = ((f32(_e149) + 0.5f) * _e153);
            i_7 = 0i;
            loop {
                let _e158 = i_7;
                let _e160 = SAMPLE_COUNT_4;
                if !((_e158 < (2i * _e160))) {
                    break;
                }
                {
                    let _e167 = i_7;
                    let _e171 = dphi_1;
                    phi_1 = ((f32(_e167) + 0.5f) * _e171);
                    let _e174 = phi_1;
                    let _e176 = theta_1;
                    let _e179 = phi_1;
                    let _e181 = theta_1;
                    let _e184 = theta_1;
                    omega_1 = vec3<f32>((cos(_e174) * sin(_e176)), (sin(_e179) * sin(_e181)), cos(_e184));
                    let _e188 = dtheta_1;
                    let _e190 = dphi_1;
                    let _e193 = theta_1;
                    domega = ((((_e188 / rad) * (_e190 / rad)) * sin(_e193)) * sr);
                    let _e198 = omega_1;
                    let _e199 = omega_s_1;
                    nu_14 = dot(_e198, _e199);
                    let _e202 = result_1;
                    let _e203 = omega_1;
                    let _e205 = nu_14;
                    let _e207 = GetScattering_1(atmosphere_28, single_rayleigh_scattering_texture_t_3, single_rayleigh_scattering_texture_s_3, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, multiple_scattering_texture_t_3, multiple_scattering_texture_s_3, r_29, _e203.z, mu_s_16, _e205, false, scattering_order_3);
                    let _e208 = omega_1;
                    let _e211 = domega;
                    result_1 = (_e202 + ((_e207 * _e208.z) * _e211));
                }
                continuing {
                    let _e164 = i_7;
                    i_7 = (_e164 + 1i);
                }
            }
        }
        continuing {
            let _e146 = j;
            j = (_e146 + 1i);
        }
    }
    let _e214 = result_1;
    return _e214;
}

fn GetRMuSFromIrradianceTextureUv(atmosphere_29: AtmosphereParameters, uv_3: vec2<f32>, r_30: ptr<function, f32>, mu_s_17: ptr<function, f32>) {
    var x_mu_s_2: f32;
    var x_r_3: f32;

    let _e112 = GetUnitRangeFromTextureCoord(uv_3.x, 64i);
    x_mu_s_2 = _e112;
    let _e116 = GetUnitRangeFromTextureCoord(uv_3.y, 16i);
    x_r_3 = _e116;
    let _e119 = x_r_3;
    (*r_30) = (atmosphere_29.bottom_radius + (_e119 * (atmosphere_29.top_radius - atmosphere_29.bottom_radius)));
    let _e126 = x_mu_s_2;
    let _e130 = ClampCosine(((2f * _e126) - 1f));
    (*mu_s_17) = _e130;
    return;
}

fn ComputeDirectIrradianceTexture(atmosphere_30: AtmosphereParameters, transmittance_texture_t_11: texture_2d<f32>, transmittance_texture_s_11: sampler, frag_coord_5: vec2<f32>) -> vec3<f32> {
    var r_31: f32;
    var mu_s_18: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_30, (frag_coord_5 / IRRADIANCE_TEXTURE_SIZE), (&r_31), (&mu_s_18));
    let _e118 = r_31;
    let _e119 = mu_s_18;
    let _e120 = ComputeDirectIrradiance(atmosphere_30, transmittance_texture_t_11, transmittance_texture_s_11, _e118, _e119);
    return _e120;
}

fn ComputeIndirectIrradianceTexture(atmosphere_31: AtmosphereParameters, single_rayleigh_scattering_texture_t_4: texture_3d<f32>, single_rayleigh_scattering_texture_s_4: sampler, single_mie_scattering_texture_t_4: texture_3d<f32>, single_mie_scattering_texture_s_4: sampler, multiple_scattering_texture_t_4: texture_3d<f32>, multiple_scattering_texture_s_4: sampler, frag_coord_6: vec2<f32>, scattering_order_4: i32) -> vec3<f32> {
    var r_32: f32;
    var mu_s_19: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_31, (frag_coord_6 / IRRADIANCE_TEXTURE_SIZE), (&r_32), (&mu_s_19));
    let _e123 = r_32;
    let _e124 = mu_s_19;
    let _e125 = ComputeIndirectIrradiance(atmosphere_31, single_rayleigh_scattering_texture_t_4, single_rayleigh_scattering_texture_s_4, single_mie_scattering_texture_t_4, single_mie_scattering_texture_s_4, multiple_scattering_texture_t_4, multiple_scattering_texture_s_4, _e123, _e124, scattering_order_4);
    return _e125;
}

fn pt_main() {
    var deltaIrradiance: vec3<f32>;
    var irradiance: vec3<f32>;

    let _e111 = ATMOSPHERE;
    let _e112 = gl_FragCoord_1;
    let _e114 = ComputeDirectIrradianceTexture(_e111, transmittanceTexture_t, transmittanceTexture_s, _e112.xy);
    deltaIrradiance = _e114;
    irradiance = vec3(0f);
    let _e117 = irradiance;
    outputColor = vec4<f32>(_e117.x, _e117.y, _e117.z, 1f);
    return;
}

fn main_1() {
    let _e110 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e110;
    let _e112 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e112;
    let _e114 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e114;
    let _e116 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e116;
    let _e122 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e122;
    let _e128 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e128;
    let _e134 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e134;
    let _e140 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e140;
    let _e146 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e146;
    let _e152 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e152;
    let _e158 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e158;
    let _e164 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e164;
    let _e170 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e170;
    let _e176 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e176;
    let _e178 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e178;
    let _e184 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e184;
    let _e190 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e190;
    let _e196 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e196;
    let _e202 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e202;
    let _e208 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e208;
    let _e214 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e214;
    let _e220 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e220;
    let _e226 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e226;
    let _e232 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e232;
    let _e238 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e238;
    let _e240 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e240;
    let _e242 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e242;
    let _e244 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e244;
    let _e250 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e250;
    let _e256 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e256;
    let _e262 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e262;
    let _e268 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e268;
    let _e274 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e274;
    let _e280 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e280;
    let _e286 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e286;
    let _e292 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e292;
    let _e298 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e298;
    let _e304 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e304;
    let _e306 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e306;
    let _e308 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e308;
    let _e310 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e310;
    pt_main();
    return;
}

@fragment 
fn main(@builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e137 = outputColor;
    return FragmentOutput(_e137);
}
