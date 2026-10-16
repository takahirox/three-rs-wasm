//! A canonical text form of a parse, for comparing the port with the
//! original loader bit for bit in the same JavaScript engine: numbers as the
//! hexadecimal bits of their double ( NaN as `NaN` ) and buffers as their
//! FNV-1a hash, with geometries, materials and textures numbered in
//! first-reference order.
//! tests/browser/fbx-loader.spec.js builds the same text from FBXLoader.
use super::{FbxImage, FbxMaterialKind, FbxNodeKind, FbxScene, FbxTrackKind};
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
fn opt_text(s: Option<&str>) -> String {
    s.map_or("null".into(), text)
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
fn f32_hash(v: &[f32]) -> String {
    let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
    hash(&bytes, v.len())
}

/// The canonical text of a parse.
pub fn canonical(scene: &FbxScene) -> String {
    let mut geometries: Vec<usize> = vec![];
    let mut materials: Vec<usize> = vec![];
    for n in &scene.nodes {
        let (g, ms) = match &n.kind {
            FbxNodeKind::Mesh(m) => (Some(m.geometry), m.materials.clone()),
            FbxNodeKind::Line { geometry, material } => (*geometry, vec![*material]),
            _ => (None, vec![]),
        };
        for m in ms {
            if !materials.contains(&m) {
                materials.push(m);
            }
        }
        if let Some(g) = g
            && !geometries.contains(&g)
        {
            geometries.push(g);
        }
    }
    let gi = |g: usize| geometries.iter().position(|&v| v == g).unwrap_or(0);
    let mi = |m: usize| materials.iter().position(|&v| v == m).unwrap_or(0);
    let mut textures: Vec<usize> = vec![];
    for &m in &materials {
        let m = &scene.materials[m];
        for t in [
            m.map,
            m.alpha_map,
            m.ao_map,
            m.bump_map,
            m.displacement_map,
            m.emissive_map,
            m.env_map,
            m.normal_map,
            m.specular_map,
        ]
        .into_iter()
        .flatten()
        {
            if !textures.contains(&t) {
                textures.push(t);
            }
        }
    }
    let ti = |t: Option<usize>| {
        t.and_then(|t| textures.iter().position(|&v| v == t))
            .map_or("null".into(), |i| i.to_string())
    };
    let color = |c: crate::math::Color| nums(&[c.0.x, c.0.y, c.0.z]);
    let mut out = String::from("{\"nodes\":[");
    for (i, n) in scene.nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let kind = match &n.kind {
            FbxNodeKind::Group => "Group",
            FbxNodeKind::Object3D => "Object3D",
            FbxNodeKind::Bone => "Bone",
            FbxNodeKind::Mesh(m) if m.skin.is_some() => "SkinnedMesh",
            FbxNodeKind::Mesh(_) => "Mesh",
            FbxNodeKind::Line { .. } => "Line",
            FbxNodeKind::PerspectiveCamera { .. } => "PerspectiveCamera",
            FbxNodeKind::PointLight { .. } => "PointLight",
            FbxNodeKind::DirectionalLight { .. } => "DirectionalLight",
            FbxNodeKind::SpotLight { .. } => "SpotLight",
            FbxNodeKind::AmbientLight { .. } => "AmbientLight",
        };
        let extra = match &n.kind {
            FbxNodeKind::Mesh(m) => format!(
                "[{},[{}],{},[{}],{}]",
                gi(m.geometry),
                m.materials
                    .iter()
                    .map(|&v| mi(v).to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                m.multi_material,
                m.morph_targets
                    .iter()
                    .map(|(k, v)| format!("[{},{v}]", text(k)))
                    .collect::<Vec<_>>()
                    .join(","),
                m.skin.as_ref().map_or("null".into(), |s| format!(
                    "[[{}],[{}],{}]",
                    s.bones
                        .iter()
                        .map(|b| b.map_or("null".into(), |b| b.to_string()))
                        .collect::<Vec<_>>()
                        .join(","),
                    s.bone_inverses
                        .iter()
                        .map(|m| nums(&m.to_cols_array()))
                        .collect::<Vec<_>>()
                        .join(","),
                    nums(&s.bind_matrix.to_cols_array())
                ))
            ),
            FbxNodeKind::Line { geometry, material } => format!(
                "[{},{}]",
                geometry.map_or("null".into(), |g| gi(g).to_string()),
                mi(*material)
            ),
            FbxNodeKind::PerspectiveCamera {
                fov,
                aspect,
                near,
                far,
            } => nums(&[*fov, *aspect, *near, *far]),
            FbxNodeKind::PointLight {
                color: c,
                intensity,
                distance,
                decay,
            } => format!("[{},{}]", color(*c), nums(&[*intensity, *distance, *decay])),
            FbxNodeKind::DirectionalLight {
                color: c,
                intensity,
                ..
            } => format!("[{},{}]", color(*c), num(*intensity)),
            FbxNodeKind::SpotLight {
                color: c,
                intensity,
                distance,
                angle,
                penumbra,
                decay,
                ..
            } => format!(
                "[{},{}]",
                color(*c),
                nums(&[*intensity, *distance, *angle, *penumbra, *decay])
            ),
            FbxNodeKind::AmbientLight {
                color: c,
                intensity,
            } => format!("[{},{}]", color(*c), num(*intensity)),
            _ => "null".into(),
        };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{}]",
            text(&n.name),
            text(kind),
            n.parent.map_or("null".into(), |p| p.to_string()),
            nums(&n.position.to_array()),
            nums(&n.quaternion.to_array()),
            nums(&n.scale.to_array()),
            nums(&n.matrix_world.to_cols_array()),
            n.cast_shadow,
            extra
        ));
    }
    out.push_str("],\"geometries\":[");
    for (i, &g) in geometries.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let fg = &scene.geometries[g];
        let g = &fg.geometry;
        let attributes = g
            .attributes
            .iter()
            .map(|(k, a)| {
                let (kind, h) = match a {
                    Attribute::F32(a) => ("Float32Array", f32_hash(a.array())),
                    Attribute::U16(a) => (
                        "Uint16Array",
                        hash(
                            &a.array()
                                .iter()
                                .flat_map(|x| x.to_le_bytes())
                                .collect::<Vec<_>>(),
                            a.array().len(),
                        ),
                    ),
                    _ => ("other", "null".into()),
                };
                format!("[{},{},{},{h}]", text(k), a.item_size(), text(kind))
            })
            .collect::<Vec<_>>()
            .join(",");
        let groups = g
            .groups
            .iter()
            .map(|v| format!("[{},{},{}]", v.start, v.count, v.material_index))
            .collect::<Vec<_>>()
            .join(",");
        let morph = g
            .morph_attributes
            .get("position")
            .map(|list| {
                list.iter()
                    .enumerate()
                    .map(|(k, a)| {
                        let h = match a {
                            Attribute::F32(a) => f32_hash(a.array()),
                            _ => "null".into(),
                        };
                        format!("[{},{h}]", text(fg.morph_names.get(k).map_or("", |s| s)))
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default();
        out.push_str(&format!(
            "[{},[{attributes}],[{groups}],[{morph}],{}]",
            text(&g.name),
            g.morph_targets_relative
        ));
    }
    out.push_str("],\"materials\":[");
    for (i, &m) in materials.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let m = &scene.materials[m];
        let (kind, lit, phong) = match m.kind {
            FbxMaterialKind::Phong => ("MeshPhongMaterial", true, true),
            FbxMaterialKind::Lambert => ("MeshLambertMaterial", true, false),
            FbxMaterialKind::LineBasic => ("LineBasicMaterial", false, false),
        };
        let or_null = |b: bool, v: String| if b { v } else { "null".into() };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{},{},{},{},{},[{}]]",
            text(kind),
            opt_text(m.name.as_deref()),
            color(m.color),
            or_null(phong, color(m.specular)),
            or_null(phong, num(m.shininess)),
            or_null(lit, color(m.emissive)),
            or_null(lit, num(m.emissive_intensity)),
            num(m.opacity),
            m.transparent,
            m.vertex_colors,
            or_null(lit, num(m.bump_scale)),
            or_null(lit, num(m.displacement_scale)),
            or_null(lit, num(m.reflectivity)),
            [
                m.map,
                m.alpha_map,
                m.ao_map,
                m.bump_map,
                m.displacement_map,
                m.emissive_map,
                m.env_map,
                m.normal_map,
                m.specular_map,
            ]
            .map(ti)
            .join(",")
        ));
    }
    out.push_str("],\"textures\":[");
    for (i, &t) in textures.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let t = &scene.textures[t];
        let image = match &t.image {
            FbxImage::None => "null,null".to_string(),
            FbxImage::Url(u) => format!("{},null", text(u)),
            FbxImage::Embedded { mime, data } => {
                format!("{},{}", text(mime), hash(data, data.len()))
            }
        };
        out.push_str(&format!(
            "[{},{image},{},{},{},{},{},{}]",
            opt_text(t.name.as_deref()),
            if t.repeat_s { 1000 } else { 1001 },
            if t.repeat_t { 1000 } else { 1001 },
            nums(&t.repeat.to_array()),
            nums(&t.offset.to_array()),
            t.srgb,
            t.equirectangular
        ));
    }
    out.push_str("],\"clips\":[");
    for (i, c) in scene.clips.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let tracks = c
            .tracks
            .iter()
            .map(|t| {
                let kind = match t.kind {
                    FbxTrackKind::Vector => "vector",
                    FbxTrackKind::Quaternion => "quaternion",
                    FbxTrackKind::Number => "number",
                };
                format!(
                    "[{},{},{},{}]",
                    text(&t.name),
                    text(kind),
                    f32_hash(&t.times),
                    f32_hash(&t.values)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "[{},{},[{tracks}]]",
            text(&c.name),
            num(c.duration)
        ));
    }
    out.push_str(&format!(
        "],\"unitScaleFactor\":{}}}",
        scene.unit_scale_factor.map_or("null".into(), num)
    ));
    out
}
