diagnostic(off, derivative_uniformity);
struct Uniforms {
    viewMatrix: mat4x4<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    toneMappingExposure: f32,
    opacity: f32,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

const LINEAR_REC2020_TO_LINEAR_SRGB: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.6605f, -0.1246f, -0.0182f), vec3<f32>(-0.5876f, 1.1329f, -0.1006f), vec3<f32>(-0.0728f, -0.0083f, 1.1187f));
const LINEAR_SRGB_TO_LINEAR_REC2020_: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.6274f, 0.0691f, 0.0164f), vec3<f32>(0.3293f, 0.9195f, 0.088f), vec3<f32>(0.0433f, 0.0113f, 0.8956f));

var<private> pc_fragColor: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var map_t_1: texture_2d<f32>;
@group(0) @binding(2) 
var map_s_1: sampler;
var<private> vUv_1: vec2<f32>;

fn pt_isnan(v: vec3<f32>) -> vec3<bool> {
    var v_1: vec3<f32>;
    var b: vec3<u32>;

    v_1 = v;
    let _e15 = v_1;
    b = (bitcast<vec3<u32>>(_e15) & vec3(2147483647u));
    let _e21 = b;
    return (_e21 > vec3(2139095040u));
}

fn pt_isinf(v_2: vec3<f32>) -> vec3<bool> {
    var v_3: vec3<f32>;
    var b_1: vec3<u32>;

    v_3 = v_2;
    let _e15 = v_3;
    b_1 = (bitcast<vec3<u32>>(_e15) & vec3(2147483647u));
    let _e21 = b_1;
    return (_e21 == vec3(2139095040u));
}

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e17 = y_1;
    let _e18 = x_1;
    return exp2((_e17 * log2(_e18)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e17 = y_3;
    let _e18 = x_3;
    return exp2((_e17 * log2(_e18)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e17 = y_5;
    let _e18 = x_5;
    return exp2((_e17 * log2(_e18)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e17 = y_7;
    let _e18 = x_7;
    return exp2((_e17 * log2(_e18)));
}

fn LinearToneMapping(color: vec3<f32>) -> vec3<f32> {
    var color_1: vec3<f32>;

    color_1 = color;
    let _e15 = global.toneMappingExposure;
    let _e16 = color_1;
    return clamp((_e15 * _e16), vec3(0f), vec3(1f));
}

fn ReinhardToneMapping(color_2: vec3<f32>) -> vec3<f32> {
    var color_3: vec3<f32>;

    color_3 = color_2;
    let _e15 = color_3;
    let _e16 = global.toneMappingExposure;
    color_3 = (_e15 * _e16);
    let _e18 = color_3;
    let _e21 = color_3;
    return clamp((_e18 / (vec3(1f) + _e21)), vec3(0f), vec3(1f));
}

fn CineonToneMapping(color_4: vec3<f32>) -> vec3<f32> {
    var color_5: vec3<f32>;

    color_5 = color_4;
    let _e15 = color_5;
    let _e16 = global.toneMappingExposure;
    color_5 = (_e15 * _e16);
    let _e20 = color_5;
    color_5 = max(vec3(0f), (_e20 - vec3(0.004f)));
    let _e25 = color_5;
    let _e27 = color_5;
    let _e33 = color_5;
    let _e35 = color_5;
    let _e47 = pt_pow_2(((_e25 * ((6.2f * _e27) + vec3(0.5f))) / ((_e33 * ((6.2f * _e35) + vec3(1.7f))) + vec3(0.06f))), vec3(2.2f));
    return _e47;
}

fn RRTAndODTFit(v_4: vec3<f32>) -> vec3<f32> {
    var v_5: vec3<f32>;
    var a: vec3<f32>;
    var b_2: vec3<f32>;

    v_5 = v_4;
    let _e15 = v_5;
    let _e16 = v_5;
    a = ((_e15 * (_e16 + vec3(0.0245786f))) - vec3(0.000090537f));
    let _e25 = v_5;
    let _e27 = v_5;
    b_2 = ((_e25 * ((0.983729f * _e27) + vec3(0.432951f))) + vec3(0.238081f));
    let _e37 = a;
    let _e38 = b_2;
    return (_e37 / _e38);
}

fn ACESFilmicToneMapping(color_6: vec3<f32>) -> vec3<f32> {
    var color_7: vec3<f32>;
    var ACESInputMat: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.59719f, 0.076f, 0.0284f), vec3<f32>(0.35458f, 0.90834f, 0.13383f), vec3<f32>(0.04823f, 0.01566f, 0.83777f));
    var ACESOutputMat: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.60475f, -0.10208f, -0.00327f), vec3<f32>(-0.53108f, 1.10813f, -0.07276f), vec3<f32>(-0.07367f, -0.00605f, 1.07602f));

    color_7 = color_6;
    let _e55 = color_7;
    let _e56 = global.toneMappingExposure;
    color_7 = (_e55 * (_e56 / 0.6f));
    let _e60 = ACESInputMat;
    let _e61 = color_7;
    color_7 = (_e60 * _e61);
    let _e63 = color_7;
    let _e64 = RRTAndODTFit(_e63);
    color_7 = _e64;
    let _e65 = ACESOutputMat;
    let _e66 = color_7;
    color_7 = (_e65 * _e66);
    let _e68 = color_7;
    return clamp(_e68, vec3(0f), vec3(1f));
}

fn agxDefaultContrastApprox(x_8: vec3<f32>) -> vec3<f32> {
    var x_9: vec3<f32>;
    var x2_: vec3<f32>;
    var x4_: vec3<f32>;

    x_9 = x_8;
    let _e17 = x_9;
    let _e18 = x_9;
    x2_ = (_e17 * _e18);
    let _e21 = x2_;
    let _e22 = x2_;
    x4_ = (_e21 * _e22);
    let _e26 = x4_;
    let _e28 = x2_;
    let _e31 = x4_;
    let _e33 = x_9;
    let _e37 = x4_;
    let _e41 = x2_;
    let _e43 = x_9;
    let _e47 = x2_;
    let _e51 = x_9;
    return ((((((((15.5f * _e26) * _e28) - ((40.14f * _e31) * _e33)) + (31.96f * _e37)) - ((6.868f * _e41) * _e43)) + (0.4298f * _e47)) + (0.1191f * _e51)) - vec3(0.00232f));
}

fn AgXToneMapping(color_8: vec3<f32>) -> vec3<f32> {
    var color_9: vec3<f32>;
    var AgXInsetMatrix: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.85662717f, 0.13731897f, 0.11189821f), vec3<f32>(0.09512124f, 0.761242f, 0.076799415f), vec3<f32>(0.048251607f, 0.10143904f, 0.81130236f));
    var AgXOutsetMatrix: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.1271006f, -0.14132977f, -0.14132977f), vec3<f32>(-0.11060664f, 1.1578237f, -0.11060664f), vec3<f32>(-0.016493939f, -0.016493939f, 1.2519364f));
    var AgxMinEv: f32 = -12.47393f;
    var AgxMaxEv: f32 = 4.026069f;

    color_9 = color_8;
    let _e62 = color_9;
    let _e63 = global.toneMappingExposure;
    color_9 = (_e62 * _e63);
    let _e65 = color_9;
    color_9 = (LINEAR_SRGB_TO_LINEAR_REC2020_ * _e65);
    let _e67 = AgXInsetMatrix;
    let _e68 = color_9;
    color_9 = (_e67 * _e68);
    let _e70 = color_9;
    color_9 = max(_e70, vec3(0.0000000001f));
    let _e74 = color_9;
    color_9 = log2(_e74);
    let _e76 = color_9;
    let _e77 = AgxMinEv;
    let _e80 = AgxMaxEv;
    let _e81 = AgxMinEv;
    color_9 = ((_e76 - vec3(_e77)) / vec3((_e80 - _e81)));
    let _e85 = color_9;
    color_9 = clamp(_e85, vec3(0f), vec3(1f));
    let _e91 = color_9;
    let _e92 = agxDefaultContrastApprox(_e91);
    color_9 = _e92;
    let _e93 = AgXOutsetMatrix;
    let _e94 = color_9;
    color_9 = (_e93 * _e94);
    let _e98 = color_9;
    let _e102 = pt_pow_2(max(vec3(0f), _e98), vec3(2.2f));
    color_9 = _e102;
    let _e103 = color_9;
    color_9 = (LINEAR_REC2020_TO_LINEAR_SRGB * _e103);
    let _e105 = color_9;
    color_9 = clamp(_e105, vec3(0f), vec3(1f));
    let _e111 = color_9;
    return _e111;
}

fn NeutralToneMapping(color_10: vec3<f32>) -> vec3<f32> {
    var color_11: vec3<f32>;
    var StartCompression: f32 = 0.76f;
    var Desaturation: f32 = 0.15f;
    var x_10: f32;
    var local: f32;
    var offset: f32;
    var peak: f32;
    var d: f32;
    var newPeak: f32;
    var g: f32;

    color_11 = color_10;
    let _e23 = color_11;
    let _e24 = global.toneMappingExposure;
    color_11 = (_e23 * _e24);
    let _e26 = color_11;
    let _e28 = color_11;
    let _e30 = color_11;
    x_10 = min(_e26.x, min(_e28.y, _e30.z));
    let _e35 = x_10;
    if (_e35 < 0.08f) {
        let _e38 = x_10;
        let _e40 = x_10;
        let _e42 = x_10;
        local = (_e38 - ((6.25f * _e40) * _e42));
    } else {
        local = 0.04f;
    }
    let _e47 = local;
    offset = _e47;
    let _e49 = color_11;
    let _e50 = offset;
    color_11 = (_e49 - vec3(_e50));
    let _e53 = color_11;
    let _e55 = color_11;
    let _e57 = color_11;
    peak = max(_e53.x, max(_e55.y, _e57.z));
    let _e62 = peak;
    let _e63 = StartCompression;
    if (_e62 < _e63) {
        let _e65 = color_11;
        return _e65;
    }
    let _e67 = StartCompression;
    d = (1f - _e67);
    let _e71 = d;
    let _e72 = d;
    let _e74 = peak;
    let _e75 = d;
    let _e77 = StartCompression;
    newPeak = (1f - ((_e71 * _e72) / ((_e74 + _e75) - _e77)));
    let _e82 = color_11;
    let _e83 = newPeak;
    let _e84 = peak;
    color_11 = (_e82 * (_e83 / _e84));
    let _e89 = Desaturation;
    let _e90 = peak;
    let _e91 = newPeak;
    g = (1f - (1f / ((_e89 * (_e90 - _e91)) + 1f)));
    let _e99 = color_11;
    let _e100 = newPeak;
    let _e102 = g;
    return mix(_e99, vec3(_e100), vec3(_e102));
}

fn CustomToneMapping(color_12: vec3<f32>) -> vec3<f32> {
    var color_13: vec3<f32>;

    color_13 = color_12;
    let _e17 = color_13;
    return _e17;
}

fn toneMapping(color_14: vec3<f32>) -> vec3<f32> {
    var color_15: vec3<f32>;

    color_15 = color_14;
    let _e17 = color_15;
    let _e18 = ACESFilmicToneMapping(_e17);
    return _e18;
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e17 = value_1;
    return _e17;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e17 = value_3;
    let _e26 = pt_pow_2(((_e17.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e27 = value_3;
    let _e31 = value_3;
    let _e41 = mix(_e26, (_e27.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e31.xyz <= vec3(0.04045f))));
    let _e42 = value_3;
    return vec4<f32>(_e41.x, _e41.y, _e41.z, _e42.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e17 = value_5;
    let _e21 = pt_pow_2(_e17.xyz, vec3(0.41666f));
    let _e27 = value_5;
    let _e31 = value_5;
    let _e41 = mix(((_e21 * 1.055f) - vec3(0.055f)), (_e27.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e31.xyz <= vec3(0.0031308f))));
    let _e42 = value_5;
    return vec4<f32>(_e41.x, _e41.y, _e41.z, _e42.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e17 = value_7;
    let _e35 = (_e17.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e36 = value_7;
    let _e42 = sRGBTransferOETF(vec4<f32>(_e35.x, _e35.y, _e35.z, _e36.w));
    return _e42;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e22 = weights;
    let _e23 = rgb_1;
    return dot(_e22, _e23);
}

fn clampedTexelFatch(map_t: texture_2d<f32>, map_s: sampler, px: vec2<i32>, lod: i32) -> vec4<f32> {
    var px_1: vec2<i32>;
    var lod_1: i32;
    var res: vec4<f32>;

    px_1 = px;
    lod_1 = lod;
    let _e22 = px_1;
    let _e24 = px_1;
    let _e28 = textureLoad(map_t, vec2<i32>(_e22.x, _e24.y), 0i);
    res = _e28;
    let _e30 = res;
    let _e31 = linearToOutputTexel(_e30);
    return _e31;
}

fn main_1() {
    var size: vec2<f32>;
    var pxUv: vec2<f32>;
    var pxCurr: vec2<f32>;
    var pxFrac: vec2<f32>;
    var pxOffset: vec2<f32>;
    var local_1: f32;
    var local_2: f32;
    var pxNext: vec2<f32>;
    var alpha: vec2<f32>;
    var p1_: vec4<f32>;
    var p2_: vec4<f32>;

    let _e17 = textureDimensions(map_t_1, 0i);
    size = vec2<f32>(vec2<i32>(_e17));
    let _e21 = vUv_1;
    let _e22 = size;
    pxUv = (_e21 * _e22);
    let _e25 = pxUv;
    pxCurr = floor(_e25);
    let _e28 = pxUv;
    pxFrac = (fract(_e28) - vec2(0.5f));
    let _e36 = pxFrac;
    if (_e36.x > 0f) {
        local_1 = 1f;
    } else {
        local_1 = -1f;
    }
    let _e44 = local_1;
    pxOffset.x = _e44;
    let _e46 = pxFrac;
    if (_e46.y > 0f) {
        local_2 = 1f;
    } else {
        local_2 = -1f;
    }
    let _e54 = local_2;
    pxOffset.y = _e54;
    let _e55 = pxOffset;
    let _e56 = pxCurr;
    let _e60 = size;
    pxNext = clamp((_e55 + _e56), vec2(0f), (_e60 - vec2(1f)));
    let _e66 = pxFrac;
    alpha = abs(_e66);
    let _e69 = pxCurr;
    let _e71 = pxCurr;
    let _e77 = clampedTexelFatch(map_t_1, map_s_1, vec2<i32>(i32(_e69.x), i32(_e71.y)), 0i);
    let _e78 = pxNext;
    let _e80 = pxCurr;
    let _e86 = clampedTexelFatch(map_t_1, map_s_1, vec2<i32>(i32(_e78.x), i32(_e80.y)), 0i);
    let _e87 = alpha;
    p1_ = mix(_e77, _e86, vec4(_e87.x));
    let _e92 = pxCurr;
    let _e94 = pxNext;
    let _e100 = clampedTexelFatch(map_t_1, map_s_1, vec2<i32>(i32(_e92.x), i32(_e94.y)), 0i);
    let _e101 = pxNext;
    let _e103 = pxNext;
    let _e109 = clampedTexelFatch(map_t_1, map_s_1, vec2<i32>(i32(_e101.x), i32(_e103.y)), 0i);
    let _e110 = alpha;
    p2_ = mix(_e100, _e109, vec4(_e110.x));
    let _e115 = p1_;
    let _e116 = p2_;
    let _e117 = alpha;
    pc_fragColor = mix(_e115, _e116, vec4(_e117.y));
    let _e122 = pc_fragColor;
    let _e124 = global.opacity;
    pc_fragColor.w = (_e122.w * _e124);
    let _e126 = pc_fragColor;
    let _e128 = pc_fragColor;
    let _e130 = pc_fragColor;
    let _e132 = (_e128.xyz * _e130.w);
    pc_fragColor.x = _e132.x;
    pc_fragColor.y = _e132.y;
    pc_fragColor.z = _e132.z;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    main_1();
    let _e25 = pc_fragColor;
    return FragmentOutput(_e25);
}
