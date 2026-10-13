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

struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    opacity: f32,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
var<private> vNormal_1: vec3<f32>;
var<private> pc_fragColor: vec4<f32>;
var<private> gl_FrontFacing_1: bool;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e20 = y_1;
    let _e21 = x_1;
    return exp2((_e20 * log2(_e21)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e20 = y_3;
    let _e21 = x_3;
    return exp2((_e20 * log2(_e21)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e20 = y_5;
    let _e21 = x_5;
    return exp2((_e20 * log2(_e21)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e20 = y_7;
    let _e21 = x_7;
    return exp2((_e20 * log2(_e21)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e19 = x_9;
    (*i) = trunc(_e19);
    let _e21 = x_9;
    let _e22 = (*i);
    return (_e21 - _e22);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e19 = x_11;
    (*i_1) = trunc(_e19);
    let _e21 = x_11;
    let _e22 = (*i_1);
    return (_e21 - _e22);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e19 = x_13;
    (*i_2) = trunc(_e19);
    let _e21 = x_13;
    let _e22 = (*i_2);
    return (_e21 - _e22);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e19 = x_15;
    (*i_3) = trunc(_e19);
    let _e21 = x_15;
    let _e22 = (*i_3);
    return (_e21 - _e22);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e18 = v_1;
    b = (bitcast<u32>(_e18) & 2147483647u);
    let _e23 = b;
    return (_e23 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e18 = v_3;
    b_1 = (bitcast<u32>(_e18) & 2147483647u);
    let _e23 = b_1;
    return (_e23 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e18 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e18) & vec2(2147483647u));
    let _e24 = b_2;
    return (_e24 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e18 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e18) & vec3(2147483647u));
    let _e24 = b_3;
    return (_e24 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e18 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e18) & vec4(2147483647u));
    let _e24 = b_4;
    return (_e24 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e18 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e18) & vec2(2147483647u));
    let _e24 = b_5;
    return (_e24 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e18 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e18) & vec3(2147483647u));
    let _e24 = b_6;
    return (_e24 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e18 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e18) & vec4(2147483647u));
    let _e24 = b_7;
    return (_e24 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e20 = value_1;
    return _e20;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e20 = value_3;
    let _e29 = pt_pow_2(((_e20.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e30 = value_3;
    let _e34 = value_3;
    let _e44 = mix(_e29, (_e30.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e34.xyz <= vec3(0.04045f))));
    let _e45 = value_3;
    return vec4<f32>(_e44.x, _e44.y, _e44.z, _e45.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e20 = value_5;
    let _e24 = pt_pow_2(_e20.xyz, vec3(0.41666f));
    let _e30 = value_5;
    let _e34 = value_5;
    let _e44 = mix(((_e24 * 1.055f) - vec3(0.055f)), (_e30.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e34.xyz <= vec3(0.0031308f))));
    let _e45 = value_5;
    return vec4<f32>(_e44.x, _e44.y, _e44.z, _e45.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e20 = value_7;
    let _e38 = (_e20.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e39 = value_7;
    let _e45 = LinearTransferOETF(vec4<f32>(_e38.x, _e38.y, _e38.z, _e39.w));
    return _e45;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e25 = weights;
    let _e26 = rgb_1;
    return dot(_e25, _e26);
}

fn main_1() {
    var diffuseColor: vec4<f32>;
    var local: f32;
    var faceDirection: f32;
    var normal: vec3<f32>;
    var nonPerturbedNormal: vec3<f32>;

    let _e21 = global.opacity;
    diffuseColor = vec4<f32>(0f, 0f, 0f, _e21);
    let _e25 = gl_FrontFacing_1;
    if _e25 {
        local = 1f;
    } else {
        local = -1f;
    }
    let _e30 = local;
    faceDirection = _e30;
    let _e32 = vNormal_1;
    normal = normalize(_e32);
    let _e35 = normal;
    nonPerturbedNormal = _e35;
    let _e37 = normal;
    let _e43 = ((normalize(_e37) * 0.5f) + vec3(0.5f));
    let _e44 = diffuseColor;
    pc_fragColor = vec4<f32>(_e43.x, _e43.y, _e43.z, _e44.w);
    pc_fragColor.w = 1f;
    return;
}

@fragment 
fn main(@location(0) vNormal: vec3<f32>, @builtin(front_facing) gl_FrontFacing: bool) -> FragmentOutput {
    vNormal_1 = vNormal;
    gl_FrontFacing_1 = gl_FrontFacing;
    main_1();
    let _e25 = pc_fragColor;
    return FragmentOutput(_e25);
}
