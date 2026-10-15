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
    scatteringOrder: i32,
    layer: i32,
}

struct FragmentOutput {
    @location(0) scatteringDensity: vec4<f32>,
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
@group(0) @binding(3) 
var singleRayleighScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(4) 
var singleRayleighScatteringTexture_s: sampler;
@group(0) @binding(5) 
var singleMieScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(6) 
var singleMieScatteringTexture_s: sampler;
@group(0) @binding(7) 
var multipleScatteringTexture_t: texture_3d<f32>;
@group(0) @binding(8) 
var multipleScatteringTexture_s: sampler;
@group(0) @binding(9) 
var irradianceTexture_t: texture_2d<f32>;
@group(0) @binding(10) 
var irradianceTexture_s: sampler;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> scatteringDensity: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e101 = y_1;
    let _e102 = x_1;
    return exp2((_e101 * log2(_e102)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e101 = y_3;
    let _e102 = x_3;
    return exp2((_e101 * log2(_e102)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e101 = y_5;
    let _e102 = x_5;
    return exp2((_e101 * log2(_e102)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e101 = y_7;
    let _e102 = x_7;
    return exp2((_e101 * log2(_e102)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e100 = x_9;
    (*i) = trunc(_e100);
    let _e102 = x_9;
    let _e103 = (*i);
    return (_e102 - _e103);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e100 = x_11;
    (*i_1) = trunc(_e100);
    let _e102 = x_11;
    let _e103 = (*i_1);
    return (_e102 - _e103);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e100 = x_13;
    (*i_2) = trunc(_e100);
    let _e102 = x_13;
    let _e103 = (*i_2);
    return (_e102 - _e103);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e100 = x_15;
    (*i_3) = trunc(_e100);
    let _e102 = x_15;
    let _e103 = (*i_3);
    return (_e102 - _e103);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e99 = v_1;
    b = (bitcast<u32>(_e99) & 2147483647u);
    let _e104 = b;
    return (_e104 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e99 = v_3;
    b_1 = (bitcast<u32>(_e99) & 2147483647u);
    let _e104 = b_1;
    return (_e104 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e99 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e99) & vec2(2147483647u));
    let _e105 = b_2;
    return (_e105 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e99 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e99) & vec3(2147483647u));
    let _e105 = b_3;
    return (_e105 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e99 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e99) & vec4(2147483647u));
    let _e105 = b_4;
    return (_e105 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e99 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e99) & vec2(2147483647u));
    let _e105 = b_5;
    return (_e105 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e99 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e99) & vec3(2147483647u));
    let _e105 = b_6;
    return (_e105 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e99 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e99) & vec4(2147483647u));
    let _e105 = b_7;
    return (_e105 == vec4(2139095040u));
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
    let _e133 = discriminant;
    let _e134 = SafeSqrt(_e133);
    let _e136 = ClampDistance(((-(r_1) * mu_1) + _e134));
    return _e136;
}

fn DistanceToBottomAtmosphereBoundary(atmosphere_2: AtmosphereParameters, r_2: f32, mu_2: f32) -> f32 {
    var discriminant_1: f32;

    discriminant_1 = (((r_2 * r_2) * ((mu_2 * mu_2) - 1f)) + (atmosphere_2.bottom_radius * atmosphere_2.bottom_radius));
    let _e133 = discriminant_1;
    let _e134 = SafeSqrt(_e133);
    let _e136 = ClampDistance(((-(r_2) * mu_2) - _e134));
    return _e136;
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
    let _e135 = SafeSqrt(((r_4 * r_4) - (atmosphere_4.bottom_radius * atmosphere_4.bottom_radius)));
    rho = _e135;
    let _e137 = DistanceToTopAtmosphereBoundary(atmosphere_4, r_4, mu_4);
    d_1 = _e137;
    d_min = (atmosphere_4.top_radius - r_4);
    let _e142 = rho;
    let _e143 = H;
    d_max = (_e142 + _e143);
    let _e146 = d_1;
    let _e147 = d_min;
    let _e149 = d_max;
    let _e150 = d_min;
    x_mu = ((_e146 - _e147) / (_e149 - _e150));
    let _e154 = rho;
    let _e155 = H;
    x_r = (_e154 / _e155);
    let _e158 = x_mu;
    let _e160 = GetTextureCoordFromUnitRange(_e158, 256i);
    let _e161 = x_r;
    let _e163 = GetTextureCoordFromUnitRange(_e161, 64i);
    return vec2<f32>(_e160, _e163);
}

fn GetTransmittanceToTopAtmosphereBoundary(atmosphere_5: AtmosphereParameters, transmittance_texture_t: texture_2d<f32>, transmittance_texture_s: sampler, r_5: f32, mu_5: f32) -> vec3<f32> {
    var uv: vec2<f32>;

    let _e123 = GetTransmittanceTextureUvFromRMu(atmosphere_5, r_5, mu_5);
    uv = _e123;
    let _e125 = uv;
    let _e126 = textureSample(transmittance_texture_t, transmittance_texture_s, _e125);
    return vec3<f32>(_e126.xyz);
}

fn GetTransmittance(atmosphere_6: AtmosphereParameters, transmittance_texture_t_1: texture_2d<f32>, transmittance_texture_s_1: sampler, r_6: f32, mu_6: f32, d_2: f32, ray_r_mu_intersects_ground: bool) -> vec3<f32> {
    var r_d: f32;
    var mu_d: f32;

    let _e134 = ClampRadius(atmosphere_6, sqrt((((d_2 * d_2) + (((2f * r_6) * mu_6) * d_2)) + (r_6 * r_6))));
    r_d = _e134;
    let _e138 = r_d;
    let _e140 = ClampCosine((((r_6 * mu_6) + d_2) / _e138));
    mu_d = _e140;
    if ray_r_mu_intersects_ground {
        {
            let _e142 = r_d;
            let _e143 = mu_d;
            let _e145 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e142, -(_e143));
            let _e147 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, -(mu_6));
            return min((_e145 / _e147), vec3(1f));
        }
    } else {
        {
            let _e152 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, r_6, mu_6);
            let _e153 = r_d;
            let _e154 = mu_d;
            let _e155 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_6, transmittance_texture_t_1, transmittance_texture_s_1, _e153, _e154);
            return min((_e152 / _e155), vec3(1f));
        }
    }
}

fn GetTransmittanceToSun(atmosphere_7: AtmosphereParameters, transmittance_texture_t_2: texture_2d<f32>, transmittance_texture_s_2: sampler, r_7: f32, mu_s: f32) -> vec3<f32> {
    var sin_theta_h: f32;
    var cos_theta_h: f32;

    sin_theta_h = (atmosphere_7.bottom_radius / r_7);
    let _e127 = sin_theta_h;
    let _e128 = sin_theta_h;
    cos_theta_h = -(sqrt(max((1f - (_e127 * _e128)), 0f)));
    let _e136 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_7, transmittance_texture_t_2, transmittance_texture_s_2, r_7, mu_s);
    let _e137 = sin_theta_h;
    let _e142 = sin_theta_h;
    let _e146 = cos_theta_h;
    return (_e136 * smoothstep(((-(_e137) * atmosphere_7.sun_angular_radius) / rad), ((_e142 * atmosphere_7.sun_angular_radius) / rad), (mu_s - _e146)));
}

fn RayleighPhaseFunction(nu: f32) -> f32 {
    var k: f32 = 0.059683103f;

    let _e127 = k;
    return (_e127 * (1f + (nu * nu)));
}

fn MiePhaseFunction(g: f32, nu_1: f32) -> f32 {
    var k_1: f32;

    k_1 = ((0.119366206f * (1f - (g * g))) / (2f + (g * g)));
    let _e136 = k_1;
    let _e149 = pt_pow(((1f + (g * g)) - ((2f * g) * nu_1)), 1.5f);
    return ((_e136 * (1f + (nu_1 * nu_1))) / _e149);
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
    let _e138 = SafeSqrt(((r_8 * r_8) - (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius)));
    rho_1 = _e138;
    let _e140 = rho_1;
    let _e141 = H_1;
    let _e144 = GetTextureCoordFromUnitRange((_e140 / _e141), 32i);
    u_r = _e144;
    r_mu = (r_8 * mu_7);
    let _e148 = r_mu;
    let _e149 = r_mu;
    discriminant_2 = (((_e148 * _e149) - (r_8 * r_8)) + (atmosphere_8.bottom_radius * atmosphere_8.bottom_radius));
    if ray_r_mu_intersects_ground_1 {
        {
            let _e159 = r_mu;
            let _e161 = discriminant_2;
            let _e162 = SafeSqrt(_e161);
            d_3 = (-(_e159) - _e162);
            d_min_1 = (r_8 - atmosphere_8.bottom_radius);
            let _e168 = rho_1;
            d_max_1 = _e168;
            let _e172 = d_max_1;
            let _e173 = d_min_1;
            if (_e172 == _e173) {
                local = 0f;
            } else {
                let _e176 = d_3;
                let _e177 = d_min_1;
                let _e179 = d_max_1;
                let _e180 = d_min_1;
                local = ((_e176 - _e177) / (_e179 - _e180));
            }
            let _e184 = local;
            let _e188 = GetTextureCoordFromUnitRange(_e184, 64i);
            u_mu = (0.5f - (0.5f * _e188));
        }
    } else {
        {
            let _e191 = r_mu;
            let _e193 = discriminant_2;
            let _e194 = H_1;
            let _e195 = H_1;
            let _e198 = SafeSqrt((_e193 + (_e194 * _e195)));
            d_4 = (-(_e191) + _e198);
            d_min_2 = (atmosphere_8.top_radius - r_8);
            let _e204 = rho_1;
            let _e205 = H_1;
            d_max_2 = (_e204 + _e205);
            let _e210 = d_4;
            let _e211 = d_min_2;
            let _e213 = d_max_2;
            let _e214 = d_min_2;
            let _e220 = GetTextureCoordFromUnitRange(((_e210 - _e211) / (_e213 - _e214)), 64i);
            u_mu = (0.5f + (0.5f * _e220));
        }
    }
    let _e224 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, mu_s_1);
    d_5 = _e224;
    d_min_3 = (atmosphere_8.top_radius - atmosphere_8.bottom_radius);
    let _e230 = H_1;
    d_max_3 = _e230;
    let _e232 = d_5;
    let _e233 = d_min_3;
    let _e235 = d_max_3;
    let _e236 = d_min_3;
    a_1 = ((_e232 - _e233) / (_e235 - _e236));
    let _e242 = DistanceToTopAtmosphereBoundary(atmosphere_8, atmosphere_8.bottom_radius, atmosphere_8.mu_s_min);
    D = _e242;
    let _e244 = D;
    let _e245 = d_min_3;
    let _e247 = d_max_3;
    let _e248 = d_min_3;
    A = ((_e244 - _e245) / (_e247 - _e248));
    let _e253 = a_1;
    let _e254 = A;
    let _e260 = a_1;
    let _e264 = GetTextureCoordFromUnitRange((max((1f - (_e253 / _e254)), 0f) / (1f + _e260)), 32i);
    u_mu_s = _e264;
    u_nu = ((nu_2 + 1f) / 2f);
    let _e271 = u_nu;
    let _e272 = u_mu_s;
    let _e273 = u_mu;
    let _e274 = u_r;
    return vec4<f32>(_e271, _e272, _e273, _e274);
}

fn GetIrradianceTextureUvFromRMuS(atmosphere_9: AtmosphereParameters, r_9: f32, mu_s_2: f32) -> vec2<f32> {
    var x_r_1: f32;
    var x_mu_s: f32;

    x_r_1 = ((r_9 - atmosphere_9.bottom_radius) / (atmosphere_9.top_radius - atmosphere_9.bottom_radius));
    x_mu_s = ((mu_s_2 * 0.5f) + 0.5f);
    let _e133 = x_mu_s;
    let _e135 = GetTextureCoordFromUnitRange(_e133, 64i);
    let _e136 = x_r_1;
    let _e138 = GetTextureCoordFromUnitRange(_e136, 16i);
    return vec2<f32>(_e135, _e138);
}

fn GetIrradiance(atmosphere_10: AtmosphereParameters, irradiance_texture_t: texture_2d<f32>, irradiance_texture_s: sampler, r_10: f32, mu_s_3: f32) -> vec3<f32> {
    var uv_1: vec2<f32>;

    let _e123 = GetIrradianceTextureUvFromRMuS(atmosphere_10, r_10, mu_s_3);
    uv_1 = _e123;
    let _e125 = uv_1;
    let _e126 = textureSample(irradiance_texture_t, irradiance_texture_s, _e125);
    return vec3<f32>(_e126.xyz);
}

fn GetLayerDensity(layer: DensityProfileLayer, altitude: f32) -> f32 {
    var density: f32;

    density = (((layer.exp_term * exp((layer.exp_scale * altitude))) + (layer.linear_term * altitude)) + layer.constant_term);
    let _e131 = density;
    return clamp(_e131, 0f, 1f);
}

fn GetProfileDensity(profile: DensityProfile, altitude_1: f32) -> f32 {
    var layers: array<DensityProfileLayer, 2>;
    var local_1: f32;

    layers[0i] = profile.layers[0];
    layers[1i] = profile.layers[1];
    let _e133 = layers[0];
    if (altitude_1 < _e133.width) {
        let _e138 = layers[0];
        let _e139 = GetLayerDensity(_e138, altitude_1);
        local_1 = _e139;
    } else {
        let _e142 = layers[1];
        let _e143 = GetLayerDensity(_e142, altitude_1);
        local_1 = _e143;
    }
    let _e145 = local_1;
    return _e145;
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

    let _e124 = DistanceToTopAtmosphereBoundary(atmosphere_11, r_11, mu_8);
    let _e125 = SAMPLE_COUNT;
    dx = (_e124 / f32(_e125));
    loop {
        let _e135 = i_4;
        let _e136 = SAMPLE_COUNT;
        if !((_e135 <= _e136)) {
            break;
        }
        {
            let _e142 = i_4;
            let _e144 = dx;
            d_i = (f32(_e142) * _e144);
            let _e147 = d_i;
            let _e148 = d_i;
            let _e153 = d_i;
            r_i = sqrt((((_e147 * _e148) + (((2f * r_11) * mu_8) * _e153)) + (r_11 * r_11)));
            let _e160 = r_i;
            let _e163 = GetProfileDensity(profile_1, (_e160 - atmosphere_11.bottom_radius));
            y_i = _e163;
            let _e165 = i_4;
            let _e168 = i_4;
            let _e169 = SAMPLE_COUNT;
            if ((_e165 == 0i) || (_e168 == _e169)) {
                local_2 = 0.5f;
            } else {
                local_2 = 1f;
            }
            let _e175 = local_2;
            weight_i = _e175;
            let _e177 = result;
            let _e178 = y_i;
            let _e179 = weight_i;
            let _e181 = dx;
            result = (_e177 + ((_e178 * _e179) * _e181));
        }
        continuing {
            let _e139 = i_4;
            i_4 = (_e139 + 1i);
        }
    }
    let _e184 = result;
    return _e184;
}

fn ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_12: AtmosphereParameters, r_12: f32, mu_9: f32) -> vec3<f32> {
    var optical_depth: vec3<f32>;

    let _e123 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.rayleigh_density, r_12, mu_9);
    let _e127 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.mie_density, r_12, mu_9);
    let _e132 = ComputeOpticalLengthToTopAtmosphereBoundary(atmosphere_12, atmosphere_12.absorption_density, r_12, mu_9);
    optical_depth = (((atmosphere_12.rayleigh_scattering * _e123) + (atmosphere_12.mie_extinction * _e127)) + (atmosphere_12.absorption_extinction * _e132));
    let _e136 = optical_depth;
    return exp(-(_e136));
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

    let _e124 = GetUnitRangeFromTextureCoord(uv_2.x, 256i);
    x_mu_1 = _e124;
    let _e128 = GetUnitRangeFromTextureCoord(uv_2.y, 64i);
    x_r_2 = _e128;
    H_2 = sqrt(((atmosphere_13.top_radius * atmosphere_13.top_radius) - (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e139 = H_2;
    let _e140 = x_r_2;
    rho_2 = (_e139 * _e140);
    let _e143 = rho_2;
    let _e144 = rho_2;
    (*r_13) = sqrt(((_e143 * _e144) + (atmosphere_13.bottom_radius * atmosphere_13.bottom_radius)));
    let _e152 = (*r_13);
    d_min_4 = (atmosphere_13.top_radius - _e152);
    let _e155 = rho_2;
    let _e156 = H_2;
    d_max_4 = (_e155 + _e156);
    let _e159 = d_min_4;
    let _e160 = x_mu_1;
    let _e161 = d_max_4;
    let _e162 = d_min_4;
    d_6 = (_e159 + (_e160 * (_e161 - _e162)));
    let _e167 = d_6;
    if (_e167 == 0f) {
        local_3 = 1f;
    } else {
        let _e174 = H_2;
        let _e175 = H_2;
        let _e177 = rho_2;
        let _e178 = rho_2;
        let _e181 = d_6;
        let _e182 = d_6;
        let _e186 = (*r_13);
        let _e188 = d_6;
        local_3 = ((((_e174 * _e175) - (_e177 * _e178)) - (_e181 * _e182)) / ((2f * _e186) * _e188));
    }
    let _e192 = local_3;
    (*mu_10) = _e192;
    let _e193 = (*mu_10);
    let _e194 = ClampCosine(_e193);
    (*mu_10) = _e194;
    return;
}

fn ComputeTransmittanceToTopAtmosphereBoundaryTexture(atmosphere_14: AtmosphereParameters, frag_coord: vec2<f32>) -> vec3<f32> {
    var TRANSMITTANCE_TEXTURE_SIZE: vec2<f32> = vec2<f32>(256f, 64f);
    var r_14: f32;
    var mu_11: f32;

    let _e128 = TRANSMITTANCE_TEXTURE_SIZE;
    GetRMuFromTransmittanceTextureUv(atmosphere_14, (frag_coord / _e128), (&r_14), (&mu_11));
    let _e134 = r_14;
    let _e135 = mu_11;
    let _e136 = ComputeTransmittanceToTopAtmosphereBoundary(atmosphere_14, _e134, _e135);
    return _e136;
}

fn ComputeSingleScatteringIntegrand(atmosphere_15: AtmosphereParameters, transmittance_texture_t_3: texture_2d<f32>, transmittance_texture_s_3: sampler, r_15: f32, mu_12: f32, mu_s_4: f32, nu_3: f32, d_7: f32, ray_r_mu_intersects_ground_2: bool, rayleigh: ptr<function, vec3<f32>>, mie: ptr<function, vec3<f32>>) {
    var r_d_1: f32;
    var mu_s_d: f32;
    var transmittance: vec3<f32>;

    let _e138 = ClampRadius(atmosphere_15, sqrt((((d_7 * d_7) + (((2f * r_15) * mu_12) * d_7)) + (r_15 * r_15))));
    r_d_1 = _e138;
    let _e143 = r_d_1;
    let _e145 = ClampCosine((((r_15 * mu_s_4) + (d_7 * nu_3)) / _e143));
    mu_s_d = _e145;
    let _e147 = GetTransmittance(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, r_15, mu_12, d_7, ray_r_mu_intersects_ground_2);
    let _e148 = r_d_1;
    let _e149 = mu_s_d;
    let _e150 = GetTransmittanceToSun(atmosphere_15, transmittance_texture_t_3, transmittance_texture_s_3, _e148, _e149);
    transmittance = (_e147 * _e150);
    let _e153 = transmittance;
    let _e155 = r_d_1;
    let _e158 = GetProfileDensity(atmosphere_15.rayleigh_density, (_e155 - atmosphere_15.bottom_radius));
    (*rayleigh) = (_e153 * _e158);
    let _e160 = transmittance;
    let _e162 = r_d_1;
    let _e165 = GetProfileDensity(atmosphere_15.mie_density, (_e162 - atmosphere_15.bottom_radius));
    (*mie) = (_e160 * _e165);
    return;
}

fn DistanceToNearestAtmosphereBoundary(atmosphere_16: AtmosphereParameters, r_16: f32, mu_13: f32, ray_r_mu_intersects_ground_3: bool) -> f32 {
    var r_17: f32;
    var mu_14: f32;
    var ray_r_mu_intersects_ground_4: bool;

    r_17 = r_16;
    mu_14 = mu_13;
    ray_r_mu_intersects_ground_4 = ray_r_mu_intersects_ground_3;
    let _e125 = ray_r_mu_intersects_ground_4;
    if _e125 {
        {
            let _e126 = r_17;
            let _e127 = mu_14;
            let _e128 = DistanceToBottomAtmosphereBoundary(atmosphere_16, _e126, _e127);
            return _e128;
        }
    } else {
        {
            let _e129 = r_17;
            let _e130 = mu_14;
            let _e131 = DistanceToTopAtmosphereBoundary(atmosphere_16, _e129, _e130);
            return _e131;
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

    let _e130 = DistanceToNearestAtmosphereBoundary(atmosphere_17, r_18, mu_15, ray_r_mu_intersects_ground_5);
    let _e131 = SAMPLE_COUNT_1;
    dx_1 = (_e130 / f32(_e131));
    loop {
        let _e143 = i_5;
        let _e144 = SAMPLE_COUNT_1;
        if !((_e143 <= _e144)) {
            break;
        }
        {
            let _e150 = i_5;
            let _e152 = dx_1;
            d_i_1 = (f32(_e150) * _e152);
            let _e157 = d_i_1;
            ComputeSingleScatteringIntegrand(atmosphere_17, transmittance_texture_t_4, transmittance_texture_s_4, r_18, mu_15, mu_s_5, nu_4, _e157, ray_r_mu_intersects_ground_5, (&rayleigh_i), (&mie_i));
            let _e162 = i_5;
            let _e165 = i_5;
            let _e166 = SAMPLE_COUNT_1;
            if ((_e162 == 0i) || (_e165 == _e166)) {
                local_4 = 0.5f;
            } else {
                local_4 = 1f;
            }
            let _e172 = local_4;
            weight_i_1 = _e172;
            let _e174 = rayleigh_sum;
            let _e175 = rayleigh_i;
            let _e176 = weight_i_1;
            rayleigh_sum = (_e174 + (_e175 * _e176));
            let _e179 = mie_sum;
            let _e180 = mie_i;
            let _e181 = weight_i_1;
            mie_sum = (_e179 + (_e180 * _e181));
        }
        continuing {
            let _e147 = i_5;
            i_5 = (_e147 + 1i);
        }
    }
    let _e184 = rayleigh_sum;
    let _e185 = dx_1;
    (*rayleigh_1) = (((_e184 * _e185) * atmosphere_17.solar_irradiance) * atmosphere_17.rayleigh_scattering);
    let _e191 = mie_sum;
    let _e192 = dx_1;
    (*mie_1) = (((_e191 * _e192) * atmosphere_17.solar_irradiance) * atmosphere_17.mie_scattering);
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
    let _e134 = H_3;
    let _e137 = GetUnitRangeFromTextureCoord(uvwz.w, 32i);
    rho_3 = (_e134 * _e137);
    let _e140 = rho_3;
    let _e141 = rho_3;
    (*r_19) = sqrt(((_e140 * _e141) + (atmosphere_18.bottom_radius * atmosphere_18.bottom_radius)));
    if (uvwz.z < 0.5f) {
        {
            let _e151 = (*r_19);
            d_min_5 = (_e151 - atmosphere_18.bottom_radius);
            let _e155 = rho_3;
            d_max_5 = _e155;
            let _e157 = d_min_5;
            let _e158 = d_max_5;
            let _e159 = d_min_5;
            let _e169 = GetUnitRangeFromTextureCoord((1f - (2f * uvwz.z)), 64i);
            d_8 = (_e157 + ((_e158 - _e159) * _e169));
            let _e173 = d_8;
            if (_e173 == 0f) {
                local_5 = -1f;
            } else {
                let _e181 = rho_3;
                let _e182 = rho_3;
                let _e184 = d_8;
                let _e185 = d_8;
                let _e190 = (*r_19);
                let _e192 = d_8;
                let _e195 = ClampCosine((-(((_e181 * _e182) + (_e184 * _e185))) / ((2f * _e190) * _e192)));
                local_5 = _e195;
            }
            let _e197 = local_5;
            (*mu_16) = _e197;
            (*ray_r_mu_intersects_ground_6) = true;
        }
    } else {
        {
            let _e200 = (*r_19);
            d_min_6 = (atmosphere_18.top_radius - _e200);
            let _e203 = rho_3;
            let _e204 = H_3;
            d_max_6 = (_e203 + _e204);
            let _e207 = d_min_6;
            let _e208 = d_max_6;
            let _e209 = d_min_6;
            let _e219 = GetUnitRangeFromTextureCoord(((2f * uvwz.z) - 1f), 64i);
            d_9 = (_e207 + ((_e208 - _e209) * _e219));
            let _e223 = d_9;
            if (_e223 == 0f) {
                local_6 = 1f;
            } else {
                let _e230 = H_3;
                let _e231 = H_3;
                let _e233 = rho_3;
                let _e234 = rho_3;
                let _e237 = d_9;
                let _e238 = d_9;
                let _e242 = (*r_19);
                let _e244 = d_9;
                let _e247 = ClampCosine(((((_e230 * _e231) - (_e233 * _e234)) - (_e237 * _e238)) / ((2f * _e242) * _e244)));
                local_6 = _e247;
            }
            let _e249 = local_6;
            (*mu_16) = _e249;
            (*ray_r_mu_intersects_ground_6) = false;
        }
    }
    let _e253 = GetUnitRangeFromTextureCoord(uvwz.y, 32i);
    x_mu_s_1 = _e253;
    d_min_7 = (atmosphere_18.top_radius - atmosphere_18.bottom_radius);
    let _e259 = H_3;
    d_max_7 = _e259;
    let _e263 = DistanceToTopAtmosphereBoundary(atmosphere_18, atmosphere_18.bottom_radius, atmosphere_18.mu_s_min);
    D_1 = _e263;
    let _e265 = D_1;
    let _e266 = d_min_7;
    let _e268 = d_max_7;
    let _e269 = d_min_7;
    A_1 = ((_e265 - _e266) / (_e268 - _e269));
    let _e273 = A_1;
    let _e274 = x_mu_s_1;
    let _e275 = A_1;
    let _e279 = x_mu_s_1;
    let _e280 = A_1;
    a_2 = ((_e273 - (_e274 * _e275)) / (1f + (_e279 * _e280)));
    let _e285 = d_min_7;
    let _e286 = a_2;
    let _e287 = A_1;
    let _e289 = d_max_7;
    let _e290 = d_min_7;
    d_10 = (_e285 + (min(_e286, _e287) * (_e289 - _e290)));
    let _e295 = d_10;
    if (_e295 == 0f) {
        local_7 = 1f;
    } else {
        let _e302 = H_3;
        let _e303 = H_3;
        let _e305 = d_10;
        let _e306 = d_10;
        let _e312 = d_10;
        let _e315 = ClampCosine((((_e302 * _e303) - (_e305 * _e306)) / ((2f * atmosphere_18.bottom_radius) * _e312)));
        local_7 = _e315;
    }
    let _e317 = local_7;
    (*mu_s_6) = _e317;
    let _e323 = ClampCosine(((uvwz.x * 2f) - 1f));
    (*nu_5) = _e323;
    return;
}

fn GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_19: AtmosphereParameters, frag_coord_1: vec3<f32>, r_20: ptr<function, f32>, mu_17: ptr<function, f32>, mu_s_7: ptr<function, f32>, nu_6: ptr<function, f32>, ray_r_mu_intersects_ground_7: ptr<function, bool>) {
    var SCATTERING_TEXTURE_SIZE: vec4<f32> = vec4<f32>(7f, 32f, 128f, 32f);
    var frag_coord_nu: f32;
    var frag_coord_mu_s: f32;
    var uvwz_1: vec4<f32>;

    frag_coord_nu = floor((frag_coord_1.x / 32f));
    frag_coord_mu_s = (frag_coord_1.x - (floor((frag_coord_1.x / 32f)) * 32f));
    let _e151 = frag_coord_nu;
    let _e152 = frag_coord_mu_s;
    let _e156 = SCATTERING_TEXTURE_SIZE;
    uvwz_1 = (vec4<f32>(_e151, _e152, frag_coord_1.y, frag_coord_1.z) / _e156);
    let _e159 = uvwz_1;
    GetRMuMuSNuFromScatteringTextureUvwz(atmosphere_19, _e159, r_20, mu_17, mu_s_7, nu_6, ray_r_mu_intersects_ground_7);
    let _e170 = (*nu_6);
    let _e171 = (*mu_17);
    let _e172 = (*mu_s_7);
    let _e175 = (*mu_17);
    let _e176 = (*mu_17);
    let _e180 = (*mu_s_7);
    let _e181 = (*mu_s_7);
    let _e187 = (*mu_17);
    let _e188 = (*mu_s_7);
    let _e191 = (*mu_17);
    let _e192 = (*mu_17);
    let _e196 = (*mu_s_7);
    let _e197 = (*mu_s_7);
    (*nu_6) = clamp(_e170, ((_e171 * _e172) - sqrt(((1f - (_e175 * _e176)) * (1f - (_e180 * _e181))))), ((_e187 * _e188) + sqrt(((1f - (_e191 * _e192)) * (1f - (_e196 * _e197))))));
    return;
}

fn ComputeSingleScatteringTexture(atmosphere_20: AtmosphereParameters, transmittance_texture_t_5: texture_2d<f32>, transmittance_texture_s_5: sampler, frag_coord_2: vec3<f32>, rayleigh_2: ptr<function, vec3<f32>>, mie_2: ptr<function, vec3<f32>>) {
    var r_21: f32;
    var mu_18: f32;
    var mu_s_8: f32;
    var nu_7: f32;
    var ray_r_mu_intersects_ground_8: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_20, frag_coord_2, (&r_21), (&mu_18), (&mu_s_8), (&nu_7), (&ray_r_mu_intersects_ground_8));
    let _e139 = r_21;
    let _e140 = mu_18;
    let _e141 = mu_s_8;
    let _e142 = nu_7;
    let _e143 = ray_r_mu_intersects_ground_8;
    ComputeSingleScattering(atmosphere_20, transmittance_texture_t_5, transmittance_texture_s_5, _e139, _e140, _e141, _e142, _e143, rayleigh_2, mie_2);
    return;
}

fn GetScattering(atmosphere_21: AtmosphereParameters, scattering_texture_t: texture_3d<f32>, scattering_texture_s: sampler, r_22: f32, mu_19: f32, mu_s_9: f32, nu_8: f32, ray_r_mu_intersects_ground_9: bool) -> vec3<f32> {
    var uvwz_2: vec4<f32>;
    var tex_coord_x: f32;
    var tex_x: f32;
    var lerp: f32;
    var uvw0_: vec3<f32>;
    var uvw1_: vec3<f32>;

    let _e126 = GetScatteringTextureUvwzFromRMuMuSNu(atmosphere_21, r_22, mu_19, mu_s_9, nu_8, ray_r_mu_intersects_ground_9);
    uvwz_2 = _e126;
    let _e128 = uvwz_2;
    tex_coord_x = (_e128.x * 7f);
    let _e136 = tex_coord_x;
    tex_x = floor(_e136);
    let _e139 = tex_coord_x;
    let _e140 = tex_x;
    lerp = (_e139 - _e140);
    let _e143 = tex_x;
    let _e144 = uvwz_2;
    let _e150 = uvwz_2;
    let _e152 = uvwz_2;
    uvw0_ = vec3<f32>(((_e143 + _e144.y) / 8f), _e150.z, _e152.w);
    let _e156 = tex_x;
    let _e159 = uvwz_2;
    let _e165 = uvwz_2;
    let _e167 = uvwz_2;
    uvw1_ = vec3<f32>((((_e156 + 1f) + _e159.y) / 8f), _e165.z, _e167.w);
    let _e171 = uvw0_;
    let _e172 = textureSample(scattering_texture_t, scattering_texture_s, _e171);
    let _e174 = lerp;
    let _e177 = uvw1_;
    let _e178 = textureSample(scattering_texture_t, scattering_texture_s, _e177);
    let _e179 = lerp;
    return vec3<f32>(((_e172 * (1f - _e174)) + (_e178 * _e179)).xyz);
}

fn GetScattering_1(atmosphere_22: AtmosphereParameters, single_rayleigh_scattering_texture_t: texture_3d<f32>, single_rayleigh_scattering_texture_s: sampler, single_mie_scattering_texture_t: texture_3d<f32>, single_mie_scattering_texture_s: sampler, multiple_scattering_texture_t: texture_3d<f32>, multiple_scattering_texture_s: sampler, r_23: f32, mu_20: f32, mu_s_10: f32, nu_9: f32, ray_r_mu_intersects_ground_10: bool, scattering_order: i32) -> vec3<f32> {
    var rayleigh_3: vec3<f32>;
    var mie_3: vec3<f32>;

    if (scattering_order == 1i) {
        {
            let _e133 = GetScattering(atmosphere_22, single_rayleigh_scattering_texture_t, single_rayleigh_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            rayleigh_3 = _e133;
            let _e135 = GetScattering(atmosphere_22, single_mie_scattering_texture_t, single_mie_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            mie_3 = _e135;
            let _e137 = rayleigh_3;
            let _e138 = RayleighPhaseFunction(nu_9);
            let _e140 = mie_3;
            let _e142 = MiePhaseFunction(atmosphere_22.mie_phase_function_g, nu_9);
            return ((_e137 * _e138) + (_e140 * _e142));
        }
    } else {
        {
            let _e145 = GetScattering(atmosphere_22, multiple_scattering_texture_t, multiple_scattering_texture_s, r_23, mu_20, mu_s_10, nu_9, ray_r_mu_intersects_ground_10);
            return _e145;
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
    let _e146 = omega;
    if (_e146.x == 0f) {
        local_8 = 0f;
    } else {
        let _e153 = omega;
        local_8 = ((nu_10 - (mu_21 * mu_s_11)) / _e153.x);
    }
    let _e157 = local_8;
    sun_dir_x = _e157;
    let _e160 = sun_dir_x;
    let _e161 = sun_dir_x;
    sun_dir_y = sqrt(max(((1f - (_e160 * _e161)) - (mu_s_11 * mu_s_11)), 0f));
    let _e170 = sun_dir_x;
    let _e171 = sun_dir_y;
    omega_s = vec3<f32>(_e170, _e171, mu_s_11);
    let _e176 = SAMPLE_COUNT_2;
    dphi = (pi / f32(_e176));
    let _e180 = SAMPLE_COUNT_2;
    dtheta = (pi / f32(_e180));
    loop {
        let _e191 = l;
        let _e192 = SAMPLE_COUNT_2;
        if !((_e191 < _e192)) {
            break;
        }
        {
            let _e198 = l;
            let _e202 = dtheta;
            theta = ((f32(_e198) + 0.5f) * _e202);
            let _e205 = theta;
            cos_theta = cos(_e205);
            let _e208 = theta;
            sin_theta = sin(_e208);
            let _e211 = cos_theta;
            let _e212 = RayIntersectsGround(atmosphere_23, r_24, _e211);
            ray_r_theta_intersects_ground = _e212;
            distance_to_ground = 0f;
            transmittance_to_ground = vec3(0f);
            ground_albedo = vec3(0f);
            let _e224 = ray_r_theta_intersects_ground;
            if _e224 {
                {
                    let _e225 = cos_theta;
                    let _e226 = DistanceToBottomAtmosphereBoundary(atmosphere_23, r_24, _e225);
                    distance_to_ground = _e226;
                    let _e227 = cos_theta;
                    let _e228 = distance_to_ground;
                    let _e230 = GetTransmittance(atmosphere_23, transmittance_texture_t_6, transmittance_texture_s_6, r_24, _e227, _e228, true);
                    transmittance_to_ground = _e230;
                    ground_albedo = atmosphere_23.ground_albedo;
                }
            }
            m = 0i;
            loop {
                let _e234 = m;
                let _e236 = SAMPLE_COUNT_2;
                if !((_e234 < (2i * _e236))) {
                    break;
                }
                {
                    let _e243 = m;
                    let _e247 = dphi;
                    phi = ((f32(_e243) + 0.5f) * _e247);
                    let _e250 = phi;
                    let _e252 = sin_theta;
                    let _e254 = phi;
                    let _e256 = sin_theta;
                    let _e258 = cos_theta;
                    omega_i = vec3<f32>((cos(_e250) * _e252), (sin(_e254) * _e256), _e258);
                    let _e261 = dtheta;
                    let _e263 = dphi;
                    let _e266 = theta;
                    domega_i = ((((_e261 / rad) * (_e263 / rad)) * sin(_e266)) * sr);
                    let _e271 = omega_s;
                    let _e272 = omega_i;
                    nu1_ = dot(_e271, _e272);
                    let _e275 = omega_i;
                    let _e277 = nu1_;
                    let _e278 = ray_r_theta_intersects_ground;
                    let _e281 = GetScattering_1(atmosphere_23, single_rayleigh_scattering_texture_t_1, single_rayleigh_scattering_texture_s_1, single_mie_scattering_texture_t_1, single_mie_scattering_texture_s_1, multiple_scattering_texture_t_1, multiple_scattering_texture_s_1, r_24, _e275.z, mu_s_11, _e277, _e278, (scattering_order_1 - 1i));
                    incident_radiance = _e281;
                    let _e283 = zenith_direction;
                    let _e285 = omega_i;
                    let _e286 = distance_to_ground;
                    ground_normal = normalize(((_e283 * r_24) + (_e285 * _e286)));
                    let _e292 = ground_normal;
                    let _e293 = omega_s;
                    let _e295 = GetIrradiance(atmosphere_23, irradiance_texture_t_1, irradiance_texture_s_1, atmosphere_23.bottom_radius, dot(_e292, _e293));
                    ground_irradiance = _e295;
                    let _e297 = incident_radiance;
                    let _e298 = transmittance_to_ground;
                    let _e299 = ground_albedo;
                    let _e307 = ground_irradiance;
                    incident_radiance = (_e297 + (((_e298 * _e299) * 0.31830987f) * _e307));
                    let _e310 = omega;
                    let _e311 = omega_i;
                    nu2_ = dot(_e310, _e311);
                    let _e317 = GetProfileDensity(atmosphere_23.rayleigh_density, (r_24 - atmosphere_23.bottom_radius));
                    rayleigh_density = _e317;
                    let _e322 = GetProfileDensity(atmosphere_23.mie_density, (r_24 - atmosphere_23.bottom_radius));
                    mie_density = _e322;
                    let _e324 = rayleigh_mie;
                    let _e325 = incident_radiance;
                    let _e327 = rayleigh_density;
                    let _e329 = nu2_;
                    let _e330 = RayleighPhaseFunction(_e329);
                    let _e333 = mie_density;
                    let _e336 = nu2_;
                    let _e337 = MiePhaseFunction(atmosphere_23.mie_phase_function_g, _e336);
                    let _e341 = domega_i;
                    rayleigh_mie = (_e324 + ((_e325 * (((atmosphere_23.rayleigh_scattering * _e327) * _e330) + ((atmosphere_23.mie_scattering * _e333) * _e337))) * _e341));
                }
                continuing {
                    let _e240 = m;
                    m = (_e240 + 1i);
                }
            }
        }
        continuing {
            let _e195 = l;
            l = (_e195 + 1i);
        }
    }
    let _e344 = rayleigh_mie;
    return _e344;
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

    let _e130 = DistanceToNearestAtmosphereBoundary(atmosphere_24, r_25, mu_22, ray_r_mu_intersects_ground_11);
    let _e131 = SAMPLE_COUNT_3;
    dx_2 = (_e130 / f32(_e131));
    loop {
        let _e142 = i_6;
        let _e143 = SAMPLE_COUNT_3;
        if !((_e142 <= _e143)) {
            break;
        }
        {
            let _e149 = i_6;
            let _e151 = dx_2;
            d_i_2 = (f32(_e149) * _e151);
            let _e154 = d_i_2;
            let _e155 = d_i_2;
            let _e160 = d_i_2;
            let _e166 = ClampRadius(atmosphere_24, sqrt((((_e154 * _e155) + (((2f * r_25) * mu_22) * _e160)) + (r_25 * r_25))));
            r_i_1 = _e166;
            let _e169 = d_i_2;
            let _e171 = r_i_1;
            let _e173 = ClampCosine((((r_25 * mu_22) + _e169) / _e171));
            mu_i = _e173;
            let _e176 = d_i_2;
            let _e179 = r_i_1;
            let _e181 = ClampCosine((((r_25 * mu_s_12) + (_e176 * nu_11)) / _e179));
            mu_s_i = _e181;
            let _e183 = r_i_1;
            let _e184 = mu_i;
            let _e185 = mu_s_i;
            let _e186 = GetScattering(atmosphere_24, scattering_density_texture_t, scattering_density_texture_s, _e183, _e184, _e185, nu_11, ray_r_mu_intersects_ground_11);
            let _e187 = d_i_2;
            let _e188 = GetTransmittance(atmosphere_24, transmittance_texture_t_7, transmittance_texture_s_7, r_25, mu_22, _e187, ray_r_mu_intersects_ground_11);
            let _e190 = dx_2;
            rayleigh_mie_i = ((_e186 * _e188) * _e190);
            let _e193 = i_6;
            let _e196 = i_6;
            let _e197 = SAMPLE_COUNT_3;
            if ((_e193 == 0i) || (_e196 == _e197)) {
                local_9 = 0.5f;
            } else {
                local_9 = 1f;
            }
            let _e203 = local_9;
            weight_i_2 = _e203;
            let _e205 = rayleigh_mie_sum;
            let _e206 = rayleigh_mie_i;
            let _e207 = weight_i_2;
            rayleigh_mie_sum = (_e205 + (_e206 * _e207));
        }
        continuing {
            let _e146 = i_6;
            i_6 = (_e146 + 1i);
        }
    }
    let _e210 = rayleigh_mie_sum;
    return _e210;
}

fn ComputeScatteringDensityTexture(atmosphere_25: AtmosphereParameters, transmittance_texture_t_8: texture_2d<f32>, transmittance_texture_s_8: sampler, single_rayleigh_scattering_texture_t_2: texture_3d<f32>, single_rayleigh_scattering_texture_s_2: sampler, single_mie_scattering_texture_t_2: texture_3d<f32>, single_mie_scattering_texture_s_2: sampler, multiple_scattering_texture_t_2: texture_3d<f32>, multiple_scattering_texture_s_2: sampler, irradiance_texture_t_2: texture_2d<f32>, irradiance_texture_s_2: sampler, frag_coord_3: vec3<f32>, scattering_order_2: i32) -> vec3<f32> {
    var r_26: f32;
    var mu_23: f32;
    var mu_s_13: f32;
    var nu_12: f32;
    var ray_r_mu_intersects_ground_12: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_25, frag_coord_3, (&r_26), (&mu_23), (&mu_s_13), (&nu_12), (&ray_r_mu_intersects_ground_12));
    let _e146 = r_26;
    let _e147 = mu_23;
    let _e148 = mu_s_13;
    let _e149 = nu_12;
    let _e150 = ComputeScatteringDensity(atmosphere_25, transmittance_texture_t_8, transmittance_texture_s_8, single_rayleigh_scattering_texture_t_2, single_rayleigh_scattering_texture_s_2, single_mie_scattering_texture_t_2, single_mie_scattering_texture_s_2, multiple_scattering_texture_t_2, multiple_scattering_texture_s_2, irradiance_texture_t_2, irradiance_texture_s_2, _e146, _e147, _e148, _e149, scattering_order_2);
    return _e150;
}

fn ComputeMultipleScatteringTexture(atmosphere_26: AtmosphereParameters, transmittance_texture_t_9: texture_2d<f32>, transmittance_texture_s_9: sampler, scattering_density_texture_t_1: texture_3d<f32>, scattering_density_texture_s_1: sampler, frag_coord_4: vec3<f32>, nu_13: ptr<function, f32>) -> vec3<f32> {
    var r_27: f32;
    var mu_24: f32;
    var mu_s_14: f32;
    var ray_r_mu_intersects_ground_13: bool;

    GetRMuMuSNuFromScatteringTextureFragCoord(atmosphere_26, frag_coord_4, (&r_27), (&mu_24), (&mu_s_14), nu_13, (&ray_r_mu_intersects_ground_13));
    let _e139 = r_27;
    let _e140 = mu_24;
    let _e141 = mu_s_14;
    let _e142 = (*nu_13);
    let _e143 = ray_r_mu_intersects_ground_13;
    let _e144 = ComputeMultipleScattering(atmosphere_26, transmittance_texture_t_9, transmittance_texture_s_9, scattering_density_texture_t_1, scattering_density_texture_s_1, _e139, _e140, _e141, _e142, _e143);
    return _e144;
}

fn ComputeDirectIrradiance(atmosphere_27: AtmosphereParameters, transmittance_texture_t_10: texture_2d<f32>, transmittance_texture_s_10: sampler, r_28: f32, mu_s_15: f32) -> vec3<f32> {
    var alpha_s: f32;
    var local_10: f32;
    var local_11: f32;
    var average_cosine_factor: f32;

    alpha_s = (atmosphere_27.sun_angular_radius / rad);
    let _e126 = alpha_s;
    if (mu_s_15 < -(_e126)) {
        local_11 = 0f;
    } else {
        let _e130 = alpha_s;
        if (mu_s_15 > _e130) {
            local_10 = mu_s_15;
        } else {
            let _e132 = alpha_s;
            let _e134 = alpha_s;
            let _e138 = alpha_s;
            local_10 = (((mu_s_15 + _e132) * (mu_s_15 + _e134)) / (4f * _e138));
        }
        let _e142 = local_10;
        local_11 = _e142;
    }
    let _e144 = local_11;
    average_cosine_factor = _e144;
    let _e147 = GetTransmittanceToTopAtmosphereBoundary(atmosphere_27, transmittance_texture_t_10, transmittance_texture_s_10, r_28, mu_s_15);
    let _e149 = average_cosine_factor;
    return ((atmosphere_27.solar_irradiance * _e147) * _e149);
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

    let _e130 = SAMPLE_COUNT_4;
    dphi_1 = (pi / f32(_e130));
    let _e134 = SAMPLE_COUNT_4;
    dtheta_1 = (pi / f32(_e134));
    omega_s_1 = vec3<f32>(sqrt((1f - (mu_s_16 * mu_s_16))), 0f, mu_s_16);
    loop {
        let _e152 = j;
        let _e153 = SAMPLE_COUNT_4;
        if !((_e152 < (_e153 / 2i))) {
            break;
        }
        {
            let _e161 = j;
            let _e165 = dtheta_1;
            theta_1 = ((f32(_e161) + 0.5f) * _e165);
            i_7 = 0i;
            loop {
                let _e170 = i_7;
                let _e172 = SAMPLE_COUNT_4;
                if !((_e170 < (2i * _e172))) {
                    break;
                }
                {
                    let _e179 = i_7;
                    let _e183 = dphi_1;
                    phi_1 = ((f32(_e179) + 0.5f) * _e183);
                    let _e186 = phi_1;
                    let _e188 = theta_1;
                    let _e191 = phi_1;
                    let _e193 = theta_1;
                    let _e196 = theta_1;
                    omega_1 = vec3<f32>((cos(_e186) * sin(_e188)), (sin(_e191) * sin(_e193)), cos(_e196));
                    let _e200 = dtheta_1;
                    let _e202 = dphi_1;
                    let _e205 = theta_1;
                    domega = ((((_e200 / rad) * (_e202 / rad)) * sin(_e205)) * sr);
                    let _e210 = omega_1;
                    let _e211 = omega_s_1;
                    nu_14 = dot(_e210, _e211);
                    let _e214 = result_1;
                    let _e215 = omega_1;
                    let _e217 = nu_14;
                    let _e219 = GetScattering_1(atmosphere_28, single_rayleigh_scattering_texture_t_3, single_rayleigh_scattering_texture_s_3, single_mie_scattering_texture_t_3, single_mie_scattering_texture_s_3, multiple_scattering_texture_t_3, multiple_scattering_texture_s_3, r_29, _e215.z, mu_s_16, _e217, false, scattering_order_3);
                    let _e220 = omega_1;
                    let _e223 = domega;
                    result_1 = (_e214 + ((_e219 * _e220.z) * _e223));
                }
                continuing {
                    let _e176 = i_7;
                    i_7 = (_e176 + 1i);
                }
            }
        }
        continuing {
            let _e158 = j;
            j = (_e158 + 1i);
        }
    }
    let _e226 = result_1;
    return _e226;
}

fn GetRMuSFromIrradianceTextureUv(atmosphere_29: AtmosphereParameters, uv_3: vec2<f32>, r_30: ptr<function, f32>, mu_s_17: ptr<function, f32>) {
    var x_mu_s_2: f32;
    var x_r_3: f32;

    let _e124 = GetUnitRangeFromTextureCoord(uv_3.x, 64i);
    x_mu_s_2 = _e124;
    let _e128 = GetUnitRangeFromTextureCoord(uv_3.y, 16i);
    x_r_3 = _e128;
    let _e131 = x_r_3;
    (*r_30) = (atmosphere_29.bottom_radius + (_e131 * (atmosphere_29.top_radius - atmosphere_29.bottom_radius)));
    let _e138 = x_mu_s_2;
    let _e142 = ClampCosine(((2f * _e138) - 1f));
    (*mu_s_17) = _e142;
    return;
}

fn ComputeDirectIrradianceTexture(atmosphere_30: AtmosphereParameters, transmittance_texture_t_11: texture_2d<f32>, transmittance_texture_s_11: sampler, frag_coord_5: vec2<f32>) -> vec3<f32> {
    var r_31: f32;
    var mu_s_18: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_30, (frag_coord_5 / IRRADIANCE_TEXTURE_SIZE), (&r_31), (&mu_s_18));
    let _e130 = r_31;
    let _e131 = mu_s_18;
    let _e132 = ComputeDirectIrradiance(atmosphere_30, transmittance_texture_t_11, transmittance_texture_s_11, _e130, _e131);
    return _e132;
}

fn ComputeIndirectIrradianceTexture(atmosphere_31: AtmosphereParameters, single_rayleigh_scattering_texture_t_4: texture_3d<f32>, single_rayleigh_scattering_texture_s_4: sampler, single_mie_scattering_texture_t_4: texture_3d<f32>, single_mie_scattering_texture_s_4: sampler, multiple_scattering_texture_t_4: texture_3d<f32>, multiple_scattering_texture_s_4: sampler, frag_coord_6: vec2<f32>, scattering_order_4: i32) -> vec3<f32> {
    var r_32: f32;
    var mu_s_19: f32;

    GetRMuSFromIrradianceTextureUv(atmosphere_31, (frag_coord_6 / IRRADIANCE_TEXTURE_SIZE), (&r_32), (&mu_s_19));
    let _e135 = r_32;
    let _e136 = mu_s_19;
    let _e137 = ComputeIndirectIrradiance(atmosphere_31, single_rayleigh_scattering_texture_t_4, single_rayleigh_scattering_texture_s_4, single_mie_scattering_texture_t_4, single_mie_scattering_texture_s_4, multiple_scattering_texture_t_4, multiple_scattering_texture_s_4, _e135, _e136, scattering_order_4);
    return _e137;
}

fn pt_main() {
    let _e121 = scatteringDensity;
    let _e123 = ATMOSPHERE;
    let _e124 = gl_FragCoord_1;
    let _e125 = _e124.xy;
    let _e126 = global.layer;
    let _e133 = global.scatteringOrder;
    let _e134 = ComputeScatteringDensityTexture(_e123, transmittanceTexture_t, transmittanceTexture_s, singleRayleighScatteringTexture_t, singleRayleighScatteringTexture_s, singleMieScatteringTexture_t, singleMieScatteringTexture_s, multipleScatteringTexture_t, multipleScatteringTexture_s, irradianceTexture_t, irradianceTexture_s, vec3<f32>(_e125.x, _e125.y, (f32(_e126) + 0.5f)), _e133);
    scatteringDensity.x = _e134.x;
    scatteringDensity.y = _e134.y;
    scatteringDensity.z = _e134.z;
    scatteringDensity.w = 1f;
    return;
}

fn main_1() {
    let _e122 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e122;
    let _e124 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e124;
    let _e126 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e126;
    let _e128 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e128;
    let _e134 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e134;
    let _e140 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e140;
    let _e146 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e146;
    let _e152 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e152;
    let _e158 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e158;
    let _e164 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e164;
    let _e170 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e170;
    let _e176 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e176;
    let _e182 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e182;
    let _e188 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e188;
    let _e190 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e190;
    let _e196 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e196;
    let _e202 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e202;
    let _e208 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e208;
    let _e214 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e214;
    let _e220 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e220;
    let _e226 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e226;
    let _e232 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e232;
    let _e238 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e238;
    let _e244 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e244;
    let _e250 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e250;
    let _e252 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e252;
    let _e254 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e254;
    let _e256 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e256;
    let _e262 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e262;
    let _e268 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e268;
    let _e274 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e274;
    let _e280 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e280;
    let _e286 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e286;
    let _e292 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e292;
    let _e298 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e298;
    let _e304 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e304;
    let _e310 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e310;
    let _e316 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e316;
    let _e318 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e318;
    let _e320 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e320;
    let _e322 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e322;
    pt_main();
    return;
}

@fragment 
fn main(@builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e157 = scatteringDensity;
    return FragmentOutput(_e157);
}
