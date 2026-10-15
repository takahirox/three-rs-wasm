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
@group(0) @binding(1) 
var transmittanceTexture_t: texture_2d<f32>;
@group(0) @binding(2) 
var transmittanceTexture_s: sampler;
var<private> ATMOSPHERE: AtmosphereParameters;
var<private> position_1: vec2<f32>;
var<private> gl_Position: vec4<f32>;

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

fn pt_main() {
    let _e87 = position_1;
    gl_Position = vec4<f32>(_e87.x, _e87.y, 1f, 1f);
    return;
}

fn main_1() {
    let _e88 = global.pt_s_ATMOSPHERE_solar_irradiance;
    ATMOSPHERE.solar_irradiance = _e88;
    let _e90 = global.pt_s_ATMOSPHERE_sun_angular_radius;
    ATMOSPHERE.sun_angular_radius = _e90;
    let _e92 = global.pt_s_ATMOSPHERE_bottom_radius;
    ATMOSPHERE.bottom_radius = _e92;
    let _e94 = global.pt_s_ATMOSPHERE_top_radius;
    ATMOSPHERE.top_radius = _e94;
    let _e100 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_width;
    ATMOSPHERE.rayleigh_density.layers[0i].width = _e100;
    let _e106 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e106;
    let _e112 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e112;
    let _e118 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e118;
    let _e124 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e124;
    let _e130 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_width;
    ATMOSPHERE.rayleigh_density.layers[1i].width = _e130;
    let _e136 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e136;
    let _e142 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e142;
    let _e148 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e148;
    let _e154 = global.pt_s_ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e154;
    let _e156 = global.pt_s_ATMOSPHERE_rayleigh_scattering;
    ATMOSPHERE.rayleigh_scattering = _e156;
    let _e162 = global.pt_s_ATMOSPHERE_mie_density_layers_0_width;
    ATMOSPHERE.mie_density.layers[0i].width = _e162;
    let _e168 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_term;
    ATMOSPHERE.mie_density.layers[0i].exp_term = _e168;
    let _e174 = global.pt_s_ATMOSPHERE_mie_density_layers_0_exp_scale;
    ATMOSPHERE.mie_density.layers[0i].exp_scale = _e174;
    let _e180 = global.pt_s_ATMOSPHERE_mie_density_layers_0_linear_term;
    ATMOSPHERE.mie_density.layers[0i].linear_term = _e180;
    let _e186 = global.pt_s_ATMOSPHERE_mie_density_layers_0_constant_term;
    ATMOSPHERE.mie_density.layers[0i].constant_term = _e186;
    let _e192 = global.pt_s_ATMOSPHERE_mie_density_layers_1_width;
    ATMOSPHERE.mie_density.layers[1i].width = _e192;
    let _e198 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_term;
    ATMOSPHERE.mie_density.layers[1i].exp_term = _e198;
    let _e204 = global.pt_s_ATMOSPHERE_mie_density_layers_1_exp_scale;
    ATMOSPHERE.mie_density.layers[1i].exp_scale = _e204;
    let _e210 = global.pt_s_ATMOSPHERE_mie_density_layers_1_linear_term;
    ATMOSPHERE.mie_density.layers[1i].linear_term = _e210;
    let _e216 = global.pt_s_ATMOSPHERE_mie_density_layers_1_constant_term;
    ATMOSPHERE.mie_density.layers[1i].constant_term = _e216;
    let _e218 = global.pt_s_ATMOSPHERE_mie_scattering;
    ATMOSPHERE.mie_scattering = _e218;
    let _e220 = global.pt_s_ATMOSPHERE_mie_extinction;
    ATMOSPHERE.mie_extinction = _e220;
    let _e222 = global.pt_s_ATMOSPHERE_mie_phase_function_g;
    ATMOSPHERE.mie_phase_function_g = _e222;
    let _e228 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_width;
    ATMOSPHERE.absorption_density.layers[0i].width = _e228;
    let _e234 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_term;
    ATMOSPHERE.absorption_density.layers[0i].exp_term = _e234;
    let _e240 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_exp_scale;
    ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e240;
    let _e246 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_linear_term;
    ATMOSPHERE.absorption_density.layers[0i].linear_term = _e246;
    let _e252 = global.pt_s_ATMOSPHERE_absorption_density_layers_0_constant_term;
    ATMOSPHERE.absorption_density.layers[0i].constant_term = _e252;
    let _e258 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_width;
    ATMOSPHERE.absorption_density.layers[1i].width = _e258;
    let _e264 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_term;
    ATMOSPHERE.absorption_density.layers[1i].exp_term = _e264;
    let _e270 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_exp_scale;
    ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e270;
    let _e276 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_linear_term;
    ATMOSPHERE.absorption_density.layers[1i].linear_term = _e276;
    let _e282 = global.pt_s_ATMOSPHERE_absorption_density_layers_1_constant_term;
    ATMOSPHERE.absorption_density.layers[1i].constant_term = _e282;
    let _e284 = global.pt_s_ATMOSPHERE_absorption_extinction;
    ATMOSPHERE.absorption_extinction = _e284;
    let _e286 = global.pt_s_ATMOSPHERE_ground_albedo;
    ATMOSPHERE.ground_albedo = _e286;
    let _e288 = global.pt_s_ATMOSPHERE_mu_s_min;
    ATMOSPHERE.mu_s_min = _e288;
    pt_main();
    let _e290 = gl_Position;
    gl_Position.y = -(_e290.y);
    let _e294 = gl_Position;
    let _e296 = gl_Position;
    gl_Position.z = ((_e294.z + _e296.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec2<f32>) -> VertexOutput {
    position_1 = position;
    main_1();
    let _e93 = gl_Position;
    return VertexOutput(_e93);
}
