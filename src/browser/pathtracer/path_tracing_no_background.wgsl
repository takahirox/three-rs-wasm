diagnostic(off, derivative_uniformity);
struct Uniforms {
    viewMatrix: mat4x4<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    envMapInfo_totalSum: f32,
    environmentRotation: mat4x4<f32>,
    environmentIntensity: f32,
    lights_count: u32,
    backgroundBlur: f32,
    backgroundAlpha: f32,
    backgroundRotation: mat4x4<f32>,
    backgroundIntensity: f32,
    cameraWorldMatrix: mat4x4<f32>,
    invProjectionMatrix: mat4x4<f32>,
    physicalCamera_focusDistance: f32,
    physicalCamera_anamorphicRatio: f32,
    physicalCamera_bokehSize: f32,
    physicalCamera_apertureBlades: i32,
    physicalCamera_apertureRotation: f32,
    bounces: i32,
    transmissiveBounces: i32,
    filterGlossyFactor: f32,
    seed: i32,
    resolution: vec2<f32>,
    opacity: f32,
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

struct Light {
    position: vec3<f32>,
    type_: i32,
    color: vec3<f32>,
    intensity: f32,
    u: vec3<f32>,
    v: vec3<f32>,
    area: f32,
    radius: f32,
    near: f32,
    decay: f32,
    distance: f32,
    coneCos: f32,
    penumbraCos: f32,
    iesProfile: i32,
}

struct Material {
    color: vec3<f32>,
    map: i32,
    metalness: f32,
    metalnessMap: i32,
    roughness: f32,
    roughnessMap: i32,
    ior: f32,
    transmission: f32,
    transmissionMap: i32,
    emissiveIntensity: f32,
    emissive: vec3<f32>,
    emissiveMap: i32,
    normalMap: i32,
    normalScale: vec2<f32>,
    clearcoat: f32,
    clearcoatMap: i32,
    clearcoatNormalMap: i32,
    clearcoatNormalScale: vec2<f32>,
    clearcoatRoughness: f32,
    clearcoatRoughnessMap: i32,
    iridescenceMap: i32,
    iridescenceThicknessMap: i32,
    iridescence: f32,
    iridescenceIor: f32,
    iridescenceThicknessMinimum: f32,
    iridescenceThicknessMaximum: f32,
    specularColor: vec3<f32>,
    specularColorMap: i32,
    specularIntensity: f32,
    specularIntensityMap: i32,
    thinFilm: bool,
    attenuationColor: vec3<f32>,
    attenuationDistance: f32,
    alphaMap: i32,
    castShadow: bool,
    opacity: f32,
    alphaTest: f32,
    side: f32,
    matte: bool,
    sheen: f32,
    sheenColor: vec3<f32>,
    sheenColorMap: i32,
    sheenRoughness: f32,
    sheenRoughnessMap: i32,
    vertexColors: bool,
    flatShading: bool,
    transparent: bool,
    fogVolume: bool,
    mapTransform: mat3x3<f32>,
    metalnessMapTransform: mat3x3<f32>,
    roughnessMapTransform: mat3x3<f32>,
    transmissionMapTransform: mat3x3<f32>,
    emissiveMapTransform: mat3x3<f32>,
    normalMapTransform: mat3x3<f32>,
    clearcoatMapTransform: mat3x3<f32>,
    clearcoatNormalMapTransform: mat3x3<f32>,
    clearcoatRoughnessMapTransform: mat3x3<f32>,
    sheenColorMapTransform: mat3x3<f32>,
    sheenRoughnessMapTransform: mat3x3<f32>,
    iridescenceMapTransform: mat3x3<f32>,
    iridescenceThicknessMapTransform: mat3x3<f32>,
    specularColorMapTransform: mat3x3<f32>,
    specularIntensityMapTransform: mat3x3<f32>,
    alphaMapTransform: mat3x3<f32>,
}

struct SurfaceRecord {
    volumeParticle: bool,
    faceNormal: vec3<f32>,
    frontFace: bool,
    normal: vec3<f32>,
    normalBasis: mat3x3<f32>,
    normalInvBasis: mat3x3<f32>,
    eta: f32,
    f0_: f32,
    roughness: f32,
    filteredRoughness: f32,
    metalness: f32,
    color: vec3<f32>,
    emission: vec3<f32>,
    ior: f32,
    transmission: f32,
    thinFilm: bool,
    attenuationColor: vec3<f32>,
    attenuationDistance: f32,
    clearcoatNormal: vec3<f32>,
    clearcoatBasis: mat3x3<f32>,
    clearcoatInvBasis: mat3x3<f32>,
    clearcoat: f32,
    clearcoatRoughness: f32,
    filteredClearcoatRoughness: f32,
    sheen: f32,
    sheenColor: vec3<f32>,
    sheenRoughness: f32,
    iridescence: f32,
    iridescenceIor: f32,
    iridescenceThickness: f32,
    specularColor: vec3<f32>,
    specularIntensity: f32,
}

struct ScatterRecord {
    specularPdf: f32,
    pdf: f32,
    direction: vec3<f32>,
    color: vec3<f32>,
}

struct LightRecord {
    dist: f32,
    direction: vec3<f32>,
    pdf: f32,
    emission: vec3<f32>,
    type_: i32,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct SurfaceHit {
    faceIndices: vec4<u32>,
    barycoord: vec3<f32>,
    faceNormal: vec3<f32>,
    side: f32,
    dist: f32,
}

struct RenderState {
    firstRay: bool,
    transmissiveRay: bool,
    isShadowRay: bool,
    accumulatedRoughness: f32,
    transmissiveTraversals: i32,
    traversals: i32,
    depth: u32,
    throughputColor: vec3<f32>,
    fogMaterial: Material,
}

struct FragmentOutput {
    @location(0) pc_fragColor: vec4<f32>,
}

const XYZ_TO_REC709_: mat3x3<f32> = mat3x3<f32>(vec3<f32>(3.2404542f, -0.969266f, 0.0556434f), vec3<f32>(-1.5371385f, 1.8760108f, -0.2040259f), vec3<f32>(-0.4985314f, 0.041556f, 1.0572252f));

var<private> pc_fragColor: vec4<f32>;
@group(0) @binding(0) 
var<uniform> global: Uniforms;
@group(0) @binding(1) 
var stratifiedTexture_t: texture_2d<f32>;
@group(0) @binding(2) 
var pt_nearest: sampler;
@group(0) @binding(3) 
var stratifiedOffsetTexture_t: texture_2d<f32>;
@group(0) @binding(4) 
var sobolTexture_t: texture_2d<f32>;
@group(0) @binding(5) 
var envMapInfo_marginalWeights_t: texture_2d<f32>;
@group(0) @binding(6) 
var pt_linear: sampler;
@group(0) @binding(7) 
var envMapInfo_conditionalWeights_t: texture_2d<f32>;
@group(0) @binding(8) 
var envMapInfo_map_t: texture_2d<f32>;
@group(0) @binding(9) 
var pt_linear_repeat_u: sampler;
@group(0) @binding(10) 
var iesProfiles_t_3: texture_2d_array<f32>;
@group(0) @binding(11) 
var lights_tex_t: texture_2d<f32>;
@group(0) @binding(12) 
var backgroundMap_t: texture_2d<f32>;
@group(0) @binding(13) 
var attributesArray_t_1: texture_2d_array<f32>;
@group(0) @binding(14) 
var materialIndexAttribute_t_1: texture_2d<u32>;
@group(0) @binding(15) 
var materials_t_2: texture_2d<f32>;
@group(0) @binding(16) 
var textures_t: texture_2d_array<f32>;
@group(0) @binding(17) 
var pt_linear_repeat: sampler;
@group(0) @binding(18) 
var bvh_index_t_1: texture_2d<u32>;
@group(0) @binding(19) 
var bvh_position_t_1: texture_2d<f32>;
@group(0) @binding(20) 
var bvh_bvhBounds_t_1: texture_2d<f32>;
@group(0) @binding(21) 
var bvh_bvhContents_t_1: texture_2d<u32>;
var<private> sobolPixelIndex: u32 = 0u;
var<private> sobolPathIndex: u32 = 0u;
var<private> sobolBounceIndex: u32 = 0u;
var<private> pixelSeed: vec4<f32> = vec4(0f);
var<private> vUv_1: vec2<f32>;
var<private> envRotation3x3_: mat3x3<f32>;
var<private> invEnvRotation3x3_: mat3x3<f32>;
var<private> lightsDenom: f32;
var<private> gl_FragCoord_1: vec4<f32>;

fn pt_isnan(v: vec3<f32>) -> vec3<bool> {
    var v_1: vec3<f32>;
    var b: vec3<u32>;

    v_1 = v;
    let _e72 = v_1;
    b = (bitcast<vec3<u32>>(_e72) & vec3(2147483647u));
    let _e78 = b;
    return (_e78 > vec3(2139095040u));
}

fn pt_isinf(v_2: vec3<f32>) -> vec3<bool> {
    var v_3: vec3<f32>;
    var b_1: vec3<u32>;

    v_3 = v_2;
    let _e72 = v_3;
    b_1 = (bitcast<vec3<u32>>(_e72) & vec3(2147483647u));
    let _e78 = b_1;
    return (_e78 == vec3(2139095040u));
}

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e74 = y_1;
    let _e75 = x_1;
    return exp2((_e74 * log2(_e75)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e74 = y_3;
    let _e75 = x_3;
    return exp2((_e74 * log2(_e75)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e74 = y_5;
    let _e75 = x_5;
    return exp2((_e74 * log2(_e75)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e74 = y_7;
    let _e75 = x_7;
    return exp2((_e74 * log2(_e75)));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e72 = value_1;
    return _e72;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e72 = value_3;
    let _e81 = pt_pow_2(((_e72.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e82 = value_3;
    let _e86 = value_3;
    let _e96 = mix(_e81, (_e82.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e86.xyz <= vec3(0.04045f))));
    let _e97 = value_3;
    return vec4<f32>(_e96.x, _e96.y, _e96.z, _e97.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e72 = value_5;
    let _e76 = pt_pow_2(_e72.xyz, vec3(0.41666f));
    let _e82 = value_5;
    let _e86 = value_5;
    let _e96 = mix(((_e76 * 1.055f) - vec3(0.055f)), (_e82.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e86.xyz <= vec3(0.0031308f))));
    let _e97 = value_5;
    return vec4<f32>(_e96.x, _e96.y, _e96.z, _e97.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e72 = value_7;
    let _e90 = (_e72.xyz * mat3x3<f32>(vec3<f32>(1f, -0f, -0f), vec3<f32>(-0f, 1f, 0f), vec3<f32>(0f, 0f, 1f)));
    let _e91 = value_7;
    let _e97 = LinearTransferOETF(vec4<f32>(_e90.x, _e90.y, _e90.z, _e91.w));
    return _e97;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e77 = weights;
    let _e78 = rgb_1;
    return dot(_e77, _e78);
}

fn envMapTexelToLinear(a: vec4<f32>) -> vec4<f32> {
    var a_1: vec4<f32>;

    a_1 = a;
    let _e72 = a_1;
    return _e72;
}

fn pow2_(x_8: f32) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e72 = x_9;
    let _e73 = x_9;
    return (_e72 * _e73);
}

fn pow2_1(x_10: vec3<f32>) -> vec3<f32> {
    var x_11: vec3<f32>;

    x_11 = x_10;
    let _e72 = x_11;
    let _e73 = x_11;
    return (_e72 * _e73);
}

fn pow3_(x_12: f32) -> f32 {
    var x_13: f32;

    x_13 = x_12;
    let _e72 = x_13;
    let _e73 = x_13;
    let _e75 = x_13;
    return ((_e72 * _e73) * _e75);
}

fn pow4_(x_14: f32) -> f32 {
    var x_15: f32;
    var x2_: f32;

    x_15 = x_14;
    let _e72 = x_15;
    let _e73 = x_15;
    x2_ = (_e72 * _e73);
    let _e76 = x2_;
    let _e77 = x2_;
    return (_e76 * _e77);
}

fn max3_(v_4: vec3<f32>) -> f32 {
    var v_5: vec3<f32>;

    v_5 = v_4;
    let _e72 = v_5;
    let _e74 = v_5;
    let _e77 = v_5;
    return max(max(_e72.x, _e74.y), _e77.z);
}

fn average(v_6: vec3<f32>) -> f32 {
    var v_7: vec3<f32>;

    v_7 = v_6;
    let _e72 = v_7;
    return dot(_e72, vec3(0.3333333f));
}

fn rand(uv: vec2<f32>) -> f32 {
    var uv_1: vec2<f32>;
    var a_2: f32 = 12.9898f;
    var b_2: f32 = 78.233f;
    var c: f32 = 43758.547f;
    var dt: f32;
    var sn: f32;

    uv_1 = uv;
    let _e78 = uv_1;
    let _e80 = a_2;
    let _e81 = b_2;
    dt = dot(_e78.xy, vec2<f32>(_e80, _e81));
    let _e85 = dt;
    sn = (_e85 - (floor((_e85 / 3.1415927f)) * 3.1415927f));
    let _e92 = sn;
    let _e94 = c;
    return fract((sin(_e92) * _e94));
}

fn precisionSafeLength(v_8: vec3<f32>) -> f32 {
    var v_9: vec3<f32>;

    v_9 = v_8;
    let _e72 = v_9;
    return length(_e72);
}

fn transformDirection(dir: vec3<f32>, matrix: mat4x4<f32>) -> vec3<f32> {
    var dir_1: vec3<f32>;
    var matrix_1: mat4x4<f32>;

    dir_1 = dir;
    matrix_1 = matrix;
    let _e74 = matrix_1;
    let _e75 = dir_1;
    return normalize((_e74 * vec4<f32>(_e75.x, _e75.y, _e75.z, 0f)).xyz);
}

fn transformNormalByInverseViewMatrix(normal: vec3<f32>, viewMatrix: mat4x4<f32>) -> vec3<f32> {
    var normal_1: vec3<f32>;
    var viewMatrix_1: mat4x4<f32>;

    normal_1 = normal;
    viewMatrix_1 = viewMatrix;
    let _e74 = normal_1;
    let _e80 = viewMatrix_1;
    return normalize((vec4<f32>(_e74.x, _e74.y, _e74.z, 0f) * _e80).xyz);
}

fn transformDirectionByInverseViewMatrix(dir_2: vec3<f32>, viewMatrix_2: mat4x4<f32>) -> vec3<f32> {
    var dir_3: vec3<f32>;
    var viewMatrix_3: mat4x4<f32>;

    dir_3 = dir_2;
    viewMatrix_3 = viewMatrix_2;
    let _e74 = dir_3;
    let _e80 = viewMatrix_3;
    return normalize((vec4<f32>(_e74.x, _e74.y, _e74.z, 0f) * _e80).xyz);
}

fn isPerspectiveMatrix(m: mat4x4<f32>) -> bool {
    var m_1: mat4x4<f32>;

    m_1 = m;
    let _e76 = m_1[2][3];
    return (_e76 == -1f);
}

fn equirectUv(dir_4: vec3<f32>) -> vec2<f32> {
    var dir_5: vec3<f32>;
    var u: f32;
    var v_10: f32;

    dir_5 = dir_4;
    let _e72 = dir_5;
    let _e74 = dir_5;
    u = ((atan2(_e72.z, _e74.x) * 0.15915494f) + 0.5f);
    let _e82 = dir_5;
    v_10 = ((asin(clamp(_e82.y, -1f, 1f)) * 0.31830987f) + 0.5f);
    let _e94 = u;
    let _e95 = v_10;
    return vec2<f32>(_e94, _e95);
}

fn BRDF_Lambert(diffuseColor: vec3<f32>) -> vec3<f32> {
    var diffuseColor_1: vec3<f32>;

    diffuseColor_1 = diffuseColor;
    let _e73 = diffuseColor_1;
    return (0.31830987f * _e73);
}

fn F_Schlick(f0_: vec3<f32>, f90_: f32, dotVH: f32) -> vec3<f32> {
    var f0_1: vec3<f32>;
    var f90_1: f32;
    var dotVH_1: f32;
    var fresnel: f32;

    f0_1 = f0_;
    f90_1 = f90_;
    dotVH_1 = dotVH;
    let _e78 = dotVH_1;
    let _e82 = dotVH_1;
    fresnel = exp2((((-5.55473f * _e78) - 6.98316f) * _e82));
    let _e86 = f0_1;
    let _e88 = fresnel;
    let _e91 = f90_1;
    let _e92 = fresnel;
    return ((_e86 * (1f - _e88)) + vec3((_e91 * _e92)));
}

fn F_Schlick_1(f0_2: f32, f90_2: f32, dotVH_2: f32) -> f32 {
    var f0_3: f32;
    var f90_3: f32;
    var dotVH_3: f32;
    var fresnel_1: f32;

    f0_3 = f0_2;
    f90_3 = f90_2;
    dotVH_3 = dotVH_2;
    let _e78 = dotVH_3;
    let _e82 = dotVH_3;
    fresnel_1 = exp2((((-5.55473f * _e78) - 6.98316f) * _e82));
    let _e86 = f0_3;
    let _e88 = fresnel_1;
    let _e91 = f90_3;
    let _e92 = fresnel_1;
    return ((_e86 * (1f - _e88)) + (_e91 * _e92));
}

fn uTexelFetch1D(tex_t: texture_2d<u32>, tex_s: sampler, index: u32) -> vec4<u32> {
    var index_1: u32;
    var width: u32;
    var uv_2: vec2<u32>;

    index_1 = index;
    let _e75 = textureDimensions(tex_t, 0i);
    width = u32(vec2<i32>(_e75).x);
    let _e82 = index_1;
    let _e83 = width;
    uv_2.x = (_e82 % _e83);
    let _e86 = index_1;
    let _e87 = width;
    uv_2.y = (_e86 / _e87);
    let _e89 = uv_2;
    let _e92 = textureLoad(tex_t, vec2<i32>(_e89), 0i);
    return _e92;
}

fn iTexelFetch1D(tex_t_1: texture_2d<i32>, tex_s_1: sampler, index_2: u32) -> vec4<i32> {
    var index_3: u32;
    var width_1: u32;
    var uv_3: vec2<u32>;

    index_3 = index_2;
    let _e75 = textureDimensions(tex_t_1, 0i);
    width_1 = u32(vec2<i32>(_e75).x);
    let _e82 = index_3;
    let _e83 = width_1;
    uv_3.x = (_e82 % _e83);
    let _e86 = index_3;
    let _e87 = width_1;
    uv_3.y = (_e86 / _e87);
    let _e89 = uv_3;
    let _e92 = textureLoad(tex_t_1, vec2<i32>(_e89), 0i);
    return _e92;
}

fn texelFetch1D(tex_t_2: texture_2d<f32>, tex_s_2: sampler, index_4: u32) -> vec4<f32> {
    var index_5: u32;
    var width_2: u32;
    var uv_4: vec2<u32>;

    index_5 = index_4;
    let _e75 = textureDimensions(tex_t_2, 0i);
    width_2 = u32(vec2<i32>(_e75).x);
    let _e82 = index_5;
    let _e83 = width_2;
    uv_4.x = (_e82 % _e83);
    let _e86 = index_5;
    let _e87 = width_2;
    uv_4.y = (_e86 / _e87);
    let _e89 = uv_4;
    let _e92 = textureLoad(tex_t_2, vec2<i32>(_e89), 0i);
    return _e92;
}

fn textureSampleBarycoord(tex_t_3: texture_2d<f32>, tex_s_3: sampler, barycoord: vec3<f32>, faceIndices: vec3<u32>) -> vec4<f32> {
    var barycoord_1: vec3<f32>;
    var faceIndices_1: vec3<u32>;

    barycoord_1 = barycoord;
    faceIndices_1 = faceIndices;
    let _e76 = barycoord_1;
    let _e78 = faceIndices_1;
    let _e80 = texelFetch1D(tex_t_3, tex_s_3, _e78.x);
    let _e82 = barycoord_1;
    let _e84 = faceIndices_1;
    let _e86 = texelFetch1D(tex_t_3, tex_s_3, _e84.y);
    let _e89 = barycoord_1;
    let _e91 = faceIndices_1;
    let _e93 = texelFetch1D(tex_t_3, tex_s_3, _e91.z);
    return (((_e76.x * _e80) + (_e82.y * _e86)) + (_e89.z * _e93));
}

fn ndcToCameraRay(coord: vec2<f32>, cameraWorld: mat4x4<f32>, invProjectionMatrix: mat4x4<f32>, rayOrigin: ptr<function, vec3<f32>>, rayDirection: ptr<function, vec3<f32>>) {
    var coord_1: vec2<f32>;
    var cameraWorld_1: mat4x4<f32>;
    var invProjectionMatrix_1: mat4x4<f32>;
    var lookDirection: vec4<f32>;
    var nearVector: vec4<f32>;
    var near: f32;
    var origin: vec4<f32>;
    var direction: vec4<f32>;

    coord_1 = coord;
    cameraWorld_1 = cameraWorld;
    invProjectionMatrix_1 = invProjectionMatrix;
    let _e78 = cameraWorld_1;
    lookDirection = (_e78 * vec4<f32>(0f, 0f, -1f, 0f));
    let _e87 = invProjectionMatrix_1;
    nearVector = (_e87 * vec4<f32>(0f, 0f, -1f, 1f));
    let _e96 = nearVector;
    let _e98 = nearVector;
    near = abs((_e96.z / _e98.w));
    let _e103 = cameraWorld_1;
    origin = (_e103 * vec4<f32>(0f, 0f, 0f, 1f));
    let _e111 = invProjectionMatrix_1;
    let _e112 = coord_1;
    direction = (_e111 * vec4<f32>(_e112.x, _e112.y, 0.5f, 1f));
    let _e120 = direction;
    let _e121 = direction;
    direction = (_e120 / vec4(_e121.w));
    let _e125 = cameraWorld_1;
    let _e126 = direction;
    let _e128 = origin;
    direction = ((_e125 * _e126) - _e128);
    let _e130 = origin;
    let _e132 = origin;
    let _e134 = direction;
    let _e136 = near;
    let _e138 = direction;
    let _e139 = lookDirection;
    let _e143 = (_e132.xyz + ((_e134.xyz * _e136) / vec3(dot(_e138, _e139))));
    origin.x = _e143.x;
    origin.y = _e143.y;
    origin.z = _e143.z;
    let _e150 = origin;
    (*rayOrigin) = _e150.xyz;
    let _e152 = direction;
    (*rayDirection) = _e152.xyz;
    return;
}

fn intersectsBounds(rayOrigin_1: vec3<f32>, rayDirection_1: vec3<f32>, boundsMin: vec3<f32>, boundsMax: vec3<f32>, dist: ptr<function, f32>) -> bool {
    var rayOrigin_2: vec3<f32>;
    var rayDirection_2: vec3<f32>;
    var boundsMin_1: vec3<f32>;
    var boundsMax_1: vec3<f32>;
    var invDir: vec3<f32>;
    var tMinPlane: vec3<f32>;
    var tMaxPlane: vec3<f32>;
    var tMinHit: vec3<f32>;
    var tMaxHit: vec3<f32>;
    var t: vec2<f32>;
    var t0_: f32;
    var t1_: f32;

    rayOrigin_2 = rayOrigin_1;
    rayDirection_2 = rayDirection_1;
    boundsMin_1 = boundsMin;
    boundsMax_1 = boundsMax;
    let _e80 = rayDirection_2;
    invDir = (vec3(1f) / _e80);
    let _e84 = invDir;
    let _e85 = boundsMin_1;
    let _e86 = rayOrigin_2;
    tMinPlane = (_e84 * (_e85 - _e86));
    let _e90 = invDir;
    let _e91 = boundsMax_1;
    let _e92 = rayOrigin_2;
    tMaxPlane = (_e90 * (_e91 - _e92));
    let _e96 = tMaxPlane;
    let _e97 = tMinPlane;
    tMinHit = min(_e96, _e97);
    let _e100 = tMaxPlane;
    let _e101 = tMinPlane;
    tMaxHit = max(_e100, _e101);
    let _e104 = tMinHit;
    let _e106 = tMinHit;
    t = max(_e104.xx, _e106.yz);
    let _e110 = t;
    let _e112 = t;
    t0_ = max(_e110.x, _e112.y);
    let _e116 = tMaxHit;
    let _e118 = tMaxHit;
    t = min(_e116.xx, _e118.yz);
    let _e121 = t;
    let _e123 = t;
    t1_ = min(_e121.x, _e123.y);
    let _e127 = t0_;
    (*dist) = max(_e127, 0f);
    let _e130 = t1_;
    let _e131 = (*dist);
    return (_e130 >= _e131);
}

fn intersectsTriangle(rayOrigin_3: vec3<f32>, rayDirection_3: vec3<f32>, a_3: vec3<f32>, b_3: vec3<f32>, c_1: vec3<f32>, barycoord_2: ptr<function, vec3<f32>>, norm: ptr<function, vec3<f32>>, dist_1: ptr<function, f32>, side: ptr<function, f32>) -> bool {
    var rayOrigin_4: vec3<f32>;
    var rayDirection_4: vec3<f32>;
    var a_4: vec3<f32>;
    var b_4: vec3<f32>;
    var c_2: vec3<f32>;
    var edge1_: vec3<f32>;
    var edge2_: vec3<f32>;
    var det: f32;
    var invdet: f32;
    var AO: vec3<f32>;
    var DAO: vec3<f32>;
    var uvt: vec4<f32>;

    rayOrigin_4 = rayOrigin_3;
    rayDirection_4 = rayDirection_3;
    a_4 = a_3;
    b_4 = b_3;
    c_2 = c_1;
    let _e84 = b_4;
    let _e85 = a_4;
    edge1_ = (_e84 - _e85);
    let _e88 = c_2;
    let _e89 = a_4;
    edge2_ = (_e88 - _e89);
    let _e92 = edge1_;
    let _e93 = edge2_;
    (*norm) = cross(_e92, _e93);
    let _e95 = rayDirection_4;
    let _e96 = (*norm);
    det = -(dot(_e95, _e96));
    let _e101 = det;
    invdet = (1f / _e101);
    let _e104 = rayOrigin_4;
    let _e105 = a_4;
    AO = (_e104 - _e105);
    let _e108 = AO;
    let _e109 = rayDirection_4;
    DAO = cross(_e108, _e109);
    let _e114 = edge2_;
    let _e115 = DAO;
    let _e117 = invdet;
    uvt.x = (dot(_e114, _e115) * _e117);
    let _e120 = edge1_;
    let _e121 = DAO;
    let _e124 = invdet;
    uvt.y = (-(dot(_e120, _e121)) * _e124);
    let _e127 = AO;
    let _e128 = (*norm);
    let _e130 = invdet;
    uvt.z = (dot(_e127, _e128) * _e130);
    let _e134 = uvt;
    let _e137 = uvt;
    uvt.w = ((1f - _e134.x) - _e137.y);
    let _e140 = uvt;
    (*barycoord_2) = _e140.wxy;
    let _e142 = uvt;
    (*dist_1) = _e142.z;
    let _e144 = det;
    (*side) = sign(_e144);
    let _e146 = (*side);
    let _e147 = (*norm);
    (*norm) = (_e146 * normalize(_e147));
    let _e150 = uvt;
    uvt = (_e150 + vec4(0.00001f));
    let _e154 = uvt;
    return all((_e154 >= vec4(0f)));
}

fn intersectTriangles(positionAttr_t: texture_2d<f32>, positionAttr_s: sampler, indexAttr_t: texture_2d<u32>, indexAttr_s: sampler, offset: u32, count: u32, rayOrigin_5: vec3<f32>, rayDirection_5: vec3<f32>, minDistance: ptr<function, f32>, faceIndices_2: ptr<function, vec4<u32>>, faceNormal: ptr<function, vec3<f32>>, barycoord_3: ptr<function, vec3<f32>>, side_1: ptr<function, f32>, dist_2: ptr<function, f32>) -> bool {
    var offset_1: u32;
    var count_1: u32;
    var rayOrigin_6: vec3<f32>;
    var rayDirection_6: vec3<f32>;
    var found: bool = false;
    var localBarycoord: vec3<f32>;
    var localNormal: vec3<f32>;
    var localDist: f32;
    var localSide: f32;
    var i: u32;
    var l: u32;
    var indices: vec3<u32>;
    var a_5: vec3<f32>;
    var b_5: vec3<f32>;
    var c_3: vec3<f32>;

    offset_1 = offset;
    count_1 = count;
    rayOrigin_6 = rayOrigin_5;
    rayDirection_6 = rayDirection_5;
    let _e94 = offset_1;
    i = _e94;
    let _e96 = offset_1;
    let _e97 = count_1;
    l = (_e96 + _e97);
    loop {
        let _e100 = i;
        let _e101 = l;
        if !((_e100 < _e101)) {
            break;
        }
        {
            let _e107 = i;
            let _e108 = uTexelFetch1D(indexAttr_t, indexAttr_s, _e107);
            indices = _e108.xyz;
            let _e111 = indices;
            let _e113 = texelFetch1D(positionAttr_t, positionAttr_s, _e111.x);
            a_5 = _e113.xyz;
            let _e116 = indices;
            let _e118 = texelFetch1D(positionAttr_t, positionAttr_s, _e116.y);
            b_5 = _e118.xyz;
            let _e121 = indices;
            let _e123 = texelFetch1D(positionAttr_t, positionAttr_s, _e121.z);
            c_3 = _e123.xyz;
            let _e126 = rayOrigin_6;
            let _e127 = rayDirection_6;
            let _e128 = a_5;
            let _e129 = b_5;
            let _e130 = c_3;
            let _e139 = intersectsTriangle(_e126, _e127, _e128, _e129, _e130, (&localBarycoord), (&localNormal), (&localDist), (&localSide));
            let _e140 = localDist;
            let _e141 = (*minDistance);
            if (_e139 && (_e140 < _e141)) {
                {
                    found = true;
                    let _e145 = localDist;
                    (*minDistance) = _e145;
                    let _e146 = indices;
                    let _e147 = _e146.xyz;
                    let _e148 = i;
                    (*faceIndices_2) = vec4<u32>(_e147.x, _e147.y, _e147.z, _e148);
                    let _e153 = localNormal;
                    (*faceNormal) = _e153;
                    let _e154 = localSide;
                    (*side_1) = _e154;
                    let _e155 = localBarycoord;
                    (*barycoord_3) = _e155;
                    let _e156 = localDist;
                    (*dist_2) = _e156;
                }
            }
        }
        continuing {
            let _e104 = i;
            i = (_e104 + 1u);
        }
    }
    let _e157 = found;
    return _e157;
}

fn intersectsBVHNodeBounds(rayOrigin_7: vec3<f32>, rayDirection_7: vec3<f32>, bvhBounds_t: texture_2d<f32>, bvhBounds_s: sampler, currNodeIndex: u32, dist_3: ptr<function, f32>) -> bool {
    var rayOrigin_8: vec3<f32>;
    var rayDirection_8: vec3<f32>;
    var currNodeIndex_1: u32;
    var cni2_: u32;
    var boundsMin_2: vec3<f32>;
    var boundsMax_2: vec3<f32>;

    rayOrigin_8 = rayOrigin_7;
    rayDirection_8 = rayDirection_7;
    currNodeIndex_1 = currNodeIndex;
    let _e79 = currNodeIndex_1;
    cni2_ = (_e79 * 2u);
    let _e83 = cni2_;
    let _e84 = texelFetch1D(bvhBounds_t, bvhBounds_s, _e83);
    boundsMin_2 = _e84.xyz;
    let _e87 = cni2_;
    let _e90 = texelFetch1D(bvhBounds_t, bvhBounds_s, (_e87 + 1u));
    boundsMax_2 = _e90.xyz;
    let _e93 = rayOrigin_8;
    let _e94 = rayDirection_8;
    let _e95 = boundsMin_2;
    let _e96 = boundsMax_2;
    let _e99 = intersectsBounds(_e93, _e94, _e95, _e96, dist_3);
    return _e99;
}

fn _bvhIntersectFirstHit(bvh_position_t: texture_2d<f32>, bvh_position_s: sampler, bvh_index_t: texture_2d<u32>, bvh_index_s: sampler, bvh_bvhBounds_t: texture_2d<f32>, bvh_bvhBounds_s: sampler, bvh_bvhContents_t: texture_2d<u32>, bvh_bvhContents_s: sampler, rayOrigin_9: vec3<f32>, rayDirection_9: vec3<f32>, faceIndices_3: ptr<function, vec4<u32>>, faceNormal_1: ptr<function, vec3<f32>>, barycoord_4: ptr<function, vec3<f32>>, side_2: ptr<function, f32>, dist_4: ptr<function, f32>) -> bool {
    var rayOrigin_10: vec3<f32>;
    var rayDirection_10: vec3<f32>;
    var pointer: i32 = 0i;
    var stack: array<u32, 60>;
    var triangleDistance: f32 = 100000000000000000000f;
    var found_1: bool = false;
    var currNodeIndex_2: u32;
    var boundsHitDistance: f32;
    var boundsInfo: vec2<u32>;
    var isLeaf: bool;
    var count_2: u32;
    var offset_2: u32;
    var leftIndex: u32;
    var splitAxis: u32;
    var rightIndex: u32;
    var leftToRight: bool;
    var local: u32;
    var c1_: u32;
    var local_1: u32;
    var c2_: u32;

    rayOrigin_10 = rayOrigin_9;
    rayDirection_10 = rayDirection_9;
    stack[0i] = 0u;
    loop {
        let _e97 = pointer;
        let _e101 = pointer;
        if !(((_e97 > -1i) && (_e101 < 60i))) {
            break;
        }
        {
            let _e106 = pointer;
            let _e108 = stack[_e106];
            currNodeIndex_2 = _e108;
            let _e110 = pointer;
            pointer = (_e110 - 1i);
            let _e114 = rayOrigin_10;
            let _e115 = rayDirection_10;
            let _e116 = currNodeIndex_2;
            let _e119 = intersectsBVHNodeBounds(_e114, _e115, bvh_bvhBounds_t, bvh_bvhBounds_s, _e116, (&boundsHitDistance));
            let _e121 = boundsHitDistance;
            let _e122 = triangleDistance;
            if (!(_e119) || (_e121 > _e122)) {
                {
                    continue;
                }
            }
            let _e125 = currNodeIndex_2;
            let _e126 = uTexelFetch1D(bvh_bvhContents_t, bvh_bvhContents_s, _e125);
            boundsInfo = _e126.xy;
            let _e129 = boundsInfo;
            isLeaf = bool((_e129.x & 4294901760u));
            let _e135 = isLeaf;
            if _e135 {
                {
                    let _e136 = boundsInfo;
                    count_2 = (_e136.x & 65535u);
                    let _e141 = boundsInfo;
                    offset_2 = _e141.y;
                    let _e144 = offset_2;
                    let _e145 = count_2;
                    let _e146 = rayOrigin_10;
                    let _e147 = rayDirection_10;
                    let _e160 = intersectTriangles(bvh_position_t, bvh_position_s, bvh_index_t, bvh_index_s, _e144, _e145, _e146, _e147, (&triangleDistance), faceIndices_3, faceNormal_1, barycoord_4, side_2, dist_4);
                    let _e161 = found_1;
                    found_1 = (_e160 || _e161);
                }
            } else {
                {
                    let _e163 = currNodeIndex_2;
                    leftIndex = (_e163 + 1u);
                    let _e167 = boundsInfo;
                    splitAxis = (_e167.x & 65535u);
                    let _e172 = currNodeIndex_2;
                    let _e173 = boundsInfo;
                    rightIndex = (_e172 + _e173.y);
                    let _e177 = splitAxis;
                    let _e179 = rayDirection_10[_e177];
                    leftToRight = (_e179 >= 0f);
                    let _e183 = leftToRight;
                    if _e183 {
                        let _e184 = leftIndex;
                        local = _e184;
                    } else {
                        let _e185 = rightIndex;
                        local = _e185;
                    }
                    let _e187 = local;
                    c1_ = _e187;
                    let _e189 = leftToRight;
                    if _e189 {
                        let _e190 = rightIndex;
                        local_1 = _e190;
                    } else {
                        let _e191 = leftIndex;
                        local_1 = _e191;
                    }
                    let _e193 = local_1;
                    c2_ = _e193;
                    let _e195 = pointer;
                    pointer = (_e195 + 1i);
                    let _e198 = pointer;
                    let _e200 = c2_;
                    stack[_e198] = _e200;
                    let _e201 = pointer;
                    pointer = (_e201 + 1i);
                    let _e204 = pointer;
                    let _e206 = c1_;
                    stack[_e204] = _e206;
                }
            }
        }
    }
    let _e207 = found_1;
    return _e207;
}

fn readLightInfo(tex_t_4: texture_2d<f32>, tex_s_4: sampler, index_6: u32) -> Light {
    var index_7: u32;
    var i_1: u32;
    var s0_: vec4<f32>;
    var s1_: vec4<f32>;
    var s2_: vec4<f32>;
    var s3_: vec4<f32>;
    var l_1: Light;
    var s4_: vec4<f32>;
    var s5_: vec4<f32>;

    index_7 = index_6;
    let _e74 = index_7;
    i_1 = (_e74 * 6u);
    let _e78 = i_1;
    let _e81 = texelFetch1D(tex_t_4, tex_s_4, (_e78 + 0u));
    s0_ = _e81;
    let _e83 = i_1;
    let _e86 = texelFetch1D(tex_t_4, tex_s_4, (_e83 + 1u));
    s1_ = _e86;
    let _e88 = i_1;
    let _e91 = texelFetch1D(tex_t_4, tex_s_4, (_e88 + 2u));
    s2_ = _e91;
    let _e93 = i_1;
    let _e96 = texelFetch1D(tex_t_4, tex_s_4, (_e93 + 3u));
    s3_ = _e96;
    let _e100 = s0_;
    l_1.position = _e100.xyz;
    let _e103 = s0_;
    l_1.type_ = i32(round(_e103.w));
    let _e108 = s1_;
    l_1.color = _e108.xyz;
    let _e111 = s1_;
    l_1.intensity = _e111.w;
    let _e114 = s2_;
    l_1.u = _e114.xyz;
    let _e117 = s3_;
    l_1.v = _e117.xyz;
    let _e120 = s3_;
    l_1.area = _e120.w;
    let _e122 = l_1;
    let _e126 = l_1;
    if ((_e122.type_ == 2i) || (_e126.type_ == 4i)) {
        {
            let _e131 = i_1;
            let _e134 = texelFetch1D(tex_t_4, tex_s_4, (_e131 + 4u));
            s4_ = _e134;
            let _e136 = i_1;
            let _e139 = texelFetch1D(tex_t_4, tex_s_4, (_e136 + 5u));
            s5_ = _e139;
            let _e142 = s4_;
            l_1.radius = _e142.x;
            let _e145 = s4_;
            l_1.decay = _e145.y;
            let _e148 = s4_;
            l_1.distance = _e148.z;
            let _e151 = s4_;
            l_1.coneCos = _e151.w;
            let _e154 = s5_;
            l_1.penumbraCos = _e154.x;
            let _e157 = s5_;
            l_1.iesProfile = i32(round(_e157.y));
        }
    } else {
        {
            l_1.radius = 0f;
            l_1.decay = 0f;
            l_1.distance = 0f;
            l_1.coneCos = 0f;
            l_1.penumbraCos = 0f;
            l_1.iesProfile = -1i;
        }
    }
    let _e174 = l_1;
    return _e174;
}

fn readTextureTransform(tex_t_5: texture_2d<f32>, tex_s_5: sampler, index_8: u32) -> mat3x3<f32> {
    var index_9: u32;
    var textureTransform: mat3x3<f32>;
    var row1_: vec4<f32>;
    var row2_: vec4<f32>;

    index_9 = index_8;
    let _e75 = index_9;
    let _e76 = texelFetch1D(tex_t_5, tex_s_5, _e75);
    row1_ = _e76;
    let _e78 = index_9;
    let _e81 = texelFetch1D(tex_t_5, tex_s_5, (_e78 + 1u));
    row2_ = _e81;
    let _e85 = row1_;
    let _e87 = row2_;
    textureTransform[0i] = vec3<f32>(_e85.x, _e87.x, 0f);
    let _e93 = row1_;
    let _e95 = row2_;
    textureTransform[1i] = vec3<f32>(_e93.y, _e95.y, 0f);
    let _e101 = row1_;
    let _e103 = row2_;
    textureTransform[2i] = vec3<f32>(_e101.z, _e103.z, 1f);
    let _e107 = textureTransform;
    return _e107;
}

fn readMaterialInfo(tex_t_6: texture_2d<f32>, tex_s_6: sampler, index_10: u32) -> Material {
    var index_11: u32;
    var i_2: u32;
    var s0_1: vec4<f32>;
    var s1_1: vec4<f32>;
    var s2_1: vec4<f32>;
    var s3_1: vec4<f32>;
    var s4_1: vec4<f32>;
    var s5_1: vec4<f32>;
    var s6_: vec4<f32>;
    var s7_: vec4<f32>;
    var s8_: vec4<f32>;
    var s9_: vec4<f32>;
    var s10_: vec4<f32>;
    var s11_: vec4<f32>;
    var s12_: vec4<f32>;
    var s13_: vec4<f32>;
    var s14_: vec4<f32>;
    var m_2: Material;
    var firstTextureTransformIdx: u32;
    var local_2: mat3x3<f32>;
    var local_3: mat3x3<f32>;
    var local_4: mat3x3<f32>;
    var local_5: mat3x3<f32>;
    var local_6: mat3x3<f32>;
    var local_7: mat3x3<f32>;
    var local_8: mat3x3<f32>;
    var local_9: mat3x3<f32>;
    var local_10: mat3x3<f32>;
    var local_11: mat3x3<f32>;
    var local_12: mat3x3<f32>;
    var local_13: mat3x3<f32>;
    var local_14: mat3x3<f32>;
    var local_15: mat3x3<f32>;
    var local_16: mat3x3<f32>;
    var local_17: mat3x3<f32>;

    index_11 = index_10;
    let _e74 = index_11;
    i_2 = (_e74 * 47u);
    let _e79 = i_2;
    let _e82 = texelFetch1D(tex_t_6, tex_s_6, (_e79 + 0u));
    s0_1 = _e82;
    let _e84 = i_2;
    let _e87 = texelFetch1D(tex_t_6, tex_s_6, (_e84 + 1u));
    s1_1 = _e87;
    let _e89 = i_2;
    let _e92 = texelFetch1D(tex_t_6, tex_s_6, (_e89 + 2u));
    s2_1 = _e92;
    let _e94 = i_2;
    let _e97 = texelFetch1D(tex_t_6, tex_s_6, (_e94 + 3u));
    s3_1 = _e97;
    let _e99 = i_2;
    let _e102 = texelFetch1D(tex_t_6, tex_s_6, (_e99 + 4u));
    s4_1 = _e102;
    let _e104 = i_2;
    let _e107 = texelFetch1D(tex_t_6, tex_s_6, (_e104 + 5u));
    s5_1 = _e107;
    let _e109 = i_2;
    let _e112 = texelFetch1D(tex_t_6, tex_s_6, (_e109 + 6u));
    s6_ = _e112;
    let _e114 = i_2;
    let _e117 = texelFetch1D(tex_t_6, tex_s_6, (_e114 + 7u));
    s7_ = _e117;
    let _e119 = i_2;
    let _e122 = texelFetch1D(tex_t_6, tex_s_6, (_e119 + 8u));
    s8_ = _e122;
    let _e124 = i_2;
    let _e127 = texelFetch1D(tex_t_6, tex_s_6, (_e124 + 9u));
    s9_ = _e127;
    let _e129 = i_2;
    let _e132 = texelFetch1D(tex_t_6, tex_s_6, (_e129 + 10u));
    s10_ = _e132;
    let _e134 = i_2;
    let _e137 = texelFetch1D(tex_t_6, tex_s_6, (_e134 + 11u));
    s11_ = _e137;
    let _e139 = i_2;
    let _e142 = texelFetch1D(tex_t_6, tex_s_6, (_e139 + 12u));
    s12_ = _e142;
    let _e144 = i_2;
    let _e147 = texelFetch1D(tex_t_6, tex_s_6, (_e144 + 13u));
    s13_ = _e147;
    let _e149 = i_2;
    let _e152 = texelFetch1D(tex_t_6, tex_s_6, (_e149 + 14u));
    s14_ = _e152;
    let _e156 = s0_1;
    m_2.color = _e156.xyz;
    let _e159 = s0_1;
    m_2.map = i32(round(_e159.w));
    let _e164 = s1_1;
    m_2.metalness = _e164.x;
    let _e167 = s1_1;
    m_2.metalnessMap = i32(round(_e167.y));
    let _e172 = s1_1;
    m_2.roughness = _e172.z;
    let _e175 = s1_1;
    m_2.roughnessMap = i32(round(_e175.w));
    let _e180 = s2_1;
    m_2.ior = _e180.x;
    let _e183 = s2_1;
    m_2.transmission = _e183.y;
    let _e186 = s2_1;
    m_2.transmissionMap = i32(round(_e186.z));
    let _e191 = s2_1;
    m_2.emissiveIntensity = _e191.w;
    let _e194 = s3_1;
    m_2.emissive = _e194.xyz;
    let _e197 = s3_1;
    m_2.emissiveMap = i32(round(_e197.w));
    let _e202 = s4_1;
    m_2.normalMap = i32(round(_e202.x));
    let _e207 = s4_1;
    m_2.normalScale = _e207.yz;
    let _e210 = s4_1;
    m_2.clearcoat = _e210.w;
    let _e213 = s5_1;
    m_2.clearcoatMap = i32(round(_e213.x));
    let _e218 = s5_1;
    m_2.clearcoatRoughness = _e218.y;
    let _e221 = s5_1;
    m_2.clearcoatRoughnessMap = i32(round(_e221.z));
    let _e226 = s5_1;
    m_2.clearcoatNormalMap = i32(round(_e226.w));
    let _e231 = s6_;
    m_2.clearcoatNormalScale = _e231.xy;
    let _e234 = s6_;
    m_2.sheen = _e234.w;
    let _e237 = s7_;
    m_2.sheenColor = _e237.xyz;
    let _e240 = s7_;
    m_2.sheenColorMap = i32(round(_e240.w));
    let _e245 = s8_;
    m_2.sheenRoughness = _e245.x;
    let _e248 = s8_;
    m_2.sheenRoughnessMap = i32(round(_e248.y));
    let _e253 = s8_;
    m_2.iridescenceMap = i32(round(_e253.z));
    let _e258 = s8_;
    m_2.iridescenceThicknessMap = i32(round(_e258.w));
    let _e263 = s9_;
    m_2.iridescence = _e263.x;
    let _e266 = s9_;
    m_2.iridescenceIor = _e266.y;
    let _e269 = s9_;
    m_2.iridescenceThicknessMinimum = _e269.z;
    let _e272 = s9_;
    m_2.iridescenceThicknessMaximum = _e272.w;
    let _e275 = s10_;
    m_2.specularColor = _e275.xyz;
    let _e278 = s10_;
    m_2.specularColorMap = i32(round(_e278.w));
    let _e283 = s11_;
    m_2.specularIntensity = _e283.x;
    let _e286 = s11_;
    m_2.specularIntensityMap = i32(round(_e286.y));
    let _e291 = s11_;
    m_2.thinFilm = bool(_e291.z);
    let _e295 = s12_;
    m_2.attenuationColor = _e295.xyz;
    let _e298 = s12_;
    m_2.attenuationDistance = _e298.w;
    let _e301 = s13_;
    m_2.alphaMap = i32(round(_e301.x));
    let _e306 = s13_;
    m_2.opacity = _e306.y;
    let _e309 = s13_;
    m_2.alphaTest = _e309.z;
    let _e312 = s13_;
    m_2.side = _e312.w;
    let _e315 = s14_;
    m_2.matte = bool(_e315.x);
    let _e319 = s14_;
    m_2.castShadow = bool(_e319.y);
    let _e323 = s14_;
    m_2.vertexColors = bool((i32(_e323.z) & 1i));
    let _e330 = s14_;
    m_2.flatShading = bool((i32(_e330.z) & 2i));
    let _e337 = s14_;
    m_2.fogVolume = bool((i32(_e337.z) & 4i));
    let _e344 = s14_;
    m_2.transparent = bool(_e344.w);
    let _e347 = i_2;
    firstTextureTransformIdx = (_e347 + 15u);
    let _e352 = m_2;
    if (_e352.map == -1i) {
        local_2 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e363 = firstTextureTransformIdx;
        let _e364 = readTextureTransform(tex_t_6, tex_s_6, _e363);
        local_2 = _e364;
    }
    let _e366 = local_2;
    m_2.mapTransform = _e366;
    let _e368 = m_2;
    if (_e368.metalnessMap == -1i) {
        local_3 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e379 = firstTextureTransformIdx;
        let _e382 = readTextureTransform(tex_t_6, tex_s_6, (_e379 + 2u));
        local_3 = _e382;
    }
    let _e384 = local_3;
    m_2.metalnessMapTransform = _e384;
    let _e386 = m_2;
    if (_e386.roughnessMap == -1i) {
        local_4 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e397 = firstTextureTransformIdx;
        let _e400 = readTextureTransform(tex_t_6, tex_s_6, (_e397 + 4u));
        local_4 = _e400;
    }
    let _e402 = local_4;
    m_2.roughnessMapTransform = _e402;
    let _e404 = m_2;
    if (_e404.transmissionMap == -1i) {
        local_5 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e415 = firstTextureTransformIdx;
        let _e418 = readTextureTransform(tex_t_6, tex_s_6, (_e415 + 6u));
        local_5 = _e418;
    }
    let _e420 = local_5;
    m_2.transmissionMapTransform = _e420;
    let _e422 = m_2;
    if (_e422.emissiveMap == -1i) {
        local_6 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e433 = firstTextureTransformIdx;
        let _e436 = readTextureTransform(tex_t_6, tex_s_6, (_e433 + 8u));
        local_6 = _e436;
    }
    let _e438 = local_6;
    m_2.emissiveMapTransform = _e438;
    let _e440 = m_2;
    if (_e440.normalMap == -1i) {
        local_7 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e451 = firstTextureTransformIdx;
        let _e454 = readTextureTransform(tex_t_6, tex_s_6, (_e451 + 10u));
        local_7 = _e454;
    }
    let _e456 = local_7;
    m_2.normalMapTransform = _e456;
    let _e458 = m_2;
    if (_e458.clearcoatMap == -1i) {
        local_8 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e469 = firstTextureTransformIdx;
        let _e472 = readTextureTransform(tex_t_6, tex_s_6, (_e469 + 12u));
        local_8 = _e472;
    }
    let _e474 = local_8;
    m_2.clearcoatMapTransform = _e474;
    let _e476 = m_2;
    if (_e476.clearcoatNormalMap == -1i) {
        local_9 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e487 = firstTextureTransformIdx;
        let _e490 = readTextureTransform(tex_t_6, tex_s_6, (_e487 + 14u));
        local_9 = _e490;
    }
    let _e492 = local_9;
    m_2.clearcoatNormalMapTransform = _e492;
    let _e494 = m_2;
    if (_e494.clearcoatRoughnessMap == -1i) {
        local_10 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e505 = firstTextureTransformIdx;
        let _e508 = readTextureTransform(tex_t_6, tex_s_6, (_e505 + 16u));
        local_10 = _e508;
    }
    let _e510 = local_10;
    m_2.clearcoatRoughnessMapTransform = _e510;
    let _e512 = m_2;
    if (_e512.sheenColorMap == -1i) {
        local_11 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e523 = firstTextureTransformIdx;
        let _e526 = readTextureTransform(tex_t_6, tex_s_6, (_e523 + 18u));
        local_11 = _e526;
    }
    let _e528 = local_11;
    m_2.sheenColorMapTransform = _e528;
    let _e530 = m_2;
    if (_e530.sheenRoughnessMap == -1i) {
        local_12 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e541 = firstTextureTransformIdx;
        let _e544 = readTextureTransform(tex_t_6, tex_s_6, (_e541 + 20u));
        local_12 = _e544;
    }
    let _e546 = local_12;
    m_2.sheenRoughnessMapTransform = _e546;
    let _e548 = m_2;
    if (_e548.iridescenceMap == -1i) {
        local_13 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e559 = firstTextureTransformIdx;
        let _e562 = readTextureTransform(tex_t_6, tex_s_6, (_e559 + 22u));
        local_13 = _e562;
    }
    let _e564 = local_13;
    m_2.iridescenceMapTransform = _e564;
    let _e566 = m_2;
    if (_e566.iridescenceThicknessMap == -1i) {
        local_14 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e577 = firstTextureTransformIdx;
        let _e580 = readTextureTransform(tex_t_6, tex_s_6, (_e577 + 24u));
        local_14 = _e580;
    }
    let _e582 = local_14;
    m_2.iridescenceThicknessMapTransform = _e582;
    let _e584 = m_2;
    if (_e584.specularColorMap == -1i) {
        local_15 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e595 = firstTextureTransformIdx;
        let _e598 = readTextureTransform(tex_t_6, tex_s_6, (_e595 + 26u));
        local_15 = _e598;
    }
    let _e600 = local_15;
    m_2.specularColorMapTransform = _e600;
    let _e602 = m_2;
    if (_e602.specularIntensityMap == -1i) {
        local_16 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e613 = firstTextureTransformIdx;
        let _e616 = readTextureTransform(tex_t_6, tex_s_6, (_e613 + 28u));
        local_16 = _e616;
    }
    let _e618 = local_16;
    m_2.specularIntensityMapTransform = _e618;
    let _e620 = m_2;
    if (_e620.alphaMap == -1i) {
        local_17 = mat3x3<f32>(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 0f, 1f));
    } else {
        let _e631 = firstTextureTransformIdx;
        let _e634 = readTextureTransform(tex_t_6, tex_s_6, (_e631 + 30u));
        local_17 = _e634;
    }
    let _e636 = local_17;
    m_2.alphaMapTransform = _e636;
    let _e637 = m_2;
    return _e637;
}

fn rand4_(v_11: i32) -> vec4<f32> {
    var v_12: i32;
    var uv_5: vec2<i32>;
    var stratifiedSample: vec4<f32>;

    v_12 = v_11;
    let _e76 = v_12;
    let _e77 = sobolBounceIndex;
    uv_5 = vec2<i32>(_e76, i32(_e77));
    let _e81 = uv_5;
    let _e83 = textureLoad(stratifiedTexture_t, _e81, 0i);
    stratifiedSample = _e83;
    let _e85 = stratifiedSample;
    let _e86 = pixelSeed;
    return fract((_e85 + vec4(_e86.x)));
}

fn rand3_(v_13: i32) -> vec3<f32> {
    var v_14: i32;

    v_14 = v_13;
    let _e76 = v_14;
    let _e77 = rand4_(_e76);
    return _e77.xyz;
}

fn rand2_(v_15: i32) -> vec2<f32> {
    var v_16: i32;

    v_16 = v_15;
    let _e76 = v_16;
    let _e77 = rand4_(_e76);
    return _e77.xy;
}

fn rand_1(v_17: i32) -> f32 {
    var v_18: i32;

    v_18 = v_17;
    let _e76 = v_18;
    let _e77 = rand4_(_e76);
    return _e77.x;
}

fn rng_initialize(screenCoord: vec2<f32>, frame: i32) {
    var screenCoord_1: vec2<f32>;
    var frame_1: i32;
    var noiseSize: vec2<i32>;
    var pixel: vec2<i32>;
    var pixelWidth: vec2<f32>;
    var uv_6: vec2<f32>;

    screenCoord_1 = screenCoord;
    frame_1 = frame;
    let _e79 = textureDimensions(stratifiedOffsetTexture_t, 0i);
    noiseSize = vec2<i32>(vec2<i32>(_e79));
    let _e83 = screenCoord_1;
    let _e86 = noiseSize;
    pixel = (vec2<i32>(_e83.xy) % _e86);
    let _e90 = noiseSize;
    pixelWidth = (vec2(1f) / vec2<f32>(_e90));
    let _e95 = pixel;
    let _e97 = pixelWidth;
    let _e99 = pixelWidth;
    uv_6 = ((vec2<f32>(_e95) * _e97) + (_e99 * 0.5f));
    let _e104 = uv_6;
    let _e105 = textureSample(stratifiedOffsetTexture_t, pt_nearest, _e104);
    pixelSeed = _e105;
    return;
}

fn texelFetch1D_1(tex_t_7: texture_2d_array<f32>, tex_s_7: sampler, layer: i32, index_12: u32) -> vec4<f32> {
    var layer_1: i32;
    var index_13: u32;
    var width_3: u32;
    var uv_7: vec2<u32>;

    layer_1 = layer;
    index_13 = index_12;
    let _e81 = textureDimensions(tex_t_7, 0i);
    let _e84 = textureNumLayers(tex_t_7);
    width_3 = u32(vec3<i32>(vec3<u32>(_e81.x, _e81.y, _e84)).x);
    let _e92 = index_13;
    let _e93 = width_3;
    uv_7.x = (_e92 % _e93);
    let _e96 = index_13;
    let _e97 = width_3;
    uv_7.y = (_e96 / _e97);
    let _e99 = uv_7;
    let _e100 = layer_1;
    let _e101 = vec2<i32>(_e99);
    let _e104 = vec3<i32>(_e101.x, _e101.y, _e100);
    let _e108 = textureLoad(tex_t_7, _e104.xy, _e104.z, 0i);
    return _e108;
}

fn textureSampleBarycoord_1(tex_t_8: texture_2d_array<f32>, tex_s_8: sampler, layer_2: i32, barycoord_5: vec3<f32>, faceIndices_4: vec3<u32>) -> vec4<f32> {
    var layer_3: i32;
    var barycoord_6: vec3<f32>;
    var faceIndices_5: vec3<u32>;

    layer_3 = layer_2;
    barycoord_6 = barycoord_5;
    faceIndices_5 = faceIndices_4;
    let _e82 = barycoord_6;
    let _e84 = layer_3;
    let _e85 = faceIndices_5;
    let _e87 = texelFetch1D_1(tex_t_8, tex_s_8, _e84, _e85.x);
    let _e89 = barycoord_6;
    let _e91 = layer_3;
    let _e92 = faceIndices_5;
    let _e94 = texelFetch1D_1(tex_t_8, tex_s_8, _e91, _e92.y);
    let _e97 = barycoord_6;
    let _e99 = layer_3;
    let _e100 = faceIndices_5;
    let _e102 = texelFetch1D_1(tex_t_8, tex_s_8, _e99, _e100.z);
    return (((_e82.x * _e87) + (_e89.y * _e94)) + (_e97.z * _e102));
}

fn totalInternalReflection(cosTheta: f32, eta: f32) -> bool {
    var cosTheta_1: f32;
    var eta_1: f32;
    var sinTheta: f32;

    cosTheta_1 = cosTheta;
    eta_1 = eta;
    let _e79 = cosTheta_1;
    let _e80 = cosTheta_1;
    sinTheta = sqrt((1f - (_e79 * _e80)));
    let _e85 = eta_1;
    let _e86 = sinTheta;
    return ((_e85 * _e86) > 1f);
}

fn schlickFresnel(cosine: f32, f0_4: f32) -> f32 {
    var cosine_1: f32;
    var f0_5: f32;

    cosine_1 = cosine;
    f0_5 = f0_4;
    let _e78 = f0_5;
    let _e80 = f0_5;
    let _e83 = cosine_1;
    let _e86 = pt_pow((1f - _e83), 5f);
    return (_e78 + ((1f - _e80) * _e86));
}

fn schlickFresnel_1(cosine_2: f32, f0_6: vec3<f32>) -> vec3<f32> {
    var cosine_3: f32;
    var f0_7: vec3<f32>;

    cosine_3 = cosine_2;
    f0_7 = f0_6;
    let _e78 = f0_7;
    let _e80 = f0_7;
    let _e84 = cosine_3;
    let _e87 = pt_pow((1f - _e84), 5f);
    return (_e78 + ((vec3(1f) - _e80) * _e87));
}

fn schlickFresnel_2(cosine_4: f32, f0_8: vec3<f32>, f90_4: vec3<f32>) -> vec3<f32> {
    var cosine_5: f32;
    var f0_9: vec3<f32>;
    var f90_5: vec3<f32>;

    cosine_5 = cosine_4;
    f0_9 = f0_8;
    f90_5 = f90_4;
    let _e80 = f0_9;
    let _e81 = f90_5;
    let _e82 = f0_9;
    let _e85 = cosine_5;
    let _e88 = pt_pow((1f - _e85), 5f);
    return (_e80 + ((_e81 - _e82) * _e88));
}

fn dielectricFresnel(cosThetaI: f32, eta_2: f32) -> f32 {
    var cosThetaI_1: f32;
    var eta_3: f32;
    var ni: f32;
    var nt: f32 = 1f;
    var sinThetaISq: f32;
    var sinThetaTSq: f32;
    var sinThetaT: f32;
    var cosThetaT: f32;
    var rParallel: f32;
    var rPerpendicular: f32;

    cosThetaI_1 = cosThetaI;
    eta_3 = eta_2;
    let _e78 = eta_3;
    ni = _e78;
    let _e83 = cosThetaI_1;
    let _e84 = cosThetaI_1;
    sinThetaISq = (1f - (_e83 * _e84));
    let _e88 = eta_3;
    let _e89 = eta_3;
    let _e91 = sinThetaISq;
    sinThetaTSq = ((_e88 * _e89) * _e91);
    let _e94 = sinThetaTSq;
    if (_e94 >= 1f) {
        {
            return 1f;
        }
    }
    let _e98 = sinThetaTSq;
    sinThetaT = sqrt(_e98);
    let _e103 = sinThetaT;
    let _e104 = sinThetaT;
    cosThetaT = sqrt(max(0f, (1f - (_e103 * _e104))));
    let _e110 = nt;
    let _e111 = cosThetaI_1;
    let _e113 = ni;
    let _e114 = cosThetaT;
    let _e117 = nt;
    let _e118 = cosThetaI_1;
    let _e120 = ni;
    let _e121 = cosThetaT;
    rParallel = (((_e110 * _e111) - (_e113 * _e114)) / ((_e117 * _e118) + (_e120 * _e121)));
    let _e126 = ni;
    let _e127 = cosThetaI_1;
    let _e129 = nt;
    let _e130 = cosThetaT;
    let _e133 = ni;
    let _e134 = cosThetaI_1;
    let _e136 = nt;
    let _e137 = cosThetaT;
    rPerpendicular = (((_e126 * _e127) - (_e129 * _e130)) / ((_e133 * _e134) + (_e136 * _e137)));
    let _e142 = rParallel;
    let _e143 = rParallel;
    let _e145 = rPerpendicular;
    let _e146 = rPerpendicular;
    return (((_e142 * _e143) + (_e145 * _e146)) / 2f);
}

fn iorRatioToF0_(eta_4: f32) -> f32 {
    var eta_5: f32;

    eta_5 = eta_4;
    let _e77 = eta_5;
    let _e80 = eta_5;
    let _e84 = pt_pow(((1f - _e77) / (1f + _e80)), 2f);
    return _e84;
}

fn evaluateFresnel(cosTheta_2: f32, eta_6: f32, f0_10: vec3<f32>, f90_6: vec3<f32>) -> vec3<f32> {
    var cosTheta_3: f32;
    var eta_7: f32;
    var f0_11: vec3<f32>;
    var f90_7: vec3<f32>;

    cosTheta_3 = cosTheta_2;
    eta_7 = eta_6;
    f0_11 = f0_10;
    f90_7 = f90_6;
    let _e82 = cosTheta_3;
    let _e83 = eta_7;
    let _e84 = totalInternalReflection(_e82, _e83);
    if _e84 {
        {
            let _e85 = f90_7;
            return _e85;
        }
    }
    let _e86 = cosTheta_3;
    let _e87 = f0_11;
    let _e88 = f90_7;
    let _e89 = schlickFresnel_2(_e86, _e87, _e88);
    return _e89;
}

fn disneyFresnel(wo: vec3<f32>, wi: vec3<f32>, wh: vec3<f32>, f0_12: f32, eta_8: f32, metalness: f32) -> f32 {
    var wo_1: vec3<f32>;
    var wi_1: vec3<f32>;
    var wh_1: vec3<f32>;
    var f0_13: f32;
    var eta_9: f32;
    var metalness_1: f32;
    var dotHV: f32;
    var dotHL: f32;
    var dielectricFresnel_1: f32;
    var metallicFresnel: f32;

    wo_1 = wo;
    wi_1 = wi;
    wh_1 = wh;
    f0_13 = f0_12;
    eta_9 = eta_8;
    metalness_1 = metalness;
    let _e86 = wo_1;
    let _e87 = wh_1;
    dotHV = dot(_e86, _e87);
    let _e90 = dotHV;
    let _e91 = eta_9;
    let _e92 = totalInternalReflection(_e90, _e91);
    if _e92 {
        {
            return 1f;
        }
    }
    let _e94 = wi_1;
    let _e95 = wh_1;
    dotHL = dot(_e94, _e95);
    let _e98 = dotHV;
    let _e100 = eta_9;
    let _e101 = dielectricFresnel(abs(_e98), _e100);
    dielectricFresnel_1 = _e101;
    let _e103 = dotHL;
    let _e104 = f0_13;
    let _e105 = schlickFresnel(_e103, _e104);
    metallicFresnel = _e105;
    let _e107 = dielectricFresnel_1;
    let _e108 = metallicFresnel;
    let _e109 = metalness_1;
    return mix(_e107, _e108, _e109);
}

fn stepRayOrigin(rayOrigin_11: vec3<f32>, rayDirection_11: vec3<f32>, offset_3: vec3<f32>, dist_5: f32) -> vec3<f32> {
    var rayOrigin_12: vec3<f32>;
    var rayDirection_12: vec3<f32>;
    var offset_4: vec3<f32>;
    var dist_6: f32;
    var point: vec3<f32>;
    var absPoint: vec3<f32>;
    var maxPoint: f32;

    rayOrigin_12 = rayOrigin_11;
    rayDirection_12 = rayDirection_11;
    offset_4 = offset_3;
    dist_6 = dist_5;
    let _e82 = rayOrigin_12;
    let _e83 = rayDirection_12;
    let _e84 = dist_6;
    point = (_e82 + (_e83 * _e84));
    let _e88 = point;
    absPoint = abs(_e88);
    let _e91 = absPoint;
    let _e93 = absPoint;
    let _e95 = absPoint;
    maxPoint = max(_e91.x, max(_e93.y, _e95.z));
    let _e100 = point;
    let _e101 = offset_4;
    let _e102 = maxPoint;
    return (_e100 + ((_e101 * (_e102 + 1f)) * 0.0001f));
}

fn transmissionAttenuation(dist_7: f32, attColor: vec3<f32>, attDist: f32) -> vec3<f32> {
    var dist_8: f32;
    var attColor_1: vec3<f32>;
    var attDist_1: f32;
    var ot: vec3<f32>;

    dist_8 = dist_7;
    attColor_1 = attColor;
    attDist_1 = attDist;
    let _e80 = attColor_1;
    let _e83 = attDist_1;
    ot = (-(log(_e80)) / vec3(_e83));
    let _e87 = ot;
    let _e89 = dist_8;
    return exp((-(_e87) * _e89));
}

fn getHalfVector(wi_2: vec3<f32>, wo_2: vec3<f32>, eta_10: f32) -> vec3<f32> {
    var wi_3: vec3<f32>;
    var wo_3: vec3<f32>;
    var eta_11: f32;
    var h: vec3<f32>;

    wi_3 = wi_2;
    wo_3 = wo_2;
    eta_11 = eta_10;
    let _e81 = wi_3;
    if (_e81.z > 0f) {
        {
            let _e85 = wi_3;
            let _e86 = wo_3;
            h = normalize((_e85 + _e86));
        }
    } else {
        {
            let _e89 = wi_3;
            let _e90 = wo_3;
            let _e91 = eta_11;
            h = normalize((_e89 + (_e90 * _e91)));
        }
    }
    let _e95 = h;
    let _e96 = h;
    h = (_e95 * sign(_e96.z));
    let _e100 = h;
    return _e100;
}

fn getHalfVector_1(a_6: vec3<f32>, b_6: vec3<f32>) -> vec3<f32> {
    var a_7: vec3<f32>;
    var b_7: vec3<f32>;

    a_7 = a_6;
    b_7 = b_6;
    let _e78 = a_7;
    let _e79 = b_7;
    return normalize((_e78 + _e79));
}

fn isDirectionValid(direction_1: vec3<f32>, surfaceNormal: vec3<f32>, geometryNormal: vec3<f32>) -> bool {
    var direction_2: vec3<f32>;
    var surfaceNormal_1: vec3<f32>;
    var geometryNormal_1: vec3<f32>;
    var aboveSurfaceNormal: bool;
    var aboveGeometryNormal: bool;

    direction_2 = direction_1;
    surfaceNormal_1 = surfaceNormal;
    geometryNormal_1 = geometryNormal;
    let _e80 = direction_2;
    let _e81 = surfaceNormal_1;
    aboveSurfaceNormal = (dot(_e80, _e81) > 0f);
    let _e86 = direction_2;
    let _e87 = geometryNormal_1;
    aboveGeometryNormal = (dot(_e86, _e87) > 0f);
    let _e92 = aboveSurfaceNormal;
    let _e93 = aboveGeometryNormal;
    return (_e92 == _e93);
}

fn equirectDirectionToUv(direction_3: vec3<f32>) -> vec2<f32> {
    var direction_4: vec3<f32>;
    var uv_8: vec2<f32>;

    direction_4 = direction_3;
    let _e76 = direction_4;
    let _e78 = direction_4;
    let _e81 = direction_4;
    uv_8 = vec2<f32>(atan2(_e76.z, _e78.x), acos(_e81.y));
    let _e86 = uv_8;
    uv_8 = (_e86 / vec2<f32>(6.2831855f, 3.1415927f));
    let _e94 = uv_8;
    uv_8.x = (_e94.x + 0.5f);
    let _e100 = uv_8;
    uv_8.y = (1f - _e100.y);
    let _e103 = uv_8;
    return _e103;
}

fn equirectUvToDirection(uv_9: vec2<f32>) -> vec3<f32> {
    var uv_10: vec2<f32>;
    var theta: f32;
    var phi: f32;
    var sinPhi: f32;

    uv_10 = uv_9;
    let _e77 = uv_10;
    uv_10.x = (_e77.x - 0.5f);
    let _e83 = uv_10;
    uv_10.y = (1f - _e83.y);
    let _e86 = uv_10;
    theta = ((_e86.x * 2f) * 3.1415927f);
    let _e93 = uv_10;
    phi = (_e93.y * 3.1415927f);
    let _e98 = phi;
    sinPhi = sin(_e98);
    let _e101 = sinPhi;
    let _e102 = theta;
    let _e105 = phi;
    let _e107 = sinPhi;
    let _e108 = theta;
    return vec3<f32>((_e101 * cos(_e102)), cos(_e105), (_e107 * sin(_e108)));
}

fn misHeuristic(a_8: f32, b_8: f32) -> f32 {
    var a_9: f32;
    var b_9: f32;
    var aa: f32;
    var bb: f32;

    a_9 = a_8;
    b_9 = b_8;
    let _e78 = a_9;
    let _e79 = a_9;
    aa = (_e78 * _e79);
    let _e82 = b_9;
    let _e83 = b_9;
    bb = (_e82 * _e83);
    let _e86 = aa;
    let _e87 = aa;
    let _e88 = bb;
    return (_e86 / (_e87 + _e88));
}

fn tentFilter(x_16: f32) -> f32 {
    var x_17: f32;
    var local_18: f32;

    x_17 = x_16;
    let _e76 = x_17;
    if (_e76 < 0.5f) {
        let _e80 = x_17;
        local_18 = (sqrt((2f * _e80)) - 1f);
    } else {
        let _e88 = x_17;
        local_18 = (1f - sqrt((2f - (2f * _e88))));
    }
    let _e94 = local_18;
    return _e94;
}

fn acosApprox(x_18: f32) -> f32 {
    var x_19: f32;

    x_19 = x_18;
    let _e76 = x_19;
    x_19 = clamp(_e76, -1f, 1f);
    let _e83 = x_19;
    let _e85 = x_19;
    let _e89 = x_19;
    return (((((-0.6981317f * _e83) * _e85) - 0.87266463f) * _e89) + 1.5707964f);
}

fn acosSafe(x_20: f32) -> f32 {
    var x_21: f32;

    x_21 = x_20;
    let _e76 = x_21;
    return acos(clamp(_e76, -1f, 1f));
}

fn saturateCos(val: f32) -> f32 {
    var val_1: f32;

    val_1 = val;
    let _e76 = val_1;
    return clamp(_e76, 0.001f, 1f);
}

fn square(t_1: f32) -> f32 {
    var t_2: f32;

    t_2 = t_1;
    let _e76 = t_2;
    let _e77 = t_2;
    return (_e76 * _e77);
}

fn square_1(t_3: vec2<f32>) -> vec2<f32> {
    var t_4: vec2<f32>;

    t_4 = t_3;
    let _e76 = t_4;
    let _e77 = t_4;
    return (_e76 * _e77);
}

fn square_2(t_5: vec3<f32>) -> vec3<f32> {
    var t_6: vec3<f32>;

    t_6 = t_5;
    let _e76 = t_6;
    let _e77 = t_6;
    return (_e76 * _e77);
}

fn square_3(t_7: vec4<f32>) -> vec4<f32> {
    var t_8: vec4<f32>;

    t_8 = t_7;
    let _e76 = t_8;
    let _e77 = t_8;
    return (_e76 * _e77);
}

fn rotateVector(v_19: vec2<f32>, t_9: f32) -> vec2<f32> {
    var v_20: vec2<f32>;
    var t_10: f32;
    var ac: f32;
    var as_: f32;

    v_20 = v_19;
    t_10 = t_9;
    let _e78 = t_10;
    ac = cos(_e78);
    let _e81 = t_10;
    as_ = sin(_e81);
    let _e84 = v_20;
    let _e86 = ac;
    let _e88 = v_20;
    let _e90 = as_;
    let _e93 = v_20;
    let _e95 = as_;
    let _e97 = v_20;
    let _e99 = ac;
    return vec2<f32>(((_e84.x * _e86) - (_e88.y * _e90)), ((_e93.x * _e95) + (_e97.y * _e99)));
}

fn getBasisFromNormal(normal_2: vec3<f32>) -> mat3x3<f32> {
    var normal_3: vec3<f32>;
    var other: vec3<f32>;
    var ortho: vec3<f32>;
    var ortho2_: vec3<f32>;

    normal_3 = normal_2;
    let _e77 = normal_3;
    if (abs(_e77.x) > 0.5f) {
        {
            other = vec3<f32>(0f, 1f, 0f);
        }
    } else {
        {
            other = vec3<f32>(1f, 0f, 0f);
        }
    }
    let _e90 = normal_3;
    let _e91 = other;
    ortho = normalize(cross(_e90, _e91));
    let _e95 = normal_3;
    let _e96 = ortho;
    ortho2_ = normalize(cross(_e95, _e96));
    let _e100 = ortho2_;
    let _e101 = ortho;
    let _e102 = normal_3;
    return mat3x3<f32>(vec3<f32>(_e100.x, _e100.y, _e100.z), vec3<f32>(_e101.x, _e101.y, _e101.z), vec3<f32>(_e102.x, _e102.y, _e102.z));
}

fn intersectsRectangle(center: vec3<f32>, normal_4: vec3<f32>, u_1: vec3<f32>, v_21: vec3<f32>, rayOrigin_13: vec3<f32>, rayDirection_13: vec3<f32>, dist_9: ptr<function, f32>) -> bool {
    var center_1: vec3<f32>;
    var normal_5: vec3<f32>;
    var u_2: vec3<f32>;
    var v_22: vec3<f32>;
    var rayOrigin_14: vec3<f32>;
    var rayDirection_14: vec3<f32>;
    var t_11: f32;
    var p: vec3<f32>;
    var vi: vec3<f32>;
    var a1_: f32;
    var a2_: f32;

    center_1 = center;
    normal_5 = normal_4;
    u_2 = u_1;
    v_22 = v_21;
    rayOrigin_14 = rayOrigin_13;
    rayDirection_14 = rayDirection_13;
    let _e87 = center_1;
    let _e88 = rayOrigin_14;
    let _e90 = normal_5;
    let _e92 = rayDirection_14;
    let _e93 = normal_5;
    t_11 = (dot((_e87 - _e88), _e90) / dot(_e92, _e93));
    let _e97 = t_11;
    if (_e97 > 0.000001f) {
        {
            let _e100 = rayOrigin_14;
            let _e101 = rayDirection_14;
            let _e102 = t_11;
            p = (_e100 + (_e101 * _e102));
            let _e106 = p;
            let _e107 = center_1;
            vi = (_e106 - _e107);
            let _e110 = u_2;
            let _e111 = vi;
            a1_ = dot(_e110, _e111);
            let _e114 = a1_;
            if (abs(_e114) <= 0.5f) {
                {
                    let _e118 = v_22;
                    let _e119 = vi;
                    a2_ = dot(_e118, _e119);
                    let _e122 = a2_;
                    if (abs(_e122) <= 0.5f) {
                        {
                            let _e126 = t_11;
                            (*dist_9) = _e126;
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn intersectsCircle(position: vec3<f32>, normal_6: vec3<f32>, u_3: vec3<f32>, v_23: vec3<f32>, rayOrigin_15: vec3<f32>, rayDirection_15: vec3<f32>, dist_10: ptr<function, f32>) -> bool {
    var position_1: vec3<f32>;
    var normal_7: vec3<f32>;
    var u_4: vec3<f32>;
    var v_24: vec3<f32>;
    var rayOrigin_16: vec3<f32>;
    var rayDirection_16: vec3<f32>;
    var t_12: f32;
    var hit: vec3<f32>;
    var vi_1: vec3<f32>;
    var a1_1: f32;
    var a2_1: f32;

    position_1 = position;
    normal_7 = normal_6;
    u_4 = u_3;
    v_24 = v_23;
    rayOrigin_16 = rayOrigin_15;
    rayDirection_16 = rayDirection_15;
    let _e87 = position_1;
    let _e88 = rayOrigin_16;
    let _e90 = normal_7;
    let _e92 = rayDirection_16;
    let _e93 = normal_7;
    t_12 = (dot((_e87 - _e88), _e90) / dot(_e92, _e93));
    let _e97 = t_12;
    if (_e97 > 0.000001f) {
        {
            let _e100 = rayOrigin_16;
            let _e101 = rayDirection_16;
            let _e102 = t_12;
            hit = (_e100 + (_e101 * _e102));
            let _e106 = hit;
            let _e107 = position_1;
            vi_1 = (_e106 - _e107);
            let _e110 = u_4;
            let _e111 = vi_1;
            a1_1 = dot(_e110, _e111);
            let _e114 = v_24;
            let _e115 = vi_1;
            a2_1 = dot(_e114, _e115);
            let _e118 = a1_1;
            let _e119 = a2_1;
            if (length(vec2<f32>(_e118, _e119)) <= 0.5f) {
                {
                    let _e124 = t_12;
                    (*dist_10) = _e124;
                    return true;
                }
            }
        }
    }
    return false;
}

fn sampleHemisphere(n: vec3<f32>, uv_11: vec2<f32>) -> vec3<f32> {
    var n_1: vec3<f32>;
    var uv_12: vec2<f32>;
    var local_19: f32;
    var pt_sign: f32;
    var a_10: f32;
    var b_10: f32;
    var b1_: vec3<f32>;
    var b2_: vec3<f32>;
    var r: f32;
    var theta_1: f32;
    var x_22: f32;
    var y_8: f32;

    n_1 = n;
    uv_12 = uv_11;
    let _e82 = n_1;
    if (_e82.z == 0f) {
        local_19 = 1f;
    } else {
        let _e87 = n_1;
        local_19 = sign(_e87.z);
    }
    let _e91 = local_19;
    pt_sign = _e91;
    let _e95 = pt_sign;
    let _e96 = n_1;
    a_10 = (-1f / (_e95 + _e96.z));
    let _e101 = n_1;
    let _e103 = n_1;
    let _e106 = a_10;
    b_10 = ((_e101.x * _e103.y) * _e106);
    let _e110 = pt_sign;
    let _e111 = n_1;
    let _e114 = n_1;
    let _e117 = a_10;
    let _e120 = pt_sign;
    let _e121 = b_10;
    let _e123 = pt_sign;
    let _e125 = n_1;
    b1_ = vec3<f32>((1f + (((_e110 * _e111.x) * _e114.x) * _e117)), (_e120 * _e121), (-(_e123) * _e125.x));
    let _e130 = b_10;
    let _e131 = pt_sign;
    let _e132 = n_1;
    let _e134 = n_1;
    let _e137 = a_10;
    let _e140 = n_1;
    b2_ = vec3<f32>(_e130, (_e131 + ((_e132.y * _e134.y) * _e137)), -(_e140.y));
    let _e145 = uv_12;
    r = sqrt(_e145.x);
    let _e152 = uv_12;
    theta_1 = (6.2831855f * _e152.y);
    let _e156 = r;
    let _e157 = theta_1;
    x_22 = (_e156 * cos(_e157));
    let _e161 = r;
    let _e162 = theta_1;
    y_8 = (_e161 * sin(_e162));
    let _e166 = x_22;
    let _e167 = b1_;
    let _e169 = y_8;
    let _e170 = b2_;
    let _e174 = uv_12;
    let _e178 = n_1;
    return (((_e166 * _e167) + (_e169 * _e170)) + (sqrt((1f - _e174.x)) * _e178));
}

fn sampleTriangle(a_11: vec2<f32>, b_11: vec2<f32>, c_4: vec2<f32>, r_1: vec2<f32>) -> vec2<f32> {
    var a_12: vec2<f32>;
    var b_12: vec2<f32>;
    var c_5: vec2<f32>;
    var r_2: vec2<f32>;
    var e1_: vec2<f32>;
    var e2_: vec2<f32>;
    var diag: vec2<f32>;

    a_12 = a_11;
    b_12 = b_11;
    c_5 = c_4;
    r_2 = r_1;
    let _e86 = a_12;
    let _e87 = b_12;
    e1_ = (_e86 - _e87);
    let _e90 = c_5;
    let _e91 = b_12;
    e2_ = (_e90 - _e91);
    let _e94 = e1_;
    let _e95 = e2_;
    diag = normalize((_e94 + _e95));
    let _e99 = r_2;
    let _e101 = r_2;
    if ((_e99.x + _e101.y) > 1f) {
        {
            let _e108 = r_2;
            r_2 = (vec2(1f) - _e108);
        }
    }
    let _e110 = e1_;
    let _e111 = r_2;
    let _e114 = e2_;
    let _e115 = r_2;
    return ((_e110 * _e111.x) + (_e114 * _e115.y));
}

fn sampleCircle(uv_13: vec2<f32>) -> vec2<f32> {
    var uv_14: vec2<f32>;
    var angle: f32;
    var radius: f32;

    uv_14 = uv_13;
    let _e83 = uv_14;
    angle = (6.2831855f * _e83.x);
    let _e87 = uv_14;
    radius = sqrt(_e87.y);
    let _e91 = angle;
    let _e93 = angle;
    let _e96 = radius;
    return (vec2<f32>(cos(_e91), sin(_e93)) * _e96);
}

fn sampleSphere(uv_15: vec2<f32>) -> vec3<f32> {
    var uv_16: vec2<f32>;
    var u_5: f32;
    var t_13: f32;
    var f: f32;

    uv_16 = uv_15;
    let _e80 = uv_16;
    u_5 = ((_e80.x - 0.5f) * 2f);
    let _e87 = uv_16;
    t_13 = ((_e87.y * 3.1415927f) * 2f);
    let _e95 = u_5;
    let _e96 = u_5;
    f = sqrt((1f - (_e95 * _e96)));
    let _e101 = f;
    let _e102 = t_13;
    let _e105 = f;
    let _e106 = t_13;
    let _e109 = u_5;
    return vec3<f32>((_e101 * cos(_e102)), (_e105 * sin(_e106)), _e109);
}

fn sampleRegularPolygon(sides: i32, uvw: vec3<f32>) -> vec2<f32> {
    var sides_1: i32;
    var uvw_1: vec3<f32>;
    var r_3: vec3<f32>;
    var anglePerSegment: f32;
    var segment: f32;
    var angle1_: f32;
    var angle2_: f32;
    var a_13: vec2<f32>;
    var b_13: vec2<f32> = vec2<f32>(0f, 0f);
    var c_6: vec2<f32>;

    sides_1 = sides;
    uvw_1 = uvw;
    let _e82 = sides_1;
    sides_1 = max(_e82, 3i);
    let _e85 = uvw_1;
    r_3 = _e85;
    let _e90 = sides_1;
    anglePerSegment = (6.2831855f / f32(_e90));
    let _e94 = sides_1;
    let _e96 = r_3;
    segment = floor((f32(_e94) * _e96.x));
    let _e101 = anglePerSegment;
    let _e102 = segment;
    angle1_ = (_e101 * _e102);
    let _e105 = angle1_;
    let _e106 = anglePerSegment;
    angle2_ = (_e105 + _e106);
    let _e109 = angle1_;
    let _e111 = angle1_;
    a_13 = vec2<f32>(sin(_e109), cos(_e111));
    let _e119 = angle2_;
    let _e121 = angle2_;
    c_6 = vec2<f32>(sin(_e119), cos(_e121));
    let _e125 = a_13;
    let _e126 = b_13;
    let _e127 = c_6;
    let _e128 = r_3;
    let _e130 = sampleTriangle(_e125, _e126, _e127, _e128.yz);
    return _e130;
}

fn sampleAperture(blades: i32, uvw_2: vec3<f32>) -> vec2<f32> {
    var blades_1: i32;
    var uvw_3: vec3<f32>;
    var local_20: vec2<f32>;

    blades_1 = blades;
    uvw_3 = uvw_2;
    let _e82 = blades_1;
    if (_e82 == 0i) {
        let _e85 = uvw_3;
        let _e87 = sampleCircle(_e85.xy);
        local_20 = _e87;
    } else {
        let _e88 = blades_1;
        let _e89 = uvw_3;
        let _e90 = sampleRegularPolygon(_e88, _e89);
        local_20 = _e90;
    }
    let _e92 = local_20;
    return _e92;
}

fn sampleEquirectColor(envMap_t: texture_2d<f32>, envMap_s: sampler, direction_5: vec3<f32>) -> vec3<f32> {
    var direction_6: vec3<f32>;

    direction_6 = direction_5;
    let _e82 = direction_6;
    let _e83 = equirectDirectionToUv(_e82);
    let _e84 = textureSample(envMap_t, envMap_s, _e83);
    return _e84.xyz;
}

fn equirectDirectionPdf(direction_7: vec3<f32>) -> f32 {
    var direction_8: vec3<f32>;
    var uv_17: vec2<f32>;
    var theta_2: f32;
    var sinTheta_1: f32;

    direction_8 = direction_7;
    let _e80 = direction_8;
    let _e81 = equirectDirectionToUv(_e80);
    uv_17 = _e81;
    let _e83 = uv_17;
    theta_2 = (_e83.y * 3.1415927f);
    let _e88 = theta_2;
    sinTheta_1 = sin(_e88);
    let _e91 = sinTheta_1;
    if (_e91 == 0f) {
        {
            return 0f;
        }
    }
    let _e101 = sinTheta_1;
    return (1f / (19.73921f * _e101));
}

fn sampleEquirect(direction_9: vec3<f32>, color: ptr<function, vec3<f32>>) -> f32 {
    var direction_10: vec3<f32>;
    var totalSum: f32;
    var uv_18: vec2<f32>;
    var lum: f32;
    var resolution: vec2<i32>;
    var pdf: f32;

    direction_10 = direction_9;
    let _e81 = global.envMapInfo_totalSum;
    totalSum = _e81;
    let _e83 = totalSum;
    if (_e83 == 0f) {
        {
            (*color) = vec3(0f);
            return 1f;
        }
    }
    let _e89 = direction_10;
    let _e90 = equirectDirectionToUv(_e89);
    uv_18 = _e90;
    let _e92 = uv_18;
    let _e93 = textureSample(envMapInfo_map_t, pt_linear_repeat_u, _e92);
    (*color) = _e93.xyz;
    let _e95 = (*color);
    let _e96 = luminance(_e95);
    lum = _e96;
    let _e99 = textureDimensions(envMapInfo_map_t, 0i);
    resolution = vec2<i32>(_e99);
    let _e102 = lum;
    let _e103 = totalSum;
    pdf = (_e102 / _e103);
    let _e106 = resolution;
    let _e108 = resolution;
    let _e112 = pdf;
    let _e114 = direction_10;
    let _e115 = equirectDirectionPdf(_e114);
    return ((f32((_e106.x * _e108.y)) * _e112) * _e115);
}

fn sampleEquirectProbability(r_4: vec2<f32>, color_1: ptr<function, vec3<f32>>, direction_11: ptr<function, vec3<f32>>) -> f32 {
    var r_5: vec2<f32>;
    var v_25: f32;
    var u_6: f32;
    var uv_19: vec2<f32>;
    var derivedDirection: vec3<f32>;
    var totalSum_1: f32;
    var lum_1: f32;
    var resolution_1: vec2<i32>;
    var pdf_1: f32;

    r_5 = r_4;
    let _e82 = r_5;
    let _e86 = textureSample(envMapInfo_marginalWeights_t, pt_linear, vec2<f32>(_e82.x, 0f));
    v_25 = _e86.x;
    let _e89 = r_5;
    let _e91 = v_25;
    let _e93 = textureSample(envMapInfo_conditionalWeights_t, pt_linear, vec2<f32>(_e89.y, _e91));
    u_6 = _e93.x;
    let _e96 = u_6;
    let _e97 = v_25;
    uv_19 = vec2<f32>(_e96, _e97);
    let _e100 = uv_19;
    let _e101 = equirectUvToDirection(_e100);
    derivedDirection = _e101;
    let _e103 = derivedDirection;
    (*direction_11) = _e103;
    let _e104 = uv_19;
    let _e105 = textureSample(envMapInfo_map_t, pt_linear_repeat_u, _e104);
    (*color_1) = _e105.xyz;
    let _e107 = global.envMapInfo_totalSum;
    totalSum_1 = _e107;
    let _e109 = (*color_1);
    let _e110 = luminance(_e109);
    lum_1 = _e110;
    let _e113 = textureDimensions(envMapInfo_map_t, 0i);
    resolution_1 = vec2<i32>(_e113);
    let _e116 = lum_1;
    let _e117 = totalSum_1;
    pdf_1 = (_e116 / _e117);
    let _e120 = resolution_1;
    let _e122 = resolution_1;
    let _e126 = pdf_1;
    let _e128 = (*direction_11);
    let _e129 = equirectDirectionPdf(_e128);
    return ((f32((_e120.x * _e122.y)) * _e126) * _e129);
}

fn getSpotAttenuation(coneCosine: f32, penumbraCosine: f32, angleCosine: f32) -> f32 {
    var coneCosine_1: f32;
    var penumbraCosine_1: f32;
    var angleCosine_1: f32;

    coneCosine_1 = coneCosine;
    penumbraCosine_1 = penumbraCosine;
    angleCosine_1 = angleCosine;
    let _e84 = coneCosine_1;
    let _e85 = penumbraCosine_1;
    let _e86 = angleCosine_1;
    return smoothstep(_e84, _e85, _e86);
}

fn getDistanceAttenuation(lightDistance: f32, cutoffDistance: f32, decayExponent: f32) -> f32 {
    var lightDistance_1: f32;
    var cutoffDistance_1: f32;
    var decayExponent_1: f32;
    var distanceFalloff: f32;

    lightDistance_1 = lightDistance;
    cutoffDistance_1 = cutoffDistance;
    decayExponent_1 = decayExponent;
    let _e85 = lightDistance_1;
    let _e86 = decayExponent_1;
    let _e87 = pt_pow(_e85, _e86);
    distanceFalloff = (1f / max(_e87, 0.000001f));
    let _e92 = cutoffDistance_1;
    if (_e92 > 0f) {
        {
            let _e95 = distanceFalloff;
            let _e97 = lightDistance_1;
            let _e98 = cutoffDistance_1;
            let _e100 = pow4_((_e97 / _e98));
            let _e105 = pow2_(clamp((1f - _e100), 0f, 1f));
            distanceFalloff = (_e95 * _e105);
        }
    }
    let _e107 = distanceFalloff;
    return _e107;
}

fn getPhotometricAttenuation(iesProfiles_t: texture_2d_array<f32>, iesProfiles_s: sampler, iesProfile: i32, posToLight: vec3<f32>, lightDir: vec3<f32>, u_7: vec3<f32>, v_26: vec3<f32>) -> f32 {
    var iesProfile_1: i32;
    var posToLight_1: vec3<f32>;
    var lightDir_1: vec3<f32>;
    var u_8: vec3<f32>;
    var v_27: vec3<f32>;
    var cosTheta_4: f32;
    var angle_1: f32;

    iesProfile_1 = iesProfile;
    posToLight_1 = posToLight;
    lightDir_1 = lightDir;
    u_8 = u_7;
    v_27 = v_26;
    let _e90 = posToLight_1;
    let _e91 = lightDir_1;
    cosTheta_4 = dot(_e90, _e91);
    let _e94 = cosTheta_4;
    angle_1 = (acos(_e94) / 3.1415927f);
    let _e99 = angle_1;
    let _e101 = iesProfile_1;
    let _e103 = vec3<f32>(_e99, 0f, f32(_e101));
    let _e107 = textureSample(iesProfiles_t, iesProfiles_s, _e103.xy, i32(_e103.z));
    return _e107.x;
}

fn intersectLightAtIndex(lights_t: texture_2d<f32>, lights_s: sampler, rayOrigin_17: vec3<f32>, rayDirection_17: vec3<f32>, l_2: u32, lightRec: ptr<function, LightRecord>) -> bool {
    var rayOrigin_18: vec3<f32>;
    var rayDirection_18: vec3<f32>;
    var l_3: u32;
    var didHit: bool = false;
    var light: Light;
    var u_9: vec3<f32>;
    var v_28: vec3<f32>;
    var normal_8: vec3<f32>;
    var dist_11: f32;
    var cosTheta_5: f32;

    rayOrigin_18 = rayOrigin_17;
    rayDirection_18 = rayDirection_17;
    l_3 = l_2;
    let _e89 = l_3;
    let _e90 = readLightInfo(lights_t, lights_s, _e89);
    light = _e90;
    let _e92 = light;
    u_9 = _e92.u;
    let _e95 = light;
    v_28 = _e95.v;
    let _e98 = u_9;
    let _e99 = v_28;
    normal_8 = normalize(cross(_e98, _e99));
    let _e103 = normal_8;
    let _e104 = rayDirection_18;
    if (dot(_e103, _e104) > 0f) {
        {
            let _e108 = u_9;
            let _e110 = u_9;
            let _e111 = u_9;
            u_9 = (_e108 * (1f / dot(_e110, _e111)));
            let _e115 = v_28;
            let _e117 = v_28;
            let _e118 = v_28;
            v_28 = (_e115 * (1f / dot(_e117, _e118)));
            let _e123 = light;
            let _e127 = light;
            let _e129 = normal_8;
            let _e130 = u_9;
            let _e131 = v_28;
            let _e132 = rayOrigin_18;
            let _e133 = rayDirection_18;
            let _e136 = intersectsRectangle(_e127.position, _e129, _e130, _e131, _e132, _e133, (&dist_11));
            let _e138 = light;
            let _e142 = light;
            let _e144 = normal_8;
            let _e145 = u_9;
            let _e146 = v_28;
            let _e147 = rayOrigin_18;
            let _e148 = rayDirection_18;
            let _e151 = intersectsCircle(_e142.position, _e144, _e145, _e146, _e147, _e148, (&dist_11));
            if (((_e123.type_ == 0i) && _e136) || ((_e138.type_ == 1i) && _e151)) {
                {
                    let _e154 = rayDirection_18;
                    let _e155 = normal_8;
                    cosTheta_5 = dot(_e154, _e155);
                    didHit = true;
                    let _e160 = dist_11;
                    (*lightRec).dist = _e160;
                    let _e162 = dist_11;
                    let _e163 = dist_11;
                    let _e165 = light;
                    let _e167 = cosTheta_5;
                    (*lightRec).pdf = ((_e162 * _e163) / (_e165.area * _e167));
                    let _e171 = light;
                    let _e173 = light;
                    (*lightRec).emission = (_e171.color * _e173.intensity);
                    let _e177 = rayDirection_18;
                    (*lightRec).direction = _e177;
                    let _e179 = light;
                    (*lightRec).type_ = _e179.type_;
                }
            }
        }
    }
    let _e181 = didHit;
    return _e181;
}

fn randomAreaLightSample(light_1: Light, rayOrigin_19: vec3<f32>, ruv: vec2<f32>) -> LightRecord {
    var light_2: Light;
    var rayOrigin_20: vec3<f32>;
    var ruv_1: vec2<f32>;
    var randomPos: vec3<f32>;
    var r_6: f32;
    var theta_3: f32;
    var x_23: f32;
    var y_9: f32;
    var toLight: vec3<f32>;
    var lightDistSq: f32;
    var dist_12: f32;
    var direction_12: vec3<f32>;
    var lightNormal: vec3<f32>;
    var lightRec_1: LightRecord;

    light_2 = light_1;
    rayOrigin_20 = rayOrigin_19;
    ruv_1 = ruv;
    let _e85 = light_2;
    if (_e85.type_ == 0i) {
        {
            let _e89 = light_2;
            let _e91 = light_2;
            let _e93 = ruv_1;
            let _e99 = light_2;
            let _e101 = ruv_1;
            randomPos = ((_e89.position + (_e91.u * (_e93.x - 0.5f))) + (_e99.v * (_e101.y - 0.5f)));
        }
    } else {
        let _e107 = light_2;
        if (_e107.type_ == 1i) {
            {
                let _e112 = ruv_1;
                r_6 = (0.5f * sqrt(_e112.x));
                let _e117 = ruv_1;
                theta_3 = ((_e117.y * 2f) * 3.1415927f);
                let _e124 = r_6;
                let _e125 = theta_3;
                x_23 = (_e124 * cos(_e125));
                let _e129 = r_6;
                let _e130 = theta_3;
                y_9 = (_e129 * sin(_e130));
                let _e134 = light_2;
                let _e136 = light_2;
                let _e138 = x_23;
                let _e141 = light_2;
                let _e143 = y_9;
                randomPos = ((_e134.position + (_e136.u * _e138)) + (_e141.v * _e143));
            }
        }
    }
    let _e146 = randomPos;
    let _e147 = rayOrigin_20;
    toLight = (_e146 - _e147);
    let _e150 = toLight;
    let _e151 = toLight;
    lightDistSq = dot(_e150, _e151);
    let _e154 = lightDistSq;
    dist_12 = sqrt(_e154);
    let _e157 = toLight;
    let _e158 = dist_12;
    direction_12 = (_e157 / vec3(_e158));
    let _e162 = light_2;
    let _e164 = light_2;
    lightNormal = normalize(cross(_e162.u, _e164.v));
    let _e171 = light_2;
    lightRec_1.type_ = _e171.type_;
    let _e174 = light_2;
    let _e176 = light_2;
    lightRec_1.emission = (_e174.color * _e176.intensity);
    let _e180 = dist_12;
    lightRec_1.dist = _e180;
    let _e182 = direction_12;
    lightRec_1.direction = _e182;
    let _e184 = lightDistSq;
    let _e185 = light_2;
    let _e187 = direction_12;
    let _e188 = lightNormal;
    lightRec_1.pdf = (_e184 / (_e185.area * dot(_e187, _e188)));
    let _e192 = lightRec_1;
    return _e192;
}

fn randomSpotLightSample(light_3: Light, iesProfiles_t_1: texture_2d_array<f32>, iesProfiles_s_1: sampler, rayOrigin_21: vec3<f32>, ruv_2: vec2<f32>) -> LightRecord {
    var light_4: Light;
    var rayOrigin_22: vec3<f32>;
    var ruv_3: vec2<f32>;
    var radius_1: f32;
    var theta_4: f32;
    var x_24: f32;
    var y_10: f32;
    var u_10: vec3<f32>;
    var v_29: vec3<f32>;
    var normal_9: vec3<f32>;
    var angle_2: f32;
    var angleTan: f32;
    var startDistance: f32;
    var randomPos_1: vec3<f32>;
    var toLight_1: vec3<f32>;
    var lightDistSq_1: f32;
    var dist_13: f32;
    var direction_13: vec3<f32>;
    var cosTheta_6: f32;
    var local_21: f32;
    var spotAttenuation: f32;
    var distanceAttenuation: f32;
    var lightRec_2: LightRecord;

    light_4 = light_3;
    rayOrigin_22 = rayOrigin_21;
    ruv_3 = ruv_2;
    let _e86 = light_4;
    let _e88 = ruv_3;
    radius_1 = (_e86.radius * sqrt(_e88.x));
    let _e93 = ruv_3;
    theta_4 = ((_e93.y * 2f) * 3.1415927f);
    let _e100 = radius_1;
    let _e101 = theta_4;
    x_24 = (_e100 * cos(_e101));
    let _e105 = radius_1;
    let _e106 = theta_4;
    y_10 = (_e105 * sin(_e106));
    let _e110 = light_4;
    u_10 = _e110.u;
    let _e113 = light_4;
    v_29 = _e113.v;
    let _e116 = u_10;
    let _e117 = v_29;
    normal_9 = normalize(cross(_e116, _e117));
    let _e121 = light_4;
    angle_2 = acos(_e121.coneCos);
    let _e125 = angle_2;
    angleTan = tan(_e125);
    let _e128 = light_4;
    let _e130 = angleTan;
    startDistance = (_e128.radius / max(_e130, 0.000001f));
    let _e135 = light_4;
    let _e137 = normal_9;
    let _e138 = startDistance;
    let _e141 = u_10;
    let _e142 = x_24;
    let _e145 = v_29;
    let _e146 = y_10;
    randomPos_1 = (((_e135.position - (_e137 * _e138)) + (_e141 * _e142)) + (_e145 * _e146));
    let _e150 = randomPos_1;
    let _e151 = rayOrigin_22;
    toLight_1 = (_e150 - _e151);
    let _e154 = toLight_1;
    let _e155 = toLight_1;
    lightDistSq_1 = dot(_e154, _e155);
    let _e158 = lightDistSq_1;
    dist_13 = sqrt(_e158);
    let _e161 = toLight_1;
    let _e162 = dist_13;
    direction_13 = (_e161 / vec3(max(_e162, 0.000001f)));
    let _e168 = direction_13;
    let _e169 = normal_9;
    cosTheta_6 = dot(_e168, _e169);
    let _e172 = light_4;
    if (_e172.iesProfile != -1i) {
        let _e177 = light_4;
        let _e179 = direction_13;
        let _e180 = normal_9;
        let _e181 = u_10;
        let _e182 = v_29;
        let _e183 = getPhotometricAttenuation(iesProfiles_t_1, iesProfiles_s_1, _e177.iesProfile, _e179, _e180, _e181, _e182);
        local_21 = _e183;
    } else {
        let _e184 = light_4;
        let _e186 = light_4;
        let _e188 = cosTheta_6;
        let _e189 = getSpotAttenuation(_e184.coneCos, _e186.penumbraCos, _e188);
        local_21 = _e189;
    }
    let _e191 = local_21;
    spotAttenuation = _e191;
    let _e193 = dist_13;
    let _e194 = light_4;
    let _e196 = light_4;
    let _e198 = getDistanceAttenuation(_e193, _e194.distance, _e196.decay);
    distanceAttenuation = _e198;
    let _e202 = light_4;
    lightRec_2.type_ = _e202.type_;
    let _e205 = dist_13;
    lightRec_2.dist = _e205;
    let _e207 = direction_13;
    lightRec_2.direction = _e207;
    let _e209 = light_4;
    let _e211 = light_4;
    let _e214 = distanceAttenuation;
    let _e216 = spotAttenuation;
    lightRec_2.emission = (((_e209.color * _e211.intensity) * _e214) * _e216);
    lightRec_2.pdf = 1f;
    let _e220 = lightRec_2;
    return _e220;
}

fn randomLightSample(lights_t_1: texture_2d<f32>, lights_s_1: sampler, iesProfiles_t_2: texture_2d_array<f32>, iesProfiles_s_2: sampler, lightCount: u32, rayOrigin_23: vec3<f32>, ruv_4: vec3<f32>) -> LightRecord {
    var lightCount_1: u32;
    var rayOrigin_24: vec3<f32>;
    var ruv_5: vec3<f32>;
    var result: LightRecord;
    var l_4: u32;
    var light_5: Light;
    var lightRay: vec3<f32>;
    var lightDist: f32;
    var cutoffDistance_2: f32;
    var distanceFalloff_1: f32;
    var rec: LightRecord;
    var rec_1: LightRecord;

    lightCount_1 = lightCount;
    rayOrigin_24 = rayOrigin_23;
    ruv_5 = ruv_4;
    let _e89 = ruv_5;
    let _e91 = lightCount_1;
    l_4 = u32((_e89.x * f32(_e91)));
    let _e96 = l_4;
    let _e97 = readLightInfo(lights_t_1, lights_s_1, _e96);
    light_5 = _e97;
    let _e99 = light_5;
    if (_e99.type_ == 2i) {
        {
            let _e103 = light_5;
            let _e104 = rayOrigin_24;
            let _e105 = ruv_5;
            let _e107 = randomSpotLightSample(_e103, iesProfiles_t_2, iesProfiles_s_2, _e104, _e105.yz);
            result = _e107;
        }
    } else {
        let _e108 = light_5;
        if (_e108.type_ == 4i) {
            {
                let _e112 = light_5;
                let _e114 = rayOrigin_24;
                lightRay = (_e112.u - _e114);
                let _e117 = lightRay;
                lightDist = length(_e117);
                let _e120 = light_5;
                cutoffDistance_2 = _e120.distance;
                let _e124 = lightDist;
                let _e125 = light_5;
                let _e127 = pt_pow(_e124, _e125.decay);
                distanceFalloff_1 = (1f / max(_e127, 0.01f));
                let _e132 = cutoffDistance_2;
                if (_e132 > 0f) {
                    {
                        let _e135 = distanceFalloff_1;
                        let _e137 = lightDist;
                        let _e138 = cutoffDistance_2;
                        let _e140 = pow4_((_e137 / _e138));
                        let _e145 = pow2_(clamp((1f - _e140), 0f, 1f));
                        distanceFalloff_1 = (_e135 * _e145);
                    }
                }
                let _e149 = lightRay;
                rec.direction = normalize(_e149);
                let _e152 = lightRay;
                rec.dist = length(_e152);
                rec.pdf = 1f;
                let _e157 = light_5;
                let _e159 = light_5;
                let _e162 = distanceFalloff_1;
                rec.emission = ((_e157.color * _e159.intensity) * _e162);
                let _e165 = light_5;
                rec.type_ = _e165.type_;
                let _e167 = rec;
                result = _e167;
            }
        } else {
            let _e168 = light_5;
            if (_e168.type_ == 3i) {
                {
                    rec_1.dist = 10000000000f;
                    let _e176 = light_5;
                    rec_1.direction = _e176.u;
                    rec_1.pdf = 1f;
                    let _e181 = light_5;
                    let _e183 = light_5;
                    rec_1.emission = (_e181.color * _e183.intensity);
                    let _e187 = light_5;
                    rec_1.type_ = _e187.type_;
                    let _e189 = rec_1;
                    result = _e189;
                }
            } else {
                {
                    let _e190 = light_5;
                    let _e191 = rayOrigin_24;
                    let _e192 = ruv_5;
                    let _e194 = randomAreaLightSample(_e190, _e191, _e192.yz);
                    result = _e194;
                }
            }
        }
    }
    let _e195 = result;
    return _e195;
}

fn isMaterialFogVolume(materials_t: texture_2d<f32>, materials_s: sampler, materialIndex: u32) -> bool {
    var materialIndex_1: u32;
    var i_3: u32;
    var s14_1: vec4<f32>;

    materialIndex_1 = materialIndex;
    let _e82 = materialIndex_1;
    i_3 = (_e82 * 47u);
    let _e87 = i_3;
    let _e90 = texelFetch1D(materials_t, materials_s, (_e87 + 14u));
    s14_1 = _e90;
    let _e92 = s14_1;
    return bool((i32(_e92.z) & 4i));
}

fn bvhIntersectFogVolumeHit(rayOrigin_25: vec3<f32>, rayDirection_19: vec3<f32>, materialIndexAttribute_t: texture_2d<u32>, materialIndexAttribute_s: sampler, materials_t_1: texture_2d<f32>, materials_s_1: sampler, material: ptr<function, Material>) -> bool {
    var rayOrigin_26: vec3<f32>;
    var rayDirection_20: vec3<f32>;
    var i_4: i32 = 0i;
    var faceIndices_6: vec4<u32>;
    var faceNormal_2: vec3<f32>;
    var barycoord_7: vec3<f32>;
    var side_3: f32;
    var dist_14: f32;
    var hit_1: bool;
    var materialIndex_2: u32;

    rayOrigin_26 = rayOrigin_25;
    rayDirection_20 = rayDirection_19;
    (*material).fogVolume = false;
    loop {
        let _e91 = i_4;
        if !((_e91 < 30i)) {
            break;
        }
        {
            faceIndices_6 = vec4(0u);
            faceNormal_2 = vec3<f32>(0f, 0f, 1f);
            barycoord_7 = vec3(0f);
            side_3 = 1f;
            dist_14 = 0f;
            let _e113 = rayOrigin_26;
            let _e114 = rayDirection_20;
            let _e125 = _bvhIntersectFirstHit(bvh_position_t_1, pt_nearest, bvh_index_t_1, pt_nearest, bvh_bvhBounds_t_1, pt_nearest, bvh_bvhContents_t_1, pt_nearest, _e113, _e114, (&faceIndices_6), (&faceNormal_2), (&barycoord_7), (&side_3), (&dist_14));
            hit_1 = _e125;
            let _e127 = hit_1;
            if _e127 {
                {
                    let _e128 = faceIndices_6;
                    let _e130 = uTexelFetch1D(materialIndexAttribute_t, materialIndexAttribute_s, _e128.x);
                    materialIndex_2 = _e130.x;
                    let _e133 = materialIndex_2;
                    let _e134 = isMaterialFogVolume(materials_t_1, materials_s_1, _e133);
                    if _e134 {
                        {
                            let _e135 = materialIndex_2;
                            let _e136 = readMaterialInfo(materials_t_1, materials_s_1, _e135);
                            (*material) = _e136;
                            let _e137 = side_3;
                            return (_e137 == -1f);
                        }
                    } else {
                        {
                            let _e141 = rayOrigin_26;
                            let _e142 = rayDirection_20;
                            let _e143 = faceNormal_2;
                            let _e145 = dist_14;
                            let _e146 = stepRayOrigin(_e141, _e142, -(_e143), _e145);
                            rayOrigin_26 = _e146;
                        }
                    }
                }
            } else {
                {
                    return false;
                }
            }
        }
        continuing {
            let _e95 = i_4;
            i_4 = (_e95 + 1i);
        }
    }
    return false;
}

fn ggxDirection(incidentDir: vec3<f32>, roughness: vec2<f32>, uv_20: vec2<f32>) -> vec3<f32> {
    var incidentDir_1: vec3<f32>;
    var roughness_1: vec2<f32>;
    var uv_21: vec2<f32>;
    var V: vec3<f32>;
    var local_22: vec3<f32>;
    var T1_: vec3<f32>;
    var T2_: vec3<f32>;
    var a_14: f32;
    var r_7: f32;
    var local_23: f32;
    var phi_1: f32;
    var P1_: f32;
    var local_24: f32;
    var P2_: f32;
    var N: vec3<f32>;

    incidentDir_1 = incidentDir;
    roughness_1 = roughness;
    uv_21 = uv_20;
    let _e84 = roughness_1;
    let _e85 = incidentDir_1;
    let _e87 = (_e84 * _e85.xy);
    let _e88 = incidentDir_1;
    V = normalize(vec3<f32>(_e87.x, _e87.y, _e88.z));
    let _e95 = V;
    if (_e95.z < 0.9999f) {
        let _e99 = V;
        local_22 = normalize(cross(_e99, vec3<f32>(0f, 0f, 1f)));
    } else {
        local_22 = vec3<f32>(1f, 0f, 0f);
    }
    let _e111 = local_22;
    T1_ = _e111;
    let _e113 = T1_;
    let _e114 = V;
    T2_ = cross(_e113, _e114);
    let _e119 = V;
    a_14 = (1f / (1f + _e119.z));
    let _e124 = uv_21;
    r_7 = sqrt(_e124.x);
    let _e128 = uv_21;
    let _e130 = a_14;
    if (_e128.y < _e130) {
        let _e132 = uv_21;
        let _e134 = a_14;
        local_23 = ((_e132.y / _e134) * 3.1415927f);
    } else {
        let _e139 = uv_21;
        let _e141 = a_14;
        let _e144 = a_14;
        local_23 = (3.1415927f + (((_e139.y - _e141) / (1f - _e144)) * 3.1415927f));
    }
    let _e151 = local_23;
    phi_1 = _e151;
    let _e153 = r_7;
    let _e154 = phi_1;
    P1_ = (_e153 * cos(_e154));
    let _e158 = r_7;
    let _e159 = phi_1;
    let _e162 = uv_21;
    let _e164 = a_14;
    if (_e162.y < _e164) {
        local_24 = 1f;
    } else {
        let _e167 = V;
        local_24 = _e167.z;
    }
    let _e170 = local_24;
    P2_ = ((_e158 * sin(_e159)) * _e170);
    let _e173 = P1_;
    let _e174 = T1_;
    let _e176 = P2_;
    let _e177 = T2_;
    let _e180 = V;
    let _e183 = P1_;
    let _e184 = P1_;
    let _e187 = P2_;
    let _e188 = P2_;
    N = (((_e173 * _e174) + (_e176 * _e177)) + (_e180 * sqrt(max(0f, ((1f - (_e183 * _e184)) - (_e187 * _e188))))));
    let _e196 = roughness_1;
    let _e197 = N;
    let _e199 = (_e196 * _e197.xy);
    let _e201 = N;
    N = normalize(vec3<f32>(_e199.x, _e199.y, max(0f, _e201.z)));
    let _e208 = N;
    return _e208;
}

fn ggxLamda(theta_5: f32, roughness_2: f32) -> f32 {
    var theta_6: f32;
    var roughness_3: f32;
    var tanTheta: f32;
    var tanTheta2_: f32;
    var alpha2_: f32;
    var numerator: f32;

    theta_6 = theta_5;
    roughness_3 = roughness_2;
    let _e82 = theta_6;
    tanTheta = tan(_e82);
    let _e85 = tanTheta;
    let _e86 = tanTheta;
    tanTheta2_ = (_e85 * _e86);
    let _e89 = roughness_3;
    let _e90 = roughness_3;
    alpha2_ = (_e89 * _e90);
    let _e96 = alpha2_;
    let _e97 = tanTheta2_;
    numerator = (-1f + sqrt((1f + (_e96 * _e97))));
    let _e103 = numerator;
    return (_e103 / 2f);
}

fn ggxShadowMaskG1_(theta_7: f32, roughness_4: f32) -> f32 {
    var theta_8: f32;
    var roughness_5: f32;

    theta_8 = theta_7;
    roughness_5 = roughness_4;
    let _e84 = theta_8;
    let _e85 = roughness_5;
    let _e86 = ggxLamda(_e84, _e85);
    return (1f / (1f + _e86));
}

fn ggxShadowMaskG2_(wi_4: vec3<f32>, wo_4: vec3<f32>, roughness_6: f32) -> f32 {
    var wi_5: vec3<f32>;
    var wo_5: vec3<f32>;
    var roughness_7: f32;
    var incidentTheta: f32;
    var scatterTheta: f32;

    wi_5 = wi_4;
    wo_5 = wo_4;
    roughness_7 = roughness_6;
    let _e84 = wi_5;
    incidentTheta = acos(_e84.z);
    let _e88 = wo_5;
    scatterTheta = acos(_e88.z);
    let _e94 = incidentTheta;
    let _e95 = roughness_7;
    let _e96 = ggxLamda(_e94, _e95);
    let _e98 = scatterTheta;
    let _e99 = roughness_7;
    let _e100 = ggxLamda(_e98, _e99);
    return (1f / ((1f + _e96) + _e100));
}

fn ggxDistribution(halfVector: vec3<f32>, roughness_8: f32) -> f32 {
    var halfVector_1: vec3<f32>;
    var roughness_9: f32;
    var a2_2: f32;
    var cosTheta_7: f32;
    var cosTheta4_: f32;
    var theta_9: f32;
    var tanTheta_1: f32;
    var tanTheta2_1: f32;
    var denom: f32;

    halfVector_1 = halfVector;
    roughness_9 = roughness_8;
    let _e82 = roughness_9;
    let _e83 = roughness_9;
    a2_2 = (_e82 * _e83);
    let _e87 = a2_2;
    a2_2 = max(0.000001f, _e87);
    let _e89 = halfVector_1;
    cosTheta_7 = _e89.z;
    let _e92 = cosTheta_7;
    let _e94 = pt_pow(_e92, 4f);
    cosTheta4_ = _e94;
    let _e96 = cosTheta_7;
    if (_e96 == 0f) {
        return 0f;
    }
    let _e100 = halfVector_1;
    let _e102 = acosSafe(_e100.z);
    theta_9 = _e102;
    let _e104 = theta_9;
    tanTheta_1 = tan(_e104);
    let _e107 = tanTheta_1;
    let _e109 = pt_pow(_e107, 2f);
    tanTheta2_1 = _e109;
    let _e112 = cosTheta4_;
    let _e114 = a2_2;
    let _e115 = tanTheta2_1;
    let _e118 = pt_pow((_e114 + _e115), 2f);
    denom = ((3.1415927f * _e112) * _e118);
    let _e121 = a2_2;
    let _e122 = denom;
    return (_e121 / _e122);
}

fn ggxPDF(wi_6: vec3<f32>, halfVector_2: vec3<f32>, roughness_10: f32) -> f32 {
    var wi_7: vec3<f32>;
    var halfVector_3: vec3<f32>;
    var roughness_11: f32;
    var incidentTheta_1: f32;
    var D: f32;
    var G1_: f32;

    wi_7 = wi_6;
    halfVector_3 = halfVector_2;
    roughness_11 = roughness_10;
    let _e84 = wi_7;
    incidentTheta_1 = acos(_e84.z);
    let _e88 = halfVector_3;
    let _e89 = roughness_11;
    let _e90 = ggxDistribution(_e88, _e89);
    D = _e90;
    let _e92 = incidentTheta_1;
    let _e93 = roughness_11;
    let _e94 = ggxShadowMaskG1_(_e92, _e93);
    G1_ = _e94;
    let _e96 = D;
    let _e97 = G1_;
    let _e100 = wi_7;
    let _e101 = halfVector_3;
    let _e105 = wi_7;
    return (((_e96 * _e97) * max(0f, dot(_e100, _e101))) / _e105.z);
}

fn velvetD(cosThetaH: f32, roughness_12: f32) -> f32 {
    var cosThetaH_1: f32;
    var roughness_13: f32;
    var alpha: f32;
    var invAlpha: f32;
    var sqrCosThetaH: f32;
    var sinThetaH: f32;

    cosThetaH_1 = cosThetaH;
    roughness_13 = roughness_12;
    let _e82 = roughness_13;
    alpha = max(_e82, 0.07f);
    let _e86 = alpha;
    let _e87 = alpha;
    alpha = (_e86 * _e87);
    let _e90 = alpha;
    invAlpha = (1f / _e90);
    let _e93 = cosThetaH_1;
    let _e94 = cosThetaH_1;
    sqrCosThetaH = (_e93 * _e94);
    let _e98 = sqrCosThetaH;
    sinThetaH = max((1f - _e98), 0.001f);
    let _e104 = invAlpha;
    let _e106 = sinThetaH;
    let _e108 = invAlpha;
    let _e110 = pt_pow(_e106, (0.5f * _e108));
    return (((2f + _e104) * _e110) / 6.2831855f);
}

fn velvetParamsInterpolate(i_5: i32, oneMinusAlphaSquared: f32) -> f32 {
    var i_6: i32;
    var oneMinusAlphaSquared_1: f32;
    var p0_: array<f32, 5> = array<f32, 5>(25.3245f, 3.32435f, 0.16801f, -1.27393f, -4.85967f);
    var p1_: array<f32, 5> = array<f32, 5>(21.5473f, 3.82987f, 0.19823f, -1.9776f, -4.32054f);

    i_6 = i_5;
    oneMinusAlphaSquared_1 = oneMinusAlphaSquared;
    let _e100 = i_6;
    let _e102 = p1_[_e100];
    let _e103 = i_6;
    let _e105 = p0_[_e103];
    let _e106 = oneMinusAlphaSquared_1;
    return mix(_e102, _e105, _e106);
}

fn velvetL(x_25: f32, alpha_1: f32) -> f32 {
    var x_26: f32;
    var alpha_2: f32;
    var oneMinusAlpha: f32;
    var oneMinusAlphaSquared_2: f32;
    var a_15: f32;
    var b_14: f32;
    var c_7: f32;
    var d: f32;
    var e: f32;

    x_26 = x_25;
    alpha_2 = alpha_1;
    let _e83 = alpha_2;
    oneMinusAlpha = (1f - _e83);
    let _e86 = oneMinusAlpha;
    let _e87 = oneMinusAlpha;
    oneMinusAlphaSquared_2 = (_e86 * _e87);
    let _e91 = oneMinusAlphaSquared_2;
    let _e92 = velvetParamsInterpolate(0i, _e91);
    a_15 = _e92;
    let _e95 = oneMinusAlphaSquared_2;
    let _e96 = velvetParamsInterpolate(1i, _e95);
    b_14 = _e96;
    let _e99 = oneMinusAlphaSquared_2;
    let _e100 = velvetParamsInterpolate(2i, _e99);
    c_7 = _e100;
    let _e103 = oneMinusAlphaSquared_2;
    let _e104 = velvetParamsInterpolate(3i, _e103);
    d = _e104;
    let _e107 = oneMinusAlphaSquared_2;
    let _e108 = velvetParamsInterpolate(4i, _e107);
    e = _e108;
    let _e110 = a_15;
    let _e112 = b_14;
    let _e113 = x_26;
    let _e115 = c_7;
    let _e116 = pt_pow(abs(_e113), _e115);
    let _e120 = d;
    let _e121 = x_26;
    let _e124 = e;
    return (((_e110 / (1f + (_e112 * _e116))) + (_e120 * _e121)) + _e124);
}

fn velvetLambda(cosTheta_8: f32, alpha_3: f32) -> f32 {
    var cosTheta_9: f32;
    var alpha_4: f32;
    var local_25: f32;

    cosTheta_9 = cosTheta_8;
    alpha_4 = alpha_3;
    let _e82 = cosTheta_9;
    if (abs(_e82) < 0.5f) {
        let _e86 = cosTheta_9;
        let _e87 = alpha_4;
        let _e88 = velvetL(_e86, _e87);
        local_25 = exp(_e88);
    } else {
        let _e92 = alpha_4;
        let _e93 = velvetL(0.5f, _e92);
        let _e96 = cosTheta_9;
        let _e98 = alpha_4;
        let _e99 = velvetL((1f - _e96), _e98);
        local_25 = exp(((2f * _e93) - _e99));
    }
    let _e103 = local_25;
    return _e103;
}

fn velvetG(cosThetaO: f32, cosThetaI_2: f32, roughness_14: f32) -> f32 {
    var cosThetaO_1: f32;
    var cosThetaI_3: f32;
    var roughness_15: f32;
    var alpha_5: f32;

    cosThetaO_1 = cosThetaO;
    cosThetaI_3 = cosThetaI_2;
    roughness_15 = roughness_14;
    let _e84 = roughness_15;
    alpha_5 = max(_e84, 0.07f);
    let _e88 = alpha_5;
    let _e89 = alpha_5;
    alpha_5 = (_e88 * _e89);
    let _e93 = cosThetaO_1;
    let _e94 = alpha_5;
    let _e95 = velvetLambda(_e93, _e94);
    let _e97 = cosThetaI_3;
    let _e98 = alpha_5;
    let _e99 = velvetLambda(_e97, _e98);
    return (1f / ((1f + _e95) + _e99));
}

fn directionalAlbedoSheen(cosTheta_10: f32, alpha_6: f32) -> f32 {
    var cosTheta_11: f32;
    var alpha_7: f32;
    var c_8: f32;
    var c3_: f32;

    cosTheta_11 = cosTheta_10;
    alpha_7 = alpha_6;
    let _e82 = cosTheta_11;
    cosTheta_11 = clamp(_e82, 0f, 1f);
    let _e87 = cosTheta_11;
    c_8 = (1f - _e87);
    let _e90 = c_8;
    let _e91 = c_8;
    let _e93 = c_8;
    c3_ = ((_e90 * _e91) * _e93);
    let _e97 = c3_;
    let _e103 = alpha_7;
    return ((0.6558446f * _e97) + (1f / (4.1652656f + exp(((-7.9729137f * sqrt(_e103)) + 6.335169f)))));
}

fn sheenAlbedoScaling(wo_6: vec3<f32>, wi_8: vec3<f32>, surf: SurfaceRecord) -> f32 {
    var wo_7: vec3<f32>;
    var wi_9: vec3<f32>;
    var surf_1: SurfaceRecord;
    var alpha_8: f32;
    var maxSheenColor: f32;
    var eWo: f32;
    var eWi: f32;

    wo_7 = wo_6;
    wi_9 = wi_8;
    surf_1 = surf;
    let _e84 = surf_1;
    alpha_8 = max(_e84.sheenRoughness, 0.07f);
    let _e89 = alpha_8;
    let _e90 = alpha_8;
    alpha_8 = (_e89 * _e90);
    let _e92 = surf_1;
    let _e95 = surf_1;
    let _e99 = surf_1;
    maxSheenColor = max(max(_e92.sheenColor.x, _e95.sheenColor.y), _e99.sheenColor.z);
    let _e104 = wo_7;
    let _e106 = saturateCos(_e104.z);
    let _e107 = alpha_8;
    let _e108 = directionalAlbedoSheen(_e106, _e107);
    eWo = _e108;
    let _e110 = wi_9;
    let _e112 = saturateCos(_e110.z);
    let _e113 = alpha_8;
    let _e114 = directionalAlbedoSheen(_e112, _e113);
    eWi = _e114;
    let _e117 = maxSheenColor;
    let _e118 = eWo;
    let _e122 = maxSheenColor;
    let _e123 = eWi;
    return min((1f - (_e117 * _e118)), (1f - (_e122 * _e123)));
}

fn sheenAlbedoScaling_1(wo_8: vec3<f32>, surf_2: SurfaceRecord) -> f32 {
    var wo_9: vec3<f32>;
    var surf_3: SurfaceRecord;
    var alpha_9: f32;
    var maxSheenColor_1: f32;
    var eWo_1: f32;

    wo_9 = wo_8;
    surf_3 = surf_2;
    let _e82 = surf_3;
    alpha_9 = max(_e82.sheenRoughness, 0.07f);
    let _e87 = alpha_9;
    let _e88 = alpha_9;
    alpha_9 = (_e87 * _e88);
    let _e90 = surf_3;
    let _e93 = surf_3;
    let _e97 = surf_3;
    maxSheenColor_1 = max(max(_e90.sheenColor.x, _e93.sheenColor.y), _e97.sheenColor.z);
    let _e102 = wo_9;
    let _e104 = saturateCos(_e102.z);
    let _e105 = alpha_9;
    let _e106 = directionalAlbedoSheen(_e104, _e105);
    eWo_1 = _e106;
    let _e109 = maxSheenColor_1;
    let _e110 = eWo_1;
    return (1f - (_e109 * _e110));
}

fn fresnel0ToIor(fresnel0_: vec3<f32>) -> vec3<f32> {
    var fresnel0_1: vec3<f32>;
    var sqrtF0_: vec3<f32>;

    fresnel0_1 = fresnel0_;
    let _e81 = fresnel0_1;
    sqrtF0_ = sqrt(_e81);
    let _e86 = sqrtF0_;
    let _e90 = sqrtF0_;
    return ((vec3(1f) + _e86) / (vec3(1f) - _e90));
}

fn iorToFresnel0_(transmittedIor: vec3<f32>, incidentIor: f32) -> vec3<f32> {
    var transmittedIor_1: vec3<f32>;
    var incidentIor_1: f32;

    transmittedIor_1 = transmittedIor;
    incidentIor_1 = incidentIor;
    let _e83 = transmittedIor_1;
    let _e84 = incidentIor_1;
    let _e87 = transmittedIor_1;
    let _e88 = incidentIor_1;
    let _e92 = square_2(((_e83 - vec3(_e84)) / (_e87 + vec3(_e88))));
    return _e92;
}

fn iorToFresnel0_1(transmittedIor_2: f32, incidentIor_2: f32) -> f32 {
    var transmittedIor_3: f32;
    var incidentIor_3: f32;

    transmittedIor_3 = transmittedIor_2;
    incidentIor_3 = incidentIor_2;
    let _e83 = transmittedIor_3;
    let _e84 = incidentIor_3;
    let _e86 = transmittedIor_3;
    let _e87 = incidentIor_3;
    let _e90 = square(((_e83 - _e84) / (_e86 + _e87)));
    return _e90;
}

fn evalSensitivity(OPD: f32, shift: vec3<f32>) -> vec3<f32> {
    var OPD_1: f32;
    var shift_1: vec3<f32>;
    var phase: f32;
    var val_2: vec3<f32> = vec3<f32>(0.00000000000054856f, 0.00000000000044201f, 0.00000000000052481f);
    var pos: vec3<f32> = vec3<f32>(1681000f, 1795300f, 2208400f);
    var var_: vec3<f32> = vec3<f32>(4327800000f, 9304600000f, 6612100000f);
    var xyz: vec3<f32>;
    var srgb: vec3<f32>;

    OPD_1 = OPD;
    shift_1 = shift;
    let _e86 = OPD_1;
    phase = ((6.2831855f * _e86) * 0.000000001f);
    let _e106 = val_2;
    let _e110 = var_;
    let _e114 = pos;
    let _e115 = phase;
    let _e117 = shift_1;
    let _e121 = phase;
    let _e122 = square(_e121);
    let _e124 = var_;
    xyz = (((_e106 * sqrt((6.2831855f * _e110))) * cos(((_e114 * _e115) + _e117))) * exp((-(_e122) * _e124)));
    let _e130 = xyz;
    let _e141 = phase;
    let _e145 = shift_1.x;
    let _e151 = phase;
    let _e152 = square(_e151);
    xyz.x = (_e130.x + ((0.00000001644083f * cos(((2239900f * _e141) + _e145))) * exp((-4528200000f * _e152))));
    let _e157 = xyz;
    xyz = (_e157 / vec3(0.00000010685f));
    let _e161 = xyz;
    srgb = (XYZ_TO_REC709_ * _e161);
    let _e164 = srgb;
    return _e164;
}

fn evalIridescence(outsideIOR: f32, eta2_: f32, cosTheta1_: f32, thinFilmThickness: f32, baseF0_: vec3<f32>) -> vec3<f32> {
    var outsideIOR_1: f32;
    var eta2_1: f32;
    var cosTheta1_1: f32;
    var thinFilmThickness_1: f32;
    var baseF0_1: vec3<f32>;
    var I: vec3<f32>;
    var iridescenceIor: f32;
    var sinTheta2Sq: f32;
    var cosTheta2Sq: f32;
    var cosTheta2_: f32;
    var R0_: f32;
    var R12_: f32;
    var R21_: f32;
    var T121_: f32;
    var phi12_: f32 = 0f;
    var phi21_: f32;
    var baseIOR: vec3<f32>;
    var R1_: vec3<f32>;
    var R23_: vec3<f32>;
    var phi23_: vec3<f32> = vec3(0f);
    var OPD_2: f32;
    var phi_2: vec3<f32>;
    var R123_: vec3<f32>;
    var r123_: vec3<f32>;
    var Rs: vec3<f32>;
    var C0_: vec3<f32>;
    var Cm: vec3<f32>;
    var m_3: i32 = 1i;
    var Sm: vec3<f32>;

    outsideIOR_1 = outsideIOR;
    eta2_1 = eta2_;
    cosTheta1_1 = cosTheta1_;
    thinFilmThickness_1 = thinFilmThickness;
    baseF0_1 = baseF0_;
    let _e90 = outsideIOR_1;
    let _e91 = eta2_1;
    let _e94 = thinFilmThickness_1;
    iridescenceIor = mix(_e90, _e91, smoothstep(0f, 0.03f, _e94));
    let _e98 = outsideIOR_1;
    let _e99 = iridescenceIor;
    let _e101 = square((_e98 / _e99));
    let _e103 = cosTheta1_1;
    let _e104 = square(_e103);
    sinTheta2Sq = (_e101 * (1f - _e104));
    let _e109 = sinTheta2Sq;
    cosTheta2Sq = (1f - _e109);
    let _e112 = cosTheta2Sq;
    if (_e112 < 0f) {
        {
            return vec3(1f);
        }
    }
    let _e117 = cosTheta2Sq;
    cosTheta2_ = sqrt(_e117);
    let _e120 = iridescenceIor;
    let _e121 = outsideIOR_1;
    let _e122 = iorToFresnel0_1(_e120, _e121);
    R0_ = _e122;
    let _e124 = cosTheta1_1;
    let _e125 = R0_;
    let _e126 = schlickFresnel(_e124, _e125);
    R12_ = _e126;
    let _e128 = R12_;
    R21_ = _e128;
    let _e131 = R12_;
    T121_ = (1f - _e131);
    let _e136 = iridescenceIor;
    let _e137 = outsideIOR_1;
    if (_e136 < _e137) {
        {
            phi12_ = 3.1415927f;
        }
    }
    let _e141 = phi12_;
    phi21_ = (3.1415927f - _e141);
    let _e144 = baseF0_1;
    let _e150 = fresnel0ToIor(clamp(_e144, vec3(0f), vec3(0.9999f)));
    baseIOR = _e150;
    let _e152 = baseIOR;
    let _e153 = iridescenceIor;
    let _e154 = iorToFresnel0_(_e152, _e153);
    R1_ = _e154;
    let _e156 = cosTheta2_;
    let _e157 = R1_;
    let _e158 = schlickFresnel_1(_e156, _e157);
    R23_ = _e158;
    let _e165 = baseIOR.x;
    let _e166 = iridescenceIor;
    if (_e165 < _e166) {
        {
            phi23_[0i] = 3.1415927f;
        }
    }
    let _e173 = baseIOR.y;
    let _e174 = iridescenceIor;
    if (_e173 < _e174) {
        {
            phi23_[1i] = 3.1415927f;
        }
    }
    let _e181 = baseIOR.z;
    let _e182 = iridescenceIor;
    if (_e181 < _e182) {
        {
            phi23_[2i] = 3.1415927f;
        }
    }
    let _e188 = iridescenceIor;
    let _e190 = thinFilmThickness_1;
    let _e192 = cosTheta2_;
    OPD_2 = (((2f * _e188) * _e190) * _e192);
    let _e195 = phi21_;
    let _e197 = phi23_;
    phi_2 = (vec3(_e195) + _e197);
    let _e200 = R12_;
    let _e201 = R23_;
    R123_ = clamp((_e200 * _e201), vec3(0.00001f), vec3(0.9999f));
    let _e209 = R123_;
    r123_ = sqrt(_e209);
    let _e212 = T121_;
    let _e213 = square(_e212);
    let _e214 = R23_;
    let _e218 = R123_;
    Rs = ((_e213 * _e214) / (vec3(1f) - _e218));
    let _e222 = R12_;
    let _e223 = Rs;
    C0_ = (vec3(_e222) + _e223);
    let _e227 = C0_;
    I = _e227;
    let _e228 = Rs;
    let _e229 = T121_;
    Cm = (_e228 - vec3(_e229));
    loop {
        let _e235 = m_3;
        if !((_e235 <= 2i)) {
            break;
        }
        {
            let _e242 = Cm;
            let _e243 = r123_;
            Cm = (_e242 * _e243);
            let _e246 = m_3;
            let _e248 = OPD_2;
            let _e250 = m_3;
            let _e252 = phi_2;
            let _e254 = evalSensitivity((f32(_e246) * _e248), (f32(_e250) * _e252));
            Sm = (2f * _e254);
            let _e257 = I;
            let _e258 = Cm;
            let _e259 = Sm;
            I = (_e257 + (_e258 * _e259));
        }
        continuing {
            let _e239 = m_3;
            m_3 = (_e239 + 1i);
        }
    }
    let _e262 = I;
    return max(_e262, vec3(0f));
}

fn intersectFogVolume(material_1: Material, u_11: f32) -> f32 {
    var material_2: Material;
    var u_12: f32;
    var local_26: f32;

    material_2 = material_1;
    u_12 = u_11;
    let _e83 = material_2;
    if (_e83.opacity == 0f) {
        local_26 = 100000000000000000000f;
    } else {
        let _e90 = material_2;
        let _e93 = u_12;
        local_26 = ((-1f / _e90.opacity) * log(_e93));
    }
    let _e97 = local_26;
    return _e97;
}

fn sampleFogVolume(surf_4: SurfaceRecord, uv_22: vec2<f32>) -> ScatterRecord {
    var surf_5: SurfaceRecord;
    var uv_23: vec2<f32>;
    var sampleRec: ScatterRecord;

    surf_5 = surf_4;
    uv_23 = uv_22;
    sampleRec.specularPdf = 0f;
    sampleRec.pdf = 0.15915494f;
    let _e93 = uv_23;
    let _e94 = sampleSphere(_e93);
    sampleRec.direction = _e94;
    let _e96 = surf_5;
    sampleRec.color = _e96.color;
    let _e98 = sampleRec;
    return _e98;
}

fn diffuseEval(wo_10: vec3<f32>, wi_10: vec3<f32>, wh_2: vec3<f32>, surf_6: SurfaceRecord, color_2: ptr<function, vec3<f32>>) -> f32 {
    var wo_11: vec3<f32>;
    var wi_11: vec3<f32>;
    var wh_3: vec3<f32>;
    var surf_7: SurfaceRecord;
    var fl: f32;
    var fv: f32;
    var metalFactor: f32;
    var transFactor: f32;
    var rr: f32;
    var retro: f32;
    var lambert: f32;
    var F: f32;

    wo_11 = wo_10;
    wi_11 = wi_10;
    wh_3 = wh_2;
    surf_7 = surf_6;
    let _e88 = wi_11;
    let _e91 = schlickFresnel(_e88.z, 0f);
    fl = _e91;
    let _e93 = wo_11;
    let _e96 = schlickFresnel(_e93.z, 0f);
    fv = _e96;
    let _e99 = surf_7;
    metalFactor = (1f - _e99.metalness);
    let _e104 = surf_7;
    transFactor = (1f - _e104.transmission);
    let _e110 = surf_7;
    let _e113 = fl;
    let _e115 = fl;
    rr = (0.5f + (((2f * _e110.roughness) * _e113) * _e115));
    let _e119 = rr;
    let _e120 = fl;
    let _e121 = fv;
    let _e123 = fl;
    let _e124 = fv;
    let _e126 = rr;
    retro = (_e119 * ((_e120 + _e121) + ((_e123 * _e124) * (_e126 - 1f))));
    let _e135 = fl;
    let _e140 = fv;
    lambert = ((1f - (0.5f * _e135)) * (1f - (0.5f * _e140)));
    let _e145 = wo_11;
    let _e146 = wi_11;
    let _e147 = wh_3;
    let _e148 = surf_7;
    let _e150 = surf_7;
    let _e152 = surf_7;
    let _e154 = disneyFresnel(_e145, _e146, _e147, _e148.f0_, _e150.eta, _e152.metalness);
    F = _e154;
    let _e157 = F;
    let _e159 = transFactor;
    let _e161 = metalFactor;
    let _e163 = wi_11;
    let _e166 = surf_7;
    let _e169 = retro;
    let _e170 = lambert;
    (*color_2) = (((((((1f - _e157) * _e159) * _e161) * _e163.z) * _e166.color) * (_e169 + _e170)) / vec3(3.1415927f));
    let _e176 = wi_11;
    return (_e176.z / 3.1415927f);
}

fn diffuseDirection(wo_12: vec3<f32>, surf_8: SurfaceRecord) -> vec3<f32> {
    var wo_13: vec3<f32>;
    var surf_9: SurfaceRecord;
    var lightDirection: vec3<f32>;

    wo_13 = wo_12;
    surf_9 = surf_8;
    let _e84 = rand2_(11i);
    let _e85 = sampleSphere(_e84);
    lightDirection = _e85;
    let _e88 = lightDirection;
    lightDirection.z = (_e88.z + 1f);
    let _e92 = lightDirection;
    lightDirection = normalize(_e92);
    let _e94 = lightDirection;
    return _e94;
}

fn specularEval(wo_14: vec3<f32>, wi_12: vec3<f32>, wh_4: vec3<f32>, surf_10: SurfaceRecord, color_3: ptr<function, vec3<f32>>) -> f32 {
    var wo_15: vec3<f32>;
    var wi_13: vec3<f32>;
    var wh_5: vec3<f32>;
    var surf_11: SurfaceRecord;
    var metalness_2: f32;
    var roughness_16: f32;
    var eta_12: f32;
    var f0_14: f32;
    var f0Color: vec3<f32>;
    var f90Color: vec3<f32>;
    var F_1: vec3<f32>;
    var iridescenceF: vec3<f32>;
    var incidentTheta_2: f32;
    var G: f32;
    var D_1: f32;
    var G1_1: f32;
    var ggxPdf: f32;

    wo_15 = wo_14;
    wi_13 = wi_12;
    wh_5 = wh_4;
    surf_11 = surf_10;
    let _e88 = surf_11;
    metalness_2 = _e88.metalness;
    let _e91 = surf_11;
    roughness_16 = _e91.filteredRoughness;
    let _e94 = surf_11;
    eta_12 = _e94.eta;
    let _e97 = surf_11;
    f0_14 = _e97.f0_;
    let _e100 = f0_14;
    let _e101 = surf_11;
    let _e104 = surf_11;
    let _e107 = surf_11;
    let _e109 = surf_11;
    f0Color = mix(((_e100 * _e101.specularColor) * _e104.specularIntensity), _e107.color, vec3(_e109.metalness));
    let _e114 = surf_11;
    let _e117 = surf_11;
    f90Color = vec3(mix(_e114.specularIntensity, 1f, _e117.metalness));
    let _e122 = wo_15;
    let _e123 = wh_5;
    let _e125 = eta_12;
    let _e126 = f0Color;
    let _e127 = f90Color;
    let _e128 = evaluateFresnel(dot(_e122, _e123), _e125, _e126, _e127);
    F_1 = _e128;
    let _e131 = surf_11;
    let _e133 = wi_13;
    let _e134 = wh_5;
    let _e136 = surf_11;
    let _e138 = f0Color;
    let _e139 = evalIridescence(1f, _e131.iridescenceIor, dot(_e133, _e134), _e136.iridescenceThickness, _e138);
    iridescenceF = _e139;
    let _e141 = F_1;
    let _e142 = iridescenceF;
    let _e143 = surf_11;
    F_1 = mix(_e141, _e142, vec3(_e143.iridescence));
    let _e147 = wo_15;
    incidentTheta_2 = acos(_e147.z);
    let _e151 = wi_13;
    let _e152 = wo_15;
    let _e153 = roughness_16;
    let _e154 = ggxShadowMaskG2_(_e151, _e152, _e153);
    G = _e154;
    let _e156 = wh_5;
    let _e157 = roughness_16;
    let _e158 = ggxDistribution(_e156, _e157);
    D_1 = _e158;
    let _e160 = incidentTheta_2;
    let _e161 = roughness_16;
    let _e162 = ggxShadowMaskG1_(_e160, _e161);
    G1_1 = _e162;
    let _e164 = D_1;
    let _e165 = G1_1;
    let _e168 = wo_15;
    let _e169 = wh_5;
    let _e174 = wo_15;
    ggxPdf = (((_e164 * _e165) * max(0f, abs(dot(_e168, _e169)))) / abs(_e174.z));
    let _e179 = wi_13;
    let _e181 = F_1;
    let _e183 = G;
    let _e185 = D_1;
    let _e188 = wi_13;
    let _e190 = wo_15;
    (*color_3) = ((((_e179.z * _e181) * _e183) * _e185) / vec3((4f * abs((_e188.z * _e190.z)))));
    let _e197 = ggxPdf;
    let _e199 = wo_15;
    let _e200 = wh_5;
    return (_e197 / (4f * dot(_e199, _e200)));
}

fn specularDirection(wo_16: vec3<f32>, surf_12: SurfaceRecord) -> vec3<f32> {
    var wo_17: vec3<f32>;
    var surf_13: SurfaceRecord;
    var roughness_17: f32;
    var halfVector_4: vec3<f32>;

    wo_17 = wo_16;
    surf_13 = surf_12;
    let _e83 = surf_13;
    roughness_17 = _e83.filteredRoughness;
    let _e86 = wo_17;
    let _e87 = roughness_17;
    let _e90 = rand2_(12i);
    let _e91 = ggxDirection(_e86, vec2(_e87), _e90);
    halfVector_4 = _e91;
    let _e93 = wo_17;
    let _e94 = halfVector_4;
    return -(reflect(_e93, _e94));
}

fn transmissionEval(wo_18: vec3<f32>, wi_14: vec3<f32>, wh_6: vec3<f32>, surf_14: SurfaceRecord, color_4: ptr<function, vec3<f32>>) -> f32 {
    var wo_19: vec3<f32>;
    var wi_15: vec3<f32>;
    var wh_7: vec3<f32>;
    var surf_15: SurfaceRecord;
    var eta_13: f32;
    var f0_15: f32;
    var cosTheta_12: f32;
    var sinTheta_2: f32;
    var reflectance: f32;
    var cannotRefract: bool;

    wo_19 = wo_18;
    wi_15 = wi_14;
    wh_7 = wh_6;
    surf_15 = surf_14;
    let _e88 = surf_15;
    let _e90 = surf_15;
    (*color_4) = (_e88.transmission * _e90.color);
    let _e93 = surf_15;
    eta_13 = _e93.eta;
    let _e96 = surf_15;
    f0_15 = _e96.f0_;
    let _e99 = wo_19;
    cosTheta_12 = min(_e99.z, 1f);
    let _e105 = cosTheta_12;
    let _e106 = cosTheta_12;
    sinTheta_2 = sqrt((1f - (_e105 * _e106)));
    let _e111 = cosTheta_12;
    let _e112 = f0_15;
    let _e113 = schlickFresnel(_e111, _e112);
    reflectance = _e113;
    let _e115 = eta_13;
    let _e116 = sinTheta_2;
    cannotRefract = ((_e115 * _e116) > 1f);
    let _e121 = cannotRefract;
    if _e121 {
        {
            return 0f;
        }
    }
    let _e125 = reflectance;
    return (1f / (1f - _e125));
}

fn transmissionDirection(wo_20: vec3<f32>, surf_16: SurfaceRecord) -> vec3<f32> {
    var wo_21: vec3<f32>;
    var surf_17: SurfaceRecord;
    var roughness_18: f32;
    var eta_14: f32;
    var halfVector_5: vec3<f32>;
    var lightDirection_1: vec3<f32>;

    wo_21 = wo_20;
    surf_17 = surf_16;
    let _e83 = surf_17;
    roughness_18 = _e83.filteredRoughness;
    let _e86 = surf_17;
    eta_14 = _e86.eta;
    let _e94 = rand2_(13i);
    let _e95 = sampleSphere(_e94);
    let _e96 = roughness_18;
    halfVector_5 = normalize((vec3<f32>(0f, 0f, 1f) + (_e95 * _e96)));
    let _e101 = wo_21;
    let _e104 = halfVector_5;
    let _e105 = eta_14;
    lightDirection_1 = refract(normalize(-(_e101)), _e104, _e105);
    let _e108 = surf_17;
    if _e108.thinFilm {
        {
            let _e110 = lightDirection_1;
            let _e123 = eta_14;
            lightDirection_1 = -(refract(normalize(-(_e110)), vec3<f32>(-0f, -0f, -1f), (1f / _e123)));
        }
    }
    let _e127 = lightDirection_1;
    return normalize(_e127);
}

fn clearcoatEval(wo_22: vec3<f32>, wi_16: vec3<f32>, wh_8: vec3<f32>, surf_18: SurfaceRecord, color_5: ptr<function, vec3<f32>>) -> f32 {
    var wo_23: vec3<f32>;
    var wi_17: vec3<f32>;
    var wh_9: vec3<f32>;
    var surf_19: SurfaceRecord;
    var ior: f32 = 1.5f;
    var f0_16: f32;
    var frontFace: bool;
    var roughness_19: f32;
    var local_27: f32;
    var eta_15: f32;
    var G_1: f32;
    var D_2: f32;
    var F_2: f32;
    var fClearcoat: f32;

    wo_23 = wo_22;
    wi_17 = wi_16;
    wh_9 = wh_8;
    surf_19 = surf_18;
    let _e90 = ior;
    let _e91 = iorRatioToF0_(_e90);
    f0_16 = _e91;
    let _e93 = surf_19;
    frontFace = _e93.frontFace;
    let _e96 = surf_19;
    roughness_19 = _e96.filteredClearcoatRoughness;
    let _e99 = frontFace;
    if _e99 {
        let _e101 = ior;
        local_27 = (1f / _e101);
    } else {
        let _e103 = ior;
        local_27 = _e103;
    }
    let _e105 = local_27;
    eta_15 = _e105;
    let _e107 = wi_17;
    let _e108 = wo_23;
    let _e109 = roughness_19;
    let _e110 = ggxShadowMaskG2_(_e107, _e108, _e109);
    G_1 = _e110;
    let _e112 = wh_9;
    let _e113 = roughness_19;
    let _e114 = ggxDistribution(_e112, _e113);
    D_2 = _e114;
    let _e116 = wi_17;
    let _e117 = wh_9;
    let _e119 = f0_16;
    let _e120 = schlickFresnel(dot(_e116, _e117), _e119);
    F_2 = _e120;
    let _e122 = F_2;
    let _e123 = D_2;
    let _e125 = G_1;
    let _e128 = wi_17;
    let _e130 = wo_23;
    fClearcoat = (((_e122 * _e123) * _e125) / (4f * abs((_e128.z * _e130.z))));
    let _e137 = (*color_5);
    let _e139 = surf_19;
    let _e141 = F_2;
    let _e145 = fClearcoat;
    let _e146 = surf_19;
    let _e149 = wi_17;
    (*color_5) = ((_e137 * (1f - (_e139.clearcoat * _e141))) + vec3(((_e145 * _e146.clearcoat) * _e149.z)));
    let _e154 = wo_23;
    let _e155 = wh_9;
    let _e156 = roughness_19;
    let _e157 = ggxPDF(_e154, _e155, _e156);
    let _e159 = wi_17;
    let _e160 = wh_9;
    return (_e157 / (4f * dot(_e159, _e160)));
}

fn clearcoatDirection(wo_24: vec3<f32>, surf_20: SurfaceRecord) -> vec3<f32> {
    var wo_25: vec3<f32>;
    var surf_21: SurfaceRecord;
    var roughness_20: f32;
    var halfVector_6: vec3<f32>;

    wo_25 = wo_24;
    surf_21 = surf_20;
    let _e83 = surf_21;
    roughness_20 = _e83.filteredClearcoatRoughness;
    let _e86 = wo_25;
    let _e87 = roughness_20;
    let _e90 = rand2_(14i);
    let _e91 = ggxDirection(_e86, vec2(_e87), _e90);
    halfVector_6 = _e91;
    let _e93 = wo_25;
    let _e94 = halfVector_6;
    return -(reflect(_e93, _e94));
}

fn sheenColor(wo_26: vec3<f32>, wi_18: vec3<f32>, wh_10: vec3<f32>, surf_22: SurfaceRecord) -> vec3<f32> {
    var wo_27: vec3<f32>;
    var wi_19: vec3<f32>;
    var wh_11: vec3<f32>;
    var surf_23: SurfaceRecord;
    var cosThetaO_2: f32;
    var cosThetaI_4: f32;
    var cosThetaH_2: f32;
    var D_3: f32;
    var G_2: f32;
    var color_6: vec3<f32>;

    wo_27 = wo_26;
    wi_19 = wi_18;
    wh_11 = wh_10;
    surf_23 = surf_22;
    let _e87 = wo_27;
    let _e89 = saturateCos(_e87.z);
    cosThetaO_2 = _e89;
    let _e91 = wi_19;
    let _e93 = saturateCos(_e91.z);
    cosThetaI_4 = _e93;
    let _e95 = wh_11;
    cosThetaH_2 = _e95.z;
    let _e98 = cosThetaH_2;
    let _e99 = surf_23;
    let _e101 = velvetD(_e98, _e99.sheenRoughness);
    D_3 = _e101;
    let _e103 = cosThetaO_2;
    let _e104 = cosThetaI_4;
    let _e105 = surf_23;
    let _e107 = velvetG(_e103, _e104, _e105.sheenRoughness);
    G_2 = _e107;
    let _e109 = surf_23;
    color_6 = _e109.sheenColor;
    let _e112 = color_6;
    let _e113 = D_3;
    let _e114 = G_2;
    let _e117 = cosThetaO_2;
    let _e118 = cosThetaI_4;
    color_6 = (_e112 * ((_e113 * _e114) / (4f * abs((_e117 * _e118)))));
    let _e124 = color_6;
    let _e125 = wi_19;
    color_6 = (_e124 * _e125.z);
    let _e128 = color_6;
    return _e128;
}

fn getLobeWeights(wo_28: vec3<f32>, wi_20: vec3<f32>, wh_12: vec3<f32>, clearcoatWo: vec3<f32>, surf_24: SurfaceRecord, diffuseWeight: ptr<function, f32>, specularWeight: ptr<function, f32>, transmissionWeight: ptr<function, f32>, clearcoatWeight: ptr<function, f32>) {
    var wo_29: vec3<f32>;
    var wi_21: vec3<f32>;
    var wh_13: vec3<f32>;
    var clearcoatWo_1: vec3<f32>;
    var surf_25: SurfaceRecord;
    var metalness_3: f32;
    var transmission: f32;
    var fEstimate: f32;
    var transSpecularProb: f32;
    var diffSpecularProb: f32;
    var totalWeight: f32;

    wo_29 = wo_28;
    wi_21 = wi_20;
    wh_13 = wh_12;
    clearcoatWo_1 = clearcoatWo;
    surf_25 = surf_24;
    let _e93 = surf_25;
    metalness_3 = _e93.metalness;
    let _e96 = surf_25;
    transmission = _e96.transmission;
    let _e99 = wo_29;
    let _e100 = wi_21;
    let _e101 = wh_13;
    let _e102 = surf_25;
    let _e104 = surf_25;
    let _e106 = surf_25;
    let _e108 = disneyFresnel(_e99, _e100, _e101, _e102.f0_, _e104.eta, _e106.metalness);
    fEstimate = _e108;
    let _e111 = fEstimate;
    let _e114 = metalness_3;
    transSpecularProb = mix(max(0.25f, _e111), 1f, _e114);
    let _e119 = metalness_3;
    diffSpecularProb = (0.5f + (0.5f * _e119));
    let _e124 = transmission;
    let _e127 = diffSpecularProb;
    (*diffuseWeight) = ((1f - _e124) * (1f - _e127));
    let _e130 = transmission;
    let _e131 = transSpecularProb;
    let _e134 = transmission;
    let _e136 = diffSpecularProb;
    (*specularWeight) = ((_e130 * _e131) + ((1f - _e134) * _e136));
    let _e139 = transmission;
    let _e141 = transSpecularProb;
    (*transmissionWeight) = (_e139 * (1f - _e141));
    let _e144 = surf_25;
    let _e146 = clearcoatWo_1;
    let _e149 = schlickFresnel(_e146.z, 0.04f);
    (*clearcoatWeight) = (_e144.clearcoat * _e149);
    let _e151 = (*diffuseWeight);
    let _e152 = (*specularWeight);
    let _e154 = (*transmissionWeight);
    let _e156 = (*clearcoatWeight);
    totalWeight = (((_e151 + _e152) + _e154) + _e156);
    let _e159 = (*diffuseWeight);
    let _e160 = totalWeight;
    (*diffuseWeight) = (_e159 / _e160);
    let _e162 = (*specularWeight);
    let _e163 = totalWeight;
    (*specularWeight) = (_e162 / _e163);
    let _e165 = (*transmissionWeight);
    let _e166 = totalWeight;
    (*transmissionWeight) = (_e165 / _e166);
    let _e168 = (*clearcoatWeight);
    let _e169 = totalWeight;
    (*clearcoatWeight) = (_e168 / _e169);
    return;
}

fn bsdfEval(wo_30: vec3<f32>, clearcoatWo_2: vec3<f32>, wi_22: vec3<f32>, clearcoatWi: vec3<f32>, surf_26: SurfaceRecord, diffuseWeight_1: f32, specularWeight_1: f32, transmissionWeight_1: f32, clearcoatWeight_1: f32, specularPdf: ptr<function, f32>, color_7: ptr<function, vec3<f32>>) -> f32 {
    var wo_31: vec3<f32>;
    var clearcoatWo_3: vec3<f32>;
    var wi_23: vec3<f32>;
    var clearcoatWi_1: vec3<f32>;
    var surf_27: SurfaceRecord;
    var diffuseWeight_2: f32;
    var specularWeight_2: f32;
    var transmissionWeight_2: f32;
    var clearcoatWeight_2: f32;
    var metalness_4: f32;
    var transmission_1: f32;
    var spdf: f32 = 0f;
    var dpdf: f32 = 0f;
    var tpdf: f32 = 0f;
    var cpdf: f32 = 0f;
    var halfVector_7: vec3<f32>;
    var outColor: vec3<f32>;
    var clearcoatHalfVector: vec3<f32>;
    var pdf_2: f32;

    wo_31 = wo_30;
    clearcoatWo_3 = clearcoatWo_2;
    wi_23 = wi_22;
    clearcoatWi_1 = clearcoatWi;
    surf_27 = surf_26;
    diffuseWeight_2 = diffuseWeight_1;
    specularWeight_2 = specularWeight_1;
    transmissionWeight_2 = transmissionWeight_1;
    clearcoatWeight_2 = clearcoatWeight_1;
    let _e99 = surf_27;
    metalness_4 = _e99.metalness;
    let _e102 = surf_27;
    transmission_1 = _e102.transmission;
    (*color_7) = vec3(0f);
    let _e115 = wi_23;
    let _e116 = wo_31;
    let _e117 = surf_27;
    let _e119 = getHalfVector(_e115, _e116, _e117.eta);
    halfVector_7 = _e119;
    let _e121 = diffuseWeight_2;
    let _e124 = wi_23;
    if ((_e121 > 0f) && (_e124.z > 0f)) {
        {
            let _e129 = wo_31;
            let _e130 = wi_23;
            let _e131 = halfVector_7;
            let _e132 = surf_27;
            let _e135 = diffuseEval(_e129, _e130, _e131, _e132, color_7);
            dpdf = _e135;
            let _e136 = (*color_7);
            let _e138 = surf_27;
            (*color_7) = (_e136 * (1f - _e138.transmission));
        }
    }
    let _e142 = specularWeight_2;
    let _e145 = wi_23;
    if ((_e142 > 0f) && (_e145.z > 0f)) {
        {
            let _e151 = wo_31;
            let _e152 = wi_23;
            let _e153 = wi_23;
            let _e154 = wo_31;
            let _e155 = getHalfVector_1(_e153, _e154);
            let _e156 = surf_27;
            let _e159 = specularEval(_e151, _e152, _e155, _e156, (&outColor));
            spdf = _e159;
            let _e160 = (*color_7);
            let _e161 = outColor;
            (*color_7) = (_e160 + _e161);
        }
    }
    let _e163 = transmissionWeight_2;
    let _e166 = wi_23;
    if ((_e163 > 0f) && (_e166.z < 0f)) {
        {
            let _e171 = wo_31;
            let _e172 = wi_23;
            let _e173 = halfVector_7;
            let _e174 = surf_27;
            let _e177 = transmissionEval(_e171, _e172, _e173, _e174, color_7);
            tpdf = _e177;
        }
    }
    let _e178 = (*color_7);
    let _e180 = wo_31;
    let _e181 = wi_23;
    let _e182 = surf_27;
    let _e183 = sheenAlbedoScaling(_e180, _e181, _e182);
    let _e184 = surf_27;
    (*color_7) = (_e178 * mix(1f, _e183, _e184.sheen));
    let _e188 = (*color_7);
    let _e189 = wo_31;
    let _e190 = wi_23;
    let _e191 = halfVector_7;
    let _e192 = surf_27;
    let _e193 = sheenColor(_e189, _e190, _e191, _e192);
    let _e194 = surf_27;
    (*color_7) = (_e188 + (_e193 * _e194.sheen));
    let _e198 = clearcoatWi_1;
    let _e202 = clearcoatWeight_2;
    if ((_e198.z >= 0f) && (_e202 > 0f)) {
        {
            let _e206 = clearcoatWo_3;
            let _e207 = clearcoatWi_1;
            let _e208 = getHalfVector_1(_e206, _e207);
            clearcoatHalfVector = _e208;
            let _e210 = clearcoatWo_3;
            let _e211 = clearcoatWi_1;
            let _e212 = clearcoatHalfVector;
            let _e213 = surf_27;
            let _e216 = clearcoatEval(_e210, _e211, _e212, _e213, color_7);
            cpdf = _e216;
        }
    }
    let _e217 = dpdf;
    let _e218 = diffuseWeight_2;
    let _e220 = spdf;
    let _e221 = specularWeight_2;
    let _e224 = tpdf;
    let _e225 = transmissionWeight_2;
    let _e228 = cpdf;
    let _e229 = clearcoatWeight_2;
    pdf_2 = ((((_e217 * _e218) + (_e220 * _e221)) + (_e224 * _e225)) + (_e228 * _e229));
    let _e233 = spdf;
    let _e234 = specularWeight_2;
    let _e236 = cpdf;
    let _e237 = clearcoatWeight_2;
    (*specularPdf) = ((_e233 * _e234) + (_e236 * _e237));
    let _e240 = pdf_2;
    return _e240;
}

fn bsdfResult(worldWo: vec3<f32>, worldWi: vec3<f32>, surf_28: SurfaceRecord, color_8: ptr<function, vec3<f32>>) -> f32 {
    var worldWo_1: vec3<f32>;
    var worldWi_1: vec3<f32>;
    var surf_29: SurfaceRecord;
    var wo_32: vec3<f32>;
    var wi_24: vec3<f32>;
    var clearcoatWo_4: vec3<f32>;
    var clearcoatWi_2: vec3<f32>;
    var wh_14: vec3<f32>;
    var diffuseWeight_3: f32;
    var specularWeight_3: f32;
    var transmissionWeight_3: f32;
    var clearcoatWeight_3: f32;
    var specularPdf_1: f32;

    worldWo_1 = worldWo;
    worldWi_1 = worldWi;
    surf_29 = surf_28;
    let _e86 = surf_29;
    if _e86.volumeParticle {
        {
            let _e88 = surf_29;
            (*color_8) = (_e88.color / vec3(12.566371f));
            return 0.07957747f;
        }
    }
    let _e100 = surf_29;
    let _e102 = worldWo_1;
    wo_32 = normalize((_e100.normalInvBasis * _e102));
    let _e106 = surf_29;
    let _e108 = worldWi_1;
    wi_24 = normalize((_e106.normalInvBasis * _e108));
    let _e112 = surf_29;
    let _e114 = worldWo_1;
    clearcoatWo_4 = normalize((_e112.clearcoatInvBasis * _e114));
    let _e118 = surf_29;
    let _e120 = worldWi_1;
    clearcoatWi_2 = normalize((_e118.clearcoatInvBasis * _e120));
    let _e124 = wo_32;
    let _e125 = wi_24;
    let _e126 = surf_29;
    let _e128 = getHalfVector(_e124, _e125, _e126.eta);
    wh_14 = _e128;
    let _e134 = wo_32;
    let _e135 = wi_24;
    let _e136 = wh_14;
    let _e137 = clearcoatWo_4;
    let _e138 = surf_29;
    getLobeWeights(_e134, _e135, _e136, _e137, _e138, (&diffuseWeight_3), (&specularWeight_3), (&transmissionWeight_3), (&clearcoatWeight_3));
    let _e148 = wo_32;
    let _e149 = clearcoatWo_4;
    let _e150 = wi_24;
    let _e151 = clearcoatWi_2;
    let _e152 = surf_29;
    let _e153 = diffuseWeight_3;
    let _e154 = specularWeight_3;
    let _e155 = transmissionWeight_3;
    let _e156 = clearcoatWeight_3;
    let _e161 = bsdfEval(_e148, _e149, _e150, _e151, _e152, _e153, _e154, _e155, _e156, (&specularPdf_1), color_8);
    return _e161;
}

fn bsdfSample(worldWo_2: vec3<f32>, surf_30: SurfaceRecord) -> ScatterRecord {
    var worldWo_3: vec3<f32>;
    var surf_31: SurfaceRecord;
    var sampleRec_1: ScatterRecord;
    var wo_33: vec3<f32>;
    var clearcoatWo_5: vec3<f32>;
    var normalBasis: mat3x3<f32>;
    var invBasis: mat3x3<f32>;
    var clearcoatNormalBasis: mat3x3<f32>;
    var clearcoatInvBasis: mat3x3<f32>;
    var diffuseWeight_4: f32;
    var specularWeight_4: f32;
    var transmissionWeight_4: f32;
    var clearcoatWeight_4: f32;
    var pdf_3: array<f32, 4>;
    var cdf: array<f32, 4>;
    var invMaxCdf: f32;
    var wi_25: vec3<f32>;
    var clearcoatWi_3: vec3<f32>;
    var r_8: f32;
    var result_1: ScatterRecord;

    worldWo_3 = worldWo_2;
    surf_31 = surf_30;
    let _e83 = surf_31;
    if _e83.volumeParticle {
        {
            sampleRec_1.specularPdf = 0f;
            sampleRec_1.pdf = 0.07957747f;
            let _e96 = rand2_(16i);
            let _e97 = sampleSphere(_e96);
            sampleRec_1.direction = _e97;
            let _e99 = surf_31;
            sampleRec_1.color = (_e99.color / vec3(12.566371f));
            let _e106 = sampleRec_1;
            return _e106;
        }
    }
    let _e107 = surf_31;
    let _e109 = worldWo_3;
    wo_33 = normalize((_e107.normalInvBasis * _e109));
    let _e113 = surf_31;
    let _e115 = worldWo_3;
    clearcoatWo_5 = normalize((_e113.clearcoatInvBasis * _e115));
    let _e119 = surf_31;
    normalBasis = _e119.normalBasis;
    let _e122 = surf_31;
    invBasis = _e122.normalInvBasis;
    let _e125 = surf_31;
    clearcoatNormalBasis = _e125.clearcoatBasis;
    let _e128 = surf_31;
    clearcoatInvBasis = _e128.clearcoatInvBasis;
    let _e135 = wo_33;
    let _e136 = wo_33;
    let _e144 = clearcoatWo_5;
    let _e145 = surf_31;
    getLobeWeights(_e135, _e136, vec3<f32>(0f, 0f, 1f), _e144, _e145, (&diffuseWeight_4), (&specularWeight_4), (&transmissionWeight_4), (&clearcoatWeight_4));
    let _e157 = diffuseWeight_4;
    pdf_3[0i] = _e157;
    let _e160 = specularWeight_4;
    pdf_3[1i] = _e160;
    let _e163 = transmissionWeight_4;
    pdf_3[2i] = _e163;
    let _e166 = clearcoatWeight_4;
    pdf_3[3i] = _e166;
    let _e172 = pdf_3[0];
    cdf[0i] = _e172;
    let _e177 = pdf_3[1];
    let _e180 = cdf[0];
    cdf[1i] = (_e177 + _e180);
    let _e186 = pdf_3[2];
    let _e189 = cdf[1];
    cdf[2i] = (_e186 + _e189);
    let _e195 = pdf_3[3];
    let _e198 = cdf[2];
    cdf[3i] = (_e195 + _e198);
    let _e202 = cdf[3];
    if (_e202 != 0f) {
        {
            let _e208 = cdf[3];
            invMaxCdf = (1f / _e208);
            let _e215 = cdf[0];
            let _e216 = invMaxCdf;
            cdf[0i] = (_e215 * _e216);
            let _e222 = cdf[1];
            let _e223 = invMaxCdf;
            cdf[1i] = (_e222 * _e223);
            let _e229 = cdf[2];
            let _e230 = invMaxCdf;
            cdf[2i] = (_e229 * _e230);
            let _e236 = cdf[3];
            let _e237 = invMaxCdf;
            cdf[3i] = (_e236 * _e237);
        }
    } else {
        {
            cdf[0i] = 1f;
            cdf[1i] = 0f;
            cdf[2i] = 0f;
            cdf[3i] = 0f;
        }
    }
    let _e254 = rand_1(15i);
    r_8 = _e254;
    let _e256 = r_8;
    let _e259 = cdf[0];
    if (_e256 <= _e259) {
        {
            let _e261 = wo_33;
            let _e262 = surf_31;
            let _e263 = diffuseDirection(_e261, _e262);
            wi_25 = _e263;
            let _e264 = clearcoatInvBasis;
            let _e265 = normalBasis;
            let _e266 = wi_25;
            clearcoatWi_3 = normalize((_e264 * normalize((_e265 * _e266))));
        }
    } else {
        let _e271 = r_8;
        let _e274 = cdf[1];
        if (_e271 <= _e274) {
            {
                let _e276 = wo_33;
                let _e277 = surf_31;
                let _e278 = specularDirection(_e276, _e277);
                wi_25 = _e278;
                let _e279 = clearcoatInvBasis;
                let _e280 = normalBasis;
                let _e281 = wi_25;
                clearcoatWi_3 = normalize((_e279 * normalize((_e280 * _e281))));
            }
        } else {
            let _e286 = r_8;
            let _e289 = cdf[2];
            if (_e286 <= _e289) {
                {
                    let _e291 = wo_33;
                    let _e292 = surf_31;
                    let _e293 = transmissionDirection(_e291, _e292);
                    wi_25 = _e293;
                    let _e294 = clearcoatInvBasis;
                    let _e295 = normalBasis;
                    let _e296 = wi_25;
                    clearcoatWi_3 = normalize((_e294 * normalize((_e295 * _e296))));
                }
            } else {
                let _e301 = r_8;
                let _e304 = cdf[3];
                if (_e301 <= _e304) {
                    {
                        let _e306 = clearcoatWo_5;
                        let _e307 = surf_31;
                        let _e308 = clearcoatDirection(_e306, _e307);
                        clearcoatWi_3 = _e308;
                        let _e309 = invBasis;
                        let _e310 = clearcoatNormalBasis;
                        let _e311 = clearcoatWi_3;
                        wi_25 = normalize((_e309 * normalize((_e310 * _e311))));
                    }
                }
            }
        }
    }
    let _e318 = wo_33;
    let _e319 = clearcoatWo_5;
    let _e320 = wi_25;
    let _e321 = clearcoatWi_3;
    let _e322 = surf_31;
    let _e323 = diffuseWeight_4;
    let _e324 = specularWeight_4;
    let _e325 = transmissionWeight_4;
    let _e326 = clearcoatWeight_4;
    let _e327 = result_1;
    let _e329 = result_1;
    var pt_inout_0 = result_1.specularPdf;
    var pt_inout_1 = result_1.color;
    let _e335 = bsdfEval(_e318, _e319, _e320, _e321, _e322, _e323, _e324, _e325, _e326, (&pt_inout_0), (&pt_inout_1));
    result_1.specularPdf = pt_inout_0;
    result_1.color = pt_inout_1;
    result_1.pdf = _e335;
    let _e337 = surf_31;
    let _e339 = wi_25;
    result_1.direction = normalize((_e337.normalBasis * _e339));
    let _e342 = result_1;
    return _e342;
}

fn applyFilteredGlossy(roughness_21: f32, accumulatedRoughness: f32) -> f32 {
    var roughness_22: f32;
    var accumulatedRoughness_1: f32;

    roughness_22 = roughness_21;
    accumulatedRoughness_1 = accumulatedRoughness;
    let _e83 = roughness_22;
    let _e84 = accumulatedRoughness_1;
    let _e85 = global.filterGlossyFactor;
    return clamp(max(_e83, ((_e84 * _e85) * 5f)), 0f, 1f);
}

fn sampleBackground(direction_14: vec3<f32>, uv_24: vec2<f32>) -> vec3<f32> {
    var direction_15: vec3<f32>;
    var uv_25: vec2<f32>;
    var sampleDir: vec3<f32>;

    direction_15 = direction_14;
    uv_25 = uv_24;
    let _e83 = direction_15;
    let _e84 = uv_25;
    let _e85 = sampleHemisphere(_e83, _e84);
    let _e88 = global.backgroundBlur;
    sampleDir = ((_e85 * 0.5f) * _e88);
    let _e91 = envRotation3x3_;
    let _e92 = direction_15;
    let _e94 = sampleDir;
    sampleDir = normalize(((_e91 * _e92) + _e94));
    let _e97 = global.environmentIntensity;
    let _e98 = sampleDir;
    let _e99 = sampleEquirectColor(envMapInfo_map_t, pt_linear_repeat_u, _e98);
    return (_e97 * _e99);
}

fn initRenderState() -> RenderState {
    var result_2: RenderState;

    result_2.firstRay = true;
    result_2.transmissiveRay = true;
    result_2.isShadowRay = false;
    result_2.accumulatedRoughness = 0f;
    result_2.transmissiveTraversals = 0i;
    result_2.traversals = 0i;
    result_2.throughputColor = vec3(1f);
    result_2.depth = 0u;
    result_2.fogMaterial.fogVolume = false;
    let _e100 = result_2;
    return _e100;
}

fn ndcToRayOrigin(coord_2: vec2<f32>) -> vec3<f32> {
    var coord_3: vec2<f32>;
    var rayOrigin4_: vec4<f32>;

    coord_3 = coord_2;
    let _e81 = global.cameraWorldMatrix;
    let _e82 = global.invProjectionMatrix;
    let _e84 = coord_3;
    rayOrigin4_ = ((_e81 * _e82) * vec4<f32>(_e84.x, _e84.y, -1f, 1f));
    let _e93 = rayOrigin4_;
    let _e95 = rayOrigin4_;
    return (_e93.xyz / vec3(_e95.w));
}

fn getCameraRay() -> Ray {
    var ssd: vec2<f32>;
    var ruv_6: vec2<f32>;
    var jitteredUv: vec2<f32>;
    var ray: Ray;
    var ndc: vec2<f32>;

    let _e81 = global.resolution;
    ssd = (vec2(1f) / _e81);
    let _e85 = rand2_(0i);
    ruv_6 = _e85;
    let _e87 = vUv_1;
    let _e88 = ruv_6;
    let _e90 = tentFilter(_e88.x);
    let _e91 = ssd;
    let _e94 = ruv_6;
    let _e96 = tentFilter(_e94.y);
    let _e97 = ssd;
    jitteredUv = (_e87 + vec2<f32>((_e90 * _e91.x), (_e96 * _e97.y)));
    let _e105 = jitteredUv;
    ndc = ((2f * _e105) - vec2(1f));
    let _e112 = ndc;
    let _e113 = ndcToRayOrigin(_e112);
    ray.origin = _e113;
    let _e115 = global.cameraWorldMatrix;
    let _e125 = global.invProjectionMatrix;
    let _e126 = ndc;
    ray.direction = normalize((mat3x3<f32>(_e115[0].xyz, _e115[1].xyz, _e115[2].xyz) * (_e125 * vec4<f32>(_e126.x, _e126.y, 0f, 1f)).xyz));
    let _e137 = ray;
    ray.direction = normalize(_e137.direction);
    let _e140 = ray;
    return _e140;
}

fn traceScene(ray_1: Ray, fogMaterial: Material, surfaceHit: ptr<function, SurfaceHit>) -> i32 {
    var ray_2: Ray;
    var fogMaterial_1: Material;
    var result_3: i32 = 0i;
    var hit_2: bool;

    ray_2 = ray_1;
    fogMaterial_1 = fogMaterial;
    let _e86 = ray_2;
    let _e88 = ray_2;
    let _e90 = (*surfaceHit);
    let _e92 = (*surfaceHit);
    let _e94 = (*surfaceHit);
    let _e96 = (*surfaceHit);
    let _e98 = (*surfaceHit);
    var pt_inout_2 = (*surfaceHit).faceIndices;
    var pt_inout_3 = (*surfaceHit).faceNormal;
    var pt_inout_4 = (*surfaceHit).barycoord;
    var pt_inout_5 = (*surfaceHit).side;
    var pt_inout_6 = (*surfaceHit).dist;
    let _e110 = _bvhIntersectFirstHit(bvh_position_t_1, pt_nearest, bvh_index_t_1, pt_nearest, bvh_bvhBounds_t_1, pt_nearest, bvh_bvhContents_t_1, pt_nearest, _e86.origin, _e88.direction, (&pt_inout_2), (&pt_inout_3), (&pt_inout_4), (&pt_inout_5), (&pt_inout_6));
    (*surfaceHit).faceIndices = pt_inout_2;
    (*surfaceHit).faceNormal = pt_inout_3;
    (*surfaceHit).barycoord = pt_inout_4;
    (*surfaceHit).side = pt_inout_5;
    (*surfaceHit).dist = pt_inout_6;
    hit_2 = _e110;
    let _e112 = hit_2;
    if _e112 {
        {
            result_3 = 1i;
        }
    }
    let _e114 = result_3;
    return _e114;
}

fn attenuateHit(state: RenderState, ray_3: Ray, rayDist: f32, color_9: ptr<function, vec3<f32>>) -> bool {
    var state_1: RenderState;
    var ray_4: Ray;
    var rayDist_1: f32;
    var originalBounceIndex: u32;
    var traversals: i32;
    var transmissiveTraversals: i32;
    var isShadowRay: bool;
    var fogMaterial_2: Material;
    var startPoint: vec3<f32>;
    var surfaceHit_1: SurfaceHit;
    var result_4: bool = true;
    var i_7: i32 = 0i;
    var hitType: i32;
    var totalDist: f32;
    var materialIndex_3: u32;
    var material_3: Material;
    var isEntering: bool;
    var uv_26: vec2<f32>;
    var vertexColor: vec4<f32>;
    var albedo: vec4<f32>;
    var uvPrime: vec3<f32>;
    var uvPrime_1: vec3<f32>;
    var transmission_2: f32;
    var uvPrime_2: vec3<f32>;
    var metalness_5: f32;
    var uvPrime_3: vec3<f32>;
    var alphaTest: f32;
    var useAlphaTest: bool;
    var transmissionFactor: f32;
    var isTransmissiveRay: bool;

    state_1 = state;
    ray_4 = ray_3;
    rayDist_1 = rayDist;
    let _e86 = sobolBounceIndex;
    originalBounceIndex = _e86;
    let _e88 = state_1;
    traversals = _e88.traversals;
    let _e91 = state_1;
    transmissiveTraversals = _e91.transmissiveTraversals;
    let _e94 = state_1;
    isShadowRay = _e94.isShadowRay;
    let _e97 = state_1;
    fogMaterial_2 = _e97.fogMaterial;
    let _e100 = ray_4;
    startPoint = _e100.origin;
    (*color_9) = vec3(1f);
    loop {
        let _e110 = i_7;
        let _e111 = traversals;
        if !((_e110 < _e111)) {
            break;
        }
        {
            let _e117 = sobolBounceIndex;
            sobolBounceIndex = (_e117 + 1u);
            let _e120 = ray_4;
            let _e121 = fogMaterial_2;
            let _e124 = traceScene(_e120, _e121, (&surfaceHit_1));
            hitType = _e124;
            let _e126 = hitType;
            if (_e126 == 3i) {
                {
                    result_4 = true;
                    break;
                }
            } else {
                let _e130 = hitType;
                if (_e130 == 1i) {
                    {
                        let _e133 = startPoint;
                        let _e134 = ray_4;
                        let _e136 = ray_4;
                        let _e138 = surfaceHit_1;
                        totalDist = distance(_e133, (_e134.origin + (_e136.direction * _e138.dist)));
                        let _e144 = totalDist;
                        let _e145 = rayDist_1;
                        if (_e144 > _e145) {
                            {
                                result_4 = false;
                                break;
                            }
                        }
                        let _e148 = surfaceHit_1;
                        let _e151 = uTexelFetch1D(materialIndexAttribute_t_1, pt_nearest, _e148.faceIndices.x);
                        materialIndex_3 = _e151.x;
                        let _e154 = materialIndex_3;
                        let _e155 = readMaterialInfo(materials_t_2, pt_nearest, _e154);
                        material_3 = _e155;
                        let _e157 = surfaceHit_1;
                        isEntering = (_e157.side == 1f);
                        let _e163 = ray_4;
                        let _e165 = ray_4;
                        let _e167 = surfaceHit_1;
                        let _e170 = surfaceHit_1;
                        let _e172 = stepRayOrigin(_e163.origin, _e165.direction, -(_e167.faceNormal), _e170.dist);
                        ray_4.origin = _e172;
                        let _e173 = material_3;
                        let _e176 = isShadowRay;
                        if (!(_e173.castShadow) && _e176) {
                            {
                                continue;
                            }
                        }
                        let _e179 = surfaceHit_1;
                        let _e181 = surfaceHit_1;
                        let _e184 = textureSampleBarycoord_1(attributesArray_t_1, pt_nearest, 2i, _e179.barycoord, _e181.faceIndices.xyz);
                        uv_26 = _e184.xy;
                        let _e188 = surfaceHit_1;
                        let _e190 = surfaceHit_1;
                        let _e193 = textureSampleBarycoord_1(attributesArray_t_1, pt_nearest, 3i, _e188.barycoord, _e190.faceIndices.xyz);
                        vertexColor = _e193;
                        let _e195 = material_3;
                        let _e197 = material_3;
                        albedo = vec4<f32>(_e195.color.x, _e195.color.y, _e195.color.z, _e197.opacity);
                        let _e204 = material_3;
                        if (_e204.map != -1i) {
                            {
                                let _e209 = material_3;
                                let _e211 = uv_26;
                                uvPrime = (_e209.mapTransform * vec3<f32>(_e211.x, _e211.y, 1f));
                                let _e219 = albedo;
                                let _e220 = uvPrime;
                                let _e221 = _e220.xy;
                                let _e222 = material_3;
                                let _e227 = vec3<f32>(_e221.x, _e221.y, f32(_e222.map));
                                let _e231 = textureSample(textures_t, pt_linear_repeat, _e227.xy, i32(_e227.z));
                                albedo = (_e219 * _e231);
                            }
                        }
                        let _e233 = material_3;
                        if _e233.vertexColors {
                            {
                                let _e235 = albedo;
                                let _e236 = vertexColor;
                                albedo = (_e235 * _e236);
                            }
                        }
                        let _e238 = material_3;
                        if (_e238.alphaMap != -1i) {
                            {
                                let _e243 = material_3;
                                let _e245 = uv_26;
                                uvPrime_1 = (_e243.alphaMapTransform * vec3<f32>(_e245.x, _e245.y, 1f));
                                let _e254 = albedo;
                                let _e256 = uvPrime_1;
                                let _e257 = _e256.xy;
                                let _e258 = material_3;
                                let _e263 = vec3<f32>(_e257.x, _e257.y, f32(_e258.alphaMap));
                                let _e267 = textureSample(textures_t, pt_linear_repeat, _e263.xy, i32(_e263.z));
                                albedo.w = (_e254.w * _e267.x);
                            }
                        }
                        let _e270 = material_3;
                        transmission_2 = _e270.transmission;
                        let _e273 = material_3;
                        if (_e273.transmissionMap != -1i) {
                            {
                                let _e278 = material_3;
                                let _e280 = uv_26;
                                uvPrime_2 = (_e278.transmissionMapTransform * vec3<f32>(_e280.x, _e280.y, 1f));
                                let _e288 = transmission_2;
                                let _e289 = uvPrime_2;
                                let _e290 = _e289.xy;
                                let _e291 = material_3;
                                let _e296 = vec3<f32>(_e290.x, _e290.y, f32(_e291.transmissionMap));
                                let _e300 = textureSample(textures_t, pt_linear_repeat, _e296.xy, i32(_e296.z));
                                transmission_2 = (_e288 * _e300.x);
                            }
                        }
                        let _e303 = material_3;
                        metalness_5 = _e303.metalness;
                        let _e306 = material_3;
                        if (_e306.metalnessMap != -1i) {
                            {
                                let _e311 = material_3;
                                let _e313 = uv_26;
                                uvPrime_3 = (_e311.metalnessMapTransform * vec3<f32>(_e313.x, _e313.y, 1f));
                                let _e321 = metalness_5;
                                let _e322 = uvPrime_3;
                                let _e323 = _e322.xy;
                                let _e324 = material_3;
                                let _e329 = vec3<f32>(_e323.x, _e323.y, f32(_e324.metalnessMap));
                                let _e333 = textureSample(textures_t, pt_linear_repeat, _e329.xy, i32(_e329.z));
                                metalness_5 = (_e321 * _e333.z);
                            }
                        }
                        let _e336 = material_3;
                        alphaTest = _e336.alphaTest;
                        let _e339 = alphaTest;
                        useAlphaTest = (_e339 != 0f);
                        let _e344 = metalness_5;
                        let _e346 = transmission_2;
                        transmissionFactor = ((1f - _e344) * _e346);
                        let _e349 = transmissionFactor;
                        let _e351 = rand_1(9i);
                        let _e353 = material_3;
                        let _e357 = surfaceHit_1;
                        let _e359 = material_3;
                        let _e363 = useAlphaTest;
                        let _e364 = albedo;
                        let _e366 = alphaTest;
                        let _e370 = material_3;
                        let _e372 = useAlphaTest;
                        let _e375 = albedo;
                        let _e378 = rand_1(10i);
                        if ((_e349 < _e351) && !(((((_e353.side != 0f) && (_e357.side == _e359.side)) || (_e363 && (_e364.w < _e366))) || ((_e370.transparent && !(_e372)) && (_e375.w < _e378))))) {
                            {
                                result_4 = true;
                                break;
                            }
                        }
                        let _e385 = surfaceHit_1;
                        let _e389 = isEntering;
                        if ((_e385.side == 1f) && _e389) {
                            {
                                let _e391 = (*color_9);
                                let _e394 = albedo;
                                let _e396 = transmissionFactor;
                                (*color_9) = (_e391 * mix(vec3(1f), _e394.xyz, vec3(_e396)));
                            }
                        } else {
                            let _e400 = surfaceHit_1;
                            if (_e400.side == -1f) {
                                {
                                    let _e405 = (*color_9);
                                    let _e406 = surfaceHit_1;
                                    let _e408 = material_3;
                                    let _e410 = material_3;
                                    let _e412 = transmissionAttenuation(_e406.dist, _e408.attenuationColor, _e410.attenuationDistance);
                                    (*color_9) = (_e405 * _e412);
                                }
                            }
                        }
                        let _e414 = ray_4;
                        let _e416 = surfaceHit_1;
                        let _e418 = surfaceHit_1;
                        isTransmissiveRay = (dot(_e414.direction, (_e416.faceNormal * _e418.side)) < 0f);
                        let _e425 = isTransmissiveRay;
                        let _e426 = isEntering;
                        let _e428 = transmissiveTraversals;
                        if ((_e425 || _e426) && (_e428 > 0i)) {
                            {
                                let _e432 = i_7;
                                let _e433 = transmissiveTraversals;
                                i_7 = (_e432 - sign(_e433));
                                let _e436 = transmissiveTraversals;
                                transmissiveTraversals = (_e436 - 1i);
                            }
                        }
                    }
                } else {
                    {
                        result_4 = false;
                        break;
                    }
                }
            }
        }
        continuing {
            let _e114 = i_7;
            i_7 = (_e114 + 1i);
        }
    }
    let _e440 = originalBounceIndex;
    sobolBounceIndex = _e440;
    let _e441 = result_4;
    return _e441;
}

fn directLightContribution(worldWo_4: vec3<f32>, surf_32: SurfaceRecord, state_2: RenderState, rayOrigin_27: vec3<f32>) -> vec3<f32> {
    var worldWo_5: vec3<f32>;
    var surf_33: SurfaceRecord;
    var state_3: RenderState;
    var rayOrigin_28: vec3<f32>;
    var result_5: vec3<f32> = vec3(0f);
    var lightRec_3: LightRecord;
    var isSampleBelowSurface: bool;
    var lightRay_1: Ray;
    var attenuatedColor: vec3<f32>;
    var sampleColor: vec3<f32>;
    var lightMaterialPdf: f32;
    var isValidSampleColor: bool;
    var lightPdf: f32;
    var local_28: f32;
    var misWeight: f32;
    var envColor: vec3<f32>;
    var envDirection: vec3<f32>;
    var envPdf: f32;
    var isSampleBelowSurface_1: bool;
    var envRay: Ray;
    var attenuatedColor_1: vec3<f32>;
    var sampleColor_1: vec3<f32>;
    var envMaterialPdf: f32;
    var isValidSampleColor_1: bool;
    var misWeight_1: f32;

    worldWo_5 = worldWo_4;
    surf_33 = surf_32;
    state_3 = state_2;
    rayOrigin_28 = rayOrigin_27;
    let _e90 = lightsDenom;
    let _e94 = rand_1(5i);
    let _e95 = global.lights_count;
    let _e97 = lightsDenom;
    if ((_e90 != 0f) && (_e94 < (f32(_e95) / _e97))) {
        {
            let _e101 = global.lights_count;
            let _e102 = rayOrigin_28;
            let _e104 = rand3_(6i);
            let _e105 = randomLightSample(lights_tex_t, pt_nearest, iesProfiles_t_3, pt_linear, _e101, _e102, _e104);
            lightRec_3 = _e105;
            let _e107 = surf_33;
            let _e110 = surf_33;
            let _e112 = lightRec_3;
            isSampleBelowSurface = (!(_e107.volumeParticle) && (dot(_e110.faceNormal, _e112.direction) < 0f));
            let _e119 = isSampleBelowSurface;
            if _e119 {
                {
                    lightRec_3.pdf = 0f;
                }
            }
            let _e124 = rayOrigin_28;
            lightRay_1.origin = _e124;
            let _e126 = lightRec_3;
            lightRay_1.direction = _e126.direction;
            let _e129 = lightRec_3;
            let _e133 = lightRec_3;
            let _e135 = surf_33;
            let _e137 = surf_33;
            let _e139 = isDirectionValid(_e133.direction, _e135.normal, _e137.faceNormal);
            let _e141 = state_3;
            let _e142 = lightRay_1;
            let _e143 = lightRec_3;
            let _e147 = attenuateHit(_e141, _e142, _e143.dist, (&attenuatedColor));
            if (((_e129.pdf > 0f) && _e139) && !(_e147)) {
                {
                    let _e151 = worldWo_5;
                    let _e152 = lightRec_3;
                    let _e154 = surf_33;
                    let _e157 = bsdfResult(_e151, _e152.direction, _e154, (&sampleColor));
                    lightMaterialPdf = _e157;
                    let _e159 = sampleColor;
                    isValidSampleColor = all((_e159 >= vec3(0f)));
                    let _e165 = lightMaterialPdf;
                    let _e168 = isValidSampleColor;
                    if ((_e165 > 0f) && _e168) {
                        {
                            let _e170 = lightRec_3;
                            let _e172 = lightsDenom;
                            lightPdf = (_e170.pdf / _e172);
                            let _e175 = lightRec_3;
                            let _e179 = lightRec_3;
                            let _e184 = lightRec_3;
                            if (((_e175.type_ == 2i) || (_e179.type_ == 3i)) || (_e184.type_ == 4i)) {
                                local_28 = 1f;
                            } else {
                                let _e190 = lightPdf;
                                let _e191 = lightMaterialPdf;
                                let _e192 = misHeuristic(_e190, _e191);
                                local_28 = _e192;
                            }
                            let _e194 = local_28;
                            misWeight = _e194;
                            let _e196 = attenuatedColor;
                            let _e197 = lightRec_3;
                            let _e200 = state_3;
                            let _e203 = sampleColor;
                            let _e205 = misWeight;
                            let _e207 = lightPdf;
                            result_5 = (((((_e196 * _e197.emission) * _e200.throughputColor) * _e203) * _e205) / vec3(_e207));
                        }
                    }
                }
            }
        }
    } else {
        let _e210 = global.envMapInfo_totalSum;
        let _e213 = global.environmentIntensity;
        if ((_e210 != 0f) && (_e213 != 0f)) {
            {
                let _e220 = rand2_(7i);
                let _e225 = sampleEquirectProbability(_e220, (&envColor), (&envDirection));
                envPdf = _e225;
                let _e227 = invEnvRotation3x3_;
                let _e228 = envDirection;
                envDirection = (_e227 * _e228);
                let _e230 = surf_33;
                let _e233 = surf_33;
                let _e235 = envDirection;
                isSampleBelowSurface_1 = (!(_e230.volumeParticle) && (dot(_e233.faceNormal, _e235) < 0f));
                let _e241 = isSampleBelowSurface_1;
                if _e241 {
                    {
                        envPdf = 0f;
                    }
                }
                let _e245 = rayOrigin_28;
                envRay.origin = _e245;
                let _e247 = envDirection;
                envRay.direction = _e247;
                let _e249 = envPdf;
                let _e252 = envDirection;
                let _e253 = surf_33;
                let _e255 = surf_33;
                let _e257 = isDirectionValid(_e252, _e253.normal, _e255.faceNormal);
                let _e259 = state_3;
                let _e260 = envRay;
                let _e264 = attenuateHit(_e259, _e260, 100000000000000000000f, (&attenuatedColor_1));
                if (((_e249 > 0f) && _e257) && !(_e264)) {
                    {
                        let _e268 = worldWo_5;
                        let _e269 = envDirection;
                        let _e270 = surf_33;
                        let _e273 = bsdfResult(_e268, _e269, _e270, (&sampleColor_1));
                        envMaterialPdf = _e273;
                        let _e275 = sampleColor_1;
                        isValidSampleColor_1 = all((_e275 >= vec3(0f)));
                        let _e281 = envMaterialPdf;
                        let _e284 = isValidSampleColor_1;
                        if ((_e281 > 0f) && _e284) {
                            {
                                let _e286 = envPdf;
                                let _e287 = lightsDenom;
                                envPdf = (_e286 / _e287);
                                let _e289 = envPdf;
                                let _e290 = envMaterialPdf;
                                let _e291 = misHeuristic(_e289, _e290);
                                misWeight_1 = _e291;
                                let _e293 = attenuatedColor_1;
                                let _e294 = global.environmentIntensity;
                                let _e296 = envColor;
                                let _e298 = state_3;
                                let _e301 = sampleColor_1;
                                let _e303 = misWeight_1;
                                let _e305 = envPdf;
                                result_5 = ((((((_e293 * _e294) * _e296) * _e298.throughputColor) * _e301) * _e303) / vec3(_e305));
                            }
                        }
                    }
                }
            }
        }
    }
    let _e308 = result_5;
    return _e308;
}

fn getSurfaceRecord(material_4: Material, surfaceHit_2: SurfaceHit, attributesArray_t: texture_2d_array<f32>, attributesArray_s: sampler, accumulatedRoughness_2: f32, surf_34: ptr<function, SurfaceRecord>) -> i32 {
    var material_5: Material;
    var surfaceHit_3: SurfaceHit;
    var accumulatedRoughness_3: f32;
    var normal_10: vec3<f32> = vec3<f32>(0f, 0f, 1f);
    var fogSurface: SurfaceRecord;
    var uv_27: vec2<f32>;
    var vertexColor_1: vec4<f32>;
    var albedo_1: vec4<f32>;
    var uvPrime_4: vec3<f32>;
    var uvPrime_5: vec3<f32>;
    var alphaTest_1: f32;
    var useAlphaTest_1: bool;
    var normal_11: vec3<f32>;
    var roughness_23: f32;
    var uvPrime_6: vec3<f32>;
    var metalness_6: f32;
    var uvPrime_7: vec3<f32>;
    var emission: vec3<f32>;
    var uvPrime_8: vec3<f32>;
    var transmission_3: f32;
    var uvPrime_9: vec3<f32>;
    var baseNormal: vec3<f32>;
    var tangentSample: vec4<f32>;
    var tangent: vec3<f32>;
    var bitangent: vec3<f32>;
    var vTBN: mat3x3<f32>;
    var uvPrime_10: vec3<f32>;
    var texNormal: vec3<f32>;
    var clearcoat: f32;
    var uvPrime_11: vec3<f32>;
    var clearcoatRoughness: f32;
    var uvPrime_12: vec3<f32>;
    var clearcoatNormal: vec3<f32>;
    var tangentSample_1: vec4<f32>;
    var tangent_1: vec3<f32>;
    var bitangent_1: vec3<f32>;
    var vTBN_1: mat3x3<f32>;
    var uvPrime_13: vec3<f32>;
    var texNormal_1: vec3<f32>;
    var sheenColor_1: vec3<f32>;
    var uvPrime_14: vec3<f32>;
    var sheenRoughness: f32;
    var uvPrime_15: vec3<f32>;
    var iridescence: f32;
    var uvPrime_16: vec3<f32>;
    var iridescenceThickness: f32;
    var uvPrime_17: vec3<f32>;
    var iridescenceThicknessSampled: f32;
    var local_29: f32;
    var specularColor: vec3<f32>;
    var uvPrime_18: vec3<f32>;
    var specularIntensity: f32;
    var uvPrime_19: vec3<f32>;
    var local_30: f32;

    material_5 = material_4;
    surfaceHit_3 = surfaceHit_2;
    accumulatedRoughness_3 = accumulatedRoughness_2;
    let _e88 = material_5;
    if _e88.fogVolume {
        {
            fogSurface.volumeParticle = true;
            let _e102 = material_5;
            fogSurface.color = _e102.color;
            let _e105 = material_5;
            let _e107 = material_5;
            fogSurface.emission = (_e105.emissiveIntensity * _e107.emissive);
            let _e111 = normal_10;
            fogSurface.normal = _e111;
            let _e113 = normal_10;
            fogSurface.faceNormal = _e113;
            let _e115 = normal_10;
            fogSurface.clearcoatNormal = _e115;
            let _e116 = fogSurface;
            (*surf_34) = _e116;
            return 1i;
        }
    }
    let _e119 = surfaceHit_3;
    let _e121 = surfaceHit_3;
    let _e124 = textureSampleBarycoord_1(attributesArray_t, attributesArray_s, 2i, _e119.barycoord, _e121.faceIndices.xyz);
    uv_27 = _e124.xy;
    let _e128 = surfaceHit_3;
    let _e130 = surfaceHit_3;
    let _e133 = textureSampleBarycoord_1(attributesArray_t, attributesArray_s, 3i, _e128.barycoord, _e130.faceIndices.xyz);
    vertexColor_1 = _e133;
    let _e135 = material_5;
    let _e137 = material_5;
    albedo_1 = vec4<f32>(_e135.color.x, _e135.color.y, _e135.color.z, _e137.opacity);
    let _e144 = material_5;
    if (_e144.map != -1i) {
        {
            let _e149 = material_5;
            let _e151 = uv_27;
            uvPrime_4 = (_e149.mapTransform * vec3<f32>(_e151.x, _e151.y, 1f));
            let _e159 = albedo_1;
            let _e160 = uvPrime_4;
            let _e161 = _e160.xy;
            let _e162 = material_5;
            let _e167 = vec3<f32>(_e161.x, _e161.y, f32(_e162.map));
            let _e171 = textureSample(textures_t, pt_linear_repeat, _e167.xy, i32(_e167.z));
            albedo_1 = (_e159 * _e171);
        }
    }
    let _e173 = material_5;
    if _e173.vertexColors {
        {
            let _e175 = albedo_1;
            let _e176 = vertexColor_1;
            albedo_1 = (_e175 * _e176);
        }
    }
    let _e178 = material_5;
    if (_e178.alphaMap != -1i) {
        {
            let _e183 = material_5;
            let _e185 = uv_27;
            uvPrime_5 = (_e183.alphaMapTransform * vec3<f32>(_e185.x, _e185.y, 1f));
            let _e194 = albedo_1;
            let _e196 = uvPrime_5;
            let _e197 = _e196.xy;
            let _e198 = material_5;
            let _e203 = vec3<f32>(_e197.x, _e197.y, f32(_e198.alphaMap));
            let _e207 = textureSample(textures_t, pt_linear_repeat, _e203.xy, i32(_e203.z));
            albedo_1.w = (_e194.w * _e207.x);
        }
    }
    let _e210 = material_5;
    alphaTest_1 = _e210.alphaTest;
    let _e213 = alphaTest_1;
    useAlphaTest_1 = (_e213 != 0f);
    let _e217 = material_5;
    let _e221 = surfaceHit_3;
    let _e223 = material_5;
    let _e227 = useAlphaTest_1;
    let _e228 = albedo_1;
    let _e230 = alphaTest_1;
    let _e234 = material_5;
    let _e236 = useAlphaTest_1;
    let _e239 = albedo_1;
    let _e242 = rand_1(3i);
    if ((((_e217.side != 0f) && (_e221.side != _e223.side)) || (_e227 && (_e228.w < _e230))) || ((_e234.transparent && !(_e236)) && (_e239.w < _e242))) {
        {
            return 0i;
        }
    }
    let _e248 = surfaceHit_3;
    let _e250 = surfaceHit_3;
    let _e253 = textureSampleBarycoord_1(attributesArray_t, attributesArray_s, 0i, _e248.barycoord, _e250.faceIndices.xyz);
    normal_11 = normalize(_e253.xyz);
    let _e257 = material_5;
    roughness_23 = _e257.roughness;
    let _e260 = material_5;
    if (_e260.roughnessMap != -1i) {
        {
            let _e265 = material_5;
            let _e267 = uv_27;
            uvPrime_6 = (_e265.roughnessMapTransform * vec3<f32>(_e267.x, _e267.y, 1f));
            let _e275 = roughness_23;
            let _e276 = uvPrime_6;
            let _e277 = _e276.xy;
            let _e278 = material_5;
            let _e283 = vec3<f32>(_e277.x, _e277.y, f32(_e278.roughnessMap));
            let _e287 = textureSample(textures_t, pt_linear_repeat, _e283.xy, i32(_e283.z));
            roughness_23 = (_e275 * _e287.y);
        }
    }
    let _e290 = material_5;
    metalness_6 = _e290.metalness;
    let _e293 = material_5;
    if (_e293.metalnessMap != -1i) {
        {
            let _e298 = material_5;
            let _e300 = uv_27;
            uvPrime_7 = (_e298.metalnessMapTransform * vec3<f32>(_e300.x, _e300.y, 1f));
            let _e308 = metalness_6;
            let _e309 = uvPrime_7;
            let _e310 = _e309.xy;
            let _e311 = material_5;
            let _e316 = vec3<f32>(_e310.x, _e310.y, f32(_e311.metalnessMap));
            let _e320 = textureSample(textures_t, pt_linear_repeat, _e316.xy, i32(_e316.z));
            metalness_6 = (_e308 * _e320.z);
        }
    }
    let _e323 = material_5;
    let _e325 = material_5;
    emission = (_e323.emissiveIntensity * _e325.emissive);
    let _e329 = material_5;
    if (_e329.emissiveMap != -1i) {
        {
            let _e334 = material_5;
            let _e336 = uv_27;
            uvPrime_8 = (_e334.emissiveMapTransform * vec3<f32>(_e336.x, _e336.y, 1f));
            let _e344 = emission;
            let _e345 = uvPrime_8;
            let _e346 = _e345.xy;
            let _e347 = material_5;
            let _e352 = vec3<f32>(_e346.x, _e346.y, f32(_e347.emissiveMap));
            let _e356 = textureSample(textures_t, pt_linear_repeat, _e352.xy, i32(_e352.z));
            emission = (_e344 * _e356.xyz);
        }
    }
    let _e359 = material_5;
    transmission_3 = _e359.transmission;
    let _e362 = material_5;
    if (_e362.transmissionMap != -1i) {
        {
            let _e367 = material_5;
            let _e369 = uv_27;
            uvPrime_9 = (_e367.transmissionMapTransform * vec3<f32>(_e369.x, _e369.y, 1f));
            let _e377 = transmission_3;
            let _e378 = uvPrime_9;
            let _e379 = _e378.xy;
            let _e380 = material_5;
            let _e385 = vec3<f32>(_e379.x, _e379.y, f32(_e380.transmissionMap));
            let _e389 = textureSample(textures_t, pt_linear_repeat, _e385.xy, i32(_e385.z));
            transmission_3 = (_e377 * _e389.x);
        }
    }
    let _e392 = material_5;
    if _e392.flatShading {
        {
            let _e394 = surfaceHit_3;
            let _e396 = surfaceHit_3;
            normal_11 = (_e394.faceNormal * _e396.side);
        }
    }
    let _e399 = normal_11;
    baseNormal = _e399;
    let _e401 = material_5;
    if (_e401.normalMap != -1i) {
        {
            let _e407 = surfaceHit_3;
            let _e409 = surfaceHit_3;
            let _e412 = textureSampleBarycoord_1(attributesArray_t, attributesArray_s, 1i, _e407.barycoord, _e409.faceIndices.xyz);
            tangentSample = _e412;
            let _e414 = tangentSample;
            if (length(_e414.xyz) > 0f) {
                {
                    let _e419 = tangentSample;
                    tangent = normalize(_e419.xyz);
                    let _e423 = normal_11;
                    let _e424 = tangent;
                    let _e426 = tangentSample;
                    bitangent = normalize((cross(_e423, _e424) * _e426.w));
                    let _e431 = tangent;
                    let _e432 = bitangent;
                    let _e433 = normal_11;
                    vTBN = mat3x3<f32>(vec3<f32>(_e431.x, _e431.y, _e431.z), vec3<f32>(_e432.x, _e432.y, _e432.z), vec3<f32>(_e433.x, _e433.y, _e433.z));
                    let _e448 = material_5;
                    let _e450 = uv_27;
                    uvPrime_10 = (_e448.normalMapTransform * vec3<f32>(_e450.x, _e450.y, 1f));
                    let _e458 = uvPrime_10;
                    let _e459 = _e458.xy;
                    let _e460 = material_5;
                    let _e465 = vec3<f32>(_e459.x, _e459.y, f32(_e460.normalMap));
                    let _e469 = textureSample(textures_t, pt_linear_repeat, _e465.xy, i32(_e465.z));
                    texNormal = ((_e469.xyz * 2f) - vec3(1f));
                    let _e477 = texNormal;
                    let _e479 = texNormal;
                    let _e481 = material_5;
                    let _e483 = (_e479.xy * _e481.normalScale);
                    texNormal.x = _e483.x;
                    texNormal.y = _e483.y;
                    let _e488 = vTBN;
                    let _e489 = texNormal;
                    normal_11 = (_e488 * _e489);
                }
            }
        }
    }
    let _e491 = normal_11;
    let _e492 = surfaceHit_3;
    normal_11 = (_e491 * _e492.side);
    let _e495 = material_5;
    clearcoat = _e495.clearcoat;
    let _e498 = material_5;
    if (_e498.clearcoatMap != -1i) {
        {
            let _e503 = material_5;
            let _e505 = uv_27;
            uvPrime_11 = (_e503.clearcoatMapTransform * vec3<f32>(_e505.x, _e505.y, 1f));
            let _e513 = clearcoat;
            let _e514 = uvPrime_11;
            let _e515 = _e514.xy;
            let _e516 = material_5;
            let _e521 = vec3<f32>(_e515.x, _e515.y, f32(_e516.clearcoatMap));
            let _e525 = textureSample(textures_t, pt_linear_repeat, _e521.xy, i32(_e521.z));
            clearcoat = (_e513 * _e525.x);
        }
    }
    let _e528 = material_5;
    clearcoatRoughness = _e528.clearcoatRoughness;
    let _e531 = material_5;
    if (_e531.clearcoatRoughnessMap != -1i) {
        {
            let _e536 = material_5;
            let _e538 = uv_27;
            uvPrime_12 = (_e536.clearcoatRoughnessMapTransform * vec3<f32>(_e538.x, _e538.y, 1f));
            let _e546 = clearcoatRoughness;
            let _e547 = uvPrime_12;
            let _e548 = _e547.xy;
            let _e549 = material_5;
            let _e554 = vec3<f32>(_e548.x, _e548.y, f32(_e549.clearcoatRoughnessMap));
            let _e558 = textureSample(textures_t, pt_linear_repeat, _e554.xy, i32(_e554.z));
            clearcoatRoughness = (_e546 * _e558.y);
        }
    }
    let _e561 = baseNormal;
    clearcoatNormal = _e561;
    let _e563 = material_5;
    if (_e563.clearcoatNormalMap != -1i) {
        {
            let _e569 = surfaceHit_3;
            let _e571 = surfaceHit_3;
            let _e574 = textureSampleBarycoord_1(attributesArray_t, attributesArray_s, 1i, _e569.barycoord, _e571.faceIndices.xyz);
            tangentSample_1 = _e574;
            let _e576 = tangentSample_1;
            if (length(_e576.xyz) > 0f) {
                {
                    let _e581 = tangentSample_1;
                    tangent_1 = normalize(_e581.xyz);
                    let _e585 = clearcoatNormal;
                    let _e586 = tangent_1;
                    let _e588 = tangentSample_1;
                    bitangent_1 = normalize((cross(_e585, _e586) * _e588.w));
                    let _e593 = tangent_1;
                    let _e594 = bitangent_1;
                    let _e595 = clearcoatNormal;
                    vTBN_1 = mat3x3<f32>(vec3<f32>(_e593.x, _e593.y, _e593.z), vec3<f32>(_e594.x, _e594.y, _e594.z), vec3<f32>(_e595.x, _e595.y, _e595.z));
                    let _e610 = material_5;
                    let _e612 = uv_27;
                    uvPrime_13 = (_e610.clearcoatNormalMapTransform * vec3<f32>(_e612.x, _e612.y, 1f));
                    let _e620 = uvPrime_13;
                    let _e621 = _e620.xy;
                    let _e622 = material_5;
                    let _e627 = vec3<f32>(_e621.x, _e621.y, f32(_e622.clearcoatNormalMap));
                    let _e631 = textureSample(textures_t, pt_linear_repeat, _e627.xy, i32(_e627.z));
                    texNormal_1 = ((_e631.xyz * 2f) - vec3(1f));
                    let _e639 = texNormal_1;
                    let _e641 = texNormal_1;
                    let _e643 = material_5;
                    let _e645 = (_e641.xy * _e643.clearcoatNormalScale);
                    texNormal_1.x = _e645.x;
                    texNormal_1.y = _e645.y;
                    let _e650 = vTBN_1;
                    let _e651 = texNormal_1;
                    clearcoatNormal = (_e650 * _e651);
                }
            }
        }
    }
    let _e653 = clearcoatNormal;
    let _e654 = surfaceHit_3;
    clearcoatNormal = (_e653 * _e654.side);
    let _e657 = material_5;
    sheenColor_1 = _e657.sheenColor;
    let _e660 = material_5;
    if (_e660.sheenColorMap != -1i) {
        {
            let _e665 = material_5;
            let _e667 = uv_27;
            uvPrime_14 = (_e665.sheenColorMapTransform * vec3<f32>(_e667.x, _e667.y, 1f));
            let _e675 = sheenColor_1;
            let _e676 = uvPrime_14;
            let _e677 = _e676.xy;
            let _e678 = material_5;
            let _e683 = vec3<f32>(_e677.x, _e677.y, f32(_e678.sheenColorMap));
            let _e687 = textureSample(textures_t, pt_linear_repeat, _e683.xy, i32(_e683.z));
            sheenColor_1 = (_e675 * _e687.xyz);
        }
    }
    let _e690 = material_5;
    sheenRoughness = _e690.sheenRoughness;
    let _e693 = material_5;
    if (_e693.sheenRoughnessMap != -1i) {
        {
            let _e698 = material_5;
            let _e700 = uv_27;
            uvPrime_15 = (_e698.sheenRoughnessMapTransform * vec3<f32>(_e700.x, _e700.y, 1f));
            let _e708 = sheenRoughness;
            let _e709 = uvPrime_15;
            let _e710 = _e709.xy;
            let _e711 = material_5;
            let _e716 = vec3<f32>(_e710.x, _e710.y, f32(_e711.sheenRoughnessMap));
            let _e720 = textureSample(textures_t, pt_linear_repeat, _e716.xy, i32(_e716.z));
            sheenRoughness = (_e708 * _e720.w);
        }
    }
    let _e723 = material_5;
    iridescence = _e723.iridescence;
    let _e726 = material_5;
    if (_e726.iridescenceMap != -1i) {
        {
            let _e731 = material_5;
            let _e733 = uv_27;
            uvPrime_16 = (_e731.iridescenceMapTransform * vec3<f32>(_e733.x, _e733.y, 1f));
            let _e741 = iridescence;
            let _e742 = uvPrime_16;
            let _e743 = _e742.xy;
            let _e744 = material_5;
            let _e749 = vec3<f32>(_e743.x, _e743.y, f32(_e744.iridescenceMap));
            let _e753 = textureSample(textures_t, pt_linear_repeat, _e749.xy, i32(_e749.z));
            iridescence = (_e741 * _e753.x);
        }
    }
    let _e756 = material_5;
    iridescenceThickness = _e756.iridescenceThicknessMaximum;
    let _e759 = material_5;
    if (_e759.iridescenceThicknessMap != -1i) {
        {
            let _e764 = material_5;
            let _e766 = uv_27;
            uvPrime_17 = (_e764.iridescenceThicknessMapTransform * vec3<f32>(_e766.x, _e766.y, 1f));
            let _e774 = uvPrime_17;
            let _e775 = _e774.xy;
            let _e776 = material_5;
            let _e781 = vec3<f32>(_e775.x, _e775.y, f32(_e776.iridescenceThicknessMap));
            let _e785 = textureSample(textures_t, pt_linear_repeat, _e781.xy, i32(_e781.z));
            iridescenceThicknessSampled = _e785.y;
            let _e788 = material_5;
            let _e790 = material_5;
            let _e792 = iridescenceThicknessSampled;
            iridescenceThickness = mix(_e788.iridescenceThicknessMinimum, _e790.iridescenceThicknessMaximum, _e792);
        }
    }
    let _e794 = iridescenceThickness;
    if (_e794 == 0f) {
        local_29 = 0f;
    } else {
        let _e798 = iridescence;
        local_29 = _e798;
    }
    let _e800 = local_29;
    iridescence = _e800;
    let _e801 = material_5;
    specularColor = _e801.specularColor;
    let _e804 = material_5;
    if (_e804.specularColorMap != -1i) {
        {
            let _e809 = material_5;
            let _e811 = uv_27;
            uvPrime_18 = (_e809.specularColorMapTransform * vec3<f32>(_e811.x, _e811.y, 1f));
            let _e819 = specularColor;
            let _e820 = uvPrime_18;
            let _e821 = _e820.xy;
            let _e822 = material_5;
            let _e827 = vec3<f32>(_e821.x, _e821.y, f32(_e822.specularColorMap));
            let _e831 = textureSample(textures_t, pt_linear_repeat, _e827.xy, i32(_e827.z));
            specularColor = (_e819 * _e831.xyz);
        }
    }
    let _e834 = material_5;
    specularIntensity = _e834.specularIntensity;
    let _e837 = material_5;
    if (_e837.specularIntensityMap != -1i) {
        {
            let _e842 = material_5;
            let _e844 = uv_27;
            uvPrime_19 = (_e842.specularIntensityMapTransform * vec3<f32>(_e844.x, _e844.y, 1f));
            let _e852 = specularIntensity;
            let _e853 = uvPrime_19;
            let _e854 = _e853.xy;
            let _e855 = material_5;
            let _e860 = vec3<f32>(_e854.x, _e854.y, f32(_e855.specularIntensityMap));
            let _e864 = textureSample(textures_t, pt_linear_repeat, _e860.xy, i32(_e860.z));
            specularIntensity = (_e852 * _e864.w);
        }
    }
    (*surf_34).volumeParticle = false;
    let _e870 = surfaceHit_3;
    (*surf_34).faceNormal = _e870.faceNormal;
    let _e873 = normal_11;
    (*surf_34).normal = _e873;
    let _e875 = metalness_6;
    (*surf_34).metalness = _e875;
    let _e877 = albedo_1;
    (*surf_34).color = _e877.xyz;
    let _e880 = emission;
    (*surf_34).emission = _e880;
    let _e882 = material_5;
    (*surf_34).ior = _e882.ior;
    let _e885 = transmission_3;
    (*surf_34).transmission = _e885;
    let _e887 = material_5;
    (*surf_34).thinFilm = _e887.thinFilm;
    let _e890 = material_5;
    (*surf_34).attenuationColor = _e890.attenuationColor;
    let _e893 = material_5;
    (*surf_34).attenuationDistance = _e893.attenuationDistance;
    let _e896 = clearcoatNormal;
    (*surf_34).clearcoatNormal = _e896;
    let _e898 = clearcoat;
    (*surf_34).clearcoat = _e898;
    let _e900 = material_5;
    (*surf_34).sheen = _e900.sheen;
    let _e903 = sheenColor_1;
    (*surf_34).sheenColor = _e903;
    let _e905 = iridescence;
    (*surf_34).iridescence = _e905;
    let _e907 = material_5;
    (*surf_34).iridescenceIor = _e907.iridescenceIor;
    let _e910 = iridescenceThickness;
    (*surf_34).iridescenceThickness = _e910;
    let _e912 = specularColor;
    (*surf_34).specularColor = _e912;
    let _e914 = specularIntensity;
    (*surf_34).specularIntensity = _e914;
    let _e916 = roughness_23;
    let _e917 = roughness_23;
    (*surf_34).roughness = (_e916 * _e917);
    let _e920 = clearcoatRoughness;
    let _e921 = clearcoatRoughness;
    (*surf_34).clearcoatRoughness = (_e920 * _e921);
    let _e924 = sheenRoughness;
    (*surf_34).sheenRoughness = _e924;
    let _e926 = surfaceHit_3;
    let _e930 = transmission_3;
    (*surf_34).frontFace = ((_e926.side == 1f) || (_e930 == 0f));
    let _e935 = material_5;
    let _e937 = (*surf_34);
    if (_e935.thinFilm || _e937.frontFace) {
        let _e941 = material_5;
        local_30 = (1f / _e941.ior);
    } else {
        let _e944 = material_5;
        local_30 = _e944.ior;
    }
    let _e947 = local_30;
    (*surf_34).eta = _e947;
    let _e949 = (*surf_34);
    let _e951 = iorRatioToF0_(_e949.eta);
    (*surf_34).f0_ = _e951;
    let _e953 = (*surf_34);
    let _e955 = accumulatedRoughness_3;
    let _e956 = applyFilteredGlossy(_e953.roughness, _e955);
    (*surf_34).filteredRoughness = _e956;
    let _e958 = (*surf_34);
    let _e960 = accumulatedRoughness_3;
    let _e961 = applyFilteredGlossy(_e958.clearcoatRoughness, _e960);
    (*surf_34).filteredClearcoatRoughness = _e961;
    let _e963 = (*surf_34);
    let _e965 = getBasisFromNormal(_e963.normal);
    (*surf_34).normalBasis = _e965;
    let _e967 = (*surf_34);
    (*surf_34).normalInvBasis = _naga_inverse_3x3_f32(_e967.normalBasis);
    let _e971 = (*surf_34);
    let _e973 = getBasisFromNormal(_e971.clearcoatNormal);
    (*surf_34).clearcoatBasis = _e973;
    let _e975 = (*surf_34);
    (*surf_34).clearcoatInvBasis = _naga_inverse_3x3_f32(_e975.clearcoatBasis);
    return 1i;
}

fn main_1() {
    var ray_5: Ray;
    var local_31: f32;
    var surfaceHit_4: SurfaceHit;
    var scatterRec: ScatterRecord;
    var state_4: RenderState;
    var i_8: i32 = 0i;
    var hitType_1: i32;
    var lightRec_4: LightRecord;
    var local_32: f32;
    var lightDist_1: f32;
    var i_9: u32;
    var misWeight_2: f32;
    var envColor_1: vec3<f32>;
    var envPdf_1: f32;
    var misWeight_3: f32;
    var materialIndex_4: u32;
    var material_6: Material;
    var surf_35: SurfaceRecord;
    var isBelowSurface: bool;
    var local_33: vec3<f32>;
    var hitPoint: vec3<f32>;
    var halfVector_8: vec3<f32>;
    var isTransmissiveRay_1: bool;
    var minBounces: u32;
    var depthProb: f32;
    var rrProb: f32;

    let _e80 = gl_FragCoord_1;
    let _e82 = global.seed;
    rng_initialize(_e80.xy, _e82);
    let _e83 = gl_FragCoord_1;
    let _e89 = gl_FragCoord_1;
    sobolPixelIndex = ((u32(_e83.x) << 16u) | u32(_e89.y));
    let _e93 = global.seed;
    sobolPathIndex = u32(_e93);
    let _e95 = getCameraRay();
    ray_5 = _e95;
    let _e97 = global.environmentRotation;
    envRotation3x3_ = mat3x3<f32>(_e97[0].xyz, _e97[1].xyz, _e97[2].xyz);
    let _e107 = envRotation3x3_;
    invEnvRotation3x3_ = _naga_inverse_3x3_f32(_e107);
    let _e109 = global.environmentIntensity;
    let _e112 = global.envMapInfo_totalSum;
    let _e116 = global.lights_count;
    if (((_e109 == 0f) || (_e112 == 0f)) && (_e116 != 0u)) {
        let _e120 = global.lights_count;
        local_31 = f32(_e120);
    } else {
        let _e122 = global.lights_count;
        local_31 = f32((_e122 + 1u));
    }
    let _e127 = local_31;
    lightsDenom = _e127;
    pc_fragColor = vec4<f32>(0f, 0f, 0f, 1f);
    let _e139 = initRenderState();
    state_4 = _e139;
    let _e142 = global.transmissiveBounces;
    state_4.transmissiveTraversals = _e142;
    loop {
        let _e145 = i_8;
        let _e146 = global.bounces;
        if !((_e145 < _e146)) {
            break;
        }
        {
            let _e152 = sobolBounceIndex;
            sobolBounceIndex = (_e152 + 1u);
            let _e156 = state_4.depth;
            state_4.depth = (_e156 + 1u);
            let _e160 = global.bounces;
            let _e161 = i_8;
            state_4.traversals = (_e160 - _e161);
            let _e164 = i_8;
            let _e167 = state_4;
            let _e169 = global.transmissiveBounces;
            state_4.firstRay = ((_e164 == 0i) && (_e167.transmissiveTraversals == _e169));
            let _e172 = ray_5;
            let _e173 = state_4;
            let _e177 = traceScene(_e172, _e173.fogMaterial, (&surfaceHit_4));
            hitType_1 = _e177;
            let _e179 = state_4;
            let _e182 = state_4;
            if (!(_e179.firstRay) && !(_e182.transmissiveRay)) {
                {
                    let _e187 = hitType_1;
                    if (_e187 == 0i) {
                        local_32 = 100000000000000000000f;
                    } else {
                        let _e191 = surfaceHit_4;
                        local_32 = _e191.dist;
                    }
                    let _e194 = local_32;
                    lightDist_1 = _e194;
                    i_9 = 0u;
                    loop {
                        let _e198 = i_9;
                        let _e199 = global.lights_count;
                        if !((_e198 < _e199)) {
                            break;
                        }
                        {
                            let _e205 = ray_5;
                            let _e207 = ray_5;
                            let _e209 = i_9;
                            let _e212 = intersectLightAtIndex(lights_tex_t, pt_nearest, _e205.origin, _e207.direction, _e209, (&lightRec_4));
                            let _e213 = lightRec_4;
                            let _e215 = lightDist_1;
                            if (_e212 && (_e213.dist < _e215)) {
                                {
                                    let _e218 = scatterRec;
                                    let _e220 = lightRec_4;
                                    let _e222 = lightsDenom;
                                    let _e224 = misHeuristic(_e218.pdf, (_e220.pdf / _e222));
                                    misWeight_2 = _e224;
                                    let _e226 = pc_fragColor;
                                    let _e228 = pc_fragColor;
                                    let _e230 = lightRec_4;
                                    let _e232 = state_4;
                                    let _e235 = misWeight_2;
                                    let _e237 = (_e228.xyz + ((_e230.emission * _e232.throughputColor) * _e235));
                                    pc_fragColor.x = _e237.x;
                                    pc_fragColor.y = _e237.y;
                                    pc_fragColor.z = _e237.z;
                                }
                            }
                        }
                        continuing {
                            let _e202 = i_9;
                            i_9 = (_e202 + 1u);
                        }
                    }
                }
            }
            let _e244 = hitType_1;
            if (_e244 == 0i) {
                {
                    let _e247 = state_4;
                    let _e249 = state_4;
                    if (_e247.firstRay || _e249.transmissiveRay) {
                        {
                            let _e252 = pc_fragColor;
                            let _e254 = pc_fragColor;
                            let _e256 = ray_5;
                            let _e259 = rand2_(2i);
                            let _e260 = sampleBackground(_e256.direction, _e259);
                            let _e261 = state_4;
                            let _e264 = (_e254.xyz + (_e260 * _e261.throughputColor));
                            pc_fragColor.x = _e264.x;
                            pc_fragColor.y = _e264.y;
                            pc_fragColor.z = _e264.z;
                            let _e272 = global.backgroundAlpha;
                            pc_fragColor.w = _e272;
                        }
                    } else {
                        {
                            let _e274 = envRotation3x3_;
                            let _e275 = ray_5;
                            let _e280 = sampleEquirect((_e274 * _e275.direction), (&envColor_1));
                            envPdf_1 = _e280;
                            let _e282 = envPdf_1;
                            let _e283 = lightsDenom;
                            envPdf_1 = (_e282 / _e283);
                            let _e285 = scatterRec;
                            let _e287 = envPdf_1;
                            let _e288 = misHeuristic(_e285.pdf, _e287);
                            misWeight_3 = _e288;
                            let _e290 = pc_fragColor;
                            let _e292 = pc_fragColor;
                            let _e294 = global.environmentIntensity;
                            let _e295 = envColor_1;
                            let _e297 = state_4;
                            let _e300 = misWeight_3;
                            let _e302 = (_e292.xyz + (((_e294 * _e295) * _e297.throughputColor) * _e300));
                            pc_fragColor.x = _e302.x;
                            pc_fragColor.y = _e302.y;
                            pc_fragColor.z = _e302.z;
                        }
                    }
                    break;
                }
            }
            let _e309 = surfaceHit_4;
            let _e312 = uTexelFetch1D(materialIndexAttribute_t_1, pt_nearest, _e309.faceIndices.x);
            materialIndex_4 = _e312.x;
            let _e315 = materialIndex_4;
            let _e316 = readMaterialInfo(materials_t_2, pt_nearest, _e315);
            material_6 = _e316;
            let _e318 = material_6;
            let _e320 = state_4;
            if (_e318.matte && _e320.firstRay) {
                {
                    pc_fragColor = vec4(0f);
                    break;
                }
            }
            let _e325 = material_6;
            let _e328 = state_4;
            if (!(_e325.castShadow) && _e328.isShadowRay) {
                {
                    let _e332 = ray_5;
                    let _e334 = ray_5;
                    let _e336 = surfaceHit_4;
                    let _e339 = surfaceHit_4;
                    let _e341 = stepRayOrigin(_e332.origin, _e334.direction, -(_e336.faceNormal), _e339.dist);
                    ray_5.origin = _e341;
                    continue;
                }
            }
            let _e343 = material_6;
            let _e344 = surfaceHit_4;
            let _e345 = state_4;
            let _e349 = getSurfaceRecord(_e343, _e344, attributesArray_t_1, pt_nearest, _e345.accumulatedRoughness, (&surf_35));
            if (_e349 == 0i) {
                {
                    let _e352 = i_8;
                    let _e353 = state_4;
                    i_8 = (_e352 - sign(_e353.transmissiveTraversals));
                    let _e358 = state_4;
                    let _e360 = state_4;
                    state_4.transmissiveTraversals = (_e358.transmissiveTraversals - sign(_e360.transmissiveTraversals));
                    let _e365 = ray_5;
                    let _e367 = ray_5;
                    let _e369 = surfaceHit_4;
                    let _e372 = surfaceHit_4;
                    let _e374 = stepRayOrigin(_e365.origin, _e367.direction, -(_e369.faceNormal), _e372.dist);
                    ray_5.origin = _e374;
                    continue;
                }
            }
            let _e375 = ray_5;
            let _e378 = surf_35;
            let _e379 = bsdfSample(-(_e375.direction), _e378);
            scatterRec = _e379;
            let _e381 = scatterRec;
            let _e384 = rand_1(4i);
            state_4.isShadowRay = (_e381.specularPdf < _e384);
            let _e386 = surf_35;
            let _e389 = scatterRec;
            let _e391 = surf_35;
            isBelowSurface = (!(_e386.volumeParticle) && (dot(_e389.direction, _e391.faceNormal) < 0f));
            let _e398 = ray_5;
            let _e400 = ray_5;
            let _e402 = isBelowSurface;
            if _e402 {
                let _e403 = surf_35;
                local_33 = -(_e403.faceNormal);
            } else {
                let _e406 = surf_35;
                local_33 = _e406.faceNormal;
            }
            let _e409 = local_33;
            let _e410 = surfaceHit_4;
            let _e412 = stepRayOrigin(_e398.origin, _e400.direction, _e409, _e410.dist);
            hitPoint = _e412;
            let _e414 = pc_fragColor;
            let _e416 = pc_fragColor;
            let _e418 = ray_5;
            let _e421 = surf_35;
            let _e422 = state_4;
            let _e423 = hitPoint;
            let _e424 = directLightContribution(-(_e418.direction), _e421, _e422, _e423);
            let _e425 = (_e416.xyz + _e424);
            pc_fragColor.x = _e425.x;
            pc_fragColor.y = _e425.y;
            pc_fragColor.z = _e425.z;
            let _e432 = surf_35;
            let _e435 = isBelowSurface;
            if (!(_e432.volumeParticle) && !(_e435)) {
                {
                    let _e438 = ray_5;
                    let _e441 = scatterRec;
                    halfVector_8 = normalize((-(_e438.direction) + _e441.direction));
                    let _e447 = state_4;
                    let _e449 = halfVector_8;
                    let _e450 = surf_35;
                    let _e453 = acosApprox(dot(_e449, _e450.normal));
                    let _e455 = halfVector_8;
                    let _e456 = surf_35;
                    let _e459 = acosApprox(dot(_e455, _e456.clearcoatNormal));
                    state_4.accumulatedRoughness = (_e447.accumulatedRoughness + max(sin(_e453), sin(_e459)));
                    state_4.transmissiveRay = false;
                }
            }
            let _e465 = pc_fragColor;
            let _e467 = pc_fragColor;
            let _e469 = surf_35;
            let _e471 = state_4;
            let _e474 = (_e467.xyz + (_e469.emission * _e471.throughputColor));
            pc_fragColor.x = _e474.x;
            pc_fragColor.y = _e474.y;
            pc_fragColor.z = _e474.z;
            let _e481 = scatterRec;
            let _e485 = scatterRec;
            let _e487 = surf_35;
            let _e489 = surf_35;
            let _e491 = isDirectionValid(_e485.direction, _e487.normal, _e489.faceNormal);
            if ((_e481.pdf <= 0f) || !(_e491)) {
                {
                    break;
                }
            }
            let _e494 = surf_35;
            let _e497 = scatterRec;
            let _e499 = surf_35;
            let _e501 = surfaceHit_4;
            isTransmissiveRay_1 = (!(_e494.volumeParticle) && (dot(_e497.direction, (_e499.faceNormal * _e501.side)) < 0f));
            let _e509 = isTransmissiveRay_1;
            let _e510 = isBelowSurface;
            let _e512 = state_4;
            if ((_e509 || _e510) && (_e512.transmissiveTraversals > 0i)) {
                {
                    let _e518 = state_4.transmissiveTraversals;
                    state_4.transmissiveTraversals = (_e518 - 1i);
                    let _e521 = i_8;
                    i_8 = (_e521 - 1i);
                }
            }
            let _e524 = surf_35;
            if !(_e524.frontFace) {
                {
                    let _e528 = state_4;
                    let _e530 = surfaceHit_4;
                    let _e532 = surf_35;
                    let _e534 = surf_35;
                    let _e536 = transmissionAttenuation(_e530.dist, _e532.attenuationColor, _e534.attenuationDistance);
                    state_4.throughputColor = (_e528.throughputColor * _e536);
                }
            }
            minBounces = 3u;
            let _e540 = state_4;
            let _e542 = minBounces;
            depthProb = select(0f, 1f, (_e540.depth < _e542));
            let _e548 = state_4;
            let _e550 = scatterRec;
            let _e553 = scatterRec;
            let _e557 = luminance(((_e548.throughputColor * _e550.color) / vec3(_e553.pdf)));
            rrProb = _e557;
            let _e559 = rrProb;
            let _e560 = state_4;
            let _e562 = luminance(_e560.throughputColor);
            rrProb = (_e559 / _e562);
            let _e564 = rrProb;
            rrProb = sqrt(_e564);
            let _e566 = rrProb;
            let _e567 = depthProb;
            rrProb = max(_e566, _e567);
            let _e569 = rrProb;
            rrProb = min(_e569, 1f);
            let _e573 = rand_1(8i);
            let _e574 = rrProb;
            if (_e573 > _e574) {
                {
                    break;
                }
            }
            let _e577 = state_4;
            let _e580 = rrProb;
            state_4.throughputColor = (_e577.throughputColor * min((1f / _e580), 20f));
            let _e586 = state_4;
            let _e588 = scatterRec;
            let _e590 = scatterRec;
            state_4.throughputColor = (_e586.throughputColor * (_e588.color / vec3(_e590.pdf)));
            let _e595 = state_4;
            let _e597 = pt_isnan(_e595.throughputColor);
            let _e599 = state_4;
            let _e601 = pt_isinf(_e599.throughputColor);
            if (any(_e597) || any(_e601)) {
                {
                    break;
                }
            }
            let _e605 = scatterRec;
            ray_5.direction = _e605.direction;
            let _e608 = hitPoint;
            ray_5.origin = _e608;
        }
        continuing {
            let _e149 = i_8;
            i_8 = (_e149 + 1i);
        }
    }
    let _e610 = pc_fragColor;
    let _e612 = global.opacity;
    pc_fragColor.w = (_e610.w * _e612);
    return;
}

@fragment 
fn main(@location(1) vUv: vec2<f32>, @builtin(position) gl_FragCoord: vec4<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e121 = pc_fragColor;
    return FragmentOutput(_e121);
}

fn _naga_inverse_3x3_f32(m: mat3x3<f32>) -> mat3x3<f32> {
    var adj: mat3x3<f32>;

    adj[0][0] =   (m[1][1] * m[2][2] - m[2][1] * m[1][2]);
    adj[1][0] = - (m[1][0] * m[2][2] - m[2][0] * m[1][2]);
    adj[2][0] =   (m[1][0] * m[2][1] - m[2][0] * m[1][1]);
    adj[0][1] = - (m[0][1] * m[2][2] - m[2][1] * m[0][2]);
    adj[1][1] =   (m[0][0] * m[2][2] - m[2][0] * m[0][2]);
    adj[2][1] = - (m[0][0] * m[2][1] - m[2][0] * m[0][1]);
    adj[0][2] =   (m[0][1] * m[1][2] - m[1][1] * m[0][2]);
    adj[1][2] = - (m[0][0] * m[1][2] - m[1][0] * m[0][2]);
    adj[2][2] =   (m[0][0] * m[1][1] - m[1][0] * m[0][1]);

    let det: f32 = (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
    		- m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
    		+ m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]));

    return adj * (1 / det);
}
