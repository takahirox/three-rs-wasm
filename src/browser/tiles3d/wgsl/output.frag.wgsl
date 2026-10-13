diagnostic(off, derivative_uniformity);
struct Uniforms {
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    toneMappingExposure: f32,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

const LINEAR_REC2020_TO_LINEAR_SRGB: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.6605f, -0.1246f, -0.0182f), vec3<f32>(-0.5876f, 1.1329f, -0.1006f), vec3<f32>(-0.0728f, -0.0083f, 1.1187f));
const LINEAR_SRGB_TO_LINEAR_REC2020_: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.6274f, 0.0691f, 0.0164f), vec3<f32>(0.3293f, 0.9195f, 0.088f), vec3<f32>(0.0433f, 0.0113f, 0.8956f));

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var tDiffuse_t: texture_2d<f32>;
@group(0) @binding(2) 
var tDiffuse_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e12 = y_1;
    let _e13 = x_1;
    return exp2((_e12 * log2(_e13)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e12 = y_3;
    let _e13 = x_3;
    return exp2((_e12 * log2(_e13)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e12 = y_5;
    let _e13 = x_5;
    return exp2((_e12 * log2(_e13)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e12 = y_7;
    let _e13 = x_7;
    return exp2((_e12 * log2(_e13)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e11 = x_9;
    (*i) = trunc(_e11);
    let _e13 = x_9;
    let _e14 = (*i);
    return (_e13 - _e14);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e11 = x_11;
    (*i_1) = trunc(_e11);
    let _e13 = x_11;
    let _e14 = (*i_1);
    return (_e13 - _e14);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e11 = x_13;
    (*i_2) = trunc(_e11);
    let _e13 = x_13;
    let _e14 = (*i_2);
    return (_e13 - _e14);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e11 = x_15;
    (*i_3) = trunc(_e11);
    let _e13 = x_15;
    let _e14 = (*i_3);
    return (_e13 - _e14);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e10 = v_1;
    b = (bitcast<u32>(_e10) & 2147483647u);
    let _e15 = b;
    return (_e15 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e10 = v_3;
    b_1 = (bitcast<u32>(_e10) & 2147483647u);
    let _e15 = b_1;
    return (_e15 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e10 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e10) & vec2(2147483647u));
    let _e16 = b_2;
    return (_e16 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e10 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e10) & vec3(2147483647u));
    let _e16 = b_3;
    return (_e16 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e10 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e10) & vec4(2147483647u));
    let _e16 = b_4;
    return (_e16 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e10 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e10) & vec2(2147483647u));
    let _e16 = b_5;
    return (_e16 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e10 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e10) & vec3(2147483647u));
    let _e16 = b_6;
    return (_e16 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e10 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e10) & vec4(2147483647u));
    let _e16 = b_7;
    return (_e16 == vec4(2139095040u));
}

fn LinearToneMapping(color: vec3<f32>) -> vec3<f32> {
    var color_1: vec3<f32>;

    color_1 = color;
    let _e12 = global.toneMappingExposure;
    let _e13 = color_1;
    return clamp((_e12 * _e13), vec3(0f), vec3(1f));
}

fn ReinhardToneMapping(color_2: vec3<f32>) -> vec3<f32> {
    var color_3: vec3<f32>;

    color_3 = color_2;
    let _e12 = color_3;
    let _e13 = global.toneMappingExposure;
    color_3 = (_e12 * _e13);
    let _e15 = color_3;
    let _e18 = color_3;
    return clamp((_e15 / (vec3(1f) + _e18)), vec3(0f), vec3(1f));
}

fn CineonToneMapping(color_4: vec3<f32>) -> vec3<f32> {
    var color_5: vec3<f32>;

    color_5 = color_4;
    let _e12 = color_5;
    let _e13 = global.toneMappingExposure;
    color_5 = (_e12 * _e13);
    let _e17 = color_5;
    color_5 = max(vec3(0f), (_e17 - vec3(0.004f)));
    let _e22 = color_5;
    let _e24 = color_5;
    let _e30 = color_5;
    let _e32 = color_5;
    let _e44 = pt_pow_2(((_e22 * ((6.2f * _e24) + vec3(0.5f))) / ((_e30 * ((6.2f * _e32) + vec3(1.7f))) + vec3(0.06f))), vec3(2.2f));
    return _e44;
}

fn RRTAndODTFit(v_16: vec3<f32>) -> vec3<f32> {
    var v_17: vec3<f32>;
    var a: vec3<f32>;
    var b_8: vec3<f32>;

    v_17 = v_16;
    let _e12 = v_17;
    let _e13 = v_17;
    a = ((_e12 * (_e13 + vec3(0.0245786f))) - vec3(0.000090537f));
    let _e22 = v_17;
    let _e24 = v_17;
    b_8 = ((_e22 * ((0.983729f * _e24) + vec3(0.432951f))) + vec3(0.238081f));
    let _e34 = a;
    let _e35 = b_8;
    return (_e34 / _e35);
}

fn ACESFilmicToneMapping(color_6: vec3<f32>) -> vec3<f32> {
    var color_7: vec3<f32>;
    var ACESInputMat: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.59719f, 0.076f, 0.0284f), vec3<f32>(0.35458f, 0.90834f, 0.13383f), vec3<f32>(0.04823f, 0.01566f, 0.83777f));
    var ACESOutputMat: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.60475f, -0.10208f, -0.00327f), vec3<f32>(-0.53108f, 1.10813f, -0.07276f), vec3<f32>(-0.07367f, -0.00605f, 1.07602f));

    color_7 = color_6;
    let _e52 = color_7;
    let _e53 = global.toneMappingExposure;
    color_7 = (_e52 * (_e53 / 0.6f));
    let _e57 = ACESInputMat;
    let _e58 = color_7;
    color_7 = (_e57 * _e58);
    let _e60 = color_7;
    let _e61 = RRTAndODTFit(_e60);
    color_7 = _e61;
    let _e62 = ACESOutputMat;
    let _e63 = color_7;
    color_7 = (_e62 * _e63);
    let _e65 = color_7;
    return clamp(_e65, vec3(0f), vec3(1f));
}

fn agxDefaultContrastApprox(x_16: vec3<f32>) -> vec3<f32> {
    var x_17: vec3<f32>;
    var x2_: vec3<f32>;
    var x4_: vec3<f32>;

    x_17 = x_16;
    let _e14 = x_17;
    let _e15 = x_17;
    x2_ = (_e14 * _e15);
    let _e18 = x2_;
    let _e19 = x2_;
    x4_ = (_e18 * _e19);
    let _e23 = x4_;
    let _e25 = x2_;
    let _e28 = x4_;
    let _e30 = x_17;
    let _e34 = x4_;
    let _e38 = x2_;
    let _e40 = x_17;
    let _e44 = x2_;
    let _e48 = x_17;
    return ((((((((15.5f * _e23) * _e25) - ((40.14f * _e28) * _e30)) + (31.96f * _e34)) - ((6.868f * _e38) * _e40)) + (0.4298f * _e44)) + (0.1191f * _e48)) - vec3(0.00232f));
}

fn AgXToneMapping(color_8: vec3<f32>) -> vec3<f32> {
    var color_9: vec3<f32>;
    var AgXInsetMatrix: mat3x3<f32> = mat3x3<f32>(vec3<f32>(0.85662717f, 0.13731897f, 0.11189821f), vec3<f32>(0.09512124f, 0.761242f, 0.076799415f), vec3<f32>(0.048251607f, 0.10143904f, 0.81130236f));
    var AgXOutsetMatrix: mat3x3<f32> = mat3x3<f32>(vec3<f32>(1.1271006f, -0.14132977f, -0.14132977f), vec3<f32>(-0.11060664f, 1.1578237f, -0.11060664f), vec3<f32>(-0.016493939f, -0.016493939f, 1.2519364f));
    var AgxMinEv: f32 = -12.47393f;
    var AgxMaxEv: f32 = 4.026069f;

    color_9 = color_8;
    let _e59 = color_9;
    let _e60 = global.toneMappingExposure;
    color_9 = (_e59 * _e60);
    let _e62 = color_9;
    color_9 = (LINEAR_SRGB_TO_LINEAR_REC2020_ * _e62);
    let _e64 = AgXInsetMatrix;
    let _e65 = color_9;
    color_9 = (_e64 * _e65);
    let _e67 = color_9;
    color_9 = max(_e67, vec3(0.0000000001f));
    let _e71 = color_9;
    color_9 = log2(_e71);
    let _e73 = color_9;
    let _e74 = AgxMinEv;
    let _e77 = AgxMaxEv;
    let _e78 = AgxMinEv;
    color_9 = ((_e73 - vec3(_e74)) / vec3((_e77 - _e78)));
    let _e82 = color_9;
    color_9 = clamp(_e82, vec3(0f), vec3(1f));
    let _e88 = color_9;
    let _e89 = agxDefaultContrastApprox(_e88);
    color_9 = _e89;
    let _e90 = AgXOutsetMatrix;
    let _e91 = color_9;
    color_9 = (_e90 * _e91);
    let _e95 = color_9;
    let _e99 = pt_pow_2(max(vec3(0f), _e95), vec3(2.2f));
    color_9 = _e99;
    let _e100 = color_9;
    color_9 = (LINEAR_REC2020_TO_LINEAR_SRGB * _e100);
    let _e102 = color_9;
    color_9 = clamp(_e102, vec3(0f), vec3(1f));
    let _e108 = color_9;
    return _e108;
}

fn NeutralToneMapping(color_10: vec3<f32>) -> vec3<f32> {
    var color_11: vec3<f32>;
    var StartCompression: f32 = 0.76f;
    var Desaturation: f32 = 0.15f;
    var x_18: f32;
    var local: f32;
    var offset: f32;
    var peak: f32;
    var d: f32;
    var newPeak: f32;
    var g: f32;

    color_11 = color_10;
    let _e20 = color_11;
    let _e21 = global.toneMappingExposure;
    color_11 = (_e20 * _e21);
    let _e23 = color_11;
    let _e25 = color_11;
    let _e27 = color_11;
    x_18 = min(_e23.x, min(_e25.y, _e27.z));
    let _e32 = x_18;
    if (_e32 < 0.08f) {
        let _e35 = x_18;
        let _e37 = x_18;
        let _e39 = x_18;
        local = (_e35 - ((6.25f * _e37) * _e39));
    } else {
        local = 0.04f;
    }
    let _e44 = local;
    offset = _e44;
    let _e46 = color_11;
    let _e47 = offset;
    color_11 = (_e46 - vec3(_e47));
    let _e50 = color_11;
    let _e52 = color_11;
    let _e54 = color_11;
    peak = max(_e50.x, max(_e52.y, _e54.z));
    let _e59 = peak;
    let _e60 = StartCompression;
    if (_e59 < _e60) {
        let _e62 = color_11;
        return _e62;
    }
    let _e64 = StartCompression;
    d = (1f - _e64);
    let _e68 = d;
    let _e69 = d;
    let _e71 = peak;
    let _e72 = d;
    let _e74 = StartCompression;
    newPeak = (1f - ((_e68 * _e69) / ((_e71 + _e72) - _e74)));
    let _e79 = color_11;
    let _e80 = newPeak;
    let _e81 = peak;
    color_11 = (_e79 * (_e80 / _e81));
    let _e86 = Desaturation;
    let _e87 = peak;
    let _e88 = newPeak;
    g = (1f - (1f / ((_e86 * (_e87 - _e88)) + 1f)));
    let _e96 = color_11;
    let _e97 = newPeak;
    let _e99 = g;
    return mix(_e96, vec3(_e97), vec3(_e99));
}

fn CustomToneMapping(color_12: vec3<f32>) -> vec3<f32> {
    var color_13: vec3<f32>;

    color_13 = color_12;
    let _e14 = color_13;
    return _e14;
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e14 = value_1;
    return _e14;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e14 = value_3;
    let _e23 = pt_pow_2(((_e14.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e24 = value_3;
    let _e28 = value_3;
    let _e38 = mix(_e23, (_e24.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e28.xyz <= vec3(0.04045f))));
    let _e39 = value_3;
    return vec4<f32>(_e38.x, _e38.y, _e38.z, _e39.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e14 = value_5;
    let _e18 = pt_pow_2(_e14.xyz, vec3(0.41666f));
    let _e24 = value_5;
    let _e28 = value_5;
    let _e38 = mix(((_e18 * 1.055f) - vec3(0.055f)), (_e24.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e28.xyz <= vec3(0.0031308f))));
    let _e39 = value_5;
    return vec4<f32>(_e38.x, _e38.y, _e38.z, _e39.w);
}

fn main_1() {
    let _e12 = vUv_1;
    let _e13 = textureSample(tDiffuse_t, tDiffuse_s, _e12);
    pc_fragColor = _e13;
    let _e14 = pc_fragColor;
    let _e16 = pc_fragColor;
    let _e18 = AgXToneMapping(_e16.xyz);
    pc_fragColor.x = _e18.x;
    pc_fragColor.y = _e18.y;
    pc_fragColor.z = _e18.z;
    let _e25 = pc_fragColor;
    let _e26 = sRGBTransferOETF(_e25);
    pc_fragColor = _e26;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    main_1();
    let _e21 = pc_fragColor;
    return FragmentOutput(_e21);
}
