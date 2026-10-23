//! webgl_loader_fbx: the page's 14 FBX models, each parsed at load by the
//! FBXLoader port ( `fbx::parse` ): the node tree, meshes with their material
//! groups, morph targets, Phong and Lambert materials with their textures,
//! skinned meshes' bones and bind matrices, the ambient light one model
//! carries, and the keyframe clips. The selected model is scaled as the page
//! scales it, its first clip plays on an AnimationMixer by the Timer, and it
//! casts and receives the SunLight's cascaded shadows over the Phong ground
//! and translucent grid in linear fog, with damped OrbitControls ( no zoom
//! or pan ) and a hemisphere light. A model loads when chosen, as the page
//! loads it. The page's GUI builds a clip list and per-mesh morph sliders
//! for each model; the gallery's fixed controls give the clip by index and
//! one morph influence by mesh and target index.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_scenes::grid_helper;
use crate::animation::{AnimationMixer, Clip, Interpolation, Property, Track};
use crate::deformation::Skin;
use crate::fbx::{self, FbxImage, FbxMaterial, FbxMaterialKind, FbxNodeKind, FbxScene};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

const ASSETS: &str = "assets/fbx-loader/models/fbx";
/// The page's asset list.
const NAMES: [&str; 14] = [
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
];
const COUNT: usize = NAMES.len();
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

/// A loaded model: its root, the clips, and the morph meshes.
struct Model {
    root: Object3D,
    clips: Vec<Arc<Clip>>,
    morphs: Vec<(Object3D, usize)>,
}
struct Loaded {
    asset: usize,
    scene: FbxScene,
    /// The decoded image of each texture ( none when it fails to load ).
    images: Vec<Option<Texture>>,
}

/// FBXLoader.load( 'models/fbx/' + asset + '.fbx' ), with
/// trimAnimationClips for the ball, and its textures.
async fn load(asset: usize) -> Result<Loaded> {
    let name = NAMES[asset];
    let dir = name.rfind('/').map_or("", |i| &name[..=i]);
    let bytes = fetch(&format!("{ASSETS}/{name}.fbx")).await?;
    let scene = fbx::parse(
        &bytes,
        &fbx::Options {
            trim_animation_clips: asset == 13,
            path: format!("{ASSETS}/{dir}"),
            ..Default::default()
        },
    )?;
    let mut images = vec![];
    for t in &scene.textures {
        // A file the asset names but the examples do not ship ( Head_69's
        // ALPHA.png ) fails to load in the page too.
        images.push(match &t.image {
            FbxImage::None => None,
            FbxImage::Url(url) => match fetch(url).await {
                Ok(b) => decode_texture_image(&b).await.ok(),
                Err(_) => None,
            },
            FbxImage::Embedded { data, .. } => decode_texture_image(data).await.ok(),
        });
    }
    Ok(Loaded {
        asset,
        scene,
        images,
    })
}

fn texture(l: &Loaded, index: Option<usize>, srgb: bool) -> Option<Arc<Texture>> {
    let index = index?;
    let mut t = l.images.get(index)?.clone()?;
    let source = &l.scene.textures[index];
    t.srgb = srgb && source.srgb;
    t.wrap_s = if source.repeat_s {
        Wrapping::Repeat
    } else {
        Wrapping::Clamp
    };
    t.wrap_t = if source.repeat_t {
        Wrapping::Repeat
    } else {
        Wrapping::Clamp
    };
    t.repeat = source.repeat;
    t.offset = source.offset;
    t.mipmap_filter = Some(Filter::Linear);
    Some(Arc::new(t))
}

fn material(l: &Loaded, m: &FbxMaterial) -> Material {
    let p = MaterialProperties {
        color: m.color,
        opacity: m.opacity,
        transparent: m.transparent,
        vertex_colors: m.vertex_colors,
        map: texture(l, m.map, true),
        ..Default::default()
    };
    let emissive = Color(m.emissive.0 * m.emissive_intensity);
    if m.kind == FbxMaterialKind::Lambert {
        Material::Lambert(MeshLambertMaterial {
            properties: p,
            emissive,
            ..Default::default()
        })
    } else {
        Material::Phong(MeshPhongMaterial {
            properties: p,
            emissive,
            specular: m.specular,
            shininess: m.shininess,
            normal_map: texture(l, m.normal_map, false),
            specular_map: texture(l, m.specular_map, true),
            emissive_map: texture(l, m.emissive_map, true),
            ..Default::default()
        })
    }
}

fn instantiate(s: &mut Scene, l: &Loaded) -> Result<Model> {
    let fbx = &l.scene;
    let geometries: Vec<Arc<BufferGeometry>> = fbx
        .geometries
        .iter()
        .map(|g| Arc::new(g.geometry.clone()))
        .collect();
    let materials: Vec<Arc<Material>> = fbx
        .materials
        .iter()
        .map(|m| Arc::new(material(l, m)))
        .collect();
    let mut handles = vec![];
    for n in &fbx.nodes {
        let kind = match &n.kind {
            FbxNodeKind::AmbientLight { color, intensity } => NodeKind::Light(Light::Ambient {
                color: *color,
                intensity: *intensity,
            }),
            _ => NodeKind::Group,
        };
        let h = s.insert(kind);
        let node = s.get_mut(h)?;
        node.name = n.name.clone();
        node.position = n.position;
        node.quaternion = n.quaternion;
        node.scale = n.scale;
        if let Some(parent) = n.parent {
            s.add(handles[parent], h)?;
        }
        handles.push(h);
    }
    let mut morphs = vec![];
    for (i, n) in fbx.nodes.iter().enumerate() {
        let FbxNodeKind::Mesh(m) = &n.kind else {
            continue;
        };
        let h = handles[i];
        let targets = m.morph_targets.len();
        let node = s.get_mut(h)?;
        node.kind = NodeKind::Mesh(Mesh {
            geometry: geometries[m.geometry].clone(),
            materials: m.materials.iter().map(|&k| materials[k].clone()).collect(),
        });
        node.cast_shadow = true;
        node.receive_shadow = true;
        if targets > 0 {
            node.morph_weights = vec![0.; targets];
            morphs.push((h, targets));
        }
        // Attached SkinnedMesh: bone world × boneInverse × bindMatrix.
        if let Some(skin) = &m.skin {
            node.skin = Some(Skin {
                joints: skin.bones.iter().map(|b| handles[b.unwrap_or(0)]).collect(),
                inverse_bind_matrices: skin
                    .bone_inverses
                    .iter()
                    .map(|b| *b * skin.bind_matrix)
                    .collect(),
            });
        }
    }
    let mut clips = vec![];
    for c in &fbx.clips {
        let mut tracks = vec![];
        for t in &c.tracks {
            let (node, property) = t.binding();
            let property = match property {
                "position" => Property::Position,
                "quaternion" => Property::Rotation,
                "scale" => Property::Scale,
                _ => continue,
            };
            let Some(target) = fbx.find_node(node) else {
                continue;
            };
            let size = if property == Property::Rotation { 4 } else { 3 };
            tracks.push(Track {
                target: handles[target],
                property,
                times: t.times.iter().map(|&v| f64::from(v)).collect(),
                values: t
                    .values
                    .chunks(size)
                    .map(|v| v.iter().map(|&x| f64::from(x)).collect())
                    .collect(),
                interpolation: Interpolation::Linear,
            });
        }
        clips.push(Arc::new(Clip {
            name: c.name.clone(),
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

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgl_loader_fbx.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_fbx(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                2000.,
                Box::new(Demo::create(scene, camera, 435, renderer).await?),
            ))
        },
    )
    .await
}
