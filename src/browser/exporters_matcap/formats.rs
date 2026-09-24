//! STLExporter, PLYExporter and OBJExporter, with JavaScript's number formatting
//! and Three.js's Matrix3 arithmetic so the text output matches byte for byte.

/// `Number.prototype.toString()`: the fewest significant digits that round-trip,
/// the closest (ties to even) among them, in decimal form from 1e-6 to 1e21.
pub(super) fn js_number(v: f64) -> String {
    if v == 0. {
        return "0".into();
    }
    if v.is_nan() {
        return "NaN".into();
    }
    if v.is_infinite() {
        return if v > 0. { "Infinity" } else { "-Infinity" }.into();
    }
    let split = |e: String| {
        let (mantissa, exponent) = e.split_once('e').expect("exponent form");
        let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
        (digits, exponent.parse::<i32>().expect("exponent"))
    };
    let a = v.abs();
    let (mut digits, mut exponent) = split(format!("{a:e}"));
    // Seventeen digits may tie between two candidates; JS takes the even one,
    // which Rust's exact formatting also chooses.
    if digits.len() == 17 {
        (digits, exponent) = split(format!("{a:.16e}"));
        digits = digits.trim_end_matches('0').to_string();
    }
    let k = digits.len() as i32;
    let n = exponent + 1;
    let body = if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let e = n - 1;
        let sign = if e < 0 { '-' } else { '+' };
        if k == 1 {
            format!("{digits}e{sign}{}", e.abs())
        } else {
            format!("{}.{}e{sign}{}", &digits[..1], &digits[1..], e.abs())
        }
    };
    if v < 0. { format!("-{body}") } else { body }
}
/// An object to export: its world matrix (column-major, as Three's elements),
/// Float32 positions and optional attributes.
pub(super) struct Item<'a> {
    pub world: [f64; 16],
    pub positions: &'a [f32],
    pub normals: Option<&'a [f32]>,
    pub uvs: Option<&'a [f32]>,
    /// Normalized Uint8 colors (PLY) or Float32 colors (OBJ points).
    pub colors: Option<Colors<'a>>,
    pub index: Option<&'a [u32]>,
    pub name: &'a str,
    pub points: bool,
}
#[derive(Clone, Copy)]
pub(super) enum Colors<'a> {
    Unorm8(&'a [u8]),
    F32(&'a [f32]),
}
impl Colors<'_> {
    fn get(&self, i: usize) -> f64 {
        match self {
            Colors::Unorm8(c) => c[i] as f64 / 255.,
            Colors::F32(c) => c[i] as f64,
        }
    }
}
/// Vector3.applyMatrix4.
fn apply4(e: &[f64; 16], v: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = v;
    let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
    [
        (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
        (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
        (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
    ]
}
/// Matrix3.getNormalMatrix: setFromMatrix4, invert, transpose.
fn normal_matrix(m: &[f64; 16]) -> [f64; 9] {
    let (n11, n21, n31) = (m[0], m[1], m[2]);
    let (n12, n22, n32) = (m[4], m[5], m[6]);
    let (n13, n23, n33) = (m[8], m[9], m[10]);
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    if det == 0. {
        return [0.; 9];
    }
    let d = 1. / det;
    let inv = [
        t11 * d,
        (n31 * n23 - n33 * n21) * d,
        (n32 * n21 - n31 * n22) * d,
        t12 * d,
        (n33 * n11 - n31 * n13) * d,
        (n31 * n12 - n32 * n11) * d,
        t13 * d,
        (n21 * n13 - n23 * n11) * d,
        (n22 * n11 - n21 * n12) * d,
    ];
    [
        inv[0], inv[3], inv[6], inv[1], inv[4], inv[7], inv[2], inv[5], inv[8],
    ]
}
/// Vector3.applyMatrix3 then normalize (divideScalar( length || 1 )).
fn apply3_normalize(e: &[f64; 9], v: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = v;
    let r = [
        e[0] * x + e[3] * y + e[6] * z,
        e[1] * x + e[4] * y + e[7] * z,
        e[2] * x + e[5] * y + e[8] * z,
    ];
    normalize(r)
}
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let s = 1. / if length == 0. { 1. } else { length };
    v.map(|c| c * s)
}
fn vertex(a: &[f32], i: usize) -> [f64; 3] {
    [a[i * 3] as f64, a[i * 3 + 1] as f64, a[i * 3 + 2] as f64]
}
/// ColorManagement.workingToColorSpace( SRGBColorSpace ).
fn to_srgb(c: f64) -> f64 {
    if c < 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(0.41666) - 0.055
    }
}
fn triangles(item: &Item) -> Vec<[usize; 3]> {
    match item.index {
        Some(index) => index
            .chunks(3)
            .map(|t| [t[0] as usize, t[1] as usize, t[2] as usize])
            .collect(),
        None => (0..item.positions.len() / 9)
            .map(|f| [f * 3, f * 3 + 1, f * 3 + 2])
            .collect(),
    }
}

/// STLExporter.parse( object, { binary } ).
pub(super) fn stl(items: &[Item], binary: bool) -> Vec<u8> {
    let faces: Vec<_> = items.iter().map(triangles).collect();
    let count: usize = faces.iter().map(Vec::len).sum();
    let mut text = String::from("solid exported\n");
    let mut out = vec![0u8; 80];
    out.extend((count as u32).to_le_bytes());
    for (item, faces) in items.iter().zip(&faces) {
        for &[a, b, c] in faces {
            let [va, vb, vc] = [a, b, c].map(|i| apply4(&item.world, vertex(item.positions, i)));
            let cb = [vc[0] - vb[0], vc[1] - vb[1], vc[2] - vb[2]];
            let ab = [va[0] - vb[0], va[1] - vb[1], va[2] - vb[2]];
            let cross = [
                cb[1] * ab[2] - cb[2] * ab[1],
                cb[2] * ab[0] - cb[0] * ab[2],
                cb[0] * ab[1] - cb[1] * ab[0],
            ];
            let n = normalize(normalize(cross));
            if binary {
                for v in [n, va, vb, vc] {
                    for c in v {
                        out.extend((c as f32).to_le_bytes());
                    }
                }
                out.extend(0u16.to_le_bytes());
            } else {
                let f = |v: [f64; 3]| v.map(js_number).join(" ");
                text += &format!("\tfacet normal {}\n\t\touter loop\n", f(n));
                for v in [va, vb, vc] {
                    text += &format!("\t\t\tvertex {}\n", f(v));
                }
                text += "\t\tendloop\n\tendfacet\n";
            }
        }
    }
    if binary {
        out
    } else {
        text += "endsolid exported\n";
        text.into_bytes()
    }
}

/// PLYExporter.parse( object, onDone, { binary, littleEndian } ) for Float32
/// positions, normals and uvs and normalized Uint8 colors.
pub(super) fn ply(items: &[Item], binary: bool, little: bool) -> Vec<u8> {
    let vertex_count: usize = items.iter().map(|i| i.positions.len() / 3).sum();
    let face_count: usize = items.iter().map(|i| triangles(i).len()).sum();
    let normals = items.iter().any(|i| i.normals.is_some());
    let uvs = items.iter().any(|i| i.uvs.is_some());
    let colors = items.iter().any(|i| i.colors.is_some());
    let format = if binary {
        if little {
            "binary_little_endian"
        } else {
            "binary_big_endian"
        }
    } else {
        "ascii"
    };
    let mut header = format!(
        "ply\nformat {format} 1.0\nelement vertex {vertex_count}\nproperty float x\nproperty float y\nproperty float z\n"
    );
    if normals {
        header += "property float nx\nproperty float ny\nproperty float nz\n";
    }
    if uvs {
        header += "property float s\nproperty float t\n";
    }
    if colors {
        header += "property uchar red\nproperty uchar green\nproperty uchar blue\n";
    }
    header +=
        &format!("element face {face_count}\nproperty list uchar int vertex_index\nend_header\n");
    let (mut vertices, mut faces) = (vec![], vec![]);
    let (mut vertex_text, mut face_text) = (String::new(), String::new());
    let float = |out: &mut Vec<u8>, v: f64| {
        let v = v as f32;
        out.extend(if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        });
    };
    let mut written = 0;
    for item in items {
        let nm = normal_matrix(&item.world);
        let count = item.positions.len() / 3;
        for i in 0..count {
            let p = apply4(&item.world, vertex(item.positions, i));
            let mut line = p.map(js_number).join(" ");
            p.iter().for_each(|&c| float(&mut vertices, c));
            if normals {
                let n = item
                    .normals
                    .map(|n| apply3_normalize(&nm, vertex(n, i)))
                    .unwrap_or([0.; 3]);
                line += &format!(" {}", n.map(js_number).join(" "));
                n.iter().for_each(|&c| float(&mut vertices, c));
            }
            if uvs {
                let uv = item
                    .uvs
                    .map(|u| [u[i * 2] as f64, u[i * 2 + 1] as f64])
                    .unwrap_or([0.; 2]);
                line += &format!(" {}", uv.map(js_number).join(" "));
                uv.iter().for_each(|&c| float(&mut vertices, c));
            }
            if colors {
                let c = match item.colors {
                    Some(c) => [0, 1, 2].map(|k| (to_srgb(c.get(i * 3 + k)) * 255.).round() as u8),
                    None => [255; 3],
                };
                line += &format!(" {} {} {}", c[0], c[1], c[2]);
                vertices.extend(c);
            }
            vertex_text += &line;
            vertex_text.push('\n');
        }
        for f in triangles(item) {
            let f = f.map(|k| (k + written) as u32);
            face_text += &format!("3 {} {} {}\n", f[0], f[1], f[2]);
            faces.push(3u8);
            for k in f {
                faces.extend(if little {
                    k.to_le_bytes()
                } else {
                    k.to_be_bytes()
                });
            }
        }
        written += count;
    }
    if binary {
        let mut out = header.into_bytes();
        out.extend(vertices);
        out.extend(faces);
        out
    } else {
        format!("{header}{vertex_text}{face_text}\n").into_bytes()
    }
}

/// OBJExporter.parse( scene ) for meshes and points.
pub(super) fn obj(items: &[Item]) -> Vec<u8> {
    let mut out = String::new();
    let (mut index_vertex, mut index_uvs, mut index_normals) = (0, 0, 0);
    for item in items {
        let count = item.positions.len() / 3;
        out += &format!("o {}\n", item.name);
        for i in 0..count {
            let p = apply4(&item.world, vertex(item.positions, i));
            out += &format!("v {}", p.map(js_number).join(" "));
            if item.points
                && let Some(c) = item.colors
            {
                let rgb = [0, 1, 2].map(|k| to_srgb(c.get(i * 3 + k)));
                out += &format!(" {}", rgb.map(js_number).join(" "));
            }
            out.push('\n');
        }
        if item.points {
            out += "p ";
            for j in 1..=count {
                out += &format!("{} ", index_vertex + j);
            }
            out.push('\n');
            index_vertex += count;
            continue;
        }
        let uv_count = item.uvs.map_or(0, |u| u.len() / 2);
        if let Some(uvs) = item.uvs {
            for i in 0..uv_count {
                out += &format!(
                    "vt {} {}\n",
                    js_number(uvs[i * 2] as f64),
                    js_number(uvs[i * 2 + 1] as f64)
                );
            }
        }
        let normal_count = item.normals.map_or(0, |n| n.len() / 3);
        if let Some(normals) = item.normals {
            let nm = normal_matrix(&item.world);
            for i in 0..normal_count {
                let n = apply3_normalize(&nm, vertex(normals, i));
                out += &format!("vn {}\n", n.map(js_number).join(" "));
            }
        }
        for f in triangles(item) {
            let face = f.map(|k| {
                let j = k + 1;
                let mut s = (index_vertex + j).to_string();
                if item.normals.is_some() || item.uvs.is_some() {
                    s.push('/');
                    if item.uvs.is_some() {
                        s += &(index_uvs + j).to_string();
                    }
                    if item.normals.is_some() {
                        s += &format!("/{}", index_normals + j);
                    }
                }
                s
            });
            out += &format!("f {}\n", face.join(" "));
        }
        index_vertex += count;
        index_uvs += uv_count;
        index_normals += normal_count;
    }
    out.into_bytes()
}
