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
    texelSize: vec2<f32>,
    thresholdLevel: f32,
    thresholdRange: f32,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var inputBuffer_t: texture_2d<f32>;
@group(0) @binding(2) 
var inputBuffer_s: sampler;
var<private> vCenterUv1_1: vec2<f32>;
var<private> vCenterUv2_1: vec2<f32>;
var<private> vCenterUv3_1: vec2<f32>;
var<private> vCenterUv4_1: vec2<f32>;
var<private> vRowUv1_1: vec2<f32>;
var<private> vRowUv2_1: vec2<f32>;
var<private> vRowUv3_1: vec2<f32>;
var<private> vRowUv4_1: vec2<f32>;
var<private> vRowUv5_1: vec2<f32>;
var<private> vRowUv6_1: vec2<f32>;
var<private> vRowUv7_1: vec2<f32>;
var<private> vRowUv8_1: vec2<f32>;
var<private> vRowUv9_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e26 = y_1;
    let _e27 = x_1;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e26 = y_3;
    let _e27 = x_3;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e26 = y_5;
    let _e27 = x_5;
    return exp2((_e26 * log2(_e27)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e26 = y_7;
    let _e27 = x_7;
    return exp2((_e26 * log2(_e27)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e25 = x_9;
    (*i) = trunc(_e25);
    let _e27 = x_9;
    let _e28 = (*i);
    return (_e27 - _e28);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e25 = x_11;
    (*i_1) = trunc(_e25);
    let _e27 = x_11;
    let _e28 = (*i_1);
    return (_e27 - _e28);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e25 = x_13;
    (*i_2) = trunc(_e25);
    let _e27 = x_13;
    let _e28 = (*i_2);
    return (_e27 - _e28);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e25 = x_15;
    (*i_3) = trunc(_e25);
    let _e27 = x_15;
    let _e28 = (*i_3);
    return (_e27 - _e28);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e24 = v_1;
    b = (bitcast<u32>(_e24) & 2147483647u);
    let _e29 = b;
    return (_e29 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e24 = v_3;
    b_1 = (bitcast<u32>(_e24) & 2147483647u);
    let _e29 = b_1;
    return (_e29 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e24 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e24) & vec2(2147483647u));
    let _e30 = b_2;
    return (_e30 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e24 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e24) & vec3(2147483647u));
    let _e30 = b_3;
    return (_e30 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e24 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e24) & vec4(2147483647u));
    let _e30 = b_4;
    return (_e30 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e24 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e24) & vec2(2147483647u));
    let _e30 = b_5;
    return (_e30 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e24 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e24) & vec3(2147483647u));
    let _e30 = b_6;
    return (_e30 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e24 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e24) & vec4(2147483647u));
    let _e30 = b_7;
    return (_e30 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e38 = value_1;
    return _e38;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e38 = value_3;
    let _e47 = pt_pow_2(((_e38.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e48 = value_3;
    let _e52 = value_3;
    let _e62 = mix(_e47, (_e48.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e52.xyz <= vec3(0.04045f))));
    let _e63 = value_3;
    return vec4<f32>(_e62.x, _e62.y, _e62.z, _e63.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e38 = value_5;
    let _e42 = pt_pow_2(_e38.xyz, vec3(0.41666f));
    let _e48 = value_5;
    let _e52 = value_5;
    let _e62 = mix(((_e42 * 1.055f) - vec3(0.055f)), (_e48.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e52.xyz <= vec3(0.0031308f))));
    let _e63 = value_5;
    return vec4<f32>(_e62.x, _e62.y, _e62.z, _e63.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e38 = value_7;
    let _e56 = (_e38.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e57 = value_7;
    let _e63 = LinearTransferOETF(vec4<f32>(_e56.x, _e56.y, _e56.z, _e57.w));
    return _e63;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e43 = weights;
    let _e44 = rgb_1;
    return dot(_e43, _e44);
}

fn pow2_(x_16: f32) -> f32 {
    var x_17: f32;

    x_17 = x_16;
    let _e38 = x_17;
    let _e39 = x_17;
    return (_e38 * _e39);
}

fn pow2_1(x_18: vec3<f32>) -> vec3<f32> {
    var x_19: vec3<f32>;

    x_19 = x_18;
    let _e38 = x_19;
    let _e39 = x_19;
    return (_e38 * _e39);
}

fn pow3_(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e38 = x_21;
    let _e39 = x_21;
    let _e41 = x_21;
    return ((_e38 * _e39) * _e41);
}

fn pow4_(x_22: f32) -> f32 {
    var x_23: f32;
    var x2_: f32;

    x_23 = x_22;
    let _e38 = x_23;
    let _e39 = x_23;
    x2_ = (_e38 * _e39);
    let _e42 = x2_;
    let _e43 = x2_;
    return (_e42 * _e43);
}

fn max3_(v_16: vec3<f32>) -> f32 {
    var v_17: vec3<f32>;

    v_17 = v_16;
    let _e38 = v_17;
    let _e40 = v_17;
    let _e43 = v_17;
    return max(max(_e38.x, _e40.y), _e43.z);
}

fn average(v_18: vec3<f32>) -> f32 {
    var v_19: vec3<f32>;

    v_19 = v_18;
    let _e38 = v_19;
    return dot(_e38, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_8: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e44 = uv_1;
    let _e46 = a;
    let _e47 = b_8;
    dt = dot(_e44.xy, vec2<f32>(_e46, _e47));
    let _e51 = dt;
    sn = (_e51 - (floor((_e51 / 3.1415927f)) * 3.1415927f));
    let _e58 = sn;
    let _e60 = c;
    return fract((sin(_e58) * _e60));
}

fn precisionSafeLength(v_20: vec3<f32>) -> f32 {
    var v_21: vec3<f32>;

    v_21 = v_20;
    let _e38 = v_21;
    return length(_e38);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e40 = matrix_1;
    let _e41 = dir_1;
    return normalize((_e40 * vec4<f32>(_e41.x, _e41.y, _e41.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e40 = normal_1;
    let _e46 = viewMatrix_1;
    return normalize((vec4<f32>(_e40.x, _e40.y, _e40.z, 0f) * _e46).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e40 = dir_3;
    let _e46 = viewMatrix_3;
    return normalize((vec4<f32>(_e40.x, _e40.y, _e40.z, 0f) * _e46).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e42 = m_1[2][3];
    return (_e42 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_22: f32;

    dir_5 = dir_4;
    let _e38 = dir_5;
    let _e40 = dir_5;
    u = ((atan2(_e38.z, _e40.x) * 0.15915494f) + 0.5f);
    let _e48 = dir_5;
    v_22 = ((asin(clamp(_e48.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e60 = u;
    let _e61 = v_22;
    return vec2<f32>(_e60, _e61);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e39 = diffuseColor_1;
    return (0.31830987f * _e39);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e44 = dotVH_1;
    let _e48 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e44) - 6.98316f) * _e48));
    let _e52 = f0_1;
    let _e54 = fresnel;
    let _e57 = f90_1;
    let _e58 = fresnel;
    return ((_e52 * (1f - _e54)) + vec3((_e57 * _e58)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e44 = dotVH_3;
    let _e48 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e44) - 6.98316f) * _e48));
    let _e52 = f0_3;
    let _e54 = fresnel_1;
    let _e57 = f90_3;
    let _e58 = fresnel_1;
    return ((_e52 * (1f - _e54)) + (_e57 * _e58));
}

fn clampToBorder(uv_2: vec2<f32>) -> f32 {
    return select(0f, 1f, ((((uv_2.x >= 0f) && (uv_2.x <= 1f)) && (uv_2.y >= 0f)) && (uv_2.y <= 1f)));
}

fn main_1() {
    var color: vec3<f32>;
    var weight: vec4<f32>;
    var l: f32;
    var scale: f32;

    let _e37 = vRowUv5_1;
    let _e39 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e37));
    color = (0.125f * _e39.xyz);
    let _e44 = vRowUv1_1;
    let _e45 = clampToBorder(_e44);
    let _e46 = vRowUv3_1;
    let _e47 = clampToBorder(_e46);
    let _e48 = vRowUv7_1;
    let _e49 = clampToBorder(_e48);
    let _e50 = vRowUv9_1;
    let _e51 = clampToBorder(_e50);
    weight = (0.03125f * vec4<f32>(_e45, _e47, _e49, _e51));
    let _e55 = color;
    let _e56 = weight;
    let _e58 = vRowUv1_1;
    let _e60 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e58));
    color = (_e55 + (_e56.x * _e60.xyz));
    let _e64 = color;
    let _e65 = weight;
    let _e67 = vRowUv3_1;
    let _e69 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e67));
    color = (_e64 + (_e65.y * _e69.xyz));
    let _e73 = color;
    let _e74 = weight;
    let _e76 = vRowUv7_1;
    let _e78 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e76));
    color = (_e73 + (_e74.z * _e78.xyz));
    let _e82 = color;
    let _e83 = weight;
    let _e85 = vRowUv9_1;
    let _e87 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e85));
    color = (_e82 + (_e83.w * _e87.xyz));
    let _e92 = vRowUv2_1;
    let _e93 = clampToBorder(_e92);
    let _e94 = vRowUv4_1;
    let _e95 = clampToBorder(_e94);
    let _e96 = vRowUv6_1;
    let _e97 = clampToBorder(_e96);
    let _e98 = vRowUv8_1;
    let _e99 = clampToBorder(_e98);
    weight = (0.0625f * vec4<f32>(_e93, _e95, _e97, _e99));
    let _e102 = color;
    let _e103 = weight;
    let _e105 = vRowUv2_1;
    let _e107 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e105));
    color = (_e102 + (_e103.x * _e107.xyz));
    let _e111 = color;
    let _e112 = weight;
    let _e114 = vRowUv4_1;
    let _e116 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e114));
    color = (_e111 + (_e112.y * _e116.xyz));
    let _e120 = color;
    let _e121 = weight;
    let _e123 = vRowUv6_1;
    let _e125 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e123));
    color = (_e120 + (_e121.z * _e125.xyz));
    let _e129 = color;
    let _e130 = weight;
    let _e132 = vRowUv8_1;
    let _e134 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e132));
    color = (_e129 + (_e130.w * _e134.xyz));
    let _e139 = vRowUv2_1;
    let _e140 = clampToBorder(_e139);
    let _e141 = vRowUv4_1;
    let _e142 = clampToBorder(_e141);
    let _e143 = vRowUv6_1;
    let _e144 = clampToBorder(_e143);
    let _e145 = vRowUv8_1;
    let _e146 = clampToBorder(_e145);
    weight = (0.125f * vec4<f32>(_e140, _e142, _e144, _e146));
    let _e149 = color;
    let _e150 = weight;
    let _e152 = vCenterUv1_1;
    let _e154 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e152));
    color = (_e149 + (_e150.x * _e154.xyz));
    let _e158 = color;
    let _e159 = weight;
    let _e161 = vCenterUv2_1;
    let _e163 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e161));
    color = (_e158 + (_e159.y * _e163.xyz));
    let _e167 = color;
    let _e168 = weight;
    let _e170 = vCenterUv3_1;
    let _e172 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e170));
    color = (_e167 + (_e168.z * _e172.xyz));
    let _e176 = color;
    let _e177 = weight;
    let _e179 = vCenterUv4_1;
    let _e181 = textureSample(inputBuffer_t, inputBuffer_s, vec2<f32>(_e179));
    color = (_e176 + (_e177.w * _e181.xyz));
    let _e185 = color;
    let _e186 = pt_isnan_2(_e185);
    if any(_e186) {
        {
            pc_fragColor = vec4<f32>(0f, 0f, 0f, 1f);
            return;
        }
    }
    let _e192 = color;
    let _e193 = luminance(_e192);
    l = _e193;
    let _e195 = global.thresholdLevel;
    let _e196 = global.thresholdLevel;
    let _e197 = global.thresholdRange;
    let _e199 = l;
    scale = clamp(smoothstep(_e195, (_e196 + _e197), _e199), 0f, 1f);
    let _e205 = color;
    let _e206 = scale;
    let _e207 = (_e205 * _e206);
    pc_fragColor = vec4<f32>(_e207.x, _e207.y, _e207.z, 1f);
    return;
}

@fragment 
fn main(@location(0) vCenterUv1_: vec2<f32>, @location(1) vCenterUv2_: vec2<f32>, @location(2) vCenterUv3_: vec2<f32>, @location(3) vCenterUv4_: vec2<f32>, @location(4) vRowUv1_: vec2<f32>, @location(5) vRowUv2_: vec2<f32>, @location(6) vRowUv3_: vec2<f32>, @location(7) vRowUv4_: vec2<f32>, @location(8) vRowUv5_: vec2<f32>, @location(9) vRowUv6_: vec2<f32>, @location(10) vRowUv7_: vec2<f32>, @location(11) vRowUv8_: vec2<f32>, @location(12) vRowUv9_: vec2<f32>) -> FragmentOutput {
    vCenterUv1_1 = vCenterUv1_;
    vCenterUv2_1 = vCenterUv2_;
    vCenterUv3_1 = vCenterUv3_;
    vCenterUv4_1 = vCenterUv4_;
    vRowUv1_1 = vRowUv1_;
    vRowUv2_1 = vRowUv2_;
    vRowUv3_1 = vRowUv3_;
    vRowUv4_1 = vRowUv4_;
    vRowUv5_1 = vRowUv5_;
    vRowUv6_1 = vRowUv6_;
    vRowUv7_1 = vRowUv7_;
    vRowUv8_1 = vRowUv8_;
    vRowUv9_1 = vRowUv9_;
    main_1();
    let _e79 = pc_fragColor;
    return FragmentOutput(_e79);
}
