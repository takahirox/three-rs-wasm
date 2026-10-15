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

struct VertexOutput {
    @location(0) vUv: vec2<f32>,
    @location(1) e1VCameraPosition: vec3<f32>,
    @location(2) e1VRayDirection: vec3<f32>,
    @location(3) e1VGeometryAltitudeCorrection: vec3<f32>,
    @location(4) e1VEllipsoidRadiiSquared: vec3<f32>,
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
@group(0) @binding(3) 
var depthBuffer_t: texture_2d<f32>;
@group(0) @binding(4) 
var depthBuffer_s: sampler;
@group(0) @binding(5) 
var e0CloudsBuffer_t: texture_2d<f32>;
@group(0) @binding(6) 
var e0CloudsBuffer_s: sampler;
@group(0) @binding(7) 
var e1Transmittance_texture_t: texture_2d<f32>;
@group(0) @binding(8) 
var e1Transmittance_texture_s: sampler;
@group(0) @binding(9) 
var e1Scattering_texture_t: texture_3d<f32>;
@group(0) @binding(10) 
var e1Scattering_texture_s: sampler;
@group(0) @binding(11) 
var e1Irradiance_texture_t: texture_2d<f32>;
@group(0) @binding(12) 
var e1Irradiance_texture_s: sampler;
@group(0) @binding(13) 
var e1Single_mie_scattering_texture_t: texture_3d<f32>;
@group(0) @binding(14) 
var e1Single_mie_scattering_texture_s: sampler;
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
var<private> position_1: vec3<f32>;
var<private> normal_1: vec3<f32>;
var<private> uv_1: vec2<f32>;
var<private> vUv: vec2<f32>;
var<private> e1VCameraPosition: vec3<f32>;
var<private> e1VRayDirection: vec3<f32>;
var<private> e1VGeometryAltitudeCorrection: vec3<f32>;
var<private> e1VEllipsoidRadiiSquared: vec3<f32>;
var<private> gl_Position: vec4<f32>;

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

fn e1GetCameraRay(origin: ptr<function, vec3<f32>>, direction: ptr<function, vec3<f32>>) {
    var isPerspective: bool;
    var viewPosition: vec4<f32>;
    var worldDirection: vec4<f32>;
    var nearPoint: vec4<f32>;
    var farPoint: vec4<f32>;
    var worldDirection_1: vec4<f32>;
    var worldOrigin: vec4<f32>;

    let _e204 = global.e1InverseProjectionMatrix[2][3];
    isPerspective = (_e204 != 0f);
    let _e208 = isPerspective;
    if _e208 {
        {
            let _e209 = global.e1InverseProjectionMatrix;
            let _e210 = position_1;
            viewPosition = (_e209 * vec4<f32>(_e210.x, _e210.y, _e210.z, 1f));
            let _e218 = global.e1InverseViewMatrix;
            let _e219 = viewPosition;
            let _e220 = _e219.xyz;
            worldDirection = (_e218 * vec4<f32>(_e220.x, _e220.y, _e220.z, 0f));
            let _e228 = global.e1CameraPosition;
            (*origin) = _e228;
            let _e229 = worldDirection;
            (*direction) = _e229.xyz;
            return;
        }
    } else {
        {
            let _e231 = global.e1InverseProjectionMatrix;
            let _e232 = position_1;
            let _e233 = _e232.xy;
            nearPoint = (_e231 * vec4<f32>(_e233.x, _e233.y, -1f, 1f));
            let _e242 = global.e1InverseProjectionMatrix;
            let _e243 = position_1;
            let _e244 = _e243.xy;
            farPoint = (_e242 * vec4<f32>(_e244.x, _e244.y, -0.9f, 1f));
            let _e253 = nearPoint;
            let _e254 = nearPoint;
            nearPoint = (_e253 / vec4(_e254.w));
            let _e258 = farPoint;
            let _e259 = farPoint;
            farPoint = (_e258 / vec4(_e259.w));
            let _e263 = global.e1InverseViewMatrix;
            let _e264 = farPoint;
            let _e266 = nearPoint;
            let _e268 = (_e264.xyz - _e266.xyz);
            worldDirection_1 = (_e263 * vec4<f32>(_e268.x, _e268.y, _e268.z, 0f));
            let _e276 = global.e1InverseViewMatrix;
            let _e277 = nearPoint;
            worldOrigin = (_e276 * _e277);
            let _e280 = worldDirection_1;
            (*direction) = _e280.xyz;
            let _e282 = worldOrigin;
            (*origin) = _e282.xyz;
            return;
        }
    }
}

fn e1MainSupport() {
    var direction_1: vec3<f32>;
    var origin_1: vec3<f32>;
    var cameraPositionECEF: vec3<f32>;
    var radii: vec3<f32>;

    e1GetCameraRay((&origin_1), (&direction_1));
    let _e204 = global.e1WorldToECEFMatrix;
    let _e205 = origin_1;
    cameraPositionECEF = (_e204 * vec4<f32>(_e205.x, _e205.y, _e205.z, 1f)).xyz;
    let _e214 = cameraPositionECEF;
    let _e215 = global.e1AltitudeCorrection;
    e1VCameraPosition = ((_e214 + _e215) * 0.001f);
    let _e219 = global.e1WorldToECEFMatrix;
    let _e220 = direction_1;
    e1VRayDirection = (_e219 * vec4<f32>(_e220.x, _e220.y, _e220.z, 0f)).xyz;
    let _e228 = global.e1AltitudeCorrection;
    e1VGeometryAltitudeCorrection = (_e228 * 0.001f);
    let _e231 = e1VGeometryAltitudeCorrection;
    let _e233 = global.e1GeometricErrorCorrectionAmount;
    e1VGeometryAltitudeCorrection = (_e231 * (1f - _e233));
    let _e236 = global.e1EllipsoidRadii;
    radii = (_e236 * 0.001f);
    let _e240 = radii;
    let _e241 = radii;
    e1VEllipsoidRadiiSquared = (_e240 * _e241);
    return;
}

fn pt_main() {
    let _e198 = position_1;
    vUv = ((_e198.xy * 0.5f) + vec2(0.5f));
    e1MainSupport();
    let _e206 = position_1;
    let _e207 = _e206.xy;
    gl_Position = vec4<f32>(_e207.x, _e207.y, 1f, 1f);
    return;
}

fn main_1() {
    let _e203 = global.pt_u_e1ShadowIntervals[0];
    e1ShadowIntervals[0i] = _e203.xy;
    let _e209 = global.pt_u_e1ShadowIntervals[1];
    e1ShadowIntervals[1i] = _e209.xy;
    let _e212 = global.pt_s_e1ATMOSPHERE_solar_irradiance;
    e1ATMOSPHERE.solar_irradiance = _e212;
    let _e214 = global.pt_s_e1ATMOSPHERE_sun_angular_radius;
    e1ATMOSPHERE.sun_angular_radius = _e214;
    let _e216 = global.pt_s_e1ATMOSPHERE_bottom_radius;
    e1ATMOSPHERE.bottom_radius = _e216;
    let _e218 = global.pt_s_e1ATMOSPHERE_top_radius;
    e1ATMOSPHERE.top_radius = _e218;
    let _e224 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_width;
    e1ATMOSPHERE.rayleigh_density.layers[0i].width = _e224;
    let _e230 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].exp_term = _e230;
    let _e236 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_exp_scale;
    e1ATMOSPHERE.rayleigh_density.layers[0i].exp_scale = _e236;
    let _e242 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_linear_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].linear_term = _e242;
    let _e248 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_0_constant_term;
    e1ATMOSPHERE.rayleigh_density.layers[0i].constant_term = _e248;
    let _e254 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_width;
    e1ATMOSPHERE.rayleigh_density.layers[1i].width = _e254;
    let _e260 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].exp_term = _e260;
    let _e266 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_exp_scale;
    e1ATMOSPHERE.rayleigh_density.layers[1i].exp_scale = _e266;
    let _e272 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_linear_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].linear_term = _e272;
    let _e278 = global.pt_s_e1ATMOSPHERE_rayleigh_density_layers_1_constant_term;
    e1ATMOSPHERE.rayleigh_density.layers[1i].constant_term = _e278;
    let _e280 = global.pt_s_e1ATMOSPHERE_rayleigh_scattering;
    e1ATMOSPHERE.rayleigh_scattering = _e280;
    let _e286 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_width;
    e1ATMOSPHERE.mie_density.layers[0i].width = _e286;
    let _e292 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_term;
    e1ATMOSPHERE.mie_density.layers[0i].exp_term = _e292;
    let _e298 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_exp_scale;
    e1ATMOSPHERE.mie_density.layers[0i].exp_scale = _e298;
    let _e304 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_linear_term;
    e1ATMOSPHERE.mie_density.layers[0i].linear_term = _e304;
    let _e310 = global.pt_s_e1ATMOSPHERE_mie_density_layers_0_constant_term;
    e1ATMOSPHERE.mie_density.layers[0i].constant_term = _e310;
    let _e316 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_width;
    e1ATMOSPHERE.mie_density.layers[1i].width = _e316;
    let _e322 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_term;
    e1ATMOSPHERE.mie_density.layers[1i].exp_term = _e322;
    let _e328 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_exp_scale;
    e1ATMOSPHERE.mie_density.layers[1i].exp_scale = _e328;
    let _e334 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_linear_term;
    e1ATMOSPHERE.mie_density.layers[1i].linear_term = _e334;
    let _e340 = global.pt_s_e1ATMOSPHERE_mie_density_layers_1_constant_term;
    e1ATMOSPHERE.mie_density.layers[1i].constant_term = _e340;
    let _e342 = global.pt_s_e1ATMOSPHERE_mie_scattering;
    e1ATMOSPHERE.mie_scattering = _e342;
    let _e344 = global.pt_s_e1ATMOSPHERE_mie_extinction;
    e1ATMOSPHERE.mie_extinction = _e344;
    let _e346 = global.pt_s_e1ATMOSPHERE_mie_phase_function_g;
    e1ATMOSPHERE.mie_phase_function_g = _e346;
    let _e352 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_width;
    e1ATMOSPHERE.absorption_density.layers[0i].width = _e352;
    let _e358 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_term;
    e1ATMOSPHERE.absorption_density.layers[0i].exp_term = _e358;
    let _e364 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_exp_scale;
    e1ATMOSPHERE.absorption_density.layers[0i].exp_scale = _e364;
    let _e370 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_linear_term;
    e1ATMOSPHERE.absorption_density.layers[0i].linear_term = _e370;
    let _e376 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_0_constant_term;
    e1ATMOSPHERE.absorption_density.layers[0i].constant_term = _e376;
    let _e382 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_width;
    e1ATMOSPHERE.absorption_density.layers[1i].width = _e382;
    let _e388 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_term;
    e1ATMOSPHERE.absorption_density.layers[1i].exp_term = _e388;
    let _e394 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_exp_scale;
    e1ATMOSPHERE.absorption_density.layers[1i].exp_scale = _e394;
    let _e400 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_linear_term;
    e1ATMOSPHERE.absorption_density.layers[1i].linear_term = _e400;
    let _e406 = global.pt_s_e1ATMOSPHERE_absorption_density_layers_1_constant_term;
    e1ATMOSPHERE.absorption_density.layers[1i].constant_term = _e406;
    let _e408 = global.pt_s_e1ATMOSPHERE_absorption_extinction;
    e1ATMOSPHERE.absorption_extinction = _e408;
    let _e410 = global.pt_s_e1ATMOSPHERE_ground_albedo;
    e1ATMOSPHERE.ground_albedo = _e410;
    let _e412 = global.pt_s_e1ATMOSPHERE_mu_s_min;
    e1ATMOSPHERE.mu_s_min = _e412;
    pt_main();
    let _e414 = gl_Position;
    gl_Position.y = -(_e414.y);
    let _e418 = gl_Position;
    let _e420 = gl_Position;
    gl_Position.z = ((_e418.z + _e420.w) * 0.5f);
    return;
}

@vertex 
fn main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>) -> VertexOutput {
    position_1 = position;
    normal_1 = normal;
    uv_1 = uv;
    main_1();
    let _e241 = vUv;
    let _e243 = e1VCameraPosition;
    let _e245 = e1VRayDirection;
    let _e247 = e1VGeometryAltitudeCorrection;
    let _e249 = e1VEllipsoidRadiiSquared;
    let _e251 = gl_Position;
    return VertexOutput(_e241, _e243, _e245, _e247, _e249, _e251);
}
