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
    resolution: vec2<f32>,
    texelSize: vec2<f32>,
    cameraNear: f32,
    cameraFar: f32,
    aspect: f32,
    time: f32,
    e0BlendOpacity: f32,
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
var e0WeightMap_t: texture_2d<f32>;
@group(0) @binding(6) 
var e0WeightMap_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> e0VOffset0_1: vec2<f32>;
var<private> e0VOffset1_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e38 = y_1;
    let _e39 = x_1;
    return exp2((_e38 * log2(_e39)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e38 = y_3;
    let _e39 = x_3;
    return exp2((_e38 * log2(_e39)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e38 = y_5;
    let _e39 = x_5;
    return exp2((_e38 * log2(_e39)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e38 = y_7;
    let _e39 = x_7;
    return exp2((_e38 * log2(_e39)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e37 = x_9;
    (*i) = trunc(_e37);
    let _e39 = x_9;
    let _e40 = (*i);
    return (_e39 - _e40);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e37 = x_11;
    (*i_1) = trunc(_e37);
    let _e39 = x_11;
    let _e40 = (*i_1);
    return (_e39 - _e40);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e37 = x_13;
    (*i_2) = trunc(_e37);
    let _e39 = x_13;
    let _e40 = (*i_2);
    return (_e39 - _e40);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e37 = x_15;
    (*i_3) = trunc(_e37);
    let _e39 = x_15;
    let _e40 = (*i_3);
    return (_e39 - _e40);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e36 = v_1;
    b = (bitcast<u32>(_e36) & 2147483647u);
    let _e41 = b;
    return (_e41 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e36 = v_3;
    b_1 = (bitcast<u32>(_e36) & 2147483647u);
    let _e41 = b_1;
    return (_e41 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e36 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e36) & vec2(2147483647u));
    let _e42 = b_2;
    return (_e42 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e36 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e36) & vec3(2147483647u));
    let _e42 = b_3;
    return (_e42 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e36 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e36) & vec4(2147483647u));
    let _e42 = b_4;
    return (_e42 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e36 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e36) & vec2(2147483647u));
    let _e42 = b_5;
    return (_e42 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e36 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e36) & vec3(2147483647u));
    let _e42 = b_6;
    return (_e42 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e36 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e36) & vec4(2147483647u));
    let _e42 = b_7;
    return (_e42 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e40 = value_1;
    return _e40;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e40 = value_3;
    let _e49 = pt_pow_2(((_e40.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e50 = value_3;
    let _e54 = value_3;
    let _e64 = mix(_e49, (_e50.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e54.xyz <= vec3(0.04045f))));
    let _e65 = value_3;
    return vec4<f32>(_e64.x, _e64.y, _e64.z, _e65.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e40 = value_5;
    let _e44 = pt_pow_2(_e40.xyz, vec3(0.41666f));
    let _e50 = value_5;
    let _e54 = value_5;
    let _e64 = mix(((_e44 * 1.055f) - vec3(0.055f)), (_e50.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e54.xyz <= vec3(0.0031308f))));
    let _e65 = value_5;
    return vec4<f32>(_e64.x, _e64.y, _e64.z, _e65.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e57 = value_7;
    let _e59 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e57.xyz);
    let _e60 = value_7;
    let _e66 = LinearTransferOETF(vec4<f32>(_e59.x, _e59.y, _e59.z, _e60.w));
    return _e66;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e45 = weights;
    let _e46 = rgb_1;
    return dot(_e45, _e46);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e40 = x_17;
    let _e41 = x_17;
    return (_e40 * _e41);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e40 = x_19;
    let _e41 = x_19;
    return (_e40 * _e41);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e40 = x_21;
    let _e41 = x_21;
    let _e43 = x_21;
    return ((_e40 * _e41) * _e43);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e40 = x_23;
    let _e41 = x_23;
    x2_ = (_e40 * _e41);
    let _e44 = x2_;
    let _e45 = x2_;
    return (_e44 * _e45);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e40 = v_17;
    let _e42 = v_17;
    let _e45 = v_17;
    return max(max(_e40.x, _e42.y), _e45.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e40 = v_19;
    return dot(_e40, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e46 = uv_1;
    let _e48 = a;
    let _e49 = b_8;
    dt = dot(_e46.xy, vec2<f32>(_e48, _e49));
    let _e53 = dt;
    sn = (_e53 - (floor((_e53 / 3.1415927f)) * 3.1415927f));
    let _e60 = sn;
    let _e62 = c;
    return fract((sin(_e60) * _e62));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e40 = v_21;
    return length(_e40);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e42 = matrix_1;
    let _e43 = dir_1;
    return normalize((_e42 * vec4<f32>(_e43.x, _e43.y, _e43.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e42 = normal_1;
    let _e48 = viewMatrix_1;
    return normalize((vec4<f32>(_e42.x, _e42.y, _e42.z, 0f) * _e48).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e42 = dir_3;
    let _e48 = viewMatrix_3;
    return normalize((vec4<f32>(_e42.x, _e42.y, _e42.z, 0f) * _e48).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e44 = m_1[2][3];
    return (_e44 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e40 = dir_5;
    let _e42 = dir_5;
    u = ((atan2(_e40.z, _e42.x) * 0.15915494f) + 0.5f);
    let _e50 = dir_5;
    v_22 = ((asin(clamp(_e50.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e62 = u;
    let _e63 = v_22;
    return vec2<f32>(_e62, _e63);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e41 = diffuseColor_1;
    return (0.31830987f * _e41);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e46 = dotVH_1;
    let _e50 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e46) - 6.98316f) * _e50));
    let _e54 = f0_1;
    let _e56 = fresnel;
    let _e59 = f90_1;
    let _e60 = fresnel;
    return ((_e54 * (1f - _e56)) + vec3((_e59 * _e60)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e46 = dotVH_3;
    let _e50 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e46) - 6.98316f) * _e50));
    let _e54 = f0_3;
    let _e56 = fresnel_1;
    let _e59 = f90_3;
    let _e60 = fresnel_1;
    return ((_e54 * (1f - _e56)) + (_e59 * _e60));
}

fn packNormalToRGB(normal_2: vec3<f32>) -> vec3<f32> {
    var normal_3: vec3<f32>;

    normal_3 = normal_2;
    let _e40 = normal_3;
    return ((normalize(_e40) * 0.5f) + vec3(0.5f));
}

fn unpackRGBToNormal(rgb_2: vec3<f32>) -> vec3<f32> {
    var rgb_3: vec3<f32>;

    rgb_3 = rgb_2;
    let _e41 = rgb_3;
    return ((2f * _e41.xyz) - vec3(1f));
}

fn packDepthToRGBA(v_23: f32) -> vec4<f32> {
    var v_24: f32;
    var vuf: f32;
    var af: f32;
    var bf: f32;
    var gf: f32;

    v_24 = v_23;
    let _e48 = v_24;
    if (_e48 <= 0f) {
        return vec4<f32>(0f, 0f, 0f, 0f);
    }
    let _e56 = v_24;
    if (_e56 >= 1f) {
        return vec4<f32>(1f, 1f, 1f, 1f);
    }
    let _e65 = v_24;
    let _e74 = pt_modf((_e65 * 16777216f), (&vuf));
    af = _e74;
    let _e76 = vuf;
    let _e80 = pt_modf((_e76 * ShiftRight8_), (&vuf));
    bf = _e80;
    let _e82 = vuf;
    let _e86 = pt_modf((_e82 * ShiftRight8_), (&vuf));
    gf = _e86;
    let _e88 = vuf;
    let _e90 = gf;
    let _e92 = bf;
    let _e94 = af;
    return vec4<f32>((_e88 * Inv255_), (_e90 * PackUpscale), (_e92 * PackUpscale), _e94);
}

fn packDepthToRGB(v_25: f32) -> vec3<f32> {
    var v_26: f32;
    var vuf_1: f32;
    var bf_1: f32;
    var gf_1: f32;

    v_26 = v_25;
    let _e48 = v_26;
    if (_e48 <= 0f) {
        return vec3<f32>(0f, 0f, 0f);
    }
    let _e55 = v_26;
    if (_e55 >= 1f) {
        return vec3<f32>(1f, 1f, 1f);
    }
    let _e63 = v_26;
    let _e72 = pt_modf((_e63 * 65536f), (&vuf_1));
    bf_1 = _e72;
    let _e74 = vuf_1;
    let _e78 = pt_modf((_e74 * ShiftRight8_), (&vuf_1));
    gf_1 = _e78;
    let _e80 = vuf_1;
    let _e82 = gf_1;
    let _e84 = bf_1;
    return vec3<f32>((_e80 * Inv255_), (_e82 * PackUpscale), _e84);
}

fn packDepthToRG(v_27: f32) -> vec2<f32> {
    var v_28: f32;
    var vuf_2: f32;
    var gf_2: f32;

    v_28 = v_27;
    let _e48 = v_28;
    if (_e48 <= 0f) {
        return vec2<f32>(0f, 0f);
    }
    let _e54 = v_28;
    if (_e54 >= 1f) {
        return vec2<f32>(1f, 1f);
    }
    let _e61 = v_28;
    let _e66 = pt_modf((_e61 * 256f), (&vuf_2));
    gf_2 = _e66;
    let _e68 = vuf_2;
    let _e70 = gf_2;
    return vec2<f32>((_e68 * Inv255_), _e70);
}

fn unpackRGBAToDepth(v_29: vec4<f32>) -> f32 {
    var v_30: vec4<f32>;

    v_30 = v_29;
    let _e48 = v_30;
    return dot(_e48, UnpackFactors4_);
}

fn unpackRGBToDepth(v_31: vec3<f32>) -> f32 {
    var v_32: vec3<f32>;

    v_32 = v_31;
    let _e48 = v_32;
    return dot(_e48, UnpackFactors3_);
}

fn unpackRGToDepth(v_33: vec2<f32>) -> f32 {
    var v_34: vec2<f32>;

    v_34 = v_33;
    let _e48 = v_34;
    let _e54 = v_34;
    return ((_e48.x * 0.99609375f) + (_e54.y * 0.00390625f));
}

fn pack2HalfToRGBA(v_35: vec2<f32>) -> vec4<f32> {
    var v_36: vec2<f32>;
    var r: vec4<f32>;

    v_36 = v_35;
    let _e48 = v_36;
    let _e50 = v_36;
    let _e55 = v_36;
    let _e57 = v_36;
    r = vec4<f32>(_e48.x, fract((_e50.x * 255f)), _e55.y, fract((_e57.y * 255f)));
    let _e64 = r;
    let _e66 = r;
    let _e71 = r;
    let _e73 = r;
    let _e75 = r;
    let _e80 = r;
    return vec4<f32>((_e64.x - (_e66.y / 255f)), _e71.y, (_e73.z - (_e75.w / 255f)), _e80.w);
}

fn unpackRGBATo2Half(v_37: vec4<f32>) -> vec2<f32> {
    var v_38: vec4<f32>;

    v_38 = v_37;
    let _e48 = v_38;
    let _e50 = v_38;
    let _e55 = v_38;
    let _e57 = v_38;
    return vec2<f32>((_e48.x + (_e50.y / 255f)), (_e55.z + (_e57.w / 255f)));
}

fn viewZToOrthographicDepth(viewZ: f32, near: f32, far: f32) -> f32 {
    var viewZ_1: f32;
    var near_1: f32;
    var far_1: f32;

    viewZ_1 = viewZ;
    near_1 = near;
    far_1 = far;
    let _e52 = viewZ_1;
    let _e53 = near_1;
    let _e55 = near_1;
    let _e56 = far_1;
    return ((_e52 + _e53) / (_e55 - _e56));
}

fn orthographicDepthToViewZ(depth: f32, near_2: f32, far_2: f32) -> f32 {
    var depth_1: f32;
    var near_3: f32;
    var far_3: f32;

    depth_1 = depth;
    near_3 = near_2;
    far_3 = far_2;
    let _e52 = depth_1;
    let _e53 = near_3;
    let _e54 = far_3;
    let _e57 = near_3;
    return ((_e52 * (_e53 - _e54)) - _e57);
}

fn viewZToPerspectiveDepth(viewZ_2: f32, near_4: f32, far_4: f32) -> f32 {
    var viewZ_3: f32;
    var near_5: f32;
    var far_5: f32;

    viewZ_3 = viewZ_2;
    near_5 = near_4;
    far_5 = far_4;
    let _e52 = near_5;
    let _e53 = viewZ_3;
    let _e55 = far_5;
    let _e57 = far_5;
    let _e58 = near_5;
    let _e60 = viewZ_3;
    return (((_e52 + _e53) * _e55) / ((_e57 - _e58) * _e60));
}

fn perspectiveDepthToViewZ(depth_2: f32, near_6: f32, far_6: f32) -> f32 {
    var depth_3: f32;
    var near_7: f32;
    var far_7: f32;

    depth_3 = depth_2;
    near_7 = near_6;
    far_7 = far_6;
    let _e52 = near_7;
    let _e53 = far_7;
    let _e55 = far_7;
    let _e56 = near_7;
    let _e58 = depth_3;
    let _e60 = far_7;
    return ((_e52 * _e53) / (((_e55 - _e56) * _e58) - _e60));
}

fn sRGBToLinear(value_8: vec4<f32>) -> vec4<f32> {
    var value_9: vec4<f32>;

    value_9 = value_8;
    let _e48 = value_9;
    let _e57 = pt_pow_2(((_e48.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e58 = value_9;
    let _e62 = value_9;
    let _e72 = mix(_e57, (_e58.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e62.xyz <= vec3(0.04045f))));
    let _e73 = value_9;
    return vec4<f32>(_e72.x, _e72.y, _e72.z, _e73.w);
}

fn readDepth(uv_2: vec2<f32>) -> f32 {
    var uv_3: vec2<f32>;
    var depth_4: f32;

    uv_3 = uv_2;
    let _e48 = uv_3;
    let _e49 = textureSample(depthBuffer_t, depthBuffer_s, _e48);
    depth_4 = _e49.x;
    let _e52 = depth_4;
    return _e52;
}

fn getViewZ(depth_5: f32) -> f32 {
    var depth_6: f32;

    depth_6 = depth_5;
    let _e48 = depth_6;
    let _e49 = global.cameraNear;
    let _e50 = global.cameraFar;
    let _e51 = perspectiveDepthToViewZ(_e48, _e49, _e50);
    return _e51;
}

fn RGBToHCV(RGB: vec3<f32>) -> vec3<f32> {
    var RGB_1: vec3<f32>;
    var P: vec4<f32>;
    var Q: vec4<f32>;
    var C: f32;
    var H: f32;

    RGB_1 = RGB;
    let _e48 = RGB_1;
    let _e49 = _e48.zy;
    let _e58 = RGB_1;
    let _e59 = _e58.yz;
    let _e68 = RGB_1;
    let _e70 = RGB_1;
    P = mix(vec4<f32>(_e49.x, _e49.y, -1f, 0.6666667f), vec4<f32>(_e59.x, _e59.y, 0f, -0.33333334f), vec4(step(_e68.z, _e70.y)));
    let _e76 = P;
    let _e77 = _e76.xyw;
    let _e78 = RGB_1;
    let _e84 = RGB_1;
    let _e86 = P;
    let _e87 = _e86.yzx;
    let _e92 = P;
    let _e94 = RGB_1;
    Q = mix(vec4<f32>(_e77.x, _e77.y, _e77.z, _e78.x), vec4<f32>(_e84.x, _e87.x, _e87.y, _e87.z), vec4(step(_e92.x, _e94.x)));
    let _e100 = Q;
    let _e102 = Q;
    let _e104 = Q;
    C = (_e100.x - min(_e102.w, _e104.y));
    let _e109 = Q;
    let _e111 = Q;
    let _e115 = C;
    let _e120 = Q;
    H = abs((((_e109.w - _e111.y) / ((6f * _e115) + 0.000001f)) + _e120.z));
    let _e125 = H;
    let _e126 = C;
    let _e127 = Q;
    return vec3<f32>(_e125, _e126, _e127.x);
}

fn RGBToHSL(RGB_2: vec3<f32>) -> vec3<f32> {
    var RGB_3: vec3<f32>;
    var HCV: vec3<f32>;
    var L: f32;
    var S: f32;

    RGB_3 = RGB_2;
    let _e48 = RGB_3;
    let _e49 = RGBToHCV(_e48);
    HCV = _e49;
    let _e51 = HCV;
    let _e53 = HCV;
    L = (_e51.z - (_e53.y * 0.5f));
    let _e59 = HCV;
    let _e62 = L;
    S = (_e59.y / ((1f - abs(((_e62 * 2f) - 1f))) + 0.000001f));
    let _e73 = HCV;
    let _e75 = S;
    let _e76 = L;
    return vec3<f32>(_e73.x, _e75, _e76);
}

fn HueToRGB(H_1: f32) -> vec3<f32> {
    var H_2: f32;
    var R: f32;
    var G: f32;
    var B: f32;

    H_2 = H_1;
    let _e48 = H_2;
    R = (abs(((_e48 * 6f) - 3f)) - 1f);
    let _e58 = H_2;
    G = (2f - abs(((_e58 * 6f) - 2f)));
    let _e67 = H_2;
    B = (2f - abs(((_e67 * 6f) - 4f)));
    let _e75 = R;
    let _e76 = G;
    let _e77 = B;
    return clamp(vec3<f32>(_e75, _e76, _e77), vec3(0f), vec3(1f));
}

fn HSLToRGB(HSL: vec3<f32>) -> vec3<f32> {
    var HSL_1: vec3<f32>;
    var RGB_4: vec3<f32>;
    var C_1: f32;

    HSL_1 = HSL;
    let _e48 = HSL_1;
    let _e50 = HueToRGB(_e48.x);
    RGB_4 = _e50;
    let _e54 = HSL_1;
    let _e61 = HSL_1;
    C_1 = ((1f - abs(((2f * _e54.z) - 1f))) * _e61.y);
    let _e65 = RGB_4;
    let _e69 = C_1;
    let _e71 = HSL_1;
    return (((_e65 - vec3(0.5f)) * _e69) + vec3(_e71.z));
}

fn e0Movec(c_1: vec2<bool>, variable: ptr<function, vec2<f32>>, value_10: vec2<f32>) {
    var c_2: vec2<bool>;
    var value_11: vec2<f32>;

    c_2 = c_1;
    value_11 = value_10;
    let _e51 = c_2;
    if _e51.x {
        {
            let _e54 = value_11;
            (*variable).x = _e54.x;
        }
    }
    let _e56 = c_2;
    if _e56.y {
        {
            let _e59 = value_11;
            (*variable).y = _e59.y;
            return;
        }
    } else {
        return;
    }
}

fn e0Movec_1(c_3: vec4<bool>, variable_1: ptr<function, vec4<f32>>, value_12: vec4<f32>) {
    var c_4: vec4<bool>;
    var value_13: vec4<f32>;
    var local: vec2<f32>;
    var local_1: vec2<f32>;

    c_4 = c_3;
    value_13 = value_12;
    let _e51 = c_4;
    let _e53 = (*variable_1);
    let _e55 = value_13;
    let _e57 = (*variable_1);
    local = _e57.xy;
    e0Movec(_e51.xy, (&local), _e55.xy);
    let _e64 = local.x;
    (*variable_1).x = _e64;
    let _e65 = local.y;
    (*variable_1).y = _e65;
    let _e66 = c_4;
    let _e68 = (*variable_1);
    let _e70 = value_13;
    let _e72 = (*variable_1);
    local_1 = _e72.zw;
    e0Movec(_e66.zw, (&local_1), _e70.zw);
    let _e79 = local_1.x;
    (*variable_1).z = _e79;
    let _e80 = local_1.y;
    (*variable_1).w = _e80;
    return;
}

fn e0MainImage(inputColor: vec4<f32>, uv_4: vec2<f32>, outputColor: ptr<function, vec4<f32>>) {
    var inputColor_1: vec4<f32>;
    var uv_5: vec2<f32>;
    var a_1: vec4<f32>;
    var color: vec4<f32>;
    var h: bool;
    var blendingOffset: vec4<f32>;
    var blendingWeight: vec2<f32>;
    var blendingCoord: vec4<f32>;

    inputColor_1 = inputColor;
    uv_5 = uv_4;
    let _e53 = e0VOffset0_1;
    let _e54 = textureSample(e0WeightMap_t, e0WeightMap_s, _e53);
    a_1.x = _e54.w;
    let _e57 = e0VOffset1_1;
    let _e58 = textureSample(e0WeightMap_t, e0WeightMap_s, _e57);
    a_1.y = _e58.y;
    let _e60 = a_1;
    let _e62 = uv_5;
    let _e63 = textureSample(e0WeightMap_t, e0WeightMap_s, _e62);
    let _e64 = _e63.xz;
    a_1.w = _e64.x;
    a_1.z = _e64.y;
    let _e69 = inputColor_1;
    color = _e69;
    let _e71 = a_1;
    if (dot(_e71, vec4(1f)) >= 0.00001f) {
        {
            let _e77 = a_1;
            let _e79 = a_1;
            let _e82 = a_1;
            let _e84 = a_1;
            h = (max(_e77.x, _e79.z) > max(_e82.y, _e84.w));
            let _e90 = a_1;
            let _e93 = a_1;
            blendingOffset = vec4<f32>(0f, _e90.y, 0f, _e93.w);
            let _e97 = a_1;
            blendingWeight = _e97.yw;
            let _e100 = h;
            let _e103 = a_1;
            let _e106 = a_1;
            e0Movec_1(vec4(_e100), (&blendingOffset), vec4<f32>(_e103.x, 0f, _e106.z, 0f));
            let _e111 = h;
            let _e114 = a_1;
            e0Movec(vec2(_e111), (&blendingWeight), _e114.xz);
            let _e117 = blendingWeight;
            let _e118 = blendingWeight;
            blendingWeight = (_e117 / vec2(dot(_e118, vec2(1f))));
            let _e124 = blendingOffset;
            let _e125 = global.texelSize;
            let _e126 = global.texelSize;
            let _e127 = -(_e126);
            let _e134 = uv_5;
            blendingCoord = ((_e124 * vec4<f32>(_e125.x, _e125.y, _e127.x, _e127.y)) + _e134.xyxy);
            let _e138 = blendingWeight;
            let _e140 = blendingCoord;
            let _e142 = textureSample(inputBuffer_t, inputBuffer_s, _e140.xy);
            color = (_e138.x * _e142);
            let _e144 = color;
            let _e145 = blendingWeight;
            let _e147 = blendingCoord;
            let _e149 = textureSample(inputBuffer_t, inputBuffer_s, _e147.zw);
            color = (_e144 + (_e145.y * _e149));
        }
    }
    let _e152 = color;
    (*outputColor) = _e152;
    return;
}

fn blend30_(dst: vec4<f32>, src: vec4<f32>, opacity: f32) -> vec4<f32> {
    var dst_1: vec4<f32>;
    var src_1: vec4<f32>;
    var opacity_1: f32;

    dst_1 = dst;
    src_1 = src;
    opacity_1 = opacity;
    let _e52 = src_1;
    return _e52;
}

fn main_1() {
    var color0_: vec4<f32>;
    var color1_: vec4<f32> = vec4(0f);

    let _e46 = vUv_1;
    let _e47 = textureSample(inputBuffer_t, inputBuffer_s, _e46);
    color0_ = _e47;
    let _e52 = color0_;
    let _e53 = vUv_1;
    e0MainImage(_e52, _e53, (&color1_));
    let _e56 = color0_;
    let _e57 = color1_;
    let _e58 = global.e0BlendOpacity;
    let _e59 = blend30_(_e56, _e57, _e58);
    color0_ = _e59;
    let _e61 = color0_;
    color0_.w = clamp(_e61.w, 0f, 1f);
    let _e66 = color0_;
    pc_fragColor = _e66;
    let _e67 = pc_fragColor;
    let _e68 = linearToOutputTexel(_e67);
    pc_fragColor = _e68;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) e0VOffset0_: vec2<f32>, @location(2) e0VOffset1_: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    e0VOffset0_1 = e0VOffset0_;
    e0VOffset1_1 = e0VOffset1_;
    main_1();
    let _e71 = pc_fragColor;
    return FragmentOutput(_e71);
}
