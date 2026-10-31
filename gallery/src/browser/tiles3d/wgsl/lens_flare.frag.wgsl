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
    e0Intensity: f32,
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
var e0BloomBuffer_t: texture_2d<f32>;
@group(0) @binding(6) 
var e0BloomBuffer_s: sampler;
@group(0) @binding(7) 
var e0FeaturesBuffer_t: texture_2d<f32>;
@group(0) @binding(8) 
var e0FeaturesBuffer_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e42 = y_1;
    let _e43 = x_1;
    return exp2((_e42 * log2(_e43)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e42 = y_3;
    let _e43 = x_3;
    return exp2((_e42 * log2(_e43)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e42 = y_5;
    let _e43 = x_5;
    return exp2((_e42 * log2(_e43)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e42 = y_7;
    let _e43 = x_7;
    return exp2((_e42 * log2(_e43)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e41 = x_9;
    (*i) = trunc(_e41);
    let _e43 = x_9;
    let _e44 = (*i);
    return (_e43 - _e44);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e41 = x_11;
    (*i_1) = trunc(_e41);
    let _e43 = x_11;
    let _e44 = (*i_1);
    return (_e43 - _e44);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e41 = x_13;
    (*i_2) = trunc(_e41);
    let _e43 = x_13;
    let _e44 = (*i_2);
    return (_e43 - _e44);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e41 = x_15;
    (*i_3) = trunc(_e41);
    let _e43 = x_15;
    let _e44 = (*i_3);
    return (_e43 - _e44);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e40 = v_1;
    b = (bitcast<u32>(_e40) & 2147483647u);
    let _e45 = b;
    return (_e45 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e40 = v_3;
    b_1 = (bitcast<u32>(_e40) & 2147483647u);
    let _e45 = b_1;
    return (_e45 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e40 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e40) & vec2(2147483647u));
    let _e46 = b_2;
    return (_e46 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e40 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e40) & vec3(2147483647u));
    let _e46 = b_3;
    return (_e46 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e40 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e40) & vec4(2147483647u));
    let _e46 = b_4;
    return (_e46 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e40 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e40) & vec2(2147483647u));
    let _e46 = b_5;
    return (_e46 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e40 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e40) & vec3(2147483647u));
    let _e46 = b_6;
    return (_e46 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e40 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e40) & vec4(2147483647u));
    let _e46 = b_7;
    return (_e46 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e42 = value_1;
    return _e42;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e42 = value_3;
    let _e51 = pt_pow_2(((_e42.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e52 = value_3;
    let _e56 = value_3;
    let _e66 = mix(_e51, (_e52.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e56.xyz <= vec3(0.04045f))));
    let _e67 = value_3;
    return vec4<f32>(_e66.x, _e66.y, _e66.z, _e67.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e42 = value_5;
    let _e46 = pt_pow_2(_e42.xyz, vec3(0.41666f));
    let _e52 = value_5;
    let _e56 = value_5;
    let _e66 = mix(((_e46 * 1.055f) - vec3(0.055f)), (_e52.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e56.xyz <= vec3(0.0031308f))));
    let _e67 = value_5;
    return vec4<f32>(_e66.x, _e66.y, _e66.z, _e67.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e59 = value_7;
    let _e61 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e59.xyz);
    let _e62 = value_7;
    let _e68 = LinearTransferOETF(vec4<f32>(_e61.x, _e61.y, _e61.z, _e62.w));
    return _e68;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e47 = weights;
    let _e48 = rgb_1;
    return dot(_e47, _e48);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e42 = x_17;
    let _e43 = x_17;
    return (_e42 * _e43);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e42 = x_19;
    let _e43 = x_19;
    return (_e42 * _e43);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e42 = x_21;
    let _e43 = x_21;
    let _e45 = x_21;
    return ((_e42 * _e43) * _e45);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e42 = x_23;
    let _e43 = x_23;
    x2_ = (_e42 * _e43);
    let _e46 = x2_;
    let _e47 = x2_;
    return (_e46 * _e47);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e42 = v_17;
    let _e44 = v_17;
    let _e47 = v_17;
    return max(max(_e42.x, _e44.y), _e47.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e42 = v_19;
    return dot(_e42, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e48 = uv_1;
    let _e50 = a;
    let _e51 = b_8;
    dt = dot(_e48.xy, vec2<f32>(_e50, _e51));
    let _e55 = dt;
    sn = (_e55 - (floor((_e55 / 3.1415927f)) * 3.1415927f));
    let _e62 = sn;
    let _e64 = c;
    return fract((sin(_e62) * _e64));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e42 = v_21;
    return length(_e42);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e44 = matrix_1;
    let _e45 = dir_1;
    return normalize((_e44 * vec4<f32>(_e45.x, _e45.y, _e45.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e44 = normal_1;
    let _e50 = viewMatrix_1;
    return normalize((vec4<f32>(_e44.x, _e44.y, _e44.z, 0f) * _e50).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e44 = dir_3;
    let _e50 = viewMatrix_3;
    return normalize((vec4<f32>(_e44.x, _e44.y, _e44.z, 0f) * _e50).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e46 = m_1[2][3];
    return (_e46 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e42 = dir_5;
    let _e44 = dir_5;
    u = ((atan2(_e42.z, _e44.x) * 0.15915494f) + 0.5f);
    let _e52 = dir_5;
    v_22 = ((asin(clamp(_e52.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e64 = u;
    let _e65 = v_22;
    return vec2<f32>(_e64, _e65);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e43 = diffuseColor_1;
    return (0.31830987f * _e43);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e48 = dotVH_1;
    let _e52 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e48) - 6.98316f) * _e52));
    let _e56 = f0_1;
    let _e58 = fresnel;
    let _e61 = f90_1;
    let _e62 = fresnel;
    return ((_e56 * (1f - _e58)) + vec3((_e61 * _e62)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e48 = dotVH_3;
    let _e52 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e48) - 6.98316f) * _e52));
    let _e56 = f0_3;
    let _e58 = fresnel_1;
    let _e61 = f90_3;
    let _e62 = fresnel_1;
    return ((_e56 * (1f - _e58)) + (_e61 * _e62));
}

fn packNormalToRGB(normal_2: vec3<f32>) -> vec3<f32> {
    var normal_3: vec3<f32>;

    normal_3 = normal_2;
    let _e42 = normal_3;
    return ((normalize(_e42) * 0.5f) + vec3(0.5f));
}

fn unpackRGBToNormal(rgb_2: vec3<f32>) -> vec3<f32> {
    var rgb_3: vec3<f32>;

    rgb_3 = rgb_2;
    let _e43 = rgb_3;
    return ((2f * _e43.xyz) - vec3(1f));
}

fn packDepthToRGBA(v_23: f32) -> vec4<f32> {
    var v_24: f32;
    var vuf: f32;
    var af: f32;
    var bf: f32;
    var gf: f32;

    v_24 = v_23;
    let _e50 = v_24;
    if (_e50 <= 0f) {
        return vec4<f32>(0f, 0f, 0f, 0f);
    }
    let _e58 = v_24;
    if (_e58 >= 1f) {
        return vec4<f32>(1f, 1f, 1f, 1f);
    }
    let _e67 = v_24;
    let _e76 = pt_modf((_e67 * 16777216f), (&vuf));
    af = _e76;
    let _e78 = vuf;
    let _e82 = pt_modf((_e78 * ShiftRight8_), (&vuf));
    bf = _e82;
    let _e84 = vuf;
    let _e88 = pt_modf((_e84 * ShiftRight8_), (&vuf));
    gf = _e88;
    let _e90 = vuf;
    let _e92 = gf;
    let _e94 = bf;
    let _e96 = af;
    return vec4<f32>((_e90 * Inv255_), (_e92 * PackUpscale), (_e94 * PackUpscale), _e96);
}

fn packDepthToRGB(v_25: f32) -> vec3<f32> {
    var v_26: f32;
    var vuf_1: f32;
    var bf_1: f32;
    var gf_1: f32;

    v_26 = v_25;
    let _e50 = v_26;
    if (_e50 <= 0f) {
        return vec3<f32>(0f, 0f, 0f);
    }
    let _e57 = v_26;
    if (_e57 >= 1f) {
        return vec3<f32>(1f, 1f, 1f);
    }
    let _e65 = v_26;
    let _e74 = pt_modf((_e65 * 65536f), (&vuf_1));
    bf_1 = _e74;
    let _e76 = vuf_1;
    let _e80 = pt_modf((_e76 * ShiftRight8_), (&vuf_1));
    gf_1 = _e80;
    let _e82 = vuf_1;
    let _e84 = gf_1;
    let _e86 = bf_1;
    return vec3<f32>((_e82 * Inv255_), (_e84 * PackUpscale), _e86);
}

fn packDepthToRG(v_27: f32) -> vec2<f32> {
    var v_28: f32;
    var vuf_2: f32;
    var gf_2: f32;

    v_28 = v_27;
    let _e50 = v_28;
    if (_e50 <= 0f) {
        return vec2<f32>(0f, 0f);
    }
    let _e56 = v_28;
    if (_e56 >= 1f) {
        return vec2<f32>(1f, 1f);
    }
    let _e63 = v_28;
    let _e68 = pt_modf((_e63 * 256f), (&vuf_2));
    gf_2 = _e68;
    let _e70 = vuf_2;
    let _e72 = gf_2;
    return vec2<f32>((_e70 * Inv255_), _e72);
}

fn unpackRGBAToDepth(v_29: vec4<f32>) -> f32 {
    var v_30: vec4<f32>;

    v_30 = v_29;
    let _e50 = v_30;
    return dot(_e50, UnpackFactors4_);
}

fn unpackRGBToDepth(v_31: vec3<f32>) -> f32 {
    var v_32: vec3<f32>;

    v_32 = v_31;
    let _e50 = v_32;
    return dot(_e50, UnpackFactors3_);
}

fn unpackRGToDepth(v_33: vec2<f32>) -> f32 {
    var v_34: vec2<f32>;

    v_34 = v_33;
    let _e50 = v_34;
    let _e56 = v_34;
    return ((_e50.x * 0.99609375f) + (_e56.y * 0.00390625f));
}

fn pack2HalfToRGBA(v_35: vec2<f32>) -> vec4<f32> {
    var v_36: vec2<f32>;
    var r: vec4<f32>;

    v_36 = v_35;
    let _e50 = v_36;
    let _e52 = v_36;
    let _e57 = v_36;
    let _e59 = v_36;
    r = vec4<f32>(_e50.x, fract((_e52.x * 255f)), _e57.y, fract((_e59.y * 255f)));
    let _e66 = r;
    let _e68 = r;
    let _e73 = r;
    let _e75 = r;
    let _e77 = r;
    let _e82 = r;
    return vec4<f32>((_e66.x - (_e68.y / 255f)), _e73.y, (_e75.z - (_e77.w / 255f)), _e82.w);
}

fn unpackRGBATo2Half(v_37: vec4<f32>) -> vec2<f32> {
    var v_38: vec4<f32>;

    v_38 = v_37;
    let _e50 = v_38;
    let _e52 = v_38;
    let _e57 = v_38;
    let _e59 = v_38;
    return vec2<f32>((_e50.x + (_e52.y / 255f)), (_e57.z + (_e59.w / 255f)));
}

fn viewZToOrthographicDepth(viewZ: f32, near: f32, far: f32) -> f32 {
    var viewZ_1: f32;
    var near_1: f32;
    var far_1: f32;

    viewZ_1 = viewZ;
    near_1 = near;
    far_1 = far;
    let _e54 = viewZ_1;
    let _e55 = near_1;
    let _e57 = near_1;
    let _e58 = far_1;
    return ((_e54 + _e55) / (_e57 - _e58));
}

fn orthographicDepthToViewZ(depth: f32, near_2: f32, far_2: f32) -> f32 {
    var depth_1: f32;
    var near_3: f32;
    var far_3: f32;

    depth_1 = depth;
    near_3 = near_2;
    far_3 = far_2;
    let _e54 = depth_1;
    let _e55 = near_3;
    let _e56 = far_3;
    let _e59 = near_3;
    return ((_e54 * (_e55 - _e56)) - _e59);
}

fn viewZToPerspectiveDepth(viewZ_2: f32, near_4: f32, far_4: f32) -> f32 {
    var viewZ_3: f32;
    var near_5: f32;
    var far_5: f32;

    viewZ_3 = viewZ_2;
    near_5 = near_4;
    far_5 = far_4;
    let _e54 = near_5;
    let _e55 = viewZ_3;
    let _e57 = far_5;
    let _e59 = far_5;
    let _e60 = near_5;
    let _e62 = viewZ_3;
    return (((_e54 + _e55) * _e57) / ((_e59 - _e60) * _e62));
}

fn perspectiveDepthToViewZ(depth_2: f32, near_6: f32, far_6: f32) -> f32 {
    var depth_3: f32;
    var near_7: f32;
    var far_7: f32;

    depth_3 = depth_2;
    near_7 = near_6;
    far_7 = far_6;
    let _e54 = near_7;
    let _e55 = far_7;
    let _e57 = far_7;
    let _e58 = near_7;
    let _e60 = depth_3;
    let _e62 = far_7;
    return ((_e54 * _e55) / (((_e57 - _e58) * _e60) - _e62));
}

fn sRGBToLinear(value_8: vec4<f32>) -> vec4<f32> {
    var value_9: vec4<f32>;

    value_9 = value_8;
    let _e50 = value_9;
    let _e59 = pt_pow_2(((_e50.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e60 = value_9;
    let _e64 = value_9;
    let _e74 = mix(_e59, (_e60.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e64.xyz <= vec3(0.04045f))));
    let _e75 = value_9;
    return vec4<f32>(_e74.x, _e74.y, _e74.z, _e75.w);
}

fn readDepth(uv_2: vec2<f32>) -> f32 {
    var uv_3: vec2<f32>;
    var depth_4: f32;

    uv_3 = uv_2;
    let _e50 = uv_3;
    let _e51 = textureSample(depthBuffer_t, depthBuffer_s, _e50);
    depth_4 = _e51.x;
    let _e54 = depth_4;
    return _e54;
}

fn getViewZ(depth_5: f32) -> f32 {
    var depth_6: f32;

    depth_6 = depth_5;
    let _e50 = depth_6;
    let _e51 = global.cameraNear;
    let _e52 = global.cameraFar;
    let _e53 = perspectiveDepthToViewZ(_e50, _e51, _e52);
    return _e53;
}

fn RGBToHCV(RGB: vec3<f32>) -> vec3<f32> {
    var RGB_1: vec3<f32>;
    var P: vec4<f32>;
    var Q: vec4<f32>;
    var C: f32;
    var H: f32;

    RGB_1 = RGB;
    let _e50 = RGB_1;
    let _e51 = _e50.zy;
    let _e60 = RGB_1;
    let _e61 = _e60.yz;
    let _e70 = RGB_1;
    let _e72 = RGB_1;
    P = mix(vec4<f32>(_e51.x, _e51.y, -1f, 0.6666667f), vec4<f32>(_e61.x, _e61.y, 0f, -0.33333334f), vec4(step(_e70.z, _e72.y)));
    let _e78 = P;
    let _e79 = _e78.xyw;
    let _e80 = RGB_1;
    let _e86 = RGB_1;
    let _e88 = P;
    let _e89 = _e88.yzx;
    let _e94 = P;
    let _e96 = RGB_1;
    Q = mix(vec4<f32>(_e79.x, _e79.y, _e79.z, _e80.x), vec4<f32>(_e86.x, _e89.x, _e89.y, _e89.z), vec4(step(_e94.x, _e96.x)));
    let _e102 = Q;
    let _e104 = Q;
    let _e106 = Q;
    C = (_e102.x - min(_e104.w, _e106.y));
    let _e111 = Q;
    let _e113 = Q;
    let _e117 = C;
    let _e122 = Q;
    H = abs((((_e111.w - _e113.y) / ((6f * _e117) + 0.000001f)) + _e122.z));
    let _e127 = H;
    let _e128 = C;
    let _e129 = Q;
    return vec3<f32>(_e127, _e128, _e129.x);
}

fn RGBToHSL(RGB_2: vec3<f32>) -> vec3<f32> {
    var RGB_3: vec3<f32>;
    var HCV: vec3<f32>;
    var L: f32;
    var S: f32;

    RGB_3 = RGB_2;
    let _e50 = RGB_3;
    let _e51 = RGBToHCV(_e50);
    HCV = _e51;
    let _e53 = HCV;
    let _e55 = HCV;
    L = (_e53.z - (_e55.y * 0.5f));
    let _e61 = HCV;
    let _e64 = L;
    S = (_e61.y / ((1f - abs(((_e64 * 2f) - 1f))) + 0.000001f));
    let _e75 = HCV;
    let _e77 = S;
    let _e78 = L;
    return vec3<f32>(_e75.x, _e77, _e78);
}

fn HueToRGB(H_1: f32) -> vec3<f32> {
    var H_2: f32;
    var R: f32;
    var G: f32;
    var B: f32;

    H_2 = H_1;
    let _e50 = H_2;
    R = (abs(((_e50 * 6f) - 3f)) - 1f);
    let _e60 = H_2;
    G = (2f - abs(((_e60 * 6f) - 2f)));
    let _e69 = H_2;
    B = (2f - abs(((_e69 * 6f) - 4f)));
    let _e77 = R;
    let _e78 = G;
    let _e79 = B;
    return clamp(vec3<f32>(_e77, _e78, _e79), vec3(0f), vec3(1f));
}

fn HSLToRGB(HSL: vec3<f32>) -> vec3<f32> {
    var HSL_1: vec3<f32>;
    var RGB_4: vec3<f32>;
    var C_1: f32;

    HSL_1 = HSL;
    let _e50 = HSL_1;
    let _e52 = HueToRGB(_e50.x);
    RGB_4 = _e52;
    let _e56 = HSL_1;
    let _e63 = HSL_1;
    C_1 = ((1f - abs(((2f * _e56.z) - 1f))) * _e63.y);
    let _e67 = RGB_4;
    let _e71 = C_1;
    let _e73 = HSL_1;
    return (((_e67 - vec3(0.5f)) * _e71) + vec3(_e73.z));
}

fn e0MainImage(inputColor: vec4<f32>, uv_4: vec2<f32>, outputColor: ptr<function, vec4<f32>>) {
    var bloom: vec3<f32>;
    var features: vec3<f32>;

    let _e51 = textureSample(e0BloomBuffer_t, e0BloomBuffer_s, uv_4);
    bloom = _e51.xyz;
    let _e54 = textureSample(e0FeaturesBuffer_t, e0FeaturesBuffer_s, uv_4);
    features = _e54.xyz;
    let _e58 = bloom;
    let _e59 = features;
    let _e61 = global.e0Intensity;
    let _e63 = (inputColor.xyz + ((_e58 + _e59) * _e61));
    (*outputColor) = vec4<f32>(_e63.x, _e63.y, _e63.z, inputColor.w);
    return;
}

fn blend23_(dst: vec4<f32>, src: vec4<f32>, opacity: f32) -> vec4<f32> {
    var dst_1: vec4<f32>;
    var src_1: vec4<f32>;
    var opacity_1: f32;

    dst_1 = dst;
    src_1 = src;
    opacity_1 = opacity;
    let _e54 = dst_1;
    let _e55 = src_1;
    let _e56 = opacity_1;
    return mix(_e54, _e55, vec4(_e56));
}

fn main_1() {
    var color0_: vec4<f32>;
    var color1_: vec4<f32> = vec4(0f);

    let _e48 = vUv_1;
    let _e49 = textureSample(inputBuffer_t, inputBuffer_s, _e48);
    color0_ = _e49;
    let _e54 = color0_;
    let _e55 = vUv_1;
    e0MainImage(_e54, _e55, (&color1_));
    let _e58 = color0_;
    let _e59 = color1_;
    let _e60 = global.e0BlendOpacity;
    let _e61 = blend23_(_e58, _e59, _e60);
    color0_ = _e61;
    let _e63 = color0_;
    color0_.w = clamp(_e63.w, 0f, 1f);
    let _e68 = color0_;
    pc_fragColor = _e68;
    let _e69 = pc_fragColor;
    let _e70 = linearToOutputTexel(_e69);
    pc_fragColor = _e70;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    main_1();
    let _e69 = pc_fragColor;
    return FragmentOutput(_e69);
}
