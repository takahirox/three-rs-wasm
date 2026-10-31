diagnostic(off, derivative_uniformity);
struct Uniforms {
    modelMatrix: mat4x4<f32>,
    modelViewMatrix: mat4x4<f32>,
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
    normalMatrix: mat3x3<f32>,
    cameraPosition: vec3<f32>,
    isOrthographic: i32,
    texelSize: vec2<f32>,
    resolution: vec2<f32>,
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
@group(0) @binding(3) 
var areaTexture_t: texture_2d<f32>;
@group(0) @binding(4) 
var areaTexture_s: sampler;
@group(0) @binding(5) 
var searchTexture_t: texture_2d<f32>;
@group(0) @binding(6) 
var searchTexture_s: sampler;
var<private> vUv_1: vec2<f32>;
var<private> vOffset_3: array<vec4<f32>, 3>;
var<private> vPixCoord_1: vec2<f32>;
var<private> pc_fragColor: vec4<f32>;

fn pt_pow(x: f32, y: f32) -> f32 {
    var x_1: f32;
    var y_1: f32;

    x_1 = x;
    y_1 = y;
    let _e28 = y_1;
    let _e29 = x_1;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_1(x_2: vec2<f32>, y_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var y_3: vec2<f32>;

    x_3 = x_2;
    y_3 = y_2;
    let _e28 = y_3;
    let _e29 = x_3;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_2(x_4: vec3<f32>, y_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var y_5: vec3<f32>;

    x_5 = x_4;
    y_5 = y_4;
    let _e28 = y_5;
    let _e29 = x_5;
    return exp2((_e28 * log2(_e29)));
}

fn pt_pow_3(x_6: vec4<f32>, y_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var y_7: vec4<f32>;

    x_7 = x_6;
    y_7 = y_6;
    let _e28 = y_7;
    let _e29 = x_7;
    return exp2((_e28 * log2(_e29)));
}

fn pt_modf(x_8: f32, i: ptr<function, f32>) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e27 = x_9;
    (*i) = trunc(_e27);
    let _e29 = x_9;
    let _e30 = (*i);
    return (_e29 - _e30);
}

fn pt_modf_1(x_10: vec2<f32>, i_1: ptr<function, vec2<f32>>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e27 = x_11;
    (*i_1) = trunc(_e27);
    let _e29 = x_11;
    let _e30 = (*i_1);
    return (_e29 - _e30);
}

fn pt_modf_2(x_12: vec3<f32>, i_2: ptr<function, vec3<f32>>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e27 = x_13;
    (*i_2) = trunc(_e27);
    let _e29 = x_13;
    let _e30 = (*i_2);
    return (_e29 - _e30);
}

fn pt_modf_3(x_14: vec4<f32>, i_3: ptr<function, vec4<f32>>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e27 = x_15;
    (*i_3) = trunc(_e27);
    let _e29 = x_15;
    let _e30 = (*i_3);
    return (_e29 - _e30);
}

fn pt_isnan(v: f32) -> bool {
    var v_1: f32;
    var b: u32;

    v_1 = v;
    let _e26 = v_1;
    b = (bitcast<u32>(_e26) & 2147483647u);
    let _e31 = b;
    return (_e31 > 2139095040u);
}

fn pt_isinf(v_2: f32) -> bool {
    var v_3: f32;
    var b_1: u32;

    v_3 = v_2;
    let _e26 = v_3;
    b_1 = (bitcast<u32>(_e26) & 2147483647u);
    let _e31 = b_1;
    return (_e31 == 2139095040u);
}

fn pt_isnan_1(v_4: vec2<f32>) -> vec2<bool> {
    var v_5: vec2<f32>;
    var b_2: vec2<u32>;

    v_5 = v_4;
    let _e26 = v_5;
    b_2 = (bitcast<vec2<u32>>(_e26) & vec2(2147483647u));
    let _e32 = b_2;
    return (_e32 > vec2(2139095040u));
}

fn pt_isnan_2(v_6: vec3<f32>) -> vec3<bool> {
    var v_7: vec3<f32>;
    var b_3: vec3<u32>;

    v_7 = v_6;
    let _e26 = v_7;
    b_3 = (bitcast<vec3<u32>>(_e26) & vec3(2147483647u));
    let _e32 = b_3;
    return (_e32 > vec3(2139095040u));
}

fn pt_isnan_3(v_8: vec4<f32>) -> vec4<bool> {
    var v_9: vec4<f32>;
    var b_4: vec4<u32>;

    v_9 = v_8;
    let _e26 = v_9;
    b_4 = (bitcast<vec4<u32>>(_e26) & vec4(2147483647u));
    let _e32 = b_4;
    return (_e32 > vec4(2139095040u));
}

fn pt_isinf_1(v_10: vec2<f32>) -> vec2<bool> {
    var v_11: vec2<f32>;
    var b_5: vec2<u32>;

    v_11 = v_10;
    let _e26 = v_11;
    b_5 = (bitcast<vec2<u32>>(_e26) & vec2(2147483647u));
    let _e32 = b_5;
    return (_e32 == vec2(2139095040u));
}

fn pt_isinf_2(v_12: vec3<f32>) -> vec3<bool> {
    var v_13: vec3<f32>;
    var b_6: vec3<u32>;

    v_13 = v_12;
    let _e26 = v_13;
    b_6 = (bitcast<vec3<u32>>(_e26) & vec3(2147483647u));
    let _e32 = b_6;
    return (_e32 == vec3(2139095040u));
}

fn pt_isinf_3(v_14: vec4<f32>) -> vec4<bool> {
    var v_15: vec4<f32>;
    var b_7: vec4<u32>;

    v_15 = v_14;
    let _e26 = v_15;
    b_7 = (bitcast<vec4<u32>>(_e26) & vec4(2147483647u));
    let _e32 = b_7;
    return (_e32 == vec4(2139095040u));
}

fn LinearTransferOETF(value: vec4<f32>) -> vec4<f32> {
    var value_1: vec4<f32>;

    value_1 = value;
    let _e30 = value_1;
    return _e30;
}

fn sRGBTransferEOTF(value_2: vec4<f32>) -> vec4<f32> {
    var value_3: vec4<f32>;

    value_3 = value_2;
    let _e30 = value_3;
    let _e39 = pt_pow_2(((_e30.xyz * 0.9478673f) + vec3(0.0521327f)), vec3(2.4f));
    let _e40 = value_3;
    let _e44 = value_3;
    let _e54 = mix(_e39, (_e40.xyz * 0.07739938f), select(vec3(0f), vec3(1f), (_e44.xyz <= vec3(0.04045f))));
    let _e55 = value_3;
    return vec4<f32>(_e54.x, _e54.y, _e54.z, _e55.w);
}

fn sRGBTransferOETF(value_4: vec4<f32>) -> vec4<f32> {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e30 = value_5;
    let _e34 = pt_pow_2(_e30.xyz, vec3(0.41666f));
    let _e40 = value_5;
    let _e44 = value_5;
    let _e54 = mix(((_e34 * 1.055f) - vec3(0.055f)), (_e40.xyz * 12.92f), select(vec3(0f), vec3(1f), (_e44.xyz <= vec3(0.0031308f))));
    let _e55 = value_5;
    return vec4<f32>(_e54.x, _e54.y, _e54.z, _e55.w);
}

fn linearToOutputTexel(value_6: vec4<f32>) -> vec4<f32> {
    var value_7: vec4<f32>;

    value_7 = value_6;
    let _e47 = value_7;
    let _e49 = (mat3x3<f32>(vec3<f32>(1f, 0f, -0f), vec3<f32>(-0f, 1f, -0f), vec3<f32>(-0f, 0f, 1f)) * _e47.xyz);
    let _e50 = value_7;
    let _e56 = LinearTransferOETF(vec4<f32>(_e49.x, _e49.y, _e49.z, _e50.w));
    return _e56;
}

fn luminance(rgb: vec3<f32>) -> f32 {
    var rgb_1: vec3<f32>;
    var weights: vec3<f32> = vec3<f32>(0.2126f, 0.7152f, 0.0722f);

    rgb_1 = rgb;
    let _e35 = weights;
    let _e36 = rgb_1;
    return dot(_e35, _e36);
}

fn movec(c: vec2<bool>, variable: ptr<function, vec2<f32>>, value_8: vec2<f32>) {
    var c_1: vec2<bool>;
    var value_9: vec2<f32>;

    c_1 = c;
    value_9 = value_8;
    let _e33 = c_1;
    if _e33.x {
        {
            let _e36 = value_9;
            (*variable).x = _e36.x;
        }
    }
    let _e38 = c_1;
    if _e38.y {
        {
            let _e41 = value_9;
            (*variable).y = _e41.y;
            return;
        }
    } else {
        return;
    }
}

fn movec_1(c_2: vec4<bool>, variable_1: ptr<function, vec4<f32>>, value_10: vec4<f32>) {
    var c_3: vec4<bool>;
    var value_11: vec4<f32>;
    var local: vec2<f32>;
    var local_1: vec2<f32>;

    c_3 = c_2;
    value_11 = value_10;
    let _e33 = c_3;
    let _e35 = (*variable_1);
    let _e37 = value_11;
    let _e39 = (*variable_1);
    local = _e39.xy;
    movec(_e33.xy, (&local), _e37.xy);
    let _e46 = local.x;
    (*variable_1).x = _e46;
    let _e47 = local.y;
    (*variable_1).y = _e47;
    let _e48 = c_3;
    let _e50 = (*variable_1);
    let _e52 = value_11;
    let _e54 = (*variable_1);
    local_1 = _e54.zw;
    movec(_e48.zw, (&local_1), _e52.zw);
    let _e61 = local_1.x;
    (*variable_1).z = _e61;
    let _e62 = local_1.y;
    (*variable_1).w = _e62;
    return;
}

fn decodeDiagBilinearAccess(e: vec2<f32>) -> vec2<f32> {
    var e_1: vec2<f32>;

    e_1 = e;
    let _e31 = e_1;
    let _e34 = e_1;
    e_1.x = (_e31.x * abs(((5f * _e34.x) - 3.75f)));
    let _e43 = e_1;
    return round(_e43);
}

fn decodeDiagBilinearAccess_1(e_2: vec4<f32>) -> vec4<f32> {
    var e_3: vec4<f32>;

    e_3 = e_2;
    let _e30 = e_3;
    let _e32 = e_3;
    let _e35 = e_3;
    let _e44 = (_e32.xz * abs(((5f * _e35.xz) - vec2(3.75f))));
    e_3.x = _e44.x;
    e_3.z = _e44.y;
    let _e49 = e_3;
    return round(_e49);
}

fn searchDiag1_(texCoord: vec2<f32>, dir: vec2<f32>, e_4: ptr<function, vec2<f32>>) -> vec2<f32> {
    var texCoord_1: vec2<f32>;
    var dir_1: vec2<f32>;
    var coord: vec4<f32>;
    var t: vec3<f32>;
    var i_4: i32 = 0i;

    texCoord_1 = texCoord;
    dir_1 = dir;
    let _e33 = texCoord_1;
    coord = vec4<f32>(_e33.x, _e33.y, -1f, 1f);
    let _e41 = global.texelSize;
    t = vec3<f32>(_e41.x, _e41.y, 1f);
    loop {
        let _e49 = i_4;
        if !((_e49 < 8i)) {
            break;
        }
        {
            let _e56 = coord;
            let _e63 = coord;
            if !(((_e56.z < 7f) && (_e63.w > 0.9f))) {
                {
                    break;
                }
            }
            let _e69 = coord;
            let _e71 = t;
            let _e72 = dir_1;
            let _e78 = coord;
            let _e80 = ((_e71 * vec3<f32>(_e72.x, _e72.y, 1f)) + _e78.xyz);
            coord.x = _e80.x;
            coord.y = _e80.y;
            coord.z = _e80.z;
            let _e87 = coord;
            let _e89 = textureSample(inputBuffer_t, inputBuffer_s, _e87.xy);
            (*e_4) = _e89.xy;
            let _e92 = (*e_4);
            coord.w = dot(_e92, vec2(0.5f));
        }
        continuing {
            let _e53 = i_4;
            i_4 = (_e53 + 1i);
        }
    }
    let _e96 = coord;
    return _e96.zw;
}

fn searchDiag2_(texCoord_2: vec2<f32>, dir_2: vec2<f32>, e_5: ptr<function, vec2<f32>>) -> vec2<f32> {
    var texCoord_3: vec2<f32>;
    var dir_3: vec2<f32>;
    var coord_1: vec4<f32>;
    var t_1: vec3<f32>;
    var i_5: i32 = 0i;

    texCoord_3 = texCoord_2;
    dir_3 = dir_2;
    let _e33 = texCoord_3;
    coord_1 = vec4<f32>(_e33.x, _e33.y, -1f, 1f);
    let _e42 = coord_1;
    let _e45 = global.texelSize;
    coord_1.x = (_e42.x + (0.25f * _e45.x));
    let _e49 = global.texelSize;
    t_1 = vec3<f32>(_e49.x, _e49.y, 1f);
    loop {
        let _e57 = i_5;
        if !((_e57 < 8i)) {
            break;
        }
        {
            let _e64 = coord_1;
            let _e71 = coord_1;
            if !(((_e64.z < 7f) && (_e71.w > 0.9f))) {
                {
                    break;
                }
            }
            let _e77 = coord_1;
            let _e79 = t_1;
            let _e80 = dir_3;
            let _e86 = coord_1;
            let _e88 = ((_e79 * vec3<f32>(_e80.x, _e80.y, 1f)) + _e86.xyz);
            coord_1.x = _e88.x;
            coord_1.y = _e88.y;
            coord_1.z = _e88.z;
            let _e95 = coord_1;
            let _e97 = textureSample(inputBuffer_t, inputBuffer_s, _e95.xy);
            (*e_5) = _e97.xy;
            let _e99 = (*e_5);
            let _e100 = decodeDiagBilinearAccess(_e99);
            (*e_5) = _e100;
            let _e102 = (*e_5);
            coord_1.w = dot(_e102, vec2(0.5f));
        }
        continuing {
            let _e61 = i_5;
            i_5 = (_e61 + 1i);
        }
    }
    let _e106 = coord_1;
    return _e106.zw;
}

fn areaDiag(dist: vec2<f32>, e_6: vec2<f32>, offset: f32) -> vec2<f32> {
    var dist_1: vec2<f32>;
    var e_7: vec2<f32>;
    var offset_1: f32;
    var texCoord_4: vec2<f32>;

    dist_1 = dist;
    e_7 = e_6;
    offset_1 = offset;
    let _e37 = e_7;
    let _e39 = dist_1;
    texCoord_4 = ((vec2<f32>(20f, 20f) * _e37) + _e39);
    let _e52 = texCoord_4;
    texCoord_4 = ((vec2<f32>(0.00625f, 0.0017857143f) * _e52) + vec2<f32>(0.003125f, 0.00089285715f));
    let _e71 = texCoord_4;
    texCoord_4.x = (_e71.x + 0.5f);
    let _e76 = texCoord_4;
    let _e81 = offset_1;
    texCoord_4.y = (_e76.y + (0.14285715f * _e81));
    let _e84 = texCoord_4;
    let _e85 = textureSample(areaTexture_t, areaTexture_s, _e84);
    return _e85.xy;
}

fn calculateDiagWeights(texCoord_5: vec2<f32>, e_8: vec2<f32>, subsampleIndices: vec4<f32>) -> vec2<f32> {
    var texCoord_6: vec2<f32>;
    var e_9: vec2<f32>;
    var subsampleIndices_1: vec4<f32>;
    var weights_1: vec2<f32> = vec2(0f);
    var d: vec4<f32>;
    var end: vec2<f32>;
    var coords: vec4<f32>;
    var c_4: vec4<f32>;
    var cc: vec2<f32>;
    var coords_1: vec4<f32>;
    var c_5: vec4<f32>;
    var cc_1: vec2<f32>;

    texCoord_6 = texCoord_5;
    e_9 = e_8;
    subsampleIndices_1 = subsampleIndices;
    let _e39 = e_9;
    if (_e39.x > 0f) {
        {
            let _e43 = d;
            let _e45 = texCoord_6;
            let _e52 = searchDiag1_(_e45, vec2<f32>(-1f, 1f), (&end));
            d.x = _e52.x;
            d.z = _e52.y;
            let _e58 = d;
            let _e60 = end;
            d.x = (_e58.x + select(0f, 1f, (_e60.y > 0.9f)));
        }
    } else {
        {
            let _e68 = d;
            d.x = 0f;
            d.z = 0f;
        }
    }
    let _e74 = d;
    let _e76 = texCoord_6;
    let _e83 = searchDiag1_(_e76, vec2<f32>(1f, -1f), (&end));
    d.y = _e83.x;
    d.w = _e83.y;
    let _e88 = d;
    let _e90 = d;
    if ((_e88.x + _e90.y) > 2f) {
        {
            let _e95 = d;
            let _e100 = d;
            let _e102 = d;
            let _e104 = d;
            let _e110 = global.texelSize;
            let _e113 = texCoord_6;
            coords = ((vec4<f32>((-(_e95.x) + 0.25f), _e100.x, _e102.y, (-(_e104.y) - 0.25f)) * _e110.xyxy) + _e113.xyxy);
            let _e118 = c_4;
            let _e120 = coords;
            let _e128 = global.texelSize;
            let _e131 = textureSample(inputBuffer_t, inputBuffer_s, (_e120.xy + (vec2<f32>(-1f, 0f) * _e128)));
            let _e132 = _e131.xy;
            c_4.x = _e132.x;
            c_4.y = _e132.y;
            let _e137 = c_4;
            let _e139 = coords;
            let _e146 = global.texelSize;
            let _e149 = textureSample(inputBuffer_t, inputBuffer_s, (_e139.zw + (vec2<f32>(1f, 0f) * _e146)));
            let _e150 = _e149.xy;
            c_4.z = _e150.x;
            c_4.w = _e150.y;
            let _e155 = c_4;
            let _e157 = c_4;
            let _e159 = decodeDiagBilinearAccess_1(_e157.xyzw);
            c_4.y = _e159.x;
            c_4.x = _e159.y;
            c_4.w = _e159.z;
            c_4.z = _e159.w;
            let _e170 = c_4;
            let _e173 = c_4;
            cc = ((vec2(2f) * _e170.xz) + _e173.yw);
            let _e178 = d;
            movec(vec2<bool>(step(vec2(0.9f), _e178.zw)), (&cc), vec2(0f));
            let _e187 = weights_1;
            let _e188 = d;
            let _e190 = cc;
            let _e191 = subsampleIndices_1;
            let _e193 = areaDiag(_e188.xy, _e190, _e191.z);
            weights_1 = (_e187 + _e193);
        }
    }
    let _e195 = d;
    let _e197 = texCoord_6;
    let _e205 = searchDiag2_(_e197, vec2<f32>(-1f, -1f), (&end));
    d.x = _e205.x;
    d.z = _e205.y;
    let _e210 = texCoord_6;
    let _e216 = global.texelSize;
    let _e219 = textureSample(inputBuffer_t, inputBuffer_s, (_e210 + (vec2<f32>(1f, 0f) * _e216)));
    if (_e219.x > 0f) {
        {
            let _e223 = d;
            let _e225 = texCoord_6;
            let _e230 = searchDiag2_(_e225, vec2(1f), (&end));
            d.y = _e230.x;
            d.w = _e230.y;
            let _e236 = d;
            let _e238 = end;
            d.y = (_e236.y + select(0f, 1f, (_e238.y > 0.9f)));
        }
    } else {
        {
            let _e246 = d;
            d.y = 0f;
            d.w = 0f;
        }
    }
    let _e252 = d;
    let _e254 = d;
    if ((_e252.x + _e254.y) > 2f) {
        {
            let _e259 = d;
            let _e262 = d;
            let _e265 = d;
            let _e267 = d;
            let _e270 = global.texelSize;
            let _e273 = texCoord_6;
            coords_1 = ((vec4<f32>(-(_e259.x), -(_e262.x), _e265.y, _e267.y) * _e270.xyxy) + _e273.xyxy);
            let _e279 = coords_1;
            let _e287 = global.texelSize;
            let _e290 = textureSample(inputBuffer_t, inputBuffer_s, (_e279.xy + (vec2<f32>(-1f, 0f) * _e287)));
            c_5.x = _e290.y;
            let _e293 = coords_1;
            let _e301 = global.texelSize;
            let _e304 = textureSample(inputBuffer_t, inputBuffer_s, (_e293.xy + (vec2<f32>(0f, -1f) * _e301)));
            c_5.y = _e304.x;
            let _e306 = c_5;
            let _e308 = coords_1;
            let _e315 = global.texelSize;
            let _e318 = textureSample(inputBuffer_t, inputBuffer_s, (_e308.zw + (vec2<f32>(1f, 0f) * _e315)));
            let _e319 = _e318.yx;
            c_5.z = _e319.x;
            c_5.w = _e319.y;
            let _e326 = c_5;
            let _e329 = c_5;
            cc_1 = ((vec2(2f) * _e326.xz) + _e329.yw);
            let _e334 = d;
            movec(vec2<bool>(step(vec2(0.9f), _e334.zw)), (&cc_1), vec2(0f));
            let _e343 = weights_1;
            let _e344 = d;
            let _e346 = cc_1;
            let _e347 = subsampleIndices_1;
            let _e349 = areaDiag(_e344.xy, _e346, _e347.w);
            weights_1 = (_e343 + _e349.yx);
        }
    }
    let _e352 = weights_1;
    return _e352;
}

fn searchLength(e_10: vec2<f32>, offset_2: f32) -> f32 {
    var e_11: vec2<f32>;
    var offset_3: f32;
    var scale: vec2<f32> = vec2<f32>(33f, -33f);
    var bias: vec2<f32>;

    e_11 = e_10;
    offset_3 = offset_2;
    let _e48 = offset_3;
    bias = (vec2<f32>(66f, 33f) * vec2<f32>(_e48, 1f));
    let _e53 = scale;
    scale = (_e53 + vec2<f32>(-1f, 1f));
    let _e59 = bias;
    bias = (_e59 + vec2<f32>(0.5f, -0.5f));
    let _e65 = scale;
    scale = (_e65 * vec2<f32>(0.015625f, 0.0625f));
    let _e77 = bias;
    bias = (_e77 * vec2<f32>(0.015625f, 0.0625f));
    let _e89 = scale;
    let _e90 = e_11;
    let _e92 = bias;
    let _e94 = textureSample(searchTexture_t, searchTexture_s, ((_e89 * _e90) + _e92));
    return _e94.x;
}

fn searchXLeft(texCoord_7: vec2<f32>, end_1: f32) -> f32 {
    var texCoord_8: vec2<f32>;
    var end_2: f32;
    var e_12: vec2<f32> = vec2<f32>(0f, 1f);
    var i_6: i32 = 0i;
    var offset_4: f32;

    texCoord_8 = texCoord_7;
    end_2 = end_1;
    loop {
        let _e38 = i_6;
        if !((_e38 < 8i)) {
            break;
        }
        {
            let _e45 = texCoord_8;
            let _e47 = end_2;
            let _e49 = e_12;
            let _e54 = e_12;
            if !((((_e45.x > _e47) && (_e49.y > 0.8281f)) && (_e54.x == 0f))) {
                {
                    break;
                }
            }
            let _e60 = texCoord_8;
            let _e61 = textureSample(inputBuffer_t, inputBuffer_s, _e60);
            e_12 = _e61.xy;
            let _e67 = global.texelSize;
            let _e69 = texCoord_8;
            texCoord_8 = ((vec2<f32>(-2f, 0f) * _e67) + _e69);
        }
        continuing {
            let _e42 = i_6;
            i_6 = (_e42 + 1i);
        }
    }
    let _e75 = e_12;
    let _e77 = searchLength(_e75, 0f);
    offset_4 = ((-2.007874f * _e77) + 3.25f);
    let _e82 = global.texelSize;
    let _e84 = offset_4;
    let _e86 = texCoord_8;
    return ((_e82.x * _e84) + _e86.x);
}

fn searchXRight(texCoord_9: vec2<f32>, end_3: f32) -> f32 {
    var texCoord_10: vec2<f32>;
    var end_4: f32;
    var e_13: vec2<f32> = vec2<f32>(0f, 1f);
    var i_7: i32 = 0i;
    var offset_5: f32;

    texCoord_10 = texCoord_9;
    end_4 = end_3;
    loop {
        let _e38 = i_7;
        if !((_e38 < 8i)) {
            break;
        }
        {
            let _e45 = texCoord_10;
            let _e47 = end_4;
            let _e49 = e_13;
            let _e54 = e_13;
            if !((((_e45.x < _e47) && (_e49.y > 0.8281f)) && (_e54.x == 0f))) {
                {
                    break;
                }
            }
            let _e60 = texCoord_10;
            let _e61 = textureSample(inputBuffer_t, inputBuffer_s, _e60);
            e_13 = _e61.xy;
            let _e66 = global.texelSize;
            let _e69 = texCoord_10;
            texCoord_10 = ((vec2<f32>(2f, 0f) * _e66.xy) + _e69);
        }
        continuing {
            let _e42 = i_7;
            i_7 = (_e42 + 1i);
        }
    }
    let _e75 = e_13;
    let _e77 = searchLength(_e75, 0.5f);
    offset_5 = ((-2.007874f * _e77) + 3.25f);
    let _e82 = global.texelSize;
    let _e85 = offset_5;
    let _e87 = texCoord_10;
    return ((-(_e82.x) * _e85) + _e87.x);
}

fn searchYUp(texCoord_11: vec2<f32>, end_5: f32) -> f32 {
    var texCoord_12: vec2<f32>;
    var end_6: f32;
    var e_14: vec2<f32> = vec2<f32>(1f, 0f);
    var i_8: i32 = 0i;
    var offset_6: f32;

    texCoord_12 = texCoord_11;
    end_6 = end_5;
    loop {
        let _e38 = i_8;
        if !((_e38 < 8i)) {
            break;
        }
        {
            let _e45 = texCoord_12;
            let _e47 = end_6;
            let _e49 = e_14;
            let _e54 = e_14;
            if !((((_e45.y > _e47) && (_e49.x > 0.8281f)) && (_e54.y == 0f))) {
                {
                    break;
                }
            }
            let _e60 = texCoord_12;
            let _e61 = textureSample(inputBuffer_t, inputBuffer_s, _e60);
            e_14 = _e61.xy;
            let _e70 = global.texelSize;
            let _e73 = texCoord_12;
            texCoord_12 = ((vec2<f32>(-0f, -2f) * _e70.xy) + _e73);
        }
        continuing {
            let _e42 = i_8;
            i_8 = (_e42 + 1i);
        }
    }
    let _e79 = e_14;
    let _e82 = searchLength(_e79.yx, 0f);
    offset_6 = ((-2.007874f * _e82) + 3.25f);
    let _e87 = global.texelSize;
    let _e89 = offset_6;
    let _e91 = texCoord_12;
    return ((_e87.y * _e89) + _e91.y);
}

fn searchYDown(texCoord_13: vec2<f32>, end_7: f32) -> f32 {
    var texCoord_14: vec2<f32>;
    var end_8: f32;
    var e_15: vec2<f32> = vec2<f32>(1f, 0f);
    var i_9: i32 = 0i;
    var offset_7: f32;

    texCoord_14 = texCoord_13;
    end_8 = end_7;
    loop {
        let _e38 = i_9;
        if !((_e38 < 8i)) {
            break;
        }
        {
            let _e45 = texCoord_14;
            let _e47 = end_8;
            let _e49 = e_15;
            let _e54 = e_15;
            if !((((_e45.y < _e47) && (_e49.x > 0.8281f)) && (_e54.y == 0f))) {
                {
                    break;
                }
            }
            let _e60 = texCoord_14;
            let _e61 = textureSample(inputBuffer_t, inputBuffer_s, _e60);
            e_15 = _e61.xy;
            let _e66 = global.texelSize;
            let _e69 = texCoord_14;
            texCoord_14 = ((vec2<f32>(0f, 2f) * _e66.xy) + _e69);
        }
        continuing {
            let _e42 = i_9;
            i_9 = (_e42 + 1i);
        }
    }
    let _e75 = e_15;
    let _e78 = searchLength(_e75.yx, 0.5f);
    offset_7 = ((-2.007874f * _e78) + 3.25f);
    let _e83 = global.texelSize;
    let _e86 = offset_7;
    let _e88 = texCoord_14;
    return ((-(_e83.y) * _e86) + _e88.y);
}

fn area(dist_2: vec2<f32>, e1_: f32, e2_: f32, offset_8: f32) -> vec2<f32> {
    var dist_3: vec2<f32>;
    var e1_1: f32;
    var e2_1: f32;
    var offset_9: f32;
    var texCoord_15: vec2<f32>;

    dist_3 = dist_2;
    e1_1 = e1_;
    e2_1 = e2_;
    offset_9 = offset_8;
    let _e39 = e1_1;
    let _e40 = e2_1;
    let _e45 = dist_3;
    texCoord_15 = ((vec2(16f) * round((4f * vec2<f32>(_e39, _e40)))) + _e45);
    let _e58 = texCoord_15;
    texCoord_15 = ((vec2<f32>(0.00625f, 0.0017857143f) * _e58) + vec2<f32>(0.003125f, 0.00089285715f));
    let _e80 = offset_9;
    let _e82 = texCoord_15;
    texCoord_15.y = ((0.14285715f * _e80) + _e82.y);
    let _e85 = texCoord_15;
    let _e86 = textureSample(areaTexture_t, areaTexture_s, _e85);
    return _e86.xy;
}

fn detectHorizontalCornerPattern(weights_2: ptr<function, vec2<f32>>, texCoord_16: vec4<f32>, d_1: vec2<f32>) {
    var texCoord_17: vec4<f32>;
    var d_2: vec2<f32>;

    texCoord_17 = texCoord_16;
    d_2 = d_1;
    return;
}

fn detectVerticalCornerPattern(weights_3: ptr<function, vec2<f32>>, texCoord_18: vec4<f32>, d_3: vec2<f32>) {
    var texCoord_19: vec4<f32>;
    var d_4: vec2<f32>;

    texCoord_19 = texCoord_18;
    d_4 = d_3;
    return;
}

fn main_1() {
    var weights_4: vec4<f32> = vec4(0f);
    var subsampleIndices_2: vec4<f32> = vec4(0f);
    var e_16: vec2<f32>;
    var d_5: vec2<f32>;
    var coords_2: vec3<f32>;
    var e1_2: f32;
    var sqrtD: vec2<f32>;
    var e2_2: f32;
    var local_2: vec2<f32>;
    var d_6: vec2<f32>;
    var coords_3: vec3<f32>;
    var e1_3: f32;
    var sqrtD_1: vec2<f32>;
    var e2_3: f32;
    var local_3: vec2<f32>;

    let _e34 = vUv_1;
    let _e35 = textureSample(inputBuffer_t, inputBuffer_s, _e34);
    e_16 = _e35.xy;
    let _e38 = e_16;
    if (_e38.y > 0f) {
        {
            let _e47 = vOffset_3[0];
            let _e51 = vOffset_3[2];
            let _e53 = searchXLeft(_e47.xy, _e51.x);
            coords_2.x = _e53;
            let _e57 = vOffset_3[1];
            coords_2.y = _e57.y;
            let _e60 = coords_2;
            d_5.x = _e60.x;
            let _e62 = coords_2;
            let _e64 = textureSample(inputBuffer_t, inputBuffer_s, _e62.xy);
            e1_2 = _e64.x;
            let _e70 = vOffset_3[0];
            let _e74 = vOffset_3[2];
            let _e76 = searchXRight(_e70.zw, _e74.y);
            coords_2.z = _e76;
            let _e78 = coords_2;
            d_5.y = _e78.z;
            let _e80 = global.resolution;
            let _e82 = d_5;
            let _e84 = vPixCoord_1;
            d_5 = round(((_e80.xx * _e82) + -(_e84.xx)));
            let _e89 = d_5;
            sqrtD = sqrt(abs(_e89));
            let _e93 = coords_2;
            let _e100 = global.texelSize;
            let _e103 = textureSample(inputBuffer_t, inputBuffer_s, (_e93.zy + (vec2<f32>(1f, 0f) * _e100)));
            e2_2 = _e103.x;
            let _e106 = weights_4;
            let _e108 = sqrtD;
            let _e109 = e1_2;
            let _e110 = e2_2;
            let _e111 = subsampleIndices_2;
            let _e113 = area(_e108, _e109, _e110, _e111.y);
            weights_4.x = _e113.x;
            weights_4.y = _e113.y;
            let _e119 = vUv_1;
            coords_2.y = _e119.y;
            let _e121 = weights_4;
            let _e123 = coords_2;
            let _e125 = d_5;
            let _e126 = weights_4;
            local_2 = _e126.xy;
            detectHorizontalCornerPattern((&local_2), _e123.xyzy, _e125);
            let _e133 = local_2.x;
            weights_4.x = _e133;
            let _e134 = local_2.y;
            weights_4.y = _e134;
        }
    }
    let _e135 = e_16;
    if (_e135.x > 0f) {
        {
            let _e144 = vOffset_3[1];
            let _e148 = vOffset_3[2];
            let _e150 = searchYUp(_e144.xy, _e148.z);
            coords_3.y = _e150;
            let _e154 = vOffset_3[0];
            coords_3.x = _e154.x;
            let _e157 = coords_3;
            d_6.x = _e157.y;
            let _e159 = coords_3;
            let _e161 = textureSample(inputBuffer_t, inputBuffer_s, _e159.xy);
            e1_3 = _e161.y;
            let _e167 = vOffset_3[1];
            let _e171 = vOffset_3[2];
            let _e173 = searchYDown(_e167.zw, _e171.w);
            coords_3.z = _e173;
            let _e175 = coords_3;
            d_6.y = _e175.z;
            let _e177 = global.resolution;
            let _e179 = d_6;
            let _e181 = vPixCoord_1;
            d_6 = round(((_e177.yy * _e179) - _e181.yy));
            let _e185 = d_6;
            sqrtD_1 = sqrt(abs(_e185));
            let _e189 = coords_3;
            let _e196 = global.texelSize;
            let _e199 = textureSample(inputBuffer_t, inputBuffer_s, (_e189.xz + (vec2<f32>(0f, 1f) * _e196)));
            e2_3 = _e199.y;
            let _e202 = weights_4;
            let _e204 = sqrtD_1;
            let _e205 = e1_3;
            let _e206 = e2_3;
            let _e207 = subsampleIndices_2;
            let _e209 = area(_e204, _e205, _e206, _e207.x);
            weights_4.z = _e209.x;
            weights_4.w = _e209.y;
            let _e215 = vUv_1;
            coords_3.x = _e215.x;
            let _e217 = weights_4;
            let _e219 = coords_3;
            let _e221 = d_6;
            let _e222 = weights_4;
            local_3 = _e222.zw;
            detectVerticalCornerPattern((&local_3), _e219.xyxz, _e221);
            let _e229 = local_3.x;
            weights_4.z = _e229;
            let _e230 = local_3.y;
            weights_4.w = _e230;
        }
    }
    let _e231 = weights_4;
    pc_fragColor = _e231;
    return;
}

@fragment 
fn main(@location(0) vUv: vec2<f32>, @location(1) vOffset: vec4<f32>, @location(2) vOffset_1: vec4<f32>, @location(3) vOffset_2: vec4<f32>, @location(4) vPixCoord: vec2<f32>) -> FragmentOutput {
    vUv_1 = vUv;
    vOffset_3[0] = vOffset;
    vOffset_3[1] = vOffset_1;
    vOffset_3[2] = vOffset_2;
    vPixCoord_1 = vPixCoord;
    main_1();
    let _e50 = pc_fragColor;
    return FragmentOutput(_e50);
}
