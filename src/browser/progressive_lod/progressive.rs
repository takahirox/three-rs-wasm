//! @needle-tools/gltf-progressive 3.2.0: NEEDLE_progressive's registration,
//! getOrLoadLOD with its PromiseQueue and cache, assignMeshLOD /
//! assignTextureLOD, and LODsManager's per-render update, in the package's f64
//! operations.
use crate::geometry::{Attribute, BufferGeometry};
use crate::gltf::{AnimatedGltf, GltfInstance};
use crate::material::{Material, Texture};
use crate::{Error, Result, camera::*, scene::*};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// GLTFLoader.load's header for needle.tools URLs ( configureLoader's
/// progressive default ).
const ACCEPT: &str = "*/*;progressive=allowed;usecase=default";
const TARGET_TRIANGLE_DENSITY: f64 = 200_000.;
const SKINNED_BOUNDS_INTERVAL: u64 = 30;
const MAX_CONCURRENT: usize = 50;

/// A loaded model: its glTF, the directory its relative LOD paths resolve
/// against ( parser.options.path ) and its JSON.
pub(crate) struct Loaded {
    pub gltf: AnimatedGltf,
    pub path: String,
    pub json: serde_json::Value,
}

async fn fetch(url: &str) -> Result<Vec<u8>> {
    let window = web_sys::window().ok_or(Error::Invalid("window"))?;
    let headers = web_sys::Headers::new().map_err(|e| Error::Asset(format!("{e:?}")))?;
    headers
        .set("Accept", ACCEPT)
        .map_err(|e| Error::Asset(format!("{e:?}")))?;
    let init = web_sys::RequestInit::new();
    init.set_headers(&headers);
    let response = JsFuture::from(window.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|e| Error::Asset(format!("{e:?}")))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| Error::Invalid("response"))?;
    if !response.ok() {
        return Err(Error::Asset(format!("HTTP {}: {url}", response.status())));
    }
    let data = JsFuture::from(
        response
            .array_buffer()
            .map_err(|e| Error::Asset(format!("{e:?}")))?,
    )
    .await
    .map_err(|e| Error::Asset(format!("{e:?}")))?;
    Ok(js_sys::Uint8Array::new(&data).to_vec())
}

/// The GLB's JSON chunk.
fn glb_json(bytes: &[u8]) -> Result<serde_json::Value> {
    let length = bytes.get(12..16).ok_or(Error::Invalid("GLB header"))?;
    let length = u32::from_le_bytes(
        length
            .try_into()
            .map_err(|_| Error::Invalid("GLB header"))?,
    ) as usize;
    let chunk = bytes
        .get(20..20 + length)
        .ok_or(Error::Invalid("GLB JSON"))?;
    serde_json::from_slice(chunk).map_err(|e| Error::Asset(e.to_string()))
}

/// GLTFLoader.load of a model with the progressive header.
pub(crate) async fn load_model(url: &str) -> Result<Loaded> {
    let bytes = fetch(url).await?;
    let base = url.rsplit_once('/').ok_or(Error::Invalid("model URL"))?.0;
    let json = glb_json(&bytes)?;
    let (asset, buffers, images) =
        super::super::gltf_viewer::load_asset_bytes(&bytes, base).await?;
    let gltf = crate::gltf::import_animated_decoded(&asset, &buffers, &images)?;
    Ok(Loaded {
        gltf,
        path: format!("{base}/"),
        json,
    })
}

#[derive(Clone, Debug)]
struct MeshLod {
    path: String,
    hash: Option<String>,
    density: f64,
    densities: Vec<f64>,
}
#[derive(Clone, Debug)]
struct TextureLod {
    path: String,
    hash: Option<String>,
    height: Option<f64>,
}
#[derive(Clone, Debug)]
enum Ext {
    Mesh(Vec<MeshLod>),
    Texture(Vec<TextureLod>),
}
impl Ext {
    fn len(&self) -> usize {
        match self {
            Ext::Mesh(l) => l.len(),
            Ext::Texture(l) => l.len(),
        }
    }
    fn path(&self, level: usize) -> Option<(&str, Option<&str>)> {
        match self {
            Ext::Mesh(l) => l.get(level).map(|l| (l.path.as_str(), l.hash.as_deref())),
            Ext::Texture(l) => l.get(level).map(|l| (l.path.as_str(), l.hash.as_deref())),
        }
    }
}
fn hash(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}
fn parse_ext(ext: &serde_json::Value, texture: bool) -> Option<(String, Ext)> {
    let guid = ext["guid"].as_str()?.to_owned();
    let lods = ext["lods"].as_array()?;
    let ext = if texture {
        Ext::Texture(
            lods.iter()
                .map(|l| TextureLod {
                    path: l["path"].as_str().unwrap_or_default().to_owned(),
                    hash: hash(&l["hash"]),
                    height: l["width"].as_f64().and(l["height"].as_f64()),
                })
                .collect(),
        )
    } else {
        Ext::Mesh(
            lods.iter()
                .map(|l| MeshLod {
                    path: l["path"].as_str().unwrap_or_default().to_owned(),
                    hash: hash(&l["hash"]),
                    density: l["density"].as_f64().unwrap_or(0.),
                    densities: l["densities"]
                        .as_array()
                        .map(|d| d.iter().filter_map(|v| v.as_f64()).collect())
                        .unwrap_or_default(),
                })
                .collect(),
        )
    };
    Some((guid, ext))
}

/// LODInformation: the model directory, the LOD key ( guid ), the level and
/// the primitive index.
#[derive(Clone, Debug)]
struct Info {
    url: String,
    key: String,
    level: usize,
    index: Option<usize>,
}

/// A loaded LOD: a texture, or a mesh's geometry ( one primitive ) or
/// geometries ( a group's children ).
#[derive(Clone)]
enum Resource {
    Texture(Arc<Texture>),
    Geometry(Arc<BufferGeometry>),
    Geometries(Vec<Arc<BufferGeometry>>),
}
type Cell = Rc<RefCell<Option<Option<Resource>>>>;

/// The material slots GLTFLoader assigns, in the order Object.keys( material )
/// lists them.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Slot {
    Map,
    Ao,
    Emissive,
    Normal,
    Roughness,
    Metalness,
}

#[derive(Default)]
struct LodState {
    frames: u64,
    last_mesh: i64,
    last_texture: i64,
    coverage: f64,
    volume: [f64; 3],
}

struct MeshEntry {
    node: Object3D,
    id: usize,
    material: usize,
    skinned: bool,
    state: LodState,
    /// mesh[ $currentLOD ] and mesh[ "LOD:requested level" ].
    current: Option<i64>,
    requested: Option<i64>,
    /// registerRaycastMesh's low-resolution geometry.
    raycast: Option<Arc<BufferGeometry>>,
    /// SkinnedMesh.boundingBox / boundingSphere and the bounds' frame offset.
    skinned_box: Option<[[f64; 3]; 2]>,
    skinned_sphere: Option<([f64; 3], f64)>,
    offset: Option<u64>,
}

struct MinMax {
    min_count: f64,
    max_count: f64,
    lods: Vec<Option<(f64, f64)>>,
}

struct MaterialEntry {
    id: usize,
    nodes: Vec<Object3D>,
    slots: Vec<(Slot, Arc<Texture>)>,
    current: Option<i64>,
    minmax: Option<MinMax>,
}

enum Target {
    Mesh {
        entry: usize,
        level: i64,
        current: Arc<BufferGeometry>,
        index: usize,
    },
    Texture {
        material: usize,
        slot: Slot,
        level: i64,
        current: Arc<Texture>,
    },
}
enum Stage {
    Slot {
        lod_url: String,
        url: String,
        key: String,
        guid: String,
        texture: bool,
    },
    Load(Cell),
}
struct Task {
    target: Target,
    stage: Stage,
    /// The low-resolution answer of getOrLoadLOD past the last level.
    immediate: Option<Option<Resource>>,
}

/// NEEDLE_progressive's static state and the LODsManager.
#[derive(Default)]
pub(crate) struct Lods {
    pub pending_frame: bool,
    groups: Vec<Object3D>,
    meshes: Vec<MeshEntry>,
    materials: Vec<MaterialEntry>,
    lod_infos: HashMap<String, Ext>,
    geometry_infos: HashMap<usize, (Arc<BufferGeometry>, Info)>,
    texture_infos: HashMap<usize, (Arc<Texture>, Info)>,
    lowres: HashMap<String, Resource>,
    previously: HashMap<String, Cell>,
    running: Vec<(String, Cell)>,
    queue: VecDeque<usize>,
    tasks: Vec<Option<Task>>,
    /// The render list of the last render: ( mesh entry, list ).
    list: Vec<usize>,
    skinned_offsets: u64,
    frame: u64,
    interval: u64,
    fps_buffer: Vec<f64>,
    clock: Option<f64>,
    next_id: usize,
}

fn ptr<T>(a: &Arc<T>) -> usize {
    Arc::as_ptr(a) as *const u8 as usize
}

impl Lods {
    fn geometry_info(&self, g: &Arc<BufferGeometry>) -> Option<&Info> {
        self.geometry_infos.get(&ptr(g)).map(|(_, i)| i)
    }
    fn texture_info(&self, t: &Arc<Texture>) -> Option<&Info> {
        self.texture_infos.get(&ptr(t)).map(|(_, i)| i)
    }
    fn assign_geometry(&mut self, g: &Arc<BufferGeometry>, info: Info) {
        self.geometry_infos.insert(ptr(g), (g.clone(), info));
    }
    fn assign_texture(&mut self, t: &Arc<Texture>, info: Info) {
        self.texture_infos.insert(ptr(t), (t.clone(), info));
    }

    /// The model's group joins the scene: afterRoot's registerTexture and
    /// registerMesh, with the materials GLTFLoader shares between primitives.
    pub fn register(
        &mut self,
        s: &mut Scene,
        group: Object3D,
        loaded: &Loaded,
        instance: &GltfInstance,
    ) -> Result<()> {
        self.groups.push(group);
        let json = &loaded.json;
        let mut texture_exts: HashMap<usize, (String, Ext)> = HashMap::new();
        for (i, t) in json["textures"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
        {
            if let Some(e) = t["extensions"]
                .get("NEEDLE_progressive")
                .and_then(|e| parse_ext(e, true))
            {
                texture_exts.insert(i, e);
            }
        }
        // ( glTF material, derivative tangents, vertex colours, flat shading ) →
        // the material GLTFLoader assigns.
        let mut shared: HashMap<(Option<usize>, bool, bool, bool), usize> = HashMap::new();
        for (&node, &(mesh, primitive)) in instance.meshes.iter().zip(&instance.sources) {
            let (geometry, material, skinned) = {
                let n = s.get(node)?;
                let NodeKind::Mesh(m) = &n.kind else { continue };
                (m.geometry.clone(), m.materials[0].clone(), n.skin.is_some())
            };
            let primitive_json = &json["meshes"][mesh]["primitives"][primitive];
            let material_index = primitive_json["material"].as_u64().map(|v| v as usize);
            let attributes = &geometry.attributes;
            let variant = (
                material_index,
                !attributes.contains_key("tangent"),
                attributes.contains_key("color"),
                !attributes.contains_key("normal"),
            );
            let material_entry = match shared.get(&variant) {
                Some(&k) => {
                    let entry: &mut MaterialEntry = &mut self.materials[k];
                    entry.nodes.push(node);
                    // The primitive takes the shared material's current textures.
                    let shared_material = s.get(entry.nodes[0])?.kind.clone();
                    if let (NodeKind::Mesh(first), NodeKind::Mesh(m)) =
                        (shared_material, &mut s.get_mut(node)?.kind)
                    {
                        m.materials[0] = first.materials[0].clone();
                    }
                    k
                }
                None => {
                    let k = self.materials.len();
                    let mut slots = vec![];
                    if let Some(mi) = material_index {
                        let m = &json["materials"][mi];
                        let pbr = &m["pbrMetallicRoughness"];
                        let mut slot =
                            |slot: Slot,
                             info: &serde_json::Value,
                             texture: Option<&Arc<Texture>>| {
                                if let (Some(index), Some(texture)) =
                                    (info["index"].as_u64(), texture)
                                {
                                    slots.push((slot, index as usize, texture.clone()));
                                }
                            };
                        let current = material_textures(&material);
                        slot(Slot::Map, &pbr["baseColorTexture"], current.map.as_ref());
                        slot(Slot::Ao, &m["occlusionTexture"], current.ao.as_ref());
                        slot(
                            Slot::Emissive,
                            &m["emissiveTexture"],
                            current.emissive.as_ref(),
                        );
                        slot(Slot::Normal, &m["normalTexture"], current.normal.as_ref());
                        slot(
                            Slot::Roughness,
                            &pbr["metallicRoughnessTexture"],
                            current.metallic_roughness.as_ref(),
                        );
                        slot(
                            Slot::Metalness,
                            &pbr["metallicRoughnessTexture"],
                            current.metallic_roughness.as_ref(),
                        );
                    }
                    let mut entry_slots = vec![];
                    for (slot, index, texture) in slots {
                        if let Some((guid, ext)) = texture_exts.get(&index) {
                            let info = Info {
                                url: loaded.path.clone(),
                                key: guid.clone(),
                                level: ext.len(),
                                index: Some(index),
                            };
                            self.assign_texture(&texture, info);
                            self.lod_infos.insert(guid.clone(), ext.clone());
                            self.lowres
                                .insert(guid.clone(), Resource::Texture(texture.clone()));
                        }
                        entry_slots.push((slot, texture));
                    }
                    self.next_id += 1;
                    self.materials.push(MaterialEntry {
                        id: self.next_id,
                        nodes: vec![node],
                        slots: entry_slots,
                        current: None,
                        minmax: None,
                    });
                    shared.insert(variant, k);
                    k
                }
            };
            let mesh_ext = json["meshes"][mesh]["extensions"]
                .get("NEEDLE_progressive")
                .and_then(|e| parse_ext(e, false));
            let mut raycast = None;
            if let Some((guid, ext)) = mesh_ext {
                let info = Info {
                    url: loaded.path.clone(),
                    key: guid.clone(),
                    level: ext.len(),
                    index: Some(primitive),
                };
                self.assign_geometry(&geometry, info);
                let lowres = self
                    .lowres
                    .entry(guid.clone())
                    .or_insert_with(|| Resource::Geometries(vec![]));
                if let Resource::Geometries(list) = lowres {
                    if list.len() <= primitive {
                        list.resize(primitive + 1, geometry.clone());
                    }
                    list[primitive] = geometry.clone();
                }
                if ext.len() > 0 {
                    raycast = Some(geometry.clone());
                }
                self.lod_infos.insert(guid, ext);
            }
            self.next_id += 1;
            self.meshes.push(MeshEntry {
                node,
                id: self.next_id,
                material: material_entry,
                skinned,
                state: LodState {
                    last_mesh: -1,
                    last_texture: -1,
                    ..Default::default()
                },
                current: None,
                requested: None,
                raycast,
                skinned_box: None,
                skinned_sphere: None,
                offset: None,
            });
        }
        Ok(())
    }

    /// The frame's render list as WebGLRenderer.projectObject builds it: visible
    /// meshes whose bounding sphere meets the frustum ( a SkinnedMesh's own
    /// sphere, computed once ), sorted opaque, then transparent, then
    /// transmissive, as painterSortStable and reversePainterSortStable order.
    pub fn before_render(&mut self, s: &Scene, c: Object3D) -> Result<()> {
        let screen = projection_screen(s, c)?;
        let planes = frustum(&screen);
        let order: HashMap<Object3D, usize> = self
            .meshes
            .iter()
            .enumerate()
            .map(|(i, m)| (m.node, i))
            .collect();
        let mut opaque = vec![];
        let mut transparent = vec![];
        let mut transmissive = vec![];
        for &group in &self.groups {
            for h in s.traverse(group, true)? {
                let Some(&i) = order.get(&h) else { continue };
                let n = s.get(h)?;
                let NodeKind::Mesh(mesh) = &n.kind else {
                    continue;
                };
                let world = n.matrix_world.to_cols_array();
                if self.meshes[i].skinned && self.meshes[i].skinned_sphere.is_none() {
                    self.meshes[i].skinned_sphere = Some(skinned_sphere(s, h, &mesh.geometry)?);
                }
                let sphere = if self.meshes[i].skinned {
                    self.meshes[i].skinned_sphere
                } else {
                    mesh.geometry
                        .bounding_sphere
                        .map(|b| (b.center.to_array(), b.radius))
                };
                if n.frustum_culled
                    && let Some((center, radius)) = sphere
                    && !intersects(&planes, apply4(center, &world), radius * max_scale(&world))
                {
                    continue;
                }
                let z = clip_z(&[world[12], world[13], world[14]], &screen);
                let material = &mesh.materials[0];
                let key = (
                    self.materials[self.meshes[i].material].id,
                    z,
                    self.meshes[i].id,
                    i,
                );
                match material.as_ref() {
                    Material::Physical(p) if p.transmission > 0. => transmissive.push(key),
                    m if m.properties().transparent => transparent.push(key),
                    _ => opaque.push(key),
                }
            }
        }
        let by = |a: &(usize, f64, usize, usize), b: &(usize, f64, usize, usize)| {
            a.0.cmp(&b.0)
                .then(a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .then(a.2.cmp(&b.2))
        };
        opaque.sort_by(by);
        let reverse = |a: &(usize, f64, usize, usize), b: &(usize, f64, usize, usize)| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.2.cmp(&b.2))
        };
        transparent.sort_by(reverse);
        transmissive.sort_by(reverse);
        self.list = opaque
            .into_iter()
            .chain(transparent)
            .chain(transmissive)
            .map(|k| k.3)
            .collect();
        Ok(())
    }

    /// The wrapped render's frame clock and onAfterRender's update interval,
    /// then internalUpdate over the render list.
    pub fn after_render(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        canvas_height: f64,
        now: f64,
    ) -> Result<()> {
        self.frame += 1;
        // three's Clock: the first getDelta starts it at zero.
        let delta = self.clock.map_or(0., |old| (now - old) / 1000.);
        self.clock = Some(now);
        if self.fps_buffer.is_empty() {
            self.fps_buffer = vec![60.; 5];
            self.interval = 1;
        }
        self.fps_buffer.remove(0);
        self.fps_buffer.push(1. / delta);
        let fps = self.fps_buffer.iter().sum::<f64>() / self.fps_buffer.len() as f64;
        if fps < 40. && self.interval < 10 {
            self.interval += 1;
        } else if fps >= 60. && self.interval > 1 {
            self.interval -= 1;
        }
        if self.interval > 0 && !self.frame.is_multiple_of(self.interval) {
            return Ok(());
        }
        let camera = camera_info(s, c)?;
        let canvas_height = canvas_client_height().unwrap_or(canvas_height);
        let avail = web_sys::window()
            .and_then(|w| w.screen().ok())
            .and_then(|sc| sc.avail_height().ok())
            .unwrap_or(0) as f64;
        let ratio = web_sys::window().map_or(1., |w| w.device_pixel_ratio());
        let mut started = vec![];
        for i in self.list.clone() {
            self.update_lods(s, &camera, i, canvas_height, avail, ratio, &mut started)?;
        }
        // The low-resolution answers resolve after the loop, as microtasks.
        for t in started {
            if let Some(task) = self.tasks[t].as_mut()
                && let Some(result) = task.immediate.take()
            {
                let task = self.tasks[t].take().ok_or(Error::Invalid("task"))?;
                self.complete(s, task.target, Some(result))?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn update_lods(
        &mut self,
        s: &mut Scene,
        camera: &CameraInfo,
        i: usize,
        canvas_height: f64,
        avail: f64,
        ratio: f64,
        started: &mut Vec<usize>,
    ) -> Result<()> {
        let frames = self.meshes[i].state.frames;
        self.meshes[i].state.frames += 1;
        if frames < 2 {
            return Ok(());
        }
        let (mut mesh_lod, mut texture_lod) =
            self.calculate_lod_level(s, camera, i, canvas_height, avail, ratio)?;
        mesh_lod = mesh_lod.round();
        texture_lod = texture_lod.round();
        let (mesh_lod, texture_lod) = (mesh_lod as i64, texture_lod as i64);
        if mesh_lod >= 0 {
            self.load_progressive_meshes(s, i, mesh_lod, started)?;
        }
        if texture_lod >= 0 {
            let material = self.meshes[i].material;
            self.load_progressive_textures(material, texture_lod, started);
        }
        self.meshes[i].state.last_mesh = mesh_lod;
        self.meshes[i].state.last_texture = texture_lod;
        Ok(())
    }

    fn minmax(&mut self, material: usize) -> &MinMax {
        if self.materials[material].minmax.is_none() {
            let mut m = MinMax {
                min_count: f64::INFINITY,
                max_count: 0.,
                lods: vec![],
            };
            for (_, texture) in &self.materials[material].slots {
                let Some(info) = self.texture_info(texture) else {
                    continue;
                };
                let Some(Ext::Texture(lods)) = self.lod_infos.get(&info.key) else {
                    continue;
                };
                m.min_count = m.min_count.min(lods.len() as f64);
                m.max_count = m.max_count.max(lods.len() as f64);
                for (k, lod) in lods.iter().enumerate() {
                    if let Some(height) = lod.height {
                        if m.lods.len() <= k {
                            m.lods.resize(k + 1, None);
                        }
                        let (lo, hi) = m.lods[k].unwrap_or((f64::INFINITY, 0.));
                        m.lods[k] = Some((lo.min(height), hi.max(height)));
                    }
                }
            }
            self.materials[material].minmax = Some(m);
        }
        self.materials[material].minmax.as_ref().expect("minmax")
    }

    /// LODsManager.calculateLodLevel.
    fn calculate_lod_level(
        &mut self,
        s: &Scene,
        camera: &CameraInfo,
        i: usize,
        canvas_height: f64,
        avail: f64,
        ratio: f64,
    ) -> Result<(f64, f64)> {
        let node = self.meshes[i].node;
        let geometry = s
            .get(node)?
            .geometry()
            .cloned()
            .ok_or(Error::Invalid("mesh geometry"))?;
        let info = self.geometry_info(&geometry).cloned();
        let mesh_lods = info
            .as_ref()
            .and_then(|i| match self.lod_infos.get(&i.key) {
                Some(Ext::Mesh(l)) => Some(l.clone()),
                _ => None,
            });
        let primitive_index = info.as_ref().and_then(|i| i.index).map_or(-1, |i| i as i64);
        let has_mesh_lods = mesh_lods.as_ref().is_some_and(|l| !l.is_empty());
        let material = self.meshes[i].material;
        let (min_count, max_count, texture_levels) = {
            let m = self.minmax(material);
            (m.min_count, m.max_count, m.lods.clone())
        };
        let has_texture_lods = min_count != f64::INFINITY && min_count >= 0. && max_count >= 0.;
        if !has_mesh_lods && !has_texture_lods {
            return Ok((0., 0.));
        }
        let max_level = 10.;
        let mut mesh_level = max_level + 1.;
        let mut mesh_level_calculated = false;
        if !has_mesh_lods {
            mesh_level_calculated = true;
            mesh_level = 0.;
        }
        let world = s.get(node)?.matrix_world.to_cols_array();
        let mut bounding_box = geometry
            .bounding_box
            .map(|b| [b.min.to_array(), b.max.to_array()]);
        if self.meshes[i].skinned {
            if self.meshes[i].skinned_box.is_none() {
                self.meshes[i].skinned_box = Some(skinned_box(s, node, &geometry)?);
            } else {
                if self.meshes[i].offset.is_none_or(|o| o == 0) {
                    self.meshes[i].offset = Some(self.skinned_offsets);
                    self.skinned_offsets += 1;
                }
                let offset = self.meshes[i].offset.unwrap_or(0);
                if (self.meshes[i].state.frames + offset).is_multiple_of(SKINNED_BOUNDS_INTERVAL) {
                    let source = self.meshes[i].raycast.clone().unwrap_or(geometry.clone());
                    self.meshes[i].skinned_box = Some(skinned_box(s, node, &source)?);
                }
            }
            bounding_box = self.meshes[i].skinned_box;
        }
        let state = &mut self.meshes[i].state;
        if let Some(bbox) = bounding_box {
            let mut box1 = apply_box(bbox, &world);
            if is_inside(&box1, &camera.screen) {
                return Ok((0., 0.));
            }
            box1 = apply_box(box1, &camera.screen);
            let centrality = 1.;
            let mut size = box_size(&box1).map(|v| v * 0.5);
            if avail > 0. && canvas_height > 0. {
                size = size.map(|v| v * (canvas_height / avail));
            }
            size[0] *= camera.aspect;
            let box2 = apply_box(apply_box(bbox, &world), &camera.view);
            let size2 = box_size(&box2);
            let max2 = size2[0].max(size2[1]);
            let max1 = size[0].max(size[1]);
            if max1 != 0. && max2 != 0. {
                size[2] = size2[2] / size2[0].max(size2[1]) * size[0].max(size[1]);
            }
            state.coverage = size[0].max(size[1]).max(size[2]);
            state.volume = size;
            state.coverage *= centrality;
            let mut expected = 999.;
            if let Some(lods) = &mesh_lods
                && state.coverage > 0.
            {
                for (l, lod) in lods.iter().enumerate() {
                    let density = lod
                        .densities
                        .get(primitive_index.max(0) as usize)
                        .copied()
                        .filter(|d| *d != 0. && primitive_index >= 0)
                        .unwrap_or(if lod.density != 0. {
                            lod.density
                        } else {
                            0.00001
                        });
                    if density / state.coverage < TARGET_TRIANGLE_DENSITY {
                        expected = l as f64;
                        break;
                    }
                }
            }
            if expected < mesh_level {
                mesh_level = expected;
                mesh_level_calculated = true;
            }
        }
        let mesh_lod = if mesh_level_calculated {
            mesh_level
        } else {
            state.last_mesh as f64
        };
        let mut texture_lod = 0.;
        if has_texture_lods {
            if state.last_texture < 0 {
                texture_lod = max_count - 1.;
            } else {
                let factor = state.coverage * 4.;
                let screen_size = canvas_height / ratio;
                let pixel_size = screen_size * factor;
                let mut found = false;
                for k in (0..texture_levels.len()).rev() {
                    let Some((_, max_height)) = texture_levels[k] else {
                        continue;
                    };
                    if max_height > pixel_size || (!found && k == 0) {
                        found = true;
                        texture_lod = k as f64;
                        break;
                    }
                }
                if !found {
                    // The loop over undefined levels leaves the result unchanged.
                    texture_lod = state.last_texture as f64;
                }
            }
        }
        Ok((mesh_lod, texture_lod))
    }

    /// loadProgressiveMeshes and assignMeshLOD.
    fn load_progressive_meshes(
        &mut self,
        s: &Scene,
        i: usize,
        level: i64,
        started: &mut Vec<usize>,
    ) -> Result<()> {
        if self.meshes[i].current == Some(level) {
            return Ok(());
        }
        self.meshes[i].current = Some(level);
        let current = s
            .get(self.meshes[i].node)?
            .geometry()
            .cloned()
            .ok_or(Error::Invalid("mesh geometry"))?;
        let Some(info) = self.geometry_info(&current).cloned() else {
            return Ok(());
        };
        self.meshes[i].requested = Some(level);
        let target = Target::Mesh {
            entry: i,
            level,
            current,
            index: info.index.unwrap_or(0),
        };
        self.get_or_load(target, &info, level, false, started);
        Ok(())
    }

    /// loadProgressiveTextures and assignTextureLOD over the material's slots.
    fn load_progressive_textures(&mut self, material: usize, level: i64, started: &mut Vec<usize>) {
        let update = self.materials[material].current.is_none_or(|c| level < c);
        if !update {
            return;
        }
        self.materials[material].current = Some(level);
        for (slot, texture) in self.materials[material].slots.clone() {
            let Some(info) = self.texture_info(&texture).cloned() else {
                continue;
            };
            let target = Target::Texture {
                material,
                slot,
                level,
                current: texture,
            };
            self.get_or_load(target, &info, level, true, started);
        }
    }

    /// getOrLoadLOD: the low-resolution answer past the last level, or a
    /// queue slot for the level's file.
    fn get_or_load(
        &mut self,
        target: Target,
        info: &Info,
        level: i64,
        texture: bool,
        started: &mut Vec<usize>,
    ) {
        let Some(ext) = self.lod_infos.get(&info.key).cloned() else {
            return;
        };
        let index = self.tasks.len();
        if level > 0 && level as usize >= ext.len() {
            let lowres = self.lowres.get(&info.key).cloned();
            self.tasks.push(Some(Task {
                target,
                stage: Stage::Load(Rc::default()),
                immediate: Some(lowres),
            }));
            started.push(index);
            return;
        }
        let Some((path, hash)) = ext.path(level.max(0) as usize) else {
            return;
        };
        let lod_url = resolve_url(&info.url, path);
        if !(lod_url.ends_with(".glb") || lod_url.ends_with(".gltf")) {
            return;
        }
        let url = match hash {
            Some(h) => format!("{lod_url}?v={h}"),
            None => lod_url.clone(),
        };
        let key = format!("{lod_url}_{}", info.key);
        let stage = Stage::Slot {
            lod_url,
            url,
            key,
            guid: info.key.clone(),
            texture,
        };
        self.tasks.push(Some(Task {
            target,
            stage,
            immediate: None,
        }));
        self.queue.push_back(index);
        let _ = texture;
    }

    /// The PromiseQueue's tick ( slots for the waiting requests ), then the
    /// continuations of the finished loads.
    pub fn tick(&mut self, s: &mut Scene) -> Result<()> {
        self.running.retain(|(_, cell)| cell.borrow().is_none());
        let diff = MAX_CONCURRENT.saturating_sub(self.running.len());
        for _ in 0..diff {
            let Some(t) = self.queue.pop_front() else {
                break;
            };
            let Some(task) = self.tasks[t].as_mut() else {
                continue;
            };
            let Stage::Slot {
                lod_url,
                url,
                key,
                guid,
                texture,
            } = &task.stage
            else {
                continue;
            };
            let cell = match self.previously.get(key) {
                Some(cell) => cell.clone(),
                None => {
                    let cell: Cell = Rc::default();
                    self.previously.insert(key.clone(), cell.clone());
                    if !self.running.iter().any(|(k, _)| k == lod_url) {
                        self.running.push((lod_url.clone(), cell.clone()));
                    }
                    let (url, guid, texture, result) =
                        (url.clone(), guid.clone(), *texture, cell.clone());
                    wasm_bindgen_futures::spawn_local(async move {
                        let loaded = load_lod(&url, &guid, texture).await;
                        if let Err(e) = &loaded {
                            web_sys::console::error_1(
                                &format!("Error loading LOD from {url}: {e}").into(),
                            );
                        }
                        *result.borrow_mut() = Some(loaded.ok().flatten());
                    });
                    cell
                }
            };
            task.stage = Stage::Load(cell);
        }
        for t in 0..self.tasks.len() {
            let ready = matches!(&self.tasks[t], Some(Task { stage: Stage::Load(cell), immediate: None, .. }) if cell.borrow().is_some());
            if !ready {
                continue;
            }
            let task = self.tasks[t].take().ok_or(Error::Invalid("task"))?;
            let Stage::Load(cell) = &task.stage else {
                continue;
            };
            let result = cell.borrow().clone().flatten();
            self.complete(s, task.target, Some(result))?;
        }
        Ok(())
    }

    /// The `then` of assignMeshLOD and assignTextureLODForSlot.
    fn complete(
        &mut self,
        s: &mut Scene,
        target: Target,
        result: Option<Option<Resource>>,
    ) -> Result<()> {
        let result = result.flatten();
        match target {
            Target::Mesh {
                entry,
                level,
                current,
                index,
            } => {
                // A loaded file's geometries take their LODInformation: the
                // level, and the primitive index within a group.
                if let Some(info) = self.geometry_info(&current).cloned() {
                    let loaded: Vec<(usize, Arc<BufferGeometry>)> = match &result {
                        Some(Resource::Geometries(list)) => {
                            list.iter().cloned().enumerate().collect()
                        }
                        Some(Resource::Geometry(g)) => vec![(0, g.clone())],
                        _ => vec![],
                    };
                    for (i, g) in loaded {
                        if !self.geometry_infos.contains_key(&ptr(&g)) {
                            self.assign_geometry(
                                &g,
                                Info {
                                    level: level.max(0) as usize,
                                    index: Some(i),
                                    ..info.clone()
                                },
                            );
                        }
                    }
                }
                let geometry = match result {
                    Some(Resource::Geometries(list)) => list.get(index).cloned(),
                    Some(Resource::Geometry(g)) => Some(g),
                    _ => None,
                };
                if self.meshes[entry].requested == Some(level) {
                    self.meshes[entry].requested = None;
                    if let Some(g) = geometry
                        && !Arc::ptr_eq(&g, &current)
                        && let NodeKind::Mesh(m) = &mut s.get_mut(self.meshes[entry].node)?.kind
                    {
                        m.geometry = g;
                    }
                }
            }
            Target::Texture {
                material,
                slot,
                level,
                current,
            } => {
                let texture = match result {
                    // The cached or loaded texture, copySettings( current ) as a clone.
                    Some(Resource::Texture(t))
                        if self.texture_info(&t).is_none_or(|i| {
                            i.level as i64
                                != self.lod_infos.get(&i.key).map_or(-1, |e| e.len() as i64)
                        }) =>
                    {
                        let info = self.texture_info(&current).cloned();
                        let copy = Arc::new(copy_settings(&current, &t));
                        if let Some(info) = info {
                            self.assign_texture(
                                &copy,
                                Info {
                                    level: level.max(0) as usize,
                                    ..info
                                },
                            );
                        }
                        Some(copy)
                    }
                    Some(Resource::Texture(t)) => Some(t),
                    _ => None,
                };
                let Some(texture) = texture else {
                    return Ok(());
                };
                if Arc::ptr_eq(&texture, &current) {
                    return Ok(());
                }
                let Some(k) = self.materials[material]
                    .slots
                    .iter()
                    .position(|(s, _)| *s == slot)
                else {
                    return Ok(());
                };
                let assigned = self.materials[material].slots[k].1.clone();
                if let Some(info) = self.texture_info(&assigned)
                    && (info.level as i64) < level
                {
                    return Ok(());
                }
                self.materials[material].slots[k].1 = texture;
                self.apply_material(s, material)?;
            }
        }
        Ok(())
    }

    /// The material's slots onto every primitive that shares it.
    fn apply_material(&self, s: &mut Scene, material: usize) -> Result<()> {
        let entry = &self.materials[material];
        let Some(&first) = entry.nodes.first() else {
            return Ok(());
        };
        let NodeKind::Mesh(m) = &s.get(first)?.kind else {
            return Ok(());
        };
        let mut updated = (*m.materials[0]).clone();
        for (slot, texture) in &entry.slots {
            set_texture(&mut updated, *slot, texture.clone());
        }
        let updated = Arc::new(updated);
        for &n in &entry.nodes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(n)?.kind {
                m.materials[0] = updated.clone();
            }
        }
        Ok(())
    }

    /// Each mesh's displayed geometry and texture levels, for the comparison.
    pub fn summary(&self, s: &Scene) -> String {
        let mut rows = vec![];
        for m in &self.meshes {
            let Ok(n) = s.get(m.node) else { continue };
            let Some(g) = n.geometry() else { continue };
            let level = self
                .geometry_info(g)
                .map_or(String::new(), |i| i.level.to_string());
            let textures: Vec<String> = self.materials[m.material]
                .slots
                .iter()
                .map(|(slot, t)| {
                    format!(
                        "{slot:?}:{}",
                        self.texture_info(t)
                            .map_or(String::new(), |i| i.level.to_string())
                    )
                })
                .collect();
            rows.push(format!(
                "{}|{}|{}|{}",
                n.name,
                level,
                g.vertex_count(),
                textures.join(",")
            ));
        }
        rows.join("\n")
    }
}

struct Textures {
    map: Option<Arc<Texture>>,
    ao: Option<Arc<Texture>>,
    emissive: Option<Arc<Texture>>,
    normal: Option<Arc<Texture>>,
    metallic_roughness: Option<Arc<Texture>>,
}
fn material_textures(m: &Material) -> Textures {
    let map = m.properties().map.clone();
    match m {
        Material::Standard(s)
        | Material::Physical(crate::material::MeshPhysicalMaterial { base: s, .. }) => Textures {
            map,
            ao: s.occlusion_map.clone(),
            emissive: s.emissive_map.clone(),
            normal: s.normal_map.clone(),
            metallic_roughness: s.metallic_roughness_map.clone(),
        },
        _ => Textures {
            map,
            ao: None,
            emissive: None,
            normal: None,
            metallic_roughness: None,
        },
    }
}
fn set_texture(m: &mut Material, slot: Slot, t: Arc<Texture>) {
    if slot == Slot::Map {
        m.properties_mut().map = Some(t);
        return;
    }
    if let Material::Standard(s)
    | Material::Physical(crate::material::MeshPhysicalMaterial { base: s, .. }) = m
    {
        match slot {
            Slot::Ao => s.occlusion_map = Some(t),
            Slot::Emissive => s.emissive_map = Some(t),
            Slot::Normal => s.normal_map = Some(t),
            // roughnessMap and metalnessMap read the same texture.
            Slot::Roughness | Slot::Metalness => s.metallic_roughness_map = Some(t),
            Slot::Map => {}
        }
    }
}

/// copySettings( source, target ): a clone of the loaded image with the
/// current texture's sampler, colour space, transform and UV set.
fn copy_settings(current: &Texture, loaded: &Texture) -> Texture {
    let mut t = current.clone();
    t.width = loaded.width;
    t.height = loaded.height;
    t.rgba = loaded.rgba.clone();
    t.basis = loaded.basis.clone();
    t.blocks = loaded.blocks.clone();
    #[cfg(target_arch = "wasm32")]
    {
        t.bitmap = loaded.bitmap.clone();
    }
    if loaded.basis.is_none() && loaded.blocks.is_none() {
        // !target.mipmaps: generateMipmaps follows the current texture.
        t.mipmap_filter = current.mipmap_filter;
    }
    t
}

/// resolveUrl( source, uri ).
fn resolve_url(source: &str, uri: &str) -> String {
    if uri.starts_with("./") || uri.starts_with("http") {
        return uri.to_owned();
    }
    match source.rfind('/') {
        Some(i) => format!("{}{}", &source[..=i], uri.trim_start_matches('/')),
        None => uri.to_owned(),
    }
}

/// The request body of getOrLoadLOD: the LOD file's texture or mesh with the
/// extension's guid.
async fn load_lod(url: &str, guid: &str, texture: bool) -> Result<Option<Resource>> {
    let bytes = fetch(url).await?;
    let json = glb_json(&bytes)?;
    let base = url
        .split('?')
        .next()
        .unwrap_or(url)
        .rsplit_once('/')
        .map_or("", |(b, _)| b);
    let (asset, buffers, images) =
        super::super::gltf_viewer::load_asset_bytes(&bytes, base).await?;
    let matches = |list: &serde_json::Value| {
        list.as_array().and_then(|l| {
            l.iter()
                .position(|e| e["extensions"]["NEEDLE_progressive"]["guid"].as_str() == Some(guid))
        })
    };
    if let Some(index) = matches(&json["textures"]) {
        let t = &json["textures"][index];
        let source = t["extensions"]["KHR_texture_basisu"]["source"]
            .as_u64()
            .or(t["extensions"]["EXT_texture_webp"]["source"].as_u64())
            .or(t["extensions"]["EXT_texture_avif"]["source"].as_u64())
            .or(t["source"].as_u64())
            .ok_or(Error::Invalid("LOD texture source"))? as usize;
        let image = images
            .get(source)
            .cloned()
            .ok_or(Error::Invalid("LOD image"))?;
        return Ok(Some(Resource::Texture(Arc::new(image))));
    }
    if texture {
        // A texture request reads only textures; a mesh request falls through.
    }
    let Some(index) = matches(&json["meshes"]) else {
        return Ok(None);
    };
    let gltf = crate::gltf::import_animated_decoded(&asset, &buffers, &images)?;
    let mut scratch = Scene::new();
    let instance = gltf.instantiate(&mut scratch)?;
    let mut geometries: Vec<(usize, Arc<BufferGeometry>)> = vec![];
    for (&node, &(mesh, primitive)) in instance.meshes.iter().zip(&instance.sources) {
        if mesh == index
            && let Some(g) = scratch.get(node)?.geometry()
            && !geometries.iter().any(|(p, _)| *p == primitive)
        {
            geometries.push((primitive, g.clone()));
        }
    }
    geometries.sort_by_key(|(p, _)| *p);
    Ok(match geometries.len() {
        0 => None,
        1 if json["meshes"][index]["primitives"]
            .as_array()
            .is_some_and(|p| p.len() == 1) =>
        {
            Some(Resource::Geometry(geometries.remove(0).1))
        }
        _ => Some(Resource::Geometries(
            geometries.into_iter().map(|(_, g)| g).collect(),
        )),
    })
}

fn canvas_client_height() -> Option<f64> {
    let canvas = web_sys::window()?
        .document()?
        .query_selector("canvas")
        .ok()??;
    Some(canvas.client_height() as f64)
}

/// The camera's matrices as three's PerspectiveCamera holds them ( WebGL clip
/// space ): projectionMatrix × matrixWorldInverse, the view matrix and aspect.
struct CameraInfo {
    screen: [f64; 16],
    view: [f64; 16],
    aspect: f64,
}
fn camera_info(s: &Scene, c: Object3D) -> Result<CameraInfo> {
    let (camera, world) = s.camera(c)?;
    let Camera::Perspective(p) = camera else {
        return Err(Error::Invalid("perspective camera"));
    };
    let view = invert(&world.to_cols_array());
    Ok(CameraInfo {
        screen: multiply(&projection(p), &view),
        view,
        aspect: p.aspect,
    })
}
fn projection_screen(s: &Scene, c: Object3D) -> Result<[f64; 16]> {
    Ok(camera_info(s, c)?.screen)
}
/// updateProjectionMatrix: makePerspective in the WebGL coordinate system.
fn projection(p: &PerspectiveCamera) -> [f64; 16] {
    let near = p.near;
    let top =
        near * (std::f64::consts::PI / 180. * 0.5 * p.fov).tan() / p.zoom.max(f64::MIN_POSITIVE);
    let height = 2. * top;
    let width = p.aspect * height;
    let left = -0.5 * width;
    let (right, bottom, far) = (left + width, top - height, p.far);
    let mut m = [0.; 16];
    m[0] = 2. * near / (right - left);
    m[5] = 2. * near / (top - bottom);
    m[8] = (right + left) / (right - left);
    m[9] = (top + bottom) / (top - bottom);
    m[10] = -(far + near) / (far - near);
    m[14] = (-2. * far * near) / (far - near);
    m[11] = -1.;
    m
}

/// Matrix4.multiplyMatrices( a, b ).
fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    let mut out = [0.; 16];
    for row in 0..4 {
        for col in 0..4 {
            out[col * 4 + row] = a[row] * b[col * 4]
                + a[4 + row] * b[col * 4 + 1]
                + a[8 + row] * b[col * 4 + 2]
                + a[12 + row] * b[col * 4 + 3];
        }
    }
    out
}
/// Matrix4.invert.
fn invert(te: &[f64; 16]) -> [f64; 16] {
    let [
        n11,
        n21,
        n31,
        n41,
        n12,
        n22,
        n32,
        n42,
        n13,
        n23,
        n33,
        n43,
        n14,
        n24,
        n34,
        n44,
    ] = *te;
    let t11 =
        n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 - n23 * n32 * n44
            + n22 * n33 * n44;
    let t12 =
        n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 + n13 * n32 * n44
            - n12 * n33 * n44;
    let t13 =
        n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 - n13 * n22 * n44
            + n12 * n23 * n44;
    let t14 =
        n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34
            - n12 * n23 * n34;
    let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;
    if det == 0. {
        return [0.; 16];
    }
    let d = 1. / det;
    [
        t11 * d,
        (n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 + n23 * n31 * n44
            - n21 * n33 * n44)
            * d,
        (n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 - n22 * n31 * n44
            + n21 * n32 * n44)
            * d,
        (n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 + n22 * n31 * n43
            - n21 * n32 * n43)
            * d,
        t12 * d,
        (n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 - n13 * n31 * n44
            + n11 * n33 * n44)
            * d,
        (n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 + n12 * n31 * n44
            - n11 * n32 * n44)
            * d,
        (n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 - n12 * n31 * n43
            + n11 * n32 * n43)
            * d,
        t13 * d,
        (n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 + n13 * n21 * n44
            - n11 * n23 * n44)
            * d,
        (n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 - n12 * n21 * n44
            + n11 * n22 * n44)
            * d,
        (n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 + n12 * n21 * n43
            - n11 * n22 * n43)
            * d,
        t14 * d,
        (n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 - n13 * n21 * n34
            + n11 * n23 * n34)
            * d,
        (n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 + n12 * n21 * n34
            - n11 * n22 * n34)
            * d,
        (n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 - n12 * n21 * n33
            + n11 * n22 * n33)
            * d,
    ]
}
/// Vector3.applyMatrix4 ( with the perspective divide ).
fn apply4(v: [f64; 3], e: &[f64; 16]) -> [f64; 3] {
    let [x, y, z] = v;
    let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
    [
        (e[0] * x + e[4] * y + e[8] * z + e[12]) * w,
        (e[1] * x + e[5] * y + e[9] * z + e[13]) * w,
        (e[2] * x + e[6] * y + e[10] * z + e[14]) * w,
    ]
}
/// Vector4.applyMatrix4's z for a point ( the render list's sort depth ).
fn clip_z(v: &[f64; 3], e: &[f64; 16]) -> f64 {
    e[2] * v[0] + e[6] * v[1] + e[10] * v[2] + e[14]
}
/// Matrix4.getMaxScaleOnAxis.
fn max_scale(te: &[f64; 16]) -> f64 {
    let x = te[0] * te[0] + te[1] * te[1] + te[2] * te[2];
    let y = te[4] * te[4] + te[5] * te[5] + te[6] * te[6];
    let z = te[8] * te[8] + te[9] * te[9] + te[10] * te[10];
    x.max(y).max(z).sqrt()
}
/// Frustum.setFromProjectionMatrix ( WebGL ): six normalized planes.
fn frustum(me: &[f64; 16]) -> [[f64; 4]; 6] {
    let plane = |x: f64, y: f64, z: f64, w: f64| {
        let inverse = 1. / (x * x + y * y + z * z).sqrt();
        [x * inverse, y * inverse, z * inverse, w * inverse]
    };
    [
        plane(
            me[3] - me[0],
            me[7] - me[4],
            me[11] - me[8],
            me[15] - me[12],
        ),
        plane(
            me[3] + me[0],
            me[7] + me[4],
            me[11] + me[8],
            me[15] + me[12],
        ),
        plane(
            me[3] + me[1],
            me[7] + me[5],
            me[11] + me[9],
            me[15] + me[13],
        ),
        plane(
            me[3] - me[1],
            me[7] - me[5],
            me[11] - me[9],
            me[15] - me[13],
        ),
        plane(
            me[3] - me[2],
            me[7] - me[6],
            me[11] - me[10],
            me[15] - me[14],
        ),
        plane(
            me[3] + me[2],
            me[7] + me[6],
            me[11] + me[10],
            me[15] + me[14],
        ),
    ]
}
/// Frustum.intersectsSphere.
fn intersects(planes: &[[f64; 4]; 6], center: [f64; 3], radius: f64) -> bool {
    planes
        .iter()
        .all(|p| p[0] * center[0] + p[1] * center[1] + p[2] * center[2] + p[3] >= -radius)
}
/// Box3.applyMatrix4: the eight corners' box ( an empty box stays empty ).
fn apply_box(b: [[f64; 3]; 2], m: &[f64; 16]) -> [[f64; 3]; 2] {
    let [lo, hi] = b;
    if hi[0] < lo[0] || hi[1] < lo[1] || hi[2] < lo[2] {
        return b;
    }
    let mut out = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
    for k in 0..8 {
        let p = apply4(
            [
                if k & 4 != 0 { hi[0] } else { lo[0] },
                if k & 2 != 0 { hi[1] } else { lo[1] },
                if k & 1 != 0 { hi[2] } else { lo[2] },
            ],
            m,
        );
        for a in 0..3 {
            out[0][a] = out[0][a].min(p[a]);
            out[1][a] = out[1][a].max(p[a]);
        }
    }
    out
}
fn box_size(b: &[[f64; 3]; 2]) -> [f64; 3] {
    let [lo, hi] = b;
    if hi[0] < lo[0] || hi[1] < lo[1] || hi[2] < lo[2] {
        return [0.; 3];
    }
    [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]]
}
/// LODsManager.isInside: the box's near-face centre behind the projection.
fn is_inside(b: &[[f64; 3]; 2], m: &[f64; 16]) -> bool {
    let [lo, hi] = b;
    let p = apply4([(lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, lo[2]], m);
    p[2] < 0.
}

/// SkinnedMesh.getVertexPosition for every vertex of `geometry` ( the mesh's
/// bones and bind matrices, GLTFLoader binding with the identity ).
fn skinned_positions(
    s: &Scene,
    node: Object3D,
    geometry: &BufferGeometry,
) -> Result<Vec<[f64; 3]>> {
    let n = s.get(node)?;
    let skin = n.skin.as_ref().ok_or(Error::Invalid("skin"))?;
    let bind_inverse = invert(&n.matrix_world.to_cols_array());
    let bones: Vec<[f64; 16]> = skin
        .joints
        .iter()
        .zip(&skin.inverse_bind_matrices)
        .map(|(&j, inverse)| {
            Ok(multiply(
                &s.get(j)?.matrix_world.to_cols_array(),
                &inverse.to_cols_array(),
            ))
        })
        .collect::<Result<_>>()?;
    let read = |name: &str| -> Option<Vec<f64>> {
        Some(match geometry.attributes.get(name)? {
            Attribute::F32(a) => a.array().iter().map(|&v| v as f64).collect(),
            Attribute::U16(a) => a.array().iter().map(|&v| v as f64).collect(),
            Attribute::U8(a) => a.array().iter().map(|&v| v as f64).collect(),
            _ => return None,
        })
    };
    let positions = read("position").ok_or(Error::Invalid("positions"))?;
    let indices = read("skinIndex").ok_or(Error::Invalid("skin indices"))?;
    let weights = read("skinWeight").ok_or(Error::Invalid("skin weights"))?;
    let mut out = Vec::with_capacity(positions.len() / 3);
    for v in 0..positions.len() / 3 {
        let base = [positions[3 * v], positions[3 * v + 1], positions[3 * v + 2]];
        let mut target = [0.; 3];
        for k in 0..4 {
            let weight = weights[4 * v + k];
            if weight != 0. {
                let bone = bones
                    .get(indices[4 * v + k] as usize)
                    .ok_or(Error::Invalid("bone index"))?;
                let p = apply4(base, bone);
                for a in 0..3 {
                    target[a] += p[a] * weight;
                }
            }
        }
        out.push(apply4(target, &bind_inverse));
    }
    Ok(out)
}
/// SkinnedMesh.computeBoundingBox.
fn skinned_box(s: &Scene, node: Object3D, geometry: &BufferGeometry) -> Result<[[f64; 3]; 2]> {
    let mut b = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
    for p in skinned_positions(s, node, geometry)? {
        for a in 0..3 {
            b[0][a] = b[0][a].min(p[a]);
            b[1][a] = b[1][a].max(p[a]);
        }
    }
    Ok(b)
}
/// SkinnedMesh.computeBoundingSphere: Sphere.expandByPoint over the vertices.
fn skinned_sphere(s: &Scene, node: Object3D, geometry: &BufferGeometry) -> Result<([f64; 3], f64)> {
    let (mut center, mut radius) = ([0.; 3], -1.);
    for p in skinned_positions(s, node, geometry)? {
        if radius < 0. {
            center = p;
            radius = 0.;
            continue;
        }
        let d = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
        let length_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        if length_sq > radius * radius {
            let length = length_sq.sqrt();
            let delta = (length - radius) * 0.5;
            for a in 0..3 {
                center[a] += d[a] * (delta / length);
            }
            radius += delta;
        }
    }
    Ok((center, radius))
}
