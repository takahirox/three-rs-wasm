//! A canonical text form of a parse, for comparing the port with the
//! original loader: numbers as the hexadecimal bits of their double ( NaN as
//! `NaN` ), buffers as their FNV-1a hash or, with `full`, their values, and
//! geometries, materials and textures numbered in first-reference order.
//! tests/browser/vrml-loader.spec.js and tools/core/vrml-oracle.mjs build
//! the same text from VRMLLoader ( tools/core/vrml-canonical.mjs ).
use super::{VrmlImage, VrmlMaterialKind, VrmlNodeKind, VrmlScene};
use crate::geometry::Attribute;

fn num(v: f64) -> String {
    if v.is_nan() {
        "\"NaN\"".into()
    } else {
        format!("\"{:016x}\"", v.to_bits())
    }
}
fn nums(v: &[f64]) -> String {
    format!(
        "[{}]",
        v.iter().map(|&v| num(v)).collect::<Vec<_>>().join(",")
    )
}
fn text(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_default()
}
/// FNV-1a ( 32-bit ) of the bytes, with the element count.
fn hash(bytes: &[u8], count: usize) -> String {
    let mut h: u32 = 0x811c_9dc5;
    for &b in bytes {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    format!("\"{count}:{h:08x}\"")
}

/// The canonical text of a parse.
pub fn canonical(scene: &VrmlScene, full: bool) -> String {
    let f32s = |v: &[f32]| {
        if full {
            nums(&v.iter().map(|&x| f64::from(x)).collect::<Vec<_>>())
        } else {
            hash(
                &v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>(),
                v.len(),
            )
        }
    };
    let mut geometries: Vec<usize> = vec![];
    let mut materials: Vec<usize> = vec![];
    for n in &scene.nodes {
        if let VrmlNodeKind::Mesh { geometry, material }
        | VrmlNodeKind::LineSegments { geometry, material }
        | VrmlNodeKind::Points { geometry, material } = n.kind
        {
            if !geometries.contains(&geometry) {
                geometries.push(geometry);
            }
            if !materials.contains(&material) {
                materials.push(material);
            }
        }
    }
    let mut textures: Vec<usize> = vec![];
    for &m in &materials {
        if let Some(t) = scene.materials[m].map
            && !textures.contains(&t)
        {
            textures.push(t);
        }
    }
    let gi = |g: usize| geometries.iter().position(|&v| v == g).unwrap_or(0);
    let mi = |m: usize| materials.iter().position(|&v| v == m).unwrap_or(0);
    let mut out = String::from("{\"nodes\":[");
    for (i, n) in scene.nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let (kind, extra) = match n.kind {
            VrmlNodeKind::Scene => ("Scene", "null".into()),
            VrmlNodeKind::Group => ("Group", "null".into()),
            VrmlNodeKind::Object3D => ("Object3D", "null".into()),
            VrmlNodeKind::Mesh { geometry, material } => {
                ("Mesh", format!("[{},{}]", gi(geometry), mi(material)))
            }
            VrmlNodeKind::LineSegments { geometry, material } => (
                "LineSegments",
                format!("[{},{}]", gi(geometry), mi(material)),
            ),
            VrmlNodeKind::Points { geometry, material } => {
                ("Points", format!("[{},{}]", gi(geometry), mi(material)))
            }
            VrmlNodeKind::PerspectiveCamera {
                fov,
                aspect,
                near,
                far,
            } => ("PerspectiveCamera", nums(&[fov, aspect, near, far])),
            VrmlNodeKind::OrthographicCamera {
                left,
                right,
                top,
                bottom,
                near,
                far,
            } => (
                "OrthographicCamera",
                nums(&[left, right, top, bottom, near, far]),
            ),
        };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{},{},{}]",
            n.id,
            text(&n.name),
            text(kind),
            n.parent.map_or("null".into(), |p| p.to_string()),
            nums(&n.position),
            nums(&n.quaternion),
            nums(&n.scale),
            num(n.render_order),
            n.visible,
            nums(&n.matrix_world),
            extra
        ));
    }
    out.push_str("],\"geometries\":[");
    for (i, &g) in geometries.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let g = &scene.geometries[g].geometry;
        let attributes = g
            .attributes
            .iter()
            .map(|(k, a)| {
                let values = match a {
                    Attribute::F32(a) => f32s(a.array()),
                    _ => "null".into(),
                };
                format!("[{},{},{values}]", text(k), a.item_size())
            })
            .collect::<Vec<_>>()
            .join(",");
        let index = g.index.as_ref().map_or("null".into(), |ix| {
            if full {
                format!(
                    "[{}]",
                    ix.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            } else {
                hash(
                    &ix.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>(),
                    ix.len(),
                )
            }
        });
        let groups = g
            .groups
            .iter()
            .map(|v| format!("[{},{},{}]", v.start, v.count, v.material_index))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "[{},[{attributes}],{index},[{groups}]]",
            text(&g.name)
        ));
    }
    out.push_str("],\"materials\":[");
    for (i, &m) in materials.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let m = &scene.materials[m];
        let (kind, phong, points) = match m.kind {
            VrmlMaterialKind::MeshBasic => ("MeshBasicMaterial", false, false),
            VrmlMaterialKind::MeshPhong => ("MeshPhongMaterial", true, false),
            VrmlMaterialKind::Points => ("PointsMaterial", false, true),
            VrmlMaterialKind::LineBasic => ("LineBasicMaterial", false, false),
        };
        let or_null = |b: bool, v: String| if b { v } else { "null".into() };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
            m.id,
            text(kind),
            text(&m.name),
            nums(&m.color),
            or_null(phong, nums(&m.emissive)),
            or_null(phong, nums(&m.specular)),
            or_null(phong, num(m.shininess)),
            num(m.opacity),
            m.transparent,
            m.side,
            m.vertex_colors,
            m.depth_write,
            m.depth_test,
            m.fog,
            or_null(points, num(m.size)),
            or_null(points, m.size_attenuation.to_string()),
            m.map
                .and_then(|t| textures.iter().position(|&v| v == t))
                .map_or("null".into(), |i| i.to_string())
        ));
    }
    out.push_str("],\"textures\":[");
    for (i, &t) in textures.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let t = &scene.textures[t];
        let image = match &t.image {
            VrmlImage::Url(u) => format!("{},null,null,null", text(u)),
            VrmlImage::Data {
                width,
                height,
                data,
            } => format!(
                "null,{},{},{}",
                hash(data, data.len()),
                num(*width),
                num(*height)
            ),
        };
        out.push_str(&format!(
            "[{},{image},{},{},{},{},{},{},{},{},{},{},{},{}]",
            text(&t.name),
            t.wrap_s,
            t.wrap_t,
            t.mag_filter,
            t.min_filter,
            t.generate_mipmaps,
            t.flip_y,
            t.srgb,
            nums(&t.offset),
            nums(&t.repeat),
            nums(&t.center),
            num(t.rotation),
            nums(&t.matrix)
        ));
    }
    out.push_str("]}");
    out
}
