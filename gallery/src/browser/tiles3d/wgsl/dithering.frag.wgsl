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
var<private> vUv_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e36 = y_1;
    let _e37 = x_1;
    return exp2((_e36 * log2(_e37)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e36 = y_3;
    let _e37 = x_3;
    return exp2((_e36 * log2(_e37)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e36 = y_5;
    let _e37 = x_5;
    return exp2((_e36 * log2(_e37)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e36 = y_7;
    let _e37 = x_7;
    return exp2((_e36 * log2(_e37)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e35 = x_9;
    (*i) = trunc(_e35);
    let _e37 = x_9;
    let _e38 = (*i);
    return (_e37 - _e38);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e35 = x_11;
    (*i_1) = trunc(_e35);
    let _e37 = x_11;
    let _e38 = (*i_1);
    return (_e37 - _e38);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e35 = x_13;
    (*i_2) = trunc(_e35);
    let _e37 = x_13;
    let _e38 = (*i_2);
    return (_e37 - _e38);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e35 = x_15;
    (*i_3) = trunc(_e35);
    let _e37 = x_15;
    let _e38 = (*i_3);
    return (_e37 - _e38);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e34 = v_1;
    b = (bitcast<u32>(_e34) & 2147483647u);
    let _e39 = b;
    return (_e39 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e34 = v_3;
    b_1 = (bitcast<u32>(_e34) & 2147483647u);
    let _e39 = b_1;
    return (_e39 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e34 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e34) & vec2(2147483647u));
    let _e40 = b_2;
    return (_e40 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e34 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e34) & vec3(2147483647u));
    let _e40 = b_3;
    return (_e40 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e34 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e34) & vec4(2147483647u));
    let _e40 = b_4;
    return (_e40 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e34 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e34) & vec2(2147483647u));
    let _e40 = b_5;
    return (_e40 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e34 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e34) & vec3(2147483647u));
    let _e40 = b_6;
    return (_e40 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e34 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e34) & vec4(2147483647u));
    let _e40 = b_7;
    return (_e40 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e36 = value_1;
    return _e36;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e36 = value_3;
    let _e45 = pt_pow_2(((_e36.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e46 = value_3;
    let _e50 = value_3;
    let _e60 = mix(_e45, (_e46.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e50.xyz <= vec3(0.04045f))));
    let _e61 = value_3;
    return vec4<f32>(_e60.x, _e60.y, _e60.z, _e61.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e36 = value_5;
    let _e40 = pt_pow_2(_e36.xyz, vec3(0.41666f));
    let _e46 = value_5;
    let _e50 = value_5;
    let _e60 = mix(((_e40 * 1.055f) - vec3(0.055f)), (_e46.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e50.xyz <= vec3(0.0031308f))));
    let _e61 = value_5;
    return vec4<f32>(_e60.x, _e60.y, _e60.z, _e61.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e53 = value_7;
    let _e55 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e53.xyz);
    let _e56 = value_7;
    let _e62 = LinearTransferOETF(vec4<f32>(_e55.x, _e55.y, _e55.z, _e56.w));
    return _e62;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e41 = weights;
    let _e42 = rgb_1;
    return dot(_e41, _e42);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e36 = x_17;
    let _e37 = x_17;
    return (_e36 * _e37);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e36 = x_19;
    let _e37 = x_19;
    return (_e36 * _e37);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e36 = x_21;
    let _e37 = x_21;
    let _e39 = x_21;
    return ((_e36 * _e37) * _e39);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e36 = x_23;
    let _e37 = x_23;
    x2_ = (_e36 * _e37);
    let _e40 = x2_;
    let _e41 = x2_;
    return (_e40 * _e41);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e36 = v_17;
    let _e38 = v_17;
    let _e41 = v_17;
    return max(max(_e36.x, _e38.y), _e41.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e36 = v_19;
    return dot(_e36, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e42 = uv_1;
    let _e44 = a;
    let _e45 = b_8;
    dt = dot(_e42.xy, vec2<f32>(_e44, _e45));
    let _e49 = dt;
    sn = (_e49 - (floor((_e49 / 3.1415927f)) * 3.1415927f));
    let _e56 = sn;
    let _e58 = c;
    return fract((sin(_e56) * _e58));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e36 = v_21;
    return length(_e36);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e38 = matrix_1;
    let _e39 = dir_1;
    return normalize((_e38 * vec4<f32>(_e39.x, _e39.y, _e39.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e38 = normal_1;
    let _e44 = viewMatrix_1;
    return normalize((vec4<f32>(_e38.x, _e38.y, _e38.z, 0f) * _e44).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e38 = dir_3;
    let _e44 = viewMatrix_3;
    return normalize((vec4<f32>(_e38.x, _e38.y, _e38.z, 0f) * _e44).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e40 = m_1[2][3];
    return (_e40 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e36 = dir_5;
    let _e38 = dir_5;
    u = ((atan2(_e36.z, _e38.x) * 0.15915494f) + 0.5f);
    let _e46 = dir_5;
    v_22 = ((asin(clamp(_e46.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e58 = u;
    let _e59 = v_22;
    return vec2<f32>(_e58, _e59);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e37 = diffuseColor_1;
    return (0.31830987f * _e37);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e42 = dotVH_1;
    let _e46 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e42) - 6.98316f) * _e46));
    let _e50 = f0_1;
    let _e52 = fresnel;
    let _e55 = f90_1;
    let _e56 = fresnel;
    return ((_e50 * (1f - _e52)) + vec3((_e55 * _e56)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e42 = dotVH_3;
    let _e46 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e42) - 6.98316f) * _e46));
    let _e50 = f0_3;
    let _e52 = fresnel_1;
    let _e55 = f90_3;
    let _e56 = fresnel_1;
    return ((_e50 * (1f - _e52)) + (_e55 * _e56));
}

fn packNormalToRGB(normal_2: vec3<f32>) -> vec3<f32> {
    var normal_3: vec3<f32>;

    normal_3 = normal_2;
    let _e36 = normal_3;
    return ((normalize(_e36) * 0.5f) + vec3(0.5f));
}

fn unpackRGBToNormal(rgb_2: vec3<f32>) -> vec3<f32> {
    var rgb_3: vec3<f32>;

    rgb_3 = rgb_2;
    let _e37 = rgb_3;
    return ((2f * _e37.xyz) - vec3(1f));
}

fn packDepthToRGBA(v_23: f32) -> vec4<f32> {
    var v_24: f32;
    var vuf: f32;
    var af: f32;
    var bf: f32;
    var gf: f32;

    v_24 = v_23;
    let _e44 = v_24;
    if (_e44 <= 0f) {
        return vec4<f32>(0f, 0f, 0f, 0f);
    }
    let _e52 = v_24;
    if (_e52 >= 1f) {
        return vec4<f32>(1f, 1f, 1f, 1f);
    }
    let _e61 = v_24;
    let _e70 = pt_modf((_e61 * 16777216f), (&vuf));
    af = _e70;
    let _e72 = vuf;
    let _e76 = pt_modf((_e72 * ShiftRight8_), (&vuf));
    bf = _e76;
    let _e78 = vuf;
    let _e82 = pt_modf((_e78 * ShiftRight8_), (&vuf));
    gf = _e82;
    let _e84 = vuf;
    let _e86 = gf;
    let _e88 = bf;
    let _e90 = af;
    return vec4<f32>((_e84 * Inv255_), (_e86 * PackUpscale), (_e88 * PackUpscale), _e90);
}

fn packDepthToRGB(v_25: f32) -> vec3<f32> {
    var v_26: f32;
    var vuf_1: f32;
    var bf_1: f32;
    var gf_1: f32;

    v_26 = v_25;
    let _e44 = v_26;
    if (_e44 <= 0f) {
        return vec3<f32>(0f, 0f, 0f);
    }
    let _e51 = v_26;
    if (_e51 >= 1f) {
        return vec3<f32>(1f, 1f, 1f);
    }
    let _e59 = v_26;
    let _e68 = pt_modf((_e59 * 65536f), (&vuf_1));
    bf_1 = _e68;
    let _e70 = vuf_1;
    let _e74 = pt_modf((_e70 * ShiftRight8_), (&vuf_1));
    gf_1 = _e74;
    let _e76 = vuf_1;
    let _e78 = gf_1;
    let _e80 = bf_1;
    return vec3<f32>((_e76 * Inv255_), (_e78 * PackUpscale), _e80);
}

fn packDepthToRG(v_27: f32) -> vec2<f32> {
    var v_28: f32;
    var vuf_2: f32;
    var gf_2: f32;

    v_28 = v_27;
    let _e44 = v_28;
    if (_e44 <= 0f) {
        return vec2<f32>(0f, 0f);
    }
    let _e50 = v_28;
    if (_e50 >= 1f) {
        return vec2<f32>(1f, 1f);
    }
    let _e57 = v_28;
    let _e62 = pt_modf((_e57 * 256f), (&vuf_2));
    gf_2 = _e62;
    let _e64 = vuf_2;
    let _e66 = gf_2;
    return vec2<f32>((_e64 * Inv255_), _e66);
}

fn unpackRGBAToDepth(v_29: vec4<f32>) -> f32 {
    var v_30: vec4<f32>;

    v_30 = v_29;
    let _e44 = v_30;
    return dot(_e44, UnpackFactors4_);
}

fn unpackRGBToDepth(v_31: vec3<f32>) -> f32 {
    var v_32: vec3<f32>;

    v_32 = v_31;
    let _e44 = v_32;
    return dot(_e44, UnpackFactors3_);
}

fn unpackRGToDepth(v_33: vec2<f32>) -> f32 {
    var v_34: vec2<f32>;

    v_34 = v_33;
    let _e44 = v_34;
    let _e50 = v_34;
    return ((_e44.x * 0.99609375f) + (_e50.y * 0.00390625f));
}

fn pack2HalfToRGBA(v_35: vec2<f32>) -> vec4<f32> {
    var v_36: vec2<f32>;
    var r: vec4<f32>;

    v_36 = v_35;
    let _e44 = v_36;
    let _e46 = v_36;
    let _e51 = v_36;
    let _e53 = v_36;
    r = vec4<f32>(_e44.x, fract((_e46.x * 255f)), _e51.y, fract((_e53.y * 255f)));
    let _e60 = r;
    let _e62 = r;
    let _e67 = r;
    let _e69 = r;
    let _e71 = r;
    let _e76 = r;
    return vec4<f32>((_e60.x - (_e62.y / 255f)), _e67.y, (_e69.z - (_e71.w / 255f)), _e76.w);
}

fn unpackRGBATo2Half(v_37: vec4<f32>) -> vec2<f32> {
    var v_38: vec4<f32>;

    v_38 = v_37;
    let _e44 = v_38;
    let _e46 = v_38;
    let _e51 = v_38;
    let _e53 = v_38;
    return vec2<f32>((_e44.x + (_e46.y / 255f)), (_e51.z + (_e53.w / 255f)));
}

fn viewZToOrthographicDepth(viewZ: f32, near: f32, far: f32) -> f32 {
    var viewZ_1: f32;
    var near_1: f32;
    var far_1: f32;

    viewZ_1 = viewZ;
    near_1 = near;
    far_1 = far;
    let _e48 = viewZ_1;
    let _e49 = near_1;
    let _e51 = near_1;
    let _e52 = far_1;
    return ((_e48 + _e49) / (_e51 - _e52));
}

fn orthographicDepthToViewZ(depth: f32, near_2: f32, far_2: f32) -> f32 {
    var depth_1: f32;
    var near_3: f32;
    var far_3: f32;

    depth_1 = depth;
    near_3 = near_2;
    far_3 = far_2;
    let _e48 = depth_1;
    let _e49 = near_3;
    let _e50 = far_3;
    let _e53 = near_3;
    return ((_e48 * (_e49 - _e50)) - _e53);
}

fn viewZToPerspectiveDepth(viewZ_2: f32, near_4: f32, far_4: f32) -> f32 {
    var viewZ_3: f32;
    var near_5: f32;
    var far_5: f32;

    viewZ_3 = viewZ_2;
    near_5 = near_4;
    far_5 = far_4;
    let _e48 = near_5;
    let _e49 = viewZ_3;
    let _e51 = far_5;
    let _e53 = far_5;
    let _e54 = near_5;
    let _e56 = viewZ_3;
    return (((_e48 + _e49) * _e51) / ((_e53 - _e54) * _e56));
}

fn perspectiveDepthToViewZ(depth_2: f32, near_6: f32, far_6: f32) -> f32 {
    var depth_3: f32;
    var near_7: f32;
    var far_7: f32;

    depth_3 = depth_2;
    near_7 = near_6;
    far_7 = far_6;
    let _e48 = near_7;
    let _e49 = far_7;
    let _e51 = far_7;
    let _e52 = near_7;
    let _e54 = depth_3;
    let _e56 = far_7;
    return ((_e48 * _e49) / (((_e51 - _e52) * _e54) - _e56));
}

fn sRGBToLinear(value_8: vec4<f32>) -> vec4<f32> {
    var value_9: vec4<f32>;

    value_9 = value_8;
    let _e44 = value_9;
    let _e53 = pt_pow_2(((_e44.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e54 = value_9;
    let _e58 = value_9;
    let _e68 = mix(_e53, (_e54.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e58.xyz <= vec3(0.04045f))));
    let _e69 = value_9;
    return vec4<f32>(_e68.x, _e68.y, _e68.z, _e69.w);
}

fn readDepth(uv_2: vec2<f32>) -> f32 {
    var uv_3: vec2<f32>;
    var depth_4: f32;

    uv_3 = uv_2;
    let _e44 = uv_3;
    let _e45 = textureSample(depthBuffer_t, depthBuffer_s, _e44);
    depth_4 = _e45.x;
    let _e48 = depth_4;
    return _e48;
}

fn getViewZ(depth_5: f32) -> f32 {
    var depth_6: f32;

    depth_6 = depth_5;
    let _e44 = depth_6;
    let _e45 = global.cameraNear;
    let _e46 = global.cameraFar;
    let _e47 = perspectiveDepthToViewZ(_e44, _e45, _e46);
    return _e47;
}

fn RGBToHCV(RGB: vec3<f32>) -> vec3<f32> {
    var RGB_1: vec3<f32>;
    var P: vec4<f32>;
    var Q: vec4<f32>;
    var C: f32;
    var H: f32;

    RGB_1 = RGB;
    let _e44 = RGB_1;
    let _e45 = _e44.zy;
    let _e54 = RGB_1;
    let _e55 = _e54.yz;
    let _e64 = RGB_1;
    let _e66 = RGB_1;
    P = mix(vec4<f32>(_e45.x, _e45.y, -1f, 0.6666667f), vec4<f32>(_e55.x, _e55.y, 0f, -0.33333334f), vec4(step(_e64.z, _e66.y)));
    let _e72 = P;
    let _e73 = _e72.xyw;
    let _e74 = RGB_1;
    let _e80 = RGB_1;
    let _e82 = P;
    let _e83 = _e82.yzx;
    let _e88 = P;
    let _e90 = RGB_1;
    Q = mix(vec4<f32>(_e73.x, _e73.y, _e73.z, _e74.x), vec4<f32>(_e80.x, _e83.x, _e83.y, _e83.z), vec4(step(_e88.x, _e90.x)));
    let _e96 = Q;
    let _e98 = Q;
    let _e100 = Q;
    C = (_e96.x - min(_e98.w, _e100.y));
    let _e105 = Q;
    let _e107 = Q;
    let _e111 = C;
    let _e116 = Q;
    H = abs((((_e105.w - _e107.y) / ((6f * _e111) + 0.000001f)) + _e116.z));
    let _e121 = H;
    let _e122 = C;
    let _e123 = Q;
    return vec3<f32>(_e121, _e122, _e123.x);
}

fn RGBToHSL(RGB_2: vec3<f32>) -> vec3<f32> {
    var RGB_3: vec3<f32>;
    var HCV: vec3<f32>;
    var L: f32;
    var S: f32;

    RGB_3 = RGB_2;
    let _e44 = RGB_3;
    let _e45 = RGBToHCV(_e44);
    HCV = _e45;
    let _e47 = HCV;
    let _e49 = HCV;
    L = (_e47.z - (_e49.y * 0.5f));
    let _e55 = HCV;
    let _e58 = L;
    S = (_e55.y / ((1f - abs(((_e58 * 2f) - 1f))) + 0.000001f));
    let _e69 = HCV;
    let _e71 = S;
    let _e72 = L;
    return vec3<f32>(_e69.x, _e71, _e72);
}

fn HueToRGB(H_1: f32) -> vec3<f32> {
    var H_2: f32;
    var R: f32;
    var G: f32;
    var B: f32;

    H_2 = H_1;
    let _e44 = H_2;
    R = (abs(((_e44 * 6f) - 3f)) - 1f);
    let _e54 = H_2;
    G = (2f - abs(((_e54 * 6f) - 2f)));
    let _e63 = H_2;
    B = (2f - abs(((_e63 * 6f) - 4f)));
    let _e71 = R;
    let _e72 = G;
    let _e73 = B;
    return clamp(vec3<f32>(_e71, _e72, _e73), vec3(0f), vec3(1f));
}

fn HSLToRGB(HSL: vec3<f32>) -> vec3<f32> {
    var HSL_1: vec3<f32>;
    var RGB_4: vec3<f32>;
    var C_1: f32;

    HSL_1 = HSL;
    let _e44 = HSL_1;
    let _e46 = HueToRGB(_e44.x);
    RGB_4 = _e46;
    let _e50 = HSL_1;
    let _e57 = HSL_1;
    C_1 = ((1f - abs(((2f * _e50.z) - 1f))) * _e57.y);
    let _e61 = RGB_4;
    let _e65 = C_1;
    let _e67 = HSL_1;
    return (((_e61 - vec3(0.5f)) * _e65) + vec3(_e67.z));
}

fn dithering(color: vec3<f32>) -> vec3<f32> {
    var color_1: vec3<f32>;
    var grid_position: f32;
    var dither_shift_RGB: vec3<f32> = vec3<f32>(0.0009803922f, -0.0009803922f, 0.0009803922f);

    color_1 = color;
    let _e45 = gl_FragCoord_1;
    let _e47 = rand(_e45.xy);
    grid_position = _e47;
    let _e62 = dither_shift_RGB;
    let _e66 = dither_shift_RGB;
    let _e68 = grid_position;
    dither_shift_RGB = mix((2f * _e62), (-2f * _e66), vec3(_e68));
    let _e71 = color_1;
    let _e72 = dither_shift_RGB;
    return (_e71 + _e72);
}

fn e0MainImage(inputColor: vec4<f32>, uv_4: vec2<f32>, outputColor: ptr<function, vec4<f32>>) {
    let _e47 = dithering(inputColor.xyz);
    let _e52 = clamp(_e47, vec3(0f), vec3(1f));
    (*outputColor) = vec4<f32>(_e52.x, _e52.y, _e52.z, inputColor.w);
    return;
}

fn blend23_(dst: vec4<f32>, src: vec4<f32>, opacity: f32) -> vec4<f32> {
    var dst_1: vec4<f32>;
    var src_1: vec4<f32>;
    var opacity_1: f32;

    dst_1 = dst;
    src_1 = src;
    opacity_1 = opacity;
    let _e49 = dst_1;
    let _e50 = src_1;
    let _e51 = opacity_1;
    return mix(_e49, _e50, vec4(_e51));
}

fn main_1() {
    var color0_: vec4<f32>;
    var color1_: vec4<f32> = vec4(0f);

    let _e43 = vUv_1;
    let _e44 = textureSample(inputBuffer_t, inputBuffer_s, _e43);
    color0_ = _e44;
    let _e49 = color0_;
    let _e50 = vUv_1;
    e0MainImage(_e49, _e50, (&color1_));
    let _e53 = color0_;
    let _e54 = color1_;
    let _e55 = global.e0BlendOpacity;
    let _e56 = blend23_(_e53, _e54, _e55);
    color0_ = _e56;
    let _e58 = color0_;
    color0_.w = clamp(_e58.w, 0f, 1f);
    let _e63 = color0_;
    pc_fragColor = _e63;
    let _e64 = pc_fragColor;
    let _e65 = linearToOutputTexel(_e64);
    pc_fragColor = _e65;
    let _e66 = pc_fragColor;
    let _e68 = pc_fragColor;
    let _e70 = dithering(_e68.xyz);
    pc_fragColor.x = _e70.x;
    pc_fragColor.y = _e70.y;
    pc_fragColor.z = _e70.z;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e61 = pc_fragColor;
    return FragmentOutput(_e61);
}
