//! The data textures PhysicalPathTracingMaterial samples, packed as
//! three-gpu-pathtracer 0.0.24 and three-mesh-bvh 0.9.10 pack them:
//! VertexAttributeTexture squares, the four-layer attribute array,
//! MaterialsTexture's 47 texels per material, GradientEquirectTexture and
//! EquirectHdrInfoUniform's sampling tables.
use super::scene::{Material, Side};

pub(crate) const MATERIAL_PIXELS: usize = 47;

/// The square side VertexAttributeTexture uses for `count` items.
pub(crate) fn dimension(count: usize) -> usize {
    ((count as f64).sqrt().ceil() as usize).max(1)
}

/// FloatVertexAttributeTexture.updateFrom: RGBA32F items ( a 1 in w for
/// three-component attributes ), or RG32F for two.
pub(crate) fn float_texture(data: &[f32], item_size: usize) -> (usize, usize, Vec<f32>) {
    let count = data.len() / item_size;
    let stride = if item_size == 3 { 4 } else { item_size };
    let dim = dimension(count);
    let mut out = vec![0f32; stride * dim * dim];
    for i in 0..count {
        for k in 0..item_size {
            out[stride * i + k] = data[item_size * i + k];
        }
        if item_size == 3 {
            out[stride * i + 3] = 1.;
        }
    }
    (dim, stride, out)
}

/// UIntVertexAttributeTexture.updateFrom for an index read three at a time
/// ( RGBA32UI with a 1 in w ).
pub(crate) fn index_texture(index: &[u32]) -> (usize, Vec<u32>) {
    let count = index.len() / 3;
    let dim = dimension(count);
    let mut out = vec![0u32; 4 * dim * dim];
    for i in 0..count {
        out[4 * i..4 * i + 3].copy_from_slice(&index[3 * i..3 * i + 3]);
        out[4 * i + 3] = 1;
    }
    (dim, out)
}

/// FloatAttributeTextureArray.setAttributes( [ normal, tangent, uv, color ] ):
/// each attribute's texture copied into a layer as RGBA, missing channels 0.
pub(crate) fn attribute_array(
    normals: &[f32],
    tangents: &[f32],
    uvs: &[f32],
    colors: &[f32],
) -> (usize, Vec<f32>) {
    let layers = [(normals, 3), (tangents, 4), (uvs, 2), (colors, 4)];
    let dim = dimension(normals.len() / 3);
    let length = dim * dim * 4;
    let mut out = vec![0f32; length * 4];
    for (layer, (data, item_size)) in layers.into_iter().enumerate() {
        let (_, stride, tex) = float_texture(data, item_size);
        let from_stride = if item_size == 3 { 4 } else { item_size };
        let count = tex.len() / stride;
        for i in 0..count {
            for j in 0..4 {
                out[layer * length + 4 * i + j] = if from_stride > j {
                    tex[from_stride * i + j]
                } else {
                    0.
                };
            }
        }
    }
    (dim, out)
}

/// MaterialsTexture.updateFrom. `map` is the floor's texture index ( the
/// only texture of the scene ).
pub(crate) fn materials(materials: &[Material]) -> (usize, Vec<f32>) {
    let pixels = materials.len() * MATERIAL_PIXELS;
    let dim = dimension(pixels);
    let mut a = vec![0f32; dim * dim * 4];
    let mut index = 0;
    macro_rules! put {
        ($v:expr) => {{
            let v: f64 = $v;
            a[index] = v as f32;
            index += 1;
        }};
    }
    for m in materials {
        let start = index;
        let tex = |present: bool| if present { 0. } else { -1. };
        put!(m.color[0]);
        put!(m.color[1]);
        put!(m.color[2]);
        put!(tex(m.map));
        put!(m.metalness);
        put!(-1.); // metalnessMap
        put!(m.roughness);
        put!(-1.); // roughnessMap
        put!(if m.physical { m.ior } else { 1.5 });
        put!(if m.physical { m.transmission } else { 0. });
        put!(-1.); // transmissionMap
        put!(m.emissive_intensity);
        put!(m.emissive[0]);
        put!(m.emissive[1]);
        put!(m.emissive[2]);
        put!(-1.); // emissiveMap
        put!(-1.); // normalMap
        put!(1.); // normalScale
        put!(1.);
        put!(0.); // clearcoat
        put!(-1.);
        put!(0.); // clearcoatRoughness
        put!(-1.);
        put!(-1.); // clearcoatNormalMap
        put!(1.); // clearcoatNormalScale ( Physical's, or the default )
        put!(1.);
        index += 1;
        put!(0.); // sheen
        put!(0.); // sheenColor
        put!(0.);
        put!(0.);
        put!(-1.);
        put!(if m.physical { 1. } else { 0. }); // sheenRoughness
        put!(-1.);
        put!(-1.); // iridescenceMap
        put!(-1.);
        put!(0.); // iridescence
        put!(1.3); // iridescenceIOR
        put!(100.); // iridescenceThicknessRange
        put!(400.);
        put!(1.); // specularColor
        put!(1.);
        put!(1.);
        put!(-1.);
        put!(1.); // specularIntensity
        put!(-1.);
        let thickness = if m.physical { m.thickness } else { 0. };
        // attenuationDistance: Physical's Infinity, or the default Infinity.
        let thin = thickness == 0.;
        put!(if thin { 1. } else { 0. });
        index += 1;
        put!(1.); // attenuationColor
        put!(1.);
        put!(1.);
        put!(f64::INFINITY); // attenuationDistance
        put!(-1.); // alphaMap
        put!(m.opacity);
        put!(0.); // alphaTest
        let transmission = if m.physical { m.transmission } else { 0. };
        put!(if !thin && transmission > 0. {
            0.
        } else {
            match m.side {
                Side::Front => 1.,
                Side::Double => 0.,
            }
        });
        put!(0.); // matte
        put!(1.); // castShadow
        put!(0.); // vertexColors | flatShading << 1
        put!(if m.transparent { 1. } else { 0. });
        // Texture matrices, 8 floats each: the floor map's setUvTransform
        // identity ( elements 0, 3, 6 and 1, 4, 7; element 1 is −0 ).
        if m.map {
            for (k, v) in [(0, 1f32), (1, 0.), (2, 0.), (4, -0.), (5, 1.), (6, 0.)] {
                a[index + k] = v;
            }
        }
        index += 8 * 16;
        debug_assert_eq!(index - start, MATERIAL_PIXELS * 4);
    }
    (dim, a)
}

/// GradientEquirectTexture( 512 ) between the page's colors, exponent 2:
/// RGBA32F rows from y = 0.
pub(crate) fn gradient(size: usize, top: [f64; 3], bottom: [f64; 3]) -> Vec<f32> {
    let mut data = vec![0f32; size * size * 4];
    for x in 0..size {
        for y in 0..size {
            let u = x as f64 / size as f64 - 0.5;
            let v = 1.0 - y as f64 / size as f64;
            let theta = u * 2.0 * std::f64::consts::PI;
            let phi = v * std::f64::consts::PI;
            // Vector3.setFromSpherical: y = cos( phi ).
            let dy = super::math::cos(phi);
            let t = dy * 0.5 + 0.5;
            let _ = theta;
            let w = pow(t, 2.);
            let i = (y * size + x) * 4;
            for k in 0..3 {
                data[i + k] = (bottom[k] + (top[k] - bottom[k]) * w) as f32;
            }
            data[i + 3] = 1.;
        }
    }
    data
}

fn pow(a: f64, b: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::pow(a, b);
    #[cfg(not(target_arch = "wasm32"))]
    a.powf(b)
}

/// DataUtils.fromHalfFloat.
pub(crate) fn from_half(h: u16) -> f64 {
    let s = if h & 0x8000 != 0 { -1. } else { 1. };
    let e = (h >> 10) & 0x1f;
    let m = (h & 0x3ff) as f64;
    match e {
        0 => s * m * 2f64.powi(-24),
        31 => {
            if m == 0. {
                s * f64::INFINITY
            } else {
                f64::NAN
            }
        }
        _ => s * (1. + m / 1024.) * 2f64.powi(e as i32 - 15),
    }
}

/// DataUtils.toHalfFloat: three's table conversion ( round to nearest with
/// the mantissa's dropped bits, ties up ).
pub(crate) fn to_half(v: f64) -> u16 {
    let f = (v.clamp(-65504., 65504.)) as f32;
    let bits = f.to_bits();
    let e = (bits >> 23) & 0x1ff;
    let (base, shift) = half_tables(e);
    base.wrapping_add(((bits & 0x007fffff) >> shift) as u16)
}

/// _generateTables: baseTable and shiftTable for an f32's sign + exponent.
fn half_tables(i: u32) -> (u16, u32) {
    let e = (i & 0xff) as i32 - 127;
    let sign: u16 = if i & 0x100 != 0 { 0x8000 } else { 0 };
    let (base, shift) = if e < -27 {
        (0x0000u16, 24)
    } else if e < -14 {
        ((0x0400u32 >> (-e - 14)) as u16, (-e - 1) as u32)
    } else if e <= 15 {
        (((e + 15) << 10) as u16, 13)
    } else if e < 128 {
        (0x7c00, 24)
    } else {
        (0x7c00, 13)
    };
    (base | sign, shift)
}

/// binarySearchFindClosestIndexOf.
fn closest(array: &[f32], target: f64, offset: usize, count: usize) -> usize {
    let (mut lower, mut upper) = (offset as i64, (offset + count) as i64 - 1);
    while lower < upper {
        let mid = (lower + upper) >> 1;
        if (array[mid as usize] as f64) < target {
            lower = mid + 1;
        } else {
            upper = mid;
        }
    }
    (lower - offset as i64) as usize
}

/// EquirectHdrInfoUniform.updateFrom on a half-float RGBA map: the marginal
/// ( one row of `height` ) and conditional ( width × height ) half-float
/// tables and the luminance sum.
pub(crate) fn equirect_info(
    width: usize,
    height: usize,
    data: &[u16],
) -> (Vec<u16>, Vec<u16>, f64) {
    let mut pdf_conditional = vec![0f32; width * height];
    let mut cdf_conditional = vec![0f32; width * height];
    let mut pdf_marginal = vec![0f32; height];
    let mut cdf_marginal = vec![0f32; height];
    let mut total = 0.0;
    let mut cumulative_marginal = 0.0;
    for y in 0..height {
        let mut row = 0.0;
        for x in 0..width {
            let i = y * width + x;
            let r = from_half(data[4 * i]);
            let g = from_half(data[4 * i + 1]);
            let b = from_half(data[4 * i + 2]);
            let weight = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            row += weight;
            total += weight;
            pdf_conditional[i] = weight as f32;
            cdf_conditional[i] = row as f32;
        }
        if row != 0. {
            for i in y * width..y * width + width {
                pdf_conditional[i] = (pdf_conditional[i] as f64 / row) as f32;
                cdf_conditional[i] = (cdf_conditional[i] as f64 / row) as f32;
            }
        }
        cumulative_marginal += row;
        pdf_marginal[y] = row as f32;
        cdf_marginal[y] = cumulative_marginal as f32;
    }
    if cumulative_marginal != 0. {
        for i in 0..height {
            pdf_marginal[i] = (pdf_marginal[i] as f64 / cumulative_marginal) as f32;
            cdf_marginal[i] = (cdf_marginal[i] as f64 / cumulative_marginal) as f32;
        }
    }
    let marginal = (0..height)
        .map(|i| {
            let dist = (i + 1) as f64 / height as f64;
            let row = closest(&cdf_marginal, dist, 0, height);
            to_half((row as f64 + 0.5) / height as f64)
        })
        .collect();
    let mut conditional = vec![0u16; width * height];
    for y in 0..height {
        for x in 0..width {
            let dist = (x + 1) as f64 / width as f64;
            let col = closest(&cdf_conditional, dist, y * width, width);
            conditional[y * width + x] = to_half((col as f64 + 0.5) / width as f64);
        }
    }
    (marginal, conditional, total)
}
