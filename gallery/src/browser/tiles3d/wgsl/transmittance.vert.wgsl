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

struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> position_1: vec2<f32>;
var<private> gl_Position: vec4<f32>;

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

fn pt_main() {
    let _e85 = position_1;
    gl_Position = vec4<f32>(_e85.x, _e85.y, 1f, 1f);
    return;
}

fn main_1() {
    let _e86 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e86;
    let _e88 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e88;
    let _e90 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e90;
    let _e92 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e92;
    let _e98 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e98;
    let _e104 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e104;
    let _e110 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e110;
    let _e116 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e116;
    let _e122 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e122;
    let _e128 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e128;
    let _e134 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e134;
    let _e140 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e140;
    let _e146 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e146;
    let _e152 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e152;
    let _e154 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e154;
    let _e160 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e160;
    let _e166 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e166;
    let _e172 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e172;
    let _e178 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e178;
    let _e184 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e184;
    let _e190 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e190;
    let _e196 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e196;
    let _e202 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e202;
    let _e208 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e208;
    let _e214 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e214;
    let _e216 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e216;
    let _e218 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e218;
    let _e220 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e220;
    let _e226 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e226;
    let _e232 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e232;
    let _e238 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e238;
    let _e244 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e244;
    let _e250 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e250;
    let _e256 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e256;
    let _e262 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e262;
    let _e268 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e268;
    let _e274 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e274;
    let _e280 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e280;
    let _e282 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e282;
    let _e284 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e284;
    let _e286 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e286;
    pt_main();
    let _e288 = gl_Position;
    gl_Position.y = -(_e288.y);
    let _e292 = gl_Position;
    let _e294 = gl_Position;
    gl_Position.z = ((_e292.z + _e294.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec2<f32>) -> VertexOutput {
    position_1 = position;
    main_1();
    let _e89 = gl_Position;
    return VertexOutput(_e89);
}
