//! The FBXLoader port against the pinned r186 FBXLoader: every FBX model the
//! examples load, compared node by node, buffer by buffer and keyframe by
//! keyframe with the original's parse ( tools/core/fbx-oracle.mjs ).
use serde_json::Value;
use three_rs_wasm::fbx::{self, FbxImage, FbxMaterialKind, FbxNodeKind, FbxTrackKind, Options};
use three_rs_wasm::geometry::Attribute;

const ASSETS: &[&str] = &[
    "Samba Dancing",
    "morph_test",
    "monkey",
    "monkey_embedded_texture",
    "vCube",
    "archer/ArcherRi01",
    "warrior/Warrior",
    "stanford-bunny",
    "mixamo",
    "RotationTest",
    "exampleWindow",
    "Head_69",
    "morph-translation",
    "ball_anims_asc_2018",
    "nurbs",
];

fn oracle() {
    if std::path::Path::new(".cache/fbx-oracle/nurbs.json").exists() {
        return;
    }
    let status = std::process::Command::new("node")
        .arg("tools/core/fbx-oracle.mjs")
        .status()
        .expect("node runs the FBXLoader oracle");
    assert!(status.success(), "the FBXLoader oracle failed");
}

struct Cmp<'a> {
    asset: &'a str,
    bin: Vec<u8>,
    errors: Vec<String>,
}
impl Cmp<'_> {
    fn fail(&mut self, path: &str, msg: String) {
        if self.errors.len() < 40 {
            self.errors.push(format!("{}: {path}: {msg}", self.asset));
        }
    }
    /// Natively, `Math.sin` / `Math.cos` differ from the host's in the last
    /// bit for about one input in a hundred, so transforms derived from
    /// rotations match to a relative 1e-12 ( the browser test,
    /// tests/browser/fbx-loader.spec.js, compares bit for bit ).
    fn num(&mut self, path: &str, expected: &Value, actual: f64) {
        let e = expected.as_f64();
        let ok = match e {
            Some(e) => {
                e == actual
                    || (e.is_nan() && actual.is_nan())
                    || (e - actual).abs() <= 1e-12 * e.abs().max(1.0)
            }
            None => actual.is_nan(),
        };
        if !ok {
            self.fail(path, format!("{actual:?} != {expected}"));
        }
    }
    fn nums(&mut self, path: &str, expected: &Value, actual: &[f64]) {
        let e = expected.as_array().cloned().unwrap_or_default();
        if e.len() != actual.len() {
            return self.fail(path, format!("length {} != {}", actual.len(), e.len()));
        }
        for (i, (e, a)) in e.iter().zip(actual).enumerate() {
            self.num(&format!("{path}[{i}]"), e, *a);
        }
    }
    fn blob(&self, v: &Value, size: usize) -> &[u8] {
        let offset = v["offset"].as_u64().unwrap() as usize;
        let length = v["length"].as_u64().unwrap() as usize;
        &self.bin[offset..offset + length * size]
    }
    fn f32s(&mut self, path: &str, expected: &Value, actual: &[f32]) {
        let e: Vec<f32> = self
            .blob(expected, 4)
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        if e.len() != actual.len() {
            return self.fail(path, format!("length {} != {}", actual.len(), e.len()));
        }
        let mut bad = 0;
        let mut first = None;
        // Float32 values from rotated transforms may round the other way.
        let close = |e: f32, a: f32| {
            e == a
                || (e.is_nan() && a.is_nan())
                || (e.to_bits() as i64 - a.to_bits() as i64).abs() <= 1
        };
        for (i, (e, a)) in e.iter().zip(actual).enumerate() {
            if !close(*e, *a) {
                bad += 1;
                first.get_or_insert((i, *e, *a));
            }
        }
        if let Some((i, e, a)) = first {
            self.fail(
                path,
                format!("{bad} values differ, first [{i}] {a:?} != {e:?}"),
            );
        }
    }
    fn attribute(&mut self, path: &str, expected: &Value, actual: Option<&Attribute>) {
        let Some(a) = actual else {
            return self.fail(path, "missing".into());
        };
        match (expected["type"].as_str(), a) {
            (Some("Float32Array"), Attribute::F32(a)) => self.f32s(path, expected, a.array()),
            (Some("Uint16Array"), Attribute::U16(a)) => {
                let e: Vec<u16> = self
                    .blob(expected, 2)
                    .chunks(2)
                    .map(|c| u16::from_le_bytes(c.try_into().unwrap()))
                    .collect();
                if e != a.array() {
                    self.fail(path, "values differ".into());
                }
            }
            (t, _) => self.fail(path, format!("type {t:?}")),
        }
        if expected["itemSize"].as_u64() != Some(a.item_size() as u64) {
            self.fail(path, "itemSize".into());
        }
    }
}

fn color(c: three_rs_wasm::math::Color) -> [f64; 3] {
    [c.0.x, c.0.y, c.0.z]
}

#[test]
fn fbx_loader_matches_pinned_three() {
    oracle();
    let mut errors = vec![];
    for asset in ASSETS {
        let name: String = asset
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let json: Value = serde_json::from_slice(
            &std::fs::read(format!(".cache/fbx-oracle/{name}.json")).unwrap(),
        )
        .unwrap();
        let bin = std::fs::read(format!(".cache/fbx-oracle/{name}.bin")).unwrap();
        let bytes =
            std::fs::read(format!(".cache/three-r187/examples/models/fbx/{asset}.fbx")).unwrap();
        let dir = asset.rfind('/').map_or("", |i| &asset[..=i]);
        let scene = match fbx::parse(
            &bytes,
            &Options {
                trim_animation_clips: *asset == "ball_anims_asc_2018",
                path: dir.to_string(),
                window_size: (1.0, 1.0),
            },
        ) {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("{asset}: parse failed: {e}"));
                continue;
            }
        };
        let mut c = Cmp {
            asset,
            bin,
            errors: vec![],
        };
        // The oracle numbers geometries and materials in first-reference order.
        let mut geometry_order: Vec<usize> = vec![];
        let mut material_order: Vec<usize> = vec![];
        for n in &scene.nodes {
            let (g, ms) = match &n.kind {
                FbxNodeKind::Mesh(m) => (Some(m.geometry), m.materials.clone()),
                FbxNodeKind::Line { geometry, material } => (*geometry, vec![*material]),
                _ => (None, vec![]),
            };
            for m in ms {
                if !material_order.contains(&m) {
                    material_order.push(m);
                }
            }
            if let Some(g) = g
                && !geometry_order.contains(&g)
            {
                geometry_order.push(g);
            }
        }
        let gi = |g: usize| geometry_order.iter().position(|&v| v == g).unwrap() as u64;
        let mi = |m: usize| material_order.iter().position(|&v| v == m).unwrap() as u64;
        let nodes = json["nodes"].as_array().unwrap();
        if nodes.len() != scene.nodes.len() {
            c.fail("nodes", format!("{} != {}", scene.nodes.len(), nodes.len()));
        }
        for (i, (e, a)) in nodes.iter().zip(&scene.nodes).enumerate() {
            let p = format!("nodes[{i}] {}", a.name);
            if e["name"].as_str() != Some(&a.name) {
                c.fail(&p, format!("name {:?} != {}", a.name, e["name"]));
            }
            let kind = match &a.kind {
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
            if e["type"].as_str() != Some(kind) {
                c.fail(&p, format!("type {kind} != {}", e["type"]));
            }
            if e["parent"].as_u64().map(|v| v as usize) != a.parent {
                c.fail(&p, format!("parent {:?} != {}", a.parent, e["parent"]));
            }
            c.nums(
                &format!("{p}.position"),
                &e["position"],
                &a.position.to_array(),
            );
            c.nums(
                &format!("{p}.quaternion"),
                &e["quaternion"],
                &a.quaternion.to_array(),
            );
            c.nums(&format!("{p}.scale"), &e["scale"], &a.scale.to_array());
            c.nums(
                &format!("{p}.matrixWorld"),
                &e["matrixWorld"],
                &a.matrix_world.to_cols_array(),
            );
            if e["castShadow"].as_bool() != Some(a.cast_shadow) {
                c.fail(&p, "castShadow".into());
            }
            match &a.kind {
                FbxNodeKind::Mesh(m) => {
                    let em = &e["mesh"];
                    if em["geometry"].as_u64() != Some(gi(m.geometry)) {
                        c.fail(&p, "geometry index".into());
                    }
                    let mats: Vec<u64> = m.materials.iter().map(|&v| mi(v)).collect();
                    let emats: Vec<u64> = em["materials"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(Value::as_u64)
                        .collect();
                    if mats != emats {
                        c.fail(&p, format!("materials {mats:?} != {emats:?}"));
                    }
                    if em["multi"].as_bool() != Some(m.multi_material) {
                        c.fail(&p, "multiMaterial".into());
                    }
                    let dict: Vec<(String, u64)> = em["morphTargetDictionary"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| (p[0].as_str().unwrap().to_string(), p[1].as_u64().unwrap()))
                        .collect();
                    let actual: Vec<(String, u64)> = m
                        .morph_targets
                        .iter()
                        .map(|(n, i)| (n.clone(), *i as u64))
                        .collect();
                    if dict != actual {
                        c.fail(&p, format!("morphTargetDictionary {actual:?} != {dict:?}"));
                    }
                    match (&m.skin, em["skin"].is_object()) {
                        (Some(s), true) => {
                            let bones: Vec<Option<u64>> = em["skin"]["bones"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(Value::as_u64)
                                .collect();
                            let actual: Vec<Option<u64>> =
                                s.bones.iter().map(|b| b.map(|v| v as u64)).collect();
                            if bones != actual {
                                c.fail(&p, "skin bones".into());
                            }
                            for (k, (ei, ai)) in em["skin"]["boneInverses"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .zip(&s.bone_inverses)
                                .enumerate()
                            {
                                c.nums(&format!("{p}.boneInverses[{k}]"), ei, &ai.to_cols_array());
                            }
                            c.nums(
                                &format!("{p}.bindMatrix"),
                                &em["skin"]["bindMatrix"],
                                &s.bind_matrix.to_cols_array(),
                            );
                        }
                        (None, false) => {}
                        _ => c.fail(&p, "skin presence".into()),
                    }
                }
                FbxNodeKind::Line { geometry, material } => {
                    if e["line"]["geometry"].as_u64() != geometry.map(gi)
                        || e["line"]["material"].as_u64() != Some(mi(*material))
                    {
                        c.fail(&p, "line".into());
                    }
                }
                FbxNodeKind::AmbientLight {
                    color: col,
                    intensity,
                } => {
                    c.nums(&format!("{p}.color"), &e["light"]["color"], &color(*col));
                    c.num(
                        &format!("{p}.intensity"),
                        &e["light"]["intensity"],
                        *intensity,
                    );
                }
                FbxNodeKind::PointLight {
                    color: col,
                    intensity,
                    distance,
                    decay,
                } => {
                    c.nums(&format!("{p}.color"), &e["light"]["color"], &color(*col));
                    c.num(
                        &format!("{p}.intensity"),
                        &e["light"]["intensity"],
                        *intensity,
                    );
                    c.num(&format!("{p}.distance"), &e["light"]["distance"], *distance);
                    c.num(&format!("{p}.decay"), &e["light"]["decay"], *decay);
                }
                FbxNodeKind::PerspectiveCamera {
                    fov,
                    aspect,
                    near,
                    far,
                } => {
                    let cam = &e["camera"];
                    c.num(&format!("{p}.fov"), &cam["fov"], *fov);
                    c.num(&format!("{p}.aspect"), &cam["aspect"], *aspect);
                    c.num(&format!("{p}.near"), &cam["near"], *near);
                    c.num(&format!("{p}.far"), &cam["far"], *far);
                }
                _ => {}
            }
        }
        let geometries = json["geometries"].as_array().unwrap();
        if geometries.len() != geometry_order.len() {
            c.fail(
                "geometries",
                format!("{} != {}", geometry_order.len(), geometries.len()),
            );
        }
        for (i, (e, &ai)) in geometries.iter().zip(&geometry_order).enumerate() {
            let a = &scene.geometries[ai];
            let p = format!("geometries[{i}]");
            let g = &a.geometry;
            if e["name"].as_str() != Some(&g.name) {
                c.fail(&p, format!("name {:?} != {}", g.name, e["name"]));
            }
            let names: Vec<&String> = e["attributes"].as_object().unwrap().keys().collect();
            let actual: Vec<&String> = g.attributes.keys().collect();
            let mut sorted = names.clone();
            sorted.sort();
            if sorted != actual {
                c.fail(&p, format!("attributes {actual:?} != {names:?}"));
            }
            for (k, ea) in e["attributes"].as_object().unwrap() {
                c.attribute(&format!("{p}.{k}"), ea, g.attributes.get(k));
            }
            let groups: Vec<[u64; 3]> = e["groups"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| [0, 1, 2].map(|k| v[k].as_u64().unwrap_or(u64::MAX)))
                .collect();
            let actual: Vec<[u64; 3]> = g
                .groups
                .iter()
                .map(|v| [v.start as u64, v.count as u64, v.material_index as u64])
                .collect();
            if groups != actual {
                c.fail(&p, format!("groups {actual:?} != {groups:?}"));
            }
            let morph = e["morph"].as_array().unwrap();
            let actual = g
                .morph_attributes
                .get("position")
                .cloned()
                .unwrap_or_default();
            if morph.len() != actual.len() {
                c.fail(
                    &p,
                    format!("morph count {} != {}", actual.len(), morph.len()),
                );
            }
            for (k, (em, am)) in morph.iter().zip(&actual).enumerate() {
                if em["name"].as_str() != a.morph_names.get(k).map(String::as_str) {
                    c.fail(
                        &p,
                        format!("morph name {:?} != {}", a.morph_names.get(k), em["name"]),
                    );
                }
                if let Attribute::F32(am) = am {
                    c.f32s(&format!("{p}.morph[{k}]"), em, am.array());
                }
            }
            if e["morphTargetsRelative"].as_bool() != Some(g.morph_targets_relative) {
                c.fail(&p, "morphTargetsRelative".into());
            }
        }
        let materials = json["materials"].as_array().unwrap();
        let order = material_order.clone();
        if order.len() != materials.len() {
            c.fail(
                "materials",
                format!("{} != {}", order.len(), materials.len()),
            );
        }
        let mut texture_order = vec![];
        for (i, (e, &mi)) in materials.iter().zip(&order).enumerate() {
            let p = format!("materials[{i}]");
            let m = &scene.materials[mi];
            let kind = match m.kind {
                FbxMaterialKind::Phong => "MeshPhongMaterial",
                FbxMaterialKind::Lambert => "MeshLambertMaterial",
                FbxMaterialKind::LineBasic => "LineBasicMaterial",
            };
            if e["type"].as_str() != Some(kind) {
                c.fail(&p, format!("type {kind} != {}", e["type"]));
            }
            if e["name"].as_str() != m.name.as_deref() {
                c.fail(&p, format!("name {:?} != {}", m.name, e["name"]));
            }
            c.nums(&format!("{p}.color"), &e["color"], &color(m.color));
            if m.kind == FbxMaterialKind::Phong {
                c.nums(&format!("{p}.specular"), &e["specular"], &color(m.specular));
                c.num(&format!("{p}.shininess"), &e["shininess"], m.shininess);
            }
            if m.kind != FbxMaterialKind::LineBasic {
                c.nums(&format!("{p}.emissive"), &e["emissive"], &color(m.emissive));
                c.num(
                    &format!("{p}.emissiveIntensity"),
                    &e["emissiveIntensity"],
                    m.emissive_intensity,
                );
                c.num(&format!("{p}.bumpScale"), &e["bumpScale"], m.bump_scale);
                c.num(
                    &format!("{p}.displacementScale"),
                    &e["displacementScale"],
                    m.displacement_scale,
                );
                c.num(
                    &format!("{p}.reflectivity"),
                    &e["reflectivity"],
                    m.reflectivity,
                );
            }
            c.num(&format!("{p}.opacity"), &e["opacity"], m.opacity);
            if e["transparent"].as_bool() != Some(m.transparent) {
                c.fail(&p, "transparent".into());
            }
            if e["vertexColors"].as_bool() != Some(m.vertex_colors) {
                c.fail(&p, "vertexColors".into());
            }
            for (key, t) in [
                ("map", m.map),
                ("alphaMap", m.alpha_map),
                ("aoMap", m.ao_map),
                ("bumpMap", m.bump_map),
                ("displacementMap", m.displacement_map),
                ("emissiveMap", m.emissive_map),
                ("envMap", m.env_map),
                ("normalMap", m.normal_map),
                ("specularMap", m.specular_map),
            ] {
                let expected = e["maps"][key].as_u64();
                if expected.is_some() != t.is_some() {
                    c.fail(&p, format!("{key} presence"));
                    continue;
                }
                if let (Some(ei), Some(t)) = (expected, t) {
                    if !texture_order.contains(&t) {
                        texture_order.push(t);
                    }
                    let ai = texture_order.iter().position(|&v| v == t).unwrap() as u64;
                    if ai != ei {
                        c.fail(&p, format!("{key} texture index"));
                    }
                }
            }
        }
        let textures = json["textures"].as_array().unwrap();
        for (i, (e, &ti)) in textures.iter().zip(&texture_order).enumerate() {
            let p = format!("textures[{i}]");
            let t = &scene.textures[ti];
            if e["name"].as_str() != t.name.as_deref() {
                c.fail(&p, format!("name {:?} != {}", t.name, e["name"]));
            }
            match &t.image {
                FbxImage::None => {
                    if !e["url"].is_null() || e.get("blob").is_some() {
                        c.fail(&p, "image".into());
                    }
                }
                FbxImage::Url(u) => {
                    if e["url"].as_str() != Some(u) {
                        c.fail(&p, format!("url {u:?} != {}", e["url"]));
                    }
                }
                FbxImage::Embedded { mime, data } => {
                    if e["mime"].as_str() != Some(mime) || c.blob(&e["blob"], 1) != data.as_slice()
                    {
                        c.fail(&p, "embedded image".into());
                    }
                }
            }
            let wrap = |r: bool| if r { 1000 } else { 1001 };
            if e["wrapS"].as_u64() != Some(wrap(t.repeat_s))
                || e["wrapT"].as_u64() != Some(wrap(t.repeat_t))
            {
                c.fail(&p, "wrap".into());
            }
            c.nums(&format!("{p}.repeat"), &e["repeat"], &t.repeat.to_array());
            c.nums(&format!("{p}.offset"), &e["offset"], &t.offset.to_array());
            let srgb = e["colorSpace"].as_str() == Some("srgb");
            if srgb != t.srgb {
                c.fail(&p, "colorSpace".into());
            }
            if (e["mapping"].as_u64() == Some(303)) != t.equirectangular {
                c.fail(&p, "mapping".into());
            }
        }
        let clips = json["clips"].as_array().unwrap();
        if clips.len() != scene.clips.len() {
            c.fail("clips", format!("{} != {}", scene.clips.len(), clips.len()));
        }
        for (i, (e, a)) in clips.iter().zip(&scene.clips).enumerate() {
            let p = format!("clips[{i}] {}", a.name);
            if e["name"].as_str() != Some(&a.name) {
                c.fail(&p, format!("name {:?} != {}", a.name, e["name"]));
            }
            c.num(&format!("{p}.duration"), &e["duration"], a.duration);
            let tracks = e["tracks"].as_array().unwrap();
            if tracks.len() != a.tracks.len() {
                c.fail(&p, format!("tracks {} != {}", a.tracks.len(), tracks.len()));
            }
            for (k, (et, at)) in tracks.iter().zip(&a.tracks).enumerate() {
                let tp = format!("{p}.tracks[{k}] {}", at.name);
                if et["name"].as_str() != Some(&at.name) {
                    c.fail(&tp, format!("name != {}", et["name"]));
                }
                let kind = match at.kind {
                    FbxTrackKind::Vector => "vector",
                    FbxTrackKind::Quaternion => "quaternion",
                    FbxTrackKind::Number => "number",
                };
                if et["type"].as_str() != Some(kind) {
                    c.fail(&tp, format!("type {kind} != {}", et["type"]));
                }
                c.f32s(&format!("{tp}.times"), &et["times"], &at.times);
                c.f32s(&format!("{tp}.values"), &et["values"], &at.values);
            }
        }
        c.num(
            "unitScaleFactor",
            &json["unitScaleFactor"],
            scene.unit_scale_factor.unwrap_or(f64::NAN),
        );
        errors.extend(c.errors);
    }
    assert!(
        errors.is_empty(),
        "{} differences:\n{}",
        errors.len(),
        errors.join("\n")
    );
}
