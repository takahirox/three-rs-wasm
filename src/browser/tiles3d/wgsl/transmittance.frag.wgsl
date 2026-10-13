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
    @location(0) transmittance: vec4<f32>,
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
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> transmittance_1: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e87 = y_1;
    let _e88 = x_1;
    return exp2((_e87 * log2(_e88)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e87 = y_3;
    let _e88 = x_3;
    return exp2((_e87 * log2(_e88)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e87 = y_5;
    let _e88 = x_5;
    return exp2((_e87 * log2(_e88)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e87 = y_7;
    let _e88 = x_7;
    return exp2((_e87 * log2(_e88)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e86 = x_9;
    (*i) = trunc(_e86);
    let _e88 = x_9;
    let _e89 = (*i);
    return (_e88 - _e89);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e86 = x_11;
    (*i_1) = trunc(_e86);
    let _e88 = x_11;
    let _e89 = (*i_1);
    return (_e88 - _e89);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e86 = x_13;
    (*i_2) = trunc(_e86);
    let _e88 = x_13;
    let _e89 = (*i_2);
    return (_e88 - _e89);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e86 = x_15;
    (*i_3) = trunc(_e86);
    let _e88 = x_15;
    let _e89 = (*i_3);
    return (_e88 - _e89);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e85 = v_1;
    b = (bitcast<u32>(_e85) & 2147483647u);
    let _e90 = b;
    return (_e90 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e85 = v_3;
    b_1 = (bitcast<u32>(_e85) & 2147483647u);
    let _e90 = b_1;
    return (_e90 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e85 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e85) & vec2(2147483647u));
    let _e91 = b_2;
    return (_e91 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e85 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e85) & vec3(2147483647u));
    let _e91 = b_3;
    return (_e91 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e85 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e85) & vec4(2147483647u));
    let _e91 = b_4;
    return (_e91 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e85 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e85) & vec2(2147483647u));
    let _e91 = b_5;
    return (_e91 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e85 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e85) & vec3(2147483647u));
    let _e91 = b_6;
    return (_e91 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e85 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e85) & vec4(2147483647u));
    let _e91 = b_7;
    return (_e91 == vec4(2139095040u));
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
    let _e119 = discriminant;
    let _e120 = SafeSqrt(_e119);
    let _e122 = ClampDistance(((-(r_1) * mu_1) + _e120));
    return _e122;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_2: f32, mu_2: f32) -> f32 {
    var discriminant_1: f32;

    discriminant_1 = (((r_2 * r_2) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e119 = discriminant_1;
    let _e120 = SafeSqrt(_e119);
    let _e122 = ClampDistance(((-(r_2) * mu_2) - _e120));
    return _e122;
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
    let _e121 = SafeSqrt(((r_4 * r_4) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e121;
    let _e123 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_4, mu_4);
    d_1 = _e123;
    d_min = (atmosphere_4.top_radius - r_4);
    let _e128 = rho;
    let _e129 = H;
    d_max = (_e128 + _e129);
    let _e132 = d_1;
    let _e133 = d_min;
    let _e135 = d_max;
    let _e136 = d_min;
    x_mu = ((_e132 - _e133) / (_e135 - _e136));
    let _e140 = rho;
    let _e141 = H;
    x_r = (_e140 / _e141);
    let _e144 = x_mu;
    let _e146 = GetTextureCoordFromUnitRange(_e144, 256i);
    let _e147 = x_r;
    let _e149 = GetTextureCoordFromUnitRange(_e147, 64i);
    return vec2<f32>(_e146, _e149);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_5: f32, mu_5: f32) -> vec3<f32> {
    var uv: vec2<f32>;

    let _e109 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_5, mu_5);
    uv = _e109;
    let _e111 = uv;
    let _e112 = textureSample(transmittance_texture_t, transmittance_texture_s, _e111);
    return vec3<f32>(_e112.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_6: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e120 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_6) * mu_6) * d_2)) + (r_6 * r_6))));
    r_d = _e120;
    let _e124 = r_d;
    let _e126 = ClampCosine((((r_6 * mu_6) + d_2) / _e124));
    mu_d = _e126;
    if ray_r_mu_intersects_ground {
        {
            let _e128 = r_d;
            let _e129 = mu_d;
            let _e131 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e128, -(_e129));
            let _e133 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, -(mu_6));
            return min((_e131 / _e133), vec3(1f));
        }
    } else {
        {
            let _e138 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, mu_6);
            let _e139 = r_d;
            let _e140 = mu_d;
            let _e141 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e139, _e140);
            return min((_e138 / _e141), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_7: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_7);
    let _e113 = sin_theta_h;
    let _e114 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e113 * _e114)), 0f)));
    let _e122 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_7, mu_s);
    let _e123 = sin_theta_h;
    let _e128 = sin_theta_h;
    let _e132 = cos_theta_h;
    return (_e122 * smoothstep(((-(_e123) * atmosphere_7.sun_angular_radius) / rad), ((_e128 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e132)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e113 = k;
    return (_e113 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e122 = k_1;
    let _e135 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e122 * (1f + (nu_1 * nu_1))) / _e135);
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
    let _e124 = SafeSqrt(((r_8 * r_8) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e124;
    let _e126 = rho_1;
    let _e127 = H_1;
    let _e130 = GetTextureCoordFromUnitRange((_e126 / _e127), 32i);
    u_r = _e130;
    r_mu = (r_8 * mu_7);
    let _e134 = r_mu;
    let _e135 = r_mu;
    discriminant_2 = (((_e134 * _e135) - (r_8 * r_8)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e145 = r_mu;
            let _e147 = discriminant_2;
            let _e148 = SafeSqrt(_e147);
            d_3 = (-(_e145) - _e148);
            d_min_1 = (r_8 - atmosphere_8.bottom_radius);
            let _e154 = rho_1;
            d_max_1 = _e154;
            let _e158 = d_max_1;
            let _e159 = d_min_1;
            if (_e158 == _e159) {
                local = 0f;
            } else {
                let _e162 = d_3;
                let _e163 = d_min_1;
                let _e165 = d_max_1;
                let _e166 = d_min_1;
                local = ((_e162 - _e163) / (_e165 - _e166));
            }
            let _e170 = local;
            let _e174 = GetTextureCoordFromUnitRange(_e170, 64i);
            u_mu = (0.5f - (0.5f * _e174));
        }
    } else {
        {
            let _e177 = r_mu;
            let _e179 = discriminant_2;
            let _e180 = H_1;
            let _e181 = H_1;
            let _e184 = SafeSqrt((_e179 + (_e180 * _e181)));
            d_4 = (-(_e177) + _e184);
            d_min_2 = (atmosphere_8.top_radius - r_8);
            let _e190 = rho_1;
            let _e191 = H_1;
            d_max_2 = (_e190 + _e191);
            let _e196 = d_4;
            let _e197 = d_min_2;
            let _e199 = d_max_2;
            let _e200 = d_min_2;
            let _e206 = GetTextureCoordFromUnitRange(((_e196 - _e197) / (_e199 - _e200)), 64i);
            u_mu = (0.5f + (0.5f * _e206));
        }
    }
    let _e210 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e210;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e216 = H_1;
    d_max_3 = _e216;
    let _e218 = d_5;
    let _e219 = d_min_3;
    let _e221 = d_max_3;
    let _e222 = d_min_3;
    a_1 = ((_e218 - _e219) / (_e221 - _e222));
    let _e228 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e228;
    let _e230 = D;
    let _e231 = d_min_3;
    let _e233 = d_max_3;
    let _e234 = d_min_3;
    A = ((_e230 - _e231) / (_e233 - _e234));
    let _e239 = a_1;
    let _e240 = A;
    let _e246 = a_1;
    let _e250 = GetTextureCoordFromUnitRange((max((1f - (_e239 / _e240)), 0f) / (1f + _e246)), 32i);
    u_mu_s = _e250;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e257 = u_nu;
    let _e258 = u_mu_s;
    let _e259 = u_mu;
    let _e260 = u_r;
    return vec4<f32>(_e257, _e258, _e259, _e260);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_9: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_9 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e119 = x_mu_s;
    let _e121 = GetTextureCoordFromUnitRange(_e119, 64i);
    let _e122 = x_r_1;
    let _e124 = GetTextureCoordFromUnitRange(_e122, 16i);
    return vec2<f32>(_e121, _e124);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_10: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_1: vec2<f32>;

    let _e109 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_10, mu_s_3);
    uv_1 = _e109;
    let _e111 = uv_1;
    let _e112 = textureSample(irradiance_texture_t, irradiance_texture_s, _e111);
    return vec3<f32>(_e112.xyz);
}

fn GetLayerDensity(layer: DensityProfileLayer, altitude: f32) -> f32 {
    var density: f32;

    density = (((layer.exp_term * exp((layer.exp_scale * altitude))) + (layer.linear_term * altitude)) + layer.constant_term);
    let _e117 = density;
    return clamp(_e117, 0f, 1f);
}

fn GetProfileDensity(profile: DensityProfile, altitude_1: f32) -> f32 {
    var layers: array<DensityProfileLayer, 2>;
    var local_1: f32;

    layers[0i] = profile.layers[0];
    layers[1i] = profile.layers[1];
    let _e119 = layers[0];
    if (altitude_1 < _e119.width) {
        let _e124 = layers[0];
        let _e125 = GetLayerDensity(_e124, altitude_1);
        local_1 = _e125;
    } else {
        let _e128 = layers[1];
        let _e129 = GetLayerDensity(_e128, altitude_1);
        local_1 = _e129;
    }
    let _e131 = local_1;
    return _e131;
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

    let _e110 = DistanceToTopAtmosphereBoundary(atmosphere_11, r_11, mu_8);
    let _e111 = SAMPLE_COUNT;
    dx = (_e110 / f32(_e111));
    loop {
        let _e121 = i_4;
        let _e122 = SAMPLE_COUNT;
        if !((_e121 <= _e122)) {
            break;
        }
        {
            let _e128 = i_4;
            let _e130 = dx;
            d_i = (f32(_e128) * _e130);
            let _e133 = d_i;
            let _e134 = d_i;
            let _e139 = d_i;
            r_i = sqrt((((_e133 * _e134) + (((2f * r_11) * mu_8) * _e139)) + (r_11 * r_11)));
            let _e146 = r_i;
            let _e149 = GetProfileDensity(profile_1, (_e146 - atmosphere_11.bottom_radius));
            y_i = _e149;
            let _e151 = i_4;
            let _e154 = i_4;
            let _e155 = SAMPLE_COUNT;
            if ((_e151 == 0i) || (_e154 == _e155)) {
                local_2 = 0.5f;
            } else {
                local_2 = 1f;
            }
            let _e161 = local_2;
            weight_i = _e161;
            let _e163 = result;
            let _e164 = y_i;
            let _e165 = weight_i;
            let _e167 = dx;
            result = (_e163 + ((_e164 * _e165) * _e167));
        }
        continuing {
            let _e125 = i_4;
            i_4 = (_e125 + 1i);
        }
    }
    let _e170 = result;
    return _e170;
}

fn ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_12: AtmosphereParameters, r_12: f32, mu_9: f32) -> vec3<f32> {
    var optical_depth: vec3<f32>;

    let _e109 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.rayleigh_density, r_12, mu_9);
    let _e113 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.mie_density, r_12, mu_9);
    let _e118 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.absorption_density, r_12, mu_9);
    optical_depth = (((atmosphere_12.rayleigh_scattering * _e109) + (atmosphere_12.mie_extinction * _e113)) + (atmosphere_12.absorption_extinction * _e118));
    let _e122 = optical_depth;
    return exp(-(_e122));
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

    let _e110 = GetUnitRangeFromTextureCoord(uv_2.x, 256i);
    x_mu_1 = _e110;
    let _e114 = GetUnitRangeFromTextureCoord(uv_2.y, 64i);
    x_r_2 = _e114;
    H_2 = sqrt(((atmosphere_13.top_radius * atmosphere_13.top_radius) - (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e125 = H_2;
    let _e126 = x_r_2;
    rho_2 = (_e125 * _e126);
    let _e129 = rho_2;
    let _e130 = rho_2;
    (*r_13) = sqrt(((_e129 * _e130) + (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e138 = (*r_13);
    d_min_4 = (atmosphere_13.top_radius - _e138);
    let _e141 = rho_2;
    let _e142 = H_2;
    d_max_4 = (_e141 + _e142);
    let _e145 = d_min_4;
    let _e146 = x_mu_1;
    let _e147 = d_max_4;
    let _e148 = d_min_4;
    d_6 = (_e145 + (_e146 * (_e147 - _e148)));
    let _e153 = d_6;
    if (_e153 == 0f) {
        local_3 = 1f;
    } else {
        let _e160 = H_2;
        let _e161 = H_2;
        let _e163 = rho_2;
        let _e164 = rho_2;
        let _e167 = d_6;
        let _e168 = d_6;
        let _e172 = (*r_13);
        let _e174 = d_6;
        local_3 = ((((_e160 * _e161) - (_e163 * _e164)) - (_e167 * _e168)) / ((2f * _e172) * _e174));
    }
    let _e178 = local_3;
    (*mu_10) = _e178;
    let _e179 = (*mu_10);
    let _e180 = ClampCosine(_e179);
    (*mu_10) = _e180;
    return;
}

fn ComputeTransmittanceToTopAtmosphereBoundaryTexture(atmosphere_14: AtmosphereParameters, frag_coord: vec2<f32>) -> vec3<f32> {
    var TRANSMITTANCE_TEXTURE_SIZE: vec2<f32> = vec2<f32>(256f, 64f);
    var r_14: f32;
    var mu_11: f32;

    let _e114 = TRANSMITTANCE_TEXTURE_SIZE;
    GetRMuFromTransmittanceTextureUv(atmosphere_14, (frag_coord / _e114), (&r_14), (&mu_11));
    let _e120 = r_14;
    let _e121 = mu_11;
    let _e122 = ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_14, _e120, _e121);
    return _e122;
}

fn ComputeSingleScatteringIntegrand(atmosphere_15: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, r_15: f32, mu_12: f32, mu_s_4: f32, nu_3: f32, d_7: f32, ray_r_mu_intersects_ground_2: bool, rayleigh: ptr<function, vec3<f32>>, mie: ptr<function, vec3<f32>>) {
    var r_d_1: f32;
    var mu_s_d: f32;
    var transmittance: vec3<f32>;

    let _e124 = ClampRadius(atmosphere_15, sqrt((((d_7 * d_7) + (((2f * r_15) * mu_12) * d_7)) + (r_15 * r_15))));
    r_d_1 = _e124;
    let _e129 = r_d_1;
    let _e131 = ClampCosine((((r_15 * mu_s_4) + (d_7 * nu_3)) / _e129));
    mu_s_d = _e131;
    let _e133 = GetTransmittance(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, r_15, mu_12, d_7, ray_r_mu_intersects_ground_2);
    let _e134 = r_d_1;
    let _e135 = mu_s_d;
    let _e136 = GetTransmittanceToSun(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, _e134, _e135);
    transmittance = (_e133 * _e136);
    let _e139 = transmittance;
    let _e141 = r_d_1;
    let _e144 = GetProfileDensity(atmosphere_15.rayleigh_density, (_e141 - atmosphere_15.bottom_radius));
    (*rayleigh) = (_e139 * _e144);
    let _e146 = transmittance;
    let _e148 = r_d_1;
    let _e151 = GetProfileDensity(atmosphere_15.mie_density, (_e148 - atmosphere_15.bottom_radius));
    (*mie) = (_e146 * _e151);
    return;
}

fn DistanceToNearestAtmosphereBoundary(atmosphere_16: AtmosphereParameters, r_16: f32, mu_13: f32, ray_r_mu_intersects_ground_3: bool) -> f32 {
    var r_17: f32;
    var mu_14: f32;
    var ray_r_mu_intersects_ground_4: bool;

    r_17 = r_16;
    mu_14 = mu_13;
    ray_r_mu_intersects_ground_4 = ray_r_mu_intersects_ground_3;
    let _e111 = ray_r_mu_intersects_ground_4;
    if _e111 {
        {
            let _e112 = r_17;
            let _e113 = mu_14;
            let _e114 = DistanceToBottomAtmosphereBoundary(atmosphere_16, _e112, _e113);
            return _e114;
        }
    } else {
        {
            let _e115 = r_17;
            let _e116 = mu_14;
            let _e117 = DistanceToTopAtmosphereBoundary(atmosphere_16, _e115, _e116);
            return _e117;
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

    let _e116 = DistanceToNearestAtmosphereBoundary(atmosphere_17, r_18, mu_15, ray_r_mu_intersects_ground_5);
    let _e117 = SAMPLE_COUNT_1;
    dx_1 = (_e116 / f32(_e117));
    loop {
        let _e129 = i_5;
        let _e130 = SAMPLE_COUNT_1;
        if !((_e129 <= _e130)) {
            break;
        }
        {
            let _e136 = i_5;
            let _e138 = dx_1;
            d_i_1 = (f32(_e136) * _e138);
            let _e143 = d_i_1;
            ComputeSingleScatteringIntegrand(atmosphere_17, transmittance_texture_t_4, transmittance_texture_s_4, r_18, mu_15, mu_s_5, nu_4, _e143, ray_r_mu_intersects_ground_5, (&rayleigh_i), (&mie_i));
            let _e148 = i_5;
            let _e151 = i_5;
            let _e152 = SAMPLE_COUNT_1;
            if ((_e148 == 0i) || (_e151 == _e152)) {
                local_4 = 0.5f;
            } else {
                local_4 = 1f;
            }
            let _e158 = local_4;
            weight_i_1 = _e158;
            let _e160 = rayleigh_sum;
            let _e161 = rayleigh_i;
            let _e162 = weight_i_1;
            rayleigh_sum = (_e160 + (_e161 * _e162));
            let _e165 = mie_sum;
            let _e166 = mie_i;
            let _e167 = weight_i_1;
            mie_sum = (_e165 + (_e166 * _e167));
        }
        continuing {
            let _e133 = i_5;
            i_5 = (_e133 + 1i);
        }
    }
    let _e170 = rayleigh_sum;
    let _e171 = dx_1;
    (*rayleigh_1) = (((_e170 * _e171) * atmosphere_17.solar_irradiance) * atmosphere_17.rayleigh_scattering);
    let _e177 = mie_sum;
    let _e178 = dx_1;
    (*mie_1) = (((_e177 * _e178) * atmosphere_17.solar_irradiance) * atmosphere_17.mie_scattering);
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
    let _e120 = H_3;
    let _e123 = GetUnitRangeFromTextureCoord(uvwz.w, 32i);
    rho_3 = (_e120 * _e123);
    let _e126 = rho_3;
    let _e127 = rho_3;
    (*r_19) = sqrt(((_e126 * _e127) + (atmosphere_18.bottom_radius * atmosphere_18.bottom_radius)));
    if (uvwz.z < 0.5f) {
        {
            let _e137 = (*r_19);
            d_min_5 = (_e137 - atmosphere_18.bottom_radius);
            let _e141 = rho_3;
            d_max_5 = _e141;
            let _e143 = d_min_5;
            let _e144 = d_max_5;
            let _e145 = d_min_5;
            let _e155 = GetUnitRangeFromTextureCoord((1f - (2f * uvwz.z)), 64i);
            d_8 = (_e143 + ((_e144 - _e145) * _e155));
            let _e159 = d_8;
            if (_e159 == 0f) {
                local_5 = -1f;
            } else {
                let _e167 = rho_3;
                let _e168 = rho_3;
                let _e170 = d_8;
                let _e171 = d_8;
                let _e176 = (*r_19);
                let _e178 = d_8;
                let _e181 = ClampCosine((-(((_e167 * _e168) + (_e170 * _e171))) / ((2f * _e176) * _e178)));
                local_5 = _e181;
            }
            let _e183 = local_5;
            (*mu_16) = _e183;
            (*ray_r_mu_intersects_ground_6) = true;
        }
    } else {
        {
            let _e186 = (*r_19);
            d_min_6 = (atmosphere_18.top_radius - _e186);
            let _e189 = rho_3;
            let _e190 = H_3;
            d_max_6 = (_e189 + _e190);
            let _e193 = d_min_6;
            let _e194 = d_max_6;
            let _e195 = d_min_6;
            let _e205 = GetUnitRangeFromTextureCoord(((2f * uvwz.z) - 1f), 64i);
            d_9 = (_e193 + ((_e194 - _e195) * _e205));
            let _e209 = d_9;
            if (_e209 == 0f) {
                local_6 = 1f;
            } else {
                let _e216 = H_3;
                let _e217 = H_3;
                let _e219 = rho_3;
                let _e220 = rho_3;
                let _e223 = d_9;
                let _e224 = d_9;
                let _e228 = (*r_19);
                let _e230 = d_9;
                let _e233 = ClampCosine(((((_e216 * _e217) - (_e219 * _e220)) - (_e223 * _e224)) / ((2f * _e228) * _e230)));
                local_6 = _e233;
            }
            let _e235 = local_6;
            (*mu_16) = _e235;
            (*ray_r_mu_intersects_ground_6) = false;
        }
    }
    let _e239 = GetUnitRangeFromTextureCoord(uvwz.y, 32i);
    x_mu_s_1 = _e239;
    d_min_7 = (atmosphere_18.top_radius - atmosphere_18.bottom_radius);
    let _e245 = H_3;
    d_max_7 = _e245;
    let _e249 = DistanceToTopAtmosphereBoundary(atmosphere_18, atmosphere_18.bottom_radius, atmosphere_18.mu_s_min);
    D_1 = _e249;
    let _e251 = D_1;
    let _e252 = d_min_7;
    let _e254 = d_max_7;
    let _e255 = d_min_7;
    A_1 = ((_e251 - _e252) / (_e254 - _e255));
    let _e259 = A_1;
    let _e260 = x_mu_s_1;
    let _e261 = A_1;
    let _e265 = x_mu_s_1;
    let _e266 = A_1;
    a_2 = ((_e259 - (_e260 * _e261)) / (1f + (_e265 * _e266)));
    let _e271 = d_min_7;
    let _e272 = a_2;
    let _e273 = A_1;
    let _e275 = d_max_7;
    let _e276 = d_min_7;
    d_10 = (_e271 + (min(_e272, _e273) * (_e275 - _e276)));
    let _e281 = d_10;
    if (_e281 == 0f) {
        local_7 = 1f;
    } else {
        let _e288 = H_3;
        let _e289 = H_3;
        let _e291 = d_10;
        let _e292 = d_10;
        let _e298 = d_10;
        let _e301 = ClampCosine((((_e288 * _e289) - (_e291 * _e292)) / ((2f * atmosphere_18.bottom_radius) * _e298)));
        local_7 = _e301;
    }
    let _e303 = local_7;
    (*mu_s_6) = _e303;
    let _e309 = ClampCosine(((uvwz.x * 2f) - 1f));
    (*nu_5) = _e309;
    return;
}

fn GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_19: AtmosphereParameters, frag_coord_1: vec3<f32>, r_20: ptr<function, f32>, mu_17: ptr<function, f32>, mu_s_7: ptr<function, f32>, nu_6: ptr<function, f32>, ray_r_mu_intersects_ground_7: ptr<function, bool>) {
    var SCATTERING_TEXTURE_SIZE: vec4<f32> = vec4<f32>(7f, 32f, 128f, 32f);
    var frag_coord_nu: f32;
    var frag_coord_mu_s: f32;
    var uvwz_1: vec4<f32>;

    frag_coord_nu = floor((frag_coord_1.x / 32f));
    frag_coord_mu_s = (frag_coord_1.x - (floor((frag_coord_1.x / 32f)) * 32f));
    let _e137 = frag_coord_nu;
    let _e138 = frag_coord_mu_s;
    let _e142 = SCATTERING_TEXTURE_SIZE;
    uvwz_1 = (vec4<f32>(_e137, _e138, frag_coord_1.y, frag_coord_1.z) / _e142);
    let _e145 = uvwz_1;
    GetRMuMuSNuFromScatteringTextureUvwz(atmosphere_19, _e145, r_20, mu_17, mu_s_7, nu_6, ray_r_mu_intersects_ground_7);
    let _e156 = (*nu_6);
    let _e157 = (*mu_17);
    let _e158 = (*mu_s_7);
    let _e161 = (*mu_17);
    let _e162 = (*mu_17);
    let _e166 = (*mu_s_7);
    let _e167 = (*mu_s_7);
    let _e173 = (*mu_17);
    let _e174 = (*mu_s_7);
    let _e177 = (*mu_17);
    let _e178 = (*mu_17);
    let _e182 = (*mu_s_7);
    let _e183 = (*mu_s_7);
    (*nu_6) = clamp(_e156, ((_e157 * _e158) - sqrt(((1f - (_e161 * _e162)) * (1f - (_e166 * _e167))))), ((_e173 * _e174) + sqrt(((1f - (_e177 * _e178)) * (1f - (_e182 * _e183))))));
    return;
}

fn ComputeSingleScatteringTexture(atmosphere_20: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, frag_coord_2: vec3<f32>, rayleigh_2: ptr<function, vec3<f32>>, mie_2: ptr<function, vec3<f32>>) {
    var r_21: f32;
    var mu_18: f32;
    var mu_s_8: f32;
    var nu_7: f32;
    var ray_r_mu_intersects_ground_8: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_20, frag_coord_2, (&r_21), (&mu_18), (&mu_s_8), (&nu_7), (&ray_r_mu_intersects_ground_8));
    let _e125 = r_21;
    let _e126 = mu_18;
    let _e127 = mu_s_8;
    let _e128 = nu_7;
    let _e129 = ray_r_mu_intersects_ground_8;
    ComputeSingleScattering(atmosphere_20, transmittance_texture_t_5, transmittance_texture_s_5, _e125, _e126, _e127, _e128, _e129, rayleigh_2, mie_2);
    return;
}

fn GetScattering(atmosphere_21: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, r_22: f32, mu_19: f32, mu_s_9: f32, nu_8: f32, ray_r_mu_intersects_ground_9: bool) -> vec3<f32> {
    var uvwz_2: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;

    let _e112 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_21, r_22, mu_19, mu_s_9, nu_8, ray_r_mu_intersects_ground_9);
    uvwz_2 = _e112;
    let _e114 = uvwz_2;
    tex_coord_x = (_e114.x * 7f);
    let _e122 = tex_coord_x;
    tex_x = floor(_e122);
    let _e125 = tex_coord_x;
    let _e126 = tex_x;
    lerp = (_e125 - _e126);
    let _e129 = tex_x;
    let _e130 = uvwz_2;
    let _e136 = uvwz_2;
    let _e138 = uvwz_2;
    uvw0_ = vec3<f32>(((_e129 + _e130.y) / 8f), _e136.z, _e138.w);
    let _e142 = tex_x;
    let _e145 = uvwz_2;
    let _e151 = uvwz_2;
    let _e153 = uvwz_2;
    uvw1_ = vec3<f32>((((_e142 + 1f) + _e145.y) / 8f), _e151.z, _e153.w);
    let _e157 = uvw0_;
    let _e158 = textureSample(scattering_texture_t, scattering_texture_s, _e157);
    let _e160 = lerp;
    let _e163 = uvw1_;
    let _e164 = textureSample(scattering_texture_t, scattering_texture_s, _e163);
    let _e165 = lerp;
    return vec3<f32>(((_e158 * (1f - _e160)) + (_e164 * _e165)).xyz);
}

fn GetScattering_1(atmosphere_22: AtmosphereParameters, single_rayleigh_scattering_texture_t: texture_3d<f32>, single_rayleigh_scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, multiple_scattering_texture_t: texture_3d<f32>, multiple_scattering_texture_s: sampler, r_23: f32, mu_20: f32, mu_s_10: f32, nu_9: f32, ray_r_mu_intersects_ground_10: bool, scattering_order: i32) -> vec3<f32> {
    var rayleigh_3: vec3<f32>;
    var mie_3: vec3<f32>;

    if (scattering_order == 1i) {
        {
            let _e119 = GetScattering(atmosphere_22, single_rayleigh_scattering_texture_t, single_rayleigh_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            rayleigh_3 = _e119;
            let _e121 = GetScattering(atmosphere_22, single_mie_scattering_texture_t, single_mie_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            mie_3 = _e121;
            let _e123 = rayleigh_3;
            let _e124 = RayleighPhaseFunction(nu_9);
            let _e126 = mie_3;
            let _e128 = MiePhaseFunction(atmosphere_22.mie_phase_function_g, nu_9);
            return ((_e123 * _e124) + (_e126 * _e128));
        }
    } else {
        {
            let _e131 = GetScattering(atmosphere_22, multiple_scattering_texture_t, multiple_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            return _e131;
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
    let _e132 = omega;
    if (_e132.x == 0f) {
        local_8 = 0f;
    } else {
        let _e139 = omega;
        local_8 = ((nu_10 - (mu_21 * mu_s_11)) / _e139.x);
    }
    let _e143 = local_8;
    sun_dir_x = _e143;
    let _e146 = sun_dir_x;
    let _e147 = sun_dir_x;
    sun_dir_y = sqrt(max(((1f - (_e146 * _e147)) - (mu_s_11 * mu_s_11)), 0f));
    let _e156 = sun_dir_x;
    let _e157 = sun_dir_y;
    omega_s = vec3<f32>(_e156, _e157, mu_s_11);
    let _e162 = SAMPLE_COUNT_2;
    dphi = (pi / f32(_e162));
    let _e166 = SAMPLE_COUNT_2;
    dtheta = (pi / f32(_e166));
    loop {
        let _e177 = l;
        let _e178 = SAMPLE_COUNT_2;
        if !((_e177 < _e178)) {
            break;
        }
        {
            let _e184 = l;
            let _e188 = dtheta;
            theta = ((f32(_e184) + 0.5f) * _e188);
            let _e191 = theta;
            cos_theta = cos(_e191);
            let _e194 = theta;
            sin_theta = sin(_e194);
            let _e197 = cos_theta;
            let _e198 = RayIntersectsGround(atmosphere_23, r_24, _e197);
            ray_r_theta_intersects_ground = _e198;
            distance_to_ground = 0f;
            transmittance_to_ground = vec3(0f);
            ground_albedo = vec3(0f);
            let _e210 = ray_r_theta_intersects_ground;
            if _e210 {
                {
                    let _e211 = cos_theta;
                    let _e212 = DistanceToBottomAtmosphereBoundary(atmosphere_23, r_24, _e211);
                    distance_to_ground = _e212;
                    let _e213 = cos_theta;
                    let _e214 = distance_to_ground;
                    let _e216 = GetTransmittance(atmosphere_23, transmittance_texture_t_6, transmittance_texture_s_6, r_24, _e213, _e214, true);
                    transmittance_to_ground = _e216;
                    ground_albedo = atmosphere_23.ground_albedo;
                }
            }
            m = 0i;
            loop {
                let _e220 = m;
                let _e222 = SAMPLE_COUNT_2;
                if !((_e220 < (2i * _e222))) {
                    break;
                }
                {
                    let _e229 = m;
                    let _e233 = dphi;
                    phi = ((f32(_e229) + 0.5f) * _e233);
                    let _e236 = phi;
                    let _e238 = sin_theta;
                    let _e240 = phi;
                    let _e242 = sin_theta;
                    let _e244 = cos_theta;
                    omega_i = vec3<f32>((cos(_e236) * _e238), (sin(_e240) * _e242), _e244);
                    let _e247 = dtheta;
                    let _e249 = dphi;
                    let _e252 = theta;
                    domega_i = ((((_e247 / rad) * (_e249 / rad)) * sin(_e252)) * sr);
                    let _e257 = omega_s;
                    let _e258 = omega_i;
                    nu1_ = dot(_e257, _e258);
                    let _e261 = omega_i;
                    let _e263 = nu1_;
                    let _e264 = ray_r_theta_intersects_ground;
                    let _e267 = GetScattering_1(atmosphere_23, single_rayleigh_scattering_texture_t_1, single_rayleigh_scattering_texture_s_1, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, multiple_scattering_texture_t_1, multiple_scattering_texture_s_1, r_24, _e261.z, mu_s_11, _e263, _e264, (scattering_order_1 - 1i));
                    incident_radiance = _e267;
                    let _e269 = zenith_direction;
                    let _e271 = omega_i;
                    let _e272 = distance_to_ground;
                    ground_normal = normalize(((_e269 * r_24) + (_e271 * _e272)));
                    let _e278 = ground_normal;
                    let _e279 = omega_s;
                    let _e281 = GetIrradiance(atmosphere_23, irradiance_texture_t_1, irradiance_texture_s_1, atmosphere_23.bottom_radius, dot(_e278, _e279));
                    ground_irradiance = _e281;
                    let _e283 = incident_radiance;
                    let _e284 = transmittance_to_ground;
                    let _e285 = ground_albedo;
                    let _e293 = ground_irradiance;
                    incident_radiance = (_e283 + (((_e284 * _e285) * 0.31830987f) * _e293));
                    let _e296 = omega;
                    let _e297 = omega_i;
                    nu2_ = dot(_e296, _e297);
                    let _e303 = GetProfileDensity(atmosphere_23.rayleigh_density, (r_24 - atmosphere_23.bottom_radius));
                    rayleigh_density = _e303;
                    let _e308 = GetProfileDensity(atmosphere_23.mie_density, (r_24 - atmosphere_23.bottom_radius));
                    mie_density = _e308;
                    let _e310 = rayleigh_mie;
                    let _e311 = incident_radiance;
                    let _e313 = rayleigh_density;
                    let _e315 = nu2_;
                    let _e316 = RayleighPhaseFunction(_e315);
                    let _e319 = mie_density;
                    let _e322 = nu2_;
                    let _e323 = MiePhaseFunction(atmosphere_23.mie_phase_function_g, _e322);
                    let _e327 = domega_i;
                    rayleigh_mie = (_e310 + ((_e311 * (((atmosphere_23.rayleigh_scattering * _e313) * _e316) + ((atmosphere_23.mie_scattering * _e319) * _e323))) * _e327));
                }
                continuing {
                    let _e226 = m;
                    m = (_e226 + 1i);
                }
            }
        }
        continuing {
            let _e181 = l;
            l = (_e181 + 1i);
        }
    }
    let _e330 = rayleigh_mie;
    return _e330;
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

    let _e116 = DistanceToNearestAtmosphereBoundary(atmosphere_24, r_25, mu_22, ray_r_mu_intersects_ground_11);
    let _e117 = SAMPLE_COUNT_3;
    dx_2 = (_e116 / f32(_e117));
    loop {
        let _e128 = i_6;
        let _e129 = SAMPLE_COUNT_3;
        if !((_e128 <= _e129)) {
            break;
        }
        {
            let _e135 = i_6;
            let _e137 = dx_2;
            d_i_2 = (f32(_e135) * _e137);
            let _e140 = d_i_2;
            let _e141 = d_i_2;
            let _e146 = d_i_2;
            let _e152 = ClampRadius(atmosphere_24, sqrt((((_e140 * _e141) + (((2f * r_25) * mu_22) * _e146)) + (r_25 * r_25))));
            r_i_1 = _e152;
            let _e155 = d_i_2;
            let _e157 = r_i_1;
            let _e159 = ClampCosine((((r_25 * mu_22) + _e155) / _e157));
            mu_i = _e159;
            let _e162 = d_i_2;
            let _e165 = r_i_1;
            let _e167 = ClampCosine((((r_25 * mu_s_12) + (_e162 * nu_11)) / _e165));
            mu_s_i = _e167;
            let _e169 = r_i_1;
            let _e170 = mu_i;
            let _e171 = mu_s_i;
            let _e172 = GetScattering(atmosphere_24, scattering_density_texture_t, scattering_density_texture_s, _e169, _e170, _e171, nu_11, ray_r_mu_intersects_ground_11);
            let _e173 = d_i_2;
            let _e174 = GetTransmittance(atmosphere_24, transmittance_texture_t_7, transmittance_texture_s_7, r_25, mu_22, _e173, ray_r_mu_intersects_ground_11);
            let _e176 = dx_2;
            rayleigh_mie_i = ((_e172 * _e174) * _e176);
            let _e179 = i_6;
            let _e182 = i_6;
            let _e183 = SAMPLE_COUNT_3;
            if ((_e179 == 0i) || (_e182 == _e183)) {
                local_9 = 0.5f;
            } else {
                local_9 = 1f;
            }
            let _e189 = local_9;
            weight_i_2 = _e189;
            let _e191 = rayleigh_mie_sum;
            let _e192 = rayleigh_mie_i;
            let _e193 = weight_i_2;
            rayleigh_mie_sum = (_e191 + (_e192 * _e193));
        }
        continuing {
            let _e132 = i_6;
            i_6 = (_e132 + 1i);
        }
    }
    let _e196 = rayleigh_mie_sum;
    return _e196;
}

fn ComputeScatteringDensityTexture(atmosphere_25: AtmosphereParameters, transmittance_texture_t_8: texture_2d<f32>, transmittance_texture_s_8: sampler, single_rayleigh_scattering_texture_t_2: texture_3d<f32>, single_rayleigh_scattering_texture_s_2: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, multiple_scattering_texture_t_2: texture_3d<f32>, multiple_scattering_texture_s_2: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, frag_coord_3: vec3<f32>, scattering_order_2: i32) -> vec3<f32> {
    var r_26: f32;
    var mu_23: f32;
    var mu_s_13: f32;
    var nu_12: f32;
    var ray_r_mu_intersects_ground_12: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_25, frag_coord_3, (&r_26), (&mu_23), (&mu_s_13), (&nu_12), (&ray_r_mu_intersects_ground_12));
    let _e132 = r_26;
    let _e133 = mu_23;
    let _e134 = mu_s_13;
    let _e135 = nu_12;
    let _e136 = ComputeScatteringDensity(atmosphere_25, transmittance_texture_t_8, transmittance_texture_s_8, single_rayleigh_scattering_texture_t_2, single_rayleigh_scattering_texture_s_2, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, multiple_scattering_texture_t_2, multiple_scattering_texture_s_2, irradiance_texture_t_2, irradiance_texture_s_2, _e132, _e133, _e134, _e135, scattering_order_2);
    return _e136;
}

fn ComputeMultipleScatteringTexture(atmosphere_26: AtmosphereParameters, transmittance_texture_t_9: texture_2d<f32>, transmittance_texture_s_9: sampler, scattering_density_texture_t_1: texture_3d<f32>, scattering_density_texture_s_1: sampler, frag_coord_4: vec3<f32>, nu_13: ptr<function, f32>) -> vec3<f32> {
    var r_27: f32;
    var mu_24: f32;
    var mu_s_14: f32;
    var ray_r_mu_intersects_ground_13: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_26, frag_coord_4, (&r_27), (&mu_24), (&mu_s_14), nu_13, (&ray_r_mu_intersects_ground_13));
    let _e125 = r_27;
    let _e126 = mu_24;
    let _e127 = mu_s_14;
    let _e128 = (*nu_13);
    let _e129 = ray_r_mu_intersects_ground_13;
    let _e130 = ComputeMultipleScattering(atmosphere_26, transmittance_texture_t_9, transmittance_texture_s_9, scattering_density_texture_t_1, scattering_density_texture_s_1, _e125, _e126, _e127, _e128, _e129);
    return _e130;
}

fn ComputeDirectIrradiance(atmosphere_27: AtmosphereParameters, transmittance_texture_t_10: texture_2d<f32>, transmittance_texture_s_10: sampler, r_28: f32, mu_s_15: f32) -> vec3<f32> {
    var alpha_s: f32;
    var local_10: f32;
    var local_11: f32;
    var average_cosine_factor: f32;

    alpha_s = (atmosphere_27.sun_angular_radius / rad);
    let _e112 = alpha_s;
    if (mu_s_15 < -(_e112)) {
        local_11 = 0f;
    } else {
        let _e116 = alpha_s;
        if (mu_s_15 > _e116) {
            local_10 = mu_s_15;
        } else {
            let _e118 = alpha_s;
            let _e120 = alpha_s;
            let _e124 = alpha_s;
            local_10 = (((mu_s_15 + _e118) * (mu_s_15 + _e120)) / (4f * _e124));
        }
        let _e128 = local_10;
        local_11 = _e128;
    }
    let _e130 = local_11;
    average_cosine_factor = _e130;
    let _e133 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_27, transmittance_texture_t_10, transmittance_texture_s_10, r_28, mu_s_15);
    let _e135 = average_cosine_factor;
    return ((atmosphere_27.solar_irradiance * _e133) * _e135);
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

    let _e116 = SAMPLE_COUNT_4;
    dphi_1 = (pi / f32(_e116));
    let _e120 = SAMPLE_COUNT_4;
    dtheta_1 = (pi / f32(_e120));
    omega_s_1 = vec3<f32>(sqrt((1f - (mu_s_16 * mu_s_16))), 0f, mu_s_16);
    loop {
        let _e138 = j;
        let _e139 = SAMPLE_COUNT_4;
        if !((_e138 < (_e139 / 2i))) {
            break;
        }
        {
            let _e147 = j;
            let _e151 = dtheta_1;
            theta_1 = ((f32(_e147) + 0.5f) * _e151);
            i_7 = 0i;
            loop {
                let _e156 = i_7;
                let _e158 = SAMPLE_COUNT_4;
                if !((_e156 < (2i * _e158))) {
                    break;
                }
                {
                    let _e165 = i_7;
                    let _e169 = dphi_1;
                    phi_1 = ((f32(_e165) + 0.5f) * _e169);
                    let _e172 = phi_1;
                    let _e174 = theta_1;
                    let _e177 = phi_1;
                    let _e179 = theta_1;
                    let _e182 = theta_1;
                    omega_1 = vec3<f32>((cos(_e172) * sin(_e174)), (sin(_e177) * sin(_e179)), cos(_e182));
                    let _e186 = dtheta_1;
                    let _e188 = dphi_1;
                    let _e191 = theta_1;
                    domega = ((((_e186 / rad) * (_e188 / rad)) * sin(_e191)) * sr);
                    let _e196 = omega_1;
                    let _e197 = omega_s_1;
                    nu_14 = dot(_e196, _e197);
                    let _e200 = result_1;
                    let _e201 = omega_1;
                    let _e203 = nu_14;
                    let _e205 = GetScattering_1(atmosphere_28, single_rayleigh_scattering_texture_t_3, single_rayleigh_scattering_texture_s_3, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, multiple_scattering_texture_t_3, multiple_scattering_texture_s_3, r_29, _e201.z, mu_s_16, _e203, false, scattering_order_3);
                    let _e206 = omega_1;
                    let _e209 = domega;
                    result_1 = (_e200 + ((_e205 * _e206.z) * _e209));
                }
                continuing {
                    let _e162 = i_7;
                    i_7 = (_e162 + 1i);
                }
            }
        }
        continuing {
            let _e144 = j;
            j = (_e144 + 1i);
        }
    }
    let _e212 = result_1;
    return _e212;
}

fn GetRMuSFromIrradianceTextureUv(atmosphere_29: AtmosphereParameters, uv_3: vec2<f32>, r_30: ptr<function, f32>, mu_s_17: ptr<function, f32>) {
    var x_mu_s_2: f32;
    var x_r_3: f32;

    let _e110 = GetUnitRangeFromTextureCoord(uv_3.x, 64i);
    x_mu_s_2 = _e110;
    let _e114 = GetUnitRangeFromTextureCoord(uv_3.y, 16i);
    x_r_3 = _e114;
    let _e117 = x_r_3;
    (*r_30) = (atmosphere_29.bottom_radius + (_e117 * (atmosphere_29.top_radius - atmosphere_29.bottom_radius)));
    let _e124 = x_mu_s_2;
    let _e128 = ClampCosine(((2f * _e124) - 1f));
    (*mu_s_17) = _e128;
    return;
}

fn ComputeDirectIrradianceTexture(atmosphere_30: AtmosphereParameters, transmittance_texture_t_11: texture_2d<f32>, transmittance_texture_s_11: sampler, frag_coord_5: vec2<f32>) -> vec3<f32> {
    var r_31: f32;
    var mu_s_18: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_30, (frag_coord_5 / IRRADIANCE_TEXTURE_SIZE), (&r_31), (&mu_s_18));
    let _e116 = r_31;
    let _e117 = mu_s_18;
    let _e118 = ComputeDirectIrradiance(atmosphere_30, transmittance_texture_t_11, transmittance_texture_s_11, _e116, _e117);
    return _e118;
}

fn ComputeIndirectIrradianceTexture(atmosphere_31: AtmosphereParameters, single_rayleigh_scattering_texture_t_4: texture_3d<f32>, single_rayleigh_scattering_texture_s_4: sampler, single_mie_scattering_texture_t_4: texture_3d<f32>, single_mie_scattering_texture_s_4: sampler, multiple_scattering_texture_t_4: texture_3d<f32>, multiple_scattering_texture_s_4: sampler, frag_coord_6: vec2<f32>, scattering_order_4: i32) -> vec3<f32> {
    var r_32: f32;
    var mu_s_19: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_31, (frag_coord_6 / IRRADIANCE_TEXTURE_SIZE), (&r_32), (&mu_s_19));
    let _e121 = r_32;
    let _e122 = mu_s_19;
    let _e123 = ComputeIndirectIrradiance(atmosphere_31, single_rayleigh_scattering_texture_t_4, single_rayleigh_scattering_texture_s_4, single_mie_scattering_texture_t_4, single_mie_scattering_texture_s_4, multiple_scattering_texture_t_4, multiple_scattering_texture_s_4, _e121, _e122, scattering_order_4);
    return _e123;
}

fn pt_main() {
    let _e107 = transmittance_1;
    let _e109 = ATMOSPHERE;
    let _e110 = gl_FragCoord_1;
    let _e112 = ComputeTransmittanceToTopAtmosphereBoundaryTexture(_e109, _e110.xy);
    transmittance_1.x = _e112.x;
    transmittance_1.y = _e112.y;
    transmittance_1.z = _e112.z;
    transmittance_1.w = 1f;
    return;
}

fn main_1() {
    let _e108 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e108;
    let _e110 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e110;
    let _e112 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e112;
    let _e114 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e114;
    let _e120 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e120;
    let _e126 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e126;
    let _e132 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e132;
    let _e138 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e138;
    let _e144 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e144;
    let _e150 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e150;
    let _e156 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e156;
    let _e162 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e162;
    let _e168 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e168;
    let _e174 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e174;
    let _e176 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e176;
    let _e182 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e182;
    let _e188 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e188;
    let _e194 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e194;
    let _e200 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e200;
    let _e206 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e206;
    let _e212 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e212;
    let _e218 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e218;
    let _e224 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e224;
    let _e230 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e230;
    let _e236 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e236;
    let _e238 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e238;
    let _e240 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e240;
    let _e242 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e242;
    let _e248 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e248;
    let _e254 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e254;
    let _e260 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e260;
    let _e266 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e266;
    let _e272 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e272;
    let _e278 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e278;
    let _e284 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e284;
    let _e290 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e290;
    let _e296 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e296;
    let _e302 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e302;
    let _e304 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e304;
    let _e306 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e306;
    let _e308 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e308;
    pt_main();
    return;
}

@fragment 
fn main(@builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e133 = transmittance_1;
    return FragmentOutput(_e133);
}
