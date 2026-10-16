//! A canonical text form of a parse, for comparing the port with the
//! original loader: numbers as the hexadecimal bits of their double ( NaN as
//! `NaN` ), buffers as their FNV-1a hash or, with `full`, their values, and
//! geometries, materials, textures and skeletons numbered in first-reference
//! order. tests/browser/usd-loader.spec.js and tools/core/usd-oracle.mjs
//! build the same text from USDLoader ( tools/core/usd-canonical.mjs ).
use super::{UsdImage, UsdNodeKind, UsdScene, UsdTrackKind};
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
fn hash(bytes: &[u8], count: usize) -> String {
    let mut h: u32 = 0x811c_9dc5;
    for &b in bytes {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    format!("\"{count}:{h:08x}\"")
}
fn opt(v: Option<String>) -> String {
    v.unwrap_or_else(|| "null".into())
}

/// The canonical text of a parse.
pub fn canonical(scene: &UsdScene, full: bool) -> String {
    let buffer = |v: &[f64], bytes: Vec<u8>| {
        if full { nums(v) } else { hash(&bytes, v.len()) }
    };
    let f32s = |v: &[f32]| {
        buffer(
            &v.iter().map(|&x| f64::from(x)).collect::<Vec<_>>(),
            v.iter().flat_map(|x| x.to_le_bytes()).collect(),
        )
    };
    let mut geometries: Vec<usize> = vec![];
    let mut materials: Vec<usize> = vec![];
    let mut skeletons: Vec<usize> = vec![];
    for n in &scene.nodes {
        if let UsdNodeKind::Mesh {
            geometry,
            materials: ms,
            skin,
            ..
        } = &n.kind
        {
            if !geometries.contains(geometry) {
                geometries.push(*geometry);
            }
            for m in ms {
                if !materials.contains(m) {
                    materials.push(*m);
                }
            }
            if let Some((Some(s), _)) = skin
                && !skeletons.contains(s)
            {
                skeletons.push(*s);
            }
        }
    }
    if let Some(s) = scene.skeleton
        && !skeletons.contains(&s)
    {
        skeletons.push(s);
    }
    let mut textures: Vec<usize> = vec![];
    for &m in &materials {
        let m = &scene.materials[m];
        for t in [
            m.map,
            m.emissive_map,
            m.normal_map,
            m.roughness_map,
            m.metalness_map,
            m.ao_map,
            m.specular_color_map,
        ]
        .into_iter()
        .flatten()
        {
            if !textures.contains(&t) {
                textures.push(t);
            }
        }
    }
    let index = |list: &[usize], v: usize| list.iter().position(|&x| x == v).unwrap_or(0);
    let mut out = String::from("{\"nodes\":[");
    for (i, n) in scene.nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let (kind, extra) = match &n.kind {
            UsdNodeKind::Group => ("Group", "null".to_string()),
            UsdNodeKind::Object3D => ("Object3D", "null".into()),
            UsdNodeKind::Bone => ("Bone", "null".into()),
            UsdNodeKind::Mesh {
                geometry,
                materials: ms,
                multi,
                skin,
            } => (
                if skin.is_some() {
                    "SkinnedMesh"
                } else {
                    "Mesh"
                },
                format!(
                    "[{},[{}],{multi},{}]",
                    index(&geometries, *geometry),
                    ms.iter()
                        .map(|&m| index(&materials, m).to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    opt(skin.map(|(s, m)| format!(
                        "[{},{}]",
                        opt(s.map(|s| index(&skeletons, s).to_string())),
                        nums(&m)
                    )))
                ),
            ),
            UsdNodeKind::PerspectiveCamera {
                fov,
                aspect,
                near,
                far,
                film_gauge,
                film_offset,
                focus,
            } => (
                "PerspectiveCamera",
                nums(&[
                    *fov,
                    *aspect,
                    *near,
                    *far,
                    *film_gauge,
                    *film_offset,
                    *focus,
                ]),
            ),
            UsdNodeKind::OrthographicCamera {
                left,
                right,
                top,
                bottom,
                near,
                far,
            } => (
                "OrthographicCamera",
                nums(&[*left, *right, *top, *bottom, *near, *far]),
            ),
            UsdNodeKind::DirectionalLight { color, intensity } => (
                "DirectionalLight",
                format!("[{},{}]", nums(color), num(*intensity)),
            ),
            UsdNodeKind::PointLight { color, intensity } => (
                "PointLight",
                format!("[{},{}]", nums(color), num(*intensity)),
            ),
            UsdNodeKind::SpotLight {
                color,
                intensity,
                angle,
                penumbra,
            } => (
                "SpotLight",
                format!(
                    "[{},{}]",
                    nums(color),
                    nums(&[*intensity, *angle, *penumbra])
                ),
            ),
            UsdNodeKind::RectAreaLight {
                color,
                intensity,
                width,
                height,
            } => (
                "RectAreaLight",
                format!("[{},{}]", nums(color), nums(&[*intensity, *width, *height])),
            ),
        };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{}]",
            text(&n.name),
            text(kind),
            opt(n.parent.map(|p| p.to_string())),
            nums(&n.position),
            nums(&n.quaternion),
            nums(&n.scale),
            opt(n.pivot.map(|p| nums(&p))),
            nums(&n.matrix_world),
            extra
        ));
    }
    out.push_str("],\"geometries\":[");
    for (i, &g) in geometries.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let g = &scene.geometries[g];
        let attributes = g
            .attributes
            .iter()
            .map(|(k, a)| {
                let (kind, values) = match a {
                    Attribute::F32(a) => ("Float32Array", f32s(a.array())),
                    Attribute::U16(a) => (
                        "Uint16Array",
                        buffer(
                            &a.array().iter().map(|&v| f64::from(v)).collect::<Vec<_>>(),
                            a.array().iter().flat_map(|x| x.to_le_bytes()).collect(),
                        ),
                    ),
                    _ => ("other", "null".into()),
                };
                format!("[{},{},{},{values}]", text(k), a.item_size(), text(kind))
            })
            .collect::<Vec<_>>()
            .join(",");
        let groups = g
            .groups
            .iter()
            .map(|v| format!("[{},{},{}]", v.start, v.count, v.material_index))
            .collect::<Vec<_>>()
            .join(",");
        let index = opt(g.index.as_ref().map(|ix| {
            buffer(
                &ix.iter().map(|&v| f64::from(v)).collect::<Vec<_>>(),
                ix.iter().flat_map(|x| x.to_le_bytes()).collect(),
            )
        }));
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
        let ti = |t: Option<usize>| {
            opt(t
                .and_then(|t| textures.iter().position(|&x| x == t))
                .map(|i| i.to_string()))
        };
        let colors = format!(
            "[{},{},{}]",
            nums(&m.specular_color),
            nums(&m.attenuation_color),
            m.transparent
        );
        out.push_str(&format!(
            "[{},{},{},{},[{}]]",
            nums(&m.color),
            nums(&m.emissive),
            nums(&[
                m.emissive_intensity,
                m.roughness,
                m.metalness,
                m.ior,
                m.specular_intensity,
                m.clearcoat,
                m.clearcoat_roughness,
                m.transmission,
                m.thickness,
                m.attenuation_distance,
                m.iridescence,
                m.iridescence_ior,
                m.iridescence_thickness_range[0],
                m.iridescence_thickness_range[1],
                m.anisotropy,
                m.opacity,
                m.alpha_test,
                m.normal_scale[0],
                m.normal_scale[1],
            ]),
            colors,
            [
                m.map,
                m.emissive_map,
                m.normal_map,
                m.roughness_map,
                m.metalness_map,
                m.ao_map,
                m.specular_color_map,
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
            UsdImage::Url(u) => format!("{},null", text(u)),
            UsdImage::Asset { data, .. } => format!("null,{}", hash(data, data.len())),
        };
        out.push_str(&format!(
            "[{image},{},{},{},{},{},{},{},{},{}]",
            t.srgb,
            t.wrap_s,
            t.wrap_t,
            nums(&t.repeat),
            nums(&t.offset),
            num(t.rotation),
            t.channel,
            opt(t.scale.as_ref().map(|v| nums(v))),
            opt(t.bias.as_ref().map(|v| nums(v))),
        ));
    }
    out.push_str("],\"skeletons\":[");
    for (i, &s) in skeletons.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let s = &scene.skeletons[s];
        out.push_str(&format!(
            "[[{}],[{}]]",
            s.bones
                .iter()
                .map(|b| opt(b.map(|b| b.to_string())))
                .collect::<Vec<_>>()
                .join(","),
            s.bone_inverses
                .iter()
                .map(|m| nums(m))
                .collect::<Vec<_>>()
                .join(",")
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
                format!(
                    "[{},{},{},{}]",
                    text(&t.name),
                    text(match t.kind {
                        UsdTrackKind::Vector => "vector",
                        UsdTrackKind::Quaternion => "quaternion",
                    }),
                    f32s(&t.times),
                    f32s(&t.values)
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
        "],\"skeleton\":{}}}",
        opt(scene.skeleton.map(|s| index(&skeletons, s).to_string()))
    ));
    out
}
