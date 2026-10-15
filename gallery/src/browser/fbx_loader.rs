//! webgl_loader_fbx: the page's 14 FBX models, each the pinned FBXLoader's
//! parse baked by tools/tsl/prepare-fbx-loader.mjs ( node tree, meshes with
//! their material groups, morph targets, Phong and Lambert materials with
//! their textures, skinned meshes' bones and bind matrices, the ambient
//! light one model carries, and the keyframe clips ). The selected model is
//! scaled as the page scales it, its first clip plays on an AnimationMixer
//! by the Timer, and it casts and receives the SunLight's cascaded shadows
//! over the Phong ground and translucent grid in linear fog, with damped
//! OrbitControls ( no zoom or pan ) and a hemisphere light. A model loads
//! when chosen, as the page loads it. The page's GUI builds a clip list and
//! per-mesh morph sliders for each model; the gallery's fixed controls give
//! the clip by index and one morph influence by mesh and target index.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_scenes::grid_helper;
use crate::animation::{AnimationMixer, Clip, Interpolation, Property, Track};
use crate::attribute::BufferAttribute;
use crate::deformation::Skin;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/fbx-loader";
const COUNT: usize = 14;
/// The page's scales map: Warrior, Archer and Head_69 × 100, the bunny ×
/// 0.001, the ball × 50.
fn scale(asset: usize) -> f64 {
    match asset {
        5 | 6 | 11 => 100.,
        7 => 0.001,
        13 => 50.,
        _ => 1.,
    }
}

type Json = serde_json::Value;
fn numbers(v: &Json) -> Vec<f64> {
    v.as_array()
        .map(|a| a.iter().filter_map(Json::as_f64).collect())
        .unwrap_or_default()
}
fn color(v: &Json) -> Color {
    let n = numbers(v);
    Color::linear(
        n.first().copied().unwrap_or(1.),
        n.get(1).copied().unwrap_or(1.),
        n.get(2).copied().unwrap_or(1.),
    )
}
fn matrix(v: &Json) -> Matrix4 {
    let n = numbers(v);
    Matrix4::from_cols_array(&std::array::from_fn(|i| n.get(i).copied().unwrap_or(0.)))
}
fn bytes<'a>(bin: &'a [u8], v: &Json, size: usize) -> Result<&'a [u8]> {
    let offset = v["offset"].as_u64().ok_or(Error::Invalid("fbx offset"))? as usize;
    let length = v["length"].as_u64().ok_or(Error::Invalid("fbx length"))? as usize;
    bin.get(offset..offset + length * size)
        .ok_or(Error::Invalid("fbx range"))
}
fn f32s(bin: &[u8], v: &Json) -> Result<Vec<f32>> {
    Ok(bytes(bin, v, 4)?
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&c| f32::from_le_bytes(c))
        .collect())
}
fn attribute(bin: &[u8], v: &Json) -> Result<Attribute> {
    let size = v["itemSize"].as_u64().unwrap_or(3) as usize;
    let normalized = v["normalized"].as_bool().unwrap_or(false);
    Ok(match v["type"].as_str() {
        Some("Uint16Array") => {
            let data = bytes(bin, v, 2)?
                .as_chunks::<2>()
                .0
                .iter()
                .map(|&c| u16::from_le_bytes(c))
                .collect();
            Attribute::U16(BufferAttribute::new(data, size, normalized)?)
        }
        _ => Attribute::F32(BufferAttribute::new(f32s(bin, v)?, size, normalized)?),
    })
}

/// A loaded model: its root, the clips, and the morph meshes.
struct Model {
    root: Object3D,
    clips: Vec<Arc<Clip>>,
    morphs: Vec<(Object3D, usize)>,
}
struct Loaded {
    asset: usize,
    json: Json,
    bin: Vec<u8>,
    images: Vec<(String, Texture)>,
}

async fn load(asset: usize) -> Result<Loaded> {
    let json: Json = serde_json::from_slice(&fetch(&format!("{ASSETS}/{asset}.json")).await?)
        .map_err(|e| Error::Asset(e.to_string()))?;
    let bin = fetch(&format!("{ASSETS}/{}", json["file"].as_str().unwrap_or(""))).await?;
    let mut images: Vec<(String, Texture)> = vec![];
    for mesh in json["meshes"].as_array().into_iter().flatten() {
        for m in mesh["materials"].as_array().into_iter().flatten() {
            for key in [
                "map",
                "alphaMap",
                "normalMap",
                "bumpMap",
                "specularMap",
                "emissiveMap",
            ] {
                if let Some(name) = m[key]["image"].as_str()
                    && !images.iter().any(|(n, _)| n == name)
                {
                    let t =
                        decode_texture_image(&fetch(&format!("{ASSETS}/{name}")).await?).await?;
                    images.push((name.to_string(), t));
                }
            }
        }
    }
    Ok(Loaded {
        asset,
        json,
        bin,
        images,
    })
}

fn texture(images: &[(String, Texture)], v: &Json, srgb: bool) -> Option<Arc<Texture>> {
    let name = v["image"].as_str()?;
    let (_, image) = images.iter().find(|(n, _)| n == name)?;
    let mut t = image.clone();
    t.srgb = srgb && v["colorSpace"].as_str() == Some("srgb");
    let wrap = |w: &Json| match w.as_u64() {
        Some(1000) => Wrapping::Repeat,
        Some(1002) => Wrapping::Mirror,
        _ => Wrapping::Clamp,
    };
    t.wrap_s = wrap(&v["wrapS"]);
    t.wrap_t = wrap(&v["wrapT"]);
    let n = numbers(&v["repeat"]);
    t.repeat = Vector2::new(
        n.first().copied().unwrap_or(1.),
        n.get(1).copied().unwrap_or(1.),
    );
    let n = numbers(&v["offset"]);
    t.offset = Vector2::new(
        n.first().copied().unwrap_or(0.),
        n.get(1).copied().unwrap_or(0.),
    );
    t.rotation = v["rotation"].as_f64().unwrap_or(0.);
    t.flip_y = v["flipY"].as_bool().unwrap_or(true);
    t.mipmap_filter = Some(Filter::Linear);
    Some(Arc::new(t))
}

fn material(images: &[(String, Texture)], m: &Json) -> Material {
    let p = MaterialProperties {
        color: color(&m["color"]),
        opacity: m["opacity"].as_f64().unwrap_or(1.),
        transparent: m["transparent"].as_bool().unwrap_or(false),
        side: match m["side"].as_u64() {
            Some(1) => Side::Back,
            Some(2) => Side::Double,
            _ => Side::Front,
        },
        flat_shading: m["flatShading"].as_bool().unwrap_or(false),
        vertex_colors: m["vertexColors"].as_bool().unwrap_or(false),
        map: texture(images, &m["map"], true),
        ..Default::default()
    };
    let emissive = color(&m["emissive"]).0 * m["emissiveIntensity"].as_f64().unwrap_or(1.);
    if m["type"].as_str() == Some("MeshLambertMaterial") {
        Material::Lambert(MeshLambertMaterial {
            properties: p,
            emissive: Color(emissive),
            ..Default::default()
        })
    } else {
        Material::Phong(MeshPhongMaterial {
            properties: p,
            emissive: Color(emissive),
            specular: color(&m["specular"]),
            shininess: m["shininess"].as_f64().unwrap_or(30.),
            normal_map: texture(images, &m["normalMap"], false),
            specular_map: texture(images, &m["specularMap"], false),
            emissive_map: texture(images, &m["emissiveMap"], true),
            ..Default::default()
        })
    }
}

fn instantiate(s: &mut Scene, l: &Loaded) -> Result<Model> {
    let (json, bin) = (&l.json, &l.bin);
    let mut handles = vec![];
    for n in json["nodes"]
        .as_array()
        .ok_or(Error::Invalid("fbx nodes"))?
    {
        let kind = match n["light"].is_object() {
            true => NodeKind::Light(Light::Ambient {
                color: color(&n["light"]["color"]),
                intensity: n["light"]["intensity"].as_f64().unwrap_or(1.),
            }),
            false => NodeKind::Group,
        };
        let h = s.insert(kind);
        let (p, q, sc) = (
            numbers(&n["position"]),
            numbers(&n["quaternion"]),
            numbers(&n["scale"]),
        );
        let node = s.get_mut(h)?;
        node.name = n["name"].as_str().unwrap_or("").to_string();
        node.position = Vector3::new(p[0], p[1], p[2]);
        node.quaternion = Quaternion::from_xyzw(q[0], q[1], q[2], q[3]);
        node.scale = Vector3::new(sc[0], sc[1], sc[2]);
        if let Some(parent) = n["parent"].as_u64() {
            s.add(handles[parent as usize], h)?;
        }
        handles.push(h);
    }
    let mut morphs = vec![];
    for m in json["meshes"].as_array().into_iter().flatten() {
        let h = handles[m["node"].as_u64().ok_or(Error::Invalid("fbx mesh node"))? as usize];
        let mut g = BufferGeometry::default();
        for (name, a) in m["attributes"].as_object().into_iter().flatten() {
            g.set_attribute(name.as_str(), attribute(bin, a)?);
        }
        if m["index"].is_object() {
            let index = match m["index"]["type"].as_str() {
                Some("Uint16Array") => bytes(bin, &m["index"], 2)?
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|&c| u32::from(u16::from_le_bytes(c)))
                    .collect(),
                _ => bytes(bin, &m["index"], 4)?
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|&c| u32::from_le_bytes(c))
                    .collect(),
            };
            g.set_index(Some(index));
        }
        for group in m["groups"].as_array().into_iter().flatten() {
            g.add_group(
                group["start"].as_u64().unwrap_or(0) as usize,
                group["count"].as_u64().unwrap_or(0) as usize,
                group["materialIndex"].as_u64().unwrap_or(0) as usize,
            );
        }
        let mut targets = 0;
        for (name, list) in m["morphAttributes"].as_object().into_iter().flatten() {
            let attributes = list
                .as_array()
                .into_iter()
                .flatten()
                .map(|a| attribute(bin, a))
                .collect::<Result<Vec<_>>>()?;
            targets = targets.max(attributes.len());
            g.morph_attributes.insert(name.clone(), attributes);
        }
        g.morph_targets_relative = m["morphTargetsRelative"].as_bool().unwrap_or(false);
        let materials: Vec<Arc<Material>> = m["materials"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|v| Arc::new(material(&l.images, v)))
            .collect();
        let node = s.get_mut(h)?;
        node.kind = NodeKind::Mesh(Mesh {
            geometry: Arc::new(g),
            materials,
        });
        node.cast_shadow = true;
        node.receive_shadow = true;
        if targets > 0 {
            node.morph_weights = vec![0.; targets];
            morphs.push((h, targets));
        }
        // Attached SkinnedMesh: bone world × boneInverse × bindMatrix.
        if m["skin"].is_object() {
            let bind = matrix(&m["skin"]["bindMatrix"]);
            let joints = m["skin"]["bones"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|b| handles[b.as_u64().unwrap_or(0) as usize])
                .collect();
            let inverse = m["skin"]["boneInverses"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|b| matrix(b) * bind)
                .collect();
            node.skin = Some(Skin {
                joints,
                inverse_bind_matrices: inverse,
            });
        }
    }
    let mut clips = vec![];
    for c in json["clips"].as_array().into_iter().flatten() {
        let mut tracks = vec![];
        for t in c["tracks"].as_array().into_iter().flatten() {
            let property = match t["property"].as_str() {
                Some("position") => Property::Position,
                Some("quaternion") => Property::Rotation,
                Some("scale") => Property::Scale,
                _ => continue,
            };
            let size = if property == Property::Rotation { 4 } else { 3 };
            let times: Vec<f64> = f32s(bin, &t["times"])?.into_iter().map(f64::from).collect();
            let values = f32s(bin, &t["values"])?
                .chunks(size)
                .map(|v| v.iter().map(|&x| f64::from(x)).collect())
                .collect();
            tracks.push(Track {
                target: handles[t["node"].as_u64().unwrap_or(0) as usize],
                property,
                times,
                values,
                interpolation: match t["interpolation"].as_u64() {
                    Some(2300) => Interpolation::Step,
                    _ => Interpolation::Linear,
                },
            });
        }
        clips.push(Arc::new(Clip {
            name: c["name"].as_str().unwrap_or("").to_string(),
            tracks,
        }));
    }
    let root = handles[0];
    s.get_mut(root)?.scale = Vector3::splat(scale(l.asset));
    Ok(Model {
        root,
        clips,
        morphs,
    })
}

pub(super) struct Demo {
    controls: Controls,
    asset: usize,
    model: Option<Model>,
    mixer: Option<AnimationMixer>,
    /// ( mesh, target, influence ) of the morph controls.
    morph: (usize, usize, f64),
    pending_morph: bool,
    pending_clip: Option<usize>,
    loaded: Rc<RefCell<Option<Result<Loaded>>>>,
    requested: Option<usize>,
    /// The example clock, and the clock the mixer has reached.
    time: f64,
    mixed: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(100., 200., 300.);
        s.background = Color::from_hex(0xa0a0a0);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xa0a0a0),
            near: 200.,
            far: 1000.,
        });
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x444444),
            intensity: 5.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(0., 200., 0.);
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 5.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(0., 200., 100.);
        n.cast_shadow = true;
        let mut ground_geometry = PlaneGeometry::build(2000., 2000., 1, 1)?;
        ground_geometry.rotate_x(-PI / 2.)?;
        let mut ground_material = MeshPhongMaterial::default();
        ground_material.properties.color = Color::from_hex(0x999999);
        ground_material.properties.depth_write = false;
        let ground = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(ground_geometry),
            materials: vec![Arc::new(Material::Phong(ground_material))],
        }));
        s.get_mut(ground)?.receive_shadow = true;
        let mut grid = grid_helper(2000., 20, 0x000000, 0x000000)?;
        let p = Arc::make_mut(&mut grid.material).properties_mut();
        p.opacity = 0.2;
        p.transparent = true;
        s.insert(NodeKind::Line(grid));
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 100., 0.));
        controls.update(s, c)?;
        let loaded = load(0).await?;
        let mut demo = Self {
            controls,
            asset: 0,
            model: None,
            mixer: None,
            morph: (0, 0, 0.),
            pending_morph: false,
            pending_clip: None,
            loaded: Rc::new(RefCell::new(Some(Ok(loaded)))),
            requested: Some(0),
            time: 0.,
            mixed: 0.,
        };
        demo.install(s)?;
        Ok(demo)
    }
    /// loadAsset()'s onLoad: the previous model leaves, the new one plays its first clip.
    fn install(&mut self, s: &mut Scene) -> Result<()> {
        let Some(loaded) = self.loaded.borrow_mut().take() else {
            return Ok(());
        };
        let loaded = loaded?;
        if loaded.asset != self.asset {
            return Ok(());
        }
        if let Some(m) = self.model.take() {
            s.dispose(m.root)?;
        }
        let model = instantiate(s, &loaded)?;
        self.mixer = None;
        if let Some(clip) = model.clips.first() {
            let mut mixer = AnimationMixer::default();
            mixer.play(clip.clone())?;
            self.mixer = Some(mixer);
        }
        self.model = Some(model);
        // The new mixer starts at the current clock.
        self.mixed = self.time;
        self.apply_morph(s)?;
        Ok(())
    }
    fn apply_morph(&mut self, s: &mut Scene) -> Result<()> {
        let Some(model) = &self.model else {
            return Ok(());
        };
        let (mesh, target, value) = self.morph;
        if let Some(&(h, targets)) = model.morphs.get(mesh)
            && target < targets
        {
            s.get_mut(h)?.morph_weights[target] = value;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if self.requested != Some(self.asset) {
            self.requested = Some(self.asset);
            let (slot, asset) = (self.loaded.clone(), self.asset);
            wasm_bindgen_futures::spawn_local(async move {
                let result = load(asset).await;
                *slot.borrow_mut() = Some(result);
            });
        }
        if self.loaded.borrow().is_some() {
            self.install(s)?;
        }
        if std::mem::take(&mut self.pending_morph) {
            self.apply_morph(s)?;
        }
        if let Some(clip) = self.pending_clip.take()
            && let (Some(model), Some(mixer)) = (&self.model, &mut self.mixer)
            && let Some(clip) = model.clips.get(clip)
        {
            // stopAllAction(): the bound properties return to their original state.
            mixer.restore(s)?;
            *mixer = AnimationMixer::default();
            mixer.play(clip.clone())?;
        }
        // mixer.update( timer.getDelta() ): a clock moved back restarts the clip.
        if let Some(mixer) = &mut self.mixer {
            if self.time < self.mixed {
                for action in &mut mixer.actions {
                    action.time = 0.;
                }
                self.mixed = self.time;
            }
            mixer.update(s, self.time - self.mixed)?;
        }
        self.mixed = self.time;
        self.controls.update(s, c)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        _wheel: f64,
        _pan: bool,
        height: f64,
    ) -> Result<()> {
        // enableZoom and enablePan are false.
        let _ = camera_state(s, c)?;
        self.controls.rotate(dx, dy, height);
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.asset = (value as usize).min(COUNT - 1),
            // The Animations folder's clip: stopAllAction(), then the clip plays.
            1 => self.pending_clip = Some(value as usize),
            2 => self.morph.0 = value as usize,
            3 => self.morph.1 = value as usize,
            4 => {
                self.morph.2 = f64::from(value);
                self.pending_morph = true;
            }
            _ => return Err(Error::Invalid("fbx parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
