diagnostic(off, derivative_uniformity);
struct Uniforms {
    viewMatrix: mat4x4<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    blur: f32,
}

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

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

var<private> pc_fragColor: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var envMap_t_2: texture_2d<f32>;
@group(0) @binding(2) 
var envMap_s_2: sampler;
var<private> vUv_1: vec2<f32>;

fn pt_isnan(v: vec3<f32>) -> vec3<bool> {
    var v_1: vec3<f32>;
    var b: vec3<u32>;

    v_1 = v;
    let _e13 = v_1;
    b = (bitcast<vec3<u32>>(_e13) & vec3(2147483647u));
    let _e19 = b;
    return (_e19 > vec3(2139095040u));
}

fn pt_isinf(v_2: vec3<f32>) -> vec3<bool> {
    var v_3: vec3<f32>;
    var b_1: vec3<u32>;

    v_3 = v_2;
    let _e13 = v_3;
    b_1 = (bitcast<vec3<u32>>(_e13) & vec3(2147483647u));
    let _e19 = b_1;
    return (_e19 == vec3(2139095040u));
}

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e15 = y_1;
    let _e16 = x_1;
    return exp2((_e15 * log2(_e16)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e15 = y_3;
    let _e16 = x_3;
    return exp2((_e15 * log2(_e16)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e15 = y_5;
    let _e16 = x_5;
    return exp2((_e15 * log2(_e16)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e15 = y_7;
    let _e16 = x_7;
    return exp2((_e15 * log2(_e16)));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e13 = value_1;
    return _e13;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e13 = value_3;
    let _e22 = pt_pow_2(((_e13.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e23 = value_3;
    let _e27 = value_3;
    let _e37 = mix(_e22, (_e23.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e27.xyz <= vec3(0.04045f))));
    let _e38 = value_3;
    return vec4<f32>(_e37.x, _e37.y, _e37.z, _e38.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e13 = value_5;
    let _e17 = pt_pow_2(_e13.xyz, vec3(0.41666f));
    let _e23 = value_5;
    let _e27 = value_5;
    let _e37 = mix(((_e17 * 1.055f) - vec3(0.055f)), (_e23.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e27.xyz <= vec3(0.0031308f))));
    let _e38 = value_5;
    return vec4<f32>(_e37.x, _e37.y, _e37.z, _e38.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e13 = value_7;
    let _e31 = (_e13.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e32 = value_7;
    let _e38 = LinearTransferOETF(vec4<f32>(_e31.x, _e31.y, _e31.z, _e32.w));
    return _e38;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e18 = weights;
    let _e19 = rgb_1;
    return dot(_e18, _e19);
}

fn pow2_(x_8: f32) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e13 = x_9;
    let _e14 = x_9;
    return (_e13 * _e14);
}

fn pow2_1(x_10: vec3<f32>) -> vec3<f32> {
    var x_11: vec3<f32>;

    x_11 = x_10;
    let _e13 = x_11;
    let _e14 = x_11;
    return (_e13 * _e14);
}

fn pow3_(x_12: f32) -> f32 {
    var x_13: f32;

    x_13 = x_12;
    let _e13 = x_13;
    let _e14 = x_13;
    let _e16 = x_13;
    return ((_e13 * _e14) * _e16);
}

fn pow4_(x_14: f32) -> f32 {
    var x_15: f32;
    var x2_: f32;

    x_15 = x_14;
    let _e13 = x_15;
    let _e14 = x_15;
    x2_ = (_e13 * _e14);
    let _e17 = x2_;
    let _e18 = x2_;
    return (_e17 * _e18);
}

fn max3_(v_4: vec3<f32>) -> f32 {
    var v_5: vec3<f32>;

    v_5 = v_4;
    let _e13 = v_5;
    let _e15 = v_5;
    let _e18 = v_5;
    return max(max(_e13.x, _e15.y), _e18.z);
}

fn average(v_6: vec3<f32>) -> f32 {
    var v_7: vec3<f32>;

    v_7 = v_6;
    let _e13 = v_7;
    return dot(_e13, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a: f32 = 12.9898f;
    var b_2: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e19 = uv_1;
    let _e21 = a;
    let _e22 = b_2;
    dt = dot(_e19.xy, vec2<f32>(_e21, _e22));
    let _e26 = dt;
    sn = (_e26 - (floor((_e26 / 3.1415927f)) * 3.1415927f));
    let _e33 = sn;
    let _e35 = c;
    return fract((sin(_e33) * _e35));
}

fn precisionSafeLength(v_8: vec3<f32>) -> f32 {
    var v_9: vec3<f32>;

    v_9 = v_8;
    let _e13 = v_9;
    return length(_e13);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e15 = matrix_1;
    let _e16 = dir_1;
    return normalize((_e15 * vec4<f32>(_e16.x, _e16.y, _e16.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e15 = normal_1;
    let _e21 = viewMatrix_1;
    return normalize((vec4<f32>(_e15.x, _e15.y, _e15.z, 0f) * _e21).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e15 = dir_3;
    let _e21 = viewMatrix_3;
    return normalize((vec4<f32>(_e15.x, _e15.y, _e15.z, 0f) * _e21).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e17 = m_1[2][3];
    return (_e17 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_10: f32;

    dir_5 = dir_4;
    let _e13 = dir_5;
    let _e15 = dir_5;
    u = ((atan2(_e13.z, _e15.x) * 0.15915494f) + 0.5f);
    let _e23 = dir_5;
    v_10 = ((asin(clamp(_e23.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e35 = u;
    let _e36 = v_10;
    return vec2<f32>(_e35, _e36);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e14 = diffuseColor_1;
    return (0.31830987f * _e14);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e19 = dotVH_1;
    let _e23 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e19) - 6.98316f) * _e23));
    let _e27 = f0_1;
    let _e29 = fresnel;
    let _e32 = f90_1;
    let _e33 = fresnel;
    return ((_e27 * (1f - _e29)) + vec3((_e32 * _e33)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e19 = dotVH_3;
    let _e23 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e19) - 6.98316f) * _e23));
    let _e27 = f0_3;
    let _e29 = fresnel_1;
    let _e32 = f90_3;
    let _e33 = fresnel_1;
    return ((_e27 * (1f - _e29)) + (_e32 * _e33));
}

fn getFace(direction: vec3<f32>) -> f32 {
    var direction_1: vec3<f32>;
    var absDirection: vec3<f32>;
    var face: f32 = -1f;
    var local: f32;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;

    direction_1 = direction;
    let _e13 = direction_1;
    absDirection = abs(_e13);
    let _e19 = absDirection;
    let _e21 = absDirection;
    if (_e19.x > _e21.z) {
        {
            let _e24 = absDirection;
            let _e26 = absDirection;
            if (_e24.x > _e26.y) {
                let _e29 = direction_1;
                if (_e29.x > 0f) {
                    local = 0f;
                } else {
                    local = 3f;
                }
                let _e36 = local;
                face = _e36;
            } else {
                let _e37 = direction_1;
                if (_e37.y > 0f) {
                    local_1 = 1f;
                } else {
                    local_1 = 4f;
                }
                let _e44 = local_1;
                face = _e44;
            }
        }
    } else {
        {
            let _e45 = absDirection;
            let _e47 = absDirection;
            if (_e45.z > _e47.y) {
                let _e50 = direction_1;
                if (_e50.z > 0f) {
                    local_2 = 2f;
                } else {
                    local_2 = 5f;
                }
                let _e57 = local_2;
                face = _e57;
            } else {
                let _e58 = direction_1;
                if (_e58.y > 0f) {
                    local_3 = 1f;
                } else {
                    local_3 = 4f;
                }
                let _e65 = local_3;
                face = _e65;
            }
        }
    }
    let _e66 = face;
    return _e66;
}

fn getUV(direction_2: vec3<f32>, face_1: f32) -> vec2<f32> {
    var direction_3: vec3<f32>;
    var face_2: f32;
    var uv_2: vec2<f32>;

    direction_3 = direction_2;
    face_2 = face_1;
    let _e16 = face_2;
    if (_e16 == 0f) {
        {
            let _e19 = direction_3;
            let _e21 = direction_3;
            let _e24 = direction_3;
            uv_2 = (vec2<f32>(_e19.z, _e21.y) / vec2(abs(_e24.x)));
        }
    } else {
        let _e29 = face_2;
        if (_e29 == 1f) {
            {
                let _e32 = direction_3;
                let _e35 = direction_3;
                let _e39 = direction_3;
                uv_2 = (vec2<f32>(-(_e32.x), -(_e35.z)) / vec2(abs(_e39.y)));
            }
        } else {
            let _e44 = face_2;
            if (_e44 == 2f) {
                {
                    let _e47 = direction_3;
                    let _e50 = direction_3;
                    let _e53 = direction_3;
                    uv_2 = (vec2<f32>(-(_e47.x), _e50.y) / vec2(abs(_e53.z)));
                }
            } else {
                let _e58 = face_2;
                if (_e58 == 3f) {
                    {
                        let _e61 = direction_3;
                        let _e64 = direction_3;
                        let _e67 = direction_3;
                        uv_2 = (vec2<f32>(-(_e61.z), _e64.y) / vec2(abs(_e67.x)));
                    }
                } else {
                    let _e72 = face_2;
                    if (_e72 == 4f) {
                        {
                            let _e75 = direction_3;
                            let _e78 = direction_3;
                            let _e81 = direction_3;
                            uv_2 = (vec2<f32>(-(_e75.x), _e78.z) / vec2(abs(_e81.y)));
                        }
                    } else {
                        {
                            let _e86 = direction_3;
                            let _e88 = direction_3;
                            let _e91 = direction_3;
                            uv_2 = (vec2<f32>(_e86.x, _e88.y) / vec2(abs(_e91.z)));
                        }
                    }
                }
            }
        }
    }
    let _e97 = uv_2;
    return (0.5f * (_e97 + vec2(1f)));
}

fn bilinearCubeUV(envMap_t: texture_2d<f32>, envMap_s: sampler, direction_4: vec3<f32>, mipInt: f32) -> vec3<f32> {
    var direction_5: vec3<f32>;
    var mipInt_1: f32;
    var face_3: f32;
    var filterInt: f32;
    var faceSize: f32;
    var uv_3: vec2<f32>;

    direction_5 = direction_4;
    mipInt_1 = mipInt;
    let _e17 = direction_5;
    let _e18 = getFace(_e17);
    face_3 = _e18;
    let _e21 = mipInt_1;
    filterInt = max((4f - _e21), 0f);
    let _e26 = mipInt_1;
    mipInt_1 = max(_e26, 4f);
    let _e29 = mipInt_1;
    faceSize = exp2(_e29);
    let _e32 = direction_5;
    let _e33 = face_3;
    let _e34 = getUV(_e32, _e33);
    let _e35 = faceSize;
    uv_3 = ((_e34 * (_e35 - 2f)) + vec2(1f));
    let _e43 = face_3;
    if (_e43 > 2f) {
        {
            let _e47 = uv_3;
            let _e49 = faceSize;
            uv_3.y = (_e47.y + _e49);
            let _e51 = face_3;
            face_3 = (_e51 - 3f);
        }
    }
    let _e55 = uv_3;
    let _e57 = face_3;
    let _e58 = faceSize;
    uv_3.x = (_e55.x + (_e57 * _e58));
    let _e62 = uv_3;
    let _e64 = filterInt;
    uv_3.x = (_e62.x + ((_e64 * 3f) * 16f));
    let _e71 = uv_3;
    let _e76 = faceSize;
    uv_3.y = (_e71.y + (4f * (512f - _e76)));
    let _e81 = uv_3;
    uv_3.x = (_e81.x * 0.0006510417f);
    let _e86 = uv_3;
    uv_3.y = (_e86.y * 0.00048828125f);
    let _e90 = uv_3;
    let _e91 = textureSample(envMap_t, envMap_s, _e90);
    return _e91.xyz;
}

fn roughnessToMip(roughness: f32) -> f32 {
    var roughness_1: f32;
    var mip: f32 = 0f;

    roughness_1 = roughness;
    let _e15 = roughness_1;
    if (_e15 >= 0.8f) {
        {
            let _e19 = roughness_1;
            mip = ((((1f - _e19) * 1f) / 0.19999999f) + -2f);
        }
    } else {
        let _e34 = roughness_1;
        if (_e34 >= 0.4f) {
            {
                let _e38 = roughness_1;
                mip = ((((0.8f - _e38) * 3f) / 0.4f) + -1f);
            }
        } else {
            let _e52 = roughness_1;
            if (_e52 >= 0.305f) {
                {
                    let _e56 = roughness_1;
                    mip = ((((0.4f - _e56) * 1f) / 0.095f) + 2f);
                }
            } else {
                let _e68 = roughness_1;
                if (_e68 >= 0.21f) {
                    {
                        let _e72 = roughness_1;
                        mip = ((((0.305f - _e72) * 1f) / 0.09500001f) + 3f);
                    }
                } else {
                    {
                        let _e87 = roughness_1;
                        mip = (-2f * log2((1.16f * _e87)));
                    }
                }
            }
        }
    }
    let _e91 = mip;
    return _e91;
}

fn textureCubeUV(envMap_t_1: texture_2d<f32>, envMap_s_1: sampler, sampleDir: vec3<f32>, roughness_2: f32) -> vec4<f32> {
    var sampleDir_1: vec3<f32>;
    var roughness_3: f32;
    var mip_1: f32;
    var mipF: f32;
    var mipInt_2: f32;
    var color0_: vec3<f32>;
    var color1_: vec3<f32>;

    sampleDir_1 = sampleDir;
    roughness_3 = roughness_2;
    let _e17 = roughness_3;
    let _e18 = roughnessToMip(_e17);
    mip_1 = clamp(_e18, -2f, 9f);
    let _e24 = mip_1;
    mipF = fract(_e24);
    let _e27 = mip_1;
    mipInt_2 = floor(_e27);
    let _e30 = sampleDir_1;
    let _e31 = mipInt_2;
    let _e32 = bilinearCubeUV(envMap_t_1, envMap_s_1, _e30, _e31);
    color0_ = _e32;
    let _e34 = mipF;
    if (_e34 == 0f) {
        {
            let _e37 = color0_;
            return vec4<f32>(_e37.x, _e37.y, _e37.z, 1f);
        }
    } else {
        {
            let _e43 = sampleDir_1;
            let _e44 = mipInt_2;
            let _e47 = bilinearCubeUV(envMap_t_1, envMap_s_1, _e43, (_e44 + 1f));
            color1_ = _e47;
            let _e49 = color0_;
            let _e50 = color1_;
            let _e51 = mipF;
            let _e53 = mix(_e49, _e50, vec3(_e51));
            return vec4<f32>(_e53.x, _e53.y, _e53.z, 1f);
        }
    }
}

fn stepRayOrigin(rayOrigin: vec3<f32>, rayDirection: vec3<f32>, offset: vec3<f32>, dist: f32) -> vec3<f32> {
    var rayOrigin_1: vec3<f32>;
    var rayDirection_1: vec3<f32>;
    var offset_1: vec3<f32>;
    var dist_1: f32;
    var point: vec3<f32>;
    var absPoint: vec3<f32>;
    var maxPoint: f32;

    rayOrigin_1 = rayOrigin;
    rayDirection_1 = rayDirection;
    offset_1 = offset;
    dist_1 = dist;
    let _e19 = rayOrigin_1;
    let _e20 = rayDirection_1;
    let _e21 = dist_1;
    point = (_e19 + (_e20 * _e21));
    let _e25 = point;
    absPoint = abs(_e25);
    let _e28 = absPoint;
    let _e30 = absPoint;
    let _e32 = absPoint;
    maxPoint = max(_e28.x, max(_e30.y, _e32.z));
    let _e37 = point;
    let _e38 = offset_1;
    let _e39 = maxPoint;
    return (_e37 + ((_e38 * (_e39 + 1f)) * 0.0001f));
}

fn transmissionAttenuation(dist_2: f32, attColor: vec3<f32>, attDist: f32) -> vec3<f32> {
    var dist_3: f32;
    var attColor_1: vec3<f32>;
    var attDist_1: f32;
    var ot: vec3<f32>;

    dist_3 = dist_2;
    attColor_1 = attColor;
    attDist_1 = attDist;
    let _e17 = attColor_1;
    let _e20 = attDist_1;
    ot = (-(log(_e17)) / vec3(_e20));
    let _e24 = ot;
    let _e26 = dist_3;
    return exp((-(_e24) * _e26));
}

fn getHalfVector(wi: vec3<f32>, wo: vec3<f32>, eta: f32) -> vec3<f32> {
    var wi_1: vec3<f32>;
    var wo_1: vec3<f32>;
    var eta_1: f32;
    var h: vec3<f32>;

    wi_1 = wi;
    wo_1 = wo;
    eta_1 = eta;
    let _e18 = wi_1;
    if (_e18.z > 0f) {
        {
            let _e22 = wi_1;
            let _e23 = wo_1;
            h = normalize((_e22 + _e23));
        }
    } else {
        {
            let _e26 = wi_1;
            let _e27 = wo_1;
            let _e28 = eta_1;
            h = normalize((_e26 + (_e27 * _e28)));
        }
    }
    let _e32 = h;
    let _e33 = h;
    h = (_e32 * sign(_e33.z));
    let _e37 = h;
    return _e37;
}

fn getHalfVector_1(a_1: vec3<f32>, b_3: vec3<f32>) -> vec3<f32> {
    var a_2: vec3<f32>;
    var b_4: vec3<f32>;

    a_2 = a_1;
    b_4 = b_3;
    let _e15 = a_2;
    let _e16 = b_4;
    return normalize((_e15 + _e16));
}

fn isDirectionValid(direction_6: vec3<f32>, surfaceNormal: vec3<f32>, geometryNormal: vec3<f32>) -> bool {
    var direction_7: vec3<f32>;
    var surfaceNormal_1: vec3<f32>;
    var geometryNormal_1: vec3<f32>;
    var aboveSurfaceNormal: bool;
    var aboveGeometryNormal: bool;

    direction_7 = direction_6;
    surfaceNormal_1 = surfaceNormal;
    geometryNormal_1 = geometryNormal;
    let _e17 = direction_7;
    let _e18 = surfaceNormal_1;
    aboveSurfaceNormal = (dot(_e17, _e18) > 0f);
    let _e23 = direction_7;
    let _e24 = geometryNormal_1;
    aboveGeometryNormal = (dot(_e23, _e24) > 0f);
    let _e29 = aboveSurfaceNormal;
    let _e30 = aboveGeometryNormal;
    return (_e29 == _e30);
}

fn equirectDirectionToUv(direction_8: vec3<f32>) -> vec2<f32> {
    var direction_9: vec3<f32>;
    var uv_4: vec2<f32>;

    direction_9 = direction_8;
    let _e13 = direction_9;
    let _e15 = direction_9;
    let _e18 = direction_9;
    uv_4 = vec2<f32>(atan2(_e13.z, _e15.x), acos(_e18.y));
    let _e23 = uv_4;
    uv_4 = (_e23 / vec2<f32>(6.2831855f, 3.1415927f));
    let _e31 = uv_4;
    uv_4.x = (_e31.x + 0.5f);
    let _e37 = uv_4;
    uv_4.y = (1f - _e37.y);
    let _e40 = uv_4;
    return _e40;
}

fn equirectUvToDirection(uv_5: vec2<f32>) -> vec3<f32> {
    var uv_6: vec2<f32>;
    var theta: f32;
    var phi: f32;
    var sinPhi: f32;

    uv_6 = uv_5;
    let _e14 = uv_6;
    uv_6.x = (_e14.x - 0.5f);
    let _e20 = uv_6;
    uv_6.y = (1f - _e20.y);
    let _e23 = uv_6;
    theta = ((_e23.x * 2f) * 3.1415927f);
    let _e30 = uv_6;
    phi = (_e30.y * 3.1415927f);
    let _e35 = phi;
    sinPhi = sin(_e35);
    let _e38 = sinPhi;
    let _e39 = theta;
    let _e42 = phi;
    let _e44 = sinPhi;
    let _e45 = theta;
    return vec3<f32>((_e38 * cos(_e39)), cos(_e42), (_e44 * sin(_e45)));
}

fn misHeuristic(a_3: f32, b_5: f32) -> f32 {
    var a_4: f32;
    var b_6: f32;
    var aa: f32;
    var bb: f32;

    a_4 = a_3;
    b_6 = b_5;
    let _e15 = a_4;
    let _e16 = a_4;
    aa = (_e15 * _e16);
    let _e19 = b_6;
    let _e20 = b_6;
    bb = (_e19 * _e20);
    let _e23 = aa;
    let _e24 = aa;
    let _e25 = bb;
    return (_e23 / (_e24 + _e25));
}

fn tentFilter(x_16: f32) -> f32 {
    var x_17: f32;
    var local_4: f32;

    x_17 = x_16;
    let _e13 = x_17;
    if (_e13 < 0.5f) {
        let _e17 = x_17;
        local_4 = (sqrt((2f * _e17)) - 1f);
    } else {
        let _e25 = x_17;
        local_4 = (1f - sqrt((2f - (2f * _e25))));
    }
    let _e31 = local_4;
    return _e31;
}

fn main_1() {
    var rayDirection_2: vec3<f32>;

    let _e12 = vUv_1;
    let _e13 = equirectUvToDirection(_e12);
    rayDirection_2 = _e13;
    let _e15 = rayDirection_2;
    let _e16 = global.blur;
    let _e17 = textureCubeUV(envMap_t_2, envMap_s_2, _e15, _e16);
    pc_fragColor = _e17;
    return;
}

@fragment 
fn main(@location(1) vUv: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    main_1();
    let _e19 = pc_fragColor;
    return FragmentOutput(_e19);
}
